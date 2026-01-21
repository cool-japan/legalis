//! Cadence (Flow blockchain) smart contract to legal format conversion.
//!
//! Cadence is the resource-oriented programming language for Flow blockchain.
//! This module extracts legal semantics from:
//! - Contracts and their capabilities
//! - Resources and their ownership semantics
//! - Transactions and scripts
//! - Access control (pub, access(all), access(self), etc.)
//! - Pre/post conditions
//!
//! This module provides bidirectional conversion between Cadence contracts
//! and legalis_core::Statute format.

use crate::{ConversionReport, FormatExporter, FormatImporter, InteropResult, LegalFormat};
use legalis_core::{Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// Cadence contract structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadenceContract {
    /// Contract name
    pub name: String,
    /// Contract resources
    pub resources: Vec<CadenceResource>,
    /// Contract functions
    pub functions: Vec<CadenceFunction>,
    /// Contract fields
    pub fields: Vec<CadenceField>,
    /// Contract events
    pub events: Vec<CadenceEvent>,
    /// Contract init function
    pub init_function: Option<CadenceFunction>,
}

/// Cadence resource (owned type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadenceResource {
    /// Resource name
    pub name: String,
    /// Access modifier
    pub access: String,
    /// Resource fields
    pub fields: Vec<CadenceField>,
    /// Resource functions
    pub functions: Vec<CadenceFunction>,
    /// Resource interfaces implemented
    pub interfaces: Vec<String>,
}

/// Cadence function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadenceFunction {
    /// Function name
    pub name: String,
    /// Access modifier (pub, access(all), access(self), access(contract), access(account))
    pub access: String,
    /// Function parameters
    pub parameters: Vec<CadenceParameter>,
    /// Return type
    pub return_type: Option<String>,
    /// Pre-conditions
    pub pre_conditions: Vec<String>,
    /// Post-conditions
    pub post_conditions: Vec<String>,
    /// Function documentation
    pub documentation: Option<String>,
}

/// Cadence parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadenceParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
}

/// Cadence field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadenceField {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: String,
    /// Access modifier
    pub access: String,
    /// Whether field is variable (var) or constant (let)
    pub is_variable: bool,
}

/// Cadence event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadenceEvent {
    /// Event name
    pub name: String,
    /// Event parameters
    pub parameters: Vec<CadenceParameter>,
    /// Event documentation
    pub documentation: Option<String>,
}

/// Cadence importer
pub struct CadenceImporter;

impl CadenceImporter {
    /// Creates a new Cadence importer
    pub fn new() -> Self {
        Self
    }

    fn parse_cadence(&self, source: &str) -> InteropResult<CadenceContract> {
        // Try to parse as JSON first (for structured contract metadata)
        if let Ok(contract) = serde_json::from_str::<CadenceContract>(source) {
            return Ok(contract);
        }

        // Simple Cadence source code parser
        self.parse_cadence_source(source)
    }

    fn parse_cadence_source(&self, source: &str) -> InteropResult<CadenceContract> {
        let mut contract = CadenceContract {
            name: "Contract".to_string(),
            resources: Vec::new(),
            functions: Vec::new(),
            fields: Vec::new(),
            events: Vec::new(),
            init_function: None,
        };

        // Extract contract name
        for line in source.lines() {
            let trimmed = line.trim();
            if (trimmed.starts_with("pub contract ")
                || trimmed.starts_with("access(all) contract "))
                && let Some(name_start) = trimmed.find("contract ")
            {
                let name_start = name_start + "contract ".len();
                if let Some(name_end) =
                    trimmed[name_start..].find(|c: char| c.is_whitespace() || c == '{' || c == ':')
                {
                    contract.name = trimmed[name_start..name_start + name_end].to_string();
                    break;
                }
            }
        }

        // Extract functions (simplified)
        for line in source.lines() {
            let trimmed = line.trim();
            if (trimmed.starts_with("pub fun ") || trimmed.starts_with("access("))
                && trimmed.contains(" fun ")
                && let Some(func) = self.parse_function_signature(trimmed)
            {
                contract.functions.push(func);
            }
        }

        // Extract resources (simplified)
        for line in source.lines() {
            let trimmed = line.trim();
            if (trimmed.starts_with("pub resource ")
                || (trimmed.starts_with("access(") && trimmed.contains(" resource ")))
                && let Some(resource_name) = self.extract_resource_name(trimmed)
            {
                contract.resources.push(CadenceResource {
                    name: resource_name,
                    access: "pub".to_string(),
                    fields: Vec::new(),
                    functions: Vec::new(),
                    interfaces: Vec::new(),
                });
            }
        }

        Ok(contract)
    }

    fn parse_function_signature(&self, line: &str) -> Option<CadenceFunction> {
        let fun_pos = line.find(" fun ")?;
        let name_start = fun_pos + " fun ".len();
        let name_end = line[name_start..].find('(')?;
        let name = line[name_start..name_start + name_end].to_string();

        let access = if line.starts_with("pub ") {
            "pub".to_string()
        } else if line.contains("access(all)") {
            "access(all)".to_string()
        } else if line.contains("access(self)") {
            "access(self)".to_string()
        } else if line.contains("access(contract)") {
            "access(contract)".to_string()
        } else {
            "access(self)".to_string()
        };

        Some(CadenceFunction {
            name,
            access,
            parameters: Vec::new(),
            return_type: None,
            pre_conditions: Vec::new(),
            post_conditions: Vec::new(),
            documentation: None,
        })
    }

    fn extract_resource_name(&self, line: &str) -> Option<String> {
        let resource_pos = line.find(" resource ")?;
        let name_start = resource_pos + " resource ".len();
        line[name_start..]
            .find(|c: char| c.is_whitespace() || c == '{' || c == ':')
            .map(|name_end| line[name_start..name_start + name_end].to_string())
    }

    fn convert_to_statutes(&self, contract: &CadenceContract) -> Vec<Statute> {
        let mut statutes = Vec::new();

        // Convert each function to a statute
        for function in &contract.functions {
            let statute_id = format!(
                "{}_{}",
                contract.name.to_lowercase(),
                function.name.to_lowercase()
            );
            let title = format!("{} - {}", contract.name, function.name);

            let effect_type = if function.return_type.is_some() {
                EffectType::Custom
            } else {
                EffectType::Grant
            };

            let effect_value = function.documentation.as_deref().unwrap_or(&function.name);

            let mut statute =
                Statute::new(&statute_id, &title, Effect::new(effect_type, effect_value));

            // Add access control as precondition
            if function.access != "pub" && function.access != "access(all)" {
                statute = statute.with_precondition(Condition::Custom {
                    description: format!("Access: {}", function.access),
                });
            }

            // Add pre-conditions
            for pre in &function.pre_conditions {
                statute = statute.with_precondition(Condition::Custom {
                    description: format!("Precondition: {}", pre),
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
            statute
                .effect
                .parameters
                .insert("blockchain".to_string(), "Flow".to_string());

            statutes.push(statute);
        }

        // Convert each resource to a statute
        for resource in &contract.resources {
            let statute_id = format!(
                "{}_resource_{}",
                contract.name.to_lowercase(),
                resource.name.to_lowercase()
            );
            let title = format!("{} - Resource: {}", contract.name, resource.name);

            let mut statute = Statute::new(
                &statute_id,
                &title,
                Effect::new(EffectType::Grant, format!("Resource: {}", resource.name)),
            );

            statute
                .effect
                .parameters
                .insert("contract".to_string(), contract.name.clone());
            statute
                .effect
                .parameters
                .insert("resource".to_string(), resource.name.clone());
            statute
                .effect
                .parameters
                .insert("blockchain".to_string(), "Flow".to_string());

            statutes.push(statute);
        }

        statutes
    }
}

impl Default for CadenceImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for CadenceImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Cadence
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let contract = self.parse_cadence(source)?;
        let statutes = self.convert_to_statutes(&contract);

        let mut report = ConversionReport::new(LegalFormat::Cadence, LegalFormat::Legalis);
        report.statutes_converted = statutes.len();

        if contract.resources.is_empty() && contract.functions.is_empty() {
            report.add_warning("Contract has no resources or functions");
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("pub contract ")
            || source.contains("access(all) contract ")
            || source.contains("pub resource ")
            || source.contains("pub fun ")
            || (source.contains('{')
                && (source.contains("\"name\"") || source.contains("\"functions\"")))
    }
}

/// Cadence exporter
pub struct CadenceExporter;

impl CadenceExporter {
    /// Creates a new Cadence exporter
    pub fn new() -> Self {
        Self
    }

    fn convert_to_cadence(&self, statutes: &[Statute]) -> CadenceContract {
        let contract_name = statutes
            .first()
            .and_then(|s| s.effect.parameters.get("contract"))
            .cloned()
            .unwrap_or_else(|| "LegalisContract".to_string());

        let mut functions = Vec::new();
        let mut resources = Vec::new();

        for statute in statutes {
            // Check if this is a resource statute
            if statute.effect.parameters.contains_key("resource") {
                let resource_name = statute
                    .effect
                    .parameters
                    .get("resource")
                    .cloned()
                    .unwrap_or_else(|| statute.id.clone());

                resources.push(CadenceResource {
                    name: resource_name,
                    access: "pub".to_string(),
                    fields: Vec::new(),
                    functions: Vec::new(),
                    interfaces: Vec::new(),
                });
            } else {
                // This is a function statute
                let function_name = statute
                    .effect
                    .parameters
                    .get("function")
                    .cloned()
                    .unwrap_or_else(|| statute.id.clone());

                let access = statute
                    .preconditions
                    .iter()
                    .find_map(|cond| {
                        if let Condition::Custom { description, .. } = cond {
                            if description.starts_with("Access: ") {
                                Some(description.strip_prefix("Access: ").unwrap().to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| "pub".to_string());

                let pre_conditions: Vec<String> = statute
                    .preconditions
                    .iter()
                    .filter_map(|cond| {
                        if let Condition::Custom { description, .. } = cond {
                            if description.starts_with("Precondition: ") {
                                Some(
                                    description
                                        .strip_prefix("Precondition: ")
                                        .unwrap()
                                        .to_string(),
                                )
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                functions.push(CadenceFunction {
                    name: function_name,
                    access,
                    parameters: Vec::new(),
                    return_type: None,
                    pre_conditions,
                    post_conditions: Vec::new(),
                    documentation: Some(statute.effect.description.clone()),
                });
            }
        }

        CadenceContract {
            name: contract_name,
            resources,
            functions,
            fields: Vec::new(),
            events: Vec::new(),
            init_function: None,
        }
    }

    fn generate_cadence_source(&self, contract: &CadenceContract) -> String {
        let mut output = String::new();

        // Contract declaration
        output.push_str(&format!("pub contract {} {{\n", contract.name));

        // Resources
        for resource in &contract.resources {
            output.push_str(&format!("\n    pub resource {} {{\n", resource.name));
            output.push_str("        // Resource implementation\n");
            output.push_str("    }\n");
        }

        // Functions
        for func in &contract.functions {
            output.push('\n');

            // Function documentation
            if let Some(doc) = &func.documentation {
                output.push_str(&format!("    /// {}\n", doc));
            }

            // Pre-conditions
            for pre in &func.pre_conditions {
                output.push_str(&format!("    // Pre: {}\n", pre));
            }

            // Function signature
            output.push_str(&format!("    {} fun {}()", func.access, func.name));

            // Return type
            if let Some(return_type) = &func.return_type {
                output.push_str(&format!(": {}", return_type));
            }

            output.push_str(" {\n");
            output.push_str("        // Implementation\n");
            output.push_str("    }\n");
        }

        // Init function
        output.push_str("\n    init() {\n");
        output.push_str("        // Initialize contract\n");
        output.push_str("    }\n");

        output.push_str("}\n");

        output
    }
}

impl Default for CadenceExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for CadenceExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Cadence
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let contract = self.convert_to_cadence(statutes);
        let output = self.generate_cadence_source(&contract);

        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Cadence);
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
    fn test_cadence_source_parsing() {
        let importer = CadenceImporter::new();

        let source = r#"
        pub contract FlowToken {
            pub resource Vault {
                pub var balance: UFix64
            }

            pub fun createEmptyVault(): @Vault {
            }

            init() {
            }
        }
        "#;

        let contract = importer.parse_cadence_source(source).unwrap();
        assert_eq!(contract.name, "FlowToken");
        assert_eq!(contract.resources.len(), 1);
        assert_eq!(contract.functions.len(), 1);
    }

    #[test]
    fn test_cadence_json_parsing() {
        let importer = CadenceImporter::new();

        let json = r#"{
            "name": "NFTContract",
            "resources": [
                {
                    "name": "NFT",
                    "access": "pub",
                    "fields": [],
                    "functions": [],
                    "interfaces": []
                }
            ],
            "functions": [
                {
                    "name": "mintNFT",
                    "access": "pub",
                    "parameters": [],
                    "return_type": null,
                    "pre_conditions": [],
                    "post_conditions": [],
                    "documentation": "Mint new NFT"
                }
            ],
            "fields": [],
            "events": [],
            "init_function": null
        }"#;

        let contract = importer.parse_cadence(json).unwrap();
        assert_eq!(contract.name, "NFTContract");
        assert_eq!(contract.resources.len(), 1);
        assert_eq!(contract.functions.len(), 1);
    }

    #[test]
    fn test_cadence_import() {
        let importer = CadenceImporter::new();

        let source = r#"
        pub contract Voting {
            pub fun vote() {
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
        assert_eq!(
            statutes[0].effect.parameters.get("blockchain"),
            Some(&"Flow".to_string())
        );
    }

    #[test]
    fn test_cadence_export() {
        let exporter = CadenceExporter::new();

        let mut statute = Statute::new(
            "token_transfer",
            "Token Transfer",
            Effect::new(EffectType::Grant, "Transfer tokens"),
        );
        statute
            .effect
            .parameters
            .insert("contract".to_string(), "FlowToken".to_string());
        statute
            .effect
            .parameters
            .insert("function".to_string(), "transfer".to_string());

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("pub contract FlowToken"));
        assert!(output.contains("pub fun transfer()"));
    }

    #[test]
    fn test_cadence_validate() {
        let importer = CadenceImporter::new();

        assert!(importer.validate("pub contract Test { }"));
        assert!(importer.validate("access(all) contract Test { }"));
        assert!(importer.validate("pub resource NFT { }"));
        assert!(!importer.validate("not cadence code"));
    }
}
