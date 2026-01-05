//! Version control system integration for statute diffs.
//!
//! This module provides hooks and integration with version control systems
//! like Git to track statute changes over time.

use crate::{StatuteDiff, VersionInfo, diff};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors that can occur during VCS operations.
#[derive(Debug, Error)]
pub enum VcsError {
    #[error("Repository not found at path: {0}")]
    RepositoryNotFound(PathBuf),

    #[error("Invalid commit reference: {0}")]
    InvalidCommit(String),

    #[error("Statute not found in repository: {0}")]
    StatuteNotFound(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Diff error: {0}")]
    Diff(#[from] crate::DiffError),
}

/// Result type for VCS operations.
pub type VcsResult<T> = Result<T, VcsError>;

/// Represents a commit in the version control system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    /// Commit hash or identifier.
    pub id: String,
    /// Author of the commit.
    pub author: String,
    /// Commit message.
    pub message: String,
    /// Timestamp of the commit.
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Parent commit IDs.
    pub parents: Vec<String>,
}

/// A statute repository that tracks changes over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteRepository {
    /// Path to the repository.
    pub path: PathBuf,
    /// Current branch name.
    pub current_branch: String,
    /// Commits indexed by ID.
    pub commits: HashMap<String, Commit>,
    /// Statutes indexed by (commit_id, statute_id).
    pub statutes: HashMap<(String, String), Statute>,
}

impl StatuteRepository {
    /// Creates a new statute repository.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            current_branch: "main".to_string(),
            commits: HashMap::new(),
            statutes: HashMap::new(),
        }
    }

    /// Initializes a repository from a directory.
    pub fn init<P: AsRef<Path>>(path: P) -> VcsResult<Self> {
        let repo_path = path.as_ref().to_path_buf();

        if !repo_path.exists() {
            return Err(VcsError::RepositoryNotFound(repo_path));
        }

        Ok(Self::new(repo_path))
    }

    /// Adds a commit to the repository.
    pub fn add_commit(&mut self, commit: Commit, statutes: Vec<Statute>) {
        let commit_id = commit.id.clone();

        for statute in statutes {
            self.statutes
                .insert((commit_id.clone(), statute.id.clone()), statute);
        }

        self.commits.insert(commit_id, commit);
    }

    /// Gets a statute at a specific commit.
    pub fn get_statute(&self, commit_id: &str, statute_id: &str) -> VcsResult<&Statute> {
        self.statutes
            .get(&(commit_id.to_string(), statute_id.to_string()))
            .ok_or_else(|| VcsError::StatuteNotFound(statute_id.to_string()))
    }

    /// Compares a statute between two commits.
    pub fn diff_commits(
        &self,
        old_commit: &str,
        new_commit: &str,
        statute_id: &str,
    ) -> VcsResult<StatuteDiff> {
        let old_statute = self.get_statute(old_commit, statute_id)?;
        let new_statute = self.get_statute(new_commit, statute_id)?;

        let mut diff_result = diff(old_statute, new_statute)?;

        // Add version info based on commits
        diff_result.version_info = Some(VersionInfo {
            old_version: Some(self.get_commit_number(old_commit)),
            new_version: Some(self.get_commit_number(new_commit)),
        });

        Ok(diff_result)
    }

    /// Gets all commits affecting a statute.
    pub fn get_statute_history(&self, statute_id: &str) -> Vec<String> {
        self.statutes
            .keys()
            .filter(|(_, sid)| sid == statute_id)
            .map(|(cid, _)| cid.clone())
            .collect()
    }

    /// Gets a commit by ID.
    pub fn get_commit(&self, commit_id: &str) -> VcsResult<&Commit> {
        self.commits
            .get(commit_id)
            .ok_or_else(|| VcsError::InvalidCommit(commit_id.to_string()))
    }

    /// Lists all commits in chronological order.
    pub fn list_commits(&self) -> Vec<&Commit> {
        let mut commits: Vec<&Commit> = self.commits.values().collect();
        commits.sort_by_key(|c| c.timestamp);
        commits
    }

    /// Gets the commit number (index in chronological order).
    fn get_commit_number(&self, commit_id: &str) -> u32 {
        let commits = self.list_commits();
        commits
            .iter()
            .position(|c| c.id == commit_id)
            .map(|pos| pos as u32 + 1)
            .unwrap_or(0)
    }
}

/// Git-specific integration helpers.
pub mod git {
    use super::*;

    /// Configuration for Git hooks.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GitHookConfig {
        /// Whether to run diff on pre-commit.
        pub pre_commit: bool,
        /// Whether to generate reports on post-commit.
        pub post_commit: bool,
        /// Output format for hook reports.
        pub report_format: ReportFormat,
    }

    /// Report format for Git hooks.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum ReportFormat {
        Json,
        Markdown,
        Html,
        Unified,
    }

    impl Default for GitHookConfig {
        fn default() -> Self {
            Self {
                pre_commit: true,
                post_commit: true,
                report_format: ReportFormat::Markdown,
            }
        }
    }

    /// Generates a Git hook script for pre-commit.
    pub fn generate_pre_commit_hook(config: &GitHookConfig) -> String {
        let mut script = String::from("#!/bin/sh\n\n");
        script.push_str("# Legalis statute diff pre-commit hook\n");
        script.push_str("# Auto-generated - DO NOT EDIT\n\n");

        if config.pre_commit {
            script.push_str("echo \"Running statute diff check...\"\n\n");
            script.push_str("# Get list of changed statute files\n");
            script.push_str("CHANGED_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\\.statute\\.json$')\n\n");
            script.push_str("if [ -z \"$CHANGED_FILES\" ]; then\n");
            script.push_str("    echo \"No statute files changed.\"\n");
            script.push_str("    exit 0\n");
            script.push_str("fi\n\n");
            script.push_str("# Run diff for each changed file\n");
            script.push_str("for file in $CHANGED_FILES; do\n");
            script.push_str("    echo \"Checking $file...\"\n");
            script.push_str("    # Add your diff logic here\n");
            script.push_str("    # legalis-diff \"$file\" || exit 1\n");
            script.push_str("done\n\n");
            script.push_str("echo \"Statute diff check completed.\"\n");
        }

        script.push_str("exit 0\n");
        script
    }

    /// Generates a Git hook script for post-commit.
    pub fn generate_post_commit_hook(config: &GitHookConfig) -> String {
        let mut script = String::from("#!/bin/sh\n\n");
        script.push_str("# Legalis statute diff post-commit hook\n");
        script.push_str("# Auto-generated - DO NOT EDIT\n\n");

        if config.post_commit {
            script.push_str("echo \"Generating statute diff reports...\"\n\n");

            let format_flag = match config.report_format {
                ReportFormat::Json => "--format=json",
                ReportFormat::Markdown => "--format=markdown",
                ReportFormat::Html => "--format=html",
                ReportFormat::Unified => "--format=unified",
            };

            script.push_str(&format!("# Format: {}\n", format_flag));
            script.push_str("# Add your report generation logic here\n");
            script.push_str(&format!(
                "# legalis-diff-report {} HEAD~1 HEAD\n",
                format_flag
            ));
            script.push_str("\necho \"Statute diff reports generated.\"\n");
        }

        script.push_str("exit 0\n");
        script
    }

    /// Installs Git hooks into a repository.
    pub fn install_hooks<P: AsRef<Path>>(repo_path: P, config: &GitHookConfig) -> VcsResult<()> {
        let hooks_dir = repo_path.as_ref().join(".git").join("hooks");

        if !hooks_dir.exists() {
            std::fs::create_dir_all(&hooks_dir)?;
        }

        // Install pre-commit hook
        let pre_commit_path = hooks_dir.join("pre-commit");
        std::fs::write(&pre_commit_path, generate_pre_commit_hook(config))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&pre_commit_path)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&pre_commit_path, perms)?;
        }

        // Install post-commit hook
        let post_commit_path = hooks_dir.join("post-commit");
        std::fs::write(&post_commit_path, generate_post_commit_hook(config))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&post_commit_path)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&post_commit_path, perms)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn test_statute() -> Statute {
        Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    fn test_commit(id: &str, message: &str) -> Commit {
        Commit {
            id: id.to_string(),
            author: "Test Author".to_string(),
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
            parents: Vec::new(),
        }
    }

    #[test]
    fn test_repository_creation() {
        let repo = StatuteRepository::new("/tmp/test-repo");
        assert_eq!(repo.current_branch, "main");
        assert!(repo.commits.is_empty());
        assert!(repo.statutes.is_empty());
    }

    #[test]
    fn test_add_commit() {
        let mut repo = StatuteRepository::new("/tmp/test-repo");
        let commit = test_commit("commit1", "Initial commit");
        let statute = test_statute();

        repo.add_commit(commit.clone(), vec![statute.clone()]);

        assert_eq!(repo.commits.len(), 1);
        assert_eq!(repo.statutes.len(), 1);
        assert!(repo.get_statute("commit1", "test-statute").is_ok());
    }

    #[test]
    fn test_diff_commits() {
        let mut repo = StatuteRepository::new("/tmp/test-repo");

        let statute_v1 = test_statute();
        let mut statute_v2 = statute_v1.clone();
        statute_v2.title = "Updated Test Statute".to_string();

        repo.add_commit(test_commit("commit1", "v1"), vec![statute_v1]);
        repo.add_commit(test_commit("commit2", "v2"), vec![statute_v2]);

        let diff_result = repo.diff_commits("commit1", "commit2", "test-statute");
        assert!(diff_result.is_ok());

        let diff = diff_result.unwrap();
        assert!(!diff.changes.is_empty());
        assert!(diff.version_info.is_some());
    }

    #[test]
    fn test_statute_history() {
        let mut repo = StatuteRepository::new("/tmp/test-repo");
        let statute = test_statute();

        repo.add_commit(test_commit("commit1", "v1"), vec![statute.clone()]);
        repo.add_commit(test_commit("commit2", "v2"), vec![statute.clone()]);

        let history = repo.get_statute_history("test-statute");
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn test_git_hook_generation() {
        let config = git::GitHookConfig::default();

        let pre_commit = git::generate_pre_commit_hook(&config);
        assert!(pre_commit.contains("#!/bin/sh"));
        assert!(pre_commit.contains("pre-commit"));

        let post_commit = git::generate_post_commit_hook(&config);
        assert!(post_commit.contains("#!/bin/sh"));
        assert!(post_commit.contains("post-commit"));
    }
}
