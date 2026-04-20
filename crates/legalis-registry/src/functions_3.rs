//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::government_import;
use super::types::RegistryError;
use super::types_4::StatuteRegistry;
use super::types_6::StatuteStatus;

/// Scheduled synchronization for periodic imports.
pub mod sync {
    use super::*;
    use chrono::{Datelike, Timelike};
    /// Synchronization schedule.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SyncSchedule {
        /// Manual synchronization only
        Manual,
        /// Hourly synchronization
        Hourly,
        /// Daily synchronization at specified hour
        Daily { hour: u8 },
        /// Weekly synchronization on specified day and hour
        Weekly { day: u8, hour: u8 },
        /// Monthly synchronization on specified day and hour
        Monthly { day: u8, hour: u8 },
        /// Custom interval in seconds
        Interval { seconds: u64 },
    }
    impl SyncSchedule {
        /// Returns the next sync time from a given timestamp.
        pub fn next_sync(&self, from: DateTime<Utc>) -> Option<DateTime<Utc>> {
            match self {
                Self::Manual => None,
                Self::Hourly => Some(from + chrono::Duration::hours(1)),
                Self::Daily { hour } => {
                    let next = from + chrono::Duration::days(1);
                    Some(next.with_hour(*hour as u32).unwrap_or(next))
                }
                Self::Weekly { day: _, hour } => {
                    let next = from + chrono::Duration::weeks(1);
                    Some(next.with_hour(*hour as u32).unwrap_or(next))
                }
                Self::Monthly { day, hour } => {
                    let next =
                        from.with_day(*day as u32).unwrap_or(from) + chrono::Duration::days(30);
                    Some(next.with_hour(*hour as u32).unwrap_or(next))
                }
                Self::Interval { seconds } => {
                    Some(from + chrono::Duration::seconds(*seconds as i64))
                }
            }
        }
        /// Checks if a sync is due from a given last sync time.
        pub fn is_due(&self, last_sync: DateTime<Utc>, now: DateTime<Utc>) -> bool {
            match self.next_sync(last_sync) {
                Some(next) => now >= next,
                None => false,
            }
        }
    }
    /// Synchronization job configuration.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SyncJob {
        /// Job ID
        pub id: Uuid,
        /// Job name
        pub name: String,
        /// Import source
        pub source: government_import::ImportSource,
        /// Schedule
        pub schedule: SyncSchedule,
        /// Last sync timestamp
        pub last_sync: Option<DateTime<Utc>>,
        /// Last sync result
        pub last_result: Option<government_import::BulkImportResult>,
        /// Whether the job is enabled
        pub enabled: bool,
    }
    impl SyncJob {
        /// Creates a new sync job.
        pub fn new(
            name: impl Into<String>,
            source: government_import::ImportSource,
            schedule: SyncSchedule,
        ) -> Self {
            Self {
                id: Uuid::new_v4(),
                name: name.into(),
                source,
                schedule,
                last_sync: None,
                last_result: None,
                enabled: true,
            }
        }
        /// Checks if the job is due for execution.
        pub fn is_due(&self, now: DateTime<Utc>) -> bool {
            if !self.enabled {
                return false;
            }
            match self.last_sync {
                Some(last) => self.schedule.is_due(last, now),
                None => true,
            }
        }
        /// Marks the job as completed with a result.
        pub fn mark_completed(&mut self, result: government_import::BulkImportResult) {
            self.last_sync = Some(Utc::now());
            self.last_result = Some(result);
        }
    }
    /// Synchronization manager.
    #[derive(Debug)]
    pub struct SyncManager {
        jobs: Vec<SyncJob>,
    }
    impl SyncManager {
        /// Creates a new sync manager.
        pub fn new() -> Self {
            Self { jobs: Vec::new() }
        }
        /// Adds a sync job.
        pub fn add_job(&mut self, job: SyncJob) {
            self.jobs.push(job);
        }
        /// Removes a sync job by ID.
        pub fn remove_job(&mut self, job_id: Uuid) -> bool {
            if let Some(pos) = self.jobs.iter().position(|j| j.id == job_id) {
                self.jobs.remove(pos);
                true
            } else {
                false
            }
        }
        /// Gets all jobs.
        pub fn jobs(&self) -> &[SyncJob] {
            &self.jobs
        }
        /// Gets all jobs that are due for execution.
        pub fn due_jobs(&self, now: DateTime<Utc>) -> Vec<&SyncJob> {
            self.jobs.iter().filter(|j| j.is_due(now)).collect()
        }
        /// Updates a job's result.
        pub fn update_job_result(
            &mut self,
            job_id: Uuid,
            result: government_import::BulkImportResult,
        ) {
            if let Some(job) = self.jobs.iter_mut().find(|j| j.id == job_id) {
                job.mark_completed(result);
            }
        }
        /// Enables or disables a job.
        pub fn set_job_enabled(&mut self, job_id: Uuid, enabled: bool) -> bool {
            if let Some(job) = self.jobs.iter_mut().find(|j| j.id == job_id) {
                job.enabled = enabled;
                true
            } else {
                false
            }
        }
    }
    impl Default for SyncManager {
        fn default() -> Self {
            Self::new()
        }
    }
}
/// Format migration utilities.
pub mod migration {
    use super::*;
    /// Supported migration formats.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum MigrationFormat {
        /// Legacy JSON v1
        JsonV1,
        /// Legacy JSON v2
        JsonV2,
        /// Current JSON format
        JsonCurrent,
        /// Legacy XML
        XmlLegacy,
        /// Akoma Ntoso XML
        AkomaNtoso,
        /// CSV format
        Csv,
    }
    /// Migration result.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MigrationResult {
        /// Source format
        pub from_format: MigrationFormat,
        /// Target format
        pub to_format: MigrationFormat,
        /// Number of statutes migrated
        pub migrated: usize,
        /// Number of statutes that failed
        pub failed: usize,
        /// Errors encountered
        pub errors: Vec<String>,
        /// Migration timestamp
        pub timestamp: DateTime<Utc>,
    }
    impl MigrationResult {
        /// Creates a new migration result.
        pub fn new(from: MigrationFormat, to: MigrationFormat) -> Self {
            Self {
                from_format: from,
                to_format: to,
                migrated: 0,
                failed: 0,
                errors: Vec::new(),
                timestamp: Utc::now(),
            }
        }
        /// Returns success rate (0.0-1.0).
        pub fn success_rate(&self) -> f64 {
            let total = self.migrated + self.failed;
            if total == 0 {
                1.0
            } else {
                self.migrated as f64 / total as f64
            }
        }
    }
    /// Format migrator.
    #[derive(Debug)]
    pub struct FormatMigrator {
        /// Whether to validate after migration
        validate: bool,
    }
    impl FormatMigrator {
        /// Creates a new format migrator.
        pub fn new() -> Self {
            Self { validate: true }
        }
        /// Enables or disables validation.
        pub fn with_validation(mut self, validate: bool) -> Self {
            self.validate = validate;
            self
        }
        /// Migrates data from one format to another.
        pub fn migrate(
            &self,
            from_format: MigrationFormat,
            to_format: MigrationFormat,
            data: &str,
        ) -> Result<(String, MigrationResult), RegistryError> {
            let mut result = MigrationResult::new(from_format, to_format);
            match (from_format, to_format) {
                (MigrationFormat::JsonCurrent, MigrationFormat::JsonCurrent) => {
                    result.migrated = 1;
                    Ok((data.to_string(), result))
                }
                _ => {
                    result.failed = 1;
                    result.errors.push(format!(
                        "Migration from {:?} to {:?} not yet implemented",
                        from_format, to_format
                    ));
                    Err(RegistryError::InvalidOperation(format!(
                        "Migration path {:?} -> {:?} not supported",
                        from_format, to_format
                    )))
                }
            }
        }
    }
    impl Default for FormatMigrator {
        fn default() -> Self {
            Self::new()
        }
    }
}
/// Export templates for reporting.
pub mod templates {
    use super::*;
    /// Report template type.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum TemplateType {
        /// Summary report (high-level statistics)
        Summary,
        /// Detailed report (full statute information)
        Detailed,
        /// Compliance report (regulatory focus)
        Compliance,
        /// Audit trail report
        AuditTrail,
        /// Custom template with name
        Custom(String),
    }
    /// Export format for templates.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ExportFormat {
        /// JSON format
        Json,
        /// CSV format
        Csv,
        /// HTML format
        Html,
        /// Markdown format
        Markdown,
        /// PDF format (requires additional dependencies)
        Pdf,
    }
    /// Report template configuration.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ReportTemplate {
        /// Template name
        pub name: String,
        /// Template type
        pub template_type: TemplateType,
        /// Export format
        pub format: ExportFormat,
        /// Fields to include
        pub fields: Vec<String>,
        /// Custom filters
        pub filters: HashMap<String, String>,
        /// Sort order
        pub sort_by: Option<String>,
    }
    impl ReportTemplate {
        /// Creates a new report template.
        pub fn new(
            name: impl Into<String>,
            template_type: TemplateType,
            format: ExportFormat,
        ) -> Self {
            Self {
                name: name.into(),
                template_type,
                format,
                fields: Vec::new(),
                filters: HashMap::new(),
                sort_by: None,
            }
        }
        /// Adds a field to include.
        pub fn with_field(mut self, field: impl Into<String>) -> Self {
            self.fields.push(field.into());
            self
        }
        /// Adds a filter.
        pub fn with_filter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.filters.insert(key.into(), value.into());
            self
        }
        /// Sets the sort order.
        pub fn with_sort_by(mut self, field: impl Into<String>) -> Self {
            self.sort_by = Some(field.into());
            self
        }
        /// Creates a summary template.
        pub fn summary(format: ExportFormat) -> Self {
            Self::new("Summary Report", TemplateType::Summary, format)
                .with_field("id")
                .with_field("title")
                .with_field("status")
                .with_field("jurisdiction")
        }
        /// Creates a detailed template.
        pub fn detailed(format: ExportFormat) -> Self {
            Self::new("Detailed Report", TemplateType::Detailed, format)
                .with_field("id")
                .with_field("title")
                .with_field("status")
                .with_field("jurisdiction")
                .with_field("tags")
                .with_field("metadata")
                .with_field("created_at")
                .with_field("modified_at")
        }
        /// Creates a compliance template.
        pub fn compliance(format: ExportFormat) -> Self {
            Self::new("Compliance Report", TemplateType::Compliance, format)
                .with_field("id")
                .with_field("title")
                .with_field("status")
                .with_field("effective_date")
                .with_field("expiry_date")
        }
    }
    /// Template manager.
    #[derive(Debug)]
    pub struct TemplateManager {
        templates: HashMap<String, ReportTemplate>,
    }
    impl TemplateManager {
        /// Creates a new template manager.
        pub fn new() -> Self {
            Self {
                templates: HashMap::new(),
            }
        }
        /// Adds a template.
        pub fn add_template(&mut self, template: ReportTemplate) {
            self.templates.insert(template.name.clone(), template);
        }
        /// Gets a template by name.
        pub fn get_template(&self, name: &str) -> Option<&ReportTemplate> {
            self.templates.get(name)
        }
        /// Removes a template.
        pub fn remove_template(&mut self, name: &str) -> bool {
            self.templates.remove(name).is_some()
        }
        /// Lists all template names.
        pub fn list_templates(&self) -> Vec<&str> {
            self.templates.keys().map(|s| s.as_str()).collect()
        }
        /// Exports registry data using a template.
        pub fn export(
            &self,
            registry: &StatuteRegistry,
            template_name: &str,
        ) -> Result<String, RegistryError> {
            let template = self.get_template(template_name).ok_or_else(|| {
                RegistryError::InvalidOperation(format!("Template '{}' not found", template_name))
            })?;
            match template.format {
                ExportFormat::Json => self.export_json(registry, template),
                ExportFormat::Csv => self.export_csv(registry, template),
                ExportFormat::Html => self.export_html(registry, template),
                ExportFormat::Markdown => self.export_markdown(registry, template),
                ExportFormat::Pdf => Err(RegistryError::InvalidOperation(
                    "PDF export not yet implemented".to_string(),
                )),
            }
        }
        fn export_json(
            &self,
            registry: &StatuteRegistry,
            _template: &ReportTemplate,
        ) -> Result<String, RegistryError> {
            let statutes: Vec<_> = registry.iter().collect();
            serde_json::to_string_pretty(&statutes)
                .map_err(|e| RegistryError::InvalidOperation(format!("JSON export failed: {}", e)))
        }
        fn export_csv(
            &self,
            registry: &StatuteRegistry,
            template: &ReportTemplate,
        ) -> Result<String, RegistryError> {
            let mut output = String::new();
            if !template.fields.is_empty() {
                output.push_str(&template.fields.join(","));
            } else {
                output.push_str("id,title,status,jurisdiction");
            }
            output.push('\n');
            for entry in registry.iter() {
                let row = format!(
                    "{},{},{:?},{}",
                    entry.statute.id, entry.statute.title, entry.status, entry.jurisdiction
                );
                output.push_str(&row);
                output.push('\n');
            }
            Ok(output)
        }
        fn export_html(
            &self,
            registry: &StatuteRegistry,
            template: &ReportTemplate,
        ) -> Result<String, RegistryError> {
            let mut html = String::from("<html><head><title>");
            html.push_str(&template.name);
            html.push_str("</title></head><body><h1>");
            html.push_str(&template.name);
            html.push_str("</h1><table border='1'><tr>");
            for field in &template.fields {
                html.push_str("<th>");
                html.push_str(field);
                html.push_str("</th>");
            }
            html.push_str("</tr>");
            for entry in registry.iter() {
                html.push_str("<tr>");
                for field in &template.fields {
                    html.push_str("<td>");
                    match field.as_str() {
                        "id" => html.push_str(&entry.statute.id),
                        "title" => html.push_str(&entry.statute.title),
                        "status" => html.push_str(&format!("{:?}", entry.status)),
                        "jurisdiction" => html.push_str(&entry.jurisdiction),
                        _ => html.push_str("N/A"),
                    }
                    html.push_str("</td>");
                }
                html.push_str("</tr>");
            }
            html.push_str("</table></body></html>");
            Ok(html)
        }
        fn export_markdown(
            &self,
            registry: &StatuteRegistry,
            template: &ReportTemplate,
        ) -> Result<String, RegistryError> {
            let mut md = format!("# {}\n\n", template.name);
            if !template.fields.is_empty() {
                md.push_str("| ");
                md.push_str(&template.fields.join(" | "));
                md.push_str(" |\n");
                md.push('|');
                for _ in &template.fields {
                    md.push_str(" --- |");
                }
                md.push('\n');
            }
            for entry in registry.iter() {
                md.push_str("| ");
                for (i, field) in template.fields.iter().enumerate() {
                    if i > 0 {
                        md.push_str(" | ");
                    }
                    match field.as_str() {
                        "id" => md.push_str(&entry.statute.id),
                        "title" => md.push_str(&entry.statute.title),
                        "status" => md.push_str(&format!("{:?}", entry.status)),
                        "jurisdiction" => md.push_str(&entry.jurisdiction),
                        _ => md.push_str("N/A"),
                    }
                }
                md.push_str(" |\n");
            }
            Ok(md)
        }
    }
    impl Default for TemplateManager {
        fn default() -> Self {
            Self::new()
        }
    }
}
/// Approval workflows for statute changes.
pub mod workflow {
    use super::*;
    /// Workflow status for a statute change.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum WorkflowStatus {
        /// Draft - not yet submitted for approval
        Draft,
        /// Pending approval
        PendingApproval,
        /// Approved and ready to apply
        Approved,
        /// Rejected with reason
        Rejected,
        /// Cancelled by submitter
        Cancelled,
    }
    /// Type of change being proposed.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ChangeType {
        /// Creating a new statute
        Create,
        /// Updating an existing statute
        Update { statute_id: String },
        /// Deleting a statute
        Delete { statute_id: String },
        /// Changing status
        StatusChange {
            statute_id: String,
            new_status: StatuteStatus,
        },
        /// Bulk operation
        Bulk { operation_count: usize },
    }
    /// An approval request for a statute change.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ApprovalRequest {
        /// Unique request ID
        pub request_id: Uuid,
        /// Type of change
        pub change_type: ChangeType,
        /// Submitter user ID
        pub submitter: String,
        /// Workflow status
        pub status: WorkflowStatus,
        /// Requested change data (JSON)
        pub change_data: String,
        /// Justification for the change
        pub justification: Option<String>,
        /// Approvers assigned
        pub approvers: Vec<String>,
        /// Approval responses
        pub responses: Vec<ApprovalResponse>,
        /// Created timestamp
        pub created_at: DateTime<Utc>,
        /// Updated timestamp
        pub updated_at: DateTime<Utc>,
        /// Due date for approval
        pub due_date: Option<DateTime<Utc>>,
    }
    impl ApprovalRequest {
        /// Creates a new approval request.
        pub fn new(
            change_type: ChangeType,
            submitter: impl Into<String>,
            change_data: impl Into<String>,
        ) -> Self {
            Self {
                request_id: Uuid::new_v4(),
                change_type,
                submitter: submitter.into(),
                status: WorkflowStatus::Draft,
                change_data: change_data.into(),
                justification: None,
                approvers: Vec::new(),
                responses: Vec::new(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                due_date: None,
            }
        }
        /// Sets the justification.
        pub fn with_justification(mut self, justification: impl Into<String>) -> Self {
            self.justification = Some(justification.into());
            self
        }
        /// Adds an approver.
        pub fn with_approver(mut self, approver: impl Into<String>) -> Self {
            self.approvers.push(approver.into());
            self
        }
        /// Sets the due date.
        pub fn with_due_date(mut self, due_date: DateTime<Utc>) -> Self {
            self.due_date = Some(due_date);
            self
        }
        /// Submits the request for approval.
        pub fn submit(&mut self) {
            self.status = WorkflowStatus::PendingApproval;
            self.updated_at = Utc::now();
        }
        /// Adds an approval response.
        pub fn add_response(&mut self, response: ApprovalResponse) {
            self.responses.push(response);
            self.updated_at = Utc::now();
        }
        /// Checks if the request is approved (all approvers approved).
        pub fn is_approved(&self) -> bool {
            if self.approvers.is_empty() {
                return false;
            }
            let approved_count = self
                .responses
                .iter()
                .filter(|r| r.decision == ApprovalDecision::Approved)
                .count();
            approved_count >= self.approvers.len()
        }
        /// Checks if the request is rejected (any approver rejected).
        pub fn is_rejected(&self) -> bool {
            self.responses
                .iter()
                .any(|r| r.decision == ApprovalDecision::Rejected)
        }
        /// Checks if the request is overdue.
        pub fn is_overdue(&self) -> bool {
            if let Some(due) = self.due_date {
                Utc::now() > due && self.status == WorkflowStatus::PendingApproval
            } else {
                false
            }
        }
    }
    /// Approval decision.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ApprovalDecision {
        /// Approved
        Approved,
        /// Rejected
        Rejected,
        /// Needs more information
        NeedsInfo,
    }
    /// An approval response from an approver.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ApprovalResponse {
        /// Approver user ID
        pub approver: String,
        /// Decision
        pub decision: ApprovalDecision,
        /// Comments
        pub comments: Option<String>,
        /// Response timestamp
        pub responded_at: DateTime<Utc>,
    }
    impl ApprovalResponse {
        /// Creates a new approval response.
        pub fn new(approver: impl Into<String>, decision: ApprovalDecision) -> Self {
            Self {
                approver: approver.into(),
                decision,
                comments: None,
                responded_at: Utc::now(),
            }
        }
        /// Sets comments.
        pub fn with_comments(mut self, comments: impl Into<String>) -> Self {
            self.comments = Some(comments.into());
            self
        }
    }
    /// Type alias for auto-approval rule functions.
    pub type AutoApproveRule = Box<dyn Fn(&ApprovalRequest) -> bool + Send + Sync>;
    /// Workflow manager for approval requests.
    pub struct WorkflowManager {
        requests: HashMap<Uuid, ApprovalRequest>,
        /// Auto-approval rules
        auto_approve_rules: Vec<AutoApproveRule>,
    }
    impl std::fmt::Debug for WorkflowManager {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("WorkflowManager")
                .field("requests", &self.requests)
                .field(
                    "auto_approve_rules",
                    &format!("<{} rules>", self.auto_approve_rules.len()),
                )
                .finish()
        }
    }
    impl WorkflowManager {
        /// Creates a new workflow manager.
        pub fn new() -> Self {
            Self {
                requests: HashMap::new(),
                auto_approve_rules: Vec::new(),
            }
        }
        /// Submits a new approval request.
        pub fn submit_request(&mut self, mut request: ApprovalRequest) -> Uuid {
            request.submit();
            let id = request.request_id;
            for rule in &self.auto_approve_rules {
                if rule(&request) {
                    request.status = WorkflowStatus::Approved;
                    break;
                }
            }
            self.requests.insert(id, request);
            id
        }
        /// Gets a request by ID.
        pub fn get_request(&self, request_id: Uuid) -> Option<&ApprovalRequest> {
            self.requests.get(&request_id)
        }
        /// Adds a response to a request.
        pub fn add_response(
            &mut self,
            request_id: Uuid,
            response: ApprovalResponse,
        ) -> Result<(), String> {
            let request = self
                .requests
                .get_mut(&request_id)
                .ok_or_else(|| "Request not found".to_string())?;
            if request.status != WorkflowStatus::PendingApproval {
                return Err("Request is not pending approval".to_string());
            }
            request.add_response(response);
            if request.is_rejected() {
                request.status = WorkflowStatus::Rejected;
            } else if request.is_approved() {
                request.status = WorkflowStatus::Approved;
            }
            Ok(())
        }
        /// Gets pending requests.
        pub fn pending_requests(&self) -> Vec<&ApprovalRequest> {
            self.requests
                .values()
                .filter(|r| r.status == WorkflowStatus::PendingApproval)
                .collect()
        }
        /// Gets overdue requests.
        pub fn overdue_requests(&self) -> Vec<&ApprovalRequest> {
            self.requests.values().filter(|r| r.is_overdue()).collect()
        }
        /// Gets requests for a specific approver.
        pub fn requests_for_approver(&self, approver: &str) -> Vec<&ApprovalRequest> {
            self.requests
                .values()
                .filter(|r| {
                    r.approvers.contains(&approver.to_string())
                        && r.status == WorkflowStatus::PendingApproval
                })
                .collect()
        }
    }
    impl Default for WorkflowManager {
        fn default() -> Self {
            Self::new()
        }
    }
}
/// Notification system for stakeholders.
pub mod notifications {
    use super::*;
    /// Notification type.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum NotificationType {
        /// Approval request submitted
        ApprovalRequested,
        /// Approval granted
        ApprovalGranted,
        /// Approval rejected
        ApprovalRejected,
        /// Task assigned
        TaskAssigned,
        /// Task completed
        TaskCompleted,
        /// SLA warning
        SlaWarning,
        /// SLA breach
        SlaBreach,
        /// Statute updated
        StatuteUpdated,
        /// Custom notification
        Custom(String),
    }
    /// Notification priority.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    pub enum NotificationPriority {
        /// Low priority
        Low,
        /// Normal priority
        Normal,
        /// High priority
        High,
        /// Critical priority
        Critical,
    }
    /// Notification channel.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum NotificationChannel {
        /// Email notification
        Email,
        /// SMS notification
        Sms,
        /// In-app notification
        InApp,
        /// Webhook notification
        Webhook { url: String },
        /// Custom channel
        Custom(String),
    }
    /// A notification.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Notification {
        /// Notification ID
        pub notification_id: Uuid,
        /// Recipient user ID
        pub recipient: String,
        /// Notification type
        pub notification_type: NotificationType,
        /// Priority
        pub priority: NotificationPriority,
        /// Title
        pub title: String,
        /// Message
        pub message: String,
        /// Related entity ID (e.g., request ID, statute ID)
        pub related_entity_id: Option<String>,
        /// Channels to send through
        pub channels: Vec<NotificationChannel>,
        /// Created timestamp
        pub created_at: DateTime<Utc>,
        /// Sent timestamp
        pub sent_at: Option<DateTime<Utc>>,
        /// Read timestamp
        pub read_at: Option<DateTime<Utc>>,
    }
    impl Notification {
        /// Creates a new notification.
        pub fn new(
            recipient: impl Into<String>,
            notification_type: NotificationType,
            title: impl Into<String>,
            message: impl Into<String>,
        ) -> Self {
            Self {
                notification_id: Uuid::new_v4(),
                recipient: recipient.into(),
                notification_type,
                priority: NotificationPriority::Normal,
                title: title.into(),
                message: message.into(),
                related_entity_id: None,
                channels: vec![NotificationChannel::InApp],
                created_at: Utc::now(),
                sent_at: None,
                read_at: None,
            }
        }
        /// Sets priority.
        pub fn with_priority(mut self, priority: NotificationPriority) -> Self {
            self.priority = priority;
            self
        }
        /// Sets related entity ID.
        pub fn with_related_entity(mut self, entity_id: impl Into<String>) -> Self {
            self.related_entity_id = Some(entity_id.into());
            self
        }
        /// Adds a channel.
        pub fn with_channel(mut self, channel: NotificationChannel) -> Self {
            self.channels.push(channel);
            self
        }
        /// Marks as sent.
        pub fn mark_sent(&mut self) {
            self.sent_at = Some(Utc::now());
        }
        /// Marks as read.
        pub fn mark_read(&mut self) {
            self.read_at = Some(Utc::now());
        }
        /// Checks if sent.
        pub fn is_sent(&self) -> bool {
            self.sent_at.is_some()
        }
        /// Checks if read.
        pub fn is_read(&self) -> bool {
            self.read_at.is_some()
        }
    }
    /// Notification manager.
    #[derive(Debug)]
    pub struct NotificationManager {
        notifications: Vec<Notification>,
        max_notifications: usize,
    }
    impl NotificationManager {
        /// Creates a new notification manager.
        pub fn new() -> Self {
            Self {
                notifications: Vec::new(),
                max_notifications: 10000,
            }
        }
        /// Sends a notification.
        pub fn send(&mut self, mut notification: Notification) {
            notification.mark_sent();
            self.notifications.push(notification);
            if self.notifications.len() > self.max_notifications {
                self.notifications
                    .drain(0..self.notifications.len() - self.max_notifications);
            }
        }
        /// Gets unread notifications for a user.
        pub fn unread_for_user(&self, user_id: &str) -> Vec<&Notification> {
            self.notifications
                .iter()
                .filter(|n| n.recipient == user_id && !n.is_read())
                .collect()
        }
        /// Marks a notification as read.
        pub fn mark_as_read(&mut self, notification_id: Uuid) -> bool {
            if let Some(notification) = self
                .notifications
                .iter_mut()
                .find(|n| n.notification_id == notification_id)
            {
                notification.mark_read();
                true
            } else {
                false
            }
        }
        /// Gets all notifications for a user.
        pub fn for_user(&self, user_id: &str) -> Vec<&Notification> {
            self.notifications
                .iter()
                .filter(|n| n.recipient == user_id)
                .collect()
        }
        /// Gets notifications by priority.
        pub fn by_priority(&self, min_priority: NotificationPriority) -> Vec<&Notification> {
            self.notifications
                .iter()
                .filter(|n| n.priority >= min_priority)
                .collect()
        }
    }
    impl Default for NotificationManager {
        fn default() -> Self {
            Self::new()
        }
    }
}
