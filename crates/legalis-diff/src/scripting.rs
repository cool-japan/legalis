//! Scripting support for custom diff analysis using Rhai.
//!
//! This module provides a scripting interface for customizing diff analysis behavior
//! using the Rhai scripting language. Scripts can define custom analyzers, validators,
//! and transformations.
//!
//! # Example
//!
//! ```
//! use legalis_diff::scripting::ScriptEngine;
//!
//! let mut engine = ScriptEngine::new();
//!
//! // Load a script
//! let script = r#"
//!     fn analyze_diff(diff) {
//!         let change_count = diff.changes.len();
//!         if change_count > 10 {
//!             return #{
//!                 severity: "high",
//!                 message: "Too many changes"
//!             };
//!         }
//!         return #{
//!             severity: "low",
//!             message: "Normal change count"
//!         };
//!     }
//! "#;
//!
//! engine.load_script("analyzer", script).unwrap();
//! ```

use crate::StatuteDiff;
use crate::plugins::{AnalysisResult, Finding, FindingSeverity, PluginError};
use rhai::{AST, Dynamic, Engine, Map, Scope};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A scripting engine for custom diff analysis.
pub struct ScriptEngine {
    engine: Engine,
    scripts: Arc<RwLock<HashMap<String, AST>>>,
}

impl ScriptEngine {
    /// Creates a new script engine.
    #[must_use]
    pub fn new() -> Self {
        let mut engine = Engine::new();

        // Register custom types
        engine.register_type::<ScriptDiff>();
        engine.register_type::<ScriptChange>();
        engine.register_type::<ScriptImpact>();

        // Register getters for ScriptDiff
        engine.register_get("change_count", |diff: &mut ScriptDiff| diff.change_count);
        engine.register_get("statute_id", |diff: &mut ScriptDiff| {
            diff.statute_id.clone()
        });
        engine.register_get("changes", |diff: &mut ScriptDiff| diff.changes.clone());
        engine.register_get("impact", |diff: &mut ScriptDiff| diff.impact.clone());

        // Register getters for ScriptChange
        engine.register_get("change_type", |change: &mut ScriptChange| {
            change.change_type.clone()
        });
        engine.register_get("target", |change: &mut ScriptChange| change.target.clone());
        engine.register_get("description", |change: &mut ScriptChange| {
            change.description.clone()
        });

        // Register getters for ScriptImpact
        engine.register_get("severity", |impact: &mut ScriptImpact| {
            impact.severity.clone()
        });
        engine.register_get("affects_eligibility", |impact: &mut ScriptImpact| {
            impact.affects_eligibility
        });
        engine.register_get("affects_outcome", |impact: &mut ScriptImpact| {
            impact.affects_outcome
        });
        engine.register_get("discretion_changed", |impact: &mut ScriptImpact| {
            impact.discretion_changed
        });

        // Register helper functions
        engine.register_fn("create_finding", create_finding);
        engine.register_fn("create_result", create_analysis_result);

        Self {
            engine,
            scripts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Loads a script into the engine.
    pub fn load_script(&mut self, name: &str, script: &str) -> Result<(), PluginError> {
        let ast = self.engine.compile(script).map_err(|e| {
            PluginError::InitializationFailed(format!("Script compilation failed: {e}"))
        })?;

        let mut scripts = self.scripts.write().unwrap();
        scripts.insert(name.to_string(), ast);

        Ok(())
    }

    /// Executes a script function with a diff.
    pub fn execute(
        &self,
        script_name: &str,
        function_name: &str,
        diff: &StatuteDiff,
    ) -> Result<AnalysisResult, PluginError> {
        let scripts = self.scripts.read().unwrap();
        let ast = scripts
            .get(script_name)
            .ok_or_else(|| PluginError::NotFound(script_name.to_string()))?;

        let script_diff = ScriptDiff::from_statute_diff(diff);

        let mut scope = Scope::new();

        let result: Dynamic = self
            .engine
            .call_fn(&mut scope, ast, function_name, (script_diff,))
            .map_err(|e| PluginError::ExecutionFailed(format!("Script execution failed: {e}")))?;

        // Convert result to AnalysisResult
        self.convert_result(result, script_name)
    }

    /// Evaluates an expression with a diff.
    pub fn evaluate(&self, script_name: &str, diff: &StatuteDiff) -> Result<Dynamic, PluginError> {
        let scripts = self.scripts.read().unwrap();
        let ast = scripts
            .get(script_name)
            .ok_or_else(|| PluginError::NotFound(script_name.to_string()))?;

        let script_diff = ScriptDiff::from_statute_diff(diff);

        let mut scope = Scope::new();
        scope.push("diff", script_diff);

        self.engine
            .eval_ast_with_scope(&mut scope, ast)
            .map_err(|e| PluginError::ExecutionFailed(format!("Evaluation failed: {e}")))
    }

    /// Converts a dynamic result to an AnalysisResult.
    fn convert_result(
        &self,
        result: Dynamic,
        script_name: &str,
    ) -> Result<AnalysisResult, PluginError> {
        if let Some(map) = result.try_cast::<Map>() {
            let mut findings = Vec::new();
            let mut metadata = HashMap::new();

            if let Some(severity) = map
                .get("severity")
                .and_then(|v| v.clone().try_cast::<String>())
                && let Some(message) = map
                    .get("message")
                    .and_then(|v| v.clone().try_cast::<String>())
            {
                let finding_severity = match severity.to_lowercase().as_str() {
                    "critical" => FindingSeverity::Critical,
                    "high" => FindingSeverity::High,
                    "medium" => FindingSeverity::Medium,
                    "low" => FindingSeverity::Low,
                    _ => FindingSeverity::Info,
                };

                findings.push(Finding {
                    severity: finding_severity,
                    category: script_name.to_string(),
                    message,
                    location: None,
                    suggestion: None,
                });
            }

            if let Some(meta) = map
                .get("metadata")
                .and_then(|v| v.clone().try_cast::<Map>())
            {
                for (key, value) in meta {
                    metadata.insert(key.to_string(), value.to_string());
                }
            }

            Ok(AnalysisResult {
                plugin_name: format!("script:{script_name}"),
                findings,
                confidence: 0.85,
                metadata,
            })
        } else {
            Err(PluginError::ExecutionFailed(
                "Script must return a map with severity and message".to_string(),
            ))
        }
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Script-friendly representation of a diff.
#[derive(Debug, Clone)]
pub struct ScriptDiff {
    pub statute_id: String,
    pub changes: Vec<ScriptChange>,
    pub impact: ScriptImpact,
    pub change_count: i64,
}

impl ScriptDiff {
    fn from_statute_diff(diff: &StatuteDiff) -> Self {
        Self {
            statute_id: diff.statute_id.clone(),
            changes: diff.changes.iter().map(ScriptChange::from).collect(),
            impact: ScriptImpact::from(&diff.impact),
            change_count: diff.changes.len() as i64,
        }
    }
}

/// Script-friendly representation of a change.
#[derive(Debug, Clone)]
pub struct ScriptChange {
    pub change_type: String,
    pub target: String,
    pub description: String,
}

impl From<&crate::Change> for ScriptChange {
    fn from(change: &crate::Change) -> Self {
        Self {
            change_type: format!("{:?}", change.change_type),
            target: change.target.to_string(),
            description: change.description.clone(),
        }
    }
}

/// Script-friendly representation of impact.
#[derive(Debug, Clone)]
pub struct ScriptImpact {
    pub severity: String,
    pub affects_eligibility: bool,
    pub affects_outcome: bool,
    pub discretion_changed: bool,
}

impl From<&crate::ImpactAssessment> for ScriptImpact {
    fn from(impact: &crate::ImpactAssessment) -> Self {
        Self {
            severity: format!("{:?}", impact.severity),
            affects_eligibility: impact.affects_eligibility,
            affects_outcome: impact.affects_outcome,
            discretion_changed: impact.discretion_changed,
        }
    }
}

/// Creates a finding from script.
#[allow(dead_code)]
fn create_finding(severity: String, category: String, message: String) -> Map {
    let mut map = Map::new();
    map.insert("severity".into(), severity.into());
    map.insert("category".into(), category.into());
    map.insert("message".into(), message.into());
    map
}

/// Creates an analysis result from script.
#[allow(dead_code)]
fn create_analysis_result(findings: Vec<Map>, metadata: Map) -> Map {
    let mut result = Map::new();
    result.insert("findings".into(), findings.into());
    result.insert("metadata".into(), metadata.into());
    result
}

/// A script-based diff analyzer plugin.
pub struct ScriptAnalyzer {
    engine: ScriptEngine,
    script_name: String,
    function_name: String,
}

impl ScriptAnalyzer {
    /// Creates a new script analyzer.
    pub fn new(script: &str, function_name: &str) -> Result<Self, PluginError> {
        let mut engine = ScriptEngine::new();
        let script_name = "analyzer".to_string();
        engine.load_script(&script_name, script)?;

        Ok(Self {
            engine,
            script_name,
            function_name: function_name.to_string(),
        })
    }

    /// Analyzes a diff using the script.
    pub fn analyze(&self, diff: &StatuteDiff) -> Result<AnalysisResult, PluginError> {
        self.engine
            .execute(&self.script_name, &self.function_name, diff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Change, ChangeTarget, ChangeType, ImpactAssessment, Severity};

    #[test]
    fn test_script_engine_basic() {
        let mut engine = ScriptEngine::new();

        let script = r#"
            fn analyze(diff) {
                #{
                    severity: "high",
                    message: "Test message"
                }
            }
        "#;

        engine.load_script("test", script).unwrap();

        let diff = create_test_diff();
        let result = engine.execute("test", "analyze", &diff).unwrap();

        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].message, "Test message");
    }

    #[test]
    fn test_script_analyzer() {
        let script = r#"
            fn analyze_changes(diff) {
                let count = diff.change_count;
                if count > 5 {
                    #{
                        severity: "high",
                        message: "Too many changes"
                    }
                } else {
                    #{
                        severity: "low",
                        message: "Normal change count"
                    }
                }
            }
        "#;

        let analyzer = ScriptAnalyzer::new(script, "analyze_changes").unwrap();
        let diff = create_test_diff();
        let result = analyzer.analyze(&diff).unwrap();

        assert_eq!(result.findings.len(), 1);
    }

    #[allow(dead_code)]
    fn create_test_diff() -> StatuteDiff {
        StatuteDiff {
            statute_id: "test-123".to_string(),
            version_info: None,
            changes: vec![
                Change {
                    change_type: ChangeType::Modified,
                    target: ChangeTarget::Precondition { index: 0 },
                    description: "Age changed".to_string(),
                    old_value: Some("65".to_string()),
                    new_value: Some("60".to_string()),
                },
                Change {
                    change_type: ChangeType::Added,
                    target: ChangeTarget::Precondition { index: 1 },
                    description: "Income requirement added".to_string(),
                    old_value: None,
                    new_value: Some("50000".to_string()),
                },
            ],
            impact: ImpactAssessment {
                severity: Severity::Moderate,
                affects_eligibility: true,
                affects_outcome: false,
                discretion_changed: false,
                notes: vec!["Test impact".to_string()],
            },
        }
    }
}
