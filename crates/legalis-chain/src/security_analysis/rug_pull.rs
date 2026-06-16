//! Rug-pull risk detector.
//!
//! Part of the `security_analysis` module. Flags owner-drain / holder-dilution
//! indicators in generated Solidity source: unrestricted or uncapped minting,
//! owner withdrawal of all funds, mutable fees with no ceiling, and privileged
//! upgrade paths without a timelock. These are the levers a malicious deployer
//! pulls to "rug" holders.

use super::scan;
use super::{FindingCategory, SecurityFinding};
use crate::types::Severity;
use crate::types_19::GeneratedContract;

/// Detects rug-pull risk indicators in `contract`'s source.
pub fn detect_rug_pull_risks(contract: &GeneratedContract) -> Vec<SecurityFinding> {
    let source = &contract.source;
    let mut findings = Vec::new();
    if !scan::is_solidity_like(source) {
        return findings;
    }

    detect_unrestricted_mint(source, &mut findings);
    detect_owner_drain(source, &mut findings);
    detect_uncapped_fee(source, &mut findings);
    detect_unguarded_upgrade(source, &mut findings);
    detect_mutable_max_tx(source, &mut findings);

    findings
}

/// An owner-callable `mint` with no supply cap lets the deployer dilute every
/// holder to zero at will.
fn detect_unrestricted_mint(source: &str, findings: &mut Vec<SecurityFinding>) {
    let has_owner_mint = scan::contains_any(
        source,
        &["function mint(address", "function mint(", "_mint("],
    ) && scan::contains_any(
        source,
        &["onlyOwner", "onlyRole", "DEFAULT_ADMIN_ROLE", "MINTER_ROLE"],
    );
    if !has_owner_mint {
        return;
    }
    // A cap (ERC20Capped, an explicit require on totalSupply, or a hard max)
    // bounds dilution. Its absence is the risk.
    let has_cap = scan::contains_any(
        source,
        &[
            "ERC20Capped",
            "require(totalSupply() + amount <= cap",
            "require(totalSupply() + amount <= MAX_SUPPLY",
            "require(totalSupply() <= cap",
            "MAX_SUPPLY",
            "<= cap",
        ],
    );
    if !has_cap {
        findings.push(SecurityFinding::new(
            FindingCategory::RugPull,
            "UNCAPPED_OWNER_MINT",
            Severity::High,
            "Owner can mint unlimited supply",
            "A privileged mint function with no enforced supply cap lets the owner \
             create arbitrary new tokens, diluting existing holders to \
             near-zero value — a primary rug-pull lever.",
            "Cap the supply (e.g. ERC20Capped or require(totalSupply()+amount <= \
             MAX_SUPPLY)), or remove minting after the initial distribution and \
             renounce the minter role.",
            scan::first_line(source, "function mint")
                .or_else(|| scan::first_line(source, "_mint(")),
        ));
    }
}

/// An owner function that sweeps the entire contract balance / token holdings is a
/// direct drain.
fn detect_owner_drain(source: &str, findings: &mut Vec<SecurityFinding>) {
    let drains_eth = scan::contains_any(
        source,
        &[
            "payable(owner()).transfer(address(this).balance)",
            "payable(msg.sender).transfer(address(this).balance)",
            "owner().call{value: address(this).balance}",
            "payable(owner).transfer(address(this).balance)",
        ],
    );
    let drains_tokens = scan::contains_any(
        source,
        &[
            "transfer(owner(), balanceOf(address(this)))",
            "safeTransfer(owner(), ",
            "transfer(msg.sender, token.balanceOf(address(this)))",
        ],
    );
    let owner_gated = scan::contains_any(source, &["onlyOwner", "onlyRole", "_checkOwner"]);
    if (drains_eth || drains_tokens) && owner_gated {
        findings.push(SecurityFinding::new(
            FindingCategory::RugPull,
            "OWNER_FUND_DRAIN",
            Severity::High,
            "Owner can withdraw the entire balance",
            "An owner-only function transfers the contract's whole ETH or token \
             balance to the owner. In a pooled/escrow/liquidity context this lets \
             the deployer abscond with user funds in a single transaction.",
            "Replace owner sweeps with pull-payments tied to individual \
             entitlements, or route any treasury withdrawal through timelocked, \
             multi-sig governance with per-period limits.",
            scan::first_line(source, "address(this).balance"),
        ));
    }
}

/// A fee/tax setter with no enforced maximum lets the owner crank fees to confiscatory levels.
fn detect_uncapped_fee(source: &str, findings: &mut Vec<SecurityFinding>) {
    let has_fee_setter = scan::contains_any(
        source,
        &[
            "function setFee(",
            "function setFees(",
            "function setTax(",
            "function setFeeBps(",
            "function updateFee(",
        ],
    );
    if !has_fee_setter {
        return;
    }
    let has_cap = scan::contains_any(
        source,
        &[
            "require(fee <=",
            "require(newFee <=",
            "require(_fee <=",
            "require(feeBps <=",
            "require(bps <=",
            "MAX_FEE",
            "MAX_TAX",
            "MAX_FEE_BPS",
        ],
    );
    if !has_cap {
        findings.push(SecurityFinding::new(
            FindingCategory::RugPull,
            "UNCAPPED_FEE",
            Severity::Medium,
            "Fee/tax is owner-settable with no maximum",
            "The owner can change the protocol fee with no on-chain ceiling, and \
             could raise it to a confiscatory level (e.g. 100%) to capture all \
             flows — a slow rug.",
            "Enforce an immutable maximum on every fee setter (require(newFee <= \
             MAX_FEE_BPS)) and emit an event so changes are observable.",
            scan::first_line(source, "function setFee")
                .or_else(|| scan::first_line(source, "function setTax")),
        ));
    }
}

/// An upgradeable contract (UUPS/Transparent) whose upgrade authorization has no
/// timelock lets the owner instantly swap in malicious logic.
fn detect_unguarded_upgrade(source: &str, findings: &mut Vec<SecurityFinding>) {
    let is_upgradeable = scan::contains_any(
        source,
        &[
            "_authorizeUpgrade",
            "UUPSUpgradeable",
            "upgradeToAndCall",
            "function upgradeTo(",
        ],
    );
    if !is_upgradeable {
        return;
    }
    // A timelock controller / delay on the upgrade path mitigates instant swaps.
    let has_timelock = scan::contains_any(
        source,
        &[
            "TimelockController",
            "timelock",
            "Timelock",
            "require(block.timestamp >= eta",
            "executeAfter",
            "UpgradeScheduled",
        ],
    );
    if !has_timelock {
        findings.push(SecurityFinding::new(
            FindingCategory::RugPull,
            "INSTANT_UPGRADE",
            Severity::High,
            "Upgradeable contract has no upgrade timelock",
            "An owner-authorized upgrade with no timelock can replace the contract \
             logic in a single block — for example swapping in code that drains \
             balances — leaving holders no time to exit. This is functionally a \
             rug-pull capability.",
            "Route upgrades through a TimelockController (or an explicit \
             schedule/execute delay) governed by a multi-sig/DAO, so any upgrade is \
             announced before it can take effect.",
            scan::first_line(source, "_authorizeUpgrade")
                .or_else(|| scan::first_line(source, "upgradeToAndCall")),
        ));
    }
}

/// A `maxTxAmount`/`maxWallet` the owner can set to zero post-launch can freeze
/// all trading (a trading-disable rug). Distinct from honeypot blacklist.
fn detect_mutable_max_tx(source: &str, findings: &mut Vec<SecurityFinding>) {
    let has_max_tx_setter = scan::contains_any(
        source,
        &[
            "function setMaxTx",
            "function setMaxWallet",
            "function setMaxTransaction",
        ],
    );
    let enforces_in_transfer = scan::contains_any(
        source,
        &["<= maxTxAmount", "<= maxWallet", "require(amount <= maxTx"],
    );
    let has_floor = scan::contains_any(
        source,
        &["require(newMax >=", "MIN_MAX_TX", "require(amount_ >="],
    );
    if has_max_tx_setter && enforces_in_transfer && !has_floor {
        findings.push(SecurityFinding::new(
            FindingCategory::RugPull,
            "MUTABLE_MAX_TX_FREEZE",
            Severity::Medium,
            "Owner can set max-transaction limit to zero",
            "A transfer-size limit the owner can lower without a floor can be set \
             to zero, freezing all transfers/sells while the owner exits — a \
             trading-disable rug.",
            "Enforce a minimum on any max-tx/max-wallet setter, or make the limit \
             one-way (only loosenable) and renounce the setter after launch.",
            scan::first_line(source, "function setMaxTx")
                .or_else(|| scan::first_line(source, "function setMaxWallet")),
        ));
    }
}
