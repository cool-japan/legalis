//! Tests for the advanced security detectors.
//!
//! Each detector is exercised with a *vulnerable* fixture (must flag) and a
//! *safe* fixture (must not flag). Several tests additionally assert that the
//! crate's own hardened generators produce no false positives.

use crate::{
    ContractGenerator, FindingCategory, GeneratedContract, HealthInvariant, Jurisdiction,
    SelfHealingConfig, Severity, TargetPlatform, analyze_security, detect_front_running_risks,
    detect_honeypots, detect_rug_pull_risks, detect_runtime_exploits, detect_sandwich_risks,
};

/// Wraps raw Solidity in a [`GeneratedContract`] on the Solidity target.
fn solidity(name: &str, source: &str) -> GeneratedContract {
    GeneratedContract {
        name: name.to_string(),
        source: source.to_string(),
        platform: TargetPlatform::Solidity,
        abi: None,
        deployment_script: None,
    }
}

/// Returns whether any finding carries `rule_id`.
fn has_rule(findings: &[crate::SecurityFinding], rule_id: &str) -> bool {
    findings.iter().any(|finding| finding.rule_id == rule_id)
}

// --- Runtime exploit -----------------------------------------------------------

#[test]
fn test_runtime_tx_origin_flagged() {
    let contract = solidity(
        "TxOrigin",
        "pragma solidity ^0.8.20;\ncontract C {\n  address owner;\n  function withdraw() external {\n    require(tx.origin == owner, \"no\");\n  }\n}\n",
    );
    let findings = detect_runtime_exploits(&contract);
    assert!(has_rule(&findings, "TX_ORIGIN_AUTH"));
    // Severity should be High.
    let finding = findings
        .iter()
        .find(|f| f.rule_id == "TX_ORIGIN_AUTH")
        .expect("present");
    assert_eq!(finding.severity, Severity::High);
    assert_eq!(finding.category, FindingCategory::RuntimeExploit);
    assert!(finding.line.is_some());
}

#[test]
fn test_runtime_tx_origin_safe_uses_msg_sender() {
    let contract = solidity(
        "MsgSender",
        "pragma solidity ^0.8.20;\ncontract C {\n  address owner;\n  function withdraw() external {\n    require(msg.sender == owner, \"no\");\n  }\n}\n",
    );
    let findings = detect_runtime_exploits(&contract);
    assert!(!has_rule(&findings, "TX_ORIGIN_AUTH"));
}

#[test]
fn test_runtime_unguarded_delegatecall_flagged() {
    let contract = solidity(
        "Proxy",
        "pragma solidity ^0.8.20;\ncontract C {\n  function forward(address impl) external {\n    (bool ok, ) = impl.delegatecall(msg.data);\n    require(ok);\n  }\n}\n",
    );
    let findings = detect_runtime_exploits(&contract);
    assert!(has_rule(&findings, "UNGUARDED_DELEGATECALL"));
    let finding = findings
        .iter()
        .find(|f| f.rule_id == "UNGUARDED_DELEGATECALL")
        .expect("present");
    assert_eq!(finding.severity, Severity::Critical);
}

#[test]
fn test_runtime_delegatecall_guarded_is_not_critical() {
    // Immutable impl + access control => no critical unguarded finding.
    let contract = solidity(
        "SafeProxy",
        "pragma solidity ^0.8.20;\ncontract C {\n  address immutable impl;\n  modifier onlyOwner() { _; }\n  function forward() external onlyOwner {\n    (bool ok, ) = impl.delegatecall(msg.data);\n    require(ok);\n  }\n}\n",
    );
    let findings = detect_runtime_exploits(&contract);
    assert!(!has_rule(&findings, "UNGUARDED_DELEGATECALL"));
    assert!(!has_rule(&findings, "DELEGATECALL_REVIEW"));
}

#[test]
fn test_runtime_selfdestruct_uncontrolled_critical() {
    let contract = solidity(
        "Boom",
        "pragma solidity ^0.8.20;\ncontract C {\n  function kill() external {\n    selfdestruct(payable(msg.sender));\n  }\n}\n",
    );
    let findings = detect_runtime_exploits(&contract);
    let finding = findings
        .iter()
        .find(|f| f.rule_id == "SELFDESTRUCT_PRESENT")
        .expect("present");
    assert_eq!(finding.severity, Severity::Critical);
}

#[test]
fn test_runtime_selfdestruct_controlled_medium() {
    let contract = solidity(
        "GuardedKill",
        "pragma solidity ^0.8.20;\ncontract C {\n  modifier onlyOwner() { _; }\n  function kill() external onlyOwner {\n    selfdestruct(payable(msg.sender));\n  }\n}\n",
    );
    let findings = detect_runtime_exploits(&contract);
    let finding = findings
        .iter()
        .find(|f| f.rule_id == "SELFDESTRUCT_PRESENT")
        .expect("present");
    assert_eq!(finding.severity, Severity::Medium);
}

#[test]
fn test_runtime_weak_randomness_flagged() {
    let contract = solidity(
        "Lottery",
        "pragma solidity ^0.8.20;\ncontract C {\n  function pick() external view returns (uint256) {\n    return uint256(keccak256(abi.encodePacked(block.timestamp, msg.sender))) % 100;\n  }\n}\n",
    );
    let findings = detect_runtime_exploits(&contract);
    assert!(has_rule(&findings, "WEAK_ONCHAIN_RANDOMNESS"));
}

#[test]
fn test_runtime_timestamp_without_randomness_not_flagged() {
    // A plain timelock using block.timestamp comparison must NOT be flagged as RNG.
    let contract = solidity(
        "Timelock",
        "pragma solidity ^0.8.20;\ncontract C {\n  uint256 releaseTime;\n  function release() external view returns (bool) {\n    return block.timestamp >= releaseTime;\n  }\n}\n",
    );
    let findings = detect_runtime_exploits(&contract);
    assert!(!has_rule(&findings, "WEAK_ONCHAIN_RANDOMNESS"));
}

#[test]
fn test_runtime_unchecked_call_flagged() {
    let contract = solidity(
        "Unchecked",
        "pragma solidity ^0.8.20;\ncontract C {\n  function pay(address to) external {\n    to.call{value: 1 ether}(\"\");\n  }\n}\n",
    );
    let findings = detect_runtime_exploits(&contract);
    assert!(has_rule(&findings, "UNCHECKED_LOW_LEVEL_CALL"));
}

#[test]
fn test_runtime_checked_call_safe() {
    let contract = solidity(
        "Checked",
        "pragma solidity ^0.8.20;\ncontract C {\n  function pay(address to) external {\n    (bool ok, ) = to.call{value: 1 ether}(\"\");\n    require(ok, \"fail\");\n  }\n}\n",
    );
    let findings = detect_runtime_exploits(&contract);
    assert!(!has_rule(&findings, "UNCHECKED_LOW_LEVEL_CALL"));
}

#[test]
fn test_runtime_arbitrary_call_sink_flagged() {
    let contract = solidity(
        "Multicall",
        "pragma solidity ^0.8.20;\ncontract C {\n  function execute(address target, bytes calldata data) external {\n    (bool ok, ) = target.call(data);\n    require(ok);\n  }\n}\n",
    );
    let findings = detect_runtime_exploits(&contract);
    assert!(has_rule(&findings, "ARBITRARY_CALL_SINK"));
    let finding = findings
        .iter()
        .find(|f| f.rule_id == "ARBITRARY_CALL_SINK")
        .expect("present");
    assert_eq!(finding.severity, Severity::Critical);
}

#[test]
fn test_runtime_arbitrary_call_sink_access_controlled_safe() {
    let contract = solidity(
        "GuardedMulticall",
        "pragma solidity ^0.8.20;\ncontract C {\n  modifier onlyOwner() { _; }\n  function execute(address target, bytes calldata data) external onlyOwner {\n    (bool ok, ) = target.call(data);\n    require(ok);\n  }\n}\n",
    );
    let findings = detect_runtime_exploits(&contract);
    assert!(!has_rule(&findings, "ARBITRARY_CALL_SINK"));
}

// --- Honeypot ------------------------------------------------------------------

#[test]
fn test_honeypot_owner_only_transfer_flagged() {
    let contract = solidity(
        "Trap",
        "pragma solidity ^0.8.20;\ncontract C {\n  address owner;\n  mapping(address=>uint256) bal;\n  function transfer(address to, uint256 amt) external returns (bool) {\n    require(from == owner, \"locked\");\n    return true;\n  }\n}\n",
    );
    let findings = detect_honeypots(&contract);
    assert!(has_rule(&findings, "OWNER_ONLY_TRANSFER"));
}

#[test]
fn test_honeypot_owner_only_transfer_with_optin_safe() {
    // A compliance gate WITH a public/role opt-in path is not a honeypot.
    let contract = solidity(
        "Compliant",
        "pragma solidity ^0.8.20;\ncontract C {\n  mapping(address=>bool) whitelisted;\n  function transfer(address to, uint256 amt) external returns (bool) {\n    require(whitelisted[from], \"kyc\");\n    return true;\n  }\n  function whitelistAddress(address a, bool s) external {\n    whitelisted[a] = s;\n  }\n}\n",
    );
    let findings = detect_honeypots(&contract);
    assert!(!has_rule(&findings, "OWNER_ONLY_TRANSFER"));
}

#[test]
fn test_honeypot_blacklist_trap_flagged() {
    let contract = solidity(
        "BL",
        "pragma solidity ^0.8.20;\ncontract C {\n  mapping(address => bool) public blacklist;\n  function transfer(address to, uint256 amt) external returns (bool) {\n    require(!blacklist[from], \"bl\");\n    return true;\n  }\n  function setBlacklist(address a, bool s) external {\n    blacklist[a] = s;\n  }\n}\n",
    );
    let findings = detect_honeypots(&contract);
    assert!(has_rule(&findings, "BLACKLIST_TRANSFER_TRAP"));
    let finding = findings
        .iter()
        .find(|f| f.rule_id == "BLACKLIST_TRANSFER_TRAP")
        .expect("present");
    // owner can blacklist anyone => High.
    assert_eq!(finding.severity, Severity::High);
}

#[test]
fn test_honeypot_uncapped_sell_tax_flagged() {
    let contract = solidity(
        "Taxed",
        "pragma solidity ^0.8.20;\ncontract C {\n  uint256 public sellTax;\n  function setSellTax(uint256 t) external {\n    sellTax = t;\n  }\n}\n",
    );
    let findings = detect_honeypots(&contract);
    assert!(has_rule(&findings, "UNCAPPED_SELL_TAX"));
}

#[test]
fn test_honeypot_capped_sell_tax_safe() {
    let contract = solidity(
        "CappedTax",
        "pragma solidity ^0.8.20;\ncontract C {\n  uint256 public sellTax;\n  uint256 constant MAX_TAX = 1000;\n  function setSellTax(uint256 t) external {\n    require(sellTax <= MAX_TAX, \"cap\");\n    sellTax = t;\n  }\n}\n",
    );
    let findings = detect_honeypots(&contract);
    assert!(!has_rule(&findings, "UNCAPPED_SELL_TAX"));
}

#[test]
fn test_honeypot_fake_withdraw_flagged() {
    let contract = solidity(
        "FakeBank",
        "pragma solidity ^0.8.20;\ncontract C {\n  mapping(address=>uint256) public balanceOf;\n  function deposit() external payable {\n    balanceOf[msg.sender] += msg.value;\n  }\n  function withdraw() external {\n    balanceOf[msg.sender] = 0;\n  }\n}\n",
    );
    let findings = detect_honeypots(&contract);
    assert!(has_rule(&findings, "FAKE_WITHDRAW"));
}

#[test]
fn test_honeypot_real_withdraw_safe() {
    let contract = solidity(
        "RealBank",
        "pragma solidity ^0.8.20;\ncontract C {\n  mapping(address=>uint256) public balanceOf;\n  function withdraw() external {\n    uint256 amt = balanceOf[msg.sender];\n    balanceOf[msg.sender] = 0;\n    (bool ok, ) = payable(msg.sender).call{value: amt}(\"\");\n    require(ok);\n  }\n}\n",
    );
    let findings = detect_honeypots(&contract);
    assert!(!has_rule(&findings, "FAKE_WITHDRAW"));
}

#[test]
fn test_honeypot_transfer_always_reverts_flagged() {
    let contract = solidity(
        "NoSell",
        "pragma solidity ^0.8.20;\ncontract C {\n  function transfer(address to, uint256 amt) external returns (bool) {\n    require(false, \"disabled\");\n    return true;\n  }\n}\n",
    );
    let findings = detect_honeypots(&contract);
    assert!(has_rule(&findings, "TRANSFER_ALWAYS_REVERTS"));
}

// --- Rug pull ------------------------------------------------------------------

#[test]
fn test_rug_uncapped_mint_flagged() {
    let contract = solidity(
        "InflationCoin",
        "pragma solidity ^0.8.20;\ncontract C {\n  modifier onlyOwner() { _; }\n  function mint(address to, uint256 amount) external onlyOwner {\n    _mint(to, amount);\n  }\n}\n",
    );
    let findings = detect_rug_pull_risks(&contract);
    assert!(has_rule(&findings, "UNCAPPED_OWNER_MINT"));
}

#[test]
fn test_rug_capped_mint_safe() {
    let contract = solidity(
        "CappedCoin",
        "pragma solidity ^0.8.20;\ncontract C {\n  uint256 cap;\n  modifier onlyOwner() { _; }\n  function mint(address to, uint256 amount) external onlyOwner {\n    require(totalSupply() + amount <= cap, \"cap\");\n    _mint(to, amount);\n  }\n}\n",
    );
    let findings = detect_rug_pull_risks(&contract);
    assert!(!has_rule(&findings, "UNCAPPED_OWNER_MINT"));
}

#[test]
fn test_rug_owner_drain_flagged() {
    let contract = solidity(
        "Drainable",
        "pragma solidity ^0.8.20;\ncontract C {\n  modifier onlyOwner() { _; }\n  function withdrawAll() external onlyOwner {\n    payable(owner()).transfer(address(this).balance);\n  }\n}\n",
    );
    let findings = detect_rug_pull_risks(&contract);
    assert!(has_rule(&findings, "OWNER_FUND_DRAIN"));
}

#[test]
fn test_rug_uncapped_fee_flagged() {
    let contract = solidity(
        "FeeCoin",
        "pragma solidity ^0.8.20;\ncontract C {\n  uint256 fee;\n  function setFee(uint256 f) external {\n    fee = f;\n  }\n}\n",
    );
    let findings = detect_rug_pull_risks(&contract);
    assert!(has_rule(&findings, "UNCAPPED_FEE"));
}

#[test]
fn test_rug_capped_fee_safe() {
    let contract = solidity(
        "FairFeeCoin",
        "pragma solidity ^0.8.20;\ncontract C {\n  uint256 fee;\n  uint256 constant MAX_FEE = 500;\n  function setFee(uint256 f) external {\n    require(newFee <= MAX_FEE, \"cap\");\n    fee = f;\n  }\n}\n",
    );
    let findings = detect_rug_pull_risks(&contract);
    assert!(!has_rule(&findings, "UNCAPPED_FEE"));
}

#[test]
fn test_rug_instant_upgrade_flagged() {
    let contract = solidity(
        "InstantUpgrade",
        "pragma solidity ^0.8.20;\ncontract C is UUPSUpgradeable {\n  function _authorizeUpgrade(address) internal override {}\n}\n",
    );
    let findings = detect_rug_pull_risks(&contract);
    assert!(has_rule(&findings, "INSTANT_UPGRADE"));
}

#[test]
fn test_rug_timelocked_upgrade_safe() {
    let contract = solidity(
        "TimelockedUpgrade",
        "pragma solidity ^0.8.20;\ncontract C is UUPSUpgradeable {\n  // Upgrades routed through a TimelockController.\n  function _authorizeUpgrade(address) internal override {\n    require(block.timestamp >= eta, \"timelock\");\n  }\n}\n",
    );
    let findings = detect_rug_pull_risks(&contract);
    assert!(!has_rule(&findings, "INSTANT_UPGRADE"));
}

// --- Sandwich ------------------------------------------------------------------

#[test]
fn test_sandwich_missing_min_out_flagged() {
    let contract = solidity(
        "NaiveSwap",
        "pragma solidity ^0.8.20;\ncontract C {\n  function swap(uint256 amountIn) external returns (uint256) {\n    uint256 amountOut = getAmountOut(amountIn);\n    return amountOut;\n  }\n  function getAmountOut(uint256 a) public pure returns (uint256) { return a; }\n}\n",
    );
    let findings = detect_sandwich_risks(&contract);
    assert!(has_rule(&findings, "MISSING_SLIPPAGE_BOUND"));
}

#[test]
fn test_sandwich_with_min_out_and_deadline_safe() {
    let contract = solidity(
        "SafeSwap",
        "pragma solidity ^0.8.20;\ncontract C {\n  function swap(uint256 amountIn, uint256 amountOutMin, uint256 deadline) external returns (uint256) {\n    require(block.timestamp <= deadline, \"expired\");\n    uint256 amountOut = quote(amountIn);\n    require(amountOut >= amountOutMin, \"slippage\");\n    return amountOut;\n  }\n  function quote(uint256 a) public pure returns (uint256) { return a; }\n}\n",
    );
    let findings = detect_sandwich_risks(&contract);
    assert!(!has_rule(&findings, "MISSING_SLIPPAGE_BOUND"));
    assert!(!has_rule(&findings, "MISSING_DEADLINE"));
}

#[test]
fn test_sandwich_spot_price_flagged() {
    let contract = solidity(
        "SpotPriced",
        "pragma solidity ^0.8.20;\ncontract C {\n  function price() external view returns (uint256) {\n    (uint112 r0, uint112 r1, ) = pair.getReserves();\n    return uint256(r1) * 1e18 / uint256(r0);\n  }\n}\n",
    );
    let findings = detect_sandwich_risks(&contract);
    // getReserves implies swap-like; spot pricing without TWAP should flag.
    assert!(has_rule(&findings, "SPOT_PRICE_PRICING"));
}

#[test]
fn test_sandwich_non_swap_contract_not_flagged() {
    let contract = solidity(
        "Plain",
        "pragma solidity ^0.8.20;\ncontract C {\n  uint256 public x;\n  function set(uint256 v) external { x = v; }\n}\n",
    );
    let findings = detect_sandwich_risks(&contract);
    assert!(findings.is_empty());
}

// --- Front-running -------------------------------------------------------------

#[test]
fn test_frontrun_first_caller_reward_flagged() {
    let contract = solidity(
        "Puzzle",
        "pragma solidity ^0.8.20;\ncontract C {\n  bytes32 answerHash;\n  bool claimed;\n  function solve(uint256 solution) external {\n    require(!claimed, \"done\");\n    require(solution == 42, \"wrong\");\n    claimed = true;\n    payable(msg.sender).transfer(1 ether);\n  }\n}\n",
    );
    let findings = detect_front_running_risks(&contract);
    assert!(has_rule(&findings, "FIRST_CALLER_REWARD"));
}

#[test]
fn test_frontrun_commit_reveal_safe() {
    let contract = solidity(
        "CommitReveal",
        "pragma solidity ^0.8.20;\ncontract C {\n  mapping(address=>bytes32) commitment;\n  function commit(bytes32 c) external { commitment[msg.sender] = c; }\n  function reveal(uint256 solution, bytes32 salt) external {\n    require(commitment[msg.sender] == keccak256(abi.encodePacked(msg.sender, solution, salt)), \"bad\");\n    payable(msg.sender).transfer(1 ether);\n  }\n}\n",
    );
    let findings = detect_front_running_risks(&contract);
    assert!(!has_rule(&findings, "FIRST_CALLER_REWARD"));
    assert!(!has_rule(&findings, "PLAINTEXT_SECRET"));
}

#[test]
fn test_frontrun_plaintext_secret_flagged() {
    let contract = solidity(
        "Vault",
        "pragma solidity ^0.8.20;\ncontract C {\n  bytes32 passwordHash;\n  function unlock(string calldata password) external {\n    require(keccak256(abi.encodePacked(password)) == passwordHash, \"no\");\n    payable(msg.sender).transfer(address(this).balance);\n  }\n}\n",
    );
    let findings = detect_front_running_risks(&contract);
    assert!(has_rule(&findings, "PLAINTEXT_SECRET"));
}

#[test]
fn test_frontrun_open_bid_flagged() {
    let contract = solidity(
        "Auction",
        "pragma solidity ^0.8.20;\ncontract C {\n  uint256 public highestBid;\n  function bid() external payable {\n    require(msg.value > highestBid, \"low\");\n    highestBid = msg.value;\n  }\n}\n",
    );
    let findings = detect_front_running_risks(&contract);
    assert!(has_rule(&findings, "OPEN_BID_AUCTION"));
}

#[test]
fn test_frontrun_approve_race_flagged() {
    let contract = solidity(
        "RaceToken",
        "pragma solidity ^0.8.20;\ncontract C {\n  mapping(address=>mapping(address=>uint256)) _allowances;\n  function approve(address spender, uint256 amount) external returns (bool) {\n    _allowances[msg.sender][spender] = amount;\n    return true;\n  }\n}\n",
    );
    let findings = detect_front_running_risks(&contract);
    assert!(has_rule(&findings, "APPROVE_RACE"));
}

#[test]
fn test_frontrun_safe_allowance_not_flagged() {
    let contract = solidity(
        "SafeToken",
        "pragma solidity ^0.8.20;\ncontract C {\n  mapping(address=>mapping(address=>uint256)) _allowances;\n  function approve(address spender, uint256 amount) external returns (bool) {\n    _allowances[msg.sender][spender] = amount;\n    return true;\n  }\n  function increaseAllowance(address spender, uint256 added) external returns (bool) {\n    return true;\n  }\n}\n",
    );
    let findings = detect_front_running_risks(&contract);
    assert!(!has_rule(&findings, "APPROVE_RACE"));
}

// --- Aggregation & scoring -----------------------------------------------------

#[test]
fn test_analyze_security_aggregates_and_scores() {
    // A deliberately awful contract: tx.origin auth + unguarded delegatecall +
    // uncapped mint.
    let contract = solidity(
        "Awful",
        "pragma solidity ^0.8.20;\ncontract C {\n  address owner;\n  function adminWithdraw() external {\n    require(tx.origin == owner, \"no\");\n    payable(owner).transfer(address(this).balance);\n  }\n  function forward(address impl) external {\n    (bool ok, ) = impl.delegatecall(msg.data);\n    require(ok);\n  }\n  function mint(address to, uint256 amount) external {\n    _mint(to, amount);\n  }\n}\n",
    );
    let scan = analyze_security(&contract);
    assert_eq!(scan.contract_name, "Awful");
    assert!(!scan.is_clean());
    assert!(scan.has_at_least(Severity::Critical));
    // A critical issue must push the score below the conventional 60 pass line.
    assert!(scan.risk_score < 60, "score was {}", scan.risk_score);
    // Findings are sorted by descending severity.
    let ranks: Vec<u8> = scan
        .findings
        .iter()
        .map(|f| match f.severity {
            Severity::Critical => 4,
            Severity::High => 3,
            Severity::Medium => 2,
            Severity::Low => 1,
        })
        .collect();
    for window in ranks.windows(2) {
        assert!(window[0] >= window[1], "findings not severity-sorted");
    }
}

#[test]
fn test_analyze_security_by_category() {
    let contract = solidity(
        "TxOnly",
        "pragma solidity ^0.8.20;\ncontract C {\n  address owner;\n  function f() external view {\n    require(tx.origin == owner, \"no\");\n  }\n}\n",
    );
    let scan = analyze_security(&contract);
    let runtime = scan.by_category(FindingCategory::RuntimeExploit);
    assert!(!runtime.is_empty());
    let honeypots = scan.by_category(FindingCategory::Honeypot);
    assert!(honeypots.is_empty());
}

#[test]
fn test_analyze_security_non_evm_is_clean() {
    let contract = GeneratedContract {
        name: "MoveThing".to_string(),
        source: "module 0x1::thing { public fun f() {} }".to_string(),
        platform: TargetPlatform::Move,
        abi: None,
        deployment_script: None,
    };
    let scan = analyze_security(&contract);
    assert!(scan.is_clean());
    assert_eq!(scan.risk_score, 100);
}

#[test]
fn test_empty_source_yields_no_findings() {
    let contract = solidity("Empty", "");
    assert!(detect_runtime_exploits(&contract).is_empty());
    assert!(detect_honeypots(&contract).is_empty());
    assert!(detect_rug_pull_risks(&contract).is_empty());
    assert!(detect_sandwich_risks(&contract).is_empty());
    assert!(detect_front_running_risks(&contract).is_empty());
}

// --- No false positives on the crate's own hardened generators -----------------

#[test]
fn test_no_false_positive_on_self_healing_generator() {
    // The crate's own self-healing contract uses checked low-level calls, CEI and
    // ReentrancyGuard; it must not trip the runtime-exploit detector's
    // unchecked-call or tx.origin rules.
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let config = SelfHealingConfig {
        name: "SelfHealingVault".to_string(),
        invariants: vec![HealthInvariant {
            key: "collateral_ratio_bps".to_string(),
            description: "Collateral ratio".to_string(),
            min_value: 11_000,
            max_value: 1_000_000,
        }],
        checkpoint_enabled: true,
        recover_after_seconds: 3600,
        keeper_reward_wei: 1_000_000_000_000_000,
        jurisdiction: Jurisdiction::Us,
    };
    let contract = generator
        .generate_self_healing(&config)
        .expect("generate self-healing");
    let runtime = detect_runtime_exploits(&contract);
    assert!(
        !has_rule(&runtime, "UNCHECKED_LOW_LEVEL_CALL"),
        "self-healing keeper reward uses a checked call"
    );
    assert!(!has_rule(&runtime, "TX_ORIGIN_AUTH"));
    assert!(!has_rule(&runtime, "ARBITRARY_CALL_SINK"));
}

#[test]
fn test_no_critical_false_positive_on_pausable_vault_template() {
    use std::collections::BTreeMap;
    let library = crate::TemplateLibrary::with_builtins();
    let mut values = BTreeMap::new();
    values.insert("contract_name".to_string(), "Vault".to_string());
    values.insert(
        "asset".to_string(),
        "0x1111111111111111111111111111111111111111".to_string(),
    );
    let contract = library
        .render("pausable_vault", &values)
        .expect("render vault");
    let scan = analyze_security(&contract);
    // The hardened vault (SafeERC20, CEI, ReentrancyGuard, role-gated) must not
    // produce any Critical finding.
    assert!(
        !scan.has_at_least(Severity::Critical),
        "unexpected critical findings: {:?}",
        scan.findings
    );
}

#[test]
fn test_escrow_template_is_low_risk() {
    use std::collections::BTreeMap;
    let library = crate::TemplateLibrary::with_builtins();
    let mut values = BTreeMap::new();
    values.insert("contract_name".to_string(), "Escrow".to_string());
    values.insert(
        "beneficiary".to_string(),
        "0x2222222222222222222222222222222222222222".to_string(),
    );
    values.insert("release_after".to_string(), "86400".to_string());
    let contract = library.render("timelock_escrow", &values).expect("render");
    let scan = analyze_security(&contract);
    // The escrow uses a checked call + ReentrancyGuard + CEI; no Critical/High.
    assert!(
        !scan.has_at_least(Severity::High),
        "findings: {:?}",
        scan.findings
    );
}
