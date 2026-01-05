//! Macro system for Legalis DSL (v0.1.5).
//!
//! This module provides macro definition, expansion, hygiene, variadic parameters,
//! and conditional expansion for creating reusable statute patterns.

use crate::ast::{ConditionNode, EffectNode, StatuteNode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A macro definition with parameters and body template
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MacroDefinition {
    /// Name of the macro
    pub name: String,
    /// Parameter names (e.g., ["age", "income"])
    pub parameters: Vec<MacroParameter>,
    /// Body of the macro (statute template)
    pub body: MacroBody,
}

/// A macro parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MacroParameter {
    /// Parameter name
    pub name: String,
    /// Whether this is a variadic parameter (can accept multiple values)
    pub is_variadic: bool,
    /// Optional default value
    pub default: Option<String>,
}

/// The body of a macro (template for statute generation)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MacroBody {
    /// Title template (may contain parameter placeholders)
    pub title: String,
    /// Condition templates
    pub conditions: Vec<ConditionTemplate>,
    /// Effect templates
    pub effects: Vec<EffectTemplate>,
    /// Conditional expansion directives
    pub conditionals: Vec<ConditionalDirective>,
}

/// A template for a condition with parameter placeholders
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionTemplate {
    /// Direct condition (will be substituted)
    Direct(ConditionNode),
    /// Placeholder for parameter substitution
    Placeholder(String),
    /// Conditional block (#IF param)
    Conditional {
        condition: String,
        then_branch: Box<ConditionTemplate>,
        else_branch: Option<Box<ConditionTemplate>>,
    },
}

/// A template for an effect with parameter placeholders
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectTemplate {
    /// Effect type (grant, revoke, etc.)
    pub effect_type: String,
    /// Description template (may contain ${param} placeholders)
    pub description: String,
}

/// Conditional expansion directive (#IF, #ELSE)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionalDirective {
    /// Parameter name to check
    pub parameter: String,
    /// Condition to evaluate (e.g., "defined", "equals:value")
    pub condition: DirectiveCondition,
}

/// Condition for a directive
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DirectiveCondition {
    /// Check if parameter is defined
    Defined,
    /// Check if parameter equals a value
    Equals(String),
    /// Check if parameter is not empty
    NotEmpty,
}

/// Macro expansion context with parameter bindings
#[derive(Debug, Clone)]
pub struct ExpansionContext {
    /// Parameter name -> value mappings
    pub bindings: HashMap<String, Vec<String>>,
    /// Hygiene counter for generating unique identifiers
    pub hygiene_counter: usize,
}

impl ExpansionContext {
    /// Creates a new expansion context
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            hygiene_counter: 0,
        }
    }

    /// Binds a parameter to a value
    pub fn bind(&mut self, param: String, value: String) {
        self.bindings.entry(param).or_default().push(value);
    }

    /// Gets a bound parameter value
    pub fn get(&self, param: &str) -> Option<&Vec<String>> {
        self.bindings.get(param)
    }

    /// Generates a hygienic identifier (unique name to avoid conflicts)
    pub fn gen_hygienic_id(&mut self, base: &str) -> String {
        self.hygiene_counter += 1;
        format!("{}_hygiene_{}", base, self.hygiene_counter)
    }

    /// Substitutes parameter placeholders in a string
    pub fn substitute(&self, template: &str) -> String {
        let mut result = template.to_string();
        for (param, values) in &self.bindings {
            // Replace ${param} with the first value
            if let Some(value) = values.first() {
                result = result.replace(&format!("${{{}}}", param), value);
                result = result.replace(&format!("${}", param), value);
            }
        }
        result
    }
}

impl Default for ExpansionContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro expander for expanding macro invocations
#[derive(Debug, Clone)]
pub struct MacroExpander {
    /// Registry of defined macros
    macros: HashMap<String, MacroDefinition>,
}

impl MacroExpander {
    /// Creates a new macro expander
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
        }
    }

    /// Registers a macro definition
    pub fn define(&mut self, macro_def: MacroDefinition) {
        self.macros.insert(macro_def.name.clone(), macro_def);
    }

    /// Checks if a macro is defined
    pub fn is_defined(&self, name: &str) -> bool {
        self.macros.contains_key(name)
    }

    /// Expands a macro invocation into a statute
    pub fn expand(
        &self,
        name: &str,
        args: Vec<String>,
    ) -> Result<StatuteNode, MacroExpansionError> {
        let macro_def = self
            .macros
            .get(name)
            .ok_or_else(|| MacroExpansionError::UndefinedMacro(name.to_string()))?;

        // Create expansion context
        let mut context = ExpansionContext::new();

        // Bind parameters
        self.bind_parameters(macro_def, args, &mut context)?;

        // Generate statute from template
        let statute = self.expand_body(macro_def, &context)?;

        Ok(statute)
    }

    /// Binds macro parameters to argument values
    fn bind_parameters(
        &self,
        macro_def: &MacroDefinition,
        args: Vec<String>,
        context: &mut ExpansionContext,
    ) -> Result<(), MacroExpansionError> {
        let mut arg_iter = args.into_iter();

        for param in &macro_def.parameters {
            if param.is_variadic {
                // Variadic parameter: consume all remaining arguments
                let remaining: Vec<String> = arg_iter.collect();
                if remaining.is_empty() {
                    if let Some(default) = &param.default {
                        context.bind(param.name.clone(), default.clone());
                    }
                } else {
                    for value in remaining {
                        context.bind(param.name.clone(), value);
                    }
                }
                break;
            } else if let Some(value) = arg_iter.next() {
                context.bind(param.name.clone(), value);
            } else if let Some(default) = &param.default {
                context.bind(param.name.clone(), default.clone());
            } else {
                return Err(MacroExpansionError::MissingParameter(param.name.clone()));
            }
        }

        Ok(())
    }

    /// Expands the macro body into a statute
    fn expand_body(
        &self,
        macro_def: &MacroDefinition,
        context: &ExpansionContext,
    ) -> Result<StatuteNode, MacroExpansionError> {
        // Generate hygienic ID
        let id = context.substitute(&format!("{}_{}", macro_def.name, context.hygiene_counter));

        // Substitute title
        let title = context.substitute(&macro_def.body.title);

        // Expand conditions
        let mut conditions = Vec::new();
        for cond_template in &macro_def.body.conditions {
            if let Some(cond) = self.expand_condition_template(cond_template, context)? {
                conditions.push(cond);
            }
        }

        // Expand effects
        let mut effects = Vec::new();
        for effect_template in &macro_def.body.effects {
            effects.push(EffectNode {
                effect_type: context.substitute(&effect_template.effect_type),
                description: context.substitute(&effect_template.description),
                parameters: vec![],
            });
        }

        Ok(StatuteNode {
            id,
            visibility: crate::module_system::Visibility::Private,
            title,
            conditions,
            effects,
            discretion: None,
            exceptions: vec![],
            amendments: vec![],
            supersedes: vec![],
            defaults: vec![],
            requires: vec![],
            delegates: vec![],
            scope: None,
            constraints: vec![],
            priority: None,
        })
    }

    /// Expands a condition template
    fn expand_condition_template(
        &self,
        template: &ConditionTemplate,
        context: &ExpansionContext,
    ) -> Result<Option<ConditionNode>, MacroExpansionError> {
        match template {
            ConditionTemplate::Direct(cond) => Ok(Some(cond.clone())),
            ConditionTemplate::Placeholder(_param) => {
                // For now, we don't support condition placeholders directly
                // This would require parsing the parameter value as a condition
                Err(MacroExpansionError::UnsupportedFeature(
                    "Condition placeholders not yet implemented".to_string(),
                ))
            }
            ConditionTemplate::Conditional {
                condition,
                then_branch,
                else_branch,
            } => {
                // Evaluate the condition
                let should_expand = self.evaluate_directive_condition(condition, context);

                if should_expand {
                    self.expand_condition_template(then_branch, context)
                } else if let Some(else_branch) = else_branch {
                    self.expand_condition_template(else_branch, context)
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// Evaluates a directive condition (e.g., #IF param)
    fn evaluate_directive_condition(&self, condition: &str, context: &ExpansionContext) -> bool {
        // Simple evaluation: check if parameter is defined and non-empty
        if let Some(values) = context.get(condition) {
            !values.is_empty() && values.iter().any(|v| !v.is_empty())
        } else {
            false
        }
    }

    /// Returns all registered macro names
    pub fn list_macros(&self) -> Vec<String> {
        self.macros.keys().cloned().collect()
    }

    /// Adds built-in macros for common patterns
    pub fn register_builtins(&mut self) {
        // Age requirement macro
        self.define(MacroDefinition {
            name: "age_requirement".to_string(),
            parameters: vec![
                MacroParameter {
                    name: "min_age".to_string(),
                    is_variadic: false,
                    default: Some("18".to_string()),
                },
                MacroParameter {
                    name: "benefit".to_string(),
                    is_variadic: false,
                    default: Some("eligibility".to_string()),
                },
            ],
            body: MacroBody {
                title: "Age requirement for ${benefit}".to_string(),
                conditions: vec![],
                effects: vec![EffectTemplate {
                    effect_type: "grant".to_string(),
                    description: "${benefit} if age >= ${min_age}".to_string(),
                }],
                conditionals: vec![],
            },
        });

        // Income threshold macro
        self.define(MacroDefinition {
            name: "income_threshold".to_string(),
            parameters: vec![
                MacroParameter {
                    name: "max_income".to_string(),
                    is_variadic: false,
                    default: None,
                },
                MacroParameter {
                    name: "program".to_string(),
                    is_variadic: false,
                    default: Some("assistance".to_string()),
                },
            ],
            body: MacroBody {
                title: "Income threshold for ${program}".to_string(),
                conditions: vec![],
                effects: vec![EffectTemplate {
                    effect_type: "grant".to_string(),
                    description: "${program} if income <= ${max_income}".to_string(),
                }],
                conditionals: vec![],
            },
        });
    }
}

impl Default for MacroExpander {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors that can occur during macro expansion
#[derive(Debug, Clone, PartialEq)]
pub enum MacroExpansionError {
    /// Macro is not defined
    UndefinedMacro(String),
    /// Missing required parameter
    MissingParameter(String),
    /// Unsupported feature
    UnsupportedFeature(String),
    /// Invalid argument
    InvalidArgument(String),
}

impl std::fmt::Display for MacroExpansionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacroExpansionError::UndefinedMacro(name) => {
                write!(f, "Undefined macro: {}", name)
            }
            MacroExpansionError::MissingParameter(param) => {
                write!(f, "Missing required parameter: {}", param)
            }
            MacroExpansionError::UnsupportedFeature(msg) => {
                write!(f, "Unsupported feature: {}", msg)
            }
            MacroExpansionError::InvalidArgument(msg) => {
                write!(f, "Invalid argument: {}", msg)
            }
        }
    }
}

impl std::error::Error for MacroExpansionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expansion_context_new() {
        let ctx = ExpansionContext::new();
        assert_eq!(ctx.bindings.len(), 0);
        assert_eq!(ctx.hygiene_counter, 0);
    }

    #[test]
    fn test_expansion_context_bind() {
        let mut ctx = ExpansionContext::new();
        ctx.bind("age".to_string(), "21".to_string());
        assert_eq!(ctx.get("age").unwrap(), &vec!["21"]);
    }

    #[test]
    fn test_expansion_context_substitute() {
        let mut ctx = ExpansionContext::new();
        ctx.bind("age".to_string(), "21".to_string());
        ctx.bind("benefit".to_string(), "voting".to_string());

        let result = ctx.substitute("Eligible for ${benefit} at age ${age}");
        assert_eq!(result, "Eligible for voting at age 21");
    }

    #[test]
    fn test_hygienic_id_generation() {
        let mut ctx = ExpansionContext::new();
        let id1 = ctx.gen_hygienic_id("test");
        let id2 = ctx.gen_hygienic_id("test");
        assert_ne!(id1, id2);
        assert!(id1.contains("hygiene"));
        assert!(id2.contains("hygiene"));
    }

    #[test]
    fn test_macro_expander_new() {
        let expander = MacroExpander::new();
        assert_eq!(expander.list_macros().len(), 0);
    }

    #[test]
    fn test_macro_definition() {
        let mut expander = MacroExpander::new();
        let macro_def = MacroDefinition {
            name: "test_macro".to_string(),
            parameters: vec![MacroParameter {
                name: "param1".to_string(),
                is_variadic: false,
                default: None,
            }],
            body: MacroBody {
                title: "Test ${param1}".to_string(),
                conditions: vec![],
                effects: vec![],
                conditionals: vec![],
            },
        };

        expander.define(macro_def);
        assert!(expander.is_defined("test_macro"));
    }

    #[test]
    fn test_simple_macro_expansion() {
        let mut expander = MacroExpander::new();
        let macro_def = MacroDefinition {
            name: "greeting".to_string(),
            parameters: vec![MacroParameter {
                name: "name".to_string(),
                is_variadic: false,
                default: Some("World".to_string()),
            }],
            body: MacroBody {
                title: "Hello ${name}".to_string(),
                conditions: vec![],
                effects: vec![EffectTemplate {
                    effect_type: "grant".to_string(),
                    description: "Greet ${name}".to_string(),
                }],
                conditionals: vec![],
            },
        };

        expander.define(macro_def);
        let result = expander.expand("greeting", vec!["Alice".to_string()]);
        assert!(result.is_ok());
        let statute = result.unwrap();
        assert_eq!(statute.title, "Hello Alice");
        assert_eq!(statute.effects[0].description, "Greet Alice");
    }

    #[test]
    fn test_macro_with_default_parameter() {
        let mut expander = MacroExpander::new();
        let macro_def = MacroDefinition {
            name: "test".to_string(),
            parameters: vec![MacroParameter {
                name: "value".to_string(),
                is_variadic: false,
                default: Some("default".to_string()),
            }],
            body: MacroBody {
                title: "Test ${value}".to_string(),
                conditions: vec![],
                effects: vec![],
                conditionals: vec![],
            },
        };

        expander.define(macro_def);
        let result = expander.expand("test", vec![]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "Test default");
    }

    #[test]
    fn test_undefined_macro_error() {
        let expander = MacroExpander::new();
        let result = expander.expand("undefined", vec![]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MacroExpansionError::UndefinedMacro(_)
        ));
    }

    #[test]
    fn test_missing_parameter_error() {
        let mut expander = MacroExpander::new();
        let macro_def = MacroDefinition {
            name: "test".to_string(),
            parameters: vec![MacroParameter {
                name: "required".to_string(),
                is_variadic: false,
                default: None,
            }],
            body: MacroBody {
                title: "Test".to_string(),
                conditions: vec![],
                effects: vec![],
                conditionals: vec![],
            },
        };

        expander.define(macro_def);
        let result = expander.expand("test", vec![]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MacroExpansionError::MissingParameter(_)
        ));
    }

    #[test]
    fn test_variadic_parameters() {
        let mut expander = MacroExpander::new();
        let macro_def = MacroDefinition {
            name: "list".to_string(),
            parameters: vec![
                MacroParameter {
                    name: "title".to_string(),
                    is_variadic: false,
                    default: None,
                },
                MacroParameter {
                    name: "items".to_string(),
                    is_variadic: true,
                    default: None,
                },
            ],
            body: MacroBody {
                title: "${title}".to_string(),
                conditions: vec![],
                effects: vec![],
                conditionals: vec![],
            },
        };

        expander.define(macro_def);
        let result = expander.expand(
            "list",
            vec![
                "My List".to_string(),
                "item1".to_string(),
                "item2".to_string(),
                "item3".to_string(),
            ],
        );
        assert!(result.is_ok());
        let statute = result.unwrap();
        assert_eq!(statute.title, "My List");
    }

    #[test]
    fn test_builtin_macros() {
        let mut expander = MacroExpander::new();
        expander.register_builtins();

        assert!(expander.is_defined("age_requirement"));
        assert!(expander.is_defined("income_threshold"));

        // Test age_requirement expansion
        let result = expander.expand(
            "age_requirement",
            vec!["21".to_string(), "voting".to_string()],
        );
        assert!(result.is_ok());
        let statute = result.unwrap();
        assert!(statute.title.contains("voting"));
        assert!(statute.effects[0].description.contains("21"));
    }
}
