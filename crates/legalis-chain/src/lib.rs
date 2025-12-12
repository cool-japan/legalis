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
        source.push_str(&format!("contract {} {{\n", contract_name));

        // State variables
        source.push_str("    address public owner;\n");
        source.push_str("    mapping(address => bool) public eligible;\n\n");

        // Constructor
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
        })
    }

    fn generate_solidity_check_function(&self, statute: &Statute) -> ChainResult<String> {
        let mut func = String::new();
        func.push_str("    function checkEligibility(\n");

        // Generate parameters based on conditions
        let params = self.extract_parameters(&statute.preconditions);
        let param_str: Vec<String> = params
            .iter()
            .map(|(name, typ)| format!("        {} {}", typ, name))
            .collect();
        func.push_str(&param_str.join(",\n"));
        func.push_str("\n    ) public pure returns (bool) {\n");

        // Generate condition checks
        for condition in &statute.preconditions {
            func.push_str(&self.condition_to_solidity(condition)?);
        }

        func.push_str("        return true;\n");
        func.push_str("    }\n");

        Ok(func)
    }

    fn generate_solidity_apply_function(&self, statute: &Statute) -> ChainResult<String> {
        let mut func = String::new();
        func.push_str("    function applyEffect(address beneficiary) public {\n");
        func.push_str("        require(msg.sender == owner, \"Only owner can apply effects\");\n");

        match statute.effect.effect_type {
            EffectType::Grant => {
                func.push_str("        eligible[beneficiary] = true;\n");
            }
            EffectType::Revoke => {
                func.push_str("        eligible[beneficiary] = false;\n");
            }
            EffectType::MonetaryTransfer => {
                func.push_str("        // Monetary transfer logic\n");
                func.push_str("        // payable(beneficiary).transfer(amount);\n");
            }
            _ => {
                func.push_str(&format!(
                    "        // Effect: {}\n",
                    statute.effect.description
                ));
            }
        }

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
                Ok(format!("        require(age {} {}, \"Age requirement not met\");\n", op, value))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_solidity(*operator);
                Ok(format!("        require(income {} {}, \"Income requirement not met\");\n", op, value))
            }
            Condition::And(left, right) => {
                let mut result = self.condition_to_solidity(left)?;
                result.push_str(&self.condition_to_solidity(right)?);
                Ok(result)
            }
            Condition::Or(left, right) => {
                Ok(format!(
                    "        require({} || {}, \"OR condition not met\");\n",
                    self.condition_to_solidity_expr(left)?,
                    self.condition_to_solidity_expr(right)?
                ))
            }
            Condition::Not(inner) => {
                Ok(format!(
                    "        require(!{}, \"NOT condition not met\");\n",
                    self.condition_to_solidity_expr(inner)?
                ))
            }
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
            self.extract_params_from_condition(condition, &mut params);
        }

        params.sort_by(|a, b| a.0.cmp(&b.0));
        params.dedup_by(|a, b| a.0 == b.0);
        params
    }

    fn extract_params_from_condition(&self, condition: &Condition, params: &mut Vec<(String, String)>) {
        match condition {
            Condition::Age { .. } => {
                params.push(("age".to_string(), "uint256".to_string()));
            }
            Condition::Income { .. } => {
                params.push(("income".to_string(), "uint256".to_string()));
            }
            Condition::And(left, right) | Condition::Or(left, right) => {
                self.extract_params_from_condition(left, params);
                self.extract_params_from_condition(right, params);
            }
            Condition::Not(inner) => {
                self.extract_params_from_condition(inner, params);
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
        })
    }

    fn condition_to_rust(&self, condition: &Condition) -> ChainResult<String> {
        match condition {
            Condition::Age { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!("        if !(age {} {}) {{ return false; }}\n", op, value))
            }
            Condition::Income { operator, value } => {
                let op = self.comparison_to_rust(*operator);
                Ok(format!("        if !(income {} {}) {{ return false; }}\n", op, value))
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
        })
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
        let statute = Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        )
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
}
