//! Incident response automation for audit trail events.
//!
//! This module provides comprehensive incident response capabilities including
//! automated remediation, escalation workflows, playbooks, and incident correlation.

use crate::{AuditError, AuditResult};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Incident severity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Incident status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IncidentStatus {
    New,
    InProgress,
    Investigating,
    Contained,
    Resolved,
    Closed,
}

/// Incident category.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IncidentCategory {
    IntegrityViolation,
    UnauthorizedAccess,
    DataBreach,
    ComplianceViolation,
    AnomalousActivity,
    SystemFailure,
    Custom(String),
}

/// Incident record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub id: Uuid,
    pub severity: IncidentSeverity,
    pub status: IncidentStatus,
    pub category: IncidentCategory,
    pub title: String,
    pub description: String,
    pub triggered_at: DateTime<Utc>,
    pub related_records: Vec<Uuid>,
    pub assigned_to: Option<String>,
    pub actions_taken: Vec<ResponseAction>,
    pub metadata: HashMap<String, String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_notes: Option<String>,
}

/// Response action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseAction {
    Alert {
        channel: String,
        sent_at: DateTime<Utc>,
    },
    Quarantine {
        resource_id: String,
        quarantined_at: DateTime<Utc>,
    },
    Escalate {
        escalated_to: String,
        escalated_at: DateTime<Utc>,
    },
    AutoRemediate {
        action: String,
        performed_at: DateTime<Utc>,
    },
    NotifyStakeholders {
        stakeholders: Vec<String>,
        notified_at: DateTime<Utc>,
    },
    CollectEvidence {
        evidence_id: String,
        collected_at: DateTime<Utc>,
    },
    BlockUser {
        user_id: String,
        blocked_at: DateTime<Utc>,
    },
    Custom {
        action: String,
        performed_at: DateTime<Utc>,
    },
}

/// Incident response playbook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playbook {
    pub id: String,
    pub name: String,
    pub description: String,
    pub triggers: Vec<PlaybookTrigger>,
    pub steps: Vec<PlaybookStep>,
    pub auto_execute: bool,
}

/// Playbook trigger condition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaybookTrigger {
    Severity(IncidentSeverity),
    Category(IncidentCategory),
    KeywordInTitle(String),
    RecordCountThreshold(usize),
}

/// Playbook step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookStep {
    pub order: usize,
    pub action_type: PlaybookActionType,
    pub description: String,
    pub required: bool,
}

/// Playbook action type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaybookActionType {
    Alert,
    Contain,
    Investigate,
    Remediate,
    Escalate,
    Document,
    Notify,
}

/// Escalation policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationPolicy {
    pub id: String,
    pub name: String,
    pub severity_threshold: IncidentSeverity,
    pub time_threshold_minutes: i64,
    pub escalation_chain: Vec<String>,
}

/// Incident responder.
pub struct IncidentResponder {
    incidents: Vec<Incident>,
    playbooks: Vec<Playbook>,
    escalation_policies: Vec<EscalationPolicy>,
}

impl IncidentResponder {
    /// Creates a new incident responder.
    pub fn new() -> Self {
        Self {
            incidents: Vec::new(),
            playbooks: Vec::new(),
            escalation_policies: Vec::new(),
        }
    }

    /// Creates an incident.
    pub fn create_incident(
        &mut self,
        severity: IncidentSeverity,
        category: IncidentCategory,
        title: String,
        description: String,
    ) -> Uuid {
        let incident = Incident {
            id: Uuid::new_v4(),
            severity,
            status: IncidentStatus::New,
            category,
            title,
            description,
            triggered_at: Utc::now(),
            related_records: Vec::new(),
            assigned_to: None,
            actions_taken: Vec::new(),
            metadata: HashMap::new(),
            resolved_at: None,
            resolution_notes: None,
        };
        let id = incident.id;
        self.incidents.push(incident);

        // Try to auto-execute playbooks
        self.execute_playbooks(id);

        id
    }

    /// Updates incident status.
    pub fn update_status(&mut self, id: Uuid, status: IncidentStatus) -> AuditResult<()> {
        if let Some(incident) = self.incidents.iter_mut().find(|i| i.id == id) {
            incident.status = status;
            Ok(())
        } else {
            Err(AuditError::RecordNotFound(id))
        }
    }

    /// Assigns an incident to a user.
    pub fn assign_incident(&mut self, id: Uuid, assignee: String) -> AuditResult<()> {
        if let Some(incident) = self.incidents.iter_mut().find(|i| i.id == id) {
            incident.assigned_to = Some(assignee);
            incident.status = IncidentStatus::InProgress;
            Ok(())
        } else {
            Err(AuditError::RecordNotFound(id))
        }
    }

    /// Records an action taken on an incident.
    pub fn record_action(&mut self, id: Uuid, action: ResponseAction) -> AuditResult<()> {
        if let Some(incident) = self.incidents.iter_mut().find(|i| i.id == id) {
            incident.actions_taken.push(action);
            Ok(())
        } else {
            Err(AuditError::RecordNotFound(id))
        }
    }

    /// Resolves an incident.
    pub fn resolve_incident(&mut self, id: Uuid, resolution_notes: String) -> AuditResult<()> {
        if let Some(incident) = self.incidents.iter_mut().find(|i| i.id == id) {
            incident.status = IncidentStatus::Resolved;
            incident.resolved_at = Some(Utc::now());
            incident.resolution_notes = Some(resolution_notes);
            Ok(())
        } else {
            Err(AuditError::RecordNotFound(id))
        }
    }

    /// Closes an incident.
    pub fn close_incident(&mut self, id: Uuid) -> AuditResult<()> {
        if let Some(incident) = self.incidents.iter_mut().find(|i| i.id == id) {
            incident.status = IncidentStatus::Closed;
            Ok(())
        } else {
            Err(AuditError::RecordNotFound(id))
        }
    }

    /// Gets an incident by ID.
    pub fn get_incident(&self, id: Uuid) -> Option<&Incident> {
        self.incidents.iter().find(|i| i.id == id)
    }

    /// Gets all incidents with a specific status.
    pub fn get_incidents_by_status(&self, status: IncidentStatus) -> Vec<&Incident> {
        self.incidents
            .iter()
            .filter(|i| i.status == status)
            .collect()
    }

    /// Gets all open incidents.
    pub fn get_open_incidents(&self) -> Vec<&Incident> {
        self.incidents
            .iter()
            .filter(|i| !matches!(i.status, IncidentStatus::Resolved | IncidentStatus::Closed))
            .collect()
    }

    /// Gets high-severity incidents.
    pub fn get_critical_incidents(&self) -> Vec<&Incident> {
        self.incidents
            .iter()
            .filter(|i| {
                matches!(
                    i.severity,
                    IncidentSeverity::Critical | IncidentSeverity::High
                )
            })
            .collect()
    }

    /// Adds a response playbook.
    pub fn add_playbook(&mut self, playbook: Playbook) {
        self.playbooks.push(playbook);
    }

    /// Removes a playbook.
    pub fn remove_playbook(&mut self, playbook_id: &str) {
        self.playbooks.retain(|p| p.id != playbook_id);
    }

    /// Adds an escalation policy.
    pub fn add_escalation_policy(&mut self, policy: EscalationPolicy) {
        self.escalation_policies.push(policy);
    }

    /// Executes applicable playbooks for an incident.
    fn execute_playbooks(&mut self, incident_id: Uuid) {
        let incident = match self.incidents.iter().find(|i| i.id == incident_id) {
            Some(inc) => inc.clone(),
            None => return,
        };

        let applicable_playbooks: Vec<_> = self
            .playbooks
            .iter()
            .filter(|p| p.auto_execute && self.playbook_matches(&incident, p))
            .cloned()
            .collect();

        for playbook in applicable_playbooks {
            for step in playbook.steps.iter() {
                let action = match step.action_type {
                    PlaybookActionType::Alert => ResponseAction::Alert {
                        channel: "default".to_string(),
                        sent_at: Utc::now(),
                    },
                    PlaybookActionType::Contain => ResponseAction::Quarantine {
                        resource_id: incident_id.to_string(),
                        quarantined_at: Utc::now(),
                    },
                    PlaybookActionType::Escalate => ResponseAction::Escalate {
                        escalated_to: "security-team".to_string(),
                        escalated_at: Utc::now(),
                    },
                    _ => ResponseAction::Custom {
                        action: step.description.clone(),
                        performed_at: Utc::now(),
                    },
                };

                let _ = self.record_action(incident_id, action);
            }
        }
    }

    /// Checks if a playbook matches an incident.
    fn playbook_matches(&self, incident: &Incident, playbook: &Playbook) -> bool {
        playbook.triggers.iter().any(|trigger| match trigger {
            PlaybookTrigger::Severity(sev) => incident.severity == *sev,
            PlaybookTrigger::Category(cat) => incident.category == *cat,
            PlaybookTrigger::KeywordInTitle(keyword) => incident.title.contains(keyword),
            PlaybookTrigger::RecordCountThreshold(threshold) => {
                incident.related_records.len() >= *threshold
            }
        })
    }

    /// Checks and executes escalations for stale incidents.
    pub fn check_escalations(&mut self) -> Vec<Uuid> {
        let mut escalated = Vec::new();

        for policy in self.escalation_policies.clone() {
            let threshold = Utc::now() - Duration::minutes(policy.time_threshold_minutes);

            for incident in self.incidents.iter_mut() {
                if incident.severity >= policy.severity_threshold
                    && incident.triggered_at < threshold
                    && !matches!(
                        incident.status,
                        IncidentStatus::Resolved | IncidentStatus::Closed
                    )
                {
                    // Escalate
                    if let Some(next_level) = policy.escalation_chain.first() {
                        incident.actions_taken.push(ResponseAction::Escalate {
                            escalated_to: next_level.clone(),
                            escalated_at: Utc::now(),
                        });
                        escalated.push(incident.id);
                    }
                }
            }
        }

        escalated
    }

    /// Correlates similar incidents.
    pub fn correlate_incidents(&self, incident_id: Uuid) -> Vec<&Incident> {
        let incident = match self.get_incident(incident_id) {
            Some(inc) => inc,
            None => return Vec::new(),
        };

        self.incidents
            .iter()
            .filter(|i| {
                i.id != incident_id
                    && i.category == incident.category
                    && (Utc::now() - i.triggered_at).num_hours() < 24
            })
            .collect()
    }

    /// Gets incident statistics.
    pub fn get_statistics(&self) -> IncidentStatistics {
        let total = self.incidents.len();
        let open = self.get_open_incidents().len();
        let critical = self
            .incidents
            .iter()
            .filter(|i| i.severity == IncidentSeverity::Critical)
            .count();

        let avg_resolution_time = self.calculate_avg_resolution_time();

        let by_category = self.group_by_category();

        IncidentStatistics {
            total_incidents: total,
            open_incidents: open,
            critical_incidents: critical,
            avg_resolution_time_minutes: avg_resolution_time,
            incidents_by_category: by_category,
        }
    }

    /// Calculates average resolution time.
    fn calculate_avg_resolution_time(&self) -> f64 {
        let resolved: Vec<_> = self
            .incidents
            .iter()
            .filter(|i| i.resolved_at.is_some())
            .collect();

        if resolved.is_empty() {
            return 0.0;
        }

        let total_minutes: i64 = resolved
            .iter()
            .map(|i| (i.resolved_at.unwrap() - i.triggered_at).num_minutes())
            .sum();

        total_minutes as f64 / resolved.len() as f64
    }

    /// Groups incidents by category.
    fn group_by_category(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for incident in &self.incidents {
            let category_name = format!("{:?}", incident.category);
            *counts.entry(category_name).or_insert(0) += 1;
        }
        counts
    }

    /// Returns all incidents.
    pub fn get_all_incidents(&self) -> &[Incident] {
        &self.incidents
    }
}

impl Default for IncidentResponder {
    fn default() -> Self {
        Self::new()
    }
}

/// Incident statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentStatistics {
    pub total_incidents: usize,
    pub open_incidents: usize,
    pub critical_incidents: usize,
    pub avg_resolution_time_minutes: f64,
    pub incidents_by_category: HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incident_responder() {
        let mut responder = IncidentResponder::new();
        let id = responder.create_incident(
            IncidentSeverity::High,
            IncidentCategory::IntegrityViolation,
            "Test".to_string(),
            "Description".to_string(),
        );

        assert_eq!(responder.get_open_incidents().len(), 1);
        responder.resolve_incident(id, "Fixed".to_string()).unwrap();
        assert_eq!(responder.get_open_incidents().len(), 0);
    }

    #[test]
    fn test_incident_assignment() {
        let mut responder = IncidentResponder::new();
        let id = responder.create_incident(
            IncidentSeverity::Medium,
            IncidentCategory::AnomalousActivity,
            "Test".to_string(),
            "Description".to_string(),
        );

        responder.assign_incident(id, "user-1".to_string()).unwrap();

        let incident = responder.get_incident(id).unwrap();
        assert_eq!(incident.assigned_to, Some("user-1".to_string()));
        assert_eq!(incident.status, IncidentStatus::InProgress);
    }

    #[test]
    fn test_incident_actions() {
        let mut responder = IncidentResponder::new();
        let id = responder.create_incident(
            IncidentSeverity::Critical,
            IncidentCategory::DataBreach,
            "Test".to_string(),
            "Description".to_string(),
        );

        let action = ResponseAction::Alert {
            channel: "slack".to_string(),
            sent_at: Utc::now(),
        };

        responder.record_action(id, action).unwrap();

        let incident = responder.get_incident(id).unwrap();
        assert!(!incident.actions_taken.is_empty());
    }

    #[test]
    fn test_playbook_execution() {
        let mut responder = IncidentResponder::new();

        let playbook = Playbook {
            id: "test-playbook".to_string(),
            name: "Test Playbook".to_string(),
            description: "Test".to_string(),
            triggers: vec![PlaybookTrigger::Severity(IncidentSeverity::Critical)],
            steps: vec![PlaybookStep {
                order: 1,
                action_type: PlaybookActionType::Alert,
                description: "Send alert".to_string(),
                required: true,
            }],
            auto_execute: true,
        };

        responder.add_playbook(playbook);

        let id = responder.create_incident(
            IncidentSeverity::Critical,
            IncidentCategory::SystemFailure,
            "Critical failure".to_string(),
            "System down".to_string(),
        );

        let incident = responder.get_incident(id).unwrap();
        assert!(!incident.actions_taken.is_empty());
    }

    #[test]
    fn test_incident_correlation() {
        let mut responder = IncidentResponder::new();

        let id1 = responder.create_incident(
            IncidentSeverity::Medium,
            IncidentCategory::AnomalousActivity,
            "Test 1".to_string(),
            "Description 1".to_string(),
        );

        let _id2 = responder.create_incident(
            IncidentSeverity::Low,
            IncidentCategory::AnomalousActivity,
            "Test 2".to_string(),
            "Description 2".to_string(),
        );

        let correlated = responder.correlate_incidents(id1);
        assert_eq!(correlated.len(), 1);
    }

    #[test]
    fn test_incident_statistics() {
        let mut responder = IncidentResponder::new();

        responder.create_incident(
            IncidentSeverity::Critical,
            IncidentCategory::DataBreach,
            "Critical".to_string(),
            "Test".to_string(),
        );

        responder.create_incident(
            IncidentSeverity::Low,
            IncidentCategory::AnomalousActivity,
            "Low".to_string(),
            "Test".to_string(),
        );

        let stats = responder.get_statistics();
        assert_eq!(stats.total_incidents, 2);
        assert_eq!(stats.open_incidents, 2);
        assert_eq!(stats.critical_incidents, 1);
    }

    #[test]
    fn test_status_updates() {
        let mut responder = IncidentResponder::new();

        let id = responder.create_incident(
            IncidentSeverity::High,
            IncidentCategory::ComplianceViolation,
            "Test".to_string(),
            "Test".to_string(),
        );

        responder
            .update_status(id, IncidentStatus::Investigating)
            .unwrap();
        let incident = responder.get_incident(id).unwrap();
        assert_eq!(incident.status, IncidentStatus::Investigating);

        responder
            .resolve_incident(id, "Resolved".to_string())
            .unwrap();
        assert!(responder.get_incident(id).unwrap().resolved_at.is_some());
    }
}
