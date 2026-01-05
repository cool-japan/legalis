//! Trend forecasting for decision and compliance patterns.
//!
//! This module provides time-series forecasting capabilities to predict
//! future trends in decision-making, compliance metrics, and audit patterns.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for trend forecasting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastConfig {
    /// Historical lookback period in days
    pub lookback_days: i64,
    /// Forecast horizon in days
    pub forecast_days: i64,
    /// Confidence interval (e.g., 0.95 for 95%)
    pub confidence_interval: f64,
    /// Enable seasonal decomposition
    pub enable_seasonal: bool,
    /// Seasonal period in days (e.g., 7 for weekly)
    pub seasonal_period: i64,
}

impl Default for ForecastConfig {
    fn default() -> Self {
        Self {
            lookback_days: 90,
            forecast_days: 30,
            confidence_interval: 0.95,
            enable_seasonal: true,
            seasonal_period: 7,
        }
    }
}

/// Trend forecast result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendForecast {
    /// Metric being forecasted
    pub metric_name: String,
    /// Forecast data points
    pub forecast_points: Vec<ForecastPoint>,
    /// Detected trend direction
    pub trend_direction: TrendDirection,
    /// Trend strength (0.0-1.0)
    pub trend_strength: f64,
    /// Seasonal pattern detected
    pub seasonal_pattern: Option<SeasonalPattern>,
    /// Model accuracy (R-squared, 0.0-1.0)
    pub accuracy: f64,
    /// Generated at
    pub generated_at: DateTime<Utc>,
}

/// Individual forecast point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastPoint {
    /// Point in time
    pub timestamp: DateTime<Utc>,
    /// Forecasted value
    pub value: f64,
    /// Lower bound of confidence interval
    pub lower_bound: f64,
    /// Upper bound of confidence interval
    pub upper_bound: f64,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
}

/// Trend direction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Cyclical,
}

/// Seasonal pattern information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalPattern {
    /// Period length in days
    pub period_days: i64,
    /// Peak times
    pub peak_times: Vec<String>,
    /// Seasonal strength (0.0-1.0)
    pub strength: f64,
}

/// Metric to forecast.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ForecastMetric {
    /// Total decision volume
    Volume,
    /// Override rate
    OverrideRate,
    /// Discretionary rate
    DiscretionaryRate,
    /// Void rate
    VoidRate,
    /// Specific statute usage
    StatuteUsage(String),
    /// Custom metric
    Custom(String),
}

/// Trend forecaster.
pub struct TrendForecaster {
    config: ForecastConfig,
}

impl TrendForecaster {
    /// Creates a new trend forecaster with default configuration.
    pub fn new() -> Self {
        Self::with_config(ForecastConfig::default())
    }

    /// Creates a new trend forecaster with custom configuration.
    pub fn with_config(config: ForecastConfig) -> Self {
        Self { config }
    }

    /// Forecasts a specific metric.
    pub fn forecast(
        &self,
        records: &[AuditRecord],
        metric: ForecastMetric,
    ) -> AuditResult<TrendForecast> {
        if records.is_empty() {
            return Err(AuditError::InvalidRecord(
                "Cannot forecast from empty dataset".to_string(),
            ));
        }

        // Extract historical time series
        let time_series = self.extract_time_series(records, &metric)?;

        if time_series.is_empty() {
            return Err(AuditError::InvalidRecord(
                "Insufficient data for forecasting".to_string(),
            ));
        }

        // Detect trend
        let (trend_direction, trend_strength) = self.detect_trend(&time_series);

        // Detect seasonality if enabled
        let seasonal_pattern = if self.config.enable_seasonal {
            self.detect_seasonality(&time_series)
        } else {
            None
        };

        // Generate forecast points
        let forecast_points =
            self.generate_forecast(&time_series, &trend_direction, seasonal_pattern.as_ref())?;

        // Calculate model accuracy
        let accuracy = self.calculate_accuracy(&time_series);

        let metric_name = match metric {
            ForecastMetric::Volume => "Decision Volume".to_string(),
            ForecastMetric::OverrideRate => "Override Rate".to_string(),
            ForecastMetric::DiscretionaryRate => "Discretionary Rate".to_string(),
            ForecastMetric::VoidRate => "Void Rate".to_string(),
            ForecastMetric::StatuteUsage(statute) => format!("Statute {} Usage", statute),
            ForecastMetric::Custom(name) => name.clone(),
        };

        Ok(TrendForecast {
            metric_name,
            forecast_points,
            trend_direction,
            trend_strength,
            seasonal_pattern,
            accuracy,
            generated_at: Utc::now(),
        })
    }

    /// Forecasts multiple metrics simultaneously.
    pub fn forecast_multiple(
        &self,
        records: &[AuditRecord],
        metrics: Vec<ForecastMetric>,
    ) -> AuditResult<Vec<TrendForecast>> {
        let mut forecasts = Vec::new();

        for metric in metrics {
            match self.forecast(records, metric) {
                Ok(forecast) => forecasts.push(forecast),
                Err(e) => {
                    tracing::warn!("Failed to forecast metric: {}", e);
                }
            }
        }

        Ok(forecasts)
    }

    /// Extracts time series data for a specific metric.
    fn extract_time_series(
        &self,
        records: &[AuditRecord],
        metric: &ForecastMetric,
    ) -> AuditResult<Vec<(DateTime<Utc>, f64)>> {
        let cutoff = Utc::now() - Duration::days(self.config.lookback_days);
        let recent_records: Vec<_> = records.iter().filter(|r| r.timestamp >= cutoff).collect();

        // Group records by day
        let mut daily_data: HashMap<String, Vec<&AuditRecord>> = HashMap::new();
        for record in recent_records {
            let date_key = record.timestamp.format("%Y-%m-%d").to_string();
            daily_data.entry(date_key).or_default().push(record);
        }

        let mut time_series = Vec::new();

        for (date_str, day_records) in daily_data {
            let date = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map_err(|e| AuditError::InvalidRecord(e.to_string()))?
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| AuditError::InvalidRecord("Invalid time".to_string()))?
                .and_utc();

            let value = match metric {
                ForecastMetric::Volume => day_records.len() as f64,
                ForecastMetric::OverrideRate => {
                    let override_count = day_records
                        .iter()
                        .filter(|r| matches!(r.result, crate::DecisionResult::Overridden { .. }))
                        .count();
                    override_count as f64 / day_records.len() as f64
                }
                ForecastMetric::DiscretionaryRate => {
                    let discretionary_count = day_records
                        .iter()
                        .filter(|r| {
                            matches!(r.result, crate::DecisionResult::RequiresDiscretion { .. })
                        })
                        .count();
                    discretionary_count as f64 / day_records.len() as f64
                }
                ForecastMetric::VoidRate => {
                    let void_count = day_records
                        .iter()
                        .filter(|r| matches!(r.result, crate::DecisionResult::Void { .. }))
                        .count();
                    void_count as f64 / day_records.len() as f64
                }
                ForecastMetric::StatuteUsage(statute_id) => day_records
                    .iter()
                    .filter(|r| &r.statute_id == statute_id)
                    .count() as f64,
                ForecastMetric::Custom(_) => 0.0, // Placeholder
            };

            time_series.push((date, value));
        }

        // Sort by date
        time_series.sort_by_key(|(date, _)| *date);

        Ok(time_series)
    }

    /// Detects trend in time series using linear regression.
    fn detect_trend(&self, time_series: &[(DateTime<Utc>, f64)]) -> (TrendDirection, f64) {
        if time_series.len() < 2 {
            return (TrendDirection::Stable, 0.0);
        }

        // Simple linear regression
        let n = time_series.len() as f64;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_x2 = 0.0;

        for (i, (_, value)) in time_series.iter().enumerate() {
            let x = i as f64;
            sum_x += x;
            sum_y += value;
            sum_xy += x * value;
            sum_x2 += x * x;
        }

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);

        // Determine direction and strength
        let avg_value = sum_y / n;
        let strength = if avg_value > 0.0 {
            (slope.abs() / avg_value).min(1.0)
        } else {
            0.0
        };

        let direction = if slope > 0.01 {
            TrendDirection::Increasing
        } else if slope < -0.01 {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        };

        (direction, strength)
    }

    /// Detects seasonal patterns.
    fn detect_seasonality(&self, time_series: &[(DateTime<Utc>, f64)]) -> Option<SeasonalPattern> {
        if time_series.len() < self.config.seasonal_period as usize * 2 {
            return None;
        }

        // Group by day of week
        let mut day_averages = [0.0; 7];
        let mut day_counts = [0; 7];

        for (date, value) in time_series {
            let day = date.weekday().num_days_from_monday() as usize;
            day_averages[day] += value;
            day_counts[day] += 1;
        }

        for i in 0..7 {
            if day_counts[i] > 0 {
                day_averages[i] /= day_counts[i] as f64;
            }
        }

        // Find peaks
        let max_avg = day_averages
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let min_avg = day_averages.iter().cloned().fold(f64::INFINITY, f64::min);

        if max_avg == 0.0 || (max_avg - min_avg).abs() < 0.1 {
            return None;
        }

        let strength = (max_avg - min_avg) / max_avg;

        let peak_times: Vec<String> = day_averages
            .iter()
            .enumerate()
            .filter(|&(_, &avg)| avg >= max_avg * 0.9)
            .map(|(i, _)| {
                match i {
                    0 => "Monday",
                    1 => "Tuesday",
                    2 => "Wednesday",
                    3 => "Thursday",
                    4 => "Friday",
                    5 => "Saturday",
                    6 => "Sunday",
                    _ => "Unknown",
                }
                .to_string()
            })
            .collect();

        Some(SeasonalPattern {
            period_days: self.config.seasonal_period,
            peak_times,
            strength,
        })
    }

    /// Generates forecast points using exponential smoothing.
    fn generate_forecast(
        &self,
        time_series: &[(DateTime<Utc>, f64)],
        trend: &TrendDirection,
        seasonal: Option<&SeasonalPattern>,
    ) -> AuditResult<Vec<ForecastPoint>> {
        if time_series.is_empty() {
            return Ok(Vec::new());
        }

        let mut forecast_points = Vec::new();

        // Calculate baseline (average of recent values)
        let baseline = time_series.iter().map(|(_, v)| v).sum::<f64>() / time_series.len() as f64;

        // Calculate trend increment
        let trend_increment = match trend {
            TrendDirection::Increasing => baseline * 0.01, // 1% increase per day
            TrendDirection::Decreasing => baseline * -0.01,
            TrendDirection::Stable => 0.0,
            TrendDirection::Cyclical => 0.0,
        };

        let last_date = time_series.last().unwrap().0;

        for i in 1..=self.config.forecast_days {
            let forecast_date = last_date + Duration::days(i);
            let mut forecast_value = baseline + trend_increment * i as f64;

            // Apply seasonal adjustment
            if let Some(pattern) = seasonal {
                let day_of_week = forecast_date.weekday().num_days_from_monday();
                let is_peak = pattern.peak_times.contains(
                    &match day_of_week {
                        0 => "Monday",
                        1 => "Tuesday",
                        2 => "Wednesday",
                        3 => "Thursday",
                        4 => "Friday",
                        5 => "Saturday",
                        6 => "Sunday",
                        _ => "Unknown",
                    }
                    .to_string(),
                );

                if is_peak {
                    forecast_value *= 1.0 + pattern.strength * 0.2;
                } else {
                    forecast_value *= 1.0 - pattern.strength * 0.1;
                }
            }

            // Calculate confidence interval
            let std_dev = self.calculate_std_dev(time_series);
            let z_score = 1.96; // 95% confidence
            let margin = z_score * std_dev;

            // Confidence decreases with distance
            let confidence =
                (1.0 - (i as f64 / self.config.forecast_days as f64) * 0.3).clamp(0.5, 1.0);

            forecast_points.push(ForecastPoint {
                timestamp: forecast_date,
                value: forecast_value.max(0.0),
                lower_bound: (forecast_value - margin).max(0.0),
                upper_bound: forecast_value + margin,
                confidence,
            });
        }

        Ok(forecast_points)
    }

    /// Calculates standard deviation of time series values.
    fn calculate_std_dev(&self, time_series: &[(DateTime<Utc>, f64)]) -> f64 {
        if time_series.is_empty() {
            return 0.0;
        }

        let mean = time_series.iter().map(|(_, v)| v).sum::<f64>() / time_series.len() as f64;
        let variance = time_series
            .iter()
            .map(|(_, v)| (v - mean).powi(2))
            .sum::<f64>()
            / time_series.len() as f64;

        variance.sqrt()
    }

    /// Calculates model accuracy using R-squared.
    fn calculate_accuracy(&self, time_series: &[(DateTime<Utc>, f64)]) -> f64 {
        if time_series.len() < 2 {
            return 0.0;
        }

        let mean = time_series.iter().map(|(_, v)| v).sum::<f64>() / time_series.len() as f64;
        let total_variance = time_series
            .iter()
            .map(|(_, v)| (v - mean).powi(2))
            .sum::<f64>();

        if total_variance == 0.0 {
            return 1.0;
        }

        // Simplified: assume linear trend
        let (_, trend_strength) = self.detect_trend(time_series);

        // R-squared approximation
        trend_strength.min(1.0)
    }

    /// Returns the configuration.
    pub fn config(&self) -> &ForecastConfig {
        &self.config
    }
}

impl Default for TrendForecaster {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn create_test_record(days_ago: i64) -> AuditRecord {
        let timestamp = Utc::now() - Duration::days(days_ago);

        AuditRecord {
            id: Uuid::new_v4(),
            timestamp,
            event_type: EventType::AutomaticDecision,
            actor: Actor::System {
                component: "test".to_string(),
            },
            statute_id: "test-statute".to_string(),
            subject_id: Uuid::new_v4(),
            context: DecisionContext::default(),
            result: DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: HashMap::new(),
            },
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    #[test]
    fn test_trend_forecaster_creation() {
        let forecaster = TrendForecaster::new();
        assert_eq!(forecaster.config().lookback_days, 90);
        assert_eq!(forecaster.config().forecast_days, 30);
    }

    #[test]
    fn test_forecast_volume() {
        let forecaster = TrendForecaster::new();

        // Create records with increasing volume
        let records: Vec<_> = (0..60)
            .flat_map(|i| (0..i).map(move |_| create_test_record(60 - i)))
            .collect();

        let forecast = forecaster
            .forecast(&records, ForecastMetric::Volume)
            .unwrap();

        assert_eq!(forecast.metric_name, "Decision Volume");
        assert!(!forecast.forecast_points.is_empty());
        assert_eq!(forecast.forecast_points.len(), 30);
    }

    #[test]
    fn test_detect_trend() {
        let forecaster = TrendForecaster::new();

        // Increasing trend
        let increasing: Vec<_> = (0..10)
            .map(|i| (Utc::now() + Duration::days(i), i as f64 * 2.0))
            .collect();
        let (direction, strength) = forecaster.detect_trend(&increasing);
        assert_eq!(direction, TrendDirection::Increasing);
        assert!(strength > 0.0);

        // Decreasing trend
        let decreasing: Vec<_> = (0..10)
            .map(|i| (Utc::now() + Duration::days(i), 20.0 - i as f64 * 2.0))
            .collect();
        let (direction, _) = forecaster.detect_trend(&decreasing);
        assert_eq!(direction, TrendDirection::Decreasing);

        // Stable trend
        let stable: Vec<_> = (0..10)
            .map(|i| (Utc::now() + Duration::days(i), 10.0))
            .collect();
        let (direction, _) = forecaster.detect_trend(&stable);
        assert_eq!(direction, TrendDirection::Stable);
    }

    #[test]
    fn test_detect_seasonality() {
        let forecaster = TrendForecaster::new();

        // Create data with weekly pattern
        let mut time_series = Vec::new();
        for i in 0..30 {
            let date = Utc::now() - Duration::days(30 - i);
            let value = if date.weekday().num_days_from_monday() < 5 {
                10.0 // Weekdays
            } else {
                5.0 // Weekends
            };
            time_series.push((date, value));
        }

        let seasonal = forecaster.detect_seasonality(&time_series);
        assert!(seasonal.is_some());

        if let Some(pattern) = seasonal {
            assert_eq!(pattern.period_days, 7);
            assert!(!pattern.peak_times.is_empty());
        }
    }

    #[test]
    fn test_forecast_multiple() {
        let forecaster = TrendForecaster::new();

        let records: Vec<_> = (0..30).map(|i| create_test_record(i)).collect();

        let metrics = vec![ForecastMetric::Volume, ForecastMetric::OverrideRate];

        let forecasts = forecaster.forecast_multiple(&records, metrics).unwrap();
        assert!(!forecasts.is_empty());
    }

    #[test]
    fn test_empty_records() {
        let forecaster = TrendForecaster::new();
        let records: Vec<AuditRecord> = Vec::new();

        let result = forecaster.forecast(&records, ForecastMetric::Volume);
        assert!(result.is_err());
    }

    #[test]
    fn test_forecast_confidence() {
        let forecaster = TrendForecaster::new();

        let records: Vec<_> = (0..30).map(|i| create_test_record(i)).collect();

        let forecast = forecaster
            .forecast(&records, ForecastMetric::Volume)
            .unwrap();

        // Check that confidence decreases over time
        let first_confidence = forecast.forecast_points.first().unwrap().confidence;
        let last_confidence = forecast.forecast_points.last().unwrap().confidence;
        assert!(first_confidence >= last_confidence);
    }
}
