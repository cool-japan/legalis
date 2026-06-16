//! Shared filesystem locations for CLI state (audit logs, usage stats,
//! checkpoints, policy, repaired configs).
//!
//! All state lives under a single, overridable base directory so that tests can
//! redirect everything to [`std::env::temp_dir`] via `LEGALIS_DATA_DIR`.
//!
//! Resolution order for the base directory:
//! 1. `LEGALIS_DATA_DIR` environment variable (used verbatim).
//! 2. The platform data directory (`dirs::data_dir()`), joined with `legalis`.
//! 3. The platform cache directory, joined with `legalis` (last-ditch fallback).

use anyhow::{Context, Result};
use std::path::PathBuf;

/// Environment variable that overrides the CLI state base directory.
pub const DATA_DIR_ENV: &str = "LEGALIS_DATA_DIR";

/// Returns the base directory for CLI state, creating it if necessary.
pub fn data_dir() -> Result<PathBuf> {
    let base = if let Ok(custom) = std::env::var(DATA_DIR_ENV) {
        PathBuf::from(custom)
    } else if let Some(dir) = dirs::data_dir() {
        dir.join("legalis")
    } else if let Some(dir) = dirs::cache_dir() {
        dir.join("legalis")
    } else {
        std::env::temp_dir().join("legalis")
    };
    std::fs::create_dir_all(&base)
        .with_context(|| format!("Failed to create data directory: {}", base.display()))?;
    Ok(base)
}

/// Returns `<data_dir>/<name>`, creating the parent directory.
pub fn data_subdir(name: &str) -> Result<PathBuf> {
    let dir = data_dir()?.join(name);
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create directory: {}", dir.display()))?;
    Ok(dir)
}

/// Returns the path to the JSONL audit log.
pub fn audit_log_path() -> Result<PathBuf> {
    Ok(data_dir()?.join("audit.jsonl"))
}

/// Returns the path to the usage-statistics database.
pub fn usage_stats_path() -> Result<PathBuf> {
    Ok(data_dir()?.join("usage_stats.json"))
}

/// Returns the directory holding crash-recovery checkpoints.
pub fn checkpoint_dir() -> Result<PathBuf> {
    data_subdir("checkpoints")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs a closure with `LEGALIS_DATA_DIR` pointed at a fresh temp directory.
    fn with_temp_data_dir<F: FnOnce(&PathBuf)>(f: F) {
        let dir = std::env::temp_dir().join(format!("legalis-paths-{}", uuid::Uuid::new_v4()));
        let saved = std::env::var(DATA_DIR_ENV).ok();
        unsafe {
            std::env::set_var(DATA_DIR_ENV, &dir);
        }
        f(&dir);
        unsafe {
            match saved {
                Some(v) => std::env::set_var(DATA_DIR_ENV, v),
                None => std::env::remove_var(DATA_DIR_ENV),
            }
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_data_dir_honors_env() {
        with_temp_data_dir(|dir| {
            let resolved = data_dir().expect("data_dir");
            assert_eq!(resolved, *dir);
            assert!(resolved.exists());
        });
    }

    #[test]
    fn test_subdir_paths() {
        with_temp_data_dir(|dir| {
            let cp = checkpoint_dir().expect("checkpoint_dir");
            assert_eq!(cp, dir.join("checkpoints"));
            assert!(cp.exists());
            let audit = audit_log_path().expect("audit_log_path");
            assert_eq!(audit, dir.join("audit.jsonl"));
            let usage = usage_stats_path().expect("usage_stats_path");
            assert_eq!(usage, dir.join("usage_stats.json"));
        });
    }
}
