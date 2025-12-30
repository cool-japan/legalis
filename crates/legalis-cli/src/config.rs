//! Configuration file support for legalis-cli.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Global lazy-loaded configuration instance.
static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

/// Configuration for legalis-cli.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Path to parent configuration file for inheritance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// Default jurisdiction for operations
    #[serde(default)]
    pub jurisdiction: Option<String>,

    /// Verification settings
    #[serde(default)]
    pub verification: VerificationConfig,

    /// Output settings
    #[serde(default)]
    pub output: OutputConfig,

    /// Linting settings
    #[serde(default)]
    pub lint: LintConfig,

    /// Command aliases (alias -> full command)
    #[serde(default)]
    pub aliases: HashMap<String, String>,
}

/// Verification configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationConfig {
    /// Fail on warnings
    #[serde(default)]
    pub strict: bool,

    /// Enable constitutional checks
    #[serde(default = "default_true")]
    pub constitutional_checks: bool,
}

impl Default for VerificationConfig {
    fn default() -> Self {
        Self {
            strict: false,
            constitutional_checks: true,
        }
    }
}

/// Output configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Default output format
    #[serde(default = "default_format")]
    pub format: String,

    /// Output directory
    #[serde(default = "default_output_dir")]
    pub directory: String,

    /// Enable colored output
    #[serde(default = "default_true")]
    pub colored: bool,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: default_format(),
            directory: default_output_dir(),
            colored: true,
        }
    }
}

/// Linting configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LintConfig {
    /// Enable specific lint rules
    #[serde(default)]
    pub rules: Vec<String>,

    /// Disable specific lint rules
    #[serde(default)]
    pub disabled_rules: Vec<String>,

    /// Fail on warnings
    #[serde(default)]
    pub strict: bool,
}

fn default_true() -> bool {
    true
}

fn default_format() -> String {
    "json".to_string()
}

fn default_output_dir() -> String {
    "./output".to_string()
}

impl Config {
    /// Load configuration from a file.
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let mut config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        // Handle inheritance
        if let Some(parent_path) = &config.parent.clone() {
            let parent_path = if Path::new(parent_path).is_relative() {
                path.parent()
                    .unwrap_or_else(|| Path::new("."))
                    .join(parent_path)
            } else {
                PathBuf::from(parent_path)
            };

            let parent_config = Self::from_file(&parent_path).with_context(|| {
                format!("Failed to load parent config: {}", parent_path.display())
            })?;

            // Merge parent config with current config (current takes precedence)
            config = config.merge_with_parent(parent_config);
        }

        Ok(config)
    }

    /// Merge this configuration with a parent configuration.
    /// Current configuration takes precedence over parent.
    fn merge_with_parent(self, parent: Config) -> Self {
        Config {
            parent: self.parent, // Keep current parent reference
            jurisdiction: self.jurisdiction.or(parent.jurisdiction),
            verification: VerificationConfig {
                strict: self.verification.strict || parent.verification.strict,
                constitutional_checks: self.verification.constitutional_checks
                    && parent.verification.constitutional_checks,
            },
            output: OutputConfig {
                format: if self.output.format == default_format() {
                    parent.output.format
                } else {
                    self.output.format
                },
                directory: if self.output.directory == default_output_dir() {
                    parent.output.directory
                } else {
                    self.output.directory
                },
                colored: self.output.colored && parent.output.colored,
            },
            lint: LintConfig {
                rules: {
                    let mut rules = parent.lint.rules;
                    rules.extend(self.lint.rules);
                    rules
                },
                disabled_rules: {
                    let mut disabled = parent.lint.disabled_rules;
                    disabled.extend(self.lint.disabled_rules);
                    disabled
                },
                strict: self.lint.strict || parent.lint.strict,
            },
            aliases: {
                let mut aliases = parent.aliases;
                aliases.extend(self.aliases);
                aliases
            },
        }
    }

    /// Load configuration from the default locations with environment variable overrides.
    ///
    /// Priority order:
    /// 1. Environment variables (LEGALIS_*)
    /// 2. ./legalis.toml (project-level)
    /// 3. ~/.config/legalis/config.toml (user-level)
    /// 4. Default config
    pub fn load() -> Self {
        // Try project-level config
        let mut config = if let Ok(config) = Self::from_file(Path::new("legalis.toml")) {
            config
        } else if let Some(config_dir) = dirs::config_dir() {
            // Try user-level config
            let user_config = config_dir.join("legalis").join("config.toml");
            Self::from_file(&user_config).unwrap_or_default()
        } else {
            Self::default()
        };

        // Apply environment variable overrides
        config.apply_env_overrides();
        config
    }

    /// Apply environment variable overrides to the configuration.
    ///
    /// Supported environment variables:
    /// - LEGALIS_JURISDICTION: Default jurisdiction
    /// - LEGALIS_VERIFY_STRICT: Strict verification mode (true/false)
    /// - LEGALIS_OUTPUT_FORMAT: Default output format
    /// - LEGALIS_OUTPUT_COLORED: Enable colored output (true/false)
    pub fn apply_env_overrides(&mut self) {
        if let Ok(jur) = std::env::var("LEGALIS_JURISDICTION") {
            self.jurisdiction = Some(jur);
        }

        if let Ok(strict) = std::env::var("LEGALIS_VERIFY_STRICT") {
            if let Ok(value) = strict.parse::<bool>() {
                self.verification.strict = value;
            }
        }

        if let Ok(format) = std::env::var("LEGALIS_OUTPUT_FORMAT") {
            self.output.format = format;
        }

        if let Ok(colored) = std::env::var("LEGALIS_OUTPUT_COLORED") {
            if let Ok(value) = colored.parse::<bool>() {
                self.output.colored = value;
            }
        }
    }

    /// Save configuration to a file.
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Get the user config directory.
    pub fn user_config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("legalis"))
    }

    /// Initialize user-level config if it doesn't exist.
    pub fn init_user_config() -> Result<PathBuf> {
        let config_dir = Self::user_config_dir().context("Failed to determine config directory")?;

        fs::create_dir_all(&config_dir).with_context(|| {
            format!(
                "Failed to create config directory: {}",
                config_dir.display()
            )
        })?;

        let config_file = config_dir.join("config.toml");

        if !config_file.exists() {
            let default_config = Self::default();
            default_config.save(&config_file)?;
        }

        Ok(config_file)
    }

    /// Expand command aliases in arguments.
    /// Returns a new vector with aliases expanded.
    pub fn expand_aliases(&self, args: Vec<String>) -> Vec<String> {
        if args.len() < 2 {
            return args;
        }

        // Check if the first argument (command name) is an alias
        if let Some(expansion) = self.aliases.get(&args[1]) {
            let mut expanded = vec![args[0].clone()];
            // Parse the expansion and add it
            expanded.extend(expansion.split_whitespace().map(String::from));
            // Add the rest of the arguments
            expanded.extend(args[2..].iter().cloned());
            expanded
        } else {
            args
        }
    }

    /// Get the global configuration instance (lazy-loaded).
    /// This function loads the configuration only once and caches it.
    pub fn global() -> &'static Config {
        GLOBAL_CONFIG.get_or_init(|| Self::load())
    }

    /// Try to get the global configuration if it's already loaded.
    /// Returns None if the configuration hasn't been loaded yet.
    pub fn try_global() -> Option<&'static Config> {
        GLOBAL_CONFIG.get()
    }

    /// Force reload the global configuration.
    /// Note: This will only work if the configuration hasn't been loaded yet.
    /// Returns true if reload was successful, false if already loaded.
    #[allow(dead_code)]
    pub fn reload_global() -> bool {
        if GLOBAL_CONFIG.get().is_none() {
            let _ = GLOBAL_CONFIG.set(Self::load());
            true
        } else {
            false
        }
    }
}
