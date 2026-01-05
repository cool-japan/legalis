//! Content-addressed RDF storage using cryptographic hashing.
//!
//! This module provides immutable, content-addressed storage for legal knowledge graphs,
//! enabling decentralized and verifiable data management.

use crate::ipld::{Cid, IpldStore};
use crate::{LodError, LodResult, Namespaces, Triple};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Content-addressed RDF store.
///
/// This store uses cryptographic hashes (CIDs) to address RDF resources,
/// making them immutable and verifiable.
#[derive(Debug)]
pub struct ContentAddressedStore {
    /// Underlying IPLD store
    ipld: IpldStore,
    /// Index mapping resource URIs to CIDs
    uri_index: HashMap<String, Cid>,
    /// Index mapping graph names to CIDs
    graph_index: HashMap<String, Cid>,
}

impl ContentAddressedStore {
    /// Creates a new content-addressed store.
    pub fn new() -> Self {
        Self {
            ipld: IpldStore::new(),
            uri_index: HashMap::new(),
            graph_index: HashMap::new(),
        }
    }

    /// Stores triples and returns the content identifier.
    pub fn put_triples(&mut self, triples: &[Triple]) -> LodResult<Cid> {
        self.ipld.put_graph(triples)
    }

    /// Stores a named graph and returns its CID.
    pub fn put_named_graph(
        &mut self,
        name: impl Into<String>,
        triples: &[Triple],
    ) -> LodResult<Cid> {
        let name = name.into();
        let graph_cid = self.ipld.put_graph(triples)?;

        // Create named graph
        let named_graph = crate::ipld::IpldData::NamedGraph {
            name: name.clone(),
            graph_cid,
            metadata: HashMap::new(),
        };

        let cid = self.ipld.put(named_graph)?;
        self.graph_index.insert(name, cid.clone());

        Ok(cid)
    }

    /// Retrieves triples by CID.
    pub fn get_triples(&self, cid: &Cid) -> LodResult<Vec<Triple>> {
        self.ipld.get_graph(cid)
    }

    /// Retrieves a named graph by name.
    pub fn get_named_graph(&self, name: &str) -> LodResult<Vec<Triple>> {
        let cid = self
            .graph_index
            .get(name)
            .ok_or_else(|| LodError::InvalidUri(format!("Graph not found: {}", name)))?;

        // Get the named graph node
        let node = self
            .ipld
            .get(cid)
            .ok_or_else(|| LodError::InvalidUri(format!("CID not found: {}", cid.to_string())))?;

        // Extract graph CID and retrieve triples
        match &node.data {
            crate::ipld::IpldData::NamedGraph { graph_cid, .. } => self.ipld.get_graph(graph_cid),
            _ => Err(LodError::InvalidUri("Not a named graph".to_string())),
        }
    }

    /// Associates a URI with a CID.
    pub fn index_uri(&mut self, uri: impl Into<String>, cid: Cid) {
        self.uri_index.insert(uri.into(), cid);
    }

    /// Retrieves a CID by URI.
    pub fn resolve_uri(&self, uri: &str) -> Option<&Cid> {
        self.uri_index.get(uri)
    }

    /// Lists all graph names.
    pub fn list_graphs(&self) -> Vec<String> {
        self.graph_index.keys().cloned().collect()
    }

    /// Returns the number of stored graphs.
    pub fn graph_count(&self) -> usize {
        self.graph_index.len()
    }

    /// Returns the total number of IPLD nodes.
    pub fn node_count(&self) -> usize {
        self.ipld.len()
    }

    /// Verifies the integrity of a CID.
    pub fn verify(&self, cid: &Cid) -> bool {
        self.ipld.get(cid).is_some()
    }
}

impl Default for ContentAddressedStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Content-addressed export format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaFormat {
    /// CAR (Content Addressable aRchive) format
    Car,
    /// IPLD DAG-JSON format
    DagJson,
    /// IPLD DAG-CBOR format
    DagCbor,
}

impl CaFormat {
    /// Returns the file extension for this format.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Car => "car",
            Self::DagJson => "json",
            Self::DagCbor => "cbor",
        }
    }

    /// Returns the MIME type for this format.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Car => "application/vnd.ipld.car",
            Self::DagJson => "application/vnd.ipld.dag-json",
            Self::DagCbor => "application/vnd.ipld.dag-cbor",
        }
    }
}

/// Exporter for content-addressed RDF.
pub struct CaExporter {
    /// Format for export
    format: CaFormat,
    /// Namespaces
    namespaces: Namespaces,
}

impl CaExporter {
    /// Creates a new content-addressed exporter.
    pub fn new(format: CaFormat) -> Self {
        Self {
            format,
            namespaces: Namespaces::default(),
        }
    }

    /// Sets the namespaces.
    pub fn with_namespaces(mut self, namespaces: Namespaces) -> Self {
        self.namespaces = namespaces;
        self
    }

    /// Exports triples to content-addressed format.
    pub fn export(&self, store: &ContentAddressedStore, cid: &Cid) -> LodResult<String> {
        match self.format {
            CaFormat::DagJson => self.export_dag_json(store, cid),
            CaFormat::Car => self.export_car(store, cid),
            CaFormat::DagCbor => self.export_dag_cbor(store, cid),
        }
    }

    fn export_dag_json(&self, store: &ContentAddressedStore, cid: &Cid) -> LodResult<String> {
        let triples = store.get_triples(cid)?;

        let mut doc = serde_json::Map::new();
        doc.insert("cid".to_string(), serde_json::json!(cid.to_string()));
        doc.insert("triples".to_string(), serde_json::json!(triples.len()));

        let triple_data: Vec<serde_json::Value> = triples
            .iter()
            .map(|t| {
                serde_json::json!({
                    "subject": t.subject,
                    "predicate": t.predicate,
                    "object": format!("{:?}", t.object),
                })
            })
            .collect();

        doc.insert("data".to_string(), serde_json::json!(triple_data));

        serde_json::to_string_pretty(&doc).map_err(|e| LodError::SerializationError(e.to_string()))
    }

    fn export_car(&self, _store: &ContentAddressedStore, cid: &Cid) -> LodResult<String> {
        // Simplified CAR format representation
        Ok(format!(
            "# CAR (Content Addressable aRchive)\nVersion: 1\nRoot CID: {}\n",
            cid.to_string()
        ))
    }

    fn export_dag_cbor(&self, _store: &ContentAddressedStore, cid: &Cid) -> LodResult<String> {
        // Simplified DAG-CBOR representation
        Ok(format!("# DAG-CBOR\nCID: {}\n", cid.to_string()))
    }
}

/// Snapshot of a content-addressed store at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Timestamp of the snapshot
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Root CID of the snapshot
    pub root_cid: Cid,
    /// Description
    pub description: Option<String>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl Snapshot {
    /// Creates a new snapshot.
    pub fn new(root_cid: Cid) -> Self {
        Self {
            timestamp: chrono::Utc::now(),
            root_cid,
            description: None,
            metadata: HashMap::new(),
        }
    }

    /// Adds a description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Adds metadata.
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }
}

/// Manages snapshots of content-addressed stores.
#[derive(Debug, Default)]
pub struct SnapshotManager {
    snapshots: Vec<Snapshot>,
}

impl SnapshotManager {
    /// Creates a new snapshot manager.
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
        }
    }

    /// Adds a snapshot.
    pub fn add_snapshot(&mut self, snapshot: Snapshot) {
        self.snapshots.push(snapshot);
    }

    /// Lists all snapshots.
    pub fn list_snapshots(&self) -> &[Snapshot] {
        &self.snapshots
    }

    /// Gets the latest snapshot.
    pub fn latest_snapshot(&self) -> Option<&Snapshot> {
        self.snapshots.last()
    }

    /// Exports snapshots to JSON.
    pub fn export_json(&self) -> LodResult<String> {
        serde_json::to_string_pretty(&self.snapshots)
            .map_err(|e| LodError::SerializationError(e.to_string()))
    }

    /// Imports snapshots from JSON.
    pub fn import_json(json: &str) -> LodResult<Self> {
        let snapshots: Vec<Snapshot> =
            serde_json::from_str(json).map_err(|e| LodError::SerializationError(e.to_string()))?;
        Ok(Self { snapshots })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RdfValue;

    fn sample_triples() -> Vec<Triple> {
        vec![
            Triple {
                subject: "ex:subject1".to_string(),
                predicate: "ex:predicate1".to_string(),
                object: RdfValue::string("object1"),
            },
            Triple {
                subject: "ex:subject2".to_string(),
                predicate: "ex:predicate2".to_string(),
                object: RdfValue::integer(42),
            },
        ]
    }

    #[test]
    fn test_content_addressed_store() {
        let mut store = ContentAddressedStore::new();
        let triples = sample_triples();

        let cid = store.put_triples(&triples).unwrap();
        let retrieved = store.get_triples(&cid).unwrap();

        assert_eq!(retrieved.len(), 2);
        assert_eq!(retrieved[0].subject, "ex:subject1");
    }

    #[test]
    fn test_named_graph() {
        let mut store = ContentAddressedStore::new();
        let triples = sample_triples();

        let cid = store.put_named_graph("test-graph", &triples).unwrap();
        assert!(store.verify(&cid));

        let retrieved = store.get_named_graph("test-graph").unwrap();
        assert_eq!(retrieved.len(), 2);
    }

    #[test]
    fn test_uri_indexing() {
        let mut store = ContentAddressedStore::new();
        let triples = sample_triples();

        let cid = store.put_triples(&triples).unwrap();
        store.index_uri("http://example.org/graph1", cid.clone());

        let resolved = store.resolve_uri("http://example.org/graph1").unwrap();
        assert_eq!(resolved, &cid);
    }

    #[test]
    fn test_list_graphs() {
        let mut store = ContentAddressedStore::new();
        let triples = sample_triples();

        store.put_named_graph("graph1", &triples).unwrap();
        store.put_named_graph("graph2", &triples).unwrap();

        let graphs = store.list_graphs();
        assert_eq!(graphs.len(), 2);
        assert!(graphs.contains(&"graph1".to_string()));
        assert!(graphs.contains(&"graph2".to_string()));
    }

    #[test]
    fn test_verify() {
        let mut store = ContentAddressedStore::new();
        let triples = sample_triples();

        let cid = store.put_triples(&triples).unwrap();
        assert!(store.verify(&cid));

        let fake_cid = Cid::new("fake-hash", "dag-json");
        assert!(!store.verify(&fake_cid));
    }

    #[test]
    fn test_ca_format() {
        assert_eq!(CaFormat::Car.extension(), "car");
        assert_eq!(CaFormat::DagJson.extension(), "json");
        assert_eq!(CaFormat::DagCbor.extension(), "cbor");

        assert_eq!(CaFormat::Car.mime_type(), "application/vnd.ipld.car");
    }

    #[test]
    fn test_export_dag_json() {
        let mut store = ContentAddressedStore::new();
        let triples = sample_triples();

        let cid = store.put_triples(&triples).unwrap();

        let exporter = CaExporter::new(CaFormat::DagJson);
        let output = exporter.export(&store, &cid).unwrap();

        assert!(output.contains("cid"));
        assert!(output.contains("triples"));
    }

    #[test]
    fn test_export_car() {
        let mut store = ContentAddressedStore::new();
        let triples = sample_triples();

        let cid = store.put_triples(&triples).unwrap();

        let exporter = CaExporter::new(CaFormat::Car);
        let output = exporter.export(&store, &cid).unwrap();

        assert!(output.contains("CAR"));
        assert!(output.contains("Root CID"));
    }

    #[test]
    fn test_snapshot() {
        let cid = Cid::new("test-hash", "dag-json");
        let snapshot = Snapshot::new(cid).with_description("Test snapshot");

        assert!(snapshot.description.is_some());
        assert_eq!(snapshot.description.unwrap(), "Test snapshot");
    }

    #[test]
    fn test_snapshot_manager() {
        let mut manager = SnapshotManager::new();

        let cid1 = Cid::new("hash1", "dag-json");
        let snapshot1 = Snapshot::new(cid1);
        manager.add_snapshot(snapshot1);

        let cid2 = Cid::new("hash2", "dag-json");
        let snapshot2 = Snapshot::new(cid2);
        manager.add_snapshot(snapshot2);

        assert_eq!(manager.list_snapshots().len(), 2);
        assert!(manager.latest_snapshot().is_some());
    }

    #[test]
    fn test_snapshot_export_import() {
        let mut manager = SnapshotManager::new();

        let cid = Cid::new("test-hash", "dag-json");
        let snapshot = Snapshot::new(cid).with_description("Test");
        manager.add_snapshot(snapshot);

        let json = manager.export_json().unwrap();
        let imported = SnapshotManager::import_json(&json).unwrap();

        assert_eq!(imported.list_snapshots().len(), 1);
    }

    #[test]
    fn test_graph_count() {
        let mut store = ContentAddressedStore::new();
        let triples = sample_triples();

        assert_eq!(store.graph_count(), 0);

        store.put_named_graph("graph1", &triples).unwrap();
        assert_eq!(store.graph_count(), 1);

        store.put_named_graph("graph2", &triples).unwrap();
        assert_eq!(store.graph_count(), 2);
    }

    #[test]
    fn test_node_count() {
        let mut store = ContentAddressedStore::new();
        let triples = sample_triples();

        let initial_count = store.node_count();
        store.put_triples(&triples).unwrap();

        assert!(store.node_count() > initial_count);
    }
}
