# Legalis-LOD Examples

This document provides examples of using the Legalis-LOD library features.

## Basic RDF Export

```rust
use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
use legalis_lod::{LodExporter, RdfFormat};

// Create a statute
let statute = Statute::new(
    "adult-rights-act",
    "Adult Rights Act 2023",
    Effect::new(EffectType::Grant, "Full legal capacity"),
)
.with_precondition(Condition::Age {
    operator: ComparisonOp::GreaterOrEqual,
    value: 18,
});

// Export to Turtle format
let exporter = LodExporter::new(RdfFormat::Turtle);
let ttl_output = exporter.export(&statute).unwrap();
println!("{}", ttl_output);
```

## Using Multiple Ontologies

```rust
use legalis_lod::{LodExporter, RdfFormat};

// Enable all ontology support (FaBiO, LKIF, LegalRuleML, Akoma Ntoso)
let exporter = LodExporter::new(RdfFormat::Turtle)
    .with_ontologies(true);

let output = exporter.export(&statute).unwrap();
// Output now includes triples from all supported ontologies
```

## Provenance and Licensing

```rust
use legalis_lod::{LodExporter, RdfFormat, ProvenanceInfo, LicenseInfo};

let prov = ProvenanceInfo::new()
    .with_agent("https://example.org/agent/legalis-system")
    .with_activity("https://example.org/activity/statute-import")
    .with_attribution("Legal Research Team");

let license = LicenseInfo::cc_by_4_0()
    .with_rights_holder("Government of Example");

let exporter = LodExporter::new(RdfFormat::Turtle)
    .with_provenance(prov)
    .with_license(license);

let output = exporter.export(&statute).unwrap();
```

## Cool URIs

```rust
use legalis_lod::linked_data::{CoolUriScheme, UriPattern};

// Create a Cool URI scheme (ELI-style)
let scheme = CoolUriScheme::new("legislation.example.org")
    .with_pattern(UriPattern::EliStyle);

// Generate URI for a statute
let uri = scheme.generate_uri(&statute);
// https://legislation.example.org/id/uk/statute/2023/adult-rights-act

// Generate version-specific URI
let version_uri = scheme.generate_version_uri(&statute);
// https://legislation.example.org/id/uk/statute/2023/adult-rights-act/version/1

// Generate format-specific URI
let format_uri = scheme.generate_format_uri(&statute, RdfFormat::JsonLd);
// https://legislation.example.org/id/uk/statute/2023/adult-rights-act.jsonld
```

## URI Dereferencing

```rust
use legalis_lod::linked_data::{CoolUriScheme, UriDereferencer};

let scheme = CoolUriScheme::new("legislation.example.org");
let mut dereferencer = UriDereferencer::new(scheme.clone());

// Register statutes
dereferencer.register(statute);

// Dereference URI
let uri = scheme.generate_uri(&statute);
let retrieved = dereferencer.dereference(&uri).unwrap();

// Content negotiation
let (format, statute) = dereferencer
    .resolve(&uri, "application/ld+json")
    .unwrap();
// format will be RdfFormat::JsonLd
```

## Entity Resolution with owl:sameAs

```rust
use legalis_lod::linked_data::EntityResolver;

let mut resolver = EntityResolver::new("https://example.org/");

// Link to EUR-Lex
resolver.link_to_eurlex("adult-rights-act", "32023R1234");

// Link to UK legislation
resolver.link_to_uk_legislation("adult-rights-act", 2023, 42);

// Link to Wikidata
resolver.link_to_wikidata("adult-rights-act", "Q12345678");

// Link to DBpedia
resolver.link_to_dbpedia("adult-rights-act", "Adult_Rights_Act_2023");

// Generate linking triples
let triples = resolver.generate_linking_triples("adult-rights-act");
```

## SKOS Concept Schemes

```rust
use legalis_lod::LodExporter;

let exporter = LodExporter::new(RdfFormat::Turtle);

// Create a concept scheme for legal effect types
let scheme_triples = exporter.generate_concept_scheme(
    "effect-types",
    "Legal Effect Types"
);

// Create individual concepts
let grant_concept = exporter.create_effect_type_concept(
    "grant",
    "Grant Effect",
    Some("An effect that grants rights or permissions to an entity")
);

// Create hierarchical relationships
let hierarchy = exporter.add_skos_hierarchy(
    "legal-effect",
    "grant-effect"
);
```

## Streaming Large Datasets

```rust
use legalis_lod::{RdfFormat, Namespaces};
use legalis_lod::streaming::StreamingSerializer;
use std::fs::File;

let file = File::create("output.ttl").unwrap();
let namespaces = Namespaces::default();

let mut serializer = StreamingSerializer::new(
    file,
    RdfFormat::Turtle,
    namespaces
);

serializer.write_header().unwrap();

for statute in large_statute_collection {
    let exporter = LodExporter::new(RdfFormat::Turtle);
    let triples = exporter.statute_to_triples(&statute).unwrap();
    serializer.write_triples(&triples).unwrap();
}

serializer.finalize().unwrap();
```

## SHACL Validation

```rust
use legalis_lod::shacl::ShaclGenerator;

let generator = ShaclGenerator::new("https://example.org/shapes/");

// Generate SHACL shapes for statute validation
let shapes = generator.generate_statute_shapes();

// Export to Turtle
let shapes_ttl = generator.to_turtle(&shapes).unwrap();
```

## SPARQL Query Generation

```rust
use legalis_lod::sparql::{SparqlQueryBuilder, SparqlTemplates};

// Basic query building
let mut builder = SparqlQueryBuilder::new();
builder
    .select("?statute")
    .select("?title")
    .where_pattern("?statute rdf:type eli:LegalResource .")
    .where_pattern("?statute eli:title ?title .")
    .where_pattern("?statute eli:date_document ?date .")
    .filter("YEAR(?date) = 2023")
    .limit(100);

let query = builder.build();
println!("{}", query);

// Using pre-built templates
let query = SparqlTemplates::find_all_statutes();
let query = SparqlTemplates::find_by_jurisdiction("UK");
let query = SparqlTemplates::find_by_effect_type("GrantEffect");
```

## Federated SPARQL Queries

```rust
use legalis_lod::sparql::FederatedQueryBuilder;

// Query local data and Wikidata in a single query
let mut builder = FederatedQueryBuilder::new();
builder
    .select("?statute")
    .select("?title")
    .select("?wikidataLabel")
    .where_pattern("?statute a legalis:Statute .")
    .where_pattern("?statute eli:title ?title .")
    .service_wikidata(vec![
        "?statute owl:sameAs ?wikidata .".to_string(),
        "?wikidata rdfs:label ?wikidataLabel .".to_string(),
        "FILTER(LANG(?wikidataLabel) = 'en')".to_string(),
    ])
    .limit(10);

let query = builder.build();

// Query EUR-Lex for additional data
builder.service_eurlex(vec![
    "?statute eli:cites ?cited .".to_string(),
]);

// Query DBpedia
builder.service_dbpedia(vec![
    "?statute rdfs:seeAlso ?dbpediaResource .".to_string(),
]);
```

## SPARQL 1.1 Update (Graph Store Protocol)

```rust
use legalis_lod::sparql::SparqlUpdate;

// Insert data into default graph
let mut update = SparqlUpdate::new();
update.insert_data(
    None,
    r#"
    <http://example.org/statute/123> a legalis:Statute .
    <http://example.org/statute/123> eli:title "New Statute" .
    "#,
);
let update_str = update.build();

// Insert data into named graph
update.insert_data(
    Some("https://example.org/graph/statutes-2023".to_string()),
    r#"<http://example.org/statute/456> a legalis:Statute ."#,
);

// Delete data
update.delete_data(
    None,
    r#"<http://example.org/statute/old> a legalis:Statute ."#,
);

// Delete where pattern matches
update.delete_where(r#"?s a legalis:Statute . ?s legalis:status "obsolete" ."#);

// Modify data (DELETE + INSERT)
update.modify(
    Some("https://example.org/graph/statutes".to_string()),
    "?s eli:title ?oldTitle .",
    "?s eli:title \"Updated Title\" .",
    "?s dcterms:identifier \"statute-123\" . ?s eli:title ?oldTitle .",
);

// Clear a graph
update.clear("https://example.org/graph/temp", true);

// Drop a graph
update.drop("https://example.org/graph/old", false);

// Create a new graph
update.create("https://example.org/graph/new-statutes", false);

// Load data from URL
update.load(
    "https://example.org/data/statutes.ttl",
    Some("https://example.org/graph/imported".to_string()),
    true,
);
```

## Named Graph Management

```rust
use legalis_lod::sparql::NamedGraphManager;

let mut manager = NamedGraphManager::new("https://legislation.example.org/");

// Register graphs
manager.register_graph("statutes-2023", "All statutes enacted in 2023")
    .description = Some("Comprehensive collection of 2023 legislation".to_string());

manager.register_graph("statutes-uk", "UK Legislation");
manager.register_graph("statutes-eu", "EU Legislation");

// Get graph URI
let uri = manager.graph_uri("statutes-2023");
println!("Graph URI: {:?}", uri);

// List all graphs
for graph in manager.list_graphs() {
    println!("Graph: {} ({})", graph.label, graph.graph_uri);
}

// Generate CREATE operation
if let Some(create_update) = manager.create_graph_update("statutes-2023") {
    println!("{}", create_update);
}

// Generate DROP operation
if let Some(drop_update) = manager.drop_graph_update("statutes-old", true) {
    println!("{}", drop_update);
}

// Query specific graph
if let Some(query) = manager.select_from_graph(
    "statutes-2023",
    vec!["?statute", "?title"],
    vec!["?statute a legalis:Statute .", "?statute eli:title ?title ."],
) {
    println!("{}", query);
}
```

## SPARQL Endpoint Framework

```rust
use legalis_lod::sparql::SparqlEndpoint;
use std::collections::HashMap;

// Create a simple in-memory endpoint
let mut data_store: HashMap<String, Vec<String>> = HashMap::new();

let endpoint = SparqlEndpoint::new()
    .with_query_executor(move |query| {
        // Validate query
        if !SparqlEndpoint::validate_query(query) {
            return Err("Invalid SPARQL query".to_string());
        }

        // Execute query logic here
        // This is a stub - real implementation would parse and execute
        Ok(r#"{"results": {"bindings": []}}"#.to_string())
    })
    .with_update_executor(|update| {
        // Validate update
        if !SparqlEndpoint::validate_update(update) {
            return Err("Invalid SPARQL update".to_string());
        }

        // Execute update logic here
        Ok(())
    });

// Execute a query
match endpoint.execute_query("SELECT * WHERE { ?s ?p ?o } LIMIT 10") {
    Ok(results) => println!("Results: {}", results),
    Err(e) => eprintln!("Error: {}", e),
}

// Execute an update
match endpoint.execute_update("INSERT DATA { <s> <p> <o> }") {
    Ok(_) => println!("Update successful"),
    Err(e) => eprintln!("Error: {}", e),
}

// Validate queries
assert!(SparqlEndpoint::validate_query("SELECT * WHERE { ?s ?p ?o }"));
assert!(SparqlEndpoint::validate_update("INSERT DATA { <s> <p> <o> }"));
```

## VOID Dataset Description

```rust
use legalis_lod::void_desc::{VoidDataset, VoidBuilder};

let mut dataset = VoidDataset::new(
    "https://example.org/dataset/statutes",
    "Legal Statutes Dataset"
);

dataset.description = Some("A comprehensive dataset of legal statutes".to_string());
dataset.publisher = Some("Example Legal Authority".to_string());
dataset.license = Some("http://creativecommons.org/licenses/by/4.0/".to_string());

// Add statistics
dataset.add_statistic("triples", 10000);
dataset.add_statistic("entities", 500);

// Add SPARQL endpoint
dataset.add_sparql_endpoint("https://example.org/sparql");

// Generate VOID description
let builder = VoidBuilder::new("https://example.org/");
let void_triples = builder.dataset_to_triples(&dataset);
```

## Content Negotiation

```rust
use legalis_lod::RdfFormat;

// Parse HTTP Accept header
let format = RdfFormat::from_accept_header("application/ld+json");
// Returns RdfFormat::JsonLd

let format = RdfFormat::from_accept_header("text/turtle");
// Returns RdfFormat::Turtle

// Get MIME type
let mime = RdfFormat::JsonLd.mime_type();
// Returns "application/ld+json"

// Get all MIME type aliases
let aliases = RdfFormat::Turtle.mime_type_aliases();
// Returns ["text/turtle", "application/x-turtle", "application/turtle"]
```

## Caching Exports

```rust
use legalis_lod::cache::ExportCache;
use std::time::Duration;

let mut cache = ExportCache::new();

// Set cache TTL
cache.set_ttl(Duration::from_secs(3600)); // 1 hour

// Export with caching
let key = "statute:adult-rights-act:turtle";
if let Some(cached) = cache.get(key) {
    println!("Using cached export");
    return cached;
}

let output = exporter.export(&statute).unwrap();
cache.insert(key.to_string(), output.clone());
```

## Validation Reports

```rust
use legalis_lod::validation::RdfValidator;

let validator = RdfValidator::new();

// Validate statute triples
let report = exporter.validate_statute(&statute).unwrap();

println!("Triples: {}", report.triple_count);
println!("Subjects: {}", report.subject_count);
println!("Issues: {}", report.issues.len());

// Check for specific issues
for issue in &report.issues {
    println!("- {}", issue.message);
}
```

## Batch Export

```rust
use legalis_lod::{LodExporter, RdfFormat};

let exporter = LodExporter::new(RdfFormat::TriG);

let statutes = vec![
    statute1,
    statute2,
    statute3,
];

// Export all statutes to TriG (each in its own named graph)
let output = exporter.export_batch(&statutes).unwrap();
```

## Custom Namespaces

```rust
use legalis_lod::{LodExporter, RdfFormat, Namespaces};

let mut namespaces = Namespaces::with_base("https://law.example.jp/");
namespaces.add("jplaw", "https://law.example.jp/ontology#");
namespaces.add("jgov", "https://gov.example.jp/");

let exporter = LodExporter::with_namespaces(RdfFormat::Turtle, namespaces);
let output = exporter.export(&statute).unwrap();
```
