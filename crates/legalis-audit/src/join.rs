//! Join queries across multiple audit trails.
//!
//! This module provides functionality to query and correlate records across
//! multiple audit trail instances. This is useful for:
//! - Cross-system analysis
//! - Multi-jurisdiction reporting
//! - Distributed audit trail correlation
//! - Temporal correlation of events

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A named source of audit records for join operations.
#[derive(Debug, Clone)]
pub struct AuditTrailSource {
    /// Name/identifier for this source
    pub name: String,
    /// Records from this source
    pub records: Vec<AuditRecord>,
}

impl AuditTrailSource {
    /// Creates a new audit trail source.
    pub fn new(name: String, records: Vec<AuditRecord>) -> Self {
        Self { name, records }
    }
}

/// Type of join operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JoinType {
    /// Inner join - only matching records from both sources
    Inner,
    /// Left join - all records from left source, matching from right
    Left,
    /// Right join - all records from right source, matching from left
    Right,
    /// Full outer join - all records from both sources
    FullOuter,
}

/// Join condition for matching records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JoinCondition {
    /// Join on subject ID
    OnSubjectId,
    /// Join on statute ID
    OnStatuteId,
    /// Join on actor (exact match)
    OnActor,
    /// Join on time window (records within N seconds)
    OnTimeWindow(i64),
    /// Custom condition using metadata key
    OnMetadata { key: String },
}

impl JoinCondition {
    /// Checks if two records match this condition.
    fn matches(&self, left: &AuditRecord, right: &AuditRecord) -> bool {
        match self {
            JoinCondition::OnSubjectId => left.subject_id == right.subject_id,
            JoinCondition::OnStatuteId => left.statute_id == right.statute_id,
            JoinCondition::OnActor => {
                // Simplified actor comparison
                format!("{:?}", left.actor) == format!("{:?}", right.actor)
            }
            JoinCondition::OnTimeWindow(seconds) => {
                let diff = (left.timestamp - right.timestamp).num_seconds().abs();
                diff <= *seconds
            }
            JoinCondition::OnMetadata { key } => {
                left.context.metadata.get(key) == right.context.metadata.get(key)
                    && left.context.metadata.contains_key(key)
            }
        }
    }
}

/// A joined record from two sources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinedRecord {
    /// Record from left source (if exists)
    pub left: Option<AuditRecord>,
    /// Record from right source (if exists)
    pub right: Option<AuditRecord>,
    /// Source name for left record
    pub left_source: Option<String>,
    /// Source name for right record
    pub right_source: Option<String>,
}

impl JoinedRecord {
    /// Gets the timestamp of the earliest record.
    pub fn earliest_timestamp(&self) -> Option<DateTime<Utc>> {
        match (&self.left, &self.right) {
            (Some(l), Some(r)) => Some(l.timestamp.min(r.timestamp)),
            (Some(l), None) => Some(l.timestamp),
            (None, Some(r)) => Some(r.timestamp),
            (None, None) => None,
        }
    }

    /// Gets the subject ID if both records have the same subject.
    pub fn common_subject_id(&self) -> Option<Uuid> {
        match (&self.left, &self.right) {
            (Some(l), Some(r)) if l.subject_id == r.subject_id => Some(l.subject_id),
            (Some(l), None) => Some(l.subject_id),
            (None, Some(r)) => Some(r.subject_id),
            _ => None,
        }
    }
}

/// Builder for constructing join queries.
pub struct JoinQueryBuilder {
    left_source: Option<AuditTrailSource>,
    right_source: Option<AuditTrailSource>,
    join_type: JoinType,
    conditions: Vec<JoinCondition>,
}

impl JoinQueryBuilder {
    /// Creates a new join query builder.
    pub fn new() -> Self {
        Self {
            left_source: None,
            right_source: None,
            join_type: JoinType::Inner,
            conditions: Vec::new(),
        }
    }

    /// Sets the left source.
    pub fn left(mut self, source: AuditTrailSource) -> Self {
        self.left_source = Some(source);
        self
    }

    /// Sets the right source.
    pub fn right(mut self, source: AuditTrailSource) -> Self {
        self.right_source = Some(source);
        self
    }

    /// Sets the join type.
    pub fn join_type(mut self, join_type: JoinType) -> Self {
        self.join_type = join_type;
        self
    }

    /// Adds a join condition.
    pub fn on(mut self, condition: JoinCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Executes the join query.
    pub fn execute(self) -> AuditResult<Vec<JoinedRecord>> {
        if self.conditions.is_empty() {
            return Err(AuditError::QueryError(
                "At least one join condition required".to_string(),
            ));
        }

        let conditions = self.conditions;
        let join_type = self.join_type;

        let left_source = self
            .left_source
            .ok_or_else(|| AuditError::QueryError("Left source not specified".to_string()))?;
        let right_source = self
            .right_source
            .ok_or_else(|| AuditError::QueryError("Right source not specified".to_string()))?;

        let mut results = Vec::new();
        let mut right_matched = vec![false; right_source.records.len()];

        // Helper to match conditions
        let matches_all = |left: &AuditRecord, right: &AuditRecord| {
            conditions.iter().all(|cond| cond.matches(left, right))
        };

        // Process based on join type
        match join_type {
            JoinType::Inner => {
                for left_record in &left_source.records {
                    for (right_idx, right_record) in right_source.records.iter().enumerate() {
                        if matches_all(left_record, right_record) {
                            results.push(JoinedRecord {
                                left: Some(left_record.clone()),
                                right: Some(right_record.clone()),
                                left_source: Some(left_source.name.clone()),
                                right_source: Some(right_source.name.clone()),
                            });
                            right_matched[right_idx] = true;
                        }
                    }
                }
            }
            JoinType::Left => {
                for left_record in &left_source.records {
                    let mut found_match = false;
                    for (right_idx, right_record) in right_source.records.iter().enumerate() {
                        if matches_all(left_record, right_record) {
                            results.push(JoinedRecord {
                                left: Some(left_record.clone()),
                                right: Some(right_record.clone()),
                                left_source: Some(left_source.name.clone()),
                                right_source: Some(right_source.name.clone()),
                            });
                            right_matched[right_idx] = true;
                            found_match = true;
                        }
                    }
                    if !found_match {
                        results.push(JoinedRecord {
                            left: Some(left_record.clone()),
                            right: None,
                            left_source: Some(left_source.name.clone()),
                            right_source: None,
                        });
                    }
                }
            }
            JoinType::Right => {
                for (right_idx, right_record) in right_source.records.iter().enumerate() {
                    let mut found_match = false;
                    for left_record in &left_source.records {
                        if matches_all(left_record, right_record) {
                            results.push(JoinedRecord {
                                left: Some(left_record.clone()),
                                right: Some(right_record.clone()),
                                left_source: Some(left_source.name.clone()),
                                right_source: Some(right_source.name.clone()),
                            });
                            found_match = true;
                        }
                    }
                    if !found_match {
                        results.push(JoinedRecord {
                            left: None,
                            right: Some(right_record.clone()),
                            left_source: None,
                            right_source: Some(right_source.name.clone()),
                        });
                    }
                    right_matched[right_idx] = true;
                }
            }
            JoinType::FullOuter => {
                // First do left join
                for left_record in &left_source.records {
                    let mut found_match = false;
                    for (right_idx, right_record) in right_source.records.iter().enumerate() {
                        if matches_all(left_record, right_record) {
                            results.push(JoinedRecord {
                                left: Some(left_record.clone()),
                                right: Some(right_record.clone()),
                                left_source: Some(left_source.name.clone()),
                                right_source: Some(right_source.name.clone()),
                            });
                            right_matched[right_idx] = true;
                            found_match = true;
                        }
                    }
                    if !found_match {
                        results.push(JoinedRecord {
                            left: Some(left_record.clone()),
                            right: None,
                            left_source: Some(left_source.name.clone()),
                            right_source: None,
                        });
                    }
                }
                // Add unmatched right records
                for (right_idx, right_record) in right_source.records.iter().enumerate() {
                    if !right_matched[right_idx] {
                        results.push(JoinedRecord {
                            left: None,
                            right: Some(right_record.clone()),
                            left_source: None,
                            right_source: Some(right_source.name.clone()),
                        });
                    }
                }
            }
        }

        Ok(results)
    }
}

impl Default for JoinQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-way join across more than two sources.
pub struct MultiJoinBuilder {
    sources: Vec<AuditTrailSource>,
    join_type: JoinType,
    conditions: Vec<JoinCondition>,
}

impl MultiJoinBuilder {
    /// Creates a new multi-join builder.
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            join_type: JoinType::Inner,
            conditions: Vec::new(),
        }
    }

    /// Adds a source to the join.
    pub fn add_source(mut self, source: AuditTrailSource) -> Self {
        self.sources.push(source);
        self
    }

    /// Sets the join type.
    pub fn join_type(mut self, join_type: JoinType) -> Self {
        self.join_type = join_type;
        self
    }

    /// Adds a join condition.
    pub fn on(mut self, condition: JoinCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Executes the multi-join.
    pub fn execute(self) -> AuditResult<Vec<MultiJoinedRecord>> {
        if self.sources.len() < 2 {
            return Err(AuditError::QueryError(
                "At least two sources required for join".to_string(),
            ));
        }

        // For simplicity, perform sequential joins
        // Start with first two sources
        let mut current_sources = self.sources;
        let first = current_sources.remove(0);
        let second = current_sources.remove(0);

        let mut join_builder = JoinQueryBuilder::new()
            .left(first)
            .right(second)
            .join_type(self.join_type);

        for condition in &self.conditions {
            join_builder = join_builder.on(condition.clone());
        }

        let initial_results = join_builder.execute()?;

        // Convert to multi-joined records
        let results: Vec<MultiJoinedRecord> = initial_results
            .into_iter()
            .map(|jr| {
                let mut records = HashMap::new();
                if let (Some(left), Some(source)) = (jr.left, jr.left_source) {
                    records.insert(source, left);
                }
                if let (Some(right), Some(source)) = (jr.right, jr.right_source) {
                    records.insert(source, right);
                }
                MultiJoinedRecord { records }
            })
            .collect();

        Ok(results)
    }
}

impl Default for MultiJoinBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A record joined from multiple sources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiJoinedRecord {
    /// Map from source name to audit record
    pub records: HashMap<String, AuditRecord>,
}

impl MultiJoinedRecord {
    /// Gets a record from a specific source.
    pub fn get_record(&self, source: &str) -> Option<&AuditRecord> {
        self.records.get(source)
    }

    /// Gets all source names.
    pub fn sources(&self) -> Vec<String> {
        self.records.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;

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
                parameters: StdHashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_join_on_subject_id() {
        let subject1 = Uuid::new_v4();
        let subject2 = Uuid::new_v4();

        let left_records = vec![
            create_test_record("statute-1", subject1),
            create_test_record("statute-2", subject2),
        ];

        let right_records = vec![
            create_test_record("statute-3", subject1),
            create_test_record("statute-4", Uuid::new_v4()),
        ];

        let left_source = AuditTrailSource::new("left".to_string(), left_records);
        let right_source = AuditTrailSource::new("right".to_string(), right_records);

        let results = JoinQueryBuilder::new()
            .left(left_source)
            .right(right_source)
            .join_type(JoinType::Inner)
            .on(JoinCondition::OnSubjectId)
            .execute()
            .unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].left.is_some());
        assert!(results[0].right.is_some());
    }

    #[test]
    fn test_left_join() {
        let subject1 = Uuid::new_v4();
        let subject2 = Uuid::new_v4();

        let left_records = vec![
            create_test_record("statute-1", subject1),
            create_test_record("statute-2", subject2),
        ];

        let right_records = vec![create_test_record("statute-3", subject1)];

        let left_source = AuditTrailSource::new("left".to_string(), left_records);
        let right_source = AuditTrailSource::new("right".to_string(), right_records);

        let results = JoinQueryBuilder::new()
            .left(left_source)
            .right(right_source)
            .join_type(JoinType::Left)
            .on(JoinCondition::OnSubjectId)
            .execute()
            .unwrap();

        assert_eq!(results.len(), 2);
        assert!(results[0].left.is_some());
        assert!(results[1].left.is_some());
        assert!(results[1].right.is_none());
    }

    #[test]
    fn test_join_on_statute() {
        let left_records = vec![
            create_test_record("statute-1", Uuid::new_v4()),
            create_test_record("statute-2", Uuid::new_v4()),
        ];

        let right_records = vec![create_test_record("statute-1", Uuid::new_v4())];

        let left_source = AuditTrailSource::new("left".to_string(), left_records);
        let right_source = AuditTrailSource::new("right".to_string(), right_records);

        let results = JoinQueryBuilder::new()
            .left(left_source)
            .right(right_source)
            .join_type(JoinType::Inner)
            .on(JoinCondition::OnStatuteId)
            .execute()
            .unwrap();

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_full_outer_join() {
        let subject1 = Uuid::new_v4();
        let subject2 = Uuid::new_v4();
        let subject3 = Uuid::new_v4();

        let left_records = vec![
            create_test_record("statute-1", subject1),
            create_test_record("statute-2", subject2),
        ];

        let right_records = vec![
            create_test_record("statute-3", subject1),
            create_test_record("statute-4", subject3),
        ];

        let left_source = AuditTrailSource::new("left".to_string(), left_records);
        let right_source = AuditTrailSource::new("right".to_string(), right_records);

        let results = JoinQueryBuilder::new()
            .left(left_source)
            .right(right_source)
            .join_type(JoinType::FullOuter)
            .on(JoinCondition::OnSubjectId)
            .execute()
            .unwrap();

        assert_eq!(results.len(), 3); // 1 match + 1 left-only + 1 right-only
    }

    #[test]
    fn test_joined_record_helpers() {
        let subject = Uuid::new_v4();
        let record1 = create_test_record("statute-1", subject);
        let record2 = create_test_record("statute-2", subject);

        let joined = JoinedRecord {
            left: Some(record1.clone()),
            right: Some(record2.clone()),
            left_source: Some("left".to_string()),
            right_source: Some("right".to_string()),
        };

        assert_eq!(joined.common_subject_id(), Some(subject));
        assert!(joined.earliest_timestamp().is_some());
    }

    #[test]
    fn test_multi_join() {
        let subject = Uuid::new_v4();

        let source1 = AuditTrailSource::new(
            "source1".to_string(),
            vec![create_test_record("statute-1", subject)],
        );
        let source2 = AuditTrailSource::new(
            "source2".to_string(),
            vec![create_test_record("statute-2", subject)],
        );

        let results = MultiJoinBuilder::new()
            .add_source(source1)
            .add_source(source2)
            .join_type(JoinType::Inner)
            .on(JoinCondition::OnSubjectId)
            .execute()
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].sources().len(), 2);
    }

    #[test]
    fn test_join_requires_condition() {
        let left_source = AuditTrailSource::new("left".to_string(), vec![]);
        let right_source = AuditTrailSource::new("right".to_string(), vec![]);

        let result = JoinQueryBuilder::new()
            .left(left_source)
            .right(right_source)
            .execute();

        assert!(result.is_err());
    }
}
