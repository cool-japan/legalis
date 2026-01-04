//! Regulatory Automation for audit compliance.
//!
//! This module provides automated compliance reporting, regulatory submission APIs,
//! multi-regulation tracking, compliance dashboards, and deadline management.

pub mod automated_reporting;
pub mod compliance_dashboard;
pub mod deadline_manager;
pub mod multi_regulation;
pub mod submission_api;

pub use automated_reporting::{AutomatedReporter, ComplianceReportSchedule, ReportTemplate};
pub use compliance_dashboard::{ComplianceDashboard, DashboardMetrics, RegulationStatus};
pub use deadline_manager::{
    Deadline, DeadlineManager, DeadlineNotification, DeadlineStatus, ReminderConfig,
};
pub use multi_regulation::{MultiRegulationTracker, RegulationCompliance, RegulationType};
pub use submission_api::{RegulatorySubmission, SubmissionApi, SubmissionFormat, SubmissionStatus};
