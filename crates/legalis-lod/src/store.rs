//! In-memory RDF triple store with optional persistence.
//!
//! This module provides an efficient in-memory storage for RDF triples
//! with support for SPARQL-like queries, index-based lookups, and file-based persistence.

use crate::{RdfValue, Triple};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

/// In-memory RDF triple store with multiple indices for efficient querying.
#[derive(Debug, Clone)]
pub struct TripleStore {
    /// All triples in the store
    triples: Vec<Triple>,
    /// Index: subject -> triple indices
    subject_index: HashMap<String, Vec<usize>>,
    /// Index: predicate -> triple indices
    predicate_index: HashMap<String, Vec<usize>>,
    /// Index: object (URIs only) -> triple indices
    object_index: HashMap<String, Vec<usize>>,
    /// Index: (subject, predicate) -> triple indices
    sp_index: HashMap<(String, String), Vec<usize>>,
}

impl Default for TripleStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TripleStore {
    /// Creates a new empty triple store.
    pub fn new() -> Self {
        Self {
            triples: Vec::new(),
            subject_index: HashMap::new(),
            predicate_index: HashMap::new(),
            object_index: HashMap::new(),
            sp_index: HashMap::new(),
        }
    }

    /// Creates a triple store from a vector of triples.
    pub fn from_triples(triples: Vec<Triple>) -> Self {
        let mut store = Self::new();
        for triple in triples {
            store.add(triple);
        }
        store
    }

    /// Adds a triple to the store.
    pub fn add(&mut self, triple: Triple) {
        let index = self.triples.len();

        // Update indices
        self.subject_index
            .entry(triple.subject.clone())
            .or_default()
            .push(index);

        self.predicate_index
            .entry(triple.predicate.clone())
            .or_default()
            .push(index);

        // Index object if it's a URI
        if let RdfValue::Uri(uri) = &triple.object {
            self.object_index
                .entry(uri.clone())
                .or_default()
                .push(index);
        }

        // Update composite index
        let sp_key = (triple.subject.clone(), triple.predicate.clone());
        self.sp_index.entry(sp_key).or_default().push(index);

        // Add the triple
        self.triples.push(triple);
    }

    /// Adds multiple triples to the store.
    pub fn add_all(&mut self, triples: Vec<Triple>) {
        for triple in triples {
            self.add(triple);
        }
    }

    /// Removes all triples matching the given pattern.
    pub fn remove(
        &mut self,
        subject: Option<&str>,
        predicate: Option<&str>,
        object: Option<&RdfValue>,
    ) -> usize {
        let to_remove: HashSet<usize> = self
            .triples
            .iter()
            .enumerate()
            .filter_map(|(i, triple)| {
                let subject_match = subject.is_none_or(|s| triple.subject == s);
                let predicate_match = predicate.is_none_or(|p| triple.predicate == p);
                let object_match = object.is_none_or(|o| match (o, &triple.object) {
                    (RdfValue::Uri(u1), RdfValue::Uri(u2)) => u1 == u2,
                    (RdfValue::Literal(l1, lang1), RdfValue::Literal(l2, lang2)) => {
                        l1 == l2 && lang1 == lang2
                    }
                    _ => false,
                });

                if subject_match && predicate_match && object_match {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        let removed_count = to_remove.len();

        // Remove triples (in reverse order to maintain indices)
        let mut indices: Vec<usize> = to_remove.into_iter().collect();
        indices.sort_unstable();
        for &i in indices.iter().rev() {
            self.triples.remove(i);
        }

        // Rebuild indices
        self.rebuild_indices();

        removed_count
    }

    /// Finds all triples with the given subject.
    pub fn find_by_subject(&self, subject: &str) -> Vec<&Triple> {
        self.subject_index
            .get(subject)
            .map(|indices| indices.iter().map(|&i| &self.triples[i]).collect())
            .unwrap_or_default()
    }

    /// Finds all triples with the given predicate.
    pub fn find_by_predicate(&self, predicate: &str) -> Vec<&Triple> {
        self.predicate_index
            .get(predicate)
            .map(|indices| indices.iter().map(|&i| &self.triples[i]).collect())
            .unwrap_or_default()
    }

    /// Finds all triples with the given object (URIs only).
    pub fn find_by_object(&self, object: &str) -> Vec<&Triple> {
        self.object_index
            .get(object)
            .map(|indices| indices.iter().map(|&i| &self.triples[i]).collect())
            .unwrap_or_default()
    }

    /// Finds all triples matching the given subject and predicate.
    pub fn find_by_sp(&self, subject: &str, predicate: &str) -> Vec<&Triple> {
        let key = (subject.to_string(), predicate.to_string());
        self.sp_index
            .get(&key)
            .map(|indices| indices.iter().map(|&i| &self.triples[i]).collect())
            .unwrap_or_default()
    }

    /// Finds all triples matching the pattern (None means wildcard).
    pub fn find(
        &self,
        subject: Option<&str>,
        predicate: Option<&str>,
        object: Option<&RdfValue>,
    ) -> Vec<&Triple> {
        // Use the most selective index
        let candidates: Vec<&Triple> = match (subject, predicate, object) {
            (Some(s), Some(p), _) => self.find_by_sp(s, p),
            (Some(s), None, _) => self.find_by_subject(s),
            (None, Some(p), _) => self.find_by_predicate(p),
            (None, None, Some(RdfValue::Uri(uri))) => self.find_by_object(uri),
            (None, None, _) => self.triples.iter().collect(),
        };

        // Filter by remaining criteria
        candidates
            .into_iter()
            .filter(|triple| {
                let subject_match = subject.is_none_or(|s| triple.subject == s);
                let predicate_match = predicate.is_none_or(|p| triple.predicate == p);
                let object_match = object.is_none_or(|o| match (o, &triple.object) {
                    (RdfValue::Uri(u1), RdfValue::Uri(u2)) => u1 == u2,
                    (RdfValue::Literal(l1, lang1), RdfValue::Literal(l2, lang2)) => {
                        l1 == l2 && lang1 == lang2
                    }
                    (RdfValue::TypedLiteral(v1, t1), RdfValue::TypedLiteral(v2, t2)) => {
                        v1 == v2 && t1 == t2
                    }
                    _ => false,
                });

                subject_match && predicate_match && object_match
            })
            .collect()
    }

    /// Returns all unique subjects in the store.
    pub fn subjects(&self) -> Vec<&str> {
        self.subject_index.keys().map(|s| s.as_str()).collect()
    }

    /// Returns all unique predicates in the store.
    pub fn predicates(&self) -> Vec<&str> {
        self.predicate_index.keys().map(|p| p.as_str()).collect()
    }

    /// Returns all triples in the store.
    pub fn all_triples(&self) -> &[Triple] {
        &self.triples
    }

    /// Returns the number of triples in the store.
    pub fn len(&self) -> usize {
        self.triples.len()
    }

    /// Checks if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.triples.is_empty()
    }

    /// Clears all triples from the store.
    pub fn clear(&mut self) {
        self.triples.clear();
        self.subject_index.clear();
        self.predicate_index.clear();
        self.object_index.clear();
        self.sp_index.clear();
    }

    /// Rebuilds all indices (used after bulk operations).
    fn rebuild_indices(&mut self) {
        self.subject_index.clear();
        self.predicate_index.clear();
        self.object_index.clear();
        self.sp_index.clear();

        for (i, triple) in self.triples.iter().enumerate() {
            self.subject_index
                .entry(triple.subject.clone())
                .or_default()
                .push(i);

            self.predicate_index
                .entry(triple.predicate.clone())
                .or_default()
                .push(i);

            if let RdfValue::Uri(uri) = &triple.object {
                self.object_index.entry(uri.clone()).or_default().push(i);
            }

            let sp_key = (triple.subject.clone(), triple.predicate.clone());
            self.sp_index.entry(sp_key).or_default().push(i);
        }
    }

    /// Gets statistics about the store.
    pub fn stats(&self) -> StoreStatistics {
        StoreStatistics {
            triple_count: self.triples.len(),
            subject_count: self.subject_index.len(),
            predicate_count: self.predicate_index.len(),
            object_count: self.object_index.len(),
        }
    }

    /// Merges another store into this one.
    pub fn merge(&mut self, other: TripleStore) {
        for triple in other.triples {
            self.add(triple);
        }
    }

    /// Partitions the store by subject prefix.
    pub fn partition_by_subject_prefix(&self, prefix: &str) -> (TripleStore, TripleStore) {
        let mut matching = TripleStore::new();
        let mut non_matching = TripleStore::new();

        for triple in &self.triples {
            if triple.subject.starts_with(prefix) {
                matching.add(triple.clone());
            } else {
                non_matching.add(triple.clone());
            }
        }

        (matching, non_matching)
    }

    /// Partitions the store by predicate.
    pub fn partition_by_predicate(&self, predicate: &str) -> (TripleStore, TripleStore) {
        let mut matching = TripleStore::new();
        let mut non_matching = TripleStore::new();

        for triple in &self.triples {
            if triple.predicate == predicate {
                matching.add(triple.clone());
            } else {
                non_matching.add(triple.clone());
            }
        }

        (matching, non_matching)
    }

    /// Partitions the store into N roughly equal parts.
    pub fn partition_by_size(&self, num_partitions: usize) -> Vec<TripleStore> {
        if num_partitions == 0 {
            return vec![];
        }

        let partition_size = self.triples.len().div_ceil(num_partitions);
        let mut partitions = Vec::new();

        for chunk in self.triples.chunks(partition_size) {
            let mut store = TripleStore::new();
            for triple in chunk {
                store.add(triple.clone());
            }
            partitions.push(store);
        }

        partitions
    }

    /// Partitions the store by named graph (using subject as graph identifier).
    pub fn partition_by_graph<F>(&self, graph_fn: F) -> HashMap<String, TripleStore>
    where
        F: Fn(&Triple) -> String,
    {
        let mut graphs: HashMap<String, TripleStore> = HashMap::new();

        for triple in &self.triples {
            let graph_name = graph_fn(triple);
            graphs.entry(graph_name).or_default().add(triple.clone());
        }

        graphs
    }

    /// Partitions large datasets by subject to enable parallel processing.
    pub fn partition_for_parallel_processing(&self) -> Vec<TripleStore> {
        const OPTIMAL_PARTITION_SIZE: usize = 10000;

        if self.triples.len() <= OPTIMAL_PARTITION_SIZE {
            return vec![self.clone()];
        }

        let num_partitions = self.triples.len().div_ceil(OPTIMAL_PARTITION_SIZE);
        self.partition_by_size(num_partitions)
    }

    /// Saves the triple store to a file in N-Triples format.
    /// This provides basic persistence for the triple store.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        for triple in &self.triples {
            let line = self.triple_to_ntriples(triple);
            writeln!(writer, "{}", line)?;
        }

        writer.flush()?;
        Ok(())
    }

    /// Loads a triple store from a file in N-Triples format.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        let mut store = Self::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(triple) = Self::parse_ntriples_line(line) {
                store.add(triple);
            }
        }

        Ok(store)
    }

    /// Converts a triple to N-Triples format.
    fn triple_to_ntriples(&self, triple: &Triple) -> String {
        let subject = format!("<{}>", triple.subject);
        let predicate = format!("<{}>", triple.predicate);
        let object = match &triple.object {
            RdfValue::Uri(uri) => format!("<{}>", uri),
            RdfValue::Literal(s, None) => format!("\"{}\"", escape_ntriples(s)),
            RdfValue::Literal(s, Some(lang)) => {
                format!("\"{}\"@{}", escape_ntriples(s), lang)
            }
            RdfValue::TypedLiteral(s, dtype) => {
                format!("\"{}\"^^<{}>", escape_ntriples(s), dtype)
            }
            RdfValue::BlankNode(id) => format!("_:{}", id),
        };

        format!("{} {} {} .", subject, predicate, object)
    }

    /// Parses a single N-Triples line into a Triple.
    fn parse_ntriples_line(line: &str) -> Option<Triple> {
        let line = line.trim_end_matches('.');
        let line = line.trim();

        let parts: Vec<&str> = line.splitn(3, char::is_whitespace).collect();
        if parts.len() < 3 {
            return None;
        }

        let subject = parts[0].trim_matches(|c| c == '<' || c == '>').to_string();
        let predicate = parts[1].trim_matches(|c| c == '<' || c == '>').to_string();

        let object_str = parts[2].trim();
        let object = if object_str.starts_with('<') && object_str.ends_with('>') {
            RdfValue::Uri(
                object_str
                    .trim_matches(|c| c == '<' || c == '>')
                    .to_string(),
            )
        } else if object_str.starts_with('\"') {
            parse_ntriples_literal(object_str)
        } else if let Some(id) = object_str.strip_prefix("_:") {
            RdfValue::BlankNode(id.to_string())
        } else {
            return None;
        };

        Some(Triple {
            subject,
            predicate,
            object,
        })
    }

    /// Saves the triple store to a JSON file (more compact than N-Triples).
    pub fn save_to_json<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        // Serialize as JSON using serde_json
        let serializable: Vec<SerializableTriple> = self
            .triples
            .iter()
            .map(SerializableTriple::from_triple)
            .collect();

        serde_json::to_writer_pretty(writer, &serializable).map_err(std::io::Error::other)?;

        Ok(())
    }

    /// Loads a triple store from a JSON file.
    pub fn load_from_json<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let serializable: Vec<SerializableTriple> =
            serde_json::from_reader(reader).map_err(std::io::Error::other)?;

        let mut store = Self::new();
        for st in serializable {
            store.add(st.to_triple());
        }

        Ok(store)
    }
}

/// Statistics about a triple store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreStatistics {
    /// Number of triples
    pub triple_count: usize,
    /// Number of unique subjects
    pub subject_count: usize,
    /// Number of unique predicates
    pub predicate_count: usize,
    /// Number of unique object URIs
    pub object_count: usize,
}

/// Serializable representation of a triple for JSON persistence.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct SerializableTriple {
    subject: String,
    predicate: String,
    object: SerializableRdfValue,
}

/// Serializable representation of an RDF value.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum SerializableRdfValue {
    Uri(String),
    Literal { value: String, lang: Option<String> },
    TypedLiteral { value: String, datatype: String },
    BlankNode(String),
}

impl SerializableTriple {
    fn from_triple(triple: &Triple) -> Self {
        Self {
            subject: triple.subject.clone(),
            predicate: triple.predicate.clone(),
            object: SerializableRdfValue::from_rdf_value(&triple.object),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_triple(self) -> Triple {
        Triple {
            subject: self.subject,
            predicate: self.predicate,
            object: self.object.into_rdf_value(),
        }
    }
}

impl SerializableRdfValue {
    fn from_rdf_value(value: &RdfValue) -> Self {
        match value {
            RdfValue::Uri(uri) => Self::Uri(uri.clone()),
            RdfValue::Literal(value, lang) => Self::Literal {
                value: value.clone(),
                lang: lang.clone(),
            },
            RdfValue::TypedLiteral(value, datatype) => Self::TypedLiteral {
                value: value.clone(),
                datatype: datatype.clone(),
            },
            RdfValue::BlankNode(id) => Self::BlankNode(id.clone()),
        }
    }

    fn into_rdf_value(self) -> RdfValue {
        match self {
            Self::Uri(uri) => RdfValue::Uri(uri),
            Self::Literal { value, lang } => RdfValue::Literal(value, lang),
            Self::TypedLiteral { value, datatype } => RdfValue::TypedLiteral(value, datatype),
            Self::BlankNode(id) => RdfValue::BlankNode(id),
        }
    }
}

/// Escapes a string for N-Triples format.
fn escape_ntriples(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Parses an N-Triples literal value.
fn parse_ntriples_literal(s: &str) -> RdfValue {
    // Handle language tags: "value"@lang
    if let Some(pos) = s.rfind("\"@") {
        let value = unescape_ntriples(&s[1..pos]);
        let lang = s[pos + 2..].to_string();
        return RdfValue::Literal(value, Some(lang));
    }

    // Handle typed literals: "value"^^<datatype>
    if let Some(pos) = s.rfind("\"^^") {
        let value = unescape_ntriples(&s[1..pos]);
        let datatype = s[pos + 3..]
            .trim_matches(|c| c == '<' || c == '>')
            .to_string();
        return RdfValue::TypedLiteral(value, datatype);
    }

    // Plain literal
    if s.starts_with('"') && s.ends_with('"') {
        let value = unescape_ntriples(&s[1..s.len() - 1]);
        return RdfValue::Literal(value, None);
    }

    RdfValue::string(s)
}

/// Unescapes an N-Triples string.
fn unescape_ntriples(s: &str) -> String {
    s.replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
}

/// Query builder for the triple store.
#[derive(Debug)]
pub struct QueryBuilder<'a> {
    store: &'a TripleStore,
    subject: Option<String>,
    predicate: Option<String>,
    object: Option<RdfValue>,
}

impl<'a> QueryBuilder<'a> {
    /// Creates a new query builder.
    pub fn new(store: &'a TripleStore) -> Self {
        Self {
            store,
            subject: None,
            predicate: None,
            object: None,
        }
    }

    /// Sets the subject filter.
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Sets the predicate filter.
    pub fn predicate(mut self, predicate: impl Into<String>) -> Self {
        self.predicate = Some(predicate.into());
        self
    }

    /// Sets the object filter.
    pub fn object(mut self, object: RdfValue) -> Self {
        self.object = Some(object);
        self
    }

    /// Executes the query and returns matching triples.
    pub fn execute(self) -> Vec<&'a Triple> {
        self.store.find(
            self.subject.as_deref(),
            self.predicate.as_deref(),
            self.object.as_ref(),
        )
    }

    /// Returns the count of matching triples.
    pub fn count(self) -> usize {
        self.execute().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_triples() -> Vec<Triple> {
        vec![
            Triple {
                subject: "statute:1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("eli:LegalResource".to_string()),
            },
            Triple {
                subject: "statute:1".to_string(),
                predicate: "eli:title".to_string(),
                object: RdfValue::string("Test Statute"),
            },
            Triple {
                subject: "statute:2".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("eli:LegalResource".to_string()),
            },
        ]
    }

    #[test]
    fn test_create_store() {
        let store = TripleStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_add_triple() {
        let mut store = TripleStore::new();
        let triples = sample_triples();

        store.add(triples[0].clone());
        assert_eq!(store.len(), 1);
        assert!(!store.is_empty());
    }

    #[test]
    fn test_add_all() {
        let mut store = TripleStore::new();
        let triples = sample_triples();

        store.add_all(triples.clone());
        assert_eq!(store.len(), 3);
    }

    #[test]
    fn test_from_triples() {
        let triples = sample_triples();
        let store = TripleStore::from_triples(triples);

        assert_eq!(store.len(), 3);
    }

    #[test]
    fn test_find_by_subject() {
        let store = TripleStore::from_triples(sample_triples());

        let results = store.find_by_subject("statute:1");
        assert_eq!(results.len(), 2);

        let results = store.find_by_subject("statute:2");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_find_by_predicate() {
        let store = TripleStore::from_triples(sample_triples());

        let results = store.find_by_predicate("rdf:type");
        assert_eq!(results.len(), 2);

        let results = store.find_by_predicate("eli:title");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_find_by_object() {
        let store = TripleStore::from_triples(sample_triples());

        let results = store.find_by_object("eli:LegalResource");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_find_by_sp() {
        let store = TripleStore::from_triples(sample_triples());

        let results = store.find_by_sp("statute:1", "rdf:type");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_find_pattern() {
        let store = TripleStore::from_triples(sample_triples());

        // Find all triples
        let results = store.find(None, None, None);
        assert_eq!(results.len(), 3);

        // Find by subject only
        let results = store.find(Some("statute:1"), None, None);
        assert_eq!(results.len(), 2);

        // Find by predicate only
        let results = store.find(None, Some("rdf:type"), None);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_remove() {
        let mut store = TripleStore::from_triples(sample_triples());

        let removed = store.remove(Some("statute:1"), None, None);
        assert_eq!(removed, 2);
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_subjects() {
        let store = TripleStore::from_triples(sample_triples());

        let subjects = store.subjects();
        assert_eq!(subjects.len(), 2);
        assert!(subjects.contains(&"statute:1"));
        assert!(subjects.contains(&"statute:2"));
    }

    #[test]
    fn test_predicates() {
        let store = TripleStore::from_triples(sample_triples());

        let predicates = store.predicates();
        assert_eq!(predicates.len(), 2);
        assert!(predicates.contains(&"rdf:type"));
        assert!(predicates.contains(&"eli:title"));
    }

    #[test]
    fn test_stats() {
        let store = TripleStore::from_triples(sample_triples());

        let stats = store.stats();
        assert_eq!(stats.triple_count, 3);
        assert_eq!(stats.subject_count, 2);
        assert_eq!(stats.predicate_count, 2);
    }

    #[test]
    fn test_clear() {
        let mut store = TripleStore::from_triples(sample_triples());

        store.clear();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_merge() {
        let mut store1 = TripleStore::from_triples(vec![sample_triples()[0].clone()]);
        let store2 = TripleStore::from_triples(vec![sample_triples()[1].clone()]);

        store1.merge(store2);
        assert_eq!(store1.len(), 2);
    }

    #[test]
    fn test_query_builder() {
        let store = TripleStore::from_triples(sample_triples());

        let results = QueryBuilder::new(&store).subject("statute:1").execute();
        assert_eq!(results.len(), 2);

        let count = QueryBuilder::new(&store).predicate("rdf:type").count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_query_builder_chaining() {
        let store = TripleStore::from_triples(sample_triples());

        let results = QueryBuilder::new(&store)
            .subject("statute:1")
            .predicate("rdf:type")
            .execute();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_partition_by_subject_prefix() {
        let store = TripleStore::from_triples(sample_triples());

        let (matching, non_matching) = store.partition_by_subject_prefix("statute:1");
        assert_eq!(matching.len(), 2);
        assert_eq!(non_matching.len(), 1);
    }

    #[test]
    fn test_partition_by_predicate() {
        let store = TripleStore::from_triples(sample_triples());

        let (matching, non_matching) = store.partition_by_predicate("rdf:type");
        assert_eq!(matching.len(), 2);
        assert_eq!(non_matching.len(), 1);
    }

    #[test]
    fn test_partition_by_size() {
        let store = TripleStore::from_triples(sample_triples());

        let partitions = store.partition_by_size(2);
        assert_eq!(partitions.len(), 2);

        let total_triples: usize = partitions.iter().map(|p| p.len()).sum();
        assert_eq!(total_triples, 3);
    }

    #[test]
    fn test_partition_by_graph() {
        let store = TripleStore::from_triples(sample_triples());

        let graphs = store.partition_by_graph(|triple| {
            // Group by subject
            triple.subject.clone()
        });

        assert_eq!(graphs.len(), 2); // statute:1 and statute:2
        assert!(graphs.contains_key("statute:1"));
        assert!(graphs.contains_key("statute:2"));
    }

    #[test]
    fn test_partition_for_parallel_processing() {
        let store = TripleStore::from_triples(sample_triples());

        let partitions = store.partition_for_parallel_processing();
        // With only 3 triples, should return a single partition
        assert_eq!(partitions.len(), 1);
    }

    #[test]
    fn test_save_and_load_ntriples() {
        let store = TripleStore::from_triples(sample_triples());
        let temp_file = "/tmp/test_store.nt";

        // Save
        store.save_to_file(temp_file).unwrap();

        // Load
        let loaded_store = TripleStore::load_from_file(temp_file).unwrap();

        assert_eq!(loaded_store.len(), store.len());
        assert_eq!(loaded_store.subjects().len(), store.subjects().len());

        // Cleanup
        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_save_and_load_json() {
        let store = TripleStore::from_triples(sample_triples());
        let temp_file = "/tmp/test_store.json";

        // Save
        store.save_to_json(temp_file).unwrap();

        // Load
        let loaded_store = TripleStore::load_from_json(temp_file).unwrap();

        assert_eq!(loaded_store.len(), store.len());
        assert_eq!(loaded_store.subjects().len(), store.subjects().len());

        // Cleanup
        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_persistence_with_special_characters() {
        let mut store = TripleStore::new();
        store.add(Triple {
            subject: "test:subject".to_string(),
            predicate: "test:predicate".to_string(),
            object: RdfValue::Literal("Line 1\nLine 2\t\"quoted\"".to_string(), None),
        });

        let temp_file = "/tmp/test_special_chars.nt";
        store.save_to_file(temp_file).unwrap();
        let loaded = TripleStore::load_from_file(temp_file).unwrap();

        assert_eq!(loaded.len(), 1);
        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_persistence_with_language_tags() {
        let mut store = TripleStore::new();
        store.add(Triple {
            subject: "test:subject".to_string(),
            predicate: "rdfs:label".to_string(),
            object: RdfValue::Literal("Hello".to_string(), Some("en".to_string())),
        });

        let temp_file = "/tmp/test_lang.json";
        store.save_to_json(temp_file).unwrap();
        let loaded = TripleStore::load_from_json(temp_file).unwrap();

        assert_eq!(loaded.len(), 1);
        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_empty_store_persistence() {
        let store = TripleStore::new();
        let temp_file = "/tmp/test_empty.nt";

        store.save_to_file(temp_file).unwrap();
        let loaded = TripleStore::load_from_file(temp_file).unwrap();

        assert!(loaded.is_empty());
        std::fs::remove_file(temp_file).ok();
    }
}
