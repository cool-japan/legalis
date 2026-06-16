//! Neural legal document format.
//!
//! Models a statute corpus as an activation graph. Nodes are statutes; edges
//! are typed cross-references — semantic-similarity links derived from the
//! [`HashingEmbedder`] plus explicit derivation links. Node salience is the
//! stationary distribution of a weighted PageRank over the similarity
//! adjacency, and a sigmoid maps salience to an `activation` value, giving each
//! statute a centrality-aware importance signal.
//!
//! This is deliberately a graph/"neural" view rather than a raw embedding store
//! (see [`super::embedding`]): it captures *relationships* between provisions,
//! which is what downstream graph-attention or message-passing models consume.

use super::{HashingEmbedder, StructuredStatute, cosine_similarity, render_statute_markdown};
use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};

/// Schema identifier for the neural document format.
pub const SCHEMA: &str = "legalis.neural-document/v1";

/// Default minimum cosine similarity for an edge to be materialised.
pub const DEFAULT_SIMILARITY_THRESHOLD: f64 = 0.05;

/// A neural document: an activation graph over a statute corpus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralDocument {
    /// Schema identifier ([`SCHEMA`]).
    pub schema: String,
    /// Hint describing the representation lineage.
    pub model_hint: String,
    /// Embedding dimension used to build the similarity graph.
    pub dimension: usize,
    /// Minimum similarity threshold used for edges.
    pub similarity_threshold: f64,
    /// Graph nodes (one per statute).
    pub nodes: Vec<NeuralNode>,
    /// Typed, weighted cross-reference edges.
    pub edges: Vec<NeuralEdge>,
}

/// A neural graph node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralNode {
    /// Statute identifier.
    pub id: String,
    /// Statute title.
    pub title: String,
    /// PageRank centrality (the node salience; sums to ~1 across the graph).
    pub salience: f64,
    /// Sigmoid-mapped activation derived from salience.
    pub activation: f64,
    /// Structured provenance enabling lossless reconstruction.
    pub provenance: StructuredStatute,
}

/// A typed, weighted edge between two statutes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NeuralEdge {
    /// Source statute identifier.
    pub source: String,
    /// Target statute identifier.
    pub target: String,
    /// Relation kind (`semantic_similarity` or `derives_from`).
    pub relation: String,
    /// Edge weight.
    pub weight: f64,
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// Computes a weighted PageRank stationary distribution over a (square,
/// non-negative) adjacency matrix, handling dangling nodes by uniform
/// redistribution. The result is normalised to sum to 1.
fn weighted_pagerank(adjacency: &[Vec<f64>], damping: f64, max_iter: usize, tol: f64) -> Vec<f64> {
    let n = adjacency.len();
    if n == 0 {
        return Vec::new();
    }
    let mut rank = vec![1.0 / n as f64; n];
    let out_weights: Vec<f64> = adjacency
        .iter()
        .map(|row| row.iter().sum::<f64>())
        .collect();
    let teleport = (1.0 - damping) / n as f64;

    for _ in 0..max_iter {
        let dangling_mass: f64 = rank
            .iter()
            .zip(out_weights.iter())
            .filter(|&(_, out)| *out <= f64::EPSILON)
            .map(|(value, _)| *value)
            .sum();
        let dangling_share = damping * dangling_mass / n as f64;

        let mut next = vec![teleport + dangling_share; n];
        for (i, row) in adjacency.iter().enumerate() {
            if out_weights[i] <= f64::EPSILON {
                continue;
            }
            let share = damping * rank[i] / out_weights[i];
            for (j, &weight) in row.iter().enumerate() {
                if weight > 0.0 {
                    next[j] += share * weight;
                }
            }
        }

        let delta: f64 = next
            .iter()
            .zip(rank.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        rank = next;
        if delta < tol {
            break;
        }
    }

    let sum: f64 = rank.iter().sum();
    if sum > f64::EPSILON {
        for value in rank.iter_mut() {
            *value /= sum;
        }
    }
    rank
}

impl NeuralDocument {
    /// Builds a neural document over the statutes.
    pub fn build(statutes: &[Statute], dimension: usize, similarity_threshold: f64) -> Self {
        let embedder = HashingEmbedder::new(dimension);
        let embeddings: Vec<Vec<f32>> = statutes
            .iter()
            .map(|statute| embedder.embed(&render_statute_markdown(statute)))
            .collect();

        let n = statutes.len();
        let mut adjacency = vec![vec![0.0f64; n]; n];
        let mut edges = Vec::new();

        for i in 0..n {
            for j in (i + 1)..n {
                let similarity = f64::from(cosine_similarity(&embeddings[i], &embeddings[j]));
                if similarity >= similarity_threshold {
                    adjacency[i][j] = similarity;
                    adjacency[j][i] = similarity;
                    edges.push(NeuralEdge {
                        source: statutes[i].id.clone(),
                        target: statutes[j].id.clone(),
                        relation: "semantic_similarity".to_string(),
                        weight: similarity,
                    });
                }
            }
        }

        for statute in statutes {
            for source in &statute.derives_from {
                edges.push(NeuralEdge {
                    source: source.clone(),
                    target: statute.id.clone(),
                    relation: "derives_from".to_string(),
                    weight: 1.0,
                });
            }
        }

        let salience = weighted_pagerank(&adjacency, 0.85, 100, 1e-9);
        let nodes = statutes
            .iter()
            .enumerate()
            .map(|(index, statute)| {
                let node_salience = salience.get(index).copied().unwrap_or(0.0);
                NeuralNode {
                    id: statute.id.clone(),
                    title: statute.title.clone(),
                    salience: node_salience,
                    activation: sigmoid(4.0 * (node_salience * n.max(1) as f64 - 1.0)),
                    provenance: StructuredStatute::from_statute(statute),
                }
            })
            .collect();

        Self {
            schema: SCHEMA.to_string(),
            model_hint: "pagerank-over-hashing-embedder".to_string(),
            dimension: embedder.dimension(),
            similarity_threshold,
            nodes,
            edges,
        }
    }

    /// Returns the most salient node, if any.
    pub fn most_salient(&self) -> Option<&NeuralNode> {
        self.nodes
            .iter()
            .max_by(|a, b| a.salience.total_cmp(&b.salience))
    }

    /// Returns the edges incident to a node identifier.
    pub fn neighbors(&self, id: &str) -> Vec<&NeuralEdge> {
        self.edges
            .iter()
            .filter(|edge| edge.source == id || edge.target == id)
            .collect()
    }

    /// Reconstructs the underlying statutes.
    pub fn to_statutes(&self) -> Vec<Statute> {
        self.nodes
            .iter()
            .map(|node| node.provenance.to_statute())
            .collect()
    }

    /// Serialises the document to pretty JSON.
    pub fn to_json(&self) -> InteropResult<String> {
        serde_json::to_string_pretty(self).map_err(|error| {
            InteropError::SerializationError(format!("Failed to serialize neural doc: {error}"))
        })
    }

    /// Parses a document from JSON.
    pub fn from_json(source: &str) -> InteropResult<Self> {
        serde_json::from_str(source).map_err(|error| {
            InteropError::ParseError(format!("Failed to parse neural JSON: {error}"))
        })
    }
}

/// Importer for the neural document format.
#[derive(Debug, Default)]
pub struct NeuralDocumentImporter;

impl NeuralDocumentImporter {
    /// Creates a new importer.
    pub fn new() -> Self {
        Self
    }
}

impl FormatImporter for NeuralDocumentImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::NeuralDocument
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let document = NeuralDocument::from_json(source)?;
        let statutes = document.to_statutes();
        let mut report = ConversionReport::new(LegalFormat::NeuralDocument, LegalFormat::Legalis);
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

/// Exporter for the neural document format.
#[derive(Debug, Clone, Copy)]
pub struct NeuralDocumentExporter {
    dimension: usize,
    similarity_threshold: f64,
}

impl NeuralDocumentExporter {
    /// Creates an exporter with default settings.
    pub fn new() -> Self {
        Self {
            dimension: super::DEFAULT_EMBEDDING_DIM,
            similarity_threshold: DEFAULT_SIMILARITY_THRESHOLD,
        }
    }

    /// Sets the embedding dimension.
    pub fn with_dimension(mut self, dimension: usize) -> Self {
        self.dimension = dimension.max(1);
        self
    }

    /// Sets the similarity threshold for edge creation.
    pub fn with_similarity_threshold(mut self, threshold: f64) -> Self {
        self.similarity_threshold = threshold;
        self
    }
}

impl Default for NeuralDocumentExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for NeuralDocumentExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::NeuralDocument
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let document = NeuralDocument::build(statutes, self.dimension, self.similarity_threshold);
        let json = document.to_json()?;
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::NeuralDocument);
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
    use legalis_core::{Effect, EffectType};

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
                "capital-buffer",
                "Bank Capital Conservation Buffer",
                Effect::new(
                    EffectType::Obligation,
                    "Maintain a capital conservation buffer above the minimum ratio for the bank",
                ),
            )
            .with_derives_from("capital-ratio"),
            Statute::new(
                "voting-age",
                "Voting Age",
                Effect::new(EffectType::Grant, "Grant the right to vote to citizens"),
            ),
        ]
    }

    #[test]
    fn test_salience_distribution_sums_to_one() {
        let document = NeuralDocument::build(&statutes(), 256, DEFAULT_SIMILARITY_THRESHOLD);
        let sum: f64 = document.nodes.iter().map(|node| node.salience).sum();
        assert!((sum - 1.0).abs() < 1e-6);
        for node in &document.nodes {
            assert!(node.activation >= 0.0 && node.activation <= 1.0);
        }
    }

    #[test]
    fn test_connected_nodes_outrank_isolated() {
        let document = NeuralDocument::build(&statutes(), 256, DEFAULT_SIMILARITY_THRESHOLD);
        let capital = document
            .nodes
            .iter()
            .find(|node| node.id == "capital-ratio")
            .expect("capital node");
        let voting = document
            .nodes
            .iter()
            .find(|node| node.id == "voting-age")
            .expect("voting node");
        assert!(capital.salience > voting.salience);
    }

    #[test]
    fn test_semantic_similarity_edges_created() {
        let document = NeuralDocument::build(&statutes(), 256, DEFAULT_SIMILARITY_THRESHOLD);
        let has_similarity = document
            .edges
            .iter()
            .any(|edge| edge.relation == "semantic_similarity");
        assert!(has_similarity);
    }

    #[test]
    fn test_derivation_edges_preserved() {
        let document = NeuralDocument::build(&statutes(), 256, DEFAULT_SIMILARITY_THRESHOLD);
        let derivation = document
            .edges
            .iter()
            .find(|edge| edge.relation == "derives_from")
            .expect("derivation edge");
        assert_eq!(derivation.source, "capital-ratio");
        assert_eq!(derivation.target, "capital-buffer");
    }

    #[test]
    fn test_neighbors_lookup() {
        let document = NeuralDocument::build(&statutes(), 256, DEFAULT_SIMILARITY_THRESHOLD);
        let neighbors = document.neighbors("capital-ratio");
        assert!(!neighbors.is_empty());
    }

    #[test]
    fn test_export_import_roundtrip() {
        let exporter = NeuralDocumentExporter::new().with_dimension(128);
        let importer = NeuralDocumentImporter::new();
        let (json, export_report) = exporter.export(&statutes()).unwrap();
        assert_eq!(export_report.statutes_converted, 3);

        let (imported, import_report) = importer.import(&json).unwrap();
        assert_eq!(import_report.statutes_converted, 3);
        assert_eq!(imported.len(), 3);
        let buffer = imported
            .iter()
            .find(|s| s.id == "capital-buffer")
            .expect("buffer statute present");
        assert_eq!(buffer.derives_from, vec!["capital-ratio".to_string()]);
    }

    #[test]
    fn test_validate() {
        let importer = NeuralDocumentImporter::new();
        let (json, _) = NeuralDocumentExporter::new().export(&statutes()).unwrap();
        assert!(importer.validate(&json));
        assert!(!importer.validate("{\"schema\":\"legalis.embedding/v1\"}"));
    }

    #[test]
    fn test_empty_corpus() {
        let document = NeuralDocument::build(&[], 64, DEFAULT_SIMILARITY_THRESHOLD);
        assert!(document.nodes.is_empty());
        assert!(document.edges.is_empty());
        assert!(document.most_salient().is_none());
    }
}
