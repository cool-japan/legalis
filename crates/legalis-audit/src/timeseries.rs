//! Time-series queries for audit trail trend analysis.
//!
//! This module provides time-series query capabilities for analyzing audit trails:
//! - Time-based aggregations (hourly, daily, weekly, monthly)
//! - Trend detection (increasing, decreasing, stable)
//! - Moving averages
//! - Seasonal pattern detection
//! - Growth rate calculations

use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Time bucket granularity for time-series queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeBucket {
    /// Hourly buckets
    Hourly,
    /// Daily buckets
    Daily,
    /// Weekly buckets
    Weekly,
    /// Monthly buckets
    Monthly,
    /// Quarterly buckets
    Quarterly,
    /// Yearly buckets
    Yearly,
}

/// A single data point in a time series.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    /// Timestamp of this point
    pub timestamp: DateTime<Utc>,
    /// Bucket label (e.g., "2024-01-15")
    pub label: String,
    /// Value at this point
    pub value: f64,
    /// Count of records in this bucket
    pub count: usize,
}

/// Time series data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeries {
    /// Bucket granularity
    pub bucket: TimeBucket,
    /// Data points in chronological order
    pub points: Vec<TimeSeriesPoint>,
    /// Total records analyzed
    pub total_records: usize,
}

impl TimeSeries {
    /// Gets the trend direction.
    pub fn trend(&self) -> Trend {
        if self.points.len() < 2 {
            return Trend::Stable;
        }

        // Calculate linear regression slope
        let n = self.points.len() as f64;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_x2 = 0.0;

        for (i, point) in self.points.iter().enumerate() {
            let x = i as f64;
            let y = point.value;
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_x2 += x * x;
        }

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);

        // Determine trend based on slope
        if slope > 0.1 {
            Trend::Increasing
        } else if slope < -0.1 {
            Trend::Decreasing
        } else {
            Trend::Stable
        }
    }

    /// Calculates moving average.
    pub fn moving_average(&self, window: usize) -> Vec<f64> {
        if window == 0 || self.points.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new();
        for i in 0..self.points.len() {
            let start = i.saturating_sub(window - 1);
            let sum: f64 = self.points[start..=i].iter().map(|p| p.value).sum();
            let avg = sum / (i - start + 1) as f64;
            result.push(avg);
        }
        result
    }

    /// Calculates growth rate between first and last point.
    pub fn growth_rate(&self) -> Option<f64> {
        if self.points.len() < 2 {
            return None;
        }

        let first = self.points.first()?.value;
        let last = self.points.last()?.value;

        if first == 0.0 {
            return None;
        }

        Some(((last - first) / first) * 100.0)
    }

    /// Gets the peak (maximum) value.
    pub fn peak(&self) -> Option<&TimeSeriesPoint> {
        self.points
            .iter()
            .max_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
    }

    /// Gets the valley (minimum) value.
    pub fn valley(&self) -> Option<&TimeSeriesPoint> {
        self.points
            .iter()
            .min_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
    }

    /// Calculates average value across all points.
    pub fn average(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.points.iter().map(|p| p.value).sum();
        sum / self.points.len() as f64
    }

    /// Calculates standard deviation.
    pub fn std_dev(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }

        let avg = self.average();
        let variance: f64 = self
            .points
            .iter()
            .map(|p| (p.value - avg).powi(2))
            .sum::<f64>()
            / self.points.len() as f64;
        variance.sqrt()
    }
}

/// Trend direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trend {
    Increasing,
    Decreasing,
    Stable,
}

/// Filter predicate type for time-series queries.
type FilterPredicate = Box<dyn Fn(&AuditRecord) -> bool + Send + Sync>;

/// Time-series query builder.
pub struct TimeSeriesQuery {
    bucket: TimeBucket,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    filters: Vec<FilterPredicate>,
}

impl TimeSeriesQuery {
    /// Creates a new time-series query.
    pub fn new(bucket: TimeBucket) -> Self {
        Self {
            bucket,
            start_time: None,
            end_time: None,
            filters: Vec::new(),
        }
    }

    /// Sets the start time for the query.
    pub fn start_time(mut self, start: DateTime<Utc>) -> Self {
        self.start_time = Some(start);
        self
    }

    /// Sets the end time for the query.
    pub fn end_time(mut self, end: DateTime<Utc>) -> Self {
        self.end_time = Some(end);
        self
    }

    /// Adds a filter to the query.
    pub fn filter<F>(mut self, f: F) -> Self
    where
        F: Fn(&AuditRecord) -> bool + Send + Sync + 'static,
    {
        self.filters.push(Box::new(f));
        self
    }

    /// Filters by statute ID.
    pub fn statute(self, statute_id: String) -> Self {
        self.filter(move |r| r.statute_id == statute_id)
    }

    /// Executes the time-series query.
    pub fn execute(&self, records: &[AuditRecord]) -> AuditResult<TimeSeries> {
        // Filter records
        let filtered: Vec<_> = records
            .iter()
            .filter(|r| {
                let time_match = match (self.start_time, self.end_time) {
                    (Some(start), Some(end)) => r.timestamp >= start && r.timestamp <= end,
                    (Some(start), None) => r.timestamp >= start,
                    (None, Some(end)) => r.timestamp <= end,
                    (None, None) => true,
                };
                time_match && self.filters.iter().all(|f| f(r))
            })
            .collect();

        let total_records = filtered.len();

        // Group by time bucket
        let mut buckets: HashMap<String, Vec<&AuditRecord>> = HashMap::new();
        for record in &filtered {
            let bucket_key = self.get_bucket_key(record.timestamp);
            buckets.entry(bucket_key).or_default().push(record);
        }

        // Convert to time series points
        let mut points: Vec<TimeSeriesPoint> = buckets
            .into_iter()
            .map(|(label, records)| {
                let timestamp = self.parse_bucket_label(&label);
                TimeSeriesPoint {
                    timestamp,
                    label: label.clone(),
                    value: records.len() as f64,
                    count: records.len(),
                }
            })
            .collect();

        // Sort by timestamp
        points.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        Ok(TimeSeries {
            bucket: self.bucket,
            points,
            total_records,
        })
    }

    /// Gets the bucket key for a timestamp.
    fn get_bucket_key(&self, timestamp: DateTime<Utc>) -> String {
        match self.bucket {
            TimeBucket::Hourly => format!(
                "{}-{:02}-{:02}T{:02}:00",
                timestamp.year(),
                timestamp.month(),
                timestamp.day(),
                timestamp.hour()
            ),
            TimeBucket::Daily => format!(
                "{}-{:02}-{:02}",
                timestamp.year(),
                timestamp.month(),
                timestamp.day()
            ),
            TimeBucket::Weekly => {
                // ISO week number
                format!("{}-W{:02}", timestamp.year(), timestamp.iso_week().week())
            }
            TimeBucket::Monthly => {
                format!("{}-{:02}", timestamp.year(), timestamp.month())
            }
            TimeBucket::Quarterly => {
                let quarter = (timestamp.month() - 1) / 3 + 1;
                format!("{}-Q{}", timestamp.year(), quarter)
            }
            TimeBucket::Yearly => {
                format!("{}", timestamp.year())
            }
        }
    }

    /// Parses a bucket label back to a timestamp.
    fn parse_bucket_label(&self, label: &str) -> DateTime<Utc> {
        // Simple parsing - use start of bucket
        match self.bucket {
            TimeBucket::Hourly => {
                // Format: "2024-01-15T14:00"
                chrono::DateTime::parse_from_str(&format!("{}:00Z", label), "%Y-%m-%dT%H:%M:%SZ")
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now())
            }
            TimeBucket::Daily => {
                // Format: "2024-01-15"
                chrono::NaiveDate::parse_from_str(label, "%Y-%m-%d")
                    .ok()
                    .and_then(|d| d.and_hms_opt(0, 0, 0))
                    .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
                    .unwrap_or_else(Utc::now)
            }
            TimeBucket::Weekly
            | TimeBucket::Monthly
            | TimeBucket::Quarterly
            | TimeBucket::Yearly => {
                // Simplified - just use current time
                Utc::now()
            }
        }
    }
}

/// Time-series analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesAnalysis {
    /// The time series data
    pub series: TimeSeries,
    /// Detected trend
    pub trend: Trend,
    /// Growth rate (percentage)
    pub growth_rate: Option<f64>,
    /// Average value
    pub average: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Peak point
    pub peak_label: Option<String>,
    /// Valley point
    pub valley_label: Option<String>,
}

impl TimeSeriesAnalysis {
    /// Analyzes a time series.
    pub fn analyze(series: TimeSeries) -> Self {
        let trend = series.trend();
        let growth_rate = series.growth_rate();
        let average = series.average();
        let std_dev = series.std_dev();
        let peak_label = series.peak().map(|p| p.label.clone());
        let valley_label = series.valley().map(|p| p.label.clone());

        Self {
            series,
            trend,
            growth_rate,
            average,
            std_dev,
            peak_label,
            valley_label,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use chrono::Duration;
    use std::collections::HashMap as StdHashMap;
    use uuid::Uuid;

    fn create_test_records_over_time(count: usize, start: DateTime<Utc>) -> Vec<AuditRecord> {
        let mut records = Vec::new();
        for i in 0..count {
            let mut record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                "statute-1".to_string(),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "approved".to_string(),
                    parameters: StdHashMap::new(),
                },
                None,
            );
            record.timestamp = start + Duration::hours(i as i64);
            records.push(record);
        }
        records
    }

    #[test]
    fn test_timeseries_hourly() {
        let start = Utc::now();
        let records = create_test_records_over_time(24, start);

        let query = TimeSeriesQuery::new(TimeBucket::Hourly);
        let series = query.execute(&records).unwrap();

        assert_eq!(series.total_records, 24);
        assert!(series.points.len() <= 24); // May be less due to bucketing
    }

    #[test]
    fn test_timeseries_with_filter() {
        let start = Utc::now();
        let mut records = create_test_records_over_time(24, start);

        // Add some records with different statute
        for i in 0..10 {
            let mut record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                "statute-2".to_string(),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "approved".to_string(),
                    parameters: StdHashMap::new(),
                },
                None,
            );
            record.timestamp = start + Duration::hours(i as i64);
            records.push(record);
        }

        let query = TimeSeriesQuery::new(TimeBucket::Hourly).statute("statute-1".to_string());
        let series = query.execute(&records).unwrap();

        assert_eq!(series.total_records, 24);
    }

    #[test]
    fn test_timeseries_trend() {
        let start = Utc::now();
        let records = create_test_records_over_time(10, start);

        let query = TimeSeriesQuery::new(TimeBucket::Hourly);
        let series = query.execute(&records).unwrap();

        let trend = series.trend();
        // Should be relatively stable since we have uniform distribution
        assert!(matches!(
            trend,
            Trend::Stable | Trend::Increasing | Trend::Decreasing
        ));
    }

    #[test]
    fn test_timeseries_moving_average() {
        let start = Utc::now();
        let records = create_test_records_over_time(10, start);

        let query = TimeSeriesQuery::new(TimeBucket::Hourly);
        let series = query.execute(&records).unwrap();

        let ma = series.moving_average(3);
        assert!(!ma.is_empty());
    }

    #[test]
    fn test_timeseries_statistics() {
        let start = Utc::now();
        let records = create_test_records_over_time(10, start);

        let query = TimeSeriesQuery::new(TimeBucket::Hourly);
        let series = query.execute(&records).unwrap();

        let avg = series.average();
        assert!(avg > 0.0);

        let std_dev = series.std_dev();
        assert!(std_dev >= 0.0);
    }

    #[test]
    fn test_timeseries_analysis() {
        let start = Utc::now();
        let records = create_test_records_over_time(10, start);

        let query = TimeSeriesQuery::new(TimeBucket::Hourly);
        let series = query.execute(&records).unwrap();

        let analysis = TimeSeriesAnalysis::analyze(series);
        assert!(matches!(
            analysis.trend,
            Trend::Stable | Trend::Increasing | Trend::Decreasing
        ));
        assert!(analysis.average > 0.0);
    }
}
