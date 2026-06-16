//! LLM-native legal format.
//!
//! A representation purpose-built for feeding statutes into a large language
//! model as prompt context. Each statute becomes a clean Markdown "block" with
//! an attached structured provenance record. Blocks are ordered by a
//! deterministic salience score and the document tracks a token budget so a
//! caller can render the highest-value subset that fits a model's context
//! window.
//!
//! The serialised form is JSON (so it carries provenance and metadata
//! losslessly), while [`LlmNativeDocument::render_prompt`] produces the
//! ready-to-paste Markdown context.

use super::{StructuredStatute, estimate_tokens, render_statute_markdown};
use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{EffectType, Statute};
use serde::{Deserialize, Serialize};

/// Schema identifier for the LLM-native format.
pub const SCHEMA: &str = "legalis.llm-native/v1";

/// Default token budget (a comfortable fit for an 8k context window alongside
/// instructions and completion).
pub const DEFAULT_TOKEN_BUDGET: usize = 4096;

/// An LLM-native document: ordered, budgeted, provenance-carrying blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmNativeDocument {
    /// Schema identifier ([`SCHEMA`]).
    pub schema: String,
    /// Generating component identifier.
    pub generator: String,
    /// Token budget the prompt ordering targets.
    pub token_budget: usize,
    /// Sum of every block's estimated token count.
    pub total_tokens: usize,
    /// Blocks in original statute order.
    pub blocks: Vec<LlmBlock>,
    /// Block indices ordered by descending salience (the prompt order).
    pub prompt_order: Vec<usize>,
}

/// A single LLM-native block: one statute rendered for prompt context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmBlock {
    /// Statute identifier.
    pub id: String,
    /// Statute title.
    pub title: String,
    /// Deterministic salience score (higher = more important to include).
    pub salience: f64,
    /// Estimated token cost of the block's Markdown.
    pub estimated_tokens: usize,
    /// Whether the block fits within the token budget under salience ordering.
    pub included: bool,
    /// Clean Markdown rendering of the statute.
    pub markdown: String,
    /// Structured provenance enabling lossless reconstruction.
    pub provenance: StructuredStatute,
}

/// Computes a deterministic salience score for a statute.
///
/// Restrictive and obligatory effects rank highest (they constrain behaviour),
/// and complexity (preconditions, exceptions, applicability) raises salience.
fn salience(statute: &Statute) -> f64 {
    let base = match statute.effect.effect_type {
        EffectType::Prohibition => 1.0,
        EffectType::Obligation => 0.9,
        EffectType::Revoke => 0.8,
        EffectType::MonetaryTransfer => 0.7,
        EffectType::Grant => 0.6,
        EffectType::StatusChange => 0.5,
        EffectType::Custom => 0.4,
    };
    base + 0.10 * statute.preconditions.len() as f64
        + 0.15 * statute.exceptions.len() as f64
        + 0.05 * statute.applies_to.len() as f64
}

impl LlmNativeDocument {
    /// Builds a document from statutes targeting the given token budget.
    pub fn build(statutes: &[Statute], token_budget: usize) -> Self {
        let mut blocks: Vec<LlmBlock> = statutes
            .iter()
            .map(|statute| {
                let markdown = render_statute_markdown(statute);
                let estimated_tokens = estimate_tokens(&markdown);
                LlmBlock {
                    id: statute.id.clone(),
                    title: statute.title.clone(),
                    salience: salience(statute),
                    estimated_tokens,
                    included: false,
                    markdown,
                    provenance: StructuredStatute::from_statute(statute),
                }
            })
            .collect();

        let mut prompt_order: Vec<usize> = (0..blocks.len()).collect();
        prompt_order.sort_by(|&a, &b| {
            blocks[b]
                .salience
                .total_cmp(&blocks[a].salience)
                .then_with(|| blocks[a].id.cmp(&blocks[b].id))
        });

        let mut cumulative = 0usize;
        for &index in &prompt_order {
            let cost = blocks[index].estimated_tokens;
            if cumulative + cost <= token_budget {
                blocks[index].included = true;
                cumulative += cost;
            }
        }

        let total_tokens = blocks.iter().map(|block| block.estimated_tokens).sum();

        Self {
            schema: SCHEMA.to_string(),
            generator: "legalis-interop/llm-native".to_string(),
            token_budget,
            total_tokens,
            blocks,
            prompt_order,
        }
    }

    /// Renders the included blocks, in salience order, as a single Markdown
    /// prompt context not exceeding the token budget.
    pub fn render_prompt(&self) -> String {
        let mut out = String::new();
        let mut cumulative = 0usize;
        for &index in &self.prompt_order {
            if let Some(block) = self.blocks.get(index) {
                if cumulative + block.estimated_tokens > self.token_budget {
                    continue;
                }
                out.push_str(&block.markdown);
                out.push('\n');
                cumulative += block.estimated_tokens;
            }
        }
        out
    }

    /// Number of blocks that fit within the token budget.
    pub fn included_count(&self) -> usize {
        self.blocks.iter().filter(|block| block.included).count()
    }

    /// Reconstructs the underlying statutes (original order).
    pub fn to_statutes(&self) -> Vec<Statute> {
        self.blocks
            .iter()
            .map(|block| block.provenance.to_statute())
            .collect()
    }

    /// Serialises the document to pretty JSON.
    pub fn to_json(&self) -> InteropResult<String> {
        serde_json::to_string_pretty(self).map_err(|error| {
            InteropError::SerializationError(format!("Failed to serialize LLM-native: {error}"))
        })
    }

    /// Parses a document from JSON.
    pub fn from_json(source: &str) -> InteropResult<Self> {
        serde_json::from_str(source).map_err(|error| {
            InteropError::ParseError(format!("Failed to parse LLM-native JSON: {error}"))
        })
    }
}

/// Importer for the LLM-native format.
#[derive(Debug, Default)]
pub struct LlmNativeImporter;

impl LlmNativeImporter {
    /// Creates a new importer.
    pub fn new() -> Self {
        Self
    }
}

impl FormatImporter for LlmNativeImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::LlmNative
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let document = LlmNativeDocument::from_json(source)?;
        let statutes = document.to_statutes();
        let mut report = ConversionReport::new(LegalFormat::LlmNative, LegalFormat::Legalis);
        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        serde_json::from_str::<serde_json::Value>(source)
            .ok()
            .and_then(|value| {
                value
                    .get("schema")
                    .and_then(|schema| schema.as_str())
                    .map(|schema| schema == SCHEMA)
            })
            .unwrap_or(false)
    }
}

/// Exporter for the LLM-native format.
#[derive(Debug, Clone, Copy)]
pub struct LlmNativeExporter {
    token_budget: usize,
}

impl LlmNativeExporter {
    /// Creates an exporter with the default token budget.
    pub fn new() -> Self {
        Self {
            token_budget: DEFAULT_TOKEN_BUDGET,
        }
    }

    /// Sets the token budget used for prompt ordering.
    pub fn with_token_budget(mut self, token_budget: usize) -> Self {
        self.token_budget = token_budget.max(1);
        self
    }
}

impl Default for LlmNativeExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for LlmNativeExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::LlmNative
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let document = LlmNativeDocument::build(statutes, self.token_budget);
        let json = document.to_json()?;
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::LlmNative);
        report.statutes_converted = statutes.len();
        if document.included_count() < statutes.len() {
            report.add_warning(format!(
                "Token budget {} fits {}/{} blocks; remainder excluded from prompt context",
                self.token_budget,
                document.included_count(),
                statutes.len()
            ));
        }
        Ok((json, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        // Provenance records make the LLM-native format fully expressive.
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect};

    fn statutes() -> Vec<Statute> {
        let prohibition = Statute::new(
            "no-insider-trading",
            "Insider Trading Prohibition",
            Effect::new(
                EffectType::Prohibition,
                "Trading on material non-public data",
            ),
        )
        .with_precondition(Condition::Custom {
            description: "possesses material non-public information".to_string(),
        });

        let grant = Statute::new(
            "voting-rights",
            "Voting Rights",
            Effect::new(EffectType::Grant, "Grant the right to vote"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_jurisdiction("US");

        vec![prohibition, grant]
    }

    #[test]
    fn test_build_orders_by_salience() {
        let document = LlmNativeDocument::build(&statutes(), DEFAULT_TOKEN_BUDGET);
        // Prohibition has higher salience than grant, so it leads the prompt.
        let first = document.prompt_order[0];
        assert_eq!(document.blocks[first].id, "no-insider-trading");
    }

    #[test]
    fn test_render_prompt_contains_markdown() {
        let document = LlmNativeDocument::build(&statutes(), DEFAULT_TOKEN_BUDGET);
        let prompt = document.render_prompt();
        assert!(prompt.contains("## Insider Trading Prohibition"));
        assert!(prompt.contains("## Voting Rights"));
        assert!(prompt.contains("age >= 18"));
    }

    #[test]
    fn test_token_budget_excludes_low_salience() {
        let document = LlmNativeDocument::build(&statutes(), 1);
        // A tiny budget includes nothing; the highest-salience block alone
        // already exceeds a 1-token budget.
        assert_eq!(document.included_count(), 0);
        assert!(document.render_prompt().is_empty());
    }

    #[test]
    fn test_token_budget_partial_inclusion() {
        let all = LlmNativeDocument::build(&statutes(), DEFAULT_TOKEN_BUDGET);
        let lead_cost = all.blocks[all.prompt_order[0]].estimated_tokens;
        let document = LlmNativeDocument::build(&statutes(), lead_cost);
        assert_eq!(document.included_count(), 1);
        let prompt = document.render_prompt();
        assert!(prompt.contains("Insider Trading Prohibition"));
        assert!(!prompt.contains("Voting Rights"));
    }

    #[test]
    fn test_export_import_roundtrip() {
        let exporter = LlmNativeExporter::new();
        let importer = LlmNativeImporter::new();
        let (json, export_report) = exporter.export(&statutes()).unwrap();
        assert_eq!(export_report.statutes_converted, 2);

        let (imported, import_report) = importer.import(&json).unwrap();
        assert_eq!(import_report.statutes_converted, 2);
        assert_eq!(imported.len(), 2);
        let ids: Vec<&str> = imported.iter().map(|s| s.id.as_str()).collect();
        assert!(ids.contains(&"voting-rights"));
        assert!(ids.contains(&"no-insider-trading"));
        let voting = imported
            .iter()
            .find(|s| s.id == "voting-rights")
            .expect("voting statute present");
        assert_eq!(voting.jurisdiction.as_deref(), Some("US"));
        assert_eq!(voting.preconditions.len(), 1);
    }

    #[test]
    fn test_validate_accepts_only_schema() {
        let importer = LlmNativeImporter::new();
        let (json, _) = LlmNativeExporter::new().export(&statutes()).unwrap();
        assert!(importer.validate(&json));
        assert!(!importer.validate("{\"schema\":\"other/v1\"}"));
        assert!(!importer.validate("not json"));
    }

    #[test]
    fn test_with_token_budget_builder() {
        let exporter = LlmNativeExporter::new().with_token_budget(32);
        let (json, _) = exporter.export(&statutes()).unwrap();
        let document = LlmNativeDocument::from_json(&json).unwrap();
        assert_eq!(document.token_budget, 32);
    }
}
