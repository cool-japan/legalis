//! Validation Framework Module
//!
//! This module provides comprehensive validation tools for simulation models including:
//! - Empirical validation against real-world data
//! - Cross-validation with holdout sets
//! - Confidence interval reporting
//! - Uncertainty quantification
//! - Automated model calibration

use crate::{CalibrationResult, GoodnessOfFit, SimResult, SimulationError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Empirical data point for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpiricalDataPoint {
    /// Time period or identifier
    pub period: String,
    /// Observed value from real-world data
    pub observed: f64,
    /// Optional standard error of observation
    pub standard_error: Option<f64>,
}

/// Empirical validation dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpiricalDataset {
    /// Name of the metric being validated
    pub metric_name: String,
    /// Description of the data source
    pub description: String,
    /// Data points
    pub data: Vec<EmpiricalDataPoint>,
}

impl EmpiricalDataset {
    /// Create a new empirical dataset
    pub fn new(metric_name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            metric_name: metric_name.into(),
            description: description.into(),
            data: Vec::new(),
        }
    }

    /// Add a data point
    pub fn add_point(&mut self, period: impl Into<String>, observed: f64) {
        self.data.push(EmpiricalDataPoint {
            period: period.into(),
            observed,
            standard_error: None,
        });
    }

    /// Add a data point with standard error
    pub fn add_point_with_error(
        &mut self,
        period: impl Into<String>,
        observed: f64,
        standard_error: f64,
    ) {
        self.data.push(EmpiricalDataPoint {
            period: period.into(),
            observed,
            standard_error: Some(standard_error),
        });
    }

    /// Get observed values
    pub fn observed_values(&self) -> Vec<f64> {
        self.data.iter().map(|d| d.observed).collect()
    }
}

/// Validation result comparing simulated vs empirical data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Metric being validated
    pub metric_name: String,
    /// Simulated values
    pub simulated: Vec<f64>,
    /// Observed (empirical) values
    pub observed: Vec<f64>,
    /// Goodness of fit metrics
    pub fit: GoodnessOfFit,
    /// Whether the model passes validation (based on thresholds)
    pub passes: bool,
    /// Validation message
    pub message: String,
}

impl ValidationResult {
    /// Generate a validation report
    pub fn report(&self) -> String {
        let mut report = String::from("Validation Report\n");
        report.push_str("=================\n\n");
        report.push_str(&format!("Metric: {}\n", self.metric_name));
        report.push_str(&format!(
            "Status: {}\n",
            if self.passes { "PASS" } else { "FAIL" }
        ));
        report.push_str(&format!("Message: {}\n\n", self.message));
        report.push_str("Goodness of Fit:\n");
        report.push_str(&format!("  MSE: {:.4}\n", self.fit.mse));
        report.push_str(&format!("  RMSE: {:.4}\n", self.fit.rmse));
        report.push_str(&format!("  MAE: {:.4}\n", self.fit.mae));
        report.push_str(&format!("  R²: {:.4}\n", self.fit.r_squared));
        report
    }
}

/// Empirical validator
#[derive(Debug, Clone)]
pub struct EmpiricalValidator {
    /// Threshold for R² to pass validation
    r_squared_threshold: f64,
    /// Threshold for RMSE to pass validation (as percentage of mean)
    rmse_threshold_pct: f64,
}

impl EmpiricalValidator {
    /// Create a new empirical validator with default thresholds
    pub fn new() -> Self {
        Self {
            r_squared_threshold: 0.8,
            rmse_threshold_pct: 20.0,
        }
    }

    /// Set R² threshold
    pub fn with_r_squared_threshold(mut self, threshold: f64) -> Self {
        self.r_squared_threshold = threshold;
        self
    }

    /// Set RMSE threshold as percentage of mean
    pub fn with_rmse_threshold_pct(mut self, threshold: f64) -> Self {
        self.rmse_threshold_pct = threshold;
        self
    }

    /// Validate simulated data against empirical data
    pub fn validate(
        &self,
        metric_name: impl Into<String>,
        simulated: Vec<f64>,
        empirical: &EmpiricalDataset,
    ) -> SimResult<ValidationResult> {
        let metric_name = metric_name.into();
        let observed = empirical.observed_values();

        if simulated.len() != observed.len() {
            return Err(SimulationError::InvalidConfiguration(
                "Simulated and observed data must have the same length".to_string(),
            ));
        }

        let fit = Self::calculate_fit(&simulated, &observed);

        // Check thresholds
        let mean_observed = observed.iter().sum::<f64>() / observed.len() as f64;
        let rmse_pct = (fit.rmse / mean_observed.abs()) * 100.0;

        let passes =
            fit.r_squared >= self.r_squared_threshold && rmse_pct <= self.rmse_threshold_pct;

        let message = if passes {
            format!(
                "Model validates successfully (R²={:.3}, RMSE={:.1}% of mean)",
                fit.r_squared, rmse_pct
            )
        } else {
            let mut reasons = Vec::new();
            if fit.r_squared < self.r_squared_threshold {
                reasons.push(format!(
                    "R²={:.3} < {:.3}",
                    fit.r_squared, self.r_squared_threshold
                ));
            }
            if rmse_pct > self.rmse_threshold_pct {
                reasons.push(format!(
                    "RMSE={:.1}% > {:.1}%",
                    rmse_pct, self.rmse_threshold_pct
                ));
            }
            format!("Validation failed: {}", reasons.join(", "))
        };

        Ok(ValidationResult {
            metric_name,
            simulated,
            observed,
            fit,
            passes,
            message,
        })
    }
}

impl Default for EmpiricalValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl EmpiricalValidator {
    /// Calculate goodness of fit metrics
    fn calculate_fit(simulated: &[f64], observed: &[f64]) -> GoodnessOfFit {
        let n = simulated.len() as f64;

        // Calculate MSE, MAE
        let mut mse = 0.0;
        let mut mae = 0.0;
        for i in 0..simulated.len() {
            let error = simulated[i] - observed[i];
            mse += error * error;
            mae += error.abs();
        }
        mse /= n;
        mae /= n;

        let rmse = mse.sqrt();

        // Calculate R²
        let mean_observed = observed.iter().sum::<f64>() / n;
        let ss_tot: f64 = observed.iter().map(|y| (y - mean_observed).powi(2)).sum();
        let ss_res: f64 = simulated
            .iter()
            .zip(observed.iter())
            .map(|(pred, obs)| (obs - pred).powi(2))
            .sum();

        let r_squared = if ss_tot > 0.0 {
            1.0 - (ss_res / ss_tot)
        } else {
            0.0
        };

        // Calculate normalized RMSE
        let range = observed.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
            - observed.iter().fold(f64::INFINITY, |a, &b| a.min(b));
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

/// K-fold validation fold
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KFoldValidationFold {
    /// Fold number
    pub fold: usize,
    /// Training set indices
    pub train_indices: Vec<usize>,
    /// Test set indices
    pub test_indices: Vec<usize>,
    /// Training error
    pub train_error: f64,
    /// Test error (out-of-sample)
    pub test_error: f64,
}

/// K-fold validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KFoldValidationResult {
    /// Number of folds
    pub n_folds: usize,
    /// Results for each fold
    pub folds: Vec<KFoldValidationFold>,
    /// Mean training error
    pub mean_train_error: f64,
    /// Mean test error
    pub mean_test_error: f64,
    /// Standard deviation of test error
    pub std_test_error: f64,
    /// Whether the model shows overfitting (test >> train)
    pub overfitting_detected: bool,
}

impl KFoldValidationResult {
    /// Generate a K-fold validation report
    pub fn report(&self) -> String {
        let mut report = String::from("Cross-Validation Report\n");
        report.push_str("=======================\n\n");
        report.push_str(&format!("K-Fold: {}\n", self.n_folds));
        report.push_str(&format!(
            "Mean Training Error: {:.4}\n",
            self.mean_train_error
        ));
        report.push_str(&format!("Mean Test Error: {:.4}\n", self.mean_test_error));
        report.push_str(&format!("Std Test Error: {:.4}\n", self.std_test_error));

        if self.overfitting_detected {
            report.push_str("\nWARNING: Overfitting detected (test error >> training error)\n");
        } else {
            report.push_str("\nNo significant overfitting detected\n");
        }

        report.push_str("\nFold Details:\n");
        for fold in &self.folds {
            report.push_str(&format!(
                "  Fold {}: Train={:.4}, Test={:.4}\n",
                fold.fold, fold.train_error, fold.test_error
            ));
        }

        report
    }
}

/// K-fold validator
#[derive(Debug, Clone)]
pub struct KFoldValidator {
    n_folds: usize,
    shuffle: bool,
}

impl KFoldValidator {
    /// Create a new K-fold validator
    pub fn new(n_folds: usize) -> Self {
        Self {
            n_folds,
            shuffle: true,
        }
    }

    /// Set whether to shuffle data before splitting
    pub fn with_shuffle(mut self, shuffle: bool) -> Self {
        self.shuffle = shuffle;
        self
    }

    /// Generate fold indices
    pub fn generate_folds(&self, n_samples: usize) -> Vec<(Vec<usize>, Vec<usize>)> {
        let mut indices: Vec<usize> = (0..n_samples).collect();

        if self.shuffle {
            // Simple deterministic shuffle for reproducibility
            for i in 0..indices.len() {
                let j = (i * 17 + 31) % indices.len();
                indices.swap(i, j);
            }
        }

        let fold_size = n_samples / self.n_folds;
        let mut folds = Vec::new();

        for fold in 0..self.n_folds {
            let test_start = fold * fold_size;
            let test_end = if fold == self.n_folds - 1 {
                n_samples
            } else {
                (fold + 1) * fold_size
            };

            let test_indices: Vec<usize> = indices[test_start..test_end].to_vec();
            let train_indices: Vec<usize> = indices[0..test_start]
                .iter()
                .chain(indices[test_end..].iter())
                .copied()
                .collect();

            folds.push((train_indices, test_indices));
        }

        folds
    }

    /// Perform K-fold validation with custom error function
    pub fn validate<F>(&self, data: &[f64], error_fn: F) -> KFoldValidationResult
    where
        F: Fn(&[f64], &[f64]) -> f64,
    {
        let folds = self.generate_folds(data.len());
        let mut cv_folds = Vec::new();

        for (fold_idx, (train_indices, test_indices)) in folds.iter().enumerate() {
            let train_data: Vec<f64> = train_indices.iter().map(|&i| data[i]).collect();
            let test_data: Vec<f64> = test_indices.iter().map(|&i| data[i]).collect();

            let train_error = error_fn(&train_data, &train_data);
            let test_error = error_fn(&test_data, &test_data);

            cv_folds.push(KFoldValidationFold {
                fold: fold_idx,
                train_indices: train_indices.clone(),
                test_indices: test_indices.clone(),
                train_error,
                test_error,
            });
        }

        let mean_train_error =
            cv_folds.iter().map(|f| f.train_error).sum::<f64>() / cv_folds.len() as f64;
        let mean_test_error =
            cv_folds.iter().map(|f| f.test_error).sum::<f64>() / cv_folds.len() as f64;

        let variance = cv_folds
            .iter()
            .map(|f| (f.test_error - mean_test_error).powi(2))
            .sum::<f64>()
            / cv_folds.len() as f64;
        let std_test_error = variance.sqrt();

        // Detect overfitting: test error is significantly higher than train error
        let overfitting_detected = mean_test_error > mean_train_error * 1.5;

        KFoldValidationResult {
            n_folds: self.n_folds,
            folds: cv_folds,
            mean_train_error,
            mean_test_error,
            std_test_error,
            overfitting_detected,
        }
    }
}

/// Confidence interval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    /// Point estimate
    pub estimate: f64,
    /// Lower bound
    pub lower: f64,
    /// Upper bound
    pub upper: f64,
    /// Confidence level (e.g., 0.95 for 95%)
    pub confidence_level: f64,
}

impl ConfidenceInterval {
    /// Create a new confidence interval
    pub fn new(estimate: f64, lower: f64, upper: f64, confidence_level: f64) -> Self {
        Self {
            estimate,
            lower,
            upper,
            confidence_level,
        }
    }

    /// Get the margin of error
    pub fn margin_of_error(&self) -> f64 {
        (self.upper - self.lower) / 2.0
    }

    /// Get the width of the interval
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }

    /// Check if a value is within the confidence interval
    pub fn contains(&self, value: f64) -> bool {
        value >= self.lower && value <= self.upper
    }
}

/// Confidence interval calculator
#[derive(Debug, Clone)]
pub struct ConfidenceIntervalCalculator;

impl ConfidenceIntervalCalculator {
    /// Calculate confidence interval for a sample mean (using t-distribution)
    pub fn for_mean(data: &[f64], confidence_level: f64) -> SimResult<ConfidenceInterval> {
        if data.is_empty() {
            return Err(SimulationError::InvalidConfiguration(
                "Cannot calculate confidence interval for empty data".to_string(),
            ));
        }

        let n = data.len() as f64;
        let mean = data.iter().sum::<f64>() / n;

        if data.len() == 1 {
            return Ok(ConfidenceInterval::new(mean, mean, mean, confidence_level));
        }

        let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
        let std_error = (variance / n).sqrt();

        // Use normal approximation for large samples, t-distribution for small
        let z_score = if n > 30.0 {
            Self::z_score(confidence_level)
        } else {
            Self::t_score(confidence_level, data.len() - 1)
        };

        let margin = z_score * std_error;

        Ok(ConfidenceInterval::new(
            mean,
            mean - margin,
            mean + margin,
            confidence_level,
        ))
    }

    /// Calculate confidence interval for a proportion
    pub fn for_proportion(
        successes: usize,
        total: usize,
        confidence_level: f64,
    ) -> SimResult<ConfidenceInterval> {
        if total == 0 {
            return Err(SimulationError::InvalidConfiguration(
                "Total must be greater than 0".to_string(),
            ));
        }

        let p = successes as f64 / total as f64;
        let n = total as f64;
        let z = Self::z_score(confidence_level);

        let std_error = ((p * (1.0 - p)) / n).sqrt();
        let margin = z * std_error;

        Ok(ConfidenceInterval::new(
            p,
            (p - margin).max(0.0),
            (p + margin).min(1.0),
            confidence_level,
        ))
    }

    /// Get z-score for a given confidence level (normal distribution)
    fn z_score(confidence_level: f64) -> f64 {
        // Common z-scores
        match (confidence_level * 100.0).round() as i32 {
            90 => 1.645,
            95 => 1.96,
            99 => 2.576,
            _ => 1.96, // Default to 95%
        }
    }

    /// Get t-score for a given confidence level and degrees of freedom
    fn t_score(confidence_level: f64, df: usize) -> f64 {
        // Simplified t-scores for common confidence levels and df
        // For production, use a proper t-distribution library
        match (confidence_level * 100.0).round() as i32 {
            95 => {
                if df < 30 {
                    2.0 + 0.5 / (df as f64).sqrt()
                } else {
                    1.96
                }
            }
            99 => {
                if df < 30 {
                    2.6 + 0.8 / (df as f64).sqrt()
                } else {
                    2.576
                }
            }
            _ => 2.0,
        }
    }
}

/// Uncertainty quantification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyQuantification {
    /// Metric name
    pub metric: String,
    /// Mean value
    pub mean: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Coefficient of variation (std/mean)
    pub cv: f64,
    /// 95% confidence interval
    pub ci_95: ConfidenceInterval,
    /// Uncertainty level categorization
    pub uncertainty_level: String,
}

impl UncertaintyQuantification {
    /// Create a new uncertainty quantification from data
    pub fn from_data(metric: impl Into<String>, data: &[f64]) -> SimResult<Self> {
        let metric = metric.into();
        let n = data.len() as f64;

        if data.is_empty() {
            return Err(SimulationError::InvalidConfiguration(
                "Cannot quantify uncertainty for empty data".to_string(),
            ));
        }

        let mean = data.iter().sum::<f64>() / n;
        let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
        let std_dev = variance.sqrt();
        let cv = if mean.abs() > 1e-10 {
            std_dev / mean.abs()
        } else {
            0.0
        };

        let ci_95 = ConfidenceIntervalCalculator::for_mean(data, 0.95)?;

        let uncertainty_level = if cv < 0.1 {
            "Low".to_string()
        } else if cv < 0.3 {
            "Moderate".to_string()
        } else if cv < 0.5 {
            "High".to_string()
        } else {
            "Very High".to_string()
        };

        Ok(Self {
            metric,
            mean,
            std_dev,
            cv,
            ci_95,
            uncertainty_level,
        })
    }

    /// Generate an uncertainty report
    pub fn report(&self) -> String {
        let mut report = String::from("Uncertainty Quantification\n");
        report.push_str("==========================\n\n");
        report.push_str(&format!("Metric: {}\n", self.metric));
        report.push_str(&format!("Mean: {:.4}\n", self.mean));
        report.push_str(&format!("Std Dev: {:.4}\n", self.std_dev));
        report.push_str(&format!("CV: {:.2}%\n", self.cv * 100.0));
        report.push_str(&format!(
            "Uncertainty Level: {}\n\n",
            self.uncertainty_level
        ));
        report.push_str(&format!(
            "95% CI: [{:.4}, {:.4}]\n",
            self.ci_95.lower, self.ci_95.upper
        ));
        report.push_str(&format!(
            "Margin of Error: {:.4}\n",
            self.ci_95.margin_of_error()
        ));
        report
    }
}

/// Automated calibration configuration
#[derive(Debug, Clone)]
pub struct AutoCalibrationConfig {
    /// Target metrics to calibrate against
    pub target_metrics: HashMap<String, f64>,
    /// Parameter search ranges
    pub param_ranges: HashMap<String, (f64, f64)>,
    /// Maximum iterations
    pub max_iterations: usize,
    /// Convergence tolerance
    pub tolerance: f64,
}

impl AutoCalibrationConfig {
    /// Create a new auto-calibration config
    pub fn new() -> Self {
        Self {
            target_metrics: HashMap::new(),
            param_ranges: HashMap::new(),
            max_iterations: 100,
            tolerance: 0.01,
        }
    }

    /// Add a target metric
    pub fn add_target(&mut self, metric: impl Into<String>, target: f64) {
        self.target_metrics.insert(metric.into(), target);
    }

    /// Add a parameter range
    pub fn add_param_range(&mut self, param: impl Into<String>, min: f64, max: f64) {
        self.param_ranges.insert(param.into(), (min, max));
    }

    /// Set maximum iterations
    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Set tolerance
    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }
}

impl Default for AutoCalibrationConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Auto-calibration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoCalibrationResult {
    /// Calibrated parameters
    pub parameters: HashMap<String, f64>,
    /// Final error
    pub final_error: f64,
    /// Number of iterations performed
    pub iterations: usize,
    /// Whether calibration converged
    pub converged: bool,
    /// Calibration quality assessment
    pub quality: CalibrationResult,
}

impl AutoCalibrationResult {
    /// Generate a calibration report
    pub fn report(&self) -> String {
        let mut report = String::from("Auto-Calibration Report\n");
        report.push_str("=======================\n\n");
        report.push_str(&format!(
            "Status: {}\n",
            if self.converged {
                "Converged"
            } else {
                "Did not converge"
            }
        ));
        report.push_str(&format!("Iterations: {}\n", self.iterations));
        report.push_str(&format!("Final Error: {:.6}\n\n", self.final_error));

        report.push_str("Calibrated Parameters:\n");
        for (param, value) in &self.parameters {
            report.push_str(&format!("  {}: {:.6}\n", param, value));
        }

        report.push_str(&format!(
            "\nGoodness of Fit: {} metrics\n",
            self.quality.goodness_of_fit.len()
        ));

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empirical_dataset() {
        let mut dataset = EmpiricalDataset::new("gdp", "GDP growth data");
        dataset.add_point("2020", 2.5);
        dataset.add_point("2021", 3.2);
        dataset.add_point_with_error("2022", 2.8, 0.3);

        assert_eq!(dataset.data.len(), 3);
        assert_eq!(dataset.observed_values(), vec![2.5, 3.2, 2.8]);
    }

    #[test]
    fn test_empirical_validator() {
        let validator = EmpiricalValidator::new()
            .with_r_squared_threshold(0.7)
            .with_rmse_threshold_pct(25.0);

        let mut dataset = EmpiricalDataset::new("test", "Test data");
        dataset.add_point("1", 10.0);
        dataset.add_point("2", 20.0);
        dataset.add_point("3", 30.0);

        let simulated = vec![11.0, 19.0, 31.0];

        let result = validator.validate("test", simulated, &dataset).unwrap();
        assert!(result.passes);
        assert!(result.fit.r_squared > 0.9);
    }

    #[test]
    fn test_empirical_validator_failure() {
        let validator = EmpiricalValidator::new().with_r_squared_threshold(0.9);

        let mut dataset = EmpiricalDataset::new("test", "Test data");
        dataset.add_point("1", 10.0);
        dataset.add_point("2", 20.0);
        dataset.add_point("3", 30.0);

        let simulated = vec![5.0, 25.0, 35.0]; // Poor fit

        let result = validator.validate("test", simulated, &dataset).unwrap();
        assert!(!result.passes);
    }

    #[test]
    fn test_kfold_validator_folds() {
        let cv = KFoldValidator::new(5);
        let folds = cv.generate_folds(100);

        assert_eq!(folds.len(), 5);

        for (train, test) in &folds {
            assert_eq!(train.len() + test.len(), 100);
        }
    }

    #[test]
    fn test_kfold_validation() {
        let cv = KFoldValidator::new(3);
        let data: Vec<f64> = (0..30).map(|i| i as f64).collect();

        let result = cv.validate(&data, |train, _test| {
            // Simple error function: variance
            let mean = train.iter().sum::<f64>() / train.len() as f64;
            train.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / train.len() as f64
        });

        assert_eq!(result.n_folds, 3);
        assert_eq!(result.folds.len(), 3);
        assert!(result.mean_train_error > 0.0);
    }

    #[test]
    fn test_confidence_interval_mean() {
        let data = vec![10.0, 12.0, 11.0, 13.0, 10.5, 12.5];
        let ci = ConfidenceIntervalCalculator::for_mean(&data, 0.95).unwrap();

        assert!((ci.estimate - 11.5).abs() < 0.1);
        assert!(ci.lower < ci.estimate);
        assert!(ci.upper > ci.estimate);
        assert!(ci.contains(11.5));
        assert!(!ci.contains(20.0));
    }

    #[test]
    fn test_confidence_interval_proportion() {
        let ci = ConfidenceIntervalCalculator::for_proportion(60, 100, 0.95).unwrap();

        assert!((ci.estimate - 0.6).abs() < 0.01);
        assert!(ci.lower < 0.6);
        assert!(ci.upper > 0.6);
        assert!(ci.contains(0.6));
    }

    #[test]
    fn test_confidence_interval_width() {
        let ci = ConfidenceInterval::new(10.0, 8.0, 12.0, 0.95);
        assert_eq!(ci.width(), 4.0);
        assert_eq!(ci.margin_of_error(), 2.0);
    }

    #[test]
    fn test_uncertainty_quantification() {
        let data = vec![10.0, 11.0, 9.0, 10.5, 10.2, 9.8];
        let uq = UncertaintyQuantification::from_data("test_metric", &data).unwrap();

        assert!((uq.mean - 10.08).abs() < 0.1);
        assert!(uq.std_dev > 0.0);
        assert!(uq.cv > 0.0);
        assert_eq!(uq.uncertainty_level, "Low");
    }

    #[test]
    fn test_uncertainty_levels() {
        // Low uncertainty
        let data_low = vec![10.0, 10.1, 9.9, 10.05, 9.95];
        let uq_low = UncertaintyQuantification::from_data("low", &data_low).unwrap();
        assert_eq!(uq_low.uncertainty_level, "Low");

        // High uncertainty
        let data_high = vec![10.0, 20.0, 5.0, 15.0, 8.0];
        let uq_high = UncertaintyQuantification::from_data("high", &data_high).unwrap();
        assert!(uq_high.uncertainty_level == "High" || uq_high.uncertainty_level == "Very High");
    }

    #[test]
    fn test_auto_calibration_config() {
        let mut config = AutoCalibrationConfig::new();
        config.add_target("revenue", 100000.0);
        config.add_target("compliance", 0.95);
        config.add_param_range("tax_rate", 0.1, 0.4);
        config.add_param_range("threshold", 1000.0, 5000.0);

        assert_eq!(config.target_metrics.len(), 2);
        assert_eq!(config.param_ranges.len(), 2);
        assert_eq!(config.param_ranges.get("tax_rate"), Some(&(0.1, 0.4)));
    }

    #[test]
    fn test_validation_result_report() {
        let result = ValidationResult {
            metric_name: "test".to_string(),
            simulated: vec![1.0, 2.0, 3.0],
            observed: vec![1.1, 2.0, 2.9],
            fit: GoodnessOfFit {
                mse: 0.01,
                rmse: 0.1,
                mae: 0.05,
                r_squared: 0.95,
                nrmse: 0.05,
            },
            passes: true,
            message: "Good fit".to_string(),
        };

        let report = result.report();
        assert!(report.contains("PASS"));
        assert!(report.contains("test"));
    }

    #[test]
    fn test_kfold_validation_report() {
        let result = KFoldValidationResult {
            n_folds: 3,
            folds: vec![KFoldValidationFold {
                fold: 0,
                train_indices: vec![],
                test_indices: vec![],
                train_error: 0.5,
                test_error: 0.6,
            }],
            mean_train_error: 0.5,
            mean_test_error: 0.6,
            std_test_error: 0.1,
            overfitting_detected: false,
        };

        let report = result.report();
        assert!(report.contains("3"));
        assert!(report.contains("0.5"));
    }

    #[test]
    fn test_uncertainty_quantification_report() {
        let data = vec![10.0, 11.0, 9.0];
        let uq = UncertaintyQuantification::from_data("metric", &data).unwrap();
        let report = uq.report();

        assert!(report.contains("Uncertainty Quantification"));
        assert!(report.contains("metric"));
        assert!(report.contains("95% CI"));
    }

    #[test]
    fn test_empty_data_error() {
        let result = ConfidenceIntervalCalculator::for_mean(&[], 0.95);
        assert!(result.is_err());

        let result = UncertaintyQuantification::from_data("test", &[]);
        assert!(result.is_err());
    }
}

// ============================================================================
// Historical Data Backtesting (v0.2.9)
// ============================================================================

/// Time series data point for backtesting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestTimeSeriesPoint {
    /// Time period (e.g., "2020-Q1", "Jan 2021")
    pub period: String,
    /// Actual observed value
    pub actual: f64,
    /// Features/predictors for this period
    pub features: HashMap<String, f64>,
}

/// Historical backtest configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestConfig {
    /// Minimum training window size
    pub min_train_size: usize,
    /// Test window size (number of periods to forecast)
    pub test_size: usize,
    /// Step size for rolling window (1 = every period)
    pub step_size: usize,
    /// Whether to use expanding window (true) or rolling window (false)
    pub expanding_window: bool,
}

impl Default for BacktestConfig {
    fn default() -> Self {
        Self {
            min_train_size: 10,
            test_size: 1,
            step_size: 1,
            expanding_window: false,
        }
    }
}

/// Single backtest fold result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestFold {
    /// Fold number
    pub fold: usize,
    /// Training period range
    pub train_periods: Vec<String>,
    /// Test period range
    pub test_periods: Vec<String>,
    /// Predicted values
    pub predicted: Vec<f64>,
    /// Actual values
    pub actual: Vec<f64>,
    /// Prediction errors
    pub errors: Vec<f64>,
}

/// Backtest result with walk-forward validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    /// All folds
    pub folds: Vec<BacktestFold>,
    /// Overall prediction accuracy metrics
    pub accuracy: PredictionAccuracy,
    /// Number of folds executed
    pub num_folds: usize,
}

impl BacktestResult {
    /// Create a new backtest result
    pub fn new(folds: Vec<BacktestFold>) -> SimResult<Self> {
        if folds.is_empty() {
            return Err(SimulationError::InvalidParameter(
                "No backtest folds provided".to_string(),
            ));
        }

        let mut all_predicted = Vec::new();
        let mut all_actual = Vec::new();

        for fold in &folds {
            all_predicted.extend(&fold.predicted);
            all_actual.extend(&fold.actual);
        }

        let accuracy = PredictionAccuracy::calculate(&all_predicted, &all_actual)?;
        let num_folds = folds.len();

        Ok(Self {
            folds,
            accuracy,
            num_folds,
        })
    }

    /// Get average error across all folds
    pub fn average_error(&self) -> f64 {
        if self.folds.is_empty() {
            return 0.0;
        }
        let total_error: f64 = self
            .folds
            .iter()
            .flat_map(|f| &f.errors)
            .map(|e| e.abs())
            .sum();
        let total_points: usize = self.folds.iter().map(|f| f.errors.len()).sum();
        if total_points > 0 {
            total_error / total_points as f64
        } else {
            0.0
        }
    }

    /// Generate backtest report
    pub fn report(&self) -> String {
        let mut report = String::from("Backtest Report\n");
        report.push_str("===============\n\n");
        report.push_str(&format!("Number of folds: {}\n", self.num_folds));
        report.push_str(&format!("Average error: {:.4}\n\n", self.average_error()));
        report.push_str(&self.accuracy.report());
        report
    }
}

/// Historical backtester for time series validation
#[derive(Debug, Clone, Default)]
pub struct HistoricalBacktester {
    /// Configuration
    pub config: BacktestConfig,
}

impl HistoricalBacktester {
    /// Create a new backtester
    pub fn new(config: BacktestConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    pub fn with_default_config() -> Self {
        Self {
            config: BacktestConfig::default(),
        }
    }

    /// Run backtest with a prediction function
    pub fn run<F>(
        &self,
        data: &[BacktestTimeSeriesPoint],
        mut predict_fn: F,
    ) -> SimResult<BacktestResult>
    where
        F: FnMut(&[BacktestTimeSeriesPoint]) -> Vec<f64>,
    {
        if data.len() < self.config.min_train_size + self.config.test_size {
            return Err(SimulationError::InvalidParameter(
                "Insufficient data for backtesting".to_string(),
            ));
        }

        let mut folds = Vec::new();
        let mut fold_num = 0;
        let mut train_end = self.config.min_train_size;

        while train_end + self.config.test_size <= data.len() {
            let train_start = if self.config.expanding_window {
                0
            } else {
                train_end.saturating_sub(self.config.min_train_size)
            };

            let train_data = &data[train_start..train_end];
            let test_start = train_end;
            let test_end = (train_end + self.config.test_size).min(data.len());
            let test_data = &data[test_start..test_end];

            let predicted = predict_fn(train_data);
            let actual: Vec<f64> = test_data.iter().map(|p| p.actual).collect();

            if predicted.len() != actual.len() {
                return Err(SimulationError::InvalidParameter(
                    "Prediction length mismatch".to_string(),
                ));
            }

            let errors: Vec<f64> = predicted.iter().zip(&actual).map(|(p, a)| p - a).collect();

            folds.push(BacktestFold {
                fold: fold_num,
                train_periods: train_data.iter().map(|p| p.period.clone()).collect(),
                test_periods: test_data.iter().map(|p| p.period.clone()).collect(),
                predicted,
                actual,
                errors,
            });

            fold_num += 1;
            train_end += self.config.step_size;
        }

        BacktestResult::new(folds)
    }
}

// ============================================================================
// Prediction Accuracy Metrics (v0.2.9)
// ============================================================================

/// Comprehensive prediction accuracy metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionAccuracy {
    /// Mean Absolute Error
    pub mae: f64,
    /// Mean Squared Error
    pub mse: f64,
    /// Root Mean Squared Error
    pub rmse: f64,
    /// Mean Absolute Percentage Error (%)
    pub mape: f64,
    /// Symmetric Mean Absolute Percentage Error (%)
    pub smape: f64,
    /// R-squared (coefficient of determination)
    pub r_squared: f64,
    /// Directional accuracy (% of correct direction predictions)
    pub directional_accuracy: f64,
    /// Number of predictions
    pub n: usize,
}

impl PredictionAccuracy {
    /// Calculate all accuracy metrics
    pub fn calculate(predicted: &[f64], actual: &[f64]) -> SimResult<Self> {
        if predicted.len() != actual.len() {
            return Err(SimulationError::InvalidParameter(
                "Predicted and actual lengths must match".to_string(),
            ));
        }

        if predicted.is_empty() {
            return Err(SimulationError::InvalidParameter(
                "Cannot calculate metrics on empty data".to_string(),
            ));
        }

        let n = predicted.len();

        // Calculate MAE
        let mae = predicted
            .iter()
            .zip(actual)
            .map(|(p, a)| (p - a).abs())
            .sum::<f64>()
            / n as f64;

        // Calculate MSE
        let mse = predicted
            .iter()
            .zip(actual)
            .map(|(p, a)| (p - a).powi(2))
            .sum::<f64>()
            / n as f64;

        // Calculate RMSE
        let rmse = mse.sqrt();

        // Calculate MAPE (avoid division by zero)
        let mape = if actual.iter().all(|&a| a != 0.0) {
            predicted
                .iter()
                .zip(actual)
                .map(|(p, a)| ((p - a).abs() / a.abs()) * 100.0)
                .sum::<f64>()
                / n as f64
        } else {
            f64::NAN
        };

        // Calculate SMAPE (Symmetric MAPE)
        let smape = predicted
            .iter()
            .zip(actual)
            .map(|(p, a)| {
                let numerator = (p - a).abs();
                let denominator = (p.abs() + a.abs()) / 2.0;
                if denominator > 0.0 {
                    (numerator / denominator) * 100.0
                } else {
                    0.0
                }
            })
            .sum::<f64>()
            / n as f64;

        // Calculate R-squared
        let mean_actual = actual.iter().sum::<f64>() / n as f64;
        let ss_tot: f64 = actual.iter().map(|a| (a - mean_actual).powi(2)).sum();
        let ss_res: f64 = predicted
            .iter()
            .zip(actual)
            .map(|(p, a)| (a - p).powi(2))
            .sum();
        let r_squared = if ss_tot > 0.0 {
            1.0 - (ss_res / ss_tot)
        } else {
            0.0
        };

        // Calculate directional accuracy (for time series with at least 2 points)
        let directional_accuracy = if n >= 2 {
            let correct_directions = (0..n - 1)
                .filter(|&i| {
                    let actual_direction = actual[i + 1] > actual[i];
                    let predicted_direction = predicted[i + 1] > predicted[i];
                    actual_direction == predicted_direction
                })
                .count();
            (correct_directions as f64 / (n - 1) as f64) * 100.0
        } else {
            0.0
        };

        Ok(Self {
            mae,
            mse,
            rmse,
            mape,
            smape,
            r_squared,
            directional_accuracy,
            n,
        })
    }

    /// Generate accuracy report
    pub fn report(&self) -> String {
        let mut report = String::from("Prediction Accuracy Metrics\n");
        report.push_str("===========================\n\n");
        report.push_str(&format!("Sample size: {}\n\n", self.n));
        report.push_str(&format!("MAE:  {:.4}\n", self.mae));
        report.push_str(&format!("MSE:  {:.4}\n", self.mse));
        report.push_str(&format!("RMSE: {:.4}\n", self.rmse));
        if !self.mape.is_nan() {
            report.push_str(&format!("MAPE: {:.2}%\n", self.mape));
        }
        report.push_str(&format!("SMAPE: {:.2}%\n", self.smape));
        report.push_str(&format!("R²:    {:.4}\n", self.r_squared));
        if self.n >= 2 {
            report.push_str(&format!(
                "Directional Accuracy: {:.2}%\n",
                self.directional_accuracy
            ));
        }
        report
    }

    /// Check if predictions are acceptable based on thresholds
    pub fn is_acceptable(&self, max_mape: f64, min_r_squared: f64) -> bool {
        (self.mape.is_nan() || self.mape <= max_mape) && self.r_squared >= min_r_squared
    }
}

// ============================================================================
// Ensemble Validation Methods (v0.2.9)
// ============================================================================

/// Bootstrap sample for ensemble validation
#[derive(Debug, Clone)]
pub struct BootstrapSample {
    /// Indices sampled with replacement
    pub indices: Vec<usize>,
    /// Out-of-bag indices (not sampled)
    pub oob_indices: Vec<usize>,
}

/// Ensemble validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsembleValidationResult {
    /// Individual model predictions
    pub model_predictions: Vec<Vec<f64>>,
    /// Ensemble (averaged) predictions
    pub ensemble_predictions: Vec<f64>,
    /// Actual values
    pub actual_values: Vec<f64>,
    /// Individual model accuracies
    pub model_accuracies: Vec<PredictionAccuracy>,
    /// Ensemble accuracy
    pub ensemble_accuracy: PredictionAccuracy,
    /// Number of models in ensemble
    pub num_models: usize,
}

impl EnsembleValidationResult {
    /// Check if ensemble outperforms individual models
    pub fn ensemble_improves(&self) -> bool {
        let avg_individual_rmse = self.model_accuracies.iter().map(|a| a.rmse).sum::<f64>()
            / self.model_accuracies.len() as f64;
        self.ensemble_accuracy.rmse < avg_individual_rmse
    }

    /// Generate ensemble validation report
    pub fn report(&self) -> String {
        let mut report = String::from("Ensemble Validation Report\n");
        report.push_str("==========================\n\n");
        report.push_str(&format!("Number of models: {}\n\n", self.num_models));

        report.push_str("Individual Model Performance:\n");
        for (i, acc) in self.model_accuracies.iter().enumerate() {
            report.push_str(&format!(
                "  Model {}: RMSE={:.4}, R²={:.4}\n",
                i + 1,
                acc.rmse,
                acc.r_squared
            ));
        }

        report.push_str("\nEnsemble Performance:\n");
        report.push_str(&format!("  RMSE: {:.4}\n", self.ensemble_accuracy.rmse));
        report.push_str(&format!(
            "  R²:   {:.4}\n",
            self.ensemble_accuracy.r_squared
        ));

        if self.ensemble_improves() {
            report.push_str("\n✓ Ensemble improves upon average individual model\n");
        } else {
            report.push_str("\n✗ Ensemble does not improve upon average individual model\n");
        }

        report
    }
}

/// Ensemble validator using bootstrap aggregating
#[derive(Debug, Clone)]
pub struct EnsembleValidator {
    /// Number of bootstrap samples
    pub num_samples: usize,
    /// Random seed for reproducibility
    pub seed: u64,
}

impl EnsembleValidator {
    /// Create a new ensemble validator
    pub fn new(num_samples: usize, seed: u64) -> Self {
        Self { num_samples, seed }
    }

    /// Generate bootstrap sample
    pub fn bootstrap_sample(&self, data_size: usize, sample_idx: usize) -> BootstrapSample {
        use std::collections::HashSet;

        // Simple deterministic sampling based on seed and sample index
        let mut indices = Vec::new();
        let mut sampled_set = HashSet::new();

        for i in 0..data_size {
            let hash_input = self
                .seed
                .wrapping_mul(sample_idx as u64)
                .wrapping_add(i as u64);
            let idx = (hash_input % data_size as u64) as usize;
            indices.push(idx);
            sampled_set.insert(idx);
        }

        let oob_indices: Vec<usize> = (0..data_size)
            .filter(|i| !sampled_set.contains(i))
            .collect();

        BootstrapSample {
            indices,
            oob_indices,
        }
    }

    /// Run ensemble validation with a model training function
    pub fn validate<F>(
        &self,
        features: &[Vec<f64>],
        targets: &[f64],
        mut train_predict_fn: F,
    ) -> SimResult<EnsembleValidationResult>
    where
        F: FnMut(&[Vec<f64>], &[f64]) -> Vec<f64>,
    {
        if features.len() != targets.len() {
            return Err(SimulationError::InvalidParameter(
                "Features and targets length mismatch".to_string(),
            ));
        }

        if features.is_empty() {
            return Err(SimulationError::InvalidParameter(
                "Empty dataset".to_string(),
            ));
        }

        let data_size = features.len();
        let mut model_predictions = Vec::new();
        let mut model_accuracies = Vec::new();

        for sample_idx in 0..self.num_samples {
            let sample = self.bootstrap_sample(data_size, sample_idx);

            // Train on bootstrap sample
            let train_features: Vec<Vec<f64>> = sample
                .indices
                .iter()
                .map(|&i| features[i].clone())
                .collect();
            let train_targets: Vec<f64> = sample.indices.iter().map(|&i| targets[i]).collect();

            // Predict on full dataset
            let predictions = train_predict_fn(&train_features, &train_targets);

            if predictions.len() != data_size {
                return Err(SimulationError::InvalidParameter(
                    "Prediction length must match dataset size".to_string(),
                ));
            }

            let accuracy = PredictionAccuracy::calculate(&predictions, targets)?;
            model_predictions.push(predictions);
            model_accuracies.push(accuracy);
        }

        // Calculate ensemble predictions (simple average)
        let mut ensemble_predictions = vec![0.0; data_size];
        for predictions in &model_predictions {
            for (i, &pred) in predictions.iter().enumerate() {
                ensemble_predictions[i] += pred;
            }
        }
        for pred in &mut ensemble_predictions {
            *pred /= self.num_samples as f64;
        }

        let ensemble_accuracy = PredictionAccuracy::calculate(&ensemble_predictions, targets)?;

        Ok(EnsembleValidationResult {
            model_predictions,
            ensemble_predictions,
            actual_values: targets.to_vec(),
            model_accuracies,
            ensemble_accuracy,
            num_models: self.num_samples,
        })
    }
}

// ============================================================================
// Sensitivity Analysis Automation (v0.2.9)
// ============================================================================

/// Parameter range for sensitivity analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterRange {
    /// Parameter name
    pub name: String,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// Number of steps
    pub steps: usize,
}

impl ParameterRange {
    /// Create a new parameter range
    pub fn new(name: impl Into<String>, min: f64, max: f64, steps: usize) -> Self {
        Self {
            name: name.into(),
            min,
            max,
            steps,
        }
    }

    /// Generate values for this parameter
    pub fn values(&self) -> Vec<f64> {
        if self.steps <= 1 {
            return vec![self.min];
        }

        let step_size = (self.max - self.min) / (self.steps - 1) as f64;
        (0..self.steps)
            .map(|i| self.min + step_size * i as f64)
            .collect()
    }
}

/// Sensitivity analysis result for a single parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterSensitivity {
    /// Parameter name
    pub parameter: String,
    /// Parameter values tested
    pub values: Vec<f64>,
    /// Corresponding output metric values
    pub outputs: Vec<f64>,
    /// Sensitivity coefficient (change in output / change in parameter)
    pub sensitivity: f64,
    /// Elasticity (% change in output / % change in parameter)
    pub elasticity: f64,
}

impl ParameterSensitivity {
    /// Calculate from values and outputs
    pub fn calculate(
        parameter: impl Into<String>,
        values: Vec<f64>,
        outputs: Vec<f64>,
    ) -> SimResult<Self> {
        if values.len() != outputs.len() || values.len() < 2 {
            return Err(SimulationError::InvalidParameter(
                "Need at least 2 value-output pairs".to_string(),
            ));
        }

        // Calculate average sensitivity (simple linear approximation)
        let mut sensitivities = Vec::new();
        for i in 1..values.len() {
            let delta_param = values[i] - values[i - 1];
            let delta_output = outputs[i] - outputs[i - 1];
            if delta_param.abs() > 1e-10 {
                sensitivities.push(delta_output / delta_param);
            }
        }

        let sensitivity = if sensitivities.is_empty() {
            0.0
        } else {
            sensitivities.iter().sum::<f64>() / sensitivities.len() as f64
        };

        // Calculate elasticity at midpoint
        let mid_idx = values.len() / 2;
        let elasticity = if mid_idx > 0
            && mid_idx < values.len()
            && values[mid_idx].abs() > 1e-10
            && outputs[mid_idx].abs() > 1e-10
        {
            let delta_param = values[mid_idx] - values[mid_idx - 1];
            let delta_output = outputs[mid_idx] - outputs[mid_idx - 1];
            let pct_change_param = delta_param / values[mid_idx];
            let pct_change_output = delta_output / outputs[mid_idx];
            if pct_change_param.abs() > 1e-10 {
                pct_change_output / pct_change_param
            } else {
                0.0
            }
        } else {
            0.0
        };

        Ok(Self {
            parameter: parameter.into(),
            values,
            outputs,
            sensitivity,
            elasticity,
        })
    }

    /// Check if parameter is highly sensitive (elasticity > 1.0)
    pub fn is_highly_sensitive(&self) -> bool {
        self.elasticity.abs() > 1.0
    }
}

/// Automated sensitivity analyzer
#[derive(Debug, Clone)]
pub struct SensitivityAnalyzer {
    /// Parameters to analyze
    pub parameters: Vec<ParameterRange>,
}

impl SensitivityAnalyzer {
    /// Create a new sensitivity analyzer
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
        }
    }

    /// Add a parameter to analyze
    pub fn add_parameter(&mut self, parameter: ParameterRange) {
        self.parameters.push(parameter);
    }

    /// Run sensitivity analysis with a simulation function
    pub fn analyze<F>(&self, mut simulate_fn: F) -> SimResult<Vec<ParameterSensitivity>>
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        let mut results = Vec::new();

        for param_range in &self.parameters {
            let values = param_range.values();
            let mut outputs = Vec::new();

            for &value in &values {
                let mut params = HashMap::new();
                params.insert(param_range.name.clone(), value);
                let output = simulate_fn(&params);
                outputs.push(output);
            }

            let sensitivity = ParameterSensitivity::calculate(&param_range.name, values, outputs)?;
            results.push(sensitivity);
        }

        Ok(results)
    }

    /// Get highly sensitive parameters
    pub fn find_sensitive_parameters<F>(
        &self,
        simulate_fn: F,
    ) -> SimResult<Vec<ParameterSensitivity>>
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        let all_results = self.analyze(simulate_fn)?;
        Ok(all_results
            .into_iter()
            .filter(|s| s.is_highly_sensitive())
            .collect())
    }
}

impl Default for SensitivityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests_v2_9 {
    use super::*;

    // Historical Backtesting Tests
    #[test]
    fn test_backtest_config_default() {
        let config = BacktestConfig::default();
        assert_eq!(config.min_train_size, 10);
        assert_eq!(config.test_size, 1);
        assert_eq!(config.step_size, 1);
        assert!(!config.expanding_window);
    }

    #[test]
    fn test_historical_backtester() {
        let mut data = Vec::new();
        for i in 0..20 {
            let mut features = HashMap::new();
            features.insert("x".to_string(), i as f64);
            data.push(BacktestTimeSeriesPoint {
                period: format!("T{}", i),
                actual: i as f64 * 2.0,
                features,
            });
        }

        let config = BacktestConfig {
            min_train_size: 5,
            test_size: 2,
            step_size: 1,
            expanding_window: false,
        };

        let backtester = HistoricalBacktester::new(config);

        let result = backtester.run(&data, |train_data| {
            // Simple prediction: return average of training data
            let avg = train_data.iter().map(|p| p.actual).sum::<f64>() / train_data.len() as f64;
            vec![avg, avg]
        });

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.num_folds > 0);
        assert!(result.average_error() >= 0.0);
    }

    #[test]
    fn test_backtest_expanding_window() {
        let mut data = Vec::new();
        for i in 0..15 {
            data.push(BacktestTimeSeriesPoint {
                period: format!("T{}", i),
                actual: i as f64,
                features: HashMap::new(),
            });
        }

        let config = BacktestConfig {
            min_train_size: 5,
            test_size: 1,
            step_size: 2,
            expanding_window: true,
        };

        let backtester = HistoricalBacktester::new(config);
        let result = backtester
            .run(&data, |train_data| vec![train_data.len() as f64])
            .unwrap();

        // Check that training window expands
        assert!(result.folds[0].train_periods.len() == 5);
        if result.folds.len() > 1 {
            assert!(result.folds[1].train_periods.len() > result.folds[0].train_periods.len());
        }
    }

    // Prediction Accuracy Tests
    #[test]
    fn test_prediction_accuracy_perfect() {
        let predicted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let actual = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let accuracy = PredictionAccuracy::calculate(&predicted, &actual).unwrap();

        assert_eq!(accuracy.mae, 0.0);
        assert_eq!(accuracy.mse, 0.0);
        assert_eq!(accuracy.rmse, 0.0);
        assert_eq!(accuracy.r_squared, 1.0);
    }

    #[test]
    fn test_prediction_accuracy_metrics() {
        let predicted = vec![10.0, 20.0, 30.0, 40.0];
        let actual = vec![12.0, 18.0, 32.0, 38.0];

        let accuracy = PredictionAccuracy::calculate(&predicted, &actual).unwrap();

        assert!(accuracy.mae > 0.0);
        assert!(accuracy.rmse > 0.0);
        assert!(accuracy.r_squared >= 0.0 && accuracy.r_squared <= 1.0);
        assert!(accuracy.directional_accuracy >= 0.0);
    }

    #[test]
    fn test_prediction_accuracy_report() {
        let predicted = vec![1.0, 2.0, 3.0];
        let actual = vec![1.1, 1.9, 3.2];

        let accuracy = PredictionAccuracy::calculate(&predicted, &actual).unwrap();
        let report = accuracy.report();

        assert!(report.contains("Prediction Accuracy"));
        assert!(report.contains("MAE"));
        assert!(report.contains("RMSE"));
    }

    #[test]
    fn test_prediction_accuracy_directional() {
        // All directions correct
        let predicted = vec![1.0, 2.0, 3.0, 4.0];
        let actual = vec![1.0, 2.0, 3.0, 4.0];

        let accuracy = PredictionAccuracy::calculate(&predicted, &actual).unwrap();
        assert_eq!(accuracy.directional_accuracy, 100.0);
    }

    // Ensemble Validation Tests
    #[test]
    fn test_bootstrap_sample() {
        let validator = EnsembleValidator::new(5, 42);
        let sample = validator.bootstrap_sample(10, 0);

        assert_eq!(sample.indices.len(), 10);
        assert!(sample.oob_indices.len() <= 10);
    }

    #[test]
    fn test_ensemble_validator() {
        let features = vec![
            vec![1.0],
            vec![2.0],
            vec![3.0],
            vec![4.0],
            vec![5.0],
            vec![6.0],
            vec![7.0],
            vec![8.0],
            vec![9.0],
            vec![10.0],
        ];
        let targets = vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0, 20.0];

        let validator = EnsembleValidator::new(3, 123);

        let result = validator.validate(&features, &targets, |train_features, train_targets| {
            // Simple mean prediction
            let mean = train_targets.iter().sum::<f64>() / train_targets.len() as f64;
            vec![mean; train_features.len()]
        });

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.num_models, 3);
        assert_eq!(result.ensemble_predictions.len(), targets.len());
    }

    #[test]
    fn test_ensemble_improvement() {
        let features = vec![vec![1.0], vec![2.0], vec![3.0], vec![4.0], vec![5.0]];
        let targets = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let validator = EnsembleValidator::new(2, 456);

        let result = validator
            .validate(&features, &targets, |_, train_targets| {
                let mean = train_targets.iter().sum::<f64>() / train_targets.len() as f64;
                vec![mean; features.len()]
            })
            .unwrap();

        // Check that ensemble_improves doesn't crash
        let _improves = result.ensemble_improves();
    }

    // Sensitivity Analysis Tests
    #[test]
    fn test_parameter_range() {
        let range = ParameterRange::new("test", 0.0, 10.0, 5);
        let values = range.values();

        assert_eq!(values.len(), 5);
        assert_eq!(values[0], 0.0);
        assert_eq!(values[4], 10.0);
    }

    #[test]
    fn test_parameter_sensitivity() {
        let values = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let outputs = vec![0.0, 2.0, 4.0, 6.0, 8.0];

        let sensitivity = ParameterSensitivity::calculate("param", values, outputs).unwrap();

        assert_eq!(sensitivity.sensitivity, 2.0); // output changes by 2 for every 1 change in param
        assert!(sensitivity.elasticity != 0.0);
    }

    #[test]
    fn test_sensitivity_analyzer() {
        let mut analyzer = SensitivityAnalyzer::new();
        analyzer.add_parameter(ParameterRange::new("x", 0.0, 10.0, 5));
        analyzer.add_parameter(ParameterRange::new("y", 0.0, 5.0, 3));

        let results = analyzer.analyze(|params| {
            let x = params.get("x").unwrap_or(&0.0);
            let y = params.get("y").unwrap_or(&0.0);
            x * 2.0 + y * 3.0
        });

        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_find_sensitive_parameters() {
        let mut analyzer = SensitivityAnalyzer::new();
        analyzer.add_parameter(ParameterRange::new("high_impact", 1.0, 10.0, 5));
        analyzer.add_parameter(ParameterRange::new("low_impact", 0.0, 1.0, 3));

        let sensitive = analyzer.find_sensitive_parameters(|params| {
            let high = params.get("high_impact").unwrap_or(&1.0);
            let low = params.get("low_impact").unwrap_or(&0.0);
            high * 100.0 + low * 0.1
        });

        assert!(sensitive.is_ok());
    }

    #[test]
    fn test_backtest_result_report() {
        let fold = BacktestFold {
            fold: 0,
            train_periods: vec!["T1".to_string(), "T2".to_string()],
            test_periods: vec!["T3".to_string()],
            predicted: vec![3.0],
            actual: vec![3.1],
            errors: vec![-0.1],
        };

        let result = BacktestResult::new(vec![fold]).unwrap();
        let report = result.report();

        assert!(report.contains("Backtest Report"));
        assert!(report.contains("Number of folds"));
    }

    #[test]
    fn test_ensemble_validation_report() {
        let features = vec![vec![1.0], vec![2.0], vec![3.0]];
        let targets = vec![2.0, 4.0, 6.0];

        let validator = EnsembleValidator::new(2, 789);
        let result = validator
            .validate(&features, &targets, |_, train_targets| {
                let mean = train_targets.iter().sum::<f64>() / train_targets.len() as f64;
                vec![mean; features.len()]
            })
            .unwrap();

        let report = result.report();
        assert!(report.contains("Ensemble Validation"));
        assert!(report.contains("Number of models"));
    }
}
