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

/// Comprehensive trend report combining all time-series analyses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendReport {
    /// Time-series statistics
    pub stats: TimeSeriesStats,
    /// Change velocity metrics
    pub velocity: ChangeVelocity,
    /// Compliance drift (if baseline provided)
    pub drift: Option<ComplianceDrift>,
    /// Report generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Time period covered
    pub period_start: DateTime<Utc>,
    /// Time period end
    pub period_end: DateTime<Utc>,
    /// Key insights and recommendations
    pub insights: Vec<String>,
    /// Warning flags
    pub warnings: Vec<String>,
}

impl TrendReport {
    /// Generates insights based on the trend data.
    fn generate_insights(&self) -> Vec<String> {
        let mut insights = Vec::new();

        // Velocity insights
        if self.velocity.is_accelerating {
            insights.push(format!(
                "Change velocity is accelerating ({:.2} changes/day acceleration)",
                self.velocity.acceleration
            ));
        } else {
            insights.push(format!(
                "Change velocity is decelerating ({:.2} changes/day deceleration)",
                self.velocity.acceleration.abs()
            ));
        }

        // Trend insights
        if self.stats.trend_slope > 0.0 {
            insights.push(format!(
                "Positive change trend detected ({:.2} changes/day slope)",
                self.stats.trend_slope
            ));
        } else if self.stats.trend_slope < 0.0 {
            insights.push(format!(
                "Negative change trend detected ({:.2} changes/day slope)",
                self.stats.trend_slope.abs()
            ));
        }

        // Peak period insight
        if let Some((period, count)) = &self.stats.peak_period {
            insights.push(format!(
                "Peak change period: {} with {} changes",
                period, count
            ));
        }

        // Compliance drift insights
        if let Some(drift) = &self.drift {
            match drift.risk_level {
                RiskLevel::Critical => {
                    insights.push(format!(
                        "CRITICAL: Severe compliance drift detected (score: {:.2})",
                        drift.drift_score
                    ));
                }
                RiskLevel::High => {
                    insights.push(format!(
                        "HIGH: Significant compliance drift detected (score: {:.2})",
                        drift.drift_score
                    ));
                }
                RiskLevel::Moderate => {
                    insights.push(format!(
                        "MODERATE: Some compliance drift detected (score: {:.2})",
                        drift.drift_score
                    ));
                }
                RiskLevel::Low => {
                    insights.push("LOW: Minimal compliance drift".to_string());
                }
            }

            if drift.is_accelerating {
                insights.push("Compliance drift is accelerating".to_string());
            }
        }

        insights
    }

    /// Generates warnings based on the trend data.
    fn generate_warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();

        // High velocity warning
        if self.velocity.changes_per_day > 10.0 {
            warnings.push(format!(
                "High change velocity detected: {:.2} changes/day",
                self.velocity.changes_per_day
            ));
        }

        // Acceleration warning
        if self.velocity.is_accelerating && self.velocity.acceleration > 5.0 {
            warnings.push(format!(
                "Rapid acceleration in changes: {:.2} changes/day increase",
                self.velocity.acceleration
            ));
        }

        // Drift warnings
        if let Some(drift) = &self.drift {
            if drift.breaking_changes > 5 {
                warnings.push(format!(
                    "High number of breaking changes: {}",
                    drift.breaking_changes
                ));
            }

            if matches!(drift.risk_level, RiskLevel::High | RiskLevel::Critical) {
                warnings.push("Immediate attention required for compliance drift".to_string());
            }

            if drift.drift_velocity > 5.0 {
                warnings.push(format!(
                    "High drift velocity: {:.2} drift units/day",
                    drift.drift_velocity
                ));
            }
        }

        warnings
    }

    /// Exports the trend report to JSON format.
    #[must_use]
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }

    /// Exports the trend report to Markdown format.
    #[must_use]
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Statute Change Trend Report\n\n");
        md.push_str(&format!(
            "**Generated:** {}\n\n",
            self.generated_at.format("%Y-%m-%d %H:%M:%S UTC")
        ));
        md.push_str(&format!(
            "**Period:** {} to {}\n\n",
            self.period_start.format("%Y-%m-%d"),
            self.period_end.format("%Y-%m-%d")
        ));

        // Summary Statistics
        md.push_str("## Summary Statistics\n\n");
        md.push_str(&format!(
            "- **Total Changes:** {}\n",
            self.stats.total_changes
        ));
        md.push_str(&format!(
            "- **Time Span:** {} days\n",
            self.stats.time_span_days
        ));
        md.push_str(&format!(
            "- **Average Changes per Period:** {:.2}\n",
            self.stats.avg_changes_per_period
        ));
        md.push_str(&format!(
            "- **Trend Slope:** {:.2} changes/day\n",
            self.stats.trend_slope
        ));
        md.push_str(&format!(
            "- **Velocity:** {:.2} changes/day\n\n",
            self.stats.velocity
        ));

        // Velocity Metrics
        md.push_str("## Change Velocity\n\n");
        md.push_str(&format!(
            "- **Changes per Day:** {:.2}\n",
            self.velocity.changes_per_day
        ));
        md.push_str(&format!(
            "- **Changes per Week:** {:.2}\n",
            self.velocity.changes_per_week
        ));
        md.push_str(&format!(
            "- **Changes per Month:** {:.2}\n",
            self.velocity.changes_per_month
        ));
        md.push_str(&format!(
            "- **Acceleration:** {:.2} changes/day²\n",
            self.velocity.acceleration
        ));
        md.push_str(&format!(
            "- **Status:** {}\n\n",
            if self.velocity.is_accelerating {
                "Accelerating"
            } else {
                "Decelerating"
            }
        ));

        // Compliance Drift
        if let Some(drift) = &self.drift {
            md.push_str("## Compliance Drift\n\n");
            md.push_str(&format!(
                "- **Baseline:** {}\n",
                drift.baseline_timestamp.format("%Y-%m-%d")
            ));
            md.push_str(&format!("- **Drift Score:** {:.2}\n", drift.drift_score));
            md.push_str(&format!(
                "- **Breaking Changes:** {}\n",
                drift.breaking_changes
            ));
            md.push_str(&format!(
                "- **Non-Breaking Changes:** {}\n",
                drift.non_breaking_changes
            ));
            md.push_str(&format!(
                "- **Drift Velocity:** {:.2} units/day\n",
                drift.drift_velocity
            ));
            md.push_str(&format!("- **Risk Level:** {:?}\n", drift.risk_level));
            md.push_str(&format!(
                "- **Status:** {}\n\n",
                if drift.is_accelerating {
                    "Accelerating"
                } else {
                    "Stable"
                }
            ));
        }

        // Severity Distribution
        if !self.stats.severity_distribution.is_empty() {
            md.push_str("## Severity Distribution\n\n");
            for (severity, count) in &self.stats.severity_distribution {
                md.push_str(&format!("- **{:?}:** {}\n", severity, count));
            }
            md.push_str("\n");
        }

        // Insights
        if !self.insights.is_empty() {
            md.push_str("## Key Insights\n\n");
            for insight in &self.insights {
                md.push_str(&format!("- {}\n", insight));
            }
            md.push_str("\n");
        }

        // Warnings
        if !self.warnings.is_empty() {
            md.push_str("## Warnings\n\n");
            for warning in &self.warnings {
                md.push_str(&format!("- ⚠️ {}\n", warning));
            }
            md.push_str("\n");
        }

        md
    }

    /// Exports the trend report to CSV format.
    #[must_use]
    pub fn to_csv(&self) -> String {
        let mut csv = String::new();

        // Header
        csv.push_str("Metric,Value\n");

        // Basic stats
        csv.push_str(&format!(
            "Generated At,{}\n",
            self.generated_at.format("%Y-%m-%d %H:%M:%S UTC")
        ));
        csv.push_str(&format!(
            "Period Start,{}\n",
            self.period_start.format("%Y-%m-%d")
        ));
        csv.push_str(&format!(
            "Period End,{}\n",
            self.period_end.format("%Y-%m-%d")
        ));
        csv.push_str(&format!("Total Changes,{}\n", self.stats.total_changes));
        csv.push_str(&format!("Time Span Days,{}\n", self.stats.time_span_days));
        csv.push_str(&format!(
            "Avg Changes Per Period,{:.2}\n",
            self.stats.avg_changes_per_period
        ));
        csv.push_str(&format!("Trend Slope,{:.2}\n", self.stats.trend_slope));
        csv.push_str(&format!("Velocity,{:.2}\n", self.stats.velocity));

        // Velocity metrics
        csv.push_str(&format!(
            "Changes Per Day,{:.2}\n",
            self.velocity.changes_per_day
        ));
        csv.push_str(&format!(
            "Changes Per Week,{:.2}\n",
            self.velocity.changes_per_week
        ));
        csv.push_str(&format!(
            "Changes Per Month,{:.2}\n",
            self.velocity.changes_per_month
        ));
        csv.push_str(&format!("Acceleration,{:.2}\n", self.velocity.acceleration));
        csv.push_str(&format!(
            "Is Accelerating,{}\n",
            self.velocity.is_accelerating
        ));

        // Drift metrics
        if let Some(drift) = &self.drift {
            csv.push_str(&format!("Drift Score,{:.2}\n", drift.drift_score));
            csv.push_str(&format!("Breaking Changes,{}\n", drift.breaking_changes));
            csv.push_str(&format!(
                "Non-Breaking Changes,{}\n",
                drift.non_breaking_changes
            ));
            csv.push_str(&format!("Drift Velocity,{:.2}\n", drift.drift_velocity));
            csv.push_str(&format!("Drift Accelerating,{}\n", drift.is_accelerating));
            csv.push_str(&format!("Risk Level,{:?}\n", drift.risk_level));
        }

        csv
    }
}

/// Generates a comprehensive trend report from timestamped diffs.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, timeseries::{TimestampedDiff, generate_trend_report}};
/// use chrono::{Utc, Duration};
///
/// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
/// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Different"));
///
/// let diff_result = diff(&old, &new).unwrap();
/// let now = Utc::now();
/// let diffs = vec![
///     TimestampedDiff::with_timestamp(diff_result.clone(), now - Duration::days(30)),
///     TimestampedDiff::with_timestamp(diff_result, now),
/// ];
///
/// let baseline = now - Duration::days(60);
/// let report = generate_trend_report(&diffs, Some(baseline));
///
/// assert!(!report.insights.is_empty());
/// ```
#[must_use]
pub fn generate_trend_report(
    diffs: &[TimestampedDiff],
    baseline_timestamp: Option<DateTime<Utc>>,
) -> TrendReport {
    let stats = analyze_time_series(diffs);
    let velocity = calculate_velocity(diffs);
    let drift = baseline_timestamp.map(|baseline| detect_compliance_drift(diffs, baseline));

    let generated_at = Utc::now();
    let period_start = diffs
        .iter()
        .map(|d| d.timestamp)
        .min()
        .unwrap_or(generated_at);
    let period_end = diffs
        .iter()
        .map(|d| d.timestamp)
        .max()
        .unwrap_or(generated_at);

    let mut report = TrendReport {
        stats,
        velocity,
        drift,
        generated_at,
        period_start,
        period_end,
        insights: Vec::new(),
        warnings: Vec::new(),
    };

    report.insights = report.generate_insights();
    report.warnings = report.generate_warnings();

    report
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

    #[test]
    fn test_generate_trend_report() {
        let statute1 = Statute::new("test", "Title", Effect::new(EffectType::Grant, "Test"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Grant, "Different"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let now = Utc::now();
        let diffs = vec![
            TimestampedDiff::with_timestamp(diff_result.clone(), now - Duration::days(30)),
            TimestampedDiff::with_timestamp(diff_result, now),
        ];

        let baseline = now - Duration::days(60);
        let report = generate_trend_report(&diffs, Some(baseline));

        assert!(!report.insights.is_empty());
        assert!(report.stats.total_changes > 0);
        assert!(report.drift.is_some());
    }

    #[test]
    fn test_trend_report_to_json() {
        let statute1 = Statute::new("test", "Title", Effect::new(EffectType::Grant, "Test"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Grant, "Different"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let now = Utc::now();
        let diffs = vec![TimestampedDiff::with_timestamp(diff_result, now)];

        let report = generate_trend_report(&diffs, None);
        let json = report.to_json();

        assert!(json.contains("stats"));
        assert!(json.contains("velocity"));
    }

    #[test]
    fn test_trend_report_to_markdown() {
        let statute1 = Statute::new("test", "Title", Effect::new(EffectType::Grant, "Test"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Grant, "Different"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let now = Utc::now();
        let diffs = vec![TimestampedDiff::with_timestamp(diff_result, now)];

        let report = generate_trend_report(&diffs, None);
        let md = report.to_markdown();

        assert!(md.contains("# Statute Change Trend Report"));
        assert!(md.contains("## Summary Statistics"));
        assert!(md.contains("## Change Velocity"));
        assert!(md.contains("## Key Insights"));
    }

    #[test]
    fn test_trend_report_to_csv() {
        let statute1 = Statute::new("test", "Title", Effect::new(EffectType::Grant, "Test"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Grant, "Different"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let now = Utc::now();
        let diffs = vec![TimestampedDiff::with_timestamp(diff_result, now)];

        let report = generate_trend_report(&diffs, None);
        let csv = report.to_csv();

        assert!(csv.contains("Metric,Value"));
        assert!(csv.contains("Total Changes"));
        assert!(csv.contains("Changes Per Day"));
    }

    #[test]
    fn test_trend_report_warnings() {
        let statute1 = Statute::new("test", "Title", Effect::new(EffectType::Grant, "Test"));
        let statute2 = Statute::new("test", "New", Effect::new(EffectType::Revoke, "Revoke"));
        let diff_result = diff(&statute1, &statute2).unwrap();

        let now = Utc::now();
        // Create many diffs to trigger warnings
        let mut diffs = Vec::new();
        for i in 0..20 {
            diffs.push(TimestampedDiff::with_timestamp(
                diff_result.clone(),
                now - Duration::days(20 - i),
            ));
        }

        let baseline = now - Duration::days(30);
        let report = generate_trend_report(&diffs, Some(baseline));

        // Should have warnings about high change velocity
        assert!(!report.warnings.is_empty());
    }
}
