//! Compliance reporting from knowledge graphs.
//!
//! This module provides tools for generating compliance reports:
//! - GDPR compliance reporting
//! - Data lineage tracking
//! - Retention policy compliance
//! - Regulatory reporting

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Compliance framework type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComplianceFramework {
    /// General Data Protection Regulation
    GDPR,
    /// California Consumer Privacy Act
    CCPA,
    /// Health Insurance Portability and Accountability Act
    HIPAA,
    /// Sarbanes-Oxley Act
    SOX,
    /// ISO 27001
    ISO27001,
    /// Custom framework
    Custom,
}

/// Data classification level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataClassification {
    /// Public data
    Public,
    /// Internal use
    Internal,
    /// Confidential
    Confidential,
    /// Personal Identifiable Information
    PII,
    /// Sensitive Personal Data
    SensitivePersonalData,
}

/// Data retention policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Policy ID
    pub id: String,
    /// Policy name
    pub name: String,
    /// Data classification this applies to
    pub classification: DataClassification,
    /// Retention period in days
    pub retention_days: usize,
    /// Auto-delete after retention period
    pub auto_delete: bool,
    /// Legal hold exemption
    pub legal_hold_exempt: bool,
}

impl RetentionPolicy {
    /// Creates a new retention policy.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        classification: DataClassification,
        retention_days: usize,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            classification,
            retention_days,
            auto_delete: false,
            legal_hold_exempt: false,
        }
    }

    /// Checks if data should be deleted based on creation date.
    pub fn should_delete(&self, created_at: DateTime<Utc>) -> bool {
        if !self.auto_delete || !self.legal_hold_exempt {
            return false;
        }

        let now = Utc::now();
        let age = now.signed_duration_since(created_at);
        age > Duration::days(self.retention_days as i64)
    }
}

/// GDPR subject rights request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectRightsRequest {
    /// Request ID
    pub id: String,
    /// Subject identifier (e.g., email, ID)
    pub subject_id: String,
    /// Request type
    pub request_type: SubjectRightType,
    /// Request timestamp
    pub requested_at: DateTime<Utc>,
    /// Status
    pub status: RequestStatus,
    /// Response deadline
    pub deadline: DateTime<Utc>,
    /// Completed timestamp
    pub completed_at: Option<DateTime<Utc>>,
}

/// GDPR subject right types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubjectRightType {
    /// Right to access
    Access,
    /// Right to rectification
    Rectification,
    /// Right to erasure ("right to be forgotten")
    Erasure,
    /// Right to restrict processing
    RestrictionOfProcessing,
    /// Right to data portability
    DataPortability,
    /// Right to object
    Object,
}

/// Request status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestStatus {
    /// Submitted
    Pending,
    /// In progress
    InProgress,
    /// Completed
    Completed,
    /// Rejected
    Rejected,
}

impl SubjectRightsRequest {
    /// Creates a new subject rights request.
    pub fn new(subject_id: impl Into<String>, request_type: SubjectRightType) -> Self {
        let requested_at = Utc::now();
        // GDPR requires response within 30 days
        let deadline = requested_at + Duration::days(30);

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            subject_id: subject_id.into(),
            request_type,
            requested_at,
            status: RequestStatus::Pending,
            deadline,
            completed_at: None,
        }
    }

    /// Marks the request as completed.
    pub fn complete(&mut self) {
        self.status = RequestStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    /// Checks if the request is overdue.
    pub fn is_overdue(&self) -> bool {
        self.status != RequestStatus::Completed && Utc::now() > self.deadline
    }
}

/// Compliance violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    /// Violation ID
    pub id: String,
    /// Framework
    pub framework: ComplianceFramework,
    /// Severity
    pub severity: ViolationSeverity,
    /// Description
    pub description: String,
    /// Affected resources
    pub affected_resources: Vec<String>,
    /// Detected timestamp
    pub detected_at: DateTime<Utc>,
    /// Remediated
    pub remediated: bool,
}

/// Violation severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

impl ComplianceViolation {
    /// Creates a new violation.
    pub fn new(
        framework: ComplianceFramework,
        severity: ViolationSeverity,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            framework,
            severity,
            description: description.into(),
            affected_resources: Vec::new(),
            detected_at: Utc::now(),
            remediated: false,
        }
    }

    /// Adds an affected resource.
    pub fn add_affected_resource(mut self, resource: impl Into<String>) -> Self {
        self.affected_resources.push(resource.into());
        self
    }
}

/// Compliance report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Report ID
    pub id: String,
    /// Framework
    pub framework: ComplianceFramework,
    /// Reporting period start
    pub period_start: DateTime<Utc>,
    /// Reporting period end
    pub period_end: DateTime<Utc>,
    /// Total triples audited
    pub total_triples: usize,
    /// Compliant triples
    pub compliant_triples: usize,
    /// Violations found
    pub violations: Vec<ComplianceViolation>,
    /// Subject rights requests processed
    pub rights_requests_processed: usize,
    /// Generated timestamp
    pub generated_at: DateTime<Utc>,
}

impl ComplianceReport {
    /// Creates a new compliance report.
    pub fn new(
        framework: ComplianceFramework,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            framework,
            period_start,
            period_end,
            total_triples: 0,
            compliant_triples: 0,
            violations: Vec::new(),
            rights_requests_processed: 0,
            generated_at: Utc::now(),
        }
    }

    /// Calculates compliance rate.
    pub fn compliance_rate(&self) -> f64 {
        if self.total_triples == 0 {
            100.0
        } else {
            (self.compliant_triples as f64 / self.total_triples as f64) * 100.0
        }
    }

    /// Gets violations by severity.
    pub fn violations_by_severity(&self, severity: ViolationSeverity) -> Vec<&ComplianceViolation> {
        self.violations
            .iter()
            .filter(|v| v.severity == severity)
            .collect()
    }

    /// Gets unresolved violations.
    pub fn unresolved_violations(&self) -> Vec<&ComplianceViolation> {
        self.violations.iter().filter(|v| !v.remediated).collect()
    }
}

/// Compliance manager.
pub struct ComplianceManager {
    /// Retention policies
    policies: HashMap<String, RetentionPolicy>,
    /// Subject rights requests
    requests: HashMap<String, SubjectRightsRequest>,
    /// Violations
    violations: Vec<ComplianceViolation>,
}

impl ComplianceManager {
    /// Creates a new compliance manager.
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
            requests: HashMap::new(),
            violations: Vec::new(),
        }
    }

    /// Adds a retention policy.
    pub fn add_policy(&mut self, policy: RetentionPolicy) {
        self.policies.insert(policy.id.clone(), policy);
    }

    /// Gets a policy.
    pub fn get_policy(&self, id: &str) -> Option<&RetentionPolicy> {
        self.policies.get(id)
    }

    /// Submits a subject rights request.
    pub fn submit_request(&mut self, request: SubjectRightsRequest) -> String {
        let id = request.id.clone();
        self.requests.insert(id.clone(), request);
        id
    }

    /// Gets a request.
    pub fn get_request(&self, id: &str) -> Option<&SubjectRightsRequest> {
        self.requests.get(id)
    }

    /// Gets a mutable request.
    pub fn get_request_mut(&mut self, id: &str) -> Option<&mut SubjectRightsRequest> {
        self.requests.get_mut(id)
    }

    /// Lists overdue requests.
    pub fn overdue_requests(&self) -> Vec<&SubjectRightsRequest> {
        self.requests.values().filter(|r| r.is_overdue()).collect()
    }

    /// Records a violation.
    pub fn record_violation(&mut self, violation: ComplianceViolation) {
        self.violations.push(violation);
    }

    /// Gets all violations.
    pub fn get_violations(&self) -> &[ComplianceViolation] {
        &self.violations
    }

    /// Generates a compliance report.
    pub fn generate_report(
        &self,
        framework: ComplianceFramework,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        total_triples: usize,
    ) -> ComplianceReport {
        let mut report = ComplianceReport::new(framework, period_start, period_end);
        report.total_triples = total_triples;

        // Count violations in period
        let period_violations: Vec<_> = self
            .violations
            .iter()
            .filter(|v| {
                v.framework == framework
                    && v.detected_at >= period_start
                    && v.detected_at <= period_end
            })
            .cloned()
            .collect();

        report.violations = period_violations;
        report.compliant_triples = total_triples.saturating_sub(report.violations.len());

        // Count completed requests
        report.rights_requests_processed = self
            .requests
            .values()
            .filter(|r| {
                r.status == RequestStatus::Completed
                    && r.completed_at
                        .map(|c| c >= period_start && c <= period_end)
                        .unwrap_or(false)
            })
            .count();

        report
    }
}

impl Default for ComplianceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retention_policy() {
        let policy = RetentionPolicy::new(
            "policy1",
            "30 Day Retention",
            DataClassification::Internal,
            30,
        );
        assert_eq!(policy.id, "policy1");
        assert_eq!(policy.retention_days, 30);

        let old_date = Utc::now() - Duration::days(60);
        assert!(!policy.should_delete(old_date)); // auto_delete is false

        let mut policy2 = policy.clone();
        policy2.auto_delete = true;
        policy2.legal_hold_exempt = true;
        assert!(policy2.should_delete(old_date));
    }

    #[test]
    fn test_subject_rights_request() {
        let mut request = SubjectRightsRequest::new("user@example.com", SubjectRightType::Access);
        assert_eq!(request.status, RequestStatus::Pending);

        request.complete();
        assert_eq!(request.status, RequestStatus::Completed);
        assert!(request.completed_at.is_some());
    }

    #[test]
    fn test_request_overdue() {
        let mut request = SubjectRightsRequest::new("user@example.com", SubjectRightType::Erasure);
        request.deadline = Utc::now() - Duration::days(1);

        assert!(request.is_overdue());

        request.complete();
        assert!(!request.is_overdue());
    }

    #[test]
    fn test_compliance_violation() {
        let violation = ComplianceViolation::new(
            ComplianceFramework::GDPR,
            ViolationSeverity::High,
            "Unauthorized access",
        )
        .add_affected_resource("http://example.org/data1");

        assert_eq!(violation.framework, ComplianceFramework::GDPR);
        assert_eq!(violation.severity, ViolationSeverity::High);
        assert_eq!(violation.affected_resources.len(), 1);
        assert!(!violation.remediated);
    }

    #[test]
    fn test_compliance_report() {
        let start = Utc::now() - Duration::days(30);
        let end = Utc::now();
        let mut report = ComplianceReport::new(ComplianceFramework::GDPR, start, end);

        report.total_triples = 1000;
        report.compliant_triples = 950;

        let rate = report.compliance_rate();
        assert!((rate - 95.0).abs() < 0.01);
    }

    #[test]
    fn test_compliance_manager() {
        let mut manager = ComplianceManager::new();

        let policy = RetentionPolicy::new("policy1", "Test Policy", DataClassification::PII, 90);
        manager.add_policy(policy);

        assert!(manager.get_policy("policy1").is_some());
    }

    #[test]
    fn test_submit_request() {
        let mut manager = ComplianceManager::new();

        let request = SubjectRightsRequest::new("user@example.com", SubjectRightType::Access);
        let request_id = manager.submit_request(request);

        assert!(manager.get_request(&request_id).is_some());
    }

    #[test]
    fn test_overdue_requests() {
        let mut manager = ComplianceManager::new();

        let mut request1 = SubjectRightsRequest::new("user1@example.com", SubjectRightType::Access);
        request1.deadline = Utc::now() - Duration::days(1);
        manager.submit_request(request1);

        let request2 = SubjectRightsRequest::new("user2@example.com", SubjectRightType::Erasure);
        manager.submit_request(request2);

        let overdue = manager.overdue_requests();
        assert_eq!(overdue.len(), 1);
    }

    #[test]
    fn test_generate_report() {
        let mut manager = ComplianceManager::new();

        let violation = ComplianceViolation::new(
            ComplianceFramework::GDPR,
            ViolationSeverity::Medium,
            "Test violation",
        );
        manager.record_violation(violation);

        let start = Utc::now() - Duration::days(30);
        let end = Utc::now();
        let report = manager.generate_report(ComplianceFramework::GDPR, start, end, 1000);

        assert_eq!(report.framework, ComplianceFramework::GDPR);
        assert_eq!(report.total_triples, 1000);
        assert_eq!(report.violations.len(), 1);
    }

    #[test]
    fn test_violations_by_severity() {
        let start = Utc::now() - Duration::days(30);
        let end = Utc::now();
        let mut report = ComplianceReport::new(ComplianceFramework::GDPR, start, end);

        report.violations.push(ComplianceViolation::new(
            ComplianceFramework::GDPR,
            ViolationSeverity::High,
            "High severity",
        ));
        report.violations.push(ComplianceViolation::new(
            ComplianceFramework::GDPR,
            ViolationSeverity::Low,
            "Low severity",
        ));

        let high = report.violations_by_severity(ViolationSeverity::High);
        assert_eq!(high.len(), 1);
    }

    #[test]
    fn test_unresolved_violations() {
        let start = Utc::now() - Duration::days(30);
        let end = Utc::now();
        let mut report = ComplianceReport::new(ComplianceFramework::GDPR, start, end);

        let mut v1 = ComplianceViolation::new(
            ComplianceFramework::GDPR,
            ViolationSeverity::High,
            "Unresolved",
        );
        v1.remediated = false;

        let mut v2 = ComplianceViolation::new(
            ComplianceFramework::GDPR,
            ViolationSeverity::Medium,
            "Resolved",
        );
        v2.remediated = true;

        report.violations.push(v1);
        report.violations.push(v2);

        let unresolved = report.unresolved_violations();
        assert_eq!(unresolved.len(), 1);
    }
}
