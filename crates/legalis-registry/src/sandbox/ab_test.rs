//! A/B testing for statute variants.
//!
//! An [`AbTest`] deterministically partitions an entity sample into a control
//! cohort and a treatment cohort, applies a different statute variant to each,
//! and compares the resulting [`ImpactReport`]s. The comparison reports the
//! absolute and relative lift in coverage, a two-proportion z-test for
//! statistical significance, and Cohen's d as an effect-size measure on the
//! monetary deltas.

use chrono::{DateTime, Utc};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::impact::{ImpactModel, ImpactPredictionSandbox, ImpactReport, SyntheticEntity};

/// Default significance level for the two-proportion test.
pub const DEFAULT_ALPHA: f64 = 0.05;

/// Which arm of an A/B test an entity is assigned to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CohortArm {
    /// The baseline cohort.
    Control,
    /// The variant-under-test cohort.
    Treatment,
}

/// A named statute variant participating in an A/B test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteVariant {
    /// Human-readable label for the variant (e.g. `"baseline"`, `"v2"`).
    pub label: String,
    /// The statute applied to the variant's cohort.
    pub statute: Statute,
}

impl StatuteVariant {
    /// Creates a new statute variant.
    #[must_use]
    pub fn new(label: impl Into<String>, statute: Statute) -> Self {
        Self {
            label: label.into(),
            statute,
        }
    }
}

/// An A/B test comparing a control statute against a treatment statute.
#[derive(Debug, Clone)]
pub struct AbTest {
    /// Human-readable test name.
    pub name: String,
    /// The control (baseline) variant.
    pub control: StatuteVariant,
    /// The treatment (variant-under-test).
    pub treatment: StatuteVariant,
    /// Fraction of entities assigned to the treatment arm (0.0 - 1.0).
    pub split_ratio: f64,
    /// Salt mixed into the cohort hash for reproducible, independent splits.
    pub salt: String,
    /// Optional entity attribute used to produce per-cohort breakdowns.
    pub cohort_attribute: Option<String>,
    /// Significance level for the two-proportion test.
    pub alpha: f64,
    /// Impact model used for both arms.
    model: ImpactModel,
}

impl AbTest {
    /// Creates a balanced (50/50) A/B test between two variants.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        control: StatuteVariant,
        treatment: StatuteVariant,
    ) -> Self {
        Self {
            name: name.into(),
            control,
            treatment,
            split_ratio: 0.5,
            salt: "legalis-ab".to_string(),
            cohort_attribute: None,
            alpha: DEFAULT_ALPHA,
            model: ImpactModel::new(),
        }
    }

    /// Sets the fraction of entities assigned to the treatment arm.
    #[must_use]
    pub fn with_split_ratio(mut self, ratio: f64) -> Self {
        self.split_ratio = ratio.clamp(0.0, 1.0);
        self
    }

    /// Sets the salt used for cohort assignment.
    #[must_use]
    pub fn with_salt(mut self, salt: impl Into<String>) -> Self {
        self.salt = salt.into();
        self
    }

    /// Sets the cohort breakdown attribute applied to both arms.
    #[must_use]
    pub fn with_cohort_attribute(mut self, attribute: impl Into<String>) -> Self {
        self.cohort_attribute = Some(attribute.into());
        self
    }

    /// Sets the significance level for the two-proportion test.
    #[must_use]
    pub fn with_alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }

    /// Sets the impact model used to score both arms.
    #[must_use]
    pub fn with_model(mut self, model: ImpactModel) -> Self {
        self.model = model;
        self
    }

    /// Deterministically assigns an entity to a cohort arm.
    #[must_use]
    pub fn assign(&self, entity_id: &str) -> CohortArm {
        if hash_fraction(&self.salt, entity_id) < self.split_ratio {
            CohortArm::Treatment
        } else {
            CohortArm::Control
        }
    }

    /// Runs the A/B test over an entity sample.
    #[must_use]
    pub fn run(&self, entities: &[SyntheticEntity]) -> AbTestResult {
        let mut control_entities = Vec::new();
        let mut treatment_entities = Vec::new();
        for entity in entities {
            match self.assign(&entity.id) {
                CohortArm::Control => control_entities.push(entity.clone()),
                CohortArm::Treatment => treatment_entities.push(entity.clone()),
            }
        }

        let predictor = ImpactPredictionSandbox::new(self.model.clone());
        let cohort_attribute = self.cohort_attribute.as_deref();
        let control_report =
            predictor.predict_statute(&self.control.statute, &control_entities, cohort_attribute);
        let treatment_report = predictor.predict_statute(
            &self.treatment.statute,
            &treatment_entities,
            cohort_attribute,
        );

        let control_monetary: Vec<f64> = control_report
            .per_entity
            .iter()
            .map(|impact| impact.monetary_delta)
            .collect();
        let treatment_monetary: Vec<f64> = treatment_report
            .per_entity
            .iter()
            .map(|impact| impact.monetary_delta)
            .collect();

        let z = two_proportion_z(
            treatment_report.affected_count,
            treatment_report.sample_size,
            control_report.affected_count,
            control_report.sample_size,
        );
        let p_value = two_sided_p_value(z);
        let effect_size = cohens_d(&treatment_monetary, &control_monetary);

        let absolute_effect = treatment_report.coverage - control_report.coverage;
        let relative_effect = if control_report.coverage > 0.0 {
            absolute_effect / control_report.coverage
        } else {
            0.0
        };
        let significant = p_value < self.alpha;

        AbTestResult {
            test_name: self.name.clone(),
            control_label: self.control.label.clone(),
            treatment_label: self.treatment.label.clone(),
            control_coverage: control_report.coverage,
            treatment_coverage: treatment_report.coverage,
            absolute_effect,
            relative_effect,
            z_statistic: z,
            p_value,
            cohens_d: effect_size,
            alpha: self.alpha,
            significant,
            control_report,
            treatment_report,
            generated_at: Utc::now(),
        }
    }
}

/// Outcome of an A/B test comparing two statute variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbTestResult {
    /// Name of the test.
    pub test_name: String,
    /// Label of the control variant.
    pub control_label: String,
    /// Label of the treatment variant.
    pub treatment_label: String,
    /// Coverage observed in the control arm.
    pub control_coverage: f64,
    /// Coverage observed in the treatment arm.
    pub treatment_coverage: f64,
    /// Absolute lift in coverage (`treatment - control`).
    pub absolute_effect: f64,
    /// Relative lift in coverage (`absolute / control`).
    pub relative_effect: f64,
    /// Two-proportion z-statistic.
    pub z_statistic: f64,
    /// Two-sided p-value derived from the normal distribution.
    pub p_value: f64,
    /// Cohen's d effect size on the monetary deltas.
    pub cohens_d: f64,
    /// Significance level used.
    pub alpha: f64,
    /// Whether the difference is statistically significant at `alpha`.
    pub significant: bool,
    /// Full impact report for the control arm.
    pub control_report: ImpactReport,
    /// Full impact report for the treatment arm.
    pub treatment_report: ImpactReport,
    /// When the result was generated.
    pub generated_at: DateTime<Utc>,
}

impl AbTestResult {
    /// Returns whether the result is statistically significant at `alpha`.
    #[must_use]
    pub fn is_significant(&self) -> bool {
        self.significant
    }

    /// Returns the winning variant label, if the difference is significant.
    #[must_use]
    pub fn winner(&self) -> Option<&str> {
        if !self.significant {
            return None;
        }
        if self.treatment_coverage > self.control_coverage {
            Some(&self.treatment_label)
        } else if self.control_coverage > self.treatment_coverage {
            Some(&self.control_label)
        } else {
            None
        }
    }

    /// Returns a qualitative magnitude label for the Cohen's d effect size.
    #[must_use]
    pub fn effect_magnitude(&self) -> &'static str {
        let d = self.cohens_d.abs();
        if d < 0.2 {
            "negligible"
        } else if d < 0.5 {
            "small"
        } else if d < 0.8 {
            "medium"
        } else {
            "large"
        }
    }
}

/// Maps a salted entity identifier to a uniform fraction in `[0, 1)`.
fn hash_fraction(salt: &str, entity_id: &str) -> f64 {
    let mut hasher = Sha256::new();
    hasher.update(salt.as_bytes());
    hasher.update(b":");
    hasher.update(entity_id.as_bytes());
    let digest = hasher.finalize();
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    let value = u64::from_be_bytes(bytes);
    value as f64 / u64::MAX as f64
}

/// Computes the arithmetic mean of a sample (0.0 for an empty sample).
#[must_use]
pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

/// Computes the unbiased sample variance (0.0 when fewer than two values).
#[must_use]
pub fn sample_variance(values: &[f64]) -> f64 {
    let n = values.len();
    if n < 2 {
        return 0.0;
    }
    let m = mean(values);
    let sum_squares: f64 = values
        .iter()
        .map(|v| {
            let diff = v - m;
            diff * diff
        })
        .sum();
    sum_squares / (n as f64 - 1.0)
}

/// Computes Cohen's d effect size between two samples using a pooled standard deviation.
#[must_use]
pub fn cohens_d(sample_a: &[f64], sample_b: &[f64]) -> f64 {
    let n1 = sample_a.len();
    let n2 = sample_b.len();
    if n1 < 2 || n2 < 2 {
        return 0.0;
    }
    let var1 = sample_variance(sample_a);
    let var2 = sample_variance(sample_b);
    let pooled = ((n1 - 1) as f64 * var1 + (n2 - 1) as f64 * var2) / (n1 + n2 - 2) as f64;
    let pooled_sd = pooled.sqrt();
    if pooled_sd == 0.0 {
        return 0.0;
    }
    (mean(sample_a) - mean(sample_b)) / pooled_sd
}

/// Computes the two-proportion z-statistic for a pooled test of equal proportions.
#[must_use]
pub fn two_proportion_z(
    successes_a: usize,
    total_a: usize,
    successes_b: usize,
    total_b: usize,
) -> f64 {
    if total_a == 0 || total_b == 0 {
        return 0.0;
    }
    let p1 = successes_a as f64 / total_a as f64;
    let p2 = successes_b as f64 / total_b as f64;
    let pooled = (successes_a + successes_b) as f64 / (total_a + total_b) as f64;
    let denominator =
        (pooled * (1.0 - pooled) * (1.0 / total_a as f64 + 1.0 / total_b as f64)).sqrt();
    if denominator == 0.0 {
        return 0.0;
    }
    (p1 - p2) / denominator
}

/// Approximates the Gauss error function using the Abramowitz & Stegun 7.1.26 formula.
#[must_use]
pub fn erf(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let y = 1.0
        - (((((1.061_405_429 * t - 1.453_152_027) * t) + 1.421_413_741) * t - 0.284_496_736) * t
            + 0.254_829_592)
            * t
            * (-x * x).exp();
    sign * y
}

/// Computes the standard normal cumulative distribution function.
#[must_use]
pub fn normal_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// Computes a two-sided p-value from a z-statistic via the normal distribution.
#[must_use]
pub fn two_sided_p_value(z: f64) -> f64 {
    let p = 2.0 * (1.0 - normal_cdf(z.abs()));
    p.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn variant(label: &str, min_age: u32) -> StatuteVariant {
        let statute = Statute::new(
            format!("statute-{label}"),
            "Benefit",
            Effect::new(EffectType::MonetaryTransfer, "subsidy").with_parameter("amount", "100"),
        )
        .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, min_age));
        StatuteVariant::new(label, statute)
    }

    fn population(count: usize) -> Vec<SyntheticEntity> {
        (0..count)
            .map(|idx| {
                // Ages spread deterministically across 40-99.
                let age = 40 + (idx % 60);
                SyntheticEntity::new(format!("entity-{idx}")).with_attribute("age", age.to_string())
            })
            .collect()
    }

    #[test]
    fn test_assignment_is_deterministic() {
        let test = AbTest::new("t", variant("c", 50), variant("v", 50));
        let first = test.assign("entity-42");
        let second = test.assign("entity-42");
        assert_eq!(first, second);
    }

    #[test]
    fn test_split_extremes() {
        let all_control =
            AbTest::new("t", variant("c", 50), variant("v", 50)).with_split_ratio(0.0);
        let all_treatment =
            AbTest::new("t", variant("c", 50), variant("v", 50)).with_split_ratio(1.0);
        for idx in 0..50 {
            let id = format!("entity-{idx}");
            assert_eq!(all_control.assign(&id), CohortArm::Control);
            assert_eq!(all_treatment.assign(&id), CohortArm::Treatment);
        }
    }

    #[test]
    fn test_run_partitions_entities() {
        let test = AbTest::new("t", variant("c", 50), variant("v", 50));
        let entities = population(200);
        let result = test.run(&entities);
        let total = result.control_report.sample_size + result.treatment_report.sample_size;
        assert_eq!(total, 200);
        assert!(result.control_report.sample_size > 0);
        assert!(result.treatment_report.sample_size > 0);
    }

    #[test]
    fn test_significant_difference_detected() {
        // Control requires age >= 100 (almost nobody), treatment age >= 40 (everybody).
        let test = AbTest::new("t", variant("c", 100), variant("v", 40));
        let entities = population(400);
        let result = test.run(&entities);
        assert!(result.treatment_coverage > result.control_coverage);
        assert!(result.is_significant());
        assert_eq!(result.winner(), Some("v"));
    }

    #[test]
    fn test_no_significance_for_identical_variants() {
        let test = AbTest::new("t", variant("c", 50), variant("v", 50));
        let entities = population(300);
        let result = test.run(&entities);
        // Both arms use age >= 50; coverage should be statistically indistinguishable.
        assert!(result.winner().is_none() || !result.significant);
    }

    #[test]
    fn test_erf_accuracy() {
        // erf(0) = 0, erf(1) ~= 0.8427007929.
        assert!(erf(0.0).abs() < 1e-7);
        assert!((erf(1.0) - 0.842_700_792_9).abs() < 1e-6);
        assert!((erf(-1.0) + 0.842_700_792_9).abs() < 1e-6);
    }

    #[test]
    fn test_normal_cdf_known_values() {
        assert!((normal_cdf(0.0) - 0.5).abs() < 1e-9);
        // Approximately 0.8413 for z = 1.
        assert!((normal_cdf(1.0) - 0.841_344_75).abs() < 1e-5);
    }

    #[test]
    fn test_cohens_d_and_variance() {
        let a = [1.0, 2.0, 3.0, 4.0];
        let b = [3.0, 4.0, 5.0, 6.0];
        // Sample variance of {1,2,3,4} is 5/3.
        assert!((sample_variance(&a) - (5.0 / 3.0)).abs() < 1e-9);
        let d = cohens_d(&a, &b);
        // means differ by -2, pooled sd ~ 1.291 -> d ~ -1.549.
        assert!(d < 0.0);
        assert!((d + 1.549).abs() < 0.01);
    }

    #[test]
    fn test_two_proportion_z_guards_zero() {
        assert_eq!(two_proportion_z(0, 0, 1, 10), 0.0);
        assert_eq!(two_proportion_z(5, 10, 5, 10), 0.0);
    }
}
