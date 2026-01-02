//! Reasoning Transparency (v0.2.7)
//!
//! This module provides tools for understanding and visualizing the reasoning
//! process of LLM operations, including chain-of-thought logging, confidence
//! calibration, uncertainty quantification, and decision audit trails.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================================
// Chain-of-Thought Logging
// ============================================================================

/// A single step in a chain of thought reasoning process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtStep {
    /// Step number in the chain
    pub step_number: usize,
    /// Description of what this step does
    pub description: String,
    /// The reasoning or thought process
    pub thought: String,
    /// Intermediate result (if any)
    pub intermediate_result: Option<String>,
    /// Confidence in this step (0.0 - 1.0)
    pub confidence: f64,
    /// Time taken for this step
    pub duration_ms: u128,
    /// Metadata for this step
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ThoughtStep {
    /// Creates a new thought step.
    pub fn new(step_number: usize, description: impl Into<String>) -> Self {
        Self {
            step_number,
            description: description.into(),
            thought: String::new(),
            intermediate_result: None,
            confidence: 1.0,
            duration_ms: 0,
            metadata: HashMap::new(),
        }
    }

    /// Sets the thought content.
    pub fn with_thought(mut self, thought: impl Into<String>) -> Self {
        self.thought = thought.into();
        self
    }

    /// Sets the intermediate result.
    pub fn with_result(mut self, result: impl Into<String>) -> Self {
        self.intermediate_result = Some(result.into());
        self
    }

    /// Sets the confidence score.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Sets the duration.
    pub fn with_duration(mut self, duration_ms: u128) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Adds metadata.
    pub fn add_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }
}

/// Chain-of-thought reasoning trace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningChain {
    /// Unique identifier for this chain
    pub id: String,
    /// The original query or problem
    pub query: String,
    /// All steps in the reasoning chain
    pub steps: Vec<ThoughtStep>,
    /// Final answer or conclusion
    pub final_answer: Option<String>,
    /// Overall confidence in the answer (0.0 - 1.0)
    pub overall_confidence: f64,
    /// Total time taken
    pub total_duration_ms: u128,
    /// Timestamp when chain started
    pub started_at: DateTime<Utc>,
    /// Timestamp when chain completed
    pub completed_at: Option<DateTime<Utc>>,
}

impl ReasoningChain {
    /// Creates a new chain of thought.
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            query: query.into(),
            steps: Vec::new(),
            final_answer: None,
            overall_confidence: 0.0,
            total_duration_ms: 0,
            started_at: Utc::now(),
            completed_at: None,
        }
    }

    /// Adds a step to the chain.
    pub fn add_step(&mut self, step: ThoughtStep) {
        self.steps.push(step);
    }

    /// Completes the chain with a final answer.
    pub fn complete(&mut self, answer: impl Into<String>) {
        self.final_answer = Some(answer.into());
        self.completed_at = Some(Utc::now());

        // Calculate overall confidence as weighted average of step confidences
        if !self.steps.is_empty() {
            self.overall_confidence =
                self.steps.iter().map(|s| s.confidence).sum::<f64>() / self.steps.len() as f64;
        }

        // Calculate total duration
        self.total_duration_ms = self.steps.iter().map(|s| s.duration_ms).sum();
    }

    /// Returns the number of steps in the chain.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Exports the chain as a formatted string.
    pub fn to_formatted_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Chain of Thought: {}\n", self.query));
        output.push_str(&format!("ID: {}\n", self.id));
        output.push_str(&format!("Started: {}\n\n", self.started_at));

        for step in &self.steps {
            output.push_str(&format!(
                "Step {}: {}\n",
                step.step_number, step.description
            ));
            output.push_str(&format!("  Thought: {}\n", step.thought));
            if let Some(ref result) = step.intermediate_result {
                output.push_str(&format!("  Result: {}\n", result));
            }
            output.push_str(&format!("  Confidence: {:.2}\n", step.confidence));
            output.push_str(&format!("  Duration: {}ms\n\n", step.duration_ms));
        }

        if let Some(ref answer) = self.final_answer {
            output.push_str(&format!("Final Answer: {}\n", answer));
        }
        output.push_str(&format!(
            "Overall Confidence: {:.2}\n",
            self.overall_confidence
        ));
        output.push_str(&format!("Total Duration: {}ms\n", self.total_duration_ms));

        output
    }
}

/// Chain-of-thought logger.
pub struct ChainOfThoughtLogger {
    chains: Arc<RwLock<Vec<ReasoningChain>>>,
    max_chains: usize,
}

impl ChainOfThoughtLogger {
    /// Creates a new chain-of-thought logger.
    pub fn new() -> Self {
        Self {
            chains: Arc::new(RwLock::new(Vec::new())),
            max_chains: 1000,
        }
    }

    /// Sets the maximum number of chains to keep in memory.
    pub fn with_max_chains(mut self, max: usize) -> Self {
        self.max_chains = max;
        self
    }

    /// Logs a chain of thought.
    pub async fn log_chain(&self, chain: ReasoningChain) {
        let mut chains = self.chains.write().await;
        chains.push(chain);

        // Evict oldest chains if we exceed the limit
        let len = chains.len();
        if len > self.max_chains {
            let to_drain = len - self.max_chains;
            chains.drain(0..to_drain);
        }
    }

    /// Retrieves a chain by ID.
    pub async fn get_chain(&self, id: &str) -> Option<ReasoningChain> {
        let chains = self.chains.read().await;
        chains.iter().find(|c| c.id == id).cloned()
    }

    /// Gets all logged chains.
    pub async fn get_all_chains(&self) -> Vec<ReasoningChain> {
        self.chains.read().await.clone()
    }

    /// Gets chains matching a query pattern.
    pub async fn search_chains(&self, query_pattern: &str) -> Vec<ReasoningChain> {
        let chains = self.chains.read().await;
        chains
            .iter()
            .filter(|c| c.query.contains(query_pattern))
            .cloned()
            .collect()
    }

    /// Clears all logged chains.
    pub async fn clear(&self) {
        let mut chains = self.chains.write().await;
        chains.clear();
    }
}

impl Default for ChainOfThoughtLogger {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Reasoning Trace Visualization
// ============================================================================

/// Visual representation format for reasoning traces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualizationFormat {
    /// Plain text format
    PlainText,
    /// Markdown format
    Markdown,
    /// HTML format
    Html,
    /// Mermaid diagram
    Mermaid,
    /// Graphviz DOT format
    Dot,
}

/// Reasoning trace visualizer.
pub struct ReasoningTraceVisualizer;

impl ReasoningTraceVisualizer {
    /// Creates a new visualizer.
    pub fn new() -> Self {
        Self
    }

    /// Visualizes a chain of thought in the specified format.
    pub fn visualize(&self, chain: &ReasoningChain, format: VisualizationFormat) -> String {
        match format {
            VisualizationFormat::PlainText => self.to_plain_text(chain),
            VisualizationFormat::Markdown => self.to_markdown(chain),
            VisualizationFormat::Html => self.to_html(chain),
            VisualizationFormat::Mermaid => self.to_mermaid(chain),
            VisualizationFormat::Dot => self.to_dot(chain),
        }
    }

    fn to_plain_text(&self, chain: &ReasoningChain) -> String {
        chain.to_formatted_string()
    }

    fn to_markdown(&self, chain: &ReasoningChain) -> String {
        let mut md = String::new();
        md.push_str(&format!("# Chain of Thought: {}\n\n", chain.query));
        md.push_str(&format!("**ID:** `{}`  \n", chain.id));
        md.push_str(&format!("**Started:** {}  \n", chain.started_at));
        md.push_str(&format!(
            "**Overall Confidence:** {:.2}%\n\n",
            chain.overall_confidence * 100.0
        ));

        md.push_str("## Reasoning Steps\n\n");
        for step in &chain.steps {
            md.push_str(&format!(
                "### Step {}: {}\n\n",
                step.step_number, step.description
            ));
            md.push_str(&format!("**Thought:** {}\n\n", step.thought));
            if let Some(ref result) = step.intermediate_result {
                md.push_str(&format!(
                    "**Intermediate Result:**\n```\n{}\n```\n\n",
                    result
                ));
            }
            md.push_str(&format!(
                "**Confidence:** {:.2}% | **Duration:** {}ms\n\n",
                step.confidence * 100.0,
                step.duration_ms
            ));
        }

        if let Some(ref answer) = chain.final_answer {
            md.push_str("## Final Answer\n\n");
            md.push_str(&format!("```\n{}\n```\n\n", answer));
        }

        md.push_str(&format!(
            "**Total Duration:** {}ms\n",
            chain.total_duration_ms
        ));

        md
    }

    fn to_html(&self, chain: &ReasoningChain) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str(".chain { background: #f5f5f5; padding: 20px; border-radius: 8px; }\n");
        html.push_str(".step { background: white; margin: 10px 0; padding: 15px; border-left: 4px solid #4CAF50; }\n");
        html.push_str(".confidence { color: #4CAF50; font-weight: bold; }\n");
        html.push_str(".answer { background: #e3f2fd; padding: 15px; margin-top: 20px; border-radius: 4px; }\n");
        html.push_str("</style>\n</head>\n<body>\n");

        html.push_str(&format!(
            "<div class='chain'>\n<h1>Chain of Thought: {}</h1>\n",
            chain.query
        ));
        html.push_str(&format!("<p><strong>ID:</strong> {}</p>\n", chain.id));
        html.push_str(&format!(
            "<p><strong>Overall Confidence:</strong> <span class='confidence'>{:.2}%</span></p>\n",
            chain.overall_confidence * 100.0
        ));

        html.push_str("<h2>Reasoning Steps</h2>\n");
        for step in &chain.steps {
            html.push_str("<div class='step'>\n");
            html.push_str(&format!(
                "<h3>Step {}: {}</h3>\n",
                step.step_number, step.description
            ));
            html.push_str(&format!(
                "<p><strong>Thought:</strong> {}</p>\n",
                step.thought
            ));
            if let Some(ref result) = step.intermediate_result {
                html.push_str(&format!(
                    "<p><strong>Result:</strong> <code>{}</code></p>\n",
                    result
                ));
            }
            html.push_str(&format!(
                "<p><strong>Confidence:</strong> {:.2}% | <strong>Duration:</strong> {}ms</p>\n",
                step.confidence * 100.0,
                step.duration_ms
            ));
            html.push_str("</div>\n");
        }

        if let Some(ref answer) = chain.final_answer {
            html.push_str(&format!(
                "<div class='answer'>\n<h2>Final Answer</h2>\n<p>{}</p>\n</div>\n",
                answer
            ));
        }

        html.push_str(&format!(
            "<p><strong>Total Duration:</strong> {}ms</p>\n",
            chain.total_duration_ms
        ));
        html.push_str("</div>\n</body>\n</html>");

        html
    }

    fn to_mermaid(&self, chain: &ReasoningChain) -> String {
        let mut mermaid = String::new();
        mermaid.push_str("graph TD\n");
        mermaid.push_str(&format!("  Start[\"{}\"]\n", chain.query));

        for (i, step) in chain.steps.iter().enumerate() {
            let step_id = format!("Step{}", i);
            let next_id = if i < chain.steps.len() - 1 {
                format!("Step{}", i + 1)
            } else {
                "End".to_string()
            };

            mermaid.push_str(&format!(
                "  {}[\"Step {}: {}<br/>Confidence: {:.2}%\"]\n",
                step_id,
                step.step_number,
                step.description,
                step.confidence * 100.0
            ));

            if i == 0 {
                mermaid.push_str(&format!("  Start --> {}\n", step_id));
            }

            if i < chain.steps.len() - 1 {
                mermaid.push_str(&format!("  {} --> {}\n", step_id, next_id));
            } else {
                mermaid.push_str(&format!("  {} --> End\n", step_id));
            }
        }

        if let Some(ref answer) = chain.final_answer {
            mermaid.push_str(&format!(
                "  End[\"Final Answer<br/>{:.50}...\"]\n",
                answer.chars().take(50).collect::<String>()
            ));
        }

        mermaid
    }

    fn to_dot(&self, chain: &ReasoningChain) -> String {
        let mut dot = String::new();
        dot.push_str("digraph ChainOfThought {\n");
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=box, style=rounded];\n\n");

        dot.push_str(&format!(
            "  start [label=\"{}\" shape=ellipse];\n",
            chain
                .query
                .replace('"', "\\\"")
                .chars()
                .take(50)
                .collect::<String>()
        ));

        for (i, step) in chain.steps.iter().enumerate() {
            let label = format!(
                "Step {}\\n{}\\nConfidence: {:.2}%",
                step.step_number,
                step.description
                    .replace('"', "\\\"")
                    .chars()
                    .take(40)
                    .collect::<String>(),
                step.confidence * 100.0
            );
            dot.push_str(&format!("  step{} [label=\"{}\"];\n", i, label));
        }

        if chain.final_answer.is_some() {
            dot.push_str("  end [label=\"Final Answer\" shape=ellipse];\n");
        }

        dot.push('\n');

        if !chain.steps.is_empty() {
            dot.push_str("  start -> step0;\n");
        }

        for i in 0..chain.steps.len() - 1 {
            dot.push_str(&format!("  step{} -> step{};\n", i, i + 1));
        }

        if !chain.steps.is_empty() && chain.final_answer.is_some() {
            dot.push_str(&format!("  step{} -> end;\n", chain.steps.len() - 1));
        }

        dot.push_str("}\n");

        dot
    }
}

impl Default for ReasoningTraceVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Confidence Calibration
// ============================================================================

/// Confidence calibration data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationPoint {
    /// Predicted confidence (0.0 - 1.0)
    pub predicted_confidence: f64,
    /// Actual correctness (0.0 or 1.0)
    pub actual_correct: f64,
    /// Task category
    pub category: String,
}

/// Confidence calibration metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationMetrics {
    /// Expected calibration error (ECE)
    pub expected_calibration_error: f64,
    /// Maximum calibration error
    pub maximum_calibration_error: f64,
    /// Brier score
    pub brier_score: f64,
    /// Calibration curve (binned)
    pub calibration_bins: Vec<CalibrationBin>,
    /// Number of samples
    pub sample_count: usize,
}

/// A bin in the calibration curve.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationBin {
    /// Lower bound of confidence bin
    pub confidence_lower: f64,
    /// Upper bound of confidence bin
    pub confidence_upper: f64,
    /// Average predicted confidence in this bin
    pub avg_predicted: f64,
    /// Average actual correctness in this bin
    pub avg_actual: f64,
    /// Number of samples in this bin
    pub count: usize,
}

/// Confidence calibration reporter.
pub struct ConfidenceCalibrator {
    data_points: Arc<RwLock<Vec<CalibrationPoint>>>,
    num_bins: usize,
}

impl ConfidenceCalibrator {
    /// Creates a new confidence calibrator.
    pub fn new() -> Self {
        Self {
            data_points: Arc::new(RwLock::new(Vec::new())),
            num_bins: 10,
        }
    }

    /// Sets the number of bins for calibration curve.
    pub fn with_num_bins(mut self, num_bins: usize) -> Self {
        self.num_bins = num_bins;
        self
    }

    /// Adds a calibration data point.
    pub async fn add_point(&self, point: CalibrationPoint) {
        let mut points = self.data_points.write().await;
        points.push(point);
    }

    /// Computes calibration metrics.
    pub async fn compute_metrics(&self) -> CalibrationMetrics {
        let points = self.data_points.read().await;

        if points.is_empty() {
            return CalibrationMetrics {
                expected_calibration_error: 0.0,
                maximum_calibration_error: 0.0,
                brier_score: 0.0,
                calibration_bins: Vec::new(),
                sample_count: 0,
            };
        }

        // Create bins
        let bin_width = 1.0 / self.num_bins as f64;
        let mut bins: Vec<Vec<&CalibrationPoint>> = vec![Vec::new(); self.num_bins];

        for point in points.iter() {
            let bin_idx =
                ((point.predicted_confidence / bin_width).floor() as usize).min(self.num_bins - 1);
            bins[bin_idx].push(point);
        }

        // Compute calibration bins
        let mut calibration_bins = Vec::new();
        let mut ece = 0.0;
        let mut mce: f64 = 0.0;

        for (i, bin) in bins.iter().enumerate() {
            if bin.is_empty() {
                continue;
            }

            let confidence_lower = i as f64 * bin_width;
            let confidence_upper = (i + 1) as f64 * bin_width;

            let avg_predicted =
                bin.iter().map(|p| p.predicted_confidence).sum::<f64>() / bin.len() as f64;

            let avg_actual = bin.iter().map(|p| p.actual_correct).sum::<f64>() / bin.len() as f64;

            let calibration_error = (avg_predicted - avg_actual).abs();
            let bin_weight = bin.len() as f64 / points.len() as f64;

            ece += bin_weight * calibration_error;
            mce = mce.max(calibration_error);

            calibration_bins.push(CalibrationBin {
                confidence_lower,
                confidence_upper,
                avg_predicted,
                avg_actual,
                count: bin.len(),
            });
        }

        // Compute Brier score
        let brier_score = points
            .iter()
            .map(|p| (p.predicted_confidence - p.actual_correct).powi(2))
            .sum::<f64>()
            / points.len() as f64;

        CalibrationMetrics {
            expected_calibration_error: ece,
            maximum_calibration_error: mce,
            brier_score,
            calibration_bins,
            sample_count: points.len(),
        }
    }

    /// Generates a calibration report.
    pub async fn generate_report(&self) -> String {
        let metrics = self.compute_metrics().await;

        let mut report = String::new();
        report.push_str("=== Confidence Calibration Report ===\n\n");
        report.push_str(&format!("Sample Count: {}\n", metrics.sample_count));
        report.push_str(&format!(
            "Expected Calibration Error (ECE): {:.4}\n",
            metrics.expected_calibration_error
        ));
        report.push_str(&format!(
            "Maximum Calibration Error (MCE): {:.4}\n",
            metrics.maximum_calibration_error
        ));
        report.push_str(&format!("Brier Score: {:.4}\n\n", metrics.brier_score));

        report.push_str("Calibration Bins:\n");
        for bin in &metrics.calibration_bins {
            report.push_str(&format!(
                "  [{:.2}, {:.2}]: Predicted={:.3}, Actual={:.3}, Count={}\n",
                bin.confidence_lower,
                bin.confidence_upper,
                bin.avg_predicted,
                bin.avg_actual,
                bin.count
            ));
        }

        report
    }

    /// Clears all data points.
    pub async fn clear(&self) {
        let mut points = self.data_points.write().await;
        points.clear();
    }
}

impl Default for ConfidenceCalibrator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Uncertainty Quantification
// ============================================================================

/// Types of uncertainty in model predictions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UncertaintyType {
    /// Aleatoric uncertainty (data uncertainty)
    Aleatoric,
    /// Epistemic uncertainty (model uncertainty)
    Epistemic,
    /// Combined uncertainty
    Total,
}

/// Uncertainty estimate for a prediction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyEstimate {
    /// Point estimate (mean prediction)
    pub point_estimate: f64,
    /// Aleatoric uncertainty (irreducible)
    pub aleatoric_uncertainty: f64,
    /// Epistemic uncertainty (model uncertainty)
    pub epistemic_uncertainty: f64,
    /// Total uncertainty
    pub total_uncertainty: f64,
    /// Confidence interval (lower bound)
    pub confidence_interval_lower: f64,
    /// Confidence interval (upper bound)
    pub confidence_interval_upper: f64,
    /// Confidence level (e.g., 0.95 for 95% CI)
    pub confidence_level: f64,
}

impl UncertaintyEstimate {
    /// Creates a new uncertainty estimate.
    pub fn new(point_estimate: f64) -> Self {
        Self {
            point_estimate,
            aleatoric_uncertainty: 0.0,
            epistemic_uncertainty: 0.0,
            total_uncertainty: 0.0,
            confidence_interval_lower: point_estimate,
            confidence_interval_upper: point_estimate,
            confidence_level: 0.95,
        }
    }

    /// Sets aleatoric uncertainty.
    pub fn with_aleatoric(mut self, uncertainty: f64) -> Self {
        self.aleatoric_uncertainty = uncertainty;
        self.update_total();
        self
    }

    /// Sets epistemic uncertainty.
    pub fn with_epistemic(mut self, uncertainty: f64) -> Self {
        self.epistemic_uncertainty = uncertainty;
        self.update_total();
        self
    }

    /// Sets confidence interval.
    pub fn with_confidence_interval(mut self, lower: f64, upper: f64, level: f64) -> Self {
        self.confidence_interval_lower = lower;
        self.confidence_interval_upper = upper;
        self.confidence_level = level;
        self
    }

    fn update_total(&mut self) {
        // Total uncertainty is the quadrature sum of aleatoric and epistemic
        self.total_uncertainty =
            (self.aleatoric_uncertainty.powi(2) + self.epistemic_uncertainty.powi(2)).sqrt();
    }

    /// Returns true if the uncertainty is high (>threshold).
    pub fn is_high_uncertainty(&self, threshold: f64) -> bool {
        self.total_uncertainty > threshold
    }
}

/// Uncertainty quantifier.
pub struct UncertaintyQuantifier {
    confidence_level: f64,
}

impl UncertaintyQuantifier {
    /// Creates a new uncertainty quantifier.
    pub fn new() -> Self {
        Self {
            confidence_level: 0.95,
        }
    }

    /// Sets the confidence level for intervals.
    pub fn with_confidence_level(mut self, level: f64) -> Self {
        self.confidence_level = level.clamp(0.0, 1.0);
        self
    }

    /// Estimates uncertainty from multiple predictions (ensemble).
    pub fn estimate_from_ensemble(&self, predictions: &[f64]) -> UncertaintyEstimate {
        if predictions.is_empty() {
            return UncertaintyEstimate::new(0.0);
        }

        let mean = predictions.iter().sum::<f64>() / predictions.len() as f64;
        let variance =
            predictions.iter().map(|p| (p - mean).powi(2)).sum::<f64>() / predictions.len() as f64;
        let std_dev = variance.sqrt();

        // For ensemble, epistemic uncertainty is the variance of predictions
        let epistemic = std_dev;

        // Compute confidence interval (assuming normal distribution)
        let z_score = self.z_score_for_confidence(self.confidence_level);
        let margin = z_score * std_dev;

        UncertaintyEstimate::new(mean)
            .with_epistemic(epistemic)
            .with_confidence_interval(mean - margin, mean + margin, self.confidence_level)
    }

    fn z_score_for_confidence(&self, level: f64) -> f64 {
        // Approximate z-scores for common confidence levels
        match (level * 100.0) as u32 {
            90 => 1.645,
            95 => 1.96,
            99 => 2.576,
            _ => 1.96, // Default to 95%
        }
    }
}

impl Default for UncertaintyQuantifier {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Decision Audit Trail
// ============================================================================

/// A single decision in an audit trail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRecord {
    /// Unique decision ID
    pub id: String,
    /// Timestamp of decision
    pub timestamp: DateTime<Utc>,
    /// Decision maker (model, agent, etc.)
    pub decision_maker: String,
    /// Input that led to the decision
    pub input: String,
    /// The decision made
    pub decision: String,
    /// Rationale or justification
    pub rationale: String,
    /// Confidence in the decision (0.0 - 1.0)
    pub confidence: f64,
    /// Alternatives considered
    pub alternatives: Vec<String>,
    /// Metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl DecisionRecord {
    /// Creates a new decision record.
    pub fn new(
        decision_maker: impl Into<String>,
        input: impl Into<String>,
        decision: impl Into<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            decision_maker: decision_maker.into(),
            input: input.into(),
            decision: decision.into(),
            rationale: String::new(),
            confidence: 1.0,
            alternatives: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Sets the rationale.
    pub fn with_rationale(mut self, rationale: impl Into<String>) -> Self {
        self.rationale = rationale.into();
        self
    }

    /// Sets the confidence.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Adds an alternative.
    pub fn add_alternative(mut self, alternative: impl Into<String>) -> Self {
        self.alternatives.push(alternative.into());
        self
    }

    /// Adds metadata.
    pub fn add_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }
}

/// Decision audit trail.
pub struct DecisionAuditTrail {
    decisions: Arc<RwLock<Vec<DecisionRecord>>>,
    max_decisions: usize,
}

impl DecisionAuditTrail {
    /// Creates a new decision audit trail.
    pub fn new() -> Self {
        Self {
            decisions: Arc::new(RwLock::new(Vec::new())),
            max_decisions: 10000,
        }
    }

    /// Sets the maximum number of decisions to keep.
    pub fn with_max_decisions(mut self, max: usize) -> Self {
        self.max_decisions = max;
        self
    }

    /// Records a decision.
    pub async fn record(&self, decision: DecisionRecord) {
        let mut decisions = self.decisions.write().await;
        decisions.push(decision);

        // Evict oldest decisions if we exceed the limit
        let len = decisions.len();
        if len > self.max_decisions {
            let to_drain = len - self.max_decisions;
            decisions.drain(0..to_drain);
        }
    }

    /// Gets a decision by ID.
    pub async fn get_decision(&self, id: &str) -> Option<DecisionRecord> {
        let decisions = self.decisions.read().await;
        decisions.iter().find(|d| d.id == id).cloned()
    }

    /// Gets all decisions.
    pub async fn get_all_decisions(&self) -> Vec<DecisionRecord> {
        self.decisions.read().await.clone()
    }

    /// Gets decisions by decision maker.
    pub async fn get_by_decision_maker(&self, decision_maker: &str) -> Vec<DecisionRecord> {
        let decisions = self.decisions.read().await;
        decisions
            .iter()
            .filter(|d| d.decision_maker == decision_maker)
            .cloned()
            .collect()
    }

    /// Gets decisions within a time range.
    pub async fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<DecisionRecord> {
        let decisions = self.decisions.read().await;
        decisions
            .iter()
            .filter(|d| d.timestamp >= start && d.timestamp <= end)
            .cloned()
            .collect()
    }

    /// Exports audit trail as JSON.
    pub async fn export_json(&self) -> Result<String> {
        let decisions = self.decisions.read().await;
        serde_json::to_string_pretty(&*decisions)
            .map_err(|e| anyhow::anyhow!("Failed to serialize audit trail: {}", e))
    }

    /// Clears the audit trail.
    pub async fn clear(&self) {
        let mut decisions = self.decisions.write().await;
        decisions.clear();
    }
}

impl Default for DecisionAuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thought_step_creation() {
        let step = ThoughtStep::new(1, "Analyze problem")
            .with_thought("This is a complex problem")
            .with_confidence(0.85)
            .with_duration(100);

        assert_eq!(step.step_number, 1);
        assert_eq!(step.description, "Analyze problem");
        assert!((step.confidence - 0.85).abs() < f64::EPSILON);
        assert_eq!(step.duration_ms, 100);
    }

    #[test]
    fn test_chain_of_thought() {
        let mut chain = ReasoningChain::new("What is 2+2?");

        let step1 = ThoughtStep::new(1, "Parse numbers")
            .with_thought("Identify 2 and 2")
            .with_result("2, 2")
            .with_confidence(1.0);

        let step2 = ThoughtStep::new(2, "Add numbers")
            .with_thought("2 + 2 = 4")
            .with_result("4")
            .with_confidence(1.0);

        chain.add_step(step1);
        chain.add_step(step2);
        chain.complete("4");

        assert_eq!(chain.step_count(), 2);
        assert_eq!(chain.final_answer, Some("4".to_string()));
        assert!((chain.overall_confidence - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_visualization_formats() {
        let mut chain = ReasoningChain::new("Test query");
        chain.add_step(ThoughtStep::new(1, "Step 1").with_thought("Thinking..."));
        chain.complete("Answer");

        let visualizer = ReasoningTraceVisualizer::new();

        let plain = visualizer.visualize(&chain, VisualizationFormat::PlainText);
        assert!(plain.contains("Test query"));

        let markdown = visualizer.visualize(&chain, VisualizationFormat::Markdown);
        assert!(markdown.contains("# Chain of Thought"));

        let html = visualizer.visualize(&chain, VisualizationFormat::Html);
        assert!(html.contains("<!DOCTYPE html>"));

        let mermaid = visualizer.visualize(&chain, VisualizationFormat::Mermaid);
        assert!(mermaid.contains("graph TD"));

        let dot = visualizer.visualize(&chain, VisualizationFormat::Dot);
        assert!(dot.contains("digraph ChainOfThought"));
    }

    #[test]
    fn test_calibration_bin() {
        let bin = CalibrationBin {
            confidence_lower: 0.5,
            confidence_upper: 0.6,
            avg_predicted: 0.55,
            avg_actual: 0.52,
            count: 10,
        };

        assert!((bin.avg_predicted - 0.55).abs() < f64::EPSILON);
        assert_eq!(bin.count, 10);
    }

    #[test]
    fn test_uncertainty_estimate() {
        let estimate = UncertaintyEstimate::new(0.8)
            .with_aleatoric(0.1)
            .with_epistemic(0.15);

        assert!((estimate.point_estimate - 0.8).abs() < f64::EPSILON);
        assert!((estimate.aleatoric_uncertainty - 0.1).abs() < f64::EPSILON);
        assert!((estimate.epistemic_uncertainty - 0.15).abs() < f64::EPSILON);

        // Total should be sqrt(0.1^2 + 0.15^2) = sqrt(0.0325) ≈ 0.1803
        assert!((estimate.total_uncertainty - 0.1803).abs() < 0.001);
    }

    #[test]
    fn test_uncertainty_from_ensemble() {
        let quantifier = UncertaintyQuantifier::new();
        let predictions = vec![0.8, 0.85, 0.75, 0.9, 0.82];

        let estimate = quantifier.estimate_from_ensemble(&predictions);

        assert!(estimate.point_estimate > 0.0);
        assert!(estimate.epistemic_uncertainty > 0.0);
        assert!(estimate.confidence_interval_lower < estimate.point_estimate);
        assert!(estimate.confidence_interval_upper > estimate.point_estimate);
    }

    #[test]
    fn test_decision_record() {
        let decision = DecisionRecord::new("Agent-1", "Input text", "Decision made")
            .with_rationale("This is why")
            .with_confidence(0.9)
            .add_alternative("Alternative 1")
            .add_alternative("Alternative 2");

        assert_eq!(decision.decision_maker, "Agent-1");
        assert!((decision.confidence - 0.9).abs() < f64::EPSILON);
        assert_eq!(decision.alternatives.len(), 2);
    }

    #[tokio::test]
    async fn test_chain_logger() {
        let logger = ChainOfThoughtLogger::new();
        let chain = ReasoningChain::new("Test");

        logger.log_chain(chain.clone()).await;

        let chains = logger.get_all_chains().await;
        assert_eq!(chains.len(), 1);

        let retrieved = logger.get_chain(&chain.id).await;
        assert!(retrieved.is_some());
    }

    #[tokio::test]
    async fn test_audit_trail() {
        let trail = DecisionAuditTrail::new();
        let decision = DecisionRecord::new("Agent-1", "Input", "Decision");

        trail.record(decision.clone()).await;

        let decisions = trail.get_all_decisions().await;
        assert_eq!(decisions.len(), 1);

        let by_maker = trail.get_by_decision_maker("Agent-1").await;
        assert_eq!(by_maker.len(), 1);
    }
}
