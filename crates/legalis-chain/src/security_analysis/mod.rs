//! Advanced smart-contract security analyzers (pure-Rust static detectors).
//!
//! This module adds a second generation of security tooling on top of the
//! existing [`crate::SecurityAnalyzer`]. Where the latter flags a handful of
//! generic EVM issues, the detectors here are *behavioural*: each targets a class
//! of economic/operational attack and returns structured [`SecurityFinding`]s
//! carrying a [`FindingCategory`], a [`crate::Severity`], a human-readable
//! explanation and a concrete remediation.
//!
//! The five detector families implement the "Advanced Security (v0.4.9)" roadmap:
//!
//! * **Runtime exploit detection** ([`detect_runtime_exploits`]) — known
//!   exploitable runtime patterns (delegatecall to user data, `tx.origin` auth,
//!   `selfdestruct`, unchecked low-level calls, `block.timestamp`/blockhash
//!   randomness, arbitrary external-call sinks).
//! * **Honeypot detection** ([`detect_honeypots`]) — anti-patterns that let funds
//!   *in* but block them coming *out* (asymmetric buy/sell gates, hidden
//!   owner-only transfer locks, balance/blacklist traps, fake withdraw).
//! * **Rug-pull prevention** ([`detect_rug_pull_risks`]) — owner-drain indicators
//!   (unrestricted/uncapped mint, owner withdraw of all funds, mutable fees with
//!   no ceiling, ownership not renounceable/timelocked, upgradeable proxy with no
//!   timelock).
//! * **Sandwich-attack mitigation** ([`detect_sandwich_risks`]) — MEV-susceptible
//!   swap/liquidity flows lacking slippage bounds or deadlines, plus suggested
//!   mitigations.
//! * **Front-running protection** ([`detect_front_running_risks`]) — order-
//!   dependent state transitions (claims, auctions, first-caller rewards) lacking
//!   a commit-reveal or private-mempool defence, plus suggested mitigations.
//!
//! Each detector consumes a [`crate::GeneratedContract`] and inspects its emitted
//! source. They are deliberately conservative pattern matchers — they never
//! mutate the contract and produce no false *positives* on the crate's own
//! hardened generators (which is asserted in the tests). The single entry point
//! [`analyze_security`] runs every detector and aggregates the results into a
//! [`SecurityScan`].

mod front_running;
mod honeypot;
mod rug_pull;
mod runtime_exploit;
mod sandwich;

#[cfg(test)]
mod tests;

pub use front_running::detect_front_running_risks;
pub use honeypot::detect_honeypots;
pub use rug_pull::detect_rug_pull_risks;
pub use runtime_exploit::detect_runtime_exploits;
pub use sandwich::detect_sandwich_risks;

use crate::tokenization::is_evm_target;
use crate::types::Severity;
use crate::types_19::GeneratedContract;

/// The class of attack a [`SecurityFinding`] belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FindingCategory {
    /// A runtime-exploitable code pattern (delegatecall, tx.origin, etc.).
    RuntimeExploit,
    /// A honeypot anti-pattern (funds enter but cannot leave).
    Honeypot,
    /// A rug-pull risk indicator (owner can drain or dilute holders).
    RugPull,
    /// A sandwich-attack (MEV) susceptibility.
    SandwichAttack,
    /// A front-running susceptibility.
    FrontRunning,
}

impl FindingCategory {
    /// A stable, human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            FindingCategory::RuntimeExploit => "Runtime Exploit",
            FindingCategory::Honeypot => "Honeypot",
            FindingCategory::RugPull => "Rug Pull",
            FindingCategory::SandwichAttack => "Sandwich Attack",
            FindingCategory::FrontRunning => "Front-Running",
        }
    }
}

/// A single structured security finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityFinding {
    /// The attack class this finding belongs to.
    pub category: FindingCategory,
    /// Stable short identifier for the specific rule (e.g. `"TX_ORIGIN_AUTH"`).
    pub rule_id: String,
    /// Severity of the finding.
    pub severity: Severity,
    /// One-line title.
    pub title: String,
    /// Detailed explanation of *why* this is dangerous.
    pub explanation: String,
    /// Concrete, actionable remediation / mitigation advice.
    pub remediation: String,
    /// First source line the pattern was matched on, if locatable (1-based).
    pub line: Option<usize>,
}

impl SecurityFinding {
    /// Convenience constructor used by the detectors.
    pub(crate) fn new(
        category: FindingCategory,
        rule_id: &str,
        severity: Severity,
        title: &str,
        explanation: &str,
        remediation: &str,
        line: Option<usize>,
    ) -> Self {
        Self {
            category,
            rule_id: rule_id.to_string(),
            severity,
            title: title.to_string(),
            explanation: explanation.to_string(),
            remediation: remediation.to_string(),
            line,
        }
    }
}

/// Aggregated result of running every detector over one contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityScan {
    /// Name of the analyzed contract.
    pub contract_name: String,
    /// All findings, ordered by descending severity then by category.
    pub findings: Vec<SecurityFinding>,
    /// `0..=100` risk score; `100` is clean, lower is riskier.
    pub risk_score: u8,
}

impl SecurityScan {
    /// Returns whether any finding of `severity` (or worse) was reported.
    pub fn has_at_least(&self, severity: Severity) -> bool {
        let threshold = severity_rank(&severity);
        self.findings
            .iter()
            .any(|finding| severity_rank(&finding.severity) >= threshold)
    }

    /// Returns the findings belonging to `category`.
    pub fn by_category(&self, category: FindingCategory) -> Vec<&SecurityFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.category == category)
            .collect()
    }

    /// Returns whether the scan found no issues at all.
    pub fn is_clean(&self) -> bool {
        self.findings.is_empty()
    }
}

/// Runs every advanced detector over `contract` and aggregates the results.
///
/// Non-EVM contracts are returned with an empty, perfect-score scan because the
/// source-pattern detectors only understand Solidity. The findings are sorted by
/// descending severity (Critical first), ties broken by category label then rule
/// id, so the output ordering is deterministic.
pub fn analyze_security(contract: &GeneratedContract) -> SecurityScan {
    let mut findings: Vec<SecurityFinding> = Vec::new();
    if is_evm_target(contract.platform) {
        findings.extend(detect_runtime_exploits(contract));
        findings.extend(detect_honeypots(contract));
        findings.extend(detect_rug_pull_risks(contract));
        findings.extend(detect_sandwich_risks(contract));
        findings.extend(detect_front_running_risks(contract));
    }

    findings.sort_by(|left, right| {
        severity_rank(&right.severity)
            .cmp(&severity_rank(&left.severity))
            .then_with(|| left.category.label().cmp(right.category.label()))
            .then_with(|| left.rule_id.cmp(&right.rule_id))
    });

    let risk_score = compute_risk_score(&findings);
    SecurityScan {
        contract_name: contract.name.clone(),
        findings,
        risk_score,
    }
}

/// Ranks a severity so higher numbers are worse (used for sorting/aggregation).
pub(crate) fn severity_rank(severity: &Severity) -> u8 {
    match severity {
        Severity::Critical => 4,
        Severity::High => 3,
        Severity::Medium => 2,
        Severity::Low => 1,
    }
}

/// Derives a `0..=100` risk score from the findings.
///
/// Each finding deducts a severity-weighted penalty from a perfect 100, saturating
/// at 0. The weights (Critical 40, High 25, Medium 12, Low 5) are chosen so a
/// single critical issue alone drops the score below the conventional 60 "fail"
/// threshold.
fn compute_risk_score(findings: &[SecurityFinding]) -> u8 {
    let mut deduction: u32 = 0;
    for finding in findings {
        deduction += match finding.severity {
            Severity::Critical => 40,
            Severity::High => 25,
            Severity::Medium => 12,
            Severity::Low => 5,
        };
    }
    let score = 100u32.saturating_sub(deduction);
    u8::try_from(score).unwrap_or(0)
}

/// Shared scanning helpers reused by the individual detector modules.
pub(crate) mod scan {
    /// Returns whether `source` contains `needle` anywhere.
    pub(crate) fn contains(source: &str, needle: &str) -> bool {
        source.contains(needle)
    }

    /// Returns whether `source` contains *any* of `needles`.
    pub(crate) fn contains_any(source: &str, needles: &[&str]) -> bool {
        needles.iter().any(|needle| source.contains(needle))
    }

    /// Returns the 1-based line number of the first occurrence of `needle`.
    pub(crate) fn first_line(source: &str, needle: &str) -> Option<usize> {
        let byte_offset = source.find(needle)?;
        // Count newlines preceding the match.
        Some(
            source[..byte_offset]
                .bytes()
                .filter(|b| *b == b'\n')
                .count()
                + 1,
        )
    }

    /// Returns whether `source` looks like Solidity (has a pragma/contract).
    pub(crate) fn is_solidity_like(source: &str) -> bool {
        source.contains("pragma solidity") || source.contains("contract ")
    }
}
