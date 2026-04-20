#![allow(clippy::type_complexity)]
#![allow(clippy::inherent_to_string)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::field_reassign_with_default)]

//! Legalis-LOD: Linked Open Data (RDF/TTL) export for Legalis-RS.
//!
//! This crate provides comprehensive RDF export and Linked Data functionality for legal statutes.
//!
//! ## RDF Formats Supported
//! - Turtle (TTL) - Human-readable RDF format
//! - N-Triples (NT) - Line-based RDF format
//! - RDF/XML - XML serialization of RDF
//! - JSON-LD - JSON-based RDF format
//! - TriG - Named graph format
//!
//! ## Ontologies and Vocabularies
//! - ELI (European Legislation Identifier)
//! - FaBiO (FRBR-aligned Bibliographic Ontology)
//! - LKIF-Core (Legal Knowledge Interchange Format)
//! - LegalRuleML (OASIS standard for legal rules)
//! - Akoma Ntoso (Legal document markup)
//! - FRBR (Functional Requirements for Bibliographic Records)
//! - Dublin Core (dc, dcterms)
//! - SKOS (Simple Knowledge Organization System)
//! - VOID (Vocabulary of Interlinked Datasets)
//! - PROV-O (Provenance Ontology)
//! - Custom Legalis ontology
//!
//! ## Linked Data Features
//! - Cool URIs for legal resources (ELI-style, hierarchical, flat patterns)
//! - URI dereferencing with content negotiation
//! - owl:sameAs linking for entity resolution
//! - rdfs:seeAlso for related resources
//! - Integration with EUR-Lex, legislation.gov.uk, Wikidata, DBpedia
//!
//! ## Additional Features
//! - SHACL and ShEx validation
//! - SPARQL query generation
//! - Streaming serialization for large datasets
//! - Export caching
//! - RDFa output for HTML embedding
//! - Provenance tracking (PROV-O)
//! - License metadata (Creative Commons, etc.)
//!
//! See EXAMPLES.md for detailed usage examples.

mod functions;
mod trait_impls;
mod types;

pub use types::*;

pub mod audit_log;
pub mod av_annotation;
pub mod bitemporal;
pub mod blockchain;
pub mod cache;
pub mod competency_questions;
pub mod compliance;
pub mod content_addressed;
pub mod continuous_query;
pub mod crossmodal_reasoning;
pub mod crowdsourced_evolution;
pub mod dcat;
pub mod did;
pub mod document_layout;
pub mod embeddings;
pub mod enterprise_deployment;
pub mod entity_linking;
pub mod external;
pub mod fusion;
pub mod geosparql;
pub mod governance;
pub mod image_rdf;
pub mod ipld;
pub mod jurisdiction_queries;
pub mod kg_completion;
pub mod knowledge_graph;
pub mod ldn;
pub mod link_prediction;
pub mod linked_data;
pub mod map_exploration;
pub mod multimodal_alignment;
pub mod neural_symbolic;
pub mod ontology;
pub mod ontology_alignment;
pub mod ontology_learning;
pub mod ontology_metrics;
pub mod ontology_versioning;
pub mod quality;
pub mod rbac;
pub mod rdfa;
pub mod rdfstar;
pub mod realtime;
pub mod reasoning;
pub mod relation_extraction;
pub mod shacl;
pub mod shex;
pub mod similarity;
pub mod sparql;
pub mod sparqlstar;
pub mod spatial_reasoning;
pub mod store;
pub mod streaming;
pub mod streaming_sparql;
pub mod temporal_consistency;
pub mod temporal_rdf;
pub mod time_aware_queries;
pub mod validation;
pub mod verifiable;
pub mod versioning;
pub mod void_desc;
