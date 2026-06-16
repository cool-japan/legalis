//! Legal template versioning.
//!
//! [`VersionedTemplate`] is a legal document / clause template with an immutable,
//! monotonically-numbered revision history. Each [`TemplateRevision`] captures
//! the full body at that point plus authoring metadata (author, message,
//! timestamp). The store can:
//!
//! * commit a new revision, returning its number;
//! * retrieve any historical revision or the latest;
//! * compute a **line-level diff** between any two revisions using a
//!   longest-common-subsequence backtrace (the same core algorithm `diff(1)`
//!   uses), yielding context / added / removed [`DiffLine`]s and summary counts;
//! * roll back by committing the body of an earlier revision as a new revision.
//!
//! [`TemplateRepository`] holds many named templates. Everything is deterministic
//! and offline; this is distinct from the prompt-oriented `templates` module.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A single immutable revision of a template.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateRevision {
    /// 1-based revision number (monotonic within a template).
    pub version: u32,
    /// Full template body at this revision.
    pub body: String,
    /// Author who committed the revision.
    pub author: String,
    /// Commit message describing the change.
    pub message: String,
    /// When the revision was committed.
    pub timestamp: DateTime<Utc>,
}

impl TemplateRevision {
    /// Returns the body split into lines (used by the diff engine).
    pub fn lines(&self) -> Vec<&str> {
        self.body.lines().collect()
    }
}

/// The kind of change a [`DiffLine`] represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffOp {
    /// Line present unchanged in both revisions.
    Context,
    /// Line added in the new revision.
    Added,
    /// Line removed from the old revision.
    Removed,
}

/// A single line in a diff.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiffLine {
    /// The change kind.
    pub op: DiffOp,
    /// The line content.
    pub text: String,
}

/// The result of diffing two revisions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateDiff {
    /// Source (old) version number.
    pub from_version: u32,
    /// Target (new) version number.
    pub to_version: u32,
    /// Ordered diff lines.
    pub lines: Vec<DiffLine>,
    /// Number of added lines.
    pub added: usize,
    /// Number of removed lines.
    pub removed: usize,
    /// Number of unchanged context lines.
    pub unchanged: usize,
}

impl TemplateDiff {
    /// Whether the two revisions are identical (no additions or removals).
    pub fn is_empty(&self) -> bool {
        self.added == 0 && self.removed == 0
    }

    /// Renders the diff in a unified-style text format (`+`/`-`/` ` prefixes).
    pub fn to_unified(&self) -> String {
        use std::fmt::Write as _;
        let mut out = String::new();
        let _ = writeln!(out, "--- v{}", self.from_version);
        let _ = writeln!(out, "+++ v{}", self.to_version);
        for line in &self.lines {
            let prefix = match line.op {
                DiffOp::Context => ' ',
                DiffOp::Added => '+',
                DiffOp::Removed => '-',
            };
            let _ = writeln!(out, "{prefix}{}", line.text);
        }
        out
    }
}

/// A versioned legal template with full revision history.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionedTemplate {
    /// Stable template name / key.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Immutable revision history (ordered by version).
    revisions: Vec<TemplateRevision>,
}

impl VersionedTemplate {
    /// Creates a template with an initial revision (version 1).
    pub fn new(
        name: impl Into<String>,
        body: impl Into<String>,
        author: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        let revision = TemplateRevision {
            version: 1,
            body: body.into(),
            author: author.into(),
            message: message.into(),
            timestamp: Utc::now(),
        };
        Self {
            name: name.into(),
            description: None,
            revisions: vec![revision],
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Commits a new revision, returning its version number.
    ///
    /// The timestamp is captured automatically. Identical-body commits are still
    /// recorded (history is append-only and never collapses).
    pub fn commit(
        &mut self,
        body: impl Into<String>,
        author: impl Into<String>,
        message: impl Into<String>,
    ) -> u32 {
        let version = self.latest_version() + 1;
        self.revisions.push(TemplateRevision {
            version,
            body: body.into(),
            author: author.into(),
            message: message.into(),
            timestamp: Utc::now(),
        });
        version
    }

    /// The number of the latest revision.
    pub fn latest_version(&self) -> u32 {
        self.revisions.last().map(|r| r.version).unwrap_or(0)
    }

    /// The latest revision.
    pub fn latest(&self) -> &TemplateRevision {
        // Invariant: at least one revision always exists (set in `new`).
        self.revisions.last().unwrap_or_else(|| &self.revisions[0])
    }

    /// The current (latest) body.
    pub fn current_body(&self) -> &str {
        &self.latest().body
    }

    /// Looks up a specific revision by version number.
    pub fn revision(&self, version: u32) -> Option<&TemplateRevision> {
        self.revisions.iter().find(|r| r.version == version)
    }

    /// The full ordered revision history.
    pub fn history(&self) -> &[TemplateRevision] {
        &self.revisions
    }

    /// The number of revisions.
    pub fn revision_count(&self) -> usize {
        self.revisions.len()
    }

    /// Computes a line-level diff between two revisions.
    ///
    /// Returns `None` if either version is unknown.
    pub fn diff(&self, from_version: u32, to_version: u32) -> Option<TemplateDiff> {
        let from = self.revision(from_version)?;
        let to = self.revision(to_version)?;
        Some(diff_lines(
            from_version,
            to_version,
            &from.lines(),
            &to.lines(),
        ))
    }

    /// Diffs the two most recent revisions, if at least two exist.
    pub fn diff_latest(&self) -> Option<TemplateDiff> {
        let n = self.revisions.len();
        if n < 2 {
            return None;
        }
        let from = self.revisions[n - 2].version;
        let to = self.revisions[n - 1].version;
        self.diff(from, to)
    }

    /// Rolls back to an earlier revision by committing its body as a new
    /// revision (history is preserved). Returns the new version number, or
    /// `None` if the target version is unknown.
    pub fn rollback(&mut self, to_version: u32, author: impl Into<String>) -> Option<u32> {
        let body = self.revision(to_version)?.body.clone();
        let message = format!("rollback to v{to_version}");
        Some(self.commit(body, author, message))
    }
}

/// A repository of named versioned templates.
#[derive(Debug, Clone, Default)]
pub struct TemplateRepository {
    templates: BTreeMap<String, VersionedTemplate>,
}

impl TemplateRepository {
    /// Creates an empty repository.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new template. Errors if the name already exists.
    pub fn register(&mut self, template: VersionedTemplate) -> Result<(), String> {
        if self.templates.contains_key(&template.name) {
            return Err(format!("template already exists: {}", template.name));
        }
        self.templates.insert(template.name.clone(), template);
        Ok(())
    }

    /// Creates and registers a template from an initial body in one step.
    pub fn create(
        &mut self,
        name: impl Into<String>,
        body: impl Into<String>,
        author: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<(), String> {
        self.register(VersionedTemplate::new(name, body, author, message))
    }

    /// Commits a new revision to an existing template, returning its version.
    pub fn commit(
        &mut self,
        name: &str,
        body: impl Into<String>,
        author: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<u32, String> {
        let template = self
            .templates
            .get_mut(name)
            .ok_or_else(|| format!("unknown template: {name}"))?;
        Ok(template.commit(body, author, message))
    }

    /// Borrows a template by name.
    pub fn get(&self, name: &str) -> Option<&VersionedTemplate> {
        self.templates.get(name)
    }

    /// Mutably borrows a template by name.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut VersionedTemplate> {
        self.templates.get_mut(name)
    }

    /// The number of registered templates.
    pub fn len(&self) -> usize {
        self.templates.len()
    }

    /// Whether the repository is empty.
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }

    /// Lists template names (sorted).
    pub fn names(&self) -> Vec<&str> {
        self.templates.keys().map(|s| s.as_str()).collect()
    }
}

/// Computes a line-level diff between two slices of lines via an LCS backtrace.
///
/// The longest common subsequence is found with the classic `O(n*m)` dynamic
/// program, then the two sequences are walked together emitting context lines
/// for the LCS, removals for old-only lines and additions for new-only lines -
/// the same shape a unified diff produces.
fn diff_lines(from_version: u32, to_version: u32, old: &[&str], new: &[&str]) -> TemplateDiff {
    let n = old.len();
    let m = new.len();

    // LCS length table.
    let mut lcs = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            lcs[i][j] = if old[i] == new[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }

    // Backtrace.
    let mut lines = Vec::new();
    let mut added = 0usize;
    let mut removed = 0usize;
    let mut unchanged = 0usize;
    let mut i = 0usize;
    let mut j = 0usize;
    while i < n && j < m {
        if old[i] == new[j] {
            lines.push(DiffLine {
                op: DiffOp::Context,
                text: old[i].to_string(),
            });
            unchanged += 1;
            i += 1;
            j += 1;
        } else if lcs[i + 1][j] >= lcs[i][j + 1] {
            lines.push(DiffLine {
                op: DiffOp::Removed,
                text: old[i].to_string(),
            });
            removed += 1;
            i += 1;
        } else {
            lines.push(DiffLine {
                op: DiffOp::Added,
                text: new[j].to_string(),
            });
            added += 1;
            j += 1;
        }
    }
    while i < n {
        lines.push(DiffLine {
            op: DiffOp::Removed,
            text: old[i].to_string(),
        });
        removed += 1;
        i += 1;
    }
    while j < m {
        lines.push(DiffLine {
            op: DiffOp::Added,
            text: new[j].to_string(),
        });
        added += 1;
        j += 1;
    }

    TemplateDiff {
        from_version,
        to_version,
        lines,
        added,
        removed,
        unchanged,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_revision() {
        let template = VersionedTemplate::new(
            "nda",
            "Section 1. Confidentiality.",
            "Smith",
            "initial draft",
        );
        assert_eq!(template.latest_version(), 1);
        assert_eq!(template.revision_count(), 1);
        assert_eq!(template.current_body(), "Section 1. Confidentiality.");
        assert_eq!(template.latest().author, "Smith");
    }

    #[test]
    fn test_commit_increments_version() {
        let mut template = VersionedTemplate::new("nda", "v1 body", "Smith", "init");
        let v2 = template.commit("v2 body", "Jones", "revise");
        assert_eq!(v2, 2);
        assert_eq!(template.latest_version(), 2);
        assert_eq!(template.current_body(), "v2 body");
        assert_eq!(
            template.revision(1).map(|r| r.body.as_str()),
            Some("v1 body")
        );
        assert_eq!(template.history().len(), 2);
    }

    #[test]
    fn test_diff_added_removed_context() {
        let mut template =
            VersionedTemplate::new("agreement", "Line A\nLine B\nLine C", "Smith", "init");
        template.commit("Line A\nLine B2\nLine C\nLine D", "Jones", "edit");
        let diff = template.diff(1, 2).expect("diff exists");
        assert_eq!(diff.from_version, 1);
        assert_eq!(diff.to_version, 2);
        // "Line A" and "Line C" are context; "Line B"->removed, "Line B2"->added,
        // "Line D"->added.
        assert_eq!(diff.unchanged, 2);
        assert_eq!(diff.removed, 1);
        assert_eq!(diff.added, 2);
        assert!(!diff.is_empty());

        let unified = diff.to_unified();
        assert!(unified.contains("--- v1"));
        assert!(unified.contains("+++ v2"));
        assert!(unified.contains("-Line B"));
        assert!(unified.contains("+Line B2"));
        assert!(unified.contains("+Line D"));
        assert!(unified.contains(" Line A"));
    }

    #[test]
    fn test_diff_identical_is_empty() {
        let mut template = VersionedTemplate::new("t", "same\nbody", "a", "init");
        template.commit("same\nbody", "a", "noop");
        let diff = template.diff(1, 2).expect("diff");
        assert!(diff.is_empty());
        assert_eq!(diff.added, 0);
        assert_eq!(diff.removed, 0);
        assert_eq!(diff.unchanged, 2);
    }

    #[test]
    fn test_diff_latest_and_unknown_version() {
        let mut template = VersionedTemplate::new("t", "one", "a", "init");
        assert!(template.diff_latest().is_none()); // only one revision
        template.commit("one\ntwo", "a", "add line");
        let diff = template.diff_latest().expect("latest diff");
        assert_eq!(diff.added, 1);
        assert!(template.diff(1, 99).is_none());
    }

    #[test]
    fn test_rollback() {
        let mut template = VersionedTemplate::new("t", "original", "a", "init");
        template.commit("changed", "b", "change");
        assert_eq!(template.current_body(), "changed");
        let v3 = template.rollback(1, "c").expect("rollback");
        assert_eq!(v3, 3);
        assert_eq!(template.current_body(), "original");
        assert_eq!(template.latest().message, "rollback to v1");
        // History retained: 3 revisions.
        assert_eq!(template.revision_count(), 3);
        assert!(template.rollback(99, "c").is_none());
    }

    #[test]
    fn test_repository() {
        let mut repo = TemplateRepository::new();
        repo.create("nda", "confidential", "Smith", "init")
            .expect("create");
        assert_eq!(repo.len(), 1);
        // Duplicate name rejected.
        assert!(repo.create("nda", "x", "y", "z").is_err());
        // Commit through the repo.
        let v2 = repo
            .commit("nda", "confidential v2", "Jones", "edit")
            .expect("commit");
        assert_eq!(v2, 2);
        assert_eq!(repo.get("nda").map(|t| t.latest_version()), Some(2));
        assert!(repo.commit("missing", "x", "y", "z").is_err());
        assert_eq!(repo.names(), vec!["nda"]);
    }

    #[test]
    fn test_diff_full_rewrite() {
        let mut template = VersionedTemplate::new("t", "alpha\nbeta", "a", "init");
        template.commit("gamma\ndelta", "a", "rewrite");
        let diff = template.diff(1, 2).expect("diff");
        assert_eq!(diff.unchanged, 0);
        assert_eq!(diff.removed, 2);
        assert_eq!(diff.added, 2);
    }
}
