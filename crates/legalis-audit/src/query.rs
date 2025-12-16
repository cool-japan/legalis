//! Query builder for flexible audit record filtering.

use crate::{Actor, AuditRecord, EventType};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Query builder for filtering audit records.
#[derive(Default, Clone)]
pub struct QueryBuilder {
    statute_ids: Vec<String>,
    subject_ids: Vec<Uuid>,
    event_types: Vec<EventType>,
    actor_filters: Vec<ActorFilter>,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<usize>,
    offset: Option<usize>,
}

/// Filter for actors.
#[derive(Clone)]
pub enum ActorFilter {
    /// Match system actors with specific component
    System { component: String },
    /// Match any system actor
    AnySystem,
    /// Match user actors with specific user ID
    User { user_id: String },
    /// Match user actors with specific role
    UserRole { role: String },
    /// Match any user actor
    AnyUser,
    /// Match external systems with specific ID
    External { system_id: String },
    /// Match any external actor
    AnyExternal,
}

impl QueryBuilder {
    /// Creates a new query builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filters by statute ID.
    pub fn statute_id(mut self, id: impl Into<String>) -> Self {
        self.statute_ids.push(id.into());
        self
    }

    /// Filters by multiple statute IDs.
    pub fn statute_ids(mut self, ids: Vec<String>) -> Self {
        self.statute_ids.extend(ids);
        self
    }

    /// Filters by subject ID.
    pub fn subject_id(mut self, id: Uuid) -> Self {
        self.subject_ids.push(id);
        self
    }

    /// Filters by multiple subject IDs.
    pub fn subject_ids(mut self, ids: Vec<Uuid>) -> Self {
        self.subject_ids.extend(ids);
        self
    }

    /// Filters by event type.
    pub fn event_type(mut self, event_type: EventType) -> Self {
        self.event_types.push(event_type);
        self
    }

    /// Filters by multiple event types.
    pub fn event_types(mut self, types: Vec<EventType>) -> Self {
        self.event_types.extend(types);
        self
    }

    /// Filters by actor.
    pub fn actor(mut self, filter: ActorFilter) -> Self {
        self.actor_filters.push(filter);
        self
    }

    /// Filters by time range start.
    pub fn start_time(mut self, time: DateTime<Utc>) -> Self {
        self.start_time = Some(time);
        self
    }

    /// Filters by time range end.
    pub fn end_time(mut self, time: DateTime<Utc>) -> Self {
        self.end_time = Some(time);
        self
    }

    /// Filters by time range.
    pub fn time_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.start_time = Some(start);
        self.end_time = Some(end);
        self
    }

    /// Limits the number of results.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the offset for pagination.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Executes the query on a list of records.
    pub fn execute(&self, records: &[AuditRecord]) -> Vec<AuditRecord> {
        let mut results: Vec<AuditRecord> = records
            .iter()
            .filter(|record| self.matches(record))
            .cloned()
            .collect();

        // Apply offset
        if let Some(offset) = self.offset {
            results = results.into_iter().skip(offset).collect();
        }

        // Apply limit
        if let Some(limit) = self.limit {
            results.truncate(limit);
        }

        results
    }

    /// Checks if a record matches the query filters.
    fn matches(&self, record: &AuditRecord) -> bool {
        // Check statute IDs
        if !self.statute_ids.is_empty() && !self.statute_ids.contains(&record.statute_id) {
            return false;
        }

        // Check subject IDs
        if !self.subject_ids.is_empty() && !self.subject_ids.contains(&record.subject_id) {
            return false;
        }

        // Check event types
        if !self.event_types.is_empty() {
            let matches = self
                .event_types
                .iter()
                .any(|et| std::mem::discriminant(et) == std::mem::discriminant(&record.event_type));
            if !matches {
                return false;
            }
        }

        // Check actor filters
        if !self.actor_filters.is_empty() {
            let matches = self.actor_filters.iter().any(|filter| match filter {
                ActorFilter::System { component } => {
                    matches!(&record.actor, Actor::System { component: c } if c == component)
                }
                ActorFilter::AnySystem => matches!(record.actor, Actor::System { .. }),
                ActorFilter::User { user_id } => {
                    matches!(&record.actor, Actor::User { user_id: u, .. } if u == user_id)
                }
                ActorFilter::UserRole { role } => {
                    matches!(&record.actor, Actor::User { role: r, .. } if r == role)
                }
                ActorFilter::AnyUser => matches!(record.actor, Actor::User { .. }),
                ActorFilter::External { system_id } => {
                    matches!(&record.actor, Actor::External { system_id: s } if s == system_id)
                }
                ActorFilter::AnyExternal => matches!(record.actor, Actor::External { .. }),
            });
            if !matches {
                return false;
            }
        }

        // Check time range
        if let Some(start) = self.start_time {
            if record.timestamp < start {
                return false;
            }
        }
        if let Some(end) = self.end_time {
            if record.timestamp > end {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DecisionContext, DecisionResult};
    use std::collections::HashMap;

    #[test]
    fn test_query_builder() {
        let records = vec![
            AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "engine".to_string(),
                },
                "statute-1".to_string(),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "test".to_string(),
                    parameters: HashMap::new(),
                },
                None,
            ),
            AuditRecord::new(
                EventType::HumanOverride,
                Actor::User {
                    user_id: "user-1".to_string(),
                    role: "admin".to_string(),
                },
                "statute-2".to_string(),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "override".to_string(),
                    parameters: HashMap::new(),
                },
                None,
            ),
        ];

        // Test filtering by statute
        let query = QueryBuilder::new().statute_id("statute-1");
        let results = query.execute(&records);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].statute_id, "statute-1");

        // Test filtering by actor
        let query = QueryBuilder::new().actor(ActorFilter::AnyUser);
        let results = query.execute(&records);
        assert_eq!(results.len(), 1);
        assert!(matches!(results[0].actor, Actor::User { .. }));

        // Test limit
        let query = QueryBuilder::new().limit(1);
        let results = query.execute(&records);
        assert_eq!(results.len(), 1);
    }
}
