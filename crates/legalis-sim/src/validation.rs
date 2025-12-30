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
