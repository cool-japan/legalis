//! Honeypot detector.
//!
//! Part of the `security_analysis` module. A "honeypot" token lets victims *buy*
//! (or deposit) freely but silently prevents them from *selling* (or
//! withdrawing), trapping their funds for the deployer. This detector flags the
//! asymmetric transfer/withdraw anti-patterns characteristic of such contracts in
//! the generated Solidity source.

use super::scan;
use super::{FindingCategory, SecurityFinding};
use crate::types::Severity;
use crate::types_19::GeneratedContract;

/// Detects honeypot anti-patterns in `contract`'s source.
pub fn detect_honeypots(contract: &GeneratedContract) -> Vec<SecurityFinding> {
    let source = &contract.source;
    let mut findings = Vec::new();
    if !scan::is_solidity_like(source) {
        return findings;
    }

    detect_owner_only_transfer_gate(source, &mut findings);
    detect_blacklist_trap(source, &mut findings);
    detect_asymmetric_tax(source, &mut findings);
    detect_fake_withdraw(source, &mut findings);
    detect_unconditional_transfer_revert(source, &mut findings);

    findings
}

/// A transfer hook that requires the sender to be the owner / a whitelisted
/// address turns the token into one only the deployer can move — buyers are stuck.
fn detect_owner_only_transfer_gate(source: &str, findings: &mut Vec<SecurityFinding>) {
    // Look for a transfer-path guard tying transfers to owner/whitelist with no
    // public opt-in path. Heuristic markers used by real honeypots.
    let in_transfer_hook = scan::contains_any(
        source,
        &[
            "_beforeTokenTransfer",
            "_update(",
            "function transfer(",
            "function transferFrom(",
        ],
    );
    let owner_gate = scan::contains_any(
        source,
        &[
            "require(from == owner",
            "require(msg.sender == owner, \"\")",
            "require(_canTransfer",
            "require(canTransfer[",
            "require(whitelisted[from]",
            "require(isWhitelisted[msg.sender]",
            "require(_whitelist[from]",
        ],
    );
    // Exclude legitimate compliance gating (SEC/KYC contracts) which expose a
    // documented public/role path AND emit a ComplianceViolation, not a silent
    // trap. Those carry an explicit accreditation/whitelist admin function.
    let has_public_optin = scan::contains_any(
        source,
        &[
            "function addToWhitelist",
            "function whitelistAddress",
            "function setCanTransfer",
            "addAccreditedInvestor",
            "function giveConsent",
        ],
    );
    if in_transfer_hook && owner_gate && !has_public_optin {
        findings.push(SecurityFinding::new(
            FindingCategory::Honeypot,
            "OWNER_ONLY_TRANSFER",
            Severity::Critical,
            "Transfers restricted to owner/whitelist with no opt-in",
            "The transfer path is gated on the owner or a private whitelist with no \
             public/role-administered way for ordinary holders to become \
             transfer-eligible. Buyers can receive tokens but can never move or \
             sell them — the defining honeypot trap.",
            "Remove the hidden transfer gate, or expose a transparent, \
             role-administered eligibility mechanism and document the policy.",
            scan::first_line(source, "require(from == owner")
                .or_else(|| scan::first_line(source, "require(whitelisted[from]")),
        ));
    }
}

/// A default-deny blacklist (everyone blacklisted unless owner clears them), or a
/// blacklist that can be set on *any* address with no constraints, traps holders.
fn detect_blacklist_trap(source: &str, findings: &mut Vec<SecurityFinding>) {
    let has_blacklist = scan::contains_any(
        source,
        &[
            "mapping(address => bool) public blacklist",
            "_blacklist[",
            "isBlacklisted[",
            "blacklisted[",
        ],
    );
    let blocks_transfer = scan::contains_any(
        source,
        &[
            "require(!blacklist",
            "require(!_blacklist",
            "require(!isBlacklisted",
            "require(!blacklisted",
            "!blacklist[from]",
            "!blacklisted[msg.sender]",
        ],
    );
    // A blacklist combined with a transfer block is a *honeypot* signal only when
    // the setter is unrestricted (anyone, or owner with no event/limit). We flag
    // the presence as Medium (it is a centralization/trap risk) and escalate if
    // the deployer can blacklist post-sale with no timelock.
    if has_blacklist && blocks_transfer {
        let owner_can_blacklist_anyone = scan::contains_any(
            source,
            &[
                "function blacklist(",
                "function setBlacklist(",
                "function addBlacklist(",
            ],
        );
        let severity = if owner_can_blacklist_anyone {
            Severity::High
        } else {
            Severity::Medium
        };
        findings.push(SecurityFinding::new(
            FindingCategory::Honeypot,
            "BLACKLIST_TRANSFER_TRAP",
            severity,
            "Holder transfers can be blocked via blacklist",
            "Transfers revert for blacklisted addresses and the deployer can add \
             addresses to the blacklist at will. After buyers accumulate, the \
             deployer can blacklist them to prevent selling — a common rug/honeypot \
             mechanism.",
            "If a blacklist is genuinely required for compliance, restrict it to \
             timelocked governance, emit events, and make the policy auditable; \
             prefer an allowlist with a public eligibility process.",
            scan::first_line(source, "blacklist")
                .or_else(|| scan::first_line(source, "blacklisted")),
        ));
    }
}

/// An asymmetric tax where the sell tax can be set to ~100% (or is far higher than
/// buy tax) makes selling economically impossible.
fn detect_asymmetric_tax(source: &str, findings: &mut Vec<SecurityFinding>) {
    let has_sell_tax =
        scan::contains_any(source, &["sellTax", "sellFee", "sellTaxBps", "_sellTax"]);
    let has_buy_tax = scan::contains_any(source, &["buyTax", "buyFee", "buyTaxBps", "_buyTax"]);
    let owner_sets_tax = scan::contains_any(
        source,
        &[
            "function setSellTax",
            "function setSellFee",
            "function setTaxes",
            "function setFees",
        ],
    );
    // A configurable sell tax with an owner setter and *no* hard cap means the
    // deployer can raise it to 100%, blocking sells.
    let has_cap = scan::contains_any(
        source,
        &[
            "require(sellTax <=",
            "require(sellFee <=",
            "MAX_SELL_TAX",
            "MAX_TAX",
            "require(newTax <=",
        ],
    );
    if (has_sell_tax || has_buy_tax) && owner_sets_tax && !has_cap {
        findings.push(SecurityFinding::new(
            FindingCategory::Honeypot,
            "UNCAPPED_SELL_TAX",
            Severity::High,
            "Sell tax is owner-settable with no cap",
            "A transfer tax that the owner can raise without an enforced maximum \
             can be set to (near) 100% on sells, confiscating proceeds and making \
             exit impossible — a soft honeypot.",
            "Enforce a hard, immutable cap on any tax (e.g. require(tax <= \
             MAX_TAX_BPS)), and consider renouncing the tax setter after launch.",
            scan::first_line(source, "setSellTax").or_else(|| scan::first_line(source, "setFees")),
        ));
    }
}

/// A `withdraw`/`claim` that emits an event or updates state but performs no
/// actual value transfer is a fake-withdraw honeypot (UI shows success, funds
/// never move).
fn detect_fake_withdraw(source: &str, findings: &mut Vec<SecurityFinding>) {
    let has_withdraw_fn = scan::contains_any(
        source,
        &[
            "function withdraw(",
            "function withdraw()",
            "function claim(",
            "function claim()",
        ],
    );
    if !has_withdraw_fn {
        return;
    }
    // A genuine withdraw moves value. If the contract defines a withdraw/claim but
    // contains no value-moving primitive at all, the "withdraw" cannot pay out.
    let moves_value = scan::contains_any(
        source,
        &[
            ".transfer(",
            ".send(",
            ".call{value:",
            ".call{ value:",
            "safeTransfer(",
            "safeTransferFrom(",
            "_transfer(",
            ".transferFrom(",
        ],
    );
    if !moves_value {
        findings.push(SecurityFinding::new(
            FindingCategory::Honeypot,
            "FAKE_WITHDRAW",
            Severity::Critical,
            "Withdraw/claim performs no value transfer",
            "The contract exposes a withdraw or claim function but contains no \
             value-moving call anywhere. The function can appear to succeed (and \
             may emit an event) while never returning funds — a fake-withdraw \
             honeypot.",
            "Ensure withdraw/claim actually transfers the owed balance via a \
             checked transfer, and add tests asserting recipient balances change.",
            scan::first_line(source, "function withdraw")
                .or_else(|| scan::first_line(source, "function claim")),
        ));
    }
}

/// A transfer override whose body is an unconditional `revert`/`require(false)`
/// (sometimes hidden behind an always-true flag) blocks all secondary transfers.
fn detect_unconditional_transfer_revert(source: &str, findings: &mut Vec<SecurityFinding>) {
    let in_transfer_context = scan::contains_any(
        source,
        &[
            "function transfer(",
            "function transferFrom(",
            "_beforeTokenTransfer",
            "_update(",
        ],
    );
    let unconditional_block = scan::contains_any(
        source,
        &[
            "require(false",
            "require(tradingEnabled)", // gated on a flag the owner may never set
            "require(!paused() || msg.sender == owner",
        ],
    );
    // `require(tradingEnabled)` alone is common & legitimate IF there is a public
    // enableTrading; flag the trap variant where trading can never be enabled by
    // anyone but the owner and there is a hard revert(false) path.
    let has_hard_revert = scan::contains(source, "require(false");
    if in_transfer_context && unconditional_block && has_hard_revert {
        findings.push(SecurityFinding::new(
            FindingCategory::Honeypot,
            "TRANSFER_ALWAYS_REVERTS",
            Severity::Critical,
            "Transfer path contains an unconditional revert",
            "The transfer/transferFrom path contains a require(false) or equivalent \
             unconditional revert, so holders other than privileged addresses can \
             never move tokens — an outright honeypot.",
            "Remove the unconditional revert; if a launch gate is needed, expose a \
             one-way, publicly-verifiable enableTrading switch.",
            scan::first_line(source, "require(false"),
        ));
    }
}
