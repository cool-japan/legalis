//! Centralized, layered configuration management.
//!
//! The existing [`crate::config::Config`] handles project/user TOML files,
//! inheritance and alias expansion. This module adds an explicit, *centralized*
//! settings layer with deterministic precedence and provenance tracking, so an
//! organization can ship a central policy-config and still let users and the
//! environment override individual keys.
//!
//! Layers, from lowest to highest precedence:
//!
//! 1. **defaults** — compiled-in defaults,
//! 2. **central** — an organization-wide file (`LEGALIS_CENTRAL_CONFIG` or
//!    `<data_dir>/central.toml`),
//! 3. **file** — a user/project config file,
//! 4. **env** — `LEGALIS_*` environment variables,
//! 5. **flags** — explicit command-line overrides.
//!
//! Each resolved key records *which layer* set it, which powers `config show`
//! provenance output and validation diagnostics. Validation is strict: unknown
//! values for enumerated keys are reported as errors, not silently ignored.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

/// Environment variable pointing at the central (organization-wide) config.
pub const CENTRAL_CONFIG_ENV: &str = "LEGALIS_CENTRAL_CONFIG";

/// The configuration layer a value originated from (lower = weaker).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Layer {
    /// Compiled-in defaults.
    Default,
    /// Organization-wide central config.
    Central,
    /// User/project config file.
    File,
    /// Environment variables.
    Env,
    /// Command-line flags.
    Flags,
}

impl Layer {
    /// All layers in ascending precedence order.
    pub const ALL: [Layer; 5] = [
        Layer::Default,
        Layer::Central,
        Layer::File,
        Layer::Env,
        Layer::Flags,
    ];

    fn label(self) -> &'static str {
        match self {
            Layer::Default => "default",
            Layer::Central => "central",
            Layer::File => "file",
            Layer::Env => "env",
            Layer::Flags => "flags",
        }
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

/// A single resolved setting plus the layer that supplied it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedValue {
    /// The effective string value.
    pub value: String,
    /// The layer that set it.
    pub source: Layer,
}

/// A raw, partial settings map keyed by dotted setting name.
///
/// Used to represent each layer before merging. Serializable so central/file
/// layers can be read from TOML/JSON/YAML.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SettingsMap {
    /// Flat key/value settings (dotted keys, e.g. `output.format`).
    #[serde(default, flatten)]
    pub values: BTreeMap<String, String>,
}

impl SettingsMap {
    /// An empty settings map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a setting.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }

    /// Whether the map has no settings.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Loads a settings map from a file (format inferred from extension).
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read settings file: {}", path.display()))?;
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("toml")
            .to_ascii_lowercase();
        let map: SettingsMap = match ext.as_str() {
            "json" => serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse JSON settings: {}", path.display()))?,
            "yaml" | "yml" => serde_yaml::from_str(&content)
                .with_context(|| format!("Failed to parse YAML settings: {}", path.display()))?,
            _ => toml::from_str(&content)
                .with_context(|| format!("Failed to parse TOML settings: {}", path.display()))?,
        };
        Ok(map)
    }
}

/// Compiled-in default settings.
fn default_settings() -> SettingsMap {
    let mut map = SettingsMap::new();
    map.set("output.format", "text");
    map.set("output.colored", "true");
    map.set("output.theme", "default");
    map.set("verbosity", "normal");
    map.set("verification.strict", "false");
    map
}

/// Reads recognized `LEGALIS_*` env vars into a settings layer.
fn env_settings() -> SettingsMap {
    let mut map = SettingsMap::new();
    let mappings = [
        ("LEGALIS_OUTPUT_FORMAT", "output.format"),
        ("LEGALIS_OUTPUT_COLORED", "output.colored"),
        ("LEGALIS_THEME", "output.theme"),
        ("LEGALIS_VERBOSITY", "verbosity"),
        ("LEGALIS_VERIFY_STRICT", "verification.strict"),
        ("LEGALIS_JURISDICTION", "jurisdiction"),
    ];
    for (env_key, setting_key) in mappings {
        if let Ok(value) = std::env::var(env_key) {
            map.set(setting_key, value);
        }
    }
    map
}

/// The fully resolved, layered configuration with provenance.
#[derive(Debug, Clone)]
pub struct CentralConfig {
    resolved: BTreeMap<String, ResolvedValue>,
}

/// Builder that assembles the layered configuration in precedence order.
#[derive(Debug, Default)]
pub struct CentralConfigBuilder {
    central: SettingsMap,
    file: SettingsMap,
    flags: SettingsMap,
}

impl CentralConfigBuilder {
    /// A new builder seeded with empty overlays.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads the central layer from a file.
    pub fn with_central_file(mut self, path: &Path) -> Result<Self> {
        self.central = SettingsMap::from_file(path)?;
        Ok(self)
    }

    /// Sets the central layer directly.
    pub fn with_central(mut self, map: SettingsMap) -> Self {
        self.central = map;
        self
    }

    /// Loads the file layer from a path.
    pub fn with_file(mut self, path: &Path) -> Result<Self> {
        self.file = SettingsMap::from_file(path)?;
        Ok(self)
    }

    /// Sets the file layer directly.
    pub fn with_file_settings(mut self, map: SettingsMap) -> Self {
        self.file = map;
        self
    }

    /// Sets a flag override (highest precedence).
    pub fn with_flag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.flags.set(key, value);
        self
    }

    /// Builds the resolved configuration, applying layers in order and reading
    /// the environment layer between file and flags.
    pub fn build(self) -> CentralConfig {
        let mut resolved: BTreeMap<String, ResolvedValue> = BTreeMap::new();
        let layers = [
            (Layer::Default, default_settings()),
            (Layer::Central, self.central),
            (Layer::File, self.file),
            (Layer::Env, env_settings()),
            (Layer::Flags, self.flags),
        ];
        for (layer, map) in layers {
            for (key, value) in map.values {
                resolved.insert(
                    key,
                    ResolvedValue {
                        value,
                        source: layer,
                    },
                );
            }
        }
        CentralConfig { resolved }
    }
}

impl CentralConfig {
    /// Convenience: discover and resolve from the environment + central file.
    ///
    /// Looks up the central config from `LEGALIS_CENTRAL_CONFIG` or
    /// `<data_dir>/central.toml`, then applies env overrides.
    pub fn discover() -> Result<Self> {
        let mut builder = CentralConfigBuilder::new();
        if let Ok(env_path) = std::env::var(CENTRAL_CONFIG_ENV) {
            builder = builder.with_central_file(Path::new(&env_path))?;
        } else if let Ok(data_dir) = crate::paths::data_dir() {
            let central = data_dir.join("central.toml");
            if central.exists() {
                builder = builder.with_central_file(&central)?;
            }
        }
        Ok(builder.build())
    }

    /// Returns the resolved value for a key, if present.
    pub fn get(&self, key: &str) -> Option<&ResolvedValue> {
        self.resolved.get(key)
    }

    /// Returns the string value for a key, if present.
    pub fn value(&self, key: &str) -> Option<&str> {
        self.resolved.get(key).map(|r| r.value.as_str())
    }

    /// Returns the source layer for a key, if present.
    pub fn source(&self, key: &str) -> Option<Layer> {
        self.resolved.get(key).map(|r| r.source)
    }

    /// Returns a boolean value, parsing common truthy/falsy spellings.
    pub fn bool(&self, key: &str) -> Option<bool> {
        self.value(key).map(|v| {
            matches!(
                v.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
    }

    /// All resolved entries, sorted by key.
    pub fn entries(&self) -> impl Iterator<Item = (&String, &ResolvedValue)> {
        self.resolved.iter()
    }

    /// Validates the resolved configuration.
    ///
    /// Returns the list of validation errors (empty when valid). Enumerated keys
    /// are checked against their allowed value sets; booleans must parse.
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        let enums: [(&str, &[&str]); 3] = [
            (
                "output.format",
                &["text", "json", "yaml", "toml", "table", "csv", "html"],
            ),
            (
                "output.theme",
                &[
                    "default",
                    "dark",
                    "light",
                    "monokai",
                    "solarized",
                    "high-contrast",
                    "none",
                ],
            ),
            (
                "verbosity",
                &["silent", "quiet", "normal", "verbose", "debug", "trace"],
            ),
        ];
        for (key, allowed) in enums {
            if let Some(resolved) = self.resolved.get(key)
                && !allowed.contains(&resolved.value.to_ascii_lowercase().as_str())
            {
                errors.push(format!(
                    "invalid value '{}' for '{}' (from {}); expected one of: {}",
                    resolved.value,
                    key,
                    resolved.source,
                    allowed.join(", ")
                ));
            }
        }

        let bools = ["output.colored", "verification.strict"];
        for key in bools {
            if let Some(resolved) = self.resolved.get(key) {
                let v = resolved.value.to_ascii_lowercase();
                let ok = matches!(
                    v.as_str(),
                    "1" | "true" | "yes" | "on" | "0" | "false" | "no" | "off"
                );
                if !ok {
                    errors.push(format!(
                        "invalid boolean '{}' for '{}' (from {})",
                        resolved.value, key, resolved.source
                    ));
                }
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_file(ext: &str) -> PathBuf {
        std::env::temp_dir().join(format!("legalis-central-{}.{}", uuid::Uuid::new_v4(), ext))
    }

    fn clean_env<F: FnOnce()>(f: F) {
        let keys = [
            "LEGALIS_OUTPUT_FORMAT",
            "LEGALIS_OUTPUT_COLORED",
            "LEGALIS_THEME",
            "LEGALIS_VERBOSITY",
            "LEGALIS_VERIFY_STRICT",
            "LEGALIS_JURISDICTION",
        ];
        let saved: Vec<(String, Option<String>)> = keys
            .iter()
            .map(|k| ((*k).to_string(), std::env::var(k).ok()))
            .collect();
        for k in &keys {
            unsafe {
                std::env::remove_var(k);
            }
        }
        f();
        for (k, v) in saved {
            unsafe {
                match v {
                    Some(value) => std::env::set_var(&k, value),
                    None => std::env::remove_var(&k),
                }
            }
        }
    }

    #[test]
    fn test_defaults_present() {
        clean_env(|| {
            let cfg = CentralConfigBuilder::new().build();
            assert_eq!(cfg.value("output.format"), Some("text"));
            assert_eq!(cfg.source("output.format"), Some(Layer::Default));
            assert_eq!(cfg.bool("output.colored"), Some(true));
        });
    }

    #[test]
    fn test_precedence_central_over_default() {
        clean_env(|| {
            let mut central = SettingsMap::new();
            central.set("output.format", "json");
            let cfg = CentralConfigBuilder::new().with_central(central).build();
            assert_eq!(cfg.value("output.format"), Some("json"));
            assert_eq!(cfg.source("output.format"), Some(Layer::Central));
        });
    }

    #[test]
    fn test_precedence_file_over_central() {
        clean_env(|| {
            let mut central = SettingsMap::new();
            central.set("output.format", "json");
            let mut file = SettingsMap::new();
            file.set("output.format", "yaml");
            let cfg = CentralConfigBuilder::new()
                .with_central(central)
                .with_file_settings(file)
                .build();
            assert_eq!(cfg.value("output.format"), Some("yaml"));
            assert_eq!(cfg.source("output.format"), Some(Layer::File));
        });
    }

    #[test]
    fn test_precedence_env_over_file() {
        clean_env(|| {
            let mut file = SettingsMap::new();
            file.set("output.format", "yaml");
            unsafe {
                std::env::set_var("LEGALIS_OUTPUT_FORMAT", "csv");
            }
            let cfg = CentralConfigBuilder::new().with_file_settings(file).build();
            assert_eq!(cfg.value("output.format"), Some("csv"));
            assert_eq!(cfg.source("output.format"), Some(Layer::Env));
            unsafe {
                std::env::remove_var("LEGALIS_OUTPUT_FORMAT");
            }
        });
    }

    #[test]
    fn test_precedence_flags_win() {
        clean_env(|| {
            unsafe {
                std::env::set_var("LEGALIS_OUTPUT_FORMAT", "csv");
            }
            let cfg = CentralConfigBuilder::new()
                .with_flag("output.format", "html")
                .build();
            assert_eq!(cfg.value("output.format"), Some("html"));
            assert_eq!(cfg.source("output.format"), Some(Layer::Flags));
            unsafe {
                std::env::remove_var("LEGALIS_OUTPUT_FORMAT");
            }
        });
    }

    #[test]
    fn test_toml_central_file() {
        clean_env(|| {
            let path = temp_file("toml");
            std::fs::write(
                &path,
                "\"output.format\" = \"toml\"\n\"verbosity\" = \"debug\"\n",
            )
            .expect("write");
            let cfg = CentralConfigBuilder::new()
                .with_central_file(&path)
                .expect("load")
                .build();
            assert_eq!(cfg.value("output.format"), Some("toml"));
            assert_eq!(cfg.value("verbosity"), Some("debug"));
            let _ = std::fs::remove_file(&path);
        });
    }

    #[test]
    fn test_validation_passes_on_defaults() {
        clean_env(|| {
            let cfg = CentralConfigBuilder::new().build();
            assert!(cfg.validate().is_empty());
        });
    }

    #[test]
    fn test_validation_rejects_bad_enum() {
        clean_env(|| {
            let cfg = CentralConfigBuilder::new()
                .with_flag("output.format", "bogus")
                .build();
            let errors = cfg.validate();
            assert_eq!(errors.len(), 1);
            assert!(errors[0].contains("output.format"));
        });
    }

    #[test]
    fn test_validation_rejects_bad_bool() {
        clean_env(|| {
            let cfg = CentralConfigBuilder::new()
                .with_flag("output.colored", "maybe")
                .build();
            let errors = cfg.validate();
            assert_eq!(errors.len(), 1);
            assert!(errors[0].contains("boolean"));
        });
    }

    #[test]
    fn test_layer_ordering() {
        assert!(Layer::Default < Layer::Central);
        assert!(Layer::Central < Layer::File);
        assert!(Layer::File < Layer::Env);
        assert!(Layer::Env < Layer::Flags);
    }
}
