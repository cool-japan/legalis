//! Scheduled report generation for audit trails.
//!
//! This module provides functionality for scheduling periodic audit reports.

use crate::{AuditError, AuditResult, AuditTrail};
use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// Schedule frequency for report generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleFrequency {
    /// Daily at specified hour (UTC)
    Daily { hour: u32 },
    /// Weekly on specified day (0-6, Sunday-Saturday) and hour
    Weekly { day: u32, hour: u32 },
    /// Monthly on specified day of month and hour
    Monthly { day: u32, hour: u32 },
    /// Custom interval in seconds
    Interval { seconds: u64 },
}

impl ScheduleFrequency {
    /// Calculate the next scheduled time from a given reference time.
    pub fn next_run(&self, from: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            ScheduleFrequency::Daily { hour } => {
                let mut next = from
                    .date_naive()
                    .and_hms_opt(*hour, 0, 0)
                    .unwrap()
                    .and_utc();
                if next <= from {
                    next += Duration::days(1);
                }
                next
            }
            ScheduleFrequency::Weekly { day, hour } => {
                let mut next = from
                    .date_naive()
                    .and_hms_opt(*hour, 0, 0)
                    .unwrap()
                    .and_utc();
                let target_weekday = *day % 7;
                let current_weekday = from.weekday().num_days_from_sunday();
                let days_until_target = if target_weekday >= current_weekday {
                    target_weekday - current_weekday
                } else {
                    7 - (current_weekday - target_weekday)
                };
                next += Duration::days(days_until_target as i64);
                if next <= from {
                    next += Duration::days(7);
                }
                next
            }
            ScheduleFrequency::Monthly { day, hour } => {
                let mut next = from
                    .date_naive()
                    .and_hms_opt(*hour, 0, 0)
                    .unwrap()
                    .and_utc();
                let target_day = (*day).min(28); // Ensure valid day
                if from.day() < target_day || (from.day() == target_day && from.hour() < *hour) {
                    next = next
                        .with_day(target_day)
                        .unwrap_or(next + Duration::days(1));
                } else {
                    // Move to next month
                    next = (next + Duration::days(32))
                        .with_day(target_day)
                        .unwrap_or(next + Duration::days(1));
                }
                next
            }
            ScheduleFrequency::Interval { seconds } => from + Duration::seconds(*seconds as i64),
        }
    }
}

/// Report format for scheduled reports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    /// CSV format
    Csv,
    /// JSON format
    Json,
    /// JSON-LD format
    JsonLd,
    /// Excel format
    Excel,
    /// PDF format
    Pdf,
    /// HTML format
    Html,
}

/// Report scope defines what data to include in the report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportScope {
    /// All records
    All,
    /// Records from last N days
    LastDays(i64),
    /// Records from last N hours
    LastHours(i64),
    /// Records since last report
    SinceLastReport,
    /// Custom time range
    TimeRange {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
}

/// Configuration for a scheduled report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledReportConfig {
    /// Unique identifier for this schedule
    pub id: Uuid,
    /// Name of the scheduled report
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Schedule frequency
    pub frequency: ScheduleFrequency,
    /// Report format
    pub format: ReportFormat,
    /// Report scope
    pub scope: ReportScope,
    /// Output directory for generated reports
    pub output_dir: PathBuf,
    /// Whether to include compliance summary
    pub include_compliance: bool,
    /// Whether to include analysis
    pub include_analysis: bool,
    /// Custom report title
    pub title: Option<String>,
    /// Whether this schedule is active
    pub enabled: bool,
    /// Next scheduled run time
    pub next_run: DateTime<Utc>,
    /// Last run time (if any)
    pub last_run: Option<DateTime<Utc>>,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

impl ScheduledReportConfig {
    /// Creates a new scheduled report configuration.
    pub fn new(
        name: String,
        frequency: ScheduleFrequency,
        format: ReportFormat,
        output_dir: PathBuf,
    ) -> Self {
        let next_run = frequency.next_run(Utc::now());
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            frequency,
            format,
            scope: ReportScope::All,
            output_dir,
            include_compliance: true,
            include_analysis: true,
            title: None,
            enabled: true,
            next_run,
            last_run: None,
            metadata: HashMap::new(),
        }
    }

    /// Sets the report scope.
    pub fn with_scope(mut self, scope: ReportScope) -> Self {
        self.scope = scope;
        self
    }

    /// Sets the description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the custom title.
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Sets whether to include compliance summary.
    pub fn with_compliance(mut self, include: bool) -> Self {
        self.include_compliance = include;
        self
    }

    /// Sets whether to include analysis.
    pub fn with_analysis(mut self, include: bool) -> Self {
        self.include_analysis = include;
        self
    }

    /// Checks if this report is due to run.
    pub fn is_due(&self) -> bool {
        self.enabled && Utc::now() >= self.next_run
    }

    /// Updates the schedule after a run.
    pub fn mark_completed(&mut self) {
        self.last_run = Some(Utc::now());
        self.next_run = self.frequency.next_run(Utc::now());
    }
}

/// Scheduler for managing scheduled reports.
pub struct ReportScheduler {
    schedules: Vec<ScheduledReportConfig>,
}

impl ReportScheduler {
    /// Creates a new report scheduler.
    pub fn new() -> Self {
        Self {
            schedules: Vec::new(),
        }
    }

    /// Adds a new scheduled report.
    pub fn add_schedule(&mut self, config: ScheduledReportConfig) {
        self.schedules.push(config);
    }

    /// Removes a scheduled report by ID.
    pub fn remove_schedule(&mut self, id: Uuid) -> bool {
        if let Some(pos) = self.schedules.iter().position(|s| s.id == id) {
            self.schedules.remove(pos);
            true
        } else {
            false
        }
    }

    /// Gets a scheduled report by ID.
    pub fn get_schedule(&self, id: Uuid) -> Option<&ScheduledReportConfig> {
        self.schedules.iter().find(|s| s.id == id)
    }

    /// Gets a mutable scheduled report by ID.
    pub fn get_schedule_mut(&mut self, id: Uuid) -> Option<&mut ScheduledReportConfig> {
        self.schedules.iter_mut().find(|s| s.id == id)
    }

    /// Lists all scheduled reports.
    pub fn list_schedules(&self) -> &[ScheduledReportConfig] {
        &self.schedules
    }

    /// Checks for due reports and returns their IDs.
    pub fn check_due_reports(&self) -> Vec<Uuid> {
        self.schedules
            .iter()
            .filter(|s| s.is_due())
            .map(|s| s.id)
            .collect()
    }

    /// Generates a scheduled report.
    pub fn generate_report(
        &mut self,
        schedule_id: Uuid,
        trail: &AuditTrail,
    ) -> AuditResult<PathBuf> {
        let schedule = self.get_schedule_mut(schedule_id).ok_or_else(|| {
            AuditError::InvalidRecord(format!("Schedule not found: {}", schedule_id))
        })?;

        if !schedule.enabled {
            return Err(AuditError::InvalidRecord(format!(
                "Schedule is disabled: {}",
                schedule_id
            )));
        }

        // Determine output filename
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let ext = match &schedule.format {
            ReportFormat::Csv => "csv",
            ReportFormat::Json => "json",
            ReportFormat::JsonLd => "jsonld",
            ReportFormat::Excel => "xlsx",
            ReportFormat::Pdf => "pdf",
            ReportFormat::Html => "html",
        };
        let filename = format!("{}_{}.{}", schedule.name, timestamp, ext);
        let output_path = schedule.output_dir.join(&filename);

        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&schedule.output_dir)?;

        // Generate the report based on format
        let title = schedule
            .title
            .clone()
            .unwrap_or_else(|| format!("{} - {}", schedule.name, timestamp));

        match &schedule.format {
            ReportFormat::Csv => {
                let mut file = std::fs::File::create(&output_path)?;
                trail.export_csv(&mut file)?;
            }
            ReportFormat::Json => {
                let json = trail.export_json()?;
                std::fs::write(&output_path, serde_json::to_string_pretty(&json)?)?;
            }
            ReportFormat::JsonLd => {
                let jsonld = trail.export_jsonld()?;
                std::fs::write(&output_path, serde_json::to_string_pretty(&jsonld)?)?;
            }
            ReportFormat::Excel => {
                trail.export_excel(&output_path)?;
            }
            ReportFormat::Pdf => {
                trail.export_pdf(&output_path, &title)?;
            }
            ReportFormat::Html => {
                let html = trail.export_html(&title)?;
                std::fs::write(&output_path, html)?;
            }
        }

        // Mark schedule as completed
        schedule.mark_completed();

        tracing::info!(
            "Generated scheduled report: {} -> {:?}",
            schedule.name,
            output_path
        );

        Ok(output_path)
    }

    /// Runs all due reports.
    pub fn run_due_reports(&mut self, trail: &AuditTrail) -> Vec<AuditResult<PathBuf>> {
        let due_ids = self.check_due_reports();
        due_ids
            .into_iter()
            .map(|id| self.generate_report(id, trail))
            .collect()
    }
}

impl Default for ReportScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, AuditRecord, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    #[test]
    fn test_schedule_frequency_daily() {
        let freq = ScheduleFrequency::Daily { hour: 14 };
        let now = Utc::now();
        let next = freq.next_run(now);
        assert!(next > now);
        assert_eq!(next.hour(), 14);
        assert_eq!(next.minute(), 0);
    }

    #[test]
    fn test_schedule_frequency_interval() {
        let freq = ScheduleFrequency::Interval { seconds: 3600 };
        let now = Utc::now();
        let next = freq.next_run(now);
        assert_eq!((next - now).num_seconds(), 3600);
    }

    #[test]
    fn test_scheduled_report_config() {
        let config = ScheduledReportConfig::new(
            "Daily Audit Report".to_string(),
            ScheduleFrequency::Daily { hour: 9 },
            ReportFormat::Pdf,
            PathBuf::from("/tmp/reports"),
        )
        .with_description("Daily compliance report".to_string())
        .with_scope(ReportScope::LastDays(1));

        assert_eq!(config.name, "Daily Audit Report");
        assert!(config.enabled);
        assert!(config.include_compliance);
    }

    #[test]
    fn test_report_scheduler() {
        let mut scheduler = ReportScheduler::new();
        let config = ScheduledReportConfig::new(
            "Test Report".to_string(),
            ScheduleFrequency::Interval { seconds: 60 },
            ReportFormat::Json,
            PathBuf::from("/tmp/reports"),
        );
        let id = config.id;

        scheduler.add_schedule(config);
        assert_eq!(scheduler.list_schedules().len(), 1);

        let schedule = scheduler.get_schedule(id).unwrap();
        assert_eq!(schedule.name, "Test Report");

        scheduler.remove_schedule(id);
        assert_eq!(scheduler.list_schedules().len(), 0);
    }

    #[test]
    fn test_is_due() {
        let mut config = ScheduledReportConfig::new(
            "Test Report".to_string(),
            ScheduleFrequency::Interval { seconds: 1 },
            ReportFormat::Json,
            PathBuf::from("/tmp/reports"),
        );
        config.next_run = Utc::now() - Duration::seconds(10);
        assert!(config.is_due());

        config.next_run = Utc::now() + Duration::seconds(3600);
        assert!(!config.is_due());
    }

    #[test]
    fn test_mark_completed() {
        let mut config = ScheduledReportConfig::new(
            "Test Report".to_string(),
            ScheduleFrequency::Interval { seconds: 60 },
            ReportFormat::Json,
            PathBuf::from("/tmp/reports"),
        );
        let original_next = config.next_run;
        config.mark_completed();
        assert!(config.last_run.is_some());
        assert!(config.next_run > original_next);
    }

    #[test]
    fn test_generate_scheduled_report() {
        use tempfile::tempdir;

        let temp_dir = tempdir().unwrap();
        let mut trail = AuditTrail::new();

        // Add some test records
        for i in 0..3 {
            let record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                format!("statute-{}", i),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "test".to_string(),
                    parameters: HashMap::new(),
                },
                None,
            );
            trail.record(record).unwrap();
        }

        let mut scheduler = ReportScheduler::new();
        let config = ScheduledReportConfig::new(
            "test_report".to_string(),
            ScheduleFrequency::Daily { hour: 9 },
            ReportFormat::Json,
            temp_dir.path().to_path_buf(),
        );
        let id = config.id;
        scheduler.add_schedule(config);

        // Force the schedule to be due
        scheduler.get_schedule_mut(id).unwrap().next_run = Utc::now() - Duration::seconds(1);

        let result = scheduler.generate_report(id, &trail);
        assert!(result.is_ok());

        let output_path = result.unwrap();
        assert!(output_path.exists());
        assert!(output_path.extension().unwrap() == "json");
    }
}
