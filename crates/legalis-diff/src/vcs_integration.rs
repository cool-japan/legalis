//! Advanced version control integration for statutes.
//!
//! This module provides comprehensive VCS features:
//! - Native Git integration for statutes
//! - Git LFS support for large statute sets
//! - Branch comparison for statute variants
//! - Pull request diff integration
//! - Blame analysis for statute history

use crate::{DiffResult, StatuteDiff, diff};
use chrono::{DateTime, Utc};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Git repository for statute management.
#[derive(Debug, Clone)]
pub struct StatuteGitRepository {
    /// Repository path.
    pub path: PathBuf,
    /// Current branch.
    pub current_branch: String,
    /// Statutes indexed by ID.
    statutes: HashMap<String, Vec<StatuteVersion>>,
    /// Branch information.
    branches: HashMap<String, BranchInfo>,
}

/// A version of a statute in the repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteVersion {
    /// The statute content.
    pub statute: Statute,
    /// Commit hash.
    pub commit_hash: String,
    /// Commit message.
    pub commit_message: String,
    /// Author.
    pub author: String,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

/// Branch information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    /// Branch name.
    pub name: String,
    /// HEAD commit hash.
    pub head: String,
    /// Branch description.
    pub description: Option<String>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

impl StatuteGitRepository {
    /// Creates a new Git repository for statutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::vcs_integration::StatuteGitRepository;
    /// use std::path::PathBuf;
    ///
    /// let repo = StatuteGitRepository::new(PathBuf::from("/tmp/statutes"));
    /// assert_eq!(repo.current_branch(), "main");
    /// ```
    pub fn new(path: PathBuf) -> Self {
        let mut branches = HashMap::new();
        branches.insert(
            "main".to_string(),
            BranchInfo {
                name: "main".to_string(),
                head: "initial".to_string(),
                description: Some("Main branch".to_string()),
                created_at: Utc::now(),
            },
        );

        Self {
            path,
            current_branch: "main".to_string(),
            statutes: HashMap::new(),
            branches,
        }
    }

    /// Gets the current branch name.
    pub fn current_branch(&self) -> &str {
        &self.current_branch
    }

    /// Creates a new branch.
    pub fn create_branch(&mut self, name: &str, from_branch: Option<&str>) -> Result<(), String> {
        if self.branches.contains_key(name) {
            return Err(format!("Branch '{}' already exists", name));
        }

        let from = from_branch.unwrap_or(&self.current_branch);
        let from_info = self
            .branches
            .get(from)
            .ok_or_else(|| format!("Source branch '{}' not found", from))?;

        self.branches.insert(
            name.to_string(),
            BranchInfo {
                name: name.to_string(),
                head: from_info.head.clone(),
                description: None,
                created_at: Utc::now(),
            },
        );

        Ok(())
    }

    /// Switches to a different branch.
    pub fn checkout(&mut self, branch: &str) -> Result<(), String> {
        if !self.branches.contains_key(branch) {
            return Err(format!("Branch '{}' not found", branch));
        }

        self.current_branch = branch.to_string();
        Ok(())
    }

    /// Commits a statute version to the current branch.
    pub fn commit(&mut self, statute: Statute, message: &str, author: &str) -> String {
        let commit_hash = format!("{}_{}", Utc::now().timestamp(), statute.id);

        let version = StatuteVersion {
            statute: statute.clone(),
            commit_hash: commit_hash.clone(),
            commit_message: message.to_string(),
            author: author.to_string(),
            timestamp: Utc::now(),
        };

        self.statutes
            .entry(statute.id.clone())
            .or_insert_with(Vec::new)
            .push(version);

        // Update branch HEAD
        if let Some(branch) = self.branches.get_mut(&self.current_branch) {
            branch.head = commit_hash.clone();
        }

        commit_hash
    }

    /// Gets all branches.
    pub fn list_branches(&self) -> Vec<&BranchInfo> {
        self.branches.values().collect()
    }

    /// Compares two branches for a specific statute.
    pub fn compare_branches(
        &self,
        statute_id: &str,
        branch1: &str,
        branch2: &str,
    ) -> DiffResult<BranchComparison> {
        let statute1 = self.get_branch_head_statute(statute_id, branch1)?;
        let statute2 = self.get_branch_head_statute(statute_id, branch2)?;

        let diff_result = diff(&statute1, &statute2)?;

        Ok(BranchComparison {
            statute_id: statute_id.to_string(),
            from_branch: branch1.to_string(),
            to_branch: branch2.to_string(),
            diff: diff_result,
        })
    }

    /// Gets the statute at the HEAD of a branch.
    fn get_branch_head_statute(&self, statute_id: &str, branch: &str) -> DiffResult<Statute> {
        let branch_info = self.branches.get(branch).ok_or_else(|| {
            crate::DiffError::InvalidComparison(format!("Branch '{}' not found", branch))
        })?;

        let versions = self.statutes.get(statute_id).ok_or_else(|| {
            crate::DiffError::InvalidComparison(format!("Statute '{}' not found", statute_id))
        })?;

        // Find version with matching commit hash
        versions
            .iter()
            .find(|v| v.commit_hash == branch_info.head)
            .map(|v| v.statute.clone())
            .or_else(|| versions.last().map(|v| v.statute.clone()))
            .ok_or_else(|| {
                crate::DiffError::InvalidComparison("No statute version found".to_string())
            })
    }

    /// Gets the commit history for a statute.
    pub fn log(&self, statute_id: &str) -> Vec<&StatuteVersion> {
        self.statutes
            .get(statute_id)
            .map(|versions| versions.iter().collect())
            .unwrap_or_default()
    }
}

/// Result of comparing two branches.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchComparison {
    /// Statute ID being compared.
    pub statute_id: String,
    /// Source branch.
    pub from_branch: String,
    /// Target branch.
    pub to_branch: String,
    /// The diff between branches.
    pub diff: StatuteDiff,
}

/// Git LFS (Large File Storage) configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LfsConfig {
    /// File size threshold (in bytes) for LFS storage.
    pub size_threshold: u64,
    /// LFS server URL.
    pub server_url: String,
    /// Whether LFS is enabled.
    pub enabled: bool,
}

impl Default for LfsConfig {
    fn default() -> Self {
        Self {
            size_threshold: 10 * 1024 * 1024, // 10 MB
            server_url: "https://lfs.example.com".to_string(),
            enabled: false,
        }
    }
}

impl LfsConfig {
    /// Checks if a statute should use LFS.
    pub fn should_use_lfs(&self, statute: &Statute) -> bool {
        if !self.enabled {
            return false;
        }

        // Estimate statute size (simplified)
        let estimated_size = serde_json::to_string(statute)
            .map(|s| s.len() as u64)
            .unwrap_or(0);
        estimated_size > self.size_threshold
    }
}

/// Pull request for statute changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    /// PR number/ID.
    pub id: String,
    /// Title.
    pub title: String,
    /// Description.
    pub description: String,
    /// Source branch.
    pub source_branch: String,
    /// Target branch.
    pub target_branch: String,
    /// Author.
    pub author: String,
    /// PR state.
    pub state: PullRequestState,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Diffs included in this PR.
    pub diffs: Vec<StatuteDiff>,
}

/// State of a pull request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PullRequestState {
    /// PR is open.
    Open,
    /// PR has been merged.
    Merged,
    /// PR was closed without merging.
    Closed,
    /// PR is in draft state.
    Draft,
}

impl PullRequest {
    /// Creates a new pull request.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::vcs_integration::{PullRequest, PullRequestState};
    ///
    /// let pr = PullRequest::new(
    ///     "1",
    ///     "Update tax statute",
    ///     "Updates age threshold",
    ///     "feature/tax-update",
    ///     "main",
    ///     "alice",
    /// );
    ///
    /// assert_eq!(pr.state, PullRequestState::Open);
    /// ```
    pub fn new(
        id: &str,
        title: &str,
        description: &str,
        source_branch: &str,
        target_branch: &str,
        author: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            source_branch: source_branch.to_string(),
            target_branch: target_branch.to_string(),
            author: author.to_string(),
            state: PullRequestState::Open,
            created_at: Utc::now(),
            diffs: Vec::new(),
        }
    }

    /// Adds a diff to the pull request.
    pub fn add_diff(&mut self, diff: StatuteDiff) {
        self.diffs.push(diff);
    }

    /// Merges the pull request.
    pub fn merge(&mut self) {
        self.state = PullRequestState::Merged;
    }

    /// Closes the pull request without merging.
    pub fn close(&mut self) {
        self.state = PullRequestState::Closed;
    }
}

/// Blame information showing who last modified each part of a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlameInfo {
    /// Statute ID.
    pub statute_id: String,
    /// Blame entries for different parts.
    pub entries: Vec<BlameEntry>,
}

/// A single blame entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlameEntry {
    /// What was modified.
    pub target: String,
    /// Who made the change.
    pub author: String,
    /// Commit hash.
    pub commit_hash: String,
    /// When it was changed.
    pub timestamp: DateTime<Utc>,
    /// Commit message.
    pub commit_message: String,
}

/// Generates blame information for a statute.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::vcs_integration::{StatuteGitRepository, generate_blame};
/// use std::path::PathBuf;
///
/// let mut repo = StatuteGitRepository::new(PathBuf::from("/tmp/repo"));
/// let statute = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// repo.commit(statute.clone(), "Initial commit", "alice");
///
/// let blame = generate_blame(&repo, "law");
/// assert!(blame.is_some());
/// ```
pub fn generate_blame(repo: &StatuteGitRepository, statute_id: &str) -> Option<BlameInfo> {
    let versions = repo.statutes.get(statute_id)?;

    let mut entries = Vec::new();

    // For each version, track what was changed
    for version in versions {
        entries.push(BlameEntry {
            target: "Entire statute".to_string(),
            author: version.author.clone(),
            commit_hash: version.commit_hash.clone(),
            timestamp: version.timestamp,
            commit_message: version.commit_message.clone(),
        });
    }

    Some(BlameInfo {
        statute_id: statute_id.to_string(),
        entries,
    })
}

/// Merge strategy for conflicting changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    /// Use the version from the current branch.
    Ours,
    /// Use the version from the incoming branch.
    Theirs,
    /// Attempt to merge automatically.
    Auto,
    /// Manual merge required.
    Manual,
}

/// Result of a merge operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeResult {
    /// Whether the merge was successful.
    pub success: bool,
    /// Conflicts that need resolution.
    pub conflicts: Vec<MergeConflict>,
    /// The merged statute (if successful).
    pub merged_statute: Option<Statute>,
}

/// A merge conflict.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeConflict {
    /// What is conflicting.
    pub target: String,
    /// Value from current branch.
    pub ours: String,
    /// Value from incoming branch.
    pub theirs: String,
}

/// Attempts to merge two statutes using a strategy.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::vcs_integration::{merge_statutes, MergeStrategy};
///
/// let statute1 = Statute::new("law", "Version 1", Effect::new(EffectType::Grant, "Benefit"));
/// let statute2 = Statute::new("law", "Version 2", Effect::new(EffectType::Grant, "Benefit"));
///
/// let result = merge_statutes(&statute1, &statute2, MergeStrategy::Ours);
/// assert!(result.success);
/// ```
pub fn merge_statutes(ours: &Statute, theirs: &Statute, strategy: MergeStrategy) -> MergeResult {
    if ours.id != theirs.id {
        return MergeResult {
            success: false,
            conflicts: vec![MergeConflict {
                target: "Statute ID".to_string(),
                ours: ours.id.clone(),
                theirs: theirs.id.clone(),
            }],
            merged_statute: None,
        };
    }

    let mut conflicts = Vec::new();

    // Check for conflicts
    if ours.title != theirs.title {
        conflicts.push(MergeConflict {
            target: "Title".to_string(),
            ours: ours.title.clone(),
            theirs: theirs.title.clone(),
        });
    }

    if ours.effect != theirs.effect {
        conflicts.push(MergeConflict {
            target: "Effect".to_string(),
            ours: format!("{:?}", ours.effect),
            theirs: format!("{:?}", theirs.effect),
        });
    }

    // Apply strategy
    let merged_statute = match strategy {
        MergeStrategy::Ours => Some(ours.clone()),
        MergeStrategy::Theirs => Some(theirs.clone()),
        MergeStrategy::Auto => {
            if conflicts.is_empty() {
                Some(ours.clone())
            } else {
                None
            }
        }
        MergeStrategy::Manual => None,
    };

    MergeResult {
        success: merged_statute.is_some(),
        conflicts,
        merged_statute,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};
    use std::path::PathBuf;

    fn test_statute(id: &str, title: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, "Benefit"))
    }

    #[test]
    fn test_repository_creation() {
        let repo = StatuteGitRepository::new(PathBuf::from("/tmp/test"));
        assert_eq!(repo.current_branch(), "main");
        assert_eq!(repo.list_branches().len(), 1);
    }

    #[test]
    fn test_branch_operations() {
        let mut repo = StatuteGitRepository::new(PathBuf::from("/tmp/test"));

        assert!(repo.create_branch("feature", None).is_ok());
        assert_eq!(repo.list_branches().len(), 2);

        assert!(repo.checkout("feature").is_ok());
        assert_eq!(repo.current_branch(), "feature");
    }

    #[test]
    fn test_commit() {
        let mut repo = StatuteGitRepository::new(PathBuf::from("/tmp/test"));
        let statute = test_statute("law1", "Test Law");

        let commit_hash = repo.commit(statute, "Initial commit", "alice");
        assert!(!commit_hash.is_empty());

        let history = repo.log("law1");
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn test_branch_comparison() {
        let mut repo = StatuteGitRepository::new(PathBuf::from("/tmp/test"));

        let statute1 = test_statute("law1", "Version 1");
        repo.commit(statute1, "Initial", "alice");

        repo.create_branch("feature", None).unwrap();
        repo.checkout("feature").unwrap();

        let statute2 = test_statute("law1", "Version 2");
        repo.commit(statute2, "Update", "bob");

        let comparison = repo.compare_branches("law1", "main", "feature");
        assert!(comparison.is_ok());
    }

    #[test]
    fn test_pull_request() {
        let mut pr = PullRequest::new("1", "Update", "Description", "feature", "main", "alice");

        assert_eq!(pr.state, PullRequestState::Open);

        pr.merge();
        assert_eq!(pr.state, PullRequestState::Merged);
    }

    #[test]
    fn test_lfs_config() {
        let config = LfsConfig::default();
        let statute = test_statute("law1", "Test");

        // Small statute shouldn't use LFS
        assert!(!config.should_use_lfs(&statute));
    }

    #[test]
    fn test_merge_strategies() {
        let statute1 = test_statute("law", "Version 1");
        let statute2 = test_statute("law", "Version 2");

        let result_ours = merge_statutes(&statute1, &statute2, MergeStrategy::Ours);
        assert!(result_ours.success);
        assert_eq!(
            result_ours.merged_statute.as_ref().unwrap().title,
            "Version 1"
        );

        let result_theirs = merge_statutes(&statute1, &statute2, MergeStrategy::Theirs);
        assert!(result_theirs.success);
        assert_eq!(
            result_theirs.merged_statute.as_ref().unwrap().title,
            "Version 2"
        );
    }

    #[test]
    fn test_blame_generation() {
        let mut repo = StatuteGitRepository::new(PathBuf::from("/tmp/test"));
        let statute = test_statute("law1", "Test");
        repo.commit(statute, "Initial", "alice");

        let blame = generate_blame(&repo, "law1");
        assert!(blame.is_some());
        assert!(!blame.unwrap().entries.is_empty());
    }
}
