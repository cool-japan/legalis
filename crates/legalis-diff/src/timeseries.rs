//! Time-series analysis for statute changes over time.
//!
//! This module provides tools for analyzing patterns of changes across time:
//! - Change frequency analysis
//! - Trend detection
//! - Seasonal patterns
//! - Change velocity metrics
//! - Compliance drift tracking

use crate::{Severity, StatuteDiff};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A timestamped diff entry for time-series analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampedDiff {
    /// The diff itself
    pub diff: StatuteDiff,
    /// When this diff was created
    pub timestamp: DateTime<Utc>,
    /// Optional version identifier
    pub version: Option<String>,
    /// Optional author/contributor
    pub author: Option<String>,
}

impl TimestampedDiff {
    /// Creates a new timestamped diff with the current time.
    #[must_use]
    pub fn new(diff: StatuteDiff) -> Self {
        Self {
            diff,
            timestamp: Utc::now(),
            version: None,
            author: None,
        }
    }

    /// Creates a timestamped diff with a specific timestamp.
    #[must_use]
    pub fn with_timestamp(diff: StatuteDiff, timestamp: DateTime<Utc>) -> Self {
        Self {
            diff,
            timestamp,
            version: None,
            author: None,
        }
    }

    /// Sets the version identifier.
    #[must_use]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Sets the author.
    #[must_use]
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }
}

/// Time-series statistics for a collection of diffs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesStats {
    /// Total number of changes over time
    pub total_changes: usize,
    /// Changes per time period (day, week, month)
    pub changes_per_period: HashMap<String, usize>,
    /// Average changes per period
    pub avg_changes_per_period: f64,
    /// Peak change period
    pub peak_period: Option<(String, usize)>,
    /// Trend direction (positive = increasing, negative = decreasing)
    pub trend_slope: f64,
    /// Change velocity (changes per day)
    pub velocity: f64,
    /// Time span covered
    pub time_span_days: i64,
    /// Severity distribution over time
    pub severity_distribution: HashMap<Severity, usize>,
}

/// Change velocity metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeVelocity {
    /// Changes per day
    pub changes_per_day: f64,
    /// Changes per week
    pub changes_per_week: f64,
    /// Changes per month
    pub changes_per_month: f64,
    /// Acceleration (change in velocity over time)
    pub acceleration: f64,
    /// Is velocity increasing?
    pub is_accelerating: bool,
}

/// Compliance drift metrics tracking how statutes diverge from a baseline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceDrift {
    /// Baseline timestamp
    pub baseline_timestamp: DateTime<Utc>,
    /// Current timestamp
    pub current_timestamp: DateTime<Utc>,
    /// Total drift score (0.0 = no drift, higher = more drift)
    pub drift_score: f64,
    /// Number of breaking changes since baseline
    pub breaking_changes: usize,
    /// Number of non-breaking changes since baseline
    pub non_breaking_changes: usize,
    /// Drift velocity (drift per day)
    pub drift_velocity: f64,
    /// Is drift accelerating?
    pub is_accelerating: bool,
    /// Risk level based on drift
    pub risk_level: RiskLevel,
}

/// Risk level assessment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk (minimal drift)
    Low,
    /// Moderate risk (some drift detected)
    Moderate,
    /// High risk (significant drift)
    High,
    /// Critical risk (severe drift, immediate attention needed)
    Critical,
}

/// Analyzes time-series statistics from a collection of timestamped diffs.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, timeseries::{TimestampedDiff, analyze_time_series}};
/// use chrono::Utc;
///
/// let old = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
///
/// let diff_result = diff(&old, &new).unwrap();
/// let timestamped = TimestampedDiff::new(diff_result);
///
/// let stats = analyze_time_series(&[timestamped]);
/// assert!(stats.total_changes > 0);
/// ```
#[must_use]
pub fn analyze_time_series(diffs: &[TimestampedDiff]) -> TimeSeriesStats {
    if diffs.is_empty() {
        return TimeSeriesStats {
            total_changes: 0,
            changes_per_period: HashMap::new(),
            avg_changes_per_period: 0.0,
            peak_period: None,
            trend_slope: 0.0,
            velocity: 0.0,
            time_span_days: 0,
            severity_distribution: HashMap::new(),
        };
    }

    let total_changes: usize = diffs.iter().map(|d| d.diff.changes.len()).sum();

    // Calculate time span
    let timestamps: Vec<_> = diffs.iter().map(|d| d.timestamp).collect();
    let min_time = timestamps.iter().min().unwrap();
    let max_time = timestamps.iter().max().unwrap();
    let time_span = (*max_time - *min_time).num_days();

    // Group by day
    let mut changes_per_period = HashMap::new();
    for diff in diffs {
        let day_key = diff.timestamp.format("%Y-%m-%d").to_string();
        *changes_per_period.entry(day_key).or_insert(0) += diff.diff.changes.len();
    }

    // Calculate average and peak
    let avg_changes_per_period = if changes_per_period.is_empty() {
        0.0
    } else {
        total_changes as f64 / changes_per_period.len() as f64
    };

    let peak_period = changes_per_period
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(period, count)| (period.clone(), *count));

    // Simple linear trend (slope)
    let trend_slope = if time_span > 0 {
        total_changes as f64 / time_span as f64
    } else {
        0.0
    };

    // Velocity (changes per day)
    let velocity = if time_span > 0 {
        total_changes as f64 / time_span as f64
    } else if !diffs.is_empty() {
        total_changes as f64
    } else {
        0.0
    };

    // Severity distribution
    let mut severity_distribution = HashMap::new();
    for diff in diffs {
        *severity_distribution
            .entry(diff.diff.impact.severity)
            .or_insert(0) += 1;
    }

    TimeSeriesStats {
        total_changes,
        changes_per_period,
        avg_changes_per_period,
        peak_period,
        trend_slope,
        velocity,
        time_span_days: time_span,
        severity_distribution,
    }
}

/// Calculates change velocity metrics.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, timeseries::{TimestampedDiff, calculate_velocity}};
/// use chrono::{Utc, Duration};
///
/// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
///
/// let diff_result = diff(&old, &new).unwrap();
/// let now = Utc::now();
/// let diffs = vec![
///     TimestampedDiff::with_timestamp(diff_result.clone(), now - Duration::days(7)),
///     TimestampedDiff::with_timestamp(diff_result, now),
/// ];
///
/// let velocity = calculate_velocity(&diffs);
/// assert!(velocity.changes_per_week > 0.0);
/// ```
#[must_use]
pub fn calculate_velocity(diffs: &[TimestampedDiff]) -> ChangeVelocity {
    if diffs.is_empty() {
        return ChangeVelocity {
            changes_per_day: 0.0,
            changes_per_week: 0.0,
            changes_per_month: 0.0,
            acceleration: 0.0,
            is_accelerating: false,
        };
    }

    let total_changes: usize = diffs.iter().map(|d| d.diff.changes.len()).sum();
    let timestamps: Vec<_> = diffs.iter().map(|d| d.timestamp).collect();
    let min_time = timestamps.iter().min().unwrap();
    let max_time = timestamps.iter().max().unwrap();
    let time_span_days = (*max_time - *min_time).num_days().max(1);

    let changes_per_day = total_changes as f64 / time_span_days as f64;
    let changes_per_week = changes_per_day * 7.0;
    let changes_per_month = changes_per_day * 30.0;

    // Calculate acceleration by comparing first half vs second half
    let mid_time = *min_time + Duration::days(time_span_days / 2);
    let first_half: usize = diffs
        .iter()
        .filter(|d| d.timestamp < mid_time)
        .map(|d| d.diff.changes.len())
        .sum();
    let second_half: usize = diffs
        .iter()
        .filter(|d| d.timestamp >= mid_time)
        .map(|d| d.diff.changes.len())
        .sum();

    let first_half_days = time_span_days / 2;
    let second_half_days = time_span_days - first_half_days;

    let first_half_velocity = if first_half_days > 0 {
        first_half as f64 / first_half_days as f64
    } else {
        0.0
    };
    let second_half_velocity = if second_half_days > 0 {
        second_half as f64 / second_half_days as f64
    } else {
        0.0
    };

    let acceleration = second_half_velocity - first_half_velocity;
    let is_accelerating = acceleration > 0.0;

    ChangeVelocity {
        changes_per_day,
        changes_per_week,
        changes_per_month,
        acceleration,
        is_accelerating,
    }
}

/// Detects compliance drift from a baseline.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, timeseries::{TimestampedDiff, detect_compliance_drift}};
/// use chrono::{Utc, Duration};
///
/// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New", Effect::new(EffectType::Revoke, "Revoke"));
///
/// let diff_result = diff(&old, &new).unwrap();
/// let baseline = Utc::now() - Duration::days(30);
/// let diffs = vec![TimestampedDiff::with_timestamp(diff_result, Utc::now())];
///
/// let drift = detect_compliance_drift(&diffs, baseline);
/// assert!(drift.drift_score > 0.0);
/// ```
#[must_use]
pub fn detect_compliance_drift(
    diffs: &[TimestampedDiff],
    baseline_timestamp: DateTime<Utc>,
) -> ComplianceDrift {
    if diffs.is_empty() {
        return ComplianceDrift {
            baseline_timestamp,
            current_timestamp: Utc::now(),
            drift_score: 0.0,
            breaking_changes: 0,
            non_breaking_changes: 0,
            drift_velocity: 0.0,
            is_accelerating: false,
            risk_level: RiskLevel::Low,
        };
    }

    let current_timestamp = diffs
        .iter()
        .map(|d| d.timestamp)
        .max()
        .unwrap_or_else(Utc::now);

    let time_span_days = (current_timestamp - baseline_timestamp).num_days().max(1);

    // Count breaking vs non-breaking changes
    let mut breaking_changes = 0;
    let mut non_breaking_changes = 0;
    let mut total_severity_score = 0.0;

    for diff in diffs {
        if diff.timestamp >= baseline_timestamp {
            let severity_score = match diff.diff.impact.severity {
                Severity::None => 0.0,
                Severity::Minor => 1.0,
                Severity::Moderate => 2.0,
                Severity::Major => 4.0,
                Severity::Breaking => 8.0,
            };

            total_severity_score += severity_score * diff.diff.changes.len() as f64;

            if diff.diff.impact.severity >= Severity::Major {
                breaking_changes += 1;
            } else {
                non_breaking_changes += 1;
            }
        }
    }

    // Drift score: weighted by severity and number of changes
    let drift_score = total_severity_score;
    let drift_velocity = drift_score / time_span_days as f64;

    // Calculate if drifting is accelerating
    let mid_time = baseline_timestamp + Duration::days(time_span_days / 2);
    let first_half_score: f64 = diffs
        .iter()
        .filter(|d| d.timestamp >= baseline_timestamp && d.timestamp < mid_time)
        .map(|d| {
            let sev = match d.diff.impact.severity {
                Severity::None => 0.0,
                Severity::Minor => 1.0,
                Severity::Moderate => 2.0,
                Severity::Major => 4.0,
                Severity::Breaking => 8.0,
            };
            sev * d.diff.changes.len() as f64
        })
        .sum();

    let second_half_score: f64 = diffs
        .iter()
        .filter(|d| d.timestamp >= mid_time)
        .map(|d| {
            let sev = match d.diff.impact.severity {
                Severity::None => 0.0,
                Severity::Minor => 1.0,
                Severity::Moderate => 2.0,
                Severity::Major => 4.0,
                Severity::Breaking => 8.0,
            };
            sev * d.diff.changes.len() as f64
        })
        .sum();

    let is_accelerating = second_half_score > first_half_score;

    // Assess risk level
    let risk_level = if drift_score > 100.0 || breaking_changes > 10 {
        RiskLevel::Critical
    } else if drift_score > 50.0 || breaking_changes > 5 {
        RiskLevel::High
    } else if drift_score > 20.0 || breaking_changes > 2 {
        RiskLevel::Moderate
    } else {
        RiskLevel::Low
    };

    ComplianceDrift {
        baseline_timestamp,
        current_timestamp,
        drift_score,
        breaking_changes,
        non_breaking_changes,
        drift_velocity,
        is_accelerating,
        risk_level,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    #[test]
    fn test_timestamped_diff_creation() {
        let statute1 = Statute::new("test", "Title", Effect::new(EffectType::Grant, "Test"));
        let statute2 = Statute::new("test", "New Title", Effect::new(EffectType::Grant, "Test"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let timestamped = TimestampedDiff::new(diff_result.clone())
            .with_version("1.0.0")
            .with_author("Test Author");

        assert_eq!(timestamped.version, Some("1.0.0".to_string()));
        assert_eq!(timestamped.author, Some("Test Author".to_string()));
    }

    #[test]
    fn test_analyze_time_series_empty() {
        let stats = analyze_time_series(&[]);
        assert_eq!(stats.total_changes, 0);
        assert_eq!(stats.velocity, 0.0);
    }

    #[test]
    fn test_analyze_time_series_single() {
        let statute1 = Statute::new("test", "Title", Effect::new(EffectType::Grant, "Test"));
        let statute2 = Statute::new("test", "New Title", Effect::new(EffectType::Grant, "Test"));
        let diff_result = diff(&statute1, &statute2).unwrap();
        let timestamped = TimestampedDiff::new(diff_result);

        let stats = analyze_time_series(&[timestamped]);
        assert!(stats.total_changes > 0);
    }

    #[test]
    fn test_calculate_velocity() {
        let statute1 = Statute::new("test", "Title", Effect::new(EffectType::Grant, "Test"));
        let statute2 = Statute::new("test", "New Title", Effect::new(EffectType::Grant, "Test"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let now = Utc::now();
        let diffs = vec![
            TimestampedDiff::with_timestamp(diff_result.clone(), now - Duration::days(7)),
            TimestampedDiff::with_timestamp(diff_result, now),
        ];

        let velocity = calculate_velocity(&diffs);
        assert!(velocity.changes_per_day > 0.0);
        assert!(velocity.changes_per_week > 0.0);
        assert!(velocity.changes_per_month > 0.0);
    }

    #[test]
    fn test_detect_compliance_drift() {
        let statute1 = Statute::new("test", "Title", Effect::new(EffectType::Grant, "Test"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Revoke, "Revoke"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let baseline = Utc::now() - Duration::days(30);
        let diffs = vec![TimestampedDiff::with_timestamp(diff_result, Utc::now())];

        let drift = detect_compliance_drift(&diffs, baseline);
        assert!(drift.drift_score > 0.0);
        assert_eq!(drift.breaking_changes, 1);
    }

    #[test]
    fn test_risk_level_assessment() {
        let statute1 = Statute::new("test", "Title", Effect::new(EffectType::Grant, "Test"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Revoke, "Revoke"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let baseline = Utc::now() - Duration::days(1);
        let diffs = vec![TimestampedDiff::with_timestamp(diff_result, Utc::now())];

        let drift = detect_compliance_drift(&diffs, baseline);
        assert!(matches!(
            drift.risk_level,
            RiskLevel::Low | RiskLevel::Moderate
        ));
    }
}
