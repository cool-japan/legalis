//! Decision replay and point-in-time reconstruction.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Replays decisions from audit records.
pub struct DecisionReplayer;

impl DecisionReplayer {
    /// Reconstructs the state at a specific point in time.
    pub fn reconstruct_at_time(
        records: &[AuditRecord],
        point_in_time: DateTime<Utc>,
    ) -> TimelineState {
        let relevant_records: Vec<_> = records
            .iter()
            .filter(|r| r.timestamp <= point_in_time)
            .cloned()
            .collect();

        TimelineState {
            point_in_time,
            records: relevant_records.clone(),
            record_count: relevant_records.len(),
            subjects_affected: Self::count_unique_subjects(&relevant_records),
            statutes_applied: Self::count_unique_statutes(&relevant_records),
        }
    }

    /// Gets the complete history for a specific subject.
    pub fn subject_history(
        records: &[AuditRecord],
        subject_id: Uuid,
    ) -> AuditResult<SubjectHistory> {
        let subject_records: Vec<_> = records
            .iter()
            .filter(|r| r.subject_id == subject_id)
            .cloned()
            .collect();

        if subject_records.is_empty() {
            return Err(AuditError::QueryError(format!(
                "No records found for subject {}",
                subject_id
            )));
        }

        let first_seen = subject_records.iter().map(|r| r.timestamp).min().unwrap();
        let last_seen = subject_records.iter().map(|r| r.timestamp).max().unwrap();

        Ok(SubjectHistory {
            subject_id,
            first_seen,
            last_seen,
            total_decisions: subject_records.len(),
            records: subject_records,
        })
    }

    /// Gets the complete history for a specific statute.
    pub fn statute_history(
        records: &[AuditRecord],
        statute_id: &str,
    ) -> AuditResult<StatuteHistory> {
        let statute_records: Vec<_> = records
            .iter()
            .filter(|r| r.statute_id == statute_id)
            .cloned()
            .collect();

        if statute_records.is_empty() {
            return Err(AuditError::QueryError(format!(
                "No records found for statute {}",
                statute_id
            )));
        }

        let first_applied = statute_records.iter().map(|r| r.timestamp).min().unwrap();
        let last_applied = statute_records.iter().map(|r| r.timestamp).max().unwrap();

        let subjects_affected = Self::count_unique_subjects(&statute_records);

        Ok(StatuteHistory {
            statute_id: statute_id.to_string(),
            first_applied,
            last_applied,
            total_applications: statute_records.len(),
            subjects_affected,
            records: statute_records,
        })
    }

    /// Compares decisions between two points in time.
    pub fn compare_timepoints(
        records: &[AuditRecord],
        time1: DateTime<Utc>,
        time2: DateTime<Utc>,
    ) -> TimelineComparison {
        let state1 = Self::reconstruct_at_time(records, time1);
        let state2 = Self::reconstruct_at_time(records, time2);

        let decisions_added = state2.record_count - state1.record_count;
        let new_subjects = state2.subjects_affected as i64 - state1.subjects_affected as i64;
        let new_statutes = state2.statutes_applied as i64 - state1.statutes_applied as i64;

        TimelineComparison {
            time1,
            time2,
            state1,
            state2,
            decisions_added,
            new_subjects,
            new_statutes,
        }
    }

    /// Performs what-if analysis by filtering out certain decisions.
    pub fn what_if_without(
        records: &[AuditRecord],
        exclude_filter: impl Fn(&AuditRecord) -> bool,
    ) -> WhatIfAnalysis {
        let original_count = records.len();
        let filtered: Vec<_> = records
            .iter()
            .filter(|r| !exclude_filter(r))
            .cloned()
            .collect();
        let new_count = filtered.len();
        let removed_count = original_count - new_count;

        WhatIfAnalysis {
            original_count,
            new_count,
            removed_count,
            filtered_records: filtered,
        }
    }

    /// Counts unique subjects in a set of records.
    fn count_unique_subjects(records: &[AuditRecord]) -> usize {
        use std::collections::HashSet;
        let mut subjects = HashSet::new();
        for record in records {
            subjects.insert(record.subject_id);
        }
        subjects.len()
    }

    /// Counts unique statutes in a set of records.
    fn count_unique_statutes(records: &[AuditRecord]) -> usize {
        use std::collections::HashSet;
        let mut statutes = HashSet::new();
        for record in records {
            statutes.insert(record.statute_id.clone());
        }
        statutes.len()
    }
}

/// State of the audit trail at a specific point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineState {
    pub point_in_time: DateTime<Utc>,
    pub records: Vec<AuditRecord>,
    pub record_count: usize,
    pub subjects_affected: usize,
    pub statutes_applied: usize,
}

/// Complete history for a specific subject.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectHistory {
    pub subject_id: Uuid,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub total_decisions: usize,
    pub records: Vec<AuditRecord>,
}

/// Complete history for a specific statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteHistory {
    pub statute_id: String,
    pub first_applied: DateTime<Utc>,
    pub last_applied: DateTime<Utc>,
    pub total_applications: usize,
    pub subjects_affected: usize,
    pub records: Vec<AuditRecord>,
}

/// Comparison between two points in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineComparison {
    pub time1: DateTime<Utc>,
    pub time2: DateTime<Utc>,
    pub state1: TimelineState,
    pub state2: TimelineState,
    pub decisions_added: usize,
    pub new_subjects: i64,
    pub new_statutes: i64,
}

/// What-if analysis results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatIfAnalysis {
    pub original_count: usize,
    pub new_count: usize,
    pub removed_count: usize,
    pub filtered_records: Vec<AuditRecord>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use chrono::Duration;
    use std::collections::HashMap;

    fn create_test_record(statute_id: &str, subject_id: Uuid) -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            statute_id.to_string(),
            subject_id,
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_reconstruct_at_time() {
        let now = Utc::now();
        let past = now - Duration::hours(2);
        let future = now + Duration::hours(2);

        let mut record = create_test_record("statute-1", Uuid::new_v4());
        record.timestamp = now;

        let records = vec![record];

        let state_before = DecisionReplayer::reconstruct_at_time(&records, past);
        assert_eq!(state_before.record_count, 0);

        let state_now = DecisionReplayer::reconstruct_at_time(&records, now);
        assert_eq!(state_now.record_count, 1);

        let state_future = DecisionReplayer::reconstruct_at_time(&records, future);
        assert_eq!(state_future.record_count, 1);
    }

    #[test]
    fn test_subject_history() {
        let subject_id = Uuid::new_v4();
        let records = vec![
            create_test_record("statute-1", subject_id),
            create_test_record("statute-2", subject_id),
            create_test_record("statute-1", Uuid::new_v4()),
        ];

        let history = DecisionReplayer::subject_history(&records, subject_id).unwrap();
        assert_eq!(history.total_decisions, 2);
        assert_eq!(history.subject_id, subject_id);
    }

    #[test]
    fn test_statute_history() {
        let records = vec![
            create_test_record("statute-1", Uuid::new_v4()),
            create_test_record("statute-1", Uuid::new_v4()),
            create_test_record("statute-2", Uuid::new_v4()),
        ];

        let history = DecisionReplayer::statute_history(&records, "statute-1").unwrap();
        assert_eq!(history.total_applications, 2);
        assert_eq!(history.subjects_affected, 2);
    }

    #[test]
    fn test_what_if_analysis() {
        let records = vec![
            create_test_record("statute-1", Uuid::new_v4()),
            create_test_record("statute-2", Uuid::new_v4()),
            create_test_record("statute-1", Uuid::new_v4()),
        ];

        let analysis = DecisionReplayer::what_if_without(&records, |r| r.statute_id == "statute-1");

        assert_eq!(analysis.original_count, 3);
        assert_eq!(analysis.new_count, 1);
        assert_eq!(analysis.removed_count, 2);
    }
}
