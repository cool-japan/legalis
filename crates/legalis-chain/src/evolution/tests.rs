//! Tests for the dynamic contract-evolution generators and domain math.

use crate::{
    AbTestConfig, AbVariant, ContractGenerator, EmergencyPauseConfig, FeatureFlag,
    FeatureFlagConfig, FlagAdminModel, GradualRolloutConfig, Jurisdiction, ProxyKind,
    RolloutStrategy, TargetPlatform, UpgradeGovernanceConfig, VariantAssignment, assign_variant,
    cumulative_thresholds, flag_constant_name, is_in_rollout_bucket, quorum_votes,
    rollout_basis_points_at, scope_constant_name, validate_ab_variants, validate_emergency_pause,
    validate_feature_flags, validate_gradual_rollout, validate_upgrade_governance,
};

const ADDR_A: &str = "0x1111111111111111111111111111111111111111";
const ADDR_B: &str = "0x2222222222222222222222222222222222222222";
const ADDR_C: &str = "0x3333333333333333333333333333333333333333";
const ADDR_D: &str = "0x4444444444444444444444444444444444444444";

// --- Config builders -----------------------------------------------------------

fn upgrade_config() -> UpgradeGovernanceConfig {
    UpgradeGovernanceConfig {
        name: "ProtocolUpgradeGovernor".to_string(),
        governance_token: ADDR_A.to_string(),
        proxy: ADDR_B.to_string(),
        proxy_kind: ProxyKind::Uups,
        voting_delay_blocks: 7200,
        voting_period_blocks: 50400,
        proposal_threshold: 100_000,
        quorum_basis_points: 400,
        timelock_delay_seconds: 172_800,
        jurisdiction: Jurisdiction::Us,
    }
}

fn flag_config() -> FeatureFlagConfig {
    FeatureFlagConfig {
        name: "FeatureRegistry".to_string(),
        flags: vec![
            FeatureFlag {
                key: "beta_checkout".to_string(),
                description: "Beta checkout flow".to_string(),
                default_enabled: true,
            },
            FeatureFlag {
                key: "new_dashboard".to_string(),
                description: "New dashboard UI".to_string(),
                default_enabled: false,
            },
        ],
        admin_model: FlagAdminModel::Owner,
        global_kill_switch: true,
        per_address_overrides: true,
        jurisdiction: Jurisdiction::Eu,
    }
}

fn ab_config() -> AbTestConfig {
    AbTestConfig {
        name: "CheckoutExperiment".to_string(),
        experiment_key: "checkout_flow".to_string(),
        variants: vec![
            AbVariant {
                label: "control".to_string(),
                weight_basis_points: 3000,
                implementation: None,
            },
            AbVariant {
                label: "variant_a".to_string(),
                weight_basis_points: 3000,
                implementation: None,
            },
            AbVariant {
                label: "variant_b".to_string(),
                weight_basis_points: 4000,
                implementation: None,
            },
        ],
        assignment: VariantAssignment::Deterministic,
        measure_conversions: false,
        jurisdiction: Jurisdiction::Us,
    }
}

fn ab_sticky_config() -> AbTestConfig {
    AbTestConfig {
        name: "RouterExperiment".to_string(),
        experiment_key: "router".to_string(),
        variants: vec![
            AbVariant {
                label: "old".to_string(),
                weight_basis_points: 5000,
                implementation: Some(ADDR_C.to_string()),
            },
            AbVariant {
                label: "new".to_string(),
                weight_basis_points: 5000,
                implementation: Some(ADDR_D.to_string()),
            },
        ],
        assignment: VariantAssignment::Sticky,
        measure_conversions: true,
        jurisdiction: Jurisdiction::Us,
    }
}

fn rollout_linear_config() -> GradualRolloutConfig {
    GradualRolloutConfig {
        name: "FeatureRollout".to_string(),
        strategy: RolloutStrategy::Linear,
        start_basis_points: 1000,
        target_basis_points: 5000,
        step_basis_points: 1000,
        step_interval_seconds: 86_400,
        guardian_rollback: false,
        jurisdiction: Jurisdiction::Us,
    }
}

fn rollout_canary_config() -> GradualRolloutConfig {
    GradualRolloutConfig {
        name: "CanaryRollout".to_string(),
        strategy: RolloutStrategy::Canary,
        start_basis_points: 0,
        target_basis_points: 10_000,
        step_basis_points: 2000,
        step_interval_seconds: 3600,
        guardian_rollback: true,
        jurisdiction: Jurisdiction::Eu,
    }
}

fn pause_full_config() -> EmergencyPauseConfig {
    EmergencyPauseConfig {
        name: "EmergencyController".to_string(),
        scopes: vec!["trading".to_string(), "withdrawals".to_string()],
        guardians: vec![ADDR_A.to_string(), ADDR_B.to_string()],
        governance: ADDR_C.to_string(),
        max_pause_seconds: 604_800,
        unpause_delay_seconds: 86_400,
        jurisdiction: Jurisdiction::Us,
    }
}

fn pause_minimal_config() -> EmergencyPauseConfig {
    EmergencyPauseConfig {
        name: "SimplePause".to_string(),
        scopes: vec![],
        guardians: vec![ADDR_A.to_string()],
        governance: ADDR_C.to_string(),
        max_pause_seconds: 0,
        unpause_delay_seconds: 0,
        jurisdiction: Jurisdiction::Us,
    }
}

// --- Domain math: quorum -------------------------------------------------------

#[test]
fn test_quorum_votes_floors() {
    assert_eq!(quorum_votes(1_000_000, 400), 40_000);
    assert_eq!(quorum_votes(1005, 100), 10); // 1005 * 100 / 10000 = 10 (floored)
}

#[test]
fn test_quorum_votes_guards_overflow() {
    assert_eq!(quorum_votes(u128::MAX, 10_000), 0);
}

// --- Domain math: upgrade governance validation --------------------------------

#[test]
fn test_validate_upgrade_governance_rejects_zero_period() {
    let mut config = upgrade_config();
    config.voting_period_blocks = 0;
    assert!(validate_upgrade_governance(&config).is_err());
}

#[test]
fn test_validate_upgrade_governance_rejects_excess_quorum() {
    let mut config = upgrade_config();
    config.quorum_basis_points = 10_001;
    assert!(validate_upgrade_governance(&config).is_err());
}

#[test]
fn test_validate_upgrade_governance_rejects_empty_proxy() {
    let mut config = upgrade_config();
    config.proxy = "   ".to_string();
    assert!(validate_upgrade_governance(&config).is_err());
}

// --- Domain math: feature flags ------------------------------------------------

#[test]
fn test_validate_feature_flags_rejects_empty_and_duplicate() {
    assert!(validate_feature_flags(&[]).is_err());
    let dupe = [
        FeatureFlag {
            key: "x".to_string(),
            description: String::new(),
            default_enabled: false,
        },
        FeatureFlag {
            key: "x".to_string(),
            description: String::new(),
            default_enabled: true,
        },
    ];
    assert!(validate_feature_flags(&dupe).is_err());
}

#[test]
fn test_flag_and_scope_constant_name_sanitization() {
    assert_eq!(flag_constant_name("beta_checkout"), "FLAG_BETA_CHECKOUT");
    assert_eq!(flag_constant_name("beta-checkout!"), "FLAG_BETA_CHECKOUT_");
    assert_eq!(scope_constant_name("trading"), "SCOPE_TRADING");
    // A digit-leading scope gets a safety prefix so the identifier stays valid.
    assert_eq!(scope_constant_name("123trading"), "SCOPE_K123TRADING");
}

// --- Domain math: A/B variants -------------------------------------------------

#[test]
fn test_validate_ab_variants_rejects_bad_sets() {
    // fewer than two variants
    let single = [AbVariant {
        label: "only".to_string(),
        weight_basis_points: 10_000,
        implementation: None,
    }];
    assert!(validate_ab_variants(&single).is_err());
    // weights that do not sum to 10000
    let mut wrong_sum = ab_config().variants;
    wrong_sum[0].weight_basis_points = 1000;
    assert!(validate_ab_variants(&wrong_sum).is_err());
    // duplicate labels
    let mut dupe = ab_config().variants;
    dupe[1].label = "control".to_string();
    assert!(validate_ab_variants(&dupe).is_err());
}

#[test]
fn test_cumulative_thresholds_are_running_sums() {
    let thresholds = cumulative_thresholds(&ab_config().variants);
    assert_eq!(thresholds, vec![3000, 6000, 10_000]);
}

#[test]
fn test_assign_variant_boundaries_match_on_chain_chain() {
    let variants = ab_config().variants;
    // control owns [0, 3000), variant_a [3000, 6000), variant_b [6000, 10000).
    assert_eq!(assign_variant(0, &variants).expect("ok"), 0);
    assert_eq!(assign_variant(2999, &variants).expect("ok"), 0);
    assert_eq!(assign_variant(3000, &variants).expect("ok"), 1);
    assert_eq!(assign_variant(5999, &variants).expect("ok"), 1);
    assert_eq!(assign_variant(6000, &variants).expect("ok"), 2);
    assert_eq!(assign_variant(9999, &variants).expect("ok"), 2);
}

#[test]
fn test_assign_variant_rejects_out_of_range_bucket() {
    assert!(assign_variant(10_000, &ab_config().variants).is_err());
}

// --- Domain math: gradual rollout ----------------------------------------------

#[test]
fn test_rollout_basis_points_at_linear_progression() {
    // start 1000, step 1000 every 86_400s, target 5000.
    assert_eq!(rollout_basis_points_at(1000, 1000, 5000, 86_400, 0), 1000);
    assert_eq!(
        rollout_basis_points_at(1000, 1000, 5000, 86_400, 86_400),
        2000
    );
    assert_eq!(
        rollout_basis_points_at(1000, 1000, 5000, 86_400, 3 * 86_400),
        4000
    );
}

#[test]
fn test_rollout_basis_points_at_caps_at_target_and_immediate() {
    // Far in the future is capped at the target.
    assert_eq!(
        rollout_basis_points_at(1000, 1000, 5000, 86_400, 1_000 * 86_400),
        5000
    );
    // A zero interval means an immediate jump to the target.
    assert_eq!(rollout_basis_points_at(0, 0, 10_000, 0, 0), 10_000);
}

#[test]
fn test_is_in_rollout_bucket() {
    assert!(is_in_rollout_bucket(0, 1));
    assert!(is_in_rollout_bucket(4999, 5000));
    assert!(!is_in_rollout_bucket(5000, 5000));
}

#[test]
fn test_validate_gradual_rollout_rejects_inconsistent_config() {
    let mut start_gt_target = rollout_linear_config();
    start_gt_target.start_basis_points = 6000;
    assert!(validate_gradual_rollout(&start_gt_target).is_err());

    let mut zero_step = rollout_linear_config();
    zero_step.step_basis_points = 0;
    assert!(validate_gradual_rollout(&zero_step).is_err());

    let mut over_full = rollout_linear_config();
    over_full.target_basis_points = 10_001;
    assert!(validate_gradual_rollout(&over_full).is_err());
}

// --- Domain math: emergency pause ----------------------------------------------

#[test]
fn test_validate_emergency_pause_rejects_bad_config() {
    let mut no_guardian = pause_full_config();
    no_guardian.guardians.clear();
    assert!(validate_emergency_pause(&no_guardian).is_err());

    let mut dupe_guardian = pause_full_config();
    dupe_guardian.guardians = vec![ADDR_A.to_string(), ADDR_A.to_string()];
    assert!(validate_emergency_pause(&dupe_guardian).is_err());

    let mut empty_gov = pause_full_config();
    empty_gov.governance = String::new();
    assert!(validate_emergency_pause(&empty_gov).is_err());

    let mut dupe_scope = pause_full_config();
    dupe_scope.scopes = vec!["a".to_string(), "a".to_string()];
    assert!(validate_emergency_pause(&dupe_scope).is_err());
}

// --- Upgrade governance generator ----------------------------------------------

#[test]
fn test_upgrade_governance_uups_structure() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_upgrade_governance(&upgrade_config())
        .expect("upgrade governance");
    let src = &contract.source;
    assert_eq!(contract.name, "ProtocolUpgradeGovernor");
    assert!(src.contains("contract ProtocolUpgradeGovernor is ReentrancyGuard"));
    assert!(src.contains("interface IUUPSProxy"));
    assert!(!src.contains("interface IProxyAdmin"));
    assert!(src.contains("uint256 public constant QUORUM_BPS = 400;"));
    assert!(src.contains(
        "function proposeUpgrade(address newImplementation, bytes calldata initCalldata)"
    ));
    assert!(src.contains("function executeUpgrade(uint256 id) external nonReentrant"));
    assert!(src.contains("p.executed = true; // CEI: effects before interaction"));
    assert!(src.contains(
        "IUUPSProxy(TARGET_PROXY).upgradeToAndCall(p.newImplementation, p.initCalldata);"
    ));
    // Snapshot-based weight guards against flash-loan voting.
    assert!(src.contains("governanceToken.getPastVotes(msg.sender, p.snapshotBlock)"));
    assert!(src.contains("require(block.timestamp >= p.eta, \"Gov: timelock active\");"));
}

#[test]
fn test_upgrade_governance_transparent_uses_proxy_admin() {
    let mut config = upgrade_config();
    config.proxy_kind = ProxyKind::Transparent;
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_upgrade_governance(&config)
        .expect("upgrade governance");
    let src = &contract.source;
    assert!(src.contains("interface IProxyAdmin"));
    assert!(!src.contains("interface IUUPSProxy"));
    assert!(src.contains("constructor(address proxyAdmin_)"));
    assert!(
        src.contains(
            "proxyAdmin.upgradeAndCall(TARGET_PROXY, p.newImplementation, p.initCalldata);"
        )
    );
}

// --- Feature flags generator ---------------------------------------------------

#[test]
fn test_feature_flags_owner_model_structure() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_feature_flags(&flag_config())
        .expect("feature flags");
    let src = &contract.source;
    assert!(src.contains("contract FeatureRegistry is Ownable2Step"));
    assert!(src.contains(
        "bytes32 public constant FLAG_BETA_CHECKOUT = keccak256(bytes(\"beta_checkout\"));"
    ));
    assert!(src.contains(
        "bytes32 public constant FLAG_NEW_DASHBOARD = keccak256(bytes(\"new_dashboard\"));"
    ));
    assert!(src.contains("function setFlag(bytes32 key, bool enabled) external onlyOwner"));
    // default_enabled flag set in constructor; the other is not.
    assert!(src.contains("_enabled[FLAG_BETA_CHECKOUT] = true;"));
    assert!(!src.contains("_enabled[FLAG_NEW_DASHBOARD] = true;"));
    // overrides + kill switch enabled.
    assert!(src.contains("function isEnabledFor(bytes32 key, address account)"));
    assert!(src.contains("bool public killSwitch;"));
    assert!(src.contains("if (killSwitch) {"));
}

#[test]
fn test_feature_flags_roles_model_without_overrides() {
    let mut config = flag_config();
    config.admin_model = FlagAdminModel::Roles;
    config.per_address_overrides = false;
    config.global_kill_switch = false;
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_feature_flags(&config)
        .expect("feature flags");
    let src = &contract.source;
    assert!(src.contains("contract FeatureRegistry is AccessControl"));
    assert!(
        src.contains("bytes32 public constant FLAG_ADMIN_ROLE = keccak256(\"FLAG_ADMIN_ROLE\");")
    );
    assert!(src.contains(
        "function setFlag(bytes32 key, bool enabled) external onlyRole(FLAG_ADMIN_ROLE)"
    ));
    assert!(src.contains("_grantRole(DEFAULT_ADMIN_ROLE, msg.sender);"));
    assert!(!src.contains("function isEnabledFor"));
    assert!(!src.contains("bool public killSwitch;"));
}

#[test]
fn test_feature_flags_rejects_empty() {
    let mut config = flag_config();
    config.flags.clear();
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    assert!(generator.generate_feature_flags(&config).is_err());
}

// --- A/B test generator --------------------------------------------------------

#[test]
fn test_ab_test_deterministic_unrolled_chain() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let config = ab_config();
    let contract = generator.generate_ab_test(&config).expect("ab test");
    let src = &contract.source;
    assert!(src.contains("contract CheckoutExperiment is Ownable2Step"));
    assert!(
        src.contains(
            "bytes32 public constant EXPERIMENT_KEY = keccak256(bytes(\"checkout_flow\"));"
        )
    );
    assert!(src.contains("uint256 public constant VARIANT_COUNT = 3;"));
    // The unrolled comparison chain must match cumulative_thresholds exactly.
    assert!(src.contains("if (bucket < 3000) {"));
    assert!(src.contains("if (bucket < 6000) {"));
    assert!(src.contains("        return 2;")); // final variant owns the rest
    assert!(!src.contains("if (bucket < 10000)"));
    assert!(src.contains("function assignVariant(address user) public view returns (uint256)"));
    // Deterministic mode: no sticky storage, no metrics.
    assert!(!src.contains("mapping(address => uint256) private _sticky;"));
    assert!(!src.contains("function recordConversion"));
    // Off-chain parity with the chain emitted above.
    for (bucket, expected) in [(0u32, 0usize), (3000, 1), (6000, 2), (9999, 2)] {
        assert_eq!(
            assign_variant(bucket, &config.variants).expect("ok"),
            expected
        );
    }
}

#[test]
fn test_ab_test_sticky_with_metrics_and_routing() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_ab_test(&ab_sticky_config())
        .expect("ab test");
    let src = &contract.source;
    assert!(src.contains("mapping(address => uint256) private _sticky;"));
    assert!(src.contains("function commitAssignment() external returns (uint256)"));
    assert!(src.contains("_sticky[msg.sender] = variant + 1;"));
    assert!(src.contains("uint256[2] public exposures;"));
    assert!(src.contains("uint256[2] public conversions;"));
    assert!(src.contains("function recordConversion(address user) external onlyOwner"));
    // Routing present because variants carry implementation addresses.
    assert!(src.contains(&format!("_implementations[0] = {ADDR_C};")));
    assert!(src.contains(&format!("_implementations[1] = {ADDR_D};")));
    assert!(src.contains("function route(address user) external view returns (address)"));
}

// --- Gradual rollout generator -------------------------------------------------

#[test]
fn test_gradual_rollout_linear_schedule() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_gradual_rollout(&rollout_linear_config())
        .expect("gradual rollout");
    let src = &contract.source;
    assert!(src.contains("contract FeatureRollout is Ownable2Step"));
    assert!(src.contains("uint256 public constant START_BPS = 1000;"));
    assert!(src.contains("uint256 public constant TARGET_BPS = 5000;"));
    assert!(src.contains("uint256 steps = (block.timestamp - startTime) / STEP_INTERVAL;"));
    assert!(src.contains("uint256 value = START_BPS + steps * STEP_BPS;"));
    assert!(src.contains("return value > TARGET_BPS ? TARGET_BPS : value;"));
    // The worked schedule NatSpec must agree with rollout_basis_points_at.
    assert!(src.contains("t = 0s: 1000 bps"));
    assert!(src.contains("t = 259200s: 4000 bps"));
    // Linear (not manual / not canary).
    assert!(!src.contains("uint256 public manualBps;"));
    assert!(!src.contains("mapping(address => bool) public allowlist;"));
    assert!(src.contains("return bucketOf(user) < currentBps();"));
}

#[test]
fn test_gradual_rollout_canary_allowlist_and_guardian() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_gradual_rollout(&rollout_canary_config())
        .expect("gradual rollout");
    let src = &contract.source;
    assert!(src.contains("mapping(address => bool) public allowlist;"));
    assert!(
        src.contains("function setAllowlist(address account, bool allowed) external onlyOwner")
    );
    assert!(src.contains("if (allowlist[user]) {"));
    assert!(src.contains("address public immutable guardian;"));
    assert!(src.contains("constructor(address guardian_) Ownable(msg.sender)"));
    assert!(src.contains("function rollback() external onlyGuardian"));
    // Canary still uses the time-based percentage path.
    assert!(src.contains("uint256 steps = (block.timestamp - startTime) / STEP_INTERVAL;"));
}

// --- Emergency pause generator -------------------------------------------------

#[test]
fn test_emergency_pause_auto_expiry_and_cooldown() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_emergency_pause(&pause_full_config())
        .expect("emergency pause");
    let src = &contract.source;
    assert!(src.contains("contract EmergencyController {"));
    assert!(src.contains("bytes32 public constant SCOPE_TRADING = keccak256(bytes(\"trading\"));"));
    assert!(src.contains(
        "bytes32 public constant SCOPE_WITHDRAWALS = keccak256(bytes(\"withdrawals\"));"
    ));
    assert!(src.contains("knownScope[SCOPE_TRADING] = true;"));
    // Guardians pause, governance unpauses.
    assert!(src.contains("function pause(bytes32 scope) external onlyGuardian"));
    assert!(src.contains("function unpause(bytes32 scope) external onlyGovernance"));
    // Auto-expiry on pause + governance-only extension.
    assert!(src.contains("info.expiry = block.timestamp + MAX_PAUSE_DURATION;"));
    assert!(src.contains("function extendPause(bytes32 scope) external onlyGovernance"));
    assert!(src.contains("if (info.expiry != 0 && block.timestamp >= info.expiry) {"));
    // Cool-down protected unpause.
    assert!(src.contains("function requestUnpause(bytes32 scope) external onlyGovernance"));
    assert!(src.contains("require(info.unpauseEta != 0 && block.timestamp >= info.unpauseEta, \"Pause: cool-down active\");"));
}

#[test]
fn test_emergency_pause_global_scope_no_cooldown() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_emergency_pause(&pause_minimal_config())
        .expect("emergency pause");
    let src = &contract.source;
    // Empty scope list collapses to a single global scope.
    assert!(src.contains("bytes32 public constant SCOPE_GLOBAL = keccak256(bytes(\"GLOBAL\"));"));
    // No auto-expiry: pause sets an indefinite expiry.
    assert!(src.contains("info.expiry = 0;"));
    assert!(!src.contains("function extendPause"));
    // No cool-down: direct unpause guarded only by pausedAt.
    assert!(!src.contains("function requestUnpause"));
    assert!(src.contains("require(info.pausedAt != 0, \"Pause: not paused\");"));
}

// --- Multi-target composition --------------------------------------------------

#[test]
fn test_evm_l2_target_is_supported_and_preserved() {
    let generator = ContractGenerator::new(TargetPlatform::Base);
    let contract = generator
        .generate_feature_flags(&flag_config())
        .expect("base generation");
    assert_eq!(contract.platform, TargetPlatform::Base);
    assert!(contract.source.contains("pragma solidity"));
}

#[test]
fn test_non_evm_targets_are_rejected() {
    for platform in [
        TargetPlatform::Move,
        TargetPlatform::Cairo,
        TargetPlatform::Solana,
    ] {
        let generator = ContractGenerator::new(platform);
        assert!(
            generator
                .generate_upgrade_governance(&upgrade_config())
                .is_err()
        );
        assert!(generator.generate_feature_flags(&flag_config()).is_err());
        assert!(generator.generate_ab_test(&ab_config()).is_err());
        assert!(
            generator
                .generate_gradual_rollout(&rollout_linear_config())
                .is_err()
        );
        assert!(
            generator
                .generate_emergency_pause(&pause_full_config())
                .is_err()
        );
    }
}
