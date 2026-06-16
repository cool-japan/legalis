//! Change-impact forecasting.
//!
//! Where [`super::predictive`] projects the *volume* of future changes, this
//! module projects their *impact*. It does so by treating the per-revision risk
//! score (from [`super::risk::assess_risk`]) as a time series, fitting the same
//! linear model used elsewhere, and extrapolating both the score and its banded
//! [`super::risk::RiskLevel`] forward.
//!
//! The forecast also exposes the *trajectory* — whether risk is escalating,
//! stable, or de-escalating — which is often the actionable signal for a
//! compliance team: a statute under sustained, increasing-risk revision merits
//! attention even before any single change is alarming.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::diff;
//! use legalis_diff::analytics::{forecast_impact, ImpactTrajectory};
//!
//! let base = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
//!
//! // A history of escalating diffs: title edit, then effect change.
//! let mut t1 = base.clone();
//! t1.title = "Edited".into();
//! let mut t2 = base.clone();
//! t2.effect = Effect::new(EffectType::Revoke, "Revoked");
//! let history = vec![
//!     diff(&base, &base).unwrap(),
//!     diff(&base, &t1).unwrap(),
//!     diff(&base, &t2).unwrap(),
//! ];
//!
//! let forecast = forecast_impact(&history, 2);
//! assert_eq!(forecast.projected_scores.len(), 2);
//! assert_eq!(forecast.trajectory, ImpactTrajectory::Escalating);
//! ```

use super::predictive::{ChangeSeriesPoint, LinearModel, fit_linear_model};
use super::risk::{RiskLevel, assess_risk};
use crate::StatuteDiff;
use serde::{Deserialize, Serialize};

/// Direction of the forecast risk trend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactTrajectory {
    /// Risk is trending materially upward.
    Escalating,
    /// Risk is roughly flat.
    Stable,
    /// Risk is trending materially downward.
    DeEscalating,
}

impl ImpactTrajectory {
    /// Derives a trajectory from a fitted model's slope (points per revision).
    ///
    /// A slope whose magnitude is below `epsilon` is treated as flat.
    fn from_slope(slope: f64, epsilon: f64) -> Self {
        if slope > epsilon {
            Self::Escalating
        } else if slope < -epsilon {
            Self::DeEscalating
        } else {
            Self::Stable
        }
    }
}

/// A single projected future-impact point.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ImpactProjection {
    /// Future revision index.
    pub revision: u64,
    /// Projected risk score in `[0, 100]`.
    pub score: f64,
    /// Banded risk level for the projected score.
    pub level: RiskLevel,
}

/// The result of an impact forecast.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpactForecast {
    /// Linear model fitted to historical risk scores.
    pub model: LinearModel,
    /// Overall trajectory of risk over the history.
    pub trajectory: ImpactTrajectory,
    /// Per-revision projected risk scores.
    pub projected_scores: Vec<ImpactProjection>,
    /// Mean historical risk score (the baseline the forecast departs from).
    pub historical_mean: f64,
}

impl ImpactForecast {
    /// The highest projected risk level across the forecast horizon, or `None`
    /// when the horizon is empty.
    pub fn peak_level(&self) -> Option<RiskLevel> {
        self.projected_scores.iter().map(|p| p.level).max()
    }
}

/// Forecasts the impact (risk) of the next `horizon` revisions from history.
///
/// The slope threshold for classifying the trajectory scales with the
/// observed history so that small absolute drifts on a long, noisy history are
/// not over-interpreted: it is fixed at two points of risk score per revision.
pub fn forecast_impact(history: &[StatuteDiff], horizon: usize) -> ImpactForecast {
    let points: Vec<ChangeSeriesPoint> = history
        .iter()
        .enumerate()
        .map(|(i, d)| ChangeSeriesPoint::from_value(i as u64, assess_risk(d).score))
        .collect();

    let model = fit_linear_model(&points);
    let trajectory = ImpactTrajectory::from_slope(model.slope, 2.0);

    let historical_mean = if points.is_empty() {
        0.0
    } else {
        points.iter().map(|p| p.change_count).sum::<f64>() / points.len() as f64
    };

    let next_revision = points.last().map(|p| p.revision + 1).unwrap_or(0);

    let projected_scores = (0..horizon as u64)
        .map(|offset| {
            let revision = next_revision + offset;
            let score = model.predict(revision as f64).clamp(0.0, 100.0);
            ImpactProjection {
                revision,
                score,
                level: RiskLevel::from_score(score),
            }
        })
        .collect();

    ImpactForecast {
        model,
        trajectory,
        projected_scores,
        historical_mean,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    fn base() -> Statute {
        Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
    }

    #[test]
    fn test_escalating_trajectory() {
        let b = base();
        let mut t1 = b.clone();
        t1.title = "Edited".into();
        let mut t2 = b.clone();
        t2.effect = Effect::new(EffectType::Revoke, "Revoked");
        let history = vec![
            crate::diff(&b, &b).expect("diff"),
            crate::diff(&b, &t1).expect("diff"),
            crate::diff(&b, &t2).expect("diff"),
        ];
        let forecast = forecast_impact(&history, 2);
        assert_eq!(forecast.trajectory, ImpactTrajectory::Escalating);
        assert_eq!(forecast.projected_scores.len(), 2);
    }

    #[test]
    fn test_stable_trajectory() {
        let b = base();
        let mut edited = b.clone();
        edited.title = "Edited".into();
        // Identical low-risk diffs => flat risk.
        let history: Vec<_> = (0..4)
            .map(|_| crate::diff(&b, &edited).expect("diff"))
            .collect();
        let forecast = forecast_impact(&history, 1);
        assert_eq!(forecast.trajectory, ImpactTrajectory::Stable);
    }

    #[test]
    fn test_de_escalating_trajectory() {
        let b = base();
        let mut big = b.clone();
        big.effect = Effect::new(EffectType::Revoke, "Revoked");
        big.discretion_logic = Some("discretion".into());
        let mut small = b.clone();
        small.title = "Edited".into();
        // High risk first, then progressively lower.
        let history = vec![
            crate::diff(&b, &big).expect("diff"),
            crate::diff(&b, &small).expect("diff"),
            crate::diff(&b, &b).expect("diff"),
        ];
        let forecast = forecast_impact(&history, 1);
        assert_eq!(forecast.trajectory, ImpactTrajectory::DeEscalating);
    }

    #[test]
    fn test_scores_clamped() {
        let b = base();
        let mut big = b.clone();
        big.effect = Effect::new(EffectType::Revoke, "Revoked");
        big.discretion_logic = Some("d".into());
        let history: Vec<_> = (0..3)
            .map(|_| crate::diff(&b, &big).expect("diff"))
            .collect();
        let forecast = forecast_impact(&history, 5);
        for p in &forecast.projected_scores {
            assert!(p.score >= 0.0 && p.score <= 100.0);
        }
    }

    #[test]
    fn test_empty_history() {
        let forecast = forecast_impact(&[], 3);
        assert_eq!(forecast.trajectory, ImpactTrajectory::Stable);
        assert_eq!(forecast.historical_mean, 0.0);
        assert_eq!(forecast.projected_scores.len(), 3);
        assert_eq!(forecast.peak_level(), Some(RiskLevel::Negligible));
    }

    #[test]
    fn test_peak_level() {
        let b = base();
        let mut t1 = b.clone();
        t1.title = "Edited".into();
        let mut t2 = b.clone();
        t2.effect = Effect::new(EffectType::Revoke, "Revoked");
        let history = vec![
            crate::diff(&b, &b).expect("diff"),
            crate::diff(&b, &t1).expect("diff"),
            crate::diff(&b, &t2).expect("diff"),
        ];
        let forecast = forecast_impact(&history, 3);
        assert!(forecast.peak_level().is_some());
    }
}
