//! Changelog Generation
//!
//! This module provides automatic changelog generation including:
//! - Change type categorization
//! - Version tagging
//! - Markdown/HTML output
//! - Release notes generation

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error types for changelog operations
#[derive(Debug, Error)]
pub enum ChangelogError {
    #[error("Changelog error: {0}")]
    Error(String),

    #[error("Invalid version: {0}")]
    InvalidVersion(String),

    #[error("Generation error: {0}")]
    GenerationError(String),
}

/// Result type for changelog operations
pub type ChangelogResult<T> = Result<T, ChangelogError>;

/// Change type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    /// New features
    Added,

    /// Changes in existing functionality
    Changed,

    /// Deprecated features
    Deprecated,

    /// Removed features
    Removed,

    /// Bug fixes
    Fixed,

    /// Security fixes
    Security,
}

impl std::fmt::Display for ChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeType::Added => write!(f, "Added"),
            ChangeType::Changed => write!(f, "Changed"),
            ChangeType::Deprecated => write!(f, "Deprecated"),
            ChangeType::Removed => write!(f, "Removed"),
            ChangeType::Fixed => write!(f, "Fixed"),
            ChangeType::Security => write!(f, "Security"),
        }
    }
}

/// Individual change entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeEntry {
    /// Change type
    pub change_type: ChangeType,

    /// Description
    pub description: String,

    /// Related issue/PR numbers
    pub references: Vec<String>,

    /// Author
    pub author: Option<String>,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Breaking change flag
    pub breaking: bool,
}

impl ChangeEntry {
    /// Create a new change entry
    pub fn new(change_type: ChangeType, description: String) -> Self {
        Self {
            change_type,
            description,
            references: Vec::new(),
            author: None,
            timestamp: Utc::now(),
            breaking: false,
        }
    }

    /// Add a reference (e.g., issue or PR number)
    pub fn with_reference(mut self, reference: String) -> Self {
        self.references.push(reference);
        self
    }

    /// Set author
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// Mark as breaking change
    pub fn as_breaking(mut self) -> Self {
        self.breaking = true;
        self
    }
}

/// Version release information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    /// Version number
    pub version: String,

    /// Release date
    pub date: DateTime<Utc>,

    /// Release title
    pub title: Option<String>,

    /// Changes in this release
    pub changes: Vec<ChangeEntry>,

    /// Release notes
    pub notes: Option<String>,
}

impl Release {
    /// Create a new release
    pub fn new(version: String) -> Self {
        Self {
            version,
            date: Utc::now(),
            title: None,
            changes: Vec::new(),
            notes: None,
        }
    }

    /// Set title
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Add change
    pub fn with_change(mut self, change: ChangeEntry) -> Self {
        self.changes.push(change);
        self
    }

    /// Set release notes
    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }

    /// Get changes by type
    pub fn changes_by_type(&self, change_type: ChangeType) -> Vec<&ChangeEntry> {
        self.changes
            .iter()
            .filter(|c| c.change_type == change_type)
            .collect()
    }

    /// Check if release has breaking changes
    pub fn has_breaking_changes(&self) -> bool {
        self.changes.iter().any(|c| c.breaking)
    }
}

/// Changelog containing multiple releases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Changelog {
    /// Project name
    pub project_name: String,

    /// Project description
    pub description: Option<String>,

    /// Releases (newest first)
    pub releases: Vec<Release>,
}

impl Changelog {
    /// Create a new changelog
    pub fn new(project_name: String) -> Self {
        Self {
            project_name,
            description: None,
            releases: Vec::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Add release
    pub fn add_release(&mut self, release: Release) {
        self.releases.push(release);
        self.releases.sort_by(|a, b| b.date.cmp(&a.date));
    }

    /// Get latest release
    pub fn latest_release(&self) -> Option<&Release> {
        self.releases.first()
    }

    /// Generate markdown output
    pub fn to_markdown(&self) -> String {
        let mut output = String::new();

        // Header
        output.push_str("# Changelog\n\n");
        output.push_str(&format!("## {}\n\n", self.project_name));

        if let Some(desc) = &self.description {
            output.push_str(&format!("{}\n\n", desc));
        }

        output.push_str("All notable changes to this project will be documented in this file.\n\n");

        // Releases
        for release in &self.releases {
            output.push_str(&self.release_to_markdown(release));
            output.push('\n');
        }

        output
    }

    fn release_to_markdown(&self, release: &Release) -> String {
        let mut output = String::new();

        // Version header
        output.push_str(&format!(
            "## [{}] - {}\n\n",
            release.version,
            release.date.format("%Y-%m-%d")
        ));

        if let Some(title) = &release.title {
            output.push_str(&format!("### {}\n\n", title));
        }

        // Breaking changes warning
        if release.has_breaking_changes() {
            output.push_str("⚠️ **BREAKING CHANGES** in this release\n\n");
        }

        // Release notes
        if let Some(notes) = &release.notes {
            output.push_str(&format!("{}\n\n", notes));
        }

        // Changes by type
        for change_type in &[
            ChangeType::Security,
            ChangeType::Added,
            ChangeType::Changed,
            ChangeType::Deprecated,
            ChangeType::Removed,
            ChangeType::Fixed,
        ] {
            let changes = release.changes_by_type(*change_type);
            if !changes.is_empty() {
                output.push_str(&format!("### {}\n\n", change_type));

                for change in changes {
                    output.push_str("- ");

                    if change.breaking {
                        output.push_str("**[BREAKING]** ");
                    }

                    output.push_str(&change.description);

                    if !change.references.is_empty() {
                        output.push_str(" (");
                        for (i, reference) in change.references.iter().enumerate() {
                            if i > 0 {
                                output.push_str(", ");
                            }
                            output.push_str(&format!("#{}", reference));
                        }
                        output.push(')');
                    }

                    if let Some(author) = &change.author {
                        output.push_str(&format!(" by @{}", author));
                    }

                    output.push('\n');
                }

                output.push('\n');
            }
        }

        output
    }

    /// Generate HTML output
    pub fn to_html(&self) -> String {
        let mut output = String::new();

        output.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        output.push_str(&format!(
            "    <title>{} - Changelog</title>\n",
            self.project_name
        ));
        output.push_str("    <style>\n");
        output.push_str("        body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }\n");
        output.push_str("        h1 { color: #333; }\n");
        output.push_str(
            "        h2 { color: #666; border-bottom: 1px solid #ddd; padding-bottom: 10px; }\n",
        );
        output.push_str("        h3 { color: #888; }\n");
        output.push_str("        .breaking { color: #d9534f; font-weight: bold; }\n");
        output.push_str("        .security { background-color: #d9534f; color: white; padding: 2px 6px; border-radius: 3px; }\n");
        output.push_str("        .added { color: #5cb85c; }\n");
        output.push_str("        .changed { color: #f0ad4e; }\n");
        output.push_str("        .deprecated { color: #888; }\n");
        output.push_str("        .removed { color: #d9534f; }\n");
        output.push_str("        .fixed { color: #5bc0de; }\n");
        output.push_str("    </style>\n");
        output.push_str("</head>\n<body>\n");

        output.push_str(&format!("    <h1>{} - Changelog</h1>\n", self.project_name));

        if let Some(desc) = &self.description {
            output.push_str(&format!("    <p>{}</p>\n", desc));
        }

        for release in &self.releases {
            output.push_str(&self.release_to_html(release));
        }

        output.push_str("</body>\n</html>");
        output
    }

    fn release_to_html(&self, release: &Release) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "    <h2>[{}] - {}</h2>\n",
            release.version,
            release.date.format("%Y-%m-%d")
        ));

        if release.has_breaking_changes() {
            output.push_str("    <p class=\"breaking\">⚠️ BREAKING CHANGES in this release</p>\n");
        }

        if let Some(notes) = &release.notes {
            output.push_str(&format!("    <p>{}</p>\n", notes));
        }

        for change_type in &[
            ChangeType::Security,
            ChangeType::Added,
            ChangeType::Changed,
            ChangeType::Deprecated,
            ChangeType::Removed,
            ChangeType::Fixed,
        ] {
            let changes = release.changes_by_type(*change_type);
            if !changes.is_empty() {
                let class = format!("{:?}", change_type).to_lowercase();
                output.push_str(&format!(
                    "    <h3 class=\"{}\">{}</h3>\n",
                    class, change_type
                ));
                output.push_str("    <ul>\n");

                for change in changes {
                    output.push_str("        <li>");

                    if change.breaking {
                        output.push_str("<span class=\"breaking\">[BREAKING]</span> ");
                    }

                    output.push_str(&change.description);
                    output.push_str("</li>\n");
                }

                output.push_str("    </ul>\n");
            }
        }

        output
    }

    /// Generate JSON output
    pub fn to_json(&self) -> ChangelogResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| ChangelogError::GenerationError(format!("JSON generation failed: {}", e)))
    }
}

/// Changelog builder for easy construction
pub struct ChangelogBuilder {
    changelog: Changelog,
    current_release: Option<Release>,
}

impl ChangelogBuilder {
    /// Create a new changelog builder
    pub fn new(project_name: String) -> Self {
        Self {
            changelog: Changelog::new(project_name),
            current_release: None,
        }
    }

    /// Set project description
    pub fn description(mut self, description: String) -> Self {
        self.changelog = self.changelog.with_description(description);
        self
    }

    /// Start a new release
    pub fn release(mut self, version: String) -> Self {
        // Finalize current release if exists
        if let Some(release) = self.current_release.take() {
            self.changelog.add_release(release);
        }

        self.current_release = Some(Release::new(version));
        self
    }

    /// Add a change to current release
    pub fn change(mut self, change_type: ChangeType, description: String) -> Self {
        if let Some(release) = &mut self.current_release {
            release
                .changes
                .push(ChangeEntry::new(change_type, description));
        }
        self
    }

    /// Add a breaking change to current release
    pub fn breaking_change(mut self, change_type: ChangeType, description: String) -> Self {
        if let Some(release) = &mut self.current_release {
            release
                .changes
                .push(ChangeEntry::new(change_type, description).as_breaking());
        }
        self
    }

    /// Build the changelog
    pub fn build(mut self) -> Changelog {
        // Finalize current release if exists
        if let Some(release) = self.current_release.take() {
            self.changelog.add_release(release);
        }

        self.changelog
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_entry() {
        let change = ChangeEntry::new(ChangeType::Added, "New feature".to_string())
            .with_reference("123".to_string())
            .with_author("alice".to_string())
            .as_breaking();

        assert_eq!(change.change_type, ChangeType::Added);
        assert_eq!(change.description, "New feature");
        assert_eq!(change.references.len(), 1);
        assert_eq!(change.author, Some("alice".to_string()));
        assert!(change.breaking);
    }

    #[test]
    fn test_release() {
        let mut release = Release::new("1.0.0".to_string());
        release = release.with_change(ChangeEntry::new(ChangeType::Added, "Feature 1".to_string()));
        release = release.with_change(ChangeEntry::new(ChangeType::Fixed, "Bug fix".to_string()));

        assert_eq!(release.version, "1.0.0");
        assert_eq!(release.changes.len(), 2);
        assert_eq!(release.changes_by_type(ChangeType::Added).len(), 1);
    }

    #[test]
    fn test_changelog_builder() {
        let changelog = ChangelogBuilder::new("Test Project".to_string())
            .description("A test project".to_string())
            .release("1.0.0".to_string())
            .change(ChangeType::Added, "Initial release".to_string())
            .build();

        assert_eq!(changelog.project_name, "Test Project");
        assert_eq!(changelog.releases.len(), 1);
        assert_eq!(changelog.releases[0].version, "1.0.0");
    }

    #[test]
    fn test_markdown_generation() {
        let changelog = ChangelogBuilder::new("Test Project".to_string())
            .release("1.0.0".to_string())
            .change(ChangeType::Added, "Feature 1".to_string())
            .change(ChangeType::Fixed, "Bug fix".to_string())
            .build();

        let markdown = changelog.to_markdown();
        assert!(markdown.contains("# Changelog"));
        assert!(markdown.contains("## Test Project"));
        assert!(markdown.contains("## [1.0.0]"));
        assert!(markdown.contains("### Added"));
        assert!(markdown.contains("### Fixed"));
    }

    #[test]
    fn test_html_generation() {
        let changelog = ChangelogBuilder::new("Test Project".to_string())
            .release("1.0.0".to_string())
            .change(ChangeType::Added, "Feature 1".to_string())
            .build();

        let html = changelog.to_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<h1>Test Project - Changelog</h1>"));
        assert!(html.contains("<h2>[1.0.0]"));
    }

    #[test]
    fn test_breaking_changes() {
        let changelog = ChangelogBuilder::new("Test Project".to_string())
            .release("2.0.0".to_string())
            .breaking_change(ChangeType::Changed, "API changed".to_string())
            .build();

        let release = changelog.releases.first().unwrap();
        assert!(release.has_breaking_changes());

        let markdown = changelog.to_markdown();
        assert!(markdown.contains("BREAKING CHANGES"));
    }

    #[test]
    fn test_json_generation() {
        let changelog = ChangelogBuilder::new("Test Project".to_string())
            .release("1.0.0".to_string())
            .change(ChangeType::Added, "Feature".to_string())
            .build();

        let json = changelog.to_json().unwrap();
        assert!(json.contains("Test Project"));
        assert!(json.contains("1.0.0"));
    }
}
