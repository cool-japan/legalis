//! Forecasting and trend analysis for simulation results.
//!
//! This module provides tools for analyzing trends in simulation data
//! and forecasting future outcomes based on historical patterns.

use crate::{SimResult, SimulationMetrics};
use serde::{Deserialize, Serialize};

/// Forecast data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastPoint {
    /// Time index (e.g., simulation iteration, time step)
    pub time: usize,
    /// Value at this time
    pub value: f64,
}

/// Time series of simulation metrics.
#[derive(Debug, Clone)]
pub struct TimeSeries {
    /// Data points in chronological order
    pub points: Vec<ForecastPoint>,
    /// Name/description of the series
    pub name: String,
}

impl TimeSeries {
    /// Creates a new time series.
    pub fn new(name: String) -> Self {
        Self {
            points: Vec::new(),
            name,
        }
    }

    /// Creates a time series from simulation metrics.
    pub fn from_metrics(name: String, metrics: Vec<SimulationMetrics>) -> Self {
        let points = metrics
            .into_iter()
            .enumerate()
            .map(|(i, m)| ForecastPoint {
                time: i,
                value: m.deterministic_ratio(),
            })
            .collect();

        Self { points, name }
    }

    /// Adds a data point.
    pub fn add_point(mut self, time: usize, value: f64) -> Self {
        self.points.push(ForecastPoint { time, value });
        self
    }

    /// Returns the number of data points.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns true if the series is empty.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Calculates the mean value.
    pub fn mean(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        self.points.iter().map(|p| p.value).sum::<f64>() / self.points.len() as f64
    }

    /// Calculates linear trend (slope).
    pub fn linear_trend(&self) -> SimResult<f64> {
        if self.points.len() < 2 {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Need at least 2 points for trend calculation".to_string(),
            ));
        }

        let n = self.points.len() as f64;
        let sum_x: f64 = self.points.iter().map(|p| p.time as f64).sum();
        let sum_y: f64 = self.points.iter().map(|p| p.value).sum();
        let sum_xy: f64 = self.points.iter().map(|p| p.time as f64 * p.value).sum();
        let sum_x2: f64 = self.points.iter().map(|p| (p.time as f64).powi(2)).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        Ok(slope)
    }

    /// Detects if there's a significant trend (positive or negative).
    pub fn has_trend(&self, threshold: f64) -> SimResult<bool> {
        let slope = self.linear_trend()?;
        Ok(slope.abs() > threshold)
    }
}

/// Linear trend forecast.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearForecast {
    /// Slope of the trend line
    pub slope: f64,
    /// Intercept of the trend line
    pub intercept: f64,
    /// R-squared value (goodness of fit)
    pub r_squared: f64,
    /// Forecasted values
    pub forecast: Vec<ForecastPoint>,
}

impl LinearForecast {
    /// Generates a linear forecast from historical data.
    ///
    /// # Arguments
    /// * `time_series` - Historical data
    /// * `forecast_horizon` - Number of periods to forecast
    pub fn generate(time_series: &TimeSeries, forecast_horizon: usize) -> SimResult<Self> {
        if time_series.points.len() < 2 {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Need at least 2 points for forecasting".to_string(),
            ));
        }

        let n = time_series.points.len() as f64;
        let sum_x: f64 = time_series.points.iter().map(|p| p.time as f64).sum();
        let sum_y: f64 = time_series.points.iter().map(|p| p.value).sum();
        let sum_xy: f64 = time_series
            .points
            .iter()
            .map(|p| p.time as f64 * p.value)
            .sum();
        let sum_x2: f64 = time_series
            .points
            .iter()
            .map(|p| (p.time as f64).powi(2))
            .sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;

        // Calculate R-squared
        let mean_y = sum_y / n;
        let ss_tot: f64 = time_series
            .points
            .iter()
            .map(|p| (p.value - mean_y).powi(2))
            .sum();
        let ss_res: f64 = time_series
            .points
            .iter()
            .map(|p| {
                let predicted = slope * p.time as f64 + intercept;
                (p.value - predicted).powi(2)
            })
            .sum();
        let r_squared = if ss_tot > 0.0 {
            1.0 - (ss_res / ss_tot)
        } else {
            0.0
        };

        // Generate forecast
        let last_time = time_series.points.last().unwrap().time;
        let forecast = (1..=forecast_horizon)
            .map(|i| {
                let time = last_time + i;
                let value = slope * time as f64 + intercept;
                ForecastPoint { time, value }
            })
            .collect();

        Ok(Self {
            slope,
            intercept,
            r_squared,
            forecast,
        })
    }

    /// Returns a human-readable summary.
    pub fn summary(&self) -> String {
        let trend = if self.slope > 0.0 {
            "increasing"
        } else if self.slope < 0.0 {
            "decreasing"
        } else {
            "stable"
        };

        format!(
            "Linear Forecast: {} trend (slope={:.4}), R²={:.3}",
            trend, self.slope, self.r_squared
        )
    }
}

/// Moving average forecast.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovingAverageForecast {
    /// Window size for moving average
    pub window_size: usize,
    /// Forecasted values
    pub forecast: Vec<ForecastPoint>,
}

impl MovingAverageForecast {
    /// Generates a moving average forecast.
    ///
    /// # Arguments
    /// * `time_series` - Historical data
    /// * `window_size` - Number of recent periods to average
    /// * `forecast_horizon` - Number of periods to forecast
    pub fn generate(
        time_series: &TimeSeries,
        window_size: usize,
        forecast_horizon: usize,
    ) -> SimResult<Self> {
        if time_series.points.len() < window_size {
            return Err(crate::SimulationError::InvalidConfiguration(format!(
                "Need at least {} points for window size {}",
                window_size, window_size
            )));
        }

        // Calculate moving average of last window_size points
        let recent_values: Vec<f64> = time_series
            .points
            .iter()
            .rev()
            .take(window_size)
            .map(|p| p.value)
            .collect();

        let average = recent_values.iter().sum::<f64>() / window_size as f64;

        // Generate flat forecast
        let last_time = time_series.points.last().unwrap().time;
        let forecast = (1..=forecast_horizon)
            .map(|i| ForecastPoint {
                time: last_time + i,
                value: average,
            })
            .collect();

        Ok(Self {
            window_size,
            forecast,
        })
    }
}

/// Exponential smoothing forecast.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExponentialSmoothingForecast {
    /// Smoothing parameter (0 to 1)
    pub alpha: f64,
    /// Current smoothed value
    pub smoothed_value: f64,
    /// Forecasted values
    pub forecast: Vec<ForecastPoint>,
}

impl ExponentialSmoothingForecast {
    /// Generates an exponential smoothing forecast.
    ///
    /// # Arguments
    /// * `time_series` - Historical data
    /// * `alpha` - Smoothing parameter (0 to 1, higher = more weight on recent values)
    /// * `forecast_horizon` - Number of periods to forecast
    pub fn generate(
        time_series: &TimeSeries,
        alpha: f64,
        forecast_horizon: usize,
    ) -> SimResult<Self> {
        if time_series.points.is_empty() {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Need at least 1 point for exponential smoothing".to_string(),
            ));
        }

        if !(0.0..=1.0).contains(&alpha) {
            return Err(crate::SimulationError::InvalidConfiguration(
                "Alpha must be between 0 and 1".to_string(),
            ));
        }

        // Calculate smoothed value
        let mut smoothed = time_series.points[0].value;
        for point in &time_series.points[1..] {
            smoothed = alpha * point.value + (1.0 - alpha) * smoothed;
        }

        // Generate flat forecast (simple exponential smoothing)
        let last_time = time_series.points.last().unwrap().time;
        let forecast = (1..=forecast_horizon)
            .map(|i| ForecastPoint {
                time: last_time + i,
                value: smoothed,
            })
            .collect();

        Ok(Self {
            alpha,
            smoothed_value: smoothed,
            forecast,
        })
    }
}

/// Comprehensive forecast with multiple methods.
#[derive(Debug, Clone)]
pub struct CompositeForecast {
    /// Linear forecast
    pub linear: LinearForecast,
    /// Moving average forecast
    pub moving_average: MovingAverageForecast,
    /// Exponential smoothing forecast
    pub exponential: ExponentialSmoothingForecast,
    /// Ensemble forecast (average of all methods)
    pub ensemble: Vec<ForecastPoint>,
}

impl CompositeForecast {
    /// Generates a composite forecast using multiple methods.
    pub fn generate(time_series: &TimeSeries, forecast_horizon: usize) -> SimResult<Self> {
        let linear = LinearForecast::generate(time_series, forecast_horizon)?;
        let moving_average = MovingAverageForecast::generate(
            time_series,
            3.min(time_series.len()),
            forecast_horizon,
        )?;
        let exponential =
            ExponentialSmoothingForecast::generate(time_series, 0.3, forecast_horizon)?;

        // Create ensemble forecast (average of all methods)
        let mut ensemble = Vec::new();
        for i in 0..forecast_horizon {
            let time = linear.forecast[i].time;
            let avg_value = (linear.forecast[i].value
                + moving_average.forecast[i].value
                + exponential.forecast[i].value)
                / 3.0;
            ensemble.push(ForecastPoint {
                time,
                value: avg_value,
            });
        }

        Ok(Self {
            linear,
            moving_average,
            exponential,
            ensemble,
        })
    }

    /// Returns the recommended forecast (ensemble).
    pub fn recommended(&self) -> &Vec<ForecastPoint> {
        &self.ensemble
    }

    /// Generates a summary report.
    pub fn summary(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Composite Forecast ===\n\n");
        report.push_str(&format!("{}\n", self.linear.summary()));
        report.push_str(&format!(
            "Moving Average (window={}): {:.3}\n",
            self.moving_average.window_size,
            self.moving_average
                .forecast
                .first()
                .map(|p| p.value)
                .unwrap_or(0.0)
        ));
        report.push_str(&format!(
            "Exponential Smoothing (α={:.2}): {:.3}\n\n",
            self.exponential.alpha, self.exponential.smoothed_value
        ));
        report.push_str("Ensemble Forecast:\n");
        for point in &self.ensemble {
            report.push_str(&format!("  t={}: {:.3}\n", point.time, point.value));
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_series() -> TimeSeries {
        TimeSeries::new("test".to_string())
            .add_point(0, 0.5)
            .add_point(1, 0.6)
            .add_point(2, 0.7)
            .add_point(3, 0.8)
            .add_point(4, 0.9)
    }

    #[test]
    fn test_time_series_creation() {
        let series = create_test_series();
        assert_eq!(series.len(), 5);
        assert!(!series.is_empty());
        assert_eq!(series.mean(), 0.7);
    }

    #[test]
    fn test_linear_trend() {
        let series = create_test_series();
        let slope = series.linear_trend().unwrap();
        assert!((slope - 0.1).abs() < 0.01); // Should be approximately 0.1
    }

    #[test]
    fn test_has_trend() {
        let series = create_test_series();
        assert!(series.has_trend(0.05).unwrap());
        assert!(!series.has_trend(0.2).unwrap());
    }

    #[test]
    fn test_linear_forecast() {
        let series = create_test_series();
        let forecast = LinearForecast::generate(&series, 3).unwrap();

        assert_eq!(forecast.forecast.len(), 3);
        assert!(forecast.slope > 0.0); // Positive trend
        assert!(forecast.r_squared > 0.9); // Good fit
    }

    #[test]
    fn test_moving_average_forecast() {
        let series = create_test_series();
        let forecast = MovingAverageForecast::generate(&series, 3, 2).unwrap();

        assert_eq!(forecast.window_size, 3);
        assert_eq!(forecast.forecast.len(), 2);
        // Should be average of last 3 values (0.7, 0.8, 0.9)
        assert!((forecast.forecast[0].value - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_exponential_smoothing() {
        let series = create_test_series();
        let forecast = ExponentialSmoothingForecast::generate(&series, 0.3, 2).unwrap();

        assert_eq!(forecast.alpha, 0.3);
        assert_eq!(forecast.forecast.len(), 2);
        assert!(forecast.smoothed_value > 0.0);
    }

    #[test]
    fn test_composite_forecast() {
        let series = create_test_series();
        let composite = CompositeForecast::generate(&series, 3).unwrap();

        assert_eq!(composite.ensemble.len(), 3);
        assert!(!composite.summary().is_empty());

        let recommended = composite.recommended();
        assert_eq!(recommended.len(), 3);
    }

    #[test]
    fn test_insufficient_data() {
        let series = TimeSeries::new("test".to_string()).add_point(0, 0.5);

        assert!(LinearForecast::generate(&series, 1).is_err());
        assert!(MovingAverageForecast::generate(&series, 3, 1).is_err());
    }

    #[test]
    fn test_invalid_alpha() {
        let series = create_test_series();
        assert!(ExponentialSmoothingForecast::generate(&series, 1.5, 1).is_err());
        assert!(ExponentialSmoothingForecast::generate(&series, -0.1, 1).is_err());
    }

    #[test]
    fn test_forecast_summary() {
        let series = create_test_series();
        let forecast = LinearForecast::generate(&series, 2).unwrap();
        let summary = forecast.summary();

        assert!(summary.contains("Linear Forecast"));
        assert!(summary.contains("trend"));
    }
}
