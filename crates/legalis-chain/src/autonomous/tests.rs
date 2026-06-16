//! Tests for the autonomous contract-management generators and control math.

use crate::{
    AutoOptimizerConfig, ContractGenerator, ControlSense, CostOptimizerConfig, HealthInvariant,
    HealthState, Jurisdiction, PerformanceMonitorConfig, ResourceManagerConfig, SelfHealingConfig,
    TargetPlatform, adjust_parameter, batch_savings, can_consume, clamp_value, classify_in_band,
    ema_update, epoch_index, health_score, invariant_constant_name, next_health_state,
    operation_constant_name, should_defer, token_bucket_available, validate_auto_optimizer,
    validate_cost_optimizer, validate_performance_monitor, validate_resource_manager,
    validate_self_healing,
};

// --- Config builders -----------------------------------------------------------

fn self_healing_config() -> SelfHealingConfig {
    SelfHealingConfig {
        name: "SelfHealingVault".to_string(),
        invariants: vec![
            HealthInvariant {
                key: "collateral_ratio_bps".to_string(),
                description: "Collateral ratio".to_string(),
                min_value: 11_000,
                max_value: 1_000_000,
            },
            HealthInvariant {
                key: "oracle_age_secs".to_string(),
                description: "Oracle freshness".to_string(),
                min_value: 0,
                max_value: 3600,
            },
        ],
        checkpoint_enabled: true,
        recover_after_seconds: 3600,
        keeper_reward_wei: 1_000_000_000_000_000,
        jurisdiction: Jurisdiction::Us,
    }
}

fn self_healing_minimal() -> SelfHealingConfig {
    SelfHealingConfig {
        name: "MinimalHealer".to_string(),
        invariants: vec![HealthInvariant {
            key: "tvl".to_string(),
            description: "Total value locked".to_string(),
            min_value: 1,
            max_value: u128::MAX,
        }],
        checkpoint_enabled: false,
        recover_after_seconds: 0,
        keeper_reward_wei: 0,
        jurisdiction: Jurisdiction::Eu,
    }
}

fn auto_optimizer_config() -> AutoOptimizerConfig {
    AutoOptimizerConfig {
        name: "FeeController".to_string(),
        parameter_label: "feeBps".to_string(),
        initial_value: 100,
        min_value: 0,
        max_value: 10_000,
        metric_label: "utilizationBps".to_string(),
        target_value: 5000,
        step_value: 10,
        sense: ControlSense::Direct,
        cooldown_seconds: 3600,
        jurisdiction: Jurisdiction::Us,
    }
}

fn resource_manager_config() -> ResourceManagerConfig {
    ResourceManagerConfig {
        name: "ApiRateLimiter".to_string(),
        bucket_capacity: 100,
        refill_per_second: 1,
        cost_per_op: 10,
        epoch_seconds: 3600,
        epoch_budget: 1000,
        per_caller: true,
        jurisdiction: Jurisdiction::Sg,
    }
}

fn performance_monitor_config() -> PerformanceMonitorConfig {
    PerformanceMonitorConfig {
        name: "OpsMonitor".to_string(),
        operations: vec!["settle".to_string(), "claim".to_string()],
        ema_alpha_bps: 2000,
        emit_events: true,
        health_score: true,
        gas_budget_per_op: 120_000,
        jurisdiction: Jurisdiction::Eu,
    }
}

fn cost_optimizer_config() -> CostOptimizerConfig {
    CostOptimizerConfig {
        name: "CostSaver".to_string(),
        max_batch_size: 50,
        base_tx_gas: 21_000,
        basefee_ceiling_wei: 50_000_000_000,
        enable_refunds: true,
        jurisdiction: Jurisdiction::Us,
    }
}

// --- Domain math: health classification & state machine ------------------------

#[test]
fn test_classify_in_band_boundaries() {
    assert!(classify_in_band(50, 10, 100));
    assert!(classify_in_band(10, 10, 100)); // inclusive lower
    assert!(classify_in_band(100, 10, 100)); // inclusive upper
    assert!(!classify_in_band(9, 10, 100));
    assert!(!classify_in_band(101, 10, 100));
}

#[test]
fn test_health_state_encoding_and_operational() {
    assert_eq!(HealthState::Healthy.as_u8(), 0);
    assert_eq!(HealthState::Degraded.as_u8(), 1);
    assert_eq!(HealthState::Recovering.as_u8(), 2);
    assert!(HealthState::Healthy.is_operational());
    assert!(!HealthState::Degraded.is_operational());
    assert!(!HealthState::Recovering.is_operational());
}

#[test]
fn test_next_health_state_full_recovery_cycle() {
    // Healthy stays healthy in band, trips to Degraded out of band.
    assert_eq!(
        next_health_state(HealthState::Healthy, true, 0, 100),
        HealthState::Healthy
    );
    assert_eq!(
        next_health_state(HealthState::Healthy, false, 0, 100),
        HealthState::Degraded
    );
    // Degraded -> Recovering once the metric returns to band.
    assert_eq!(
        next_health_state(HealthState::Degraded, true, 0, 100),
        HealthState::Recovering
    );
    // Recovering holds until the cool-down, then auto-resumes to Healthy.
    assert_eq!(
        next_health_state(HealthState::Recovering, true, 99, 100),
        HealthState::Recovering
    );
    assert_eq!(
        next_health_state(HealthState::Recovering, true, 100, 100),
        HealthState::Healthy
    );
}

#[test]
fn test_next_health_state_regression_and_hold() {
    // Degraded stays Degraded while out of band, regardless of elapsed time.
    assert_eq!(
        next_health_state(HealthState::Degraded, false, 9999, 100),
        HealthState::Degraded
    );
    // A relapse during Recovering drops back to Degraded.
    assert_eq!(
        next_health_state(HealthState::Recovering, false, 200, 100),
        HealthState::Degraded
    );
}

// --- Domain math: controller ---------------------------------------------------

#[test]
fn test_adjust_parameter_direct_sense() {
    // observed above target raises the parameter by one step.
    assert_eq!(
        adjust_parameter(100, 80, 50, 10, 0, 200, ControlSense::Direct),
        110
    );
    // observed below target lowers it by one step.
    assert_eq!(
        adjust_parameter(100, 20, 50, 10, 0, 200, ControlSense::Direct),
        90
    );
    // raise is clamped to the maximum.
    assert_eq!(
        adjust_parameter(195, 80, 50, 10, 0, 200, ControlSense::Direct),
        200
    );
    // lower saturates at the minimum (no underflow).
    assert_eq!(
        adjust_parameter(5, 20, 50, 10, 0, 200, ControlSense::Direct),
        0
    );
}

#[test]
fn test_adjust_parameter_inverse_sense() {
    // Inverse flips the response: above target lowers, below target raises.
    assert_eq!(
        adjust_parameter(100, 80, 50, 10, 0, 200, ControlSense::Inverse),
        90
    );
    assert_eq!(
        adjust_parameter(100, 20, 50, 10, 0, 200, ControlSense::Inverse),
        110
    );
}

#[test]
fn test_adjust_parameter_holds_and_clamp_value() {
    // At the set-point the parameter holds.
    assert_eq!(
        adjust_parameter(100, 50, 50, 10, 0, 200, ControlSense::Direct),
        100
    );
    // clamp_value tolerates an inverted band by returning the input unchanged.
    assert_eq!(clamp_value(123, 200, 100), 123);
    assert_eq!(clamp_value(50, 0, 40), 40);
    assert_eq!(clamp_value(50, 60, 100), 60);
}

// --- Domain math: resource management ------------------------------------------

#[test]
fn test_token_bucket_available_refills_and_caps() {
    // 10s at 5 tokens/s from empty = 50.
    assert_eq!(token_bucket_available(0, 0, 10, 100, 5), 50);
    // Refill is capped at capacity.
    assert_eq!(token_bucket_available(80, 0, 10, 100, 5), 100);
    // A clock that has not advanced (or went backwards) adds nothing.
    assert_eq!(token_bucket_available(30, 100, 50, 100, 5), 30);
}

#[test]
fn test_can_consume_and_epoch_index() {
    assert!(can_consume(10, 10));
    assert!(!can_consume(9, 10));
    assert_eq!(epoch_index(0, 0, 100), 0);
    assert_eq!(epoch_index(250, 0, 100), 2);
    assert_eq!(epoch_index(50, 100, 100), 0); // now before start saturates
    assert_eq!(epoch_index(999, 0, 0), 0); // zero-length epoch collapses to 0
}

// --- Domain math: monitoring ---------------------------------------------------

#[test]
fn test_ema_update_parity_and_alpha_clamp() {
    assert_eq!(ema_update(0, 100, 5000), 50);
    assert_eq!(ema_update(100, 100, 5000), 100);
    assert_eq!(ema_update(40, 100, 10_000), 100); // alpha 100% == sample
    assert_eq!(ema_update(40, 100, 20_000), 100); // alpha clamped to 100%
    assert_eq!(ema_update(50, 100, 2000), 60); // 0.2*100 + 0.8*50
}

#[test]
fn test_health_score_degrades_past_budget() {
    assert_eq!(health_score(0, 100), 10_000); // no load is perfect
    assert_eq!(health_score(100, 100), 10_000); // within budget is perfect
    assert_eq!(health_score(80, 100), 10_000);
    assert_eq!(health_score(200, 100), 5000); // double the budget halves the score
    assert_eq!(health_score(400, 100), 2500);
}

// --- Domain math: cost ---------------------------------------------------------

#[test]
fn test_batch_savings_and_should_defer() {
    assert_eq!(batch_savings(0, 21_000), 0);
    assert_eq!(batch_savings(1, 21_000), 0);
    assert_eq!(batch_savings(5, 21_000), 84_000); // (5-1)*21000
    assert!(!should_defer(100, 0)); // ceiling 0 disables the guard
    assert!(should_defer(150, 100));
    assert!(!should_defer(100, 100)); // strictly greater only
    assert!(!should_defer(50, 100));
}

#[test]
fn test_constant_name_helpers() {
    assert_eq!(
        invariant_constant_name("collateral_ratio_bps"),
        "INV_COLLATERAL_RATIO_BPS"
    );
    assert_eq!(invariant_constant_name("ratio-1!"), "INV_RATIO_1_");
    assert_eq!(operation_constant_name("settle"), "OP_SETTLE");
    assert_eq!(operation_constant_name("9lives"), "OP_K9LIVES");
}

// --- Validation ----------------------------------------------------------------

#[test]
fn test_validate_self_healing_rejects_bad_config() {
    assert!(validate_self_healing(&self_healing_config()).is_ok());

    let mut empty = self_healing_config();
    empty.invariants.clear();
    assert!(validate_self_healing(&empty).is_err());

    let mut inverted = self_healing_config();
    inverted.invariants[0].min_value = 100;
    inverted.invariants[0].max_value = 10;
    assert!(validate_self_healing(&inverted).is_err());

    let mut duplicate = self_healing_config();
    duplicate.invariants[1].key = "collateral_ratio_bps".to_string();
    assert!(validate_self_healing(&duplicate).is_err());

    let mut blank = self_healing_config();
    blank.name = "   ".to_string();
    assert!(validate_self_healing(&blank).is_err());
}

#[test]
fn test_validate_auto_optimizer_rejects_bad_config() {
    assert!(validate_auto_optimizer(&auto_optimizer_config()).is_ok());

    let mut bad_bounds = auto_optimizer_config();
    bad_bounds.min_value = 100;
    bad_bounds.max_value = 10;
    assert!(validate_auto_optimizer(&bad_bounds).is_err());

    let mut bad_initial = auto_optimizer_config();
    bad_initial.initial_value = 999_999;
    assert!(validate_auto_optimizer(&bad_initial).is_err());

    let mut zero_step = auto_optimizer_config();
    zero_step.step_value = 0;
    assert!(validate_auto_optimizer(&zero_step).is_err());

    let mut blank_label = auto_optimizer_config();
    blank_label.parameter_label = String::new();
    assert!(validate_auto_optimizer(&blank_label).is_err());
}

#[test]
fn test_validate_resource_manager_rejects_bad_config() {
    assert!(validate_resource_manager(&resource_manager_config()).is_ok());

    let mut zero_capacity = resource_manager_config();
    zero_capacity.bucket_capacity = 0;
    assert!(validate_resource_manager(&zero_capacity).is_err());

    let mut zero_cost = resource_manager_config();
    zero_cost.cost_per_op = 0;
    assert!(validate_resource_manager(&zero_cost).is_err());

    let mut cost_too_high = resource_manager_config();
    cost_too_high.cost_per_op = 200; // > capacity
    assert!(validate_resource_manager(&cost_too_high).is_err());
}

#[test]
fn test_validate_performance_monitor_rejects_bad_config() {
    assert!(validate_performance_monitor(&performance_monitor_config()).is_ok());

    let mut empty = performance_monitor_config();
    empty.operations.clear();
    assert!(validate_performance_monitor(&empty).is_err());

    let mut duplicate = performance_monitor_config();
    duplicate.operations = vec!["x".to_string(), "x".to_string()];
    assert!(validate_performance_monitor(&duplicate).is_err());

    let mut zero_alpha = performance_monitor_config();
    zero_alpha.ema_alpha_bps = 0;
    assert!(validate_performance_monitor(&zero_alpha).is_err());

    let mut excess_alpha = performance_monitor_config();
    excess_alpha.ema_alpha_bps = 10_001;
    assert!(validate_performance_monitor(&excess_alpha).is_err());
}

#[test]
fn test_validate_cost_optimizer_rejects_bad_config() {
    assert!(validate_cost_optimizer(&cost_optimizer_config()).is_ok());

    let mut zero_batch = cost_optimizer_config();
    zero_batch.max_batch_size = 0;
    assert!(validate_cost_optimizer(&zero_batch).is_err());

    let mut blank = cost_optimizer_config();
    blank.name = String::new();
    assert!(validate_cost_optimizer(&blank).is_err());
}

// --- Self-healing generator ----------------------------------------------------

#[test]
fn test_self_healing_checkpoint_and_reward_structure() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_self_healing(&self_healing_config())
        .expect("self healing");
    let src = &contract.source;
    assert_eq!(contract.name, "SelfHealingVault");
    assert!(src.contains("contract SelfHealingVault is Ownable2Step, ReentrancyGuard {"));
    assert!(src.contains("import \"@openzeppelin/contracts/utils/ReentrancyGuard.sol\";"));
    assert!(src.contains(
        "bytes32 public constant INV_COLLATERAL_RATIO_BPS = keccak256(bytes(\"collateral_ratio_bps\"));"
    ));
    assert!(src.contains("uint256 public constant RECOVER_AFTER = 3600 seconds;"));
    assert!(src.contains("uint256 public constant KEEPER_REWARD = 1000000000000000 wei;"));
    assert!(src.contains("band[INV_COLLATERAL_RATIO_BPS] = Band(11000, 1000000, true);"));
    // Reentrancy-guarded, checkpoint-restoring report path.
    assert!(
        src.contains("function reportHealth(bytes32 key, uint256 value) external nonReentrant {")
    );
    assert!(src.contains("_restoreCheckpoint();"));
    assert!(src.contains("function setParam(bytes32 key, uint256 value) external onlyOwner {"));
    // Keeper reward + funding sink.
    assert!(src.contains("payable(msg.sender).call{value: KEEPER_REWARD}(\"\")"));
    assert!(src.contains("receive() external payable {}"));
    // FSM transition matches next_health_state.
    assert!(src.contains("return elapsed >= RECOVER_AFTER ? HEALTHY : RECOVERING;"));
    assert!(src.contains("return inBand ? RECOVERING : DEGRADED;"));
}

#[test]
fn test_self_healing_minimal_structure() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_self_healing(&self_healing_minimal())
        .expect("self healing");
    let src = &contract.source;
    assert!(src.contains("contract MinimalHealer is Ownable2Step {"));
    // No reentrancy guard, no checkpoint machinery, no reward sink.
    assert!(!src.contains("ReentrancyGuard"));
    assert!(!src.contains("_restoreCheckpoint"));
    assert!(!src.contains("function setParam"));
    assert!(!src.contains("receive() external payable"));
    assert!(src.contains("function reportHealth(bytes32 key, uint256 value) external {"));
}

// --- Auto-optimizer generator --------------------------------------------------

#[test]
fn test_auto_optimizer_direct_and_inverse() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let direct = generator
        .generate_auto_optimizer(&auto_optimizer_config())
        .expect("auto optimizer");
    let dsrc = &direct.source;
    assert!(dsrc.contains("contract FeeController is Ownable2Step {"));
    assert!(dsrc.contains("uint256 public constant TARGET = 5000;"));
    assert!(dsrc.contains("uint256 public constant STEP = 10;"));
    assert!(
        dsrc.contains("function observe(uint256 metric) external onlyReporter returns (uint256) {")
    );
    assert!(dsrc.contains("function setReporter(address newReporter) external onlyOwner {"));
    // Direct sense: above target raises the parameter.
    assert!(dsrc.contains(
        "        if (observed > TARGET) {\n            next = current + STEP;\n        } else if (observed < TARGET) {\n"
    ));

    let mut inverse_config = auto_optimizer_config();
    inverse_config.sense = ControlSense::Inverse;
    let inverse = generator
        .generate_auto_optimizer(&inverse_config)
        .expect("auto optimizer");
    // Inverse sense: above target lowers the parameter.
    assert!(inverse.source.contains(
        "        if (observed > TARGET) {\n            next = current >= STEP ? current - STEP : 0;\n        } else if (observed < TARGET) {\n"
    ));
}

// --- Resource-manager generator ------------------------------------------------

#[test]
fn test_resource_manager_per_caller_and_global() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let per_caller = generator
        .generate_resource_manager(&resource_manager_config())
        .expect("resource manager");
    let psrc = &per_caller.source;
    assert!(psrc.contains("mapping(address => Bucket) private _buckets;"));
    assert!(psrc.contains("mapping(uint256 => uint256) public epochSpent;"));
    assert!(psrc.contains("function available(address caller) public view returns (uint256) {"));
    assert!(psrc.contains("Bucket storage b = _buckets[caller];"));
    assert!(psrc.contains(
        "require(epochSpent[epoch] + COST_PER_OP <= EPOCH_BUDGET, \"RL: epoch budget exhausted\");"
    ));
    assert!(psrc.contains("uint256 avail = available(caller);"));

    let mut global_config = resource_manager_config();
    global_config.per_caller = false;
    global_config.epoch_budget = 0; // disable the budget
    let global = generator
        .generate_resource_manager(&global_config)
        .expect("resource manager");
    let gsrc = &global.source;
    assert!(gsrc.contains("Bucket private _global;"));
    assert!(gsrc.contains("function available() public view returns (uint256) {"));
    assert!(gsrc.contains("uint256 avail = available();"));
    assert!(!gsrc.contains("EPOCH_BUDGET"));
    assert!(!gsrc.contains("epochSpent"));
}

// --- Performance-monitor generator ---------------------------------------------

#[test]
fn test_performance_monitor_structure() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_performance_monitor(&performance_monitor_config())
        .expect("performance monitor");
    let src = &contract.source;
    assert!(src.contains("bytes32 public constant OP_SETTLE = keccak256(bytes(\"settle\"));"));
    assert!(src.contains("uint256 public constant EMA_ALPHA_BPS = 2000;"));
    assert!(src.contains("uint256 public constant GAS_BUDGET = 120000;"));
    assert!(src.contains("metrics[OP_SETTLE].known = true;"));
    // Self-instrumenting modifier (autonomous gas measurement).
    assert!(src.contains("modifier measured(bytes32 op) {"));
    assert!(src.contains("_record(op, gasStart - gasleft());"));
    assert!(src.contains("function record(bytes32 op, uint256 gasUsed) external onlyReporter {"));
    assert!(src.contains("event Recorded(bytes32 indexed op, uint256 gasUsed, uint256 emaGas);"));
    // EMA + health score mirror the domain math.
    assert!(src.contains(
        "return (EMA_ALPHA_BPS * sample + (BPS_DENOMINATOR - EMA_ALPHA_BPS) * prev) / BPS_DENOMINATOR;"
    ));
    assert!(src.contains("function healthScore(bytes32 op) external view returns (uint256) {"));

    // Without events/health the corresponding members disappear.
    let mut lean = performance_monitor_config();
    lean.emit_events = false;
    lean.health_score = false;
    let lean_src = generator
        .generate_performance_monitor(&lean)
        .expect("performance monitor")
        .source;
    assert!(!lean_src.contains("event Recorded("));
    assert!(!lean_src.contains("function healthScore"));
    assert!(!lean_src.contains("GAS_BUDGET"));
}

// --- Cost-optimizer generator --------------------------------------------------

#[test]
fn test_cost_optimizer_full_and_minimal() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let full = generator
        .generate_cost_optimizer(&cost_optimizer_config())
        .expect("cost optimizer");
    let fsrc = &full.source;
    assert!(fsrc.contains("contract CostSaver is Ownable2Step, ReentrancyGuard {"));
    assert!(fsrc.contains("uint256 public constant BASEFEE_CEILING = 50000000000 wei;"));
    assert!(fsrc.contains("function executeBatch(Call[] calldata calls, bool urgent)"));
    assert!(fsrc.contains("if (!urgent && _shouldDefer()) {"));
    assert!(fsrc.contains("return BASEFEE_CEILING != 0 && block.basefee > BASEFEE_CEILING;"));
    assert!(fsrc.contains("mapping(bytes32 => uint256) public scratch;"));
    assert!(fsrc.contains("function clearSlots(bytes32[] calldata slots) external onlyOwner {"));
    assert!(fsrc.contains("delete scratch[slots[i]]; // harvests SSTORE refund"));
    assert!(fsrc.contains("return (numCalls - 1) * BASE_TX_GAS;"));

    let mut minimal = cost_optimizer_config();
    minimal.basefee_ceiling_wei = 0; // disable guard
    minimal.enable_refunds = false;
    let minimal_src = generator
        .generate_cost_optimizer(&minimal)
        .expect("cost optimizer")
        .source;
    assert!(minimal_src.contains("function executeBatch(Call[] calldata calls)\n"));
    assert!(!minimal_src.contains("bool urgent"));
    assert!(!minimal_src.contains("BASEFEE_CEILING"));
    assert!(!minimal_src.contains("_shouldDefer"));
    assert!(!minimal_src.contains("scratch"));
    assert!(!minimal_src.contains("clearSlots"));
}

// --- Multi-target composition --------------------------------------------------

#[test]
fn test_evm_l2_target_is_supported_and_preserved() {
    let generator = ContractGenerator::new(TargetPlatform::Base);
    let contract = generator
        .generate_performance_monitor(&performance_monitor_config())
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
                .generate_self_healing(&self_healing_config())
                .is_err()
        );
        assert!(
            generator
                .generate_auto_optimizer(&auto_optimizer_config())
                .is_err()
        );
        assert!(
            generator
                .generate_resource_manager(&resource_manager_config())
                .is_err()
        );
        assert!(
            generator
                .generate_performance_monitor(&performance_monitor_config())
                .is_err()
        );
        assert!(
            generator
                .generate_cost_optimizer(&cost_optimizer_config())
                .is_err()
        );
    }
}
