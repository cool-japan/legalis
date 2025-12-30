//! Statute template system for parameterized statute generation.
//!
//! This module provides a template system that allows creating reusable
//! statute patterns with parameters that can be instantiated with specific values.

use crate::{DslError, DslResult};
use std::collections::HashMap;

/// A template for generating statutes with parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct StatuteTemplate {
    /// Template name/identifier
    pub name: String,
    /// Parameter names expected by this template
    pub parameters: Vec<String>,
    /// Template body (uses {{param_name}} placeholders)
    pub body: String,
    /// Optional description of what this template does
    pub description: Option<String>,
}

impl StatuteTemplate {
    /// Creates a new statute template.
    pub fn new(name: impl Into<String>, parameters: Vec<String>, body: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parameters,
            body: body.into(),
            description: None,
        }
    }

    /// Sets the template description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Instantiates the template with the given parameter values.
    pub fn instantiate(&self, values: &HashMap<String, String>) -> DslResult<String> {
        // Validate that all required parameters are provided
        for param in &self.parameters {
            if !values.contains_key(param) {
                return Err(DslError::parse_error(format!(
                    "Missing required parameter: {}",
                    param
                )));
            }
        }

        let mut result = self.body.clone();

        // Replace all {{param}} placeholders with values
        for (key, value) in values {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        // Check for any remaining placeholders (indicates unused parameters or typos)
        if result.contains("{{") {
            let remaining: Vec<&str> = result
                .split("{{")
                .skip(1)
                .filter_map(|s| s.split("}}").next())
                .collect();
            if !remaining.is_empty() {
                return Err(DslError::parse_error(format!(
                    "Template contains undefined parameters: {}",
                    remaining.join(", ")
                )));
            }
        }

        Ok(result)
    }

    /// Validates the template structure.
    pub fn validate(&self) -> DslResult<()> {
        // Check that all parameters declared are used in the body
        for param in &self.parameters {
            let placeholder = format!("{{{{{}}}}}", param);
            if !self.body.contains(&placeholder) {
                return Err(DslError::parse_error(format!(
                    "Parameter '{}' is declared but not used in template body",
                    param
                )));
            }
        }

        // Check that all placeholders in the body have corresponding parameters
        let mut body_clone = self.body.clone();
        while let Some(start) = body_clone.find("{{") {
            if let Some(end) = body_clone[start..].find("}}") {
                let param_name = &body_clone[start + 2..start + end];
                if !self.parameters.contains(&param_name.to_string()) {
                    return Err(DslError::parse_error(format!(
                        "Template uses undefined parameter: {}",
                        param_name
                    )));
                }
                body_clone = body_clone[start + end + 2..].to_string();
            } else {
                return Err(DslError::parse_error("Unclosed parameter placeholder {{"));
            }
        }

        Ok(())
    }
}

/// A library of predefined statute templates.
pub struct TemplateLibrary {
    templates: HashMap<String, StatuteTemplate>,
}

impl TemplateLibrary {
    /// Creates a new template library with standard templates.
    pub fn new() -> Self {
        let mut lib = Self {
            templates: HashMap::new(),
        };
        lib.add_standard_templates();
        lib
    }

    /// Adds a template to the library.
    pub fn add(&mut self, template: StatuteTemplate) -> DslResult<()> {
        template.validate()?;
        self.templates.insert(template.name.clone(), template);
        Ok(())
    }

    /// Gets a template by name.
    pub fn get(&self, name: &str) -> Option<&StatuteTemplate> {
        self.templates.get(name)
    }

    /// Lists all available template names.
    pub fn list_templates(&self) -> Vec<&str> {
        self.templates.keys().map(|s| s.as_str()).collect()
    }

    /// Instantiates a template by name with the given values.
    pub fn instantiate(
        &self,
        template_name: &str,
        values: &HashMap<String, String>,
    ) -> DslResult<String> {
        let template = self.get(template_name).ok_or_else(|| {
            DslError::parse_error(format!("Template not found: {}", template_name))
        })?;
        template.instantiate(values)
    }

    /// Adds standard templates to the library.
    fn add_standard_templates(&mut self) {
        // Age-based eligibility template
        let age_eligibility = StatuteTemplate::new(
            "age_eligibility",
            vec![
                "statute_id".to_string(),
                "title".to_string(),
                "min_age".to_string(),
                "benefit".to_string(),
            ],
            r#"STATUTE {{statute_id}}: "{{title}}" {
    WHEN AGE >= {{min_age}}
    THEN GRANT "{{benefit}}"
}"#,
        )
        .with_description("Basic age-based eligibility statute");
        let _ = self.add(age_eligibility);

        // Income-based benefit template
        let income_benefit = StatuteTemplate::new(
            "income_benefit",
            vec![
                "statute_id".to_string(),
                "title".to_string(),
                "max_income".to_string(),
                "benefit".to_string(),
            ],
            r#"STATUTE {{statute_id}}: "{{title}}" {
    WHEN INCOME <= {{max_income}}
    THEN GRANT "{{benefit}}"
}"#,
        )
        .with_description("Income-based benefit eligibility");

        let _ = self.add(income_benefit);

        // Age and income combined template
        let age_income_combined = StatuteTemplate::new(
            "age_income_eligibility",
            vec![
                "statute_id".to_string(),
                "title".to_string(),
                "min_age".to_string(),
                "max_income".to_string(),
                "benefit".to_string(),
            ],
            r#"STATUTE {{statute_id}}: "{{title}}" {
    WHEN AGE >= {{min_age}} AND INCOME <= {{max_income}}
    THEN GRANT "{{benefit}}"
}"#,
        )
        .with_description("Combined age and income eligibility");

        let _ = self.add(age_income_combined);

        // Temporal validity template
        let temporal = StatuteTemplate::new(
            "temporal_statute",
            vec![
                "statute_id".to_string(),
                "title".to_string(),
                "effective_date".to_string(),
                "expiry_date".to_string(),
                "condition".to_string(),
                "effect".to_string(),
            ],
            r#"STATUTE {{statute_id}}: "{{title}}" {
    EFFECTIVE_DATE {{effective_date}}
    EXPIRY_DATE {{expiry_date}}
    WHEN {{condition}}
    THEN {{effect}}
}"#,
        )
        .with_description("Statute with temporal validity");

        let _ = self.add(temporal);

        // Jurisdiction-specific template
        let jurisdictional = StatuteTemplate::new(
            "jurisdictional_statute",
            vec![
                "statute_id".to_string(),
                "title".to_string(),
                "jurisdiction".to_string(),
                "version".to_string(),
                "condition".to_string(),
                "effect".to_string(),
            ],
            r#"STATUTE {{statute_id}}: "{{title}}" {
    JURISDICTION "{{jurisdiction}}"
    VERSION {{version}}
    WHEN {{condition}}
    THEN {{effect}}
}"#,
        )
        .with_description("Jurisdiction-specific statute with versioning");

        let _ = self.add(jurisdictional);

        // Exception handling template
        let with_exception = StatuteTemplate::new(
            "statute_with_exception",
            vec![
                "statute_id".to_string(),
                "title".to_string(),
                "condition".to_string(),
                "effect".to_string(),
                "exception_condition".to_string(),
                "exception_desc".to_string(),
            ],
            r#"STATUTE {{statute_id}}: "{{title}}" {
    WHEN {{condition}}
    THEN {{effect}}
    EXCEPTION WHEN {{exception_condition}} "{{exception_desc}}"
}"#,
        )
        .with_description("Statute with exception clause");

        let _ = self.add(with_exception);

        // Attribute-based template
        let attribute_check = StatuteTemplate::new(
            "attribute_eligibility",
            vec![
                "statute_id".to_string(),
                "title".to_string(),
                "attribute".to_string(),
                "benefit".to_string(),
            ],
            r#"STATUTE {{statute_id}}: "{{title}}" {
    WHEN HAS {{attribute}}
    THEN GRANT "{{benefit}}"
}"#,
        )
        .with_description("Simple attribute-based eligibility");

        let _ = self.add(attribute_check);
    }
}

impl Default for TemplateLibrary {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating statute templates programmatically.
pub struct TemplateBuilder {
    name: String,
    parameters: Vec<String>,
    body_parts: Vec<String>,
    description: Option<String>,
}

impl TemplateBuilder {
    /// Creates a new template builder.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parameters: Vec::new(),
            body_parts: Vec::new(),
            description: None,
        }
    }

    /// Adds a parameter to the template.
    pub fn parameter(mut self, name: impl Into<String>) -> Self {
        self.parameters.push(name.into());
        self
    }

    /// Adds multiple parameters at once.
    pub fn parameters<I, S>(mut self, params: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.parameters.extend(params.into_iter().map(|s| s.into()));
        self
    }

    /// Adds a line to the template body.
    pub fn line(mut self, line: impl Into<String>) -> Self {
        self.body_parts.push(line.into());
        self
    }

    /// Sets the template description.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Builds the template.
    pub fn build(self) -> StatuteTemplate {
        let body = self.body_parts.join("\n");
        let mut template = StatuteTemplate::new(self.name, self.parameters, body);
        if let Some(desc) = self.description {
            template = template.with_description(desc);
        }
        template
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_instantiation() {
        let template = StatuteTemplate::new(
            "test",
            vec!["id".to_string(), "age".to_string()],
            "STATUTE {{id}}: \"Test\" { WHEN AGE >= {{age}} }",
        );

        let mut values = HashMap::new();
        values.insert("id".to_string(), "test-1".to_string());
        values.insert("age".to_string(), "18".to_string());

        let result = template.instantiate(&values).unwrap();
        assert_eq!(result, "STATUTE test-1: \"Test\" { WHEN AGE >= 18 }");
    }

    #[test]
    fn test_template_missing_parameter() {
        let template = StatuteTemplate::new(
            "test",
            vec!["id".to_string(), "age".to_string()],
            "STATUTE {{id}}: \"Test\" { WHEN AGE >= {{age}} }",
        );

        let mut values = HashMap::new();
        values.insert("id".to_string(), "test-1".to_string());

        let result = template.instantiate(&values);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Missing required parameter: age")
        );
    }

    #[test]
    fn test_template_validation() {
        let template = StatuteTemplate::new(
            "test",
            vec!["id".to_string(), "age".to_string()],
            "STATUTE {{id}}: \"Test\" { WHEN AGE >= {{age}} }",
        );

        assert!(template.validate().is_ok());
    }

    #[test]
    fn test_template_validation_unused_parameter() {
        let template = StatuteTemplate::new(
            "test",
            vec!["id".to_string(), "age".to_string(), "unused".to_string()],
            "STATUTE {{id}}: \"Test\" { WHEN AGE >= {{age}} }",
        );

        let result = template.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("declared but not used")
        );
    }

    #[test]
    fn test_template_validation_undefined_parameter() {
        let template = StatuteTemplate::new(
            "test",
            vec!["id".to_string()],
            "STATUTE {{id}}: \"Test\" { WHEN AGE >= {{age}} }",
        );

        let result = template.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("undefined parameter: age")
        );
    }

    #[test]
    fn test_template_library() {
        let lib = TemplateLibrary::new();

        assert!(lib.get("age_eligibility").is_some());
        assert!(lib.get("income_benefit").is_some());
        assert!(lib.get("nonexistent").is_none());

        let templates = lib.list_templates();
        assert!(templates.contains(&"age_eligibility"));
        assert!(templates.contains(&"income_benefit"));
    }

    #[test]
    fn test_template_library_instantiation() {
        let lib = TemplateLibrary::new();

        let mut values = HashMap::new();
        values.insert("statute_id".to_string(), "senior-benefit".to_string());
        values.insert("title".to_string(), "Senior Citizen Benefit".to_string());
        values.insert("min_age".to_string(), "65".to_string());
        values.insert("benefit".to_string(), "Free public transport".to_string());

        let result = lib.instantiate("age_eligibility", &values).unwrap();
        assert!(result.contains("STATUTE senior-benefit"));
        assert!(result.contains("Senior Citizen Benefit"));
        assert!(result.contains("AGE >= 65"));
        assert!(result.contains("Free public transport"));
    }

    #[test]
    fn test_template_builder() {
        let template = TemplateBuilder::new("custom")
            .parameter("id")
            .parameter("value")
            .line("STATUTE {{id}}: \"Custom\" {")
            .line("    WHEN AGE > {{value}}")
            .line("}")
            .description("Custom template")
            .build();

        assert_eq!(template.name, "custom");
        assert_eq!(template.parameters.len(), 2);
        assert!(template.description.is_some());
        assert!(template.validate().is_ok());
    }

    #[test]
    fn test_complex_template() {
        let lib = TemplateLibrary::new();

        let mut values = HashMap::new();
        values.insert("statute_id".to_string(), "low-income-housing".to_string());
        values.insert(
            "title".to_string(),
            "Low Income Housing Assistance".to_string(),
        );
        values.insert("min_age".to_string(), "18".to_string());
        values.insert("max_income".to_string(), "30000".to_string());
        values.insert("benefit".to_string(), "Housing subsidy".to_string());

        let result = lib.instantiate("age_income_eligibility", &values).unwrap();
        assert!(result.contains("AGE >= 18"));
        assert!(result.contains("INCOME <= 30000"));
        assert!(result.contains("Housing subsidy"));
    }
}
