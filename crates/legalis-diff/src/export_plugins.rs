//! Export plugin system for custom output formats.
//!
//! This module provides a plugin architecture for adding custom export formats
//! to the diff system.

use crate::{StatuteDiff, plugins::PluginMetadata};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;

/// Result type for export operations.
pub type ExportResult<T> = Result<T, ExportError>;

/// Errors that can occur during export.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ExportError {
    /// Export failed
    #[error("Export failed: {0}")]
    ExportFailed(String),

    /// Invalid configuration
    #[error("Invalid export configuration: {0}")]
    InvalidConfiguration(String),

    /// Format not supported
    #[error("Format not supported: {0}")]
    UnsupportedFormat(String),

    /// IO error during export
    #[error("IO error: {0}")]
    IoError(String),
}

/// Configuration for export operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Format-specific options
    pub options: HashMap<String, String>,
    /// Output encoding (e.g., "utf-8")
    pub encoding: Option<String>,
    /// Include metadata in output
    pub include_metadata: bool,
    /// Pretty-print output (if applicable)
    pub pretty_print: bool,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            options: HashMap::new(),
            encoding: Some("utf-8".to_string()),
            include_metadata: true,
            pretty_print: true,
        }
    }
}

impl ExportConfig {
    /// Creates a new export configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets an option.
    #[must_use]
    pub fn with_option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.insert(key.into(), value.into());
        self
    }

    /// Sets the encoding.
    #[must_use]
    pub fn with_encoding(mut self, encoding: impl Into<String>) -> Self {
        self.encoding = Some(encoding.into());
        self
    }

    /// Sets whether to include metadata.
    #[must_use]
    pub fn with_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }

    /// Sets whether to pretty-print.
    #[must_use]
    pub fn with_pretty_print(mut self, pretty: bool) -> Self {
        self.pretty_print = pretty;
        self
    }
}

/// Trait for custom export format plugins.
pub trait ExportPlugin: Send + Sync {
    /// Returns plugin metadata.
    fn metadata(&self) -> &PluginMetadata;

    /// Returns the file extension for this format (e.g., "xml", "yaml").
    fn file_extension(&self) -> &str;

    /// Returns the MIME type for this format (e.g., "application/xml").
    fn mime_type(&self) -> &str;

    /// Initializes the plugin with configuration.
    fn initialize(&mut self, config: &ExportConfig) -> ExportResult<()> {
        let _ = config;
        Ok(())
    }

    /// Exports a diff to the custom format.
    fn export(&self, diff: &StatuteDiff, config: &ExportConfig) -> ExportResult<String>;

    /// Exports multiple diffs to the custom format.
    fn export_batch(&self, diffs: &[StatuteDiff], config: &ExportConfig) -> ExportResult<String> {
        // Default implementation: concatenate individual exports
        let mut result = String::new();
        for diff in diffs {
            result.push_str(&self.export(diff, config)?);
            result.push('\n');
        }
        Ok(result)
    }

    /// Returns the plugin as Any for downcasting.
    fn as_any(&self) -> &dyn Any;
}

/// Registry for export plugins.
pub struct ExportPluginRegistry {
    plugins: HashMap<String, Box<dyn ExportPlugin>>,
}

impl Default for ExportPluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ExportPluginRegistry {
    /// Creates a new export plugin registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    /// Registers an export plugin.
    ///
    /// # Errors
    ///
    /// Returns error if plugin with same name already exists.
    pub fn register(&mut self, plugin: Box<dyn ExportPlugin>) -> ExportResult<()> {
        let name = plugin.metadata().name.clone();
        if self.plugins.contains_key(&name) {
            return Err(ExportError::ExportFailed(format!(
                "Export plugin '{}' already registered",
                name
            )));
        }
        self.plugins.insert(name, plugin);
        Ok(())
    }

    /// Gets an export plugin by name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&dyn ExportPlugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }

    /// Lists all registered export plugins.
    #[must_use]
    pub fn list(&self) -> Vec<&PluginMetadata> {
        self.plugins.values().map(|p| p.metadata()).collect()
    }

    /// Lists all supported formats (plugin names).
    #[must_use]
    pub fn supported_formats(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    /// Exports a diff using a specific plugin.
    ///
    /// # Errors
    ///
    /// Returns error if plugin not found or export fails.
    pub fn export(
        &self,
        format: &str,
        diff: &StatuteDiff,
        config: &ExportConfig,
    ) -> ExportResult<String> {
        let plugin = self
            .get(format)
            .ok_or_else(|| ExportError::UnsupportedFormat(format.to_string()))?;
        plugin.export(diff, config)
    }

    /// Exports multiple diffs using a specific plugin.
    ///
    /// # Errors
    ///
    /// Returns error if plugin not found or export fails.
    pub fn export_batch(
        &self,
        format: &str,
        diffs: &[StatuteDiff],
        config: &ExportConfig,
    ) -> ExportResult<String> {
        let plugin = self
            .get(format)
            .ok_or_else(|| ExportError::UnsupportedFormat(format.to_string()))?;
        plugin.export_batch(diffs, config)
    }
}

// Built-in export plugins

/// XML export plugin.
pub struct XmlExportPlugin {
    metadata: PluginMetadata,
}

impl Default for XmlExportPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl XmlExportPlugin {
    /// Creates a new XML export plugin.
    #[must_use]
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata::new("xml", "1.0.0", "Legalis Team", "XML export format"),
        }
    }
}

impl ExportPlugin for XmlExportPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn file_extension(&self) -> &str {
        "xml"
    }

    fn mime_type(&self) -> &str {
        "application/xml"
    }

    fn export(&self, diff: &StatuteDiff, config: &ExportConfig) -> ExportResult<String> {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<statute-diff>\n");

        if config.include_metadata {
            xml.push_str("  <metadata>\n");
            xml.push_str(&format!(
                "    <statute-id>{}</statute-id>\n",
                diff.statute_id
            ));
            if let Some(ver) = &diff.version_info {
                if let Some(old_ver) = ver.old_version {
                    xml.push_str(&format!("    <old-version>{}</old-version>\n", old_ver));
                }
                if let Some(new_ver) = ver.new_version {
                    xml.push_str(&format!("    <new-version>{}</new-version>\n", new_ver));
                }
            }
            xml.push_str("  </metadata>\n");
        }

        xml.push_str("  <changes>\n");
        for change in &diff.changes {
            xml.push_str(&format!("    <change type=\"{:?}\">\n", change.change_type));
            xml.push_str(&format!("      <target>{:?}</target>\n", change.target));
            if let Some(old) = &change.old_value {
                xml.push_str(&format!(
                    "      <old-value>{}</old-value>\n",
                    escape_xml(old)
                ));
            }
            if let Some(new) = &change.new_value {
                xml.push_str(&format!(
                    "      <new-value>{}</new-value>\n",
                    escape_xml(new)
                ));
            }
            xml.push_str("    </change>\n");
        }
        xml.push_str("  </changes>\n");

        xml.push_str("  <impact>\n");
        xml.push_str(&format!(
            "    <severity>{:?}</severity>\n",
            diff.impact.severity
        ));
        xml.push_str(&format!(
            "    <affects-eligibility>{}</affects-eligibility>\n",
            diff.impact.affects_eligibility
        ));
        xml.push_str(&format!(
            "    <affects-outcome>{}</affects-outcome>\n",
            diff.impact.affects_outcome
        ));
        xml.push_str("  </impact>\n");

        xml.push_str("</statute-diff>");

        Ok(xml)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// YAML export plugin.
pub struct YamlExportPlugin {
    metadata: PluginMetadata,
}

impl Default for YamlExportPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl YamlExportPlugin {
    /// Creates a new YAML export plugin.
    #[must_use]
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata::new("yaml", "1.0.0", "Legalis Team", "YAML export format"),
        }
    }
}

impl ExportPlugin for YamlExportPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn file_extension(&self) -> &str {
        "yaml"
    }

    fn mime_type(&self) -> &str {
        "application/x-yaml"
    }

    fn export(&self, diff: &StatuteDiff, config: &ExportConfig) -> ExportResult<String> {
        let mut yaml = String::new();
        yaml.push_str("---\n");
        yaml.push_str("statute_diff:\n");

        if config.include_metadata {
            yaml.push_str("  metadata:\n");
            yaml.push_str(&format!("    statute_id: \"{}\"\n", diff.statute_id));
            if let Some(ver) = &diff.version_info {
                if let Some(old_ver) = ver.old_version {
                    yaml.push_str(&format!("    old_version: {}\n", old_ver));
                }
                if let Some(new_ver) = ver.new_version {
                    yaml.push_str(&format!("    new_version: {}\n", new_ver));
                }
            }
        }

        yaml.push_str("  changes:\n");
        for change in &diff.changes {
            yaml.push_str(&format!("    - type: {:?}\n", change.change_type));
            yaml.push_str(&format!("      target: {:?}\n", change.target));
            if let Some(old) = &change.old_value {
                yaml.push_str(&format!("      old_value: \"{}\"\n", escape_yaml(old)));
            }
            if let Some(new) = &change.new_value {
                yaml.push_str(&format!("      new_value: \"{}\"\n", escape_yaml(new)));
            }
        }

        yaml.push_str("  impact:\n");
        yaml.push_str(&format!("    severity: {:?}\n", diff.impact.severity));
        yaml.push_str(&format!(
            "    affects_eligibility: {}\n",
            diff.impact.affects_eligibility
        ));
        yaml.push_str(&format!(
            "    affects_outcome: {}\n",
            diff.impact.affects_outcome
        ));

        Ok(yaml)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Escapes XML special characters.
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Escapes YAML special characters.
fn escape_yaml(s: &str) -> String {
    s.replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    #[test]
    fn test_export_config() {
        let config = ExportConfig::new()
            .with_option("indent", "2")
            .with_encoding("utf-8")
            .with_metadata(true)
            .with_pretty_print(false);

        assert_eq!(config.options.get("indent"), Some(&"2".to_string()));
        assert_eq!(config.encoding, Some("utf-8".to_string()));
        assert!(config.include_metadata);
        assert!(!config.pretty_print);
    }

    #[test]
    fn test_xml_export_plugin() {
        let statute1 = Statute::new("test", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Grant, "Benefit"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let plugin = XmlExportPlugin::new();
        let config = ExportConfig::new();
        let xml = plugin.export(&diff_result, &config).unwrap();

        assert!(xml.contains("<?xml"));
        assert!(xml.contains("<statute-diff>"));
        assert!(xml.contains("<metadata>"));
        assert!(xml.contains("<changes>"));
        assert!(xml.contains("<impact>"));
    }

    #[test]
    fn test_yaml_export_plugin() {
        let statute1 = Statute::new("test", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Grant, "Benefit"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let plugin = YamlExportPlugin::new();
        let config = ExportConfig::new();
        let yaml = plugin.export(&diff_result, &config).unwrap();

        assert!(yaml.contains("---"));
        assert!(yaml.contains("statute_diff:"));
        assert!(yaml.contains("metadata:"));
        assert!(yaml.contains("changes:"));
        assert!(yaml.contains("impact:"));
    }

    #[test]
    fn test_export_plugin_registry() {
        let mut registry = ExportPluginRegistry::new();

        registry.register(Box::new(XmlExportPlugin::new())).unwrap();
        registry
            .register(Box::new(YamlExportPlugin::new()))
            .unwrap();

        assert_eq!(registry.supported_formats().len(), 2);
        assert!(registry.get("xml").is_some());
        assert!(registry.get("yaml").is_some());
    }

    #[test]
    fn test_export_using_registry() {
        let mut registry = ExportPluginRegistry::new();
        registry.register(Box::new(XmlExportPlugin::new())).unwrap();

        let statute1 = Statute::new("test", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Grant, "Benefit"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let config = ExportConfig::new();
        let xml = registry.export("xml", &diff_result, &config).unwrap();

        assert!(xml.contains("<statute-diff>"));
    }

    #[test]
    fn test_unsupported_format() {
        let registry = ExportPluginRegistry::new();
        let statute1 = Statute::new("test", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Grant, "Benefit"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let config = ExportConfig::new();
        let result = registry.export("nonexistent", &diff_result, &config);

        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_registration() {
        let mut registry = ExportPluginRegistry::new();
        registry.register(Box::new(XmlExportPlugin::new())).unwrap();

        let result = registry.register(Box::new(XmlExportPlugin::new()));
        assert!(result.is_err());
    }

    #[test]
    fn test_escape_xml() {
        let input = "<tag>content & \"quotes\"</tag>";
        let escaped = escape_xml(input);
        assert!(escaped.contains("&lt;"));
        assert!(escaped.contains("&gt;"));
        assert!(escaped.contains("&amp;"));
        assert!(escaped.contains("&quot;"));
    }

    #[test]
    fn test_escape_yaml() {
        let input = "line1\nline2\"quoted\"";
        let escaped = escape_yaml(input);
        assert!(escaped.contains("\\n"));
        assert!(escaped.contains("\\\""));
    }
}
