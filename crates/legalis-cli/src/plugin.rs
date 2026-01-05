//! Plugin system for extending legalis-cli functionality.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Plugin manifest describing a plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Plugin name
    pub name: String,

    /// Plugin version
    pub version: String,

    /// Plugin description
    pub description: String,

    /// Plugin author
    pub author: String,

    /// Minimum legalis version required
    pub min_legalis_version: Option<String>,

    /// Plugin entry point (script or executable path)
    pub entry_point: String,

    /// Plugin type
    pub plugin_type: PluginType,

    /// Commands provided by this plugin
    #[serde(default)]
    pub commands: Vec<String>,

    /// Hooks this plugin subscribes to
    #[serde(default)]
    pub hooks: Vec<String>,
}

/// Type of plugin.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PluginType {
    /// Command plugin (adds new commands)
    Command,

    /// Hook plugin (extends existing commands)
    Hook,

    /// Output formatter plugin
    Formatter,

    /// Linter plugin
    Linter,

    /// Generic extension plugin
    Extension,
}

/// Plugin hook points where plugins can intercept and extend functionality.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginHook {
    /// Before parsing a statute
    PreParse,

    /// After parsing a statute
    PostParse,

    /// Before verification
    PreVerify,

    /// After verification
    PostVerify,

    /// Before export
    PreExport,

    /// After export
    PostExport,

    /// Custom hook
    Custom(String),
}

impl PluginHook {
    /// Convert hook name string to enum.
    pub fn from_hook_name(s: &str) -> Self {
        match s {
            "pre-parse" => Self::PreParse,
            "post-parse" => Self::PostParse,
            "pre-verify" => Self::PreVerify,
            "post-verify" => Self::PostVerify,
            "pre-export" => Self::PreExport,
            "post-export" => Self::PostExport,
            custom => Self::Custom(custom.to_string()),
        }
    }

    /// Convert hook enum to string.
    pub fn to_str(&self) -> &str {
        match self {
            Self::PreParse => "pre-parse",
            Self::PostParse => "post-parse",
            Self::PreVerify => "pre-verify",
            Self::PostVerify => "post-verify",
            Self::PreExport => "pre-export",
            Self::PostExport => "post-export",
            Self::Custom(s) => s,
        }
    }
}

/// Plugin state configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PluginState {
    enabled_plugins: Vec<String>,
}

impl Default for PluginState {
    fn default() -> Self {
        Self {
            enabled_plugins: Vec::new(),
        }
    }
}

/// Plugin manager for discovering and loading plugins.
pub struct PluginManager {
    plugin_dir: PathBuf,
    plugins: HashMap<String, PluginManifest>,
    state: PluginState,
    state_file: PathBuf,
}

impl PluginManager {
    /// Create a new plugin manager.
    pub fn new() -> Result<Self> {
        let plugin_dir = Self::plugin_directory()?;

        if !plugin_dir.exists() {
            fs::create_dir_all(&plugin_dir).with_context(|| {
                format!(
                    "Failed to create plugin directory: {}",
                    plugin_dir.display()
                )
            })?;
        }

        let state_file = plugin_dir.join("state.toml");
        let state = if state_file.exists() {
            let content = fs::read_to_string(&state_file)?;
            toml::from_str(&content).unwrap_or_default()
        } else {
            PluginState::default()
        };

        Ok(Self {
            plugin_dir,
            plugins: HashMap::new(),
            state,
            state_file,
        })
    }

    /// Get the plugin directory path.
    pub fn plugin_directory() -> Result<PathBuf> {
        let plugin_dir = dirs::data_dir()
            .context("Failed to determine data directory")?
            .join("legalis")
            .join("plugins");
        Ok(plugin_dir)
    }

    /// Discover and load all plugins from the plugin directory.
    pub fn discover_plugins(&mut self) -> Result<usize> {
        let mut count = 0;

        if !self.plugin_dir.exists() {
            return Ok(0);
        }

        for entry in fs::read_dir(&self.plugin_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let manifest_path = path.join("plugin.toml");
                if manifest_path.exists() {
                    match self.load_plugin(&manifest_path) {
                        Ok(manifest) => {
                            self.plugins.insert(manifest.name.clone(), manifest);
                            count += 1;
                        }
                        Err(e) => {
                            eprintln!(
                                "Warning: Failed to load plugin from {}: {}",
                                path.display(),
                                e
                            );
                        }
                    }
                }
            }
        }

        Ok(count)
    }

    /// Load a plugin from its manifest file.
    fn load_plugin(&self, manifest_path: &Path) -> Result<PluginManifest> {
        let content = fs::read_to_string(manifest_path).with_context(|| {
            format!(
                "Failed to read plugin manifest: {}",
                manifest_path.display()
            )
        })?;

        let manifest: PluginManifest = toml::from_str(&content).with_context(|| {
            format!(
                "Failed to parse plugin manifest: {}",
                manifest_path.display()
            )
        })?;

        Ok(manifest)
    }

    /// Get a plugin by name.
    pub fn get_plugin(&self, name: &str) -> Option<&PluginManifest> {
        self.plugins.get(name)
    }

    /// List all loaded plugins.
    pub fn list_plugins(&self) -> Vec<&PluginManifest> {
        self.plugins.values().collect()
    }

    /// Get plugins that subscribe to a specific hook.
    pub fn get_hook_plugins(&self, hook: &PluginHook) -> Vec<&PluginManifest> {
        let hook_str = hook.to_str();

        self.plugins
            .values()
            .filter(|plugin| plugin.hooks.iter().any(|h| h == hook_str))
            .collect()
    }

    /// Get plugins that provide commands.
    pub fn get_command_plugins(&self) -> Vec<&PluginManifest> {
        self.plugins
            .values()
            .filter(|plugin| {
                plugin.plugin_type == PluginType::Command && !plugin.commands.is_empty()
            })
            .collect()
    }

    /// Install a plugin from a directory.
    pub fn install_plugin(&mut self, source_dir: &Path, force: bool) -> Result<()> {
        let manifest_path = source_dir.join("plugin.toml");

        if !manifest_path.exists() {
            anyhow::bail!("No plugin.toml found in {}", source_dir.display());
        }

        let manifest = self.load_plugin(&manifest_path)?;

        let target_dir = self.plugin_dir.join(&manifest.name);

        if target_dir.exists() {
            if !force {
                anyhow::bail!(
                    "Plugin {} is already installed. Use --force to reinstall",
                    manifest.name
                );
            }
            // Remove existing plugin if force is true
            fs::remove_dir_all(&target_dir)?;
            self.plugins.remove(&manifest.name);
        }

        // Copy plugin directory to plugins folder
        copy_dir_all(source_dir, &target_dir)?;

        let plugin_name = manifest.name.clone();
        self.plugins.insert(plugin_name.clone(), manifest);

        // Auto-enable newly installed plugin
        if !self.is_enabled(&plugin_name) {
            self.state.enabled_plugins.push(plugin_name);
            self.save_state()?;
        }

        Ok(())
    }

    /// Uninstall a plugin by name.
    pub fn uninstall_plugin(&mut self, name: &str) -> Result<()> {
        if !self.plugins.contains_key(name) {
            anyhow::bail!("Plugin {} is not installed", name);
        }

        let plugin_dir = self.plugin_dir.join(name);

        if plugin_dir.exists() {
            fs::remove_dir_all(&plugin_dir).with_context(|| {
                format!(
                    "Failed to remove plugin directory: {}",
                    plugin_dir.display()
                )
            })?;
        }

        self.plugins.remove(name);

        // Remove from enabled list
        self.state.enabled_plugins.retain(|n| n != name);
        self.save_state()?;

        Ok(())
    }

    /// Get plugin count.
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Check if a plugin is enabled.
    pub fn is_enabled(&self, name: &str) -> bool {
        self.state.enabled_plugins.contains(&name.to_string())
    }

    /// Enable a plugin.
    pub fn enable_plugin(&mut self, name: &str) -> Result<()> {
        if !self.plugins.contains_key(name) {
            anyhow::bail!("Plugin {} is not installed", name);
        }

        if !self.is_enabled(name) {
            self.state.enabled_plugins.push(name.to_string());
            self.save_state()?;
        }

        Ok(())
    }

    /// Disable a plugin.
    pub fn disable_plugin(&mut self, name: &str) -> Result<()> {
        if !self.plugins.contains_key(name) {
            anyhow::bail!("Plugin {} is not installed", name);
        }

        self.state.enabled_plugins.retain(|n| n != name);
        self.save_state()?;

        Ok(())
    }

    /// Save plugin state to disk.
    fn save_state(&self) -> Result<()> {
        let content = toml::to_string_pretty(&self.state)?;
        fs::write(&self.state_file, content).with_context(|| {
            format!("Failed to save plugin state: {}", self.state_file.display())
        })?;
        Ok(())
    }

    /// Get all enabled plugins.
    pub fn get_enabled_plugins(&self) -> Vec<&PluginManifest> {
        self.plugins
            .iter()
            .filter(|(name, _)| self.is_enabled(name))
            .map(|(_, manifest)| manifest)
            .collect()
    }

    /// Execute a plugin hook with given data.
    pub fn execute_hook(&self, hook: &PluginHook, data: &str) -> Result<Vec<String>> {
        let plugins = self.get_hook_plugins(hook);
        let mut results = Vec::new();

        for plugin in plugins {
            if !self.is_enabled(&plugin.name) {
                continue;
            }

            let result = self.execute_plugin(plugin, data)?;
            results.push(result);
        }

        Ok(results)
    }

    /// Execute a plugin with input data.
    fn execute_plugin(&self, plugin: &PluginManifest, input: &str) -> Result<String> {
        use std::io::Write;
        use std::process::{Command, Stdio};

        let plugin_path = self.plugin_dir.join(&plugin.name).join(&plugin.entry_point);

        if !plugin_path.exists() {
            anyhow::bail!("Plugin entry point not found: {}", plugin_path.display());
        }

        // Execute plugin in a sandboxed subprocess
        let mut child = Command::new(&plugin_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to execute plugin: {}", plugin.name))?;

        // Write input to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(input.as_bytes())?;
        }

        // Wait for plugin to complete and collect output
        let output = child.wait_with_output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Plugin {} failed: {}", plugin.name, stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new().expect("Failed to create plugin manager")
    }
}

/// Copy a directory recursively.
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_hook_conversion() {
        assert_eq!(
            PluginHook::from_hook_name("pre-parse").to_str(),
            "pre-parse"
        );
        assert_eq!(
            PluginHook::from_hook_name("post-verify").to_str(),
            "post-verify"
        );

        let custom = PluginHook::from_hook_name("custom-hook");
        assert_eq!(custom.to_str(), "custom-hook");
    }

    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_plugin_manifest_serialization() {
        let manifest = PluginManifest {
            name: "test-plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test Author".to_string(),
            min_legalis_version: Some("0.2.0".to_string()),
            entry_point: "./plugin.sh".to_string(),
            plugin_type: PluginType::Hook,
            commands: vec![],
            hooks: vec!["pre-verify".to_string(), "post-verify".to_string()],
        };

        let toml = toml::to_string(&manifest).unwrap();
        assert!(toml.contains("test-plugin"));
        assert!(toml.contains("pre-verify"));

        let deserialized: PluginManifest = toml::from_str(&toml).unwrap();
        assert_eq!(deserialized.name, "test-plugin");
        assert_eq!(deserialized.hooks.len(), 2);
    }
}
