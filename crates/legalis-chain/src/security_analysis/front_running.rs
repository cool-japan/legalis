//! Front-running susceptibility detector.
//!
//! Part of the `security_analysis` module. Front-running exploits the public
//! mempool: an attacker observes a profitable pending transaction and pays more
//! gas to have their own copy mined first. This detector flags order-dependent
//! flows — first-caller rewards, plaintext bid/answer submissions, approve-race
//! patterns — that lack a commit-reveal or equivalent defence, and suggests the
//! appropriate mitigation for each.

use super::scan;
use super::{FindingCategory, SecurityFinding};
use crate::types::Severity;
use crate::types_19::GeneratedContract;

/// Detects front-running-susceptible flows in `contract`'s source.
pub fn detect_front_running_risks(contract: &GeneratedContract) -> Vec<SecurityFinding> {
    let source = &contract.source;
    let mut findings = Vec::new();
    if !scan::is_solidity_like(source) {
        return findings;
    }

    let has_commit_reveal = uses_commit_reveal(source);

    detect_first_caller_reward(source, has_commit_reveal, &mut findings);
    detect_plaintext_secret(source, has_commit_reveal, &mut findings);
    detect_open_bid(source, has_commit_reveal, &mut findings);
    detect_approve_race(source, &mut findings);

    findings
}

/// Heuristic: does the contract already implement a commit-reveal scheme?
fn uses_commit_reveal(source: &str) -> bool {
    scan::contains_any(
        source,
        &[
            "commit(",
            "reveal(",
            "commitment",
            "commitHash",
            "keccak256(abi.encodePacked(msg.sender, secret",
            "function commit",
        ],
    )
}

/// A reward paid to the *first* caller who supplies a winning value (puzzle
/// solutions, claim-first airdrops) is a textbook front-running target.
fn detect_first_caller_reward(
    source: &str,
    has_commit_reveal: bool,
    findings: &mut Vec<SecurityFinding>,
) {
    let first_caller_reward = scan::contains_any(
        source,
        &[
            "require(!claimed",
            "require(answer == ",
            "require(_answer == ",
            "require(solution == ",
            "function solve(",
            "function claimReward(",
        ],
    );
    let pays_out = scan::contains_any(
        source,
        &[
            ".transfer(",
            ".call{value:",
            ".call{ value:",
            "_mint(",
            "safeTransfer(",
        ],
    );
    if first_caller_reward && pays_out && !has_commit_reveal {
        findings.push(SecurityFinding::new(
            FindingCategory::FrontRunning,
            "FIRST_CALLER_REWARD",
            Severity::High,
            "Reward paid to first caller of a submitted value",
            "A payout granted to whoever first submits a winning value (a puzzle \
             answer, a claim, a solution) can be front-run: an attacker watching \
             the mempool copies the winning input and pays higher gas to be mined \
             first, stealing the reward from the honest submitter.",
            "Use a commit-reveal scheme: callers first submit \
             keccak256(answer, salt, msg.sender), then reveal in a later \
             transaction so the plaintext is never in the mempool while \
             unclaimed. Optionally bind the reward to the committer's address.",
            scan::first_line(source, "function solve(")
                .or_else(|| scan::first_line(source, "function claimReward(")),
        ));
    }
}

/// A function that accepts a secret/password in plaintext as calldata exposes it
/// to every mempool observer before it is mined.
fn detect_plaintext_secret(
    source: &str,
    has_commit_reveal: bool,
    findings: &mut Vec<SecurityFinding>,
) {
    let takes_plaintext_secret = scan::contains_any(
        source,
        &[
            "string calldata password",
            "string memory password",
            "bytes32 secret",
            "string calldata secret",
            "uint256 secretNumber",
        ],
    );
    let compares_secret = scan::contains_any(
        source,
        &[
            "keccak256(abi.encodePacked(password)) ==",
            "== passwordHash",
            "== secretHash",
            "require(keccak256(bytes(password))",
        ],
    );
    if takes_plaintext_secret && compares_secret && !has_commit_reveal {
        findings.push(SecurityFinding::new(
            FindingCategory::FrontRunning,
            "PLAINTEXT_SECRET",
            Severity::High,
            "Secret/password submitted in plaintext calldata",
            "Comparing a plaintext secret submitted as calldata against a stored \
             hash leaks the secret to the entire mempool: an attacker reads the \
             pending transaction, resubmits the same secret with higher gas, and \
             claims the gated reward first.",
            "Never submit the plaintext while it still unlocks value. Use \
             commit-reveal (commit keccak256(secret, salt, msg.sender) first, \
             reveal later) so the secret is only disclosed once it can no longer be \
             stolen.",
            scan::first_line(source, "password").or_else(|| scan::first_line(source, "secret")),
        ));
    }
}

/// An auction that records bids in plaintext (and especially one whose highest bid
/// is publicly readable as it is placed) invites bid-sniping front-runs.
fn detect_open_bid(source: &str, has_commit_reveal: bool, findings: &mut Vec<SecurityFinding>) {
    let is_auction = scan::contains_any(
        source,
        &[
            "function bid(",
            "highestBid",
            "function placeBid(",
            "currentBid",
        ],
    );
    if is_auction && !has_commit_reveal {
        findings.push(SecurityFinding::new(
            FindingCategory::FrontRunning,
            "OPEN_BID_AUCTION",
            Severity::Medium,
            "Open-bid auction without sealed bids",
            "An auction that accepts and exposes plaintext bids is front-runnable: \
             a watcher can observe an incoming high bid and submit a marginally \
             higher one with greater gas, or snipe at the last moment. Bidders \
             cannot bid honestly without revealing their valuation.",
            "Use a sealed-bid (commit-reveal) auction: bidders commit \
             keccak256(amount, salt) during the bidding phase and reveal during a \
             separate phase; refund non-revealed deposits. Consider anti-snipe \
             auction extensions.",
            scan::first_line(source, "function bid(")
                .or_else(|| scan::first_line(source, "highestBid")),
        ));
    }
}

/// The classic ERC-20 `approve` race: changing a non-zero allowance to another
/// non-zero value can be front-run to spend both. Flagged when `approve` is
/// overridden without `increaseAllowance`/safe handling.
fn detect_approve_race(source: &str, findings: &mut Vec<SecurityFinding>) {
    // Only relevant if the contract *defines its own* approve (overriding OZ),
    // since OZ's standard approve carries the documented caveat and ecosystem
    // tooling uses increase/decreaseAllowance.
    let defines_approve = scan::contains(source, "function approve(")
        && scan::contains_any(source, &["allowance[", "_allowances[", "_approve("]);
    let has_safe_change = scan::contains_any(
        source,
        &[
            "increaseAllowance",
            "decreaseAllowance",
            "require(amount == 0 || allowance",
            "require(_allowances[msg.sender][spender] == 0",
        ],
    );
    if defines_approve && !has_safe_change {
        findings.push(SecurityFinding::new(
            FindingCategory::FrontRunning,
            "APPROVE_RACE",
            Severity::Low,
            "Custom approve() exposed to the allowance double-spend race",
            "The ERC-20 approve race: when an owner changes an existing non-zero \
             allowance to a new non-zero value, a malicious spender can front-run \
             the change to spend the old allowance and then the new one. A custom \
             approve without mitigations re-exposes this.",
            "Provide increaseAllowance/decreaseAllowance, or require the allowance \
             to be set to zero before a new non-zero value; document the caveat for \
             integrators.",
            scan::first_line(source, "function approve("),
        ));
    }
}
