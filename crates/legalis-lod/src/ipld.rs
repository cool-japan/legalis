//! IPLD (InterPlanetary Linked Data) support for legal knowledge graphs.
//!
//! This module provides content-addressed RDF storage using IPLD data structures,
//! enabling decentralized legal data storage and retrieval.

use crate::{LodError, LodResult, RdfValue, Triple};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Content Identifier (CID) representation.
///
/// In a full implementation, this would use the `cid` crate,
/// but for now we use a simplified representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cid {
    /// Base58-encoded multihash
    pub hash: String,
    /// Codec identifier (e.g., "dag-cbor", "dag-json")
    pub codec: String,
}

impl Cid {
    /// Creates a new CID from hash and codec.
    pub fn new(hash: impl Into<String>, codec: impl Into<String>) -> Self {
        Self {
            hash: hash.into(),
            codec: codec.into(),
        }
    }

    /// Generates a CID from content using SHA-256.
    pub fn from_bytes(content: &[u8], codec: impl Into<String>) -> Self {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(content);
        let hash = format!("{:x}", hasher.finalize());
        Self::new(hash, codec)
    }

    /// Returns the string representation (base58-encoded).
    pub fn to_string(&self) -> String {
        format!("{}/{}", self.codec, self.hash)
    }
}

/// IPLD node representing a legal resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpldNode {
    /// Content identifier for this node
    pub cid: Cid,
    /// Node data
    pub data: IpldData,
}

/// IPLD data types for legal resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IpldData {
    /// RDF triple stored as IPLD
    Triple {
        subject: String,
        predicate: String,
        object: IpldValue,
    },
    /// Collection of triples (graph)
    Graph { triples: Vec<Cid> },
    /// Named graph with metadata
    NamedGraph {
        name: String,
        graph_cid: Cid,
        metadata: HashMap<String, String>,
    },
    /// Legal statute representation
    Statute {
        id: String,
        title: String,
        graph_cid: Cid,
    },
}

/// IPLD value representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "value_type")]
pub enum IpldValue {
    /// URI reference
    Uri { uri: String },
    /// String literal
    Literal { value: String, lang: Option<String> },
    /// Typed literal
    TypedLiteral { value: String, datatype: String },
    /// Link to another IPLD node
    Link { cid: Cid },
}

impl From<&RdfValue> for IpldValue {
    fn from(value: &RdfValue) -> Self {
        match value {
            RdfValue::Uri(uri) => IpldValue::Uri { uri: uri.clone() },
            RdfValue::Literal(s, lang) => IpldValue::Literal {
                value: s.clone(),
                lang: lang.clone(),
            },
            RdfValue::TypedLiteral(s, dtype) => IpldValue::TypedLiteral {
                value: s.clone(),
                datatype: dtype.clone(),
            },
            RdfValue::BlankNode(id) => IpldValue::Literal {
                value: format!("_:{}", id),
                lang: None,
            },
        }
    }
}

/// IPLD store for content-addressed RDF storage.
#[derive(Debug)]
pub struct IpldStore {
    /// Storage for IPLD nodes indexed by CID
    nodes: HashMap<String, IpldNode>,
}

impl IpldStore {
    /// Creates a new IPLD store.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Stores an IPLD node and returns its CID.
    pub fn put(&mut self, data: IpldData) -> LodResult<Cid> {
        // Serialize data to compute CID
        let json =
            serde_json::to_vec(&data).map_err(|e| LodError::SerializationError(e.to_string()))?;

        let cid = Cid::from_bytes(&json, "dag-json");

        let node = IpldNode {
            cid: cid.clone(),
            data,
        };

        self.nodes.insert(cid.to_string(), node);
        Ok(cid)
    }

    /// Retrieves an IPLD node by CID.
    pub fn get(&self, cid: &Cid) -> Option<&IpldNode> {
        self.nodes.get(&cid.to_string())
    }

    /// Stores a triple as an IPLD node.
    pub fn put_triple(&mut self, triple: &Triple) -> LodResult<Cid> {
        let data = IpldData::Triple {
            subject: triple.subject.clone(),
            predicate: triple.predicate.clone(),
            object: IpldValue::from(&triple.object),
        };
        self.put(data)
    }

    /// Stores a collection of triples as a graph.
    pub fn put_graph(&mut self, triples: &[Triple]) -> LodResult<Cid> {
        let mut triple_cids = Vec::new();

        for triple in triples {
            let cid = self.put_triple(triple)?;
            triple_cids.push(cid);
        }

        let data = IpldData::Graph {
            triples: triple_cids,
        };
        self.put(data)
    }

    /// Retrieves all triples from a graph CID.
    pub fn get_graph(&self, cid: &Cid) -> LodResult<Vec<Triple>> {
        let node = self
            .get(cid)
            .ok_or_else(|| LodError::InvalidUri(format!("CID not found: {}", cid.to_string())))?;

        match &node.data {
            IpldData::Graph { triples } => {
                let mut result = Vec::new();
                for triple_cid in triples {
                    if let Some(triple_node) = self.get(triple_cid) {
                        if let IpldData::Triple {
                            subject,
                            predicate,
                            object,
                        } = &triple_node.data
                        {
                            result.push(Triple {
                                subject: subject.clone(),
                                predicate: predicate.clone(),
                                object: ipld_value_to_rdf(object),
                            });
                        }
                    }
                }
                Ok(result)
            }
            _ => Err(LodError::InvalidUri("Not a graph CID".to_string())),
        }
    }

    /// Returns the number of nodes in the store.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns true if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Lists all CIDs in the store.
    pub fn list_cids(&self) -> Vec<Cid> {
        self.nodes.values().map(|node| node.cid.clone()).collect()
    }

    /// Exports the entire store as JSON.
    pub fn export_json(&self) -> LodResult<String> {
        serde_json::to_string_pretty(&self.nodes)
            .map_err(|e| LodError::SerializationError(e.to_string()))
    }

    /// Imports a store from JSON.
    pub fn import_json(json: &str) -> LodResult<Self> {
        let nodes: HashMap<String, IpldNode> =
            serde_json::from_str(json).map_err(|e| LodError::SerializationError(e.to_string()))?;
        Ok(Self { nodes })
    }
}

impl Default for IpldStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Converts an IPLD value back to RDF value.
fn ipld_value_to_rdf(value: &IpldValue) -> RdfValue {
    match value {
        IpldValue::Uri { uri } => RdfValue::Uri(uri.clone()),
        IpldValue::Literal { value, lang } => RdfValue::Literal(value.clone(), lang.clone()),
        IpldValue::TypedLiteral { value, datatype } => {
            RdfValue::TypedLiteral(value.clone(), datatype.clone())
        }
        IpldValue::Link { cid } => RdfValue::Uri(format!("ipld://{}", cid.to_string())),
    }
}

/// IPLD path for traversing linked data structures.
#[derive(Debug, Clone)]
pub struct IpldPath {
    /// Path segments
    segments: Vec<String>,
}

impl IpldPath {
    /// Creates a new IPLD path from a string.
    pub fn new(path: &str) -> Self {
        let segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        Self { segments }
    }

    /// Traverses the path in the IPLD store.
    pub fn traverse(&self, store: &IpldStore, start_cid: &Cid) -> LodResult<Option<IpldNode>> {
        let mut current_cid = start_cid.clone();

        for segment in &self.segments {
            let node = store.get(&current_cid).ok_or_else(|| {
                LodError::InvalidUri(format!("CID not found: {}", current_cid.to_string()))
            })?;

            // Navigate based on segment
            match &node.data {
                IpldData::Triple { object, .. } => {
                    if let IpldValue::Link { cid } = object {
                        current_cid = cid.clone();
                    } else {
                        return Ok(None);
                    }
                }
                IpldData::NamedGraph { graph_cid, .. } if segment == "graph" => {
                    current_cid = graph_cid.clone();
                }
                _ => return Ok(None),
            }
        }

        Ok(store.get(&current_cid).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cid_creation() {
        let cid = Cid::new("QmTest123", "dag-json");
        assert_eq!(cid.hash, "QmTest123");
        assert_eq!(cid.codec, "dag-json");
    }

    #[test]
    fn test_cid_from_bytes() {
        let content = b"hello world";
        let cid = Cid::from_bytes(content, "dag-json");
        assert_eq!(cid.codec, "dag-json");
        assert!(!cid.hash.is_empty());

        // Same content should produce same CID
        let cid2 = Cid::from_bytes(content, "dag-json");
        assert_eq!(cid.hash, cid2.hash);
    }

    #[test]
    fn test_ipld_store_basic() {
        let mut store = IpldStore::new();
        assert!(store.is_empty());

        let data = IpldData::Triple {
            subject: "s".to_string(),
            predicate: "p".to_string(),
            object: IpldValue::Literal {
                value: "o".to_string(),
                lang: None,
            },
        };

        let cid = store.put(data).unwrap();
        assert_eq!(store.len(), 1);
        assert!(store.get(&cid).is_some());
    }

    #[test]
    fn test_put_triple() {
        let mut store = IpldStore::new();

        let triple = Triple {
            subject: "ex:subject".to_string(),
            predicate: "ex:predicate".to_string(),
            object: RdfValue::string("object"),
        };

        let cid = store.put_triple(&triple).unwrap();
        assert!(store.get(&cid).is_some());
    }

    #[test]
    fn test_put_get_graph() {
        let mut store = IpldStore::new();

        let triples = vec![
            Triple {
                subject: "ex:s1".to_string(),
                predicate: "ex:p1".to_string(),
                object: RdfValue::string("o1"),
            },
            Triple {
                subject: "ex:s2".to_string(),
                predicate: "ex:p2".to_string(),
                object: RdfValue::integer(42),
            },
        ];

        let graph_cid = store.put_graph(&triples).unwrap();
        let retrieved = store.get_graph(&graph_cid).unwrap();

        assert_eq!(retrieved.len(), 2);
        assert_eq!(retrieved[0].subject, "ex:s1");
        assert_eq!(retrieved[1].subject, "ex:s2");
    }

    #[test]
    fn test_ipld_value_conversion() {
        let uri_value = RdfValue::Uri("http://example.org".to_string());
        let ipld = IpldValue::from(&uri_value);

        match ipld {
            IpldValue::Uri { uri } => assert_eq!(uri, "http://example.org"),
            _ => panic!("Wrong conversion"),
        }
    }

    #[test]
    fn test_export_import_json() {
        let mut store = IpldStore::new();

        let triple = Triple {
            subject: "ex:subject".to_string(),
            predicate: "ex:predicate".to_string(),
            object: RdfValue::string("object"),
        };

        store.put_triple(&triple).unwrap();

        let json = store.export_json().unwrap();
        let imported = IpldStore::import_json(&json).unwrap();

        assert_eq!(store.len(), imported.len());
    }

    #[test]
    fn test_list_cids() {
        let mut store = IpldStore::new();

        let triple1 = Triple {
            subject: "ex:s1".to_string(),
            predicate: "ex:p1".to_string(),
            object: RdfValue::string("o1"),
        };

        let triple2 = Triple {
            subject: "ex:s2".to_string(),
            predicate: "ex:p2".to_string(),
            object: RdfValue::string("o2"),
        };

        store.put_triple(&triple1).unwrap();
        store.put_triple(&triple2).unwrap();

        let cids = store.list_cids();
        assert_eq!(cids.len(), 2);
    }

    #[test]
    fn test_ipld_path() {
        let path = IpldPath::new("/graph/triples");
        assert_eq!(path.segments.len(), 2);
        assert_eq!(path.segments[0], "graph");
        assert_eq!(path.segments[1], "triples");
    }

    #[test]
    fn test_named_graph() {
        let mut store = IpldStore::new();

        let triples = vec![Triple {
            subject: "ex:s".to_string(),
            predicate: "ex:p".to_string(),
            object: RdfValue::string("o"),
        }];

        let graph_cid = store.put_graph(&triples).unwrap();

        let mut metadata = HashMap::new();
        metadata.insert("title".to_string(), "Test Graph".to_string());

        let named_graph = IpldData::NamedGraph {
            name: "test-graph".to_string(),
            graph_cid,
            metadata,
        };

        let cid = store.put(named_graph).unwrap();
        let node = store.get(&cid).unwrap();

        match &node.data {
            IpldData::NamedGraph { name, .. } => assert_eq!(name, "test-graph"),
            _ => panic!("Wrong node type"),
        }
    }
}
