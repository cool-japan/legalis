//! Index-backed query planner and accelerator.
//!
//! Where [`crate::query_plan`] *explains* a query's cost textually, this module
//! actually *executes* queries faster by probing an [`AuditIndex`]. The strategy
//! is the classic one used by columnar / search engines:
//!
//! 1. For every constrained, indexed field, obtain a sorted candidate row set.
//! 2. Order the sets by ascending cardinality ("most selective first").
//! 3. Seed the result with the smallest set and successively intersect with the
//!    rest (linear two-pointer merges over ascending [`RowId`]s).
//! 4. Resolve the surviving rows to records and apply the exact (precise)
//!    predicate — only the time bound needs re-checking, because the index time
//!    buckets are millisecond-quantised and deliberately over-select.
//!
//! [`QueryAccelerator`] can run a native [`IndexQuery`] or transparently
//! accelerate an existing [`crate::query::QueryBuilder`]: the index produces a
//! superset of candidates and `QueryBuilder::execute` applies the full filter
//! (including actor predicates and pagination), so results are identical to a
//! full scan but far cheaper.

use crate::query::QueryBuilder;
use crate::scale::index::{ActorKind, AuditIndex, ResultKind, RowId, intersect_sorted};
use crate::storage::AuditStorage;
use crate::{AuditError, AuditRecord, AuditResult, EventType};
use chrono::{DateTime, Utc};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use uuid::Uuid;

/// A stable, order-insensitive fingerprint of an [`IndexQuery`], used as a
/// read-cache key (see [`crate::scale::ReadCache`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuerySignature(pub u64);

/// A query expressed purely in terms of indexed fields.
///
/// Within a field, multiple values are OR-ed; across fields they are AND-ed.
/// Empty collections mean "no constraint on this field". Two queries that
/// describe the same constraint share a [`QuerySignature`] (see
/// [`IndexQuery::signature`]).
#[derive(Debug, Clone, Default)]
pub struct IndexQuery {
    /// Statute ids to match (OR).
    pub statute_ids: Vec<String>,
    /// Subjects to match (OR).
    pub subject_ids: Vec<Uuid>,
    /// Event types to match (OR, by discriminant).
    pub event_types: Vec<EventType>,
    /// Actor kinds to match (OR).
    pub actor_kinds: Vec<ActorKind>,
    /// Decision-result kinds to match (OR).
    pub result_kinds: Vec<ResultKind>,
    /// Inclusive lower time bound.
    pub start: Option<DateTime<Utc>>,
    /// Inclusive upper time bound.
    pub end: Option<DateTime<Utc>>,
    /// Maximum number of rows to return (applied after ordering by time).
    pub limit: Option<usize>,
    /// Number of leading rows to skip (applied before `limit`).
    pub offset: Option<usize>,
}

impl IndexQuery {
    /// Creates an empty query (matches every live row).
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a statute-id constraint.
    pub fn statute(mut self, id: impl Into<String>) -> Self {
        self.statute_ids.push(id.into());
        self
    }

    /// Adds a subject constraint.
    pub fn subject(mut self, id: Uuid) -> Self {
        self.subject_ids.push(id);
        self
    }

    /// Adds an event-type constraint.
    pub fn event_type(mut self, event_type: EventType) -> Self {
        self.event_types.push(event_type);
        self
    }

    /// Adds an actor-kind constraint.
    pub fn actor_kind(mut self, kind: ActorKind) -> Self {
        self.actor_kinds.push(kind);
        self
    }

    /// Adds a result-kind constraint.
    pub fn result_kind(mut self, kind: ResultKind) -> Self {
        self.result_kinds.push(kind);
        self
    }

    /// Sets an inclusive time range.
    pub fn time_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.start = Some(start);
        self.end = Some(end);
        self
    }

    /// Sets the result limit.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the result offset.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Whether any indexed field is constrained.
    pub fn has_constraint(&self) -> bool {
        !self.statute_ids.is_empty()
            || !self.subject_ids.is_empty()
            || !self.event_types.is_empty()
            || !self.actor_kinds.is_empty()
            || !self.result_kinds.is_empty()
            || self.start.is_some()
            || self.end.is_some()
    }

    /// Derives an [`IndexQuery`] from a [`QueryBuilder`].
    ///
    /// Only the exactly-translatable indexed fields (statute, subject, event
    /// type, time bounds) are carried over; richer actor predicates and
    /// pagination stay on the builder. The accelerator therefore yields a
    /// *superset*, and `QueryBuilder::execute` applies the precise filter.
    pub fn from_query_builder(builder: &QueryBuilder) -> Self {
        let (start, end) = builder.time_bounds();
        Self {
            statute_ids: builder.statute_id_filters().to_vec(),
            subject_ids: builder.subject_id_filters().to_vec(),
            event_types: builder.event_type_filters().to_vec(),
            actor_kinds: Vec::new(),
            result_kinds: Vec::new(),
            start,
            end,
            limit: None,
            offset: None,
        }
    }

    /// Computes an order-insensitive signature of this query.
    pub fn signature(&self) -> QuerySignature {
        let mut hasher = DefaultHasher::new();

        let mut statutes = self.statute_ids.clone();
        statutes.sort();
        statutes.dedup();
        statutes.hash(&mut hasher);

        let mut subjects: Vec<u128> = self.subject_ids.iter().map(|u| u.as_u128()).collect();
        subjects.sort_unstable();
        subjects.dedup();
        subjects.hash(&mut hasher);

        let mut events: Vec<u8> = self.event_types.iter().map(event_type_code).collect();
        events.sort_unstable();
        events.dedup();
        events.hash(&mut hasher);

        let mut actors: Vec<u8> = self
            .actor_kinds
            .iter()
            .map(|k| actor_kind_code(*k))
            .collect();
        actors.sort_unstable();
        actors.dedup();
        actors.hash(&mut hasher);

        let mut results: Vec<u8> = self
            .result_kinds
            .iter()
            .map(|k| result_kind_code(*k))
            .collect();
        results.sort_unstable();
        results.dedup();
        results.hash(&mut hasher);

        self.start.map(|t| t.timestamp_millis()).hash(&mut hasher);
        self.end.map(|t| t.timestamp_millis()).hash(&mut hasher);
        self.limit.hash(&mut hasher);
        self.offset.hash(&mut hasher);

        QuerySignature(hasher.finish())
    }

    /// The distinct statutes touched by this query (used as cache tags).
    pub fn statute_tags(&self) -> Vec<String> {
        let mut tags = self.statute_ids.clone();
        tags.sort();
        tags.dedup();
        tags
    }

    /// The distinct subjects touched by this query (used as cache tags).
    pub fn subject_tags(&self) -> Vec<Uuid> {
        let mut tags = self.subject_ids.clone();
        tags.sort();
        tags.dedup();
        tags
    }
}

/// A single index access path chosen by the planner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessPath {
    /// Probe the statute inverted index.
    Statute,
    /// Probe the subject inverted index.
    Subject,
    /// Probe the event-type inverted index.
    EventType,
    /// Probe the actor-kind inverted index.
    ActorKind,
    /// Probe the result-kind inverted index.
    ResultKind,
    /// Scan the time `BTreeMap` range.
    TimeRange,
    /// No usable index — scan all live rows.
    FullScan,
}

/// The plan the accelerator will (or did) execute.
#[derive(Debug, Clone)]
pub struct AccessPlan {
    /// The seeding (most selective) path.
    pub driving_path: AccessPath,
    /// Estimated cardinality of the driving path.
    pub driving_cardinality: usize,
    /// Remaining paths, applied as intersections in this order.
    pub intersect_paths: Vec<AccessPath>,
    /// Whether a full scan was required.
    pub full_scan: bool,
    /// Estimated number of rows after all index probes.
    pub estimated_rows: usize,
}

/// An index-backed accelerator over an [`AuditIndex`].
pub struct QueryAccelerator<'a> {
    index: &'a AuditIndex,
}

impl<'a> QueryAccelerator<'a> {
    /// Wraps an index.
    pub fn new(index: &'a AuditIndex) -> Self {
        Self { index }
    }

    /// Builds an access plan from cheap cardinality estimates (no row probing).
    pub fn plan(&self, query: &IndexQuery) -> AccessPlan {
        let mut paths: Vec<(AccessPath, usize)> = Vec::new();
        if !query.statute_ids.is_empty() {
            paths.push((
                AccessPath::Statute,
                self.index.statute_cardinality(&query.statute_ids),
            ));
        }
        if !query.subject_ids.is_empty() {
            paths.push((
                AccessPath::Subject,
                self.index.subject_cardinality(&query.subject_ids),
            ));
        }
        if !query.event_types.is_empty() {
            paths.push((
                AccessPath::EventType,
                self.index.event_type_cardinality(&query.event_types),
            ));
        }
        if !query.actor_kinds.is_empty() {
            paths.push((
                AccessPath::ActorKind,
                self.index.actor_kind_cardinality(&query.actor_kinds),
            ));
        }
        if !query.result_kinds.is_empty() {
            paths.push((
                AccessPath::ResultKind,
                self.index.result_kind_cardinality(&query.result_kinds),
            ));
        }
        if query.start.is_some() || query.end.is_some() {
            paths.push((
                AccessPath::TimeRange,
                self.index.time_cardinality(query.start, query.end),
            ));
        }

        if paths.is_empty() {
            let n = self.index.live_len();
            return AccessPlan {
                driving_path: AccessPath::FullScan,
                driving_cardinality: n,
                intersect_paths: Vec::new(),
                full_scan: true,
                estimated_rows: n,
            };
        }

        // Most selective first.
        paths.sort_by_key(|(_, card)| *card);
        let (driving_path, driving_cardinality) = paths[0].clone();
        let intersect_paths = paths[1..].iter().map(|(p, _)| p.clone()).collect();

        AccessPlan {
            driving_path,
            driving_cardinality,
            intersect_paths,
            full_scan: false,
            // The intersection cannot be larger than the smallest probe.
            estimated_rows: driving_cardinality,
        }
    }

    /// Executes the query and returns the matching live row ids (sorted).
    pub fn candidate_rows(&self, query: &IndexQuery) -> Vec<RowId> {
        let mut lists = self.candidate_lists(query);
        if lists.is_empty() {
            return self.index.live_rows_sorted();
        }
        // Smallest first to minimise intersection work.
        lists.sort_by_key(|l| l.len());
        let mut acc = lists.remove(0);
        for list in &lists {
            if acc.is_empty() {
                break;
            }
            acc = intersect_sorted(&acc, list);
        }
        acc
    }

    /// Executes the query against `storage`, applying the exact predicate,
    /// time-ordering, offset and limit. Records that vanished from `storage`
    /// since indexing are skipped rather than erroring.
    pub fn execute(
        &self,
        query: &IndexQuery,
        storage: &dyn AuditStorage,
    ) -> AuditResult<Vec<AuditRecord>> {
        let rows = self.candidate_rows(query);
        let ids = self.index.resolve(&rows);
        let records = fetch_records(storage, &ids)?;
        Ok(finalize_records(records, query))
    }

    /// Accelerates a [`QueryBuilder`]: probe the index for a candidate superset,
    /// fetch those records, then apply the builder's full filter + pagination.
    /// The result is identical to `builder.execute(&storage.get_all()?)`.
    pub fn execute_builder(
        &self,
        builder: &QueryBuilder,
        storage: &dyn AuditStorage,
    ) -> AuditResult<Vec<AuditRecord>> {
        let iq = IndexQuery::from_query_builder(builder);
        let rows = self.candidate_rows(&iq);
        let ids = self.index.resolve(&rows);
        let records = fetch_records(storage, &ids)?;
        Ok(builder.execute(&records))
    }

    /// Builds the candidate row lists for every constrained field.
    fn candidate_lists(&self, query: &IndexQuery) -> Vec<Vec<RowId>> {
        let mut lists: Vec<Vec<RowId>> = Vec::new();
        if !query.statute_ids.is_empty() {
            lists.push(self.index.statute_candidates(&query.statute_ids));
        }
        if !query.subject_ids.is_empty() {
            lists.push(self.index.subject_candidates(&query.subject_ids));
        }
        if !query.event_types.is_empty() {
            lists.push(self.index.event_type_candidates(&query.event_types));
        }
        if !query.actor_kinds.is_empty() {
            lists.push(self.index.actor_kind_candidates(&query.actor_kinds));
        }
        if !query.result_kinds.is_empty() {
            lists.push(self.index.result_kind_candidates(&query.result_kinds));
        }
        if query.start.is_some() || query.end.is_some() {
            lists.push(self.index.time_candidates(query.start, query.end));
        }
        lists
    }
}

/// Fetches records by id from `storage`, skipping ones that have since been
/// removed (a benign index/storage race) and propagating any other error.
pub(crate) fn fetch_records(
    storage: &dyn AuditStorage,
    ids: &[Uuid],
) -> AuditResult<Vec<AuditRecord>> {
    let mut records = Vec::with_capacity(ids.len());
    for id in ids {
        match storage.get(*id) {
            Ok(record) => records.push(record),
            Err(AuditError::RecordNotFound(_)) => {}
            Err(other) => return Err(other),
        }
    }
    Ok(records)
}

/// Applies the exact (precise) time bound, deterministic time ordering, then
/// offset and limit to a candidate record set.
pub(crate) fn finalize_records(
    mut records: Vec<AuditRecord>,
    query: &IndexQuery,
) -> Vec<AuditRecord> {
    if query.start.is_some() || query.end.is_some() {
        records.retain(|r| {
            query.start.map(|s| r.timestamp >= s).unwrap_or(true)
                && query.end.map(|e| r.timestamp <= e).unwrap_or(true)
        });
    }
    records.sort_by(|a, b| a.timestamp.cmp(&b.timestamp).then(a.id.cmp(&b.id)));
    if let Some(offset) = query.offset {
        if offset >= records.len() {
            records.clear();
        } else {
            records.drain(0..offset);
        }
    }
    if let Some(limit) = query.limit {
        records.truncate(limit);
    }
    records
}

fn event_type_code(event_type: &EventType) -> u8 {
    match event_type {
        EventType::AutomaticDecision => 0,
        EventType::DiscretionaryReview => 1,
        EventType::HumanOverride => 2,
        EventType::Appeal => 3,
        EventType::StatuteModified => 4,
        EventType::SimulationRun => 5,
    }
}

fn actor_kind_code(kind: ActorKind) -> u8 {
    match kind {
        ActorKind::System => 0,
        ActorKind::User => 1,
        ActorKind::External => 2,
    }
}

fn result_kind_code(kind: ResultKind) -> u8 {
    match kind {
        ResultKind::Deterministic => 0,
        ResultKind::RequiresDiscretion => 1,
        ResultKind::Void => 2,
        ResultKind::Overridden => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::{ActorFilter, QueryBuilder};
    use crate::storage::memory::MemoryStorage;
    use crate::{Actor, DecisionContext, DecisionResult};
    use chrono::Duration;
    use std::collections::HashMap as Map;

    fn seed() -> (MemoryStorage, AuditIndex, Vec<AuditRecord>) {
        let mut storage = MemoryStorage::new();
        let mut index = AuditIndex::new();
        let base = Utc::now();
        let subject = Uuid::new_v4();
        let mut records = Vec::new();
        for i in 0..20u32 {
            let statute = format!("statute-{}", i % 4);
            let subj = if i % 5 == 0 { subject } else { Uuid::new_v4() };
            let mut r = AuditRecord::new(
                if i % 3 == 0 {
                    EventType::HumanOverride
                } else {
                    EventType::AutomaticDecision
                },
                Actor::System {
                    component: "engine".to_string(),
                },
                statute,
                subj,
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "ok".to_string(),
                    parameters: Map::new(),
                },
                None,
            );
            r.timestamp = base + Duration::minutes(i as i64);
            index.insert(&r);
            storage.store(r.clone()).expect("store");
            records.push(r);
        }
        (storage, index, records)
    }

    #[test]
    fn test_accelerated_statute_matches_full_scan() {
        let (storage, index, records) = seed();
        let accel = QueryAccelerator::new(&index);

        let iq = IndexQuery::new().statute("statute-1");
        let fast = accel.execute(&iq, &storage).expect("execute");

        let slow: Vec<_> = records
            .iter()
            .filter(|r| r.statute_id == "statute-1")
            .cloned()
            .collect();
        assert_eq!(fast.len(), slow.len());
        assert!(!fast.is_empty());
        for r in &fast {
            assert_eq!(r.statute_id, "statute-1");
        }
    }

    #[test]
    fn test_plan_picks_most_selective() {
        let (_storage, index, _records) = seed();
        let accel = QueryAccelerator::new(&index);

        // statute-1 has ~5 rows; AutomaticDecision has ~13 rows; the planner
        // should drive from the smaller statute set.
        let iq = IndexQuery::new()
            .statute("statute-1")
            .event_type(EventType::AutomaticDecision);
        let plan = accel.plan(&iq);
        assert_eq!(plan.driving_path, AccessPath::Statute);
        assert!(plan.intersect_paths.contains(&AccessPath::EventType));
        assert!(!plan.full_scan);
    }

    #[test]
    fn test_plan_full_scan_when_unconstrained() {
        let (_storage, index, _records) = seed();
        let accel = QueryAccelerator::new(&index);
        let plan = accel.plan(&IndexQuery::new());
        assert!(plan.full_scan);
        assert_eq!(plan.driving_path, AccessPath::FullScan);
    }

    #[test]
    fn test_intersection_of_two_fields() {
        let (storage, index, records) = seed();
        let accel = QueryAccelerator::new(&index);

        let iq = IndexQuery::new()
            .statute("statute-0")
            .event_type(EventType::HumanOverride);
        let fast = accel.execute(&iq, &storage).expect("execute");

        let slow: Vec<_> = records
            .iter()
            .filter(|r| {
                r.statute_id == "statute-0" && matches!(r.event_type, EventType::HumanOverride)
            })
            .cloned()
            .collect();
        assert_eq!(fast.len(), slow.len());
        for r in &fast {
            assert_eq!(r.statute_id, "statute-0");
            assert!(matches!(r.event_type, EventType::HumanOverride));
        }
    }

    #[test]
    fn test_time_range_exact_bounds() {
        let (storage, index, records) = seed();
        let accel = QueryAccelerator::new(&index);
        let base = records[0].timestamp;

        let iq =
            IndexQuery::new().time_range(base + Duration::minutes(3), base + Duration::minutes(7));
        let fast = accel.execute(&iq, &storage).expect("execute");
        // minutes 3,4,5,6,7 inclusive => 5 records.
        assert_eq!(fast.len(), 5);
        // Returned in ascending time order.
        assert!(fast.windows(2).all(|w| w[0].timestamp <= w[1].timestamp));
    }

    #[test]
    fn test_offset_and_limit() {
        let (storage, index, _records) = seed();
        let accel = QueryAccelerator::new(&index);
        let iq = IndexQuery::new()
            .event_type(EventType::AutomaticDecision)
            .offset(2)
            .limit(3);
        let fast = accel.execute(&iq, &storage).expect("execute");
        assert_eq!(fast.len(), 3);
    }

    #[test]
    fn test_execute_builder_matches_full_scan() {
        let (storage, index, records) = seed();
        let accel = QueryAccelerator::new(&index);

        let builder = QueryBuilder::new()
            .statute_id("statute-2")
            .actor(ActorFilter::AnySystem)
            .limit(2);
        let fast = accel.execute_builder(&builder, &storage).expect("execute");
        let slow = builder.execute(&records);
        assert_eq!(fast.len(), slow.len());
        assert!(fast.len() <= 2);
    }

    #[test]
    fn test_signature_is_order_insensitive() {
        let a = IndexQuery::new().statute("x").statute("y");
        let b = IndexQuery::new().statute("y").statute("x");
        assert_eq!(a.signature(), b.signature());

        let c = IndexQuery::new().statute("z");
        assert_ne!(a.signature(), c.signature());
    }

    #[test]
    fn test_candidate_rows_unconstrained_is_full() {
        let (_storage, index, records) = seed();
        let accel = QueryAccelerator::new(&index);
        let rows = accel.candidate_rows(&IndexQuery::new());
        assert_eq!(rows.len(), records.len());
    }
}
