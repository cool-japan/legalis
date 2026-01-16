//! Move (Aptos/Sui) smart contract to legal format conversion.
//!
//! Move is a resource-oriented programming language originally developed for
//! Libra/Diem, now used by Aptos and Sui blockchains. This module extracts
//! legal semantics from:
//! - Modules and their functions
//! - Resources (structs with key/store abilities)
//! - Entry functions (transaction endpoints)
//! - Acquires clauses (resource access)
//! - Abilities (copy, drop, store, key)
//!
//! This module provides bidirectional conversion between Move modules
//! and legalis_core::Statute format.

use crate::{ConversionReport, FormatExporter, FormatImporter, InteropResult, LegalFormat};
use legalis_core::{Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// Move module structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveModule {
    /// Module address
    pub address: String,
    /// Module name
    pub name: String,
    /// Module structs
    pub structs: Vec<MoveStruct>,
    /// Module functions
    pub functions: Vec<MoveFunction>,
    /// Module constants
    pub constants: Vec<MoveConstant>,
}

/// Move struct (resource or regular struct)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveStruct {
    /// Struct name
    pub name: String,
    /// Struct abilities (copy, drop, store, key)
    pub abilities: Vec<String>,
    /// Struct fields
    pub fields: Vec<MoveField>,
    /// Whether this is a resource (has key ability)
    pub is_resource: bool,
}

/// Move function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFunction {
    /// Function name
    pub name: String,
    /// Visibility (public, public(friend), or module-private)
    pub visibility: String,
    /// Whether this is an entry function
    pub is_entry: bool,
    /// Function parameters
    pub parameters: Vec<MoveParameter>,
    /// Return types
    pub return_types: Vec<String>,
    /// Resources acquired by this function
    pub acquires: Vec<String>,
    /// Function documentation
    pub documentation: Option<String>,
}

/// Move parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
}

/// Move field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveField {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: String,
}

/// Move constant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveConstant {
    /// Constant name
    pub name: String,
    /// Constant type
    pub const_type: String,
    /// Constant value
    pub value: String,
}

/// Move importer
pub struct MoveImporter;

impl MoveImporter {
    /// Creates a new Move importer
    pub fn new() -> Self {
        Self
    }

    fn parse_move(&self, source: &str) -> InteropResult<MoveModule> {
        // Try to parse as JSON first (for structured module metadata)
        if let Ok(module) = serde_json::from_str::<MoveModule>(source) {
            return Ok(module);
        }

        // Simple Move source code parser
        self.parse_move_source(source)
    }

    fn parse_move_source(&self, source: &str) -> InteropResult<MoveModule> {
        let mut module = MoveModule {
            address: "0x1".to_string(),
            name: "Module".to_string(),
            structs: Vec::new(),
            functions: Vec::new(),
            constants: Vec::new(),
        };

        // Extract module declaration
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("module ") {
                if let Some(parts) = self.parse_module_declaration(trimmed) {
                    module.address = parts.0;
                    module.name = parts.1;
                    break;
                }
            }
        }

        // Extract structs
        let mut in_struct = false;
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("struct ") {
                if let Some(struct_def) = self.parse_struct_declaration(trimmed) {
                    module.structs.push(struct_def);
                    in_struct = true;
                }
            } else if in_struct && trimmed == "}" {
                in_struct = false;
            }
        }

        // Extract functions
        for line in source.lines() {
            let trimmed = line.trim();
            if (trimmed.starts_with("public ")
                || trimmed.starts_with("public(")
                || trimmed.starts_with("fun "))
                && trimmed.contains(" fun ")
            {
                if let Some(func) = self.parse_function_signature(trimmed) {
                    module.functions.push(func);
                }
            }
        }

        Ok(module)
    }

    fn parse_module_declaration(&self, line: &str) -> Option<(String, String)> {
        // Format: "module address::name {"
        let module_start = line.find("module ")? + "module ".len();
        let module_end = line[module_start..].find(|c: char| c == '{' || c.is_whitespace())?;
        let full_name = &line[module_start..module_start + module_end];

        if let Some(separator) = full_name.find("::") {
            let address = full_name[..separator].to_string();
            let name = full_name[separator + 2..].to_string();
            Some((address, name))
        } else {
            Some(("0x1".to_string(), full_name.to_string()))
        }
    }

    fn parse_struct_declaration(&self, line: &str) -> Option<MoveStruct> {
        let struct_start = line.find("struct ")? + "struct ".len();
        let name_end =
            line[struct_start..].find(|c: char| c.is_whitespace() || c == '<' || c == '{')?;
        let name = line[struct_start..struct_start + name_end].to_string();

        let mut abilities = Vec::new();
        if line.contains(" has ") {
            let has_start = line.find(" has ")? + " has ".len();
            if let Some(has_end) = line[has_start..].find(['{', '<']) {
                let abilities_str = &line[has_start..has_start + has_end];
                abilities = abilities_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
            }
        }

        let is_resource = abilities.contains(&"key".to_string());

        Some(MoveStruct {
            name,
            abilities,
            fields: Vec::new(),
            is_resource,
        })
    }

    fn parse_function_signature(&self, line: &str) -> Option<MoveFunction> {
        let fun_pos = line.find(" fun ")?;
        let name_start = fun_pos + " fun ".len();
        let name_end = line[name_start..].find(['(', '<'])?;
        let name = line[name_start..name_start + name_end].to_string();

        let visibility = if line.starts_with("public entry ")
            || line.contains("public(friend) entry ")
            || line.starts_with("public ")
        {
            "public".to_string()
        } else if line.contains("public(friend)") {
            "public(friend)".to_string()
        } else {
            "private".to_string()
        };

        let is_entry = line.contains(" entry ") || line.starts_with("public entry ");

        let mut acquires = Vec::new();
        if line.contains(" acquires ") {
            let acq_start = line.find(" acquires ")? + " acquires ".len();
            if let Some(acq_end) = line[acq_start..].find(['{', ';']) {
                let acquires_str = &line[acq_start..acq_start + acq_end].trim();
                acquires = acquires_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
            }
        }

        Some(MoveFunction {
            name,
            visibility,
            is_entry,
            parameters: Vec::new(),
            return_types: Vec::new(),
            acquires,
            documentation: None,
        })
    }

    fn convert_to_statutes(&self, module: &MoveModule) -> Vec<Statute> {
        let mut statutes = Vec::new();

        // Convert each function to a statute
        for function in &module.functions {
            let statute_id = format!(
                "{}_{}_{}",
                module.address.replace("::", "_"),
                module.name.to_lowercase(),
                function.name.to_lowercase()
            );
            let title = format!("{}::{} - {}", module.address, module.name, function.name);

            let effect_type = if function.is_entry {
                EffectType::Grant
            } else if !function.return_types.is_empty() {
                EffectType::Custom
            } else {
                EffectType::Grant
            };

            let effect_value = function.documentation.as_deref().unwrap_or(&function.name);

            let mut statute =
                Statute::new(&statute_id, &title, Effect::new(effect_type, effect_value));

            // Add visibility as precondition
            if function.visibility != "public" {
                statute = statute.with_precondition(Condition::Custom {
                    description: format!("Visibility: {}", function.visibility),
                });
            }

            // Add acquires as preconditions
            for acquire in &function.acquires {
                statute = statute.with_precondition(Condition::Custom {
                    description: format!("Acquires: {}", acquire),
                });
            }

            // Add module metadata
            statute
                .effect
                .parameters
                .insert("module_address".to_string(), module.address.clone());
            statute
                .effect
                .parameters
                .insert("module_name".to_string(), module.name.clone());
            statute
                .effect
                .parameters
                .insert("function".to_string(), function.name.clone());
            statute
                .effect
                .parameters
                .insert("blockchain".to_string(), "Move".to_string());
            if function.is_entry {
                statute
                    .effect
                    .parameters
                    .insert("entry".to_string(), "true".to_string());
            }

            statutes.push(statute);
        }

        // Convert each resource struct to a statute
        for struct_def in &module.structs {
            if struct_def.is_resource {
                let statute_id = format!(
                    "{}_{}_{}",
                    module.address.replace("::", "_"),
                    module.name.to_lowercase(),
                    struct_def.name.to_lowercase()
                );
                let title = format!(
                    "{}::{} - Resource: {}",
                    module.address, module.name, struct_def.name
                );

                let mut statute = Statute::new(
                    &statute_id,
                    &title,
                    Effect::new(EffectType::Grant, format!("Resource: {}", struct_def.name)),
                );

                statute
                    .effect
                    .parameters
                    .insert("module_address".to_string(), module.address.clone());
                statute
                    .effect
                    .parameters
                    .insert("module_name".to_string(), module.name.clone());
                statute
                    .effect
                    .parameters
                    .insert("struct".to_string(), struct_def.name.clone());
                statute
                    .effect
                    .parameters
                    .insert("abilities".to_string(), struct_def.abilities.join(", "));
                statute
                    .effect
                    .parameters
                    .insert("blockchain".to_string(), "Move".to_string());

                statutes.push(statute);
            }
        }

        statutes
    }
}

impl Default for MoveImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for MoveImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Move
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let module = self.parse_move(source)?;
        let statutes = self.convert_to_statutes(&module);

        let mut report = ConversionReport::new(LegalFormat::Move, LegalFormat::Legalis);
        report.statutes_converted = statutes.len();

        if module.structs.is_empty() && module.functions.is_empty() {
            report.add_warning("Module has no structs or functions");
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("module ")
            || source.contains("struct ") && source.contains(" has ")
            || source.contains(" acquires ")
            || source.contains("public entry ")
            || (source.contains('{')
                && (source.contains("\"name\"") || source.contains("\"functions\"")))
    }
}

/// Move exporter
pub struct MoveExporter;

impl MoveExporter {
    /// Creates a new Move exporter
    pub fn new() -> Self {
        Self
    }

    fn convert_to_move(&self, statutes: &[Statute]) -> MoveModule {
        let module_address = statutes
            .first()
            .and_then(|s| s.effect.parameters.get("module_address"))
            .cloned()
            .unwrap_or_else(|| "0x1".to_string());

        let module_name = statutes
            .first()
            .and_then(|s| s.effect.parameters.get("module_name"))
            .cloned()
            .unwrap_or_else(|| "LegalisModule".to_string());

        let mut functions = Vec::new();
        let mut structs = Vec::new();

        for statute in statutes {
            // Check if this is a resource statute
            if statute.effect.parameters.contains_key("struct") {
                let struct_name = statute
                    .effect
                    .parameters
                    .get("struct")
                    .cloned()
                    .unwrap_or_else(|| statute.id.clone());

                let abilities_str = statute
                    .effect
                    .parameters
                    .get("abilities")
                    .cloned()
                    .unwrap_or_else(|| "key, store".to_string());

                let abilities: Vec<String> = abilities_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();

                structs.push(MoveStruct {
                    name: struct_name,
                    abilities: abilities.clone(),
                    fields: Vec::new(),
                    is_resource: abilities.contains(&"key".to_string()),
                });
            } else {
                // This is a function statute
                let function_name = statute
                    .effect
                    .parameters
                    .get("function")
                    .cloned()
                    .unwrap_or_else(|| statute.id.clone());

                let visibility = statute
                    .preconditions
                    .iter()
                    .find_map(|cond| {
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
                    })
                    .unwrap_or_else(|| "public".to_string());

                let acquires: Vec<String> = statute
                    .preconditions
                    .iter()
                    .filter_map(|cond| {
                        if let Condition::Custom { description, .. } = cond {
                            if description.starts_with("Acquires: ") {
                                Some(description.strip_prefix("Acquires: ").unwrap().to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                let is_entry = statute
                    .effect
                    .parameters
                    .get("entry")
                    .map(|v| v == "true")
                    .unwrap_or(false);

                functions.push(MoveFunction {
                    name: function_name,
                    visibility,
                    is_entry,
                    parameters: Vec::new(),
                    return_types: Vec::new(),
                    acquires,
                    documentation: Some(statute.effect.description.clone()),
                });
            }
        }

        MoveModule {
            address: module_address,
            name: module_name,
            structs,
            functions,
            constants: Vec::new(),
        }
    }

    fn generate_move_source(&self, module: &MoveModule) -> String {
        let mut output = String::new();

        // Module declaration
        output.push_str(&format!("module {}::{} {{\n", module.address, module.name));

        // Structs
        for struct_def in &module.structs {
            output.push_str(&format!("\n    struct {}", struct_def.name));
            if !struct_def.abilities.is_empty() {
                output.push_str(&format!(" has {}", struct_def.abilities.join(", ")));
            }
            output.push_str(" {\n");
            output.push_str("        // Fields\n");
            output.push_str("    }\n");
        }

        // Functions
        for func in &module.functions {
            output.push('\n');

            // Function documentation
            if let Some(doc) = &func.documentation {
                output.push_str(&format!("    /// {}\n", doc));
            }

            // Function signature
            output.push_str("    ");
            if func.visibility == "public" {
                output.push_str("public ");
            } else if func.visibility == "public(friend)" {
                output.push_str("public(friend) ");
            }

            if func.is_entry {
                output.push_str("entry ");
            }

            output.push_str(&format!("fun {}()", func.name));

            // Acquires
            if !func.acquires.is_empty() {
                output.push_str(&format!(" acquires {}", func.acquires.join(", ")));
            }

            output.push_str(" {\n");
            output.push_str("        // Implementation\n");
            output.push_str("    }\n");
        }

        output.push_str("}\n");

        output
    }
}

impl Default for MoveExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for MoveExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Move
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let module = self.convert_to_move(statutes);
        let output = self.generate_move_source(&module);

        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Move);
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
    fn test_move_source_parsing() {
        let importer = MoveImporter::new();

        let source = r#"
        module 0x1::Coin {
            struct Coin has key, store {
                value: u64
            }

            public entry fun transfer() {
            }

            public fun balance() acquires Coin {
            }
        }
        "#;

        let module = importer.parse_move_source(source).unwrap();
        assert_eq!(module.address, "0x1");
        assert_eq!(module.name, "Coin");
        assert_eq!(module.structs.len(), 1);
        assert_eq!(module.functions.len(), 2);
    }

    #[test]
    fn test_move_json_parsing() {
        let importer = MoveImporter::new();

        let json = r#"{
            "address": "0x1",
            "name": "Token",
            "structs": [
                {
                    "name": "Token",
                    "abilities": ["key", "store"],
                    "fields": [],
                    "is_resource": true
                }
            ],
            "functions": [
                {
                    "name": "mint",
                    "visibility": "public",
                    "is_entry": true,
                    "parameters": [],
                    "return_types": [],
                    "acquires": [],
                    "documentation": "Mint tokens"
                }
            ],
            "constants": []
        }"#;

        let module = importer.parse_move(json).unwrap();
        assert_eq!(module.name, "Token");
        assert_eq!(module.structs.len(), 1);
        assert_eq!(module.functions.len(), 1);
    }

    #[test]
    fn test_move_import() {
        let importer = MoveImporter::new();

        let source = r#"
        module 0x1::Voting {
            public entry fun vote() {
            }
        }
        "#;

        let (statutes, report) = importer.import(source).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(!statutes.is_empty());
        assert_eq!(
            statutes[0].effect.parameters.get("module_name"),
            Some(&"Voting".to_string())
        );
        assert_eq!(
            statutes[0].effect.parameters.get("blockchain"),
            Some(&"Move".to_string())
        );
    }

    #[test]
    fn test_move_export() {
        let exporter = MoveExporter::new();

        let mut statute = Statute::new(
            "token_transfer",
            "Token Transfer",
            Effect::new(EffectType::Grant, "Transfer tokens"),
        );
        statute
            .effect
            .parameters
            .insert("module_address".to_string(), "0x1".to_string());
        statute
            .effect
            .parameters
            .insert("module_name".to_string(), "Coin".to_string());
        statute
            .effect
            .parameters
            .insert("function".to_string(), "transfer".to_string());
        statute
            .effect
            .parameters
            .insert("entry".to_string(), "true".to_string());

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("module 0x1::Coin"));
        assert!(output.contains("public entry fun transfer()"));
    }

    #[test]
    fn test_move_validate() {
        let importer = MoveImporter::new();

        assert!(importer.validate("module 0x1::Test { }"));
        assert!(importer.validate("struct Token has key, store { }"));
        assert!(importer.validate("public entry fun mint() { }"));
        assert!(!importer.validate("not move code"));
    }
}
