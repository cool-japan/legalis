//! Incident response automation for audit trail events.

use crate::AuditResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Incident severity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Incident record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub id: Uuid,
    pub severity: IncidentSeverity,
    pub title: String,
    pub description: String,
    pub triggered_at: DateTime<Utc>,
    pub related_records: Vec<Uuid>,
    pub resolved: bool,
}

/// Response action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseAction {
    Alert,
    Quarantine,
    Escalate,
    AutoRemediate,
}

/// Incident responder.
pub struct IncidentResponder {
    incidents: Vec<Incident>,
}

impl IncidentResponder {
    pub fn new() -> Self {
        Self {
            incidents: Vec::new(),
        }
    }

    pub fn create_incident(
        &mut self,
        severity: IncidentSeverity,
        title: String,
        description: String,
    ) -> Uuid {
        let incident = Incident {
            id: Uuid::new_v4(),
            severity,
            title,
            description,
            triggered_at: Utc::now(),
            related_records: Vec::new(),
            resolved: false,
        };
        let id = incident.id;
        self.incidents.push(incident);
        id
    }

    pub fn resolve_incident(&mut self, id: Uuid) -> AuditResult<()> {
        if let Some(incident) = self.incidents.iter_mut().find(|i| i.id == id) {
            incident.resolved = true;
            Ok(())
        } else {
            Err(crate::AuditError::RecordNotFound(id))
        }
    }

    pub fn get_open_incidents(&self) -> Vec<&Incident> {
        self.incidents.iter().filter(|i| !i.resolved).collect()
    }
}

impl Default for IncidentResponder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incident_responder() {
        let mut responder = IncidentResponder::new();
        let id = responder.create_incident(
            IncidentSeverity::High,
            "Test".to_string(),
            "Description".to_string(),
        );

        assert_eq!(responder.get_open_incidents().len(), 1);
        responder.resolve_incident(id).unwrap();
        assert_eq!(responder.get_open_incidents().len(), 0);
    }
}
