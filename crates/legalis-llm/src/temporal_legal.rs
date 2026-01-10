//! Temporal Legal Reasoning
//!
//! Time-aware legal analysis including temporal validity, legal change detection,
//! retroactive vs. prospective application, and historical legal analysis.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Temporal validity period for a legal rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidityPeriod {
    /// Start date (Unix timestamp)
    pub start: i64,
    /// End date (Unix timestamp), None if still valid
    pub end: Option<i64>,
    /// Reason for start
    pub start_reason: String,
    /// Reason for end (if applicable)
    pub end_reason: Option<String>,
}

impl ValidityPeriod {
    /// Creates a new validity period.
    pub fn new(start: i64, start_reason: impl Into<String>) -> Self {
        Self {
            start,
            end: None,
            start_reason: start_reason.into(),
            end_reason: None,
        }
    }

    /// Sets the end date.
    pub fn with_end(mut self, end: i64, reason: impl Into<String>) -> Self {
        self.end = Some(end);
        self.end_reason = Some(reason.into());
        self
    }

    /// Checks if the period is valid at a given time.
    pub fn is_valid_at(&self, timestamp: i64) -> bool {
        timestamp >= self.start && self.end.is_none_or(|end| timestamp <= end)
    }

    /// Checks if the period overlaps with another.
    pub fn overlaps(&self, other: &ValidityPeriod) -> bool {
        let self_end = self.end.unwrap_or(i64::MAX);
        let other_end = other.end.unwrap_or(i64::MAX);

        self.start <= other_end && other.start <= self_end
    }
}

/// Time-aware legal rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalLegalRule {
    /// Rule ID
    pub id: String,
    /// Rule text
    pub text: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Validity periods (can have multiple for complex histories)
    pub validity_periods: Vec<ValidityPeriod>,
    /// Application type
    pub application_type: ApplicationType,
    /// Amendments over time
    pub amendments: Vec<Amendment>,
}

impl TemporalLegalRule {
    /// Creates a new temporal legal rule.
    pub fn new(
        id: impl Into<String>,
        text: impl Into<String>,
        jurisdiction: impl Into<String>,
        valid_from: i64,
    ) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            jurisdiction: jurisdiction.into(),
            validity_periods: vec![ValidityPeriod::new(valid_from, "Enacted")],
            application_type: ApplicationType::Prospective,
            amendments: Vec::new(),
        }
    }

    /// Adds a validity period.
    pub fn add_validity_period(&mut self, period: ValidityPeriod) {
        self.validity_periods.push(period);
    }

    /// Sets the application type.
    pub fn with_application_type(mut self, app_type: ApplicationType) -> Self {
        self.application_type = app_type;
        self
    }

    /// Adds an amendment.
    pub fn add_amendment(&mut self, amendment: Amendment) {
        self.amendments.push(amendment);
    }

    /// Checks if the rule is valid at a given time.
    pub fn is_valid_at(&self, timestamp: i64) -> bool {
        self.validity_periods
            .iter()
            .any(|p| p.is_valid_at(timestamp))
    }

    /// Gets the rule text at a specific time (accounting for amendments).
    pub fn get_text_at(&self, timestamp: i64) -> String {
        let mut text = self.text.clone();

        for amendment in &self.amendments {
            if amendment.effective_date <= timestamp {
                text = amendment.amended_text.clone();
            }
        }

        text
    }

    /// Determines if the rule applies to events at a given time.
    pub fn applies_to_event_at(&self, event_time: i64, current_time: i64) -> bool {
        match self.application_type {
            ApplicationType::Prospective => {
                // Only applies to events after enactment
                self.validity_periods
                    .iter()
                    .any(|p| event_time >= p.start && p.is_valid_at(event_time))
            }
            ApplicationType::Retroactive => {
                // Applies to past events if rule is valid now
                self.is_valid_at(current_time)
            }
            ApplicationType::Hybrid { retroactive_date } => {
                // Applies retroactively to specific date
                event_time >= retroactive_date && self.is_valid_at(current_time)
            }
        }
    }
}

/// Application type for legal rules
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ApplicationType {
    /// Applies only to future events
    Prospective,
    /// Applies to past events
    Retroactive,
    /// Hybrid: retroactive to a specific date
    Hybrid { retroactive_date: i64 },
}

/// Amendment to a legal rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amendment {
    /// Amendment ID
    pub id: String,
    /// Effective date
    pub effective_date: i64,
    /// Amended text
    pub amended_text: String,
    /// Reason for amendment
    pub reason: String,
}

impl Amendment {
    /// Creates a new amendment.
    pub fn new(
        id: impl Into<String>,
        effective_date: i64,
        amended_text: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            effective_date,
            amended_text: amended_text.into(),
            reason: reason.into(),
        }
    }
}

/// Legal change detection
pub struct LegalChangeDetector {
    /// Historical snapshots of legal rules
    snapshots: HashMap<String, Vec<LegalSnapshot>>,
}

impl LegalChangeDetector {
    /// Creates a new change detector.
    pub fn new() -> Self {
        Self {
            snapshots: HashMap::new(),
        }
    }

    /// Records a snapshot of a legal rule.
    pub fn record_snapshot(&mut self, rule_id: impl Into<String>, snapshot: LegalSnapshot) {
        self.snapshots
            .entry(rule_id.into())
            .or_default()
            .push(snapshot);
    }

    /// Detects changes between two time points.
    pub fn detect_changes(
        &self,
        rule_id: &str,
        from_time: i64,
        to_time: i64,
    ) -> Result<Vec<LegalChange>> {
        let snapshots = self
            .snapshots
            .get(rule_id)
            .ok_or_else(|| anyhow!("Rule not found: {}", rule_id))?;

        let before = snapshots
            .iter()
            .filter(|s| s.timestamp <= from_time)
            .max_by_key(|s| s.timestamp);

        let after = snapshots
            .iter()
            .filter(|s| s.timestamp <= to_time)
            .max_by_key(|s| s.timestamp);

        let mut changes = Vec::new();

        if let (Some(before), Some(after)) = (before, after) {
            if before.text != after.text {
                changes.push(LegalChange {
                    change_type: LegalChangeType::TextModified,
                    timestamp: after.timestamp,
                    description: "Rule text was modified".to_string(),
                    before_value: Some(before.text.clone()),
                    after_value: Some(after.text.clone()),
                });
            }

            if before.status != after.status {
                changes.push(LegalChange {
                    change_type: LegalChangeType::StatusChanged,
                    timestamp: after.timestamp,
                    description: format!(
                        "Status changed from {:?} to {:?}",
                        before.status, after.status
                    ),
                    before_value: Some(format!("{:?}", before.status)),
                    after_value: Some(format!("{:?}", after.status)),
                });
            }
        }

        Ok(changes)
    }

    /// Gets all snapshots for a rule.
    pub fn get_history(&self, rule_id: &str) -> Option<&Vec<LegalSnapshot>> {
        self.snapshots.get(rule_id)
    }
}

impl Default for LegalChangeDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of a legal rule at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalSnapshot {
    /// Timestamp
    pub timestamp: i64,
    /// Rule text at this time
    pub text: String,
    /// Status at this time
    pub status: RuleStatus,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl LegalSnapshot {
    /// Creates a new snapshot.
    pub fn new(timestamp: i64, text: impl Into<String>, status: RuleStatus) -> Self {
        Self {
            timestamp,
            text: text.into(),
            status,
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Status of a legal rule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleStatus {
    /// Active and enforceable
    Active,
    /// Suspended temporarily
    Suspended,
    /// Repealed
    Repealed,
    /// Superseded by another rule
    Superseded,
    /// Pending enactment
    Pending,
}

/// Detected legal change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalChange {
    /// Type of change
    pub change_type: LegalChangeType,
    /// When the change occurred
    pub timestamp: i64,
    /// Description
    pub description: String,
    /// Value before change
    pub before_value: Option<String>,
    /// Value after change
    pub after_value: Option<String>,
}

/// Type of legal change
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalChangeType {
    /// Rule text was modified
    TextModified,
    /// Rule status changed
    StatusChanged,
    /// Rule was added
    Added,
    /// Rule was removed
    Removed,
    /// Jurisdiction changed
    JurisdictionChanged,
}

/// Temporal conflict resolver
pub struct TemporalConflictResolver;

impl TemporalConflictResolver {
    /// Resolves conflicts between overlapping rules.
    pub fn resolve_conflicts(
        rules: &[TemporalLegalRule],
        timestamp: i64,
    ) -> Vec<ConflictResolution> {
        let mut resolutions = Vec::new();
        let valid_rules: Vec<&TemporalLegalRule> =
            rules.iter().filter(|r| r.is_valid_at(timestamp)).collect();

        for i in 0..valid_rules.len() {
            for j in (i + 1)..valid_rules.len() {
                if Self::rules_conflict(valid_rules[i], valid_rules[j]) {
                    resolutions.push(ConflictResolution {
                        rule1_id: valid_rules[i].id.clone(),
                        rule2_id: valid_rules[j].id.clone(),
                        resolution_strategy: ResolutionStrategy::LaterEnacted,
                        winner: if Self::is_later(valid_rules[i], valid_rules[j]) {
                            valid_rules[i].id.clone()
                        } else {
                            valid_rules[j].id.clone()
                        },
                    });
                }
            }
        }

        resolutions
    }

    fn rules_conflict(rule1: &TemporalLegalRule, rule2: &TemporalLegalRule) -> bool {
        // Simplified: same jurisdiction indicates potential conflict
        rule1.jurisdiction == rule2.jurisdiction
    }

    fn is_later(rule1: &TemporalLegalRule, rule2: &TemporalLegalRule) -> bool {
        let start1 = rule1.validity_periods.first().map(|p| p.start).unwrap_or(0);
        let start2 = rule2.validity_periods.first().map(|p| p.start).unwrap_or(0);
        start1 > start2
    }
}

/// Conflict resolution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolution {
    /// First rule ID
    pub rule1_id: String,
    /// Second rule ID
    pub rule2_id: String,
    /// Strategy used
    pub resolution_strategy: ResolutionStrategy,
    /// Winning rule ID
    pub winner: String,
}

/// Strategy for resolving temporal conflicts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    /// Later enacted rule prevails
    LaterEnacted,
    /// More specific rule prevails
    MoreSpecific,
    /// Higher authority prevails
    HigherAuthority,
}

/// Statute with sunset clause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunsetStatute {
    /// Rule
    pub rule: TemporalLegalRule,
    /// Sunset date (automatic expiration)
    pub sunset_date: i64,
    /// Whether sunset can be extended
    pub extendable: bool,
}

impl SunsetStatute {
    /// Creates a new sunset statute.
    pub fn new(rule: TemporalLegalRule, sunset_date: i64) -> Self {
        Self {
            rule,
            sunset_date,
            extendable: false,
        }
    }

    /// Sets extendability.
    pub fn with_extendable(mut self, extendable: bool) -> Self {
        self.extendable = extendable;
        self
    }

    /// Checks if the statute has expired.
    pub fn has_expired(&self, current_time: i64) -> bool {
        current_time > self.sunset_date
    }

    /// Extends the sunset date.
    pub fn extend_sunset(&mut self, new_sunset: i64) -> Result<()> {
        if !self.extendable {
            return Err(anyhow!("Statute is not extendable"));
        }
        self.sunset_date = new_sunset;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validity_period() {
        let period = ValidityPeriod::new(1000, "Enacted");

        assert!(period.is_valid_at(1500));
        assert!(!period.is_valid_at(500));

        let period_with_end = period.with_end(2000, "Repealed");
        assert!(period_with_end.is_valid_at(1500));
        assert!(!period_with_end.is_valid_at(2500));
    }

    #[test]
    fn test_validity_overlap() {
        let period1 = ValidityPeriod::new(1000, "Start").with_end(2000, "End");
        let period2 = ValidityPeriod::new(1500, "Start").with_end(2500, "End");
        let period3 = ValidityPeriod::new(2500, "Start");

        assert!(period1.overlaps(&period2));
        assert!(!period1.overlaps(&period3));
    }

    #[test]
    fn test_temporal_legal_rule() {
        let rule = TemporalLegalRule::new("rule1", "Original text", "US", 1000);

        assert!(rule.is_valid_at(1500));
        assert!(!rule.is_valid_at(500));
    }

    #[test]
    fn test_rule_with_amendment() {
        let mut rule = TemporalLegalRule::new("rule1", "Original text", "US", 1000);

        let amendment = Amendment::new("amend1", 1500, "Amended text", "Clarification");
        rule.add_amendment(amendment);

        assert_eq!(rule.get_text_at(1200), "Original text");
        assert_eq!(rule.get_text_at(1600), "Amended text");
    }

    #[test]
    fn test_prospective_application() {
        let rule = TemporalLegalRule::new("rule1", "Test", "US", 1000)
            .with_application_type(ApplicationType::Prospective);

        assert!(rule.applies_to_event_at(1500, 2000));
        assert!(!rule.applies_to_event_at(500, 2000));
    }

    #[test]
    fn test_retroactive_application() {
        let rule = TemporalLegalRule::new("rule1", "Test", "US", 1000)
            .with_application_type(ApplicationType::Retroactive);

        assert!(rule.applies_to_event_at(500, 2000));
        assert!(rule.applies_to_event_at(1500, 2000));
    }

    #[test]
    fn test_change_detector() {
        let mut detector = LegalChangeDetector::new();

        let snap1 = LegalSnapshot::new(1000, "Original", RuleStatus::Active);
        let snap2 = LegalSnapshot::new(2000, "Modified", RuleStatus::Active);

        detector.record_snapshot("rule1", snap1);
        detector.record_snapshot("rule1", snap2);

        let changes = detector.detect_changes("rule1", 1000, 2000).unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].change_type, LegalChangeType::TextModified);
    }

    #[test]
    fn test_conflict_resolution() {
        let rule1 = TemporalLegalRule::new("rule1", "Old rule", "US", 1000);
        let rule2 = TemporalLegalRule::new("rule2", "New rule", "US", 2000);

        let resolutions = TemporalConflictResolver::resolve_conflicts(&[rule1, rule2], 2500);
        assert!(!resolutions.is_empty());
        assert_eq!(resolutions[0].winner, "rule2");
    }

    #[test]
    fn test_sunset_statute() {
        let rule = TemporalLegalRule::new("rule1", "Temporary rule", "US", 1000);
        let sunset = SunsetStatute::new(rule, 2000);

        assert!(!sunset.has_expired(1500));
        assert!(sunset.has_expired(2500));
    }

    #[test]
    fn test_sunset_extension() {
        let rule = TemporalLegalRule::new("rule1", "Test", "US", 1000);
        let mut sunset = SunsetStatute::new(rule, 2000).with_extendable(true);

        sunset.extend_sunset(3000).unwrap();
        assert_eq!(sunset.sunset_date, 3000);
    }

    #[test]
    fn test_non_extendable_sunset() {
        let rule = TemporalLegalRule::new("rule1", "Test", "US", 1000);
        let mut sunset = SunsetStatute::new(rule, 2000);

        let result = sunset.extend_sunset(3000);
        assert!(result.is_err());
    }
}
