//! Case Law Search Engine (判例検索エンジン)
//!
//! Provides search and filtering capabilities for court decisions.

use super::error::{CaseLawError, Result};
use super::types::{CaseSearchQuery, CaseSearchResult, CourtDecision};
use chrono::Datelike;
use std::collections::HashMap;

/// Trait for case law database backends
pub trait CaseLawDatabase {
    /// Searches for cases matching the query
    fn search(&self, query: &CaseSearchQuery) -> Result<Vec<CaseSearchResult>>;

    /// Retrieves a case by ID
    fn get_by_id(&self, id: &str) -> Result<CourtDecision>;

    /// Adds a new case to the database
    fn add_case(&mut self, decision: CourtDecision) -> Result<()>;

    /// Updates an existing case
    fn update_case(&mut self, decision: CourtDecision) -> Result<()>;

    /// Deletes a case by ID
    fn delete_case(&mut self, id: &str) -> Result<()>;

    /// Returns the total number of cases
    fn count(&self) -> usize;
}

/// In-memory case law database (for testing and small datasets)
#[derive(Debug, Default)]
pub struct InMemoryCaseDatabase {
    cases: HashMap<String, CourtDecision>,
}

impl InMemoryCaseDatabase {
    /// Creates a new empty in-memory database
    pub fn new() -> Self {
        Self {
            cases: HashMap::new(),
        }
    }

    /// Loads sample cases for testing
    #[cfg(test)]
    pub fn with_sample_cases() -> Self {
        // Sample cases would be added here for testing
        Self::new()
    }
}

impl CaseLawDatabase for InMemoryCaseDatabase {
    fn search(&self, query: &CaseSearchQuery) -> Result<Vec<CaseSearchResult>> {
        let mut results = Vec::new();

        for decision in self.cases.values() {
            if !Self::matches_query(decision, query) {
                continue;
            }

            let relevance_score = Self::calculate_relevance(decision, query);
            let mut result = CaseSearchResult::new(decision.clone(), relevance_score);

            // Track matching keywords
            for keyword in &query.keywords {
                if decision.contains_keyword(keyword) {
                    result.add_matching_keyword(keyword);
                }
            }

            results.push(result);
        }

        // Sort by relevance score (highest first)
        results.sort_by(|a, b| {
            b.relevance_score
                .partial_cmp(&a.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        if results.is_empty() {
            return Err(CaseLawError::NoResultsFound);
        }

        Ok(results)
    }

    fn get_by_id(&self, id: &str) -> Result<CourtDecision> {
        self.cases
            .get(id)
            .cloned()
            .ok_or_else(|| CaseLawError::CaseNotFound {
                case_id: id.to_string(),
            })
    }

    fn add_case(&mut self, decision: CourtDecision) -> Result<()> {
        let id = decision.id.clone();
        self.cases.insert(id, decision);
        Ok(())
    }

    fn update_case(&mut self, decision: CourtDecision) -> Result<()> {
        let id = decision.id.clone();
        if !self.cases.contains_key(&id) {
            return Err(CaseLawError::CaseNotFound { case_id: id });
        }
        self.cases.insert(id, decision);
        Ok(())
    }

    fn delete_case(&mut self, id: &str) -> Result<()> {
        self.cases
            .remove(id)
            .ok_or_else(|| CaseLawError::CaseNotFound {
                case_id: id.to_string(),
            })?;
        Ok(())
    }

    fn count(&self) -> usize {
        self.cases.len()
    }
}

impl InMemoryCaseDatabase {
    /// Checks if a decision matches the query filters
    fn matches_query(decision: &CourtDecision, query: &CaseSearchQuery) -> bool {
        // Court level filter
        if let Some(level) = query.court_level
            && decision.metadata.court.level != level
        {
            return false;
        }

        // Legal area filter
        if let Some(area) = query.legal_area
            && decision.metadata.legal_area != area
        {
            return false;
        }

        // Date range filter
        if let Some(date_from) = query.date_from
            && decision.metadata.decision_date < date_from
        {
            return false;
        }

        if let Some(date_to) = query.date_to
            && decision.metadata.decision_date > date_to
        {
            return false;
        }

        // Cited statute filter
        if let Some(statute) = &query.cited_statute
            && !decision.cites_statute(statute)
        {
            return false;
        }

        // Case outcome filter
        if let Some(outcome) = query.outcome
            && decision.metadata.outcome != outcome
        {
            return false;
        }

        // Keyword filter (must match at least one keyword if keywords are specified)
        if !query.keywords.is_empty() {
            let has_keyword_match = query
                .keywords
                .iter()
                .any(|kw| decision.contains_keyword(kw));

            if !has_keyword_match {
                return false;
            }
        }

        true
    }

    /// Calculates relevance score for a decision (0.0 to 1.0)
    fn calculate_relevance(decision: &CourtDecision, query: &CaseSearchQuery) -> f64 {
        let mut score = 0.0;
        let mut max_score = 0.0;

        // Keyword matching (weight: 0.4)
        if !query.keywords.is_empty() {
            max_score += 0.4;
            let mut keyword_score = 0.0;

            for keyword in &query.keywords {
                if decision.contains_keyword(keyword) {
                    keyword_score += 1.0;
                }
            }

            score += (keyword_score / query.keywords.len() as f64) * 0.4;
        }

        // Court level precedent weight (weight: 0.2)
        max_score += 0.2;
        let precedent_score = match decision.metadata.court.level.precedent_weight() {
            0 => 1.0,  // Supreme Court
            1 => 0.75, // High Court
            2 => 0.5,  // District/Family Court
            _ => 0.25, // Summary Court
        };
        score += precedent_score * 0.2;

        // Leading case bonus (weight: 0.2)
        max_score += 0.2;
        let has_leading_case = decision.holdings.iter().any(|h| h.is_leading_case);
        if has_leading_case {
            score += 0.2;
        }

        // Recency (weight: 0.2) - more recent cases score higher
        max_score += 0.2;
        let years_old = chrono::Utc::now().year() - decision.decision_year();
        let recency_score = if years_old <= 5 {
            1.0
        } else if years_old <= 10 {
            0.75
        } else if years_old <= 20 {
            0.5
        } else {
            0.25
        };
        score += recency_score * 0.2;

        // Normalize to 0.0-1.0 range
        if max_score > 0.0 {
            score / max_score
        } else {
            0.5 // Default relevance if no criteria
        }
    }
}

/// Search engine with advanced features
pub struct CaseLawSearchEngine<D: CaseLawDatabase> {
    database: D,
}

impl<D: CaseLawDatabase> CaseLawSearchEngine<D> {
    /// Creates a new search engine with the given database
    pub fn new(database: D) -> Self {
        Self { database }
    }

    /// Performs a search
    pub fn search(&self, query: &CaseSearchQuery) -> Result<Vec<CaseSearchResult>> {
        self.validate_query(query)?;
        self.database.search(query)
    }

    /// Gets a case by ID
    pub fn get(&self, id: &str) -> Result<CourtDecision> {
        self.database.get_by_id(id)
    }

    /// Finds similar cases based on keywords and legal area
    pub fn find_similar(
        &self,
        decision: &CourtDecision,
        limit: usize,
    ) -> Result<Vec<CaseSearchResult>> {
        let mut query = CaseSearchQuery::new()
            .with_legal_area(decision.metadata.legal_area)
            .with_limit(limit + 1); // +1 because we'll filter out the original

        // Add keywords from the original decision
        for keyword in &decision.metadata.keywords {
            query = query.with_keyword(keyword);
        }

        let mut results = self.search(&query)?;

        // Remove the original decision from results
        results.retain(|r| r.decision.id != decision.id);

        // Ensure we don't exceed the limit after filtering
        results.truncate(limit);

        Ok(results)
    }

    /// Validates the search query
    fn validate_query(&self, query: &CaseSearchQuery) -> Result<()> {
        // Check date range validity
        if let (Some(from), Some(to)) = (query.date_from, query.date_to)
            && from > to
        {
            return Err(CaseLawError::InvalidDateRange);
        }

        // Check if keywords are not empty strings
        for keyword in &query.keywords {
            if keyword.trim().is_empty() {
                return Err(CaseLawError::InvalidSearchQuery {
                    reason: "Keywords cannot be empty".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Returns database statistics
    pub fn stats(&self) -> DatabaseStats {
        DatabaseStats {
            total_cases: self.database.count(),
        }
    }
}

/// Database statistics
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub total_cases: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::case_law::types::{CaseMetadata, CaseOutcome, Court, CourtLevel, LegalArea};
    use chrono::Utc;

    #[test]
    fn test_in_memory_database_crud() {
        let mut db = InMemoryCaseDatabase::new();

        let metadata = CaseMetadata::new(
            "令和2年(受)第1234号",
            Utc::now(),
            Court::new(CourtLevel::Supreme),
            LegalArea::Civil,
            CaseOutcome::PlaintiffWins,
        );

        let decision = CourtDecision::new("case-001", metadata, "Test case");

        // Add
        assert!(db.add_case(decision.clone()).is_ok());
        assert_eq!(db.count(), 1);

        // Get
        let retrieved = db.get_by_id("case-001").unwrap();
        assert_eq!(retrieved.id, "case-001");

        // Update
        let mut updated = decision.clone();
        updated.summary = "Updated summary".to_string();
        assert!(db.update_case(updated).is_ok());

        // Delete
        assert!(db.delete_case("case-001").is_ok());
        assert_eq!(db.count(), 0);
    }

    #[test]
    fn test_search_with_filters() {
        let mut db = InMemoryCaseDatabase::new();

        // Add test cases
        let mut metadata1 = CaseMetadata::new(
            "令和2年(受)第1234号",
            Utc::now(),
            Court::new(CourtLevel::Supreme),
            LegalArea::Civil,
            CaseOutcome::PlaintiffWins,
        );
        metadata1.add_keyword("tort");

        let decision1 = CourtDecision::new("case-001", metadata1, "Tort case summary");
        db.add_case(decision1).unwrap();

        // Search by keyword
        let query = CaseSearchQuery::new().with_keyword("tort");
        let results = db.search(&query).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].decision.id, "case-001");
    }

    #[test]
    fn test_search_no_results() {
        let db = InMemoryCaseDatabase::new();

        let query = CaseSearchQuery::new().with_keyword("nonexistent");
        let result = db.search(&query);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CaseLawError::NoResultsFound));
    }

    #[test]
    fn test_relevance_scoring() {
        let mut metadata = CaseMetadata::new(
            "令和2年(受)第1234号",
            Utc::now(),
            Court::new(CourtLevel::Supreme),
            LegalArea::Civil,
            CaseOutcome::PlaintiffWins,
        );
        metadata.add_keyword("test");

        let decision = CourtDecision::new("case-001", metadata, "Test summary");

        let query = CaseSearchQuery::new().with_keyword("test");

        let score = InMemoryCaseDatabase::calculate_relevance(&decision, &query);

        assert!(score > 0.0);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_search_engine_validation() {
        let db = InMemoryCaseDatabase::new();
        let engine = CaseLawSearchEngine::new(db);

        // Invalid date range
        let query = CaseSearchQuery::new()
            .with_date_range(Utc::now(), Utc::now() - chrono::Duration::days(1));

        let result = engine.search(&query);
        assert!(result.is_err());
    }
}
