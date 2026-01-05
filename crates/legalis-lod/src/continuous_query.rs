//! Continuous query evaluation for real-time legal intelligence.
//!
//! This module provides continuous evaluation of queries over changing RDF data,
//! maintaining up-to-date results as the knowledge graph evolves.

#[cfg(test)]
use crate::RdfValue;
use crate::{LodResult, Triple};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

/// Continuous query that maintains results over changing data.
pub struct ContinuousQuery {
    /// Query ID
    pub id: String,
    /// Query pattern
    pub pattern: QueryPattern,
    /// Current result set
    results: Arc<Mutex<Vec<QueryResult>>>,
    /// Callback for result updates
    callbacks: Arc<Mutex<Vec<Box<dyn Fn(&[QueryResult]) + Send + Sync>>>>,
}

impl ContinuousQuery {
    /// Creates a new continuous query.
    pub fn new(id: impl Into<String>, pattern: QueryPattern) -> Self {
        Self {
            id: id.into(),
            pattern,
            results: Arc::new(Mutex::new(Vec::new())),
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Registers a callback for result updates.
    pub fn on_update<F>(&mut self, callback: F)
    where
        F: Fn(&[QueryResult]) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(Box::new(callback));
    }

    /// Processes a triple addition.
    pub fn add_triple(&mut self, triple: &Triple) -> LodResult<()> {
        if self.pattern.matches(triple) {
            let result = self.pattern.extract_bindings(triple);

            let mut results = self.results.lock().unwrap();
            if !results.iter().any(|r| r.bindings == result.bindings) {
                results.push(result);
                drop(results);
                self.notify_callbacks()?;
            }
        }
        Ok(())
    }

    /// Processes a triple removal.
    pub fn remove_triple(&mut self, triple: &Triple) -> LodResult<()> {
        let result = self.pattern.extract_bindings(triple);

        let mut results = self.results.lock().unwrap();
        if let Some(pos) = results.iter().position(|r| r.bindings == result.bindings) {
            results.remove(pos);
            drop(results);
            self.notify_callbacks()?;
        }
        Ok(())
    }

    /// Returns the current results.
    pub fn get_results(&self) -> Vec<QueryResult> {
        self.results.lock().unwrap().clone()
    }

    /// Notifies all registered callbacks.
    fn notify_callbacks(&self) -> LodResult<()> {
        let results = self.results.lock().unwrap().clone();
        let callbacks = self.callbacks.lock().unwrap();

        for callback in callbacks.iter() {
            callback(&results);
        }
        Ok(())
    }

    /// Returns the number of current results.
    pub fn result_count(&self) -> usize {
        self.results.lock().unwrap().len()
    }
}

/// Query pattern for matching triples.
#[derive(Debug, Clone)]
pub struct QueryPattern {
    /// Subject pattern (None for variable)
    pub subject: Option<String>,
    /// Predicate pattern (None for variable)
    pub predicate: Option<String>,
    /// Object pattern (None for variable)
    pub object: Option<String>,
}

impl QueryPattern {
    /// Creates a new query pattern.
    pub fn new(subject: Option<String>, predicate: Option<String>, object: Option<String>) -> Self {
        Self {
            subject,
            predicate,
            object,
        }
    }

    /// Creates a pattern that matches any triple.
    pub fn any() -> Self {
        Self {
            subject: None,
            predicate: None,
            object: None,
        }
    }

    /// Checks if a triple matches this pattern.
    pub fn matches(&self, triple: &Triple) -> bool {
        if let Some(ref s) = self.subject {
            if &triple.subject != s {
                return false;
            }
        }

        if let Some(ref p) = self.predicate {
            if &triple.predicate != p {
                return false;
            }
        }

        if let Some(ref o) = self.object {
            let obj_str = format!("{:?}", triple.object);
            if !obj_str.contains(o) {
                return false;
            }
        }

        true
    }

    /// Extracts variable bindings from a matching triple.
    pub fn extract_bindings(&self, triple: &Triple) -> QueryResult {
        let mut bindings = HashMap::new();

        if self.subject.is_none() {
            bindings.insert("subject".to_string(), triple.subject.clone());
        }

        if self.predicate.is_none() {
            bindings.insert("predicate".to_string(), triple.predicate.clone());
        }

        if self.object.is_none() {
            bindings.insert("object".to_string(), format!("{:?}", triple.object));
        }

        QueryResult { bindings }
    }
}

/// Query result with variable bindings.
#[derive(Debug, Clone, PartialEq)]
pub struct QueryResult {
    /// Variable bindings
    pub bindings: HashMap<String, String>,
}

/// Continuous query manager for multiple queries.
pub struct ContinuousQueryManager {
    /// Active queries
    queries: HashMap<String, ContinuousQuery>,
    /// Triple index for efficient lookups
    triple_index: TripleIndex,
}

impl ContinuousQueryManager {
    /// Creates a new query manager.
    pub fn new() -> Self {
        Self {
            queries: HashMap::new(),
            triple_index: TripleIndex::new(),
        }
    }

    /// Registers a continuous query.
    pub fn register_query(&mut self, query: ContinuousQuery) {
        self.queries.insert(query.id.clone(), query);
    }

    /// Unregisters a query.
    pub fn unregister_query(&mut self, id: &str) -> bool {
        self.queries.remove(id).is_some()
    }

    /// Processes a triple addition.
    pub fn add_triple(&mut self, triple: Triple) -> LodResult<()> {
        self.triple_index.add(triple.clone());

        for query in self.queries.values_mut() {
            query.add_triple(&triple)?;
        }
        Ok(())
    }

    /// Processes a triple removal.
    pub fn remove_triple(&mut self, triple: &Triple) -> LodResult<()> {
        self.triple_index.remove(triple);

        for query in self.queries.values_mut() {
            query.remove_triple(triple)?;
        }
        Ok(())
    }

    /// Returns a query by ID.
    pub fn get_query(&self, id: &str) -> Option<&ContinuousQuery> {
        self.queries.get(id)
    }

    /// Returns all registered query IDs.
    pub fn list_queries(&self) -> Vec<String> {
        self.queries.keys().cloned().collect()
    }

    /// Returns the number of registered queries.
    pub fn query_count(&self) -> usize {
        self.queries.len()
    }

    /// Returns the number of indexed triples.
    pub fn triple_count(&self) -> usize {
        self.triple_index.count()
    }
}

impl Default for ContinuousQueryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Index for efficient triple lookups.
#[allow(dead_code)]
struct TripleIndex {
    /// Triples indexed by subject
    by_subject: HashMap<String, Vec<Triple>>,
    /// Triples indexed by predicate
    by_predicate: HashMap<String, Vec<Triple>>,
    /// All triples
    all_triples: HashSet<TripleKey>,
}

impl TripleIndex {
    fn new() -> Self {
        Self {
            by_subject: HashMap::new(),
            by_predicate: HashMap::new(),
            all_triples: HashSet::new(),
        }
    }

    fn add(&mut self, triple: Triple) {
        let key = TripleKey::from(&triple);
        if self.all_triples.insert(key) {
            self.by_subject
                .entry(triple.subject.clone())
                .or_default()
                .push(triple.clone());

            self.by_predicate
                .entry(triple.predicate.clone())
                .or_default()
                .push(triple);
        }
    }

    fn remove(&mut self, triple: &Triple) {
        let key = TripleKey::from(triple);
        if self.all_triples.remove(&key) {
            if let Some(triples) = self.by_subject.get_mut(&triple.subject) {
                triples.retain(|t| TripleKey::from(t) != key);
            }

            if let Some(triples) = self.by_predicate.get_mut(&triple.predicate) {
                triples.retain(|t| TripleKey::from(t) != key);
            }
        }
    }

    fn count(&self) -> usize {
        self.all_triples.len()
    }
}

/// Triple key for hashing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TripleKey {
    subject: String,
    predicate: String,
    object: String,
}

impl From<&Triple> for TripleKey {
    fn from(triple: &Triple) -> Self {
        Self {
            subject: triple.subject.clone(),
            predicate: triple.predicate.clone(),
            object: format!("{:?}", triple.object),
        }
    }
}

/// Query optimizer for continuous queries.
pub struct QueryOptimizer;

impl QueryOptimizer {
    /// Analyzes a query pattern for optimization opportunities.
    pub fn analyze(pattern: &QueryPattern) -> QueryStats {
        let mut selectivity = 1.0;

        if pattern.subject.is_some() {
            selectivity *= 0.1; // Subject specified = high selectivity
        }

        if pattern.predicate.is_some() {
            selectivity *= 0.2; // Predicate specified = medium selectivity
        }

        if pattern.object.is_some() {
            selectivity *= 0.3; // Object specified = lower selectivity
        }

        QueryStats { selectivity }
    }

    /// Suggests an index strategy for a pattern.
    pub fn suggest_index(pattern: &QueryPattern) -> IndexStrategy {
        if pattern.subject.is_some() {
            IndexStrategy::Subject
        } else if pattern.predicate.is_some() {
            IndexStrategy::Predicate
        } else {
            IndexStrategy::Full
        }
    }
}

/// Query statistics.
#[derive(Debug, Clone)]
pub struct QueryStats {
    /// Estimated selectivity (0.0 to 1.0)
    pub selectivity: f64,
}

/// Index strategy recommendation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexStrategy {
    Subject,
    Predicate,
    Full,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_triple() -> Triple {
        Triple {
            subject: "ex:s1".to_string(),
            predicate: "ex:p1".to_string(),
            object: RdfValue::string("o1"),
        }
    }

    #[test]
    fn test_query_pattern_matches() {
        let pattern = QueryPattern::new(Some("ex:s1".to_string()), None, None);

        let triple = sample_triple();
        assert!(pattern.matches(&triple));

        let mut triple2 = sample_triple();
        triple2.subject = "ex:s2".to_string();
        assert!(!pattern.matches(&triple2));
    }

    #[test]
    fn test_query_pattern_any() {
        let pattern = QueryPattern::any();
        let triple = sample_triple();
        assert!(pattern.matches(&triple));
    }

    #[test]
    fn test_extract_bindings() {
        let pattern = QueryPattern::new(None, Some("ex:p1".to_string()), None);
        let triple = sample_triple();
        let result = pattern.extract_bindings(&triple);

        assert!(result.bindings.contains_key("subject"));
        assert!(result.bindings.contains_key("object"));
        assert!(!result.bindings.contains_key("predicate"));
    }

    #[test]
    fn test_continuous_query() {
        let pattern = QueryPattern::any();
        let mut query = ContinuousQuery::new("test-query", pattern);

        let triple = sample_triple();
        query.add_triple(&triple).unwrap();

        assert_eq!(query.result_count(), 1);
    }

    #[test]
    fn test_query_remove_triple() {
        let pattern = QueryPattern::any();
        let mut query = ContinuousQuery::new("test-query", pattern);

        let triple = sample_triple();
        query.add_triple(&triple).unwrap();
        assert_eq!(query.result_count(), 1);

        query.remove_triple(&triple).unwrap();
        assert_eq!(query.result_count(), 0);
    }

    #[test]
    fn test_query_callback() {
        let pattern = QueryPattern::any();
        let mut query = ContinuousQuery::new("test-query", pattern);

        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();

        query.on_update(move |_results| {
            *called_clone.lock().unwrap() = true;
        });

        let triple = sample_triple();
        query.add_triple(&triple).unwrap();

        assert!(*called.lock().unwrap());
    }

    #[test]
    fn test_query_manager() {
        let mut manager = ContinuousQueryManager::new();

        let pattern = QueryPattern::any();
        let query = ContinuousQuery::new("q1", pattern);
        manager.register_query(query);

        assert_eq!(manager.query_count(), 1);

        let triple = sample_triple();
        manager.add_triple(triple).unwrap();

        assert_eq!(manager.triple_count(), 1);
    }

    #[test]
    fn test_manager_unregister() {
        let mut manager = ContinuousQueryManager::new();

        let pattern = QueryPattern::any();
        let query = ContinuousQuery::new("q1", pattern);
        manager.register_query(query);

        assert!(manager.unregister_query("q1"));
        assert_eq!(manager.query_count(), 0);
    }

    #[test]
    fn test_triple_index() {
        let mut index = TripleIndex::new();

        let triple = sample_triple();
        index.add(triple.clone());
        assert_eq!(index.count(), 1);

        index.remove(&triple);
        assert_eq!(index.count(), 0);
    }

    #[test]
    fn test_query_optimizer() {
        let pattern = QueryPattern::new(Some("ex:s1".to_string()), Some("ex:p1".to_string()), None);

        let stats = QueryOptimizer::analyze(&pattern);
        assert!(stats.selectivity < 1.0);

        let strategy = QueryOptimizer::suggest_index(&pattern);
        assert_eq!(strategy, IndexStrategy::Subject);
    }

    #[test]
    fn test_optimizer_strategies() {
        let subject_pattern = QueryPattern::new(Some("ex:s1".to_string()), None, None);
        assert_eq!(
            QueryOptimizer::suggest_index(&subject_pattern),
            IndexStrategy::Subject
        );

        let predicate_pattern = QueryPattern::new(None, Some("ex:p1".to_string()), None);
        assert_eq!(
            QueryOptimizer::suggest_index(&predicate_pattern),
            IndexStrategy::Predicate
        );

        let any_pattern = QueryPattern::any();
        assert_eq!(
            QueryOptimizer::suggest_index(&any_pattern),
            IndexStrategy::Full
        );
    }

    #[test]
    fn test_manager_list_queries() {
        let mut manager = ContinuousQueryManager::new();

        let q1 = ContinuousQuery::new("q1", QueryPattern::any());
        let q2 = ContinuousQuery::new("q2", QueryPattern::any());

        manager.register_query(q1);
        manager.register_query(q2);

        let ids = manager.list_queries();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&"q1".to_string()));
        assert!(ids.contains(&"q2".to_string()));
    }

    #[test]
    fn test_duplicate_triple_add() {
        let mut query = ContinuousQuery::new("test", QueryPattern::any());

        let triple = sample_triple();
        query.add_triple(&triple).unwrap();
        query.add_triple(&triple).unwrap(); // Add same triple again

        // Should not create duplicate results
        assert_eq!(query.result_count(), 1);
    }
}
