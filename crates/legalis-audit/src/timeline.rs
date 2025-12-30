//! Timeline reconstruction tools for forensic analysis.
//!
//! This module provides timeline reconstruction capabilities:
//! - Chronological event timelines
//! - Actor-based timelines
//! - Statute-based timelines
//! - Timeline visualization data
//! - Event correlation

use crate::{Actor, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A timeline event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Record ID
    pub record_id: Uuid,
    /// Event type description
    pub event_type: String,
    /// Actor description
    pub actor: String,
    /// Statute ID
    pub statute_id: String,
    /// Subject ID
    pub subject_id: Uuid,
    /// Event description
    pub description: String,
    /// Related event IDs
    pub related_events: Vec<Uuid>,
}

/// A reconstructed timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    /// Timeline title
    pub title: String,
    /// Events in chronological order
    pub events: Vec<TimelineEvent>,
    /// Timeline start time
    pub start_time: Option<DateTime<Utc>>,
    /// Timeline end time
    pub end_time: Option<DateTime<Utc>>,
    /// Total duration in seconds
    pub duration_seconds: Option<i64>,
}

impl Timeline {
    /// Creates a new empty timeline.
    pub fn new(title: String) -> Self {
        Self {
            title,
            events: Vec::new(),
            start_time: None,
            end_time: None,
            duration_seconds: None,
        }
    }

    /// Adds an event to the timeline.
    pub fn add_event(&mut self, event: TimelineEvent) {
        self.events.push(event);
        self.sort_and_update();
    }

    /// Sorts events and updates timeline metadata.
    fn sort_and_update(&mut self) {
        if self.events.is_empty() {
            self.start_time = None;
            self.end_time = None;
            self.duration_seconds = None;
            return;
        }

        // Sort by timestamp
        self.events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // Update metadata
        self.start_time = Some(self.events.first().unwrap().timestamp);
        self.end_time = Some(self.events.last().unwrap().timestamp);

        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            self.duration_seconds = Some(end.signed_duration_since(start).num_seconds());
        }
    }

    /// Gets events within a time range.
    pub fn events_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&TimelineEvent> {
        self.events
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }

    /// Gets events for a specific actor.
    pub fn events_by_actor(&self, actor: &str) -> Vec<&TimelineEvent> {
        self.events
            .iter()
            .filter(|e| e.actor.contains(actor))
            .collect()
    }

    /// Gets events for a specific statute.
    pub fn events_by_statute(&self, statute_id: &str) -> Vec<&TimelineEvent> {
        self.events
            .iter()
            .filter(|e| e.statute_id == statute_id)
            .collect()
    }

    /// Gets events for a specific subject.
    pub fn events_by_subject(&self, subject_id: Uuid) -> Vec<&TimelineEvent> {
        self.events
            .iter()
            .filter(|e| e.subject_id == subject_id)
            .collect()
    }

    /// Exports timeline to JSON format.
    pub fn to_json(&self) -> AuditResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| crate::AuditError::ExportError(e.to_string()))
    }

    /// Exports timeline to HTML format.
    pub fn to_html(&self) -> String {
        let mut html = String::from("<html><head><title>");
        html.push_str(&self.title);
        html.push_str("</title><style>");
        html.push_str("body { font-family: Arial, sans-serif; margin: 20px; }");
        html.push_str(".timeline { border-left: 3px solid #3498db; padding-left: 20px; }");
        html.push_str(".event { margin-bottom: 20px; padding: 10px; background: #f8f9fa; border-radius: 5px; }");
        html.push_str(".timestamp { color: #7f8c8d; font-size: 0.9em; }");
        html.push_str(".actor { color: #2980b9; font-weight: bold; }");
        html.push_str("</style></head><body>");
        html.push_str("<h1>");
        html.push_str(&self.title);
        html.push_str("</h1>");

        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            html.push_str("<p>Duration: ");
            html.push_str(&format!(
                "{} to {} ({} seconds)",
                start.format("%Y-%m-%d %H:%M:%S UTC"),
                end.format("%Y-%m-%d %H:%M:%S UTC"),
                self.duration_seconds.unwrap_or(0)
            ));
            html.push_str("</p>");
        }

        html.push_str("<div class='timeline'>");

        for event in &self.events {
            html.push_str("<div class='event'>");
            html.push_str("<div class='timestamp'>");
            html.push_str(&event.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string());
            html.push_str("</div>");
            html.push_str("<div class='actor'>Actor: ");
            html.push_str(&event.actor);
            html.push_str("</div>");
            html.push_str("<div>Event: ");
            html.push_str(&event.event_type);
            html.push_str("</div>");
            html.push_str("<div>Description: ");
            html.push_str(&event.description);
            html.push_str("</div>");
            html.push_str("</div>");
        }

        html.push_str("</div></body></html>");
        html
    }
}

/// Timeline reconstruction builder.
pub struct TimelineReconstructor;

impl TimelineReconstructor {
    /// Reconstructs a complete chronological timeline.
    pub fn reconstruct_chronological(
        records: &[AuditRecord],
        title: String,
    ) -> AuditResult<Timeline> {
        let mut timeline = Timeline::new(title);

        for record in records {
            let event = Self::record_to_event(record);
            timeline.add_event(event);
        }

        Ok(timeline)
    }

    /// Reconstructs a timeline for a specific subject.
    pub fn reconstruct_subject(records: &[AuditRecord], subject_id: Uuid) -> AuditResult<Timeline> {
        let filtered: Vec<_> = records
            .iter()
            .filter(|r| r.subject_id == subject_id)
            .cloned()
            .collect();

        Self::reconstruct_chronological(&filtered, format!("Subject {} Timeline", subject_id))
    }

    /// Reconstructs a timeline for a specific statute.
    pub fn reconstruct_statute(records: &[AuditRecord], statute_id: &str) -> AuditResult<Timeline> {
        let filtered: Vec<_> = records
            .iter()
            .filter(|r| r.statute_id == statute_id)
            .cloned()
            .collect();

        Self::reconstruct_chronological(&filtered, format!("Statute {} Timeline", statute_id))
    }

    /// Reconstructs a timeline for a specific actor.
    pub fn reconstruct_actor(
        records: &[AuditRecord],
        actor_pattern: &str,
    ) -> AuditResult<Timeline> {
        let filtered: Vec<_> = records
            .iter()
            .filter(|r| Self::actor_matches(&r.actor, actor_pattern))
            .cloned()
            .collect();

        Self::reconstruct_chronological(&filtered, format!("Actor {} Timeline", actor_pattern))
    }

    /// Converts an audit record to a timeline event.
    fn record_to_event(record: &AuditRecord) -> TimelineEvent {
        let actor_str = Self::format_actor(&record.actor);
        let event_type = format!("{:?}", record.event_type);

        let description = match &record.result {
            crate::DecisionResult::Deterministic { effect_applied, .. } => {
                format!("Decision: {}", effect_applied)
            }
            crate::DecisionResult::RequiresDiscretion { issue, .. } => {
                format!("Requires discretion: {}", issue)
            }
            crate::DecisionResult::Void { reason } => {
                format!("Void: {}", reason)
            }
            crate::DecisionResult::Overridden { justification, .. } => {
                format!("Overridden: {}", justification)
            }
        };

        TimelineEvent {
            timestamp: record.timestamp,
            record_id: record.id,
            event_type,
            actor: actor_str,
            statute_id: record.statute_id.clone(),
            subject_id: record.subject_id,
            description,
            related_events: Vec::new(),
        }
    }

    /// Formats actor for display.
    fn format_actor(actor: &Actor) -> String {
        match actor {
            Actor::System { component } => format!("System:{}", component),
            Actor::User { user_id, role } => format!("User:{}:{}", user_id, role),
            Actor::External { system_id } => format!("External:{}", system_id),
        }
    }

    /// Checks if an actor matches a pattern.
    fn actor_matches(actor: &Actor, pattern: &str) -> bool {
        let actor_str = Self::format_actor(actor);
        actor_str.to_lowercase().contains(&pattern.to_lowercase())
    }
}

/// Timeline comparison result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineComparison {
    /// Events only in first timeline
    pub only_in_first: Vec<TimelineEvent>,
    /// Events only in second timeline
    pub only_in_second: Vec<TimelineEvent>,
    /// Events in both timelines
    pub in_both: Vec<(TimelineEvent, TimelineEvent)>,
}

impl TimelineComparison {
    /// Compares two timelines.
    pub fn compare(timeline1: &Timeline, timeline2: &Timeline) -> Self {
        let mut only_in_first = Vec::new();
        let mut only_in_second = Vec::new();
        let mut in_both = Vec::new();

        let ids1: HashMap<Uuid, &TimelineEvent> =
            timeline1.events.iter().map(|e| (e.record_id, e)).collect();
        let ids2: HashMap<Uuid, &TimelineEvent> =
            timeline2.events.iter().map(|e| (e.record_id, e)).collect();

        for event in &timeline1.events {
            if let Some(event2) = ids2.get(&event.record_id) {
                in_both.push((event.clone(), (*event2).clone()));
            } else {
                only_in_first.push(event.clone());
            }
        }

        for event in &timeline2.events {
            if !ids1.contains_key(&event.record_id) {
                only_in_second.push(event.clone());
            }
        }

        Self {
            only_in_first,
            only_in_second,
            in_both,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;

    fn create_test_record(statute: &str, component: &str) -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: component.to_string(),
            },
            statute.to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: StdHashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_timeline_reconstruction() {
        let records = vec![
            create_test_record("statute-1", "engine"),
            create_test_record("statute-2", "engine"),
        ];

        let timeline =
            TimelineReconstructor::reconstruct_chronological(&records, "Test Timeline".to_string())
                .unwrap();

        assert_eq!(timeline.events.len(), 2);
        assert_eq!(timeline.title, "Test Timeline");
        assert!(timeline.start_time.is_some());
        assert!(timeline.end_time.is_some());
    }

    #[test]
    fn test_timeline_filtering() {
        let subject_id = Uuid::new_v4();
        let mut record = create_test_record("statute-1", "engine");
        record.subject_id = subject_id;

        let records = vec![record, create_test_record("statute-2", "engine")];

        let timeline = TimelineReconstructor::reconstruct_subject(&records, subject_id).unwrap();

        assert_eq!(timeline.events.len(), 1);
        assert_eq!(timeline.events[0].subject_id, subject_id);
    }

    #[test]
    fn test_timeline_html_export() {
        let records = vec![create_test_record("statute-1", "engine")];
        let timeline =
            TimelineReconstructor::reconstruct_chronological(&records, "Test".to_string()).unwrap();

        let html = timeline.to_html();
        assert!(html.contains("<html>"));
        assert!(html.contains("Test"));
    }

    #[test]
    fn test_timeline_json_export() {
        let records = vec![create_test_record("statute-1", "engine")];
        let timeline =
            TimelineReconstructor::reconstruct_chronological(&records, "Test".to_string()).unwrap();

        let json = timeline.to_json().unwrap();
        assert!(json.contains("Test"));
    }

    #[test]
    fn test_timeline_comparison() {
        let records1 = vec![create_test_record("statute-1", "engine")];
        let records2 = vec![create_test_record("statute-2", "engine")];

        let timeline1 =
            TimelineReconstructor::reconstruct_chronological(&records1, "T1".to_string()).unwrap();
        let timeline2 =
            TimelineReconstructor::reconstruct_chronological(&records2, "T2".to_string()).unwrap();

        let comparison = TimelineComparison::compare(&timeline1, &timeline2);

        assert_eq!(comparison.only_in_first.len(), 1);
        assert_eq!(comparison.only_in_second.len(), 1);
        assert_eq!(comparison.in_both.len(), 0);
    }
}
