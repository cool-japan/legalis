//! Vyper smart contract to legal format conversion.
//!
//! Vyper is a Pythonic smart contract language for Ethereum that emphasizes
//! security and auditability. This module extracts legal semantics from:
//! - Contract functions (external, internal, view, pure, payable)
//! - State variables and their visibility
//! - Events and their indexed parameters
//! - Decorators (@external, @internal, @view, @pure, @payable, @nonreentrant)
//! - NatSpec-style documentation
//!
//! This module provides bidirectional conversion between Vyper contracts
//! and legalis_core::Statute format.

use crate::{ConversionReport, FormatExporter, FormatImporter, InteropResult, LegalFormat};
use legalis_core::{Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// Vyper contract structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VyperContract {
    /// Contract name (from filename or first comment)
    pub name: String,
    /// Vyper version pragma
    pub version: String,
    /// License
    pub license: Option<String>,
    /// Contract documentation
    pub documentation: Option<String>,
    /// Contract functions
    pub functions: Vec<VyperFunction>,
    /// State variables
    pub state_variables: Vec<VyperStateVariable>,
    /// Events
    pub events: Vec<VyperEvent>,
    /// Interfaces
    pub interfaces: Vec<String>,
}

/// Vyper function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VyperFunction {
    /// Function name
    pub name: String,
    /// Function decorators (@external, @internal, @view, @pure, @payable, @nonreentrant)
    pub decorators: Vec<String>,
    /// Function parameters
    pub parameters: Vec<VyperParameter>,
    /// Return type
    pub return_type: Option<String>,
    /// Function documentation
    pub documentation: Option<String>,
}

/// Vyper parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VyperParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type (uint256, address, bool, etc.)
    pub param_type: String,
}

/// Vyper state variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VyperStateVariable {
    /// Variable name
    pub name: String,
    /// Variable type
    pub var_type: String,
    /// Visibility (public creates automatic getter)
    pub is_public: bool,
    /// Whether it's constant
    pub is_constant: bool,
    /// Whether it's immutable
    pub is_immutable: bool,
}

/// Vyper event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VyperEvent {
    /// Event name
    pub name: String,
    /// Event parameters
    pub parameters: Vec<VyperEventParameter>,
}

/// Vyper event parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VyperEventParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
    /// Whether parameter is indexed
    pub indexed: bool,
}

/// Vyper importer
pub struct VyperImporter;

impl VyperImporter {
    /// Creates a new Vyper importer
    pub fn new() -> Self {
        Self
    }

    fn parse_vyper(&self, source: &str) -> InteropResult<VyperContract> {
        // Try to parse as JSON first (for structured contract metadata)
        if let Ok(contract) = serde_json::from_str::<VyperContract>(source) {
            return Ok(contract);
        }

        // Simple Vyper source code parser
        self.parse_vyper_source(source)
    }

    fn parse_vyper_source(&self, source: &str) -> InteropResult<VyperContract> {
        let mut contract = VyperContract {
            name: "Contract".to_string(),
            version: "0.3.0".to_string(),
            license: None,
            documentation: None,
            functions: Vec::new(),
            state_variables: Vec::new(),
            events: Vec::new(),
            interfaces: Vec::new(),
        };

        // Extract version pragma
        if let Some(version_line) = source.lines().find(|l| l.trim().starts_with("# @version ")) {
            if let Some(version) = version_line.split("@version").nth(1) {
                contract.version = version.trim().to_string();
            }
        }

        // Extract license
        if let Some(license_line) = source
            .lines()
            .find(|l| l.contains("# @license ") || l.contains("#@license"))
        {
            if let Some(license) = license_line.split("@license").nth(1) {
                contract.license = Some(license.trim().to_string());
            }
        }

        // Extract contract name from first comment or use default
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") && !trimmed.contains("@") {
                contract.name = trimmed.strip_prefix("# ").unwrap().to_string();
                break;
            }
        }

        // Extract events
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("event ") {
                if let Some(event) = self.parse_event_declaration(trimmed) {
                    contract.events.push(event);
                }
            }
        }

        // Extract state variables
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.contains(": ")
                && !trimmed.starts_with("def ")
                && !trimmed.starts_with("#")
                && (trimmed.starts_with("public(") || !trimmed.starts_with("@"))
            {
                if let Some(var) = self.parse_state_variable(trimmed) {
                    contract.state_variables.push(var);
                }
            }
        }

        // Extract functions
        let mut current_decorators = Vec::new();
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('@') {
                current_decorators.push(trimmed.to_string());
            } else if trimmed.starts_with("def ") {
                if let Some(func) = self.parse_function_signature(trimmed, &current_decorators) {
                    contract.functions.push(func);
                }
                current_decorators.clear();
            } else if !trimmed.is_empty() && !trimmed.starts_with("#") {
                current_decorators.clear();
            }
        }

        Ok(contract)
    }

    fn parse_event_declaration(&self, line: &str) -> Option<VyperEvent> {
        let event_start = line.find("event ")? + "event ".len();
        let name_end = line[event_start..].find([':', '('])?;
        let name = line[event_start..event_start + name_end].to_string();

        Some(VyperEvent {
            name,
            parameters: Vec::new(),
        })
    }

    fn parse_state_variable(&self, line: &str) -> Option<VyperStateVariable> {
        let is_public = line.starts_with("public(");
        let is_constant = line.contains("constant(");
        let is_immutable = line.contains("immutable(");

        let colon_pos = line.find(": ")?;

        let name_start = if is_public || is_constant || is_immutable {
            let paren_start = line.find('(')?;
            paren_start + 1
        } else {
            0
        };

        let name_end = if is_public || is_constant || is_immutable {
            line[name_start..].find(')')?
        } else {
            colon_pos
        };

        let name = line[name_start..name_start + name_end].trim().to_string();

        let type_start = colon_pos + 2;
        let type_end = line[type_start..]
            .find(['=', '#'])
            .unwrap_or(line[type_start..].len());
        let var_type = line[type_start..type_start + type_end].trim().to_string();

        Some(VyperStateVariable {
            name,
            var_type,
            is_public,
            is_constant,
            is_immutable,
        })
    }

    fn parse_function_signature(&self, line: &str, decorators: &[String]) -> Option<VyperFunction> {
        let def_start = line.find("def ")? + "def ".len();
        let name_end = line[def_start..].find('(')?;
        let name = line[def_start..def_start + name_end].to_string();

        let decorators = decorators.to_vec();

        // Extract return type
        let return_type = if line.contains(" -> ") {
            let ret_start = line.find(" -> ")? + " -> ".len();
            let ret_end = line[ret_start..]
                .find(':')
                .unwrap_or(line[ret_start..].len());
            Some(line[ret_start..ret_start + ret_end].trim().to_string())
        } else {
            None
        };

        Some(VyperFunction {
            name,
            decorators,
            parameters: Vec::new(),
            return_type,
            documentation: None,
        })
    }

    fn convert_to_statutes(&self, contract: &VyperContract) -> Vec<Statute> {
        let mut statutes = Vec::new();

        // Convert each function to a statute
        for function in &contract.functions {
            let statute_id = format!(
                "{}_{}",
                contract.name.to_lowercase().replace(' ', "_"),
                function.name.to_lowercase()
            );
            let title = format!("{} - {}", contract.name, function.name);

            let effect_type = if function.decorators.contains(&"@view".to_string())
                || function.decorators.contains(&"@pure".to_string())
            {
                EffectType::Custom
            } else if function.decorators.contains(&"@payable".to_string()) {
                EffectType::MonetaryTransfer
            } else {
                EffectType::Grant
            };

            let effect_value = function.documentation.as_deref().unwrap_or(&function.name);

            let mut statute =
                Statute::new(&statute_id, &title, Effect::new(effect_type, effect_value));

            // Add decorators as preconditions
            for decorator in &function.decorators {
                if decorator == "@external" || decorator == "@internal" {
                    statute = statute.with_precondition(Condition::Custom {
                        description: format!("Visibility: {}", decorator),
                    });
                } else if decorator != "@view" && decorator != "@pure" && decorator != "@payable" {
                    statute = statute.with_precondition(Condition::Custom {
                        description: format!("Modifier: {}", decorator),
                    });
                }
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
            statute
                .effect
                .parameters
                .insert("language".to_string(), "Vyper".to_string());

            statutes.push(statute);
        }

        statutes
    }
}

impl Default for VyperImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for VyperImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Vyper
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let contract = self.parse_vyper(source)?;
        let statutes = self.convert_to_statutes(&contract);

        let mut report = ConversionReport::new(LegalFormat::Vyper, LegalFormat::Legalis);
        report.statutes_converted = statutes.len();

        if contract.functions.is_empty() {
            report.add_warning("Contract has no functions");
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("# @version ")
            || source.contains("@external")
            || source.contains("@internal")
            || source.contains("def ") && (source.contains("@view") || source.contains("@pure"))
            || (source.contains('{')
                && (source.contains("\"name\"") || source.contains("\"functions\"")))
    }
}

/// Vyper exporter
pub struct VyperExporter;

impl VyperExporter {
    /// Creates a new Vyper exporter
    pub fn new() -> Self {
        Self
    }

    fn convert_to_vyper(&self, statutes: &[Statute]) -> VyperContract {
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

            let mut decorators = Vec::new();

            // Add visibility decorator
            let visibility = statute.preconditions.iter().find_map(|cond| {
                if let Condition::Custom { description, .. } = cond {
                    if description.starts_with("Visibility: ") {
                        Some(
                            description
                                .strip_prefix("Visibility: ")
                                .unwrap()
                                .to_string(),
                        )
                    } else {
                        None
                    }
                } else {
                    None
                }
            });

            if let Some(vis) = visibility {
                decorators.push(vis);
            } else {
                decorators.push("@external".to_string());
            }

            // Add state mutability decorators
            match statute.effect.effect_type {
                EffectType::Custom => decorators.push("@view".to_string()),
                EffectType::MonetaryTransfer => decorators.push("@payable".to_string()),
                _ => {}
            }

            // Add other modifiers
            for cond in &statute.preconditions {
                if let Condition::Custom { description, .. } = cond {
                    if description.starts_with("Modifier: ") {
                        decorators
                            .push(description.strip_prefix("Modifier: ").unwrap().to_string());
                    }
                }
            }

            functions.push(VyperFunction {
                name: function_name,
                decorators,
                parameters: Vec::new(),
                return_type: None,
                documentation: Some(statute.effect.description.clone()),
            });
        }

        VyperContract {
            name: contract_name,
            version: "0.3.0".to_string(),
            license,
            documentation: Some("Generated from Legalis statutes".to_string()),
            functions,
            state_variables: Vec::new(),
            events: Vec::new(),
            interfaces: Vec::new(),
        }
    }

    fn generate_vyper_source(&self, contract: &VyperContract) -> String {
        let mut output = String::new();

        // License
        if let Some(license) = &contract.license {
            output.push_str(&format!("# @license {}\n", license));
        }

        // Version
        output.push_str(&format!("# @version {}\n\n", contract.version));

        // Contract name as comment
        output.push_str(&format!("# {}\n", contract.name));
        if let Some(doc) = &contract.documentation {
            output.push_str(&format!("# {}\n", doc));
        }
        output.push('\n');

        // Events
        for event in &contract.events {
            output.push_str(&format!("event {}:\n", event.name));
            output.push_str("    pass\n\n");
        }

        // State variables
        for var in &contract.state_variables {
            if var.is_public {
                output.push_str(&format!("public({}: {})\n", var.name, var.var_type));
            } else if var.is_constant {
                output.push_str(&format!("constant({}: {})\n", var.name, var.var_type));
            } else {
                output.push_str(&format!("{}: {}\n", var.name, var.var_type));
            }
        }

        if !contract.state_variables.is_empty() {
            output.push('\n');
        }

        // Functions
        for func in &contract.functions {
            // Function documentation
            if let Some(doc) = &func.documentation {
                output.push_str(&format!("# {}\n", doc));
            }

            // Decorators
            for decorator in &func.decorators {
                output.push_str(&format!("{}\n", decorator));
            }

            // Function signature
            output.push_str(&format!("def {}()", func.name));

            // Return type
            if let Some(return_type) = &func.return_type {
                output.push_str(&format!(" -> {}", return_type));
            }

            output.push_str(":\n");
            output.push_str("    # Implementation\n");
            output.push_str("    pass\n\n");
        }

        output
    }
}

impl Default for VyperExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for VyperExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Vyper
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let contract = self.convert_to_vyper(statutes);
        let output = self.generate_vyper_source(&contract);

        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Vyper);
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
    fn test_vyper_source_parsing() {
        let importer = VyperImporter::new();

        let source = r#"
        # @version 0.3.0
        # @license MIT

        event Transfer:
            sender: indexed(address)

        owner: public(address)

        @external
        def transfer():
            pass

        @external
        @view
        def balance() -> uint256:
            pass
        "#;

        let contract = importer.parse_vyper_source(source).unwrap();
        assert_eq!(contract.version, "0.3.0");
        assert_eq!(contract.license, Some("MIT".to_string()));
        assert_eq!(contract.events.len(), 1);
        assert_eq!(contract.functions.len(), 2);
    }

    #[test]
    fn test_vyper_json_parsing() {
        let importer = VyperImporter::new();

        let json = r#"{
            "name": "Token",
            "version": "0.3.0",
            "license": "MIT",
            "documentation": "ERC20 Token",
            "functions": [
                {
                    "name": "transfer",
                    "decorators": ["@external"],
                    "parameters": [],
                    "return_type": null,
                    "documentation": "Transfer tokens"
                }
            ],
            "state_variables": [],
            "events": [],
            "interfaces": []
        }"#;

        let contract = importer.parse_vyper(json).unwrap();
        assert_eq!(contract.name, "Token");
        assert_eq!(contract.functions.len(), 1);
    }

    #[test]
    fn test_vyper_import() {
        let importer = VyperImporter::new();

        let source = r#"
        # @version 0.3.0

        @external
        def vote():
            pass
        "#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(!statutes.is_empty());
        assert_eq!(
            statutes[0].effect.parameters.get("language"),
            Some(&"Vyper".to_string())
        );
    }

    #[test]
    fn test_vyper_export() {
        let exporter = VyperExporter::new();

        let mut statute = Statute::new(
            "token_transfer",
            "Token Transfer",
            Effect::new(EffectType::Grant, "Transfer tokens"),
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
        assert!(output.contains("# ERC20"));
        assert!(output.contains("@external"));
        assert!(output.contains("def transfer()"));
        assert!(output.contains("# @license MIT"));
    }

    #[test]
    fn test_vyper_validate() {
        let importer = VyperImporter::new();

        assert!(importer.validate("# @version 0.3.0"));
        assert!(importer.validate("@external\ndef test(): pass"));
        assert!(importer.validate("@view\ndef balance() -> uint256: pass"));
        assert!(!importer.validate("not vyper code"));
    }
}
