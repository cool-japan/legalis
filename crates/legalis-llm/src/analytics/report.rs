//! Custom report generation: a composable analytics report builder.
//!
//! [`ReportBuilder`] assembles a [`Report`] from typed [`ReportBlock`]s -
//! headings, paragraphs, key/value metric lists, tables, ordered/unordered
//! lists and nested sections - and renders the result to Markdown or plain
//! text. It is purely structural: no LLM is involved, so reports are
//! deterministic and reproducible. Convenience constructors fold the other
//! analytics outputs ([`TrendReport`], [`JurisdictionComparison`],
//! [`RiskHeatmap`], [`PatternReport`]) straight into report blocks.

use super::{JurisdictionComparison, PatternReport, RiskHeatmap, TrendDirection, TrendReport};
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;

/// A single composable content block within a report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReportBlock {
    /// A heading at the given level (`1..=6`).
    Heading { level: u8, text: String },
    /// A free-text paragraph.
    Paragraph(String),
    /// A list of key/value metric pairs.
    Metrics(Vec<(String, String)>),
    /// A table with a header row and body rows.
    Table {
        /// Column headers.
        headers: Vec<String>,
        /// Body rows (each row should match the header arity).
        rows: Vec<Vec<String>>,
    },
    /// A bulleted list.
    BulletList(Vec<String>),
    /// A numbered list.
    NumberedList(Vec<String>),
    /// A nested, titled sub-section with its own blocks.
    Section {
        /// Section title.
        title: String,
        /// Section body blocks.
        blocks: Vec<ReportBlock>,
    },
    /// A verbatim code/preformatted block (rendered fenced in Markdown).
    Preformatted(String),
}

impl ReportBlock {
    fn render_markdown(&self, out: &mut String, base_level: u8) {
        match self {
            ReportBlock::Heading { level, text } => {
                let lvl = (*level).clamp(1, 6) as usize;
                let _ = writeln!(out, "{} {}\n", "#".repeat(lvl), text);
            }
            ReportBlock::Paragraph(text) => {
                let _ = writeln!(out, "{text}\n");
            }
            ReportBlock::Metrics(pairs) => {
                for (key, value) in pairs {
                    let _ = writeln!(out, "- **{key}**: {value}");
                }
                out.push('\n');
            }
            ReportBlock::Table { headers, rows } => {
                let _ = writeln!(out, "| {} |", headers.join(" | "));
                let _ = writeln!(
                    out,
                    "|{}",
                    headers.iter().map(|_| "---|").collect::<String>()
                );
                for row in rows {
                    let _ = writeln!(out, "| {} |", row.join(" | "));
                }
                out.push('\n');
            }
            ReportBlock::BulletList(items) => {
                for item in items {
                    let _ = writeln!(out, "- {item}");
                }
                out.push('\n');
            }
            ReportBlock::NumberedList(items) => {
                for (i, item) in items.iter().enumerate() {
                    let _ = writeln!(out, "{}. {item}", i + 1);
                }
                out.push('\n');
            }
            ReportBlock::Section { title, blocks } => {
                let lvl = (base_level + 1).clamp(1, 6) as usize;
                let _ = writeln!(out, "{} {}\n", "#".repeat(lvl), title);
                for block in blocks {
                    block.render_markdown(out, base_level + 1);
                }
            }
            ReportBlock::Preformatted(text) => {
                let _ = writeln!(out, "```\n{text}\n```\n");
            }
        }
    }

    fn render_text(&self, out: &mut String, indent: usize) {
        let pad = "  ".repeat(indent);
        match self {
            ReportBlock::Heading { level, text } => {
                let underline = if *level <= 1 { '=' } else { '-' };
                let _ = writeln!(out, "{pad}{text}");
                let _ = writeln!(out, "{pad}{}\n", underline.to_string().repeat(text.len()));
            }
            ReportBlock::Paragraph(text) => {
                let _ = writeln!(out, "{pad}{text}\n");
            }
            ReportBlock::Metrics(pairs) => {
                for (key, value) in pairs {
                    let _ = writeln!(out, "{pad}{key}: {value}");
                }
                out.push('\n');
            }
            ReportBlock::Table { headers, rows } => {
                let _ = writeln!(out, "{pad}{}", headers.join(" | "));
                for row in rows {
                    let _ = writeln!(out, "{pad}{}", row.join(" | "));
                }
                out.push('\n');
            }
            ReportBlock::BulletList(items) => {
                for item in items {
                    let _ = writeln!(out, "{pad}* {item}");
                }
                out.push('\n');
            }
            ReportBlock::NumberedList(items) => {
                for (i, item) in items.iter().enumerate() {
                    let _ = writeln!(out, "{pad}{}. {item}", i + 1);
                }
                out.push('\n');
            }
            ReportBlock::Section { title, blocks } => {
                let _ = writeln!(out, "{pad}{title}");
                let _ = writeln!(out, "{pad}{}\n", "-".repeat(title.len()));
                for block in blocks {
                    block.render_text(out, indent + 1);
                }
            }
            ReportBlock::Preformatted(text) => {
                for line in text.lines() {
                    let _ = writeln!(out, "{pad}    {line}");
                }
                out.push('\n');
            }
        }
    }
}

/// A complete, renderable report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Report {
    /// Report title.
    pub title: String,
    /// Optional subtitle / context line.
    pub subtitle: Option<String>,
    /// Ordered content blocks.
    pub blocks: Vec<ReportBlock>,
}

impl Report {
    /// Renders the report to Markdown.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "# {}\n", self.title);
        if let Some(subtitle) = &self.subtitle {
            let _ = writeln!(out, "_{subtitle}_\n");
        }
        for block in &self.blocks {
            block.render_markdown(&mut out, 1);
        }
        out.trim_end().to_string() + "\n"
    }

    /// Renders the report to plain text.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "{}", self.title);
        let _ = writeln!(out, "{}\n", "=".repeat(self.title.len()));
        if let Some(subtitle) = &self.subtitle {
            let _ = writeln!(out, "{subtitle}\n");
        }
        for block in &self.blocks {
            block.render_text(&mut out, 0);
        }
        out.trim_end().to_string() + "\n"
    }

    /// Counts the total number of blocks, descending into sections.
    pub fn block_count(&self) -> usize {
        fn count(blocks: &[ReportBlock]) -> usize {
            blocks
                .iter()
                .map(|block| match block {
                    ReportBlock::Section { blocks, .. } => 1 + count(blocks),
                    _ => 1,
                })
                .sum()
        }
        count(&self.blocks)
    }
}

/// A fluent builder for composing a [`Report`].
#[derive(Debug, Clone)]
pub struct ReportBuilder {
    title: String,
    subtitle: Option<String>,
    blocks: Vec<ReportBlock>,
}

impl ReportBuilder {
    /// Starts a new report with the given title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            blocks: Vec::new(),
        }
    }

    /// Sets the subtitle.
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Appends an arbitrary block.
    pub fn block(mut self, block: ReportBlock) -> Self {
        self.blocks.push(block);
        self
    }

    /// Appends a heading.
    pub fn heading(self, level: u8, text: impl Into<String>) -> Self {
        self.block(ReportBlock::Heading {
            level,
            text: text.into(),
        })
    }

    /// Appends a paragraph.
    pub fn paragraph(self, text: impl Into<String>) -> Self {
        self.block(ReportBlock::Paragraph(text.into()))
    }

    /// Appends a metric list.
    pub fn metrics(self, pairs: Vec<(String, String)>) -> Self {
        self.block(ReportBlock::Metrics(pairs))
    }

    /// Appends a table.
    pub fn table(self, headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        self.block(ReportBlock::Table { headers, rows })
    }

    /// Appends a bullet list.
    pub fn bullets(self, items: Vec<String>) -> Self {
        self.block(ReportBlock::BulletList(items))
    }

    /// Appends a numbered list.
    pub fn numbered(self, items: Vec<String>) -> Self {
        self.block(ReportBlock::NumberedList(items))
    }

    /// Appends a nested section.
    pub fn section(self, title: impl Into<String>, blocks: Vec<ReportBlock>) -> Self {
        self.block(ReportBlock::Section {
            title: title.into(),
            blocks,
        })
    }

    /// Folds a [`TrendReport`] into a titled section.
    pub fn trend_section(self, title: impl Into<String>, report: &TrendReport) -> Self {
        let direction = match report.direction {
            TrendDirection::Increasing => "increasing",
            TrendDirection::Decreasing => "decreasing",
            TrendDirection::Stable => "stable (no significant trend)",
        };
        let metrics = vec![
            ("Direction".to_string(), direction.to_string()),
            (
                "OLS slope / bucket".to_string(),
                format!("{:.4}", report.linear_fit.slope),
            ),
            (
                "R-squared".to_string(),
                format!("{:.4}", report.linear_fit.r_squared),
            ),
            (
                "Sen's slope".to_string(),
                format!("{:.4}", report.sen_slope),
            ),
            (
                "Mann-Kendall tau".to_string(),
                format!("{:.4}", report.mann_kendall.tau),
            ),
            (
                "Mann-Kendall p-value".to_string(),
                format!("{:.4}", report.mann_kendall.p_value),
            ),
            (
                "Change first->last".to_string(),
                format!("{:.1}%", report.percent_change),
            ),
            (
                "Mean bucket value".to_string(),
                format!("{:.2}", report.mean_value),
            ),
        ];
        let rows: Vec<Vec<String>> = report
            .series
            .iter()
            .map(|point| {
                vec![
                    point.bucket.clone(),
                    format!("{:.2}", point.value),
                    point.count.to_string(),
                ]
            })
            .collect();
        self.section(
            title,
            vec![
                ReportBlock::Metrics(metrics),
                ReportBlock::Table {
                    headers: vec![
                        "Bucket".to_string(),
                        "Value".to_string(),
                        "Count".to_string(),
                    ],
                    rows,
                },
            ],
        )
    }

    /// Folds a [`JurisdictionComparison`] into a titled section.
    pub fn jurisdiction_section(
        self,
        title: impl Into<String>,
        comparison: &JurisdictionComparison,
    ) -> Self {
        let rows: Vec<Vec<String>> = comparison
            .ranking
            .iter()
            .map(|entry| {
                vec![
                    entry.rank.to_string(),
                    entry.jurisdiction.description(),
                    format!("{:.2}", entry.value),
                ]
            })
            .collect();
        let metrics = vec![
            (
                "Spread (max - min)".to_string(),
                format!("{:.2}", comparison.metric_max - comparison.metric_min),
            ),
            (
                "Coefficient of variation".to_string(),
                format!("{:.4}", comparison.coefficient_of_variation),
            ),
            ("Gini".to_string(), format!("{:.4}", comparison.gini)),
            (
                "HHI (activity)".to_string(),
                format!("{:.4}", comparison.hhi),
            ),
        ];
        self.section(
            title,
            vec![
                ReportBlock::Metrics(metrics),
                ReportBlock::Table {
                    headers: vec![
                        "Rank".to_string(),
                        "Jurisdiction".to_string(),
                        "Metric".to_string(),
                    ],
                    rows,
                },
            ],
        )
    }

    /// Folds a [`RiskHeatmap`] into a titled section (grid + register table).
    pub fn risk_section(self, title: impl Into<String>, heatmap: &RiskHeatmap) -> Self {
        let register_rows: Vec<Vec<String>> = heatmap
            .risks
            .iter()
            .map(|scored| {
                vec![
                    scored.item.name.clone(),
                    scored
                        .item
                        .category
                        .clone()
                        .unwrap_or_else(|| "-".to_string()),
                    scored.item.likelihood.to_string(),
                    scored.item.impact.to_string(),
                    scored.raw_score.to_string(),
                    scored.severity.label().to_string(),
                ]
            })
            .collect();
        self.section(
            title,
            vec![
                ReportBlock::Metrics(vec![(
                    "Mean normalised risk".to_string(),
                    format!("{:.4}", heatmap.mean_normalized_score()),
                )]),
                ReportBlock::Preformatted(heatmap.to_markdown_grid()),
                ReportBlock::Table {
                    headers: vec![
                        "Risk".to_string(),
                        "Category".to_string(),
                        "Likelihood".to_string(),
                        "Impact".to_string(),
                        "Score".to_string(),
                        "Severity".to_string(),
                    ],
                    rows: register_rows,
                },
            ],
        )
    }

    /// Folds a [`PatternReport`] into a titled section (top associations).
    pub fn pattern_section(self, title: impl Into<String>, report: &PatternReport) -> Self {
        let rows: Vec<Vec<String>> = report
            .associations
            .iter()
            .take(10)
            .map(|assoc| {
                vec![
                    assoc.value.clone(),
                    assoc.outcome.clone(),
                    format!("{:.2}", assoc.confidence),
                    format!("{:.2}", assoc.lift),
                    assoc.count.to_string(),
                ]
            })
            .collect();
        self.section(
            title,
            vec![
                ReportBlock::Paragraph(format!(
                    "Baseline outcome entropy: {:.3} bits over {} observations grouped by `{}`.",
                    report.baseline.entropy_bits, report.baseline.total, report.attribute
                )),
                ReportBlock::Table {
                    headers: vec![
                        "Value".to_string(),
                        "Outcome".to_string(),
                        "Confidence".to_string(),
                        "Lift".to_string(),
                        "Count".to_string(),
                    ],
                    rows,
                },
            ],
        )
    }

    /// Finalises the report.
    pub fn build(self) -> Report {
        Report {
            title: self.title,
            subtitle: self.subtitle,
            blocks: self.blocks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Jurisdiction;
    use crate::analytics::{
        Aggregation, AnalyticsGranularity, ComparisonMetric, JurisdictionComparator, LegalEvent,
        PatternAnalyzer, RiskHeatmap, RiskItem, TrendAnalyzer,
    };
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_basic_report_markdown_and_text() {
        let report = ReportBuilder::new("Quarterly Legal Review")
            .subtitle("Prepared 2026-06-14")
            .heading(2, "Overview")
            .paragraph("This report summarises key legal metrics.")
            .metrics(vec![
                ("Cases".to_string(), "42".to_string()),
                ("Avg award".to_string(), "$120k".to_string()),
            ])
            .bullets(vec!["Point one".to_string(), "Point two".to_string()])
            .numbered(vec!["First".to_string(), "Second".to_string()])
            .table(
                vec!["A".to_string(), "B".to_string()],
                vec![vec!["1".to_string(), "2".to_string()]],
            )
            .build();

        let md = report.to_markdown();
        assert!(md.contains("# Quarterly Legal Review"));
        assert!(md.contains("_Prepared 2026-06-14_"));
        assert!(md.contains("## Overview"));
        assert!(md.contains("- **Cases**: 42"));
        assert!(md.contains("- Point one"));
        assert!(md.contains("1. First"));
        assert!(md.contains("| A | B |"));

        let text = report.to_text();
        assert!(text.contains("Quarterly Legal Review"));
        assert!(text.contains("Cases: 42"));
        assert!(text.contains("* Point one"));
    }

    #[test]
    fn test_nested_section_levels() {
        let report = ReportBuilder::new("Root")
            .section(
                "Level A",
                vec![ReportBlock::Section {
                    title: "Level B".to_string(),
                    blocks: vec![ReportBlock::Paragraph("deep".to_string())],
                }],
            )
            .build();
        let md = report.to_markdown();
        // Root section -> ## Level A, nested -> ### Level B.
        assert!(md.contains("## Level A"));
        assert!(md.contains("### Level B"));
        assert!(md.contains("deep"));
        assert_eq!(report.block_count(), 3); // Level A + Level B + paragraph
    }

    #[test]
    fn test_trend_section_integration() {
        let analyzer = TrendAnalyzer::new(AnalyticsGranularity::Monthly, Aggregation::Sum);
        let events: Vec<LegalEvent> = (1..=6)
            .map(|m| {
                let ts = Utc
                    .with_ymd_and_hms(2024, m, 1, 0, 0, 0)
                    .single()
                    .expect("valid");
                LegalEvent::new(format!("e{m}"), ts, "award").with_value(100.0 * m as f64)
            })
            .collect();
        let trend = analyzer.analyze(&events);
        let report = ReportBuilder::new("Trend")
            .trend_section("Award Trend", &trend)
            .build();
        let md = report.to_markdown();
        assert!(md.contains("## Award Trend"));
        assert!(md.contains("Direction"));
        assert!(md.contains("Bucket"));
    }

    #[test]
    fn test_jurisdiction_and_risk_and_pattern_sections() {
        let ts = Utc
            .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
            .single()
            .expect("valid");
        let events = vec![
            LegalEvent::new("a", ts, "tort")
                .with_value(100.0)
                .with_jurisdiction(Jurisdiction::UsFederal)
                .with_attribute("judge", "Smith"),
            LegalEvent::new("b", ts, "tort")
                .with_value(50.0)
                .with_jurisdiction(Jurisdiction::Uk)
                .with_attribute("judge", "Jones"),
        ];
        let comparison = JurisdictionComparator::new().compare(&events, ComparisonMetric::Sum);
        let heatmap = RiskHeatmap::from_items(5, vec![RiskItem::new("r", "Risk", 4, 4)]);
        let pattern = PatternAnalyzer::new().analyze(&events, "judge");

        let report = ReportBuilder::new("Combined")
            .jurisdiction_section("By Jurisdiction", &comparison)
            .risk_section("Risk Matrix", &heatmap)
            .pattern_section("Judge Patterns", &pattern)
            .build();
        let md = report.to_markdown();
        assert!(md.contains("## By Jurisdiction"));
        assert!(md.contains("## Risk Matrix"));
        assert!(md.contains("## Judge Patterns"));
        assert!(md.contains("United States federal law"));
        assert!(md.contains("```")); // preformatted grid
    }
}
