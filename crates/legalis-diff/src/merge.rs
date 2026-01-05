//! Three-way merge support for concurrent statute amendments.
//!
//! This module provides functionality for merging concurrent changes
//! to statutes, detecting conflicts, and applying merge strategies.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::merge::{three_way_merge, MergeStrategy};
//!
//! // Common base version
//! let base = Statute::new("law", "Original Title", Effect::new(EffectType::Grant, "Benefit"));
//!
//! // Our version - change title
//! let mut ours = base.clone();
//! ours.title = "Our New Title".to_string();
//!
//! // Their version - different title change
//! let mut theirs = base.clone();
//! theirs.title = "Their New Title".to_string();
//!
//! // Merge with strategy
//! let result = three_way_merge(&base, &ours, &theirs, MergeStrategy::Ours).unwrap();
//!
//! // Check if there were conflicts
//! if !result.clean {
//!     println!("Conflicts: {}", result.conflicts.len());
//! }
//! ```

use crate::{DiffError, DiffResult};
use legalis_core::{Effect, Statute};
use serde::{Deserialize, Serialize};

/// Merge strategy for resolving conflicts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeStrategy {
    /// Prefer changes from "ours" (current branch).
    Ours,
    /// Prefer changes from "theirs" (incoming branch).
    Theirs,
    /// Attempt to combine both changes (union).
    Union,
    /// Fail on any conflict (manual resolution required).
    Strict,
}

/// Result of a merge operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeResult {
    /// The merged statute.
    pub statute: Statute,
    /// List of conflicts encountered.
    pub conflicts: Vec<MergeConflict>,
    /// Whether the merge was successful without conflicts.
    pub clean: bool,
}

/// A conflict during merge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeConflict {
    /// Type of conflict.
    pub conflict_type: ConflictType,
    /// Description of the conflict.
    pub description: String,
    /// How the conflict was resolved (if resolved).
    pub resolution: Option<ConflictResolution>,
}

/// Type of merge conflict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictType {
    /// Title changed in both branches.
    TitleConflict,
    /// Precondition modified in both branches.
    PreconditionConflict { index: usize },
    /// Effect changed in both branches.
    EffectConflict,
    /// Discretion logic changed in both branches.
    DiscretionConflict,
    /// Precondition added in one branch, modified in another.
    PreconditionAddModifyConflict,
    /// Cannot merge - incompatible changes.
    IncompatibleChanges,
}

/// How a conflict was resolved.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Used value from "ours".
    UsedOurs,
    /// Used value from "theirs".
    UsedTheirs,
    /// Combined both values.
    Combined,
    /// Left unresolved (requires manual intervention).
    Unresolved,
}

/// Conflict resolution suggestion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionSuggestion {
    /// Type of resolution.
    pub resolution_type: ConflictResolution,
    /// Confidence in this suggestion (0.0 to 1.0).
    pub confidence: f64,
    /// Explanation of why this resolution is suggested.
    pub rationale: String,
    /// Potential risks of this resolution.
    pub risks: Vec<String>,
}

/// Merge preview with impact assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergePreview {
    /// Predicted conflicts.
    pub predicted_conflicts: Vec<MergeConflict>,
    /// Impact assessment of the merge.
    pub impact: MergeImpact,
    /// Suggestions for conflict resolution.
    pub suggestions: Vec<(usize, Vec<ResolutionSuggestion>)>,
    /// Whether the merge is safe to proceed.
    pub safe_to_merge: bool,
}

/// Impact assessment of a merge operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeImpact {
    /// Number of conflicts.
    pub conflict_count: usize,
    /// Severity of the merge.
    pub severity: crate::Severity,
    /// Whether eligibility criteria will change.
    pub affects_eligibility: bool,
    /// Whether the effect will change.
    pub affects_outcome: bool,
    /// Description of the impact.
    pub description: String,
}

/// History entry for merge operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeHistoryEntry {
    /// Timestamp of the merge.
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Strategy used for the merge.
    pub strategy: MergeStrategy,
    /// Number of conflicts encountered.
    pub conflict_count: usize,
    /// Whether the merge was clean.
    pub was_clean: bool,
    /// Conflicts encountered.
    pub conflicts: Vec<MergeConflict>,
}

/// Merge history tracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeHistory {
    /// List of merge operations.
    pub entries: Vec<MergeHistoryEntry>,
}

impl MergeHistory {
    /// Creates a new merge history tracker.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Adds a merge result to the history.
    pub fn add_entry(&mut self, result: &MergeResult, strategy: MergeStrategy) {
        self.entries.push(MergeHistoryEntry {
            timestamp: chrono::Utc::now(),
            strategy,
            conflict_count: result.conflicts.len(),
            was_clean: result.clean,
            conflicts: result.conflicts.clone(),
        });
    }

    /// Gets the total number of merges.
    pub fn total_merges(&self) -> usize {
        self.entries.len()
    }

    /// Gets the number of clean merges.
    pub fn clean_merges(&self) -> usize {
        self.entries.iter().filter(|e| e.was_clean).count()
    }

    /// Gets the number of conflicted merges.
    pub fn conflicted_merges(&self) -> usize {
        self.entries.iter().filter(|e| !e.was_clean).count()
    }
}

impl Default for MergeHistory {
    fn default() -> Self {
        Self::new()
    }
}

/// Performs a three-way merge of statutes.
///
/// # Arguments
/// * `base` - The common ancestor statute
/// * `ours` - Our version of the statute
/// * `theirs` - Their version of the statute
/// * `strategy` - The merge strategy to use
///
/// # Returns
/// A `MergeResult` containing the merged statute and any conflicts.
pub fn three_way_merge(
    base: &Statute,
    ours: &Statute,
    theirs: &Statute,
    strategy: MergeStrategy,
) -> DiffResult<MergeResult> {
    // Verify all statutes have the same ID
    if base.id != ours.id || base.id != theirs.id {
        return Err(DiffError::InvalidComparison(
            "Cannot merge statutes with different IDs".to_string(),
        ));
    }

    let mut conflicts = Vec::new();
    let mut merged = base.clone();

    // Merge title
    merge_title(base, ours, theirs, &mut merged, &mut conflicts, strategy);

    // Merge preconditions
    merge_preconditions(base, ours, theirs, &mut merged, &mut conflicts, strategy)?;

    // Merge effect
    merge_effect(base, ours, theirs, &mut merged, &mut conflicts, strategy);

    // Merge discretion logic
    merge_discretion(base, ours, theirs, &mut merged, &mut conflicts, strategy);

    let clean = conflicts.is_empty();

    Ok(MergeResult {
        statute: merged,
        conflicts,
        clean,
    })
}

fn merge_title(
    base: &Statute,
    ours: &Statute,
    theirs: &Statute,
    merged: &mut Statute,
    conflicts: &mut Vec<MergeConflict>,
    strategy: MergeStrategy,
) {
    let base_title = &base.title;
    let our_title = &ours.title;
    let their_title = &theirs.title;

    if our_title == their_title {
        // Both changed to the same value, or both unchanged
        merged.title = our_title.clone();
    } else if our_title == base_title {
        // Only theirs changed
        merged.title = their_title.clone();
    } else if their_title == base_title {
        // Only ours changed
        merged.title = our_title.clone();
    } else {
        // Conflict: both changed to different values
        let conflict = MergeConflict {
            conflict_type: ConflictType::TitleConflict,
            description: format!(
                "Title conflict: ours='{}', theirs='{}'",
                our_title, their_title
            ),
            resolution: match strategy {
                MergeStrategy::Ours => {
                    merged.title = our_title.clone();
                    Some(ConflictResolution::UsedOurs)
                }
                MergeStrategy::Theirs => {
                    merged.title = their_title.clone();
                    Some(ConflictResolution::UsedTheirs)
                }
                MergeStrategy::Union => {
                    // For titles, union doesn't make sense, default to ours
                    merged.title = our_title.clone();
                    Some(ConflictResolution::UsedOurs)
                }
                MergeStrategy::Strict => {
                    merged.title = our_title.clone();
                    Some(ConflictResolution::Unresolved)
                }
            },
        };
        conflicts.push(conflict);
    }
}

fn merge_preconditions(
    base: &Statute,
    ours: &Statute,
    theirs: &Statute,
    merged: &mut Statute,
    conflicts: &mut Vec<MergeConflict>,
    strategy: MergeStrategy,
) -> DiffResult<()> {
    // For simplicity, we'll use a set-based approach:
    // - Include all preconditions from ours
    // - Add preconditions from theirs that aren't in ours and weren't removed
    // - Detect conflicts when the same precondition is modified differently

    let base_preconds = &base.preconditions;
    let our_preconds = &ours.preconditions;
    let their_preconds = &theirs.preconditions;

    if our_preconds == their_preconds {
        merged.preconditions = our_preconds.clone();
        return Ok(());
    }

    // Start with our preconditions
    let mut result_preconds = our_preconds.clone();

    // Check for additions in theirs that aren't in ours
    for (idx, their_cond) in their_preconds.iter().enumerate() {
        if !base_preconds.contains(their_cond) && !our_preconds.contains(their_cond) {
            // This is a new condition added in theirs
            match strategy {
                MergeStrategy::Theirs | MergeStrategy::Union => {
                    result_preconds.push(their_cond.clone());
                }
                MergeStrategy::Ours => {
                    // Don't add their new conditions
                    conflicts.push(MergeConflict {
                        conflict_type: ConflictType::PreconditionAddModifyConflict,
                        description: format!(
                            "Precondition added in theirs (index {}), ignored due to Ours strategy",
                            idx
                        ),
                        resolution: Some(ConflictResolution::UsedOurs),
                    });
                }
                MergeStrategy::Strict => {
                    conflicts.push(MergeConflict {
                        conflict_type: ConflictType::PreconditionAddModifyConflict,
                        description: format!("Precondition added in theirs (index {})", idx),
                        resolution: Some(ConflictResolution::Unresolved),
                    });
                }
            }
        }
    }

    merged.preconditions = result_preconds;
    Ok(())
}

fn merge_effect(
    base: &Statute,
    ours: &Statute,
    theirs: &Statute,
    merged: &mut Statute,
    conflicts: &mut Vec<MergeConflict>,
    strategy: MergeStrategy,
) {
    let base_effect = &base.effect;
    let our_effect = &ours.effect;
    let their_effect = &theirs.effect;

    if our_effect == their_effect {
        merged.effect = our_effect.clone();
    } else if our_effect == base_effect {
        // Only theirs changed
        merged.effect = their_effect.clone();
    } else if their_effect == base_effect {
        // Only ours changed
        merged.effect = our_effect.clone();
    } else {
        // Conflict: both changed
        let conflict = MergeConflict {
            conflict_type: ConflictType::EffectConflict,
            description: "Effect changed in both branches".to_string(),
            resolution: match strategy {
                MergeStrategy::Ours => {
                    merged.effect = our_effect.clone();
                    Some(ConflictResolution::UsedOurs)
                }
                MergeStrategy::Theirs => {
                    merged.effect = their_effect.clone();
                    Some(ConflictResolution::UsedTheirs)
                }
                MergeStrategy::Union => {
                    // For effects, union doesn't make sense, default to ours
                    merged.effect = our_effect.clone();
                    Some(ConflictResolution::UsedOurs)
                }
                MergeStrategy::Strict => {
                    merged.effect = our_effect.clone();
                    Some(ConflictResolution::Unresolved)
                }
            },
        };
        conflicts.push(conflict);
    }
}

fn merge_discretion(
    base: &Statute,
    ours: &Statute,
    theirs: &Statute,
    merged: &mut Statute,
    conflicts: &mut Vec<MergeConflict>,
    strategy: MergeStrategy,
) {
    let base_disc = &base.discretion_logic;
    let our_disc = &ours.discretion_logic;
    let their_disc = &theirs.discretion_logic;

    if our_disc == their_disc {
        merged.discretion_logic = our_disc.clone();
    } else if our_disc == base_disc {
        // Only theirs changed
        merged.discretion_logic = their_disc.clone();
    } else if their_disc == base_disc {
        // Only ours changed
        merged.discretion_logic = our_disc.clone();
    } else {
        // Conflict: both changed
        let conflict = MergeConflict {
            conflict_type: ConflictType::DiscretionConflict,
            description: "Discretion logic changed in both branches".to_string(),
            resolution: match strategy {
                MergeStrategy::Ours => {
                    merged.discretion_logic = our_disc.clone();
                    Some(ConflictResolution::UsedOurs)
                }
                MergeStrategy::Theirs => {
                    merged.discretion_logic = their_disc.clone();
                    Some(ConflictResolution::UsedTheirs)
                }
                MergeStrategy::Union => {
                    // Combine both discretion logics
                    let combined = match (our_disc, their_disc) {
                        (Some(ours_text), Some(theirs_text)) => {
                            Some(format!("{} AND {}", ours_text, theirs_text))
                        }
                        (Some(text), None) | (None, Some(text)) => Some(text.clone()),
                        (None, None) => None,
                    };
                    merged.discretion_logic = combined;
                    Some(ConflictResolution::Combined)
                }
                MergeStrategy::Strict => {
                    merged.discretion_logic = our_disc.clone();
                    Some(ConflictResolution::Unresolved)
                }
            },
        };
        conflicts.push(conflict);
    }
}

/// Auto-merge two statutes without a common base (simple merge).
/// This is useful when you don't have the base version available.
pub fn auto_merge(
    ours: &Statute,
    theirs: &Statute,
    strategy: MergeStrategy,
) -> DiffResult<MergeResult> {
    // Use an empty base (or the simpler of the two)
    let base = Statute::new(
        &ours.id,
        "",
        Effect::new(
            legalis_core::EffectType::Custom,
            "Placeholder for auto-merge",
        ),
    );

    three_way_merge(&base, ours, theirs, strategy)
}

/// Generates a preview of a merge operation with impact assessment.
///
/// This allows you to see what would happen if you merged without actually merging.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::merge::preview_merge;
///
/// let base = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut ours = base.clone();
/// ours.title = "Our Title".to_string();
/// let mut theirs = base.clone();
/// theirs.title = "Their Title".to_string();
///
/// let preview = preview_merge(&base, &ours, &theirs).unwrap();
/// assert!(!preview.safe_to_merge || !preview.predicted_conflicts.is_empty());
/// ```
pub fn preview_merge(base: &Statute, ours: &Statute, theirs: &Statute) -> DiffResult<MergePreview> {
    // Perform a trial merge with Strict strategy to detect all conflicts
    let trial_result = three_way_merge(base, ours, theirs, MergeStrategy::Strict)?;

    let conflict_count = trial_result.conflicts.len();
    let severity = if conflict_count == 0 {
        crate::Severity::None
    } else if conflict_count == 1 {
        crate::Severity::Minor
    } else if conflict_count <= 3 {
        crate::Severity::Moderate
    } else {
        crate::Severity::Major
    };

    // Analyze impact
    let our_diff = crate::diff(base, ours)?;
    let their_diff = crate::diff(base, theirs)?;

    let affects_eligibility =
        our_diff.impact.affects_eligibility || their_diff.impact.affects_eligibility;
    let affects_outcome = our_diff.impact.affects_outcome || their_diff.impact.affects_outcome;

    let description = if conflict_count == 0 {
        "Merge can proceed cleanly without conflicts.".to_string()
    } else {
        format!(
            "Merge will encounter {} conflict(s). Manual resolution may be required.",
            conflict_count
        )
    };

    // Generate suggestions for each conflict
    let mut suggestions = Vec::new();
    for (idx, conflict) in trial_result.conflicts.iter().enumerate() {
        suggestions.push((idx, generate_resolution_suggestions(conflict)));
    }

    let safe_to_merge = conflict_count == 0
        || trial_result
            .conflicts
            .iter()
            .all(|c| !matches!(c.conflict_type, ConflictType::IncompatibleChanges));

    Ok(MergePreview {
        predicted_conflicts: trial_result.conflicts,
        impact: MergeImpact {
            conflict_count,
            severity,
            affects_eligibility,
            affects_outcome,
            description,
        },
        suggestions,
        safe_to_merge,
    })
}

/// Generates resolution suggestions for a conflict.
fn generate_resolution_suggestions(conflict: &MergeConflict) -> Vec<ResolutionSuggestion> {
    let mut suggestions = Vec::new();

    match &conflict.conflict_type {
        ConflictType::TitleConflict => {
            suggestions.push(ResolutionSuggestion {
                resolution_type: ConflictResolution::UsedOurs,
                confidence: 0.5,
                rationale: "Preserve your version of the title".to_string(),
                risks: vec!["May lose clarity from the other version".to_string()],
            });
            suggestions.push(ResolutionSuggestion {
                resolution_type: ConflictResolution::UsedTheirs,
                confidence: 0.5,
                rationale: "Adopt their version of the title".to_string(),
                risks: vec!["May lose your intended changes".to_string()],
            });
        }
        ConflictType::EffectConflict => {
            suggestions.push(ResolutionSuggestion {
                resolution_type: ConflictResolution::UsedOurs,
                confidence: 0.7,
                rationale: "Effects are fundamental - prefer the version you control".to_string(),
                risks: vec!["May create inconsistency with their changes".to_string()],
            });
            suggestions.push(ResolutionSuggestion {
                resolution_type: ConflictResolution::Unresolved,
                confidence: 0.9,
                rationale: "Effect conflicts are critical and should be manually reviewed"
                    .to_string(),
                risks: vec!["Requires manual intervention".to_string()],
            });
        }
        ConflictType::PreconditionConflict { .. } => {
            suggestions.push(ResolutionSuggestion {
                resolution_type: ConflictResolution::Combined,
                confidence: 0.6,
                rationale: "Combine both preconditions if they are compatible".to_string(),
                risks: vec!["May create overly restrictive conditions".to_string()],
            });
            suggestions.push(ResolutionSuggestion {
                resolution_type: ConflictResolution::Unresolved,
                confidence: 0.8,
                rationale: "Precondition conflicts should be carefully reviewed".to_string(),
                risks: vec!["Requires manual intervention".to_string()],
            });
        }
        ConflictType::DiscretionConflict => {
            suggestions.push(ResolutionSuggestion {
                resolution_type: ConflictResolution::Combined,
                confidence: 0.8,
                rationale: "Combine both discretion requirements for comprehensive judgment"
                    .to_string(),
                risks: vec!["May create complex or contradictory guidelines".to_string()],
            });
        }
        ConflictType::PreconditionAddModifyConflict => {
            suggestions.push(ResolutionSuggestion {
                resolution_type: ConflictResolution::UsedTheirs,
                confidence: 0.7,
                rationale: "Include the newly added precondition".to_string(),
                risks: vec!["May tighten eligibility unexpectedly".to_string()],
            });
        }
        ConflictType::IncompatibleChanges => {
            suggestions.push(ResolutionSuggestion {
                resolution_type: ConflictResolution::Unresolved,
                confidence: 0.95,
                rationale: "Incompatible changes require manual resolution".to_string(),
                risks: vec!["Cannot automatically merge".to_string()],
            });
        }
    }

    suggestions
}

/// Performs a semantic merge that considers the meaning of changes.
///
/// This is smarter than a simple three-way merge as it understands when
/// changes are semantically compatible even if they conflict structurally.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::merge::semantic_merge;
///
/// let base = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut ours = base.clone();
/// ours.title = "Improved Title".to_string();
/// let mut theirs = base.clone();
/// theirs.title = "Enhanced Title".to_string();
///
/// let result = semantic_merge(&base, &ours, &theirs).unwrap();
/// // Semantic merge may detect that both changes are just improvements
/// ```
pub fn semantic_merge(base: &Statute, ours: &Statute, theirs: &Statute) -> DiffResult<MergeResult> {
    // First, analyze the semantic differences
    let our_diff = crate::diff(base, ours)?;
    let their_diff = crate::diff(base, theirs)?;

    use crate::semantic::analyze_semantic_diff;
    let our_semantic = analyze_semantic_diff(&our_diff);
    let their_semantic = analyze_semantic_diff(&their_diff);

    // Check if changes are semantically compatible
    let both_meaning_preserving = our_semantic.semantic_impact.primarily_clarifying
        && their_semantic.semantic_impact.primarily_clarifying;

    let both_expand = our_semantic.semantic_impact.eligibility_broadened
        && their_semantic.semantic_impact.eligibility_broadened;

    let both_contract = our_semantic.semantic_impact.eligibility_narrowed
        && their_semantic.semantic_impact.eligibility_narrowed;

    // Choose strategy based on semantic analysis
    let strategy = if both_meaning_preserving {
        // Both are just clarifications - prefer union
        MergeStrategy::Union
    } else if both_expand || both_contract {
        // Both moving in the same direction - can combine
        MergeStrategy::Union
    } else if our_semantic.semantic_impact.intent_changed
        || their_semantic.semantic_impact.intent_changed
    {
        // Intent changes require manual resolution
        MergeStrategy::Strict
    } else {
        // Default to union for compatible changes
        MergeStrategy::Union
    };

    three_way_merge(base, ours, theirs, strategy)
}

/// Interactive merge mode that allows step-by-step conflict resolution.
///
/// This returns a series of conflicts with suggestions, allowing for
/// interactive resolution.
pub fn interactive_merge_start(
    base: &Statute,
    ours: &Statute,
    theirs: &Statute,
) -> DiffResult<InteractiveMerge> {
    let preview = preview_merge(base, ours, theirs)?;

    Ok(InteractiveMerge {
        base: base.clone(),
        ours: ours.clone(),
        theirs: theirs.clone(),
        conflicts: preview.predicted_conflicts,
        suggestions: preview.suggestions,
        resolutions: Vec::new(),
        current_index: 0,
    })
}

/// Interactive merge session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveMerge {
    /// Base statute.
    pub base: Statute,
    /// Our version.
    pub ours: Statute,
    /// Their version.
    pub theirs: Statute,
    /// Conflicts to resolve.
    pub conflicts: Vec<MergeConflict>,
    /// Suggestions for each conflict.
    pub suggestions: Vec<(usize, Vec<ResolutionSuggestion>)>,
    /// User-chosen resolutions.
    pub resolutions: Vec<ConflictResolution>,
    /// Current conflict index.
    pub current_index: usize,
}

impl InteractiveMerge {
    /// Gets the current conflict to resolve.
    pub fn current_conflict(&self) -> Option<&MergeConflict> {
        self.conflicts.get(self.current_index)
    }

    /// Gets suggestions for the current conflict.
    pub fn current_suggestions(&self) -> Option<&Vec<ResolutionSuggestion>> {
        self.suggestions
            .iter()
            .find(|(idx, _)| *idx == self.current_index)
            .map(|(_, sugs)| sugs)
    }

    /// Resolves the current conflict with the given resolution.
    pub fn resolve_current(&mut self, resolution: ConflictResolution) {
        self.resolutions.push(resolution);
        self.current_index += 1;
    }

    /// Checks if all conflicts have been resolved.
    pub fn is_complete(&self) -> bool {
        self.current_index >= self.conflicts.len()
    }

    /// Completes the merge with the chosen resolutions.
    pub fn finalize(&self) -> DiffResult<MergeResult> {
        if !self.is_complete() {
            return Err(DiffError::InvalidComparison(
                "Not all conflicts have been resolved".to_string(),
            ));
        }

        // Apply resolutions manually based on user choices
        let mut merged = self.base.clone();
        let mut final_conflicts = Vec::new();

        for (conflict, resolution) in self.conflicts.iter().zip(self.resolutions.iter()) {
            let mut resolved_conflict = conflict.clone();
            resolved_conflict.resolution = Some(resolution.clone());
            final_conflicts.push(resolved_conflict);

            // Apply the resolution to the merged statute
            apply_resolution(&mut merged, &self.ours, &self.theirs, conflict, resolution);
        }

        Ok(MergeResult {
            statute: merged,
            conflicts: final_conflicts,
            clean: false, // Interactive merges are never "clean" since they had conflicts
        })
    }
}

/// Applies a conflict resolution to a statute.
#[allow(dead_code)]
fn apply_resolution(
    merged: &mut Statute,
    ours: &Statute,
    theirs: &Statute,
    conflict: &MergeConflict,
    resolution: &ConflictResolution,
) {
    match &conflict.conflict_type {
        ConflictType::TitleConflict => match resolution {
            ConflictResolution::UsedOurs => merged.title = ours.title.clone(),
            ConflictResolution::UsedTheirs => merged.title = theirs.title.clone(),
            _ => {}
        },
        ConflictType::EffectConflict => match resolution {
            ConflictResolution::UsedOurs => merged.effect = ours.effect.clone(),
            ConflictResolution::UsedTheirs => merged.effect = theirs.effect.clone(),
            _ => {}
        },
        ConflictType::DiscretionConflict => match resolution {
            ConflictResolution::UsedOurs => merged.discretion_logic = ours.discretion_logic.clone(),
            ConflictResolution::UsedTheirs => {
                merged.discretion_logic = theirs.discretion_logic.clone()
            }
            ConflictResolution::Combined => {
                if let (Some(o), Some(t)) = (&ours.discretion_logic, &theirs.discretion_logic) {
                    merged.discretion_logic = Some(format!("{} AND {}", o, t));
                }
            }
            _ => {}
        },
        _ => {
            // For other conflict types, basic handling
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn base_statute() -> Statute {
        Statute::new(
            "test-statute",
            "Base Title",
            Effect::new(EffectType::Grant, "Base effect"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_no_conflict_both_unchanged() {
        let base = base_statute();
        let ours = base.clone();
        let theirs = base.clone();

        let result = three_way_merge(&base, &ours, &theirs, MergeStrategy::Union).unwrap();
        assert!(result.clean);
        assert!(result.conflicts.is_empty());
    }

    #[test]
    fn test_no_conflict_only_ours_changed() {
        let base = base_statute();
        let mut ours = base.clone();
        ours.title = "Our Title".to_string();
        let theirs = base.clone();

        let result = three_way_merge(&base, &ours, &theirs, MergeStrategy::Union).unwrap();
        assert!(result.clean);
        assert_eq!(result.statute.title, "Our Title");
    }

    #[test]
    fn test_no_conflict_only_theirs_changed() {
        let base = base_statute();
        let ours = base.clone();
        let mut theirs = base.clone();
        theirs.title = "Their Title".to_string();

        let result = three_way_merge(&base, &ours, &theirs, MergeStrategy::Union).unwrap();
        assert!(result.clean);
        assert_eq!(result.statute.title, "Their Title");
    }

    #[test]
    fn test_conflict_both_changed_title() {
        let base = base_statute();
        let mut ours = base.clone();
        ours.title = "Our Title".to_string();
        let mut theirs = base.clone();
        theirs.title = "Their Title".to_string();

        let result = three_way_merge(&base, &ours, &theirs, MergeStrategy::Ours).unwrap();
        assert!(!result.clean);
        assert_eq!(result.conflicts.len(), 1);
        assert_eq!(result.statute.title, "Our Title");
    }

    #[test]
    fn test_merge_strategy_theirs() {
        let base = base_statute();
        let mut ours = base.clone();
        ours.title = "Our Title".to_string();
        let mut theirs = base.clone();
        theirs.title = "Their Title".to_string();

        let result = three_way_merge(&base, &ours, &theirs, MergeStrategy::Theirs).unwrap();
        assert!(!result.clean);
        assert_eq!(result.statute.title, "Their Title");
    }

    #[test]
    fn test_precondition_added_in_theirs() {
        let base = base_statute();
        let ours = base.clone();
        let mut theirs = base.clone();
        theirs.preconditions.push(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });

        let result = three_way_merge(&base, &ours, &theirs, MergeStrategy::Union).unwrap();
        assert!(result.clean);
        assert_eq!(result.statute.preconditions.len(), 2);
    }

    #[test]
    fn test_effect_conflict() {
        let base = base_statute();
        let mut ours = base.clone();
        ours.effect = Effect::new(EffectType::Revoke, "Our effect");
        let mut theirs = base.clone();
        theirs.effect = Effect::new(EffectType::Obligation, "Their effect");

        let result = three_way_merge(&base, &ours, &theirs, MergeStrategy::Ours).unwrap();
        assert!(!result.clean);
        assert_eq!(result.conflicts.len(), 1);
        assert!(matches!(
            result.conflicts[0].conflict_type,
            ConflictType::EffectConflict
        ));
    }

    #[test]
    fn test_discretion_union_merge() {
        let base = base_statute();
        let mut ours = base.clone();
        ours.discretion_logic = Some("Consider circumstance A".to_string());
        let mut theirs = base.clone();
        theirs.discretion_logic = Some("Consider circumstance B".to_string());

        let result = three_way_merge(&base, &ours, &theirs, MergeStrategy::Union).unwrap();
        assert!(!result.clean);
        let merged_discretion = result.statute.discretion_logic.unwrap();
        assert!(merged_discretion.contains("circumstance A"));
        assert!(merged_discretion.contains("circumstance B"));
    }

    #[test]
    fn test_auto_merge() {
        let mut ours = base_statute();
        ours.title = "Our Title".to_string();
        let mut theirs = base_statute();
        theirs.preconditions.push(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 5000000,
        });

        let result = auto_merge(&ours, &theirs, MergeStrategy::Union).unwrap();
        assert_eq!(result.statute.title, "Our Title");
        assert_eq!(result.statute.preconditions.len(), 2);
    }
}
