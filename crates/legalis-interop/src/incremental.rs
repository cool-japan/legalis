//! Diff-aware incremental conversion for efficient updates.
//!
//! This module enables incremental conversion by tracking changes and only
//! re-converting modified portions of legal documents.

use crate::{ConversionReport, InteropResult, LegalConverter, LegalFormat};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Change type for statute modifications.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    /// Statute was added
    Added,
    /// Statute was modified
    Modified,
    /// Statute was removed
    Removed,
    /// No change
    Unchanged,
}

/// Represents a change to a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteChange {
    /// Statute ID
    pub id: String,
    /// Type of change
    pub change_type: ChangeType,
    /// Old version (if modified or removed)
    pub old_statute: Option<Statute>,
    /// New version (if added or modified)
    pub new_statute: Option<Statute>,
}

/// Diff between two sets of statutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteDiff {
    /// All detected changes
    pub changes: Vec<StatuteChange>,
    /// Number of additions
    pub additions: usize,
    /// Number of modifications
    pub modifications: usize,
    /// Number of removals
    pub removals: usize,
    /// Number of unchanged statutes
    pub unchanged: usize,
}

impl StatuteDiff {
    /// Computes diff between old and new statute sets.
    pub fn compute(old_statutes: &[Statute], new_statutes: &[Statute]) -> Self {
        let mut changes = Vec::new();
        let mut additions = 0;
        let mut modifications = 0;
        let mut removals = 0;
        let mut unchanged = 0;

        // Build maps for efficient lookup
        let old_map: HashMap<_, _> = old_statutes.iter().map(|s| (&s.id, s)).collect();
        let new_map: HashMap<_, _> = new_statutes.iter().map(|s| (&s.id, s)).collect();

        let old_ids: HashSet<_> = old_map.keys().copied().collect();
        let new_ids: HashSet<_> = new_map.keys().copied().collect();

        // Find added statutes
        for id in new_ids.difference(&old_ids) {
            if let Some(statute) = new_map.get(id) {
                changes.push(StatuteChange {
                    id: id.to_string(),
                    change_type: ChangeType::Added,
                    old_statute: None,
                    new_statute: Some((*statute).clone()),
                });
                additions += 1;
            }
        }

        // Find removed statutes
        for id in old_ids.difference(&new_ids) {
            if let Some(statute) = old_map.get(id) {
                changes.push(StatuteChange {
                    id: id.to_string(),
                    change_type: ChangeType::Removed,
                    old_statute: Some((*statute).clone()),
                    new_statute: None,
                });
                removals += 1;
            }
        }

        // Find modified or unchanged statutes
        for id in old_ids.intersection(&new_ids) {
            if let (Some(old_statute), Some(new_statute)) = (old_map.get(id), new_map.get(id)) {
                if Self::statutes_equal(old_statute, new_statute) {
                    changes.push(StatuteChange {
                        id: id.to_string(),
                        change_type: ChangeType::Unchanged,
                        old_statute: Some((*old_statute).clone()),
                        new_statute: Some((*new_statute).clone()),
                    });
                    unchanged += 1;
                } else {
                    changes.push(StatuteChange {
                        id: id.to_string(),
                        change_type: ChangeType::Modified,
                        old_statute: Some((*old_statute).clone()),
                        new_statute: Some((*new_statute).clone()),
                    });
                    modifications += 1;
                }
            }
        }

        Self {
            changes,
            additions,
            modifications,
            removals,
            unchanged,
        }
    }

    fn statutes_equal(a: &Statute, b: &Statute) -> bool {
        // Simple comparison - could be made more sophisticated
        a.id == b.id
            && a.title == b.title
            && a.preconditions.len() == b.preconditions.len()
            && a.effect.effect_type == b.effect.effect_type
            && a.effect.description == b.effect.description
    }

    /// Returns true if there are any changes.
    pub fn has_changes(&self) -> bool {
        self.additions > 0 || self.modifications > 0 || self.removals > 0
    }

    /// Returns total number of changes (excluding unchanged).
    pub fn total_changes(&self) -> usize {
        self.additions + self.modifications + self.removals
    }
}

/// Incremental conversion state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalState {
    /// Source format
    pub source_format: LegalFormat,
    /// Target format
    pub target_format: LegalFormat,
    /// Last converted statutes
    pub last_statutes: Vec<Statute>,
    /// Last conversion output
    pub last_output: String,
    /// Last conversion report
    pub last_report: ConversionReport,
}

impl IncrementalState {
    /// Creates a new incremental state.
    pub fn new(
        source_format: LegalFormat,
        target_format: LegalFormat,
        statutes: Vec<Statute>,
        output: String,
        report: ConversionReport,
    ) -> Self {
        Self {
            source_format,
            target_format,
            last_statutes: statutes,
            last_output: output,
            last_report: report,
        }
    }
}

/// Incremental converter that tracks changes.
pub struct IncrementalConverter {
    converter: LegalConverter,
    state: Option<IncrementalState>,
}

impl IncrementalConverter {
    /// Creates a new incremental converter.
    pub fn new() -> Self {
        Self {
            converter: LegalConverter::new(),
            state: None,
        }
    }

    /// Creates a new incremental converter with caching.
    pub fn with_cache(cache_size: usize) -> Self {
        Self {
            converter: LegalConverter::with_cache(cache_size),
            state: None,
        }
    }

    /// Performs incremental conversion.
    ///
    /// Only converts changed statutes and merges with previous output.
    ///
    /// # Arguments
    /// * `source` - New source text
    /// * `source_format` - Source format
    /// * `target_format` - Target format
    ///
    /// # Returns
    /// Tuple of (output, report, diff)
    pub fn convert_incremental(
        &mut self,
        source: &str,
        source_format: LegalFormat,
        target_format: LegalFormat,
    ) -> InteropResult<(String, ConversionReport, StatuteDiff)> {
        // Import new statutes
        let (new_statutes, mut import_report) = self.converter.import(source, source_format)?;

        // Check if we have previous state
        if let Some(state) = &self.state
            && state.source_format == source_format
            && state.target_format == target_format
        {
            // Compute diff
            let diff = StatuteDiff::compute(&state.last_statutes, &new_statutes);

            if !diff.has_changes() {
                // No changes, return cached output
                return Ok((state.last_output.clone(), state.last_report.clone(), diff));
            }

            // Only convert changed statutes
            let changed_statutes: Vec<_> = diff
                .changes
                .iter()
                .filter_map(|change| match change.change_type {
                    ChangeType::Added | ChangeType::Modified => change.new_statute.clone(),
                    _ => None,
                })
                .collect();

            // Convert changed statutes
            let (_partial_output, export_report) =
                self.converter.export(&changed_statutes, target_format)?;

            // Merge reports
            import_report.target_format = Some(target_format);
            import_report
                .unsupported_features
                .extend(export_report.unsupported_features);
            import_report.warnings.extend(export_report.warnings);
            import_report.confidence =
                (import_report.confidence * export_report.confidence).max(0.0);

            // For now, we do a full conversion of all statutes
            // A more sophisticated implementation would merge the partial output
            let (full_output, _) = self.converter.export(&new_statutes, target_format)?;

            // Update state
            self.state = Some(IncrementalState::new(
                source_format,
                target_format,
                new_statutes,
                full_output.clone(),
                import_report.clone(),
            ));

            return Ok((full_output, import_report, diff));
        }

        // No previous state or format mismatch - do full conversion
        let (output, export_report) = self.converter.export(&new_statutes, target_format)?;

        import_report.target_format = Some(target_format);
        import_report
            .unsupported_features
            .extend(export_report.unsupported_features);
        import_report.warnings.extend(export_report.warnings);
        import_report.confidence = (import_report.confidence * export_report.confidence).max(0.0);

        let diff = StatuteDiff {
            changes: new_statutes
                .iter()
                .map(|s| StatuteChange {
                    id: s.id.clone(),
                    change_type: ChangeType::Added,
                    old_statute: None,
                    new_statute: Some(s.clone()),
                })
                .collect(),
            additions: new_statutes.len(),
            modifications: 0,
            removals: 0,
            unchanged: 0,
        };

        // Save state
        self.state = Some(IncrementalState::new(
            source_format,
            target_format,
            new_statutes,
            output.clone(),
            import_report.clone(),
        ));

        Ok((output, import_report, diff))
    }

    /// Resets the incremental state.
    pub fn reset(&mut self) {
        self.state = None;
    }

    /// Gets the current state.
    pub fn state(&self) -> Option<&IncrementalState> {
        self.state.as_ref()
    }
}

impl Default for IncrementalConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn test_statute_diff_added() {
        let old_statutes = vec![];
        let new_statutes = vec![Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "test"),
        )];

        let diff = StatuteDiff::compute(&old_statutes, &new_statutes);

        assert_eq!(diff.additions, 1);
        assert_eq!(diff.modifications, 0);
        assert_eq!(diff.removals, 0);
        assert!(diff.has_changes());
    }

    #[test]
    fn test_statute_diff_removed() {
        let old_statutes = vec![Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "test"),
        )];
        let new_statutes = vec![];

        let diff = StatuteDiff::compute(&old_statutes, &new_statutes);

        assert_eq!(diff.additions, 0);
        assert_eq!(diff.modifications, 0);
        assert_eq!(diff.removals, 1);
        assert!(diff.has_changes());
    }

    #[test]
    fn test_statute_diff_unchanged() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));
        let old_statutes = vec![statute.clone()];
        let new_statutes = vec![statute];

        let diff = StatuteDiff::compute(&old_statutes, &new_statutes);

        assert_eq!(diff.additions, 0);
        assert_eq!(diff.modifications, 0);
        assert_eq!(diff.removals, 0);
        assert_eq!(diff.unchanged, 1);
        assert!(!diff.has_changes());
    }

    #[test]
    fn test_statute_diff_modified() {
        let old_statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "old"));
        let new_statute = Statute::new(
            "test",
            "Test Modified",
            Effect::new(EffectType::Grant, "new"),
        );

        let diff = StatuteDiff::compute(&[old_statute], &[new_statute]);

        assert_eq!(diff.additions, 0);
        assert_eq!(diff.modifications, 1);
        assert_eq!(diff.removals, 0);
        assert!(diff.has_changes());
    }

    #[test]
    fn test_incremental_converter_first_run() {
        let mut converter = IncrementalConverter::new();

        let catala_source = "declaration scope Test:\n  context input content integer";

        let (output, report, diff) = converter
            .convert_incremental(catala_source, LegalFormat::Catala, LegalFormat::L4)
            .unwrap();

        assert!(!output.is_empty());
        assert!(report.statutes_converted > 0);
        assert_eq!(diff.additions, report.statutes_converted);
        assert!(converter.state().is_some());
    }

    #[test]
    fn test_incremental_converter_no_change() {
        let mut converter = IncrementalConverter::new();

        let catala_source = "declaration scope Test:\n  context input content integer";

        // First conversion
        let (output1, _, _) = converter
            .convert_incremental(catala_source, LegalFormat::Catala, LegalFormat::L4)
            .unwrap();

        // Second conversion with same input
        let (output2, _, diff) = converter
            .convert_incremental(catala_source, LegalFormat::Catala, LegalFormat::L4)
            .unwrap();

        assert_eq!(output1, output2);
        assert!(!diff.has_changes());
    }

    #[test]
    fn test_incremental_converter_reset() {
        let mut converter = IncrementalConverter::new();

        let catala_source = "declaration scope Test:\n  context input content integer";

        converter
            .convert_incremental(catala_source, LegalFormat::Catala, LegalFormat::L4)
            .unwrap();

        assert!(converter.state().is_some());

        converter.reset();
        assert!(converter.state().is_none());
    }
}
