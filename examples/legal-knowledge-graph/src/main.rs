//! Legal Knowledge Graph Example
//!
//! This example demonstrates how to use `legalis-lod` to export legal statutes
//! to Linked Open Data formats for semantic web integration.
//!
//! ## Supported Formats
//!
//! - **Turtle (TTL)**: Human-readable RDF syntax
//! - **JSON-LD**: JSON-based linked data
//! - **N-Triples**: Line-based RDF
//! - **RDF/XML**: XML serialization
//! - **TriG**: Turtle with named graphs
//!
//! ## Ontologies Used
//!
//! - ELI (European Legislation Identifier)
//! - FaBiO (FRBR-aligned Bibliographic Ontology)
//! - LKIF-Core (Legal Knowledge Interchange Format)
//! - PROV-O (Provenance Ontology)

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
use legalis_lod::sparql::{SparqlQueryBuilder, SparqlTemplates};
use legalis_lod::{LicenseInfo, LodExporter, ProvenanceInfo, RdfFormat};

fn create_test_statutes() -> Vec<Statute> {
    vec![
        Statute::new(
            "voting-rights-act",
            "Voting Rights Act 2024",
            Effect::new(EffectType::Grant, "Right to vote in national elections"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_jurisdiction("INTL"),
        Statute::new(
            "data-protection-act",
            "Data Protection Act 2024",
            Effect::new(EffectType::Obligation, "Must protect personal data"),
        )
        .with_precondition(Condition::HasAttribute {
            key: "data_controller".to_string(),
        })
        .with_jurisdiction("EU"),
        Statute::new(
            "senior-benefits",
            "Senior Citizens Benefits Act",
            Effect::new(EffectType::Grant, "Eligible for senior benefits"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 65,
        })
        .with_jurisdiction("JP"),
    ]
}

fn print_output_preview(output: &str, title: &str, max_lines: usize) {
    println!("   {}", "-".repeat(70));
    println!("   {}", title);
    println!("   {}", "-".repeat(70));
    let lines: Vec<&str> = output.lines().collect();
    let display_lines = lines.len().min(max_lines);
    for line in lines.iter().take(display_lines) {
        println!("   {}", line);
    }
    if lines.len() > max_lines {
        println!("   ... ({} more lines)", lines.len() - max_lines);
    }
    println!();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(80));
    println!("   LEGAL KNOWLEDGE GRAPH - Legalis-LOD Demo");
    println!("   法令をLinked Open Dataとして出力");
    println!("{}", "=".repeat(80));
    println!();

    let statutes = create_test_statutes();
    println!("Step 1: Created {} test statutes\n", statutes.len());
    for statute in &statutes {
        println!(
            "   - {} ({}) [{}]",
            statute.title,
            statute.id,
            statute.jurisdiction.as_deref().unwrap_or("N/A")
        );
    }
    println!();

    // =========================================================================
    // Turtle Format
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 2: Export to Turtle (TTL) Format\n");

    let turtle_exporter = LodExporter::new(RdfFormat::Turtle);
    let turtle_output = turtle_exporter.export(&statutes[0])?;
    print_output_preview(&turtle_output, "Turtle Output (Single Statute)", 20);

    // =========================================================================
    // JSON-LD Format
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 3: Export to JSON-LD Format\n");

    let jsonld_exporter = LodExporter::new(RdfFormat::JsonLd);
    let jsonld_output = jsonld_exporter.export(&statutes[0])?;
    print_output_preview(&jsonld_output, "JSON-LD Output", 25);

    // =========================================================================
    // With Provenance and License
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 4: Export with Provenance (PROV-O) and License\n");

    let provenance = ProvenanceInfo::new()
        .with_agent("https://example.org/agent/legalis-system")
        .with_activity("https://example.org/activity/statute-digitization")
        .with_source("https://example.org/source/official-gazette")
        .with_attribution("Legal Digitization Team");

    let license = LicenseInfo::cc_by_4_0().with_rights_holder("Government of Example Country");

    let prov_exporter = LodExporter::new(RdfFormat::Turtle)
        .with_provenance(provenance)
        .with_license(license);

    let prov_output = prov_exporter.export(&statutes[0])?;
    print_output_preview(&prov_output, "Turtle with PROV-O & CC-BY-4.0", 30);

    // =========================================================================
    // Batch Export to TriG (Named Graphs)
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 5: Batch Export to TriG (Named Graphs)\n");

    let trig_exporter = LodExporter::new(RdfFormat::TriG);
    let trig_output = trig_exporter.export_batch(&statutes)?;
    print_output_preview(&trig_output, "TriG Output (Multiple Statutes)", 35);

    // =========================================================================
    // SPARQL Query Generation
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 6: SPARQL Query Templates\n");

    // Find all statutes
    let all_statutes_query = SparqlTemplates::find_all_statutes();
    print_output_preview(&all_statutes_query, "Query: Find All Statutes", 15);

    // Find by jurisdiction
    let jp_statutes_query = SparqlTemplates::find_by_jurisdiction("JP");
    print_output_preview(&jp_statutes_query, "Query: Find JP Jurisdiction", 15);

    // Find statutes with age conditions
    let age_query = SparqlTemplates::find_with_age_condition();
    print_output_preview(&age_query, "Query: Find Statutes with Age Conditions", 15);

    // Custom query builder
    let mut builder = SparqlQueryBuilder::new();
    builder
        .select("?statute")
        .select("?title")
        .select("?effectType")
        .where_pattern("?statute rdf:type eli:LegalResource .")
        .where_pattern("?statute eli:title ?title .")
        .where_pattern("?statute legalis:effectType ?effectType .")
        .filter("?effectType = 'Grant'")
        .limit(100);
    let custom_query = builder.build();
    print_output_preview(&custom_query, "Custom Query: Grant Effects Only", 15);

    // =========================================================================
    // Summary
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("   LINKED OPEN DATA SUMMARY");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Supported RDF Formats:");
    println!("   | Format     | Extension | Use Case                           |");
    println!("   |------------|-----------|-------------------------------------|");
    println!("   | Turtle     | .ttl      | Human-readable, widely supported    |");
    println!("   | JSON-LD    | .jsonld   | Web APIs, JavaScript integration    |");
    println!("   | N-Triples  | .nt       | Streaming, large datasets           |");
    println!("   | RDF/XML    | .rdf      | Legacy systems, XML pipelines       |");
    println!("   | TriG       | .trig     | Named graphs, versioned data        |");
    println!();
    println!("   Ontologies:");
    println!("   - ELI: European Legislation Identifier (legal resources)");
    println!("   - PROV-O: W3C Provenance (data lineage)");
    println!("   - SKOS: Simple Knowledge Organization System");
    println!("   - Dublin Core: General metadata");
    println!();
    println!("   Integration Points:");
    println!("   - SPARQL Endpoints for federated queries");
    println!("   - EUR-Lex, UK Legislation, Wikidata linking");
    println!("   - Knowledge graph embeddings for ML");
    println!();

    Ok(())
}
