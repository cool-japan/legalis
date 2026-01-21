//! Template Engine (テンプレートエンジン)
//!
//! This module provides the template rendering engine with variable substitution.

use super::error::{Result, TemplateError};
use super::types::{ContractTemplate, GeneratedContract, TemplateContext};
use std::collections::HashMap;

/// Template engine for rendering contracts (テンプレートエンジン)
pub struct TemplateEngine {
    /// Template registry
    templates: HashMap<String, ContractTemplate>,
}

impl TemplateEngine {
    /// Create a new template engine
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    /// Register a template
    pub fn register_template(&mut self, template: ContractTemplate) -> &mut Self {
        self.templates.insert(template.id.clone(), template);
        self
    }

    /// Get a template by ID
    pub fn get_template(&self, template_id: &str) -> Result<&ContractTemplate> {
        self.templates
            .get(template_id)
            .ok_or_else(|| TemplateError::TemplateNotFound {
                template_id: template_id.to_string(),
            })
    }

    /// Render a template with the given context
    pub fn render(
        &self,
        template_id: &str,
        context: &TemplateContext,
    ) -> Result<GeneratedContract> {
        let template = self.get_template(template_id)?;

        // Validate required variables
        template
            .validate_context(context)
            .map_err(|missing| TemplateError::MissingRequiredVariables { variables: missing })?;

        // Render template content
        let rendered_content = self.render_template_string(&template.content, context)?;

        // Create generated contract
        let contract = GeneratedContract::new(
            template_id,
            template.template_type,
            rendered_content,
            context.variables.clone(),
        );

        Ok(contract)
    }

    /// Render a template string with variable substitution
    pub fn render_template_string(
        &self,
        template: &str,
        context: &TemplateContext,
    ) -> Result<String> {
        let mut result = template.to_string();

        // Replace {{variable}} placeholders
        for (key, value) in &context.variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, &value.as_string());
        }

        // Process conditional blocks: {{#if variable}}...{{/if}}
        result = self.process_conditionals(&result, context)?;

        // Check for unresolved placeholders
        if result.contains("{{") && result.contains("}}") {
            // Extract unresolved placeholder
            if let Some(start) = result.find("{{")
                && let Some(end) = result[start..].find("}}")
            {
                let placeholder = &result[start + 2..start + end];
                return Err(TemplateError::TemplateRenderingError {
                    reason: format!("Unresolved placeholder: {}", placeholder),
                });
            }
        }

        Ok(result)
    }

    /// Process conditional blocks in template
    fn process_conditionals(&self, template: &str, context: &TemplateContext) -> Result<String> {
        let mut result = template.to_string();

        // Process {{#if variable}}...{{/if}} blocks
        while let Some(if_start) = result.find("{{#if ") {
            // Find the end of {{#if variable}}
            let if_header_end = match result[if_start..].find("}}") {
                Some(pos) => if_start + pos + 2,
                None => {
                    return Err(TemplateError::TemplateParsingError {
                        reason: "Unclosed {{#if}} tag".to_string(),
                    });
                }
            };

            // Extract variable name
            let variable_name = result[if_start + 6..if_header_end - 2].trim();

            // Find matching {{/if}}
            let if_end = match result[if_header_end..].find("{{/if}}") {
                Some(pos) => if_header_end + pos,
                None => {
                    return Err(TemplateError::TemplateParsingError {
                        reason: "Missing {{/if}} tag".to_string(),
                    });
                }
            };

            // Extract content between {{#if}} and {{/if}}
            let content = &result[if_header_end..if_end];

            // Check if variable is truthy
            let include_content = context.is_truthy(variable_name);

            // Replace the entire conditional block
            let replacement = if include_content {
                content.to_string()
            } else {
                String::new()
            };

            result.replace_range(if_start..if_end + 7, &replacement);
        }

        // Process {{#unless variable}}...{{/unless}} blocks
        while let Some(unless_start) = result.find("{{#unless ") {
            let unless_header_end = match result[unless_start..].find("}}") {
                Some(pos) => unless_start + pos + 2,
                None => {
                    return Err(TemplateError::TemplateParsingError {
                        reason: "Unclosed {{#unless}} tag".to_string(),
                    });
                }
            };

            let variable_name = result[unless_start + 10..unless_header_end - 2].trim();

            let unless_end = match result[unless_header_end..].find("{{/unless}}") {
                Some(pos) => unless_header_end + pos,
                None => {
                    return Err(TemplateError::TemplateParsingError {
                        reason: "Missing {{/unless}} tag".to_string(),
                    });
                }
            };

            let content = &result[unless_header_end..unless_end];

            // Include content if variable is falsy
            let include_content = !context.is_truthy(variable_name);

            let replacement = if include_content {
                content.to_string()
            } else {
                String::new()
            };

            result.replace_range(unless_start..unless_end + 11, &replacement);
        }

        Ok(result)
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract_templates::types::TemplateType;

    #[test]
    fn test_simple_variable_substitution() {
        let engine = TemplateEngine::new();
        let template_str = "従業員: {{employee_name}}\n雇用主: {{employer_name}}";

        let mut context = TemplateContext::new();
        context.set_string("employee_name", "山田太郎");
        context.set_string("employer_name", "株式会社ABC");

        let result = engine
            .render_template_string(template_str, &context)
            .unwrap();
        assert!(result.contains("山田太郎"));
        assert!(result.contains("株式会社ABC"));
    }

    #[test]
    fn test_conditional_rendering() {
        let engine = TemplateEngine::new();
        let template_str = "{{#if has_probation}}試用期間: {{probation_period}}ヶ月{{/if}}";

        // With probation
        let mut context = TemplateContext::new();
        context.set_boolean("has_probation", true);
        context.set_integer("probation_period", 3);

        let result = engine
            .render_template_string(template_str, &context)
            .unwrap();
        assert!(result.contains("試用期間: 3ヶ月"));

        // Without probation
        let mut context2 = TemplateContext::new();
        context2.set_boolean("has_probation", false);

        let result2 = engine
            .render_template_string(template_str, &context2)
            .unwrap();
        assert!(!result2.contains("試用期間"));
    }

    #[test]
    fn test_unless_conditional() {
        let engine = TemplateEngine::new();
        let template_str = "{{#unless is_part_time}}フルタイム雇用{{/unless}}";

        let mut context = TemplateContext::new();
        context.set_boolean("is_part_time", false);

        let result = engine
            .render_template_string(template_str, &context)
            .unwrap();
        assert!(result.contains("フルタイム雇用"));

        let mut context2 = TemplateContext::new();
        context2.set_boolean("is_part_time", true);

        let result2 = engine
            .render_template_string(template_str, &context2)
            .unwrap();
        assert!(!result2.contains("フルタイム雇用"));
    }

    #[test]
    fn test_template_registration_and_rendering() {
        let mut engine = TemplateEngine::new();

        let mut template = ContractTemplate::new(
            "simple_employment",
            "簡易雇用契約書",
            TemplateType::Employment,
            "{{employer_name}}は{{employee_name}}を雇用します。",
        );
        template.require_variable("employer_name");
        template.require_variable("employee_name");

        engine.register_template(template);

        let mut context = TemplateContext::new();
        context.set_string("employer_name", "株式会社ABC");
        context.set_string("employee_name", "山田太郎");

        let contract = engine.render("simple_employment", &context).unwrap();
        assert!(contract.content_ja.contains("株式会社ABC"));
        assert!(contract.content_ja.contains("山田太郎"));
    }

    #[test]
    fn test_missing_required_variable() {
        let mut engine = TemplateEngine::new();

        let mut template =
            ContractTemplate::new("test", "テスト", TemplateType::Custom, "{{required_var}}");
        template.require_variable("required_var");

        engine.register_template(template);

        let context = TemplateContext::new(); // Empty context

        let result = engine.render("test", &context);
        assert!(result.is_err());
        match result.unwrap_err() {
            TemplateError::MissingRequiredVariables { variables } => {
                assert_eq!(variables, vec!["required_var"]);
            }
            _ => panic!("Expected MissingRequiredVariables error"),
        }
    }

    #[test]
    fn test_unresolved_placeholder() {
        let engine = TemplateEngine::new();
        let template_str = "Hello {{unknown_variable}}";
        let context = TemplateContext::new();

        let result = engine.render_template_string(template_str, &context);
        assert!(result.is_err());
        match result.unwrap_err() {
            TemplateError::TemplateRenderingError { reason } => {
                assert!(reason.contains("unknown_variable"));
            }
            _ => panic!("Expected TemplateRenderingError"),
        }
    }
}
