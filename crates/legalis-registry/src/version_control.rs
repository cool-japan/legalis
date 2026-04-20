use super::*;
use sha2::{Digest, Sha256};
use std::fmt;

/// Branch in the version control system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    /// Branch name
    pub name: String,
    /// Branch ID
    pub branch_id: Uuid,
    /// Parent branch (None for main branch)
    pub parent_branch: Option<String>,
    /// Current head commit
    pub head_commit: Option<Uuid>,
    /// When the branch was created
    pub created_at: DateTime<Utc>,
    /// Who created the branch
    pub created_by: String,
    /// Branch description
    pub description: Option<String>,
    /// Whether the branch is protected (cannot be deleted)
    pub protected: bool,
}

impl Branch {
    /// Creates a new branch.
    pub fn new(name: impl Into<String>, created_by: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            branch_id: Uuid::new_v4(),
            parent_branch: None,
            head_commit: None,
            created_at: Utc::now(),
            created_by: created_by.into(),
            description: None,
            protected: false,
        }
    }

    /// Creates a branch from a parent.
    pub fn from_parent(
        name: impl Into<String>,
        parent: impl Into<String>,
        created_by: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            branch_id: Uuid::new_v4(),
            parent_branch: Some(parent.into()),
            head_commit: None,
            created_at: Utc::now(),
            created_by: created_by.into(),
            description: None,
            protected: false,
        }
    }

    /// Sets the branch description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Marks the branch as protected.
    pub fn with_protected(mut self, protected: bool) -> Self {
        self.protected = protected;
        self
    }
}

/// Commit in the version control system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    /// Commit ID
    pub commit_id: Uuid,
    /// Branch name
    pub branch_name: String,
    /// Parent commit IDs
    pub parent_commits: Vec<Uuid>,
    /// Statute ID being committed
    pub statute_id: String,
    /// Statute snapshot at this commit
    pub statute_entry: StatuteEntry,
    /// Commit message
    pub message: String,
    /// Who made the commit
    pub author: String,
    /// When the commit was made
    pub timestamp: DateTime<Utc>,
    /// Commit signature (for verification)
    pub signature: Option<String>,
    /// Commit hash (SHA-256 of content)
    pub commit_hash: String,
}

impl Commit {
    /// Creates a new commit.
    pub fn new(
        branch_name: impl Into<String>,
        statute_id: impl Into<String>,
        statute_entry: StatuteEntry,
        message: impl Into<String>,
        author: impl Into<String>,
    ) -> Self {
        let branch_name = branch_name.into();
        let statute_id = statute_id.into();
        let message = message.into();
        let author = author.into();
        let timestamp = Utc::now();

        // Calculate commit hash
        let mut hasher = Sha256::new();
        hasher.update(branch_name.as_bytes());
        hasher.update(statute_id.as_bytes());
        hasher.update(message.as_bytes());
        hasher.update(author.as_bytes());
        hasher.update(timestamp.to_rfc3339().as_bytes());
        let commit_hash = hex::encode(hasher.finalize());

        Self {
            commit_id: Uuid::new_v4(),
            branch_name,
            parent_commits: Vec::new(),
            statute_id,
            statute_entry,
            message,
            author,
            timestamp,
            signature: None,
            commit_hash,
        }
    }

    /// Adds a parent commit.
    pub fn with_parent(mut self, parent_id: Uuid) -> Self {
        self.parent_commits.push(parent_id);
        self
    }

    /// Signs the commit with a signature.
    pub fn with_signature(mut self, signature: impl Into<String>) -> Self {
        self.signature = Some(signature.into());
        self
    }

    /// Verifies the commit signature.
    pub fn verify_signature(&self, public_key: &str) -> bool {
        if let Some(signature) = &self.signature {
            // Placeholder: In production, use proper cryptographic verification
            // e.g., ed25519_dalek, RSA, ECDSA
            signature.starts_with("SIG:") && signature.contains(public_key)
        } else {
            false
        }
    }

    /// Gets a short commit hash (first 8 characters).
    pub fn short_hash(&self) -> String {
        self.commit_hash.chars().take(8).collect()
    }
}

/// Merge conflict during branch merging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchMergeConflict {
    /// Field name with conflict
    pub field_name: String,
    /// Value from source branch
    pub source_value: String,
    /// Value from target branch
    pub target_value: String,
    /// Value from common ancestor (if any)
    pub base_value: Option<String>,
}

impl fmt::Display for BranchMergeConflict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Conflict in '{}': source='{}' vs target='{}'",
            self.field_name, self.source_value, self.target_value
        )
    }
}

/// Result of a branch merge operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeBranchResult {
    /// Merge commit ID
    pub merge_commit_id: Option<Uuid>,
    /// Conflicts encountered
    pub conflicts: Vec<BranchMergeConflict>,
    /// Whether the merge was successful
    pub success: bool,
    /// Merge message
    pub message: String,
}

impl MergeBranchResult {
    /// Checks if there are conflicts.
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }

    /// Gets conflict count.
    pub fn conflict_count(&self) -> usize {
        self.conflicts.len()
    }
}

/// Pull request status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PullRequestStatus {
    /// Open and awaiting review
    Open,
    /// Under review
    InReview,
    /// Approved and ready to merge
    Approved,
    /// Changes requested
    ChangesRequested,
    /// Merged
    Merged,
    /// Closed without merging
    Closed,
}

/// Review decision on a pull request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewDecision {
    /// Approve the changes
    Approve,
    /// Request changes
    RequestChanges,
    /// Comment only (no approval/rejection)
    Comment,
}

/// Review on a pull request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestReview {
    /// Review ID
    pub review_id: Uuid,
    /// Pull request ID
    pub pull_request_id: Uuid,
    /// Reviewer name
    pub reviewer: String,
    /// Review decision
    pub decision: ReviewDecision,
    /// Review comments
    pub comment: String,
    /// When the review was submitted
    pub submitted_at: DateTime<Utc>,
}

impl PullRequestReview {
    /// Creates a new review.
    pub fn new(
        pull_request_id: Uuid,
        reviewer: impl Into<String>,
        decision: ReviewDecision,
        comment: impl Into<String>,
    ) -> Self {
        Self {
            review_id: Uuid::new_v4(),
            pull_request_id,
            reviewer: reviewer.into(),
            decision,
            comment: comment.into(),
            submitted_at: Utc::now(),
        }
    }
}

/// Pull request for merging branches.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    /// Pull request ID
    pub pr_id: Uuid,
    /// Pull request number (incremental)
    pub pr_number: u32,
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// Source branch
    pub source_branch: String,
    /// Target branch
    pub target_branch: String,
    /// Who created the PR
    pub author: String,
    /// Current status
    pub status: PullRequestStatus,
    /// Reviews
    pub reviews: Vec<PullRequestReview>,
    /// Commits included
    pub commits: Vec<Uuid>,
    /// When created
    pub created_at: DateTime<Utc>,
    /// When merged (if merged)
    pub merged_at: Option<DateTime<Utc>>,
    /// Who merged (if merged)
    pub merged_by: Option<String>,
}

impl PullRequest {
    /// Creates a new pull request.
    pub fn new(
        pr_number: u32,
        title: impl Into<String>,
        description: impl Into<String>,
        source_branch: impl Into<String>,
        target_branch: impl Into<String>,
        author: impl Into<String>,
    ) -> Self {
        Self {
            pr_id: Uuid::new_v4(),
            pr_number,
            title: title.into(),
            description: description.into(),
            source_branch: source_branch.into(),
            target_branch: target_branch.into(),
            author: author.into(),
            status: PullRequestStatus::Open,
            reviews: Vec::new(),
            commits: Vec::new(),
            created_at: Utc::now(),
            merged_at: None,
            merged_by: None,
        }
    }

    /// Adds a review to the pull request.
    pub fn add_review(&mut self, review: PullRequestReview) {
        self.reviews.push(review);
        // Update status based on reviews
        self.update_status();
    }

    /// Updates the status based on reviews.
    fn update_status(&mut self) {
        if self.status == PullRequestStatus::Merged || self.status == PullRequestStatus::Closed {
            return;
        }

        let approvals = self
            .reviews
            .iter()
            .filter(|r| r.decision == ReviewDecision::Approve)
            .count();
        let changes_requested = self
            .reviews
            .iter()
            .filter(|r| r.decision == ReviewDecision::RequestChanges)
            .count();

        if changes_requested > 0 {
            self.status = PullRequestStatus::ChangesRequested;
        } else if approvals > 0 {
            self.status = PullRequestStatus::Approved;
        } else if !self.reviews.is_empty() {
            self.status = PullRequestStatus::InReview;
        }
    }

    /// Checks if the PR is approved.
    pub fn is_approved(&self) -> bool {
        self.status == PullRequestStatus::Approved
    }

    /// Marks the PR as merged.
    pub fn mark_merged(&mut self, merged_by: impl Into<String>) {
        self.status = PullRequestStatus::Merged;
        self.merged_at = Some(Utc::now());
        self.merged_by = Some(merged_by.into());
    }

    /// Closes the PR without merging.
    pub fn close(&mut self) {
        self.status = PullRequestStatus::Closed;
    }
}

/// Field-level change tracking for blame.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldHistory {
    /// Field name
    pub field_name: String,
    /// Old value (serialized)
    pub old_value: Option<String>,
    /// New value (serialized)
    pub new_value: String,
    /// Commit that made the change
    pub commit_id: Uuid,
    /// Who made the change
    pub author: String,
    /// When the change was made
    pub timestamp: DateTime<Utc>,
    /// Commit message
    pub message: String,
}

impl FieldHistory {
    /// Creates a new field history entry.
    pub fn new(
        field_name: impl Into<String>,
        old_value: Option<String>,
        new_value: impl Into<String>,
        commit_id: Uuid,
        author: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            field_name: field_name.into(),
            old_value,
            new_value: new_value.into(),
            commit_id,
            author: author.into(),
            timestamp: Utc::now(),
            message: message.into(),
        }
    }
}

/// Blame information for a specific field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldBlame {
    /// Field name
    pub field_name: String,
    /// Current value
    pub current_value: String,
    /// Who last modified this field
    pub last_author: String,
    /// When last modified
    pub last_modified: DateTime<Utc>,
    /// Commit that last modified this field
    pub last_commit_id: Uuid,
    /// Complete history of this field
    pub history: Vec<FieldHistory>,
}

impl FieldBlame {
    /// Creates a new field blame.
    pub fn new(field_name: impl Into<String>, current_value: impl Into<String>) -> Self {
        Self {
            field_name: field_name.into(),
            current_value: current_value.into(),
            last_author: String::new(),
            last_modified: Utc::now(),
            last_commit_id: Uuid::nil(),
            history: Vec::new(),
        }
    }

    /// Adds a history entry.
    pub fn add_history(&mut self, history: FieldHistory) {
        self.last_author = history.author.clone();
        self.last_modified = history.timestamp;
        self.last_commit_id = history.commit_id;
        self.history.push(history);
    }

    /// Gets the number of times this field was modified.
    pub fn modification_count(&self) -> usize {
        self.history.len()
    }

    /// Gets all authors who modified this field.
    pub fn all_authors(&self) -> HashSet<String> {
        self.history.iter().map(|h| h.author.clone()).collect()
    }
}

/// Version control manager.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlManager {
    /// All branches
    branches: HashMap<String, Branch>,
    /// All commits
    commits: HashMap<Uuid, Commit>,
    /// All pull requests
    pull_requests: HashMap<Uuid, PullRequest>,
    /// Next PR number
    next_pr_number: u32,
    /// Field-level blame tracking
    field_blame: HashMap<String, HashMap<String, FieldBlame>>, // statute_id -> field_name -> blame
}

impl VersionControlManager {
    /// Creates a new version control manager.
    pub fn new() -> Self {
        let mut manager = Self {
            branches: HashMap::new(),
            commits: HashMap::new(),
            pull_requests: HashMap::new(),
            next_pr_number: 1,
            field_blame: HashMap::new(),
        };

        // Create the main branch
        let main_branch = Branch::new("main", "system").with_protected(true);
        manager.branches.insert("main".to_string(), main_branch);

        manager
    }

    /// Creates a new branch.
    pub fn create_branch(
        &mut self,
        name: impl Into<String>,
        parent: Option<String>,
        created_by: impl Into<String>,
    ) -> Result<&Branch, String> {
        let name = name.into();
        if self.branches.contains_key(&name) {
            return Err(format!("Branch '{}' already exists", name));
        }

        let branch = if let Some(parent_name) = parent {
            if !self.branches.contains_key(&parent_name) {
                return Err(format!("Parent branch '{}' does not exist", parent_name));
            }
            Branch::from_parent(name.clone(), parent_name, created_by)
        } else {
            Branch::new(name.clone(), created_by)
        };

        self.branches.insert(name.clone(), branch);
        Ok(self
            .branches
            .get(&name)
            .expect("invariant: branch was just inserted"))
    }

    /// Deletes a branch.
    pub fn delete_branch(&mut self, name: &str) -> Result<(), String> {
        if name == "main" {
            return Err("Cannot delete main branch".to_string());
        }

        if let Some(branch) = self.branches.get(name) {
            if branch.protected {
                return Err(format!("Branch '{}' is protected", name));
            }
        } else {
            return Err(format!("Branch '{}' does not exist", name));
        }

        self.branches.remove(name);
        Ok(())
    }

    /// Gets a branch.
    pub fn get_branch(&self, name: &str) -> Option<&Branch> {
        self.branches.get(name)
    }

    /// Gets a mutable reference to a branch.
    pub fn get_branch_mut(&mut self, name: &str) -> Option<&mut Branch> {
        self.branches.get_mut(name)
    }

    /// Lists all branches.
    pub fn list_branches(&self) -> Vec<&Branch> {
        self.branches.values().collect()
    }

    /// Creates a commit.
    pub fn commit(
        &mut self,
        branch_name: impl Into<String>,
        statute_id: impl Into<String>,
        statute_entry: StatuteEntry,
        message: impl Into<String>,
        author: impl Into<String>,
    ) -> Result<Uuid, String> {
        let branch_name = branch_name.into();
        if !self.branches.contains_key(&branch_name) {
            return Err(format!("Branch '{}' does not exist", branch_name));
        }

        let statute_id = statute_id.into();
        let mut commit = Commit::new(
            branch_name.clone(),
            statute_id.clone(),
            statute_entry,
            message,
            author,
        );

        // Set parent to current head
        if let Some(branch) = self.branches.get(&branch_name)
            && let Some(head) = branch.head_commit
        {
            commit = commit.with_parent(head);
        }

        let commit_id = commit.commit_id;
        self.commits.insert(commit_id, commit);

        // Update branch head
        if let Some(branch) = self.branches.get_mut(&branch_name) {
            branch.head_commit = Some(commit_id);
        }

        // Track field-level changes for blame
        self.track_field_changes(commit_id);

        Ok(commit_id)
    }

    /// Signs a commit.
    pub fn sign_commit(
        &mut self,
        commit_id: Uuid,
        signature: impl Into<String>,
    ) -> Result<(), String> {
        if let Some(commit) = self.commits.get_mut(&commit_id) {
            commit.signature = Some(signature.into());
            Ok(())
        } else {
            Err("Commit not found".to_string())
        }
    }

    /// Gets a commit.
    pub fn get_commit(&self, commit_id: Uuid) -> Option<&Commit> {
        self.commits.get(&commit_id)
    }

    /// Gets commits for a branch.
    pub fn get_branch_commits(&self, branch_name: &str) -> Vec<&Commit> {
        self.commits
            .values()
            .filter(|c| c.branch_name == branch_name)
            .collect()
    }

    /// Gets commit history for a branch (following parent chain).
    /// Returns commits in chronological order (oldest first).
    pub fn get_commit_history(&self, branch_name: &str) -> Vec<&Commit> {
        let mut history = Vec::new();
        if let Some(branch) = self.branches.get(branch_name)
            && let Some(head) = branch.head_commit
        {
            self.collect_commit_history(head, &mut history);
        }
        // History is collected in reverse order (newest first from recursion),
        // but we want chronological order (oldest first)
        history
    }

    fn collect_commit_history<'a>(&'a self, commit_id: Uuid, history: &mut Vec<&'a Commit>) {
        if let Some(commit) = self.commits.get(&commit_id) {
            for parent_id in &commit.parent_commits {
                self.collect_commit_history(*parent_id, history);
            }
            history.push(commit);
        }
    }

    /// Merges a source branch into a target branch.
    pub fn merge_branch(
        &mut self,
        source_branch: &str,
        target_branch: &str,
        author: impl Into<String>,
    ) -> MergeBranchResult {
        // Check if branches exist
        if !self.branches.contains_key(source_branch) {
            return MergeBranchResult {
                merge_commit_id: None,
                conflicts: Vec::new(),
                success: false,
                message: format!("Source branch '{}' does not exist", source_branch),
            };
        }
        if !self.branches.contains_key(target_branch) {
            return MergeBranchResult {
                merge_commit_id: None,
                conflicts: Vec::new(),
                success: false,
                message: format!("Target branch '{}' does not exist", target_branch),
            };
        }

        // Get head commits
        let source_head = self.branches.get(source_branch).and_then(|b| b.head_commit);
        let target_head = self.branches.get(target_branch).and_then(|b| b.head_commit);

        if source_head.is_none() {
            return MergeBranchResult {
                merge_commit_id: None,
                conflicts: Vec::new(),
                success: false,
                message: "Source branch has no commits".to_string(),
            };
        }

        // Get commits
        let source_head_id =
            source_head.expect("invariant: source_head was checked non-None above");
        let source_commit = self
            .commits
            .get(&source_head_id)
            .expect("invariant: source_head refers to existing commit");
        let target_commit = target_head.and_then(|id| self.commits.get(&id));

        // Detect conflicts
        let conflicts = if let Some(target_commit) = target_commit {
            self.detect_conflicts(&source_commit.statute_entry, &target_commit.statute_entry)
        } else {
            Vec::new()
        };

        if !conflicts.is_empty() {
            return MergeBranchResult {
                merge_commit_id: None,
                conflicts,
                success: false,
                message: "Merge conflicts detected".to_string(),
            };
        }

        // Create merge commit
        let message = format!("Merge branch '{}' into '{}'", source_branch, target_branch);
        let statute_id = source_commit.statute_id.clone();
        let statute_entry = source_commit.statute_entry.clone();

        match self.commit(target_branch, statute_id, statute_entry, message, author) {
            Ok(merge_commit_id) => {
                // Add source branch head as second parent to the merge commit
                // (First parent is already set by commit() to the target branch head)
                if let Some(commit) = self.commits.get_mut(&merge_commit_id) {
                    commit.parent_commits.push(source_head_id);
                }

                MergeBranchResult {
                    merge_commit_id: Some(merge_commit_id),
                    conflicts: Vec::new(),
                    success: true,
                    message: "Merge successful".to_string(),
                }
            }
            Err(e) => MergeBranchResult {
                merge_commit_id: None,
                conflicts: Vec::new(),
                success: false,
                message: format!("Failed to create merge commit: {}", e),
            },
        }
    }

    fn detect_conflicts(
        &self,
        source_entry: &StatuteEntry,
        target_entry: &StatuteEntry,
    ) -> Vec<BranchMergeConflict> {
        let mut conflicts = Vec::new();

        // Check title
        if source_entry.statute.title != target_entry.statute.title {
            conflicts.push(BranchMergeConflict {
                field_name: "title".to_string(),
                source_value: source_entry.statute.title.clone(),
                target_value: target_entry.statute.title.clone(),
                base_value: None,
            });
        }

        // Check status
        if source_entry.status != target_entry.status {
            conflicts.push(BranchMergeConflict {
                field_name: "status".to_string(),
                source_value: format!("{:?}", source_entry.status),
                target_value: format!("{:?}", target_entry.status),
                base_value: None,
            });
        }

        // Check jurisdiction
        if source_entry.jurisdiction != target_entry.jurisdiction {
            conflicts.push(BranchMergeConflict {
                field_name: "jurisdiction".to_string(),
                source_value: source_entry.jurisdiction.clone(),
                target_value: target_entry.jurisdiction.clone(),
                base_value: None,
            });
        }

        conflicts
    }

    /// Creates a pull request.
    pub fn create_pull_request(
        &mut self,
        title: impl Into<String>,
        description: impl Into<String>,
        source_branch: impl Into<String>,
        target_branch: impl Into<String>,
        author: impl Into<String>,
    ) -> Result<Uuid, String> {
        let source_branch = source_branch.into();
        let target_branch = target_branch.into();

        if !self.branches.contains_key(&source_branch) {
            return Err(format!("Source branch '{}' does not exist", source_branch));
        }
        if !self.branches.contains_key(&target_branch) {
            return Err(format!("Target branch '{}' does not exist", target_branch));
        }

        let pr = PullRequest::new(
            self.next_pr_number,
            title,
            description,
            source_branch,
            target_branch,
            author,
        );
        let pr_id = pr.pr_id;
        self.next_pr_number += 1;

        self.pull_requests.insert(pr_id, pr);
        Ok(pr_id)
    }

    /// Adds a review to a pull request.
    pub fn add_review(
        &mut self,
        pr_id: Uuid,
        reviewer: impl Into<String>,
        decision: ReviewDecision,
        comment: impl Into<String>,
    ) -> Result<(), String> {
        if let Some(pr) = self.pull_requests.get_mut(&pr_id) {
            let review = PullRequestReview::new(pr_id, reviewer, decision, comment);
            pr.add_review(review);
            Ok(())
        } else {
            Err("Pull request not found".to_string())
        }
    }

    /// Merges a pull request.
    pub fn merge_pull_request(
        &mut self,
        pr_id: Uuid,
        merged_by: impl Into<String>,
    ) -> Result<MergeBranchResult, String> {
        let (source_branch, target_branch, is_approved) = {
            let pr = self
                .pull_requests
                .get(&pr_id)
                .ok_or("Pull request not found")?;

            if !pr.is_approved() {
                return Err("Pull request is not approved".to_string());
            }

            (
                pr.source_branch.clone(),
                pr.target_branch.clone(),
                pr.is_approved(),
            )
        };

        if !is_approved {
            return Err("Pull request is not approved".to_string());
        }

        let merged_by_str = merged_by.into();
        let result = self.merge_branch(&source_branch, &target_branch, merged_by_str.clone());

        if result.success
            && let Some(pr) = self.pull_requests.get_mut(&pr_id)
        {
            pr.mark_merged(merged_by_str);
        }

        Ok(result)
    }

    /// Gets a pull request.
    pub fn get_pull_request(&self, pr_id: Uuid) -> Option<&PullRequest> {
        self.pull_requests.get(&pr_id)
    }

    /// Lists all pull requests.
    pub fn list_pull_requests(&self) -> Vec<&PullRequest> {
        self.pull_requests.values().collect()
    }

    /// Lists open pull requests.
    pub fn list_open_pull_requests(&self) -> Vec<&PullRequest> {
        self.pull_requests
            .values()
            .filter(|pr| {
                pr.status == PullRequestStatus::Open
                    || pr.status == PullRequestStatus::InReview
                    || pr.status == PullRequestStatus::Approved
            })
            .collect()
    }

    /// Closes a pull request without merging.
    pub fn close_pull_request(&mut self, pr_id: Uuid) -> Result<(), String> {
        if let Some(pr) = self.pull_requests.get_mut(&pr_id) {
            pr.close();
            Ok(())
        } else {
            Err("Pull request not found".to_string())
        }
    }

    /// Tracks field-level changes for blame.
    fn track_field_changes(&mut self, commit_id: Uuid) {
        if let Some(commit) = self.commits.get(&commit_id).cloned() {
            let statute_id = commit.statute_id.clone();

            // Track title changes
            self.track_field(
                &statute_id,
                "title",
                &commit.statute_entry.statute.title,
                commit_id,
                &commit.author,
                &commit.message,
            );

            // Track jurisdiction changes
            self.track_field(
                &statute_id,
                "jurisdiction",
                &commit.statute_entry.jurisdiction,
                commit_id,
                &commit.author,
                &commit.message,
            );

            // Track status changes
            self.track_field(
                &statute_id,
                "status",
                &format!("{:?}", commit.statute_entry.status),
                commit_id,
                &commit.author,
                &commit.message,
            );
        }
    }

    fn track_field(
        &mut self,
        statute_id: &str,
        field_name: &str,
        new_value: &str,
        commit_id: Uuid,
        author: &str,
        message: &str,
    ) {
        let statute_blame = self.field_blame.entry(statute_id.to_string()).or_default();

        let old_value = statute_blame
            .get(field_name)
            .map(|blame| blame.current_value.clone());

        let history = FieldHistory::new(
            field_name,
            old_value.clone(),
            new_value,
            commit_id,
            author,
            message,
        );

        let blame = statute_blame
            .entry(field_name.to_string())
            .or_insert_with(|| FieldBlame::new(field_name, new_value));

        blame.add_history(history);
        blame.current_value = new_value.to_string();
    }

    /// Gets blame information for a field.
    pub fn get_field_blame(&self, statute_id: &str, field_name: &str) -> Option<&FieldBlame> {
        self.field_blame
            .get(statute_id)
            .and_then(|fields| fields.get(field_name))
    }

    /// Gets all blame information for a statute.
    pub fn get_statute_blame(&self, statute_id: &str) -> Option<&HashMap<String, FieldBlame>> {
        self.field_blame.get(statute_id)
    }
}

impl Default for VersionControlManager {
    fn default() -> Self {
        Self::new()
    }
}
