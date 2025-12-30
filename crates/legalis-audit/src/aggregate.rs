//! Aggregate queries for audit trail analytics.
//!
//! This module provides aggregate query capabilities for analyzing audit trails:
//! - Count by statute, outcome, actor, event type
//! - Time-series aggregations
//! - Statistical summaries
//! - Custom grouping and aggregations

use crate::{AuditRecord, AuditResult, EventType};
use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type alias for filter predicates in aggregate queries.
type FilterPredicate = Box<dyn Fn(&AuditRecord) -> bool + Send + Sync>;

/// Aggregation dimension for grouping records.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AggregationDimension {
    /// Group by statute ID
    Statute,
    /// Group by subject ID
    Subject,
    /// Group by event type
    EventType,
    /// Group by actor type (System/User/External)
    ActorType,
    /// Group by specific actor
    Actor,
    /// Group by decision outcome
    Outcome,
    /// Group by year
    Year,
    /// Group by month
    Month,
    /// Group by day
    Day,
    /// Group by hour
    Hour,
}

/// Aggregation function to apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregationFunction {
    /// Count records
    Count,
    /// Count distinct values
    CountDistinct,
}

/// Result of an aggregation query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationResult {
    /// Dimension used for grouping
    pub dimension: AggregationDimension,
    /// Function applied
    pub function: AggregationFunction,
    /// Aggregated values (group key -> value)
    pub values: HashMap<String, usize>,
    /// Total records processed
    pub total_records: usize,
}

impl AggregationResult {
    /// Gets the top N entries by value.
    pub fn top(&self, n: usize) -> Vec<(String, usize)> {
        let mut entries: Vec<_> = self.values.iter().map(|(k, v)| (k.clone(), *v)).collect();
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries.into_iter().take(n).collect()
    }

    /// Gets the bottom N entries by value.
    pub fn bottom(&self, n: usize) -> Vec<(String, usize)> {
        let mut entries: Vec<_> = self.values.iter().map(|(k, v)| (k.clone(), *v)).collect();
        entries.sort_by(|a, b| a.1.cmp(&b.1));
        entries.into_iter().take(n).collect()
    }

    /// Gets total count across all groups.
    pub fn total(&self) -> usize {
        self.values.values().sum()
    }

    /// Gets average value across all groups.
    pub fn average(&self) -> f64 {
        if self.values.is_empty() {
            0.0
        } else {
            self.total() as f64 / self.values.len() as f64
        }
    }
}

/// Aggregate query builder.
pub struct AggregateQuery {
    dimension: AggregationDimension,
    function: AggregationFunction,
    filters: Vec<FilterPredicate>,
}

impl AggregateQuery {
    /// Creates a new aggregate query.
    pub fn new(dimension: AggregationDimension) -> Self {
        Self {
            dimension,
            function: AggregationFunction::Count,
            filters: Vec::new(),
        }
    }

    /// Sets the aggregation function.
    pub fn function(mut self, function: AggregationFunction) -> Self {
        self.function = function;
        self
    }

    /// Adds a filter to the query.
    pub fn filter<F>(mut self, f: F) -> Self
    where
        F: Fn(&AuditRecord) -> bool + Send + Sync + 'static,
    {
        self.filters.push(Box::new(f));
        self
    }

    /// Filters by statute ID.
    pub fn statute(self, statute_id: String) -> Self {
        self.filter(move |r| r.statute_id == statute_id)
    }

    /// Filters by event type.
    pub fn event_type(self, event_type: EventType) -> Self {
        self.filter(move |r| {
            std::mem::discriminant(&r.event_type) == std::mem::discriminant(&event_type)
        })
    }

    /// Filters by time range.
    pub fn time_range(self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.filter(move |r| r.timestamp >= start && r.timestamp <= end)
    }

    /// Executes the aggregate query.
    pub fn execute(&self, records: &[AuditRecord]) -> AuditResult<AggregationResult> {
        // Apply filters
        let filtered: Vec<_> = records
            .iter()
            .filter(|r| self.filters.iter().all(|f| f(r)))
            .collect();

        let total_records = filtered.len();
        let mut values: HashMap<String, usize> = HashMap::new();

        match self.function {
            AggregationFunction::Count => {
                for record in &filtered {
                    let key = self.get_dimension_key(record);
                    *values.entry(key).or_insert(0) += 1;
                }
            }
            AggregationFunction::CountDistinct => {
                let mut seen: HashMap<String, std::collections::HashSet<String>> = HashMap::new();
                for record in &filtered {
                    let key = self.get_dimension_key(record);
                    let value = record.id.to_string();
                    seen.entry(key).or_default().insert(value);
                }
                for (key, set) in seen {
                    values.insert(key, set.len());
                }
            }
        }

        Ok(AggregationResult {
            dimension: self.dimension.clone(),
            function: self.function,
            values,
            total_records,
        })
    }

    /// Gets the key for a record based on the dimension.
    fn get_dimension_key(&self, record: &AuditRecord) -> String {
        match &self.dimension {
            AggregationDimension::Statute => record.statute_id.clone(),
            AggregationDimension::Subject => record.subject_id.to_string(),
            AggregationDimension::EventType => format!("{:?}", record.event_type),
            AggregationDimension::ActorType => match &record.actor {
                crate::Actor::System { .. } => "System".to_string(),
                crate::Actor::User { .. } => "User".to_string(),
                crate::Actor::External { .. } => "External".to_string(),
            },
            AggregationDimension::Actor => match &record.actor {
                crate::Actor::System { component } => format!("System:{}", component),
                crate::Actor::User { user_id, role } => format!("User:{}:{}", user_id, role),
                crate::Actor::External { system_id } => format!("External:{}", system_id),
            },
            AggregationDimension::Outcome => match &record.result {
                crate::DecisionResult::Deterministic { effect_applied, .. } => {
                    format!("Deterministic:{}", effect_applied)
                }
                crate::DecisionResult::RequiresDiscretion { .. } => {
                    "RequiresDiscretion".to_string()
                }
                crate::DecisionResult::Void { .. } => "Void".to_string(),
                crate::DecisionResult::Overridden { .. } => "Overridden".to_string(),
            },
            AggregationDimension::Year => record.timestamp.year().to_string(),
            AggregationDimension::Month => format!(
                "{}-{:02}",
                record.timestamp.year(),
                record.timestamp.month()
            ),
            AggregationDimension::Day => format!(
                "{}-{:02}-{:02}",
                record.timestamp.year(),
                record.timestamp.month(),
                record.timestamp.day()
            ),
            AggregationDimension::Hour => format!(
                "{}-{:02}-{:02} {:02}:00",
                record.timestamp.year(),
                record.timestamp.month(),
                record.timestamp.day(),
                record.timestamp.hour()
            ),
        }
    }
}

/// Multi-dimensional aggregation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiAggregationResult {
    /// Individual aggregation results
    pub results: Vec<AggregationResult>,
}

impl MultiAggregationResult {
    /// Creates a new multi-aggregation result.
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Adds an aggregation result.
    pub fn add(&mut self, result: AggregationResult) {
        self.results.push(result);
    }

    /// Gets a result by dimension.
    pub fn get(&self, dimension: &AggregationDimension) -> Option<&AggregationResult> {
        self.results.iter().find(|r| &r.dimension == dimension)
    }
}

impl Default for MultiAggregationResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult};
    use std::collections::HashMap as StdHashMap;
    use uuid::Uuid;

    fn create_test_records() -> Vec<AuditRecord> {
        let mut records = Vec::new();
        for i in 0..10 {
            let record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: format!("component-{}", i % 3),
                },
                format!("statute-{}", i % 2),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "approved".to_string(),
                    parameters: StdHashMap::new(),
                },
                None,
            );
            records.push(record);
        }
        records
    }

    #[test]
    fn test_aggregate_by_statute() {
        let records = create_test_records();
        let query = AggregateQuery::new(AggregationDimension::Statute);
        let result = query.execute(&records).unwrap();

        assert_eq!(result.values.len(), 2);
        assert_eq!(result.total_records, 10);
        assert_eq!(*result.values.get("statute-0").unwrap(), 5);
        assert_eq!(*result.values.get("statute-1").unwrap(), 5);
    }

    #[test]
    fn test_aggregate_by_actor_type() {
        let records = create_test_records();
        let query = AggregateQuery::new(AggregationDimension::ActorType);
        let result = query.execute(&records).unwrap();

        assert_eq!(result.values.len(), 1);
        assert_eq!(*result.values.get("System").unwrap(), 10);
    }

    #[test]
    fn test_aggregate_top_n() {
        let records = create_test_records();
        let query = AggregateQuery::new(AggregationDimension::Statute);
        let result = query.execute(&records).unwrap();

        let top = result.top(1);
        assert_eq!(top.len(), 1);
        assert_eq!(top[0].1, 5);
    }

    #[test]
    fn test_aggregate_with_filter() {
        let records = create_test_records();
        let query =
            AggregateQuery::new(AggregationDimension::Statute).statute("statute-0".to_string());
        let result = query.execute(&records).unwrap();

        assert_eq!(result.total_records, 5);
        assert_eq!(result.values.len(), 1);
    }

    #[test]
    fn test_aggregate_average() {
        let records = create_test_records();
        let query = AggregateQuery::new(AggregationDimension::Statute);
        let result = query.execute(&records).unwrap();

        assert_eq!(result.average(), 5.0);
    }

    #[test]
    fn test_multi_aggregation() {
        let records = create_test_records();

        let mut multi = MultiAggregationResult::new();

        let statute_result = AggregateQuery::new(AggregationDimension::Statute)
            .execute(&records)
            .unwrap();
        multi.add(statute_result);

        let actor_result = AggregateQuery::new(AggregationDimension::ActorType)
            .execute(&records)
            .unwrap();
        multi.add(actor_result);

        assert_eq!(multi.results.len(), 2);
        assert!(multi.get(&AggregationDimension::Statute).is_some());
        assert!(multi.get(&AggregationDimension::ActorType).is_some());
    }
}
