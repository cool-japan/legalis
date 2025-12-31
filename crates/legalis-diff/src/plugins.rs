//! Plugin system for extending diff analysis capabilities.
//!
//! This module provides a flexible plugin architecture for adding custom analyzers,
//! validators, and transformers to the diff system.

use crate::StatuteDiff;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;

/// Result type for plugin operations.
pub type PluginResult<T> = Result<T, PluginError>;

/// Errors that can occur in plugin operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum PluginError {
    /// Plugin initialization failed
    #[error("Plugin initialization failed: {0}")]
    InitializationFailed(String),

    /// Plugin execution failed
    #[error("Plugin execution failed: {0}")]
    ExecutionFailed(String),

    /// Plugin configuration invalid
    #[error("Invalid plugin configuration: {0}")]
    InvalidConfiguration(String),

    /// Plugin not found
    #[error("Plugin not found: {0}")]
    NotFound(String),

    /// Plugin dependency missing
    #[error("Missing plugin dependency: {0}")]
    MissingDependency(String),
}

/// Metadata about a plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin author
    pub author: String,
    /// Plugin description
    pub description: String,
    /// Plugin dependencies (names of other plugins)
    pub dependencies: Vec<String>,
}

impl PluginMetadata {
    /// Creates new plugin metadata.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        author: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            author: author.into(),
            description: description.into(),
            dependencies: Vec::new(),
        }
    }

    /// Adds a dependency to the plugin.
    #[must_use]
    pub fn with_dependency(mut self, dep: impl Into<String>) -> Self {
        self.dependencies.push(dep.into());
        self
    }
}

/// Analysis result from a custom analyzer plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Plugin that generated this result
    pub plugin_name: String,
    /// Analysis findings
    pub findings: Vec<Finding>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// A finding from plugin analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// Finding severity
    pub severity: FindingSeverity,
    /// Finding category
    pub category: String,
    /// Finding message
    pub message: String,
    /// Location in diff (optional)
    pub location: Option<String>,
    /// Suggested action
    pub suggestion: Option<String>,
}

/// Severity level for findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FindingSeverity {
    /// Informational finding
    Info,
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Trait for custom diff analyzer plugins.
pub trait AnalyzerPlugin: Send + Sync {
    /// Returns plugin metadata.
    fn metadata(&self) -> &PluginMetadata;

    /// Initializes the plugin with configuration.
    fn initialize(&mut self, config: &HashMap<String, String>) -> PluginResult<()> {
        let _ = config;
        Ok(())
    }

    /// Analyzes a diff and returns findings.
    fn analyze(&self, diff: &StatuteDiff) -> PluginResult<AnalysisResult>;

    /// Returns the plugin as Any for downcasting.
    fn as_any(&self) -> &dyn Any;
}

/// Trait for custom validator plugins.
pub trait ValidatorPlugin: Send + Sync {
    /// Returns plugin metadata.
    fn metadata(&self) -> &PluginMetadata;

    /// Initializes the plugin with configuration.
    fn initialize(&mut self, config: &HashMap<String, String>) -> PluginResult<()> {
        let _ = config;
        Ok(())
    }

    /// Validates a diff.
    fn validate(&self, diff: &StatuteDiff) -> PluginResult<ValidationResult>;

    /// Returns the plugin as Any for downcasting.
    fn as_any(&self) -> &dyn Any;
}

/// Result of validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether validation passed
    pub passed: bool,
    /// Validation score (0.0 to 1.0)
    pub score: f64,
    /// Validation messages
    pub messages: Vec<String>,
    /// Failed rules
    pub failed_rules: Vec<String>,
}

/// Trait for custom transformer plugins.
pub trait TransformerPlugin: Send + Sync {
    /// Returns plugin metadata.
    fn metadata(&self) -> &PluginMetadata;

    /// Initializes the plugin with configuration.
    fn initialize(&mut self, config: &HashMap<String, String>) -> PluginResult<()> {
        let _ = config;
        Ok(())
    }

    /// Transforms a diff.
    fn transform(&self, diff: StatuteDiff) -> PluginResult<StatuteDiff>;

    /// Returns the plugin as Any for downcasting.
    fn as_any(&self) -> &dyn Any;
}

/// Plugin registry for managing all plugins.
pub struct PluginRegistry {
    analyzers: HashMap<String, Box<dyn AnalyzerPlugin>>,
    validators: HashMap<String, Box<dyn ValidatorPlugin>>,
    transformers: HashMap<String, Box<dyn TransformerPlugin>>,
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginRegistry {
    /// Creates a new plugin registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            analyzers: HashMap::new(),
            validators: HashMap::new(),
            transformers: HashMap::new(),
        }
    }

    /// Registers an analyzer plugin.
    ///
    /// # Errors
    ///
    /// Returns error if plugin with same name already exists.
    pub fn register_analyzer(&mut self, plugin: Box<dyn AnalyzerPlugin>) -> PluginResult<()> {
        let name = plugin.metadata().name.clone();
        if self.analyzers.contains_key(&name) {
            return Err(PluginError::InitializationFailed(format!(
                "Analyzer plugin '{}' already registered",
                name
            )));
        }
        self.analyzers.insert(name, plugin);
        Ok(())
    }

    /// Registers a validator plugin.
    ///
    /// # Errors
    ///
    /// Returns error if plugin with same name already exists.
    pub fn register_validator(&mut self, plugin: Box<dyn ValidatorPlugin>) -> PluginResult<()> {
        let name = plugin.metadata().name.clone();
        if self.validators.contains_key(&name) {
            return Err(PluginError::InitializationFailed(format!(
                "Validator plugin '{}' already registered",
                name
            )));
        }
        self.validators.insert(name, plugin);
        Ok(())
    }

    /// Registers a transformer plugin.
    ///
    /// # Errors
    ///
    /// Returns error if plugin with same name already exists.
    pub fn register_transformer(&mut self, plugin: Box<dyn TransformerPlugin>) -> PluginResult<()> {
        let name = plugin.metadata().name.clone();
        if self.transformers.contains_key(&name) {
            return Err(PluginError::InitializationFailed(format!(
                "Transformer plugin '{}' already registered",
                name
            )));
        }
        self.transformers.insert(name, plugin);
        Ok(())
    }

    /// Gets an analyzer plugin by name.
    #[must_use]
    pub fn get_analyzer(&self, name: &str) -> Option<&dyn AnalyzerPlugin> {
        self.analyzers.get(name).map(|p| p.as_ref())
    }

    /// Gets a validator plugin by name.
    #[must_use]
    pub fn get_validator(&self, name: &str) -> Option<&dyn ValidatorPlugin> {
        self.validators.get(name).map(|p| p.as_ref())
    }

    /// Gets a transformer plugin by name.
    #[must_use]
    pub fn get_transformer(&self, name: &str) -> Option<&dyn TransformerPlugin> {
        self.transformers.get(name).map(|p| p.as_ref())
    }

    /// Lists all registered analyzer plugins.
    #[must_use]
    pub fn list_analyzers(&self) -> Vec<&PluginMetadata> {
        self.analyzers.values().map(|p| p.metadata()).collect()
    }

    /// Lists all registered validator plugins.
    #[must_use]
    pub fn list_validators(&self) -> Vec<&PluginMetadata> {
        self.validators.values().map(|p| p.metadata()).collect()
    }

    /// Lists all registered transformer plugins.
    #[must_use]
    pub fn list_transformers(&self) -> Vec<&PluginMetadata> {
        self.transformers.values().map(|p| p.metadata()).collect()
    }

    /// Runs all analyzer plugins on a diff.
    ///
    /// # Errors
    ///
    /// Returns error if any plugin fails to execute.
    pub fn analyze_all(&self, diff: &StatuteDiff) -> PluginResult<Vec<AnalysisResult>> {
        let mut results = Vec::new();
        for plugin in self.analyzers.values() {
            results.push(plugin.analyze(diff)?);
        }
        Ok(results)
    }

    /// Runs all validator plugins on a diff.
    ///
    /// # Errors
    ///
    /// Returns error if any plugin fails to execute.
    pub fn validate_all(&self, diff: &StatuteDiff) -> PluginResult<Vec<ValidationResult>> {
        let mut results = Vec::new();
        for plugin in self.validators.values() {
            results.push(plugin.validate(diff)?);
        }
        Ok(results)
    }

    /// Runs a specific analyzer plugin.
    ///
    /// # Errors
    ///
    /// Returns error if plugin not found or execution fails.
    pub fn run_analyzer(&self, name: &str, diff: &StatuteDiff) -> PluginResult<AnalysisResult> {
        let plugin = self
            .get_analyzer(name)
            .ok_or_else(|| PluginError::NotFound(name.to_string()))?;
        plugin.analyze(diff)
    }

    /// Runs a specific validator plugin.
    ///
    /// # Errors
    ///
    /// Returns error if plugin not found or execution fails.
    pub fn run_validator(&self, name: &str, diff: &StatuteDiff) -> PluginResult<ValidationResult> {
        let plugin = self
            .get_validator(name)
            .ok_or_else(|| PluginError::NotFound(name.to_string()))?;
        plugin.validate(diff)
    }

    /// Runs a specific transformer plugin.
    ///
    /// # Errors
    ///
    /// Returns error if plugin not found or execution fails.
    pub fn run_transformer(&self, name: &str, diff: StatuteDiff) -> PluginResult<StatuteDiff> {
        let plugin = self
            .get_transformer(name)
            .ok_or_else(|| PluginError::NotFound(name.to_string()))?;
        plugin.transform(diff)
    }

    /// Checks if all dependencies for plugins are satisfied.
    ///
    /// # Errors
    ///
    /// Returns error if any dependencies are missing.
    pub fn check_dependencies(&self) -> PluginResult<()> {
        // Check analyzer dependencies
        for plugin in self.analyzers.values() {
            for dep in &plugin.metadata().dependencies {
                if !self.analyzers.contains_key(dep)
                    && !self.validators.contains_key(dep)
                    && !self.transformers.contains_key(dep)
                {
                    return Err(PluginError::MissingDependency(format!(
                        "Plugin '{}' requires '{}'",
                        plugin.metadata().name,
                        dep
                    )));
                }
            }
        }

        // Check validator dependencies
        for plugin in self.validators.values() {
            for dep in &plugin.metadata().dependencies {
                if !self.analyzers.contains_key(dep)
                    && !self.validators.contains_key(dep)
                    && !self.transformers.contains_key(dep)
                {
                    return Err(PluginError::MissingDependency(format!(
                        "Plugin '{}' requires '{}'",
                        plugin.metadata().name,
                        dep
                    )));
                }
            }
        }

        // Check transformer dependencies
        for plugin in self.transformers.values() {
            for dep in &plugin.metadata().dependencies {
                if !self.analyzers.contains_key(dep)
                    && !self.validators.contains_key(dep)
                    && !self.transformers.contains_key(dep)
                {
                    return Err(PluginError::MissingDependency(format!(
                        "Plugin '{}' requires '{}'",
                        plugin.metadata().name,
                        dep
                    )));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Severity, diff};
    use legalis_core::{Effect, EffectType, Statute};

    // Example analyzer plugin
    struct ExampleAnalyzer {
        metadata: PluginMetadata,
    }

    impl ExampleAnalyzer {
        fn new() -> Self {
            Self {
                metadata: PluginMetadata::new(
                    "example-analyzer",
                    "1.0.0",
                    "Test Author",
                    "Example analyzer plugin",
                ),
            }
        }
    }

    impl AnalyzerPlugin for ExampleAnalyzer {
        fn metadata(&self) -> &PluginMetadata {
            &self.metadata
        }

        fn analyze(&self, diff: &StatuteDiff) -> PluginResult<AnalysisResult> {
            let findings = if diff.changes.is_empty() {
                vec![]
            } else {
                vec![Finding {
                    severity: FindingSeverity::Info,
                    category: "change-detection".to_string(),
                    message: format!("Detected {} changes", diff.changes.len()),
                    location: None,
                    suggestion: Some("Review changes carefully".to_string()),
                }]
            };

            Ok(AnalysisResult {
                plugin_name: self.metadata.name.clone(),
                findings,
                confidence: 1.0,
                metadata: HashMap::new(),
            })
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Example validator plugin
    struct ExampleValidator {
        metadata: PluginMetadata,
    }

    impl ExampleValidator {
        fn new() -> Self {
            Self {
                metadata: PluginMetadata::new(
                    "example-validator",
                    "1.0.0",
                    "Test Author",
                    "Example validator plugin",
                ),
            }
        }
    }

    impl ValidatorPlugin for ExampleValidator {
        fn metadata(&self) -> &PluginMetadata {
            &self.metadata
        }

        fn validate(&self, diff: &StatuteDiff) -> PluginResult<ValidationResult> {
            let passed = diff.impact.severity != Severity::Breaking;
            Ok(ValidationResult {
                passed,
                score: if passed { 1.0 } else { 0.0 },
                messages: vec!["Validation complete".to_string()],
                failed_rules: if passed {
                    vec![]
                } else {
                    vec!["no-breaking-changes".to_string()]
                },
            })
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn test_plugin_metadata_creation() {
        let metadata = PluginMetadata::new("test", "1.0.0", "Author", "Description")
            .with_dependency("dep1")
            .with_dependency("dep2");

        assert_eq!(metadata.name, "test");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.dependencies.len(), 2);
    }

    #[test]
    fn test_analyzer_plugin() {
        let statute1 = Statute::new("test", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Grant, "Benefit"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let plugin = ExampleAnalyzer::new();
        let result = plugin.analyze(&diff_result).unwrap();

        assert_eq!(result.plugin_name, "example-analyzer");
        assert!(!result.findings.is_empty());
        assert_eq!(result.confidence, 1.0);
    }

    #[test]
    fn test_validator_plugin() {
        let statute1 = Statute::new("test", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Grant, "Benefit"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let plugin = ExampleValidator::new();
        let result = plugin.validate(&diff_result).unwrap();

        assert!(result.passed);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn test_plugin_registry() {
        let mut registry = PluginRegistry::new();

        // Register plugins
        registry
            .register_analyzer(Box::new(ExampleAnalyzer::new()))
            .unwrap();
        registry
            .register_validator(Box::new(ExampleValidator::new()))
            .unwrap();

        // Check plugins are registered
        assert!(registry.get_analyzer("example-analyzer").is_some());
        assert!(registry.get_validator("example-validator").is_some());

        // List plugins
        assert_eq!(registry.list_analyzers().len(), 1);
        assert_eq!(registry.list_validators().len(), 1);
    }

    #[test]
    fn test_run_specific_plugin() {
        let mut registry = PluginRegistry::new();
        registry
            .register_analyzer(Box::new(ExampleAnalyzer::new()))
            .unwrap();

        let statute1 = Statute::new("test", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Grant, "Benefit"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let result = registry
            .run_analyzer("example-analyzer", &diff_result)
            .unwrap();
        assert_eq!(result.plugin_name, "example-analyzer");
    }

    #[test]
    fn test_plugin_not_found() {
        let registry = PluginRegistry::new();
        let statute1 = Statute::new("test", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Grant, "Benefit"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let result = registry.run_analyzer("nonexistent", &diff_result);
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_registration() {
        let mut registry = PluginRegistry::new();
        registry
            .register_analyzer(Box::new(ExampleAnalyzer::new()))
            .unwrap();

        let result = registry.register_analyzer(Box::new(ExampleAnalyzer::new()));
        assert!(result.is_err());
    }

    #[test]
    fn test_dependency_checking() {
        let mut registry = PluginRegistry::new();

        let mut analyzer = ExampleAnalyzer::new();
        analyzer
            .metadata
            .dependencies
            .push("nonexistent".to_string());

        registry.register_analyzer(Box::new(analyzer)).unwrap();

        let result = registry.check_dependencies();
        assert!(result.is_err());
    }
}
