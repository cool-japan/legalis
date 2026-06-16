//! Due-diligence checklist automation.
//!
//! [`DueDiligenceChecklist`] is a configurable, stateful checklist: items carry
//! a [`Criticality`], a [`ItemStatus`], an assignee, evidence references and
//! optional dependencies on other items. The checklist tracks completion,
//! detects *gaps* (unresolved critical items and items blocked by unresolved
//! dependencies) and produces a [`DueDiligenceReport`]. A handful of standard
//! checklists (corporate acquisition, commercial lease) are provided as starting
//! points and are fully editable.

use super::Criticality;
use crate::Jurisdiction;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The status of a single checklist item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ItemStatus {
    /// Not yet started.
    #[default]
    NotStarted,
    /// Work is underway.
    InProgress,
    /// Completed satisfactorily.
    Complete,
    /// Determined not to apply.
    NotApplicable,
    /// Cannot proceed (e.g. awaiting third party).
    Blocked,
}

impl ItemStatus {
    /// Returns whether the item no longer represents outstanding work.
    pub fn is_resolved(&self) -> bool {
        matches!(self, ItemStatus::Complete | ItemStatus::NotApplicable)
    }

    /// Returns a human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            ItemStatus::NotStarted => "not started",
            ItemStatus::InProgress => "in progress",
            ItemStatus::Complete => "complete",
            ItemStatus::NotApplicable => "not applicable",
            ItemStatus::Blocked => "blocked",
        }
    }
}

/// A single due-diligence checklist item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChecklistItem {
    /// Stable identifier.
    pub id: String,
    /// What must be verified / obtained.
    pub description: String,
    /// Grouping (e.g. `corporate`, `financial`, `ip`).
    pub category: String,
    /// How critical the item is to the matter.
    pub criticality: Criticality,
    /// Current status.
    pub status: ItemStatus,
    /// Person responsible.
    pub assignee: Option<String>,
    /// Free-form notes.
    pub notes: Option<String>,
    /// Ids of items that must be resolved before this one can proceed.
    pub depends_on: Vec<String>,
    /// Evidence references gathered for the item.
    pub evidence: Vec<String>,
}

impl ChecklistItem {
    /// Creates a new item (defaults to `NotStarted`, `Medium` criticality).
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        category: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            category: category.into(),
            criticality: Criticality::Medium,
            status: ItemStatus::NotStarted,
            assignee: None,
            notes: None,
            depends_on: Vec::new(),
            evidence: Vec::new(),
        }
    }

    /// Sets the criticality.
    pub fn with_criticality(mut self, criticality: Criticality) -> Self {
        self.criticality = criticality;
        self
    }

    /// Sets the status.
    pub fn with_status(mut self, status: ItemStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the assignee.
    pub fn with_assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    /// Sets notes.
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Adds a dependency on another item id.
    pub fn depending_on(mut self, item_id: impl Into<String>) -> Self {
        self.depends_on.push(item_id.into());
        self
    }

    /// Adds an evidence reference.
    pub fn with_evidence(mut self, evidence: impl Into<String>) -> Self {
        self.evidence.push(evidence.into());
        self
    }

    /// Returns whether the item is resolved.
    pub fn is_resolved(&self) -> bool {
        self.status.is_resolved()
    }
}

/// A gap identified during checklist analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChecklistGap {
    /// Id of the affected item.
    pub item_id: String,
    /// Item description.
    pub description: String,
    /// Item criticality.
    pub criticality: Criticality,
    /// Why the item is flagged.
    pub reason: GapReason,
}

/// The reason an item was flagged as a gap.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GapReason {
    /// The item is unresolved.
    Unresolved,
    /// The item is explicitly blocked.
    Blocked,
    /// The item depends on one or more unresolved items.
    UnsatisfiedDependencies(Vec<String>),
    /// A dependency id does not exist in the checklist.
    DanglingDependency(String),
}

impl GapReason {
    /// Returns a human-readable explanation.
    pub fn explain(&self) -> String {
        match self {
            GapReason::Unresolved => "item is not yet resolved".to_string(),
            GapReason::Blocked => "item is blocked".to_string(),
            GapReason::UnsatisfiedDependencies(ids) => {
                format!("waiting on unresolved dependencies: {}", ids.join(", "))
            }
            GapReason::DanglingDependency(id) => {
                format!("references unknown dependency '{}'", id)
            }
        }
    }
}

/// A configurable, stateful due-diligence checklist.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DueDiligenceChecklist {
    /// Stable identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Checklist items.
    pub items: Vec<ChecklistItem>,
    /// Optional jurisdiction the checklist targets.
    pub jurisdiction: Option<Jurisdiction>,
}

impl DueDiligenceChecklist {
    /// Creates an empty checklist.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            items: Vec::new(),
            jurisdiction: None,
        }
    }

    /// Adds an item (builder style).
    pub fn with_item(mut self, item: ChecklistItem) -> Self {
        self.items.push(item);
        self
    }

    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Adds an item.
    pub fn add(&mut self, item: ChecklistItem) {
        self.items.push(item);
    }

    /// Returns an item by id.
    pub fn get(&self, id: &str) -> Option<&ChecklistItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Updates an item's status, returning an error if the id is unknown.
    pub fn set_status(&mut self, id: &str, status: ItemStatus) -> Result<()> {
        let item = self
            .items
            .iter_mut()
            .find(|item| item.id == id)
            .ok_or_else(|| anyhow!("unknown checklist item: {}", id))?;
        item.status = status;
        Ok(())
    }

    /// Attaches an evidence reference to an item.
    pub fn add_evidence(&mut self, id: &str, evidence: impl Into<String>) -> Result<()> {
        let item = self
            .items
            .iter_mut()
            .find(|item| item.id == id)
            .ok_or_else(|| anyhow!("unknown checklist item: {}", id))?;
        item.evidence.push(evidence.into());
        Ok(())
    }

    /// Returns the distinct categories present (sorted).
    pub fn categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self
            .items
            .iter()
            .map(|item| item.category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        categories
    }

    /// Returns items in a category.
    pub fn by_category(&self, category: &str) -> Vec<&ChecklistItem> {
        self.items
            .iter()
            .filter(|item| item.category == category)
            .collect()
    }

    /// Returns the fraction of items that are resolved (`0.0`..=`1.0`).
    pub fn completion_ratio(&self) -> f64 {
        if self.items.is_empty() {
            return 1.0;
        }
        let resolved = self.items.iter().filter(|item| item.is_resolved()).count();
        resolved as f64 / self.items.len() as f64
    }

    /// Returns whether every item is resolved.
    pub fn is_complete(&self) -> bool {
        self.items.iter().all(|item| item.is_resolved())
    }

    /// Detects gaps: unresolved items, blocked items, and items whose
    /// dependencies are unresolved or dangling.
    pub fn gaps(&self) -> Vec<ChecklistGap> {
        let mut resolved_ids = std::collections::HashSet::new();
        let mut known_ids = std::collections::HashSet::new();
        for item in &self.items {
            known_ids.insert(item.id.as_str());
            if item.is_resolved() {
                resolved_ids.insert(item.id.as_str());
            }
        }

        let mut gaps = Vec::new();
        for item in &self.items {
            // Dependency-driven gaps first (most actionable).
            let mut unmet = Vec::new();
            let mut dangling = None;
            for dependency in &item.depends_on {
                if !known_ids.contains(dependency.as_str()) {
                    dangling = Some(dependency.clone());
                    break;
                }
                if !resolved_ids.contains(dependency.as_str()) {
                    unmet.push(dependency.clone());
                }
            }

            if let Some(id) = dangling {
                gaps.push(ChecklistGap {
                    item_id: item.id.clone(),
                    description: item.description.clone(),
                    criticality: item.criticality,
                    reason: GapReason::DanglingDependency(id),
                });
                continue;
            }

            if item.status == ItemStatus::Blocked {
                gaps.push(ChecklistGap {
                    item_id: item.id.clone(),
                    description: item.description.clone(),
                    criticality: item.criticality,
                    reason: GapReason::Blocked,
                });
                continue;
            }

            if !item.is_resolved() && !unmet.is_empty() {
                gaps.push(ChecklistGap {
                    item_id: item.id.clone(),
                    description: item.description.clone(),
                    criticality: item.criticality,
                    reason: GapReason::UnsatisfiedDependencies(unmet),
                });
                continue;
            }

            if !item.is_resolved() {
                gaps.push(ChecklistGap {
                    item_id: item.id.clone(),
                    description: item.description.clone(),
                    criticality: item.criticality,
                    reason: GapReason::Unresolved,
                });
            }
        }

        // Surface the most critical gaps first.
        gaps.sort_by(|a, b| {
            b.criticality
                .cmp(&a.criticality)
                .then(a.item_id.cmp(&b.item_id))
        });
        gaps
    }

    /// Returns the ids of unresolved items at or above a criticality threshold.
    pub fn open_at_or_above(&self, threshold: Criticality) -> Vec<String> {
        let mut ids: Vec<String> = self
            .items
            .iter()
            .filter(|item| !item.is_resolved() && item.criticality >= threshold)
            .map(|item| item.id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Builds a structured report of the checklist's current state.
    pub fn report(&self) -> DueDiligenceReport {
        let mut by_status: HashMap<ItemStatus, usize> = HashMap::new();
        for item in &self.items {
            *by_status.entry(item.status).or_insert(0) += 1;
        }

        let gaps = self.gaps();
        let open_critical = self.open_at_or_above(Criticality::Critical);

        DueDiligenceReport {
            checklist_id: self.id.clone(),
            checklist_name: self.name.clone(),
            total_items: self.items.len(),
            resolved_items: self.items.iter().filter(|item| item.is_resolved()).count(),
            completion_ratio: self.completion_ratio(),
            by_status,
            open_critical,
            gaps,
        }
    }

    /// Builds a standard corporate-acquisition due-diligence checklist.
    pub fn corporate_acquisition() -> Self {
        Self::new("corp_acquisition", "Corporate Acquisition Due Diligence")
            .with_item(
                ChecklistItem::new(
                    "corp_good_standing",
                    "Verify target's certificate of good standing",
                    "corporate",
                )
                .with_criticality(Criticality::High),
            )
            .with_item(
                ChecklistItem::new(
                    "cap_table",
                    "Confirm capitalization table and ownership",
                    "corporate",
                )
                .with_criticality(Criticality::Critical),
            )
            .with_item(
                ChecklistItem::new(
                    "financials_audited",
                    "Review audited financial statements (3 years)",
                    "financial",
                )
                .with_criticality(Criticality::Critical),
            )
            .with_item(
                ChecklistItem::new(
                    "material_contracts",
                    "Review material contracts and change-of-control terms",
                    "contracts",
                )
                .with_criticality(Criticality::High)
                .depending_on("corp_good_standing"),
            )
            .with_item(
                ChecklistItem::new(
                    "ip_assignments",
                    "Confirm IP ownership and employee assignments",
                    "ip",
                )
                .with_criticality(Criticality::High),
            )
            .with_item(
                ChecklistItem::new(
                    "litigation_search",
                    "Search pending and threatened litigation",
                    "litigation",
                )
                .with_criticality(Criticality::Medium),
            )
            .with_item(
                ChecklistItem::new(
                    "tax_compliance",
                    "Verify tax filings and outstanding liabilities",
                    "tax",
                )
                .with_criticality(Criticality::High)
                .depending_on("financials_audited"),
            )
    }

    /// Builds a standard commercial-lease due-diligence checklist.
    pub fn commercial_lease() -> Self {
        Self::new("commercial_lease", "Commercial Lease Due Diligence")
            .with_item(
                ChecklistItem::new(
                    "title_review",
                    "Review landlord's title to premises",
                    "title",
                )
                .with_criticality(Criticality::High),
            )
            .with_item(
                ChecklistItem::new(
                    "zoning",
                    "Confirm zoning permits intended use",
                    "regulatory",
                )
                .with_criticality(Criticality::Critical),
            )
            .with_item(
                ChecklistItem::new(
                    "rent_schedule",
                    "Verify rent schedule and escalation terms",
                    "financial",
                )
                .with_criticality(Criticality::High),
            )
            .with_item(
                ChecklistItem::new("estoppel", "Obtain estoppel certificate", "documents")
                    .with_criticality(Criticality::Medium)
                    .depending_on("title_review"),
            )
    }
}

/// A structured report of a checklist's state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DueDiligenceReport {
    /// Source checklist id.
    pub checklist_id: String,
    /// Source checklist name.
    pub checklist_name: String,
    /// Total number of items.
    pub total_items: usize,
    /// Number of resolved items.
    pub resolved_items: usize,
    /// Completion ratio (`0.0`..=`1.0`).
    pub completion_ratio: f64,
    /// Item counts grouped by status.
    pub by_status: HashMap<ItemStatus, usize>,
    /// Ids of unresolved critical items.
    pub open_critical: Vec<String>,
    /// Detected gaps (most critical first).
    pub gaps: Vec<ChecklistGap>,
}

impl DueDiligenceReport {
    /// Returns whether the checklist is fully resolved with no gaps.
    pub fn is_clear(&self) -> bool {
        self.gaps.is_empty() && self.open_critical.is_empty()
    }

    /// Renders the report as Markdown.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "# Due Diligence Report: {}\n\n",
            self.checklist_name
        ));
        out.push_str(&format!(
            "Progress: {}/{} resolved ({:.0}%)\n\n",
            self.resolved_items,
            self.total_items,
            self.completion_ratio * 100.0
        ));
        if !self.open_critical.is_empty() {
            out.push_str("## Open Critical Items\n\n");
            for id in &self.open_critical {
                out.push_str(&format!("- {}\n", id));
            }
            out.push('\n');
        }
        if self.gaps.is_empty() {
            out.push_str("No outstanding gaps.\n");
        } else {
            out.push_str("## Gaps\n\n");
            for gap in &self.gaps {
                out.push_str(&format!(
                    "- [{}] {} ({}): {}\n",
                    gap.criticality.label(),
                    gap.item_id,
                    gap.description,
                    gap.reason.explain()
                ));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_tracking_and_completion() {
        let mut checklist = DueDiligenceChecklist::new("c", "Test")
            .with_item(ChecklistItem::new("a", "Item A", "general"))
            .with_item(ChecklistItem::new("b", "Item B", "general"));
        assert!((checklist.completion_ratio() - 0.0).abs() < f64::EPSILON);

        checklist.set_status("a", ItemStatus::Complete).expect("ok");
        assert!((checklist.completion_ratio() - 0.5).abs() < f64::EPSILON);
        assert!(!checklist.is_complete());

        checklist
            .set_status("b", ItemStatus::NotApplicable)
            .expect("ok");
        assert!(checklist.is_complete());
        assert!(
            checklist
                .set_status("missing", ItemStatus::Complete)
                .is_err()
        );
    }

    #[test]
    fn test_gap_detection_dependencies() {
        let mut checklist = DueDiligenceChecklist::new("c", "Test")
            .with_item(ChecklistItem::new("base", "Base", "general"))
            .with_item(
                ChecklistItem::new("dep", "Depends on base", "general")
                    .with_criticality(Criticality::Critical)
                    .depending_on("base"),
            )
            .with_item(ChecklistItem::new("dangle", "Bad dep", "general").depending_on("nope"));

        let gaps = checklist.gaps();
        // dep has unmet dependency, dangle has dangling dep, base is unresolved.
        assert!(gaps.iter().any(
            |g| g.item_id == "dep" && matches!(g.reason, GapReason::UnsatisfiedDependencies(_))
        ));
        assert!(
            gaps.iter()
                .any(|g| g.item_id == "dangle"
                    && matches!(g.reason, GapReason::DanglingDependency(_)))
        );
        // Most critical gap surfaces first.
        assert_eq!(gaps[0].item_id, "dep");

        checklist
            .set_status("base", ItemStatus::Complete)
            .expect("ok");
        let gaps2 = checklist.gaps();
        assert!(
            gaps2
                .iter()
                .any(|g| g.item_id == "dep" && g.reason == GapReason::Unresolved)
        );
    }

    #[test]
    fn test_standard_checklists_and_report() {
        let mut checklist = DueDiligenceChecklist::corporate_acquisition();
        assert!(checklist.items.len() >= 7);
        assert!(checklist.categories().contains(&"financial".to_string()));

        let report = checklist.report();
        assert_eq!(report.total_items, checklist.items.len());
        assert!(!report.open_critical.is_empty());
        assert!(!report.is_clear());

        // Resolve everything; report should clear.
        let ids: Vec<String> = checklist.items.iter().map(|i| i.id.clone()).collect();
        for id in ids {
            checklist.set_status(&id, ItemStatus::Complete).expect("ok");
        }
        let report2 = checklist.report();
        assert!(report2.is_clear());
        assert!(report2.to_markdown().contains("No outstanding gaps"));
    }

    #[test]
    fn test_blocked_and_critical_open() {
        let mut checklist = DueDiligenceChecklist::commercial_lease();
        checklist
            .set_status("zoning", ItemStatus::Blocked)
            .expect("ok");
        let gaps = checklist.gaps();
        assert!(
            gaps.iter()
                .any(|g| g.item_id == "zoning" && g.reason == GapReason::Blocked)
        );
        assert!(
            checklist
                .open_at_or_above(Criticality::Critical)
                .contains(&"zoning".to_string())
        );
    }
}
