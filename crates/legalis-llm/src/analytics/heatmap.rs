//! Risk heatmaps: structured likelihood x impact risk matrices.
//!
//! [`RiskHeatmap`] is a structured (non-GUI) representation of a classic risk
//! matrix. Each [`RiskItem`] carries a likelihood and impact on a configurable
//! ordinal scale; the heatmap derives a composite *risk score* and a categorical
//! [`RiskSeverity`], bins items into the cells of the matrix, and exports the
//! whole thing to CSV, a Markdown grid, or a flat Markdown table for inclusion
//! in a report. No rendering library is used - the output is plain text /
//! structured data that any front-end can visualise.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// Discrete severity band derived from a risk score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskSeverity {
    /// Lowest band.
    Low,
    /// Below the midpoint.
    Moderate,
    /// Above the midpoint.
    High,
    /// Highest band.
    Critical,
}

impl RiskSeverity {
    /// A short human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            RiskSeverity::Low => "Low",
            RiskSeverity::Moderate => "Moderate",
            RiskSeverity::High => "High",
            RiskSeverity::Critical => "Critical",
        }
    }

    /// A single-character symbol useful for compact text grids.
    pub fn symbol(&self) -> char {
        match self {
            RiskSeverity::Low => 'L',
            RiskSeverity::Moderate => 'M',
            RiskSeverity::High => 'H',
            RiskSeverity::Critical => 'C',
        }
    }
}

/// A single risk under assessment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskItem {
    /// Stable identifier.
    pub id: String,
    /// Human-readable risk name / description.
    pub name: String,
    /// Likelihood rating on `1..=scale` (higher = more likely).
    pub likelihood: u8,
    /// Impact rating on `1..=scale` (higher = more severe).
    pub impact: u8,
    /// Optional category for grouping (e.g. "Litigation", "Regulatory").
    pub category: Option<String>,
}

impl RiskItem {
    /// Creates a new risk item; likelihood and impact are caller-supplied on the
    /// matrix scale (validated when added to a [`RiskHeatmap`]).
    pub fn new(id: impl Into<String>, name: impl Into<String>, likelihood: u8, impact: u8) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            likelihood,
            impact,
            category: None,
        }
    }

    /// Sets the category.
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }
}

/// A scored risk item (item plus its derived score and severity).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoredRisk {
    /// The underlying item.
    pub item: RiskItem,
    /// Raw product score (`likelihood * impact`).
    pub raw_score: u32,
    /// Normalised score in `[0, 1]`.
    pub normalized_score: f64,
    /// Derived severity band.
    pub severity: RiskSeverity,
}

/// A single occupied cell of the matrix.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeatmapCell {
    /// Likelihood coordinate (`1..=scale`).
    pub likelihood: u8,
    /// Impact coordinate (`1..=scale`).
    pub impact: u8,
    /// Severity of this cell.
    pub severity: RiskSeverity,
    /// Ids of the risks falling in this cell.
    pub item_ids: Vec<String>,
}

/// A structured likelihood x impact risk matrix.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskHeatmap {
    /// Maximum rating on each axis (matrix is `scale x scale`).
    pub scale: u8,
    /// The scored risks, ordered by descending raw score.
    pub risks: Vec<ScoredRisk>,
}

impl RiskHeatmap {
    /// Creates an empty heatmap with the given axis scale (clamped to `>= 2`).
    pub fn new(scale: u8) -> Self {
        Self {
            scale: scale.max(2),
            risks: Vec::new(),
        }
    }

    /// Builds a heatmap from a set of items on a given scale.
    ///
    /// Likelihood and impact ratings are clamped into `1..=scale`. Risks are
    /// stored ordered by descending raw score (ties broken by id).
    pub fn from_items(scale: u8, items: impl IntoIterator<Item = RiskItem>) -> Self {
        let mut heatmap = Self::new(scale);
        for item in items {
            heatmap.add(item);
        }
        heatmap
    }

    /// Adds an item, scoring it and keeping the risk list sorted.
    pub fn add(&mut self, mut item: RiskItem) {
        item.likelihood = item.likelihood.clamp(1, self.scale);
        item.impact = item.impact.clamp(1, self.scale);
        let raw_score = item.likelihood as u32 * item.impact as u32;
        let max_score = self.scale as u32 * self.scale as u32;
        let normalized_score = raw_score as f64 / max_score as f64;
        let severity = severity_from_normalized(normalized_score);
        self.risks.push(ScoredRisk {
            item,
            raw_score,
            normalized_score,
            severity,
        });
        self.risks.sort_by(|a, b| {
            b.raw_score
                .cmp(&a.raw_score)
                .then_with(|| a.item.id.cmp(&b.item.id))
        });
    }

    /// Number of risks in the heatmap.
    pub fn len(&self) -> usize {
        self.risks.len()
    }

    /// Whether the heatmap has no risks.
    pub fn is_empty(&self) -> bool {
        self.risks.is_empty()
    }

    /// Returns the occupied cells of the matrix.
    pub fn cells(&self) -> Vec<HeatmapCell> {
        let mut grouped: BTreeMap<(u8, u8), Vec<String>> = BTreeMap::new();
        for scored in &self.risks {
            grouped
                .entry((scored.item.likelihood, scored.item.impact))
                .or_default()
                .push(scored.item.id.clone());
        }
        grouped
            .into_iter()
            .map(|((likelihood, impact), item_ids)| {
                let max_score = self.scale as u32 * self.scale as u32;
                let normalized = (likelihood as u32 * impact as u32) as f64 / max_score as f64;
                HeatmapCell {
                    likelihood,
                    impact,
                    severity: severity_from_normalized(normalized),
                    item_ids,
                }
            })
            .collect()
    }

    /// Counts risks in each severity band.
    pub fn severity_counts(&self) -> BTreeMap<RiskSeverity, usize> {
        let mut counts = BTreeMap::new();
        for scored in &self.risks {
            *counts.entry(scored.severity).or_insert(0) += 1;
        }
        counts
    }

    /// The risks with the highest raw score (up to `n`).
    pub fn top_risks(&self, n: usize) -> &[ScoredRisk] {
        let end = n.min(self.risks.len());
        &self.risks[..end]
    }

    /// Mean normalised score across all risks (an aggregate risk indicator).
    pub fn mean_normalized_score(&self) -> f64 {
        if self.risks.is_empty() {
            return 0.0;
        }
        self.risks.iter().map(|r| r.normalized_score).sum::<f64>() / self.risks.len() as f64
    }

    /// Exports the risk register as CSV (one row per risk).
    ///
    /// Fields are comma-separated with double-quote escaping; the header is
    /// `id,name,category,likelihood,impact,raw_score,normalized_score,severity`.
    pub fn to_csv(&self) -> String {
        let mut out = String::from(
            "id,name,category,likelihood,impact,raw_score,normalized_score,severity\n",
        );
        for scored in &self.risks {
            let category = scored.item.category.as_deref().unwrap_or("");
            let _ = writeln!(
                out,
                "{},{},{},{},{},{},{:.4},{}",
                csv_escape(&scored.item.id),
                csv_escape(&scored.item.name),
                csv_escape(category),
                scored.item.likelihood,
                scored.item.impact,
                scored.raw_score,
                scored.normalized_score,
                scored.severity.label(),
            );
        }
        out
    }

    /// Renders the matrix as a Markdown grid of severity symbols.
    ///
    /// Rows are likelihood (high at top), columns are impact (low at left); each
    /// cell shows the number of risks and the dominant severity symbol.
    pub fn to_markdown_grid(&self) -> String {
        let cells = self.cells();
        let mut by_coord: BTreeMap<(u8, u8), (usize, RiskSeverity)> = BTreeMap::new();
        for cell in &cells {
            by_coord.insert(
                (cell.likelihood, cell.impact),
                (cell.item_ids.len(), cell.severity),
            );
        }

        let mut out = String::new();
        // Header row: impact axis.
        let _ = write!(out, "| L \\ I |");
        for impact in 1..=self.scale {
            let _ = write!(out, " {impact} |");
        }
        out.push('\n');
        let _ = write!(out, "|---|");
        for _ in 1..=self.scale {
            out.push_str("---|");
        }
        out.push('\n');
        // Likelihood rows, highest first.
        for likelihood in (1..=self.scale).rev() {
            let _ = write!(out, "| {likelihood} |");
            for impact in 1..=self.scale {
                match by_coord.get(&(likelihood, impact)) {
                    Some((count, severity)) => {
                        let _ = write!(out, " {}{} |", severity.symbol(), count);
                    }
                    None => out.push_str(" . |"),
                }
            }
            out.push('\n');
        }
        out
    }

    /// Renders the scored risk register as a flat Markdown table.
    pub fn to_markdown_table(&self) -> String {
        let mut out = String::from(
            "| Risk | Category | Likelihood | Impact | Score | Severity |\n\
             |------|----------|-----------:|-------:|------:|----------|\n",
        );
        for scored in &self.risks {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} | {} | {} |",
                scored.item.name,
                scored.item.category.as_deref().unwrap_or("-"),
                scored.item.likelihood,
                scored.item.impact,
                scored.raw_score,
                scored.severity.label(),
            );
        }
        out
    }
}

/// Maps a normalised `[0, 1]` score to a severity band.
fn severity_from_normalized(normalized: f64) -> RiskSeverity {
    if normalized >= 0.75 {
        RiskSeverity::Critical
    } else if normalized >= 0.5 {
        RiskSeverity::High
    } else if normalized >= 0.25 {
        RiskSeverity::Moderate
    } else {
        RiskSeverity::Low
    }
}

/// Escapes a CSV field, quoting it when it contains a comma, quote or newline.
fn csv_escape(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RiskHeatmap {
        RiskHeatmap::from_items(
            5,
            vec![
                RiskItem::new("r1", "Adverse precedent", 5, 5).with_category("Litigation"),
                RiskItem::new("r2", "Minor filing delay", 2, 1).with_category("Process"),
                RiskItem::new("r3", "Regulatory fine", 3, 4).with_category("Regulatory"),
                RiskItem::new("r4", "Data breach", 2, 5).with_category("Security"),
            ],
        )
    }

    #[test]
    fn test_scoring_and_severity() {
        let heatmap = sample();
        assert_eq!(heatmap.len(), 4);
        // Highest score first: r1 with 25.
        assert_eq!(heatmap.risks[0].item.id, "r1");
        assert_eq!(heatmap.risks[0].raw_score, 25);
        assert!((heatmap.risks[0].normalized_score - 1.0).abs() < 1e-9);
        assert_eq!(heatmap.risks[0].severity, RiskSeverity::Critical);
        // r2: 2*1 = 2 => normalized 0.08 => Low.
        let r2 = heatmap.risks.iter().find(|r| r.item.id == "r2").unwrap();
        assert_eq!(r2.severity, RiskSeverity::Low);
    }

    #[test]
    fn test_rating_clamping() {
        let mut heatmap = RiskHeatmap::new(3);
        heatmap.add(RiskItem::new("x", "Overflow", 9, 0));
        let scored = &heatmap.risks[0];
        assert_eq!(scored.item.likelihood, 3);
        assert_eq!(scored.item.impact, 1);
    }

    #[test]
    fn test_cells_grouping() {
        let mut heatmap = RiskHeatmap::new(5);
        heatmap.add(RiskItem::new("a", "A", 3, 3));
        heatmap.add(RiskItem::new("b", "B", 3, 3));
        heatmap.add(RiskItem::new("c", "C", 1, 1));
        let cells = heatmap.cells();
        let shared = cells
            .iter()
            .find(|c| c.likelihood == 3 && c.impact == 3)
            .expect("cell");
        assert_eq!(shared.item_ids.len(), 2);
        assert!(shared.item_ids.contains(&"a".to_string()));
        assert!(shared.item_ids.contains(&"b".to_string()));
    }

    #[test]
    fn test_severity_counts_and_top() {
        let heatmap = sample();
        let counts = heatmap.severity_counts();
        assert_eq!(counts.get(&RiskSeverity::Critical).copied().unwrap_or(0), 1);
        let top = heatmap.top_risks(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].item.id, "r1");
        assert!(heatmap.mean_normalized_score() > 0.0);
    }

    #[test]
    fn test_csv_export() {
        let heatmap = sample();
        let csv = heatmap.to_csv();
        assert!(csv.starts_with(
            "id,name,category,likelihood,impact,raw_score,normalized_score,severity\n"
        ));
        assert!(csv.contains("r1,Adverse precedent,Litigation,5,5,25,1.0000,Critical"));
        // Exactly header + 4 rows.
        assert_eq!(csv.lines().count(), 5);
    }

    #[test]
    fn test_csv_escaping() {
        let mut heatmap = RiskHeatmap::new(5);
        heatmap.add(RiskItem::new("q", "Risk, with \"quote\"", 2, 2));
        let csv = heatmap.to_csv();
        assert!(csv.contains("\"Risk, with \"\"quote\"\"\""));
    }

    #[test]
    fn test_markdown_grid_and_table() {
        let heatmap = sample();
        let grid = heatmap.to_markdown_grid();
        assert!(grid.contains("| L \\ I |"));
        // Top-right corner (likelihood 5, impact 5) has the critical risk.
        assert!(grid.contains("C1"));
        let table = heatmap.to_markdown_table();
        assert!(table.contains("| Risk | Category |"));
        assert!(table.contains("Adverse precedent"));
        assert!(table.contains("Critical"));
    }

    #[test]
    fn test_severity_ordering() {
        assert!(RiskSeverity::Critical > RiskSeverity::High);
        assert!(RiskSeverity::High > RiskSeverity::Moderate);
        assert!(RiskSeverity::Moderate > RiskSeverity::Low);
        assert_eq!(RiskSeverity::Critical.symbol(), 'C');
    }
}
