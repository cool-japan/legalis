//! Compliance dashboard for regulatory monitoring.
//!
//! Provides real-time compliance monitoring, metrics visualization,
//! and status tracking across multiple regulatory frameworks.

use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::multi_regulation::{ComplianceStatus, RegulationType};

/// Compliance dashboard
pub struct ComplianceDashboard {
    metrics: DashboardMetrics,
    regulation_status: HashMap<RegulationType, RegulationStatus>,
    alerts: Vec<ComplianceAlert>,
}

impl ComplianceDashboard {
    /// Create a new compliance dashboard
    pub fn new() -> Self {
        Self {
            metrics: DashboardMetrics::default(),
            regulation_status: HashMap::new(),
            alerts: Vec::new(),
        }
    }

    /// Update dashboard metrics from audit records
    pub fn update_metrics(&mut self, records: &[AuditRecord]) -> AuditResult<()> {
        self.metrics.total_records = records.len();
        self.metrics.last_updated = Utc::now();

        // Calculate automatic vs manual decisions
        self.metrics.automatic_decisions = records
            .iter()
            .filter(|r| matches!(r.result, crate::DecisionResult::Deterministic { .. }))
            .count();

        self.metrics.manual_decisions = records
            .iter()
            .filter(|r| matches!(r.result, crate::DecisionResult::RequiresDiscretion { .. }))
            .count();

        self.metrics.overrides = records
            .iter()
            .filter(|r| matches!(r.result, crate::DecisionResult::Overridden { .. }))
            .count();

        // Calculate integrity metrics
        let verified_records = records.iter().filter(|r| r.verify()).count();
        self.metrics.integrity_pass_rate = (verified_records as f64 / records.len() as f64) * 100.0;

        // Time range
        if let Some(first) = records.first() {
            self.metrics.earliest_record = Some(first.timestamp);
        }
        if let Some(last) = records.last() {
            self.metrics.latest_record = Some(last.timestamp);
        }

        Ok(())
    }

    /// Update regulation status
    pub fn update_regulation_status(
        &mut self,
        regulation: RegulationType,
        status: RegulationStatus,
    ) {
        self.regulation_status.insert(regulation, status);
    }

    /// Add a compliance alert
    pub fn add_alert(&mut self, alert: ComplianceAlert) {
        self.alerts.push(alert);
    }

    /// Get current metrics
    pub fn metrics(&self) -> &DashboardMetrics {
        &self.metrics
    }

    /// Get regulation status
    pub fn get_regulation_status(&self, regulation: &RegulationType) -> Option<&RegulationStatus> {
        self.regulation_status.get(regulation)
    }

    /// Get all regulation statuses
    pub fn all_regulation_statuses(&self) -> Vec<(&RegulationType, &RegulationStatus)> {
        self.regulation_status.iter().collect()
    }

    /// Get active alerts
    pub fn get_active_alerts(&self) -> Vec<&ComplianceAlert> {
        self.alerts.iter().filter(|a| !a.resolved).collect()
    }

    /// Get all alerts
    pub fn get_all_alerts(&self) -> &[ComplianceAlert] {
        &self.alerts
    }

    /// Resolve an alert
    pub fn resolve_alert(&mut self, alert_id: &str) -> AuditResult<()> {
        let alert = self
            .alerts
            .iter_mut()
            .find(|a| a.id == alert_id)
            .ok_or_else(|| {
                crate::AuditError::InvalidRecord(format!("Alert not found: {}", alert_id))
            })?;

        alert.resolved = true;
        alert.resolved_at = Some(Utc::now());
        Ok(())
    }

    /// Export dashboard as JSON
    pub fn export_json(&self) -> AuditResult<String> {
        let dashboard_export = DashboardExport {
            metrics: self.metrics.clone(),
            regulation_status: self.regulation_status.clone(),
            active_alerts: self.get_active_alerts().len(),
            total_alerts: self.alerts.len(),
            exported_at: Utc::now(),
        };

        serde_json::to_string_pretty(&dashboard_export)
            .map_err(crate::AuditError::SerializationError)
    }

    /// Export dashboard as HTML
    pub fn export_html(&self) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Compliance Dashboard</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .metric {{ background: #f5f5f5; padding: 15px; margin: 10px 0; border-radius: 5px; }}
        .compliant {{ color: green; }}
        .non-compliant {{ color: red; }}
        .partial {{ color: orange; }}
        .alert {{ background: #fff3cd; padding: 10px; margin: 5px 0; border-left: 4px solid #ffc107; }}
    </style>
</head>
<body>
    <h1>Compliance Dashboard</h1>
    <div class="metric">
        <h2>Metrics</h2>
        <p>Total Records: {}</p>
        <p>Automatic Decisions: {}</p>
        <p>Manual Decisions: {}</p>
        <p>Overrides: {}</p>
        <p>Integrity Pass Rate: {:.2}%</p>
        <p>Last Updated: {}</p>
    </div>
    <div class="metric">
        <h2>Regulations</h2>
        {}
    </div>
    <div class="metric">
        <h2>Active Alerts ({})</h2>
        {}
    </div>
</body>
</html>"#,
            self.metrics.total_records,
            self.metrics.automatic_decisions,
            self.metrics.manual_decisions,
            self.metrics.overrides,
            self.metrics.integrity_pass_rate,
            self.metrics.last_updated.to_rfc3339(),
            self.render_regulations_html(),
            self.get_active_alerts().len(),
            self.render_alerts_html()
        )
    }

    fn render_regulations_html(&self) -> String {
        self.regulation_status
            .iter()
            .map(|(reg, status)| {
                let class = match status.status {
                    ComplianceStatus::Compliant => "compliant",
                    ComplianceStatus::PartiallyCompliant => "partial",
                    ComplianceStatus::NonCompliant | ComplianceStatus::Unknown => "non-compliant",
                };
                format!(
                    "<p class=\"{}\"><strong>{:?}</strong>: {:?} ({:.1}%)</p>",
                    class, reg, status.status, status.compliance_percentage
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn render_alerts_html(&self) -> String {
        let active_alerts = self.get_active_alerts();
        if active_alerts.is_empty() {
            "<p>No active alerts</p>".to_string()
        } else {
            active_alerts
                .iter()
                .map(|alert| {
                    format!(
                        "<div class=\"alert\"><strong>{:?}</strong>: {} ({})</div>",
                        alert.severity, alert.message, alert.regulation
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}

impl Default for ComplianceDashboard {
    fn default() -> Self {
        Self::new()
    }
}

/// Dashboard metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMetrics {
    pub total_records: usize,
    pub automatic_decisions: usize,
    pub manual_decisions: usize,
    pub overrides: usize,
    pub integrity_pass_rate: f64,
    pub earliest_record: Option<DateTime<Utc>>,
    pub latest_record: Option<DateTime<Utc>>,
    pub last_updated: DateTime<Utc>,
}

impl Default for DashboardMetrics {
    fn default() -> Self {
        Self {
            total_records: 0,
            automatic_decisions: 0,
            manual_decisions: 0,
            overrides: 0,
            integrity_pass_rate: 0.0,
            earliest_record: None,
            latest_record: None,
            last_updated: Utc::now(),
        }
    }
}

/// Regulation status on dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulationStatus {
    pub status: ComplianceStatus,
    pub compliance_percentage: f64,
    pub last_check: DateTime<Utc>,
    pub next_deadline: Option<DateTime<Utc>>,
}

/// Compliance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAlert {
    pub id: String,
    pub regulation: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub resolved: bool,
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Alert severity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Critical,
    Warning,
    Info,
}

/// Dashboard export
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DashboardExport {
    metrics: DashboardMetrics,
    regulation_status: HashMap<RegulationType, RegulationStatus>,
    active_alerts: usize,
    total_alerts: usize,
    exported_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use uuid::Uuid;

    fn create_test_records(count: usize) -> Vec<AuditRecord> {
        (0..count)
            .map(|_| {
                AuditRecord::new(
                    EventType::AutomaticDecision,
                    Actor::System {
                        component: "test".to_string(),
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
            })
            .collect()
    }

    #[test]
    fn test_dashboard_creation() {
        let dashboard = ComplianceDashboard::new();
        assert_eq!(dashboard.metrics.total_records, 0);
        assert!(dashboard.regulation_status.is_empty());
        assert!(dashboard.alerts.is_empty());
    }

    #[test]
    fn test_update_metrics() {
        let mut dashboard = ComplianceDashboard::new();
        let records = create_test_records(10);

        dashboard.update_metrics(&records).unwrap();

        assert_eq!(dashboard.metrics.total_records, 10);
        assert!(dashboard.metrics.integrity_pass_rate > 0.0);
    }

    #[test]
    fn test_update_regulation_status() {
        let mut dashboard = ComplianceDashboard::new();
        let status = RegulationStatus {
            status: ComplianceStatus::Compliant,
            compliance_percentage: 100.0,
            last_check: Utc::now(),
            next_deadline: None,
        };

        dashboard.update_regulation_status(RegulationType::GDPR, status);

        assert_eq!(dashboard.regulation_status.len(), 1);
        assert_eq!(
            dashboard
                .get_regulation_status(&RegulationType::GDPR)
                .unwrap()
                .status,
            ComplianceStatus::Compliant
        );
    }

    #[test]
    fn test_add_alert() {
        let mut dashboard = ComplianceDashboard::new();
        let alert = ComplianceAlert {
            id: "alert-1".to_string(),
            regulation: "GDPR".to_string(),
            severity: AlertSeverity::Warning,
            message: "Test alert".to_string(),
            created_at: Utc::now(),
            resolved: false,
            resolved_at: None,
        };

        dashboard.add_alert(alert);

        assert_eq!(dashboard.alerts.len(), 1);
        assert_eq!(dashboard.get_active_alerts().len(), 1);
    }

    #[test]
    fn test_resolve_alert() {
        let mut dashboard = ComplianceDashboard::new();
        let alert = ComplianceAlert {
            id: "alert-1".to_string(),
            regulation: "GDPR".to_string(),
            severity: AlertSeverity::Warning,
            message: "Test alert".to_string(),
            created_at: Utc::now(),
            resolved: false,
            resolved_at: None,
        };

        dashboard.add_alert(alert);
        dashboard.resolve_alert("alert-1").unwrap();

        assert_eq!(dashboard.get_active_alerts().len(), 0);
        assert_eq!(dashboard.alerts.len(), 1);
        assert!(dashboard.alerts[0].resolved);
    }

    #[test]
    fn test_export_json() {
        let mut dashboard = ComplianceDashboard::new();
        let records = create_test_records(5);
        dashboard.update_metrics(&records).unwrap();

        let json = dashboard.export_json().unwrap();
        assert!(json.contains("metrics"));
        assert!(json.contains("total_records"));
    }

    #[test]
    fn test_export_html() {
        let mut dashboard = ComplianceDashboard::new();
        let records = create_test_records(5);
        dashboard.update_metrics(&records).unwrap();

        let html = dashboard.export_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Compliance Dashboard"));
        assert!(html.contains("Total Records: 5"));
    }

    #[test]
    fn test_all_regulation_statuses() {
        let mut dashboard = ComplianceDashboard::new();

        dashboard.update_regulation_status(
            RegulationType::GDPR,
            RegulationStatus {
                status: ComplianceStatus::Compliant,
                compliance_percentage: 100.0,
                last_check: Utc::now(),
                next_deadline: None,
            },
        );

        dashboard.update_regulation_status(
            RegulationType::SOX,
            RegulationStatus {
                status: ComplianceStatus::PartiallyCompliant,
                compliance_percentage: 75.0,
                last_check: Utc::now(),
                next_deadline: None,
            },
        );

        let statuses = dashboard.all_regulation_statuses();
        assert_eq!(statuses.len(), 2);
    }
}
