//! Embedding-based legal format.
//!
//! Each statute is rendered to text and paired with a fixed-dimension float
//! embedding vector produced by the deterministic [`HashingEmbedder`]. The
//! resulting document supports cosine-similarity retrieval entirely offline —
//! there is no learned model and no external service, so results are perfectly
//! reproducible.
//!
//! The format is the natural backbone of a pure-Rust retrieval-augmented
//! generation (RAG) store over a corpus of statutes.

use super::{
    EMBEDDER_MODEL_ID, HashingEmbedder, StructuredStatute, cosine_similarity,
    render_statute_markdown,
};
use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};

/// Schema identifier for the embedding format.
pub const SCHEMA: &str = "legalis.embedding/v1";

/// An embedding store: one vector per statute plus retrieval metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingDocument {
    /// Schema identifier ([`SCHEMA`]).
    pub schema: String,
    /// Embedder model identifier ([`EMBEDDER_MODEL_ID`]).
    pub embedder: String,
    /// Embedding dimension.
    pub dimension: usize,
    /// Whether vectors are L2-normalised (always `true` here).
    pub normalized: bool,
    /// Embedding records.
    pub records: Vec<EmbeddingRecord>,
}

/// A single embedded statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRecord {
    /// Statute identifier.
    pub id: String,
    /// Statute title.
    pub title: String,
    /// Text that was embedded (the retrieval payload).
    pub text: String,
    /// The embedding vector.
    pub embedding: Vec<f32>,
    /// Structured provenance enabling lossless reconstruction.
    pub provenance: StructuredStatute,
}

/// A single retrieval result.
#[derive(Debug, Clone, PartialEq)]
pub struct RetrievalHit {
    /// Matched statute identifier.
    pub id: String,
    /// Matched statute title.
    pub title: String,
    /// Cosine similarity score in `[-1.0, 1.0]`.
    pub score: f32,
}

impl EmbeddingDocument {
    /// Builds an embedding document over the statutes using the given
    /// dimension.
    pub fn build(statutes: &[Statute], dimension: usize) -> Self {
        let embedder = HashingEmbedder::new(dimension);
        let records = statutes
            .iter()
            .map(|statute| {
                let text = render_statute_markdown(statute);
                let embedding = embedder.embed(&text);
                EmbeddingRecord {
                    id: statute.id.clone(),
                    title: statute.title.clone(),
                    text,
                    embedding,
                    provenance: StructuredStatute::from_statute(statute),
                }
            })
            .collect();

        Self {
            schema: SCHEMA.to_string(),
            embedder: EMBEDDER_MODEL_ID.to_string(),
            dimension: embedder.dimension(),
            normalized: true,
            records,
        }
    }

    /// Retrieves the `top_k` records most similar to a free-text query, ranked
    /// by descending cosine similarity (ties broken by identifier).
    pub fn search(&self, query: &str, top_k: usize) -> Vec<RetrievalHit> {
        let embedder = HashingEmbedder::new(self.dimension);
        let query_vector = embedder.embed(query);
        let mut hits: Vec<RetrievalHit> = self
            .records
            .iter()
            .map(|record| RetrievalHit {
                id: record.id.clone(),
                title: record.title.clone(),
                score: cosine_similarity(&query_vector, &record.embedding),
            })
            .collect();
        hits.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.id.cmp(&b.id)));
        hits.truncate(top_k);
        hits
    }

    /// Reconstructs the underlying statutes.
    pub fn to_statutes(&self) -> Vec<Statute> {
        self.records
            .iter()
            .map(|record| record.provenance.to_statute())
            .collect()
    }

    /// Serialises the document to pretty JSON.
    pub fn to_json(&self) -> InteropResult<String> {
        serde_json::to_string_pretty(self).map_err(|error| {
            InteropError::SerializationError(format!("Failed to serialize embedding doc: {error}"))
        })
    }

    /// Parses a document from JSON.
    pub fn from_json(source: &str) -> InteropResult<Self> {
        serde_json::from_str(source).map_err(|error| {
            InteropError::ParseError(format!("Failed to parse embedding JSON: {error}"))
        })
    }
}

/// Importer for the embedding format.
#[derive(Debug, Default)]
pub struct EmbeddingImporter;

impl EmbeddingImporter {
    /// Creates a new importer.
    pub fn new() -> Self {
        Self
    }
}

impl FormatImporter for EmbeddingImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Embedding
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let document = EmbeddingDocument::from_json(source)?;
        let statutes = document.to_statutes();
        let mut report = ConversionReport::new(LegalFormat::Embedding, LegalFormat::Legalis);
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

/// Exporter for the embedding format.
#[derive(Debug, Clone, Copy)]
pub struct EmbeddingExporter {
    dimension: usize,
}

impl EmbeddingExporter {
    /// Creates an exporter with the default embedding dimension.
    pub fn new() -> Self {
        Self {
            dimension: super::DEFAULT_EMBEDDING_DIM,
        }
    }

    /// Sets the embedding dimension.
    pub fn with_dimension(mut self, dimension: usize) -> Self {
        self.dimension = dimension.max(1);
        self
    }
}

impl Default for EmbeddingExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for EmbeddingExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Embedding
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let document = EmbeddingDocument::build(statutes, self.dimension);
        let json = document.to_json()?;
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Embedding);
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
                "capital-ratio",
                "Bank Capital Adequacy Ratio",
                Effect::new(
                    EffectType::Obligation,
                    "Maintain a minimum capital adequacy ratio for the bank",
                ),
            ),
            Statute::new(
                "voting-age",
                "Voting Age",
                Effect::new(EffectType::Grant, "Grant the right to vote to citizens"),
            )
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Statute::new(
                "liquidity-coverage",
                "Liquidity Coverage Requirement",
                Effect::new(
                    EffectType::Obligation,
                    "Hold sufficient liquid assets to cover bank outflows",
                ),
            ),
        ]
    }

    #[test]
    fn test_build_has_consistent_dimension() {
        let document = EmbeddingDocument::build(&statutes(), 64);
        assert_eq!(document.dimension, 64);
        assert_eq!(document.records.len(), 3);
        for record in &document.records {
            assert_eq!(record.embedding.len(), 64);
            let norm: f32 = record.embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
            assert!((norm - 1.0).abs() < 1e-4);
        }
    }

    #[test]
    fn test_search_ranks_relevant_first() {
        let document = EmbeddingDocument::build(&statutes(), 256);
        let hits = document.search("bank capital ratio requirement", 3);
        assert_eq!(hits.len(), 3);
        assert_eq!(hits[0].id, "capital-ratio");
        // The unrelated voting statute should not be the top match.
        assert_ne!(hits[0].id, "voting-age");
    }

    #[test]
    fn test_search_top_k_truncates() {
        let document = EmbeddingDocument::build(&statutes(), 128);
        let hits = document.search("liquidity assets", 1);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].id, "liquidity-coverage");
    }

    #[test]
    fn test_export_import_roundtrip() {
        let exporter = EmbeddingExporter::new().with_dimension(96);
        let importer = EmbeddingImporter::new();
        let (json, export_report) = exporter.export(&statutes()).unwrap();
        assert_eq!(export_report.statutes_converted, 3);

        let (imported, import_report) = importer.import(&json).unwrap();
        assert_eq!(import_report.statutes_converted, 3);
        assert_eq!(imported.len(), 3);
        let voting = imported
            .iter()
            .find(|s| s.id == "voting-age")
            .expect("voting statute present");
        assert_eq!(voting.preconditions.len(), 1);
    }

    #[test]
    fn test_validate() {
        let importer = EmbeddingImporter::new();
        let (json, _) = EmbeddingExporter::new().export(&statutes()).unwrap();
        assert!(importer.validate(&json));
        assert!(!importer.validate("{\"schema\":\"legalis.llm-native/v1\"}"));
        assert!(!importer.validate("[]"));
    }

    #[test]
    fn test_embedder_metadata_recorded() {
        let document = EmbeddingDocument::build(&statutes(), 32);
        assert_eq!(document.embedder, EMBEDDER_MODEL_ID);
        assert!(document.normalized);
    }
}
