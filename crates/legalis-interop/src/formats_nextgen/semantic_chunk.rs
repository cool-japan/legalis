//! Semantic chunk format.
//!
//! Splits a statute corpus into overlap-controlled, RAG-oriented chunks. The
//! chunker walks the per-statute Markdown line by line, packs lines up to a
//! target token size, prefers to start a fresh chunk at statute boundaries
//! (for semantic coherence), and seeds each new chunk with a token-bounded
//! overlap copied from the tail of the previous one (so context isn't lost at
//! chunk borders). Every chunk gets a stable, content-addressed identifier.
//!
//! Alongside the chunks, the document stores a structured provenance list so
//! the original statutes can be reconstructed losslessly on import.

use super::{
    StructuredStatute, build_structured, content_hash_id, estimate_tokens, render_statute_markdown,
};
use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Schema identifier for the semantic chunk format.
pub const SCHEMA: &str = "legalis.semantic-chunk/v1";

/// Chunking configuration.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SemanticChunkConfig {
    /// Preferred chunk size; statute boundaries past this size start a new
    /// chunk.
    pub target_tokens: usize,
    /// Hard upper bound on content tokens before a forced split.
    pub max_tokens: usize,
    /// Token budget for the overlap prefix carried into each new chunk.
    pub overlap_tokens: usize,
}

impl Default for SemanticChunkConfig {
    fn default() -> Self {
        Self {
            target_tokens: 180,
            max_tokens: 256,
            overlap_tokens: 32,
        }
    }
}

impl SemanticChunkConfig {
    /// Returns a sanitised copy with consistent, non-degenerate bounds.
    fn sanitized(self) -> Self {
        let max_tokens = self.max_tokens.max(1);
        let target_tokens = self.target_tokens.clamp(1, max_tokens);
        let overlap_tokens = self.overlap_tokens.min(max_tokens.saturating_sub(1));
        Self {
            target_tokens,
            max_tokens,
            overlap_tokens,
        }
    }
}

/// A chunked document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkDocument {
    /// Schema identifier ([`SCHEMA`]).
    pub schema: String,
    /// Chunking configuration used.
    pub config: SemanticChunkConfig,
    /// Generated chunks in order.
    pub chunks: Vec<SemanticChunk>,
    /// Structured provenance enabling lossless reconstruction.
    pub provenance: Vec<StructuredStatute>,
}

/// A single semantic chunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticChunk {
    /// Stable, content-addressed identifier.
    pub id: String,
    /// Zero-based position in the document.
    pub ordinal: usize,
    /// Source statute identifiers contributing to the chunk (in order).
    pub source_ids: Vec<String>,
    /// Chunk text.
    pub text: String,
    /// Estimated token count of the chunk text.
    pub token_count: usize,
    /// Tokens at the start of the chunk copied as overlap from the previous one.
    pub overlap_prefix_tokens: usize,
    /// Arbitrary chunk metadata.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,
}

#[derive(Default)]
struct ChunkBuilder {
    lines: Vec<(String, String)>,
    sources: Vec<String>,
    tokens: usize,
    overlap_prefix_tokens: usize,
    content_lines: usize,
    last_source: Option<String>,
}

impl ChunkBuilder {
    fn has_content(&self) -> bool {
        self.content_lines > 0
    }

    fn note_source(&mut self, source: &str) {
        if !self.sources.iter().any(|existing| existing == source) {
            self.sources.push(source.to_string());
        }
        self.last_source = Some(source.to_string());
    }

    fn add_content_line(&mut self, source: &str, text: &str) {
        self.tokens += estimate_tokens(text);
        self.lines.push((source.to_string(), text.to_string()));
        self.content_lines += 1;
        self.note_source(source);
    }

    fn add_overlap_line(&mut self, source: &str, text: &str) {
        let cost = estimate_tokens(text);
        self.tokens += cost;
        self.overlap_prefix_tokens += cost;
        self.lines.push((source.to_string(), text.to_string()));
        self.note_source(source);
    }

    fn text(&self) -> String {
        self.lines
            .iter()
            .map(|(_, text)| text.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn finish(self, ordinal: usize) -> SemanticChunk {
        let text = self.text();
        let id = format!("chunk-{:04}-{}", ordinal, content_hash_id(text.as_bytes()));
        let mut metadata = BTreeMap::new();
        metadata.insert("source_count".to_string(), self.sources.len().to_string());
        metadata.insert("line_count".to_string(), self.lines.len().to_string());
        SemanticChunk {
            id,
            ordinal,
            source_ids: self.sources,
            token_count: self.tokens,
            overlap_prefix_tokens: self.overlap_prefix_tokens,
            text,
            metadata,
        }
    }
}

/// Extracts the trailing lines of a finished chunk within an overlap token
/// budget, preserving their original order.
fn overlap_tail(lines: &[(String, String)], overlap_tokens: usize) -> Vec<(String, String)> {
    if overlap_tokens == 0 {
        return Vec::new();
    }
    let mut accumulated = 0usize;
    let mut tail: Vec<(String, String)> = Vec::new();
    for (source, text) in lines.iter().rev() {
        let cost = estimate_tokens(text);
        if !tail.is_empty() && accumulated + cost > overlap_tokens {
            break;
        }
        accumulated += cost;
        tail.push((source.clone(), text.clone()));
        if accumulated >= overlap_tokens {
            break;
        }
    }
    tail.reverse();
    tail
}

impl ChunkDocument {
    /// Builds a chunked document from statutes using the given configuration.
    pub fn build(statutes: &[Statute], config: SemanticChunkConfig) -> Self {
        let config = config.sanitized();
        let mut chunks: Vec<SemanticChunk> = Vec::new();
        let mut current = ChunkBuilder::default();

        for statute in statutes {
            let block = render_statute_markdown(statute);
            for line in block.lines().filter(|line| !line.trim().is_empty()) {
                let line_tokens = estimate_tokens(line);
                let crosses_boundary = current
                    .last_source
                    .as_deref()
                    .is_some_and(|last| last != statute.id.as_str());
                let exceeds_max = current.tokens + line_tokens > config.max_tokens;
                let boundary_after_target =
                    crosses_boundary && current.tokens >= config.target_tokens;

                if current.has_content() && (exceeds_max || boundary_after_target) {
                    let tail = overlap_tail(&current.lines, config.overlap_tokens);
                    let ordinal = chunks.len();
                    chunks.push(std::mem::take(&mut current).finish(ordinal));
                    for (source, text) in &tail {
                        current.add_overlap_line(source, text);
                    }
                }

                current.add_content_line(&statute.id, line);
            }
        }

        if current.has_content() {
            let ordinal = chunks.len();
            chunks.push(current.finish(ordinal));
        }

        Self {
            schema: SCHEMA.to_string(),
            config,
            chunks,
            provenance: build_structured(statutes),
        }
    }

    /// Number of chunks.
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    /// Reconstructs the underlying statutes from provenance.
    pub fn to_statutes(&self) -> Vec<Statute> {
        self.provenance
            .iter()
            .map(StructuredStatute::to_statute)
            .collect()
    }

    /// Serialises the document to pretty JSON.
    pub fn to_json(&self) -> InteropResult<String> {
        serde_json::to_string_pretty(self).map_err(|error| {
            InteropError::SerializationError(format!("Failed to serialize chunk doc: {error}"))
        })
    }

    /// Parses a document from JSON.
    pub fn from_json(source: &str) -> InteropResult<Self> {
        serde_json::from_str(source).map_err(|error| {
            InteropError::ParseError(format!("Failed to parse chunk JSON: {error}"))
        })
    }
}

/// Importer for the semantic chunk format.
#[derive(Debug, Default)]
pub struct SemanticChunkImporter;

impl SemanticChunkImporter {
    /// Creates a new importer.
    pub fn new() -> Self {
        Self
    }
}

impl FormatImporter for SemanticChunkImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::SemanticChunk
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let document = ChunkDocument::from_json(source)?;
        let statutes = document.to_statutes();
        let mut report = ConversionReport::new(LegalFormat::SemanticChunk, LegalFormat::Legalis);
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

/// Exporter for the semantic chunk format.
#[derive(Debug, Clone, Copy)]
pub struct SemanticChunkExporter {
    config: SemanticChunkConfig,
}

impl SemanticChunkExporter {
    /// Creates an exporter with default chunking configuration.
    pub fn new() -> Self {
        Self {
            config: SemanticChunkConfig::default(),
        }
    }

    /// Sets the chunking configuration.
    pub fn with_config(mut self, config: SemanticChunkConfig) -> Self {
        self.config = config;
        self
    }
}

impl Default for SemanticChunkExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for SemanticChunkExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::SemanticChunk
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let document = ChunkDocument::build(statutes, self.config);
        let json = document.to_json()?;
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::SemanticChunk);
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
        (0..4u32)
            .map(|index| {
                Statute::new(
                    format!("statute-{index}"),
                    format!("Provision Number {index}"),
                    Effect::new(
                        EffectType::Obligation,
                        format!("Comply with regulatory requirement number {index}"),
                    ),
                )
                .with_precondition(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18 + index,
                })
                .with_jurisdiction("US")
            })
            .collect()
    }

    fn small_config() -> SemanticChunkConfig {
        SemanticChunkConfig {
            target_tokens: 20,
            max_tokens: 40,
            overlap_tokens: 10,
        }
    }

    #[test]
    fn test_chunking_produces_multiple_chunks() {
        let document = ChunkDocument::build(&statutes(), small_config());
        assert!(document.chunk_count() >= 2);
        for (index, chunk) in document.chunks.iter().enumerate() {
            assert_eq!(chunk.ordinal, index);
            assert!(!chunk.text.is_empty());
        }
    }

    #[test]
    fn test_chunks_respect_token_budget() {
        let config = small_config();
        let document = ChunkDocument::build(&statutes(), config);
        for chunk in &document.chunks {
            // Content stays under max plus the carried overlap budget.
            assert!(chunk.token_count <= config.max_tokens + config.overlap_tokens);
        }
    }

    #[test]
    fn test_overlap_recorded_after_first_chunk() {
        let document = ChunkDocument::build(&statutes(), small_config());
        // At least one later chunk must carry an overlap prefix.
        let has_overlap = document
            .chunks
            .iter()
            .skip(1)
            .any(|chunk| chunk.overlap_prefix_tokens > 0);
        assert!(has_overlap);
    }

    #[test]
    fn test_stable_ids() {
        let first = ChunkDocument::build(&statutes(), small_config());
        let second = ChunkDocument::build(&statutes(), small_config());
        let first_ids: Vec<&str> = first.chunks.iter().map(|c| c.id.as_str()).collect();
        let second_ids: Vec<&str> = second.chunks.iter().map(|c| c.id.as_str()).collect();
        assert_eq!(first_ids, second_ids);
        assert!(first.chunks[0].id.starts_with("chunk-0000-"));
    }

    #[test]
    fn test_source_attribution() {
        let document = ChunkDocument::build(&statutes(), small_config());
        let first = &document.chunks[0];
        assert!(first.source_ids.contains(&"statute-0".to_string()));
        let expected_count = first.source_ids.len().to_string();
        assert_eq!(first.metadata.get("source_count"), Some(&expected_count));
    }

    #[test]
    fn test_export_import_roundtrip() {
        let exporter = SemanticChunkExporter::new().with_config(small_config());
        let importer = SemanticChunkImporter::new();
        let (json, export_report) = exporter.export(&statutes()).unwrap();
        assert_eq!(export_report.statutes_converted, 4);

        let (imported, import_report) = importer.import(&json).unwrap();
        assert_eq!(import_report.statutes_converted, 4);
        assert_eq!(imported.len(), 4);
        assert_eq!(imported[0].id, "statute-0");
        assert_eq!(imported[3].jurisdiction.as_deref(), Some("US"));
        assert_eq!(imported[2].preconditions.len(), 1);
    }

    #[test]
    fn test_validate() {
        let importer = SemanticChunkImporter::new();
        let (json, _) = SemanticChunkExporter::new().export(&statutes()).unwrap();
        assert!(importer.validate(&json));
        assert!(!importer.validate("{\"schema\":\"legalis.attention-markup/v1\"}"));
        assert!(!importer.validate("{}"));
    }

    #[test]
    fn test_default_config_single_chunk_for_small_corpus() {
        let document = ChunkDocument::build(&statutes(), SemanticChunkConfig::default());
        // The whole small corpus fits comfortably in one default-sized chunk.
        assert_eq!(document.chunk_count(), 1);
        assert_eq!(document.chunks[0].overlap_prefix_tokens, 0);
    }
}
