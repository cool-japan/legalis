//! Smart search: an in-memory inverted-index ranked search engine.
//!
//! [`SearchIndex`] is a domain-agnostic, pure-Rust full-text search engine over
//! arbitrary [`IndexedDocument`]s. It maintains an inverted index built
//! incrementally as documents are added and supports ranked retrieval with
//! either Okapi BM25 or TF-IDF cosine similarity, optional metadata filtering, a
//! "more like this" similarity query and phrase-presence boosting. It is
//! independent of the precedent library so it can index any firm document
//! (memos, briefs, knowledge-base articles, ...).

use super::{stem, tokenize};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// A document to be indexed and searched.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexedDocument {
    /// Stable unique identifier.
    pub id: String,
    /// Short title (indexed with extra implicit weight via repetition-free
    /// concatenation into the body).
    pub title: String,
    /// Full body text.
    pub body: String,
    /// Arbitrary metadata used for filtering (e.g. `author`, `practice_area`).
    pub metadata: BTreeMap<String, String>,
}

impl IndexedDocument {
    /// Creates a new document.
    pub fn new(id: impl Into<String>, title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: body.into(),
            metadata: BTreeMap::new(),
        }
    }

    /// Adds a metadata entry.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// The full text used for indexing (title then body).
    pub fn indexable_text(&self) -> String {
        format!("{} {}", self.title, self.body)
    }
}

/// Ranking model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchRanking {
    /// Okapi BM25 probabilistic ranking.
    Bm25,
    /// TF-IDF cosine similarity.
    TfIdf,
}

/// Options controlling a search.
#[derive(Debug, Clone)]
pub struct SearchQuery {
    /// The raw query text.
    pub text: String,
    /// Ranking model.
    pub ranking: SearchRanking,
    /// Maximum hits to return.
    pub top_k: usize,
    /// Required metadata equality filters (all must match).
    pub filters: BTreeMap<String, String>,
    /// Optional exact phrase to boost (case-insensitive substring presence in
    /// the document's indexable text adds [`SearchQuery::phrase_boost`]).
    pub phrase: Option<String>,
    /// Multiplicative boost applied to the score when the phrase is present.
    pub phrase_boost: f64,
}

impl SearchQuery {
    /// Builds a default BM25 query returning up to `top_k` hits.
    pub fn new(text: impl Into<String>, top_k: usize) -> Self {
        Self {
            text: text.into(),
            ranking: SearchRanking::Bm25,
            top_k,
            filters: BTreeMap::new(),
            phrase: None,
            phrase_boost: 1.5,
        }
    }

    /// Sets the ranking model.
    pub fn with_ranking(mut self, ranking: SearchRanking) -> Self {
        self.ranking = ranking;
        self
    }

    /// Adds a metadata equality filter.
    pub fn with_filter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters.insert(key.into(), value.into());
        self
    }

    /// Sets a phrase to boost and the multiplicative boost factor.
    pub fn with_phrase(mut self, phrase: impl Into<String>, boost: f64) -> Self {
        self.phrase = Some(phrase.into());
        self.phrase_boost = boost.max(1.0);
        self
    }
}

/// A single search result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchHit {
    /// Matched document id.
    pub id: String,
    /// Relevance score (BM25 magnitude, or TF-IDF cosine in `[0, 1]` before any
    /// phrase boost).
    pub score: f64,
    /// Distinct query terms that matched.
    pub matched_terms: Vec<String>,
    /// Whether the boosting phrase was present (if one was supplied).
    pub phrase_matched: bool,
}

/// Aggregate statistics about the index.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexStatistics {
    /// Number of indexed documents.
    pub num_documents: usize,
    /// Distinct terms in the vocabulary.
    pub vocabulary_size: usize,
    /// Average document length in tokens.
    pub avg_doc_length: f64,
    /// Total indexed tokens.
    pub total_tokens: usize,
}

/// An append-only, full-text-searchable in-memory index.
#[derive(Debug, Clone)]
pub struct SearchIndex {
    documents: Vec<IndexedDocument>,
    id_to_index: HashMap<String, usize>,
    inverted_index: HashMap<String, Vec<(usize, u32)>>,
    doc_terms: Vec<HashMap<String, u32>>,
    doc_lengths: Vec<usize>,
    total_tokens: usize,
    k1: f64,
    b: f64,
}

impl Default for SearchIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchIndex {
    /// Creates an empty index with standard BM25 parameters (`k1 = 1.5`,
    /// `b = 0.75`).
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
            id_to_index: HashMap::new(),
            inverted_index: HashMap::new(),
            doc_terms: Vec::new(),
            doc_lengths: Vec::new(),
            total_tokens: 0,
            k1: 1.5,
            b: 0.75,
        }
    }

    /// Overrides BM25 `k1` and `b`.
    pub fn with_bm25_params(mut self, k1: f64, b: f64) -> Self {
        self.k1 = k1.max(0.0);
        self.b = b.clamp(0.0, 1.0);
        self
    }

    /// Adds a document and updates the inverted index. Errors on duplicate id.
    pub fn add(&mut self, document: IndexedDocument) -> Result<(), String> {
        if self.id_to_index.contains_key(&document.id) {
            return Err(format!("duplicate document id: {}", document.id));
        }
        let idx = self.documents.len();
        let tokens = tokenize(&document.indexable_text());

        let mut term_freq: HashMap<String, u32> = HashMap::new();
        for token in &tokens {
            *term_freq.entry(token.clone()).or_insert(0) += 1;
        }
        for (term, freq) in &term_freq {
            self.inverted_index
                .entry(term.clone())
                .or_default()
                .push((idx, *freq));
        }

        self.doc_lengths.push(tokens.len());
        self.total_tokens += tokens.len();
        self.doc_terms.push(term_freq);
        self.id_to_index.insert(document.id.clone(), idx);
        self.documents.push(document);
        Ok(())
    }

    /// Adds many documents, stopping at the first error.
    pub fn add_many<I>(&mut self, documents: I) -> Result<(), String>
    where
        I: IntoIterator<Item = IndexedDocument>,
    {
        for document in documents {
            self.add(document)?;
        }
        Ok(())
    }

    /// Number of indexed documents.
    pub fn len(&self) -> usize {
        self.documents.len()
    }

    /// Whether the index is empty.
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }

    /// Looks up a document by id.
    pub fn get(&self, id: &str) -> Option<&IndexedDocument> {
        self.id_to_index.get(id).map(|&i| &self.documents[i])
    }

    /// All indexed documents.
    pub fn documents(&self) -> &[IndexedDocument] {
        &self.documents
    }

    /// Document frequency of a term (after stemming).
    pub fn document_frequency(&self, term: &str) -> usize {
        let stemmed = stem(&term.to_lowercase());
        self.inverted_index
            .get(&stemmed)
            .map(|p| p.len())
            .unwrap_or(0)
    }

    /// Aggregate index statistics.
    pub fn statistics(&self) -> IndexStatistics {
        let n = self.documents.len();
        let avg = if n == 0 {
            0.0
        } else {
            self.total_tokens as f64 / n as f64
        };
        IndexStatistics {
            num_documents: n,
            vocabulary_size: self.inverted_index.len(),
            avg_doc_length: avg,
            total_tokens: self.total_tokens,
        }
    }

    /// Convenience BM25 search returning up to `top_k` hits.
    pub fn search(&self, query: &str, top_k: usize) -> Vec<SearchHit> {
        self.search_with(&SearchQuery::new(query, top_k))
    }

    /// Searches with a fully-specified [`SearchQuery`].
    pub fn search_with(&self, query: &SearchQuery) -> Vec<SearchHit> {
        let phrase_lower = query.phrase.as_ref().map(|p| p.to_lowercase());
        let predicate = |doc: &IndexedDocument| {
            query
                .filters
                .iter()
                .all(|(k, v)| doc.metadata.get(k).is_some_and(|mv| mv == v))
        };

        let mut hits = self.score_query(&query.text, query.ranking, predicate);

        // Apply phrase boost.
        if let Some(phrase) = &phrase_lower {
            for hit in hits.iter_mut() {
                if let Some(&idx) = self.id_to_index.get(&hit.id) {
                    let present = self.documents[idx]
                        .indexable_text()
                        .to_lowercase()
                        .contains(phrase);
                    hit.phrase_matched = present;
                    if present {
                        hit.score *= query.phrase_boost;
                    }
                }
            }
            // Re-sort after boosting.
            hits.sort_by(|a, b| {
                b.score
                    .partial_cmp(&a.score)
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| a.id.cmp(&b.id))
            });
        }

        hits.truncate(query.top_k);
        hits
    }

    /// Finds documents similar to an existing one ("more like this") using its
    /// own indexed text as a TF-IDF query.
    pub fn find_similar(&self, id: &str, top_k: usize) -> Vec<SearchHit> {
        let Some(document) = self.get(id) else {
            return Vec::new();
        };
        let query = SearchQuery::new(document.indexable_text(), top_k + 1)
            .with_ranking(SearchRanking::TfIdf);
        let mut hits = self.search_with(&query);
        hits.retain(|h| h.id != id);
        hits.truncate(top_k);
        hits
    }

    /// Resolves a hit back to its document.
    pub fn resolve(&self, hit: &SearchHit) -> Option<&IndexedDocument> {
        self.get(&hit.id)
    }

    // --- internal scoring ---------------------------------------------------

    fn idf_bm25(&self, df: usize) -> f64 {
        let n = self.documents.len() as f64;
        (((n - df as f64 + 0.5) / (df as f64 + 0.5)) + 1.0).ln()
    }

    fn idf_tfidf(&self, df: usize) -> f64 {
        let n = self.documents.len() as f64;
        (n / (1.0 + df as f64)).ln() + 1.0
    }

    fn doc_norm_tfidf(&self, doc: usize) -> f64 {
        let mut sum_sq = 0.0;
        for (term, &freq) in &self.doc_terms[doc] {
            let df = self.inverted_index.get(term).map(|p| p.len()).unwrap_or(1);
            let weight = freq as f64 * self.idf_tfidf(df);
            sum_sq += weight * weight;
        }
        sum_sq.sqrt()
    }

    fn score_query<F>(&self, query: &str, ranking: SearchRanking, predicate: F) -> Vec<SearchHit>
    where
        F: Fn(&IndexedDocument) -> bool,
    {
        let query_tokens = tokenize(query);
        if query_tokens.is_empty() || self.documents.is_empty() {
            return Vec::new();
        }
        let mut query_tf: HashMap<String, u32> = HashMap::new();
        for token in &query_tokens {
            *query_tf.entry(token.clone()).or_insert(0) += 1;
        }

        let n = self.documents.len() as f64;
        let avgdl = (self.total_tokens as f64 / n).max(1.0);

        let mut scores: HashMap<usize, f64> = HashMap::new();
        let mut matched: HashMap<usize, BTreeSet<String>> = HashMap::new();
        let mut query_norm_sq = 0.0;

        for (term, &qf) in &query_tf {
            let Some(postings) = self.inverted_index.get(term) else {
                continue;
            };
            let df = postings.len();
            match ranking {
                SearchRanking::Bm25 => {
                    let idf = self.idf_bm25(df);
                    for &(doc, freq) in postings {
                        if !predicate(&self.documents[doc]) {
                            continue;
                        }
                        let freq = freq as f64;
                        let dl = self.doc_lengths[doc] as f64;
                        let denom = freq + self.k1 * (1.0 - self.b + self.b * dl / avgdl);
                        let contrib = idf * (freq * (self.k1 + 1.0)) / denom;
                        *scores.entry(doc).or_insert(0.0) += contrib;
                        matched.entry(doc).or_default().insert(term.clone());
                    }
                }
                SearchRanking::TfIdf => {
                    let idf = self.idf_tfidf(df);
                    let q_weight = qf as f64 * idf;
                    query_norm_sq += q_weight * q_weight;
                    for &(doc, freq) in postings {
                        if !predicate(&self.documents[doc]) {
                            continue;
                        }
                        let d_weight = freq as f64 * idf;
                        *scores.entry(doc).or_insert(0.0) += q_weight * d_weight;
                        matched.entry(doc).or_default().insert(term.clone());
                    }
                }
            }
        }

        if ranking == SearchRanking::TfIdf {
            let query_norm = query_norm_sq.sqrt();
            for (&doc, score) in scores.iter_mut() {
                let doc_norm = self.doc_norm_tfidf(doc);
                if query_norm > 0.0 && doc_norm > 0.0 {
                    *score /= query_norm * doc_norm;
                } else {
                    *score = 0.0;
                }
            }
        }

        let mut hits: Vec<SearchHit> = scores
            .into_iter()
            .map(|(doc, score)| SearchHit {
                id: self.documents[doc].id.clone(),
                score,
                matched_terms: matched
                    .remove(&doc)
                    .map(|s| s.into_iter().collect())
                    .unwrap_or_default(),
                phrase_matched: false,
            })
            .collect();
        hits.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(Ordering::Equal)
                .then_with(|| a.id.cmp(&b.id))
        });
        hits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_index() -> SearchIndex {
        let mut index = SearchIndex::new();
        index
            .add_many(vec![
                IndexedDocument::new(
                    "d1",
                    "Force majeure clause drafting",
                    "A force majeure clause excuses performance when extraordinary events \
                     beyond the control of the parties prevent fulfilment of the contract.",
                )
                .with_metadata("practice_area", "contracts")
                .with_metadata("author", "Smith"),
                IndexedDocument::new(
                    "d2",
                    "Indemnification provisions",
                    "An indemnification provision allocates risk by requiring one party to \
                     compensate another for specified losses and liabilities.",
                )
                .with_metadata("practice_area", "contracts")
                .with_metadata("author", "Jones"),
                IndexedDocument::new(
                    "d3",
                    "Negligence and duty of care",
                    "Negligence requires a duty of care, breach of that duty, causation, and \
                     damages suffered by the plaintiff.",
                )
                .with_metadata("practice_area", "torts")
                .with_metadata("author", "Smith"),
            ])
            .expect("index builds");
        index
    }

    #[test]
    fn test_add_and_duplicate() {
        let mut index = SearchIndex::new();
        let doc = IndexedDocument::new("x", "T", "body about contracts");
        index.add(doc.clone()).expect("first add");
        assert_eq!(index.len(), 1);
        assert!(index.add(doc).is_err());
    }

    #[test]
    fn test_bm25_relevance() {
        let index = sample_index();
        let hits = index.search("force majeure performance excused", 5);
        assert!(!hits.is_empty());
        assert_eq!(hits[0].id, "d1");
        assert!(hits[0].score > 0.0);
        assert!(
            hits[0]
                .matched_terms
                .iter()
                .any(|t| t.contains("majeur") || t == "perform")
        );
    }

    #[test]
    fn test_metadata_filter() {
        let index = sample_index();
        let query =
            SearchQuery::new("duty care negligence", 10).with_filter("practice_area", "torts");
        let hits = index.search_with(&query);
        assert!(hits.iter().all(|h| h.id == "d3"));
        assert_eq!(hits.len(), 1);

        // Author filter narrowing.
        let q2 = SearchQuery::new("clause provision risk", 10).with_filter("author", "Jones");
        let hits2 = index.search_with(&q2);
        assert!(hits2.iter().all(|h| h.id == "d2"));
    }

    #[test]
    fn test_tfidf_bounded() {
        let index = sample_index();
        let query = SearchQuery::new("indemnification risk losses liabilities", 5)
            .with_ranking(SearchRanking::TfIdf);
        let hits = index.search_with(&query);
        assert!(!hits.is_empty());
        assert_eq!(hits[0].id, "d2");
        for hit in &hits {
            assert!(hit.score >= 0.0 && hit.score <= 1.0 + 1e-9);
        }
    }

    #[test]
    fn test_phrase_boost() {
        let index = sample_index();
        let base = index.search("clause performance contract parties", 5);
        let base_d1 = base
            .iter()
            .find(|h| h.id == "d1")
            .map(|h| h.score)
            .unwrap_or(0.0);

        let boosted_query = SearchQuery::new("clause performance contract parties", 5)
            .with_phrase("force majeure", 2.0);
        let boosted = index.search_with(&boosted_query);
        let boosted_d1 = boosted.iter().find(|h| h.id == "d1").expect("d1 present");
        assert!(boosted_d1.phrase_matched);
        assert!(boosted_d1.score > base_d1);
    }

    #[test]
    fn test_find_similar() {
        let index = sample_index();
        let hits = index.find_similar("d1", 5);
        assert!(hits.iter().all(|h| h.id != "d1"));
        // d2 (contracts) should rank above d3 (torts) for the contracts doc.
        if hits.len() >= 2 {
            let d2_pos = hits.iter().position(|h| h.id == "d2");
            let d3_pos = hits.iter().position(|h| h.id == "d3");
            if let (Some(p2), Some(p3)) = (d2_pos, d3_pos) {
                assert!(p2 < p3);
            }
        }
    }

    #[test]
    fn test_statistics_and_df() {
        let index = sample_index();
        let stats = index.statistics();
        assert_eq!(stats.num_documents, 3);
        assert!(stats.vocabulary_size > 0);
        assert!(stats.avg_doc_length > 0.0);
        // "clause" appears in d1 (and stemmed forms).
        assert!(index.document_frequency("clause") >= 1);
        assert_eq!(index.document_frequency("zzzznonexistent"), 0);
    }

    #[test]
    fn test_empty_query() {
        let index = sample_index();
        assert!(index.search("   ", 5).is_empty());
        assert!(index.search("", 5).is_empty());
    }
}
