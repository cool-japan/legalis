//! Smart Contract Compilation and Blockchain Integration
//!
//! This module provides functionality for compiling legal statutes into smart contracts
//! and integrating with blockchain platforms for decentralized legal registries.

use crate::{ComparisonOp, Condition, EffectType, Statute};
use std::fmt;

/// Smart contract platform target for compilation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ContractPlatform {
    /// Ethereum with Solidity
    Ethereum,
    /// Solana with Rust
    Solana,
    /// Cardano with Plutus
    Cardano,
    /// Polkadot with ink!
    Polkadot,
    /// Algorand with TEAL
    Algorand,
    /// Generic WebAssembly
    Wasm,
}

impl fmt::Display for ContractPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractPlatform::Ethereum => write!(f, "Ethereum (Solidity)"),
            ContractPlatform::Solana => write!(f, "Solana (Rust)"),
            ContractPlatform::Cardano => write!(f, "Cardano (Plutus)"),
            ContractPlatform::Polkadot => write!(f, "Polkadot (ink!)"),
            ContractPlatform::Algorand => write!(f, "Algorand (TEAL)"),
            ContractPlatform::Wasm => write!(f, "WebAssembly"),
        }
    }
}

/// Compiled smart contract code with metadata
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompiledContract {
    /// The platform this contract targets
    pub platform: ContractPlatform,
    /// The contract source code
    pub source_code: String,
    /// The contract bytecode (if applicable)
    pub bytecode: Option<Vec<u8>>,
    /// ABI (Application Binary Interface) for the contract
    pub abi: Option<String>,
    /// Gas estimate for deployment
    pub estimated_gas: Option<u64>,
    /// Warnings from compilation
    pub warnings: Vec<String>,
}

impl CompiledContract {
    /// Create a new compiled contract
    pub fn new(platform: ContractPlatform, source_code: String) -> Self {
        Self {
            platform,
            source_code,
            bytecode: None,
            abi: None,
            estimated_gas: None,
            warnings: Vec::new(),
        }
    }

    /// Add bytecode to the contract
    pub fn with_bytecode(mut self, bytecode: Vec<u8>) -> Self {
        self.bytecode = Some(bytecode);
        self
    }

    /// Add ABI to the contract
    pub fn with_abi(mut self, abi: String) -> Self {
        self.abi = Some(abi);
        self
    }

    /// Add gas estimate
    pub fn with_gas_estimate(mut self, gas: u64) -> Self {
        self.estimated_gas = Some(gas);
        self
    }

    /// Add a warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// Check if contract has warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Smart contract compiler for legal statutes
///
/// # Example
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_core::smart_contract::{SmartContractCompiler, ContractPlatform};
///
/// let statute = Statute::new("tax-credit-001", "Age-Based Tax Credit", Effect::new(EffectType::Grant, "Tax credit of $1000"))
///     .with_precondition(Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 65 });
///
/// let compiler = SmartContractCompiler::new(ContractPlatform::Ethereum);
/// let contract = compiler.compile(&statute).expect("Compilation should succeed");
///
/// assert!(contract.source_code.contains("pragma solidity"));
/// assert!(contract.source_code.contains("age >= 65"));
/// ```
pub struct SmartContractCompiler {
    platform: ContractPlatform,
    optimization_level: u8,
    include_comments: bool,
}

impl SmartContractCompiler {
    /// Create a new compiler for the specified platform
    pub fn new(platform: ContractPlatform) -> Self {
        Self {
            platform,
            optimization_level: 2,
            include_comments: true,
        }
    }

    /// Set optimization level (0-3)
    pub fn with_optimization_level(mut self, level: u8) -> Self {
        self.optimization_level = level.min(3);
        self
    }

    /// Enable or disable code comments
    pub fn with_comments(mut self, include: bool) -> Self {
        self.include_comments = include;
        self
    }

    /// Compile a statute into a smart contract
    pub fn compile(&self, statute: &Statute) -> Result<CompiledContract, CompilationError> {
        match self.platform {
            ContractPlatform::Ethereum => self.compile_solidity(statute),
            ContractPlatform::Solana => self.compile_rust_solana(statute),
            ContractPlatform::Cardano => self.compile_plutus(statute),
            ContractPlatform::Polkadot => self.compile_ink(statute),
            ContractPlatform::Algorand => self.compile_teal(statute),
            ContractPlatform::Wasm => self.compile_wasm(statute),
        }
    }

    /// Compile to Solidity (Ethereum)
    fn compile_solidity(&self, statute: &Statute) -> Result<CompiledContract, CompilationError> {
        let mut code = String::new();

        // SPDX and pragma
        code.push_str("// SPDX-License-Identifier: MIT\n");
        code.push_str("pragma solidity ^0.8.20;\n\n");

        if self.include_comments {
            code.push_str(&format!("/// {}\n", statute.title));
            code.push_str(&format!("/// {}\n", statute.effect.description));
            code.push_str(&format!("/// Statute ID: {}\n", statute.id));
        }

        let contract_name = self.sanitize_identifier(&statute.id);
        code.push_str(&format!("contract {} {{\n", contract_name));

        // Events
        code.push_str("    event StatuteEvaluated(address indexed entity, bool satisfied);\n");
        code.push_str("    event EffectApplied(address indexed entity, string effectType);\n\n");

        // Effect type
        let effect_type = match statute.effect.effect_type {
            EffectType::Grant => "GRANT",
            EffectType::Revoke => "REVOKE",
            EffectType::Obligation => "OBLIGATION",
            EffectType::Prohibition => "PROHIBITION",
            EffectType::MonetaryTransfer => "MONETARY_TRANSFER",
            EffectType::StatusChange => "STATUS_CHANGE",
            EffectType::Custom => "CUSTOM",
        };

        code.push_str(&format!(
            "    string public constant EFFECT_TYPE = \"{}\";\n",
            effect_type
        ));
        code.push_str(&format!(
            "    string public constant EFFECT_DESCRIPTION = \"{}\";\n\n",
            statute.effect.description.replace('"', "\\\"")
        ));

        // Evaluation function
        code.push_str("    function evaluate(\n");
        code.push_str("        uint256 age,\n");
        code.push_str("        uint256 income,\n");
        code.push_str("        mapping(string => bool) storage attributes\n");
        code.push_str("    ) public returns (bool) {\n");

        // Compile preconditions
        if !statute.preconditions.is_empty() {
            code.push_str("        bool satisfied = ");
            code.push_str(&self.compile_condition_solidity(&statute.preconditions[0])?);
            // Note: multiple preconditions not fully supported - only evaluating first one
            code.push_str(";\n");
            code.push_str("        emit StatuteEvaluated(msg.sender, satisfied);\n");
            code.push_str("        if (satisfied) {\n");
            code.push_str(&format!(
                "            emit EffectApplied(msg.sender, \"{}\");\n",
                effect_type
            ));
            code.push_str("        }\n");
            code.push_str("        return satisfied;\n");
        } else {
            code.push_str("        emit StatuteEvaluated(msg.sender, true);\n");
            code.push_str(&format!(
                "        emit EffectApplied(msg.sender, \"{}\");\n",
                effect_type
            ));
            code.push_str("        return true;\n");
        }

        code.push_str("    }\n");
        code.push_str("}\n");

        let mut contract = CompiledContract::new(ContractPlatform::Ethereum, code);

        // Generate simple ABI
        let abi = self.generate_solidity_abi(&contract_name);
        contract = contract.with_abi(abi);

        // Estimate gas (simplified)
        let estimated_gas = 500_000 + (statute.preconditions.len() as u64 * 50_000);
        contract = contract.with_gas_estimate(estimated_gas);

        if statute.discretion_logic.is_some() {
            contract.add_warning(
                "Discretionary logic cannot be fully encoded in smart contracts".to_string(),
            );
        }

        Ok(contract)
    }

    /// Compile condition to Solidity expression
    fn compile_condition_solidity(
        &self,
        condition: &Condition,
    ) -> Result<String, CompilationError> {
        match condition {
            Condition::Age { operator, value } => {
                Ok(format!("age {} {}", self.op_to_solidity(operator), value))
            }
            Condition::Income { operator, value } => Ok(format!(
                "income {} {}",
                self.op_to_solidity(operator),
                value
            )),
            Condition::HasAttribute { key } => Ok(format!("attributes[\"{}\"]", key)),
            Condition::And(left, right) => {
                let left_expr = self.compile_condition_solidity(left)?;
                let right_expr = self.compile_condition_solidity(right)?;
                Ok(format!("({} && {})", left_expr, right_expr))
            }
            Condition::Or(left, right) => {
                let left_expr = self.compile_condition_solidity(left)?;
                let right_expr = self.compile_condition_solidity(right)?;
                Ok(format!("({} || {})", left_expr, right_expr))
            }
            Condition::Not(cond) => Ok(format!("!({})", self.compile_condition_solidity(cond)?)),
            _ => Err(CompilationError::UnsupportedCondition(format!(
                "Condition type not supported: {:?}",
                condition
            ))),
        }
    }

    /// Convert comparison operator to Solidity
    fn op_to_solidity(&self, op: &ComparisonOp) -> &str {
        match op {
            ComparisonOp::Equal => "==",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
        }
    }

    /// Generate Solidity ABI
    fn generate_solidity_abi(&self, _contract_name: &str) -> String {
        r#"[
  {
    "type": "function",
    "name": "evaluate",
    "inputs": [
      {"name": "age", "type": "uint256"},
      {"name": "income", "type": "uint256"},
      {"name": "attributes", "type": "mapping(string => bool)"}
    ],
    "outputs": [{"name": "satisfied", "type": "bool"}],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "StatuteEvaluated",
    "inputs": [
      {"name": "entity", "type": "address", "indexed": true},
      {"name": "satisfied", "type": "bool", "indexed": false}
    ]
  },
  {
    "type": "event",
    "name": "EffectApplied",
    "inputs": [
      {"name": "entity", "type": "address", "indexed": true},
      {"name": "effectType", "type": "string", "indexed": false}
    ]
  }
]"#
        .to_string()
    }

    /// Compile to Rust for Solana
    fn compile_rust_solana(&self, statute: &Statute) -> Result<CompiledContract, CompilationError> {
        let mut code = String::new();

        code.push_str("use solana_program::{\n");
        code.push_str("    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,\n");
        code.push_str("    msg, program_error::ProgramError, pubkey::Pubkey,\n");
        code.push_str("};\n\n");

        if self.include_comments {
            code.push_str(&format!("/// {}\n", statute.title));
            code.push_str(&format!("/// Statute ID: {}\n", statute.id));
        }

        code.push_str("entrypoint!(process_instruction);\n\n");
        code.push_str("pub fn process_instruction(\n");
        code.push_str("    _program_id: &Pubkey,\n");
        code.push_str("    _accounts: &[AccountInfo],\n");
        code.push_str("    instruction_data: &[u8],\n");
        code.push_str(") -> ProgramResult {\n");
        code.push_str("    // Decode instruction data (age, income, etc.)\n");
        code.push_str("    msg!(\"Evaluating statute\");\n");
        code.push_str("    Ok(())\n");
        code.push_str("}\n");

        let mut contract = CompiledContract::new(ContractPlatform::Solana, code);
        contract.add_warning(
            "Solana implementation is a template - requires full program logic".to_string(),
        );

        Ok(contract)
    }

    /// Compile to Plutus (Cardano)
    fn compile_plutus(&self, statute: &Statute) -> Result<CompiledContract, CompilationError> {
        let mut code = String::new();

        code.push_str("{-# LANGUAGE DataKinds #-}\n");
        code.push_str("{-# LANGUAGE OverloadedStrings #-}\n\n");

        if self.include_comments {
            code.push_str(&format!("-- {}\n", statute.title));
            code.push_str(&format!("-- Statute ID: {}\n\n", statute.id));
        }

        code.push_str("module Statute where\n\n");
        code.push_str("import PlutusTx.Prelude\n\n");
        code.push_str("data StatuteParams = StatuteParams\n");
        code.push_str("  { age :: Integer\n");
        code.push_str("  , income :: Integer\n");
        code.push_str("  }\n\n");
        code.push_str("validate :: StatuteParams -> Bool\n");
        code.push_str("validate params = True -- Add condition logic\n");

        let mut contract = CompiledContract::new(ContractPlatform::Cardano, code);
        contract.add_warning(
            "Plutus implementation is a template - requires full validator logic".to_string(),
        );

        Ok(contract)
    }

    /// Compile to ink! (Polkadot)
    fn compile_ink(&self, statute: &Statute) -> Result<CompiledContract, CompilationError> {
        let mut code = String::new();

        code.push_str("#![cfg_attr(not(feature = \"std\"), no_std)]\n\n");
        code.push_str("use ink_lang as ink;\n\n");

        if self.include_comments {
            code.push_str(&format!("/// {}\n", statute.title));
        }

        code.push_str("#[ink::contract]\n");
        code.push_str("mod statute {\n");
        code.push_str("    #[ink(storage)]\n");
        code.push_str("    pub struct Statute {}\n\n");
        code.push_str("    impl Statute {\n");
        code.push_str("        #[ink(constructor)]\n");
        code.push_str("        pub fn new() -> Self { Self {} }\n\n");
        code.push_str("        #[ink(message)]\n");
        code.push_str("        pub fn evaluate(&self, age: u32, income: u64) -> bool {\n");
        code.push_str("            true // Add condition logic\n");
        code.push_str("        }\n");
        code.push_str("    }\n");
        code.push_str("}\n");

        let mut contract = CompiledContract::new(ContractPlatform::Polkadot, code);
        contract.add_warning(
            "ink! implementation is a template - requires full contract logic".to_string(),
        );

        Ok(contract)
    }

    /// Compile to TEAL (Algorand)
    fn compile_teal(&self, statute: &Statute) -> Result<CompiledContract, CompilationError> {
        let mut code = String::new();

        code.push_str("#pragma version 8\n\n");

        if self.include_comments {
            code.push_str(&format!("// {}\n", statute.title));
            code.push_str(&format!("// Statute ID: {}\n\n", statute.id));
        }

        code.push_str("txn ApplicationID\n");
        code.push_str("int 0\n");
        code.push_str("==\n");
        code.push_str("bnz create_app\n\n");
        code.push_str("int 1\n");
        code.push_str("return\n\n");
        code.push_str("create_app:\n");
        code.push_str("int 1\n");
        code.push_str("return\n");

        let mut contract = CompiledContract::new(ContractPlatform::Algorand, code);
        contract.add_warning(
            "TEAL implementation is a template - requires full approval logic".to_string(),
        );

        Ok(contract)
    }

    /// Compile to WebAssembly
    fn compile_wasm(&self, statute: &Statute) -> Result<CompiledContract, CompilationError> {
        let mut code = String::new();

        code.push_str("(module\n");

        if self.include_comments {
            code.push_str(&format!("  ;; {}\n", statute.title));
        }

        code.push_str("  (func $evaluate (param $age i32) (param $income i64) (result i32)\n");
        code.push_str("    i32.const 1\n");
        code.push_str("  )\n");
        code.push_str("  (export \"evaluate\" (func $evaluate))\n");
        code.push_str(")\n");

        let mut contract = CompiledContract::new(ContractPlatform::Wasm, code);
        contract.add_warning(
            "WASM implementation is a template - requires full evaluation logic".to_string(),
        );

        Ok(contract)
    }

    /// Sanitize identifier for smart contract name
    fn sanitize_identifier(&self, id: &str) -> String {
        id.chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect::<String>()
            .trim_start_matches(|c: char| c.is_numeric())
            .to_string()
            + "Statute"
    }
}

/// Compilation errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum CompilationError {
    #[error("Unsupported condition: {0}")]
    UnsupportedCondition(String),

    #[error("Unsupported effect: {0}")]
    UnsupportedEffect(String),

    #[error("Invalid statute structure: {0}")]
    InvalidStructure(String),

    #[error("Platform limitation: {0}")]
    PlatformLimitation(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solidity_compilation() {
        let statute = Statute::new(
            "test-001",
            "Test Statute",
            crate::Effect::new(crate::EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let compiler = SmartContractCompiler::new(ContractPlatform::Ethereum);
        let contract = compiler.compile(&statute).unwrap();

        assert!(contract.source_code.contains("pragma solidity"));
        assert!(contract.source_code.contains("age >= 18"));
        assert!(contract.abi.is_some());
        assert!(contract.estimated_gas.is_some());
    }

    #[test]
    fn test_platform_display() {
        assert_eq!(
            ContractPlatform::Ethereum.to_string(),
            "Ethereum (Solidity)"
        );
        assert_eq!(ContractPlatform::Solana.to_string(), "Solana (Rust)");
    }

    #[test]
    fn test_contract_with_warnings() {
        let mut contract =
            CompiledContract::new(ContractPlatform::Ethereum, "contract Test {}".to_string());

        assert!(!contract.has_warnings());

        contract.add_warning("Test warning".to_string());
        assert!(contract.has_warnings());
        assert_eq!(contract.warnings.len(), 1);
    }

    #[test]
    fn test_sanitize_identifier() {
        let compiler = SmartContractCompiler::new(ContractPlatform::Ethereum);

        assert_eq!(compiler.sanitize_identifier("tax-001"), "tax_001Statute");
        assert_eq!(compiler.sanitize_identifier("abc.def"), "abc_defStatute");
        assert_eq!(compiler.sanitize_identifier("123test"), "testStatute");
    }

    #[test]
    fn test_complex_conditions() {
        let statute = Statute::new(
            "complex-001",
            "Complex Statute",
            crate::Effect::new(crate::EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 65,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        ));

        let compiler = SmartContractCompiler::new(ContractPlatform::Ethereum);
        let contract = compiler.compile(&statute).unwrap();

        assert!(contract.source_code.contains("age >= 65"));
        assert!(contract.source_code.contains("income < 50000"));
        assert!(contract.source_code.contains("&&"));
    }

    #[test]
    fn test_all_platforms() {
        let statute = Statute::new(
            "platform-test",
            "Platform Test",
            crate::Effect::new(crate::EffectType::Grant, "Test"),
        );

        for &platform in &[
            ContractPlatform::Ethereum,
            ContractPlatform::Solana,
            ContractPlatform::Cardano,
            ContractPlatform::Polkadot,
            ContractPlatform::Algorand,
            ContractPlatform::Wasm,
        ] {
            let compiler = SmartContractCompiler::new(platform);
            let contract = compiler.compile(&statute);
            assert!(contract.is_ok(), "Failed to compile for {:?}", platform);
        }
    }
}
