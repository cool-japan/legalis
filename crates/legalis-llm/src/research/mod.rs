//! Legal Research Assistant
//!
//! A self-contained, pure-Rust legal research engine that operates over an
//! in-memory corpus of legal authorities (cases, statutes, regulations,
//! constitutional provisions and secondary sources). None of the features in
//! this module require a live LLM call to function: case-law and statute
//! search are powered by a TF-IDF / BM25 inverted index, citations are parsed
//! and validated with a hand-written recogniser, precedent is analysed with a
//! court-hierarchy model, authority strength is ranked from recency, court
//! level, citation count and subsequent treatment, legal issues are spotted
//! with an extensible issue catalogue, and research memos are generated in an
//! IRAC structure.
//!
//! Where a [`crate::LLMProvider`] is available the generated memo can be
//! *optionally* enriched (see [`assistant::LegalResearchAssistant::augment_memo`]),
//! but the assistant is fully functional offline.
//!
//! ## Sub-modules
//!
//! * [`corpus`] - the searchable in-memory authority index (TF-IDF + BM25).
//! * [`citation`] - citation parsing, normalisation and validation.
//! * [`precedent`] - binding/persuasive precedent analysis.
//! * [`authority`] - authority strength ranking.
//! * [`issues`] - legal issue identification (issue spotting).
//! * [`memo`] - IRAC-style research memo generation.
//! * [`assistant`] - top-level orchestrator that ties everything together.

mod assistant;
mod authority;
mod citation;
mod corpus;
mod issues;
mod memo;
mod precedent;

pub use assistant::*;
pub use authority::*;
pub use citation::*;
pub use corpus::*;
pub use issues::*;
pub use memo::*;
pub use precedent::*;

use crate::{CourtLevel, Jurisdiction, TreatmentType};
use serde::{Deserialize, Serialize};

// ============================================================================
// Shared data model
// ============================================================================

/// The kind of legal authority an entry in the research corpus represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthorityType {
    /// A judicial decision / case law.
    Case,
    /// A legislative statute.
    Statute,
    /// An administrative regulation.
    Regulation,
    /// A constitutional provision.
    Constitution,
    /// A secondary source (treatise, law review, restatement, etc.).
    SecondarySource,
}

impl AuthorityType {
    /// Returns whether this authority is primary (binding-capable) law.
    ///
    /// Secondary sources are never primary authority - they can only ever be
    /// persuasive regardless of jurisdiction.
    pub fn is_primary(&self) -> bool {
        !matches!(self, AuthorityType::SecondarySource)
    }

    /// Returns a human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            AuthorityType::Case => "case",
            AuthorityType::Statute => "statute",
            AuthorityType::Regulation => "regulation",
            AuthorityType::Constitution => "constitutional provision",
            AuthorityType::SecondarySource => "secondary source",
        }
    }
}

/// A single legal authority stored in the research corpus.
///
/// The `text` field is the body that gets indexed for search (a headnote,
/// summary, operative text or full opinion). All other fields feed precedent
/// analysis, authority ranking and jurisdiction-specific filtering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalAuthority {
    /// Stable unique identifier.
    pub id: String,
    /// Title (case name, statute heading, etc.).
    pub title: String,
    /// Canonical citation string.
    pub citation: String,
    /// Indexed body text.
    pub text: String,
    /// Kind of authority.
    pub authority_type: AuthorityType,
    /// Jurisdiction the authority belongs to.
    pub jurisdiction: Jurisdiction,
    /// Court level (only meaningful for case law).
    pub court_level: Option<CourtLevel>,
    /// Year decided / enacted.
    pub year: Option<i32>,
    /// How many times this authority has been cited by others.
    pub citation_count: u32,
    /// Current treatment of the authority (overruled, followed, ...).
    pub treatment: Option<TreatmentType>,
    /// Topical tags / subject areas.
    pub topics: Vec<String>,
}

impl LegalAuthority {
    /// Creates a new authority with the mandatory fields.
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        citation: impl Into<String>,
        text: impl Into<String>,
        authority_type: AuthorityType,
        jurisdiction: Jurisdiction,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            citation: citation.into(),
            text: text.into(),
            authority_type,
            jurisdiction,
            court_level: None,
            year: None,
            citation_count: 0,
            treatment: None,
            topics: Vec::new(),
        }
    }

    /// Sets the court level.
    pub fn with_court_level(mut self, level: CourtLevel) -> Self {
        self.court_level = Some(level);
        self
    }

    /// Sets the year.
    pub fn with_year(mut self, year: i32) -> Self {
        self.year = Some(year);
        self
    }

    /// Sets the citation count.
    pub fn with_citation_count(mut self, count: u32) -> Self {
        self.citation_count = count;
        self
    }

    /// Sets the treatment.
    pub fn with_treatment(mut self, treatment: TreatmentType) -> Self {
        self.treatment = Some(treatment);
        self
    }

    /// Adds a topical tag.
    pub fn with_topic(mut self, topic: impl Into<String>) -> Self {
        self.topics.push(topic.into());
        self
    }

    /// Returns the text used for full-text indexing (title + body).
    pub fn indexable_text(&self) -> String {
        format!("{} {}", self.title, self.text)
    }

    /// Returns whether the authority is still good law.
    ///
    /// An authority is considered no longer good law if it has been overruled.
    pub fn is_good_law(&self) -> bool {
        !matches!(self.treatment, Some(TreatmentType::Overruled))
    }
}

/// A forum (the court in which an issue is being litigated).
///
/// Used by precedent analysis to determine whether a given authority is
/// binding or merely persuasive in this court.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Forum {
    /// Jurisdiction of the forum court.
    pub jurisdiction: Jurisdiction,
    /// Level of the forum court.
    pub court_level: CourtLevel,
}

impl Forum {
    /// Creates a new forum.
    pub fn new(jurisdiction: Jurisdiction, court_level: CourtLevel) -> Self {
        Self {
            jurisdiction,
            court_level,
        }
    }
}

// ============================================================================
// Shared helpers
// ============================================================================

/// Returns the relative authority weight of a court level in `[0, 1]`.
///
/// Higher courts carry more weight: a supreme court decision outranks a
/// circuit decision, which outranks an intermediate appellate decision, which
/// outranks a trial decision.
pub(crate) fn court_authority_weight(level: CourtLevel) -> f64 {
    match level {
        CourtLevel::Supreme => 1.0,
        CourtLevel::Circuit => 0.85,
        CourtLevel::Appellate => 0.65,
        CourtLevel::Trial => 0.35,
    }
}

/// Returns a monotone rank for a court level (higher = more authoritative).
///
/// Used to compare two courts within the same hierarchy.
pub(crate) fn court_rank(level: CourtLevel) -> u8 {
    match level {
        CourtLevel::Trial => 1,
        CourtLevel::Appellate => 2,
        CourtLevel::Circuit => 3,
        CourtLevel::Supreme => 4,
    }
}

/// Returns whether a jurisdiction is part of the United States system.
pub(crate) fn is_us_jurisdiction(jurisdiction: &Jurisdiction) -> bool {
    matches!(
        jurisdiction,
        Jurisdiction::UsFederal | Jurisdiction::UsState(_)
    )
}

// ============================================================================
// Text utilities (shared tokeniser + conservative stemmer)
// ============================================================================

/// Returns whether a lowercase token is a stopword.
///
/// The list deliberately excludes legally-loaded modal verbs (`shall`, `may`,
/// `must`, `will`) so that obligation/permission language survives tokenisation.
pub(crate) fn is_stopword(token: &str) -> bool {
    const STOPWORDS: &[&str] = &[
        "a", "an", "and", "are", "as", "at", "be", "been", "being", "but", "by", "for", "from",
        "had", "has", "have", "he", "her", "his", "i", "if", "in", "into", "is", "it", "its", "of",
        "on", "or", "our", "out", "she", "so", "than", "that", "the", "their", "them", "then",
        "there", "these", "they", "this", "those", "to", "up", "was", "we", "were", "what", "when",
        "where", "which", "who", "whom", "whose", "why", "with", "would", "you", "your",
    ];
    STOPWORDS.contains(&token)
}

/// Tokenises text into normalised, stemmed terms.
///
/// The pipeline is: split on non-alphanumeric boundaries, lowercase, drop
/// single characters and stopwords, then apply a conservative inflectional
/// stemmer. The same pipeline is applied to both documents and queries so that
/// surface variations match consistently.
pub(crate) fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for raw in text.split(|c: char| !c.is_alphanumeric()) {
        if raw.is_empty() {
            continue;
        }
        let lower = raw.to_lowercase();
        if lower.chars().count() < 2 {
            continue;
        }
        if is_stopword(&lower) {
            continue;
        }
        let stemmed = stem(&lower);
        if !stemmed.is_empty() {
            tokens.push(stemmed);
        }
    }
    tokens
}

/// Computes the cosine similarity between two free-text passages.
///
/// Both passages are run through [`tokenize`], turned into term-frequency
/// vectors and compared with cosine similarity. The result lies in `[0, 1]`
/// (negative values are impossible because term frequencies are non-negative).
pub(crate) fn text_cosine_similarity(a: &str, b: &str) -> f64 {
    use std::collections::HashMap;

    let tokens_a = tokenize(a);
    let tokens_b = tokenize(b);
    if tokens_a.is_empty() || tokens_b.is_empty() {
        return 0.0;
    }

    let mut freq_a: HashMap<String, u32> = HashMap::new();
    for token in tokens_a {
        *freq_a.entry(token).or_insert(0) += 1;
    }
    let mut freq_b: HashMap<String, u32> = HashMap::new();
    for token in tokens_b {
        *freq_b.entry(token).or_insert(0) += 1;
    }

    let norm_a: f64 = freq_a.values().map(|&v| (v * v) as f64).sum::<f64>().sqrt();
    let norm_b: f64 = freq_b.values().map(|&v| (v * v) as f64).sum::<f64>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    let mut dot = 0.0;
    for (term, &count_a) in &freq_a {
        if let Some(&count_b) = freq_b.get(term) {
            dot += (count_a * count_b) as f64;
        }
    }
    dot / (norm_a * norm_b)
}

/// A conservative, deterministic inflectional stemmer.
///
/// It strips common plural and verb endings (`-ies`, `-sses`, `-es`, `-s`,
/// `-ing`, `-ed`) with minimum-length guards so that related surface forms
/// such as `contracts`/`contract`, `duties`/`duty`, `breaching`/`breach` and
/// `damages`/`damage` collapse to a shared stem without mangling short words.
pub(crate) fn stem(token: &str) -> String {
    let len = token.chars().count();
    if len <= 3 {
        return token.to_string();
    }

    // -ies / -ied -> -y (studies -> study, applied -> apply)
    if len > 4 && (token.ends_with("ies") || token.ends_with("ied")) {
        let base = &token[..token.len() - 3];
        return format!("{base}y");
    }

    // -sses -> -ss (classes -> class, possesses -> possess)
    if token.ends_with("sses") {
        return token[..token.len() - 2].to_string();
    }

    // -es after a sibilant -> strip (boxes -> box, watches -> watch)
    if len > 4 && token.ends_with("es") {
        let base = &token[..token.len() - 2];
        if base.ends_with('s')
            || base.ends_with('x')
            || base.ends_with('z')
            || base.ends_with("ch")
            || base.ends_with("sh")
        {
            return base.to_string();
        }
    }

    // plural -s (but never -ss)
    if len > 3 && token.ends_with('s') && !token.ends_with("ss") {
        return token[..token.len() - 1].to_string();
    }

    // -ing -> strip (breaching -> breach), require a substantive stem
    if len > 5 && token.ends_with("ing") {
        return token[..token.len() - 3].to_string();
    }

    // -ed -> strip (breached -> breach), require a substantive stem
    if len > 4 && token.ends_with("ed") {
        return token[..token.len() - 2].to_string();
    }

    token.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authority_type_primary() {
        assert!(AuthorityType::Case.is_primary());
        assert!(AuthorityType::Statute.is_primary());
        assert!(!AuthorityType::SecondarySource.is_primary());
        assert_eq!(
            AuthorityType::Constitution.label(),
            "constitutional provision"
        );
    }

    #[test]
    fn test_authority_builder_and_good_law() {
        let auth = LegalAuthority::new(
            "c1",
            "Brown v. Board of Education",
            "347 U.S. 483",
            "Separate educational facilities are inherently unequal.",
            AuthorityType::Case,
            Jurisdiction::UsFederal,
        )
        .with_court_level(CourtLevel::Supreme)
        .with_year(1954)
        .with_citation_count(25_000)
        .with_topic("equal protection");

        assert_eq!(auth.court_level, Some(CourtLevel::Supreme));
        assert_eq!(auth.year, Some(1954));
        assert_eq!(auth.citation_count, 25_000);
        assert_eq!(auth.topics, vec!["equal protection".to_string()]);
        assert!(auth.is_good_law());
        assert!(auth.indexable_text().contains("Brown"));

        let overruled = auth.with_treatment(TreatmentType::Overruled);
        assert!(!overruled.is_good_law());
    }

    #[test]
    fn test_court_weighting_and_rank() {
        assert!(
            court_authority_weight(CourtLevel::Supreme) > court_authority_weight(CourtLevel::Trial)
        );
        assert!(court_rank(CourtLevel::Supreme) > court_rank(CourtLevel::Circuit));
        assert!(court_rank(CourtLevel::Appellate) > court_rank(CourtLevel::Trial));
    }

    #[test]
    fn test_is_us_jurisdiction() {
        assert!(is_us_jurisdiction(&Jurisdiction::UsFederal));
        assert!(is_us_jurisdiction(&Jurisdiction::UsState(
            "California".into()
        )));
        assert!(!is_us_jurisdiction(&Jurisdiction::Uk));
    }

    #[test]
    fn test_tokenizer_drops_stopwords_and_stems() {
        let tokens = tokenize("The defendant breached the contracts and caused damages.");
        assert!(!tokens.iter().any(|t| t == "the"));
        assert!(tokens.contains(&"breach".to_string()));
        assert!(tokens.contains(&"contract".to_string()));
        assert!(tokens.contains(&"damage".to_string()));
        // single characters removed
        assert!(tokens.iter().all(|t| t.chars().count() >= 2));
    }

    #[test]
    fn test_stemmer_rules() {
        assert_eq!(stem("duties"), "duty");
        assert_eq!(stem("classes"), "class");
        assert_eq!(stem("boxes"), "box");
        assert_eq!(stem("contracts"), "contract");
        assert_eq!(stem("breaching"), "breach");
        assert_eq!(stem("breached"), "breach");
        // short words untouched
        assert_eq!(stem("law"), "law");
        assert_eq!(stem("is"), "is");
        // never strips -ss
        assert_eq!(stem("class"), "class");
    }

    #[test]
    fn test_modal_verbs_preserved() {
        let tokens = tokenize("The party shall and may perform but must not breach");
        assert!(tokens.contains(&"shall".to_string()));
        assert!(tokens.contains(&"may".to_string()));
        assert!(tokens.contains(&"must".to_string()));
    }
}
