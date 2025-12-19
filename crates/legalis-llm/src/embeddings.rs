//! Embedding generation and vector operations.
//!
//! This module provides support for generating embeddings from text,
//! useful for semantic search, clustering, and similarity operations.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// A vector embedding representation of text.
#[derive(Debug, Clone, PartialEq)]
pub struct Embedding {
    /// The embedding vector
    pub vector: Vec<f32>,
    /// The dimensionality of the embedding
    pub dimensions: usize,
}

impl Embedding {
    /// Creates a new embedding from a vector.
    pub fn new(vector: Vec<f32>) -> Self {
        let dimensions = vector.len();
        Self { vector, dimensions }
    }

    /// Computes cosine similarity with another embedding.
    /// Returns a value between -1.0 and 1.0, where 1.0 is most similar.
    pub fn cosine_similarity(&self, other: &Embedding) -> Result<f32> {
        if self.dimensions != other.dimensions {
            anyhow::bail!(
                "Dimension mismatch: {} vs {}",
                self.dimensions,
                other.dimensions
            );
        }

        let dot_product: f32 = self
            .vector
            .iter()
            .zip(&other.vector)
            .map(|(a, b)| a * b)
            .sum();

        let norm_a: f32 = self.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = other.vector.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return Ok(0.0);
        }

        Ok(dot_product / (norm_a * norm_b))
    }

    /// Computes Euclidean distance to another embedding.
    pub fn euclidean_distance(&self, other: &Embedding) -> Result<f32> {
        if self.dimensions != other.dimensions {
            anyhow::bail!(
                "Dimension mismatch: {} vs {}",
                self.dimensions,
                other.dimensions
            );
        }

        let distance: f32 = self
            .vector
            .iter()
            .zip(&other.vector)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt();

        Ok(distance)
    }

    /// Computes dot product with another embedding.
    pub fn dot_product(&self, other: &Embedding) -> Result<f32> {
        if self.dimensions != other.dimensions {
            anyhow::bail!(
                "Dimension mismatch: {} vs {}",
                self.dimensions,
                other.dimensions
            );
        }

        Ok(self
            .vector
            .iter()
            .zip(&other.vector)
            .map(|(a, b)| a * b)
            .sum())
    }

    /// Normalizes the embedding to unit length.
    pub fn normalize(&mut self) {
        let norm: f32 = self.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for v in &mut self.vector {
                *v /= norm;
            }
        }
    }

    /// Returns a normalized copy of this embedding.
    pub fn normalized(&self) -> Self {
        let mut copy = self.clone();
        copy.normalize();
        copy
    }
}

/// Trait for embedding providers.
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    /// Generates an embedding for a single text input.
    async fn embed(&self, text: &str) -> Result<Embedding>;

    /// Generates embeddings for multiple text inputs.
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Embedding>>;

    /// Returns the name of the embedding provider.
    fn provider_name(&self) -> &str;

    /// Returns the model being used.
    fn model_name(&self) -> &str;

    /// Returns the dimensionality of embeddings produced by this provider.
    fn dimensions(&self) -> usize;
}

/// OpenAI embedding provider.
pub struct OpenAiEmbedding {
    api_key: String,
    model: String,
    base_url: String,
    client: reqwest::Client,
    dimensions: usize,
}

impl OpenAiEmbedding {
    /// Creates a new OpenAI embedding provider.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_model(api_key, "text-embedding-3-small")
    }

    /// Creates a new OpenAI embedding provider with a specific model.
    pub fn with_model(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        let model_str = model.into();
        let dimensions = match model_str.as_str() {
            "text-embedding-3-small" => 1536,
            "text-embedding-3-large" => 3072,
            "text-embedding-ada-002" => 1536,
            _ => 1536, // default
        };

        Self {
            api_key: api_key.into(),
            model: model_str,
            base_url: "https://api.openai.com/v1".to_string(),
            client: reqwest::Client::new(),
            dimensions,
        }
    }

    /// Sets a custom base URL (for OpenAI-compatible APIs).
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Sets custom dimensions (for models that support it).
    pub fn with_dimensions(mut self, dimensions: usize) -> Self {
        self.dimensions = dimensions;
        self
    }
}

#[derive(Serialize)]
struct EmbeddingRequest {
    model: String,
    input: EmbeddingInput,
}

#[derive(Serialize)]
#[serde(untagged)]
enum EmbeddingInput {
    Single(String),
    Batch(Vec<String>),
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

#[async_trait]
impl EmbeddingProvider for OpenAiEmbedding {
    async fn embed(&self, text: &str) -> Result<Embedding> {
        let request = EmbeddingRequest {
            model: self.model.clone(),
            input: EmbeddingInput::Single(text.to_string()),
        };

        let response = self
            .client
            .post(format!("{}/embeddings", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI embeddings API")?;

        let embedding_response: EmbeddingResponse = response
            .json()
            .await
            .context("Failed to parse OpenAI embeddings response")?;

        embedding_response
            .data
            .first()
            .map(|d| Embedding::new(d.embedding.clone()))
            .ok_or_else(|| anyhow::anyhow!("No embedding returned from OpenAI"))
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Embedding>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let request = EmbeddingRequest {
            model: self.model.clone(),
            input: EmbeddingInput::Batch(texts.to_vec()),
        };

        let response = self
            .client
            .post(format!("{}/embeddings", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send batch request to OpenAI embeddings API")?;

        let embedding_response: EmbeddingResponse = response
            .json()
            .await
            .context("Failed to parse OpenAI embeddings response")?;

        Ok(embedding_response
            .data
            .into_iter()
            .map(|d| Embedding::new(d.embedding))
            .collect())
    }

    fn provider_name(&self) -> &str {
        "OpenAI"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    fn dimensions(&self) -> usize {
        self.dimensions
    }
}

/// Local embedding provider using sentence transformers via Ollama or similar.
pub struct LocalEmbedding {
    base_url: String,
    model: String,
    client: reqwest::Client,
    dimensions: usize,
}

impl LocalEmbedding {
    /// Creates a new local embedding provider.
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            model: model.into(),
            client: reqwest::Client::new(),
            dimensions: 768, // typical for sentence transformers
        }
    }

    /// Sets the base URL for the local embedding service.
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Sets the expected embedding dimensions.
    pub fn with_dimensions(mut self, dimensions: usize) -> Self {
        self.dimensions = dimensions;
        self
    }
}

#[derive(Serialize)]
struct LocalEmbeddingRequest {
    model: String,
    prompt: String,
}

#[derive(Deserialize)]
struct LocalEmbeddingResponse {
    embedding: Vec<f32>,
}

#[async_trait]
impl EmbeddingProvider for LocalEmbedding {
    async fn embed(&self, text: &str) -> Result<Embedding> {
        let request = LocalEmbeddingRequest {
            model: self.model.clone(),
            prompt: text.to_string(),
        };

        let response = self
            .client
            .post(format!("{}/api/embeddings", self.base_url))
            .json(&request)
            .send()
            .await
            .context("Failed to send request to local embedding service")?;

        let embedding_response: LocalEmbeddingResponse = response
            .json()
            .await
            .context("Failed to parse local embedding response")?;

        Ok(Embedding::new(embedding_response.embedding))
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Embedding>> {
        // Local embedding services typically don't support batching,
        // so we process sequentially
        let mut embeddings = Vec::with_capacity(texts.len());
        for text in texts {
            embeddings.push(self.embed(text).await?);
        }
        Ok(embeddings)
    }

    fn provider_name(&self) -> &str {
        "Local"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    fn dimensions(&self) -> usize {
        self.dimensions
    }
}

/// Utilities for working with embeddings.
pub mod utils {
    use super::*;

    /// Finds the top-k most similar embeddings to a query.
    pub fn find_top_k(
        query: &Embedding,
        candidates: &[(Embedding, usize)], // (embedding, original_index)
        k: usize,
    ) -> Result<Vec<(usize, f32)>> {
        let mut similarities: Vec<(usize, f32)> = candidates
            .iter()
            .map(|(emb, idx)| {
                let sim = query.cosine_similarity(emb).unwrap_or(0.0);
                (*idx, sim)
            })
            .collect();

        // Sort by similarity (descending)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Take top k
        similarities.truncate(k);

        Ok(similarities)
    }

    /// Clusters embeddings using simple k-means.
    pub fn cluster_kmeans(
        embeddings: &[Embedding],
        k: usize,
        max_iterations: usize,
    ) -> Result<Vec<usize>> {
        if embeddings.is_empty() {
            return Ok(Vec::new());
        }
        if k == 0 {
            anyhow::bail!("k must be greater than 0");
        }
        if k > embeddings.len() {
            anyhow::bail!("k cannot be greater than number of embeddings");
        }

        let dim = embeddings[0].dimensions;

        // Initialize centroids: use evenly spaced points for deterministic results
        let step = embeddings.len() / k;
        let mut centroids: Vec<Vec<f32>> = Vec::with_capacity(k);
        for i in 0..k {
            let idx = (i * step).min(embeddings.len() - 1);
            centroids.push(embeddings[idx].vector.clone());
        }

        let mut assignments = vec![0; embeddings.len()];

        for _ in 0..max_iterations {
            // Assign each embedding to nearest centroid
            let mut changed = false;
            for (i, emb) in embeddings.iter().enumerate() {
                let mut min_dist = f32::MAX;
                let mut best_cluster = 0;

                for (c_idx, centroid) in centroids.iter().enumerate() {
                    let centroid_emb = Embedding::new(centroid.clone());
                    let dist = emb.euclidean_distance(&centroid_emb)?;
                    if dist < min_dist {
                        min_dist = dist;
                        best_cluster = c_idx;
                    }
                }

                if assignments[i] != best_cluster {
                    assignments[i] = best_cluster;
                    changed = true;
                }
            }

            if !changed {
                break;
            }

            // Update centroids
            for (c_idx, centroid) in centroids.iter_mut().enumerate().take(k) {
                let cluster_members: Vec<&Embedding> = embeddings
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| assignments[*i] == c_idx)
                    .map(|(_, e)| e)
                    .collect();

                if !cluster_members.is_empty() {
                    let mut new_centroid = vec![0.0; dim];
                    for member in &cluster_members {
                        for (i, val) in member.vector.iter().enumerate() {
                            new_centroid[i] += val;
                        }
                    }
                    for val in &mut new_centroid {
                        *val /= cluster_members.len() as f32;
                    }
                    *centroid = new_centroid;
                }
            }
        }

        Ok(assignments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_creation() {
        let vec = vec![1.0, 2.0, 3.0];
        let emb = Embedding::new(vec.clone());
        assert_eq!(emb.dimensions, 3);
        assert_eq!(emb.vector, vec);
    }

    #[test]
    fn test_cosine_similarity() {
        let emb1 = Embedding::new(vec![1.0, 0.0, 0.0]);
        let emb2 = Embedding::new(vec![1.0, 0.0, 0.0]);
        let sim = emb1.cosine_similarity(&emb2).unwrap();
        assert!((sim - 1.0).abs() < 1e-6);

        let emb3 = Embedding::new(vec![0.0, 1.0, 0.0]);
        let sim2 = emb1.cosine_similarity(&emb3).unwrap();
        assert!((sim2 - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_euclidean_distance() {
        let emb1 = Embedding::new(vec![0.0, 0.0, 0.0]);
        let emb2 = Embedding::new(vec![1.0, 1.0, 1.0]);
        let dist = emb1.euclidean_distance(&emb2).unwrap();
        assert!((dist - 3.0_f32.sqrt()).abs() < 1e-6);
    }

    #[test]
    fn test_dot_product() {
        let emb1 = Embedding::new(vec![1.0, 2.0, 3.0]);
        let emb2 = Embedding::new(vec![4.0, 5.0, 6.0]);
        let dot = emb1.dot_product(&emb2).unwrap();
        assert!((dot - 32.0).abs() < 1e-6); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_normalize() {
        let mut emb = Embedding::new(vec![3.0, 4.0]);
        emb.normalize();
        let norm: f32 = emb.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_dimension_mismatch() {
        let emb1 = Embedding::new(vec![1.0, 2.0]);
        let emb2 = Embedding::new(vec![1.0, 2.0, 3.0]);
        assert!(emb1.cosine_similarity(&emb2).is_err());
        assert!(emb1.euclidean_distance(&emb2).is_err());
        assert!(emb1.dot_product(&emb2).is_err());
    }

    #[test]
    fn test_find_top_k() {
        let query = Embedding::new(vec![1.0, 0.0]);
        let candidates = vec![
            (Embedding::new(vec![1.0, 0.0]), 0),  // Most similar
            (Embedding::new(vec![0.9, 0.1]), 1),  // Similar
            (Embedding::new(vec![0.0, 1.0]), 2),  // Orthogonal
            (Embedding::new(vec![-1.0, 0.0]), 3), // Opposite
        ];

        let top = utils::find_top_k(&query, &candidates, 2).unwrap();
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].0, 0); // Index of most similar
    }

    #[test]
    fn test_cluster_kmeans() {
        let embeddings = vec![
            Embedding::new(vec![1.0, 0.0]),
            Embedding::new(vec![1.1, 0.1]),
            Embedding::new(vec![0.0, 1.0]),
            Embedding::new(vec![0.1, 1.1]),
        ];

        let assignments = utils::cluster_kmeans(&embeddings, 2, 10).unwrap();
        assert_eq!(assignments.len(), 4);

        // Check that we have exactly 2 clusters
        let unique_clusters: std::collections::HashSet<_> = assignments.iter().copied().collect();
        assert_eq!(unique_clusters.len(), 2);

        // Points that are close should be in the same cluster
        // Check distance-based clustering quality
        for i in 0..embeddings.len() {
            for j in (i + 1)..embeddings.len() {
                let dist = embeddings[i].euclidean_distance(&embeddings[j]).unwrap();
                // If points are very close (< 0.2), they should be in same cluster
                if dist < 0.2 {
                    assert_eq!(
                        assignments[i], assignments[j],
                        "Close points {} and {} should be in same cluster",
                        i, j
                    );
                }
            }
        }
    }
}
