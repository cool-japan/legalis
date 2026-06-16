//! Inferential statistics for production monitoring.
//!
//! Pure-Rust implementations of the distribution functions and hypothesis tests
//! needed for rigorous A/B test analysis and anomaly scoring, with no external
//! numerical dependency:
//!
//! * [`erf`] / [`normal_cdf`] - the Gauss error function and standard-normal CDF.
//! * [`ln_gamma`] - the natural log of the gamma function (Lanczos approximation).
//! * [`regularized_incomplete_beta`] - `I_x(a, b)` via the Lentz continued fraction.
//! * [`student_t_cdf`] - the Student-t cumulative distribution function.
//! * [`TwoProportionTest`] - a two-sided two-proportion z-test (for success rates).
//! * [`WelchTTest`] - Welch's unequal-variance t-test (for continuous metrics).

use serde::{Deserialize, Serialize};

/// The Gauss error function `erf(x)`.
///
/// Uses the Abramowitz & Stegun 7.1.26 rational approximation, which has a
/// maximum absolute error below `1.5e-7` across the whole real line - more than
/// enough precision for p-value computation.
pub fn erf(x: f64) -> f64 {
    // Constants for A&S 7.1.26.
    const A1: f64 = 0.254_829_592;
    const A2: f64 = -0.284_496_736;
    const A3: f64 = 1.421_413_741;
    const A4: f64 = -1.453_152_027;
    const A5: f64 = 1.061_405_429;
    const P: f64 = 0.327_591_1;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x_abs = x.abs();

    let t = 1.0 / (1.0 + P * x_abs);
    let y = 1.0 - (((((A5 * t + A4) * t) + A3) * t + A2) * t + A1) * t * (-x_abs * x_abs).exp();

    sign * y
}

/// The standard-normal cumulative distribution function `Phi(z)`.
pub fn normal_cdf(z: f64) -> f64 {
    0.5 * (1.0 + erf(z / std::f64::consts::SQRT_2))
}

/// The natural logarithm of the gamma function, `ln Gamma(x)`, for `x > 0`.
///
/// Uses the Lanczos approximation (g = 7, n = 9), accurate to ~15 significant
/// digits for positive real arguments.
pub fn ln_gamma(x: f64) -> f64 {
    const G: f64 = 7.0;
    // Standard Lanczos g=7 coefficients; written at full published precision.
    #[allow(clippy::excessive_precision)]
    const COEFFICIENTS: [f64; 9] = [
        0.999_999_999_999_809_93,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_13,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];

    if x < 0.5 {
        // Reflection formula: Gamma(x)Gamma(1-x) = pi / sin(pi x).
        let pi = std::f64::consts::PI;
        (pi / (pi * x).sin()).ln() - ln_gamma(1.0 - x)
    } else {
        let x = x - 1.0;
        let mut a = COEFFICIENTS[0];
        let t = x + G + 0.5;
        for (index, coefficient) in COEFFICIENTS.iter().enumerate().skip(1) {
            a += coefficient / (x + index as f64);
        }
        0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
    }
}

/// The regularized incomplete beta function `I_x(a, b)`.
///
/// Evaluated with the modified Lentz continued fraction (Numerical Recipes
/// `betacf`), switching to the symmetry relation `I_x(a,b) = 1 - I_{1-x}(b,a)`
/// for fast convergence. Returns values in `[0, 1]`; out-of-domain `x` is
/// clamped to the boundary.
pub fn regularized_incomplete_beta(x: f64, a: f64, b: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }

    let ln_beta = ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b);
    let front = (a * x.ln() + b * (1.0 - x).ln() + ln_beta).exp();

    if x < (a + 1.0) / (a + b + 2.0) {
        front * beta_continued_fraction(x, a, b) / a
    } else {
        1.0 - front * beta_continued_fraction(1.0 - x, b, a) / b
    }
}

/// Lentz's algorithm for the continued fraction of the incomplete beta function.
fn beta_continued_fraction(x: f64, a: f64, b: f64) -> f64 {
    const MAX_ITERATIONS: usize = 200;
    const EPSILON: f64 = 1e-12;
    const TINY: f64 = 1e-30;

    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;

    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < TINY {
        d = TINY;
    }
    d = 1.0 / d;
    let mut result = d;

    for m in 1..=MAX_ITERATIONS {
        let m_f = m as f64;
        let two_m = 2.0 * m_f;

        // Even step.
        let numerator_even = m_f * (b - m_f) * x / ((qam + two_m) * (a + two_m));
        d = 1.0 + numerator_even * d;
        if d.abs() < TINY {
            d = TINY;
        }
        c = 1.0 + numerator_even / c;
        if c.abs() < TINY {
            c = TINY;
        }
        d = 1.0 / d;
        result *= d * c;

        // Odd step.
        let numerator_odd = -(a + m_f) * (qab + m_f) * x / ((a + two_m) * (qap + two_m));
        d = 1.0 + numerator_odd * d;
        if d.abs() < TINY {
            d = TINY;
        }
        c = 1.0 + numerator_odd / c;
        if c.abs() < TINY {
            c = TINY;
        }
        d = 1.0 / d;
        let delta = d * c;
        result *= delta;

        if (delta - 1.0).abs() < EPSILON {
            break;
        }
    }

    result
}

/// The Student-t cumulative distribution function `P(T <= t)` with `df` degrees
/// of freedom.
///
/// Expressed through the regularized incomplete beta function. For very large
/// `df` it converges to [`normal_cdf`], which is also used as a guard for
/// degenerate (`df <= 0`) inputs.
pub fn student_t_cdf(t: f64, df: f64) -> f64 {
    if df <= 0.0 {
        return normal_cdf(t);
    }
    let x = df / (df + t * t);
    let ib = 0.5 * regularized_incomplete_beta(x, df / 2.0, 0.5);
    if t > 0.0 { 1.0 - ib } else { ib }
}

/// The two-sided p-value for a standard-normal test statistic `z`.
pub fn two_sided_normal_p_value(z: f64) -> f64 {
    2.0 * (1.0 - normal_cdf(z.abs()))
}

/// The two-sided p-value for a Student-t statistic `t` with `df` degrees of
/// freedom.
pub fn two_sided_t_p_value(t: f64, df: f64) -> f64 {
    2.0 * (1.0 - student_t_cdf(t.abs(), df))
}

/// The result of a two-proportion z-test comparing two success rates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TwoProportionTest {
    /// Successes in group A.
    pub successes_a: usize,
    /// Trials in group A.
    pub trials_a: usize,
    /// Successes in group B.
    pub successes_b: usize,
    /// Trials in group B.
    pub trials_b: usize,
    /// Observed proportion in group A.
    pub proportion_a: f64,
    /// Observed proportion in group B.
    pub proportion_b: f64,
    /// The z test statistic (`B - A` direction).
    pub z_statistic: f64,
    /// The two-sided p-value.
    pub p_value: f64,
}

impl TwoProportionTest {
    /// Runs a two-sided two-proportion z-test.
    ///
    /// The pooled-variance estimator is used under the null hypothesis that the
    /// two proportions are equal. Returns `None` when either group has no trials
    /// (the test is undefined) or the pooled variance is degenerate.
    pub fn run(
        successes_a: usize,
        trials_a: usize,
        successes_b: usize,
        trials_b: usize,
    ) -> Option<Self> {
        if trials_a == 0 || trials_b == 0 {
            return None;
        }
        let n_a = trials_a as f64;
        let n_b = trials_b as f64;
        let p_a = successes_a as f64 / n_a;
        let p_b = successes_b as f64 / n_b;
        let pooled = (successes_a + successes_b) as f64 / (n_a + n_b);
        let standard_error = (pooled * (1.0 - pooled) * (1.0 / n_a + 1.0 / n_b)).sqrt();
        if standard_error <= 0.0 {
            return None;
        }
        let z = (p_b - p_a) / standard_error;
        Some(Self {
            successes_a,
            trials_a,
            successes_b,
            trials_b,
            proportion_a: p_a,
            proportion_b: p_b,
            z_statistic: z,
            p_value: two_sided_normal_p_value(z),
        })
    }
}

/// The result of Welch's unequal-variance two-sample t-test.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WelchTTest {
    /// Mean of group A.
    pub mean_a: f64,
    /// Mean of group B.
    pub mean_b: f64,
    /// Number of observations in group A.
    pub n_a: usize,
    /// Number of observations in group B.
    pub n_b: usize,
    /// The t test statistic (`B - A` direction).
    pub t_statistic: f64,
    /// The Welch-Satterthwaite degrees of freedom.
    pub degrees_of_freedom: f64,
    /// The two-sided p-value.
    pub p_value: f64,
}

impl WelchTTest {
    /// Runs Welch's two-sided t-test on two samples.
    ///
    /// Returns `None` when either sample has fewer than two observations (the
    /// variance is undefined) or both variances are zero.
    pub fn run(sample_a: &[f64], sample_b: &[f64]) -> Option<Self> {
        if sample_a.len() < 2 || sample_b.len() < 2 {
            return None;
        }
        let n_a = sample_a.len() as f64;
        let n_b = sample_b.len() as f64;
        let mean_a = super::mean(sample_a);
        let mean_b = super::mean(sample_b);
        let var_a = super::sample_variance(sample_a);
        let var_b = super::sample_variance(sample_b);

        let se_a = var_a / n_a;
        let se_b = var_b / n_b;
        let standard_error = (se_a + se_b).sqrt();
        if standard_error <= 0.0 {
            return None;
        }

        let t = (mean_b - mean_a) / standard_error;
        // Welch-Satterthwaite degrees of freedom.
        let df_numerator = (se_a + se_b).powi(2);
        let df_denominator = se_a.powi(2) / (n_a - 1.0) + se_b.powi(2) / (n_b - 1.0);
        let df = if df_denominator > 0.0 {
            df_numerator / df_denominator
        } else {
            n_a + n_b - 2.0
        };

        Some(Self {
            mean_a,
            mean_b,
            n_a: sample_a.len(),
            n_b: sample_b.len(),
            t_statistic: t,
            degrees_of_freedom: df,
            p_value: two_sided_t_p_value(t, df),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erf_known_values() {
        assert!((erf(0.0)).abs() < 1e-7);
        assert!((erf(1.0) - 0.842_700_79).abs() < 1e-6);
        assert!((erf(-1.0) + 0.842_700_79).abs() < 1e-6);
        // erf saturates to +-1.
        assert!((erf(5.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_normal_cdf() {
        assert!((normal_cdf(0.0) - 0.5).abs() < 1e-7);
        // ~95% of mass below 1.645, ~97.5% below 1.96.
        assert!((normal_cdf(1.644_854) - 0.95).abs() < 1e-3);
        assert!((normal_cdf(1.959_964) - 0.975).abs() < 1e-3);
        assert!((normal_cdf(-1.959_964) - 0.025).abs() < 1e-3);
    }

    #[test]
    fn test_ln_gamma() {
        // Gamma(5) = 24 => ln(24).
        assert!((ln_gamma(5.0) - 24.0_f64.ln()).abs() < 1e-9);
        // Gamma(0.5) = sqrt(pi).
        assert!((ln_gamma(0.5) - std::f64::consts::PI.sqrt().ln()).abs() < 1e-9);
        // Gamma(1) = Gamma(2) = 1 => ln = 0.
        assert!(ln_gamma(1.0).abs() < 1e-9);
        assert!(ln_gamma(2.0).abs() < 1e-9);
    }

    #[test]
    fn test_incomplete_beta_symmetry() {
        // I_0.5(a, a) = 0.5 by symmetry.
        assert!((regularized_incomplete_beta(0.5, 2.0, 2.0) - 0.5).abs() < 1e-9);
        assert!((regularized_incomplete_beta(0.5, 5.0, 5.0) - 0.5).abs() < 1e-9);
        // Boundaries.
        assert_eq!(regularized_incomplete_beta(0.0, 2.0, 3.0), 0.0);
        assert_eq!(regularized_incomplete_beta(1.0, 2.0, 3.0), 1.0);
    }

    #[test]
    fn test_student_t_cdf() {
        // Symmetric about zero.
        assert!((student_t_cdf(0.0, 10.0) - 0.5).abs() < 1e-9);
        // With large df it approaches the normal CDF.
        let t_large_df = student_t_cdf(1.96, 100_000.0);
        assert!((t_large_df - normal_cdf(1.96)).abs() < 1e-3);
        // Classic table value: P(T <= 2.228) ~ 0.975 with df = 10.
        assert!((student_t_cdf(2.228, 10.0) - 0.975).abs() < 1e-3);
    }

    #[test]
    fn test_two_proportion_test_significant() {
        // 90/100 vs 60/100 is a large, significant difference.
        let test = TwoProportionTest::run(60, 100, 90, 100).expect("defined");
        assert!(test.proportion_b > test.proportion_a);
        assert!(test.z_statistic > 0.0);
        assert!(test.p_value < 0.01);
    }

    #[test]
    fn test_two_proportion_test_not_significant() {
        // 50/100 vs 52/100 is well within noise.
        let test = TwoProportionTest::run(50, 100, 52, 100).expect("defined");
        assert!(test.p_value > 0.05);
        // Undefined when a group is empty.
        assert!(TwoProportionTest::run(0, 0, 1, 10).is_none());
    }

    #[test]
    fn test_welch_t_test() {
        let a = [10.0, 11.0, 9.0, 10.5, 9.5, 10.0, 10.2];
        let b = [20.0, 21.0, 19.0, 20.5, 19.5, 20.0, 20.2];
        let test = WelchTTest::run(&a, &b).expect("defined");
        assert!(test.mean_b > test.mean_a);
        assert!(test.t_statistic > 0.0);
        assert!(test.p_value < 0.001);

        // Identical-ish samples are not significant.
        let c = [10.0, 11.0, 9.0, 10.5, 9.5];
        let d = [10.1, 10.9, 9.1, 10.4, 9.6];
        let test2 = WelchTTest::run(&c, &d).expect("defined");
        assert!(test2.p_value > 0.05);

        assert!(WelchTTest::run(&[1.0], &[1.0, 2.0]).is_none());
    }
}
