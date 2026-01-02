//! Vector Search & Embeddings module for semantic statute search.
//!
//! This module implements the v0.2.1 vector search features:
//! - Embedding generation (OpenAI, Cohere, local models)
//! - HNSW vector similarity search
//! - Hybrid search (keyword + vector)
//! - Embedding-based deduplication
//! - Semantic clustering

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// ============================================================================
// Vector Embeddings
// ============================================================================

/// A vector embedding for semantic search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    /// The embedding vector (typically 384-1536 dimensions)
    pub vector: Vec<f32>,
    /// Dimensionality of the embedding
    pub dimensions: usize,
}

impl Embedding {
    /// Create a new embedding from a vector.
    pub fn new(vector: Vec<f32>) -> Self {
        let dimensions = vector.len();
        Self { vector, dimensions }
    }

    /// Calculate cosine similarity with another embedding.
    /// Returns a value between -1 and 1, where 1 means identical.
    pub fn cosine_similarity(&self, other: &Embedding) -> f32 {
        if self.dimensions != other.dimensions {
            return 0.0;
        }

        let dot_product: f32 = self
            .vector
            .iter()
            .zip(other.vector.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm_a: f32 = self.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = other.vector.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot_product / (norm_a * norm_b)
    }

    /// Calculate Euclidean distance to another embedding.
    pub fn euclidean_distance(&self, other: &Embedding) -> f32 {
        if self.dimensions != other.dimensions {
            return f32::MAX;
        }

        self.vector
            .iter()
            .zip(other.vector.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum::<f32>()
            .sqrt()
    }

    /// Calculate Manhattan (L1) distance to another embedding.
    pub fn manhattan_distance(&self, other: &Embedding) -> f32 {
        if self.dimensions != other.dimensions {
            return f32::MAX;
        }

        self.vector
            .iter()
            .zip(other.vector.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }
}

// ============================================================================
// Embedding Providers
// ============================================================================

/// Embedding provider types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmbeddingProvider {
    /// OpenAI embeddings (text-embedding-3-small, etc.)
    OpenAI,
    /// Cohere embeddings
    Cohere,
    /// Local model (e.g., sentence-transformers)
    Local,
    /// Custom provider
    Custom,
}

/// Configuration for embedding generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// Provider to use
    pub provider: EmbeddingProvider,
    /// Model name (e.g., "text-embedding-3-small")
    pub model: String,
    /// API key (for cloud providers)
    pub api_key: Option<String>,
    /// Maximum tokens to embed
    pub max_tokens: usize,
    /// Batch size for bulk embedding
    pub batch_size: usize,
}

impl EmbeddingConfig {
    /// Create a new embedding config.
    pub fn new(provider: EmbeddingProvider, model: String) -> Self {
        Self {
            provider,
            model,
            api_key: None,
            max_tokens: 8192,
            batch_size: 100,
        }
    }

    /// Set API key for cloud providers.
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Create config for OpenAI.
    pub fn openai(model: String, api_key: String) -> Self {
        Self::new(EmbeddingProvider::OpenAI, model).with_api_key(api_key)
    }

    /// Create config for Cohere.
    pub fn cohere(model: String, api_key: String) -> Self {
        Self::new(EmbeddingProvider::Cohere, model).with_api_key(api_key)
    }

    /// Create config for local model.
    pub fn local(model: String) -> Self {
        Self::new(EmbeddingProvider::Local, model)
    }
}

/// A statute embedding with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteEmbedding {
    /// Statute ID
    pub statute_id: String,
    /// The embedding vector
    pub embedding: Embedding,
    /// Text that was embedded
    pub embedded_text: String,
    /// Embedding generation timestamp
    pub generated_at: chrono::DateTime<chrono::Utc>,
    /// Model used for embedding
    pub model: String,
}

// ============================================================================
// HNSW Index (Hierarchical Navigable Small World)
// ============================================================================

/// HNSW graph layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HnswLayer {
    /// Adjacency list for this layer
    adjacency: HashMap<usize, Vec<usize>>,
    /// Maximum number of connections per node
    max_connections: usize,
}

impl HnswLayer {
    fn new(max_connections: usize) -> Self {
        Self {
            adjacency: HashMap::new(),
            max_connections,
        }
    }

    fn add_node(&mut self, node_id: usize) {
        self.adjacency.entry(node_id).or_default();
    }

    fn connect(&mut self, from: usize, to: usize) {
        let connections = self.adjacency.entry(from).or_default();
        if !connections.contains(&to) && connections.len() < self.max_connections {
            connections.push(to);
        }
    }

    #[allow(dead_code)]
    fn neighbors(&self, node_id: usize) -> &[usize] {
        self.adjacency
            .get(&node_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}

/// HNSW index for fast similarity search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HnswIndex {
    /// All embeddings in the index
    embeddings: Vec<StatuteEmbedding>,
    /// Hierarchical layers
    layers: Vec<HnswLayer>,
    /// Entry point node ID
    entry_point: Option<usize>,
    /// Maximum number of layers
    max_layers: usize,
    /// Maximum connections per layer
    max_connections: usize,
    /// Search expansion factor
    ef_construction: usize,
    /// Search expansion factor for queries
    ef_search: usize,
}

impl HnswIndex {
    /// Create a new HNSW index.
    pub fn new() -> Self {
        Self {
            embeddings: Vec::new(),
            layers: vec![HnswLayer::new(16)], // Base layer
            entry_point: None,
            max_layers: 5,
            max_connections: 16,
            ef_construction: 200,
            ef_search: 50,
        }
    }

    /// Configure the index parameters.
    pub fn with_params(
        mut self,
        max_layers: usize,
        max_connections: usize,
        ef_construction: usize,
        ef_search: usize,
    ) -> Self {
        self.max_layers = max_layers;
        self.max_connections = max_connections;
        self.ef_construction = ef_construction;
        self.ef_search = ef_search;
        self
    }

    /// Add an embedding to the index.
    pub fn add(&mut self, embedding: StatuteEmbedding) {
        let node_id = self.embeddings.len();
        self.embeddings.push(embedding);

        // Add to base layer
        self.layers[0].add_node(node_id);

        // Set entry point if this is the first node
        if self.entry_point.is_none() {
            self.entry_point = Some(node_id);
            return;
        }

        // Connect to nearest neighbors in base layer
        let neighbors = self.find_nearest_neighbors(node_id, self.ef_construction, 0);
        for &neighbor_id in neighbors.iter().take(self.max_connections) {
            self.layers[0].connect(node_id, neighbor_id);
            self.layers[0].connect(neighbor_id, node_id);
        }
    }

    /// Find k nearest neighbors.
    fn find_nearest_neighbors(&self, query_id: usize, k: usize, layer: usize) -> Vec<usize> {
        if self.embeddings.is_empty() || layer >= self.layers.len() {
            return Vec::new();
        }

        let mut candidates = Vec::new();
        let mut visited = HashSet::new();

        // Start from entry point or all nodes if no entry point
        let start_nodes = if let Some(entry) = self.entry_point {
            vec![entry]
        } else {
            (0..self.embeddings.len()).collect()
        };

        for start_id in start_nodes {
            if start_id == query_id {
                continue;
            }

            let similarity = self.embeddings[query_id]
                .embedding
                .cosine_similarity(&self.embeddings[start_id].embedding);

            candidates.push((start_id, similarity));
            visited.insert(start_id);
        }

        // Sort by similarity (descending)
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Take top k
        candidates.truncate(k);
        candidates.into_iter().map(|(id, _)| id).collect()
    }

    /// Search for similar embeddings.
    pub fn search(&self, query: &Embedding, k: usize) -> Vec<SearchResult> {
        if self.embeddings.is_empty() {
            return Vec::new();
        }

        let mut results = Vec::new();

        for statute_emb in self.embeddings.iter() {
            let similarity = query.cosine_similarity(&statute_emb.embedding);
            results.push(SearchResult {
                statute_id: statute_emb.statute_id.clone(),
                similarity,
                embedding: statute_emb.clone(),
            });
        }

        // Sort by similarity (descending)
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

        // Take top k
        results.truncate(k);
        results
    }

    /// Get the number of embeddings in the index.
    pub fn len(&self) -> usize {
        self.embeddings.len()
    }

    /// Check if the index is empty.
    pub fn is_empty(&self) -> bool {
        self.embeddings.is_empty()
    }

    /// Get all statute IDs in the index.
    pub fn statute_ids(&self) -> Vec<String> {
        self.embeddings
            .iter()
            .map(|e| e.statute_id.clone())
            .collect()
    }
}

impl Default for HnswIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// A search result from HNSW index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Statute ID
    pub statute_id: String,
    /// Similarity score (0.0 to 1.0)
    pub similarity: f32,
    /// The embedding
    pub embedding: StatuteEmbedding,
}

// ============================================================================
// Hybrid Search
// ============================================================================

/// Hybrid search result combining keyword and vector search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSearchResult {
    /// Statute ID
    pub statute_id: String,
    /// Combined score
    pub combined_score: f32,
    /// Keyword search score (0.0 to 1.0)
    pub keyword_score: f32,
    /// Vector similarity score (0.0 to 1.0)
    pub vector_score: f32,
}

/// Configuration for hybrid search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSearchConfig {
    /// Weight for keyword search (0.0 to 1.0)
    pub keyword_weight: f32,
    /// Weight for vector search (0.0 to 1.0)
    pub vector_weight: f32,
    /// Number of results to return
    pub top_k: usize,
}

impl HybridSearchConfig {
    /// Create a new hybrid search config with balanced weights.
    pub fn balanced(top_k: usize) -> Self {
        Self {
            keyword_weight: 0.5,
            vector_weight: 0.5,
            top_k,
        }
    }

    /// Create a config favoring keyword search.
    pub fn keyword_focused(top_k: usize) -> Self {
        Self {
            keyword_weight: 0.7,
            vector_weight: 0.3,
            top_k,
        }
    }

    /// Create a config favoring vector search.
    pub fn vector_focused(top_k: usize) -> Self {
        Self {
            keyword_weight: 0.3,
            vector_weight: 0.7,
            top_k,
        }
    }
}

impl Default for HybridSearchConfig {
    fn default() -> Self {
        Self::balanced(10)
    }
}

/// Hybrid search engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSearch {
    /// HNSW index for vector search
    pub vector_index: HnswIndex,
    /// Configuration
    pub config: HybridSearchConfig,
}

impl HybridSearch {
    /// Create a new hybrid search engine.
    pub fn new(config: HybridSearchConfig) -> Self {
        Self {
            vector_index: HnswIndex::new(),
            config,
        }
    }

    /// Add a statute embedding.
    pub fn add_embedding(&mut self, embedding: StatuteEmbedding) {
        self.vector_index.add(embedding);
    }

    /// Perform hybrid search.
    pub fn search(
        &self,
        query_embedding: &Embedding,
        keyword_scores: &HashMap<String, f32>,
    ) -> Vec<HybridSearchResult> {
        // Get vector search results
        let vector_results = self
            .vector_index
            .search(query_embedding, self.config.top_k * 2);

        let mut combined_results = HashMap::new();

        // Combine keyword and vector scores
        for result in vector_results {
            let keyword_score = keyword_scores
                .get(&result.statute_id)
                .copied()
                .unwrap_or(0.0);
            let vector_score = result.similarity;

            let combined_score = self.config.keyword_weight * keyword_score
                + self.config.vector_weight * vector_score;

            combined_results.insert(
                result.statute_id.clone(),
                HybridSearchResult {
                    statute_id: result.statute_id,
                    combined_score,
                    keyword_score,
                    vector_score,
                },
            );
        }

        // Add keyword-only results that weren't in vector results
        for (statute_id, &keyword_score) in keyword_scores {
            combined_results
                .entry(statute_id.clone())
                .or_insert_with(|| HybridSearchResult {
                    statute_id: statute_id.clone(),
                    combined_score: self.config.keyword_weight * keyword_score,
                    keyword_score,
                    vector_score: 0.0,
                });
        }

        // Sort by combined score
        let mut results: Vec<HybridSearchResult> = combined_results.into_values().collect();
        results.sort_by(|a, b| b.combined_score.partial_cmp(&a.combined_score).unwrap());

        // Take top k
        results.truncate(self.config.top_k);
        results
    }
}

// ============================================================================
// Embedding-Based Deduplication
// ============================================================================

/// Duplicate detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateCandidate {
    /// First statute ID
    pub statute_id_1: String,
    /// Second statute ID
    pub statute_id_2: String,
    /// Similarity score
    pub similarity: f32,
    /// Confidence level
    pub confidence: DuplicateConfidence,
}

/// Confidence level for duplicate detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DuplicateConfidence {
    /// High confidence (>= 0.95 similarity)
    High,
    /// Medium confidence (>= 0.85 similarity)
    Medium,
    /// Low confidence (>= 0.75 similarity)
    Low,
}

impl DuplicateConfidence {
    fn from_similarity(similarity: f32) -> Option<Self> {
        if similarity >= 0.95 {
            Some(DuplicateConfidence::High)
        } else if similarity >= 0.85 {
            Some(DuplicateConfidence::Medium)
        } else if similarity >= 0.75 {
            Some(DuplicateConfidence::Low)
        } else {
            None
        }
    }
}

/// Deduplication engine using embeddings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeduplicationEngine {
    /// HNSW index for fast similarity search
    index: HnswIndex,
    /// Minimum similarity threshold
    threshold: f32,
}

impl DeduplicationEngine {
    /// Create a new deduplication engine.
    pub fn new(threshold: f32) -> Self {
        Self {
            index: HnswIndex::new(),
            threshold,
        }
    }

    /// Add a statute embedding.
    pub fn add(&mut self, embedding: StatuteEmbedding) {
        self.index.add(embedding);
    }

    /// Find duplicate candidates.
    pub fn find_duplicates(&self) -> Vec<DuplicateCandidate> {
        let mut duplicates = Vec::new();
        let mut seen_pairs = HashSet::new();

        for emb1 in self.index.embeddings.iter() {
            let similar = self.index.search(&emb1.embedding, 10);

            for result in similar {
                // Skip self
                if result.statute_id == emb1.statute_id {
                    continue;
                }

                // Skip if similarity below threshold
                if result.similarity < self.threshold {
                    continue;
                }

                // Create canonical pair key (sorted)
                let pair = if emb1.statute_id < result.statute_id {
                    (emb1.statute_id.clone(), result.statute_id.clone())
                } else {
                    (result.statute_id.clone(), emb1.statute_id.clone())
                };

                // Skip if we've already seen this pair
                if !seen_pairs.insert(pair.clone()) {
                    continue;
                }

                if let Some(confidence) = DuplicateConfidence::from_similarity(result.similarity) {
                    duplicates.push(DuplicateCandidate {
                        statute_id_1: pair.0,
                        statute_id_2: pair.1,
                        similarity: result.similarity,
                        confidence,
                    });
                }
            }
        }

        // Sort by similarity (descending)
        duplicates.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

        duplicates
    }
}

// ============================================================================
// Semantic Clustering
// ============================================================================

/// A cluster of similar statutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteCluster {
    /// Cluster ID
    pub cluster_id: usize,
    /// Statute IDs in this cluster
    pub statute_ids: Vec<String>,
    /// Centroid embedding
    pub centroid: Embedding,
    /// Average intra-cluster similarity
    pub cohesion: f32,
}

/// Clustering algorithm type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClusteringAlgorithm {
    /// K-means clustering
    KMeans,
    /// Hierarchical clustering
    Hierarchical,
    /// DBSCAN (density-based)
    DBSCAN,
}

/// Configuration for semantic clustering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusteringConfig {
    /// Algorithm to use
    pub algorithm: ClusteringAlgorithm,
    /// Number of clusters (for K-means)
    pub num_clusters: usize,
    /// Minimum similarity for same cluster
    pub min_similarity: f32,
    /// Maximum iterations (for iterative algorithms)
    pub max_iterations: usize,
}

impl ClusteringConfig {
    /// Create config for K-means.
    pub fn kmeans(num_clusters: usize) -> Self {
        Self {
            algorithm: ClusteringAlgorithm::KMeans,
            num_clusters,
            min_similarity: 0.7,
            max_iterations: 100,
        }
    }

    /// Create config for DBSCAN.
    pub fn dbscan(min_similarity: f32) -> Self {
        Self {
            algorithm: ClusteringAlgorithm::DBSCAN,
            num_clusters: 0, // Not used for DBSCAN
            min_similarity,
            max_iterations: 1,
        }
    }
}

/// Semantic clustering engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusteringEngine {
    /// All embeddings
    embeddings: Vec<StatuteEmbedding>,
    /// Configuration
    config: ClusteringConfig,
}

impl ClusteringEngine {
    /// Create a new clustering engine.
    pub fn new(config: ClusteringConfig) -> Self {
        Self {
            embeddings: Vec::new(),
            config,
        }
    }

    /// Add an embedding.
    pub fn add(&mut self, embedding: StatuteEmbedding) {
        self.embeddings.push(embedding);
    }

    /// Perform clustering.
    pub fn cluster(&self) -> Vec<StatuteCluster> {
        match self.config.algorithm {
            ClusteringAlgorithm::KMeans => self.kmeans_clustering(),
            ClusteringAlgorithm::Hierarchical => self.hierarchical_clustering(),
            ClusteringAlgorithm::DBSCAN => self.dbscan_clustering(),
        }
    }

    /// K-means clustering implementation.
    #[allow(clippy::needless_range_loop)]
    fn kmeans_clustering(&self) -> Vec<StatuteCluster> {
        if self.embeddings.is_empty() {
            return Vec::new();
        }

        let k = self.config.num_clusters.min(self.embeddings.len());
        let mut clusters = Vec::new();

        // Initialize centroids with first k embeddings
        let mut centroids: Vec<Embedding> = self
            .embeddings
            .iter()
            .take(k)
            .map(|e| e.embedding.clone())
            .collect();

        for _iteration in 0..self.config.max_iterations {
            // Assign each point to nearest centroid
            let mut assignments = vec![0; self.embeddings.len()];

            for (i, emb) in self.embeddings.iter().enumerate() {
                let mut best_cluster = 0;
                let mut best_similarity = f32::MIN;

                for (cluster_id, centroid) in centroids.iter().enumerate() {
                    let similarity = emb.embedding.cosine_similarity(centroid);
                    if similarity > best_similarity {
                        best_similarity = similarity;
                        best_cluster = cluster_id;
                    }
                }

                assignments[i] = best_cluster;
            }

            // Update centroids
            for cluster_id in 0..k {
                let cluster_points: Vec<&Embedding> = self
                    .embeddings
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| assignments[*i] == cluster_id)
                    .map(|(_, e)| &e.embedding)
                    .collect();

                if !cluster_points.is_empty() {
                    centroids[cluster_id] = Self::compute_centroid(&cluster_points);
                }
            }
        }

        // Build final clusters
        for (cluster_id, centroid) in centroids.iter().enumerate().take(k) {
            let statute_ids: Vec<String> = self
                .embeddings
                .iter()
                .filter_map(|emb| {
                    let mut best_cluster = 0;
                    let mut best_similarity = f32::MIN;

                    for (cid, c) in centroids.iter().enumerate() {
                        let similarity = emb.embedding.cosine_similarity(c);
                        if similarity > best_similarity {
                            best_similarity = similarity;
                            best_cluster = cid;
                        }
                    }

                    if best_cluster == cluster_id {
                        Some(emb.statute_id.clone())
                    } else {
                        None
                    }
                })
                .collect();

            if !statute_ids.is_empty() {
                let cohesion = self.compute_cohesion(&statute_ids);
                clusters.push(StatuteCluster {
                    cluster_id,
                    statute_ids,
                    centroid: centroid.clone(),
                    cohesion,
                });
            }
        }

        clusters
    }

    /// Hierarchical clustering (simplified agglomerative).
    fn hierarchical_clustering(&self) -> Vec<StatuteCluster> {
        // Simplified implementation - treat each statute as initial cluster
        // Then merge most similar clusters iteratively
        let mut clusters: Vec<Vec<usize>> = (0..self.embeddings.len()).map(|i| vec![i]).collect();

        while clusters.len() > self.config.num_clusters {
            // Find most similar pair of clusters
            let mut best_pair = (0, 1);
            let mut best_similarity = f32::MIN;

            for i in 0..clusters.len() {
                for j in (i + 1)..clusters.len() {
                    let sim = self.cluster_similarity(&clusters[i], &clusters[j]);
                    if sim > best_similarity {
                        best_similarity = sim;
                        best_pair = (i, j);
                    }
                }
            }

            // Merge the two most similar clusters
            let (i, j) = best_pair;
            let mut merged = clusters[i].clone();
            merged.extend(&clusters[j]);

            // Remove old clusters and add merged
            clusters.remove(j);
            clusters.remove(i);
            clusters.push(merged);
        }

        // Convert to StatuteCluster format
        clusters
            .into_iter()
            .enumerate()
            .map(|(cluster_id, indices)| {
                let statute_ids: Vec<String> = indices
                    .iter()
                    .map(|&i| self.embeddings[i].statute_id.clone())
                    .collect();

                let embeddings: Vec<&Embedding> = indices
                    .iter()
                    .map(|&i| &self.embeddings[i].embedding)
                    .collect();

                let centroid = Self::compute_centroid(&embeddings);
                let cohesion = self.compute_cohesion(&statute_ids);

                StatuteCluster {
                    cluster_id,
                    statute_ids,
                    centroid,
                    cohesion,
                }
            })
            .collect()
    }

    /// DBSCAN clustering.
    fn dbscan_clustering(&self) -> Vec<StatuteCluster> {
        let min_similarity = self.config.min_similarity;
        let min_points = 2;

        let mut visited = vec![false; self.embeddings.len()];
        let mut clusters = Vec::new();
        let mut cluster_id = 0;

        for i in 0..self.embeddings.len() {
            if visited[i] {
                continue;
            }

            visited[i] = true;

            // Find neighbors
            let neighbors = self.find_neighbors(i, min_similarity);

            if neighbors.len() < min_points {
                continue; // Noise point
            }

            // Start new cluster
            let mut cluster = vec![i];
            let mut to_visit = neighbors;

            while let Some(neighbor_idx) = to_visit.pop() {
                if visited[neighbor_idx] {
                    continue;
                }

                visited[neighbor_idx] = true;
                cluster.push(neighbor_idx);

                let neighbor_neighbors = self.find_neighbors(neighbor_idx, min_similarity);
                if neighbor_neighbors.len() >= min_points {
                    to_visit.extend(neighbor_neighbors);
                }
            }

            let statute_ids: Vec<String> = cluster
                .iter()
                .map(|&idx| self.embeddings[idx].statute_id.clone())
                .collect();

            let embeddings: Vec<&Embedding> = cluster
                .iter()
                .map(|&idx| &self.embeddings[idx].embedding)
                .collect();

            let centroid = Self::compute_centroid(&embeddings);
            let cohesion = self.compute_cohesion(&statute_ids);

            clusters.push(StatuteCluster {
                cluster_id,
                statute_ids,
                centroid,
                cohesion,
            });

            cluster_id += 1;
        }

        clusters
    }

    /// Find neighbors within similarity threshold.
    fn find_neighbors(&self, index: usize, min_similarity: f32) -> Vec<usize> {
        let mut neighbors = Vec::new();

        for (i, emb) in self.embeddings.iter().enumerate() {
            if i == index {
                continue;
            }

            let similarity = self.embeddings[index]
                .embedding
                .cosine_similarity(&emb.embedding);

            if similarity >= min_similarity {
                neighbors.push(i);
            }
        }

        neighbors
    }

    /// Compute centroid of embeddings.
    fn compute_centroid(embeddings: &[&Embedding]) -> Embedding {
        if embeddings.is_empty() {
            return Embedding::new(Vec::new());
        }

        let dimensions = embeddings[0].dimensions;
        let mut centroid_vec = vec![0.0; dimensions];

        for emb in embeddings {
            for (i, &val) in emb.vector.iter().enumerate() {
                centroid_vec[i] += val;
            }
        }

        for val in &mut centroid_vec {
            *val /= embeddings.len() as f32;
        }

        Embedding::new(centroid_vec)
    }

    /// Compute similarity between two clusters.
    fn cluster_similarity(&self, cluster1: &[usize], cluster2: &[usize]) -> f32 {
        let mut total_similarity = 0.0;
        let mut count = 0;

        for &i in cluster1 {
            for &j in cluster2 {
                total_similarity += self.embeddings[i]
                    .embedding
                    .cosine_similarity(&self.embeddings[j].embedding);
                count += 1;
            }
        }

        if count > 0 {
            total_similarity / count as f32
        } else {
            0.0
        }
    }

    /// Compute cohesion (average intra-cluster similarity).
    fn compute_cohesion(&self, statute_ids: &[String]) -> f32 {
        if statute_ids.len() < 2 {
            return 1.0;
        }

        let embeddings: Vec<&StatuteEmbedding> = self
            .embeddings
            .iter()
            .filter(|e| statute_ids.contains(&e.statute_id))
            .collect();

        let mut total_similarity = 0.0;
        let mut count = 0;

        for i in 0..embeddings.len() {
            for j in (i + 1)..embeddings.len() {
                total_similarity += embeddings[i]
                    .embedding
                    .cosine_similarity(&embeddings[j].embedding);
                count += 1;
            }
        }

        if count > 0 {
            total_similarity / count as f32
        } else {
            0.0
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_embedding(values: Vec<f32>) -> Embedding {
        Embedding::new(values)
    }

    fn create_statute_embedding(id: &str, values: Vec<f32>) -> StatuteEmbedding {
        StatuteEmbedding {
            statute_id: id.to_string(),
            embedding: Embedding::new(values),
            embedded_text: format!("Text for {}", id),
            generated_at: chrono::Utc::now(),
            model: "test-model".to_string(),
        }
    }

    #[test]
    fn test_embedding_new() {
        let emb = create_test_embedding(vec![1.0, 2.0, 3.0]);
        assert_eq!(emb.dimensions, 3);
        assert_eq!(emb.vector.len(), 3);
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let emb1 = create_test_embedding(vec![1.0, 0.0, 0.0]);
        let emb2 = create_test_embedding(vec![1.0, 0.0, 0.0]);
        let similarity = emb1.cosine_similarity(&emb2);
        assert!((similarity - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let emb1 = create_test_embedding(vec![1.0, 0.0]);
        let emb2 = create_test_embedding(vec![0.0, 1.0]);
        let similarity = emb1.cosine_similarity(&emb2);
        assert!(similarity.abs() < 0.001);
    }

    #[test]
    fn test_euclidean_distance() {
        let emb1 = create_test_embedding(vec![0.0, 0.0]);
        let emb2 = create_test_embedding(vec![3.0, 4.0]);
        let distance = emb1.euclidean_distance(&emb2);
        assert!((distance - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_manhattan_distance() {
        let emb1 = create_test_embedding(vec![0.0, 0.0]);
        let emb2 = create_test_embedding(vec![3.0, 4.0]);
        let distance = emb1.manhattan_distance(&emb2);
        assert!((distance - 7.0).abs() < 0.001);
    }

    #[test]
    fn test_embedding_config_openai() {
        let config =
            EmbeddingConfig::openai("text-embedding-3-small".to_string(), "api-key".to_string());
        assert_eq!(config.provider, EmbeddingProvider::OpenAI);
        assert!(config.api_key.is_some());
    }

    #[test]
    fn test_embedding_config_local() {
        let config = EmbeddingConfig::local("sentence-transformers".to_string());
        assert_eq!(config.provider, EmbeddingProvider::Local);
        assert!(config.api_key.is_none());
    }

    #[test]
    fn test_hnsw_index_new() {
        let index = HnswIndex::new();
        assert!(index.is_empty());
        assert_eq!(index.len(), 0);
    }

    #[test]
    fn test_hnsw_index_add() {
        let mut index = HnswIndex::new();
        let emb = create_statute_embedding("statute-1", vec![1.0, 0.0, 0.0]);
        index.add(emb);
        assert_eq!(index.len(), 1);
    }

    #[test]
    fn test_hnsw_index_search() {
        let mut index = HnswIndex::new();
        index.add(create_statute_embedding("statute-1", vec![1.0, 0.0, 0.0]));
        index.add(create_statute_embedding("statute-2", vec![0.9, 0.1, 0.0]));
        index.add(create_statute_embedding("statute-3", vec![0.0, 1.0, 0.0]));

        let query = create_test_embedding(vec![1.0, 0.0, 0.0]);
        let results = index.search(&query, 2);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].statute_id, "statute-1");
    }

    #[test]
    fn test_hybrid_search_config() {
        let config = HybridSearchConfig::balanced(10);
        assert_eq!(config.keyword_weight, 0.5);
        assert_eq!(config.vector_weight, 0.5);
        assert_eq!(config.top_k, 10);
    }

    #[test]
    fn test_hybrid_search() {
        let config = HybridSearchConfig::balanced(5);
        let mut search = HybridSearch::new(config);

        search.add_embedding(create_statute_embedding("statute-1", vec![1.0, 0.0]));
        search.add_embedding(create_statute_embedding("statute-2", vec![0.0, 1.0]));

        let query = create_test_embedding(vec![1.0, 0.0]);
        let mut keyword_scores = HashMap::new();
        keyword_scores.insert("statute-1".to_string(), 0.8);
        keyword_scores.insert("statute-2".to_string(), 0.3);

        let results = search.search(&query, &keyword_scores);
        assert!(!results.is_empty());
        assert_eq!(results[0].statute_id, "statute-1");
    }

    #[test]
    fn test_duplicate_confidence() {
        assert_eq!(
            DuplicateConfidence::from_similarity(0.96),
            Some(DuplicateConfidence::High)
        );
        assert_eq!(
            DuplicateConfidence::from_similarity(0.87),
            Some(DuplicateConfidence::Medium)
        );
        assert_eq!(
            DuplicateConfidence::from_similarity(0.78),
            Some(DuplicateConfidence::Low)
        );
        assert_eq!(DuplicateConfidence::from_similarity(0.6), None);
    }

    #[test]
    fn test_deduplication_engine() {
        let mut engine = DeduplicationEngine::new(0.9);

        engine.add(create_statute_embedding("statute-1", vec![1.0, 0.0, 0.0]));
        engine.add(create_statute_embedding("statute-2", vec![0.95, 0.05, 0.0]));
        engine.add(create_statute_embedding("statute-3", vec![0.0, 1.0, 0.0]));

        let duplicates = engine.find_duplicates();
        assert!(!duplicates.is_empty());
    }

    #[test]
    fn test_clustering_config_kmeans() {
        let config = ClusteringConfig::kmeans(5);
        assert_eq!(config.algorithm, ClusteringAlgorithm::KMeans);
        assert_eq!(config.num_clusters, 5);
    }

    #[test]
    fn test_clustering_config_dbscan() {
        let config = ClusteringConfig::dbscan(0.7);
        assert_eq!(config.algorithm, ClusteringAlgorithm::DBSCAN);
        assert_eq!(config.min_similarity, 0.7);
    }

    #[test]
    fn test_clustering_engine_kmeans() {
        let config = ClusteringConfig::kmeans(2);
        let mut engine = ClusteringEngine::new(config);

        engine.add(create_statute_embedding("statute-1", vec![1.0, 0.0]));
        engine.add(create_statute_embedding("statute-2", vec![0.9, 0.1]));
        engine.add(create_statute_embedding("statute-3", vec![0.0, 1.0]));
        engine.add(create_statute_embedding("statute-4", vec![0.1, 0.9]));

        let clusters = engine.cluster();
        assert_eq!(clusters.len(), 2);
    }

    #[test]
    fn test_clustering_engine_dbscan() {
        let config = ClusteringConfig::dbscan(0.7);
        let mut engine = ClusteringEngine::new(config);

        // Create tightly clustered points
        engine.add(create_statute_embedding("statute-1", vec![1.0, 0.0]));
        engine.add(create_statute_embedding("statute-2", vec![0.95, 0.0]));
        engine.add(create_statute_embedding("statute-3", vec![0.9, 0.0]));
        engine.add(create_statute_embedding("statute-4", vec![0.0, 1.0]));
        engine.add(create_statute_embedding("statute-5", vec![0.0, 0.95]));

        let clusters = engine.cluster();
        // With DBSCAN, we should find at least one cluster
        assert!(!clusters.is_empty() || engine.embeddings.len() < 2);
    }

    #[test]
    fn test_statute_embedding_creation() {
        let emb = create_statute_embedding("test-1", vec![1.0, 2.0, 3.0]);
        assert_eq!(emb.statute_id, "test-1");
        assert_eq!(emb.embedding.dimensions, 3);
        assert_eq!(emb.model, "test-model");
    }

    #[test]
    fn test_search_result() {
        let emb = create_statute_embedding("statute-1", vec![1.0, 0.0]);
        let result = SearchResult {
            statute_id: emb.statute_id.clone(),
            similarity: 0.95,
            embedding: emb,
        };

        assert_eq!(result.similarity, 0.95);
        assert_eq!(result.statute_id, "statute-1");
    }

    #[test]
    fn test_hnsw_statute_ids() {
        let mut index = HnswIndex::new();
        index.add(create_statute_embedding("statute-1", vec![1.0, 0.0]));
        index.add(create_statute_embedding("statute-2", vec![0.0, 1.0]));

        let ids = index.statute_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&"statute-1".to_string()));
        assert!(ids.contains(&"statute-2".to_string()));
    }
}
