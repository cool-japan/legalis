//! Precedent library management.
//!
//! [`PrecedentLibrary`] stores [`PrecedentRecord`]s and indexes them three ways
//! for fast retrieval:
//!
//! * a **full-text** inverted index (BM25) over the holding / summary, via the
//!   reusable [`crate::knowledge::SearchIndex`];
//! * a **citation** index that normalises each precedent's citation and the
//!   citations it relies on, enabling exact citation lookup and reverse
//!   "what cites this" queries;
//! * **topic** and **jurisdiction** secondary indexes for faceted browsing.
//!
//! Each record carries structured [`PrecedentCitation`]s (the authorities it
//! relies on), court/jurisdiction metadata and a treatment status. The library
//! is fully offline and deterministic.

use super::{IndexedDocument, SearchIndex, SearchQuery};
use crate::{CourtLevel, Jurisdiction, TreatmentType};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// A structured citation a precedent relies on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrecedentCitation {
    /// The raw citation string as written (e.g. `347 U.S. 483`).
    pub raw: String,
    /// Optional pinpoint / page reference within the cited authority.
    pub pinpoint: Option<String>,
    /// Optional id of the cited precedent within this library (resolved link).
    pub target_id: Option<String>,
}

impl PrecedentCitation {
    /// Creates a citation from a raw string.
    pub fn new(raw: impl Into<String>) -> Self {
        Self {
            raw: raw.into(),
            pinpoint: None,
            target_id: None,
        }
    }

    /// Sets a pinpoint reference.
    pub fn with_pinpoint(mut self, pinpoint: impl Into<String>) -> Self {
        self.pinpoint = Some(pinpoint.into());
        self
    }

    /// Links this citation to a precedent id within the library.
    pub fn with_target(mut self, target_id: impl Into<String>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }

    /// Returns the normalised form of the raw citation (see
    /// [`normalize_citation`]).
    pub fn normalized(&self) -> String {
        normalize_citation(&self.raw)
    }
}

/// A precedent stored in the library.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrecedentRecord {
    /// Stable unique identifier.
    pub id: String,
    /// Case name / title.
    pub title: String,
    /// Canonical citation for this precedent.
    pub citation: String,
    /// The holding / summary (indexed for full-text search).
    pub holding: String,
    /// Jurisdiction.
    pub jurisdiction: Jurisdiction,
    /// Court level, if applicable.
    pub court_level: Option<CourtLevel>,
    /// Year decided.
    pub year: Option<i32>,
    /// Treatment / current status.
    pub treatment: Option<TreatmentType>,
    /// Topical tags.
    pub topics: Vec<String>,
    /// Authorities this precedent relies on.
    pub citations: Vec<PrecedentCitation>,
}

impl PrecedentRecord {
    /// Creates a precedent with the mandatory fields.
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        citation: impl Into<String>,
        holding: impl Into<String>,
        jurisdiction: Jurisdiction,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            citation: citation.into(),
            holding: holding.into(),
            jurisdiction,
            court_level: None,
            year: None,
            treatment: None,
            topics: Vec::new(),
            citations: Vec::new(),
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

    /// Adds a relied-on citation.
    pub fn with_citation(mut self, citation: PrecedentCitation) -> Self {
        self.citations.push(citation);
        self
    }

    /// Whether this precedent is still good law (not overruled).
    pub fn is_good_law(&self) -> bool {
        !matches!(self.treatment, Some(TreatmentType::Overruled))
    }

    /// The normalised canonical citation.
    pub fn normalized_citation(&self) -> String {
        normalize_citation(&self.citation)
    }
}

/// A precedent retrieval hit (record plus relevance score).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrecedentHit {
    /// The matched precedent id.
    pub id: String,
    /// Relevance score from the underlying full-text index.
    pub score: f64,
}

/// An indexed, searchable library of precedents.
#[derive(Debug, Clone, Default)]
pub struct PrecedentLibrary {
    records: BTreeMap<String, PrecedentRecord>,
    index: SearchIndex,
    /// Normalised canonical citation -> precedent id.
    by_citation: BTreeMap<String, String>,
    /// Normalised cited authority -> ids of precedents that cite it.
    citing: BTreeMap<String, BTreeSet<String>>,
    /// Topic (lowercased) -> precedent ids.
    by_topic: BTreeMap<String, BTreeSet<String>>,
    /// Jurisdiction description -> precedent ids.
    by_jurisdiction: BTreeMap<String, BTreeSet<String>>,
}

impl PrecedentLibrary {
    /// Creates an empty library.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a precedent, updating all indexes. Errors on duplicate id.
    pub fn add(&mut self, record: PrecedentRecord) -> Result<(), String> {
        if self.records.contains_key(&record.id) {
            return Err(format!("duplicate precedent id: {}", record.id));
        }

        // Full-text index over title + holding.
        let doc = IndexedDocument::new(
            record.id.clone(),
            record.title.clone(),
            record.holding.clone(),
        )
        .with_metadata("jurisdiction", record.jurisdiction.description());
        self.index.add(doc)?;

        // Citation index.
        self.by_citation
            .insert(record.normalized_citation(), record.id.clone());
        for citation in &record.citations {
            self.citing
                .entry(citation.normalized())
                .or_default()
                .insert(record.id.clone());
        }
        // Topic / jurisdiction facets.
        for topic in &record.topics {
            self.by_topic
                .entry(topic.to_lowercase())
                .or_default()
                .insert(record.id.clone());
        }
        self.by_jurisdiction
            .entry(record.jurisdiction.description())
            .or_default()
            .insert(record.id.clone());

        self.records.insert(record.id.clone(), record);
        Ok(())
    }

    /// Adds many precedents, stopping at the first error.
    pub fn add_many<I>(&mut self, records: I) -> Result<(), String>
    where
        I: IntoIterator<Item = PrecedentRecord>,
    {
        for record in records {
            self.add(record)?;
        }
        Ok(())
    }

    /// Number of stored precedents.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Whether the library is empty.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Looks up a precedent by id.
    pub fn get(&self, id: &str) -> Option<&PrecedentRecord> {
        self.records.get(id)
    }

    /// Looks up a precedent by (raw or normalised) citation.
    pub fn find_by_citation(&self, citation: &str) -> Option<&PrecedentRecord> {
        let key = normalize_citation(citation);
        self.by_citation
            .get(&key)
            .and_then(|id| self.records.get(id))
    }

    /// Full-text search over holdings, returning up to `top_k` precedents.
    pub fn search(&self, query: &str, top_k: usize) -> Vec<PrecedentHit> {
        self.index
            .search(query, top_k)
            .into_iter()
            .map(|hit| PrecedentHit {
                id: hit.id,
                score: hit.score,
            })
            .collect()
    }

    /// Full-text search restricted to a jurisdiction.
    pub fn search_in_jurisdiction(
        &self,
        query: &str,
        jurisdiction: &Jurisdiction,
        top_k: usize,
    ) -> Vec<PrecedentHit> {
        let q =
            SearchQuery::new(query, top_k).with_filter("jurisdiction", jurisdiction.description());
        self.index
            .search_with(&q)
            .into_iter()
            .map(|hit| PrecedentHit {
                id: hit.id,
                score: hit.score,
            })
            .collect()
    }

    /// Returns all precedents tagged with a topic (case-insensitive).
    pub fn by_topic(&self, topic: &str) -> Vec<&PrecedentRecord> {
        self.by_topic
            .get(&topic.to_lowercase())
            .map(|ids| ids.iter().filter_map(|id| self.records.get(id)).collect())
            .unwrap_or_default()
    }

    /// Returns all precedents in a jurisdiction.
    pub fn by_jurisdiction(&self, jurisdiction: &Jurisdiction) -> Vec<&PrecedentRecord> {
        self.by_jurisdiction
            .get(&jurisdiction.description())
            .map(|ids| ids.iter().filter_map(|id| self.records.get(id)).collect())
            .unwrap_or_default()
    }

    /// Returns the precedents that cite the given authority (by raw or
    /// normalised citation) - a reverse-citation ("cited by") query.
    pub fn cited_by(&self, citation: &str) -> Vec<&PrecedentRecord> {
        let key = normalize_citation(citation);
        self.citing
            .get(&key)
            .map(|ids| ids.iter().filter_map(|id| self.records.get(id)).collect())
            .unwrap_or_default()
    }

    /// Returns the precedents that the given precedent cites and that are
    /// resolvable within the library (forward-citation traversal).
    pub fn cites(&self, id: &str) -> Vec<&PrecedentRecord> {
        let Some(record) = self.records.get(id) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        let mut seen = BTreeSet::new();
        for citation in &record.citations {
            // Prefer an explicit target link, else resolve by citation string.
            let resolved = citation
                .target_id
                .as_ref()
                .and_then(|t| self.records.get(t))
                .or_else(|| {
                    self.by_citation
                        .get(&citation.normalized())
                        .and_then(|cid| self.records.get(cid))
                });
            if let Some(found) = resolved
                && seen.insert(found.id.clone())
            {
                out.push(found);
            }
        }
        out
    }

    /// Finds precedents whose holding is similar to the given precedent.
    pub fn find_similar(&self, id: &str, top_k: usize) -> Vec<PrecedentHit> {
        self.index
            .find_similar(id, top_k)
            .into_iter()
            .map(|hit| PrecedentHit {
                id: hit.id,
                score: hit.score,
            })
            .collect()
    }

    /// Iterates over all stored precedents (ordered by id).
    pub fn records(&self) -> impl Iterator<Item = &PrecedentRecord> {
        self.records.values()
    }
}

/// Normalises a citation string for indexing and exact lookup.
///
/// Lowercases, collapses internal whitespace, and strips characters that vary
/// between citation styles (`.`, `,`, `§`, `(`, `)`), so that `347 U.S. 483`,
/// `347 U. S. 483` and `347 us 483` all collapse to the same key.
pub fn normalize_citation(citation: &str) -> String {
    let cleaned: String = citation
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect();
    cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_library() -> PrecedentLibrary {
        let mut library = PrecedentLibrary::new();
        library
            .add_many(vec![
                PrecedentRecord::new(
                    "brown",
                    "Brown v. Board of Education",
                    "347 U.S. 483",
                    "Separate educational facilities are inherently unequal and violate \
                     the Equal Protection Clause.",
                    Jurisdiction::UsFederal,
                )
                .with_court_level(CourtLevel::Supreme)
                .with_year(1954)
                .with_topic("Equal Protection")
                .with_topic("Education")
                .with_citation(PrecedentCitation::new("163 U.S. 537").with_target("plessy")),
                PrecedentRecord::new(
                    "plessy",
                    "Plessy v. Ferguson",
                    "163 U.S. 537",
                    "The separate but equal doctrine permits racial segregation in public \
                     accommodations.",
                    Jurisdiction::UsFederal,
                )
                .with_court_level(CourtLevel::Supreme)
                .with_year(1896)
                .with_treatment(TreatmentType::Overruled)
                .with_topic("Equal Protection"),
                PrecedentRecord::new(
                    "palsgraf",
                    "Palsgraf v. Long Island Railroad",
                    "248 N.Y. 339",
                    "Negligence liability requires a duty of care owed to a foreseeable \
                     plaintiff within the zone of danger.",
                    Jurisdiction::UsState("New York".into()),
                )
                .with_court_level(CourtLevel::Appellate)
                .with_year(1928)
                .with_topic("Negligence"),
            ])
            .expect("library builds");
        library
    }

    #[test]
    fn test_add_and_duplicate() {
        let mut library = PrecedentLibrary::new();
        let rec = PrecedentRecord::new("x", "T", "1 U.S. 1", "holding", Jurisdiction::UsFederal);
        library.add(rec.clone()).expect("first add");
        assert_eq!(library.len(), 1);
        assert!(library.add(rec).is_err());
    }

    #[test]
    fn test_full_text_search() {
        let library = sample_library();
        let hits = library.search("duty of care foreseeable plaintiff negligence", 5);
        assert!(!hits.is_empty());
        assert_eq!(hits[0].id, "palsgraf");
    }

    #[test]
    fn test_search_in_jurisdiction() {
        let library = sample_library();
        let hits = library.search_in_jurisdiction(
            "duty plaintiff",
            &Jurisdiction::UsState("New York".into()),
            5,
        );
        assert!(hits.iter().all(|h| h.id == "palsgraf"));

        let federal = library.search_in_jurisdiction(
            "equal protection segregation",
            &Jurisdiction::UsFederal,
            5,
        );
        assert!(federal.iter().all(|h| h.id == "brown" || h.id == "plessy"));
    }

    #[test]
    fn test_citation_lookup_normalization() {
        let library = sample_library();
        // Punctuation/spacing variants of the reporter map to the same record:
        // both "U.S." and "U. S." normalise to the tokens "u s".
        assert_eq!(
            library
                .find_by_citation("347 U.S. 483")
                .map(|r| r.id.as_str()),
            Some("brown")
        );
        assert_eq!(
            library
                .find_by_citation("347 U. S. 483")
                .map(|r| r.id.as_str()),
            Some("brown")
        );
        assert_eq!(
            library
                .find_by_citation("  347  u.s.  483 ")
                .map(|r| r.id.as_str()),
            Some("brown")
        );
        assert!(library.find_by_citation("999 F.3d 1").is_none());
    }

    #[test]
    fn test_topic_and_jurisdiction_facets() {
        let library = sample_library();
        let equal_protection = library.by_topic("equal protection");
        let ids: BTreeSet<&str> = equal_protection.iter().map(|r| r.id.as_str()).collect();
        assert!(ids.contains("brown"));
        assert!(ids.contains("plessy"));

        let ny = library.by_jurisdiction(&Jurisdiction::UsState("New York".into()));
        assert_eq!(ny.len(), 1);
        assert_eq!(ny[0].id, "palsgraf");
    }

    #[test]
    fn test_forward_and_reverse_citations() {
        let library = sample_library();
        // Brown cites Plessy (linked target).
        let cites = library.cites("brown");
        assert_eq!(cites.len(), 1);
        assert_eq!(cites[0].id, "plessy");

        // Reverse: Plessy is cited by Brown.
        let cited_by = library.cited_by("163 U.S. 537");
        assert_eq!(cited_by.len(), 1);
        assert_eq!(cited_by[0].id, "brown");
    }

    #[test]
    fn test_good_law_flag() {
        let library = sample_library();
        assert!(
            library
                .get("brown")
                .map(|r| r.is_good_law())
                .unwrap_or(false)
        );
        assert!(
            !library
                .get("plessy")
                .map(|r| r.is_good_law())
                .unwrap_or(true)
        );
    }

    #[test]
    fn test_find_similar() {
        let library = sample_library();
        let hits = library.find_similar("brown", 5);
        assert!(hits.iter().all(|h| h.id != "brown"));
        // Plessy (same equal-protection topic vocabulary) should appear.
        assert!(hits.iter().any(|h| h.id == "plessy"));
    }

    #[test]
    fn test_normalize_citation() {
        assert_eq!(normalize_citation("347 U.S. 483"), "347 u s 483");
        assert_eq!(normalize_citation("42 U.S.C. § 1983"), "42 u s c 1983");
        assert_eq!(normalize_citation("  163   U.S.  537  "), "163 u s 537");
    }
}
