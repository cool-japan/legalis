//! Dynamic contract-evolution generators.
//!
//! This module extends [`ContractGenerator`] with production-grade generators for
//! evolving a deployed contract system *after* launch without redeploying from
//! scratch — the "Governance & Upgradability" toolkit:
//!
//! * **On-chain upgrade governance** — a self-contained, token-weighted governor
//!   whose *only* power is to vote a proxy onto a new implementation, gated behind
//!   a quorum check and a timelock delay ([`UpgradeGovernanceConfig`]).
//! * **Feature flags** — a registry of named on/off switches with per-address
//!   overrides and a global kill switch ([`FeatureFlagConfig`]).
//! * **A/B testing** — a deterministic, weight-bucketed variant router with
//!   optional sticky assignment and conversion accounting ([`AbTestConfig`]).
//! * **Gradual rollout** — time-based (linear), manual, or canary percentage
//!   rollout of a feature with guardian rollback ([`GradualRolloutConfig`]).
//! * **Emergency pause** — a tiered, guardian-triggered pause controller with
//!   automatic expiry (no permanent freeze) and governance-gated unpause
//!   ([`EmergencyPauseConfig`]).
//!
//! As with the [`crate::tokenization`] module, the apportionment and scheduling
//! math lives in pure Rust ([`assign_variant`], [`rollout_basis_points_at`],
//! [`quorum_votes`]) so that it is validated *before* any contract source is
//! emitted and is independently unit-testable for on-chain/off-chain parity.
//!
//! Every generator targets the EVM family (Solidity); non-EVM targets return a
//! [`ChainError::GenerationError`]. The target abstraction, error type and
//! [`crate::BASIS_POINTS_DENOMINATOR`] constant are reused from the existing
//! crate so no behaviour is duplicated.

mod ab_test;
mod emergency_pause;
mod feature_flags;
mod gradual_rollout;
mod upgrade_governance;

#[cfg(test)]
mod tests;

use super::contractgenerator_type::ContractGenerator;
use super::functions::ChainResult;
use super::types_19::{ChainError, GeneratedContract, Jurisdiction};
use crate::BASIS_POINTS_DENOMINATOR;
use crate::tokenization::is_evm_target;

/// Maximum number of A/B variants or rollout/pause scopes accepted in one
/// contract. Bounds the size of generated source and the gas of any on-chain
/// loop over the set.
pub const MAX_EVOLUTION_ENTRIES: usize = 64;

/// Proxy standard governed by an [`UpgradeGovernanceConfig`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyKind {
    /// UUPS (EIP-1822): upgrade logic lives in the implementation, invoked via
    /// `upgradeToAndCall` on the proxy itself.
    Uups,
    /// Transparent proxy (EIP-1967) administered through a `ProxyAdmin` whose
    /// `upgradeAndCall(proxy, impl, data)` entry point performs the upgrade.
    Transparent,
}

impl ProxyKind {
    /// Human-readable label used in generated NatSpec documentation.
    pub(crate) fn label(self) -> &'static str {
        match self {
            ProxyKind::Uups => "UUPS (EIP-1822)",
            ProxyKind::Transparent => "Transparent (EIP-1967 ProxyAdmin)",
        }
    }
}

/// Strategy used to assign a caller to an A/B variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariantAssignment {
    /// Pure function of the caller address — same address always maps to the same
    /// variant, no storage writes required.
    Deterministic,
    /// Deterministic on first contact, then persisted so the assignment survives
    /// even if the variant weights are later changed.
    Sticky,
}

impl VariantAssignment {
    /// Human-readable label used in generated NatSpec documentation.
    pub(crate) fn label(self) -> &'static str {
        match self {
            VariantAssignment::Deterministic => "deterministic (stateless)",
            VariantAssignment::Sticky => "sticky (persisted on first contact)",
        }
    }
}

/// Scheduling strategy for a gradual feature rollout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RolloutStrategy {
    /// Percentage auto-increments by a fixed step every fixed interval until the
    /// target share is reached.
    Linear,
    /// Percentage only changes when an administrator explicitly advances it.
    Manual,
    /// An allowlist is served first; the percentage bucket applies to everyone
    /// else (the classic canary pattern).
    Canary,
}

impl RolloutStrategy {
    /// Human-readable label used in generated NatSpec documentation.
    pub(crate) fn label(self) -> &'static str {
        match self {
            RolloutStrategy::Linear => "linear (time-based auto-increment)",
            RolloutStrategy::Manual => "manual (admin-stepped)",
            RolloutStrategy::Canary => "canary (allowlist then percentage)",
        }
    }
}

/// Access-control model applied to a feature-flag registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagAdminModel {
    /// A single owner (via `Ownable2Step`) administers every flag.
    Owner,
    /// A dedicated `FLAG_ADMIN_ROLE` (via `AccessControl`) administers flags,
    /// allowing the role to be held by several operators or a multisig.
    Roles,
}

/// A single named feature flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureFlag {
    /// Stable key hashed on-chain (`keccak256(bytes(key))`).
    pub key: String,
    /// Human-readable description embedded in NatSpec.
    pub description: String,
    /// Whether the flag is enabled at deployment.
    pub default_enabled: bool,
}

/// One A/B-test variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbVariant {
    /// Unique variant label (e.g. `"control"`, `"treatment"`).
    pub label: String,
    /// Selection weight in basis points; the set must sum to
    /// [`crate::BASIS_POINTS_DENOMINATOR`].
    pub weight_basis_points: u32,
    /// Optional implementation/handler address calls are routed to for this
    /// variant. `None` records the assignment without routing.
    pub implementation: Option<String>,
}

/// On-chain upgrade-governance configuration.
#[derive(Debug, Clone)]
pub struct UpgradeGovernanceConfig {
    /// Governor contract name.
    pub name: String,
    /// `ERC20Votes`/`IVotes` token whose snapshot weight decides upgrades.
    pub governance_token: String,
    /// Proxy whose implementation this governor is authorised to change.
    pub proxy: String,
    /// Proxy standard governed.
    pub proxy_kind: ProxyKind,
    /// Blocks between proposal creation and the start of voting.
    pub voting_delay_blocks: u64,
    /// Length of the voting window in blocks.
    pub voting_period_blocks: u64,
    /// Minimum snapshot votes required to create a proposal.
    pub proposal_threshold: u64,
    /// Quorum as a fraction of the snapshot total supply, in basis points.
    pub quorum_basis_points: u32,
    /// Delay between queueing a passed proposal and executing the upgrade.
    pub timelock_delay_seconds: u64,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
}

/// Feature-flag registry configuration.
#[derive(Debug, Clone)]
pub struct FeatureFlagConfig {
    /// Contract name.
    pub name: String,
    /// Flags declared at construction time.
    pub flags: Vec<FeatureFlag>,
    /// Access-control model for flag administration.
    pub admin_model: FlagAdminModel,
    /// Whether to include a global kill switch that disables every flag at once.
    pub global_kill_switch: bool,
    /// Whether to support per-address flag overrides.
    pub per_address_overrides: bool,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
}

/// A/B-test router configuration.
#[derive(Debug, Clone)]
pub struct AbTestConfig {
    /// Contract name.
    pub name: String,
    /// Experiment identifier mixed into the assignment hash (namespaces the test).
    pub experiment_key: String,
    /// Variants; weights must sum to [`crate::BASIS_POINTS_DENOMINATOR`].
    pub variants: Vec<AbVariant>,
    /// How callers are assigned to variants.
    pub assignment: VariantAssignment,
    /// Whether to track per-variant exposure and conversion counters.
    pub measure_conversions: bool,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
}

/// Gradual-rollout configuration.
#[derive(Debug, Clone)]
pub struct GradualRolloutConfig {
    /// Contract name.
    pub name: String,
    /// Rollout scheduling strategy.
    pub strategy: RolloutStrategy,
    /// Starting share, in basis points.
    pub start_basis_points: u32,
    /// Final share, in basis points (must be `>= start` and `<= 10000`).
    pub target_basis_points: u32,
    /// Increment per step, in basis points (used by `Linear`).
    pub step_basis_points: u32,
    /// Seconds between automatic steps (used by `Linear`).
    pub step_interval_seconds: u64,
    /// Whether a guardian may roll the share back to `start` in an emergency.
    pub guardian_rollback: bool,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
}

/// Emergency-pause controller configuration.
#[derive(Debug, Clone)]
pub struct EmergencyPauseConfig {
    /// Contract name.
    pub name: String,
    /// Named pausable subsystems. Empty means a single global scope.
    pub scopes: Vec<String>,
    /// Guardian addresses permitted to *trigger* a pause.
    pub guardians: Vec<String>,
    /// Governance address that may *unpause*, extend, or manage guardians.
    pub governance: String,
    /// Maximum seconds a pause stays effective before auto-expiring
    /// (`0` disables auto-expiry).
    pub max_pause_seconds: u64,
    /// Mandatory cool-down between requesting and finalising an unpause.
    pub unpause_delay_seconds: u64,
    /// Governing jurisdiction for compliance hooks.
    pub jurisdiction: Jurisdiction,
}

/// Sanitises an arbitrary key into an upper-snake Solidity-safe identifier
/// suffix (used to name generated `bytes32` constants).
///
/// Non-alphanumeric characters collapse to `_`; an empty or digit-leading result
/// is prefixed so the identifier is always valid.
pub(crate) fn sanitize_identifier(key: &str) -> String {
    let mut out = String::with_capacity(key.len());
    for ch in key.chars() {
        if ch.is_ascii_alphanumeric() {
            out.extend(ch.to_uppercase());
        } else {
            out.push('_');
        }
    }
    let needs_prefix = out
        .chars()
        .next()
        .is_none_or(|first| !first.is_ascii_alphabetic() && first != '_');
    if needs_prefix {
        let mut prefixed = String::with_capacity(out.len() + 1);
        prefixed.push('K');
        prefixed.push_str(&out);
        prefixed
    } else {
        out
    }
}

/// Returns the generated `bytes32` constant name for a feature-flag key.
pub fn flag_constant_name(key: &str) -> String {
    format!("FLAG_{}", sanitize_identifier(key))
}

/// Returns the generated `bytes32` constant name for a pause scope.
pub fn scope_constant_name(scope: &str) -> String {
    format!("SCOPE_{}", sanitize_identifier(scope))
}

/// Computes the quorum vote count for a snapshot supply and basis-point fraction.
///
/// Mirrors the on-chain `(supply * QUORUM_BPS) / 10000` integer arithmetic so
/// off-chain tooling agrees with the generated governor.
///
/// Returns `0` if the multiplication would overflow `u128` (the on-chain code
/// uses `uint256`, so this only bounds the *off-chain* preview).
pub fn quorum_votes(total_supply: u128, quorum_basis_points: u32) -> u128 {
    let denominator = u128::from(BASIS_POINTS_DENOMINATOR);
    match total_supply.checked_mul(u128::from(quorum_basis_points)) {
        Some(product) => product / denominator,
        None => 0,
    }
}

/// Validates an [`UpgradeGovernanceConfig`] for coherence before codegen.
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] if the name, token or proxy is empty,
/// if the voting period is zero, or if the quorum exceeds 100%.
pub fn validate_upgrade_governance(config: &UpgradeGovernanceConfig) -> ChainResult<()> {
    if config.name.trim().is_empty() {
        return Err(ChainError::GenerationError(
            "UpgradeGovernanceConfig.name must not be empty".to_string(),
        ));
    }
    if config.governance_token.trim().is_empty() {
        return Err(ChainError::GenerationError(
            "UpgradeGovernanceConfig.governance_token must not be empty".to_string(),
        ));
    }
    if config.proxy.trim().is_empty() {
        return Err(ChainError::GenerationError(
            "UpgradeGovernanceConfig.proxy must not be empty".to_string(),
        ));
    }
    if config.voting_period_blocks == 0 {
        return Err(ChainError::GenerationError(
            "UpgradeGovernanceConfig.voting_period_blocks must be greater than zero".to_string(),
        ));
    }
    if config.quorum_basis_points > BASIS_POINTS_DENOMINATOR {
        return Err(ChainError::GenerationError(format!(
            "UpgradeGovernanceConfig.quorum_basis_points must be <= {BASIS_POINTS_DENOMINATOR}"
        )));
    }
    Ok(())
}

/// Validates a set of feature flags: non-empty, bounded, with unique non-empty
/// keys.
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] describing the first violated invariant.
pub fn validate_feature_flags(flags: &[FeatureFlag]) -> ChainResult<()> {
    if flags.is_empty() {
        return Err(ChainError::GenerationError(
            "feature-flag registry must declare at least one flag".to_string(),
        ));
    }
    if flags.len() > MAX_EVOLUTION_ENTRIES {
        return Err(ChainError::GenerationError(format!(
            "feature-flag registry exceeds the {MAX_EVOLUTION_ENTRIES}-flag limit"
        )));
    }
    for (index, flag) in flags.iter().enumerate() {
        if flag.key.trim().is_empty() {
            return Err(ChainError::GenerationError(
                "feature-flag key must not be empty".to_string(),
            ));
        }
        for other in flags.iter().skip(index + 1) {
            if flag.key == other.key {
                return Err(ChainError::GenerationError(format!(
                    "duplicate feature-flag key: '{}'",
                    flag.key
                )));
            }
        }
    }
    Ok(())
}

/// Validates A/B variants: at least two, bounded, unique labels, strictly
/// positive weights summing to exactly [`crate::BASIS_POINTS_DENOMINATOR`].
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] describing the first violated invariant.
pub fn validate_ab_variants(variants: &[AbVariant]) -> ChainResult<()> {
    if variants.len() < 2 {
        return Err(ChainError::GenerationError(
            "A/B test must declare at least two variants".to_string(),
        ));
    }
    if variants.len() > MAX_EVOLUTION_ENTRIES {
        return Err(ChainError::GenerationError(format!(
            "A/B test exceeds the {MAX_EVOLUTION_ENTRIES}-variant limit"
        )));
    }
    let mut total: u32 = 0;
    for (index, variant) in variants.iter().enumerate() {
        if variant.label.trim().is_empty() {
            return Err(ChainError::GenerationError(
                "A/B variant label must not be empty".to_string(),
            ));
        }
        if variant.weight_basis_points == 0 {
            return Err(ChainError::GenerationError(format!(
                "A/B variant '{}' weight must be greater than zero",
                variant.label
            )));
        }
        total = total
            .checked_add(variant.weight_basis_points)
            .ok_or_else(|| {
                ChainError::GenerationError("A/B variant weights overflow u32".to_string())
            })?;
        for other in variants.iter().skip(index + 1) {
            if variant.label == other.label {
                return Err(ChainError::GenerationError(format!(
                    "duplicate A/B variant label: '{}'",
                    variant.label
                )));
            }
        }
    }
    if total != BASIS_POINTS_DENOMINATOR {
        return Err(ChainError::GenerationError(format!(
            "A/B variant weights must sum to {BASIS_POINTS_DENOMINATOR} basis points (got {total})"
        )));
    }
    Ok(())
}

/// Returns the exclusive cumulative weight thresholds for `variants`.
///
/// For weights `[3000, 3000, 4000]` the thresholds are `[3000, 6000, 10000]`,
/// i.e. variant `i` owns buckets in `[thresholds[i-1], thresholds[i])`.
pub fn cumulative_thresholds(variants: &[AbVariant]) -> Vec<u32> {
    let mut running: u32 = 0;
    variants
        .iter()
        .map(|variant| {
            running = running.saturating_add(variant.weight_basis_points);
            running
        })
        .collect()
}

/// Assigns a bucket in `[0, 10000)` to a variant index using the cumulative
/// weights of `variants`.
///
/// Mirrors the on-chain loop `for i: if (bucket < CUMULATIVE[i]) return i;`,
/// guaranteeing off-chain/on-chain parity.
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] if the variants are invalid (see
/// [`validate_ab_variants`]) or if `bucket >= 10000`.
pub fn assign_variant(bucket: u32, variants: &[AbVariant]) -> ChainResult<usize> {
    validate_ab_variants(variants)?;
    if bucket >= BASIS_POINTS_DENOMINATOR {
        return Err(ChainError::GenerationError(format!(
            "A/B bucket must be < {BASIS_POINTS_DENOMINATOR} (got {bucket})"
        )));
    }
    let thresholds = cumulative_thresholds(variants);
    for (index, threshold) in thresholds.iter().enumerate() {
        if bucket < *threshold {
            return Ok(index);
        }
    }
    // Unreachable in practice because the weights sum to 10000 and bucket < 10000,
    // but returned as an error rather than panicking to honour the no-panic policy.
    Err(ChainError::GenerationError(
        "A/B bucket fell outside every variant range".to_string(),
    ))
}

/// Validates a [`GradualRolloutConfig`] for coherence before codegen.
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] if any basis-point field exceeds 100%,
/// if `start > target`, or if a `Linear` schedule has a zero step/interval while
/// it still needs to make progress.
pub fn validate_gradual_rollout(config: &GradualRolloutConfig) -> ChainResult<()> {
    if config.name.trim().is_empty() {
        return Err(ChainError::GenerationError(
            "GradualRolloutConfig.name must not be empty".to_string(),
        ));
    }
    for (field, value) in [
        ("start_basis_points", config.start_basis_points),
        ("target_basis_points", config.target_basis_points),
        ("step_basis_points", config.step_basis_points),
    ] {
        if value > BASIS_POINTS_DENOMINATOR {
            return Err(ChainError::GenerationError(format!(
                "GradualRolloutConfig.{field} must be <= {BASIS_POINTS_DENOMINATOR}"
            )));
        }
    }
    if config.start_basis_points > config.target_basis_points {
        return Err(ChainError::GenerationError(
            "GradualRolloutConfig.start_basis_points must be <= target_basis_points".to_string(),
        ));
    }
    let time_based = matches!(
        config.strategy,
        RolloutStrategy::Linear | RolloutStrategy::Canary
    );
    if time_based
        && config.start_basis_points < config.target_basis_points
        && (config.step_basis_points == 0 || config.step_interval_seconds == 0)
    {
        return Err(ChainError::GenerationError(
            "time-based rollout needs a non-zero step_basis_points and step_interval_seconds"
                .to_string(),
        ));
    }
    Ok(())
}

/// Computes the rollout share (basis points) `elapsed_seconds` after activation
/// for a linear schedule.
///
/// `value = min(start + floor(elapsed / interval) * step, target)`. An interval
/// of `0` returns `target` immediately. All arithmetic saturates, so the result
/// is always in `[start, target]`. Mirrors the generated `currentBps()` view.
pub fn rollout_basis_points_at(
    start_basis_points: u32,
    step_basis_points: u32,
    target_basis_points: u32,
    step_interval_seconds: u64,
    elapsed_seconds: u64,
) -> u32 {
    if step_interval_seconds == 0 {
        return target_basis_points;
    }
    let steps = elapsed_seconds / step_interval_seconds;
    let steps = u32::try_from(steps).unwrap_or(u32::MAX);
    let increment = step_basis_points.saturating_mul(steps);
    let value = start_basis_points.saturating_add(increment);
    value.min(target_basis_points)
}

/// Returns whether a caller hashed to `bucket` (in `[0, 10000)`) is inside a
/// rollout of `basis_points`. Mirrors on-chain `bucket < currentBps()`.
pub fn is_in_rollout_bucket(bucket: u32, basis_points: u32) -> bool {
    bucket < basis_points
}

/// Validates an [`EmergencyPauseConfig`] for coherence before codegen.
///
/// # Errors
///
/// Returns [`ChainError::GenerationError`] if governance is empty, if the
/// guardian set is empty/duplicated, or if scope names collide or exceed the
/// entry limit.
pub fn validate_emergency_pause(config: &EmergencyPauseConfig) -> ChainResult<()> {
    if config.name.trim().is_empty() {
        return Err(ChainError::GenerationError(
            "EmergencyPauseConfig.name must not be empty".to_string(),
        ));
    }
    if config.governance.trim().is_empty() {
        return Err(ChainError::GenerationError(
            "EmergencyPauseConfig.governance must not be empty".to_string(),
        ));
    }
    if config.guardians.is_empty() {
        return Err(ChainError::GenerationError(
            "EmergencyPauseConfig must declare at least one guardian".to_string(),
        ));
    }
    for (index, guardian) in config.guardians.iter().enumerate() {
        if guardian.trim().is_empty() {
            return Err(ChainError::GenerationError(
                "EmergencyPauseConfig guardian address must not be empty".to_string(),
            ));
        }
        for other in config.guardians.iter().skip(index + 1) {
            if guardian == other {
                return Err(ChainError::GenerationError(format!(
                    "duplicate guardian in EmergencyPauseConfig: '{guardian}'"
                )));
            }
        }
    }
    if config.scopes.len() > MAX_EVOLUTION_ENTRIES {
        return Err(ChainError::GenerationError(format!(
            "EmergencyPauseConfig exceeds the {MAX_EVOLUTION_ENTRIES}-scope limit"
        )));
    }
    for (index, scope) in config.scopes.iter().enumerate() {
        if scope.trim().is_empty() {
            return Err(ChainError::GenerationError(
                "EmergencyPauseConfig scope name must not be empty".to_string(),
            ));
        }
        for other in config.scopes.iter().skip(index + 1) {
            if scope == other {
                return Err(ChainError::GenerationError(format!(
                    "duplicate pause scope: '{scope}'"
                )));
            }
        }
    }
    Ok(())
}

impl ContractGenerator {
    /// Generates an on-chain upgrade-governance contract.
    ///
    /// Emits a self-contained, token-weighted governor whose proposals each carry
    /// a candidate implementation address; a passed proposal is queued behind a
    /// timelock and then executed as a proxy upgrade. This is distinct from the
    /// general-purpose DAO governor: its execution path is hard-wired to a single
    /// proxy upgrade, which is the property auditors care about for upgradability.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_chain::{
    ///     ContractGenerator, Jurisdiction, ProxyKind, TargetPlatform, UpgradeGovernanceConfig,
    /// };
    ///
    /// let generator = ContractGenerator::new(TargetPlatform::Solidity);
    /// let config = UpgradeGovernanceConfig {
    ///     name: "ProtocolUpgradeGovernor".to_string(),
    ///     governance_token: "0x1111111111111111111111111111111111111111".to_string(),
    ///     proxy: "0x2222222222222222222222222222222222222222".to_string(),
    ///     proxy_kind: ProxyKind::Uups,
    ///     voting_delay_blocks: 7200,
    ///     voting_period_blocks: 50400,
    ///     proposal_threshold: 100_000,
    ///     quorum_basis_points: 400,
    ///     timelock_delay_seconds: 172_800,
    ///     jurisdiction: Jurisdiction::Us,
    /// };
    /// let contract = generator.generate_upgrade_governance(&config).unwrap();
    /// assert!(contract.source.contains("function executeUpgrade"));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if the configuration is invalid (see
    /// [`validate_upgrade_governance`]).
    pub fn generate_upgrade_governance(
        &self,
        config: &UpgradeGovernanceConfig,
    ) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_upgrade_governance(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "Upgrade governance not supported for {:?}",
                self.platform
            )))
        }
    }

    /// Generates a feature-flag registry contract.
    ///
    /// Emits a registry of named on/off switches with optional per-address
    /// overrides and an optional global kill switch, administered by an owner or
    /// an access-control role.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if the flags are invalid (see [`validate_feature_flags`]).
    pub fn generate_feature_flags(
        &self,
        config: &FeatureFlagConfig,
    ) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_feature_flags(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "Feature flags not supported for {:?}",
                self.platform
            )))
        }
    }

    /// Generates an A/B-test router contract.
    ///
    /// Validates that variant weights sum to 100% before emitting a contract that
    /// buckets callers into variants by a keccak hash of the experiment key and
    /// the caller address, optionally persisting the assignment and recording
    /// conversion metrics.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if the variants are invalid (see [`validate_ab_variants`]).
    pub fn generate_ab_test(&self, config: &AbTestConfig) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_ab_test(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "A/B testing not supported for {:?}",
                self.platform
            )))
        }
    }

    /// Generates a gradual-rollout controller contract.
    ///
    /// Emits a contract that exposes the current rollout share (computed from a
    /// linear schedule, manual steps, or a canary allowlist) and an
    /// `isActiveFor(address)` predicate, with optional guardian rollback.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if the configuration is invalid (see
    /// [`validate_gradual_rollout`]).
    pub fn generate_gradual_rollout(
        &self,
        config: &GradualRolloutConfig,
    ) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_gradual_rollout(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "Gradual rollout not supported for {:?}",
                self.platform
            )))
        }
    }

    /// Generates an emergency-pause controller contract.
    ///
    /// Emits a tiered pause controller where guardians may pause subsystems but
    /// only governance may unpause, every pause auto-expires after a maximum
    /// duration (preventing a permanent freeze), and unpausing is subject to a
    /// mandatory cool-down.
    ///
    /// # Errors
    ///
    /// Returns [`ChainError::GenerationError`] if the target platform is not
    /// EVM-compatible, or if the configuration is invalid (see
    /// [`validate_emergency_pause`]).
    pub fn generate_emergency_pause(
        &self,
        config: &EmergencyPauseConfig,
    ) -> ChainResult<GeneratedContract> {
        if is_evm_target(self.platform) {
            self.generate_solidity_emergency_pause(config)
        } else {
            Err(ChainError::GenerationError(format!(
                "Emergency pause not supported for {:?}",
                self.platform
            )))
        }
    }
}
