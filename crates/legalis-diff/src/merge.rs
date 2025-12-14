//! Three-way merge support for concurrent statute amendments.
//!
//! This module provides functionality for merging concurrent changes
//! to statutes, detecting conflicts, and applying merge strategies.

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
