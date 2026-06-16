//! Advanced analytics & insights for statute diffs (v0.5.8).
//!
//! This module mines a statute's revision history — a sequence of
//! [`crate::StatuteDiff`]s — for forward-looking signals. It is organised into
//! five pure-Rust, self-contained sub-modules:
//!
//! - [`predictive`] — **predictive analytics for future changes**: ordinary
//!   least-squares trend extrapolation over the change-volume history
//!   ([`predictive::forecast_change_volume`]).
//! - [`anomaly`] — **anomaly detection in diff patterns**: robust statistical
//!   outlier detection (z-score and MAD-based modified z-score) over
//!   configurable change metrics ([`anomaly::detect_anomalies`]).
//! - [`forecast`] — **change-impact forecasting**: projects the per-revision
//!   risk score forward and classifies the risk trajectory
//!   ([`forecast::forecast_impact`]).
//! - [`risk`] — **risk-assessment automation**: an explainable, factor-weighted
//!   risk score for a single diff ([`risk::assess_risk`]).
//! - [`dashboard`] — **custom analytics dashboard data**: a serializable,
//!   render-agnostic dashboard model ([`dashboard::build_dashboard`]) with JSON
//!   export.
//!
//! All analyses are deterministic and depend only on stable `legalis-core` and
//! the diff types defined in this crate. There is no GUI here — the dashboard
//! sub-module produces the *data* a front-end would render.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//! use legalis_diff::diff;
//! use legalis_diff::analytics::{assess_risk, build_dashboard, detect_anomalies, AnomalyConfig};
//!
//! let base = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
//! let mut revised = base.clone();
//! revised.effect = Effect::new(EffectType::Revoke, "Revoked");
//! revised.discretion_logic = Some("officer review".into());
//! revised = revised.with_precondition(Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 65,
//! });
//! let d = diff(&base, &revised).unwrap();
//!
//! // Score a single diff: altered outcome + eligibility + discretion → high risk.
//! let risk = assess_risk(&d);
//! assert!(risk.requires_review());
//!
//! // Aggregate a history into a dashboard.
//! let history = vec![diff(&base, &base).unwrap(), d];
//! let dashboard = build_dashboard("law", &history);
//! assert!(!dashboard.widgets.is_empty());
//!
//! // Look for outliers (needs >= 3 points to estimate dispersion).
//! let report = detect_anomalies(&history, &AnomalyConfig::default());
//! assert_eq!(report.population_size, 2);
//! ```

pub mod anomaly;
pub mod dashboard;
pub mod forecast;
pub mod predictive;
pub mod risk;

pub use anomaly::{
    Anomaly, AnomalyConfig, AnomalyMethod, AnomalyMetric, AnomalyReport, detect_anomalies,
};
pub use dashboard::{
    AnalyticsDashboard, DashboardWidget, DistributionBucket, SeriesDatum, TableRow, build_dashboard,
};
pub use forecast::{ImpactForecast, ImpactProjection, ImpactTrajectory, forecast_impact};
pub use predictive::{
    ChangeForecast, ChangeProjection, ChangeSeriesPoint, LinearModel, fit_linear_model,
    forecast_change_volume, series_from_diffs,
};
pub use risk::{RiskAssessment, RiskFactor, RiskLevel, assess_risk};
