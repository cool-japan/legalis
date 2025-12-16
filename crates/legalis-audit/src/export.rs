//! Export functionality for audit trails.

use crate::{AuditRecord, AuditResult};
use serde_json::{Value, json};
use std::io::Write;

/// Exports audit records to CSV format.
pub fn to_csv<W: Write>(records: &[AuditRecord], writer: &mut W) -> AuditResult<()> {
    // Write header
    writeln!(
        writer,
        "id,timestamp,event_type,actor_type,statute_id,subject_id,result_type,record_hash"
    )?;

    // Write records
    for record in records {
        let event_type = format!("{:?}", record.event_type);
        let actor_type = match &record.actor {
            crate::Actor::System { component } => format!("System({})", component),
            crate::Actor::User { user_id, role } => format!("User({}, {})", user_id, role),
            crate::Actor::External { system_id } => format!("External({})", system_id),
        };
        let result_type = match &record.result {
            crate::DecisionResult::Deterministic { .. } => "Deterministic",
            crate::DecisionResult::RequiresDiscretion { .. } => "RequiresDiscretion",
            crate::DecisionResult::Void { .. } => "Void",
            crate::DecisionResult::Overridden { .. } => "Overridden",
        };

        writeln!(
            writer,
            "{},{},{},{},{},{},{},{}",
            record.id,
            record.timestamp.to_rfc3339(),
            event_type,
            actor_type,
            record.statute_id,
            record.subject_id,
            result_type,
            record.record_hash
        )?;
    }

    Ok(())
}

/// Exports audit records to JSON-LD format.
pub fn to_jsonld(records: &[AuditRecord]) -> AuditResult<Value> {
    let context = json!({
        "@vocab": "http://schema.org/",
        "audit": "http://legalis.example.org/audit#",
        "id": "@id",
        "type": "@type",
        "AuditRecord": "audit:AuditRecord",
        "DecisionEvent": "audit:DecisionEvent",
        "timestamp": {
            "@id": "audit:timestamp",
            "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
        },
        "actor": "audit:actor",
        "statute": "audit:statute",
        "subject": "audit:subject",
        "result": "audit:result",
        "previousHash": "audit:previousHash",
        "recordHash": "audit:recordHash"
    });

    let graph: Vec<Value> = records
        .iter()
        .map(|record| {
            let actor = match &record.actor {
                crate::Actor::System { component } => json!({
                    "@type": "audit:SystemActor",
                    "component": component
                }),
                crate::Actor::User { user_id, role } => json!({
                    "@type": "audit:UserActor",
                    "userId": user_id,
                    "role": role
                }),
                crate::Actor::External { system_id } => json!({
                    "@type": "audit:ExternalActor",
                    "systemId": system_id
                }),
            };

            let result = match &record.result {
                crate::DecisionResult::Deterministic {
                    effect_applied,
                    parameters,
                } => json!({
                    "@type": "audit:DeterministicResult",
                    "effectApplied": effect_applied,
                    "parameters": parameters
                }),
                crate::DecisionResult::RequiresDiscretion {
                    issue,
                    narrative_hint,
                    assigned_to,
                } => json!({
                    "@type": "audit:DiscretionaryResult",
                    "issue": issue,
                    "narrativeHint": narrative_hint,
                    "assignedTo": assigned_to
                }),
                crate::DecisionResult::Void { reason } => json!({
                    "@type": "audit:VoidResult",
                    "reason": reason
                }),
                crate::DecisionResult::Overridden {
                    original_result: _,
                    new_result: _,
                    justification,
                } => json!({
                    "@type": "audit:OverriddenResult",
                    "justification": justification
                }),
            };

            json!({
                "@type": "AuditRecord",
                "@id": format!("urn:uuid:{}", record.id),
                "timestamp": record.timestamp.to_rfc3339(),
                "eventType": format!("{:?}", record.event_type),
                "actor": actor,
                "statute": record.statute_id,
                "subject": format!("urn:uuid:{}", record.subject_id),
                "result": result,
                "previousHash": record.previous_hash,
                "recordHash": record.record_hash
            })
        })
        .collect();

    Ok(json!({
        "@context": context,
        "@graph": graph
    }))
}

/// Exports audit records to JSON format.
pub fn to_json(records: &[AuditRecord]) -> AuditResult<Value> {
    Ok(serde_json::to_value(records)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_csv_export() {
        let records = vec![AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "test-statute".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )];

        let mut output = Vec::new();
        to_csv(&records, &mut output).unwrap();
        let csv = String::from_utf8(output).unwrap();

        assert!(csv.contains("id,timestamp"));
        assert!(csv.contains("test-statute"));
    }

    #[test]
    fn test_jsonld_export() {
        let records = vec![AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "test-statute".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )];

        let jsonld = to_jsonld(&records).unwrap();
        assert!(jsonld.get("@context").is_some());
        assert!(jsonld.get("@graph").is_some());
    }

    #[test]
    fn test_json_export() {
        let records = vec![AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "test-statute".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )];

        let json = to_json(&records).unwrap();
        assert!(json.is_array());
    }
}
