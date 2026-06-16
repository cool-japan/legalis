//! Structured analytics dashboard data model.
//!
//! This module assembles the outputs of the other analytics sub-modules into a
//! single, serializable **dashboard model** — the *data* behind a dashboard, not
//! a GUI. It is organised as a list of typed [`DashboardWidget`]s (scorecards,
//! gauges, time-series, distributions, tables) that a front-end of any
//! technology can render verbatim. The model serializes to JSON via
//! [`AnalyticsDashboard::to_json`] for transport to such a front-end.
//!
//! The dashboard is intentionally decoupled from rendering: it carries values,
//! labels, units and thresholds, and leaves pixels to the consumer.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::diff;
//! use legalis_diff::analytics::build_dashboard;
//!
//! let base = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
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
//! let dashboard = build_dashboard("law", &history);
//! assert!(!dashboard.widgets.is_empty());
//! let json = dashboard.to_json().unwrap();
//! assert!(json.contains("\"widgets\""));
//! ```

use super::anomaly::{AnomalyConfig, detect_anomalies};
use super::forecast::forecast_impact;
use super::predictive::{forecast_change_volume, series_from_diffs};
use super::risk::assess_risk;
use crate::{DiffError, StatuteDiff};
use serde::{Deserialize, Serialize};

/// A single point in a dashboard time-series widget.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SeriesDatum {
    /// X coordinate (typically a revision index).
    pub x: f64,
    /// Y coordinate (the measured or projected value).
    pub y: f64,
    /// Whether this point is a forecast rather than an observation.
    pub projected: bool,
}

/// A labelled bucket in a distribution widget.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributionBucket {
    /// Bucket label.
    pub label: String,
    /// Bucket count.
    pub count: u64,
}

/// A row in a tabular widget (ordered key/value cells).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableRow {
    /// Ordered cells of the row.
    pub cells: Vec<String>,
}

/// One dashboard widget. Each variant is self-describing so a renderer needs no
/// out-of-band schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DashboardWidget {
    /// A single headline number.
    Scorecard {
        /// Human-readable title.
        title: String,
        /// The value to display.
        value: f64,
        /// Optional unit (e.g. `"changes"`, `"%"`).
        unit: Option<String>,
    },
    /// A bounded gauge with a danger threshold.
    Gauge {
        /// Human-readable title.
        title: String,
        /// Current value.
        value: f64,
        /// Lower bound of the gauge.
        min: f64,
        /// Upper bound of the gauge.
        max: f64,
        /// Value at or above which the gauge is in the danger zone.
        danger_threshold: f64,
    },
    /// A line/area time-series, possibly mixing history and forecast.
    TimeSeries {
        /// Human-readable title.
        title: String,
        /// Ordered data points.
        points: Vec<SeriesDatum>,
    },
    /// A categorical distribution (bar/pie).
    Distribution {
        /// Human-readable title.
        title: String,
        /// Buckets in display order.
        buckets: Vec<DistributionBucket>,
    },
    /// A free-form table.
    Table {
        /// Human-readable title.
        title: String,
        /// Column headers.
        headers: Vec<String>,
        /// Body rows.
        rows: Vec<TableRow>,
    },
}

impl DashboardWidget {
    /// The display title of the widget.
    pub fn title(&self) -> &str {
        match self {
            Self::Scorecard { title, .. }
            | Self::Gauge { title, .. }
            | Self::TimeSeries { title, .. }
            | Self::Distribution { title, .. }
            | Self::Table { title, .. } => title,
        }
    }
}

/// The complete dashboard model for one subject (a statute or a corpus).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsDashboard {
    /// Identifier of the subject this dashboard describes.
    pub subject: String,
    /// Number of diffs the dashboard was built from.
    pub revision_count: usize,
    /// The widgets, in display order.
    pub widgets: Vec<DashboardWidget>,
}

impl AnalyticsDashboard {
    /// Serializes the dashboard to pretty-printed JSON.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if serialization fails.
    pub fn to_json(&self) -> Result<String, DiffError> {
        serde_json::to_string_pretty(self)
            .map_err(|e| DiffError::SerializationError(format!("dashboard JSON: {e}")))
    }

    /// Parses a dashboard from JSON previously produced by [`Self::to_json`].
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if the JSON is malformed.
    pub fn from_json(json: &str) -> Result<Self, DiffError> {
        serde_json::from_str(json)
            .map_err(|e| DiffError::SerializationError(format!("dashboard JSON: {e}")))
    }

    /// Looks up a widget by its title.
    pub fn widget(&self, title: &str) -> Option<&DashboardWidget> {
        self.widgets.iter().find(|w| w.title() == title)
    }
}

/// Builds a full analytics dashboard for a statute's diff history.
///
/// The dashboard combines: a change-volume scorecard, a current-risk gauge, a
/// risk-trajectory time series (history + a short forecast), a change-volume
/// forecast time series, a severity distribution, and an anomaly table.
pub fn build_dashboard(subject: impl Into<String>, history: &[StatuteDiff]) -> AnalyticsDashboard {
    let subject = subject.into();
    let mut widgets = Vec::new();

    // 1. Total change-volume scorecard.
    let total_changes: usize = history.iter().map(|d| d.changes.len()).sum();
    widgets.push(DashboardWidget::Scorecard {
        title: "Total Changes".to_string(),
        value: total_changes as f64,
        unit: Some("changes".to_string()),
    });

    // 2. Current risk gauge (most recent diff, or zero when empty).
    let current_risk = history.last().map(|d| assess_risk(d).score).unwrap_or(0.0);
    widgets.push(DashboardWidget::Gauge {
        title: "Current Risk".to_string(),
        value: current_risk,
        min: 0.0,
        max: 100.0,
        danger_threshold: 60.0,
    });

    // 3. Risk-trajectory time series (historical scores + forecast).
    let impact_forecast = forecast_impact(history, 3);
    let mut risk_points: Vec<SeriesDatum> = history
        .iter()
        .enumerate()
        .map(|(i, d)| SeriesDatum {
            x: i as f64,
            y: assess_risk(d).score,
            projected: false,
        })
        .collect();
    for p in &impact_forecast.projected_scores {
        risk_points.push(SeriesDatum {
            x: p.revision as f64,
            y: p.score,
            projected: true,
        });
    }
    widgets.push(DashboardWidget::TimeSeries {
        title: "Risk Trajectory".to_string(),
        points: risk_points,
    });

    // 4. Change-volume forecast time series.
    let series = series_from_diffs(history);
    let volume_forecast = forecast_change_volume(&series, 3);
    let mut volume_points: Vec<SeriesDatum> = series
        .iter()
        .map(|p| SeriesDatum {
            x: p.revision as f64,
            y: p.change_count,
            projected: false,
        })
        .collect();
    for p in &volume_forecast.projections {
        volume_points.push(SeriesDatum {
            x: p.revision as f64,
            y: p.expected_changes,
            projected: true,
        });
    }
    widgets.push(DashboardWidget::TimeSeries {
        title: "Change Volume Forecast".to_string(),
        points: volume_points,
    });

    // 5. Severity distribution.
    let mut buckets = [
        ("None", 0u64),
        ("Minor", 0),
        ("Moderate", 0),
        ("Major", 0),
        ("Breaking", 0),
    ];
    for d in history {
        let idx = match d.impact.severity {
            crate::Severity::None => 0,
            crate::Severity::Minor => 1,
            crate::Severity::Moderate => 2,
            crate::Severity::Major => 3,
            crate::Severity::Breaking => 4,
        };
        buckets[idx].1 += 1;
    }
    widgets.push(DashboardWidget::Distribution {
        title: "Severity Distribution".to_string(),
        buckets: buckets
            .iter()
            .map(|(label, count)| DistributionBucket {
                label: (*label).to_string(),
                count: *count,
            })
            .collect(),
    });

    // 6. Anomaly table.
    let anomaly_report = detect_anomalies(history, &AnomalyConfig::default());
    let rows: Vec<TableRow> = anomaly_report
        .anomalies
        .iter()
        .map(|a| TableRow {
            cells: vec![
                a.index.to_string(),
                format!("{:.2}", a.value),
                format!("{:.2}", a.score),
            ],
        })
        .collect();
    widgets.push(DashboardWidget::Table {
        title: "Detected Anomalies".to_string(),
        headers: vec![
            "Revision".to_string(),
            "Value".to_string(),
            "Score".to_string(),
        ],
        rows,
    });

    AnalyticsDashboard {
        subject,
        revision_count: history.len(),
        widgets,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    fn sample_history() -> Vec<StatuteDiff> {
        let base = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let mut t1 = base.clone();
        t1.title = "Edited".into();
        let mut t2 = base.clone();
        t2.effect = Effect::new(EffectType::Revoke, "Revoked");
        vec![
            crate::diff(&base, &base).expect("diff"),
            crate::diff(&base, &t1).expect("diff"),
            crate::diff(&base, &t2).expect("diff"),
        ]
    }

    #[test]
    fn test_build_dashboard_has_widgets() {
        let dash = build_dashboard("law", &sample_history());
        assert_eq!(dash.subject, "law");
        assert_eq!(dash.revision_count, 3);
        // Six widgets per the builder.
        assert_eq!(dash.widgets.len(), 6);
    }

    #[test]
    fn test_dashboard_widget_lookup() {
        let dash = build_dashboard("law", &sample_history());
        assert!(dash.widget("Current Risk").is_some());
        assert!(dash.widget("Nonexistent").is_none());
    }

    #[test]
    fn test_dashboard_json_roundtrip() {
        let dash = build_dashboard("law", &sample_history());
        let json = dash.to_json().expect("serialize");
        assert!(json.contains("\"widgets\""));
        let restored = AnalyticsDashboard::from_json(&json).expect("deserialize");
        // Structure must round-trip exactly. Numeric payloads are compared with a
        // tolerance because a JSON float round-trip may differ by a ULP.
        assert_eq!(restored.subject, dash.subject);
        assert_eq!(restored.revision_count, dash.revision_count);
        assert_eq!(restored.widgets.len(), dash.widgets.len());
        for (a, b) in dash.widgets.iter().zip(restored.widgets.iter()) {
            assert!(widgets_approx_eq(a, b), "widget mismatch: {a:?} vs {b:?}");
        }
    }

    /// Compares two widgets for equality, treating f64 fields with a tolerance.
    fn widgets_approx_eq(a: &DashboardWidget, b: &DashboardWidget) -> bool {
        let close = |x: f64, y: f64| (x - y).abs() < 1e-9;
        match (a, b) {
            (
                DashboardWidget::Scorecard {
                    title: t1,
                    value: v1,
                    unit: u1,
                },
                DashboardWidget::Scorecard {
                    title: t2,
                    value: v2,
                    unit: u2,
                },
            ) => t1 == t2 && close(*v1, *v2) && u1 == u2,
            (
                DashboardWidget::Gauge {
                    title: t1,
                    value: v1,
                    min: mn1,
                    max: mx1,
                    danger_threshold: d1,
                },
                DashboardWidget::Gauge {
                    title: t2,
                    value: v2,
                    min: mn2,
                    max: mx2,
                    danger_threshold: d2,
                },
            ) => {
                t1 == t2
                    && close(*v1, *v2)
                    && close(*mn1, *mn2)
                    && close(*mx1, *mx2)
                    && close(*d1, *d2)
            }
            (
                DashboardWidget::TimeSeries {
                    title: t1,
                    points: p1,
                },
                DashboardWidget::TimeSeries {
                    title: t2,
                    points: p2,
                },
            ) => {
                t1 == t2
                    && p1.len() == p2.len()
                    && p1.iter().zip(p2.iter()).all(|(x, y)| {
                        close(x.x, y.x) && close(x.y, y.y) && x.projected == y.projected
                    })
            }
            (
                DashboardWidget::Distribution {
                    title: t1,
                    buckets: b1,
                },
                DashboardWidget::Distribution {
                    title: t2,
                    buckets: b2,
                },
            ) => t1 == t2 && b1 == b2,
            (
                DashboardWidget::Table {
                    title: t1,
                    headers: h1,
                    rows: r1,
                },
                DashboardWidget::Table {
                    title: t2,
                    headers: h2,
                    rows: r2,
                },
            ) => t1 == t2 && h1 == h2 && r1 == r2,
            _ => false,
        }
    }

    #[test]
    fn test_empty_history_dashboard() {
        let dash = build_dashboard("empty", &[]);
        assert_eq!(dash.revision_count, 0);
        // Still produces the full widget set, just with empty data.
        assert_eq!(dash.widgets.len(), 6);
        let gauge = dash.widget("Current Risk").expect("gauge");
        if let DashboardWidget::Gauge { value, .. } = gauge {
            assert_eq!(*value, 0.0);
        } else {
            panic!("expected gauge");
        }
    }

    #[test]
    fn test_scorecard_total_changes() {
        let history = sample_history();
        let expected: usize = history.iter().map(|d| d.changes.len()).sum();
        let dash = build_dashboard("law", &history);
        let card = dash.widget("Total Changes").expect("card");
        if let DashboardWidget::Scorecard { value, .. } = card {
            assert_eq!(*value, expected as f64);
        } else {
            panic!("expected scorecard");
        }
    }

    #[test]
    fn test_severity_distribution_counts() {
        let dash = build_dashboard("law", &sample_history());
        let dist = dash.widget("Severity Distribution").expect("dist");
        if let DashboardWidget::Distribution { buckets, .. } = dist {
            let total: u64 = buckets.iter().map(|b| b.count).sum();
            assert_eq!(total, 3);
        } else {
            panic!("expected distribution");
        }
    }

    #[test]
    fn test_timeseries_has_projected_points() {
        let dash = build_dashboard("law", &sample_history());
        let ts = dash.widget("Risk Trajectory").expect("ts");
        if let DashboardWidget::TimeSeries { points, .. } = ts {
            assert!(points.iter().any(|p| p.projected));
            assert!(points.iter().any(|p| !p.projected));
        } else {
            panic!("expected time series");
        }
    }

    #[test]
    fn test_from_json_rejects_garbage() {
        assert!(AnalyticsDashboard::from_json("{not json").is_err());
    }
}
