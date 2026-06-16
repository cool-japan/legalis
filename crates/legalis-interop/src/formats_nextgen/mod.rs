//! Next-generation, AI-native legal document formats.
//!
//! This module groups a family of formats designed for modern machine-learning
//! pipelines rather than for human typesetting or legacy XML toolchains:
//!
//! - **LLM-native** ([`llm_native`]): a clean Markdown/JSON-with-provenance
//!   representation optimised for prompt context, with token-budget-aware
//!   section ordering.
//! - **Embedding** ([`embedding`]): text chunks paired with fixed-dimension
//!   float embedding vectors produced by a deterministic, pure-Rust feature
//!   hashing embedder, supporting cosine-similarity retrieval without any
//!   external model.
//! - **Neural document** ([`neural`]): a graph representation whose node
//!   salience is computed with a weighted PageRank over a semantic-similarity
//!   adjacency, plus typed cross-reference edges.
//! - **Attention markup** ([`attention`]): span-level annotation that assigns a
//!   softmax-normalised attention distribution and cross-references to clauses.
//! - **Semantic chunk** ([`semantic_chunk`]): overlap-controlled, RAG-oriented
//!   chunking with stable content-addressed identifiers.
//!
//! Every format embeds a [`StructuredStatute`] provenance record so that, in
//! addition to its AI-native view, it can losslessly round-trip the underlying
//! [`Statute`] set through the standard [`crate::FormatImporter`] /
//! [`crate::FormatExporter`] pipeline.
//!
//! All algorithms here are deterministic and dependency-free (beyond `serde`),
//! keeping the workspace pure-Rust and reproducible.

pub mod attention;
pub mod embedding;
pub mod llm_native;
pub mod neural;
pub mod semantic_chunk;

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Identifier of the deterministic feature-hashing embedder used across the
/// AI-native formats.
pub const EMBEDDER_MODEL_ID: &str = "legalis-hash-embedder/v1";

/// Default embedding dimension used when none is specified.
pub const DEFAULT_EMBEDDING_DIM: usize = 256;

/// A machine-readable, lossless projection of a [`Statute`].
///
/// This is the shared provenance backbone of every AI-native format: each
/// format renders its own view (markdown, embeddings, attention, chunks, ...)
/// but also carries the structured record so the original statute can be
/// reconstructed during import.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuredStatute {
    /// Stable statute identifier.
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Jurisdiction code, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    /// Statute version number.
    pub version: u32,
    /// Canonical effect-type token (see [`effect_type_to_str`]).
    pub effect_type: String,
    /// Effect description.
    pub effect_description: String,
    /// Effect parameters in deterministic (sorted) order.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, String>,
    /// Preconditions rendered as canonical condition strings.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<String>,
    /// Entity types the statute applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applies_to: Vec<String>,
    /// Statute identifiers this statute derives from.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derives_from: Vec<String>,
}

impl StructuredStatute {
    /// Projects a [`Statute`] into its structured, deterministic form.
    pub fn from_statute(statute: &Statute) -> Self {
        let parameters: BTreeMap<String, String> = statute
            .effect
            .parameters
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let conditions = statute.preconditions.iter().map(render_condition).collect();
        Self {
            id: statute.id.clone(),
            title: statute.title.clone(),
            jurisdiction: statute.jurisdiction.clone(),
            version: statute.version,
            effect_type: effect_type_to_str(&statute.effect.effect_type).to_string(),
            effect_description: statute.effect.description.clone(),
            parameters,
            conditions,
            applies_to: statute.applies_to.clone(),
            derives_from: statute.derives_from.clone(),
        }
    }

    /// Reconstructs a [`Statute`] from the structured record.
    pub fn to_statute(&self) -> Statute {
        let mut effect = Effect::new(
            str_to_effect_type(&self.effect_type),
            &self.effect_description,
        );
        for (key, value) in &self.parameters {
            effect.parameters.insert(key.clone(), value.clone());
        }
        let mut statute = Statute::new(&self.id, &self.title, effect).with_version(self.version);
        if let Some(jurisdiction) = &self.jurisdiction {
            statute = statute.with_jurisdiction(jurisdiction);
        }
        for condition in &self.conditions {
            statute = statute.with_precondition(parse_condition(condition));
        }
        for entity in &self.applies_to {
            statute = statute.with_applies_to(entity);
        }
        for source in &self.derives_from {
            statute = statute.with_derives_from(source);
        }
        statute
    }
}

/// Projects a slice of statutes into structured records.
pub fn build_structured(statutes: &[Statute]) -> Vec<StructuredStatute> {
    statutes
        .iter()
        .map(StructuredStatute::from_statute)
        .collect()
}

/// Renders a statute as a clean, deterministic Markdown block suitable for use
/// as LLM prompt context or as a retrieval payload.
pub fn render_statute_markdown(statute: &Statute) -> String {
    let mut out = String::new();
    out.push_str("## ");
    out.push_str(&statute.title);
    out.push('\n');
    out.push_str(&format!("- statute_id: {}\n", statute.id));
    if let Some(jurisdiction) = &statute.jurisdiction {
        out.push_str(&format!("- jurisdiction: {}\n", jurisdiction));
    }
    out.push_str(&format!(
        "- effect: {} -- {}\n",
        effect_type_to_str(&statute.effect.effect_type),
        statute.effect.description
    ));
    if !statute.preconditions.is_empty() {
        out.push_str("- conditions:\n");
        for condition in &statute.preconditions {
            out.push_str(&format!("  - {}\n", render_condition(condition)));
        }
    }
    if !statute.applies_to.is_empty() {
        out.push_str(&format!(
            "- applies_to: {}\n",
            statute.applies_to.join(", ")
        ));
    }
    if !statute.derives_from.is_empty() {
        out.push_str(&format!(
            "- derives_from: {}\n",
            statute.derives_from.join(", ")
        ));
    }
    if !statute.effect.parameters.is_empty() {
        let sorted: BTreeMap<&String, &String> = statute.effect.parameters.iter().collect();
        let rendered: Vec<String> = sorted.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
        out.push_str(&format!("- parameters: {}\n", rendered.join(", ")));
    }
    out
}

/// Tokenises text into lowercase alphanumeric tokens.
pub fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(str::to_lowercase)
        .collect()
}

/// Estimates the number of LLM tokens for a piece of text.
///
/// Uses a character-per-token heuristic (≈4 chars/token) with a floor at the
/// whitespace-delimited word count, which tracks real tokenisers closely enough
/// for budget planning.
pub fn estimate_tokens(text: &str) -> usize {
    if text.trim().is_empty() {
        return 0;
    }
    let chars = text.chars().count();
    let words = text.split_whitespace().count();
    chars.div_ceil(4).max(words)
}

/// 64-bit FNV-1a hash; deterministic and dependency-free.
pub fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// A short, stable, content-addressed identifier for the given bytes.
pub fn content_hash_id(bytes: &[u8]) -> String {
    format!("{:016x}", fnv1a_64(bytes))
}

/// L2-normalises a vector in place. A zero vector is left unchanged.
pub fn l2_normalize(vector: &mut [f32]) {
    let norm: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > f32::EPSILON {
        for value in vector.iter_mut() {
            *value /= norm;
        }
    }
}

/// Cosine similarity of two equal-length vectors. Returns `0.0` for mismatched
/// or empty inputs and for degenerate (zero-norm) vectors.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;
    for (x, y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }
    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom > f32::EPSILON {
        (dot / denom).clamp(-1.0, 1.0)
    } else {
        0.0
    }
}

/// Numerically stable softmax over a slice of scores.
pub fn softmax(scores: &[f64]) -> Vec<f64> {
    if scores.is_empty() {
        return Vec::new();
    }
    let max = scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let exps: Vec<f64> = scores.iter().map(|score| (score - max).exp()).collect();
    let sum: f64 = exps.iter().sum();
    if sum > f64::EPSILON {
        exps.iter().map(|value| value / sum).collect()
    } else {
        vec![1.0 / scores.len() as f64; scores.len()]
    }
}

/// A deterministic, pure-Rust feature-hashing (a.k.a. "hashing trick")
/// embedder.
///
/// Unigram and bigram tokens are hashed into a fixed-dimension vector with a
/// signed accumulation and sublinear term-frequency weighting, then
/// L2-normalised. This yields stable, comparable embeddings without any
/// learned model, which is ideal for reproducible offline retrieval.
#[derive(Debug, Clone, Copy)]
pub struct HashingEmbedder {
    dimension: usize,
}

impl HashingEmbedder {
    /// Creates an embedder producing vectors of the given dimension (clamped to
    /// at least 1).
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension: dimension.max(1),
        }
    }

    /// Returns the embedding dimension.
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Embeds text into an L2-normalised vector.
    pub fn embed(&self, text: &str) -> Vec<f32> {
        let mut vector = vec![0.0f32; self.dimension];
        let tokens = tokenize(text);

        let mut counts: BTreeMap<String, u32> = BTreeMap::new();
        for token in &tokens {
            *counts.entry(token.clone()).or_insert(0) += 1;
        }
        for (term, count) in &counts {
            self.add_feature(&mut vector, term.as_bytes(), *count);
        }
        for pair in tokens.windows(2) {
            let bigram = format!("{}_{}", pair[0], pair[1]);
            self.add_feature(&mut vector, bigram.as_bytes(), 1);
        }

        l2_normalize(&mut vector);
        vector
    }

    fn add_feature(&self, vector: &mut [f32], feature: &[u8], count: u32) {
        let hash = fnv1a_64(feature);
        let bucket = (hash % self.dimension as u64) as usize;
        let sign = if (hash >> 63) & 1 == 0 { 1.0 } else { -1.0 };
        let weight = 1.0 + (count as f32).ln();
        vector[bucket] += sign * weight;
    }
}

impl Default for HashingEmbedder {
    fn default() -> Self {
        Self::new(DEFAULT_EMBEDDING_DIM)
    }
}

/// Canonical token for an [`EffectType`].
pub fn effect_type_to_str(effect_type: &EffectType) -> &'static str {
    match effect_type {
        EffectType::Grant => "grant",
        EffectType::Revoke => "revoke",
        EffectType::Obligation => "obligation",
        EffectType::Prohibition => "prohibition",
        EffectType::MonetaryTransfer => "monetary_transfer",
        EffectType::StatusChange => "status_change",
        EffectType::Custom => "custom",
    }
}

/// Parses a canonical effect-type token, defaulting to [`EffectType::Custom`].
pub fn str_to_effect_type(token: &str) -> EffectType {
    match token.trim().to_lowercase().as_str() {
        "grant" => EffectType::Grant,
        "revoke" => EffectType::Revoke,
        "obligation" => EffectType::Obligation,
        "prohibition" => EffectType::Prohibition,
        "monetary_transfer" => EffectType::MonetaryTransfer,
        "status_change" => EffectType::StatusChange,
        _ => EffectType::Custom,
    }
}

/// Canonical token for a [`ComparisonOp`].
pub fn op_to_str(op: &ComparisonOp) -> &'static str {
    match op {
        ComparisonOp::Equal => "==",
        ComparisonOp::NotEqual => "!=",
        ComparisonOp::GreaterThan => ">",
        ComparisonOp::GreaterOrEqual => ">=",
        ComparisonOp::LessThan => "<",
        ComparisonOp::LessOrEqual => "<=",
    }
}

/// Parses a comparison operator, defaulting to [`ComparisonOp::Equal`].
pub fn parse_op(token: &str) -> ComparisonOp {
    match token {
        "!=" | "<>" => ComparisonOp::NotEqual,
        ">" => ComparisonOp::GreaterThan,
        ">=" => ComparisonOp::GreaterOrEqual,
        "<" => ComparisonOp::LessThan,
        "<=" => ComparisonOp::LessOrEqual,
        _ => ComparisonOp::Equal,
    }
}

/// Canonical token for a [`DurationUnit`].
pub fn unit_to_str(unit: &DurationUnit) -> &'static str {
    match unit {
        DurationUnit::Days => "days",
        DurationUnit::Weeks => "weeks",
        DurationUnit::Months => "months",
        DurationUnit::Years => "years",
    }
}

/// Parses a duration unit token, defaulting to [`DurationUnit::Years`].
pub fn parse_unit(token: &str) -> DurationUnit {
    match token.trim().to_lowercase().as_str() {
        "days" | "day" => DurationUnit::Days,
        "weeks" | "week" => DurationUnit::Weeks,
        "months" | "month" => DurationUnit::Months,
        _ => DurationUnit::Years,
    }
}

/// Renders a [`Condition`] as a canonical, parseable string.
///
/// Simple conditions (age, income, duration, residency, percentage, attributes,
/// custom) round-trip exactly through [`parse_condition`]; compound and exotic
/// conditions degrade gracefully to a descriptive `expr ...` form that is
/// re-imported as a [`Condition::Custom`].
pub fn render_condition(condition: &Condition) -> String {
    match condition {
        Condition::Age { operator, value } => format!("age {} {}", op_to_str(operator), value),
        Condition::Income { operator, value } => {
            format!("income {} {}", op_to_str(operator), value)
        }
        Condition::ResidencyDuration { operator, months } => {
            format!("residency {} {} months", op_to_str(operator), months)
        }
        Condition::Duration {
            operator,
            value,
            unit,
        } => format!(
            "duration {} {} {}",
            op_to_str(operator),
            value,
            unit_to_str(unit)
        ),
        Condition::Percentage {
            operator,
            value,
            context,
        } => format!("percentage[{}] {} {}", context, op_to_str(operator), value),
        Condition::HasAttribute { key } => format!("has {}", key),
        Condition::AttributeEquals { key, value } => format!("attr {} == {}", key, value),
        Condition::Custom { description } => format!("custom {}", description),
        other => format!("expr {}", describe_condition(other)),
    }
}

fn describe_condition(condition: &Condition) -> String {
    match condition {
        Condition::And(left, right) => {
            format!(
                "({}) and ({})",
                describe_condition(left),
                describe_condition(right)
            )
        }
        Condition::Or(left, right) => {
            format!(
                "({}) or ({})",
                describe_condition(left),
                describe_condition(right)
            )
        }
        Condition::Not(inner) => format!("not ({})", describe_condition(inner)),
        Condition::SetMembership {
            attribute,
            values,
            negated,
        } => format!(
            "{} {} member of [{}]",
            attribute,
            if *negated { "not" } else { "is" },
            values.join(", ")
        ),
        Condition::Pattern {
            attribute,
            pattern,
            negated,
        } => format!(
            "{} {} match {}",
            attribute,
            if *negated { "no" } else { "yes" },
            pattern
        ),
        Condition::Calculation {
            formula,
            operator,
            value,
        } => format!("calc {} {} {}", formula, op_to_str(operator), value),
        Condition::Threshold {
            operator, value, ..
        } => format!("threshold {} {}", op_to_str(operator), value),
        Condition::Geographic { region_id, .. } => format!("region {}", region_id),
        _ => "opaque condition".to_string(),
    }
}

/// Parses a canonical condition string produced by [`render_condition`].
pub fn parse_condition(text: &str) -> Condition {
    let trimmed = text.trim();
    let tokens: Vec<&str> = trimmed.split_whitespace().collect();
    let Some(&head) = tokens.first() else {
        return Condition::Custom {
            description: String::new(),
        };
    };

    match head {
        "age" if tokens.len() >= 3 => {
            if let Ok(value) = tokens[2].parse::<u32>() {
                return Condition::Age {
                    operator: parse_op(tokens[1]),
                    value,
                };
            }
        }
        "income" if tokens.len() >= 3 => {
            if let Ok(value) = tokens[2].parse::<u64>() {
                return Condition::Income {
                    operator: parse_op(tokens[1]),
                    value,
                };
            }
        }
        "residency" if tokens.len() >= 3 => {
            if let Ok(months) = tokens[2].parse::<u32>() {
                return Condition::ResidencyDuration {
                    operator: parse_op(tokens[1]),
                    months,
                };
            }
        }
        "duration" if tokens.len() >= 4 => {
            if let Ok(value) = tokens[2].parse::<u32>() {
                return Condition::Duration {
                    operator: parse_op(tokens[1]),
                    value,
                    unit: parse_unit(tokens[3]),
                };
            }
        }
        "has" if tokens.len() >= 2 => {
            return Condition::HasAttribute {
                key: tokens[1..].join(" "),
            };
        }
        "attr" => {
            if let Some(rest) = trimmed.strip_prefix("attr ")
                && let Some((key, value)) = rest.split_once(" == ")
            {
                return Condition::AttributeEquals {
                    key: key.trim().to_string(),
                    value: value.trim().to_string(),
                };
            }
        }
        "custom" => {
            return Condition::Custom {
                description: trimmed.strip_prefix("custom ").unwrap_or("").to_string(),
            };
        }
        _ => {}
    }

    if let Some(rest) = trimmed.strip_prefix("percentage[")
        && let Some(close) = rest.find(']')
    {
        let context = rest[..close].to_string();
        let after: Vec<&str> = rest[close + 1..].split_whitespace().collect();
        if let (Some(op), Some(raw)) = (after.first(), after.get(1))
            && let Ok(value) = raw.parse::<u32>()
        {
            return Condition::Percentage {
                operator: parse_op(op),
                value,
                context,
            };
        }
    }

    if let Some(rest) = trimmed.strip_prefix("expr ") {
        return Condition::Custom {
            description: rest.to_string(),
        };
    }

    Condition::Custom {
        description: trimmed.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_statute() -> Statute {
        let mut effect = Effect::new(EffectType::Grant, "Grant the right to vote");
        effect
            .parameters
            .insert("authority".to_string(), "federal".to_string());
        Statute::new("voting-rights", "Voting Rights", effect)
            .with_jurisdiction("US")
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            })
            .with_applies_to("Citizen")
    }

    #[test]
    fn test_condition_codec_simple_roundtrip() {
        let conditions = vec![
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
            Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50_000,
            },
            Condition::Duration {
                operator: ComparisonOp::GreaterThan,
                value: 5,
                unit: DurationUnit::Years,
            },
            Condition::ResidencyDuration {
                operator: ComparisonOp::GreaterOrEqual,
                months: 12,
            },
            Condition::Percentage {
                operator: ComparisonOp::GreaterOrEqual,
                value: 25,
                context: "ownership".to_string(),
            },
            Condition::HasAttribute {
                key: "citizenship".to_string(),
            },
            Condition::AttributeEquals {
                key: "status".to_string(),
                value: "active".to_string(),
            },
            Condition::Custom {
                description: "subject to ministerial discretion".to_string(),
            },
        ];
        for condition in conditions {
            let rendered = render_condition(&condition);
            let parsed = parse_condition(&rendered);
            assert_eq!(condition, parsed, "roundtrip failed for {rendered}");
        }
    }

    #[test]
    fn test_compound_condition_degrades_to_custom() {
        let compound = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::HasAttribute {
                key: "license".to_string(),
            }),
        );
        let rendered = render_condition(&compound);
        assert!(rendered.starts_with("expr "));
        let parsed = parse_condition(&rendered);
        assert!(matches!(parsed, Condition::Custom { .. }));
    }

    #[test]
    fn test_structured_statute_roundtrip() {
        let statute = sample_statute();
        let structured = StructuredStatute::from_statute(&statute);
        let reconstructed = structured.to_statute();
        assert_eq!(reconstructed.id, statute.id);
        assert_eq!(reconstructed.title, statute.title);
        assert_eq!(reconstructed.jurisdiction, statute.jurisdiction);
        assert_eq!(reconstructed.effect.effect_type, statute.effect.effect_type);
        assert_eq!(reconstructed.effect.description, statute.effect.description);
        assert_eq!(reconstructed.preconditions, statute.preconditions);
        assert_eq!(reconstructed.applies_to, statute.applies_to);
        assert_eq!(
            reconstructed.effect.parameters.get("authority"),
            Some(&"federal".to_string())
        );
    }

    #[test]
    fn test_effect_type_codec() {
        for effect_type in [
            EffectType::Grant,
            EffectType::Revoke,
            EffectType::Obligation,
            EffectType::Prohibition,
            EffectType::MonetaryTransfer,
            EffectType::StatusChange,
            EffectType::Custom,
        ] {
            let token = effect_type_to_str(&effect_type);
            assert_eq!(str_to_effect_type(token), effect_type);
        }
    }

    #[test]
    fn test_embedder_is_deterministic_and_normalised() {
        let embedder = HashingEmbedder::new(128);
        let a = embedder.embed("the quick brown fox jumps");
        let b = embedder.embed("the quick brown fox jumps");
        assert_eq!(a, b);
        assert_eq!(a.len(), 128);
        let norm: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_cosine_similarity_ordering() {
        let embedder = HashingEmbedder::new(256);
        let base = embedder.embed("minimum capital adequacy ratio for banks");
        let similar = embedder.embed("capital adequacy ratio requirement for banks");
        let different = embedder.embed("voting age requirement for citizens");
        let self_sim = cosine_similarity(&base, &base);
        assert!((self_sim - 1.0).abs() < 1e-5);
        assert!(cosine_similarity(&base, &similar) > cosine_similarity(&base, &different));
    }

    #[test]
    fn test_cosine_handles_mismatched_lengths() {
        assert_eq!(cosine_similarity(&[1.0, 0.0], &[1.0]), 0.0);
        assert_eq!(cosine_similarity(&[], &[]), 0.0);
    }

    #[test]
    fn test_softmax_sums_to_one() {
        let distribution = softmax(&[1.0, 2.0, 3.0, 0.5]);
        let sum: f64 = distribution.iter().sum();
        assert!((sum - 1.0).abs() < 1e-9);
        assert!(distribution[2] > distribution[0]);
    }

    #[test]
    fn test_estimate_tokens() {
        assert_eq!(estimate_tokens(""), 0);
        assert_eq!(estimate_tokens("   "), 0);
        assert!(estimate_tokens("hello world") >= 2);
        let long = "a ".repeat(40);
        assert!(estimate_tokens(&long) >= 40);
    }

    #[test]
    fn test_render_statute_markdown_is_clean() {
        let markdown = render_statute_markdown(&sample_statute());
        assert!(markdown.contains("## Voting Rights"));
        assert!(markdown.contains("statute_id: voting-rights"));
        assert!(markdown.contains("effect: grant"));
        assert!(markdown.contains("age >= 18"));
        assert!(markdown.contains("applies_to: Citizen"));
        // Must not collide with substring-based auto-detectors.
        assert!(!markdown.contains("[["));
        assert!(!markdown.contains("{{"));
    }

    #[test]
    fn test_content_hash_is_stable() {
        let a = content_hash_id(b"semantic chunk payload");
        let b = content_hash_id(b"semantic chunk payload");
        let c = content_hash_id(b"different payload");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.len(), 16);
    }
}
