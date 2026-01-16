//! Smart contract documentation generation.
//!
//! This module provides automated documentation generation for smart contracts
//! across different blockchain platforms (Solidity, Vyper, Cadence, Move).
//! It generates:
//! - Function-level documentation with NatSpec format
//! - Contract-level overview and usage guides
//! - Security considerations and access control documentation
//! - Integration guides and examples
//! - ABI/Interface specifications
//!
//! The documentation can be generated in multiple formats:
//! - Markdown
//! - HTML
//! - JSON (structured)
//! - NatSpec (for Solidity/Vyper)

use crate::{InteropError, InteropResult, LegalFormat};
use legalis_core::{EffectType, Statute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Documentation format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocFormat {
    /// Markdown documentation
    Markdown,
    /// HTML documentation
    Html,
    /// JSON structured documentation
    Json,
    /// NatSpec format (Solidity/Vyper)
    NatSpec,
}

/// Smart contract documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDocumentation {
    /// Contract name
    pub contract_name: String,
    /// Blockchain platform
    pub platform: String,
    /// Contract overview
    pub overview: String,
    /// Function documentation
    pub functions: Vec<FunctionDoc>,
    /// Security considerations
    pub security: Vec<String>,
    /// Usage examples
    pub examples: Vec<String>,
    /// Access control summary
    pub access_control: HashMap<String, Vec<String>>,
}

/// Function documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDoc {
    /// Function name
    pub name: String,
    /// Function description
    pub description: String,
    /// Function visibility/access
    pub visibility: String,
    /// State mutability
    pub mutability: String,
    /// Parameters documentation
    pub parameters: Vec<ParameterDoc>,
    /// Return value documentation
    pub returns: Vec<String>,
    /// Preconditions
    pub preconditions: Vec<String>,
    /// Security notes
    pub security_notes: Vec<String>,
}

/// Parameter documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDoc {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
    /// Parameter description
    pub description: String,
}

/// Documentation generator
pub struct DocGenerator {
    format: DocFormat,
}

impl DocGenerator {
    /// Creates a new documentation generator
    pub fn new(format: DocFormat) -> Self {
        Self { format }
    }

    /// Generates documentation from statutes
    pub fn generate(&self, statutes: &[Statute], platform: LegalFormat) -> InteropResult<String> {
        let doc = self.build_documentation(statutes, platform)?;

        match self.format {
            DocFormat::Markdown => Ok(self.generate_markdown(&doc)),
            DocFormat::Html => Ok(self.generate_html(&doc)),
            DocFormat::Json => self.generate_json(&doc),
            DocFormat::NatSpec => Ok(self.generate_natspec(&doc)),
        }
    }

    fn build_documentation(
        &self,
        statutes: &[Statute],
        platform: LegalFormat,
    ) -> InteropResult<ContractDocumentation> {
        let contract_name = statutes
            .first()
            .and_then(|s| {
                s.effect
                    .parameters
                    .get("contract")
                    .or_else(|| s.effect.parameters.get("module_name"))
            })
            .cloned()
            .unwrap_or_else(|| "Contract".to_string());

        let platform_name = match platform {
            LegalFormat::Solidity => "Ethereum (Solidity)",
            LegalFormat::Vyper => "Ethereum (Vyper)",
            LegalFormat::Cadence => "Flow (Cadence)",
            LegalFormat::Move => "Aptos/Sui (Move)",
            _ => "Unknown",
        }
        .to_string();

        let overview = format!(
            "This smart contract is deployed on {} and implements {} functions.",
            platform_name,
            statutes.len()
        );

        let mut functions = Vec::new();
        let mut security = Vec::new();
        let mut access_control: HashMap<String, Vec<String>> = HashMap::new();

        for statute in statutes {
            let function_name = statute
                .effect
                .parameters
                .get("function")
                .cloned()
                .unwrap_or_else(|| statute.id.clone());

            let description = statute.effect.description.clone();

            let visibility = statute
                .preconditions
                .iter()
                .find_map(|cond| {
                    if let legalis_core::Condition::Custom { description, .. } = cond {
                        if description.starts_with("Visibility: ")
                            || description.starts_with("Access: ")
                        {
                            Some(description.split(": ").nth(1).unwrap().to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| "public".to_string());

            let mutability = match statute.effect.effect_type {
                EffectType::Custom => "view/read-only".to_string(),
                EffectType::MonetaryTransfer => "payable".to_string(),
                _ => "state-changing".to_string(),
            };

            let preconditions: Vec<String> = statute
                .preconditions
                .iter()
                .map(|cond| format!("{:?}", cond))
                .collect();

            let security_notes: Vec<String> = statute
                .preconditions
                .iter()
                .filter_map(|cond| {
                    if let legalis_core::Condition::Custom { description, .. } = cond {
                        if description.contains("Modifier: @nonreentrant")
                            || description.contains("Requires:")
                        {
                            Some(description.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            if !security_notes.is_empty() {
                security.push(format!(
                    "Function '{}' has security requirements: {}",
                    function_name,
                    security_notes.join(", ")
                ));
            }

            // Build access control map
            access_control
                .entry(visibility.clone())
                .or_default()
                .push(function_name.clone());

            functions.push(FunctionDoc {
                name: function_name,
                description,
                visibility,
                mutability,
                parameters: Vec::new(),
                returns: Vec::new(),
                preconditions,
                security_notes,
            });
        }

        let examples = vec![format!("// Example: Call a function on {}", contract_name)];

        Ok(ContractDocumentation {
            contract_name,
            platform: platform_name,
            overview,
            functions,
            security,
            examples,
            access_control,
        })
    }

    fn generate_markdown(&self, doc: &ContractDocumentation) -> String {
        let mut output = String::new();

        // Title
        output.push_str(&format!("# {}\n\n", doc.contract_name));

        // Overview
        output.push_str("## Overview\n\n");
        output.push_str(&format!("{}\n\n", doc.overview));
        output.push_str(&format!("**Platform**: {}\n\n", doc.platform));

        // Access Control
        if !doc.access_control.is_empty() {
            output.push_str("## Access Control\n\n");
            for (level, funcs) in &doc.access_control {
                output.push_str(&format!("- **{}**: {}\n", level, funcs.join(", ")));
            }
            output.push('\n');
        }

        // Functions
        output.push_str("## Functions\n\n");
        for func in &doc.functions {
            output.push_str(&format!("### {}\n\n", func.name));
            output.push_str(&format!("{}\n\n", func.description));
            output.push_str(&format!("- **Visibility**: {}\n", func.visibility));
            output.push_str(&format!("- **Mutability**: {}\n", func.mutability));

            if !func.preconditions.is_empty() {
                output.push_str(&format!(
                    "- **Preconditions**: {}\n",
                    func.preconditions.len()
                ));
            }

            if !func.security_notes.is_empty() {
                output.push_str("\n**Security Notes**:\n");
                for note in &func.security_notes {
                    output.push_str(&format!("- {}\n", note));
                }
            }

            output.push('\n');
        }

        // Security
        if !doc.security.is_empty() {
            output.push_str("## Security Considerations\n\n");
            for item in &doc.security {
                output.push_str(&format!("- {}\n", item));
            }
            output.push('\n');
        }

        // Examples
        if !doc.examples.is_empty() {
            output.push_str("## Usage Examples\n\n");
            for example in &doc.examples {
                output.push_str(&format!("```\n{}\n```\n\n", example));
            }
        }

        output
    }

    fn generate_html(&self, doc: &ContractDocumentation) -> String {
        let mut output = String::new();

        output.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        output.push_str(&format!("<title>{}</title>\n", doc.contract_name));
        output.push_str("<style>\nbody { font-family: Arial, sans-serif; margin: 20px; }\n");
        output
            .push_str("h1 { color: #333; }\nh2 { color: #666; border-bottom: 1px solid #ccc; }\n");
        output.push_str(".function { margin: 20px 0; padding: 10px; background: #f5f5f5; }\n");
        output.push_str(".security { background: #fff3cd; padding: 10px; margin: 10px 0; }\n");
        output.push_str("</style>\n</head>\n<body>\n");

        // Title
        output.push_str(&format!("<h1>{}</h1>\n", doc.contract_name));

        // Overview
        output.push_str("<h2>Overview</h2>\n");
        output.push_str(&format!("<p>{}</p>\n", doc.overview));
        output.push_str(&format!(
            "<p><strong>Platform:</strong> {}</p>\n",
            doc.platform
        ));

        // Functions
        output.push_str("<h2>Functions</h2>\n");
        for func in &doc.functions {
            output.push_str("<div class=\"function\">\n");
            output.push_str(&format!("<h3>{}</h3>\n", func.name));
            output.push_str(&format!("<p>{}</p>\n", func.description));
            output.push_str(&format!(
                "<p><strong>Visibility:</strong> {}</p>\n",
                func.visibility
            ));
            output.push_str(&format!(
                "<p><strong>Mutability:</strong> {}</p>\n",
                func.mutability
            ));

            if !func.security_notes.is_empty() {
                output.push_str("<div class=\"security\">\n");
                output.push_str("<strong>Security Notes:</strong>\n<ul>\n");
                for note in &func.security_notes {
                    output.push_str(&format!("<li>{}</li>\n", note));
                }
                output.push_str("</ul>\n</div>\n");
            }

            output.push_str("</div>\n");
        }

        // Security
        if !doc.security.is_empty() {
            output.push_str("<h2>Security Considerations</h2>\n<ul>\n");
            for item in &doc.security {
                output.push_str(&format!("<li>{}</li>\n", item));
            }
            output.push_str("</ul>\n");
        }

        output.push_str("</body>\n</html>\n");

        output
    }

    fn generate_json(&self, doc: &ContractDocumentation) -> InteropResult<String> {
        serde_json::to_string_pretty(doc).map_err(|e| {
            InteropError::SerializationError(format!("Failed to serialize documentation: {}", e))
        })
    }

    fn generate_natspec(&self, doc: &ContractDocumentation) -> String {
        let mut output = String::new();

        // Contract-level NatSpec
        output.push_str(&format!("/// @title {}\n", doc.contract_name));
        output.push_str(&format!("/// @notice {}\n", doc.overview));

        for func in &doc.functions {
            output.push('\n');
            output.push_str(&format!("/// @notice {}\n", func.description));

            for note in &func.security_notes {
                output.push_str(&format!("/// @dev {}\n", note));
            }

            output.push_str(&format!(
                "function {}() {} {{\n",
                func.name, func.visibility
            ));
            output.push_str("    // Implementation\n");
            output.push_str("}\n");
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Condition, Effect};

    #[test]
    fn test_markdown_generation() {
        let generator = DocGenerator::new(DocFormat::Markdown);

        let mut statute = Statute::new(
            "transfer",
            "Transfer Function",
            Effect::new(
                EffectType::MonetaryTransfer,
                "Transfer tokens between accounts",
            ),
        );
        statute
            .effect
            .parameters
            .insert("contract".to_string(), "ERC20".to_string());
        statute
            .effect
            .parameters
            .insert("function".to_string(), "transfer".to_string());

        let markdown = generator
            .generate(&[statute], LegalFormat::Solidity)
            .unwrap();

        assert!(markdown.contains("# ERC20"));
        assert!(markdown.contains("## Functions"));
        assert!(markdown.contains("### transfer"));
        assert!(markdown.contains("Transfer tokens between accounts"));
    }

    #[test]
    fn test_html_generation() {
        let generator = DocGenerator::new(DocFormat::Html);

        let mut statute = Statute::new(
            "vote",
            "Vote Function",
            Effect::new(EffectType::Grant, "Cast a vote"),
        );
        statute
            .effect
            .parameters
            .insert("contract".to_string(), "Voting".to_string());
        statute
            .effect
            .parameters
            .insert("function".to_string(), "vote".to_string());

        let html = generator
            .generate(&[statute], LegalFormat::Solidity)
            .unwrap();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<h1>Voting</h1>"));
        assert!(html.contains("<h3>vote</h3>"));
        assert!(html.contains("Cast a vote"));
    }

    #[test]
    fn test_json_generation() {
        let generator = DocGenerator::new(DocFormat::Json);

        let mut statute = Statute::new(
            "mint",
            "Mint Function",
            Effect::new(EffectType::Grant, "Mint new tokens"),
        );
        statute
            .effect
            .parameters
            .insert("contract".to_string(), "Token".to_string());
        statute
            .effect
            .parameters
            .insert("function".to_string(), "mint".to_string());

        let json = generator.generate(&[statute], LegalFormat::Move).unwrap();

        assert!(json.contains("\"contract_name\": \"Token\""));
        assert!(json.contains("\"name\": \"mint\""));
        assert!(json.contains("Mint new tokens"));
    }

    #[test]
    fn test_natspec_generation() {
        let generator = DocGenerator::new(DocFormat::NatSpec);

        let mut statute = Statute::new(
            "withdraw",
            "Withdraw Function",
            Effect::new(EffectType::MonetaryTransfer, "Withdraw funds"),
        );
        statute
            .effect
            .parameters
            .insert("contract".to_string(), "Vault".to_string());
        statute
            .effect
            .parameters
            .insert("function".to_string(), "withdraw".to_string());

        let natspec = generator.generate(&[statute], LegalFormat::Vyper).unwrap();

        assert!(natspec.contains("/// @title Vault"));
        assert!(natspec.contains("/// @notice Withdraw funds"));
        assert!(natspec.contains("function withdraw()"));
    }

    #[test]
    fn test_security_notes() {
        let generator = DocGenerator::new(DocFormat::Markdown);

        let mut statute = Statute::new(
            "transfer",
            "Transfer Function",
            Effect::new(EffectType::MonetaryTransfer, "Transfer funds"),
        );
        statute
            .effect
            .parameters
            .insert("contract".to_string(), "Token".to_string());
        statute
            .effect
            .parameters
            .insert("function".to_string(), "transfer".to_string());
        statute = statute.with_precondition(Condition::Custom {
            description: "Modifier: @nonreentrant".to_string(),
        });

        let markdown = generator
            .generate(&[statute], LegalFormat::Solidity)
            .unwrap();

        assert!(markdown.contains("Security Notes"));
        assert!(markdown.contains("@nonreentrant"));
    }
}
