//! Compliance mode for the Legalis CLI.
//!
//! Compliance mode is a hardened operating posture for regulated environments.
//! When active it:
//!
//! - **forces audit logging** for every operation (handlers must record),
//! - **guards sensitive/destructive operations** — by default they are blocked
//!   unless explicitly confirmed via `--yes`/force flags, and a configurable set
//!   can be hard-disabled outright,
//! - exposes the active set so commands can decide whether to proceed.
//!
//! It is activated by the `--compliance` global flag or `LEGALIS_COMPLIANCE=1`.
//! Inactive mode is fully permissive (backward compatible).

use std::collections::BTreeSet;

/// Environment variable that activates compliance mode.
pub const COMPLIANCE_ENV: &str = "LEGALIS_COMPLIANCE";

/// Commands considered sensitive/destructive in a regulated context.
///
/// These either mutate shared registry state, delete data, expose a network
/// service, or push artifacts outside the local machine.
pub const SENSITIVE_COMMANDS: &[&str] = &[
    "publish",
    "uninstall",
    "clean",
    "serve",
    "registry",
    "install",
    "update",
    "add",
    "port",
    "migrate",
    "cloud",
];

/// Reason a command was blocked while compliance mode was active.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplianceBlock {
    /// The operation is sensitive and was not explicitly confirmed.
    NeedsConfirmation(String),
    /// The operation is hard-disabled by the compliance configuration.
    Disabled(String),
}

impl ComplianceBlock {
    /// A human-readable explanation of the block.
    pub fn message(&self) -> &str {
        match self {
            ComplianceBlock::NeedsConfirmation(m) | ComplianceBlock::Disabled(m) => m,
        }
    }
}

/// Compliance-mode configuration and gate.
#[derive(Debug, Clone)]
pub struct ComplianceMode {
    active: bool,
    /// Sensitive commands that are entirely forbidden in compliance mode.
    disabled_commands: BTreeSet<String>,
    /// Sensitive commands recognized by this gate.
    sensitive_commands: BTreeSet<String>,
}

impl Default for ComplianceMode {
    fn default() -> Self {
        Self::inactive()
    }
}

impl ComplianceMode {
    /// An inactive (permissive) compliance mode.
    pub fn inactive() -> Self {
        Self {
            active: false,
            disabled_commands: BTreeSet::new(),
            sensitive_commands: SENSITIVE_COMMANDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// An active compliance mode with the default sensitive command set.
    pub fn active() -> Self {
        Self {
            active: true,
            ..Self::inactive()
        }
    }

    /// Resolves compliance mode from a flag plus the environment.
    pub fn resolve(flag: bool) -> Self {
        let active = flag
            || std::env::var(COMPLIANCE_ENV)
                .map(|v| matches!(v.trim(), "1" | "true" | "yes" | "on"))
                .unwrap_or(false);
        if active {
            Self::active()
        } else {
            Self::inactive()
        }
    }

    /// Whether compliance mode is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Whether audit logging is mandatory (true iff active).
    pub fn requires_audit_log(&self) -> bool {
        self.active
    }

    /// Marks a command as hard-disabled under compliance mode.
    pub fn disable_command(&mut self, command: impl Into<String>) {
        self.disabled_commands.insert(command.into());
    }

    /// Whether a command is considered sensitive by this gate.
    pub fn is_sensitive(&self, command: &str) -> bool {
        self.sensitive_commands.contains(command)
    }

    /// Evaluates whether a command may proceed.
    ///
    /// `confirmed` should be `true` when the user passed an explicit
    /// confirmation/force flag (e.g. `--yes`, `--force`). When compliance mode
    /// is inactive this always returns `Ok(())`.
    pub fn guard(&self, command: &str, confirmed: bool) -> Result<(), ComplianceBlock> {
        if !self.active {
            return Ok(());
        }
        if self.disabled_commands.contains(command) {
            return Err(ComplianceBlock::Disabled(format!(
                "command '{command}' is disabled in compliance mode"
            )));
        }
        if self.is_sensitive(command) && !confirmed {
            return Err(ComplianceBlock::NeedsConfirmation(format!(
                "command '{command}' is sensitive; in compliance mode it must be explicitly confirmed (pass --yes/--force)"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inactive_is_permissive() {
        let mode = ComplianceMode::inactive();
        assert!(!mode.is_active());
        assert!(!mode.requires_audit_log());
        assert!(mode.guard("publish", false).is_ok());
        assert!(mode.guard("clean", false).is_ok());
    }

    #[test]
    fn test_active_guards_sensitive() {
        let mode = ComplianceMode::active();
        assert!(mode.is_active());
        assert!(mode.requires_audit_log());
        // Sensitive without confirmation -> blocked.
        let blocked = mode.guard("publish", false);
        assert!(blocked.is_err());
        assert!(matches!(
            blocked.unwrap_err(),
            ComplianceBlock::NeedsConfirmation(_)
        ));
        // Sensitive with confirmation -> allowed.
        assert!(mode.guard("publish", true).is_ok());
        // Non-sensitive -> always allowed.
        assert!(mode.guard("verify", false).is_ok());
    }

    #[test]
    fn test_disabled_command() {
        let mut mode = ComplianceMode::active();
        mode.disable_command("clean");
        let blocked = mode.guard("clean", true);
        assert!(blocked.is_err());
        assert!(matches!(blocked.unwrap_err(), ComplianceBlock::Disabled(_)));
    }

    #[test]
    fn test_resolve_from_flag() {
        assert!(ComplianceMode::resolve(true).is_active());
    }

    #[test]
    fn test_resolve_from_env() {
        let saved = std::env::var(COMPLIANCE_ENV).ok();
        unsafe {
            std::env::set_var(COMPLIANCE_ENV, "yes");
        }
        assert!(ComplianceMode::resolve(false).is_active());
        unsafe {
            std::env::set_var(COMPLIANCE_ENV, "0");
        }
        assert!(!ComplianceMode::resolve(false).is_active());
        unsafe {
            match saved {
                Some(v) => std::env::set_var(COMPLIANCE_ENV, v),
                None => std::env::remove_var(COMPLIANCE_ENV),
            }
        }
    }

    #[test]
    fn test_block_messages() {
        let block = ComplianceBlock::Disabled("x disabled".to_string());
        assert!(block.message().contains("disabled"));
        let block = ComplianceBlock::NeedsConfirmation("confirm x".to_string());
        assert!(block.message().contains("confirm"));
    }
}
