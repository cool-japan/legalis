//! Hierarchical statute relationships and amendment tracking.

use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents an amendment to a statute.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Amendment {
    /// Amendment identifier
    pub id: String,
    /// Amending statute ID (the statute that makes the change)
    pub amending_statute_id: String,
    /// Description of the amendment
    pub description: String,
    /// Timestamp when the amendment was enacted
    pub enacted_at: DateTime<Utc>,
    /// Amendment type
    pub amendment_type: AmendmentType,
}

impl Amendment {
    /// Creates a new Amendment.
    pub fn new(
        id: impl Into<String>,
        amending_statute_id: impl Into<String>,
        description: impl Into<String>,
        amendment_type: AmendmentType,
    ) -> Self {
        Self {
            id: id.into(),
            amending_statute_id: amending_statute_id.into(),
            description: description.into(),
            enacted_at: Utc::now(),
            amendment_type,
        }
    }

    /// Sets the enactment timestamp.
    pub fn with_enacted_at(mut self, enacted_at: DateTime<Utc>) -> Self {
        self.enacted_at = enacted_at;
        self
    }
}

/// Types of amendments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AmendmentType {
    /// Text modification (rewording)
    Modification,
    /// Addition of new provisions
    Addition,
    /// Deletion/removal of provisions
    Deletion,
    /// Substitution (replace with new text)
    Substitution,
    /// Clarification (no substantive change)
    Clarification,
    /// Renumbering or reorganization
    Reorganization,
}

impl std::fmt::Display for AmendmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Modification => write!(f, "Modification"),
            Self::Addition => write!(f, "Addition"),
            Self::Deletion => write!(f, "Deletion"),
            Self::Substitution => write!(f, "Substitution"),
            Self::Clarification => write!(f, "Clarification"),
            Self::Reorganization => write!(f, "Reorganization"),
        }
    }
}

/// Hierarchical relationship data for a statute.
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StatuteHierarchy {
    /// Parent statute ID (if this is a sub-section or derived statute)
    pub parent_id: Option<String>,
    /// Child statute IDs (sub-sections or derived statutes)
    pub child_ids: Vec<String>,
    /// Statute IDs that this statute supersedes (replaces)
    pub supersedes: Vec<String>,
    /// Statute ID that supersedes this one (if any)
    pub superseded_by: Option<String>,
    /// List of amendments made to this statute
    pub amendments: Vec<Amendment>,
    /// Cross-references to related statutes
    pub cross_references: Vec<String>,
}

impl StatuteHierarchy {
    /// Creates a new StatuteHierarchy with no relationships.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the parent statute ID.
    pub fn with_parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    /// Adds a child statute ID.
    pub fn with_child(mut self, child_id: impl Into<String>) -> Self {
        self.child_ids.push(child_id.into());
        self
    }

    /// Adds a superseded statute ID.
    pub fn with_supersedes(mut self, statute_id: impl Into<String>) -> Self {
        self.supersedes.push(statute_id.into());
        self
    }

    /// Sets the statute that supersedes this one.
    pub fn with_superseded_by(mut self, statute_id: impl Into<String>) -> Self {
        self.superseded_by = Some(statute_id.into());
        self
    }

    /// Adds an amendment.
    pub fn with_amendment(mut self, amendment: Amendment) -> Self {
        self.amendments.push(amendment);
        self
    }

    /// Adds a cross-reference.
    pub fn with_cross_reference(mut self, statute_id: impl Into<String>) -> Self {
        self.cross_references.push(statute_id.into());
        self
    }

    /// Returns true if this statute has a parent.
    pub fn has_parent(&self) -> bool {
        self.parent_id.is_some()
    }

    /// Returns true if this statute has children.
    pub fn has_children(&self) -> bool {
        !self.child_ids.is_empty()
    }

    /// Returns true if this statute supersedes others.
    pub fn supersedes_others(&self) -> bool {
        !self.supersedes.is_empty()
    }

    /// Returns true if this statute has been superseded.
    pub fn is_superseded(&self) -> bool {
        self.superseded_by.is_some()
    }

    /// Returns true if this statute has amendments.
    pub fn has_amendments(&self) -> bool {
        !self.amendments.is_empty()
    }

    /// Gets the number of amendments.
    pub fn amendment_count(&self) -> usize {
        self.amendments.len()
    }

    /// Gets the most recent amendment.
    pub fn latest_amendment(&self) -> Option<&Amendment> {
        self.amendments
            .iter()
            .max_by_key(|a| a.enacted_at)
    }

    /// Checks if this statute is a root (no parent).
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    /// Checks if this statute is a leaf (no children).
    pub fn is_leaf(&self) -> bool {
        self.child_ids.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amendment_creation() {
        let amendment = Amendment::new(
            "amend-1",
            "statute-2",
            "Updated age requirement",
            AmendmentType::Modification,
        );

        assert_eq!(amendment.id, "amend-1");
        assert_eq!(amendment.amending_statute_id, "statute-2");
        assert_eq!(amendment.description, "Updated age requirement");
        assert_eq!(amendment.amendment_type, AmendmentType::Modification);
    }

    #[test]
    fn test_statute_hierarchy_builder() {
        let hierarchy = StatuteHierarchy::new()
            .with_parent("parent-statute")
            .with_child("child-1")
            .with_child("child-2")
            .with_supersedes("old-statute-1")
            .with_cross_reference("related-statute-1");

        assert_eq!(hierarchy.parent_id, Some("parent-statute".to_string()));
        assert_eq!(hierarchy.child_ids.len(), 2);
        assert_eq!(hierarchy.supersedes.len(), 1);
        assert_eq!(hierarchy.cross_references.len(), 1);
    }

    #[test]
    fn test_statute_hierarchy_predicates() {
        let mut hierarchy = StatuteHierarchy::new();
        assert!(hierarchy.is_root());
        assert!(hierarchy.is_leaf());
        assert!(!hierarchy.has_parent());
        assert!(!hierarchy.has_children());

        hierarchy = hierarchy.with_parent("parent");
        assert!(!hierarchy.is_root());
        assert!(hierarchy.has_parent());

        hierarchy = hierarchy.with_child("child");
        assert!(!hierarchy.is_leaf());
        assert!(hierarchy.has_children());
    }

    #[test]
    fn test_latest_amendment() {
        let hierarchy = StatuteHierarchy::new()
            .with_amendment(Amendment::new(
                "amend-1",
                "statute-1",
                "First amendment",
                AmendmentType::Modification,
            ))
            .with_amendment(Amendment::new(
                "amend-2",
                "statute-2",
                "Second amendment",
                AmendmentType::Addition,
            ));

        let latest = hierarchy.latest_amendment();
        assert!(latest.is_some());
        assert!(latest.unwrap().id == "amend-1" || latest.unwrap().id == "amend-2");
    }
}
