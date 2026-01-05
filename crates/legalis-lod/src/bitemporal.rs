//! Bitemporal knowledge modeling for legal data.
//!
//! This module provides comprehensive bitemporal modeling capabilities,
//! supporting both valid time (when facts are true in reality) and
//! transaction time (when facts were recorded in the system).
//!
//! ## Use Cases
//! - Legal statute amendments with retroactive effects
//! - Corrections to historical legal data
//! - Audit trails for legal knowledge changes
//! - Time-travel queries (what did we know when?)

use crate::temporal_rdf::{BitemporalTriple, TimePeriod};
use crate::{RdfValue, Triple};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Bitemporal database for legal knowledge
#[derive(Debug, Clone)]
pub struct BitemporalDatabase {
    /// Base namespace
    pub base_uri: String,
    /// Bitemporal triples
    pub triples: Vec<BitemporalTriple>,
}

impl BitemporalDatabase {
    /// Creates a new bitemporal database
    pub fn new(base_uri: impl Into<String>) -> Self {
        Self {
            base_uri: base_uri.into(),
            triples: Vec::new(),
        }
    }

    /// Inserts a new bitemporal triple
    pub fn insert(&mut self, triple: BitemporalTriple) {
        self.triples.push(triple);
    }

    /// Inserts a triple with current transaction time
    pub fn insert_now(&mut self, triple: Triple, valid_time: TimePeriod) {
        let transaction_time = TimePeriod::ongoing(Utc::now());
        let bitemp = BitemporalTriple::new(triple, valid_time, transaction_time);
        self.triples.push(bitemp);
    }

    /// Updates a triple (creates a new version with end transaction time on old version)
    pub fn update(
        &mut self,
        subject: &str,
        predicate: &str,
        new_object: RdfValue,
        valid_time: TimePeriod,
    ) {
        let now = Utc::now();

        // Find and close existing triples
        for bitemp in &mut self.triples {
            if bitemp.temporal.triple.subject == subject
                && bitemp.temporal.triple.predicate == predicate
            {
                if let Some(ref mut tt) = bitemp.temporal.transaction_time {
                    if tt.end.is_none() {
                        tt.end = Some(now);
                    }
                }
            }
        }

        // Insert new version
        let new_triple = Triple {
            subject: subject.to_string(),
            predicate: predicate.to_string(),
            object: new_object,
        };
        self.insert_now(new_triple, valid_time);
    }

    /// Queries what was known at a specific transaction time about a specific valid time
    pub fn query_at(
        &self,
        valid_time: DateTime<Utc>,
        transaction_time: DateTime<Utc>,
    ) -> Vec<Triple> {
        self.triples
            .iter()
            .filter(|bt| bt.is_valid_and_recorded_at(valid_time, transaction_time))
            .map(|bt| bt.temporal.triple.clone())
            .collect()
    }

    /// Queries the current state (latest transaction time) at a valid time
    pub fn current_at(&self, valid_time: DateTime<Utc>) -> Vec<Triple> {
        let now = Utc::now();
        self.query_at(valid_time, now)
    }

    /// Reconstructs the database as it was known at a past transaction time
    pub fn as_of(&self, transaction_time: DateTime<Utc>) -> Vec<Triple> {
        self.triples
            .iter()
            .filter(|bt| bt.temporal.was_recorded_at(transaction_time))
            .map(|bt| bt.temporal.triple.clone())
            .collect()
    }

    /// Gets the complete history of a subject
    pub fn history(&self, subject: &str) -> Vec<BitemporalVersion> {
        let mut versions = Vec::new();

        for bitemp in &self.triples {
            if bitemp.temporal.triple.subject == subject {
                versions.push(BitemporalVersion {
                    triple: bitemp.temporal.triple.clone(),
                    valid_time: bitemp.temporal.valid_time,
                    transaction_time: bitemp.temporal.transaction_time,
                });
            }
        }

        // Sort by transaction time
        versions.sort_by(|a, b| {
            let a_time = a.transaction_time.as_ref().map(|t| t.start);
            let b_time = b.transaction_time.as_ref().map(|t| t.start);
            a_time.cmp(&b_time)
        });

        versions
    }

    /// Finds corrections (changes to past valid times)
    pub fn corrections(&self) -> Vec<Correction> {
        let mut corrections = Vec::new();

        // Group by subject+predicate
        let mut groups: HashMap<String, Vec<&BitemporalTriple>> = HashMap::new();
        for bitemp in &self.triples {
            let key = format!(
                "{}::{}",
                bitemp.temporal.triple.subject, bitemp.temporal.triple.predicate
            );
            groups.entry(key).or_default().push(bitemp);
        }

        // Find corrections within each group
        for (_key, bitemps) in groups {
            for i in 0..bitemps.len() {
                for j in (i + 1)..bitemps.len() {
                    let earlier = bitemps[i];
                    let later = bitemps[j];

                    if let (Some(vt_early), Some(vt_late), Some(tt_early), Some(tt_late)) = (
                        &earlier.temporal.valid_time,
                        &later.temporal.valid_time,
                        &earlier.temporal.transaction_time,
                        &later.temporal.transaction_time,
                    ) {
                        // A correction is when valid time overlaps but transaction time is later
                        if vt_early.overlaps(vt_late) && tt_late.start > tt_early.start {
                            corrections.push(Correction {
                                subject: earlier.temporal.triple.subject.clone(),
                                predicate: earlier.temporal.triple.predicate.clone(),
                                old_value: earlier.temporal.triple.object.clone(),
                                new_value: later.temporal.triple.object.clone(),
                                valid_time: *vt_late,
                                correction_time: tt_late.start,
                            });
                        }
                    }
                }
            }
        }

        corrections
    }

    /// Gets retroactive changes (valid time before transaction time)
    pub fn retroactive_changes(&self) -> Vec<&BitemporalTriple> {
        self.triples
            .iter()
            .filter(|bt| {
                if let (Some(vt), Some(tt)) =
                    (&bt.temporal.valid_time, &bt.temporal.transaction_time)
                {
                    vt.start < tt.start
                } else {
                    false
                }
            })
            .collect()
    }
}

/// A version in the bitemporal history
#[derive(Debug, Clone)]
pub struct BitemporalVersion {
    /// The triple
    pub triple: Triple,
    /// Valid time period
    pub valid_time: Option<TimePeriod>,
    /// Transaction time period
    pub transaction_time: Option<TimePeriod>,
}

/// A correction to historical data
#[derive(Debug, Clone)]
pub struct Correction {
    /// Subject that was corrected
    pub subject: String,
    /// Predicate that was corrected
    pub predicate: String,
    /// Old (incorrect) value
    pub old_value: RdfValue,
    /// New (corrected) value
    pub new_value: RdfValue,
    /// Valid time that was corrected
    pub valid_time: TimePeriod,
    /// When the correction was made
    pub correction_time: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn sample_triple(value: &str) -> Triple {
        Triple {
            subject: "ex:law1".to_string(),
            predicate: "ex:hasStatus".to_string(),
            object: RdfValue::string(value),
        }
    }

    #[test]
    fn test_bitemporal_database_insert() {
        let mut db = BitemporalDatabase::new("https://example.org/");

        let now = Utc::now();
        let valid_period = TimePeriod::new(now, None);
        let trans_period = TimePeriod::new(now, None);

        let bitemp = BitemporalTriple::new(sample_triple("active"), valid_period, trans_period);

        db.insert(bitemp);
        assert_eq!(db.triples.len(), 1);
    }

    #[test]
    fn test_insert_now() {
        let mut db = BitemporalDatabase::new("https://example.org/");

        let now = Utc::now();
        let valid_period = TimePeriod::new(now, None);

        db.insert_now(sample_triple("active"), valid_period);
        assert_eq!(db.triples.len(), 1);

        // Transaction time should be set to now (ongoing)
        assert!(db.triples[0].temporal.transaction_time.is_some());
    }

    #[test]
    fn test_update() {
        let mut db = BitemporalDatabase::new("https://example.org/");

        let now = Utc::now();
        let valid_period = TimePeriod::new(now, None);

        // Insert initial version
        db.insert_now(sample_triple("active"), valid_period);

        // Update to new value
        let new_valid = TimePeriod::new(now + chrono::Duration::days(1), None);
        db.update(
            "ex:law1",
            "ex:hasStatus",
            RdfValue::string("inactive"),
            new_valid,
        );

        assert_eq!(db.triples.len(), 2);

        // First version should have end transaction time
        assert!(
            db.triples[0]
                .temporal
                .transaction_time
                .as_ref()
                .unwrap()
                .end
                .is_some()
        );
    }

    #[test]
    fn test_query_at() {
        let mut db = BitemporalDatabase::new("https://example.org/");

        let base_time = Utc::now();
        let valid1 = TimePeriod::new(base_time, Some(base_time + chrono::Duration::days(30)));
        let trans1 = TimePeriod::new(base_time, Some(base_time + chrono::Duration::days(60)));

        let bitemp = BitemporalTriple::new(sample_triple("active"), valid1, trans1);
        db.insert(bitemp);

        // Query at a time when both valid and transaction are active
        let query_time = base_time + chrono::Duration::days(15);
        let results = db.query_at(query_time, query_time);

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_current_at() {
        let mut db = BitemporalDatabase::new("https://example.org/");

        let now = Utc::now();
        let valid_period = TimePeriod::new(now, None);

        db.insert_now(sample_triple("active"), valid_period);

        let results = db.current_at(now + chrono::Duration::hours(1));
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_as_of() {
        let mut db = BitemporalDatabase::new("https://example.org/");

        let base_time = Utc::now();
        let valid_period = TimePeriod::new(base_time, None);

        // First version with explicit transaction time
        let trans1 = TimePeriod::new(base_time, None);
        let bitemp1 = BitemporalTriple::new(sample_triple("v1"), valid_period, trans1);
        db.insert(bitemp1);

        // Second version a bit later
        let update_time = base_time + chrono::Duration::seconds(1);
        let trans2 = TimePeriod::new(update_time, None);
        let bitemp2 = BitemporalTriple::new(sample_triple("v2"), valid_period, trans2);
        db.insert(bitemp2);

        // Query as of first version time
        let as_of_first = db.as_of(base_time);
        assert_eq!(as_of_first.len(), 1);

        // Query as of second version time
        let as_of_second = db.as_of(update_time);
        assert_eq!(as_of_second.len(), 2);
    }

    #[test]
    fn test_history() {
        let mut db = BitemporalDatabase::new("https://example.org/");

        let now = Utc::now();

        // Add multiple versions
        for i in 0..3 {
            let valid = TimePeriod::new(now + chrono::Duration::hours(i as i64), None);
            db.insert_now(sample_triple(&format!("v{}", i)), valid);
        }

        let history = db.history("ex:law1");
        assert_eq!(history.len(), 3);

        // Should be sorted by transaction time
        for i in 1..history.len() {
            let prev_time = history[i - 1].transaction_time.as_ref().unwrap().start;
            let curr_time = history[i].transaction_time.as_ref().unwrap().start;
            assert!(curr_time >= prev_time);
        }
    }

    #[test]
    fn test_retroactive_changes() {
        let mut db = BitemporalDatabase::new("https://example.org/");

        let now = Utc::now();
        let past_valid = TimePeriod::new(now - chrono::Duration::days(30), None);
        let current_trans = TimePeriod::new(now, None);

        // Insert a retroactive change (valid time in the past, transaction time now)
        let bitemp = BitemporalTriple::new(sample_triple("retroactive"), past_valid, current_trans);
        db.insert(bitemp);

        let retroactive = db.retroactive_changes();
        assert_eq!(retroactive.len(), 1);
    }
}
