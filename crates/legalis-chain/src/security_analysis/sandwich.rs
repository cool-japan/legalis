//! Sandwich-attack (MEV) susceptibility detector.
//!
//! Part of the `security_analysis` module. A sandwich attack front-runs a victim
//! swap with a buy, lets the victim's trade move the price, then back-runs with a
//! sell — profiting from the slippage the victim absorbs. This detector flags
//! swap/liquidity flows that lack the slippage bounds and deadlines that make
//! sandwiching unprofitable, and each finding carries a concrete mitigation.

use super::scan;
use super::{FindingCategory, SecurityFinding};
use crate::types::Severity;
use crate::types_19::GeneratedContract;

/// Detects sandwich/MEV-susceptible patterns in `contract`'s source.
pub fn detect_sandwich_risks(contract: &GeneratedContract) -> Vec<SecurityFinding> {
    let source = &contract.source;
    let mut findings = Vec::new();
    if !scan::is_solidity_like(source) {
        return findings;
    }

    let does_swap = is_swap_like(source);
    if !does_swap {
        return findings;
    }

    detect_missing_min_out(source, &mut findings);
    detect_missing_deadline(source, &mut findings);
    detect_spot_price_oracle(source, &mut findings);

    findings
}

/// Heuristic: does the contract perform AMM-style swaps, liquidity ops, or price
/// off the pool reserves? Any of these makes it exposed to MEV/sandwich risk.
fn is_swap_like(source: &str) -> bool {
    scan::contains_any(
        source,
        &[
            "function swap(",
            "swapExactTokensForTokens",
            "swapExactETHForTokens",
            "swapTokensForExactTokens",
            "getAmountOut",
            "getAmountsOut",
            "addLiquidity",
            "function buy(",
            "function sell(",
            // Reserve-based pricing is itself a sandwich/oracle-manipulation
            // surface even without an explicit swap function.
            "getReserves()",
            "reserve0",
            "reserve1",
        ],
    )
}

/// A swap that does not accept (and enforce) a caller-supplied minimum output is
/// fully sandwichable — the attacker can move the price arbitrarily against the
/// victim.
fn detect_missing_min_out(source: &str, findings: &mut Vec<SecurityFinding>) {
    let has_min_out_param = scan::contains_any(
        source,
        &[
            "amountOutMin",
            "minAmountOut",
            "minOut",
            "minReturn",
            "amountOutMinimum",
        ],
    );
    let enforces_min_out = scan::contains_any(
        source,
        &[
            "require(amountOut >= amountOutMin",
            "require(out >= minOut",
            "require(received >= minAmountOut",
            "require(amountOut >= minAmountOut",
            ">= amountOutMin",
            ">= minAmountOut",
            ">= minOut",
            ">= minReturn",
        ],
    );
    if !has_min_out_param || !enforces_min_out {
        findings.push(SecurityFinding::new(
            FindingCategory::SandwichAttack,
            "MISSING_SLIPPAGE_BOUND",
            Severity::High,
            "Swap lacks an enforced minimum-output (slippage) bound",
            "A swap that computes its output from the current pool reserves without \
             requiring a caller-supplied minimum can be sandwiched: an attacker \
             front-runs to skew the price, the victim trades at the worsened rate, \
             and the attacker back-runs for profit. The victim has no protection \
             against arbitrary slippage.",
            "Add an amountOutMin parameter and require(amountOut >= amountOutMin); \
             let callers compute it from an off-chain quote with a tolerance. \
             Combine with a deadline and, ideally, route sensitive swaps through a \
             private mempool / MEV-protected RPC.",
            scan::first_line(source, "function swap(")
                .or_else(|| scan::first_line(source, "getAmountOut")),
        ));
    }
}

/// A swap without a `deadline` lets a validator hold the transaction and execute
/// it later at a more favourable (for them) price.
fn detect_missing_deadline(source: &str, findings: &mut Vec<SecurityFinding>) {
    let has_deadline = scan::contains_any(
        source,
        &[
            "deadline",
            "require(block.timestamp <= deadline",
            "expiry",
            "validUntil",
        ],
    );
    if !has_deadline {
        findings.push(SecurityFinding::new(
            FindingCategory::SandwichAttack,
            "MISSING_DEADLINE",
            Severity::Medium,
            "Swap lacks a transaction deadline",
            "Without a deadline a pending swap can be held in the mempool and \
             executed by a validator at a later, less favourable price (a delayed \
             sandwich / time-bandit variant). The user cannot bound how stale their \
             trade may be.",
            "Add a deadline parameter and require(block.timestamp <= deadline) so \
             stale transactions revert instead of executing at an attacker-chosen \
             time.",
            scan::first_line(source, "function swap("),
        ));
    }
}

/// Pricing a swap from the instantaneous reserve ratio (spot price) rather than a
/// TWAP makes a single-block sandwich trivially profitable.
fn detect_spot_price_oracle(source: &str, findings: &mut Vec<SecurityFinding>) {
    let uses_spot = scan::contains_any(
        source,
        &[
            "getReserves()",
            "reserve0",
            "reserve1",
            "balanceOf(address(this)) * ",
            "* reserveOut) / reserveIn",
        ],
    );
    let uses_twap = scan::contains_any(
        source,
        &[
            "TWAP",
            "twap",
            "consult(",
            "OracleLibrary",
            "cumulativePrice",
            "observe(",
        ],
    );
    if uses_spot && !uses_twap {
        findings.push(SecurityFinding::new(
            FindingCategory::SandwichAttack,
            "SPOT_PRICE_PRICING",
            Severity::High,
            "Pricing derived from spot reserves (no TWAP)",
            "Computing price or output from instantaneous pool reserves lets an \
             attacker move the reserves within the same block (flash-loan-funded), \
             execute the victim/contract action at the manipulated price, then \
             revert the move — the core of sandwich and oracle-manipulation \
             attacks.",
            "Price against a time-weighted average (TWAP) over several blocks, or a \
             robust external oracle; never make value-bearing decisions from a \
             single-block spot reserve ratio.",
            scan::first_line(source, "getReserves()")
                .or_else(|| scan::first_line(source, "reserve0")),
        ));
    }
}
