//! OpenTelemetry tracing integration for audit operations.
//!
//! This module provides integration with OpenTelemetry for distributed tracing
//! of audit operations.

use crate::{AuditRecord, EventType};
use std::collections::HashMap;

/// OpenTelemetry span attributes for audit records.
pub struct AuditSpan {
    attributes: HashMap<String, String>,
}

impl AuditSpan {
    /// Creates span attributes from an audit record.
    pub fn from_record(record: &AuditRecord) -> Self {
        let mut attributes = HashMap::new();

        // Core attributes
        attributes.insert("audit.record.id".to_string(), record.id.to_string());
        attributes.insert("audit.timestamp".to_string(), record.timestamp.to_rfc3339());
        attributes.insert(
            "audit.event_type".to_string(),
            format!("{:?}", record.event_type),
        );
        attributes.insert("audit.statute_id".to_string(), record.statute_id.clone());
        attributes.insert(
            "audit.subject_id".to_string(),
            record.subject_id.to_string(),
        );

        // Actor attributes
        match &record.actor {
            crate::Actor::System { component } => {
                attributes.insert("audit.actor.type".to_string(), "system".to_string());
                attributes.insert("audit.actor.component".to_string(), component.clone());
            }
            crate::Actor::User { user_id, role } => {
                attributes.insert("audit.actor.type".to_string(), "user".to_string());
                attributes.insert("audit.actor.user_id".to_string(), user_id.clone());
                attributes.insert("audit.actor.role".to_string(), role.clone());
            }
            crate::Actor::External { system_id } => {
                attributes.insert("audit.actor.type".to_string(), "external".to_string());
                attributes.insert("audit.actor.system_id".to_string(), system_id.clone());
            }
        }

        // Result attributes
        match &record.result {
            crate::DecisionResult::Deterministic { effect_applied, .. } => {
                attributes.insert("audit.result.type".to_string(), "deterministic".to_string());
                attributes.insert("audit.result.effect".to_string(), effect_applied.clone());
            }
            crate::DecisionResult::RequiresDiscretion { issue, .. } => {
                attributes.insert(
                    "audit.result.type".to_string(),
                    "requires_discretion".to_string(),
                );
                attributes.insert("audit.result.issue".to_string(), issue.clone());
            }
            crate::DecisionResult::Void { reason } => {
                attributes.insert("audit.result.type".to_string(), "void".to_string());
                attributes.insert("audit.result.reason".to_string(), reason.clone());
            }
            crate::DecisionResult::Overridden { justification, .. } => {
                attributes.insert("audit.result.type".to_string(), "overridden".to_string());
                attributes.insert(
                    "audit.result.justification".to_string(),
                    justification.clone(),
                );
            }
        }

        // Hash chain attributes
        attributes.insert("audit.record_hash".to_string(), record.record_hash.clone());
        if let Some(ref prev_hash) = record.previous_hash {
            attributes.insert("audit.previous_hash".to_string(), prev_hash.clone());
        }

        Self { attributes }
    }

    /// Gets the attributes as a HashMap.
    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    /// Gets the span name for an event type.
    pub fn span_name(event_type: &EventType) -> &'static str {
        match event_type {
            EventType::AutomaticDecision => "audit.automatic_decision",
            EventType::DiscretionaryReview => "audit.discretionary_review",
            EventType::HumanOverride => "audit.human_override",
            EventType::Appeal => "audit.appeal",
            EventType::StatuteModified => "audit.statute_modified",
            EventType::SimulationRun => "audit.simulation_run",
        }
    }
}

/// Telemetry metrics for audit operations.
#[derive(Debug, Clone, Default)]
pub struct AuditMetrics {
    /// Total number of audit records created
    pub total_records: u64,
    /// Number of automatic decisions
    pub automatic_decisions: u64,
    /// Number of discretionary reviews
    pub discretionary_reviews: u64,
    /// Number of human overrides
    pub human_overrides: u64,
    /// Number of appeals
    pub appeals: u64,
    /// Number of integrity verification failures
    pub integrity_failures: u64,
    /// Number of integrity verification successes
    pub integrity_successes: u64,
}

impl AuditMetrics {
    /// Creates a new metrics instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a new audit event.
    pub fn record_event(&mut self, event_type: &EventType) {
        self.total_records += 1;
        match event_type {
            EventType::AutomaticDecision => self.automatic_decisions += 1,
            EventType::DiscretionaryReview => self.discretionary_reviews += 1,
            EventType::HumanOverride => self.human_overrides += 1,
            EventType::Appeal => self.appeals += 1,
            EventType::StatuteModified | EventType::SimulationRun => {}
        }
    }

    /// Records an integrity verification result.
    pub fn record_verification(&mut self, success: bool) {
        if success {
            self.integrity_successes += 1;
        } else {
            self.integrity_failures += 1;
        }
    }

    /// Gets metric names and values for OpenTelemetry.
    pub fn to_otel_metrics(&self) -> Vec<(&'static str, u64)> {
        vec![
            ("audit.records.total", self.total_records),
            ("audit.decisions.automatic", self.automatic_decisions),
            ("audit.decisions.discretionary", self.discretionary_reviews),
            ("audit.decisions.overrides", self.human_overrides),
            ("audit.appeals.total", self.appeals),
            ("audit.integrity.failures", self.integrity_failures),
            ("audit.integrity.successes", self.integrity_successes),
        ]
    }
}

/// Trace context for distributed tracing.
#[derive(Debug, Clone)]
pub struct TraceContext {
    /// Trace ID
    pub trace_id: String,
    /// Span ID
    pub span_id: String,
    /// Parent span ID (optional)
    pub parent_span_id: Option<String>,
    /// Trace flags
    pub trace_flags: u8,
}

impl TraceContext {
    /// Creates a new trace context.
    pub fn new(trace_id: String, span_id: String) -> Self {
        Self {
            trace_id,
            span_id,
            parent_span_id: None,
            trace_flags: 0,
        }
    }

    /// Sets the parent span ID.
    pub fn with_parent(mut self, parent_span_id: String) -> Self {
        self.parent_span_id = Some(parent_span_id);
        self
    }

    /// Encodes the trace context as W3C traceparent header.
    pub fn to_traceparent(&self) -> String {
        format!(
            "00-{}-{}-{:02x}",
            self.trace_id, self.span_id, self.trace_flags
        )
    }

    /// Parses a W3C traceparent header.
    pub fn from_traceparent(traceparent: &str) -> Option<Self> {
        let parts: Vec<&str> = traceparent.split('-').collect();
        if parts.len() != 4 || parts[0] != "00" {
            return None;
        }

        Some(Self {
            trace_id: parts[1].to_string(),
            span_id: parts[2].to_string(),
            parent_span_id: None,
            trace_flags: u8::from_str_radix(parts[3], 16).ok()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn create_test_record() -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test-engine".to_string(),
            },
            "statute-123".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_audit_span_from_record() {
        let record = create_test_record();
        let span = AuditSpan::from_record(&record);

        let attrs = span.attributes();
        assert!(attrs.contains_key("audit.record.id"));
        assert!(attrs.contains_key("audit.timestamp"));
        assert!(attrs.contains_key("audit.event_type"));
        assert!(attrs.contains_key("audit.statute_id"));
        assert!(attrs.contains_key("audit.subject_id"));
        assert_eq!(attrs.get("audit.actor.type"), Some(&"system".to_string()));
    }

    #[test]
    fn test_span_names() {
        assert_eq!(
            AuditSpan::span_name(&EventType::AutomaticDecision),
            "audit.automatic_decision"
        );
        assert_eq!(
            AuditSpan::span_name(&EventType::HumanOverride),
            "audit.human_override"
        );
    }

    #[test]
    fn test_audit_metrics() {
        let mut metrics = AuditMetrics::new();

        metrics.record_event(&EventType::AutomaticDecision);
        metrics.record_event(&EventType::AutomaticDecision);
        metrics.record_event(&EventType::HumanOverride);
        metrics.record_verification(true);
        metrics.record_verification(false);

        assert_eq!(metrics.total_records, 3);
        assert_eq!(metrics.automatic_decisions, 2);
        assert_eq!(metrics.human_overrides, 1);
        assert_eq!(metrics.integrity_successes, 1);
        assert_eq!(metrics.integrity_failures, 1);
    }

    #[test]
    fn test_otel_metrics() {
        let mut metrics = AuditMetrics::new();
        metrics.record_event(&EventType::AutomaticDecision);

        let otel_metrics = metrics.to_otel_metrics();
        assert!(
            otel_metrics
                .iter()
                .any(|(name, _)| *name == "audit.records.total")
        );
        assert!(
            otel_metrics
                .iter()
                .any(|(name, val)| *name == "audit.decisions.automatic" && *val == 1)
        );
    }

    #[test]
    fn test_trace_context_traceparent() {
        let ctx = TraceContext::new(
            "0af7651916cd43dd8448eb211c80319c".to_string(),
            "00f067aa0ba902b7".to_string(),
        );

        let traceparent = ctx.to_traceparent();
        assert_eq!(
            traceparent,
            "00-0af7651916cd43dd8448eb211c80319c-00f067aa0ba902b7-00"
        );
    }

    #[test]
    fn test_parse_traceparent() {
        let traceparent = "00-0af7651916cd43dd8448eb211c80319c-00f067aa0ba902b7-01";
        let ctx = TraceContext::from_traceparent(traceparent).unwrap();

        assert_eq!(ctx.trace_id, "0af7651916cd43dd8448eb211c80319c");
        assert_eq!(ctx.span_id, "00f067aa0ba902b7");
        assert_eq!(ctx.trace_flags, 1);
    }

    #[test]
    fn test_parse_invalid_traceparent() {
        assert!(TraceContext::from_traceparent("invalid").is_none());
        assert!(TraceContext::from_traceparent("01-abc-def-00").is_none());
    }
}
