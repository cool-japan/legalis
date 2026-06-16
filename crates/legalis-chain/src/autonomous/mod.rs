//! Autonomous contract-management generators.
//!
//! This module extends [`ContractGenerator`] with production-grade generators for
//! contracts that *manage themselves* at runtime — the "Autonomous Management"
//! toolkit. Unlike the domain-specific compliance/environmental monitors elsewhere
//! in the crate (which watch *legal* state) and unlike the guardian-triggered
//! [`crate::EmergencyPauseConfig`] (which needs a human to pause *and* unpause),
//! every generator here closes an *operational* feedback loop with no privileged
//! operator in the hot path:
//!
//! * **Self-healing** — a finite state machine that autonomously detects a health
//!   invariant breach, restores the last-known-good checkpoint, and *auto-resumes*
//!   once the metric recovers and a cool-down elapses ([`SelfHealingConfig`]).
//! * **Automatic optimization** — an on-chain step/feedback controller that nudges
//!   a tunable parameter toward a metric set-point within bounds ([`AutoOptimizerConfig`]).
//! * **Resource management** — a token-bucket rate limiter with per-epoch global
//!   budgets and optional per-caller accounting ([`ResourceManagerConfig`]).
//! * **Performance monitoring** — per-operation on-chain metrics (count, cumulative
//!   gas, EMA gas, min/max) with an optional health score ([`PerformanceMonitorConfig`]).
//! * **Cost optimization** — a batched multicall with storage-refund harvesting and
//!   a base-fee guard that defers non-urgent work when gas is expensive
//!   ([`CostOptimizerConfig`]).
//!
//! As with the [`crate::tokenization`] and [`crate::evolution`] modules, all of the
//! control math lives in pure Rust ([`next_health_state`], [`adjust_parameter`],
//! [`token_bucket_available`], [`ema_update`], [`health_score`]) so that it is
//! validated *before* any source is emitted and is independently unit-testable for
//! on-chain/off-chain parity. Every generator targets the EVM family (Solidity);
//! non-EVM targets return a [`ChainError::GenerationError`].

mod auto_optimizer;
mod cost_optimizer;
mod performance_monitor;
mod resource_manager;
mod self_healing;

#[cfg(test)]
mod tests;

use std::cmp::Ordering;

use super::contractgenerator_type::ContractGenerator;
use super::functions::ChainResult;
use super::types_19::{ChainError, GeneratedContract, Jurisdiction};
use crate::BASIS_POINTS_DENOMINATOR;
use crate::evolution::sanitize_identifier;
use crate::tokenization::is_evm_target;

/// Maximum number of invariants, operations or quotas accepted in one contract.
/// Bounds the size of generated source and the gas of any on-chain loop.
pub const MAX_AUTONOMOUS_ENTRIES: usize = 64;

/// Operational health state of a self-healing subsystem.
///
/// The numeric values match the `uint8` encoding emitted on-chain so that the
/// off-chain [`next_health_state`] state machine agrees with the generated
/// contract bit-for-bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthState {
    /// Operating normally; guarded operations are permitted.
    Healthy,
    /// A breach was detected; the checkpoint was restored and guarded operations
    /// are blocked until the metric recovers.
    Degraded,
    /// The metric is back in band; the contract is serving out the recovery
    /// cool-down before it auto-resumes.
    Recovering,
}

impl HealthState {
    /// The `uint8` encoding used on-chain.
    pub fn as_u8(self) -> u8 {
        match self {
            HealthState::Healthy => 0,
            HealthState::Degraded => 1,
            HealthState::Recovering => 2,
        }
    }

    /// Whether guarded operations are permitted in this state.
    pub fn is_operational(self) -> bool {
        matches!(self, HealthState::Healthy)
    }
}

/// Direction of the relationship between a controlled parameter and its driving
/// metric, used by [`adjust_parameter`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlSense {
    /// Raise the parameter when the observed metric is *above* its target
    /// (positive correlation desired — e.g. raise a fee as demand rises).
    Direct,
    /// Lower the parameter when the observed metric is *above* its target
    /// (negative correlation desired — e.g. lower a reward as utilisation rises).
    Inverse,
}

impl ControlSense {
    /// Human-readable label used in generated NatSpec documentation.
    pub(crate) fn label(self) -> &'static str {
        match self {
            ControlSense::Direct => "direct (param tracks metric)",
            ControlSense::Inverse => "inverse (param opposes metric)",
        }
    }
}

/// A single named health invariant for a self-healing contract.
///
/// The metric reported for `key` is healthy while it lies within the inclusive
/// band `[min_value, max_value]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthInvariant {
    /// Stable key hashed on-chain (`keccak256(bytes(key))`).
    pub key: String,
    /// Human-readable description embedded in NatSpec.
    pub description: String,
    /// Inclusive lower bound of the healthy band.
    pub min_value: u128,
    /// Inclusive upper bound of the healthy band.
    pub max_value: u128,
}

/// Self-healing controller configuration.
#[derive(Debug, Clone)]
pub struct SelfHealingConfig {
    /// Contract name.
    pub name: String,
    /// Health invariants the contract monitors and heals against.
    pub invariants: Vec<HealthInvariant>,
    /// Whether to generate a checkpoint/restore of named tunable parameters that
    /// is rolled back automatically on a breach.
    pub checkpoint_enabled: bool,
    /// Seconds the metric must stay in band before the contract auto-resumes.
    pub recover_after_seconds: u64,
    /// Wei paid to whoever reports the health update that triggers a state change
    /// (a keeper incentive). `0` disables the reward.
    pub keeper_reward_wei: u64,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
}

/// Automatic-optimization (feedback controller) configuration.
#[derive(Debug, Clone)]
pub struct AutoOptimizerConfig {
    /// Contract name.
    pub name: String,
    /// Label of the tunable parameter (e.g. `"feeBps"`).
    pub parameter_label: String,
    /// Initial parameter value (must lie within `[min_value, max_value]`).
    pub initial_value: u128,
    /// Inclusive minimum the parameter may be tuned to.
    pub min_value: u128,
    /// Inclusive maximum the parameter may be tuned to.
    pub max_value: u128,
    /// Label of the driving metric (e.g. `"utilizationBps"`).
    pub metric_label: String,
    /// Set-point the controller steers the metric toward.
    pub target_value: u128,
    /// Maximum change applied to the parameter per adjustment.
    pub step_value: u128,
    /// Relationship between the parameter and the metric.
    pub sense: ControlSense,
    /// Minimum seconds between successive adjustments.
    pub cooldown_seconds: u64,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
}

/// Resource-management (rate-limit + budget) configuration.
#[derive(Debug, Clone)]
pub struct ResourceManagerConfig {
    /// Contract name.
    pub name: String,
    /// Maximum tokens a bucket can hold (burst capacity).
    pub bucket_capacity: u64,
    /// Tokens added to a bucket per second.
    pub refill_per_second: u64,
    /// Tokens consumed by one guarded operation.
    pub cost_per_op: u64,
    /// Length of a budget epoch in seconds.
    pub epoch_seconds: u64,
    /// Maximum tokens consumable across *all* callers within one epoch.
    pub epoch_budget: u64,
    /// Whether buckets are tracked per caller (`true`) or globally (`false`).
    pub per_caller: bool,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
}

/// Performance-monitor configuration.
#[derive(Debug, Clone)]
pub struct PerformanceMonitorConfig {
    /// Contract name.
    pub name: String,
    /// Named operations whose metrics are tracked.
    pub operations: Vec<String>,
    /// EMA smoothing factor in basis points (`1..=10000`); higher reacts faster.
    pub ema_alpha_bps: u32,
    /// Whether to emit a metrics event on every sample for off-chain indexing.
    pub emit_events: bool,
    /// Whether to expose a `healthScore` view derived from the EMA vs the budget.
    pub health_score: bool,
    /// Per-operation gas budget used to compute the health score.
    pub gas_budget_per_op: u64,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
}

/// Cost-optimization configuration.
#[derive(Debug, Clone)]
pub struct CostOptimizerConfig {
    /// Contract name.
    pub name: String,
    /// Maximum number of calls accepted in one batch.
    pub max_batch_size: u32,
    /// Base transaction gas amortised across a batch (typically `21000`).
    pub base_tx_gas: u64,
    /// Base fee (wei) above which non-urgent operations are deferred (`0` disables
    /// the guard).
    pub basefee_ceiling_wei: u64,
    /// Whether to harvest storage refunds by clearing slots after use.
    pub enable_refunds: bool,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
}

/// Returns the generated `bytes32` constant name for a health-invariant key.
pub fn invariant_constant_name(key: &str) -> String {
    format!("INV_{}", sanitize_identifier(key))
}

/// Returns the generated `bytes32` constant name for a monitored operation.
pub fn operation_constant_name(op: &str) -> String {
    format!("OP_{}", sanitize_identifier(op))
}

/// Clamps `value` into `[min, max]` without panicking.
///
/// Returns `value` unchanged when `min > max` (an inverted band), so callers that
/// have not validated their bounds still get a total function rather than the
/// panic that [`u128::clamp`] would raise.
pub fn clamp_value(value: u128, min: u128, max: u128) -> u128 {
    if min > max {
        return value;
    }
    value.clamp(min, max)
}

/// Returns whether `value` lies within the inclusive healthy band `[min, max]`.
///
/// Mirrors the on-chain `value >= min && value <= max` check.
pub fn classify_in_band(value: u128, min: u128, max: u128) -> bool {
    value >= min && value <= max
}

/// Computes the next [`HealthState`] of a self-healing subsystem.
///
/// `elapsed_in_state_seconds` is the time since the subsystem last *entered* its
/// current state, and `recover_after_seconds` is the mandatory in-band cool-down
/// before the contract auto-resumes. The transition table is:
///
/// | from         | in band            | out of band |
/// |--------------|--------------------|-------------|
/// | `Healthy`    | `Healthy`          | `Degraded`  |
/// | `Degraded`   | `Recovering`       | `Degraded`  |
/// | `Recovering` | `Healthy`¹ / hold  | `Degraded`  |
///
/// ¹ only once `elapsed_in_state_seconds >= recover_after_seconds`.
///
/// This is the exact state machine the generated `reportHealth` function executes,
/// guaranteeing on-chain/off-chain parity.
pub fn next_health_state(
    current: HealthState,
    in_band: bool,
    elapsed_in_state_seconds: u64,
    recover_after_seconds: u64,
) -> HealthState {
    match (current, in_band) {
        (HealthState::Healthy, true) => HealthState::Healthy,
        (HealthState::Healthy, false) => HealthState::Degraded,
        (HealthState::Degraded, true) => HealthState::Recovering,
        (HealthState::Degraded, false) => HealthState::Degraded,
        (HealthState::Recovering, false) => HealthState::Degraded,
        (HealthState::Recovering, true) => {
            if elapsed_in_state_seconds >= recover_after_seconds {
                HealthState::Healthy
            } else {
                HealthState::Recovering
            }
        }
    }
}

/// Steps a controlled parameter one increment toward its metric set-point.
///
/// When the observed metric equals the target the parameter holds; otherwise it
/// moves by at most `step` (saturating) in the direction dictated by `sense`, and
/// the result is clamped into `[min, max]` via [`clamp_value`]. Mirrors the
/// generated `_adjust` routine for on-chain/off-chain parity.
pub fn adjust_parameter(
    current: u128,
    observed: u128,
    target: u128,
    step: u128,
    min: u128,
    max: u128,
    sense: ControlSense,
) -> u128 {
    let next = match (sense, observed.cmp(&target)) {
        (_, Ordering::Equal) => current,
        (ControlSense::Direct, Ordering::Greater) | (ControlSense::Inverse, Ordering::Less) => {
            current.saturating_add(step)
        }
        (ControlSense::Direct, Ordering::Less) | (ControlSense::Inverse, Ordering::Greater) => {
            current.saturating_sub(step)
        }
    };
    clamp_value(next, min, max)
}

/// Computes the tokens available in a leaky/token bucket at time `now`.
///
/// `available = min(capacity, stored + (now - last_refill) * refill_per_second)`,
/// with every step saturating so the result never overflows. Mirrors the
/// generated `_available` view.
pub fn token_bucket_available(
    stored: u64,
    last_refill: u64,
    now: u64,
    capacity: u64,
    refill_per_second: u64,
) -> u64 {
    let elapsed = now.saturating_sub(last_refill);
    let refill = elapsed.saturating_mul(refill_per_second);
    stored.saturating_add(refill).min(capacity)
}

/// Returns whether `available` tokens can cover `cost`.
pub fn can_consume(available: u64, cost: u64) -> bool {
    available >= cost
}

/// Returns the zero-based epoch index containing `now`.
///
/// An `epoch_seconds` of `0` collapses every timestamp into epoch `0`. Mirrors
/// the generated `currentEpoch` view.
pub fn epoch_index(now: u64, start: u64, epoch_seconds: u64) -> u64 {
    if epoch_seconds == 0 {
        return 0;
    }
    now.saturating_sub(start) / epoch_seconds
}

/// Updates an exponential moving average.
///
/// `next = (alpha * sample + (10000 - alpha) * prev) / 10000`, where `alpha` is
/// `alpha_bps` clamped to `[0, 10000]`. All multiplications saturate. Mirrors the
/// generated `_ema` routine for on-chain/off-chain parity.
pub fn ema_update(prev: u128, sample: u128, alpha_bps: u32) -> u128 {
    let denominator = u128::from(BASIS_POINTS_DENOMINATOR);
    let alpha = u128::from(alpha_bps).min(denominator);
    let inverse = denominator - alpha;
    let weighted = alpha
        .saturating_mul(sample)
        .saturating_add(inverse.saturating_mul(prev));
    weighted / denominator
}

/// Computes a `0..=10000` health score from an observed EMA gas figure and a
/// budget.
///
/// A zero EMA (no load) or an EMA within budget scores a perfect `10000`;
/// otherwise the score degrades as `budget * 10000 / ema`. Mirrors the generated
/// `healthScore` view.
pub fn health_score(ema_gas: u128, budget: u128) -> u32 {
    if ema_gas == 0 || budget >= ema_gas {
        return BASIS_POINTS_DENOMINATOR;
    }
    let denominator = u128::from(BASIS_POINTS_DENOMINATOR);
    let score = budget.saturating_mul(denominator) / ema_gas;
    u32::try_from(score).unwrap_or(0)
}

/// Estimates the gas saved by amortising the base transaction cost across a batch
/// of `num_calls` operations instead of sending them individually.
///
/// `savings = (num_calls - 1) * base_tx_gas`; a batch of zero or one saves
/// nothing. Mirrors the documented estimate emitted in NatSpec.
pub fn batch_savings(num_calls: u32, base_tx_gas: u64) -> u64 {
    u64::from(num_calls.saturating_sub(1)).saturating_mul(base_tx_gas)
}

/// Returns whether a non-urgent operation should be deferred at the current base
/// fee.
///
/// A `ceiling` of `0` disables the guard. Mirrors the generated `_shouldDefer`
/// check.
pub fn should_defer(basefee: u64, ceiling: u64) -> bool {
    ceiling != 0 && basefee > ceiling
}

/// Validates a [`SelfHealingConfig`] for coherence before codegen.
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] if the name is empty, if the invariant
/// set is empty / over the entry limit / has empty or duplicate keys, or if any
/// invariant band is inverted (`min > max`).
pub fn validate_self_healing(config: &SelfHealingConfig) -> ChainResult<()> {
    if config.name.trim().is_empty() {
        return Err(ChainError::GenerationError(
            "SelfHealingConfig.name must not be empty".to_string(),
        ));
    }
    if config.invariants.is_empty() {
        return Err(ChainError::GenerationError(
            "SelfHealingConfig must declare at least one invariant".to_string(),
        ));
    }
    if config.invariants.len() > MAX_AUTONOMOUS_ENTRIES {
        return Err(ChainError::GenerationError(format!(
            "SelfHealingConfig exceeds the {MAX_AUTONOMOUS_ENTRIES}-invariant limit"
        )));
    }
    for (index, invariant) in config.invariants.iter().enumerate() {
        if invariant.key.trim().is_empty() {
            return Err(ChainError::GenerationError(
                "health-invariant key must not be empty".to_string(),
            ));
        }
        if invariant.min_value > invariant.max_value {
            return Err(ChainError::GenerationError(format!(
                "health-invariant '{}' has an inverted band (min > max)",
                invariant.key
            )));
        }
        for other in config.invariants.iter().skip(index + 1) {
            if invariant.key == other.key {
                return Err(ChainError::GenerationError(format!(
                    "duplicate health-invariant key: '{}'",
                    invariant.key
                )));
            }
        }
    }
    Ok(())
}

/// Validates an [`AutoOptimizerConfig`] for coherence before codegen.
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] if the name, parameter or metric label
/// is empty, if `min_value > max_value`, if `initial_value` is outside
/// `[min_value, max_value]`, or if `step_value` is zero.
pub fn validate_auto_optimizer(config: &AutoOptimizerConfig) -> ChainResult<()> {
    if config.name.trim().is_empty() {
        return Err(ChainError::GenerationError(
            "AutoOptimizerConfig.name must not be empty".to_string(),
        ));
    }
    if config.parameter_label.trim().is_empty() {
        return Err(ChainError::GenerationError(
            "AutoOptimizerConfig.parameter_label must not be empty".to_string(),
        ));
    }
    if config.metric_label.trim().is_empty() {
        return Err(ChainError::GenerationError(
            "AutoOptimizerConfig.metric_label must not be empty".to_string(),
        ));
    }
    if config.min_value > config.max_value {
        return Err(ChainError::GenerationError(
            "AutoOptimizerConfig.min_value must be <= max_value".to_string(),
        ));
    }
    if config.initial_value < config.min_value || config.initial_value > config.max_value {
        return Err(ChainError::GenerationError(
            "AutoOptimizerConfig.initial_value must lie within [min_value, max_value]".to_string(),
        ));
    }
    if config.step_value == 0 {
        return Err(ChainError::GenerationError(
            "AutoOptimizerConfig.step_value must be greater than zero".to_string(),
        ));
    }
    Ok(())
}

/// Validates a [`ResourceManagerConfig`] for coherence before codegen.
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] if the name is empty, if the bucket
/// capacity or per-op cost is zero, or if the per-op cost exceeds the capacity
/// (an operation that can never be afforded).
pub fn validate_resource_manager(config: &ResourceManagerConfig) -> ChainResult<()> {
    if config.name.trim().is_empty() {
        return Err(ChainError::GenerationError(
            "ResourceManagerConfig.name must not be empty".to_string(),
        ));
    }
    if config.bucket_capacity == 0 {
        return Err(ChainError::GenerationError(
            "ResourceManagerConfig.bucket_capacity must be greater than zero".to_string(),
        ));
    }
    if config.cost_per_op == 0 {
        return Err(ChainError::GenerationError(
            "ResourceManagerConfig.cost_per_op must be greater than zero".to_string(),
        ));
    }
    if config.cost_per_op > config.bucket_capacity {
        return Err(ChainError::GenerationError(
            "ResourceManagerConfig.cost_per_op must be <= bucket_capacity".to_string(),
        ));
    }
    Ok(())
}

/// Validates a [`PerformanceMonitorConfig`] for coherence before codegen.
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] if the name is empty, if the operation
/// set is empty / over the entry limit / has empty or duplicate names, or if
/// `ema_alpha_bps` is not within `1..=10000`.
pub fn validate_performance_monitor(config: &PerformanceMonitorConfig) -> ChainResult<()> {
    if config.name.trim().is_empty() {
        return Err(ChainError::GenerationError(
            "PerformanceMonitorConfig.name must not be empty".to_string(),
        ));
    }
    if config.operations.is_empty() {
        return Err(ChainError::GenerationError(
            "PerformanceMonitorConfig must declare at least one operation".to_string(),
        ));
    }
    if config.operations.len() > MAX_AUTONOMOUS_ENTRIES {
        return Err(ChainError::GenerationError(format!(
            "PerformanceMonitorConfig exceeds the {MAX_AUTONOMOUS_ENTRIES}-operation limit"
        )));
    }
    if config.ema_alpha_bps == 0 || config.ema_alpha_bps > BASIS_POINTS_DENOMINATOR {
        return Err(ChainError::GenerationError(format!(
            "PerformanceMonitorConfig.ema_alpha_bps must be within 1..={BASIS_POINTS_DENOMINATOR}"
        )));
    }
    for (index, op) in config.operations.iter().enumerate() {
        if op.trim().is_empty() {
            return Err(ChainError::GenerationError(
                "monitored operation name must not be empty".to_string(),
            ));
        }
        for other in config.operations.iter().skip(index + 1) {
            if op == other {
                return Err(ChainError::GenerationError(format!(
                    "duplicate monitored operation: '{op}'"
                )));
            }
        }
    }
    Ok(())
}

/// Validates a [`CostOptimizerConfig`] for coherence before codegen.
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] if the name is empty or if
/// `max_batch_size` is zero.
pub fn validate_cost_optimizer(config: &CostOptimizerConfig) -> ChainResult<()> {
    if config.name.trim().is_empty() {
        return Err(ChainError::GenerationError(
            "CostOptimizerConfig.name must not be empty".to_string(),
        ));
    }
    if config.max_batch_size == 0 {
        return Err(ChainError::GenerationError(
            "CostOptimizerConfig.max_batch_size must be greater than zero".to_string(),
        ));
    }
    Ok(())
}

impl ContractGenerator {
    /// Generates a self-healing controller contract.
    ///
    /// Emits a finite state machine that autonomously moves a subsystem through
    /// `Healthy -> Degraded -> Recovering -> Healthy`: a reported metric outside
    /// its healthy band trips the breach, the last-known-good checkpoint is
    /// restored automatically, and the contract auto-resumes after the metric
    /// recovers and a cool-down elapses — no operator action required. This is
    /// distinct from the guardian-triggered [`crate::EmergencyPauseConfig`], whose
    /// unpause is a privileged manual step.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_chain::{
    ///     ContractGenerator, HealthInvariant, Jurisdiction, SelfHealingConfig, TargetPlatform,
    /// };
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let config = SelfHealingConfig {
    ///     name: "SelfHealingVault".to_string(),
    ///     invariants: vec![HealthInvariant {
    ///         key: "collateral_ratio_bps".to_string(),
    ///         description: "Collateral ratio in basis points".to_string(),
    ///         min_value: 11_000,
    ///         max_value: 1_000_000,
    ///     }],
    ///     checkpoint_enabled: true,
    ///     recover_after_seconds: 3600,
    ///     keeper_reward_wei: 0,
    ///     jurisdiction: Jurisdiction::Us,
    /// };
    /// let contract = generator.generate_self_healing(&config).unwrap();
    /// assert!(contract.source.contains("function reportHealth"));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if the configuration is invalid (see
    /// [`validate_self_healing`]).
    pub fn generate_self_healing(
        &self,
        config: &SelfHealingConfig,
    ) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_self_healing(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "Self-healing contracts not supported for {:?}",
                self.platform
            )))
        }
    }

    /// Generates an automatic-optimization (feedback controller) contract.
    ///
    /// Emits an on-chain step controller that nudges a tunable parameter toward a
    /// metric set-point, bounded by `[min_value, max_value]`, by at most
    /// `step_value` per cool-down period. Off-chain tooling can predict every move
    /// with [`adjust_parameter`].
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if the configuration is invalid (see
    /// [`validate_auto_optimizer`]).
    pub fn generate_auto_optimizer(
        &self,
        config: &AutoOptimizerConfig,
    ) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_auto_optimizer(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "Automatic optimization not supported for {:?}",
                self.platform
            )))
        }
    }

    /// Generates a resource-management (rate-limit + budget) contract.
    ///
    /// Emits a token-bucket rate limiter — global or per-caller — with a per-epoch
    /// consumption budget. The refill math mirrors [`token_bucket_available`].
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if the configuration is invalid (see
    /// [`validate_resource_manager`]).
    pub fn generate_resource_manager(
        &self,
        config: &ResourceManagerConfig,
    ) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_resource_manager(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "Resource management not supported for {:?}",
                self.platform
            )))
        }
    }

    /// Generates a performance-monitoring contract.
    ///
    /// Emits per-operation on-chain metrics (call count, cumulative gas, EMA gas,
    /// min/max) with optional metric events and a `healthScore` view. The EMA and
    /// score mirror [`ema_update`] and [`health_score`].
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_chain::{
    ///     ContractGenerator, Jurisdiction, PerformanceMonitorConfig, TargetPlatform,
    /// };
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let config = PerformanceMonitorConfig {
    ///     name: "OpsMonitor".to_string(),
    ///     operations: vec!["settle".to_string(), "claim".to_string()],
    ///     ema_alpha_bps: 2000,
    ///     emit_events: true,
    ///     health_score: true,
    ///     gas_budget_per_op: 120_000,
    ///     jurisdiction: Jurisdiction::Eu,
    /// };
    /// let contract = generator.generate_performance_monitor(&config).unwrap();
    /// assert!(contract.source.contains("function record"));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if the configuration is invalid (see
    /// [`validate_performance_monitor`]).
    pub fn generate_performance_monitor(
        &self,
        config: &PerformanceMonitorConfig,
    ) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_performance_monitor(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "Performance monitoring not supported for {:?}",
                self.platform
            )))
        }
    }

    /// Generates a cost-optimization contract.
    ///
    /// Emits a batched multicall that amortises the base transaction cost across
    /// many operations, optionally harvests storage refunds, and (when a base-fee
    /// ceiling is set) defers non-urgent work while gas is expensive. The savings
    /// estimate mirrors [`batch_savings`] and the deferral mirrors [`should_defer`].
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if the configuration is invalid (see
    /// [`validate_cost_optimizer`]).
    pub fn generate_cost_optimizer(
        &self,
        config: &CostOptimizerConfig,
    ) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_cost_optimizer(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "Cost optimization not supported for {:?}",
                self.platform
            )))
        }
    }
}
