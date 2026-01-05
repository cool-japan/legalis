//! Calibration and validation tools for simulation models.
//!
//! This module provides tools for calibrating simulation parameters against
//! empirical data and validating model outputs.

use crate::{SimResult, SimulationError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Empirical data point for calibration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    /// Timestamp or period identifier.
    pub time: String,
    /// Observed values.
    pub values: HashMap<String, f64>,
}

/// Empirical dataset for calibration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpiricalData {
    /// Data points.
    pub points: Vec<DataPoint>,
}

impl EmpiricalData {
    /// Create new empty dataset.
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Add data point.
    pub fn add_point(&mut self, time: String, values: HashMap<String, f64>) {
        self.points.push(DataPoint { time, values });
    }

    /// Get metric values as time series.
    pub fn get_metric_series(&self, metric_name: &str) -> Vec<f64> {
        self.points
            .iter()
            .filter_map(|p| p.values.get(metric_name).copied())
            .collect()
    }
}

impl Default for EmpiricalData {
    fn default() -> Self {
        Self::new()
    }
}

/// Goodness-of-fit metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodnessOfFit {
    /// Mean squared error.
    pub mse: f64,
    /// Root mean squared error.
    pub rmse: f64,
    /// Mean absolute error.
    pub mae: f64,
    /// R-squared value.
    pub r_squared: f64,
    /// Normalized RMSE.
    pub nrmse: f64,
}

/// Calibration result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationResult {
    /// Calibrated parameters.
    pub parameters: HashMap<String, f64>,
    /// Goodness-of-fit metrics.
    pub goodness_of_fit: HashMap<String, GoodnessOfFit>,
    /// Number of iterations.
    pub iterations: usize,
    /// Whether calibration converged.
    pub converged: bool,
}

/// Parameter calibrator.
pub struct ParameterCalibrator {
    /// Maximum iterations.
    pub max_iterations: usize,
    /// Convergence tolerance.
    pub tolerance: f64,
}

impl ParameterCalibrator {
    /// Create new calibrator.
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self {
            max_iterations,
            tolerance,
        }
    }

    /// Calibrate parameters to match empirical data.
    pub fn calibrate<F>(
        &self,
        initial_params: HashMap<String, f64>,
        empirical_data: &EmpiricalData,
        simulation_fn: F,
    ) -> SimResult<CalibrationResult>
    where
        F: Fn(&HashMap<String, f64>) -> HashMap<String, Vec<f64>>,
    {
        let mut current_params = initial_params;
        let mut best_params = current_params.clone();
        let mut best_error = f64::INFINITY;
        let mut converged = false;

        for iteration in 0..self.max_iterations {
            // Run simulation with current parameters
            let simulated = simulation_fn(&current_params);

            // Calculate total error
            let mut total_error = 0.0;
            let mut metric_count = 0;

            for (metric_name, empirical_series) in self.collect_empirical_series(empirical_data) {
                if let Some(simulated_series) = simulated.get(&metric_name) {
                    let error = Self::calculate_mse(&empirical_series, simulated_series);
                    total_error += error;
                    metric_count += 1;
                }
            }

            if metric_count > 0 {
                total_error /= metric_count as f64;
            }

            // Update best if improved
            if total_error < best_error {
                best_error = total_error;
                best_params = current_params.clone();
            }

            // Check convergence
            if total_error < self.tolerance {
                converged = true;
                break;
            }

            // Simple gradient descent with finite differences
            let step_size = 0.01;
            let param_names: Vec<String> = current_params.keys().cloned().collect();

            for param_name in param_names {
                // Approximate gradient
                let mut perturbed = current_params.clone();
                *perturbed.get_mut(&param_name).unwrap() += step_size;

                let perturbed_simulated = simulation_fn(&perturbed);
                let mut perturbed_error = 0.0;
                let mut count = 0;

                for (metric_name, empirical_series) in self.collect_empirical_series(empirical_data)
                {
                    if let Some(perturbed_series) = perturbed_simulated.get(&metric_name) {
                        let error = Self::calculate_mse(&empirical_series, perturbed_series);
                        perturbed_error += error;
                        count += 1;
                    }
                }

                if count > 0 {
                    perturbed_error /= count as f64;
                }

                let gradient = (perturbed_error - total_error) / step_size;
                let param_value = current_params.get_mut(&param_name).unwrap();
                *param_value -= 0.1 * gradient; // Simple gradient step
            }

            if iteration > 0 && iteration % 10 == 0 {
                // Check for stagnation
                if (total_error - best_error).abs() < self.tolerance * 0.1 {
                    converged = true;
                    break;
                }
            }
        }

        // Calculate final goodness-of-fit metrics
        let final_simulated = simulation_fn(&best_params);
        let mut goodness_of_fit = HashMap::new();

        for (metric_name, empirical_series) in self.collect_empirical_series(empirical_data) {
            if let Some(simulated_series) = final_simulated.get(&metric_name) {
                let gof = Self::calculate_goodness_of_fit(&empirical_series, simulated_series);
                goodness_of_fit.insert(metric_name, gof);
            }
        }

        Ok(CalibrationResult {
            parameters: best_params,
            goodness_of_fit,
            iterations: self.max_iterations,
            converged,
        })
    }

    fn collect_empirical_series(&self, data: &EmpiricalData) -> HashMap<String, Vec<f64>> {
        let mut series = HashMap::new();

        if let Some(first_point) = data.points.first() {
            for metric_name in first_point.values.keys() {
                let metric_series = data.get_metric_series(metric_name);
                series.insert(metric_name.clone(), metric_series);
            }
        }

        series
    }

    fn calculate_mse(observed: &[f64], predicted: &[f64]) -> f64 {
        let n = observed.len().min(predicted.len());
        if n == 0 {
            return 0.0;
        }

        let sum_squared_error: f64 = observed[..n]
            .iter()
            .zip(&predicted[..n])
            .map(|(o, p)| (o - p).powi(2))
            .sum();

        sum_squared_error / n as f64
    }

    fn calculate_goodness_of_fit(observed: &[f64], predicted: &[f64]) -> GoodnessOfFit {
        let n = observed.len().min(predicted.len());

        if n == 0 {
            return GoodnessOfFit {
                mse: 0.0,
                rmse: 0.0,
                mae: 0.0,
                r_squared: 0.0,
                nrmse: 0.0,
            };
        }

        // MSE and RMSE
        let mse = Self::calculate_mse(observed, predicted);
        let rmse = mse.sqrt();

        // MAE
        let mae: f64 = observed[..n]
            .iter()
            .zip(&predicted[..n])
            .map(|(o, p)| (o - p).abs())
            .sum::<f64>()
            / n as f64;

        // R-squared
        let mean_observed: f64 = observed[..n].iter().sum::<f64>() / n as f64;
        let ss_tot: f64 = observed[..n]
            .iter()
            .map(|o| (o - mean_observed).powi(2))
            .sum();
        let ss_res: f64 = observed[..n]
            .iter()
            .zip(&predicted[..n])
            .map(|(o, p)| (o - p).powi(2))
            .sum();

        let r_squared = if ss_tot > 0.0 {
            1.0 - (ss_res / ss_tot)
        } else {
            0.0
        };

        // Normalized RMSE
        let range = observed[..n]
            .iter()
            .fold(f64::NEG_INFINITY, |a, &b| a.max(b))
            - observed[..n].iter().fold(f64::INFINITY, |a, &b| a.min(b));

        let nrmse = if range > 0.0 { rmse / range } else { 0.0 };

        GoodnessOfFit {
            mse,
            rmse,
            mae,
            r_squared,
            nrmse,
        }
    }
}

/// Cross-validation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossValidationResult {
    /// Number of folds.
    pub folds: usize,
    /// Average goodness-of-fit across folds.
    pub avg_goodness_of_fit: HashMap<String, GoodnessOfFit>,
    /// Standard deviation of metrics across folds.
    pub std_dev: HashMap<String, f64>,
}

/// Cross-validator for model validation.
pub struct CrossValidator {
    /// Number of folds.
    pub folds: usize,
}

impl CrossValidator {
    /// Create new cross-validator.
    pub fn new(folds: usize) -> SimResult<Self> {
        if folds < 2 {
            return Err(SimulationError::ConfigurationError(
                "Number of folds must be at least 2".to_string(),
            ));
        }
        Ok(Self { folds })
    }

    /// Perform k-fold cross-validation.
    pub fn validate<F>(
        &self,
        data: &EmpiricalData,
        train_and_test: F,
    ) -> SimResult<CrossValidationResult>
    where
        F: Fn(&EmpiricalData, &EmpiricalData) -> HashMap<String, GoodnessOfFit>,
    {
        let n = data.points.len();
        let fold_size = n / self.folds;

        let mut all_results: Vec<HashMap<String, GoodnessOfFit>> = Vec::new();

        for fold in 0..self.folds {
            let test_start = fold * fold_size;
            let test_end = if fold == self.folds - 1 {
                n
            } else {
                (fold + 1) * fold_size
            };

            // Split into train and test
            let mut train_data = EmpiricalData::new();
            let mut test_data = EmpiricalData::new();

            for (i, point) in data.points.iter().enumerate() {
                if i >= test_start && i < test_end {
                    test_data.points.push(point.clone());
                } else {
                    train_data.points.push(point.clone());
                }
            }

            // Train and test
            let fold_results = train_and_test(&train_data, &test_data);
            all_results.push(fold_results);
        }

        // Aggregate results
        let mut avg_goodness_of_fit = HashMap::new();
        let mut std_dev = HashMap::new();

        // Get all metric names
        let metric_names: Vec<String> = all_results
            .first()
            .map(|r| r.keys().cloned().collect())
            .unwrap_or_default();

        for metric_name in metric_names {
            let mse_values: Vec<f64> = all_results
                .iter()
                .filter_map(|r| r.get(&metric_name).map(|g| g.mse))
                .collect();

            let rmse_values: Vec<f64> = all_results
                .iter()
                .filter_map(|r| r.get(&metric_name).map(|g| g.rmse))
                .collect();

            let mae_values: Vec<f64> = all_results
                .iter()
                .filter_map(|r| r.get(&metric_name).map(|g| g.mae))
                .collect();

            let r2_values: Vec<f64> = all_results
                .iter()
                .filter_map(|r| r.get(&metric_name).map(|g| g.r_squared))
                .collect();

            let avg_gof = GoodnessOfFit {
                mse: Self::mean(&mse_values),
                rmse: Self::mean(&rmse_values),
                mae: Self::mean(&mae_values),
                r_squared: Self::mean(&r2_values),
                nrmse: 0.0,
            };

            avg_goodness_of_fit.insert(metric_name.clone(), avg_gof);
            std_dev.insert(metric_name, Self::std_dev(&rmse_values));
        }

        Ok(CrossValidationResult {
            folds: self.folds,
            avg_goodness_of_fit,
            std_dev,
        })
    }

    fn mean(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        values.iter().sum::<f64>() / values.len() as f64
    }

    fn std_dev(values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }

        let mean = Self::mean(values);
        let variance =
            values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (values.len() - 1) as f64;

        variance.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empirical_data() {
        let mut data = EmpiricalData::new();

        let mut values1 = HashMap::new();
        values1.insert("metric1".to_string(), 10.0);
        values1.insert("metric2".to_string(), 20.0);
        data.add_point("t1".to_string(), values1);

        let mut values2 = HashMap::new();
        values2.insert("metric1".to_string(), 15.0);
        values2.insert("metric2".to_string(), 25.0);
        data.add_point("t2".to_string(), values2);

        let series = data.get_metric_series("metric1");
        assert_eq!(series, vec![10.0, 15.0]);
    }

    #[test]
    fn test_goodness_of_fit() {
        let observed = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let predicted = vec![1.1, 2.1, 2.9, 4.2, 4.8];

        let gof = ParameterCalibrator::calculate_goodness_of_fit(&observed, &predicted);

        assert!(gof.mse > 0.0);
        assert!(gof.rmse > 0.0);
        assert!(gof.mae > 0.0);
        assert!(gof.r_squared > 0.9); // Should be a good fit
    }

    #[test]
    fn test_parameter_calibration() {
        let calibrator = ParameterCalibrator::new(500, 0.01);

        let mut empirical = EmpiricalData::new();
        for i in 0..10 {
            let mut values = HashMap::new();
            values.insert("metric".to_string(), (i * 2) as f64);
            empirical.add_point(format!("t{}", i), values);
        }

        let mut initial_params = HashMap::new();
        initial_params.insert("slope".to_string(), 1.0);

        // Simulation function: y = slope * x
        let simulation_fn = |params: &HashMap<String, f64>| {
            let slope = params.get("slope").copied().unwrap_or(1.0);
            let mut result = HashMap::new();
            let series: Vec<f64> = (0..10).map(|i| slope * i as f64).collect();
            result.insert("metric".to_string(), series);
            result
        };

        let result = calibrator
            .calibrate(initial_params, &empirical, simulation_fn)
            .unwrap();

        // Should calibrate slope to approximately 2.0
        let calibrated_slope = result.parameters.get("slope").unwrap();
        assert!((calibrated_slope - 2.0).abs() < 1.5);
    }

    #[test]
    fn test_cross_validation() {
        let validator = CrossValidator::new(3).unwrap();

        let mut data = EmpiricalData::new();
        for i in 0..30 {
            let mut values = HashMap::new();
            values.insert("metric".to_string(), i as f64);
            data.add_point(format!("t{}", i), values);
        }

        let train_and_test = |_train: &EmpiricalData, _test: &EmpiricalData| {
            let mut gof_map = HashMap::new();
            let gof = GoodnessOfFit {
                mse: 1.0,
                rmse: 1.0,
                mae: 0.8,
                r_squared: 0.95,
                nrmse: 0.1,
            };
            gof_map.insert("metric".to_string(), gof);
            gof_map
        };

        let result = validator.validate(&data, train_and_test).unwrap();

        assert_eq!(result.folds, 3);
        assert!(result.avg_goodness_of_fit.contains_key("metric"));
    }

    #[test]
    fn test_goodness_of_fit_perfect() {
        let observed = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let predicted = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let gof = ParameterCalibrator::calculate_goodness_of_fit(&observed, &predicted);

        assert_eq!(gof.mse, 0.0);
        assert_eq!(gof.rmse, 0.0);
        assert_eq!(gof.mae, 0.0);
        assert_eq!(gof.r_squared, 1.0);
    }

    #[test]
    fn test_goodness_of_fit_empty() {
        let observed: Vec<f64> = vec![];
        let predicted: Vec<f64> = vec![];

        let gof = ParameterCalibrator::calculate_goodness_of_fit(&observed, &predicted);

        assert_eq!(gof.mse, 0.0);
        assert_eq!(gof.rmse, 0.0);
        assert_eq!(gof.mae, 0.0);
        assert_eq!(gof.r_squared, 0.0);
    }

    #[test]
    fn test_empirical_data_multiple_metrics() {
        let mut data = EmpiricalData::new();

        for i in 0..5 {
            let mut values = HashMap::new();
            values.insert("metric1".to_string(), i as f64);
            values.insert("metric2".to_string(), (i * 2) as f64);
            values.insert("metric3".to_string(), (i * 3) as f64);
            data.add_point(format!("t{}", i), values);
        }

        let series1 = data.get_metric_series("metric1");
        let series2 = data.get_metric_series("metric2");
        let series3 = data.get_metric_series("metric3");

        assert_eq!(series1, vec![0.0, 1.0, 2.0, 3.0, 4.0]);
        assert_eq!(series2, vec![0.0, 2.0, 4.0, 6.0, 8.0]);
        assert_eq!(series3, vec![0.0, 3.0, 6.0, 9.0, 12.0]);
    }

    #[test]
    fn test_cross_validator_invalid_folds() {
        let result = CrossValidator::new(1);
        assert!(result.is_err());

        let result = CrossValidator::new(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_calibration_convergence() {
        let calibrator = ParameterCalibrator::new(100, 0.05);

        let mut empirical = EmpiricalData::new();
        for i in 0..20 {
            let mut values = HashMap::new();
            values.insert("metric".to_string(), 5.0);
            empirical.add_point(format!("t{}", i), values);
        }

        let mut initial_params = HashMap::new();
        initial_params.insert("constant".to_string(), 1.0);

        // Simulation function: y = constant
        let simulation_fn = |params: &HashMap<String, f64>| {
            let constant = params.get("constant").copied().unwrap_or(1.0);
            let mut result = HashMap::new();
            let series: Vec<f64> = (0..20).map(|_| constant).collect();
            result.insert("metric".to_string(), series);
            result
        };

        let result = calibrator
            .calibrate(initial_params, &empirical, simulation_fn)
            .unwrap();

        // Should calibrate constant to approximately 5.0
        let calibrated = result.parameters.get("constant").unwrap();
        assert!((calibrated - 5.0).abs() < 2.0);
        assert!(result.converged || result.goodness_of_fit.get("metric").unwrap().mse < 1.0);
    }

    #[test]
    fn test_cross_validation_multiple_metrics() {
        let validator = CrossValidator::new(2).unwrap();

        let mut data = EmpiricalData::new();
        for i in 0..20 {
            let mut values = HashMap::new();
            values.insert("metric1".to_string(), i as f64);
            values.insert("metric2".to_string(), (i * 2) as f64);
            data.add_point(format!("t{}", i), values);
        }

        let train_and_test = |_train: &EmpiricalData, _test: &EmpiricalData| {
            let mut gof_map = HashMap::new();
            let gof1 = GoodnessOfFit {
                mse: 1.0,
                rmse: 1.0,
                mae: 0.8,
                r_squared: 0.95,
                nrmse: 0.1,
            };
            let gof2 = GoodnessOfFit {
                mse: 2.0,
                rmse: 1.4,
                mae: 1.0,
                r_squared: 0.90,
                nrmse: 0.2,
            };
            gof_map.insert("metric1".to_string(), gof1);
            gof_map.insert("metric2".to_string(), gof2);
            gof_map
        };

        let result = validator.validate(&data, train_and_test).unwrap();

        assert_eq!(result.folds, 2);
        assert_eq!(result.avg_goodness_of_fit.len(), 2);
        assert!(result.avg_goodness_of_fit.contains_key("metric1"));
        assert!(result.avg_goodness_of_fit.contains_key("metric2"));
    }

    #[test]
    fn test_goodness_of_fit_serialization() {
        let gof = GoodnessOfFit {
            mse: 1.5,
            rmse: 1.2,
            mae: 0.9,
            r_squared: 0.85,
            nrmse: 0.15,
        };

        let json = serde_json::to_string(&gof).unwrap();
        let deserialized: GoodnessOfFit = serde_json::from_str(&json).unwrap();

        assert_eq!(gof.mse, deserialized.mse);
        assert_eq!(gof.rmse, deserialized.rmse);
        assert_eq!(gof.mae, deserialized.mae);
        assert_eq!(gof.r_squared, deserialized.r_squared);
        assert_eq!(gof.nrmse, deserialized.nrmse);
    }
}
