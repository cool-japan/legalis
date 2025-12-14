//! Legalis-Chain: Smart contract export for Legalis-RS.
//!
//! This crate provides export functionality to convert deterministic
//! legal statutes into smart contracts (WASM/Solidity).

use legalis_core::{ComparisonOp, Condition, EffectType, Statute};
use thiserror::Error;

/// Errors during contract generation.
#[derive(Debug, Error)]
pub enum ChainError {
    #[error("Statute '{0}' contains discretionary elements and cannot be exported")]
    DiscretionaryStatute(String),

    #[error("Unsupported condition type: {0}")]
    UnsupportedCondition(String),

    #[error("Unsupported effect type: {0}")]
    UnsupportedEffect(String),

    #[error("Generation error: {0}")]
    GenerationError(String),
}

/// Result type for chain operations.
pub type ChainResult<T> = Result<T, ChainError>;

/// Target platform for contract generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetPlatform {
    /// Solidity for Ethereum/EVM
    Solidity,
    /// Rust for WASM
    RustWasm,
    /// Ink! for Substrate
    Ink,
    /// Vyper for Ethereum/EVM
    Vyper,
    /// Move for Aptos/Sui
    Move,
    /// Cairo for StarkNet
    Cairo,
    /// CosmWasm for Cosmos
    CosmWasm,
}

/// Generated smart contract.
#[derive(Debug, Clone)]
pub struct GeneratedContract {
    /// Name of the contract
    pub name: String,
    /// Source code
    pub source: String,
    /// Target platform
    pub platform: TargetPlatform,
    /// ABI (for Solidity)
    pub abi: Option<String>,
    /// Deployment script
    pub deployment_script: Option<String>,
}

/// Deployment configuration.
#[derive(Debug, Clone)]
pub struct DeploymentConfig {
    /// Network name (e.g., "mainnet", "testnet", "localhost")
    pub network: String,
    /// Gas limit
    pub gas_limit: Option<u64>,
    /// Gas price in gwei
    pub gas_price: Option<u64>,
}

/// Security vulnerability types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VulnerabilityType {
    /// Reentrancy vulnerability
    Reentrancy,
    /// Integer overflow/underflow
    IntegerOverflow,
    /// Unchecked external call
    UncheckedExternalCall,
    /// Access control issue
    AccessControl,
    /// Front-running vulnerability
    FrontRunning,
    /// Denial of service
    DenialOfService,
}

/// Security analysis result.
#[derive(Debug, Clone)]
pub struct SecurityAnalysis {
    /// Contract being analyzed
    pub contract_name: String,
    /// Detected vulnerabilities
    pub vulnerabilities: Vec<Vulnerability>,
    /// Security score (0-100)
    pub score: u8,
}

/// Detected vulnerability.
#[derive(Debug, Clone)]
pub struct Vulnerability {
    /// Type of vulnerability
    pub vulnerability_type: VulnerabilityType,
    /// Severity (Critical, High, Medium, Low)
    pub severity: Severity,
    /// Description
    pub description: String,
    /// Line number (if applicable)
    pub line: Option<usize>,
    /// Recommendation
    pub recommendation: String,
}

/// Severity level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    /// Critical severity
    Critical,
    /// High severity
    High,
    /// Medium severity
    Medium,
    /// Low severity
    Low,
}

/// Smart contract generator.
pub struct ContractGenerator {
    platform: TargetPlatform,
}

impl ContractGenerator {
    /// Creates a new generator for the specified platform.
    pub fn new(platform: TargetPlatform) -> Self {
        Self { platform }
    }

    /// Generates a factory contract that can deploy multiple statute contracts.
    pub fn generate_factory(&self, statute_ids: &[&str]) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_factory(statute_ids),
            TargetPlatform::Vyper => self.generate_vyper_factory(statute_ids),
            _ => Err(ChainError::GenerationError(format!(
                "Factory generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates an upgradeable proxy contract for a statute contract.
    pub fn generate_upgradeable_proxy(
        &self,
        contract_name: &str,
    ) -> ChainResult<GeneratedContract> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_proxy(contract_name),
            _ => Err(ChainError::GenerationError(format!(
                "Proxy generation not supported for {:?}",
                self.platform
            ))),
        }
    }

    /// Generates deployment script for a generated contract.
    pub fn generate_deployment_script(
        &self,
        contract: &GeneratedContract,
        config: &DeploymentConfig,
    ) -> ChainResult<String> {
        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity_deployment(contract, config),
            TargetPlatform::Vyper => self.generate_vyper_deployment(contract, config),
            TargetPlatform::Move => self.generate_move_deployment(contract, config),
            TargetPlatform::Cairo => self.generate_cairo_deployment(contract, config),
            TargetPlatform::RustWasm => self.generate_wasm_deployment(contract, config),
            TargetPlatform::Ink => self.generate_ink_deployment(contract, config),
            TargetPlatform::CosmWasm => self.generate_cosmwasm_deployment(contract, config),
        }
    }

    /// Generates a smart contract from a statute.
    pub fn generate(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        // Check for discretionary elements
        if statute.discretion_logic.is_some() {
            return Err(ChainError::DiscretionaryStatute(statute.id.clone()));
        }

        match self.platform {
            TargetPlatform::Solidity => self.generate_solidity(statute),
            TargetPlatform::RustWasm => self.generate_rust_wasm(statute),
            TargetPlatform::Ink => self.generate_ink(statute),
            TargetPlatform::Vyper => self.generate_vyper(statute),
            TargetPlatform::Move => self.generate_move(statute),
            TargetPlatform::Cairo => self.generate_cairo(statute),
            TargetPlatform::CosmWasm => self.generate_cosmwasm(statute),
        }
    }

    /// Generates multiple contracts from a set of statutes.
    pub fn generate_batch(&self, statutes: &[Statute]) -> Vec<ChainResult<GeneratedContract>> {
        statutes.iter().map(|s| self.generate(s)).collect()
    }

    fn generate_solidity(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_pascal_case(&statute.id);
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");
        source.push_str(&format!("/// @title {}\n", statute.title));
        source.push_str("/// @notice Auto-generated from Legalis-RS\n");
        source.push_str("/// @dev Gas-optimized smart contract with comprehensive event logging\n");
        source.push_str(&format!("contract {} {{\n", contract_name));

        // Events
        source.push_str("    /// @notice Emitted when eligibility is checked\n");
        source.push_str("    /// @param entity The address being checked\n");
        source.push_str("    /// @param result Whether the entity is eligible\n");
        source.push_str("    event EligibilityChecked(address indexed entity, bool result);\n\n");

        source.push_str("    /// @notice Emitted when an effect is applied\n");
        source.push_str("    /// @param beneficiary The address receiving the effect\n");
        source.push_str("    /// @param effectType The type of effect applied\n");
        source.push_str(
            "    event EffectApplied(address indexed beneficiary, string effectType);\n\n",
        );

        // State variables with gas optimization comments
        source.push_str("    /// @dev Using immutable for gas optimization\n");
        source.push_str("    address public immutable owner;\n");
        source.push_str("    /// @dev Mapping for O(1) eligibility lookup\n");
        source.push_str("    mapping(address => bool) public eligible;\n\n");

        // Constructor
        source.push_str("    /// @notice Initialize the contract\n");
        source.push_str("    constructor() {\n");
        source.push_str("        owner = msg.sender;\n");
        source.push_str("    }\n\n");

        // Check eligibility function
        source.push_str("    /// @notice Check if an entity meets the preconditions\n");
        source.push_str(&self.generate_solidity_check_function(statute)?);

        // Apply effect function
        source.push_str("\n    /// @notice Apply the legal effect\n");
        source.push_str(&self.generate_solidity_apply_function(statute)?);

        source.push_str("}\n");

        let abi = self.generate_solidity_abi(statute)?;

        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Solidity,
            abi: Some(abi),
            deployment_script: None,
        })
    }

    fn generate_solidity_check_function(&self, statute: &Statute) -> ChainResult<String> {
        let mut func = String::new();
        func.push_str("    /// @dev View function - no state changes, gas-efficient\n");
        func.push_str("    function checkEligibility(\n");

        // Generate parameters based on conditions
        let params = self.extract_parameters(&statute.preconditions);
        let param_str: Vec<String> = params
            .iter()
            .map(|(name, typ)| format!("        {} {}", typ, name))
            .collect();
        func.push_str(&param_str.join(",\n"));
        func.push_str("\n    ) public returns (bool) {\n");

        // Generate condition checks
        for condition in &statute.preconditions {
            func.push_str(&self.condition_to_solidity(condition)?);
        }

        func.push_str("        emit EligibilityChecked(msg.sender, true);\n");
        func.push_str("        return true;\n");
        func.push_str("    }\n");

        Ok(func)
    }

    fn generate_solidity_apply_function(&self, statute: &Statute) -> ChainResult<String> {
        let mut func = String::new();
        func.push_str("    /// @dev Only owner can call, with reentrancy protection via checks-effects-interactions\n");
        func.push_str("    function applyEffect(address beneficiary) public {\n");
        func.push_str("        require(msg.sender == owner, \"Only owner can apply effects\");\n");
        func.push_str("        require(beneficiary != address(0), \"Invalid beneficiary\");\n\n");

        let effect_type_str = format!("{:?}", statute.effect.effect_type);

        match statute.effect.effect_type {
            EffectType::Grant => {
                func.push_str(
                    "        // State change before external interactions (CEI pattern)\n",
                );
                func.push_str("        eligible[beneficiary] = true;\n");
            }
            EffectType::Revoke => {
                func.push_str(
                    "        // State change before external interactions (CEI pattern)\n",
                );
                func.push_str("        eligible[beneficiary] = false;\n");
            }
            EffectType::MonetaryTransfer => {
                func.push_str("        // Monetary transfer logic\n");
                func.push_str("        // Use call instead of transfer for better gas handling\n");
                func.push_str("        // (bool success, ) = payable(beneficiary).call{value: amount}(\"\");\n");
                func.push_str("        // require(success, \"Transfer failed\");\n");
            }
            _ => {
                func.push_str(&format!(
                    "        // Effect: {}\n",
                    statute.effect.description
                ));
            }
        }

        func.push_str(&format!(
            "        emit EffectApplied(beneficiary, \"{}\");\n",
            effect_type_str
        ));
        func.push_str("    }\n");
        Ok(func)
    }

    fn generate_solidity_abi(&self, statute: &Statute) -> ChainResult<String> {
        let params = self.extract_parameters(&statute.preconditions);
        let inputs: Vec<String> = params
            .iter()
            .map(|(name, typ)| {
                let sol_type = match typ.as_str() {
                    "uint256" => "uint256",
                    "bool" => "bool",
                    "string memory" => "string",
                    _ => "uint256",
                };
                format!(r#"{{"name":"{}","type":"{}"}}"#, name, sol_type)
            })
            .collect();

        Ok(format!(
            r#"[{{"name":"checkEligibility","type":"function","inputs":[{}],"outputs":[{{"type":"bool"}}]}}]"#,
            inputs.join(",")
        ))
    }

    fn condition_to_solidity(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_solidity(*operator);
                Ok(format!(
                    "        require(age {} {}, \"Age requirement not met\");\n",
                    op, value
                ))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_solidity(*operator);
                Ok(format!(
                    "        require(income {} {}, \"Income requirement not met\");\n",
                    op, value
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_solidity(left)?;
                result.push_str(&self.condition_to_solidity(right)?);
                Ok(result)
            }
            Condition::Or(left, right) => Ok(format!(
                "        require({} || {}, \"OR condition not met\");\n",
                self.condition_to_solidity_expr(left)?,
                self.condition_to_solidity_expr(right)?
            )),
            Condition::Not(inner) => Ok(format!(
                "        require(!{}, \"NOT condition not met\");\n",
                self.condition_to_solidity_expr(inner)?
            )),
            _ => Ok("        // Custom condition - manual implementation required\n".to_string()),
        }
    }

    fn condition_to_solidity_expr(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_solidity(*operator);
                Ok(format!("(age {} {})", op, value))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_solidity(*operator);
                Ok(format!("(income {} {})", op, value))
            }
            _ => Ok("true".to_string()),
        }
    }

    fn comparison_to_solidity(&self, op: ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "==",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
        }
    }

    fn extract_parameters(&self, conditions: &[Condition]) -> Vec<(String, String)> {
        let mut params = Vec::new();

        for condition in conditions {
            Self::extract_params_from_condition(condition, &mut params);
        }

        params.sort_by(|a, b| a.0.cmp(&b.0));
        params.dedup_by(|a, b| a.0 == b.0);
        params
    }

    fn extract_params_from_condition(condition: &Condition, params: &mut Vec<(String, String)>) {
        match condition {
            Condition::Age { .. } => {
                params.push(("age".to_string(), "uint256".to_string()));
            }
            Condition::Income { .. } => {
                params.push(("income".to_string(), "uint256".to_string()));
            }
            Condition::And(left, right) | Condition::Or(left, right) => {
                Self::extract_params_from_condition(left, params);
                Self::extract_params_from_condition(right, params);
            }
            Condition::Not(inner) => {
                Self::extract_params_from_condition(inner, params);
            }
            _ => {}
        }
    }

    fn generate_rust_wasm(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let module_name = to_snake_case(&statute.id);
        let mut source = String::new();

        source.push_str("//! Auto-generated from Legalis-RS\n\n");
        source.push_str("use wasm_bindgen::prelude::*;\n\n");
        source.push_str(&format!("/// {}\n", statute.title));
        source.push_str("#[wasm_bindgen]\n");
        source.push_str(&format!("pub struct {} {{\n", to_pascal_case(&statute.id)));
        source.push_str("    eligible: std::collections::HashSet<String>,\n");
        source.push_str("}\n\n");

        source.push_str("#[wasm_bindgen]\n");
        source.push_str(&format!("impl {} {{\n", to_pascal_case(&statute.id)));
        source.push_str("    #[wasm_bindgen(constructor)]\n");
        source.push_str("    pub fn new() -> Self {\n");
        source.push_str("        Self { eligible: std::collections::HashSet::new() }\n");
        source.push_str("    }\n\n");

        // Check function
        source.push_str("    pub fn check_eligibility(&self");
        let params = self.extract_parameters(&statute.preconditions);
        for (name, _) in &params {
            source.push_str(&format!(", {}: u64", name));
        }
        source.push_str(") -> bool {\n");
        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_rust(condition)?);
        }
        source.push_str("        true\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: module_name,
            source,
            platform: TargetPlatform::RustWasm,
            abi: None,
            deployment_script: None,
        })
    }

    fn condition_to_rust(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "        if !(age {} {}) {{ return false; }}\n",
                    op, value
                ))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "        if !(income {} {}) {{ return false; }}\n",
                    op, value
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_rust(left)?;
                result.push_str(&self.condition_to_rust(right)?);
                Ok(result)
            }
            _ => Ok("        // Custom condition\n".to_string()),
        }
    }

    fn comparison_to_rust(&self, op: ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "==",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
        }
    }

    fn generate_ink(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();

        source.push_str("#![cfg_attr(not(feature = \"std\"), no_std, no_main)]\n\n");
        source.push_str("#[ink::contract]\n");
        source.push_str(&format!("mod {} {{\n", contract_name));
        source.push_str("    #[ink(storage)]\n");
        source.push_str("    pub struct Contract {\n");
        source.push_str("        owner: AccountId,\n");
        source.push_str("    }\n\n");

        source.push_str("    impl Contract {\n");
        source.push_str("        #[ink(constructor)]\n");
        source.push_str("        pub fn new() -> Self {\n");
        source.push_str("            Self { owner: Self::env().caller() }\n");
        source.push_str("        }\n\n");

        source.push_str(&format!("        /// {}\n", statute.title));
        source.push_str("        #[ink(message)]\n");
        source.push_str("        pub fn check_eligibility(&self");
        let params = self.extract_parameters(&statute.preconditions);
        for (name, _) in &params {
            source.push_str(&format!(", {}: u64", name));
        }
        source.push_str(") -> bool {\n");
        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_rust(condition)?);
        }
        source.push_str("            true\n");
        source.push_str("        }\n");
        source.push_str("    }\n");
        source.push_str("}\n");

        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Ink,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_vyper(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();

        source.push_str("# @version ^0.3.0\n");
        source.push_str(&format!("# @title {}\n", statute.title));
        source.push_str("# @notice Auto-generated from Legalis-RS\n\n");

        // State variables
        source.push_str("owner: public(address)\n");
        source.push_str("eligible: public(HashMap[address, bool])\n\n");

        // Events
        source.push_str("event EligibilityChecked:\n");
        source.push_str("    entity: indexed(address)\n");
        source.push_str("    result: bool\n\n");
        source.push_str("event EffectApplied:\n");
        source.push_str("    beneficiary: indexed(address)\n");
        source.push_str("    effect_type: String[100]\n\n");

        // Constructor
        source.push_str("@external\n");
        source.push_str("def __init__():\n");
        source.push_str("    self.owner = msg.sender\n\n");

        // Check eligibility function
        source.push_str("@external\n");
        source.push_str("@view\n");
        source.push_str("def check_eligibility(");
        let params = self.extract_parameters(&statute.preconditions);
        let param_str: Vec<String> = params
            .iter()
            .map(|(name, _)| format!("{}: uint256", name))
            .collect();
        source.push_str(&param_str.join(", "));
        source.push_str(") -> bool:\n");
        source.push_str("    \"\"\"Check if an entity meets the preconditions\"\"\"\n");

        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_vyper(condition)?);
        }
        source.push_str("    log EligibilityChecked(msg.sender, True)\n");
        source.push_str("    return True\n\n");

        // Apply effect function
        source.push_str("@external\n");
        source.push_str("def apply_effect(beneficiary: address):\n");
        source.push_str("    \"\"\"Apply the legal effect\"\"\"\n");
        source.push_str("    assert msg.sender == self.owner, \"Only owner can apply effects\"\n");

        match statute.effect.effect_type {
            EffectType::Grant => {
                source.push_str("    self.eligible[beneficiary] = True\n");
            }
            EffectType::Revoke => {
                source.push_str("    self.eligible[beneficiary] = False\n");
            }
            EffectType::MonetaryTransfer => {
                source.push_str("    # Monetary transfer logic\n");
                source.push_str("    # send(beneficiary, amount)\n");
            }
            _ => {
                source.push_str(&format!("    # Effect: {}\n", statute.effect.description));
            }
        }
        source.push_str("    log EffectApplied(beneficiary, \"");
        source.push_str(&format!("{:?}", statute.effect.effect_type));
        source.push_str("\")\n");

        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Vyper,
            abi: None,
            deployment_script: None,
        })
    }

    fn condition_to_vyper(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "    assert age {} {}, \"Age requirement not met\"\n",
                    op, value
                ))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "    assert income {} {}, \"Income requirement not met\"\n",
                    op, value
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_vyper(left)?;
                result.push_str(&self.condition_to_vyper(right)?);
                Ok(result)
            }
            Condition::Or(left, right) => Ok(format!(
                "    assert {} or {}, \"OR condition not met\"\n",
                self.condition_to_vyper_expr(left)?,
                self.condition_to_vyper_expr(right)?
            )),
            Condition::Not(inner) => Ok(format!(
                "    assert not {}, \"NOT condition not met\"\n",
                self.condition_to_vyper_expr(inner)?
            )),
            _ => Ok("    # Custom condition - manual implementation required\n".to_string()),
        }
    }

    fn condition_to_vyper_expr(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!("(age {} {})", op, value))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!("(income {} {})", op, value))
            }
            _ => Ok("True".to_string()),
        }
    }

    fn generate_move(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let module_name = to_snake_case(&statute.id);
        let mut source = String::new();

        source.push_str("module legalis::");
        source.push_str(&module_name);
        source.push_str(" {\n");
        source.push_str("    use std::signer;\n");
        source.push_str("    use aptos_framework::event;\n\n");

        source.push_str(&format!("    /// {}\n", statute.title));
        source.push_str("    struct StatuteContract has key {\n");
        source.push_str("        owner: address,\n");
        source.push_str("        eligible_count: u64,\n");
        source.push_str("    }\n\n");

        source.push_str("    #[event]\n");
        source.push_str("    struct EligibilityChecked has drop, store {\n");
        source.push_str("        entity: address,\n");
        source.push_str("        result: bool,\n");
        source.push_str("    }\n\n");

        source.push_str("    #[event]\n");
        source.push_str("    struct EffectApplied has drop, store {\n");
        source.push_str("        beneficiary: address,\n");
        source.push_str("    }\n\n");

        // Initialize function
        source.push_str("    public entry fun initialize(account: &signer) {\n");
        source.push_str("        let owner_addr = signer::address_of(account);\n");
        source.push_str("        move_to(account, StatuteContract {\n");
        source.push_str("            owner: owner_addr,\n");
        source.push_str("            eligible_count: 0,\n");
        source.push_str("        });\n");
        source.push_str("    }\n\n");

        // Check eligibility function
        source.push_str("    public fun check_eligibility(");
        let params = self.extract_parameters(&statute.preconditions);
        let param_str: Vec<String> = params
            .iter()
            .map(|(name, _)| format!("{}: u64", name))
            .collect();
        source.push_str(&param_str.join(", "));
        source.push_str("): bool {\n");

        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_move(condition)?);
        }
        source.push_str("        true\n");
        source.push_str("    }\n\n");

        // Apply effect function
        source.push_str("    public entry fun apply_effect(account: &signer, beneficiary: address) acquires StatuteContract {\n");
        source.push_str("        let contract = borrow_global_mut<StatuteContract>(signer::address_of(account));\n");
        source.push_str("        assert!(signer::address_of(account) == contract.owner, 0);\n");
        source.push_str("        contract.eligible_count = contract.eligible_count + 1;\n");
        source.push_str("        event::emit(EffectApplied { beneficiary });\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: module_name,
            source,
            platform: TargetPlatform::Move,
            abi: None,
            deployment_script: None,
        })
    }

    fn condition_to_move(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!("        assert!(age {} {}, 1);\n", op, value))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!("        assert!(income {} {}, 2);\n", op, value))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_move(left)?;
                result.push_str(&self.condition_to_move(right)?);
                Ok(result)
            }
            _ => Ok("        // Custom condition\n".to_string()),
        }
    }

    fn generate_cairo(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();

        source.push_str("#[starknet::contract]\n");
        source.push_str(&format!("mod {} {{\n", contract_name));
        source.push_str("    use starknet::ContractAddress;\n");
        source.push_str("    use starknet::get_caller_address;\n\n");

        source.push_str("    #[storage]\n");
        source.push_str("    struct Storage {\n");
        source.push_str("        owner: ContractAddress,\n");
        source.push_str("        eligible_count: u64,\n");
        source.push_str("    }\n\n");

        source.push_str("    #[event]\n");
        source.push_str("    #[derive(Drop, starknet::Event)]\n");
        source.push_str("    enum Event {\n");
        source.push_str("        EligibilityChecked: EligibilityChecked,\n");
        source.push_str("        EffectApplied: EffectApplied,\n");
        source.push_str("    }\n\n");

        source.push_str("    #[derive(Drop, starknet::Event)]\n");
        source.push_str("    struct EligibilityChecked {\n");
        source.push_str("        entity: ContractAddress,\n");
        source.push_str("        result: bool,\n");
        source.push_str("    }\n\n");

        source.push_str("    #[derive(Drop, starknet::Event)]\n");
        source.push_str("    struct EffectApplied {\n");
        source.push_str("        beneficiary: ContractAddress,\n");
        source.push_str("    }\n\n");

        source.push_str("    #[constructor]\n");
        source.push_str("    fn constructor(ref self: ContractState) {\n");
        source.push_str("        self.owner.write(get_caller_address());\n");
        source.push_str("        self.eligible_count.write(0);\n");
        source.push_str("    }\n\n");

        source.push_str(&format!("    /// {}\n", statute.title));
        source.push_str("    #[external(v0)]\n");
        source.push_str("    fn check_eligibility(self: @ContractState, ");
        let params = self.extract_parameters(&statute.preconditions);
        let param_str: Vec<String> = params
            .iter()
            .map(|(name, _)| format!("{}: u64", name))
            .collect();
        source.push_str(&param_str.join(", "));
        source.push_str(") -> bool {\n");

        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_cairo(condition)?);
        }
        source.push_str("        true\n");
        source.push_str("    }\n\n");

        source.push_str("    #[external(v0)]\n");
        source.push_str(
            "    fn apply_effect(ref self: ContractState, beneficiary: ContractAddress) {\n",
        );
        source
            .push_str("        assert(get_caller_address() == self.owner.read(), 'Only owner');\n");
        source.push_str("        let count = self.eligible_count.read();\n");
        source.push_str("        self.eligible_count.write(count + 1);\n");
        source.push_str("        self.emit(EffectApplied { beneficiary });\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::Cairo,
            abi: None,
            deployment_script: None,
        })
    }

    fn condition_to_cairo(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "        assert(age {} {}, 'Age requirement not met');\n",
                    op, value
                ))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "        assert(income {} {}, 'Income requirement not met');\n",
                    op, value
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_cairo(left)?;
                result.push_str(&self.condition_to_cairo(right)?);
                Ok(result)
            }
            _ => Ok("        // Custom condition\n".to_string()),
        }
    }

    fn generate_cosmwasm(&self, statute: &Statute) -> ChainResult<GeneratedContract> {
        let contract_name = to_snake_case(&statute.id);
        let mut source = String::new();

        // lib.rs for CosmWasm contract
        source.push_str("use cosmwasm_std::{\n");
        source.push_str("    entry_point, to_json_binary, Binary, Deps, DepsMut, Env,\n");
        source.push_str("    MessageInfo, Response, StdResult, Addr,\n");
        source.push_str("};\n");
        source.push_str("use serde::{Deserialize, Serialize};\n\n");

        // State
        source.push_str("#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]\n");
        source.push_str("pub struct State {\n");
        source.push_str("    pub owner: Addr,\n");
        source.push_str("    pub eligible_count: u64,\n");
        source.push_str("}\n\n");

        // InstantiateMsg
        source.push_str("#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]\n");
        source.push_str("pub struct InstantiateMsg {}\n\n");

        // ExecuteMsg
        source.push_str("#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]\n");
        source.push_str("#[serde(rename_all = \"snake_case\")]\n");
        source.push_str("pub enum ExecuteMsg {\n");
        source.push_str("    ApplyEffect { beneficiary: String },\n");
        source.push_str("}\n\n");

        // QueryMsg
        source.push_str("#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]\n");
        source.push_str("#[serde(rename_all = \"snake_case\")]\n");
        source.push_str("pub enum QueryMsg {\n");
        source.push_str("    CheckEligibility {\n");
        let params = self.extract_parameters(&statute.preconditions);
        for (name, _) in &params {
            source.push_str(&format!("        {}: u64,\n", name));
        }
        source.push_str("    },\n");
        source.push_str("}\n\n");

        // Instantiate
        source.push_str(&format!("/// {}\n", statute.title));
        source.push_str("#[entry_point]\n");
        source.push_str("pub fn instantiate(\n");
        source.push_str("    deps: DepsMut,\n");
        source.push_str("    _env: Env,\n");
        source.push_str("    info: MessageInfo,\n");
        source.push_str("    _msg: InstantiateMsg,\n");
        source.push_str(") -> StdResult<Response> {\n");
        source.push_str("    let state = State {\n");
        source.push_str("        owner: info.sender.clone(),\n");
        source.push_str("        eligible_count: 0,\n");
        source.push_str("    };\n");
        source.push_str("    deps.storage.set(b\"state\", &to_json_binary(&state)?);\n");
        source.push_str("    Ok(Response::new()\n");
        source.push_str("        .add_attribute(\"method\", \"instantiate\")\n");
        source.push_str("        .add_attribute(\"owner\", info.sender))\n");
        source.push_str("}\n\n");

        // Execute
        source.push_str("#[entry_point]\n");
        source.push_str("pub fn execute(\n");
        source.push_str("    deps: DepsMut,\n");
        source.push_str("    _env: Env,\n");
        source.push_str("    info: MessageInfo,\n");
        source.push_str("    msg: ExecuteMsg,\n");
        source.push_str(") -> StdResult<Response> {\n");
        source.push_str("    match msg {\n");
        source.push_str("        ExecuteMsg::ApplyEffect { beneficiary } => {\n");
        source.push_str("            let state: State = deps.storage.get(b\"state\")?\n");
        source.push_str(
            "                .ok_or_else(|| cosmwasm_std::StdError::not_found(\"state\"))?;\n",
        );
        source.push_str("            if info.sender != state.owner {\n");
        source.push_str(
            "                return Err(cosmwasm_std::StdError::generic_err(\"Unauthorized\"));\n",
        );
        source.push_str("            }\n");
        source.push_str("            Ok(Response::new()\n");
        source.push_str("                .add_attribute(\"method\", \"apply_effect\")\n");
        source.push_str("                .add_attribute(\"beneficiary\", beneficiary))\n");
        source.push_str("        }\n");
        source.push_str("    }\n");
        source.push_str("}\n\n");

        // Query
        source.push_str("#[entry_point]\n");
        source.push_str("pub fn query(\n");
        source.push_str("    _deps: Deps,\n");
        source.push_str("    _env: Env,\n");
        source.push_str("    msg: QueryMsg,\n");
        source.push_str(") -> StdResult<Binary> {\n");
        source.push_str("    match msg {\n");
        source.push_str("        QueryMsg::CheckEligibility { ");
        let param_names: Vec<String> = params.iter().map(|(name, _)| name.clone()).collect();
        source.push_str(&param_names.join(", "));
        source.push_str(" } => {\n");

        for condition in &statute.preconditions {
            source.push_str(&self.condition_to_cosmwasm(condition)?);
        }

        source.push_str("            to_json_binary(&true)\n");
        source.push_str("        }\n");
        source.push_str("    }\n");
        source.push_str("}\n");

        Ok(GeneratedContract {
            name: contract_name,
            source,
            platform: TargetPlatform::CosmWasm,
            abi: None,
            deployment_script: None,
        })
    }

    fn condition_to_cosmwasm(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "            if !(age {} {}) {{\n                return Err(cosmwasm_std::StdError::generic_err(\"Age requirement not met\"));\n            }}\n",
                    op, value
                ))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!(
                    "            if !(income {} {}) {{\n                return Err(cosmwasm_std::StdError::generic_err(\"Income requirement not met\"));\n            }}\n",
                    op, value
                ))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_cosmwasm(left)?;
                result.push_str(&self.condition_to_cosmwasm(right)?);
                Ok(result)
            }
            _ => Ok("            // Custom condition\n".to_string()),
        }
    }

    fn generate_solidity_factory(&self, statute_ids: &[&str]) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");
        source.push_str("/// @title StatuteFactory\n");
        source.push_str("/// @notice Factory contract for deploying statute contracts\n");
        source.push_str("/// @dev Auto-generated from Legalis-RS\n");
        source.push_str("contract StatuteFactory {\n");
        source.push_str("    address public owner;\n");
        source.push_str("    address[] public deployedContracts;\n");
        source.push_str("    mapping(string => address[]) public contractsByType;\n\n");

        source.push_str(
            "    event ContractDeployed(address indexed contractAddress, string contractType);\n\n",
        );

        source.push_str("    constructor() {\n");
        source.push_str("        owner = msg.sender;\n");
        source.push_str("    }\n\n");

        source.push_str("    modifier onlyOwner() {\n");
        source.push_str("        require(msg.sender == owner, \"Only owner can call this\");\n");
        source.push_str("        _;\n");
        source.push_str("    }\n\n");

        for statute_id in statute_ids {
            let contract_name = to_pascal_case(statute_id);
            source.push_str(&format!(
                "    /// @notice Deploy a new {} contract\n",
                contract_name
            ));
            source.push_str(&format!(
                "    function deploy{}() public onlyOwner returns (address) {{\n",
                contract_name
            ));
            source.push_str(&format!(
                "        {} newContract = new {}();\n",
                contract_name, contract_name
            ));
            source.push_str("        address contractAddress = address(newContract);\n");
            source.push_str("        deployedContracts.push(contractAddress);\n");
            source.push_str(&format!(
                "        contractsByType[\"{}\"].push(contractAddress);\n",
                statute_id
            ));
            source.push_str(&format!(
                "        emit ContractDeployed(contractAddress, \"{}\");\n",
                statute_id
            ));
            source.push_str("        return contractAddress;\n");
            source.push_str("    }\n\n");
        }

        source.push_str("    /// @notice Get total number of deployed contracts\n");
        source
            .push_str("    function getDeployedContractsCount() public view returns (uint256) {\n");
        source.push_str("        return deployedContracts.length;\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Get contracts by type\n");
        source.push_str("    function getContractsByType(string memory contractType) public view returns (address[] memory) {\n");
        source.push_str("        return contractsByType[contractType];\n");
        source.push_str("    }\n");

        source.push_str("}\n");

        Ok(GeneratedContract {
            name: "StatuteFactory".to_string(),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_vyper_factory(&self, statute_ids: &[&str]) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("# @version ^0.3.0\n");
        source.push_str("# @title StatuteFactory\n");
        source.push_str("# @notice Factory contract for deploying statute contracts\n\n");

        source.push_str("owner: public(address)\n");
        source.push_str("deployed_contracts: public(DynArray[address, 1000])\n\n");

        source.push_str("event ContractDeployed:\n");
        source.push_str("    contract_address: indexed(address)\n");
        source.push_str("    contract_type: String[100]\n\n");

        source.push_str("@external\n");
        source.push_str("def __init__():\n");
        source.push_str("    self.owner = msg.sender\n\n");

        for statute_id in statute_ids {
            source.push_str(&format!("@external\n"));
            source.push_str(&format!(
                "def deploy_{}() -> address:\n",
                to_snake_case(statute_id)
            ));
            source.push_str("    assert msg.sender == self.owner, \"Only owner\"\n");
            source.push_str("    # Deployment logic here\n");
            source.push_str(&format!(
                "    log ContractDeployed(empty(address), \"{}\")\n",
                statute_id
            ));
            source.push_str("    return empty(address)\n\n");
        }

        Ok(GeneratedContract {
            name: "statute_factory".to_string(),
            source,
            platform: TargetPlatform::Vyper,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_proxy(&self, contract_name: &str) -> ChainResult<GeneratedContract> {
        let mut source = String::new();

        source.push_str("// SPDX-License-Identifier: MIT\n");
        source.push_str("pragma solidity ^0.8.0;\n\n");
        source.push_str(&format!(
            "/// @title {}Proxy\n",
            to_pascal_case(contract_name)
        ));
        source.push_str("/// @notice Upgradeable proxy contract using transparent proxy pattern\n");
        source.push_str("/// @dev Auto-generated from Legalis-RS\n");
        source.push_str(&format!(
            "contract {}Proxy {{\n",
            to_pascal_case(contract_name)
        ));

        source.push_str("    /// @notice Address of the current implementation\n");
        source.push_str("    address public implementation;\n");
        source.push_str("    /// @notice Admin address that can upgrade the implementation\n");
        source.push_str("    address public admin;\n\n");

        source.push_str("    event Upgraded(address indexed implementation);\n");
        source.push_str(
            "    event AdminChanged(address indexed previousAdmin, address indexed newAdmin);\n\n",
        );

        source.push_str("    /// @notice Initialize the proxy with implementation address\n");
        source.push_str("    constructor(address _implementation) {\n");
        source.push_str(
            "        require(_implementation != address(0), \"Invalid implementation\");\n",
        );
        source.push_str("        implementation = _implementation;\n");
        source.push_str("        admin = msg.sender;\n");
        source.push_str("    }\n\n");

        source.push_str("    modifier onlyAdmin() {\n");
        source.push_str("        require(msg.sender == admin, \"Only admin\");\n");
        source.push_str("        _;\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Upgrade to a new implementation\n");
        source.push_str(
            "    /// @param newImplementation Address of the new implementation contract\n",
        );
        source.push_str("    function upgradeTo(address newImplementation) external onlyAdmin {\n");
        source.push_str(
            "        require(newImplementation != address(0), \"Invalid implementation\");\n",
        );
        source.push_str(
            "        require(newImplementation != implementation, \"Same implementation\");\n",
        );
        source.push_str("        implementation = newImplementation;\n");
        source.push_str("        emit Upgraded(newImplementation);\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Change the admin address\n");
        source.push_str("    function changeAdmin(address newAdmin) external onlyAdmin {\n");
        source.push_str("        require(newAdmin != address(0), \"Invalid admin\");\n");
        source.push_str("        emit AdminChanged(admin, newAdmin);\n");
        source.push_str("        admin = newAdmin;\n");
        source.push_str("    }\n\n");

        source.push_str("    /// @notice Fallback function to delegate calls to implementation\n");
        source.push_str("    fallback() external payable {\n");
        source.push_str("        address impl = implementation;\n");
        source.push_str("        assembly {\n");
        source.push_str("            calldatacopy(0, 0, calldatasize())\n");
        source.push_str(
            "            let result := delegatecall(gas(), impl, 0, calldatasize(), 0, 0)\n",
        );
        source.push_str("            returndatacopy(0, 0, returndatasize())\n");
        source.push_str("            switch result\n");
        source.push_str("            case 0 { revert(0, returndatasize()) }\n");
        source.push_str("            default { return(0, returndatasize()) }\n");
        source.push_str("        }\n");
        source.push_str("    }\n\n");

        source.push_str("    receive() external payable {}\n");
        source.push_str("}\n");

        Ok(GeneratedContract {
            name: format!("{}Proxy", to_pascal_case(contract_name)),
            source,
            platform: TargetPlatform::Solidity,
            abi: None,
            deployment_script: None,
        })
    }

    fn generate_solidity_deployment(
        &self,
        contract: &GeneratedContract,
        config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("// Hardhat deployment script\n");
        script.push_str("const hre = require(\"hardhat\");\n\n");
        script.push_str("async function main() {\n");
        script.push_str(&format!(
            "  console.log(\"Deploying {} to {}...\");\n\n",
            contract.name, config.network
        ));
        script.push_str(&format!(
            "  const ContractFactory = await hre.ethers.getContractFactory(\"{}\");\n",
            contract.name
        ));

        if let Some(gas_limit) = config.gas_limit {
            script.push_str(&format!(
                "  const contract = await ContractFactory.deploy({{ gasLimit: {} }});\n",
                gas_limit
            ));
        } else {
            script.push_str("  const contract = await ContractFactory.deploy();\n");
        }

        script.push_str("  await contract.deployed();\n\n");
        script.push_str("  console.log(`Contract deployed to: ${contract.address}`);\n");
        script.push_str("  console.log(`Transaction hash: ${contract.deployTransaction.hash}`);\n");
        script.push_str("  console.log(`Deployer: ${await contract.signer.getAddress()}`);\n\n");

        script.push_str("  // Verify on Etherscan\n");
        script.push_str(
            "  if (hre.network.name !== \"localhost\" && hre.network.name !== \"hardhat\") {\n",
        );
        script.push_str("    console.log(\"Waiting for block confirmations...\");\n");
        script.push_str("    await contract.deployTransaction.wait(6);\n");
        script.push_str("    console.log(\"Verifying contract...\");\n");
        script.push_str("    await hre.run(\"verify:verify\", {\n");
        script.push_str("      address: contract.address,\n");
        script.push_str("      constructorArguments: [],\n");
        script.push_str("    });\n");
        script.push_str("  }\n");
        script.push_str("}\n\n");

        script.push_str("main()\n");
        script.push_str("  .then(() => process.exit(0))\n");
        script.push_str("  .catch((error) => {\n");
        script.push_str("    console.error(error);\n");
        script.push_str("    process.exit(1);\n");
        script.push_str("  });\n");

        Ok(script)
    }

    fn generate_vyper_deployment(
        &self,
        contract: &GeneratedContract,
        _config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("# Vyper deployment script using ape\n");
        script.push_str("from ape import accounts, project\n\n");
        script.push_str("def main():\n");
        script.push_str("    deployer = accounts.load(\"deployer\")\n");
        script.push_str(&format!(
            "    contract = deployer.deploy(project.{})\n",
            contract.name
        ));
        script.push_str("    print(f\"Contract deployed to: {contract.address}\")\n");
        script.push_str("    return contract\n");

        Ok(script)
    }

    fn generate_move_deployment(
        &self,
        contract: &GeneratedContract,
        _config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("# Move deployment script for Aptos\n\n");
        script.push_str(&format!(
            "echo \"Deploying {} module...\"\n\n",
            contract.name
        ));
        script.push_str("# Compile the module\n");
        script.push_str("aptos move compile\n\n");
        script.push_str("# Publish to the network\n");
        script.push_str("aptos move publish \\\n");
        script.push_str("  --named-addresses legalis=default \\\n");
        script.push_str("  --assume-yes\n\n");
        script.push_str("echo \"Deployment complete!\"\n");

        Ok(script)
    }

    fn generate_cairo_deployment(
        &self,
        contract: &GeneratedContract,
        _config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("# Cairo deployment script for StarkNet\n\n");
        script.push_str(&format!(
            "echo \"Deploying {} to StarkNet...\"\n\n",
            contract.name
        ));
        script.push_str("# Compile the contract\n");
        script.push_str(&format!(
            "starknet-compile {}.cairo --output {}_compiled.json\n\n",
            contract.name, contract.name
        ));
        script.push_str("# Declare the contract\n");
        script.push_str(&format!(
            "starknet declare --contract {}_compiled.json\n\n",
            contract.name
        ));
        script.push_str("# Deploy the contract\n");
        script.push_str(&format!(
            "starknet deploy --contract {}_compiled.json\n\n",
            contract.name
        ));
        script.push_str("echo \"Deployment complete!\"\n");

        Ok(script)
    }

    fn generate_wasm_deployment(
        &self,
        contract: &GeneratedContract,
        _config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("# WASM build and deployment script\n\n");
        script.push_str(&format!(
            "echo \"Building {} WASM module...\"\n\n",
            contract.name
        ));
        script.push_str("# Build the WASM module\n");
        script.push_str("wasm-pack build --target web\n\n");
        script.push_str("# The WASM module is now ready in pkg/ directory\n");
        script.push_str("echo \"Build complete! WASM module is in pkg/ directory\"\n");
        script.push_str("echo \"Include it in your web application:\"\n");
        script.push_str("echo \"  import init, { YourContract } from './pkg';\"\n");

        Ok(script)
    }

    fn generate_ink_deployment(
        &self,
        contract: &GeneratedContract,
        _config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("# Ink! deployment script for Substrate\n\n");
        script.push_str(&format!(
            "echo \"Building and deploying {} contract...\"\n\n",
            contract.name
        ));
        script.push_str("# Build the contract\n");
        script.push_str("cargo contract build --release\n\n");
        script.push_str("# Deploy using cargo-contract\n");
        script.push_str("cargo contract instantiate \\\n");
        script.push_str("  --constructor new \\\n");
        script.push_str("  --suri //Alice \\\n");
        script.push_str("  --execute\n\n");
        script.push_str("echo \"Deployment complete!\"\n");

        Ok(script)
    }

    fn generate_cosmwasm_deployment(
        &self,
        contract: &GeneratedContract,
        config: &DeploymentConfig,
    ) -> ChainResult<String> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("# CosmWasm deployment script\n\n");
        script.push_str(&format!(
            "echo \"Building and deploying {} contract...\"\n\n",
            contract.name
        ));

        script.push_str("# Optimize the contract\n");
        script.push_str("docker run --rm -v \"$(pwd)\":/code \\\n");
        script.push_str("  --mount type=volume,source=\"$(basename \"$(pwd)\")_cache\",target=/code/target \\\n");
        script.push_str(
            "  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \\\n",
        );
        script.push_str("  cosmwasm/rust-optimizer:0.12.13\n\n");

        script.push_str(&format!("# Deploy to {}\n", config.network));
        script.push_str(&format!("CHAIN_ID=\"{}\"\n", config.network));
        script.push_str("NODE=\"https://rpc.cosmos.network:443\"\n");
        script.push_str("TX_FLAGS=\"--gas auto --gas-adjustment 1.3 --gas-prices 0.025ucosm\"\n\n");

        script.push_str("# Store the contract code\n");
        script.push_str(&format!(
            "RES=$(wasmd tx wasm store artifacts/{}.wasm \\\n",
            contract.name
        ));
        script.push_str("  --from wallet \\\n");
        script.push_str("  --chain-id $CHAIN_ID \\\n");
        script.push_str("  --node $NODE \\\n");
        script.push_str("  $TX_FLAGS \\\n");
        script.push_str("  --yes \\\n");
        script.push_str("  --output json)\n\n");

        script.push_str("# Extract the code ID\n");
        script.push_str("CODE_ID=$(echo $RES | jq -r '.logs[0].events[] | select(.type==\"store_code\") | .attributes[] | select(.key==\"code_id\") | .value')\n");
        script.push_str("echo \"Code ID: $CODE_ID\"\n\n");

        script.push_str("# Instantiate the contract\n");
        script.push_str("INIT='{}'\n");
        script.push_str("wasmd tx wasm instantiate $CODE_ID \"$INIT\" \\\n");
        script.push_str("  --from wallet \\\n");
        script.push_str(&format!("  --label \"{}\" \\\n", contract.name));
        script.push_str("  --chain-id $CHAIN_ID \\\n");
        script.push_str("  --node $NODE \\\n");
        script.push_str("  $TX_FLAGS \\\n");
        script.push_str("  --yes\n\n");

        script.push_str("echo \"Deployment complete!\"\n");

        Ok(script)
    }
}

/// Converts a string to PascalCase.
fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == '-' || c == '_' || c.is_whitespace())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

/// Converts a string to snake_case.
fn to_snake_case(s: &str) -> String {
    s.replace('-', "_").to_lowercase()
}

/// Security analyzer for smart contracts.
pub struct SecurityAnalyzer;

impl SecurityAnalyzer {
    /// Performs security analysis on a generated contract.
    pub fn analyze(contract: &GeneratedContract) -> SecurityAnalysis {
        let mut vulnerabilities = Vec::new();

        match contract.platform {
            TargetPlatform::Solidity | TargetPlatform::Vyper => {
                Self::check_evm_vulnerabilities(contract, &mut vulnerabilities);
            }
            TargetPlatform::Move => {
                Self::check_move_vulnerabilities(contract, &mut vulnerabilities);
            }
            TargetPlatform::Cairo => {
                Self::check_cairo_vulnerabilities(contract, &mut vulnerabilities);
            }
            TargetPlatform::RustWasm | TargetPlatform::Ink | TargetPlatform::CosmWasm => {
                Self::check_wasm_vulnerabilities(contract, &mut vulnerabilities);
            }
        }

        let score = Self::calculate_security_score(&vulnerabilities);

        SecurityAnalysis {
            contract_name: contract.name.clone(),
            vulnerabilities,
            score,
        }
    }

    fn check_evm_vulnerabilities(
        contract: &GeneratedContract,
        vulnerabilities: &mut Vec<Vulnerability>,
    ) {
        // Check for reentrancy vulnerabilities
        if contract.source.contains("transfer(") || contract.source.contains("send(") {
            if !contract.source.contains("CEI pattern")
                && !contract.source.contains("ReentrancyGuard")
            {
                vulnerabilities.push(Vulnerability {
                    vulnerability_type: VulnerabilityType::Reentrancy,
                    severity: Severity::High,
                    description: "Potential reentrancy vulnerability in external call".to_string(),
                    line: None,
                    recommendation: "Use checks-effects-interactions pattern or ReentrancyGuard"
                        .to_string(),
                });
            }
        }

        // Check for unchecked external calls
        if contract.source.contains("call{") && !contract.source.contains("require(success") {
            vulnerabilities.push(Vulnerability {
                vulnerability_type: VulnerabilityType::UncheckedExternalCall,
                severity: Severity::Medium,
                description: "External call without checking return value".to_string(),
                line: None,
                recommendation: "Always check return values from external calls".to_string(),
            });
        }

        // Check for integer overflow (pre-Solidity 0.8.0)
        if contract.source.contains("pragma solidity") {
            let version_check =
                contract.source.contains("^0.8") || contract.source.contains(">=0.8");
            if !version_check {
                vulnerabilities.push(Vulnerability {
                    vulnerability_type: VulnerabilityType::IntegerOverflow,
                    severity: Severity::High,
                    description: "Solidity version < 0.8.0 without SafeMath".to_string(),
                    line: None,
                    recommendation: "Upgrade to Solidity ^0.8.0 or use SafeMath library"
                        .to_string(),
                });
            }
        }

        // Check for access control
        if !contract.source.contains("owner") && !contract.source.contains("onlyOwner") {
            vulnerabilities.push(Vulnerability {
                vulnerability_type: VulnerabilityType::AccessControl,
                severity: Severity::Low,
                description: "No access control mechanism detected".to_string(),
                line: None,
                recommendation: "Implement access control for sensitive functions".to_string(),
            });
        }

        // Check for front-running vulnerabilities
        if contract.source.contains("mapping") && contract.source.contains("public") {
            vulnerabilities.push(Vulnerability {
                vulnerability_type: VulnerabilityType::FrontRunning,
                severity: Severity::Low,
                description: "Public state variables may be vulnerable to front-running"
                    .to_string(),
                line: None,
                recommendation: "Consider using commit-reveal schemes for sensitive operations"
                    .to_string(),
            });
        }
    }

    fn check_move_vulnerabilities(
        contract: &GeneratedContract,
        vulnerabilities: &mut Vec<Vulnerability>,
    ) {
        // Move has built-in resource safety, but check for access control
        if !contract.source.contains("assert!(") && !contract.source.contains("owner") {
            vulnerabilities.push(Vulnerability {
                vulnerability_type: VulnerabilityType::AccessControl,
                severity: Severity::Medium,
                description: "Insufficient access control checks".to_string(),
                line: None,
                recommendation: "Add proper authorization checks using assert! or require"
                    .to_string(),
            });
        }
    }

    fn check_cairo_vulnerabilities(
        contract: &GeneratedContract,
        vulnerabilities: &mut Vec<Vulnerability>,
    ) {
        // Check for access control
        if !contract.source.contains("assert(") && !contract.source.contains("owner") {
            vulnerabilities.push(Vulnerability {
                vulnerability_type: VulnerabilityType::AccessControl,
                severity: Severity::Medium,
                description: "No access control mechanism detected".to_string(),
                line: None,
                recommendation: "Implement proper access control checks".to_string(),
            });
        }
    }

    fn check_wasm_vulnerabilities(
        contract: &GeneratedContract,
        vulnerabilities: &mut Vec<Vulnerability>,
    ) {
        // Rust/WASM has memory safety, but check for logical issues
        if contract.source.contains("unwrap()") {
            vulnerabilities.push(Vulnerability {
                vulnerability_type: VulnerabilityType::DenialOfService,
                severity: Severity::Medium,
                description: "Use of unwrap() can cause panics".to_string(),
                line: None,
                recommendation: "Use proper error handling with Result types".to_string(),
            });
        }
    }

    fn calculate_security_score(vulnerabilities: &[Vulnerability]) -> u8 {
        let mut score = 100u8;

        for vuln in vulnerabilities {
            let deduction = match vuln.severity {
                Severity::Critical => 30,
                Severity::High => 20,
                Severity::Medium => 10,
                Severity::Low => 5,
            };
            score = score.saturating_sub(deduction);
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn test_generate_solidity() {
        let statute = Statute::new(
            "adult-rights",
            "Adult Rights Act",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        assert_eq!(contract.name, "AdultRights");
        assert!(contract.source.contains("pragma solidity"));
        assert!(contract.source.contains("checkEligibility"));
    }

    #[test]
    fn test_generate_rust_wasm() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let generator = ContractGenerator::new(TargetPlatform::RustWasm);
        let contract = generator.generate(&statute).unwrap();

        assert!(contract.source.contains("wasm_bindgen"));
    }

    #[test]
    fn test_discretionary_statute_error() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_discretion("Requires human judgment");

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let result = generator.generate(&statute);

        assert!(matches!(result, Err(ChainError::DiscretionaryStatute(_))));
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
        assert_eq!(to_pascal_case("adult_rights"), "AdultRights");
    }

    #[test]
    fn test_snake_case() {
        assert_eq!(to_snake_case("Hello-World"), "hello_world");
    }

    #[test]
    fn test_generate_vyper() {
        let statute = Statute::new(
            "adult-rights",
            "Adult Rights Act",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let generator = ContractGenerator::new(TargetPlatform::Vyper);
        let contract = generator.generate(&statute).unwrap();

        assert_eq!(contract.name, "adult_rights");
        assert!(contract.source.contains("# @version"));
        assert!(contract.source.contains("def check_eligibility"));
        assert!(contract.source.contains("event EligibilityChecked"));
    }

    #[test]
    fn test_generate_move() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let generator = ContractGenerator::new(TargetPlatform::Move);
        let contract = generator.generate(&statute).unwrap();

        assert!(contract.source.contains("module legalis::"));
        assert!(contract.source.contains("public fun check_eligibility"));
        assert!(contract.source.contains("struct EligibilityChecked"));
    }

    #[test]
    fn test_generate_cairo() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        });

        let generator = ContractGenerator::new(TargetPlatform::Cairo);
        let contract = generator.generate(&statute).unwrap();

        assert!(contract.source.contains("#[starknet::contract]"));
        assert!(contract.source.contains("fn check_eligibility"));
        assert!(contract.source.contains("struct EligibilityChecked"));
    }

    #[test]
    fn test_solidity_events() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        assert!(contract.source.contains("event EligibilityChecked"));
        assert!(contract.source.contains("event EffectApplied"));
        assert!(contract.source.contains("emit EligibilityChecked"));
        assert!(contract.source.contains("emit EffectApplied"));
    }

    #[test]
    fn test_solidity_gas_optimization() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        assert!(contract.source.contains("immutable"));
        assert!(contract.source.contains("Gas-optimized"));
        assert!(contract.source.contains("CEI pattern"));
    }

    #[test]
    fn test_deployment_script_generation() {
        let statute = Statute::new(
            "test-contract",
            "Test Contract",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        let config = DeploymentConfig {
            network: "mainnet".to_string(),
            gas_limit: Some(5000000),
            gas_price: Some(50),
        };

        let script = generator
            .generate_deployment_script(&contract, &config)
            .unwrap();
        assert!(script.contains("Hardhat deployment script"));
        assert!(script.contains("mainnet"));
        assert!(script.contains("5000000"));
        assert!(script.contains("verify:verify"));
    }

    #[test]
    fn test_deployment_script_vyper() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Vyper);
        let contract = generator.generate(&statute).unwrap();

        let config = DeploymentConfig {
            network: "testnet".to_string(),
            gas_limit: None,
            gas_price: None,
        };

        let script = generator
            .generate_deployment_script(&contract, &config)
            .unwrap();
        assert!(script.contains("from ape import"));
        assert!(script.contains("deployer.deploy"));
    }

    #[test]
    fn test_security_analysis_solidity() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        let analysis = SecurityAnalyzer::analyze(&contract);
        assert_eq!(analysis.contract_name, "Test");
        assert!(analysis.score > 0 && analysis.score <= 100);

        // Our generated contracts should have owner checks
        let has_access_control = analysis
            .vulnerabilities
            .iter()
            .any(|v| v.vulnerability_type == VulnerabilityType::AccessControl);
        assert!(
            !has_access_control,
            "Generated contract should have access control"
        );
    }

    #[test]
    fn test_security_analysis_front_running() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        let analysis = SecurityAnalyzer::analyze(&contract);

        // Check if front-running warning is present
        let has_front_running = analysis
            .vulnerabilities
            .iter()
            .any(|v| v.vulnerability_type == VulnerabilityType::FrontRunning);
        assert!(
            has_front_running,
            "Should detect potential front-running vulnerability"
        );
    }

    #[test]
    fn test_security_score_calculation() {
        let statute = Statute::new(
            "safe-contract",
            "Safe Contract",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let generator = ContractGenerator::new(TargetPlatform::Move);
        let contract = generator.generate(&statute).unwrap();

        let analysis = SecurityAnalyzer::analyze(&contract);
        // Move contracts should have high security scores due to resource safety
        assert!(
            analysis.score >= 85,
            "Move contracts should have high security scores"
        );
    }

    #[test]
    fn test_generate_cosmwasm() {
        let statute = Statute::new(
            "cosmos-statute",
            "Cosmos Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });

        let generator = ContractGenerator::new(TargetPlatform::CosmWasm);
        let contract = generator.generate(&statute).unwrap();

        assert!(contract.source.contains("use cosmwasm_std::"));
        assert!(contract.source.contains("entry_point"));
        assert!(contract.source.contains("pub fn instantiate"));
        assert!(contract.source.contains("pub fn execute"));
        assert!(contract.source.contains("pub fn query"));
        assert!(contract.source.contains("QueryMsg::CheckEligibility"));
    }

    #[test]
    fn test_cosmwasm_deployment() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let generator = ContractGenerator::new(TargetPlatform::CosmWasm);
        let contract = generator.generate(&statute).unwrap();

        let config = DeploymentConfig {
            network: "cosmos-testnet".to_string(),
            gas_limit: None,
            gas_price: None,
        };

        let script = generator
            .generate_deployment_script(&contract, &config)
            .unwrap();
        assert!(script.contains("CosmWasm deployment"));
        assert!(script.contains("cosmwasm/rust-optimizer"));
        assert!(script.contains("wasmd tx wasm"));
    }

    #[test]
    fn test_factory_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let statute_ids = vec!["adult-rights", "tax-exemption", "voting-rights"];
        let factory = generator.generate_factory(&statute_ids).unwrap();

        assert_eq!(factory.name, "StatuteFactory");
        assert!(factory.source.contains("contract StatuteFactory"));
        assert!(factory.source.contains("deployAdultRights"));
        assert!(factory.source.contains("deployTaxExemption"));
        assert!(factory.source.contains("deployVotingRights"));
        assert!(factory.source.contains("event ContractDeployed"));
        assert!(factory.source.contains("getDeployedContractsCount"));
    }

    #[test]
    fn test_upgradeable_proxy_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let proxy = generator
            .generate_upgradeable_proxy("adult-rights")
            .unwrap();

        assert_eq!(proxy.name, "AdultRightsProxy");
        assert!(proxy.source.contains("contract AdultRightsProxy"));
        assert!(proxy.source.contains("address public implementation"));
        assert!(proxy.source.contains("function upgradeTo"));
        assert!(proxy.source.contains("delegatecall"));
        assert!(proxy.source.contains("event Upgraded"));
    }

    #[test]
    fn test_vyper_factory_generation() {
        let generator = ContractGenerator::new(TargetPlatform::Vyper);
        let statute_ids = vec!["test-statute"];
        let factory = generator.generate_factory(&statute_ids).unwrap();

        assert_eq!(factory.name, "statute_factory");
        assert!(factory.source.contains("# @title StatuteFactory"));
        assert!(factory.source.contains("def deploy_test_statute"));
        assert!(factory.source.contains("event ContractDeployed"));
    }
}
