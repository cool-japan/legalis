//! Temporal RDF with RDF-star support for time-aware legal knowledge graphs.
//!
//! This module provides temporal extensions to RDF using RDF-star (RDF 1.2) for
//! representing time-varying facts and legal provisions.
//!
//! ## Features
//! - Valid time (when a fact is true in the real world)
//! - Transaction time (when a fact was recorded in the database)
//! - Bitemporal modeling (both valid and transaction time)
//! - Temporal annotations using RDF-star
//! - Time intervals and instants

use crate::{RdfValue, Triple};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Temporal namespace
pub const TEMPORAL_NS: &str = "https://legalis.dev/temporal#";

/// Time ontology namespace (W3C)
pub const TIME_NS: &str = "http://www.w3.org/2006/time#";

/// A temporal triple with time annotations
#[derive(Debug, Clone, PartialEq)]
pub struct TemporalTriple {
    /// The base RDF triple
    pub triple: Triple,
    /// Valid time period (when the fact is/was true)
    pub valid_time: Option<TimePeriod>,
    /// Transaction time period (when the fact was recorded)
    pub transaction_time: Option<TimePeriod>,
}

impl TemporalTriple {
    /// Creates a new temporal triple
    pub fn new(triple: Triple) -> Self {
        Self {
            triple,
            valid_time: None,
            transaction_time: None,
        }
    }

    /// Sets the valid time period
    pub fn with_valid_time(mut self, period: TimePeriod) -> Self {
        self.valid_time = Some(period);
        self
    }

    /// Sets the transaction time period
    pub fn with_transaction_time(mut self, period: TimePeriod) -> Self {
        self.transaction_time = Some(period);
        self
    }

    /// Converts to RDF-star quoted triples
    pub fn to_rdfstar_triples(&self, base_uri: &str) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Add the base triple
        triples.push(self.triple.clone());

        // Create a unique identifier for this temporal assertion
        let assertion_id = format!("{}temporal/{}", base_uri, generate_id());

        // Valid time annotations
        if let Some(ref vt) = self.valid_time {
            triples.push(Triple {
                subject: assertion_id.clone(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("temporal:TemporalAssertion".to_string()),
            });

            triples.push(Triple {
                subject: assertion_id.clone(),
                predicate: "temporal:assertedTriple".to_string(),
                object: RdfValue::Uri(format!(
                    "<<{} {} {}>>",
                    self.triple.subject, self.triple.predicate, self.triple.object
                )),
            });

            triples.push(Triple {
                subject: assertion_id.clone(),
                predicate: "temporal:validFrom".to_string(),
                object: RdfValue::datetime(vt.start),
            });

            if let Some(end) = vt.end {
                triples.push(Triple {
                    subject: assertion_id.clone(),
                    predicate: "temporal:validTo".to_string(),
                    object: RdfValue::datetime(end),
                });
            }
        }

        // Transaction time annotations
        if let Some(ref tt) = self.transaction_time {
            triples.push(Triple {
                subject: assertion_id.clone(),
                predicate: "temporal:transactionFrom".to_string(),
                object: RdfValue::datetime(tt.start),
            });

            if let Some(end) = tt.end {
                triples.push(Triple {
                    subject: assertion_id.clone(),
                    predicate: "temporal:transactionTo".to_string(),
                    object: RdfValue::datetime(end),
                });
            }
        }

        triples
    }

    /// Checks if this triple is valid at a given time
    pub fn is_valid_at(&self, time: DateTime<Utc>) -> bool {
        if let Some(ref vt) = self.valid_time {
            vt.contains(time)
        } else {
            true // If no valid time specified, assume always valid
        }
    }

    /// Checks if this triple was recorded at a given time
    pub fn was_recorded_at(&self, time: DateTime<Utc>) -> bool {
        if let Some(ref tt) = self.transaction_time {
            tt.contains(time)
        } else {
            true // If no transaction time specified, assume always recorded
        }
    }
}

/// A time period with start and optional end
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimePeriod {
    /// Start of the period (inclusive)
    pub start: DateTime<Utc>,
    /// End of the period (exclusive), None means ongoing
    pub end: Option<DateTime<Utc>>,
}

impl TimePeriod {
    /// Creates a new time period
    pub fn new(start: DateTime<Utc>, end: Option<DateTime<Utc>>) -> Self {
        Self { start, end }
    }

    /// Creates an instant (point in time)
    pub fn instant(time: DateTime<Utc>) -> Self {
        Self {
            start: time,
            end: Some(time),
        }
    }

    /// Creates an ongoing period (no end time)
    pub fn ongoing(start: DateTime<Utc>) -> Self {
        Self { start, end: None }
    }

    /// Checks if this period contains a given time
    pub fn contains(&self, time: DateTime<Utc>) -> bool {
        if time < self.start {
            return false;
        }
        if let Some(end) = self.end {
            // For instants (start == end), we need inclusive check
            if self.start == end {
                time == self.start
            } else {
                time < end
            }
        } else {
            true // Ongoing period
        }
    }

    /// Checks if this period overlaps with another
    pub fn overlaps(&self, other: &TimePeriod) -> bool {
        if let (Some(self_end), Some(other_end)) = (self.end, other.end) {
            // Both have end times
            self.start < other_end && other.start < self_end
        } else if self.end.is_none() && other.end.is_none() {
            // Both ongoing
            true
        } else if let Some(self_end) = self.end {
            // Self has end, other ongoing
            other.start < self_end
        } else {
            // Self ongoing, other has end
            if let Some(other_end) = other.end {
                self.start < other_end
            } else {
                true
            }
        }
    }

    /// Returns the duration in seconds (if period has an end)
    pub fn duration_secs(&self) -> Option<i64> {
        self.end.map(|end| end.timestamp() - self.start.timestamp())
    }
}

/// Bitemporal triple with both valid and transaction time
#[derive(Debug, Clone, PartialEq)]
pub struct BitemporalTriple {
    /// The temporal triple
    pub temporal: TemporalTriple,
}

impl BitemporalTriple {
    /// Creates a new bitemporal triple
    pub fn new(triple: Triple, valid_time: TimePeriod, transaction_time: TimePeriod) -> Self {
        Self {
            temporal: TemporalTriple::new(triple)
                .with_valid_time(valid_time)
                .with_transaction_time(transaction_time),
        }
    }

    /// Checks if this triple is valid and recorded at given times
    pub fn is_valid_and_recorded_at(
        &self,
        valid_at: DateTime<Utc>,
        transaction_at: DateTime<Utc>,
    ) -> bool {
        self.temporal.is_valid_at(valid_at) && self.temporal.was_recorded_at(transaction_at)
    }
}

/// Temporal graph that stores time-varying triples
#[derive(Debug, Clone)]
pub struct TemporalGraph {
    /// Base namespace
    pub base_uri: String,
    /// Temporal triples
    pub triples: Vec<TemporalTriple>,
}

impl TemporalGraph {
    /// Creates a new temporal graph
    pub fn new(base_uri: impl Into<String>) -> Self {
        Self {
            base_uri: base_uri.into(),
            triples: Vec::new(),
        }
    }

    /// Adds a temporal triple
    pub fn add_triple(&mut self, triple: TemporalTriple) {
        self.triples.push(triple);
    }

    /// Queries triples valid at a specific time
    pub fn query_at_time(&self, time: DateTime<Utc>) -> Vec<&TemporalTriple> {
        self.triples
            .iter()
            .filter(|t| t.is_valid_at(time))
            .collect()
    }

    /// Queries triples valid during a time period
    pub fn query_during_period(&self, period: &TimePeriod) -> Vec<&TemporalTriple> {
        self.triples
            .iter()
            .filter(|t| {
                if let Some(ref vt) = t.valid_time {
                    vt.overlaps(period)
                } else {
                    true
                }
            })
            .collect()
    }

    /// Converts the entire graph to RDF-star triples
    pub fn to_rdfstar_triples(&self) -> Vec<Triple> {
        let mut result = Vec::new();
        for temporal_triple in &self.triples {
            result.extend(temporal_triple.to_rdfstar_triples(&self.base_uri));
        }
        result
    }

    /// Gets the temporal history of a subject
    pub fn history_of(&self, subject: &str) -> Vec<&TemporalTriple> {
        self.triples
            .iter()
            .filter(|t| t.triple.subject == subject)
            .collect()
    }
}

/// Temporal snapshot of the graph at a specific time
#[derive(Debug, Clone)]
pub struct TemporalSnapshot {
    /// Snapshot time
    pub time: DateTime<Utc>,
    /// Triples valid at this time
    pub triples: Vec<Triple>,
}

impl TemporalSnapshot {
    /// Creates a snapshot from a temporal graph
    pub fn from_graph(graph: &TemporalGraph, time: DateTime<Utc>) -> Self {
        let triples = graph
            .query_at_time(time)
            .into_iter()
            .map(|t| t.triple.clone())
            .collect();

        Self { time, triples }
    }
}

/// Generates a unique ID for temporal assertions
fn generate_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_triple() -> Triple {
        Triple {
            subject: "ex:law1".to_string(),
            predicate: "ex:hasStatus".to_string(),
            object: RdfValue::string("active"),
        }
    }

    #[test]
    fn test_time_period_contains() {
        let start = Utc::now();
        let end = start + chrono::Duration::hours(1);
        let period = TimePeriod::new(start, Some(end));

        let middle = start + chrono::Duration::minutes(30);
        assert!(period.contains(middle));

        let before = start - chrono::Duration::minutes(1);
        assert!(!period.contains(before));

        let after = end + chrono::Duration::minutes(1);
        assert!(!period.contains(after));
    }

    #[test]
    fn test_time_period_overlaps() {
        let start1 = Utc::now();
        let end1 = start1 + chrono::Duration::hours(2);
        let period1 = TimePeriod::new(start1, Some(end1));

        let start2 = start1 + chrono::Duration::hours(1);
        let end2 = start2 + chrono::Duration::hours(2);
        let period2 = TimePeriod::new(start2, Some(end2));

        assert!(period1.overlaps(&period2));
        assert!(period2.overlaps(&period1));
    }

    #[test]
    fn test_ongoing_period() {
        let start = Utc::now();
        let period = TimePeriod::ongoing(start);

        let future = start + chrono::Duration::days(365);
        assert!(period.contains(future));

        assert!(period.end.is_none());
    }

    #[test]
    fn test_temporal_triple_valid_at() {
        let triple = sample_triple();
        let now = Utc::now();
        let future = now + chrono::Duration::hours(1);

        let period = TimePeriod::new(now, Some(future));
        let temp_triple = TemporalTriple::new(triple).with_valid_time(period);

        let middle = now + chrono::Duration::minutes(30);
        assert!(temp_triple.is_valid_at(middle));

        let way_future = future + chrono::Duration::hours(1);
        assert!(!temp_triple.is_valid_at(way_future));
    }

    #[test]
    fn test_bitemporal_triple() {
        let triple = sample_triple();
        let now = Utc::now();

        let valid_period = TimePeriod::new(now, Some(now + chrono::Duration::days(30)));
        let trans_period = TimePeriod::new(now, Some(now + chrono::Duration::days(60)));

        let bitemp = BitemporalTriple::new(triple, valid_period, trans_period);

        let test_time = now + chrono::Duration::days(15);
        assert!(bitemp.is_valid_and_recorded_at(test_time, test_time));

        let future = now + chrono::Duration::days(40);
        assert!(!bitemp.is_valid_and_recorded_at(future, test_time));
    }

    #[test]
    fn test_temporal_graph() {
        let mut graph = TemporalGraph::new("https://example.org/");

        let now = Utc::now();
        let period1 = TimePeriod::new(now, Some(now + chrono::Duration::hours(1)));
        let period2 = TimePeriod::new(
            now + chrono::Duration::hours(2),
            Some(now + chrono::Duration::hours(3)),
        );

        let triple1 = TemporalTriple::new(sample_triple()).with_valid_time(period1);
        let triple2 = TemporalTriple::new(sample_triple()).with_valid_time(period2);

        graph.add_triple(triple1);
        graph.add_triple(triple2);

        // Query at time when only first triple is valid
        let query_time1 = now + chrono::Duration::minutes(30);
        let results1 = graph.query_at_time(query_time1);
        assert_eq!(results1.len(), 1);

        // Query at time when only second triple is valid
        let query_time2 = now + chrono::Duration::hours(2) + chrono::Duration::minutes(30);
        let results2 = graph.query_at_time(query_time2);
        assert_eq!(results2.len(), 1);

        // Query at time when no triples are valid
        let query_time3 = now + chrono::Duration::hours(5);
        let results3 = graph.query_at_time(query_time3);
        assert_eq!(results3.len(), 0);
    }

    #[test]
    fn test_temporal_snapshot() {
        let mut graph = TemporalGraph::new("https://example.org/");

        let now = Utc::now();
        let period = TimePeriod::new(now, Some(now + chrono::Duration::hours(1)));

        let triple = TemporalTriple::new(sample_triple()).with_valid_time(period);
        graph.add_triple(triple);

        let snapshot_time = now + chrono::Duration::minutes(30);
        let snapshot = TemporalSnapshot::from_graph(&graph, snapshot_time);

        assert_eq!(snapshot.triples.len(), 1);
        assert_eq!(snapshot.time, snapshot_time);
    }

    #[test]
    fn test_history_of_subject() {
        let mut graph = TemporalGraph::new("https://example.org/");

        let now = Utc::now();

        // Add multiple versions of the same subject over time
        for i in 0..3 {
            let start = now + chrono::Duration::hours(i as i64);
            let end = start + chrono::Duration::hours(1);
            let period = TimePeriod::new(start, Some(end));

            let mut triple = sample_triple();
            triple.object = RdfValue::string(format!("status_{}", i));

            let temp_triple = TemporalTriple::new(triple).with_valid_time(period);
            graph.add_triple(temp_triple);
        }

        let history = graph.history_of("ex:law1");
        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_rdfstar_conversion() {
        let triple = sample_triple();
        let now = Utc::now();
        let period = TimePeriod::new(now, Some(now + chrono::Duration::hours(1)));

        let temp_triple = TemporalTriple::new(triple).with_valid_time(period);

        let rdfstar_triples = temp_triple.to_rdfstar_triples("https://example.org/");

        // Should include base triple + temporal annotations
        assert!(rdfstar_triples.len() > 1);

        // Check for temporal annotations
        assert!(
            rdfstar_triples
                .iter()
                .any(|t| t.predicate == "temporal:validFrom")
        );
    }

    #[test]
    fn test_period_duration() {
        let start = Utc::now();
        let end = start + chrono::Duration::hours(2);
        let period = TimePeriod::new(start, Some(end));

        let duration = period.duration_secs();
        assert_eq!(duration, Some(7200)); // 2 hours in seconds
    }

    #[test]
    fn test_instant_period() {
        let time = Utc::now();
        let instant = TimePeriod::instant(time);

        assert_eq!(instant.start, instant.end.unwrap());
        assert!(instant.contains(time));
    }
}
