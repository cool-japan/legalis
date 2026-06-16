//! In-memory research corpus with TF-IDF and BM25 ranking.
//!
//! [`ResearchCorpus`] maintains an append-only collection of
//! [`LegalAuthority`] entries and an inverted index built incrementally as
//! authorities are added. It supports full-text ranked retrieval using either
//! the Okapi BM25 model or classic TF-IDF cosine similarity, with optional
//! filtering by authority type, jurisdiction and treatment status.

use super::{AuthorityType, LegalAuthority, tokenize};
use crate::Jurisdiction;
use anyhow::{Result, bail};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

/// Ranking model used for retrieval.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankingMethod {
    /// Okapi BM25 probabilistic ranking.
    Bm25,
    /// TF-IDF cosine similarity.
    TfIdf,
}

/// Options controlling a corpus search.
#[derive(Debug, Clone)]
pub struct SearchOptions {
    /// Ranking model.
    pub method: RankingMethod,
    /// Maximum number of hits to return.
    pub top_k: usize,
    /// Restrict to a single authority type.
    pub authority_type: Option<AuthorityType>,
    /// Restrict to a single jurisdiction.
    pub jurisdiction: Option<Jurisdiction>,
    /// Drop authorities that are no longer good law (overruled).
    pub exclude_overruled: bool,
}

impl SearchOptions {
    /// Creates default options returning up to `top_k` BM25-ranked hits.
    pub fn new(top_k: usize) -> Self {
        Self {
            method: RankingMethod::Bm25,
            top_k,
            authority_type: None,
            jurisdiction: None,
            exclude_overruled: false,
        }
    }

    /// Sets the ranking method.
    pub fn with_method(mut self, method: RankingMethod) -> Self {
        self.method = method;
        self
    }

    /// Restricts results to a single authority type.
    pub fn with_authority_type(mut self, authority_type: AuthorityType) -> Self {
        self.authority_type = Some(authority_type);
        self
    }

    /// Restricts results to a single jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Excludes overruled authorities from the results.
    pub fn exclude_overruled(mut self, exclude: bool) -> Self {
        self.exclude_overruled = exclude;
        self
    }
}

/// A single search result.
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchHit {
    /// Identifier of the matched authority.
    pub id: String,
    /// Relevance score (BM25 magnitude or TF-IDF cosine in `[0, 1]`).
    pub score: f64,
    /// Distinct query terms that matched this authority.
    pub matched_terms: Vec<String>,
}

/// Aggregate statistics about a corpus.
#[derive(Debug, Clone, PartialEq)]
pub struct CorpusStatistics {
    /// Number of indexed authorities.
    pub num_authorities: usize,
    /// Number of distinct terms in the vocabulary.
    pub vocabulary_size: usize,
    /// Average document length in tokens.
    pub avg_doc_length: f64,
    /// Total number of indexed tokens.
    pub total_tokens: usize,
}

/// An append-only, full-text-searchable corpus of legal authorities.
#[derive(Debug, Clone)]
pub struct ResearchCorpus {
    authorities: Vec<LegalAuthority>,
    id_to_index: HashMap<String, usize>,
    inverted_index: HashMap<String, Vec<(usize, u32)>>,
    doc_terms: Vec<HashMap<String, u32>>,
    doc_lengths: Vec<usize>,
    total_tokens: usize,
    k1: f64,
    b: f64,
}

impl Default for ResearchCorpus {
    fn default() -> Self {
        Self::new()
    }
}

impl ResearchCorpus {
    /// Creates an empty corpus with standard BM25 parameters (`k1 = 1.5`,
    /// `b = 0.75`).
    pub fn new() -> Self {
        Self {
            authorities: Vec::new(),
            id_to_index: HashMap::new(),
            inverted_index: HashMap::new(),
            doc_terms: Vec::new(),
            doc_lengths: Vec::new(),
            total_tokens: 0,
            k1: 1.5,
            b: 0.75,
        }
    }

    /// Overrides the BM25 `k1` (term-frequency saturation) and `b` (length
    /// normalisation) parameters.
    pub fn with_bm25_params(mut self, k1: f64, b: f64) -> Self {
        self.k1 = k1.max(0.0);
        self.b = b.clamp(0.0, 1.0);
        self
    }

    /// Adds an authority and updates the inverted index incrementally.
    ///
    /// Returns an error if an authority with the same id already exists.
    pub fn add(&mut self, authority: LegalAuthority) -> Result<()> {
        if self.id_to_index.contains_key(&authority.id) {
            bail!("duplicate authority id: {}", authority.id);
        }

        let idx = self.authorities.len();
        let tokens = tokenize(&authority.indexable_text());

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
        self.id_to_index.insert(authority.id.clone(), idx);
        self.authorities.push(authority);
        Ok(())
    }

    /// Adds many authorities, stopping at the first error.
    pub fn add_many<I>(&mut self, authorities: I) -> Result<()>
    where
        I: IntoIterator<Item = LegalAuthority>,
    {
        for authority in authorities {
            self.add(authority)?;
        }
        Ok(())
    }

    /// Number of indexed authorities.
    pub fn len(&self) -> usize {
        self.authorities.len()
    }

    /// Returns whether the corpus is empty.
    pub fn is_empty(&self) -> bool {
        self.authorities.is_empty()
    }

    /// Looks up an authority by id.
    pub fn get(&self, id: &str) -> Option<&LegalAuthority> {
        self.id_to_index.get(id).map(|&i| &self.authorities[i])
    }

    /// Returns all indexed authorities.
    pub fn authorities(&self) -> &[LegalAuthority] {
        &self.authorities
    }

    /// Returns the document frequency of a term (after stemming).
    pub fn document_frequency(&self, term: &str) -> usize {
        let stemmed = super::stem(&term.to_lowercase());
        self.inverted_index
            .get(&stemmed)
            .map(|p| p.len())
            .unwrap_or(0)
    }

    /// Returns aggregate corpus statistics.
    pub fn statistics(&self) -> CorpusStatistics {
        let n = self.authorities.len();
        let avg = if n == 0 {
            0.0
        } else {
            self.total_tokens as f64 / n as f64
        };
        CorpusStatistics {
            num_authorities: n,
            vocabulary_size: self.inverted_index.len(),
            avg_doc_length: avg,
            total_tokens: self.total_tokens,
        }
    }

    /// Searches the corpus with BM25, returning up to `top_k` hits.
    pub fn search(&self, query: &str, top_k: usize) -> Vec<ResearchHit> {
        self.search_with(query, &SearchOptions::new(top_k))
    }

    /// Searches the corpus with fully-specified [`SearchOptions`].
    pub fn search_with(&self, query: &str, options: &SearchOptions) -> Vec<ResearchHit> {
        let predicate = |authority: &LegalAuthority| {
            if options
                .authority_type
                .is_some_and(|t| authority.authority_type != t)
            {
                return false;
            }
            if options
                .jurisdiction
                .as_ref()
                .is_some_and(|j| &authority.jurisdiction != j)
            {
                return false;
            }
            if options.exclude_overruled && !authority.is_good_law() {
                return false;
            }
            true
        };

        let mut hits = self.score_query(query, options.method, predicate);
        hits.truncate(options.top_k);
        hits
    }

    /// Convenience: searches only case law, optionally within a jurisdiction.
    pub fn search_cases(
        &self,
        query: &str,
        top_k: usize,
        jurisdiction: Option<Jurisdiction>,
    ) -> Vec<ResearchHit> {
        let mut options = SearchOptions::new(top_k).with_authority_type(AuthorityType::Case);
        options.jurisdiction = jurisdiction;
        self.search_with(query, &options)
    }

    /// Convenience: searches only statutes, optionally within a jurisdiction.
    pub fn search_statutes(
        &self,
        query: &str,
        top_k: usize,
        jurisdiction: Option<Jurisdiction>,
    ) -> Vec<ResearchHit> {
        let mut options = SearchOptions::new(top_k).with_authority_type(AuthorityType::Statute);
        options.jurisdiction = jurisdiction;
        self.search_with(query, &options)
    }

    /// Finds authorities similar to an existing one ("more like this") by
    /// using its own indexed text as the query.
    pub fn find_similar(&self, id: &str, top_k: usize) -> Vec<ResearchHit> {
        let Some(authority) = self.get(id) else {
            return Vec::new();
        };
        let query = authority.indexable_text();
        let options = SearchOptions::new(top_k + 1).with_method(RankingMethod::TfIdf);
        let mut hits = self.search_with(&query, &options);
        hits.retain(|h| h.id != id);
        hits.truncate(top_k);
        hits
    }

    /// Resolves a hit back to its authority.
    pub fn resolve(&self, hit: &ResearchHit) -> Option<&LegalAuthority> {
        self.get(&hit.id)
    }

    // --- internal scoring ---------------------------------------------------

    fn idf_bm25(&self, df: usize) -> f64 {
        let n = self.authorities.len() as f64;
        (((n - df as f64 + 0.5) / (df as f64 + 0.5)) + 1.0).ln()
    }

    fn idf_tfidf(&self, df: usize) -> f64 {
        let n = self.authorities.len() as f64;
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

    fn score_query<F>(&self, query: &str, method: RankingMethod, predicate: F) -> Vec<ResearchHit>
    where
        F: Fn(&LegalAuthority) -> bool,
    {
        let query_tokens = tokenize(query);
        if query_tokens.is_empty() || self.authorities.is_empty() {
            return Vec::new();
        }

        let mut query_tf: HashMap<String, u32> = HashMap::new();
        for token in &query_tokens {
            *query_tf.entry(token.clone()).or_insert(0) += 1;
        }

        let n = self.authorities.len() as f64;
        let avgdl = (self.total_tokens as f64 / n).max(1.0);

        let mut scores: HashMap<usize, f64> = HashMap::new();
        let mut matched: HashMap<usize, BTreeSet<String>> = HashMap::new();
        let mut query_norm_sq = 0.0;

        for (term, &qf) in &query_tf {
            let Some(postings) = self.inverted_index.get(term) else {
                continue;
            };
            let df = postings.len();

            match method {
                RankingMethod::Bm25 => {
                    let idf = self.idf_bm25(df);
                    for &(doc, freq) in postings {
                        if !predicate(&self.authorities[doc]) {
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
                RankingMethod::TfIdf => {
                    let idf = self.idf_tfidf(df);
                    let q_weight = qf as f64 * idf;
                    query_norm_sq += q_weight * q_weight;
                    for &(doc, freq) in postings {
                        if !predicate(&self.authorities[doc]) {
                            continue;
                        }
                        let d_weight = freq as f64 * idf;
                        *scores.entry(doc).or_insert(0.0) += q_weight * d_weight;
                        matched.entry(doc).or_default().insert(term.clone());
                    }
                }
            }
        }

        if method == RankingMethod::TfIdf {
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

        let mut hits: Vec<ResearchHit> = scores
            .into_iter()
            .map(|(doc, score)| ResearchHit {
                id: self.authorities[doc].id.clone(),
                score,
                matched_terms: matched
                    .remove(&doc)
                    .map(|s| s.into_iter().collect())
                    .unwrap_or_default(),
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
    use crate::CourtLevel;

    fn sample_corpus() -> ResearchCorpus {
        let mut corpus = ResearchCorpus::new();
        corpus
            .add_many(vec![
                LegalAuthority::new(
                    "neg1",
                    "Palsgraf v. Long Island Railroad",
                    "248 N.Y. 339",
                    "Negligence requires a duty of care owed to a foreseeable plaintiff; \
                     liability does not extend to unforeseeable consequences.",
                    AuthorityType::Case,
                    Jurisdiction::UsState("New York".into()),
                )
                .with_court_level(CourtLevel::Appellate)
                .with_year(1928)
                .with_citation_count(5000),
                LegalAuthority::new(
                    "con1",
                    "Restatement of Contracts on Consideration",
                    "Restatement (Second) of Contracts § 71",
                    "A contract requires offer, acceptance and consideration; a bargained-for \
                     exchange of promises forms a binding agreement.",
                    AuthorityType::SecondarySource,
                    Jurisdiction::UsFederal,
                )
                .with_year(1981),
                LegalAuthority::new(
                    "stat1",
                    "Civil Rights Act Section 1983",
                    "42 U.S.C. § 1983",
                    "Every person who under color of law subjects any citizen to a deprivation \
                     of constitutional rights shall be liable to the party injured.",
                    AuthorityType::Statute,
                    Jurisdiction::UsFederal,
                )
                .with_year(1871)
                .with_citation_count(12000),
            ])
            .expect("sample corpus builds");
        corpus
    }

    #[test]
    fn test_add_and_duplicate_detection() {
        let mut corpus = ResearchCorpus::new();
        let auth = LegalAuthority::new(
            "x",
            "Title",
            "1 U.S. 1",
            "some text about contracts",
            AuthorityType::Case,
            Jurisdiction::UsFederal,
        );
        corpus.add(auth.clone()).expect("first add succeeds");
        assert_eq!(corpus.len(), 1);
        assert!(corpus.add(auth).is_err());
    }

    #[test]
    fn test_bm25_ranking_relevance() {
        let corpus = sample_corpus();
        let hits = corpus.search("duty of care negligence foreseeable", 5);
        assert!(!hits.is_empty());
        assert_eq!(hits[0].id, "neg1");
        assert!(hits[0].score > 0.0);
        assert!(
            hits[0]
                .matched_terms
                .iter()
                .any(|t| t == "neglig" || t == "duty")
        );
    }

    #[test]
    fn test_statute_search_filter() {
        let corpus = sample_corpus();
        let hits =
            corpus.search_statutes("constitutional rights deprivation color of law", 5, None);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].id, "stat1");
    }

    #[test]
    fn test_jurisdiction_filter() {
        let corpus = sample_corpus();
        let opts =
            SearchOptions::new(10).with_jurisdiction(Jurisdiction::UsState("New York".into()));
        let hits = corpus.search_with("liability", &opts);
        assert!(hits.iter().all(|h| h.id == "neg1"));
    }

    #[test]
    fn test_tfidf_cosine_bounded() {
        let corpus = sample_corpus();
        let opts = SearchOptions::new(5).with_method(RankingMethod::TfIdf);
        let hits = corpus.search_with("contract offer acceptance consideration", &opts);
        assert!(!hits.is_empty());
        assert_eq!(hits[0].id, "con1");
        for hit in &hits {
            assert!(hit.score >= 0.0 && hit.score <= 1.0 + 1e-9);
        }
    }

    #[test]
    fn test_statistics_and_document_frequency() {
        let corpus = sample_corpus();
        let stats = corpus.statistics();
        assert_eq!(stats.num_authorities, 3);
        assert!(stats.vocabulary_size > 0);
        assert!(stats.avg_doc_length > 0.0);
        // "contract" appears (stemmed) in the consideration restatement
        assert!(corpus.document_frequency("contracts") >= 1);
        assert_eq!(corpus.document_frequency("nonexistentterm"), 0);
    }

    #[test]
    fn test_find_similar_excludes_self() {
        let corpus = sample_corpus();
        let hits = corpus.find_similar("neg1", 5);
        assert!(hits.iter().all(|h| h.id != "neg1"));
    }

    #[test]
    fn test_empty_query_returns_nothing() {
        let corpus = sample_corpus();
        assert!(corpus.search("   ", 5).is_empty());
        assert!(corpus.search("", 5).is_empty());
    }
}
