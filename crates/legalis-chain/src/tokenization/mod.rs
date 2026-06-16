//! Real-world asset (RWA) tokenization generators.
//!
//! This module extends [`ContractGenerator`] with production-grade generators for
//! tokenizing real-world assets into smart contracts:
//!
//! * **Real estate** — security-token style fractional ownership of a property with
//!   regulatory lock-ups, KYC gating and pull-based rental-income distribution.
//! * **Commodities** — asset-backed tokens redeemable against a custodied physical
//!   reserve, with optional Chainlink proof-of-reserves attestation.
//! * **Intellectual property** — ERC-721 NFTs representing patents, trademarks and
//!   copyrights with on-chain licensing and EIP-2981 royalties.
//! * **Revenue sharing** — pull-payment splitters that distribute received revenue
//!   pro-rata across a set of legally-defined ownership allocations.
//! * **Fractionalized ownership** — vaults that lock an NFT and mint fungible
//!   fractions with a reserve-price buyout mechanism.
//!
//! The legal-domain modeling lives in pure Rust ([`validate_ownership_allocations`],
//! [`distribute_revenue`], [`price_per_share`]) so that the apportionment math is
//! validated *before* any contract source is emitted, and is independently testable.

mod commodity;
mod fractional;
mod ip_nft;
mod real_estate;
mod revenue_share;

#[cfg(test)]
mod tests;

use super::contractgenerator_type::ContractGenerator;
use super::functions::ChainResult;
use super::types_19::{ChainError, GeneratedContract, Jurisdiction, TargetPlatform};

/// Basis-point denominator (`10_000` basis points == 100%).
///
/// All fractional ownership and revenue-share allocations in this module are
/// expressed in basis points so that splits can be represented without floating
/// point and validated for exact summation.
pub const BASIS_POINTS_DENOMINATOR: u32 = 10_000;

/// Classification of a tokenized real-estate asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyType {
    /// Residential dwelling (single- or multi-family).
    Residential,
    /// Commercial property (office, retail).
    Commercial,
    /// Industrial property (warehouse, factory).
    Industrial,
    /// Undeveloped land / parcel.
    Land,
    /// Mixed-use development.
    MixedUse,
    /// Agricultural / farmland.
    Agricultural,
}

impl PropertyType {
    /// Human-readable label used in generated NatSpec documentation.
    pub(crate) fn label(self) -> &'static str {
        match self {
            PropertyType::Residential => "Residential",
            PropertyType::Commercial => "Commercial",
            PropertyType::Industrial => "Industrial",
            PropertyType::Land => "Land",
            PropertyType::MixedUse => "Mixed-Use",
            PropertyType::Agricultural => "Agricultural",
        }
    }
}

/// Type of physical commodity backing a commodity token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommodityType {
    /// Gold bullion.
    Gold,
    /// Silver bullion.
    Silver,
    /// Crude oil.
    CrudeOil,
    /// Natural gas.
    NaturalGas,
    /// Wheat / grain.
    Wheat,
    /// Coffee.
    Coffee,
    /// Copper.
    Copper,
    /// Any other commodity, named explicitly.
    Custom(String),
}

impl CommodityType {
    /// Human-readable label used in generated NatSpec documentation.
    pub(crate) fn label(&self) -> &str {
        match self {
            CommodityType::Gold => "Gold",
            CommodityType::Silver => "Silver",
            CommodityType::CrudeOil => "Crude Oil",
            CommodityType::NaturalGas => "Natural Gas",
            CommodityType::Wheat => "Wheat",
            CommodityType::Coffee => "Coffee",
            CommodityType::Copper => "Copper",
            CommodityType::Custom(name) => name.as_str(),
        }
    }
}

/// Category of intellectual-property right represented by an IP NFT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpAssetType {
    /// Granted patent.
    Patent,
    /// Registered trademark.
    Trademark,
    /// Copyright.
    Copyright,
    /// Trade secret / know-how.
    TradeSecret,
    /// Registered design right.
    DesignRight,
}

impl IpAssetType {
    /// Human-readable label used in generated NatSpec documentation.
    pub(crate) fn label(self) -> &'static str {
        match self {
            IpAssetType::Patent => "Patent",
            IpAssetType::Trademark => "Trademark",
            IpAssetType::Copyright => "Copyright",
            IpAssetType::TradeSecret => "Trade Secret",
            IpAssetType::DesignRight => "Design Right",
        }
    }
}

/// Redemption policy for an asset-backed commodity token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedemptionPolicy {
    /// Holders may redeem tokens for physical delivery of the commodity.
    PhysicalDelivery,
    /// Holders may redeem tokens for cash settlement at market value.
    CashSettlement,
    /// Both physical delivery and cash settlement are offered.
    Both,
    /// Tokens are not redeemable (synthetic exposure only).
    NonRedeemable,
}

impl RedemptionPolicy {
    /// Human-readable label used in generated NatSpec documentation.
    pub(crate) fn label(self) -> &'static str {
        match self {
            RedemptionPolicy::PhysicalDelivery => "Physical delivery",
            RedemptionPolicy::CashSettlement => "Cash settlement",
            RedemptionPolicy::Both => "Physical delivery or cash settlement",
            RedemptionPolicy::NonRedeemable => "Non-redeemable",
        }
    }

    /// Whether the policy permits physical delivery.
    pub(crate) fn allows_physical(self) -> bool {
        matches!(
            self,
            RedemptionPolicy::PhysicalDelivery | RedemptionPolicy::Both
        )
    }

    /// Whether the policy permits cash settlement.
    pub(crate) fn allows_cash(self) -> bool {
        matches!(
            self,
            RedemptionPolicy::CashSettlement | RedemptionPolicy::Both
        )
    }
}

/// Cadence at which accrued revenue becomes claimable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevenueDistributionFrequency {
    /// Funds become claimable as soon as they are received.
    OnReceipt,
    /// Funds become claimable once per day.
    Daily,
    /// Funds become claimable once per week.
    Weekly,
    /// Funds become claimable once per (30-day) month.
    Monthly,
    /// Funds become claimable once per (90-day) quarter.
    Quarterly,
}

impl RevenueDistributionFrequency {
    /// The minimum number of seconds between successive distribution epochs.
    ///
    /// `OnReceipt` returns `0` (no enforced delay); the other cadences return
    /// their canonical period length used to gate the on-chain epoch counter.
    pub(crate) fn period_seconds(self) -> u64 {
        match self {
            RevenueDistributionFrequency::OnReceipt => 0,
            RevenueDistributionFrequency::Daily => 86_400,
            RevenueDistributionFrequency::Weekly => 604_800,
            RevenueDistributionFrequency::Monthly => 2_592_000,
            RevenueDistributionFrequency::Quarterly => 7_776_000,
        }
    }

    /// Human-readable label used in generated NatSpec documentation.
    pub(crate) fn label(self) -> &'static str {
        match self {
            RevenueDistributionFrequency::OnReceipt => "on receipt",
            RevenueDistributionFrequency::Daily => "daily",
            RevenueDistributionFrequency::Weekly => "weekly",
            RevenueDistributionFrequency::Monthly => "monthly",
            RevenueDistributionFrequency::Quarterly => "quarterly",
        }
    }
}

/// A single ownership allocation expressed in basis points.
///
/// A complete set of allocations passed to [`validate_ownership_allocations`] or
/// [`distribute_revenue`] must reference unique holders and have `basis_points`
/// summing to exactly [`BASIS_POINTS_DENOMINATOR`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnershipAllocation {
    /// Holder identifier (address, legal-entity id, or label).
    pub holder: String,
    /// Share of the asset expressed in basis points (1 == 0.01%).
    pub basis_points: u32,
}

/// One holder's computed slice of a revenue distribution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevenueDistribution {
    /// Holder identifier mirroring the input [`OwnershipAllocation::holder`].
    pub holder: String,
    /// Amount apportioned to this holder, in the smallest payment unit.
    pub amount: u128,
}

/// Real-estate tokenization configuration.
#[derive(Debug, Clone)]
pub struct RealEstateToken {
    /// Token name (e.g. `"123 Main Street Holdings"`).
    pub name: String,
    /// Token symbol (e.g. `"MAIN"`).
    pub symbol: String,
    /// Classification of the underlying property.
    pub property_type: PropertyType,
    /// Legal identifier of the property (title/parcel/registry number).
    pub property_identifier: String,
    /// Appraised value in the smallest unit of the reference currency (e.g. USD cents).
    pub appraised_value: u64,
    /// Total number of fractional shares to mint.
    pub total_shares: u64,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
    /// Whether transfers require KYC-verified counterparties.
    pub kyc_required: bool,
    /// Whether rental-income distribution is enabled.
    pub rental_income_enabled: bool,
    /// Regulatory transfer lock-up period in days (0 == none).
    pub transfer_lockup_days: u32,
    /// ERC-20 token used to pay dividends; `None` distributes the native coin.
    pub dividend_token: Option<String>,
}

/// Commodity tokenization configuration.
#[derive(Debug, Clone)]
pub struct CommodityToken {
    /// Token name (e.g. `"Vaulted Gold"`).
    pub name: String,
    /// Token symbol (e.g. `"VGLD"`).
    pub symbol: String,
    /// The physical commodity backing the token.
    pub commodity_type: CommodityType,
    /// Label of the backing unit (e.g. `"troy ounce"`, `"barrel"`).
    pub unit_label: String,
    /// Backing units (scaled by 10^9, i.e. nano-units) represented by one whole token.
    pub backing_units_per_token: u64,
    /// Name of the custodian / vault operator holding the reserve.
    pub custodian: String,
    /// Redemption policy offered to holders.
    pub redemption: RedemptionPolicy,
    /// Whether a Chainlink proof-of-reserves oracle gates minting.
    pub proof_of_reserves: bool,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
}

/// Intellectual-property NFT configuration.
#[derive(Debug, Clone)]
pub struct IpNft {
    /// Collection name.
    pub name: String,
    /// Collection symbol.
    pub symbol: String,
    /// Category of IP right represented.
    pub asset_type: IpAssetType,
    /// Registration / filing number with the relevant authority.
    pub registration_number: String,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
    /// Whether on-chain licensing is enabled.
    pub licensing_enabled: bool,
    /// EIP-2981 secondary-sale royalty in basis points (must be <= 10000).
    pub royalty_basis_points: u16,
    /// Address receiving EIP-2981 royalties.
    pub royalty_receiver: String,
    /// Optional Unix timestamp at which the IP right expires.
    pub expiry_timestamp: Option<u64>,
    /// Whether the NFT may be transferred (some IP rights are non-assignable).
    pub transferable: bool,
}

/// Revenue-sharing contract configuration.
#[derive(Debug, Clone)]
pub struct RevenueShareContract {
    /// Contract name.
    pub name: String,
    /// ERC-20 token whose balance is distributed; `None` distributes the native coin.
    pub payment_token: Option<String>,
    /// Ownership allocations; must sum to [`BASIS_POINTS_DENOMINATOR`].
    pub allocations: Vec<OwnershipAllocation>,
    /// Cadence at which received revenue becomes claimable.
    pub distribution_frequency: RevenueDistributionFrequency,
    /// Whether to use pull payments (recommended) instead of push transfers.
    pub pull_payments: bool,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
}

/// Fractionalized-ownership (NFT vault) configuration.
#[derive(Debug, Clone)]
pub struct FractionalOwnership {
    /// Fraction token name.
    pub name: String,
    /// Fraction token symbol.
    pub symbol: String,
    /// Address of the ERC-721 contract whose token is being fractionalized.
    pub underlying_nft: String,
    /// Token id locked in the vault.
    pub underlying_token_id: u64,
    /// Total number of fungible fractions to mint.
    pub total_fractions: u64,
    /// Reserve (buyout) price in the smallest payment unit.
    pub reserve_price: u64,
    /// Curator address entitled to a fee on buyout.
    pub curator: String,
    /// Curator fee in basis points (must be <= 10000).
    pub curator_fee_basis_points: u16,
    /// Whether the reserve-price buyout mechanism is enabled.
    pub buyout_enabled: bool,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
}

/// Returns `true` if `platform` produces EVM bytecode and therefore accepts
/// Solidity source generated by this module.
pub(crate) fn is_evm_target(platform: TargetPlatform) -> bool {
    matches!(
        platform,
        TargetPlatform::Solidity
            | TargetPlatform::ZkSyncEra
            | TargetPlatform::Base
            | TargetPlatform::PolygonZkEvm
            | TargetPlatform::Scroll
            | TargetPlatform::Linea
            | TargetPlatform::AvalancheSubnet
    )
}

/// Validates a set of ownership allocations for legal coherence.
///
/// Allocations must be non-empty, reference non-empty unique holders, have
/// strictly positive basis points, and sum to exactly [`BASIS_POINTS_DENOMINATOR`].
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] describing the first violated invariant.
pub fn validate_ownership_allocations(allocations: &[OwnershipAllocation]) -> ChainResult<()> {
    if allocations.is_empty() {
        return Err(ChainError::GenerationError(
            "ownership allocations must not be empty".to_string(),
        ));
    }
    let mut total: u32 = 0;
    for allocation in allocations {
        if allocation.holder.trim().is_empty() {
            return Err(ChainError::GenerationError(
                "ownership allocation holder must not be empty".to_string(),
            ));
        }
        if allocation.basis_points == 0 {
            return Err(ChainError::GenerationError(format!(
                "ownership allocation for '{}' must be greater than zero",
                allocation.holder
            )));
        }
        total = total.checked_add(allocation.basis_points).ok_or_else(|| {
            ChainError::GenerationError("ownership allocations overflow u32".to_string())
        })?;
    }
    if total != BASIS_POINTS_DENOMINATOR {
        return Err(ChainError::GenerationError(format!(
            "ownership allocations must sum to {BASIS_POINTS_DENOMINATOR} basis points (got {total})"
        )));
    }
    for (index, allocation) in allocations.iter().enumerate() {
        for other in allocations.iter().skip(index + 1) {
            if allocation.holder == other.holder {
                return Err(ChainError::GenerationError(format!(
                    "duplicate holder in allocations: '{}'",
                    allocation.holder
                )));
            }
        }
    }
    Ok(())
}

/// Apportions `total` revenue across `allocations` using the largest-remainder
/// (Hamilton) method so that the returned amounts sum *exactly* to `total`.
///
/// Each holder first receives `floor(total * basis_points / 10000)`. The leftover
/// units (caused by integer truncation) are then distributed one at a time to the
/// holders with the largest fractional remainders, ties broken by input order.
/// This guarantees no "dust" is lost — a legal requirement for faithful
/// pro-rata distribution.
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] if the allocations are invalid (see
/// [`validate_ownership_allocations`]) or if the basis-point multiplication would
/// overflow `u128`.
pub fn distribute_revenue(
    total: u128,
    allocations: &[OwnershipAllocation],
) -> ChainResult<Vec<RevenueDistribution>> {
    validate_ownership_allocations(allocations)?;
    let denominator = u128::from(BASIS_POINTS_DENOMINATOR);
    let mut entries: Vec<RevenueDistribution> = Vec::with_capacity(allocations.len());
    let mut remainders: Vec<(usize, u128)> = Vec::with_capacity(allocations.len());
    let mut allocated: u128 = 0;
    for (index, allocation) in allocations.iter().enumerate() {
        let numerator = total
            .checked_mul(u128::from(allocation.basis_points))
            .ok_or_else(|| {
                ChainError::GenerationError(
                    "revenue distribution overflow: total too large for basis-point math"
                        .to_string(),
                )
            })?;
        let base = numerator / denominator;
        let remainder = numerator % denominator;
        allocated = allocated.checked_add(base).ok_or_else(|| {
            ChainError::GenerationError("revenue distribution overflow on accumulation".to_string())
        })?;
        entries.push(RevenueDistribution {
            holder: allocation.holder.clone(),
            amount: base,
        });
        remainders.push((index, remainder));
    }
    // `allocated <= total` always holds, so this subtraction cannot underflow.
    let leftover = total - allocated;
    let leftover_count = usize::try_from(leftover).map_err(|_| {
        ChainError::GenerationError("revenue distribution leftover exceeds index range".to_string())
    })?;
    remainders.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    for slot in remainders.iter().take(leftover_count) {
        if let Some(entry) = entries.get_mut(slot.0) {
            entry.amount = entry.amount.saturating_add(1);
        }
    }
    Ok(entries)
}

/// Computes the price of a single share given a total asset value.
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] if `total_shares` is zero.
pub fn price_per_share(total_value: u64, total_shares: u64) -> ChainResult<u64> {
    if total_shares == 0 {
        return Err(ChainError::GenerationError(
            "total_shares must be greater than zero".to_string(),
        ));
    }
    Ok(total_value / total_shares)
}

/// Returns the canonical compliance-framework tag for a jurisdiction, embedded
/// into generated contracts so downstream tooling can route compliance checks.
pub(crate) fn jurisdiction_compliance_tag(jurisdiction: Jurisdiction) -> &'static str {
    match jurisdiction {
        Jurisdiction::Us => "US-SEC/Reg-D",
        Jurisdiction::Eu => "EU-MiCA",
        Jurisdiction::Uk => "UK-FCA",
        Jurisdiction::Sg => "SG-MAS",
        Jurisdiction::Jp => "JP-FSA",
        Jurisdiction::Ch => "CH-FINMA",
        Jurisdiction::Custom => "CUSTOM",
    }
}

impl ContractGenerator {
    /// Generates a real-estate tokenization contract (fractional property shares).
    ///
    /// Emits an EVM security-token contract with KYC gating, regulatory transfer
    /// lock-ups and pull-based rental-income distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_chain::{
    ///     ContractGenerator, Jurisdiction, PropertyType, RealEstateToken, TargetPlatform,
    /// };
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let config = RealEstateToken {
    ///     name: "123 Main Street".to_string(),
    ///     symbol: "MAIN".to_string(),
    ///     property_type: PropertyType::Residential,
    ///     property_identifier: "TITLE-0001".to_string(),
    ///     appraised_value: 1_000_000_00,
    ///     total_shares: 100_000,
    ///     jurisdiction: Jurisdiction::Us,
    ///     kyc_required: true,
    ///     rental_income_enabled: true,
    ///     transfer_lockup_days: 365,
    ///     dividend_token: None,
    /// };
    /// let contract = generator.generate_real_estate_token(&config).unwrap();
    /// assert!(contract.source.contains("contract"));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if `total_shares` is zero.
    pub fn generate_real_estate_token(
        &self,
        config: &RealEstateToken,
    ) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_real_estate_token(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "Real estate tokenization not supported for {:?}",
                self.platform
            )))
        }
    }

    /// Generates a commodity tokenization contract (asset-backed token).
    ///
    /// Emits an EVM token backed by a custodied physical reserve, with optional
    /// Chainlink proof-of-reserves attestation and a configurable redemption flow.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if `backing_units_per_token` is zero.
    pub fn generate_commodity_token(
        &self,
        config: &CommodityToken,
    ) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_commodity_token(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "Commodity tokenization not supported for {:?}",
                self.platform
            )))
        }
    }

    /// Generates an intellectual-property NFT contract.
    ///
    /// Emits an ERC-721 collection with EIP-2981 royalties, optional on-chain
    /// licensing, optional expiry and optional non-transferability.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if `royalty_basis_points` exceeds 100%.
    pub fn generate_ip_nft(&self, config: &IpNft) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_ip_nft(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "IP NFT generation not supported for {:?}",
                self.platform
            )))
        }
    }

    /// Generates a revenue-sharing (pull-payment splitter) contract.
    ///
    /// Validates that the configured allocations sum to 100% before emitting an
    /// EVM contract that apportions received revenue pro-rata.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if the allocations are invalid (see
    /// [`validate_ownership_allocations`]).
    pub fn generate_revenue_share(
        &self,
        config: &RevenueShareContract,
    ) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_revenue_share(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "Revenue sharing not supported for {:?}",
                self.platform
            )))
        }
    }

    /// Generates a fractionalized-ownership (NFT vault) contract.
    ///
    /// Emits an EVM vault that escrows an ERC-721, mints fungible fractions and
    /// (optionally) exposes a reserve-price buyout that redeems the fractions.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if `total_fractions` is zero, or if
    /// `curator_fee_basis_points` exceeds 100%.
    pub fn generate_fractional_ownership(
        &self,
        config: &FractionalOwnership,
    ) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_fractional_ownership(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "Fractional ownership not supported for {:?}",
                self.platform
            )))
        }
    }
}
