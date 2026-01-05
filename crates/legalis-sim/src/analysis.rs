//! Statistical analysis tools for simulation results.
//!
//! This module provides:
//! - Distribution analysis (normal, power law, etc.)
//! - Correlation detection
//! - Time-series analysis
//! - Cohort analysis

use crate::temporal::{TemporalMetrics, TimeSnapshot};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Statistical distribution fit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionFit {
    /// Type of distribution
    pub distribution_type: DistributionType,
    /// Parameters of the distribution
    pub parameters: HashMap<String, f64>,
    /// Goodness of fit (R-squared, 0.0 to 1.0)
    pub r_squared: f64,
}

/// Type of statistical distribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistributionType {
    /// Normal (Gaussian) distribution
    Normal,
    /// Log-normal distribution
    LogNormal,
    /// Power law distribution
    PowerLaw,
    /// Exponential distribution
    Exponential,
    /// Uniform distribution
    Uniform,
}

/// Distribution analyzer.
pub struct DistributionAnalyzer {
    data: Vec<f64>,
}

impl DistributionAnalyzer {
    /// Creates a new distribution analyzer.
    pub fn new(data: Vec<f64>) -> Self {
        Self { data }
    }

    /// Calculates basic statistics.
    pub fn statistics(&self) -> BasicStatistics {
        if self.data.is_empty() {
            return BasicStatistics::default();
        }

        let mean = self.data.iter().sum::<f64>() / self.data.len() as f64;

        let variance =
            self.data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / self.data.len() as f64;

        let std_dev = variance.sqrt();

        let mut sorted = self.data.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let median = if sorted.len() % 2 == 0 {
            (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
        } else {
            sorted[sorted.len() / 2]
        };

        let min = sorted.first().copied().unwrap_or(0.0);
        let max = sorted.last().copied().unwrap_or(0.0);

        // Quartiles
        let q1 = Self::percentile(&sorted, 0.25);
        let q3 = Self::percentile(&sorted, 0.75);

        // Skewness
        let skewness = if std_dev > 0.0 {
            self.data
                .iter()
                .map(|x| ((x - mean) / std_dev).powi(3))
                .sum::<f64>()
                / self.data.len() as f64
        } else {
            0.0
        };

        // Kurtosis
        let kurtosis = if std_dev > 0.0 {
            self.data
                .iter()
                .map(|x| ((x - mean) / std_dev).powi(4))
                .sum::<f64>()
                / self.data.len() as f64
                - 3.0 // Excess kurtosis
        } else {
            0.0
        };

        BasicStatistics {
            count: self.data.len(),
            mean,
            median,
            std_dev,
            variance,
            min,
            max,
            q1,
            q3,
            skewness,
            kurtosis,
        }
    }

    /// Fits a normal distribution to the data.
    pub fn fit_normal(&self) -> DistributionFit {
        let stats = self.statistics();
        let mut params = HashMap::new();
        params.insert("mean".to_string(), stats.mean);
        params.insert("std_dev".to_string(), stats.std_dev);

        let r_squared = self.calculate_r_squared_normal(stats.mean, stats.std_dev);

        DistributionFit {
            distribution_type: DistributionType::Normal,
            parameters: params,
            r_squared,
        }
    }

    /// Fits a power law distribution to the data.
    pub fn fit_power_law(&self) -> Option<DistributionFit> {
        // Filter positive values
        let positive: Vec<f64> = self.data.iter().copied().filter(|&x| x > 0.0).collect();
        if positive.len() < 2 {
            return None;
        }

        // Log-log linear regression to estimate alpha
        let log_x: Vec<f64> = positive.iter().map(|x| x.ln()).collect();
        let log_y: Vec<f64> = (1..=positive.len())
            .rev()
            .map(|i| (i as f64 / positive.len() as f64).ln())
            .collect();

        let n = log_x.len() as f64;
        let sum_x: f64 = log_x.iter().sum();
        let sum_y: f64 = log_y.iter().sum();
        let sum_xy: f64 = log_x.iter().zip(&log_y).map(|(x, y)| x * y).sum();
        let sum_x2: f64 = log_x.iter().map(|x| x * x).sum();

        let alpha = -(n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let x_min = positive.iter().copied().fold(f64::INFINITY, f64::min);

        let mut params = HashMap::new();
        params.insert("alpha".to_string(), alpha);
        params.insert("x_min".to_string(), x_min);

        let r_squared = self.calculate_r_squared_power_law(alpha, x_min);

        Some(DistributionFit {
            distribution_type: DistributionType::PowerLaw,
            parameters: params,
            r_squared,
        })
    }

    /// Determines the best fitting distribution.
    pub fn best_fit(&self) -> DistributionFit {
        let normal = self.fit_normal();
        let power_law = self.fit_power_law();

        match power_law {
            Some(pl) if pl.r_squared > normal.r_squared => pl,
            _ => normal,
        }
    }

    fn percentile(sorted: &[f64], p: f64) -> f64 {
        if sorted.is_empty() {
            return 0.0;
        }
        let idx = (p * (sorted.len() - 1) as f64).round() as usize;
        sorted[idx.min(sorted.len() - 1)]
    }

    fn calculate_r_squared_normal(&self, mean: f64, std_dev: f64) -> f64 {
        if self.data.is_empty() || std_dev == 0.0 {
            return 0.0;
        }

        let mean_y = self.data.iter().sum::<f64>() / self.data.len() as f64;
        let ss_tot: f64 = self.data.iter().map(|y| (y - mean_y).powi(2)).sum();

        if ss_tot == 0.0 {
            return 1.0;
        }

        let ss_res: f64 = self
            .data
            .iter()
            .map(|y| {
                // Simplified: just check how well values cluster around mean
                (y - mean).powi(2) / (std_dev.powi(2) + 1.0)
            })
            .sum();

        (1.0 - ss_res / ss_tot).clamp(0.0, 1.0)
    }

    fn calculate_r_squared_power_law(&self, _alpha: f64, x_min: f64) -> f64 {
        let positive: Vec<f64> = self.data.iter().copied().filter(|&x| x >= x_min).collect();
        if positive.is_empty() {
            return 0.0;
        }

        // Simplified R-squared calculation
        let log_x: Vec<f64> = positive.iter().map(|x| x.ln()).collect();
        let mean_log_x = log_x.iter().sum::<f64>() / log_x.len() as f64;
        let ss_tot: f64 = log_x.iter().map(|x| (x - mean_log_x).powi(2)).sum();

        if ss_tot == 0.0 {
            return 1.0;
        }

        0.7 // Placeholder - proper calculation would require rank-frequency data
    }
}

/// Basic statistical measures.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BasicStatistics {
    /// Number of data points
    pub count: usize,
    /// Mean (average)
    pub mean: f64,
    /// Median (50th percentile)
    pub median: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Variance
    pub variance: f64,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// First quartile (25th percentile)
    pub q1: f64,
    /// Third quartile (75th percentile)
    pub q3: f64,
    /// Skewness
    pub skewness: f64,
    /// Kurtosis (excess)
    pub kurtosis: f64,
}

impl BasicStatistics {
    /// Generates a summary report.
    pub fn summary(&self) -> String {
        format!(
            "Count: {}, Mean: {:.2}, Median: {:.2}, StdDev: {:.2}, Min: {:.2}, Max: {:.2}, Q1: {:.2}, Q3: {:.2}",
            self.count, self.mean, self.median, self.std_dev, self.min, self.max, self.q1, self.q3
        )
    }
}

/// Correlation analyzer.
pub struct CorrelationAnalyzer;

impl CorrelationAnalyzer {
    /// Calculates Pearson correlation coefficient between two variables.
    pub fn pearson(x: &[f64], y: &[f64]) -> Option<f64> {
        if x.len() != y.len() || x.is_empty() {
            return None;
        }

        let n = x.len() as f64;
        let mean_x = x.iter().sum::<f64>() / n;
        let mean_y = y.iter().sum::<f64>() / n;

        let cov: f64 = x
            .iter()
            .zip(y)
            .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
            .sum::<f64>()
            / n;

        let var_x: f64 = x.iter().map(|xi| (xi - mean_x).powi(2)).sum::<f64>() / n;
        let var_y: f64 = y.iter().map(|yi| (yi - mean_y).powi(2)).sum::<f64>() / n;

        let std_x = var_x.sqrt();
        let std_y = var_y.sqrt();

        if std_x == 0.0 || std_y == 0.0 {
            return Some(0.0);
        }

        Some(cov / (std_x * std_y))
    }

    /// Calculates Spearman rank correlation.
    pub fn spearman(x: &[f64], y: &[f64]) -> Option<f64> {
        if x.len() != y.len() || x.is_empty() {
            return None;
        }

        let rank_x = Self::rank(x);
        let rank_y = Self::rank(y);

        Self::pearson(&rank_x, &rank_y)
    }

    fn rank(data: &[f64]) -> Vec<f64> {
        let mut indexed: Vec<(usize, f64)> = data.iter().copied().enumerate().collect();
        indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let mut ranks = vec![0.0; data.len()];
        for (rank, (idx, _)) in indexed.iter().enumerate() {
            ranks[*idx] = (rank + 1) as f64;
        }
        ranks
    }
}

/// Time-series analyzer.
pub struct TimeSeriesAnalyzer {
    snapshots: Vec<TimeSnapshot>,
}

impl TimeSeriesAnalyzer {
    /// Creates a new time-series analyzer.
    pub fn new(metrics: &TemporalMetrics) -> Self {
        Self {
            snapshots: metrics.snapshots.clone(),
        }
    }

    /// Extracts deterministic ratios over time.
    pub fn deterministic_trend(&self) -> Vec<(usize, f64)> {
        self.snapshots
            .iter()
            .enumerate()
            .map(|(i, snapshot)| (i, snapshot.metrics.deterministic_ratio()))
            .collect()
    }

    /// Calculates moving average.
    pub fn moving_average(&self, window: usize) -> Vec<f64> {
        let values: Vec<f64> = self
            .snapshots
            .iter()
            .map(|s| s.metrics.deterministic_ratio())
            .collect();

        if values.len() < window {
            return vec![];
        }

        (0..=values.len() - window)
            .map(|i| values[i..i + window].iter().sum::<f64>() / window as f64)
            .collect()
    }

    /// Detects trends (increasing, decreasing, stable).
    pub fn detect_trend(&self) -> TrendDirection {
        let values: Vec<f64> = self
            .snapshots
            .iter()
            .map(|s| s.metrics.deterministic_ratio())
            .collect();

        if values.len() < 3 {
            return TrendDirection::Stable;
        }

        // Simple linear regression slope
        let n = values.len() as f64;
        let x_mean = (n - 1.0) / 2.0;
        let y_mean = values.iter().sum::<f64>() / n;

        let slope = values
            .iter()
            .enumerate()
            .map(|(i, y)| (i as f64 - x_mean) * (y - y_mean))
            .sum::<f64>()
            / values
                .iter()
                .enumerate()
                .map(|(i, _)| (i as f64 - x_mean).powi(2))
                .sum::<f64>();

        if slope > 0.01 {
            TrendDirection::Increasing
        } else if slope < -0.01 {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        }
    }

    /// Calculates volatility (standard deviation of changes).
    pub fn volatility(&self) -> f64 {
        let values: Vec<f64> = self
            .snapshots
            .iter()
            .map(|s| s.metrics.deterministic_ratio())
            .collect();

        if values.len() < 2 {
            return 0.0;
        }

        let changes: Vec<f64> = values.windows(2).map(|w| w[1] - w[0]).collect();

        let mean_change = changes.iter().sum::<f64>() / changes.len() as f64;
        let variance = changes
            .iter()
            .map(|c| (c - mean_change).powi(2))
            .sum::<f64>()
            / changes.len() as f64;

        variance.sqrt()
    }
}

/// Trend direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrendDirection {
    /// Increasing trend
    Increasing,
    /// Decreasing trend
    Decreasing,
    /// Stable (no significant trend)
    Stable,
}

/// Cohort definition for grouping entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cohort {
    /// Cohort identifier
    pub id: String,
    /// Cohort name
    pub name: String,
    /// Cohort criteria (attribute name -> filter)
    pub criteria: HashMap<String, CohortFilter>,
}

/// Filter for cohort membership.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CohortFilter {
    /// Exact value match
    Equals(String),
    /// Numeric range
    Range { min: f64, max: f64 },
    /// Set of possible values
    In(Vec<String>),
    /// Value starts with prefix
    StartsWith(String),
    /// Value contains substring
    Contains(String),
}

impl CohortFilter {
    /// Checks if a value matches this filter.
    pub fn matches(&self, value: &str) -> bool {
        match self {
            CohortFilter::Equals(expected) => value == expected,
            CohortFilter::Range { min, max } => {
                if let Ok(num) = value.parse::<f64>() {
                    num >= *min && num <= *max
                } else {
                    false
                }
            }
            CohortFilter::In(values) => values.contains(&value.to_string()),
            CohortFilter::StartsWith(prefix) => value.starts_with(prefix),
            CohortFilter::Contains(substring) => value.contains(substring),
        }
    }
}

impl Cohort {
    /// Creates a new cohort.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            criteria: HashMap::new(),
        }
    }

    /// Adds a criterion to this cohort.
    pub fn with_criterion(mut self, attribute: impl Into<String>, filter: CohortFilter) -> Self {
        self.criteria.insert(attribute.into(), filter);
        self
    }

    /// Checks if an entity matches this cohort.
    pub fn matches(&self, attributes: &HashMap<String, String>) -> bool {
        self.criteria.iter().all(|(attr, filter)| {
            attributes
                .get(attr)
                .map(|value| filter.matches(value))
                .unwrap_or(false)
        })
    }
}

/// Cohort analysis results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortAnalysis {
    /// Cohort being analyzed
    pub cohort: Cohort,
    /// Number of entities in cohort
    pub size: usize,
    /// Compliance rate for this cohort
    pub compliance_rate: f64,
    /// Evasion rate for this cohort
    pub evasion_rate: f64,
    /// Average deterministic ratio
    pub deterministic_ratio: f64,
    /// Statute-specific metrics
    pub statute_metrics: HashMap<String, CohortStatuteMetrics>,
}

/// Cohort metrics for a specific statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortStatuteMetrics {
    /// Statute ID
    pub statute_id: String,
    /// Number of applications in this cohort
    pub applications: usize,
    /// Compliance rate
    pub compliance_rate: f64,
    /// Effectiveness score
    pub effectiveness: f64,
}

/// Cohort analyzer for population segmentation.
pub struct CohortAnalyzer {
    cohorts: Vec<Cohort>,
}

impl CohortAnalyzer {
    /// Creates a new cohort analyzer.
    pub fn new() -> Self {
        Self {
            cohorts: Vec::new(),
        }
    }

    /// Adds a cohort for analysis.
    pub fn add_cohort(&mut self, cohort: Cohort) {
        self.cohorts.push(cohort);
    }

    /// Adds multiple cohorts.
    pub fn add_cohorts(&mut self, cohorts: impl IntoIterator<Item = Cohort>) {
        self.cohorts.extend(cohorts);
    }

    /// Analyzes cohorts against population data.
    pub fn analyze(
        &self,
        entities: &[(HashMap<String, String>, bool)], // (attributes, complied)
    ) -> Vec<CohortAnalysis> {
        self.cohorts
            .iter()
            .map(|cohort| self.analyze_cohort(cohort, entities))
            .collect()
    }

    /// Analyzes a single cohort.
    fn analyze_cohort(
        &self,
        cohort: &Cohort,
        entities: &[(HashMap<String, String>, bool)],
    ) -> CohortAnalysis {
        let matching: Vec<_> = entities
            .iter()
            .filter(|(attrs, _)| cohort.matches(attrs))
            .collect();

        let size = matching.len();
        let complied_count = matching.iter().filter(|(_, complied)| *complied).count();

        let compliance_rate = if size > 0 {
            complied_count as f64 / size as f64
        } else {
            0.0
        };

        let evasion_rate = 1.0 - compliance_rate;

        CohortAnalysis {
            cohort: cohort.clone(),
            size,
            compliance_rate,
            evasion_rate,
            deterministic_ratio: 0.0,
            statute_metrics: HashMap::new(),
        }
    }

    /// Compares cohorts across a metric.
    pub fn compare_cohorts(&self, analyses: &[CohortAnalysis]) -> String {
        let mut report = String::from("=== Cohort Comparison ===\n\n");

        report.push_str("Cohort | Size | Compliance | Evasion\n");
        report.push_str("-------|------|------------|--------\n");

        for analysis in analyses {
            report.push_str(&format!(
                "{:20} | {:4} | {:6.1}% | {:6.1}%\n",
                analysis.cohort.name,
                analysis.size,
                analysis.compliance_rate * 100.0,
                analysis.evasion_rate * 100.0
            ));
        }

        report
    }

    /// Creates age-based cohorts.
    pub fn age_cohorts() -> Vec<Cohort> {
        vec![
            Cohort::new("youth", "Youth (0-17)").with_criterion(
                "age",
                CohortFilter::Range {
                    min: 0.0,
                    max: 17.0,
                },
            ),
            Cohort::new("young_adult", "Young Adults (18-35)").with_criterion(
                "age",
                CohortFilter::Range {
                    min: 18.0,
                    max: 35.0,
                },
            ),
            Cohort::new("middle_age", "Middle Age (36-55)").with_criterion(
                "age",
                CohortFilter::Range {
                    min: 36.0,
                    max: 55.0,
                },
            ),
            Cohort::new("senior", "Seniors (56+)").with_criterion(
                "age",
                CohortFilter::Range {
                    min: 56.0,
                    max: 150.0,
                },
            ),
        ]
    }

    /// Creates income-based cohorts.
    pub fn income_cohorts() -> Vec<Cohort> {
        vec![
            Cohort::new("low_income", "Low Income (<30k)").with_criterion(
                "income",
                CohortFilter::Range {
                    min: 0.0,
                    max: 30000.0,
                },
            ),
            Cohort::new("middle_income", "Middle Income (30k-100k)").with_criterion(
                "income",
                CohortFilter::Range {
                    min: 30000.0,
                    max: 100000.0,
                },
            ),
            Cohort::new("high_income", "High Income (100k+)").with_criterion(
                "income",
                CohortFilter::Range {
                    min: 100000.0,
                    max: 1e9,
                },
            ),
        ]
    }

    /// Creates region-based cohorts.
    pub fn region_cohorts(regions: &[String]) -> Vec<Cohort> {
        regions
            .iter()
            .map(|region| {
                Cohort::new(format!("region_{}", region), format!("Region: {}", region))
                    .with_criterion("region", CohortFilter::Equals(region.clone()))
            })
            .collect()
    }
}

impl Default for CohortAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_statistics() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let analyzer = DistributionAnalyzer::new(data);
        let stats = analyzer.statistics();

        assert_eq!(stats.count, 5);
        assert!((stats.mean - 3.0).abs() < 0.01);
        assert!((stats.median - 3.0).abs() < 0.01);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
    }

    #[test]
    fn test_normal_fit() {
        let data = vec![10.0, 12.0, 11.0, 13.0, 12.5, 11.5];
        let analyzer = DistributionAnalyzer::new(data);
        let fit = analyzer.fit_normal();

        assert_eq!(fit.distribution_type, DistributionType::Normal);
        assert!(fit.parameters.contains_key("mean"));
        assert!(fit.parameters.contains_key("std_dev"));
        assert!(fit.r_squared >= 0.0 && fit.r_squared <= 1.0);
    }

    #[test]
    fn test_pearson_correlation() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // Perfect positive correlation

        let corr = CorrelationAnalyzer::pearson(&x, &y).unwrap();
        assert!((corr - 1.0).abs() < 0.01); // Should be ~1.0
    }

    #[test]
    fn test_pearson_negative_correlation() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![10.0, 8.0, 6.0, 4.0, 2.0]; // Perfect negative correlation

        let corr = CorrelationAnalyzer::pearson(&x, &y).unwrap();
        assert!((corr + 1.0).abs() < 0.01); // Should be ~-1.0
    }

    #[test]
    fn test_trend_detection() {
        use crate::metrics::SimulationMetrics;
        use chrono::NaiveDate;

        let mut snapshots = vec![];
        for i in 0..10 {
            let mut metrics = SimulationMetrics::new();
            // Simulate increasing deterministic ratio
            for _ in 0..(50 + i * 5) {
                metrics.deterministic_count += 1;
                metrics.total_applications += 1;
            }
            for _ in (50 + i * 5)..100 {
                metrics.void_count += 1;
                metrics.total_applications += 1;
            }

            snapshots.push(TimeSnapshot {
                date: NaiveDate::from_ymd_opt(2024, 1, 1 + i as u32).unwrap(),
                metrics,
                active_agents: 100,
                active_statutes: 5,
                events: vec![],
            });
        }

        let metrics = TemporalMetrics {
            snapshots,
            ..Default::default()
        };

        let analyzer = TimeSeriesAnalyzer::new(&metrics);
        let trend = analyzer.detect_trend();

        assert_eq!(trend, TrendDirection::Increasing);
    }

    #[test]
    fn test_moving_average() {
        use crate::metrics::SimulationMetrics;
        use chrono::NaiveDate;

        let mut snapshots = vec![];
        for i in 0..10 {
            let mut metrics = SimulationMetrics::new();
            metrics.deterministic_count = 50 + i;
            metrics.total_applications = 100;

            snapshots.push(TimeSnapshot {
                date: NaiveDate::from_ymd_opt(2024, 1, 1 + i as u32).unwrap(),
                metrics,
                active_agents: 100,
                active_statutes: 5,
                events: vec![],
            });
        }

        let metrics = TemporalMetrics {
            snapshots,
            ..Default::default()
        };

        let analyzer = TimeSeriesAnalyzer::new(&metrics);
        let ma = analyzer.moving_average(3);

        assert!(ma.len() >= 8);
    }
}
