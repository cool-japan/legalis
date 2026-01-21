//! Solidity smart contract to legal format conversion.
//!
//! Solidity is the primary programming language for Ethereum and EVM-compatible
//! blockchain smart contracts. This module extracts legal semantics from:
//! - Contract functions and their modifiers
//! - Access control (onlyOwner, etc.)
//! - State-changing operations
//! - NatSpec documentation comments
//! - Events and their parameters
//!
//! This module provides bidirectional conversion between Solidity contracts
//! and legalis_core::Statute format.

use crate::{ConversionReport, FormatExporter, FormatImporter, InteropResult, LegalFormat};
use legalis_core::{Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// Solidity contract structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolidityContract {
    /// Contract name
    pub name: String,
    /// Contract version (from pragma)
    pub solidity_version: String,
    /// SPDX license identifier
    pub license: Option<String>,
    /// NatSpec contract documentation
    pub documentation: Option<String>,
    /// Contract functions
    pub functions: Vec<SolidityFunction>,
    /// Contract modifiers
    pub modifiers: Vec<SolidityModifier>,
    /// State variables
    pub state_variables: Vec<SolidityStateVariable>,
    /// Events
    pub events: Vec<SolidityEvent>,
}

/// Solidity function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolidityFunction {
    /// Function name
    pub name: String,
    /// Function visibility (public, external, internal, private)
    pub visibility: String,
    /// State mutability (pure, view, payable, or empty for state-changing)
    pub mutability: Option<String>,
    /// Function modifiers applied (e.g., onlyOwner)
    pub modifiers: Vec<String>,
    /// Function parameters
    pub parameters: Vec<SolidityParameter>,
    /// Return parameters
    pub returns: Vec<SolidityParameter>,
    /// NatSpec documentation
    pub documentation: Option<String>,
    /// Function body (simplified)
    pub body: Option<String>,
}

/// Solidity modifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolidityModifier {
    /// Modifier name
    pub name: String,
    /// Modifier parameters
    pub parameters: Vec<SolidityParameter>,
    /// Modifier body/requirement
    pub requirement: String,
    /// NatSpec documentation
    pub documentation: Option<String>,
}

/// Solidity parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolidityParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type (address, uint256, bool, etc.)
    pub param_type: String,
}

/// Solidity state variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolidityStateVariable {
    /// Variable name
    pub name: String,
    /// Variable type
    pub var_type: String,
    /// Visibility
    pub visibility: String,
    /// Whether it's constant
    pub is_constant: bool,
    /// Whether it's immutable
    pub is_immutable: bool,
}

/// Solidity event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolidityEvent {
    /// Event name
    pub name: String,
    /// Event parameters
    pub parameters: Vec<SolidityEventParameter>,
    /// NatSpec documentation
    pub documentation: Option<String>,
}

/// Solidity event parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolidityEventParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
    /// Whether parameter is indexed
    pub indexed: bool,
}

/// Solidity importer
pub struct SolidityImporter;

impl SolidityImporter {
    /// Creates a new Solidity importer
    pub fn new() -> Self {
        Self
    }

    fn parse_solidity(&self, source: &str) -> InteropResult<SolidityContract> {
        // Try to parse as JSON first (for structured contract metadata)
        if let Ok(contract) = serde_json::from_str::<SolidityContract>(source) {
            return Ok(contract);
        }

        // Simple Solidity source code parser
        self.parse_solidity_source(source)
    }

    fn parse_solidity_source(&self, source: &str) -> InteropResult<SolidityContract> {
        let mut contract = SolidityContract {
            name: "Contract".to_string(),
            solidity_version: "^0.8.0".to_string(),
            license: None,
            documentation: None,
            functions: Vec::new(),
            modifiers: Vec::new(),
            state_variables: Vec::new(),
            events: Vec::new(),
        };

        // Extract SPDX license
        if let Some(license_line) = source
            .lines()
            .find(|l| l.contains("SPDX-License-Identifier"))
            && let Some(license) = license_line.split(':').nth(1)
        {
            contract.license = Some(license.trim().to_string());
        }

        // Extract pragma version
        if let Some(pragma_line) = source.lines().find(|l| l.contains("pragma solidity"))
            && let Some(version) = pragma_line.split("solidity").nth(1)
        {
            contract.solidity_version = version.trim().trim_end_matches(';').to_string();
        }

        // Extract contract name
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("contract ") || trimmed.starts_with("abstract contract ") {
                let name_start = trimmed.find("contract ").unwrap() + "contract ".len();
                if let Some(name_end) =
                    trimmed[name_start..].find(|c: char| c.is_whitespace() || c == '{')
                {
                    contract.name = trimmed[name_start..name_start + name_end].to_string();
                    break;
                }
            }
        }

        // Extract functions (simplified)
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("function ")
                && !trimmed.contains(';')
                && let Some(func) = self.parse_function_signature(trimmed)
            {
                contract.functions.push(func);
            }
        }

        Ok(contract)
    }

    fn parse_function_signature(&self, line: &str) -> Option<SolidityFunction> {
        let name_start = line.find("function ")? + "function ".len();
        let name_end = line[name_start..].find('(')?;
        let name = line[name_start..name_start + name_end].to_string();

        let mut visibility = "public".to_string();
        let mut mutability = None;
        let modifiers = Vec::new();

        // Extract visibility
        for vis in &["public", "external", "internal", "private"] {
            if line.contains(vis) {
                visibility = vis.to_string();
                break;
            }
        }

        // Extract mutability
        for mut_type in &["pure", "view", "payable"] {
            if line.contains(mut_type) {
                mutability = Some(mut_type.to_string());
                break;
            }
        }

        Some(SolidityFunction {
            name,
            visibility,
            mutability,
            modifiers,
            parameters: Vec::new(),
            returns: Vec::new(),
            documentation: None,
            body: None,
        })
    }

    fn convert_to_statutes(&self, contract: &SolidityContract) -> Vec<Statute> {
        let mut statutes = Vec::new();

        // Convert each function to a statute
        for function in &contract.functions {
            let statute_id = format!(
                "{}_{}",
                contract.name.to_lowercase(),
                function.name.to_lowercase()
            );
            let title = format!("{} - {}", contract.name, function.name);

            let effect_type = match function.mutability.as_deref() {
                Some("view") | Some("pure") => EffectType::Custom,
                Some("payable") => EffectType::MonetaryTransfer,
                _ => EffectType::Grant,
            };

            let effect_value = function.documentation.as_deref().unwrap_or(&function.name);

            let mut statute =
                Statute::new(&statute_id, &title, Effect::new(effect_type, effect_value));

            // Add visibility as a precondition
            if function.visibility != "public" {
                statute = statute.with_precondition(Condition::Custom {
                    description: format!("Visibility: {}", function.visibility),
                });
            }

            // Add modifiers as preconditions
            for modifier in &function.modifiers {
                statute = statute.with_precondition(Condition::Custom {
                    description: format!("Requires: {}", modifier),
                });
            }

            // Add contract metadata
            statute
                .effect
                .parameters
                .insert("contract".to_string(), contract.name.clone());
            statute
                .effect
                .parameters
                .insert("function".to_string(), function.name.clone());
            if let Some(license) = &contract.license {
                statute
                    .effect
                    .parameters
                    .insert("license".to_string(), license.clone());
            }

            statutes.push(statute);
        }

        statutes
    }
}

impl Default for SolidityImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for SolidityImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Solidity
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let contract = self.parse_solidity(source)?;
        let statutes = self.convert_to_statutes(&contract);

        let mut report = ConversionReport::new(LegalFormat::Solidity, LegalFormat::Legalis);
        report.statutes_converted = statutes.len();

        if contract.modifiers.is_empty() {
            report.add_warning("Contract has no modifiers - access control may be incomplete");
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("contract ")
            || source.contains("function ")
            || source.contains("pragma solidity")
            || (source.contains('{')
                && (source.contains("\"name\"") || source.contains("\"functions\"")))
    }
}

/// Solidity exporter
pub struct SolidityExporter;

impl SolidityExporter {
    /// Creates a new Solidity exporter
    pub fn new() -> Self {
        Self
    }

    fn convert_to_solidity(&self, statutes: &[Statute]) -> SolidityContract {
        let contract_name = statutes
            .first()
            .and_then(|s| s.effect.parameters.get("contract"))
            .cloned()
            .unwrap_or_else(|| "LegalisContract".to_string());

        let license = statutes
            .first()
            .and_then(|s| s.effect.parameters.get("license"))
            .cloned();

        let mut functions = Vec::new();

        for statute in statutes {
            let function_name = statute
                .effect
                .parameters
                .get("function")
                .cloned()
                .unwrap_or_else(|| statute.id.clone());

            let mutability = match statute.effect.effect_type {
                EffectType::Custom => Some("view".to_string()),
                EffectType::MonetaryTransfer => Some("payable".to_string()),
                _ => None,
            };

            let modifiers: Vec<String> = statute
                .preconditions
                .iter()
                .filter_map(|cond| {
                    if let Condition::Custom { description, .. } = cond {
                        if description.starts_with("Requires: ") {
                            Some(description.strip_prefix("Requires: ").unwrap().to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            functions.push(SolidityFunction {
                name: function_name,
                visibility: "public".to_string(),
                mutability,
                modifiers,
                parameters: Vec::new(),
                returns: Vec::new(),
                documentation: Some(statute.effect.description.clone()),
                body: None,
            });
        }

        SolidityContract {
            name: contract_name,
            solidity_version: "^0.8.0".to_string(),
            license,
            documentation: Some("Generated from Legalis statutes".to_string()),
            functions,
            modifiers: Vec::new(),
            state_variables: Vec::new(),
            events: Vec::new(),
        }
    }

    fn generate_solidity_source(&self, contract: &SolidityContract) -> String {
        let mut output = String::new();

        // SPDX license
        if let Some(license) = &contract.license {
            output.push_str(&format!("// SPDX-License-Identifier: {}\n", license));
        }

        // Pragma
        output.push_str(&format!(
            "pragma solidity {};\n\n",
            contract.solidity_version
        ));

        // Contract documentation
        if let Some(doc) = &contract.documentation {
            output.push_str(&format!("/// @title {}\n", doc));
        }

        // Contract declaration
        output.push_str(&format!("contract {} {{\n", contract.name));

        // Functions
        for func in &contract.functions {
            output.push('\n');

            // Function documentation
            if let Some(doc) = &func.documentation {
                output.push_str(&format!("    /// @notice {}\n", doc));
            }

            // Function signature
            output.push_str(&format!("    function {}()", func.name));

            // Visibility
            output.push_str(&format!(" {}", func.visibility));

            // Mutability
            if let Some(mutability) = &func.mutability {
                output.push_str(&format!(" {}", mutability));
            }

            // Modifiers
            for modifier in &func.modifiers {
                output.push_str(&format!(" {}", modifier));
            }

            output.push_str(" {\n");
            output.push_str("        // Implementation\n");
            output.push_str("    }\n");
        }

        output.push_str("}\n");

        output
    }
}

impl Default for SolidityExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for SolidityExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Solidity
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let contract = self.convert_to_solidity(statutes);
        let output = self.generate_solidity_source(&contract);

        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Solidity);
        report.statutes_converted = statutes.len();

        if statutes.is_empty() {
            report.add_warning("No statutes to convert");
        }

        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solidity_source_parsing() {
        let importer = SolidityImporter::new();

        let source = r#"
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.0;

        contract SimpleStorage {
            function store(uint256 value) public {
            }

            function retrieve() public view returns (uint256) {
            }
        }
        "#;

        let contract = importer.parse_solidity_source(source).unwrap();
        assert_eq!(contract.name, "SimpleStorage");
        assert_eq!(contract.license, Some("MIT".to_string()));
        assert_eq!(contract.functions.len(), 2);
    }

    #[test]
    fn test_solidity_json_parsing() {
        let importer = SolidityImporter::new();

        let json = r#"{
            "name": "Token",
            "solidity_version": "^0.8.0",
            "license": "MIT",
            "documentation": "ERC20 Token",
            "functions": [
                {
                    "name": "transfer",
                    "visibility": "public",
                    "mutability": null,
                    "modifiers": [],
                    "parameters": [],
                    "returns": [],
                    "documentation": "Transfer tokens",
                    "body": null
                }
            ],
            "modifiers": [],
            "state_variables": [],
            "events": []
        }"#;

        let contract = importer.parse_solidity(json).unwrap();
        assert_eq!(contract.name, "Token");
        assert_eq!(contract.functions.len(), 1);
    }

    #[test]
    fn test_solidity_import() {
        let importer = SolidityImporter::new();

        let source = r#"
        pragma solidity ^0.8.0;

        contract Voting {
            function vote() public {
            }
        }
        "#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(!statutes.is_empty());
        assert_eq!(
            statutes[0].effect.parameters.get("contract"),
            Some(&"Voting".to_string())
        );
    }

    #[test]
    fn test_solidity_export() {
        let exporter = SolidityExporter::new();

        let mut statute = Statute::new(
            "token_transfer",
            "Token Transfer",
            Effect::new(EffectType::MonetaryTransfer, "Transfer tokens"),
        );
        statute
            .effect
            .parameters
            .insert("contract".to_string(), "ERC20".to_string());
        statute
            .effect
            .parameters
            .insert("function".to_string(), "transfer".to_string());
        statute
            .effect
            .parameters
            .insert("license".to_string(), "MIT".to_string());

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("contract ERC20"));
        assert!(output.contains("function transfer()"));
        assert!(output.contains("SPDX-License-Identifier: MIT"));
    }

    #[test]
    fn test_solidity_validate() {
        let importer = SolidityImporter::new();

        assert!(importer.validate("contract Test { }"));
        assert!(importer.validate("pragma solidity ^0.8.0;"));
        assert!(importer.validate("function test() public { }"));
        assert!(!importer.validate("not solidity code"));
    }
}
