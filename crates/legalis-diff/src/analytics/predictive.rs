//! Predictive analytics for future statute changes.
//!
//! This module extrapolates the historical change behaviour of a statute (as
//! captured by a sequence of [`StatuteDiff`]s) into the future. The core model
//! is ordinary least-squares (OLS) linear regression over the per-revision
//! change count, augmented with the regression's coefficient of determination
//! (`r_squared`) so callers can judge how trustworthy a projection is.
//!
//! Unlike [`crate::timeseries`], which works against wall-clock timestamps,
//! this module operates on the *revision index* (revision 0, 1, 2, …). That
//! makes it usable even when timestamps are unavailable and keeps the forecast
//! purely a function of the ordered diff history.
//!
//! # Example
//!
//! ```
//! use legalis_diff::analytics::{forecast_change_volume, ChangeSeriesPoint};
//!
//! // Each revision touched a growing number of provisions.
//! let history = vec![
//!     ChangeSeriesPoint::new(0, 1),
//!     ChangeSeriesPoint::new(1, 2),
//!     ChangeSeriesPoint::new(2, 3),
//!     ChangeSeriesPoint::new(3, 4),
//! ];
//!
//! let forecast = forecast_change_volume(&history, 2);
//! // Two future revisions are projected.
//! assert_eq!(forecast.projections.len(), 2);
//! // The trend is unmistakably linear and increasing.
//! assert!(forecast.model.slope > 0.9);
//! assert!(forecast.model.r_squared > 0.99);
//! ```

use crate::StatuteDiff;
use serde::{Deserialize, Serialize};

/// A single observation in a change-volume time series.
///
/// `revision` is a monotonically increasing ordinal (it need not start at zero
/// or be contiguous); `change_count` is the number of changes recorded for that
/// revision.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ChangeSeriesPoint {
    /// Ordinal revision index used as the regression's independent variable.
    pub revision: u64,
    /// Number of changes observed at this revision (the dependent variable).
    pub change_count: f64,
}

impl ChangeSeriesPoint {
    /// Creates a new series point from a revision index and an integral count.
    pub fn new(revision: u64, change_count: u64) -> Self {
        Self {
            revision,
            change_count: change_count as f64,
        }
    }

    /// Creates a series point from an explicit floating-point value (useful when
    /// the dependent variable is a continuous metric such as a risk score).
    pub fn from_value(revision: u64, value: f64) -> Self {
        Self {
            revision,
            change_count: value,
        }
    }
}

/// Builds a change-volume series from an ordered slice of diffs.
///
/// Diff `i` becomes the point `(i, diffs[i].changes.len())`.
pub fn series_from_diffs(diffs: &[StatuteDiff]) -> Vec<ChangeSeriesPoint> {
    diffs
        .iter()
        .enumerate()
        .map(|(i, d)| ChangeSeriesPoint::new(i as u64, d.changes.len() as u64))
        .collect()
}

/// A fitted simple-linear-regression model: `y = slope * x + intercept`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LinearModel {
    /// Rate of change of the dependent variable per unit revision.
    pub slope: f64,
    /// Value of the dependent variable at revision 0 (the fitted intercept).
    pub intercept: f64,
    /// Coefficient of determination in `[0, 1]`; higher means a better fit.
    pub r_squared: f64,
    /// Number of observations the model was fitted from.
    pub sample_size: usize,
}

impl LinearModel {
    /// Predicts the dependent variable at an arbitrary revision index.
    pub fn predict(&self, revision: f64) -> f64 {
        self.slope * revision + self.intercept
    }

    /// Whether the model can be trusted for extrapolation.
    ///
    /// A model is considered reliable when it was fitted from at least three
    /// points and explains at least 50 % of the observed variance.
    pub fn is_reliable(&self) -> bool {
        self.sample_size >= 3 && self.r_squared >= 0.5
    }
}

/// Fits an OLS linear regression to a change series.
///
/// Returns a degenerate zero-slope model anchored at the mean (or at the origin
/// for an empty series) when the independent variable has no variance, which
/// keeps the function total and panic-free.
pub fn fit_linear_model(points: &[ChangeSeriesPoint]) -> LinearModel {
    let n = points.len();
    if n == 0 {
        return LinearModel {
            slope: 0.0,
            intercept: 0.0,
            r_squared: 0.0,
            sample_size: 0,
        };
    }

    let n_f = n as f64;
    let mean_x = points.iter().map(|p| p.revision as f64).sum::<f64>() / n_f;
    let mean_y = points.iter().map(|p| p.change_count).sum::<f64>() / n_f;

    let mut s_xy = 0.0;
    let mut s_xx = 0.0;
    let mut s_yy = 0.0;
    for p in points {
        let dx = p.revision as f64 - mean_x;
        let dy = p.change_count - mean_y;
        s_xy += dx * dy;
        s_xx += dx * dx;
        s_yy += dy * dy;
    }

    // No spread in x (e.g. a single point, or all identical revisions): the best
    // line is horizontal through the mean of y.
    if s_xx.abs() < f64::EPSILON {
        return LinearModel {
            slope: 0.0,
            intercept: mean_y,
            r_squared: 0.0,
            sample_size: n,
        };
    }

    let slope = s_xy / s_xx;
    let intercept = mean_y - slope * mean_x;

    // r^2 = (explained variance) / (total variance). When y is constant the
    // total variance is zero and the fit is, by convention, perfect.
    let r_squared = if s_yy.abs() < f64::EPSILON {
        1.0
    } else {
        let r = s_xy / (s_xx.sqrt() * s_yy.sqrt());
        (r * r).clamp(0.0, 1.0)
    };

    LinearModel {
        slope,
        intercept,
        r_squared,
        sample_size: n,
    }
}

/// A single projected future revision.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ChangeProjection {
    /// The future revision index this projection is for.
    pub revision: u64,
    /// Point estimate of the change count (never negative).
    pub expected_changes: f64,
    /// Lower bound of the projection interval (never negative).
    pub lower_bound: f64,
    /// Upper bound of the projection interval.
    pub upper_bound: f64,
}

/// The result of a change-volume forecast.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeForecast {
    /// The regression model the forecast is built on.
    pub model: LinearModel,
    /// Projections for each requested future revision.
    pub projections: Vec<ChangeProjection>,
    /// Residual standard deviation of the fit, used to size the interval.
    pub residual_std: f64,
}

impl ChangeForecast {
    /// Total number of changes expected across the whole forecast horizon.
    pub fn total_expected(&self) -> f64 {
        self.projections.iter().map(|p| p.expected_changes).sum()
    }
}

/// Forecasts the change volume of the next `horizon` revisions.
///
/// The interval half-width is the residual standard deviation of the fit,
/// giving a roughly one-sigma band around each point estimate. Estimates and
/// bounds are floored at zero because a negative number of changes is
/// meaningless.
pub fn forecast_change_volume(history: &[ChangeSeriesPoint], horizon: usize) -> ChangeForecast {
    let model = fit_linear_model(history);

    // Residual standard deviation (population form over the fitted points).
    let residual_std = if history.len() >= 2 {
        let sse: f64 = history
            .iter()
            .map(|p| {
                let predicted = model.predict(p.revision as f64);
                let resid = p.change_count - predicted;
                resid * resid
            })
            .sum();
        (sse / history.len() as f64).sqrt()
    } else {
        0.0
    };

    let next_revision = history.last().map(|p| p.revision + 1).unwrap_or(0);

    let projections = (0..horizon as u64)
        .map(|offset| {
            let revision = next_revision + offset;
            let expected = model.predict(revision as f64).max(0.0);
            ChangeProjection {
                revision,
                expected_changes: expected,
                lower_bound: (expected - residual_std).max(0.0),
                upper_bound: expected + residual_std,
            }
        })
        .collect();

    ChangeForecast {
        model,
        projections,
        residual_std,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linear_history(n: u64, slope: u64) -> Vec<ChangeSeriesPoint> {
        (0..n)
            .map(|i| ChangeSeriesPoint::new(i, i * slope))
            .collect()
    }

    #[test]
    fn test_fit_perfect_linear() {
        let pts = linear_history(5, 2);
        let model = fit_linear_model(&pts);
        assert!((model.slope - 2.0).abs() < 1e-9);
        assert!(model.intercept.abs() < 1e-9);
        assert!((model.r_squared - 1.0).abs() < 1e-9);
        assert_eq!(model.sample_size, 5);
        assert!(model.is_reliable());
    }

    #[test]
    fn test_fit_empty() {
        let model = fit_linear_model(&[]);
        assert_eq!(model.slope, 0.0);
        assert_eq!(model.sample_size, 0);
        assert!(!model.is_reliable());
    }

    #[test]
    fn test_fit_single_point() {
        let model = fit_linear_model(&[ChangeSeriesPoint::new(7, 3)]);
        assert_eq!(model.slope, 0.0);
        assert!((model.intercept - 3.0).abs() < 1e-9);
        assert!(!model.is_reliable());
    }

    #[test]
    fn test_fit_constant_y() {
        let pts: Vec<_> = (0..4).map(|i| ChangeSeriesPoint::new(i, 5)).collect();
        let model = fit_linear_model(&pts);
        assert!(model.slope.abs() < 1e-9);
        assert!((model.intercept - 5.0).abs() < 1e-9);
        // Constant y is treated as a perfect (if uninformative) fit.
        assert!((model.r_squared - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_forecast_projects_horizon() {
        let pts = linear_history(4, 1);
        let forecast = forecast_change_volume(&pts, 3);
        assert_eq!(forecast.projections.len(), 3);
        assert_eq!(forecast.projections[0].revision, 4);
        assert_eq!(forecast.projections[2].revision, 6);
        // Increasing trend => later projections are larger.
        assert!(
            forecast.projections[2].expected_changes > forecast.projections[0].expected_changes
        );
    }

    #[test]
    fn test_forecast_floors_at_zero() {
        // Strongly decreasing trend should not yield negative projections.
        let pts: Vec<_> = (0..5)
            .map(|i| ChangeSeriesPoint::from_value(i, 10.0 - 3.0 * i as f64))
            .collect();
        let forecast = forecast_change_volume(&pts, 5);
        for p in &forecast.projections {
            assert!(p.expected_changes >= 0.0);
            assert!(p.lower_bound >= 0.0);
        }
    }

    #[test]
    fn test_total_expected() {
        let pts = linear_history(3, 2);
        let forecast = forecast_change_volume(&pts, 2);
        let total: f64 = forecast
            .projections
            .iter()
            .map(|p| p.expected_changes)
            .sum();
        assert!((forecast.total_expected() - total).abs() < 1e-9);
    }

    #[test]
    fn test_residual_std_zero_for_perfect_fit() {
        let pts = linear_history(6, 4);
        let forecast = forecast_change_volume(&pts, 1);
        assert!(forecast.residual_std < 1e-9);
    }

    #[test]
    fn test_predict_arbitrary() {
        let model = LinearModel {
            slope: 1.5,
            intercept: 2.0,
            r_squared: 1.0,
            sample_size: 4,
        };
        assert!((model.predict(4.0) - 8.0).abs() < 1e-9);
    }
}
