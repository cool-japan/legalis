//! Scalable secondary / inverted index over audit records.
//!
//! The existing storage backends only maintain coarse `HashMap<key, Vec<Uuid>>`
//! lookups for *statute* and *subject*. This module provides a dedicated,
//! memory-compact, multi-field **inverted index** designed to accelerate
//! arbitrary filtered queries at scale:
//!
//! - Every record is assigned a dense [`RowId`] ordinal (a `u32`, so a single
//!   index segment addresses up to ~4.29 billion rows). The ordinal → UUID map
//!   is stored once; every posting list stores compact ordinals instead of
//!   16-byte UUIDs, roughly quartering the memory of a UUID-keyed index.
//! - Inverted posting lists are kept for `statute_id`, `subject_id`,
//!   `event_type`, actor *kind*, and decision-result *kind*.
//! - A [`std::collections::BTreeMap`] keyed by timestamp gives `O(log n + k)`
//!   range scans.
//! - Deletion is *lazy* (tombstones); [`AuditIndex::compact`] reclaims the
//!   tombstoned space and renumbers the surviving rows.
//!
//! Posting lists are always built in ascending [`RowId`] order (ordinals are
//! handed out monotonically), so set operations use linear merge / two-pointer
//! intersection rather than re-sorting.
//!
//! This is a building block: [`crate::scale::QueryAccelerator`] turns a query
//! into the minimal set of index probes, and [`crate::scale::ScaleEngine`]
//! shards many indexes into time-ordered segments for billion-record corpora.

use crate::{Actor, AuditRecord, DecisionResult, EventType};
use chrono::{DateTime, Utc};
use std::collections::{BTreeMap, HashMap};
use std::ops::Bound;
use uuid::Uuid;

/// A dense per-record ordinal used inside an [`AuditIndex`].
pub type RowId = u32;

/// Coarse classification of an [`Actor`], used as an index key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorKind {
    /// [`Actor::System`].
    System,
    /// [`Actor::User`].
    User,
    /// [`Actor::External`].
    External,
}

impl ActorKind {
    /// Classifies an [`Actor`].
    pub fn of(actor: &Actor) -> Self {
        match actor {
            Actor::System { .. } => ActorKind::System,
            Actor::User { .. } => ActorKind::User,
            Actor::External { .. } => ActorKind::External,
        }
    }

    fn code(self) -> u8 {
        match self {
            ActorKind::System => 0,
            ActorKind::User => 1,
            ActorKind::External => 2,
        }
    }
}

/// Coarse classification of a [`DecisionResult`], used as an index key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResultKind {
    /// [`DecisionResult::Deterministic`].
    Deterministic,
    /// [`DecisionResult::RequiresDiscretion`].
    RequiresDiscretion,
    /// [`DecisionResult::Void`].
    Void,
    /// [`DecisionResult::Overridden`].
    Overridden,
}

impl ResultKind {
    /// Classifies a [`DecisionResult`].
    pub fn of(result: &DecisionResult) -> Self {
        match result {
            DecisionResult::Deterministic { .. } => ResultKind::Deterministic,
            DecisionResult::RequiresDiscretion { .. } => ResultKind::RequiresDiscretion,
            DecisionResult::Void { .. } => ResultKind::Void,
            DecisionResult::Overridden { .. } => ResultKind::Overridden,
        }
    }

    fn code(self) -> u8 {
        match self {
            ResultKind::Deterministic => 0,
            ResultKind::RequiresDiscretion => 1,
            ResultKind::Void => 2,
            ResultKind::Overridden => 3,
        }
    }
}

/// Stable numeric code for an [`EventType`] (its index key).
fn event_code(event_type: &EventType) -> u8 {
    match event_type {
        EventType::AutomaticDecision => 0,
        EventType::DiscretionaryReview => 1,
        EventType::HumanOverride => 2,
        EventType::Appeal => 3,
        EventType::StatuteModified => 4,
        EventType::SimulationRun => 5,
    }
}

/// Statistics describing the shape and memory profile of an [`AuditIndex`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexStats {
    /// Number of rows ever inserted (including tombstoned ones).
    pub total_rows: usize,
    /// Number of live (non-tombstoned) rows.
    pub live_rows: usize,
    /// Number of tombstoned rows awaiting compaction.
    pub tombstoned_rows: usize,
    /// Number of distinct statute ids.
    pub distinct_statutes: usize,
    /// Number of distinct subjects.
    pub distinct_subjects: usize,
    /// Number of distinct timestamp buckets in the time index.
    pub time_buckets: usize,
    /// Total number of posting entries across every inverted index.
    pub total_postings: usize,
    /// Rough heap-byte estimate for the whole index.
    pub estimated_bytes: usize,
}

/// A multi-field inverted index over [`AuditRecord`]s.
#[derive(Debug, Default)]
pub struct AuditIndex {
    row_to_uuid: Vec<Uuid>,
    row_to_time: Vec<i64>,
    alive: Vec<bool>,
    live_rows: usize,
    uuid_to_row: HashMap<Uuid, RowId>,
    statute: HashMap<String, Vec<RowId>>,
    subject: HashMap<Uuid, Vec<RowId>>,
    event_type: HashMap<u8, Vec<RowId>>,
    actor_kind: HashMap<u8, Vec<RowId>>,
    result_kind: HashMap<u8, Vec<RowId>>,
    by_time: BTreeMap<i64, Vec<RowId>>,
}

impl AuditIndex {
    /// Creates an empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty index pre-sized for `capacity` rows.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            row_to_uuid: Vec::with_capacity(capacity),
            row_to_time: Vec::with_capacity(capacity),
            alive: Vec::with_capacity(capacity),
            uuid_to_row: HashMap::with_capacity(capacity),
            ..Self::default()
        }
    }

    /// Builds an index from a batch of records.
    pub fn from_records(records: &[AuditRecord]) -> Self {
        let mut index = Self::with_capacity(records.len());
        index.insert_many(records);
        index
    }

    /// Number of rows ever inserted, including tombstoned rows.
    pub fn len(&self) -> usize {
        self.row_to_uuid.len()
    }

    /// Returns `true` if no rows have ever been inserted.
    pub fn is_empty(&self) -> bool {
        self.row_to_uuid.is_empty()
    }

    /// Number of live (non-tombstoned) rows.
    pub fn live_len(&self) -> usize {
        self.live_rows
    }

    /// Returns whether the record id is present and live.
    pub fn contains(&self, id: Uuid) -> bool {
        self.uuid_to_row.contains_key(&id)
    }

    /// Returns the [`RowId`] for a live record id, if present.
    pub fn row_of(&self, id: Uuid) -> Option<RowId> {
        self.uuid_to_row.get(&id).copied()
    }

    /// Returns the record id for a live row, if present.
    pub fn uuid_of(&self, row: RowId) -> Option<Uuid> {
        let idx = row as usize;
        if idx < self.alive.len() && self.alive[idx] {
            self.row_to_uuid.get(idx).copied()
        } else {
            None
        }
    }

    /// Inserts a single record, returning its assigned [`RowId`].
    ///
    /// Re-inserting an id that is already live is a no-op that returns the
    /// existing row, so an index can be (re-)built idempotently from a backend.
    pub fn insert(&mut self, record: &AuditRecord) -> RowId {
        if let Some(existing) = self.uuid_to_row.get(&record.id) {
            return *existing;
        }

        let row = self.row_to_uuid.len() as RowId;
        let millis = record.timestamp.timestamp_millis();

        self.row_to_uuid.push(record.id);
        self.row_to_time.push(millis);
        self.alive.push(true);
        self.live_rows += 1;
        self.uuid_to_row.insert(record.id, row);

        self.statute
            .entry(record.statute_id.clone())
            .or_default()
            .push(row);
        self.subject.entry(record.subject_id).or_default().push(row);
        self.event_type
            .entry(event_code(&record.event_type))
            .or_default()
            .push(row);
        self.actor_kind
            .entry(ActorKind::of(&record.actor).code())
            .or_default()
            .push(row);
        self.result_kind
            .entry(ResultKind::of(&record.result).code())
            .or_default()
            .push(row);
        self.by_time.entry(millis).or_default().push(row);

        row
    }

    /// Inserts a batch of records.
    pub fn insert_many(&mut self, records: &[AuditRecord]) {
        self.row_to_uuid.reserve(records.len());
        for record in records {
            self.insert(record);
        }
    }

    /// Tombstones a record by id. Returns `true` if a live row was removed.
    ///
    /// The posting lists keep the (now dead) ordinal until [`Self::compact`] is
    /// called; candidate queries transparently skip tombstoned rows.
    pub fn remove(&mut self, id: Uuid) -> bool {
        if let Some(row) = self.uuid_to_row.remove(&id) {
            let idx = row as usize;
            if idx < self.alive.len() && self.alive[idx] {
                self.alive[idx] = false;
                self.live_rows -= 1;
                return true;
            }
        }
        false
    }

    /// Fraction of rows that are tombstoned (0.0 when fully compact).
    pub fn tombstone_ratio(&self) -> f64 {
        if self.row_to_uuid.is_empty() {
            return 0.0;
        }
        let dead = self.row_to_uuid.len() - self.live_rows;
        dead as f64 / self.row_to_uuid.len() as f64
    }

    /// Rebuilds the index in place, dropping tombstoned rows and renumbering the
    /// survivors into a dense `0..live_len()` range.
    pub fn compact(&mut self) {
        if self.live_rows == self.row_to_uuid.len() {
            return; // already dense
        }

        // old row -> new row (None for tombstoned rows).
        let mut remap: Vec<Option<RowId>> = vec![None; self.row_to_uuid.len()];
        let mut next: RowId = 0;
        let mut new_uuid = Vec::with_capacity(self.live_rows);
        let mut new_time = Vec::with_capacity(self.live_rows);
        for (old, alive) in self.alive.iter().enumerate() {
            if *alive {
                remap[old] = Some(next);
                new_uuid.push(self.row_to_uuid[old]);
                new_time.push(self.row_to_time[old]);
                next += 1;
            }
        }

        let remap_list = |rows: &[RowId]| -> Vec<RowId> {
            rows.iter()
                .filter_map(|r| remap[*r as usize])
                .collect::<Vec<_>>()
        };

        self.statute = self
            .statute
            .iter()
            .filter_map(|(k, v)| {
                let mapped = remap_list(v);
                if mapped.is_empty() {
                    None
                } else {
                    Some((k.clone(), mapped))
                }
            })
            .collect();
        self.subject = self
            .subject
            .iter()
            .filter_map(|(k, v)| {
                let mapped = remap_list(v);
                if mapped.is_empty() {
                    None
                } else {
                    Some((*k, mapped))
                }
            })
            .collect();
        let remap_u8 = |map: &HashMap<u8, Vec<RowId>>| -> HashMap<u8, Vec<RowId>> {
            map.iter()
                .filter_map(|(k, v)| {
                    let mapped = remap_list(v);
                    if mapped.is_empty() {
                        None
                    } else {
                        Some((*k, mapped))
                    }
                })
                .collect()
        };
        self.event_type = remap_u8(&self.event_type);
        self.actor_kind = remap_u8(&self.actor_kind);
        self.result_kind = remap_u8(&self.result_kind);
        self.by_time = self
            .by_time
            .iter()
            .filter_map(|(k, v)| {
                let mapped = remap_list(v);
                if mapped.is_empty() {
                    None
                } else {
                    Some((*k, mapped))
                }
            })
            .collect();

        self.uuid_to_row = new_uuid
            .iter()
            .enumerate()
            .map(|(i, id)| (*id, i as RowId))
            .collect();
        self.row_to_uuid = new_uuid;
        self.row_to_time = new_time;
        self.alive = vec![true; self.live_rows];
    }

    /// Returns all live rows in ascending order (a full scan path).
    pub fn live_rows_sorted(&self) -> Vec<RowId> {
        (0..self.row_to_uuid.len() as RowId)
            .filter(|r| self.alive[*r as usize])
            .collect()
    }

    /// Candidate rows matching any of the given statute ids (sorted, live).
    pub fn statute_candidates(&self, statutes: &[String]) -> Vec<RowId> {
        let lists: Vec<&[RowId]> = statutes
            .iter()
            .filter_map(|s| self.statute.get(s).map(|v| v.as_slice()))
            .collect();
        self.union_alive(&lists)
    }

    /// Candidate rows matching any of the given subjects (sorted, live).
    pub fn subject_candidates(&self, subjects: &[Uuid]) -> Vec<RowId> {
        let lists: Vec<&[RowId]> = subjects
            .iter()
            .filter_map(|s| self.subject.get(s).map(|v| v.as_slice()))
            .collect();
        self.union_alive(&lists)
    }

    /// Candidate rows matching any of the given event types (sorted, live).
    pub fn event_type_candidates(&self, events: &[EventType]) -> Vec<RowId> {
        let lists: Vec<&[RowId]> = events
            .iter()
            .filter_map(|e| self.event_type.get(&event_code(e)).map(|v| v.as_slice()))
            .collect();
        self.union_alive(&lists)
    }

    /// Candidate rows matching any of the given actor kinds (sorted, live).
    pub fn actor_kind_candidates(&self, kinds: &[ActorKind]) -> Vec<RowId> {
        let lists: Vec<&[RowId]> = kinds
            .iter()
            .filter_map(|k| self.actor_kind.get(&k.code()).map(|v| v.as_slice()))
            .collect();
        self.union_alive(&lists)
    }

    /// Candidate rows matching any of the given result kinds (sorted, live).
    pub fn result_kind_candidates(&self, kinds: &[ResultKind]) -> Vec<RowId> {
        let lists: Vec<&[RowId]> = kinds
            .iter()
            .filter_map(|k| self.result_kind.get(&k.code()).map(|v| v.as_slice()))
            .collect();
        self.union_alive(&lists)
    }

    /// Candidate rows whose timestamp falls in `[start, end]` (sorted, live).
    ///
    /// The time index is keyed by whole milliseconds, so the scan is widened by
    /// 1ms on each side to guarantee a *superset* of the exact match; callers
    /// apply the precise bound afterwards.
    pub fn time_range_candidates(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<RowId> {
        self.time_candidates(Some(start), Some(end))
    }

    /// Candidate rows for an optionally half-open time range (sorted, live).
    ///
    /// A `None` bound is unbounded on that side. See [`Self::time_range_candidates`]
    /// for the millisecond-widening note.
    pub fn time_candidates(
        &self,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Vec<RowId> {
        let mut out = Vec::new();
        for (_, rows) in self.by_time.range(time_bounds(start, end)) {
            for r in rows {
                if self.alive[*r as usize] {
                    out.push(*r);
                }
            }
        }
        out.sort_unstable();
        out
    }

    /// Estimated cardinality (upper bound) of a statute probe — the summed
    /// posting length, used by the planner for selectivity ordering.
    pub fn statute_cardinality(&self, statutes: &[String]) -> usize {
        statutes
            .iter()
            .filter_map(|s| self.statute.get(s).map(|v| v.len()))
            .sum()
    }

    /// Estimated cardinality (upper bound) of a subject probe.
    pub fn subject_cardinality(&self, subjects: &[Uuid]) -> usize {
        subjects
            .iter()
            .filter_map(|s| self.subject.get(s).map(|v| v.len()))
            .sum()
    }

    /// Estimated cardinality (upper bound) of an event-type probe.
    pub fn event_type_cardinality(&self, events: &[EventType]) -> usize {
        events
            .iter()
            .filter_map(|e| self.event_type.get(&event_code(e)).map(|v| v.len()))
            .sum()
    }

    /// Estimated cardinality (upper bound) of an actor-kind probe.
    pub fn actor_kind_cardinality(&self, kinds: &[ActorKind]) -> usize {
        kinds
            .iter()
            .filter_map(|k| self.actor_kind.get(&k.code()).map(|v| v.len()))
            .sum()
    }

    /// Estimated cardinality (upper bound) of a result-kind probe.
    pub fn result_kind_cardinality(&self, kinds: &[ResultKind]) -> usize {
        kinds
            .iter()
            .filter_map(|k| self.result_kind.get(&k.code()).map(|v| v.len()))
            .sum()
    }

    /// Estimated cardinality (upper bound) of a time-range probe — the summed
    /// length of the time buckets that overlap the range (incl. tombstones).
    pub fn time_cardinality(
        &self,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> usize {
        self.by_time
            .range(time_bounds(start, end))
            .map(|(_, v)| v.len())
            .sum()
    }

    /// Maps live rows back to record ids, preserving order.
    pub fn resolve(&self, rows: &[RowId]) -> Vec<Uuid> {
        rows.iter().filter_map(|r| self.uuid_of(*r)).collect()
    }

    /// Computes index statistics.
    pub fn stats(&self) -> IndexStats {
        let total_postings = posting_len(&self.statute)
            + self.subject.values().map(|v| v.len()).sum::<usize>()
            + self.event_type.values().map(|v| v.len()).sum::<usize>()
            + self.actor_kind.values().map(|v| v.len()).sum::<usize>()
            + self.result_kind.values().map(|v| v.len()).sum::<usize>();

        let estimated_bytes = self.row_to_uuid.len() * (16 + 8 + 1)
            + self.uuid_to_row.len() * (16 + 4)
            + total_postings * std::mem::size_of::<RowId>()
            + self.by_time.values().map(|v| v.len()).sum::<usize>() * std::mem::size_of::<RowId>()
            + self.statute.keys().map(|k| k.len()).sum::<usize>();

        IndexStats {
            total_rows: self.row_to_uuid.len(),
            live_rows: self.live_rows,
            tombstoned_rows: self.row_to_uuid.len() - self.live_rows,
            distinct_statutes: self.statute.len(),
            distinct_subjects: self.subject.len(),
            time_buckets: self.by_time.len(),
            total_postings,
            estimated_bytes,
        }
    }

    /// Sorted-unique union of posting lists, keeping only live rows.
    fn union_alive(&self, lists: &[&[RowId]]) -> Vec<RowId> {
        if lists.is_empty() {
            return Vec::new();
        }
        if lists.len() == 1 {
            return lists[0]
                .iter()
                .copied()
                .filter(|r| self.alive[*r as usize])
                .collect();
        }
        let mut merged: Vec<RowId> = Vec::new();
        for list in lists {
            merged.extend(list.iter().copied().filter(|r| self.alive[*r as usize]));
        }
        merged.sort_unstable();
        merged.dedup();
        merged
    }
}

/// Intersection of two ascending, de-duplicated row-id slices.
pub fn intersect_sorted(a: &[RowId], b: &[RowId]) -> Vec<RowId> {
    let mut out = Vec::with_capacity(a.len().min(b.len()));
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                out.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    out
}

fn posting_len(map: &HashMap<String, Vec<RowId>>) -> usize {
    map.values().map(|v| v.len()).sum()
}

/// Builds the millisecond `BTreeMap` bounds for an optional time range,
/// widening present bounds by 1ms so truncation never drops a true match.
fn time_bounds(
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
) -> (Bound<i64>, Bound<i64>) {
    let lo = match start {
        Some(s) => Bound::Included(s.timestamp_millis().saturating_sub(1)),
        None => Bound::Unbounded,
    };
    let hi = match end {
        Some(e) => Bound::Included(e.timestamp_millis().saturating_add(1)),
        None => Bound::Unbounded,
    };
    (lo, hi)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DecisionContext, DecisionResult};
    use chrono::Duration;
    use std::collections::HashMap as Map;

    fn record(statute: &str, subject: Uuid, ts: DateTime<Utc>) -> AuditRecord {
        let mut r = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "engine".to_string(),
            },
            statute.to_string(),
            subject,
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "ok".to_string(),
                parameters: Map::new(),
            },
            None,
        );
        r.timestamp = ts;
        r
    }

    fn override_record(statute: &str, ts: DateTime<Utc>) -> AuditRecord {
        let mut r = AuditRecord::new(
            EventType::HumanOverride,
            Actor::User {
                user_id: "u1".to_string(),
                role: "admin".to_string(),
            },
            statute.to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Overridden {
                original_result: Box::new(DecisionResult::Void {
                    reason: "x".to_string(),
                }),
                new_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "fixed".to_string(),
                    parameters: Map::new(),
                }),
                justification: "review".to_string(),
            },
            None,
        );
        r.timestamp = ts;
        r
    }

    #[test]
    fn test_insert_and_resolve() {
        let now = Utc::now();
        let s = Uuid::new_v4();
        let recs = vec![
            record("statute-1", s, now),
            record("statute-2", Uuid::new_v4(), now),
        ];
        let index = AuditIndex::from_records(&recs);
        assert_eq!(index.len(), 2);
        assert_eq!(index.live_len(), 2);

        let rows = index.statute_candidates(&["statute-1".to_string()]);
        assert_eq!(rows.len(), 1);
        let ids = index.resolve(&rows);
        assert_eq!(ids[0], recs[0].id);
    }

    #[test]
    fn test_idempotent_insert() {
        let mut index = AuditIndex::new();
        let r = record("s", Uuid::new_v4(), Utc::now());
        let a = index.insert(&r);
        let b = index.insert(&r);
        assert_eq!(a, b);
        assert_eq!(index.len(), 1);
    }

    #[test]
    fn test_subject_union() {
        let now = Utc::now();
        let s1 = Uuid::new_v4();
        let s2 = Uuid::new_v4();
        let recs = vec![
            record("a", s1, now),
            record("b", s2, now),
            record("c", s1, now),
        ];
        let index = AuditIndex::from_records(&recs);
        let rows = index.subject_candidates(&[s1, s2]);
        assert_eq!(rows.len(), 3);
        // Sorted ascending and unique.
        assert!(rows.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn test_event_and_result_kind() {
        let now = Utc::now();
        let recs = vec![record("a", Uuid::new_v4(), now), override_record("b", now)];
        let index = AuditIndex::from_records(&recs);

        let overrides = index.result_kind_candidates(&[ResultKind::Overridden]);
        assert_eq!(overrides.len(), 1);
        let det = index.result_kind_candidates(&[ResultKind::Deterministic]);
        assert_eq!(det.len(), 1);

        let users = index.actor_kind_candidates(&[ActorKind::User]);
        assert_eq!(users.len(), 1);
        let human = index.event_type_candidates(&[EventType::HumanOverride]);
        assert_eq!(human.len(), 1);
    }

    #[test]
    fn test_time_range() {
        let base = Utc::now();
        let recs = vec![
            record("a", Uuid::new_v4(), base),
            record("b", Uuid::new_v4(), base + Duration::hours(2)),
            record("c", Uuid::new_v4(), base + Duration::hours(5)),
        ];
        let index = AuditIndex::from_records(&recs);
        let rows =
            index.time_range_candidates(base + Duration::hours(1), base + Duration::hours(3));
        assert_eq!(rows.len(), 1);
        assert_eq!(index.resolve(&rows)[0], recs[1].id);
    }

    #[test]
    fn test_remove_tombstones() {
        let now = Utc::now();
        let recs = vec![
            record("a", Uuid::new_v4(), now),
            record("a", Uuid::new_v4(), now),
        ];
        let mut index = AuditIndex::from_records(&recs);
        assert!(index.remove(recs[0].id));
        assert!(!index.remove(recs[0].id)); // already gone
        assert_eq!(index.live_len(), 1);

        let rows = index.statute_candidates(&["a".to_string()]);
        assert_eq!(rows.len(), 1);
        assert_eq!(index.resolve(&rows)[0], recs[1].id);
        assert!(index.tombstone_ratio() > 0.0);
    }

    #[test]
    fn test_compact_renumbers() {
        let now = Utc::now();
        let s = Uuid::new_v4();
        let recs = vec![
            record("a", s, now),
            record("b", Uuid::new_v4(), now),
            record("a", s, now),
        ];
        let mut index = AuditIndex::from_records(&recs);
        index.remove(recs[1].id);
        index.compact();

        assert_eq!(index.len(), 2);
        assert_eq!(index.live_len(), 2);
        assert_eq!(index.tombstone_ratio(), 0.0);

        // Surviving records still resolve correctly post-renumbering.
        let rows = index.statute_candidates(&["a".to_string()]);
        let ids = index.resolve(&rows);
        assert!(ids.contains(&recs[0].id));
        assert!(ids.contains(&recs[2].id));
        assert_eq!(ids.len(), 2);

        let by_subject = index.subject_candidates(&[s]);
        assert_eq!(by_subject.len(), 2);
    }

    #[test]
    fn test_intersect_sorted() {
        assert_eq!(
            intersect_sorted(&[1, 2, 3, 5], &[2, 3, 4, 5]),
            vec![2, 3, 5]
        );
        assert_eq!(intersect_sorted(&[], &[1, 2]), Vec::<RowId>::new());
        assert_eq!(intersect_sorted(&[1, 2], &[3, 4]), Vec::<RowId>::new());
    }

    #[test]
    fn test_cardinality_estimates() {
        let now = Utc::now();
        let recs = vec![
            record("a", Uuid::new_v4(), now),
            record("a", Uuid::new_v4(), now),
            record("b", Uuid::new_v4(), now),
        ];
        let index = AuditIndex::from_records(&recs);
        assert_eq!(index.statute_cardinality(&["a".to_string()]), 2);
        assert_eq!(index.statute_cardinality(&["b".to_string()]), 1);
        assert_eq!(index.statute_cardinality(&["missing".to_string()]), 0);
    }

    #[test]
    fn test_stats() {
        let now = Utc::now();
        let recs = vec![record("a", Uuid::new_v4(), now), override_record("b", now)];
        let index = AuditIndex::from_records(&recs);
        let stats = index.stats();
        assert_eq!(stats.total_rows, 2);
        assert_eq!(stats.live_rows, 2);
        assert_eq!(stats.distinct_statutes, 2);
        assert!(stats.total_postings >= 10); // 5 fields x 2 rows
        assert!(stats.estimated_bytes > 0);
    }
}
