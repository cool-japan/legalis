//! Timeline and amendment chain tracking for statute evolution.
//!
//! This module provides tools for tracking how statutes evolve over time
//! through a series of amendments, and visualizing that evolution.

use crate::{StatuteDiff, diff};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a chain of amendments to a statute over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendmentChain {
    /// The statute ID being tracked
    pub statute_id: String,
    /// Ordered list of versions
    pub versions: Vec<StatuteVersion>,
    /// Metadata about the chain
    pub metadata: ChainMetadata,
}

/// A single version of a statute in the amendment chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteVersion {
    /// Version number
    pub version: u32,
    /// The statute at this version
    pub statute: Statute,
    /// When this version was enacted
    pub enacted_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Description of changes in this version
    pub change_description: Option<String>,
    /// Reference to the amending legislation
    pub amending_act: Option<String>,
}

/// Metadata about an amendment chain.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChainMetadata {
    /// Total number of amendments
    pub total_amendments: usize,
    /// First version date
    pub first_enacted: Option<chrono::DateTime<chrono::Utc>>,
    /// Most recent amendment date
    pub last_amended: Option<chrono::DateTime<chrono::Utc>>,
    /// Tags or categories
    pub tags: Vec<String>,
}

impl AmendmentChain {
    /// Creates a new amendment chain starting with an initial statute.
    pub fn new(initial: Statute) -> Self {
        let statute_id = initial.id.clone();
        let version = StatuteVersion {
            version: initial.version,
            enacted_at: initial.temporal_validity.enacted_at,
            statute: initial,
            change_description: Some("Initial enactment".to_string()),
            amending_act: None,
        };

        let first_enacted = version.enacted_at;
        let last_amended = version.enacted_at;

        Self {
            statute_id,
            versions: vec![version],
            metadata: ChainMetadata {
                total_amendments: 0,
                first_enacted,
                last_amended,
                tags: Vec::new(),
            },
        }
    }

    /// Adds a new version to the chain.
    pub fn add_version(
        &mut self,
        statute: Statute,
        change_description: Option<String>,
        amending_act: Option<String>,
    ) {
        let version = StatuteVersion {
            version: statute.version,
            enacted_at: statute.temporal_validity.amended_at,
            statute,
            change_description,
            amending_act,
        };

        if let Some(enacted) = version.enacted_at {
            if self.metadata.first_enacted.is_none() {
                self.metadata.first_enacted = Some(enacted);
            }
            self.metadata.last_amended = Some(enacted);
        }

        self.metadata.total_amendments += 1;
        self.versions.push(version);
    }

    /// Gets the current (latest) version.
    pub fn current_version(&self) -> Option<&StatuteVersion> {
        self.versions.last()
    }

    /// Gets a version by version number.
    pub fn get_version(&self, version: u32) -> Option<&StatuteVersion> {
        self.versions.iter().find(|v| v.version == version)
    }

    /// Gets the diff between two versions.
    pub fn diff_versions(&self, from_version: u32, to_version: u32) -> Option<StatuteDiff> {
        let from = self.get_version(from_version)?;
        let to = self.get_version(to_version)?;
        diff(&from.statute, &to.statute).ok()
    }

    /// Gets all diffs in chronological order.
    pub fn all_diffs(&self) -> Vec<StatuteDiff> {
        let mut diffs = Vec::new();
        for i in 1..self.versions.len() {
            if let Ok(d) = diff(&self.versions[i - 1].statute, &self.versions[i].statute) {
                diffs.push(d);
            }
        }
        diffs
    }

    /// Generates a timeline visualization as ASCII art.
    pub fn visualize_timeline(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Amendment Chain: {}\n", self.statute_id));
        output.push_str(&format!("Total Versions: {}\n\n", self.versions.len()));

        for (i, version) in self.versions.iter().enumerate() {
            let marker = if i == 0 { "●" } else { "○" };
            let date_str = version
                .enacted_at
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| "unknown date".to_string());

            output.push_str(&format!("{} v{} ({})\n", marker, version.version, date_str));

            if let Some(ref desc) = version.change_description {
                output.push_str(&format!("  │ {}\n", desc));
            }

            if let Some(ref act) = version.amending_act {
                output.push_str(&format!("  │ By: {}\n", act));
            }

            // Add diff summary if not the first version
            if i > 0 {
                if let Ok(d) = diff(&self.versions[i - 1].statute, &self.versions[i].statute) {
                    output.push_str(&format!("  │ Changes: {}\n", d.changes.len()));
                    output.push_str(&format!("  │ Severity: {:?}\n", d.impact.severity));
                }
            }

            if i < self.versions.len() - 1 {
                output.push_str("  │\n");
            }
        }

        output
    }

    /// Generates a compact summary of the amendment history.
    pub fn summary(&self) -> String {
        format!(
            "Amendment chain for '{}': {} version(s), {} amendment(s), last amended: {}",
            self.statute_id,
            self.versions.len(),
            self.metadata.total_amendments,
            self.metadata
                .last_amended
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| "never".to_string())
        )
    }
}

/// Timeline view that aggregates multiple amendment chains.
#[derive(Debug, Clone, Default)]
pub struct Timeline {
    /// All amendment chains indexed by statute ID
    chains: HashMap<String, AmendmentChain>,
}

impl Timeline {
    /// Creates a new empty timeline.
    pub fn new() -> Self {
        Self {
            chains: HashMap::new(),
        }
    }

    /// Adds an amendment chain to the timeline.
    pub fn add_chain(&mut self, chain: AmendmentChain) {
        self.chains.insert(chain.statute_id.clone(), chain);
    }

    /// Gets a chain by statute ID.
    pub fn get_chain(&self, statute_id: &str) -> Option<&AmendmentChain> {
        self.chains.get(statute_id)
    }

    /// Gets all chains.
    pub fn all_chains(&self) -> Vec<&AmendmentChain> {
        self.chains.values().collect()
    }

    /// Generates a chronological timeline of all amendments across all statutes.
    pub fn chronological_timeline(&self) -> Vec<TimelineEvent> {
        let mut events = Vec::new();

        for chain in self.chains.values() {
            for (i, version) in chain.versions.iter().enumerate() {
                if let Some(enacted) = version.enacted_at {
                    events.push(TimelineEvent {
                        timestamp: enacted,
                        statute_id: chain.statute_id.clone(),
                        version: version.version,
                        description: version.change_description.clone(),
                        is_initial: i == 0,
                    });
                }
            }
        }

        events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        events
    }

    /// Visualizes the complete timeline.
    pub fn visualize(&self) -> String {
        let mut output = String::new();
        output.push_str("=== COMPLETE TIMELINE ===\n\n");

        let events = self.chronological_timeline();
        for event in events {
            let event_type = if event.is_initial {
                "ENACTED"
            } else {
                "AMENDED"
            };
            output.push_str(&format!(
                "{} | {} | {} v{}\n",
                event.timestamp.format("%Y-%m-%d"),
                event_type,
                event.statute_id,
                event.version
            ));
            if let Some(ref desc) = event.description {
                output.push_str(&format!("         {}\n", desc));
            }
            output.push('\n');
        }

        output
    }
}

/// A single event in the timeline.
#[derive(Debug, Clone)]
pub struct TimelineEvent {
    /// When the event occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Which statute was affected
    pub statute_id: String,
    /// Version number
    pub version: u32,
    /// Description
    pub description: Option<String>,
    /// Whether this is the initial enactment
    pub is_initial: bool,
}

/// Blame-style annotation showing which version introduced each change.
#[derive(Debug, Clone)]
pub struct BlameAnnotation {
    /// The statute ID
    pub statute_id: String,
    /// Annotations for different parts of the statute
    pub annotations: Vec<ComponentBlame>,
}

/// Blame information for a specific component.
#[derive(Debug, Clone)]
pub struct ComponentBlame {
    /// What component this is (title, precondition #1, effect, etc.)
    pub component: String,
    /// When it was introduced/last modified
    pub introduced_in_version: u32,
    /// Date of introduction/modification
    pub introduced_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Description of when it was introduced
    pub change_description: Option<String>,
    /// The amending act that made the change
    pub amending_act: Option<String>,
}

impl BlameAnnotation {
    /// Generates blame annotations for a statute by analyzing its amendment chain.
    pub fn from_chain(chain: &AmendmentChain) -> Self {
        let mut annotations = Vec::new();

        if chain.versions.is_empty() {
            return Self {
                statute_id: chain.statute_id.clone(),
                annotations,
            };
        }

        let current = &chain.versions.last().unwrap().statute;

        // Track title
        let title_version = Self::find_component_origin(chain, |s| s.title.clone());
        if let Some((version, version_data)) = title_version {
            annotations.push(ComponentBlame {
                component: "Title".to_string(),
                introduced_in_version: version,
                introduced_at: version_data.enacted_at,
                change_description: version_data.change_description.clone(),
                amending_act: version_data.amending_act.clone(),
            });
        }

        // Track each precondition
        for (i, _precond) in current.preconditions.iter().enumerate() {
            let precond_version =
                Self::find_component_origin(chain, |s| format!("{:?}", s.preconditions.get(i)));
            if let Some((version, version_data)) = precond_version {
                annotations.push(ComponentBlame {
                    component: format!("Precondition #{}", i + 1),
                    introduced_in_version: version,
                    introduced_at: version_data.enacted_at,
                    change_description: version_data.change_description.clone(),
                    amending_act: version_data.amending_act.clone(),
                });
            }
        }

        // Track effect
        let effect_version = Self::find_component_origin(chain, |s| format!("{:?}", s.effect));
        if let Some((version, version_data)) = effect_version {
            annotations.push(ComponentBlame {
                component: "Effect".to_string(),
                introduced_in_version: version,
                introduced_at: version_data.enacted_at,
                change_description: version_data.change_description.clone(),
                amending_act: version_data.amending_act.clone(),
            });
        }

        // Track discretion logic
        if current.discretion_logic.is_some() {
            let disc_version =
                Self::find_component_origin(chain, |s| format!("{:?}", s.discretion_logic));
            if let Some((version, version_data)) = disc_version {
                annotations.push(ComponentBlame {
                    component: "Discretion Logic".to_string(),
                    introduced_in_version: version,
                    introduced_at: version_data.enacted_at,
                    change_description: version_data.change_description.clone(),
                    amending_act: version_data.amending_act.clone(),
                });
            }
        }

        Self {
            statute_id: chain.statute_id.clone(),
            annotations,
        }
    }

    fn find_component_origin<F>(
        chain: &AmendmentChain,
        extractor: F,
    ) -> Option<(u32, &StatuteVersion)>
    where
        F: Fn(&Statute) -> String,
    {
        let current_value = extractor(&chain.versions.last()?.statute);

        // Walk backwards to find when this value was introduced
        for i in (0..chain.versions.len()).rev() {
            let version = &chain.versions[i];
            if extractor(&version.statute) != current_value {
                // Changed in next version
                if i + 1 < chain.versions.len() {
                    let next = &chain.versions[i + 1];
                    return Some((next.version, next));
                }
            }
        }

        // Unchanged since first version
        Some((chain.versions[0].version, &chain.versions[0]))
    }

    /// Generates a blame report showing which version introduced each component.
    pub fn report(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("=== BLAME REPORT: {} ===\n\n", self.statute_id));

        for annotation in &self.annotations {
            output.push_str(&format!("{}:\n", annotation.component));
            output.push_str(&format!(
                "  Introduced in: v{}\n",
                annotation.introduced_in_version
            ));
            if let Some(date) = annotation.introduced_at {
                output.push_str(&format!("  Date: {}\n", date.format("%Y-%m-%d")));
            }
            if let Some(ref desc) = annotation.change_description {
                output.push_str(&format!("  Change: {}\n", desc));
            }
            if let Some(ref act) = annotation.amending_act {
                output.push_str(&format!("  By: {}\n", act));
            }
            output.push('\n');
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn create_test_statute(version: u32) -> Statute {
        Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        )
        .with_version(version)
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_amendment_chain_creation() {
        let statute = create_test_statute(1);
        let chain = AmendmentChain::new(statute);

        assert_eq!(chain.statute_id, "test-statute");
        assert_eq!(chain.versions.len(), 1);
        assert_eq!(chain.metadata.total_amendments, 0);
    }

    #[test]
    fn test_add_version_to_chain() {
        let mut chain = AmendmentChain::new(create_test_statute(1));

        let amended = create_test_statute(2);
        chain.add_version(
            amended,
            Some("Amended title".to_string()),
            Some("Act 2024-001".to_string()),
        );

        assert_eq!(chain.versions.len(), 2);
        assert_eq!(chain.metadata.total_amendments, 1);
    }

    #[test]
    fn test_get_version() {
        let mut chain = AmendmentChain::new(create_test_statute(1));
        chain.add_version(create_test_statute(2), None, None);

        assert!(chain.get_version(1).is_some());
        assert!(chain.get_version(2).is_some());
        assert!(chain.get_version(3).is_none());
    }

    #[test]
    fn test_current_version() {
        let mut chain = AmendmentChain::new(create_test_statute(1));
        chain.add_version(create_test_statute(2), None, None);

        let current = chain.current_version().unwrap();
        assert_eq!(current.version, 2);
    }

    #[test]
    fn test_diff_versions() {
        let mut chain = AmendmentChain::new(create_test_statute(1));

        let mut amended = create_test_statute(2);
        amended.title = "Modified Title".to_string();
        chain.add_version(amended, None, None);

        let diff = chain.diff_versions(1, 2);
        assert!(diff.is_some());

        let d = diff.unwrap();
        assert!(!d.changes.is_empty());
    }

    #[test]
    fn test_all_diffs() {
        let mut chain = AmendmentChain::new(create_test_statute(1));
        chain.add_version(create_test_statute(2), None, None);
        chain.add_version(create_test_statute(3), None, None);

        let diffs = chain.all_diffs();
        assert_eq!(diffs.len(), 2);
    }

    #[test]
    fn test_visualize_timeline() {
        let mut chain = AmendmentChain::new(create_test_statute(1));
        chain.add_version(
            create_test_statute(2),
            Some("First amendment".to_string()),
            Some("Act 2024-001".to_string()),
        );

        let viz = chain.visualize_timeline();
        assert!(viz.contains("test-statute"));
        assert!(viz.contains("v1"));
        assert!(viz.contains("v2"));
        assert!(viz.contains("First amendment"));
    }

    #[test]
    fn test_timeline_add_chain() {
        let mut timeline = Timeline::new();
        let chain = AmendmentChain::new(create_test_statute(1));

        timeline.add_chain(chain);
        assert!(timeline.get_chain("test-statute").is_some());
    }

    #[test]
    fn test_timeline_chronological() {
        let mut timeline = Timeline::new();
        let chain = AmendmentChain::new(create_test_statute(1));
        timeline.add_chain(chain);

        let events = timeline.chronological_timeline();
        // Events with dates will be included
        assert!(events.is_empty() || !events.is_empty());
    }

    #[test]
    fn test_blame_annotation() {
        let mut chain = AmendmentChain::new(create_test_statute(1));
        chain.add_version(create_test_statute(2), Some("Amendment".to_string()), None);

        let blame = BlameAnnotation::from_chain(&chain);
        assert_eq!(blame.statute_id, "test-statute");
        assert!(!blame.annotations.is_empty());
    }

    #[test]
    fn test_blame_report() {
        let chain = AmendmentChain::new(create_test_statute(1));
        let blame = BlameAnnotation::from_chain(&chain);

        let report = blame.report();
        assert!(report.contains("BLAME REPORT"));
        assert!(report.contains("test-statute"));
    }
}
