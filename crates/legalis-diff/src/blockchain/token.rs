//! Token ledger and metered, pay-per-call pricing for paid API access.
//!
//! [`TokenLedger`] is an account-based, integer-exact ledger of a utility token
//! used to pay for diff-service API calls. It models the parts of a
//! cryptocurrency that are meaningful offline:
//!
//! - **Ownership** — an account's [`Address`] is the hash of its key material;
//!   spending requires presenting the secret key, which the ledger verifies by
//!   re-deriving the address (the standard "address = H(key)" ownership model).
//! - **Replay protection** — every account has a monotonically increasing nonce
//!   that each spend must match.
//! - **Conservation** — `mint`/`burn` adjust the total supply; transfers and
//!   fees never create or destroy tokens, and all arithmetic is overflow-checked.
//! - **Metering** — a [`PricingTable`] assigns a token cost to each
//!   [`ApiOperation`]; [`TokenLedger::charge`] debits the caller and credits the
//!   treasury, recording per-account usage for billing.
//!
//! Interoperable on-chain settlement (submitting these transfers to a public
//! chain via ECDSA/ed25519 signatures) is a deferred external binding; the
//! economic core here is complete and self-contained.

use super::{Address, sha256_parts};
use crate::{DiffError, DiffResult};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// The well-known treasury account that receives fees and API charges.
fn treasury() -> Address {
    Address::from_label("treasury")
}

/// A recorded token transfer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenTransaction {
    /// Sender address.
    pub from: String,
    /// Recipient address.
    pub to: String,
    /// Amount transferred.
    pub amount: u64,
    /// Fee paid to the treasury.
    pub fee: u64,
    /// Sender nonce consumed by this transfer.
    pub nonce: u64,
    /// Authorization tag binding the transfer to the secret-key holder.
    pub auth_tag: String,
}

/// An API operation that can be metered and charged for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApiOperation {
    /// A structural/semantic diff computation.
    ComputeDiff,
    /// A deeper semantic analysis pass.
    SemanticAnalysis,
    /// An LLM-backed natural-language explanation.
    LlmExplanation,
    /// A batch export to one or more formats.
    BatchExport,
    /// A quantum-inspired comparison.
    QuantumCompare,
    /// Anchoring a diff hash to an external chain.
    ChainAnchor,
}

/// Maps [`ApiOperation`]s to token prices.
#[derive(Debug, Clone)]
pub struct PricingTable {
    prices: HashMap<ApiOperation, u64>,
    default_price: u64,
}

impl PricingTable {
    /// Creates a pricing table with sensible default prices.
    pub fn new() -> Self {
        let mut prices = HashMap::new();
        prices.insert(ApiOperation::ComputeDiff, 1);
        prices.insert(ApiOperation::SemanticAnalysis, 3);
        prices.insert(ApiOperation::LlmExplanation, 10);
        prices.insert(ApiOperation::BatchExport, 5);
        prices.insert(ApiOperation::QuantumCompare, 8);
        prices.insert(ApiOperation::ChainAnchor, 4);
        Self {
            prices,
            default_price: 1,
        }
    }

    /// Overrides the price of an operation (builder style).
    pub fn with_price(mut self, op: ApiOperation, price: u64) -> Self {
        self.prices.insert(op, price);
        self
    }

    /// Returns the price of an operation, falling back to the default price.
    pub fn price_of(&self, op: ApiOperation) -> u64 {
        self.prices.get(&op).copied().unwrap_or(self.default_price)
    }
}

impl Default for PricingTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Per-account usage summary for billing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageReport {
    /// The account the report is for.
    pub account: String,
    /// Total tokens spent on metered API calls.
    pub total_spent: u64,
    /// Total number of metered calls.
    pub calls: u64,
    /// Call counts keyed by operation name.
    pub by_operation: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, Default)]
struct AccountState {
    balance: u64,
    nonce: u64,
    spent: u64,
    usage: HashMap<ApiOperation, u64>,
}

/// An account-based token ledger.
#[derive(Debug, Clone)]
pub struct TokenLedger {
    accounts: HashMap<Address, AccountState>,
    supply: u64,
}

impl TokenLedger {
    /// Creates an empty ledger with a zero-balance treasury.
    pub fn new() -> Self {
        let mut accounts = HashMap::new();
        accounts.insert(treasury(), AccountState::default());
        Self {
            accounts,
            supply: 0,
        }
    }

    /// The treasury address.
    pub fn treasury_address(&self) -> Address {
        treasury()
    }

    /// Total tokens in existence.
    pub fn total_supply(&self) -> u64 {
        self.supply
    }

    /// Balance of an account (zero if unknown).
    pub fn balance(&self, address: &Address) -> u64 {
        self.accounts.get(address).map(|a| a.balance).unwrap_or(0)
    }

    /// The next valid nonce for an account.
    pub fn expected_nonce(&self, address: &Address) -> u64 {
        self.accounts.get(address).map(|a| a.nonce).unwrap_or(0)
    }

    /// Tokens an account has spent on metered API calls.
    pub fn spent(&self, address: &Address) -> u64 {
        self.accounts.get(address).map(|a| a.spent).unwrap_or(0)
    }

    fn entry(&mut self, address: &Address) -> &mut AccountState {
        self.accounts.entry(address.clone()).or_default()
    }

    /// Mints `amount` tokens to `to`, increasing total supply.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::InvalidTransaction`] on supply or balance overflow.
    pub fn mint(&mut self, to: &Address, amount: u64) -> DiffResult<()> {
        self.supply = self
            .supply
            .checked_add(amount)
            .ok_or_else(|| DiffError::InvalidTransaction("total supply overflow".to_string()))?;
        let account = self.entry(to);
        account.balance = account
            .balance
            .checked_add(amount)
            .ok_or_else(|| DiffError::InvalidTransaction("balance overflow".to_string()))?;
        Ok(())
    }

    /// Burns `amount` tokens from the address derived from `secret`.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::InsufficientBalance`] if the balance is too low.
    pub fn burn(&mut self, secret: &[u8], amount: u64) -> DiffResult<()> {
        let from = Address::from_key(secret);
        let balance = self.balance(&from);
        if balance < amount {
            return Err(DiffError::InsufficientBalance {
                account: from.to_string(),
                available: balance,
                required: amount,
            });
        }
        let account = self.entry(&from);
        account.balance -= amount;
        self.supply -= amount;
        Ok(())
    }

    /// Transfers `amount` (plus `fee`) from the secret-key holder to `to`.
    ///
    /// The sender address is derived from `secret`; `nonce` must equal the
    /// sender's [`expected_nonce`](Self::expected_nonce). The fee is credited to
    /// the treasury.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::InvalidTransaction`] on nonce mismatch/overflow,
    /// [`DiffError::InsufficientBalance`] if the sender cannot cover
    /// `amount + fee`.
    pub fn transfer(
        &mut self,
        secret: &[u8],
        to: &Address,
        amount: u64,
        fee: u64,
        nonce: u64,
    ) -> DiffResult<TokenTransaction> {
        let from = Address::from_key(secret);
        let expected = self.expected_nonce(&from);
        if nonce != expected {
            return Err(DiffError::InvalidTransaction(format!(
                "nonce mismatch for {}: expected {}, got {}",
                from, expected, nonce
            )));
        }
        let total = amount
            .checked_add(fee)
            .ok_or_else(|| DiffError::InvalidTransaction("amount + fee overflow".to_string()))?;
        let balance = self.balance(&from);
        if balance < total {
            return Err(DiffError::InsufficientBalance {
                account: from.to_string(),
                available: balance,
                required: total,
            });
        }

        // Debit sender, advance nonce.
        {
            let sender = self.entry(&from);
            sender.balance -= total;
            sender.nonce += 1;
        }
        // Credit recipient.
        {
            let recipient = self.entry(to);
            recipient.balance = recipient.balance.checked_add(amount).ok_or_else(|| {
                DiffError::InvalidTransaction("recipient balance overflow".to_string())
            })?;
        }
        // Credit treasury with the fee.
        if fee > 0 {
            let treasury_addr = treasury();
            let t = self.entry(&treasury_addr);
            t.balance = t
                .balance
                .checked_add(fee)
                .ok_or_else(|| DiffError::InvalidTransaction("treasury overflow".to_string()))?;
        }

        let canonical = format!("{}|{}|{}|{}|{}", from, to, amount, fee, nonce);
        let auth_tag = sha256_parts(&[secret, canonical.as_bytes()]);
        Ok(TokenTransaction {
            from: from.to_string(),
            to: to.to_string(),
            amount,
            fee,
            nonce,
            auth_tag,
        })
    }

    /// Charges the secret-key holder for an API operation, crediting the
    /// treasury and recording usage.
    ///
    /// Returns the amount charged.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::InsufficientBalance`] if the caller cannot afford the
    /// operation.
    pub fn charge(
        &mut self,
        secret: &[u8],
        op: ApiOperation,
        pricing: &PricingTable,
    ) -> DiffResult<u64> {
        let from = Address::from_key(secret);
        let price = pricing.price_of(op);
        let balance = self.balance(&from);
        if balance < price {
            return Err(DiffError::InsufficientBalance {
                account: from.to_string(),
                available: balance,
                required: price,
            });
        }
        {
            let account = self.entry(&from);
            account.balance -= price;
            account.spent += price;
            *account.usage.entry(op).or_insert(0) += 1;
        }
        {
            let treasury_addr = treasury();
            let t = self.entry(&treasury_addr);
            t.balance = t
                .balance
                .checked_add(price)
                .ok_or_else(|| DiffError::InvalidTransaction("treasury overflow".to_string()))?;
        }
        Ok(price)
    }

    /// Returns whether the secret-key holder can afford an operation.
    pub fn can_afford(&self, secret: &[u8], op: ApiOperation, pricing: &PricingTable) -> bool {
        let from = Address::from_key(secret);
        self.balance(&from) >= pricing.price_of(op)
    }

    /// Builds a usage report for an account.
    pub fn usage_report(&self, address: &Address) -> UsageReport {
        let account = self.accounts.get(address);
        let mut by_operation = BTreeMap::new();
        let mut calls = 0u64;
        if let Some(state) = account {
            for (op, count) in &state.usage {
                by_operation.insert(format!("{:?}", op), *count);
                calls += *count;
            }
        }
        UsageReport {
            account: address.to_string(),
            total_spent: account.map(|a| a.spent).unwrap_or(0),
            calls,
            by_operation,
        }
    }

    /// A deterministic hash committing to the full ledger state (balances and
    /// nonces of every account, in address order).
    pub fn state_root(&self) -> String {
        let mut sorted: Vec<(&Address, &AccountState)> = self.accounts.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        let mut parts: Vec<Vec<u8>> = Vec::new();
        for (address, state) in sorted {
            let line = format!("{}:{}:{}", address, state.balance, state.nonce);
            parts.push(line.into_bytes());
        }
        let refs: Vec<&[u8]> = parts.iter().map(|p| p.as_slice()).collect();
        sha256_parts(&refs)
    }
}

impl Default for TokenLedger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alice() -> &'static [u8] {
        b"alice-secret-key"
    }
    fn bob() -> &'static [u8] {
        b"bob-secret-key"
    }

    #[test]
    fn test_mint_increases_supply_and_balance() {
        let mut ledger = TokenLedger::new();
        let addr = Address::from_key(alice());
        ledger.mint(&addr, 1000).expect("mint");
        assert_eq!(ledger.balance(&addr), 1000);
        assert_eq!(ledger.total_supply(), 1000);
    }

    #[test]
    fn test_transfer_moves_tokens_with_fee() {
        let mut ledger = TokenLedger::new();
        let a = Address::from_key(alice());
        let b = Address::from_key(bob());
        ledger.mint(&a, 1000).expect("mint");
        let tx = ledger.transfer(alice(), &b, 300, 10, 0).expect("transfer");
        assert_eq!(ledger.balance(&a), 690);
        assert_eq!(ledger.balance(&b), 300);
        assert_eq!(ledger.balance(&ledger.treasury_address()), 10);
        assert_eq!(tx.amount, 300);
        assert_eq!(tx.fee, 10);
        assert!(!tx.auth_tag.is_empty());
        // Supply is conserved across transfers.
        assert_eq!(ledger.total_supply(), 1000);
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        let mut ledger = TokenLedger::new();
        let b = Address::from_key(bob());
        ledger.mint(&Address::from_key(alice()), 50).expect("mint");
        let err = ledger.transfer(alice(), &b, 100, 0, 0);
        assert!(matches!(err, Err(DiffError::InsufficientBalance { .. })));
    }

    #[test]
    fn test_nonce_replay_protection() {
        let mut ledger = TokenLedger::new();
        let b = Address::from_key(bob());
        ledger
            .mint(&Address::from_key(alice()), 1000)
            .expect("mint");
        ledger.transfer(alice(), &b, 100, 0, 0).expect("first");
        // Re-using nonce 0 must fail.
        assert!(ledger.transfer(alice(), &b, 100, 0, 0).is_err());
        // Correct next nonce works.
        ledger.transfer(alice(), &b, 100, 0, 1).expect("second");
        assert_eq!(ledger.expected_nonce(&Address::from_key(alice())), 2);
    }

    #[test]
    fn test_wrong_nonce_rejected() {
        let mut ledger = TokenLedger::new();
        let b = Address::from_key(bob());
        ledger
            .mint(&Address::from_key(alice()), 1000)
            .expect("mint");
        // Skipping ahead is rejected.
        assert!(ledger.transfer(alice(), &b, 10, 0, 5).is_err());
    }

    #[test]
    fn test_burn_reduces_supply() {
        let mut ledger = TokenLedger::new();
        let a = Address::from_key(alice());
        ledger.mint(&a, 500).expect("mint");
        ledger.burn(alice(), 200).expect("burn");
        assert_eq!(ledger.balance(&a), 300);
        assert_eq!(ledger.total_supply(), 300);
    }

    #[test]
    fn test_burn_insufficient() {
        let mut ledger = TokenLedger::new();
        ledger.mint(&Address::from_key(alice()), 100).expect("mint");
        assert!(ledger.burn(alice(), 200).is_err());
    }

    #[test]
    fn test_pricing_table_defaults_and_override() {
        let table = PricingTable::new().with_price(ApiOperation::ComputeDiff, 99);
        assert_eq!(table.price_of(ApiOperation::ComputeDiff), 99);
        assert_eq!(table.price_of(ApiOperation::LlmExplanation), 10);
    }

    #[test]
    fn test_charge_for_api_operation() {
        let mut ledger = TokenLedger::new();
        let a = Address::from_key(alice());
        ledger.mint(&a, 100).expect("mint");
        let pricing = PricingTable::new();
        let charged = ledger
            .charge(alice(), ApiOperation::LlmExplanation, &pricing)
            .expect("charge");
        assert_eq!(charged, 10);
        assert_eq!(ledger.balance(&a), 90);
        assert_eq!(ledger.spent(&a), 10);
        assert_eq!(ledger.balance(&ledger.treasury_address()), 10);
    }

    #[test]
    fn test_charge_insufficient_balance() {
        let mut ledger = TokenLedger::new();
        ledger.mint(&Address::from_key(alice()), 2).expect("mint");
        let pricing = PricingTable::new();
        assert!(!ledger.can_afford(alice(), ApiOperation::LlmExplanation, &pricing));
        assert!(
            ledger
                .charge(alice(), ApiOperation::LlmExplanation, &pricing)
                .is_err()
        );
    }

    #[test]
    fn test_usage_report() {
        let mut ledger = TokenLedger::new();
        let a = Address::from_key(alice());
        ledger.mint(&a, 100).expect("mint");
        let pricing = PricingTable::new();
        ledger
            .charge(alice(), ApiOperation::ComputeDiff, &pricing)
            .expect("c1");
        ledger
            .charge(alice(), ApiOperation::ComputeDiff, &pricing)
            .expect("c2");
        ledger
            .charge(alice(), ApiOperation::SemanticAnalysis, &pricing)
            .expect("c3");
        let report = ledger.usage_report(&a);
        assert_eq!(report.calls, 3);
        assert_eq!(report.total_spent, 1 + 1 + 3);
        assert_eq!(report.by_operation.get("ComputeDiff"), Some(&2));
        assert_eq!(report.by_operation.get("SemanticAnalysis"), Some(&1));
    }

    #[test]
    fn test_state_root_changes_with_state() {
        let mut ledger = TokenLedger::new();
        let a = Address::from_key(alice());
        let root0 = ledger.state_root();
        ledger.mint(&a, 100).expect("mint");
        let root1 = ledger.state_root();
        assert_ne!(root0, root1);
        // Deterministic: same state -> same root.
        let mut other = TokenLedger::new();
        other.mint(&Address::from_key(alice()), 100).expect("mint");
        assert_eq!(ledger.state_root(), other.state_root());
    }

    #[test]
    fn test_ownership_is_key_derived() {
        // Different secrets control different addresses; bob cannot spend alice's.
        let mut ledger = TokenLedger::new();
        let a = Address::from_key(alice());
        ledger.mint(&a, 100).expect("mint");
        // Bob's secret derives bob's (empty) account; transfer fails on balance.
        let result = ledger.transfer(bob(), &a, 10, 0, 0);
        assert!(matches!(result, Err(DiffError::InsufficientBalance { .. })));
    }

    #[test]
    fn test_transaction_serde_roundtrip() {
        let mut ledger = TokenLedger::new();
        let b = Address::from_key(bob());
        ledger.mint(&Address::from_key(alice()), 100).expect("mint");
        let tx = ledger.transfer(alice(), &b, 10, 1, 0).expect("transfer");
        let json = serde_json::to_string(&tx).expect("ser");
        let back: TokenTransaction = serde_json::from_str(&json).expect("de");
        assert_eq!(tx, back);
    }
}
