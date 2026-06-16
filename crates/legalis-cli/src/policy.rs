//! Enterprise policy enforcement.
//!
//! Loads a policy document (TOML, JSON, or YAML) that an administrator can ship
//! alongside a deployment and uses it to gate which commands may run and within
//! which numeric limits. A policy can:
//!
//! - allow-list commands (deny everything not listed), or
//! - deny-list specific commands (allow everything else),
//! - require [compliance mode](crate::compliance) to be active,
//! - mandate audit logging,
//! - cap numeric resources (population size, batch workers, iterations, …).
//!
//! Policies are discovered from (first match wins):
//! 1. an explicit path,
//! 2. `LEGALIS_POLICY_FILE`,
//! 3. `./legalis-policy.toml`,
//! 4. `<data_dir>/policy.toml`.
//!
//! When no policy is found the [`Policy::permissive`] default applies and
//! nothing is restricted (fully backward compatible).

use crate::paths;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Environment variable pointing at a policy file.
pub const POLICY_FILE_ENV: &str = "LEGALIS_POLICY_FILE";

/// Numeric resource limits enforced by a policy.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyLimits {
    /// Maximum simulation population size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_population: Option<usize>,
    /// Maximum number of parallel batch workers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_workers: Option<usize>,
    /// Maximum benchmark/profile iterations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_iterations: Option<usize>,
    /// Maximum number of input files accepted by a single command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_input_files: Option<usize>,
}

/// A named numeric limit kind, used when validating a value against a [`Policy`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitKind {
    /// Simulation population size.
    Population,
    /// Parallel worker count.
    Workers,
    /// Iteration count.
    Iterations,
    /// Input file count.
    InputFiles,
}

impl LimitKind {
    fn name(self) -> &'static str {
        match self {
            LimitKind::Population => "population",
            LimitKind::Workers => "workers",
            LimitKind::Iterations => "iterations",
            LimitKind::InputFiles => "input files",
        }
    }
}

/// An enterprise policy document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Policy {
    /// Human-readable policy name.
    #[serde(default = "default_policy_name")]
    pub name: String,

    /// If non-empty, *only* these commands are permitted (allow-list mode).
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub allowed_commands: BTreeSet<String>,

    /// Commands that are explicitly forbidden (deny-list mode).
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub denied_commands: BTreeSet<String>,

    /// Whether compliance mode must be active for any command to run.
    #[serde(default)]
    pub require_compliance: bool,

    /// Whether audit logging is mandatory (handlers must record operations).
    #[serde(default)]
    pub require_audit_log: bool,

    /// Numeric resource limits.
    #[serde(default)]
    pub limits: PolicyLimits,
}

fn default_policy_name() -> String {
    "default".to_string()
}

impl Default for Policy {
    fn default() -> Self {
        Self::permissive()
    }
}

/// The result of evaluating a command against a policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    /// The command is permitted.
    Allowed,
    /// The command is denied, with a human-readable reason.
    Denied(String),
}

impl PolicyDecision {
    /// Whether the decision permits the command.
    pub fn is_allowed(&self) -> bool {
        matches!(self, PolicyDecision::Allowed)
    }

    /// Returns the denial reason, if denied.
    pub fn reason(&self) -> Option<&str> {
        match self {
            PolicyDecision::Allowed => None,
            PolicyDecision::Denied(reason) => Some(reason),
        }
    }
}

impl Policy {
    /// A fully permissive policy (the implicit default; restricts nothing).
    pub fn permissive() -> Self {
        Self {
            name: default_policy_name(),
            allowed_commands: BTreeSet::new(),
            denied_commands: BTreeSet::new(),
            require_compliance: false,
            require_audit_log: false,
            limits: PolicyLimits::default(),
        }
    }

    /// Loads a policy from a file (format inferred from extension).
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read policy file: {}", path.display()))?;
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("toml")
            .to_ascii_lowercase();
        let policy: Policy = match ext.as_str() {
            "json" => serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse JSON policy: {}", path.display()))?,
            "yaml" | "yml" => serde_yaml::from_str(&content)
                .with_context(|| format!("Failed to parse YAML policy: {}", path.display()))?,
            _ => toml::from_str(&content)
                .with_context(|| format!("Failed to parse TOML policy: {}", path.display()))?,
        };
        policy.validate()?;
        Ok(policy)
    }

    /// Discovers and loads the active policy, falling back to [`Policy::permissive`].
    ///
    /// `explicit` is an optional caller-provided path that takes top priority.
    pub fn discover(explicit: Option<&str>) -> Result<Self> {
        if let Some(path) = explicit {
            return Self::from_file(Path::new(path));
        }
        if let Ok(env_path) = std::env::var(POLICY_FILE_ENV) {
            return Self::from_file(Path::new(&env_path));
        }
        let project = Path::new("legalis-policy.toml");
        if project.exists() {
            return Self::from_file(project);
        }
        if let Ok(data_dir) = paths::data_dir() {
            let global = data_dir.join("policy.toml");
            if global.exists() {
                return Self::from_file(&global);
            }
        }
        Ok(Self::permissive())
    }

    /// Saves the policy to a file as TOML.
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).context("Failed to serialize policy")?;
        std::fs::write(path, content)
            .with_context(|| format!("Failed to write policy: {}", path.display()))?;
        Ok(())
    }

    /// Returns the default global policy path under the data directory.
    pub fn default_path() -> Result<PathBuf> {
        Ok(paths::data_dir()?.join("policy.toml"))
    }

    /// Validates the policy for internal consistency.
    pub fn validate(&self) -> Result<()> {
        // A command appearing in both allow- and deny- lists is contradictory.
        let overlap: Vec<&String> = self
            .allowed_commands
            .intersection(&self.denied_commands)
            .collect();
        if !overlap.is_empty() {
            anyhow::bail!(
                "policy '{}' lists command(s) as both allowed and denied: {}",
                self.name,
                overlap
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        Ok(())
    }

    /// Whether this policy restricts anything at all.
    pub fn is_restrictive(&self) -> bool {
        !self.allowed_commands.is_empty()
            || !self.denied_commands.is_empty()
            || self.require_compliance
            || self.limits != PolicyLimits::default()
    }

    /// Evaluates whether a command name is permitted, considering whether
    /// compliance mode is currently active.
    pub fn evaluate(&self, command: &str, compliance_active: bool) -> PolicyDecision {
        if self.require_compliance && !compliance_active {
            return PolicyDecision::Denied(format!(
                "policy '{}' requires compliance mode (run with --compliance or set LEGALIS_COMPLIANCE=1)",
                self.name
            ));
        }
        if self.denied_commands.contains(command) {
            return PolicyDecision::Denied(format!(
                "command '{}' is denied by policy '{}'",
                command, self.name
            ));
        }
        if !self.allowed_commands.is_empty() && !self.allowed_commands.contains(command) {
            return PolicyDecision::Denied(format!(
                "command '{}' is not in the allow-list of policy '{}'",
                command, self.name
            ));
        }
        PolicyDecision::Allowed
    }

    /// Validates a numeric value against the relevant configured limit.
    ///
    /// Returns `Err` when the value exceeds the limit; `Ok(())` otherwise (and
    /// when no limit is configured for that kind).
    pub fn check_limit(&self, kind: LimitKind, value: usize) -> Result<()> {
        let limit = match kind {
            LimitKind::Population => self.limits.max_population,
            LimitKind::Workers => self.limits.max_workers,
            LimitKind::Iterations => self.limits.max_iterations,
            LimitKind::InputFiles => self.limits.max_input_files,
        };
        if let Some(max) = limit
            && value > max
        {
            anyhow::bail!(
                "policy '{}' limits {} to {}, but {} was requested",
                self.name,
                kind.name(),
                max,
                value
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file(ext: &str) -> PathBuf {
        std::env::temp_dir().join(format!("legalis-policy-{}.{}", uuid::Uuid::new_v4(), ext))
    }

    #[test]
    fn test_permissive_allows_everything() {
        let policy = Policy::permissive();
        assert!(!policy.is_restrictive());
        assert!(policy.evaluate("publish", false).is_allowed());
        assert!(policy.evaluate("uninstall", false).is_allowed());
    }

    #[test]
    fn test_deny_list() {
        let mut policy = Policy::permissive();
        policy.denied_commands.insert("publish".to_string());
        let decision = policy.evaluate("publish", true);
        assert!(!decision.is_allowed());
        assert!(decision.reason().unwrap().contains("denied"));
        assert!(policy.evaluate("verify", true).is_allowed());
    }

    #[test]
    fn test_allow_list() {
        let mut policy = Policy::permissive();
        policy.allowed_commands.insert("verify".to_string());
        policy.allowed_commands.insert("lint".to_string());
        assert!(policy.evaluate("verify", true).is_allowed());
        let decision = policy.evaluate("publish", true);
        assert!(!decision.is_allowed());
        assert!(decision.reason().unwrap().contains("allow-list"));
    }

    #[test]
    fn test_require_compliance() {
        let mut policy = Policy::permissive();
        policy.require_compliance = true;
        assert!(!policy.evaluate("verify", false).is_allowed());
        assert!(policy.evaluate("verify", true).is_allowed());
    }

    #[test]
    fn test_limits() {
        let mut policy = Policy::permissive();
        policy.limits.max_population = Some(1000);
        policy.limits.max_workers = Some(4);
        assert!(policy.check_limit(LimitKind::Population, 500).is_ok());
        assert!(policy.check_limit(LimitKind::Population, 1000).is_ok());
        assert!(policy.check_limit(LimitKind::Population, 1001).is_err());
        assert!(policy.check_limit(LimitKind::Workers, 8).is_err());
        // Unconfigured kinds are unrestricted.
        assert!(policy.check_limit(LimitKind::Iterations, 1_000_000).is_ok());
    }

    #[test]
    fn test_contradictory_policy_rejected() {
        let mut policy = Policy::permissive();
        policy.allowed_commands.insert("verify".to_string());
        policy.denied_commands.insert("verify".to_string());
        assert!(policy.validate().is_err());
    }

    #[test]
    fn test_toml_roundtrip() {
        let path = temp_file("toml");
        let mut policy = Policy::permissive();
        policy.name = "enterprise".to_string();
        policy.denied_commands.insert("clean".to_string());
        policy.require_audit_log = true;
        policy.limits.max_iterations = Some(50);
        policy.save(&path).expect("save");
        let loaded = Policy::from_file(&path).expect("load");
        assert_eq!(loaded, policy);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_json_load() {
        let path = temp_file("json");
        let json = r#"{"name":"j","denied_commands":["serve"],"require_compliance":true}"#;
        std::fs::write(&path, json).expect("write");
        let policy = Policy::from_file(&path).expect("load json");
        assert_eq!(policy.name, "j");
        assert!(policy.require_compliance);
        assert!(policy.denied_commands.contains("serve"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_discover_falls_back_to_permissive() {
        // With no env and no project file in temp cwd, discovery is permissive.
        let dir = std::env::temp_dir().join(format!("legalis-pol-disc-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let saved_data = std::env::var(paths::DATA_DIR_ENV).ok();
        let saved_pol = std::env::var(POLICY_FILE_ENV).ok();
        unsafe {
            std::env::set_var(paths::DATA_DIR_ENV, &dir);
            std::env::remove_var(POLICY_FILE_ENV);
        }
        let policy = Policy::discover(None).expect("discover");
        assert!(!policy.is_restrictive());
        unsafe {
            match saved_data {
                Some(v) => std::env::set_var(paths::DATA_DIR_ENV, v),
                None => std::env::remove_var(paths::DATA_DIR_ENV),
            }
            if let Some(v) = saved_pol {
                std::env::set_var(POLICY_FILE_ENV, v);
            }
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
