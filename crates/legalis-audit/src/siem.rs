//! SIEM (Security Information and Event Management) integration.
//!
//! This module provides integration with SIEM systems through standard formats:
//! - Syslog (RFC 5424)
//! - Common Event Format (CEF)
//! - LEEF (Log Event Extended Format)

use crate::{AuditRecord, AuditResult, EventType};
use serde::{Deserialize, Serialize};

/// SIEM export format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SiemFormat {
    /// Syslog format (RFC 5424)
    Syslog,
    /// Common Event Format (CEF)
    Cef,
    /// Log Event Extended Format (LEEF)
    Leef,
}

/// Syslog severity levels (RFC 5424).
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum SyslogSeverity {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Informational = 6,
    Debug = 7,
}

/// Syslog facility codes (RFC 5424).
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum SyslogFacility {
    Kernel = 0,
    User = 1,
    Mail = 2,
    Daemon = 3,
    Auth = 4,
    Syslog = 5,
    Lpr = 6,
    News = 7,
    Uucp = 8,
    Cron = 9,
    Authpriv = 10,
    Ftp = 11,
    Local0 = 16,
    Local1 = 17,
    Local2 = 18,
    Local3 = 19,
    Local4 = 20,
    Local5 = 21,
    Local6 = 22,
    Local7 = 23,
}

/// SIEM exporter configuration.
#[derive(Debug, Clone)]
pub struct SiemConfig {
    /// Export format
    pub format: SiemFormat,
    /// Application name (for syslog)
    pub app_name: String,
    /// Hostname (for syslog)
    pub hostname: String,
    /// Facility (for syslog)
    pub facility: SyslogFacility,
    /// Vendor (for CEF/LEEF)
    pub vendor: String,
    /// Product (for CEF/LEEF)
    pub product: String,
    /// Version (for CEF/LEEF)
    pub version: String,
}

impl Default for SiemConfig {
    fn default() -> Self {
        Self {
            format: SiemFormat::Syslog,
            app_name: "legalis-audit".to_string(),
            hostname: hostname::get()
                .ok()
                .and_then(|h| h.into_string().ok())
                .unwrap_or_else(|| "unknown".to_string()),
            facility: SyslogFacility::Local0,
            vendor: "Legalis".to_string(),
            product: "Legalis-Audit".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// SIEM event exporter.
pub struct SiemExporter {
    config: SiemConfig,
}

impl SiemExporter {
    /// Creates a new SIEM exporter with the given configuration.
    pub fn new(config: SiemConfig) -> Self {
        Self { config }
    }

    /// Creates a new SIEM exporter with default configuration.
    pub fn with_format(format: SiemFormat) -> Self {
        let config = SiemConfig {
            format,
            ..Default::default()
        };
        Self { config }
    }

    /// Exports an audit record to SIEM format.
    pub fn export_record(&self, record: &AuditRecord) -> AuditResult<String> {
        match self.config.format {
            SiemFormat::Syslog => self.to_syslog(record),
            SiemFormat::Cef => self.to_cef(record),
            SiemFormat::Leef => self.to_leef(record),
        }
    }

    /// Exports multiple audit records to SIEM format.
    pub fn export_records(&self, records: &[AuditRecord]) -> AuditResult<Vec<String>> {
        records.iter().map(|r| self.export_record(r)).collect()
    }

    /// Converts an audit record to Syslog format (RFC 5424).
    fn to_syslog(&self, record: &AuditRecord) -> AuditResult<String> {
        let severity = self.event_type_to_severity(&record.event_type);
        let priority = (self.config.facility as u8) * 8 + (severity as u8);

        let timestamp = record.timestamp.to_rfc3339();
        let msg_id = format!("{:?}", record.event_type);
        let structured_data = self.build_structured_data(record);

        Ok(format!(
            "<{}> 1 {} {} {} - {} {} {}",
            priority,
            timestamp,
            self.config.hostname,
            self.config.app_name,
            msg_id,
            structured_data,
            self.build_syslog_message(record)
        ))
    }

    /// Converts an audit record to CEF format.
    fn to_cef(&self, record: &AuditRecord) -> AuditResult<String> {
        let event_name = format!("{:?}", record.event_type);
        let severity = self.event_type_to_cef_severity(&record.event_type);

        let mut extensions = Vec::new();
        extensions.push(format!("rt={}", record.timestamp.timestamp_millis()));
        extensions.push(format!("suid={}", record.id));
        extensions.push(format!("cs1Label=StatuteID cs1={}", record.statute_id));
        extensions.push(format!("cs2Label=SubjectID cs2={}", record.subject_id));
        extensions.push(format!(
            "cs3Label=Actor cs3={}",
            self.format_actor(&record.actor)
        ));

        if let Some(ref prev_hash) = record.previous_hash {
            extensions.push(format!("cs4Label=PreviousHash cs4={}", prev_hash));
        }
        extensions.push(format!("cs5Label=RecordHash cs5={}", record.record_hash));

        Ok(format!(
            "CEF:0|{}|{}|{}|{}|{}|{}|{}",
            self.config.vendor,
            self.config.product,
            self.config.version,
            record.id,
            event_name,
            severity,
            extensions.join(" ")
        ))
    }

    /// Converts an audit record to LEEF format.
    fn to_leef(&self, record: &AuditRecord) -> AuditResult<String> {
        let event_id = format!("{:?}", record.event_type);

        let mut fields = Vec::new();
        fields.push(format!("devTime={}", record.timestamp.to_rfc3339()));
        fields.push("devTimeFormat=ISO8601".to_string());
        fields.push(format!("cat={}", event_id));
        fields.push(format!(
            "sev={}",
            self.event_type_to_leef_severity(&record.event_type)
        ));
        fields.push(format!("usrName={}", self.format_actor(&record.actor)));
        fields.push(format!("src={}", record.subject_id));
        fields.push(format!("dst={}", record.statute_id));
        fields.push(format!("identSrc={}", record.id));

        if let Some(ref prev_hash) = record.previous_hash {
            fields.push(format!("proto={}", prev_hash));
        }

        Ok(format!(
            "LEEF:2.0|{}|{}|{}|{}|{}",
            self.config.vendor,
            self.config.product,
            self.config.version,
            event_id,
            fields.join("\t")
        ))
    }

    /// Builds structured data for syslog.
    fn build_structured_data(&self, record: &AuditRecord) -> String {
        format!(
            "[audit@legalis id=\"{}\" statute=\"{}\" subject=\"{}\" actor=\"{}\"]",
            record.id,
            record.statute_id,
            record.subject_id,
            self.format_actor(&record.actor)
        )
    }

    /// Builds the syslog message part.
    fn build_syslog_message(&self, record: &AuditRecord) -> String {
        format!(
            "Audit event {:?} for statute {} on subject {} by {}",
            record.event_type,
            record.statute_id,
            record.subject_id,
            self.format_actor(&record.actor)
        )
    }

    /// Formats actor for display.
    fn format_actor(&self, actor: &crate::Actor) -> String {
        match actor {
            crate::Actor::System { component } => format!("System:{}", component),
            crate::Actor::User { user_id, role } => format!("User:{}:{}", user_id, role),
            crate::Actor::External { system_id } => format!("External:{}", system_id),
        }
    }

    /// Maps event type to syslog severity.
    fn event_type_to_severity(&self, event_type: &EventType) -> SyslogSeverity {
        match event_type {
            EventType::AutomaticDecision => SyslogSeverity::Informational,
            EventType::DiscretionaryReview => SyslogSeverity::Notice,
            EventType::HumanOverride => SyslogSeverity::Warning,
            EventType::Appeal => SyslogSeverity::Notice,
            EventType::StatuteModified => SyslogSeverity::Warning,
            EventType::SimulationRun => SyslogSeverity::Debug,
        }
    }

    /// Maps event type to CEF severity (0-10).
    fn event_type_to_cef_severity(&self, event_type: &EventType) -> u8 {
        match event_type {
            EventType::AutomaticDecision => 3,
            EventType::DiscretionaryReview => 5,
            EventType::HumanOverride => 7,
            EventType::Appeal => 5,
            EventType::StatuteModified => 8,
            EventType::SimulationRun => 2,
        }
    }

    /// Maps event type to LEEF severity (0-10).
    fn event_type_to_leef_severity(&self, event_type: &EventType) -> u8 {
        self.event_type_to_cef_severity(event_type)
    }
}

/// Syslog sender for real-time streaming to SIEM.
pub struct SyslogSender {
    exporter: SiemExporter,
    #[allow(dead_code)]
    destination: String,
}

impl SyslogSender {
    /// Creates a new syslog sender.
    ///
    /// `destination` should be in the format "host:port" (e.g., "localhost:514").
    pub fn new(destination: String, config: SiemConfig) -> Self {
        Self {
            exporter: SiemExporter::new(config),
            destination,
        }
    }

    /// Sends an audit record to the syslog destination.
    pub fn send(&self, record: &AuditRecord) -> AuditResult<()> {
        let message = self.exporter.export_record(record)?;

        // In a real implementation, this would send via UDP/TCP to the syslog server
        // For now, we just log it
        tracing::info!("Syslog message: {}", message);

        Ok(())
    }

    /// Sends multiple audit records to the syslog destination.
    pub fn send_batch(&self, records: &[AuditRecord]) -> AuditResult<()> {
        for record in records {
            self.send(record)?;
        }
        Ok(())
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
    fn test_syslog_export() {
        let exporter = SiemExporter::with_format(SiemFormat::Syslog);
        let record = create_test_record();

        let syslog = exporter.export_record(&record).unwrap();
        assert!(syslog.starts_with("<"));
        assert!(syslog.contains("legalis-audit"));
        assert!(syslog.contains(&record.id.to_string()));
    }

    #[test]
    fn test_cef_export() {
        let exporter = SiemExporter::with_format(SiemFormat::Cef);
        let record = create_test_record();

        let cef = exporter.export_record(&record).unwrap();
        assert!(cef.starts_with("CEF:0|"));
        assert!(cef.contains("Legalis"));
        assert!(cef.contains("Legalis-Audit"));
        assert!(cef.contains(&record.id.to_string()));
    }

    #[test]
    fn test_leef_export() {
        let exporter = SiemExporter::with_format(SiemFormat::Leef);
        let record = create_test_record();

        let leef = exporter.export_record(&record).unwrap();
        assert!(leef.starts_with("LEEF:2.0|"));
        assert!(leef.contains("Legalis"));
        assert!(leef.contains("Legalis-Audit"));
        assert!(leef.contains(&record.id.to_string()));
    }

    #[test]
    fn test_export_batch() {
        let exporter = SiemExporter::with_format(SiemFormat::Syslog);
        let records = vec![
            create_test_record(),
            create_test_record(),
            create_test_record(),
        ];

        let exported = exporter.export_records(&records).unwrap();
        assert_eq!(exported.len(), 3);
        assert!(exported.iter().all(|s| s.starts_with("<")));
    }

    #[test]
    fn test_syslog_sender() {
        let config = SiemConfig::default();
        let sender = SyslogSender::new("localhost:514".to_string(), config);
        let record = create_test_record();

        // Should not fail (just logs internally in test mode)
        assert!(sender.send(&record).is_ok());
    }

    #[test]
    fn test_event_type_severities() {
        let exporter = SiemExporter::with_format(SiemFormat::Syslog);

        assert_eq!(
            exporter.event_type_to_severity(&EventType::AutomaticDecision) as u8,
            SyslogSeverity::Informational as u8
        );
        assert_eq!(
            exporter.event_type_to_severity(&EventType::HumanOverride) as u8,
            SyslogSeverity::Warning as u8
        );
        assert_eq!(
            exporter.event_type_to_severity(&EventType::SimulationRun) as u8,
            SyslogSeverity::Debug as u8
        );
    }
}
