//! Comparison reports for audit trail analytics.
//!
//! This module provides comparison reporting capabilities:
//! - Month-over-month comparisons
//! - Year-over-year comparisons
//! - Period-to-period analysis
//! - Trend detection

use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Time period for comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonPeriod {
    /// Compare by day
    Daily,
    /// Compare by week
    Weekly,
    /// Compare by month
    Monthly,
    /// Compare by quarter
    Quarterly,
    /// Compare by year
    Yearly,
}

/// Comparison report between two time periods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonReport {
    /// Period type
    pub period: ComparisonPeriod,
    /// Current period label
    pub current_period: String,
    /// Previous period label
    pub previous_period: String,
    /// Metrics for current period
    pub current_metrics: PeriodMetrics,
    /// Metrics for previous period
    pub previous_metrics: PeriodMetrics,
    /// Changes between periods
    pub changes: MetricChanges,
}

/// Metrics for a specific time period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodMetrics {
    /// Total decisions
    pub total_decisions: usize,
    /// Automatic decisions
    pub automatic_decisions: usize,
    /// Discretionary reviews
    pub discretionary_reviews: usize,
    /// Human overrides
    pub human_overrides: usize,
    /// Appeals
    pub appeals: usize,
    /// Unique statutes used
    pub unique_statutes: usize,
    /// Unique subjects
    pub unique_subjects: usize,
    /// Decisions by statute
    pub by_statute: HashMap<String, usize>,
    /// Decisions by day
    pub by_day: HashMap<String, usize>,
}

impl PeriodMetrics {
    /// Creates new empty metrics.
    pub fn new() -> Self {
        Self {
            total_decisions: 0,
            automatic_decisions: 0,
            discretionary_reviews: 0,
            human_overrides: 0,
            appeals: 0,
            unique_statutes: 0,
            unique_subjects: 0,
            by_statute: HashMap::new(),
            by_day: HashMap::new(),
        }
    }

    /// Computes metrics from audit records.
    pub fn from_records(records: &[AuditRecord]) -> Self {
        let mut metrics = Self::new();
        let mut statutes = std::collections::HashSet::new();
        let mut subjects = std::collections::HashSet::new();

        for record in records {
            metrics.total_decisions += 1;

            match record.event_type {
                crate::EventType::AutomaticDecision => metrics.automatic_decisions += 1,
                crate::EventType::DiscretionaryReview => metrics.discretionary_reviews += 1,
                crate::EventType::HumanOverride => metrics.human_overrides += 1,
                crate::EventType::Appeal => metrics.appeals += 1,
                _ => {}
            }

            statutes.insert(record.statute_id.clone());
            subjects.insert(record.subject_id);

            *metrics
                .by_statute
                .entry(record.statute_id.clone())
                .or_insert(0) += 1;

            let day_key = format!(
                "{}-{:02}-{:02}",
                record.timestamp.year(),
                record.timestamp.month(),
                record.timestamp.day()
            );
            *metrics.by_day.entry(day_key).or_insert(0) += 1;
        }

        metrics.unique_statutes = statutes.len();
        metrics.unique_subjects = subjects.len();

        metrics
    }

    /// Gets the average decisions per day.
    pub fn avg_per_day(&self) -> f64 {
        if self.by_day.is_empty() {
            0.0
        } else {
            self.total_decisions as f64 / self.by_day.len() as f64
        }
    }
}

impl Default for PeriodMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Changes between two periods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricChanges {
    /// Total decisions change (absolute)
    pub total_decisions_change: i64,
    /// Total decisions change (percentage)
    pub total_decisions_change_pct: f64,
    /// Automatic decisions change
    pub automatic_decisions_change: i64,
    /// Automatic decisions change (percentage)
    pub automatic_decisions_change_pct: f64,
    /// Discretionary reviews change
    pub discretionary_reviews_change: i64,
    /// Human overrides change
    pub human_overrides_change: i64,
    /// Appeals change
    pub appeals_change: i64,
    /// Unique statutes change
    pub unique_statutes_change: i64,
    /// Unique subjects change
    pub unique_subjects_change: i64,
    /// Average per day change
    pub avg_per_day_change: f64,
    /// Top growing statutes (by absolute count)
    pub top_growing_statutes: Vec<(String, i64)>,
    /// Top declining statutes (by absolute count)
    pub top_declining_statutes: Vec<(String, i64)>,
}

impl MetricChanges {
    /// Computes changes between two periods.
    pub fn from_periods(current: &PeriodMetrics, previous: &PeriodMetrics) -> Self {
        let total_change = current.total_decisions as i64 - previous.total_decisions as i64;
        let total_pct = if previous.total_decisions > 0 {
            (total_change as f64 / previous.total_decisions as f64) * 100.0
        } else {
            0.0
        };

        let auto_change = current.automatic_decisions as i64 - previous.automatic_decisions as i64;
        let auto_pct = if previous.automatic_decisions > 0 {
            (auto_change as f64 / previous.automatic_decisions as f64) * 100.0
        } else {
            0.0
        };

        // Compute statute changes
        let mut statute_changes: HashMap<String, i64> = HashMap::new();
        for (statute, count) in &current.by_statute {
            let prev_count = previous.by_statute.get(statute).copied().unwrap_or(0);
            let change = *count as i64 - prev_count as i64;
            statute_changes.insert(statute.clone(), change);
        }
        for (statute, count) in &previous.by_statute {
            if !current.by_statute.contains_key(statute) {
                statute_changes.insert(statute.clone(), -(*count as i64));
            }
        }

        let mut growing: Vec<_> = statute_changes
            .iter()
            .filter(|&(_, &change)| change > 0)
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        growing.sort_by(|a, b| b.1.cmp(&a.1));
        let top_growing = growing.into_iter().take(5).collect();

        let mut declining: Vec<_> = statute_changes
            .iter()
            .filter(|&(_, &change)| change < 0)
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        declining.sort_by(|a, b| a.1.cmp(&b.1));
        let top_declining = declining.into_iter().take(5).collect();

        Self {
            total_decisions_change: total_change,
            total_decisions_change_pct: total_pct,
            automatic_decisions_change: auto_change,
            automatic_decisions_change_pct: auto_pct,
            discretionary_reviews_change: current.discretionary_reviews as i64
                - previous.discretionary_reviews as i64,
            human_overrides_change: current.human_overrides as i64
                - previous.human_overrides as i64,
            appeals_change: current.appeals as i64 - previous.appeals as i64,
            unique_statutes_change: current.unique_statutes as i64
                - previous.unique_statutes as i64,
            unique_subjects_change: current.unique_subjects as i64
                - previous.unique_subjects as i64,
            avg_per_day_change: current.avg_per_day() - previous.avg_per_day(),
            top_growing_statutes: top_growing,
            top_declining_statutes: top_declining,
        }
    }
}

/// Comparison report generator.
pub struct ComparisonGenerator;

impl ComparisonGenerator {
    /// Generates a month-over-month comparison report.
    pub fn month_over_month(
        records: &[AuditRecord],
        reference_date: DateTime<Utc>,
    ) -> AuditResult<ComparisonReport> {
        let current_month_start = reference_date
            .with_day(1)
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap();

        let current_month_end = if current_month_start.month() == 12 {
            current_month_start
                .with_year(current_month_start.year() + 1)
                .unwrap()
                .with_month(1)
                .unwrap()
        } else {
            current_month_start
                .with_month(current_month_start.month() + 1)
                .unwrap()
        };

        let previous_month_start = if current_month_start.month() == 1 {
            current_month_start
                .with_year(current_month_start.year() - 1)
                .unwrap()
                .with_month(12)
                .unwrap()
        } else {
            current_month_start
                .with_month(current_month_start.month() - 1)
                .unwrap()
        };

        let current_records: Vec<_> = records
            .iter()
            .filter(|r| r.timestamp >= current_month_start && r.timestamp < current_month_end)
            .cloned()
            .collect();

        let previous_records: Vec<_> = records
            .iter()
            .filter(|r| r.timestamp >= previous_month_start && r.timestamp < current_month_start)
            .cloned()
            .collect();

        let current_metrics = PeriodMetrics::from_records(&current_records);
        let previous_metrics = PeriodMetrics::from_records(&previous_records);
        let changes = MetricChanges::from_periods(&current_metrics, &previous_metrics);

        Ok(ComparisonReport {
            period: ComparisonPeriod::Monthly,
            current_period: format!(
                "{}-{:02}",
                current_month_start.year(),
                current_month_start.month()
            ),
            previous_period: format!(
                "{}-{:02}",
                previous_month_start.year(),
                previous_month_start.month()
            ),
            current_metrics,
            previous_metrics,
            changes,
        })
    }

    /// Generates a year-over-year comparison report.
    pub fn year_over_year(
        records: &[AuditRecord],
        reference_date: DateTime<Utc>,
    ) -> AuditResult<ComparisonReport> {
        let current_year_start = reference_date
            .with_month(1)
            .unwrap()
            .with_day(1)
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap();

        let current_year_end = current_year_start
            .with_year(current_year_start.year() + 1)
            .unwrap();

        let previous_year_start = current_year_start
            .with_year(current_year_start.year() - 1)
            .unwrap();

        let current_records: Vec<_> = records
            .iter()
            .filter(|r| r.timestamp >= current_year_start && r.timestamp < current_year_end)
            .cloned()
            .collect();

        let previous_records: Vec<_> = records
            .iter()
            .filter(|r| r.timestamp >= previous_year_start && r.timestamp < current_year_start)
            .cloned()
            .collect();

        let current_metrics = PeriodMetrics::from_records(&current_records);
        let previous_metrics = PeriodMetrics::from_records(&previous_records);
        let changes = MetricChanges::from_periods(&current_metrics, &previous_metrics);

        Ok(ComparisonReport {
            period: ComparisonPeriod::Yearly,
            current_period: current_year_start.year().to_string(),
            previous_period: previous_year_start.year().to_string(),
            current_metrics,
            previous_metrics,
            changes,
        })
    }

    /// Generates a custom period comparison report.
    pub fn custom_period(
        records: &[AuditRecord],
        current_start: DateTime<Utc>,
        current_end: DateTime<Utc>,
        previous_start: DateTime<Utc>,
        previous_end: DateTime<Utc>,
    ) -> AuditResult<ComparisonReport> {
        let current_records: Vec<_> = records
            .iter()
            .filter(|r| r.timestamp >= current_start && r.timestamp < current_end)
            .cloned()
            .collect();

        let previous_records: Vec<_> = records
            .iter()
            .filter(|r| r.timestamp >= previous_start && r.timestamp < previous_end)
            .cloned()
            .collect();

        let current_metrics = PeriodMetrics::from_records(&current_records);
        let previous_metrics = PeriodMetrics::from_records(&previous_records);
        let changes = MetricChanges::from_periods(&current_metrics, &previous_metrics);

        Ok(ComparisonReport {
            period: ComparisonPeriod::Daily, // Generic for custom
            current_period: format!(
                "{} to {}",
                current_start.format("%Y-%m-%d"),
                current_end.format("%Y-%m-%d")
            ),
            previous_period: format!(
                "{} to {}",
                previous_start.format("%Y-%m-%d"),
                previous_end.format("%Y-%m-%d")
            ),
            current_metrics,
            previous_metrics,
            changes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use chrono::{Duration, TimeZone};
    use std::collections::HashMap as StdHashMap;
    use uuid::Uuid;

    fn create_records_for_month(year: i32, month: u32, count: usize) -> Vec<AuditRecord> {
        let mut records = Vec::new();
        let start = chrono::Utc
            .with_ymd_and_hms(year, month, 1, 0, 0, 0)
            .unwrap();

        for i in 0..count {
            let timestamp = start + Duration::days(i as i64 % 28);
            let mut record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                format!("statute-{}", i % 3),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "approved".to_string(),
                    parameters: StdHashMap::new(),
                },
                None,
            );
            record.timestamp = timestamp;
            records.push(record);
        }

        records
    }

    #[test]
    fn test_period_metrics() {
        let records = create_records_for_month(2024, 1, 10);
        let metrics = PeriodMetrics::from_records(&records);

        assert_eq!(metrics.total_decisions, 10);
        assert_eq!(metrics.automatic_decisions, 10);
        assert_eq!(metrics.unique_statutes, 3);
    }

    #[test]
    fn test_month_over_month() {
        let mut records = Vec::new();
        records.extend(create_records_for_month(2024, 1, 10));
        records.extend(create_records_for_month(2024, 2, 15));

        let reference = chrono::Utc.with_ymd_and_hms(2024, 2, 15, 0, 0, 0).unwrap();
        let report = ComparisonGenerator::month_over_month(&records, reference).unwrap();

        assert_eq!(report.current_period, "2024-02");
        assert_eq!(report.previous_period, "2024-01");
        assert_eq!(report.current_metrics.total_decisions, 15);
        assert_eq!(report.previous_metrics.total_decisions, 10);
        assert_eq!(report.changes.total_decisions_change, 5);
        assert_eq!(report.changes.total_decisions_change_pct, 50.0);
    }

    #[test]
    fn test_year_over_year() {
        let mut records = Vec::new();
        records.extend(create_records_for_month(2023, 1, 100));
        records.extend(create_records_for_month(2024, 1, 150));

        let reference = chrono::Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();
        let report = ComparisonGenerator::year_over_year(&records, reference).unwrap();

        assert_eq!(report.current_period, "2024");
        assert_eq!(report.previous_period, "2023");
        assert_eq!(report.current_metrics.total_decisions, 150);
        assert_eq!(report.previous_metrics.total_decisions, 100);
        assert_eq!(report.changes.total_decisions_change, 50);
    }

    #[test]
    fn test_metric_changes() {
        let current = PeriodMetrics {
            total_decisions: 150,
            automatic_decisions: 120,
            discretionary_reviews: 20,
            human_overrides: 10,
            appeals: 0,
            unique_statutes: 5,
            unique_subjects: 100,
            by_statute: HashMap::new(),
            by_day: HashMap::new(),
        };

        let previous = PeriodMetrics {
            total_decisions: 100,
            automatic_decisions: 80,
            discretionary_reviews: 15,
            human_overrides: 5,
            appeals: 0,
            unique_statutes: 4,
            unique_subjects: 80,
            by_statute: HashMap::new(),
            by_day: HashMap::new(),
        };

        let changes = MetricChanges::from_periods(&current, &previous);

        assert_eq!(changes.total_decisions_change, 50);
        assert_eq!(changes.total_decisions_change_pct, 50.0);
        assert_eq!(changes.automatic_decisions_change, 40);
        assert_eq!(changes.automatic_decisions_change_pct, 50.0);
        assert_eq!(changes.discretionary_reviews_change, 5);
        assert_eq!(changes.human_overrides_change, 5);
    }
}
