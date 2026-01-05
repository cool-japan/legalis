//! Advanced temporal logic for legal reasoning.
//!
//! This module provides comprehensive temporal reasoning capabilities including:
//! - Allen's interval algebra for temporal relations
//! - Event calculus for legal narrative reasoning
//! - Timeline merging for multi-statute histories
//! - Temporal query language
//! - Bitemporal modeling (valid time + transaction time)

use chrono::{DateTime, NaiveDate, Utc};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Allen's interval algebra relation types.
///
/// These thirteen relations comprehensively describe all possible
/// relationships between two time intervals.
///
/// # Examples
///
/// ```
/// use legalis_core::temporal::{AllenRelation, TimeInterval};
/// use chrono::NaiveDate;
///
/// let interval1 = TimeInterval::new(
///     NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
///     NaiveDate::from_ymd_opt(2025, 6, 30).unwrap(),
/// );
/// let interval2 = TimeInterval::new(
///     NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
///     NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
/// );
///
/// assert_eq!(interval1.relate(&interval2), AllenRelation::Before);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum AllenRelation {
    /// Interval A ends before interval B starts (A < B).
    Before,
    /// Interval A meets interval B (A.end == B.start).
    Meets,
    /// Interval A overlaps with interval B (A.start < B.start < A.end < B.end).
    Overlaps,
    /// Interval A finishes at the same time as B (A.end == B.end, A.start > B.start).
    FinishedBy,
    /// Interval A contains interval B (A.start < B.start, A.end > B.end).
    Contains,
    /// Interval A starts at the same time as B (A.start == B.start, A.end < B.end).
    Starts,
    /// Interval A equals interval B (same start and end).
    Equal,
    /// Interval A is started by interval B (inverse of Starts).
    StartedBy,
    /// Interval A is during interval B (inverse of Contains).
    During,
    /// Interval A finishes at the same time as B (inverse of FinishedBy).
    Finishes,
    /// Interval A is overlapped by B (inverse of Overlaps).
    OverlappedBy,
    /// Interval A is met by B (inverse of Meets).
    MetBy,
    /// Interval A is after B (inverse of Before).
    After,
}

impl fmt::Display for AllenRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllenRelation::Before => write!(f, "before"),
            AllenRelation::Meets => write!(f, "meets"),
            AllenRelation::Overlaps => write!(f, "overlaps"),
            AllenRelation::FinishedBy => write!(f, "finished-by"),
            AllenRelation::Contains => write!(f, "contains"),
            AllenRelation::Starts => write!(f, "starts"),
            AllenRelation::Equal => write!(f, "equals"),
            AllenRelation::StartedBy => write!(f, "started-by"),
            AllenRelation::During => write!(f, "during"),
            AllenRelation::Finishes => write!(f, "finishes"),
            AllenRelation::OverlappedBy => write!(f, "overlapped-by"),
            AllenRelation::MetBy => write!(f, "met-by"),
            AllenRelation::After => write!(f, "after"),
        }
    }
}

/// A time interval with a start and end date.
///
/// # Examples
///
/// ```
/// use legalis_core::temporal::TimeInterval;
/// use chrono::NaiveDate;
///
/// let interval = TimeInterval::new(
///     NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
///     NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
/// );
///
/// assert_eq!(interval.duration_days(), 364);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TimeInterval {
    /// Start date of the interval (inclusive).
    pub start: NaiveDate,
    /// End date of the interval (inclusive).
    pub end: NaiveDate,
}

impl TimeInterval {
    /// Creates a new time interval.
    ///
    /// # Panics
    ///
    /// Panics if start > end.
    pub fn new(start: NaiveDate, end: NaiveDate) -> Self {
        assert!(
            start <= end,
            "Start date must be before or equal to end date"
        );
        Self { start, end }
    }

    /// Creates a time interval, returning None if start > end.
    pub fn try_new(start: NaiveDate, end: NaiveDate) -> Option<Self> {
        if start <= end {
            Some(Self { start, end })
        } else {
            None
        }
    }

    /// Returns the duration of the interval in days.
    pub fn duration_days(&self) -> i64 {
        (self.end - self.start).num_days()
    }

    /// Checks if this interval contains the given date.
    pub fn contains_date(&self, date: NaiveDate) -> bool {
        date >= self.start && date <= self.end
    }

    /// Determines the Allen relation between this interval and another.
    pub fn relate(&self, other: &TimeInterval) -> AllenRelation {
        if self.end < other.start {
            AllenRelation::Before
        } else if self.end == other.start {
            AllenRelation::Meets
        } else if self.start < other.start && self.end > other.start && self.end < other.end {
            AllenRelation::Overlaps
        } else if self.start > other.start && self.end == other.end {
            AllenRelation::Finishes
        } else if self.start < other.start && self.end > other.end {
            AllenRelation::Contains
        } else if self.start == other.start && self.end < other.end {
            AllenRelation::Starts
        } else if self.start == other.start && self.end == other.end {
            AllenRelation::Equal
        } else if self.start == other.start && self.end > other.end {
            AllenRelation::StartedBy
        } else if self.start > other.start && self.end < other.end {
            AllenRelation::During
        } else if self.start < other.start && self.end == other.end {
            AllenRelation::FinishedBy
        } else if self.start > other.start && self.start < other.end && self.end > other.end {
            AllenRelation::OverlappedBy
        } else if self.start == other.end {
            AllenRelation::MetBy
        } else {
            AllenRelation::After
        }
    }

    /// Computes the intersection of two intervals, if it exists.
    pub fn intersection(&self, other: &TimeInterval) -> Option<TimeInterval> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        TimeInterval::try_new(start, end)
    }

    /// Computes the union of two intervals if they overlap or are adjacent.
    pub fn union(&self, other: &TimeInterval) -> Option<TimeInterval> {
        match self.relate(other) {
            AllenRelation::Before | AllenRelation::After => None,
            _ => Some(TimeInterval::new(
                self.start.min(other.start),
                self.end.max(other.end),
            )),
        }
    }
}

impl fmt::Display for TimeInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} to {}]", self.start, self.end)
    }
}

/// Event type for event calculus.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum EventType {
    /// Statute enactment.
    Enactment,
    /// Statute amendment.
    Amendment,
    /// Statute repeal.
    Repeal,
    /// Case decision.
    Decision,
    /// Contract signing.
    ContractSigning,
    /// Contract termination.
    ContractTermination,
    /// Custom event type.
    Custom(String),
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::Enactment => write!(f, "enactment"),
            EventType::Amendment => write!(f, "amendment"),
            EventType::Repeal => write!(f, "repeal"),
            EventType::Decision => write!(f, "decision"),
            EventType::ContractSigning => write!(f, "contract-signing"),
            EventType::ContractTermination => write!(f, "contract-termination"),
            EventType::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Legal event for event calculus.
///
/// # Examples
///
/// ```
/// use legalis_core::temporal::{LegalEvent, EventType};
/// use chrono::Utc;
///
/// let event = LegalEvent::new(
///     "statute-123-enacted",
///     EventType::Enactment,
///     Utc::now(),
/// );
///
/// assert_eq!(event.event_type, EventType::Enactment);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegalEvent {
    /// Unique identifier for the event.
    pub id: String,
    /// Type of event.
    pub event_type: EventType,
    /// Timestamp when the event occurred.
    pub timestamp: DateTime<Utc>,
    /// Optional reference to a statute ID.
    pub statute_id: Option<String>,
    /// Optional reference to a case ID.
    pub case_id: Option<String>,
    /// Additional metadata.
    pub metadata: HashMap<String, String>,
}

impl LegalEvent {
    /// Creates a new legal event.
    pub fn new(id: impl Into<String>, event_type: EventType, timestamp: DateTime<Utc>) -> Self {
        Self {
            id: id.into(),
            event_type,
            timestamp,
            statute_id: None,
            case_id: None,
            metadata: HashMap::new(),
        }
    }

    /// Adds a statute reference.
    pub fn with_statute(mut self, statute_id: impl Into<String>) -> Self {
        self.statute_id = Some(statute_id.into());
        self
    }

    /// Adds a case reference.
    pub fn with_case(mut self, case_id: impl Into<String>) -> Self {
        self.case_id = Some(case_id.into());
        self
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Fluent state for event calculus.
///
/// A fluent is a property that may change over time.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Fluent {
    /// Name of the fluent (e.g., "in_force", "applies_to").
    pub name: String,
    /// Arguments (e.g., statute ID, jurisdiction).
    pub args: Vec<String>,
}

impl Fluent {
    /// Creates a new fluent.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            args: Vec::new(),
        }
    }

    /// Adds an argument.
    pub fn with_arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }
}

impl fmt::Display for Fluent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.args.join(", "))
    }
}

/// Event calculus engine for legal narrative reasoning.
///
/// The event calculus is a logical formalism for representing and reasoning
/// about actions and their effects over time.
///
/// # Examples
///
/// ```
/// use legalis_core::temporal::{EventCalculus, LegalEvent, EventType, Fluent};
/// use chrono::Utc;
///
/// let mut ec = EventCalculus::new();
///
/// // Define initial state
/// let fluent = Fluent::new("in_force").with_arg("statute-123");
/// ec.set_initially_true(fluent.clone());
///
/// // Add event
/// let event = LegalEvent::new("repeal-123", EventType::Repeal, Utc::now())
///     .with_statute("statute-123");
/// ec.add_event(event);
///
/// // Query state
/// assert!(ec.initially_true(&fluent));
/// ```
#[derive(Debug, Clone)]
pub struct EventCalculus {
    /// Initial fluents that are true at the start.
    initially: HashSet<Fluent>,
    /// Events ordered by timestamp.
    events: Vec<LegalEvent>,
    /// Initiates rules: event -> fluents that become true.
    initiates: HashMap<String, HashSet<Fluent>>,
    /// Terminates rules: event -> fluents that become false.
    terminates: HashMap<String, HashSet<Fluent>>,
}

impl EventCalculus {
    /// Creates a new event calculus engine.
    pub fn new() -> Self {
        Self {
            initially: HashSet::new(),
            events: Vec::new(),
            initiates: HashMap::new(),
            terminates: HashMap::new(),
        }
    }

    /// Sets a fluent as initially true.
    pub fn set_initially_true(&mut self, fluent: Fluent) {
        self.initially.insert(fluent);
    }

    /// Checks if a fluent is initially true.
    pub fn initially_true(&self, fluent: &Fluent) -> bool {
        self.initially.contains(fluent)
    }

    /// Adds a legal event.
    pub fn add_event(&mut self, event: LegalEvent) {
        self.events.push(event);
        self.events.sort_by_key(|e| e.timestamp);
    }

    /// Adds an initiates rule: event initiates fluent.
    pub fn add_initiates(&mut self, event_id: impl Into<String>, fluent: Fluent) {
        self.initiates
            .entry(event_id.into())
            .or_default()
            .insert(fluent);
    }

    /// Adds a terminates rule: event terminates fluent.
    pub fn add_terminates(&mut self, event_id: impl Into<String>, fluent: Fluent) {
        self.terminates
            .entry(event_id.into())
            .or_default()
            .insert(fluent);
    }

    /// Checks if a fluent holds at a given time.
    ///
    /// This implements the basic event calculus query:
    /// HoldsAt(fluent, time) is true if:
    /// 1. Initially(fluent) is true and no event has terminated it before time, OR
    /// 2. Some event initiated it before time and no subsequent event terminated it
    pub fn holds_at(&self, fluent: &Fluent, time: DateTime<Utc>) -> bool {
        let mut holds = self.initially.contains(fluent);

        for event in &self.events {
            if event.timestamp > time {
                break;
            }

            // Check if this event initiates the fluent
            if let Some(initiated) = self.initiates.get(&event.id) {
                if initiated.contains(fluent) {
                    holds = true;
                }
            }

            // Check if this event terminates the fluent
            if let Some(terminated) = self.terminates.get(&event.id) {
                if terminated.contains(fluent) {
                    holds = false;
                }
            }
        }

        holds
    }

    /// Returns all events in chronological order.
    pub fn events(&self) -> &[LegalEvent] {
        &self.events
    }

    /// Returns events within a time interval.
    pub fn events_in_interval(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&LegalEvent> {
        self.events
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }
}

impl Default for EventCalculus {
    fn default() -> Self {
        Self::new()
    }
}

/// Timeline entry for statute history.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimelineEntry {
    /// Statute ID.
    pub statute_id: String,
    /// Validity interval.
    pub interval: TimeInterval,
    /// Version number.
    pub version: u32,
    /// Amendment notes.
    pub notes: Option<String>,
}

impl TimelineEntry {
    /// Creates a new timeline entry.
    pub fn new(statute_id: impl Into<String>, interval: TimeInterval, version: u32) -> Self {
        Self {
            statute_id: statute_id.into(),
            interval,
            version,
            notes: None,
        }
    }

    /// Adds notes.
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }
}

/// Timeline merger for multi-statute histories.
///
/// # Examples
///
/// ```
/// use legalis_core::temporal::{Timeline, TimelineEntry, TimeInterval};
/// use chrono::NaiveDate;
///
/// let mut timeline = Timeline::new();
///
/// let entry1 = TimelineEntry::new(
///     "tax-law",
///     TimeInterval::new(
///         NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
///         NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
///     ),
///     1,
/// );
///
/// timeline.add_entry(entry1);
/// assert_eq!(timeline.entries().len(), 1);
/// ```
#[derive(Debug, Clone, Default)]
pub struct Timeline {
    entries: Vec<TimelineEntry>,
}

impl Timeline {
    /// Creates a new timeline.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Adds a timeline entry.
    pub fn add_entry(&mut self, entry: TimelineEntry) {
        self.entries.push(entry);
        self.entries
            .sort_by_key(|e| (e.statute_id.clone(), e.interval.start));
    }

    /// Returns all entries.
    pub fn entries(&self) -> &[TimelineEntry] {
        &self.entries
    }

    /// Returns entries for a specific statute.
    pub fn entries_for_statute(&self, statute_id: &str) -> Vec<&TimelineEntry> {
        self.entries
            .iter()
            .filter(|e| e.statute_id == statute_id)
            .collect()
    }

    /// Returns the active version of a statute at a given date.
    pub fn active_version_at(&self, statute_id: &str, date: NaiveDate) -> Option<&TimelineEntry> {
        self.entries
            .iter()
            .filter(|e| e.statute_id == statute_id && e.interval.contains_date(date))
            .max_by_key(|e| e.version)
    }

    /// Merges two timelines.
    pub fn merge(&mut self, other: Timeline) {
        for entry in other.entries {
            self.add_entry(entry);
        }
    }

    /// Detects gaps in the timeline for a statute.
    pub fn detect_gaps(&self, statute_id: &str) -> Vec<TimeInterval> {
        let mut entries: Vec<_> = self.entries_for_statute(statute_id);
        entries.sort_by_key(|e| e.interval.start);

        let mut gaps = Vec::new();
        for window in entries.windows(2) {
            let current = &window[0].interval;
            let next = &window[1].interval;

            if current.end < next.start {
                if let Some(gap) = TimeInterval::try_new(
                    current.end.succ_opt().unwrap_or(current.end),
                    next.start.pred_opt().unwrap_or(next.start),
                ) {
                    gaps.push(gap);
                }
            }
        }

        gaps
    }

    /// Detects overlaps in the timeline for a statute.
    pub fn detect_overlaps(&self, statute_id: &str) -> Vec<(TimeInterval, u32, u32)> {
        let mut entries: Vec<_> = self.entries_for_statute(statute_id);
        entries.sort_by_key(|e| e.interval.start);

        let mut overlaps = Vec::new();
        for i in 0..entries.len() {
            for j in (i + 1)..entries.len() {
                if let Some(overlap) = entries[i].interval.intersection(&entries[j].interval) {
                    overlaps.push((overlap, entries[i].version, entries[j].version));
                }
            }
        }

        overlaps
    }
}

/// Bitemporal time representation.
///
/// Bitemporal modeling tracks two distinct timelines:
/// - **Valid time**: When the fact was true in reality
/// - **Transaction time**: When the fact was recorded in the database
///
/// # Examples
///
/// ```
/// use legalis_core::temporal::BitemporalTime;
/// use chrono::{NaiveDate, Utc};
///
/// let bt = BitemporalTime::new(
///     NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),  // Valid from
///     NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(), // Valid to
///     Utc::now(),                                      // Recorded at
/// );
///
/// assert!(bt.is_currently_valid(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap()));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BitemporalTime {
    /// When the fact was valid in reality (start).
    pub valid_from: NaiveDate,
    /// When the fact was valid in reality (end).
    pub valid_to: NaiveDate,
    /// When the fact was recorded in the system.
    pub transaction_time: DateTime<Utc>,
    /// When the fact was superseded in the system (if any).
    pub transaction_to: Option<DateTime<Utc>>,
}

impl BitemporalTime {
    /// Creates a new bitemporal time.
    pub fn new(
        valid_from: NaiveDate,
        valid_to: NaiveDate,
        transaction_time: DateTime<Utc>,
    ) -> Self {
        Self {
            valid_from,
            valid_to,
            transaction_time,
            transaction_to: None,
        }
    }

    /// Marks this record as superseded.
    pub fn supersede(mut self, transaction_to: DateTime<Utc>) -> Self {
        self.transaction_to = Some(transaction_to);
        self
    }

    /// Checks if this record is currently valid (in valid time).
    pub fn is_currently_valid(&self, as_of: NaiveDate) -> bool {
        as_of >= self.valid_from && as_of <= self.valid_to
    }

    /// Checks if this record is current (in transaction time).
    pub fn is_current(&self) -> bool {
        self.transaction_to.is_none()
    }

    /// Gets the valid time interval.
    pub fn valid_interval(&self) -> TimeInterval {
        TimeInterval::new(self.valid_from, self.valid_to)
    }
}

/// Bitemporal statute record.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BitemporalStatute {
    /// Statute ID.
    pub statute_id: String,
    /// Version number.
    pub version: u32,
    /// Bitemporal time.
    pub time: BitemporalTime,
    /// Additional data (e.g., full statute content).
    pub data: HashMap<String, String>,
}

impl BitemporalStatute {
    /// Creates a new bitemporal statute record.
    pub fn new(statute_id: impl Into<String>, version: u32, time: BitemporalTime) -> Self {
        Self {
            statute_id: statute_id.into(),
            version,
            time,
            data: HashMap::new(),
        }
    }

    /// Adds data.
    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }
}

/// Bitemporal statute database.
///
/// # Examples
///
/// ```
/// use legalis_core::temporal::{BitemporalDatabase, BitemporalStatute, BitemporalTime};
/// use chrono::{Utc, NaiveDate};
///
/// let mut db = BitemporalDatabase::new();
///
/// let record = BitemporalStatute::new(
///     "tax-law",
///     1,
///     BitemporalTime::new(
///         NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
///         NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
///         Utc::now(),
///     ),
/// );
///
/// db.insert(record);
/// assert_eq!(db.len(), 1);
/// ```
#[derive(Debug, Clone, Default)]
pub struct BitemporalDatabase {
    records: Vec<BitemporalStatute>,
}

impl BitemporalDatabase {
    /// Creates a new bitemporal database.
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    /// Inserts a new record.
    pub fn insert(&mut self, record: BitemporalStatute) {
        self.records.push(record);
    }

    /// Returns the number of records.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Checks if the database is empty.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Queries records valid at a specific valid time and transaction time.
    pub fn query_as_of(
        &self,
        valid_time: NaiveDate,
        transaction_time: DateTime<Utc>,
    ) -> Vec<&BitemporalStatute> {
        self.records
            .iter()
            .filter(|r| {
                r.time.is_currently_valid(valid_time)
                    && r.time.transaction_time <= transaction_time
                    && r.time.transaction_to.is_none_or(|t| t > transaction_time)
            })
            .collect()
    }

    /// Queries current records (latest transaction time).
    pub fn query_current(&self, valid_time: NaiveDate) -> Vec<&BitemporalStatute> {
        self.records
            .iter()
            .filter(|r| r.time.is_currently_valid(valid_time) && r.time.is_current())
            .collect()
    }

    /// Returns the entire history for a statute.
    pub fn history(&self, statute_id: &str) -> Vec<&BitemporalStatute> {
        self.records
            .iter()
            .filter(|r| r.statute_id == statute_id)
            .collect()
    }

    /// Returns all current records.
    pub fn all_current(&self) -> Vec<&BitemporalStatute> {
        self.records
            .iter()
            .filter(|r| r.time.is_current())
            .collect()
    }
}

/// Temporal query for filtering statutes by time.
///
/// # Examples
///
/// ```
/// use legalis_core::temporal::{TemporalQuery, BitemporalDatabase};
/// use chrono::{NaiveDate, Utc};
///
/// let db = BitemporalDatabase::new();
/// let query = TemporalQuery::new()
///     .valid_on(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap())
///     .current_only(true);
///
/// let results = query.execute(&db);
/// assert_eq!(results.len(), 0); // Empty database
/// ```
#[derive(Debug, Clone)]
pub struct TemporalQuery {
    valid_on: Option<NaiveDate>,
    transaction_at: Option<DateTime<Utc>>,
    current_only: bool,
    statute_id: Option<String>,
}

impl TemporalQuery {
    /// Creates a new temporal query.
    pub fn new() -> Self {
        Self {
            valid_on: None,
            transaction_at: None,
            current_only: false,
            statute_id: None,
        }
    }

    /// Filters by valid time.
    pub fn valid_on(mut self, date: NaiveDate) -> Self {
        self.valid_on = Some(date);
        self
    }

    /// Filters by transaction time.
    pub fn transaction_at(mut self, time: DateTime<Utc>) -> Self {
        self.transaction_at = Some(time);
        self
    }

    /// Filters to current records only.
    pub fn current_only(mut self, current: bool) -> Self {
        self.current_only = current;
        self
    }

    /// Filters by statute ID.
    pub fn statute_id(mut self, id: impl Into<String>) -> Self {
        self.statute_id = Some(id.into());
        self
    }

    /// Executes the query against a bitemporal database.
    pub fn execute<'a>(&self, db: &'a BitemporalDatabase) -> Vec<&'a BitemporalStatute> {
        let mut results: Vec<&BitemporalStatute> = db.records.iter().collect();

        if let Some(valid_on) = self.valid_on {
            results.retain(|r| r.time.is_currently_valid(valid_on));
        }

        if let Some(transaction_at) = self.transaction_at {
            results.retain(|r| {
                r.time.transaction_time <= transaction_at
                    && r.time.transaction_to.is_none_or(|t| t > transaction_at)
            });
        }

        if self.current_only {
            results.retain(|r| r.time.is_current());
        }

        if let Some(statute_id) = &self.statute_id {
            results.retain(|r| r.statute_id == *statute_id);
        }

        results
    }
}

impl Default for TemporalQuery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_allen_relations() {
        let i1 = TimeInterval::new(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2025, 6, 30).unwrap(),
        );
        let i2 = TimeInterval::new(
            NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
        );

        assert_eq!(i1.relate(&i2), AllenRelation::Before);
        assert_eq!(i2.relate(&i1), AllenRelation::After);
    }

    #[test]
    fn test_interval_intersection() {
        let i1 = TimeInterval::new(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2025, 6, 30).unwrap(),
        );
        let i2 = TimeInterval::new(
            NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            NaiveDate::from_ymd_opt(2025, 9, 30).unwrap(),
        );

        let intersection = i1.intersection(&i2).unwrap();
        assert_eq!(
            intersection.start,
            NaiveDate::from_ymd_opt(2025, 4, 1).unwrap()
        );
        assert_eq!(
            intersection.end,
            NaiveDate::from_ymd_opt(2025, 6, 30).unwrap()
        );
    }

    #[test]
    fn test_event_calculus() {
        let mut ec = EventCalculus::new();

        let fluent = Fluent::new("in_force").with_arg("statute-123");
        ec.set_initially_true(fluent.clone());

        let event = LegalEvent::new("repeal-123", EventType::Repeal, Utc::now())
            .with_statute("statute-123");
        let event_time = event.timestamp;

        ec.add_terminates("repeal-123", fluent.clone());
        ec.add_event(event);

        // Before the event, fluent holds
        assert!(ec.holds_at(&fluent, event_time - chrono::Duration::seconds(1)));

        // After the event, fluent does not hold
        assert!(!ec.holds_at(&fluent, event_time + chrono::Duration::seconds(1)));
    }

    #[test]
    fn test_timeline() {
        let mut timeline = Timeline::new();

        let entry1 = TimelineEntry::new(
            "tax-law",
            TimeInterval::new(
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
            ),
            1,
        );

        timeline.add_entry(entry1);
        assert_eq!(timeline.entries().len(), 1);

        let active =
            timeline.active_version_at("tax-law", NaiveDate::from_ymd_opt(2021, 6, 1).unwrap());
        assert!(active.is_some());
        assert_eq!(active.unwrap().version, 1);
    }

    #[test]
    fn test_bitemporal_database() {
        let mut db = BitemporalDatabase::new();

        let record = BitemporalStatute::new(
            "tax-law",
            1,
            BitemporalTime::new(
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
                Utc::now(),
            ),
        );

        db.insert(record);
        assert_eq!(db.len(), 1);

        let current = db.query_current(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap());
        assert_eq!(current.len(), 1);
    }

    #[test]
    fn test_temporal_query() {
        let mut db = BitemporalDatabase::new();

        let record = BitemporalStatute::new(
            "tax-law",
            1,
            BitemporalTime::new(
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
                Utc::now(),
            ),
        );

        db.insert(record);

        let query = TemporalQuery::new()
            .valid_on(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap())
            .current_only(true);

        let results = query.execute(&db);
        assert_eq!(results.len(), 1);
    }
}
