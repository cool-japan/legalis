//! Attention-aware legal markup.
//!
//! Decomposes each statute into role-tagged spans (title, conditions, effect)
//! and assigns every span an attention weight. Weights are a genuine attention
//! distribution: each span is scored by the summed TF-IDF salience of its
//! tokens against the whole corpus, then the scores are passed through a
//! numerically-stable softmax so a statute's span weights sum to 1. Spans also
//! carry cross-references (to derived statutes and applicable entity types).
//!
//! Besides the structured spans, each unit exposes an inline `rendered` markup
//! string using white-square-bracket delimiters, e.g.
//! `⟦effect|a=0.421⟧ ... ⟦/⟧`. These delimiters are chosen to avoid colliding
//! with the substring signatures of other template formats (OpenLaw `[[`,
//! Cicero `{{`, ContractExpress `«»`).

use super::{StructuredStatute, effect_type_to_str, render_condition, softmax, tokenize};
use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Schema identifier for the attention markup format.
pub const SCHEMA: &str = "legalis.attention-markup/v1";

/// An attention-markup document over a statute corpus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionDocument {
    /// Schema identifier ([`SCHEMA`]).
    pub schema: String,
    /// Attention scoring method.
    pub method: String,
    /// Per-statute annotated units.
    pub units: Vec<AttentionUnit>,
}

/// One annotated statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionUnit {
    /// Statute identifier.
    pub id: String,
    /// Statute title.
    pub title: String,
    /// Role-tagged, attention-weighted spans.
    pub spans: Vec<AttentionSpan>,
    /// Inline markup rendering of the spans.
    pub rendered: String,
    /// Structured provenance enabling lossless reconstruction.
    pub provenance: StructuredStatute,
}

/// A single attention-weighted span.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttentionSpan {
    /// Local span identifier (`s0`, `s1`, ...).
    pub span_id: String,
    /// Semantic role (`title`, `condition`, `effect`).
    pub role: String,
    /// Span text.
    pub text: String,
    /// Softmax attention weight (the unit's weights sum to 1).
    pub attention: f64,
    /// Cross-references (`statute:<id>`, `entity:<type>`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cross_refs: Vec<String>,
}

struct RawSpan {
    role: String,
    text: String,
    cross_refs: Vec<String>,
    tokens: Vec<String>,
}

fn raw_spans(statute: &Statute) -> Vec<RawSpan> {
    let mut spans = Vec::new();

    let mut cross_refs = Vec::new();
    for source in &statute.derives_from {
        cross_refs.push(format!("statute:{source}"));
    }
    for entity in &statute.applies_to {
        cross_refs.push(format!("entity:{entity}"));
    }
    spans.push(RawSpan {
        role: "title".to_string(),
        tokens: tokenize(&statute.title),
        text: statute.title.clone(),
        cross_refs,
    });

    for condition in &statute.preconditions {
        let text = render_condition(condition);
        let tokens = tokenize(&text);
        spans.push(RawSpan {
            role: "condition".to_string(),
            text,
            cross_refs: Vec::new(),
            tokens,
        });
    }

    let effect_text = format!(
        "{}: {}",
        effect_type_to_str(&statute.effect.effect_type),
        statute.effect.description
    );
    let tokens = tokenize(&effect_text);
    spans.push(RawSpan {
        role: "effect".to_string(),
        text: effect_text,
        cross_refs: Vec::new(),
        tokens,
    });

    spans
}

fn render_inline(spans: &[AttentionSpan]) -> String {
    spans
        .iter()
        .map(|span| format!("⟦{}|a={:.3}⟧ {} ⟦/⟧", span.role, span.attention, span.text))
        .collect::<Vec<_>>()
        .join(" ")
}

impl AttentionDocument {
    /// Builds an attention-markup document over the statutes.
    pub fn build(statutes: &[Statute]) -> Self {
        let raw: Vec<Vec<RawSpan>> = statutes.iter().map(raw_spans).collect();

        // Corpus-wide document frequency (number of spans containing a token).
        let mut document_frequency: BTreeMap<String, usize> = BTreeMap::new();
        let mut total_spans = 0usize;
        for unit in &raw {
            for span in unit {
                let unique: BTreeSet<&String> = span.tokens.iter().collect();
                for token in unique {
                    *document_frequency.entry(token.clone()).or_insert(0) += 1;
                }
                total_spans += 1;
            }
        }
        let corpus = total_spans.max(1) as f64;

        let units = statutes
            .iter()
            .zip(raw)
            .map(|(statute, unit)| {
                let scores: Vec<f64> = unit
                    .iter()
                    .map(|span| {
                        span.tokens
                            .iter()
                            .map(|token| {
                                let df = *document_frequency.get(token).unwrap_or(&1) as f64;
                                ((corpus + 1.0) / (df + 1.0)).ln() + 1.0
                            })
                            .sum::<f64>()
                    })
                    .collect();
                let attention = softmax(&scores);

                let spans: Vec<AttentionSpan> = unit
                    .into_iter()
                    .enumerate()
                    .map(|(index, span)| AttentionSpan {
                        span_id: format!("s{index}"),
                        role: span.role,
                        text: span.text,
                        attention: attention.get(index).copied().unwrap_or(0.0),
                        cross_refs: span.cross_refs,
                    })
                    .collect();

                let rendered = render_inline(&spans);
                AttentionUnit {
                    id: statute.id.clone(),
                    title: statute.title.clone(),
                    spans,
                    rendered,
                    provenance: StructuredStatute::from_statute(statute),
                }
            })
            .collect();

        Self {
            schema: SCHEMA.to_string(),
            method: "tfidf-softmax".to_string(),
            units,
        }
    }

    /// Reconstructs the underlying statutes.
    pub fn to_statutes(&self) -> Vec<Statute> {
        self.units
            .iter()
            .map(|unit| unit.provenance.to_statute())
            .collect()
    }

    /// Serialises the document to pretty JSON.
    pub fn to_json(&self) -> InteropResult<String> {
        serde_json::to_string_pretty(self).map_err(|error| {
            InteropError::SerializationError(format!("Failed to serialize attention doc: {error}"))
        })
    }

    /// Parses a document from JSON.
    pub fn from_json(source: &str) -> InteropResult<Self> {
        serde_json::from_str(source).map_err(|error| {
            InteropError::ParseError(format!("Failed to parse attention JSON: {error}"))
        })
    }
}

/// Importer for the attention markup format.
#[derive(Debug, Default)]
pub struct AttentionMarkupImporter;

impl AttentionMarkupImporter {
    /// Creates a new importer.
    pub fn new() -> Self {
        Self
    }
}

impl FormatImporter for AttentionMarkupImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::AttentionMarkup
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let document = AttentionDocument::from_json(source)?;
        let statutes = document.to_statutes();
        let mut report = ConversionReport::new(LegalFormat::AttentionMarkup, LegalFormat::Legalis);
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

/// Exporter for the attention markup format.
#[derive(Debug, Default, Clone, Copy)]
pub struct AttentionMarkupExporter;

impl AttentionMarkupExporter {
    /// Creates a new exporter.
    pub fn new() -> Self {
        Self
    }
}

impl FormatExporter for AttentionMarkupExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::AttentionMarkup
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let document = AttentionDocument::build(statutes);
        let json = document.to_json()?;
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::AttentionMarkup);
        report.statutes_converted = statutes.len();
        Ok((json, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn statutes() -> Vec<Statute> {
        vec![
            Statute::new(
                "voting-rights",
                "Voting Rights",
                Effect::new(EffectType::Grant, "Grant the right to vote"),
            )
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            })
            .with_applies_to("Citizen")
            .with_derives_from("constitution-15th-amendment"),
            Statute::new(
                "tax-liability",
                "Income Tax Liability",
                Effect::new(EffectType::Obligation, "Pay income tax on earnings"),
            )
            .with_precondition(Condition::Income {
                operator: ComparisonOp::GreaterOrEqual,
                value: 12_000,
            }),
        ]
    }

    #[test]
    fn test_attention_sums_to_one_per_unit() {
        let document = AttentionDocument::build(&statutes());
        for unit in &document.units {
            let sum: f64 = unit.spans.iter().map(|span| span.attention).sum();
            assert!((sum - 1.0).abs() < 1e-9, "unit {} sum {}", unit.id, sum);
            for span in &unit.spans {
                assert!(span.attention >= 0.0 && span.attention <= 1.0);
            }
        }
    }

    #[test]
    fn test_span_roles_present() {
        let document = AttentionDocument::build(&statutes());
        let unit = &document.units[0];
        let roles: Vec<&str> = unit.spans.iter().map(|span| span.role.as_str()).collect();
        assert!(roles.contains(&"title"));
        assert!(roles.contains(&"condition"));
        assert!(roles.contains(&"effect"));
    }

    #[test]
    fn test_cross_references_recorded() {
        let document = AttentionDocument::build(&statutes());
        let title_span = document.units[0]
            .spans
            .iter()
            .find(|span| span.role == "title")
            .expect("title span");
        assert!(
            title_span
                .cross_refs
                .contains(&"statute:constitution-15th-amendment".to_string())
        );
        assert!(
            title_span
                .cross_refs
                .contains(&"entity:Citizen".to_string())
        );
    }

    #[test]
    fn test_rendered_markup_uses_safe_delimiters() {
        let document = AttentionDocument::build(&statutes());
        let rendered = &document.units[0].rendered;
        assert!(rendered.contains("⟦title|a="));
        assert!(rendered.contains("⟦/⟧"));
        // Must not collide with OpenLaw / Cicero / ContractExpress detectors.
        assert!(!rendered.contains("[["));
        assert!(!rendered.contains("{{"));
        assert!(!rendered.contains('«'));
    }

    #[test]
    fn test_export_import_roundtrip() {
        let exporter = AttentionMarkupExporter::new();
        let importer = AttentionMarkupImporter::new();
        let (json, export_report) = exporter.export(&statutes()).unwrap();
        assert_eq!(export_report.statutes_converted, 2);

        let (imported, import_report) = importer.import(&json).unwrap();
        assert_eq!(import_report.statutes_converted, 2);
        let voting = imported
            .iter()
            .find(|s| s.id == "voting-rights")
            .expect("voting statute present");
        assert_eq!(voting.applies_to, vec!["Citizen".to_string()]);
        assert_eq!(
            voting.derives_from,
            vec!["constitution-15th-amendment".to_string()]
        );
        assert_eq!(voting.preconditions.len(), 1);
    }

    #[test]
    fn test_validate() {
        let importer = AttentionMarkupImporter::new();
        let (json, _) = AttentionMarkupExporter::new().export(&statutes()).unwrap();
        assert!(importer.validate(&json));
        assert!(!importer.validate("{\"schema\":\"legalis.neural-document/v1\"}"));
    }
}
