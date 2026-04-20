//! Tests for legalis-lod core types and functions.

#[cfg(test)]
mod tests {
    use crate::types::{escape_string, escape_uri};
    use crate::{LicenseInfo, LodExporter, Namespaces, ProvenanceInfo, RdfFormat, RdfValue};
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};
    fn sample_statute() -> Statute {
        Statute::new(
            "adult-rights",
            "Adult Rights Act",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }
    #[test]
    fn test_export_turtle() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();
        assert!(output.contains("@prefix eli:"));
        assert!(output.contains("@prefix legalis:"));
        assert!(output.contains("eli:LegalResource"));
        assert!(output.contains("Adult Rights Act"));
    }
    #[test]
    fn test_export_ntriples() {
        let exporter = LodExporter::new(RdfFormat::NTriples);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();
        assert!(output.contains("<http://data.europa.eu/eli/ontology#LegalResource>"));
        assert!(output.contains("Adult Rights Act"));
    }
    #[test]
    fn test_export_rdf_xml() {
        let exporter = LodExporter::new(RdfFormat::RdfXml);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();
        assert!(output.contains("<?xml version"));
        assert!(output.contains("rdf:RDF"));
        assert!(output.contains("Adult Rights Act"));
    }
    #[test]
    fn test_export_json_ld() {
        let exporter = LodExporter::new(RdfFormat::JsonLd);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();
        assert!(output.contains("\"@context\""));
        assert!(output.contains("\"@id\""));
        assert!(output.contains("Adult Rights Act"));
    }
    #[test]
    fn test_condition_triples() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let statute = Statute::new(
            "complex-law",
            "Complex Law",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        ));
        let output = exporter.export(&statute).unwrap();
        assert!(output.contains("legalis:AndCondition"));
        assert!(output.contains("legalis:AgeCondition"));
        assert!(output.contains("legalis:IncomeCondition"));
    }
    #[test]
    fn test_custom_namespace() {
        let mut namespaces = Namespaces::with_base("https://law.example.jp/");
        namespaces.add("jplaw", "https://law.example.jp/ontology#");
        let exporter = LodExporter::with_namespaces(RdfFormat::Turtle, namespaces);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();
        assert!(output.contains("https://law.example.jp/statute/adult-rights"));
    }
    #[test]
    fn test_format_extensions() {
        assert_eq!(RdfFormat::Turtle.extension(), "ttl");
        assert_eq!(RdfFormat::NTriples.extension(), "nt");
        assert_eq!(RdfFormat::RdfXml.extension(), "rdf");
        assert_eq!(RdfFormat::JsonLd.extension(), "jsonld");
    }
    #[test]
    fn test_escape_uri() {
        assert_eq!(escape_uri("hello world"), "hello_world");
        assert_eq!(escape_uri("a/b"), "a-b");
    }
    #[test]
    fn test_escape_string() {
        assert_eq!(escape_string("hello\nworld"), "hello\\nworld");
        assert_eq!(escape_string("say \"hi\""), "say \\\"hi\\\"");
    }
    #[test]
    fn test_batch_export() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let statutes = vec![
            sample_statute(),
            Statute::new(
                "minor-protection",
                "Minor Protection Act",
                Effect::new(EffectType::Grant, "Protection rights"),
            ),
        ];
        let output = exporter.export_batch(&statutes).unwrap();
        assert!(output.contains("adult-rights"));
        assert!(output.contains("minor-protection"));
    }
    #[test]
    fn test_export_trig() {
        let exporter = LodExporter::new(RdfFormat::TriG);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();
        assert!(output.contains("@prefix eli:"));
        assert!(output.contains("@prefix legalis:"));
        assert!(output.contains("graph/adult-rights"));
        assert!(output.contains("{"));
        assert!(output.contains("}"));
        assert!(output.contains("Adult Rights Act"));
    }
    #[test]
    fn test_export_trig_batch() {
        let exporter = LodExporter::new(RdfFormat::TriG);
        let statutes = vec![
            sample_statute(),
            Statute::new(
                "test-law",
                "Test Law",
                Effect::new(EffectType::Grant, "Test rights"),
            ),
        ];
        let output = exporter.export_batch(&statutes).unwrap();
        assert!(output.contains("graph/adult-rights"));
        assert!(output.contains("graph/test-law"));
        assert!(output.contains("Adult Rights Act"));
        assert!(output.contains("Test Law"));
    }
    #[test]
    fn test_trig_extension() {
        assert_eq!(RdfFormat::TriG.extension(), "trig");
        assert_eq!(RdfFormat::TriG.mime_type(), "application/trig");
    }
    #[test]
    fn test_skos_concept_scheme() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let triples = exporter.generate_concept_scheme("effect-types", "Legal Effect Types");
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(& t.object,
            RdfValue::Uri(u) if u == "skos:ConceptScheme")));
        assert!(triples.iter().any(|t| t.predicate == "skos:prefLabel"));
    }
    #[test]
    fn test_skos_effect_concept() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let triples = exporter.create_effect_type_concept(
            "grant",
            "Grant Effect",
            Some("An effect that grants rights or permissions"),
        );
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(& t.object,
            RdfValue::Uri(u) if u == "skos:Concept")));
        assert!(triples.iter().any(|t| t.predicate == "skos:prefLabel"));
        assert!(triples.iter().any(|t| t.predicate == "skos:definition"));
        assert!(triples.iter().any(|t| t.predicate == "skos:inScheme"));
    }
    #[test]
    fn test_skos_hierarchy() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let triples = exporter.add_skos_hierarchy("legal-effect", "grant-effect");
        assert!(triples.iter().any(|t| t.predicate == "skos:broader"));
        assert!(triples.iter().any(|t| t.predicate == "skos:narrower"));
        assert_eq!(triples.len(), 2);
    }
    #[test]
    fn test_content_negotiation() {
        assert_eq!(
            RdfFormat::from_accept_header("application/ld+json"),
            RdfFormat::JsonLd
        );
        assert_eq!(
            RdfFormat::from_accept_header("text/turtle"),
            RdfFormat::Turtle
        );
        assert_eq!(
            RdfFormat::from_accept_header("application/rdf+xml"),
            RdfFormat::RdfXml
        );
        assert_eq!(
            RdfFormat::from_accept_header("application/n-triples"),
            RdfFormat::NTriples
        );
        assert_eq!(
            RdfFormat::from_accept_header("application/trig"),
            RdfFormat::TriG
        );
        assert_eq!(
            RdfFormat::from_accept_header("text/html"),
            RdfFormat::Turtle
        );
    }
    #[test]
    fn test_mime_type_aliases() {
        let turtle_aliases = RdfFormat::Turtle.mime_type_aliases();
        assert!(turtle_aliases.contains(&"text/turtle"));
        assert!(turtle_aliases.contains(&"application/x-turtle"));
    }
    #[test]
    fn test_provenance_info() {
        let prov = ProvenanceInfo::new()
            .with_agent("https://example.org/agent/legalis")
            .with_activity("https://example.org/activity/export")
            .with_source("https://example.org/original")
            .with_attribution("Legalis Project");
        assert!(prov.agent.is_some());
        assert!(prov.activity.is_some());
        assert!(prov.derived_from.is_some());
        assert!(prov.attribution.is_some());
    }
    #[test]
    fn test_license_info() {
        let license = LicenseInfo::cc_by_4_0().with_rights_holder("Example Organization");
        assert!(license.license_uri.contains("creativecommons.org"));
        assert!(license.label.is_some());
        assert_eq!(
            license.rights_holder,
            Some("Example Organization".to_string())
        );
    }
    #[test]
    fn test_export_with_provenance() {
        let prov = ProvenanceInfo::new()
            .with_agent("https://example.org/agent/legalis")
            .with_attribution("Legalis Team");
        let exporter = LodExporter::new(RdfFormat::Turtle).with_provenance(prov);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();
        assert!(output.contains("prov:wasAttributedTo"));
        assert!(output.contains("dcterms:creator"));
        assert!(output.contains("prov:generatedAtTime"));
    }
    #[test]
    fn test_export_with_license() {
        let license = LicenseInfo::cc_by_4_0();
        let exporter = LodExporter::new(RdfFormat::Turtle).with_license(license);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();
        assert!(output.contains("dcterms:license"));
        assert!(output.contains("cc:license"));
        assert!(output.contains("creativecommons.org"));
    }
    #[test]
    fn test_all_formats() {
        let formats = RdfFormat::all_formats();
        assert_eq!(formats.len(), 5);
        assert!(formats.contains(&RdfFormat::Turtle));
        assert!(formats.contains(&RdfFormat::JsonLd));
        assert!(formats.contains(&RdfFormat::TriG));
    }
    #[test]
    fn test_round_trip_basic_statute() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let statute = sample_statute();
        let triples = exporter.statute_to_triples(&statute).unwrap();
        assert!(triples.iter().any(|t| t.predicate == "eli:title"));
        assert!(triples.iter().any(|t| t.predicate == "dcterms:identifier"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:hasEffect"));
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:hasPrecondition")
        );
    }
    #[test]
    fn test_round_trip_with_metadata() {
        let prov = ProvenanceInfo::new()
            .with_agent("https://example.org/agent/test")
            .with_attribution("Test Team");
        let license = LicenseInfo::cc_by_4_0();
        let exporter = LodExporter::new(RdfFormat::Turtle)
            .with_provenance(prov)
            .with_license(license);
        let statute = sample_statute();
        let triples = exporter.statute_to_triples(&statute).unwrap();
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "prov:wasAttributedTo")
        );
        assert!(triples.iter().any(|t| t.predicate == "dcterms:creator"));
        assert!(triples.iter().any(|t| t.predicate == "dcterms:license"));
    }
    #[test]
    fn test_round_trip_complex_conditions() {
        let statute = Statute::new(
            "complex-law",
            "Complex Law",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        ));
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let triples = exporter.statute_to_triples(&statute).unwrap();
        assert!(
            triples
                .iter()
                .any(|t| matches!(& t.object, RdfValue::Uri(u) if u ==
            "legalis:AndCondition"))
        );
        assert!(
            triples
                .iter()
                .any(|t| matches!(& t.object, RdfValue::Uri(u) if u ==
            "legalis:AgeCondition"))
        );
        assert!(
            triples
                .iter()
                .any(|t| matches!(& t.object, RdfValue::Uri(u) if u ==
            "legalis:IncomeCondition"))
        );
        assert!(triples.iter().any(|t| t.predicate == "legalis:leftOperand"));
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:rightOperand")
        );
    }
    #[test]
    fn test_round_trip_validation_consistency() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let statute = sample_statute();
        let report = exporter.validate_statute(&statute).unwrap();
        assert!(report.triple_count > 0);
        assert!(report.subject_count > 0);
    }
    #[test]
    fn test_round_trip_all_formats_consistency() {
        let statute = sample_statute();
        for format in RdfFormat::all_formats() {
            let exporter = LodExporter::new(format);
            let output = exporter.export(&statute);
            assert!(output.is_ok(), "Failed to export to {:?}", format);
            let output = output.unwrap();
            assert!(!output.is_empty(), "{:?} produced empty output", format);
            assert!(
                output.contains("Adult Rights Act"),
                "{:?} missing title",
                format
            );
        }
    }
    #[test]
    fn test_round_trip_batch_consistency() {
        let statutes = vec![
            sample_statute(),
            Statute::new(
                "test-law",
                "Test Law",
                Effect::new(EffectType::Grant, "Test rights"),
            ),
        ];
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let batch_output = exporter.export_batch(&statutes).unwrap();
        assert!(batch_output.contains("adult-rights"));
        assert!(batch_output.contains("test-law"));
        assert!(batch_output.contains("Adult Rights Act"));
        assert!(batch_output.contains("Test Law"));
    }
    #[test]
    fn test_round_trip_special_characters() {
        let statute = Statute::new(
            "special-chars",
            "Law with \"quotes\" and <tags> & symbols",
            Effect::new(EffectType::Grant, "Special\ncharacters\ttab"),
        );
        let exporter_turtle = LodExporter::new(RdfFormat::Turtle);
        let turtle_output = exporter_turtle.export(&statute).unwrap();
        assert!(turtle_output.contains("\\\"quotes\\\""));
        assert!(turtle_output.contains("\\n"));
        let exporter_xml = LodExporter::new(RdfFormat::RdfXml);
        let xml_output = exporter_xml.export(&statute).unwrap();
        assert!(xml_output.contains("&lt;tags&gt;") || xml_output.contains("&quot;quotes&quot;"));
    }
    #[test]
    fn test_benchmark_single_statute_export() {
        let statute = sample_statute();
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let _ = exporter.export(&statute);
        let start = std::time::Instant::now();
        for _ in 0..100 {
            let _ = exporter.export(&statute);
        }
        let duration = start.elapsed();
        println!("Single statute export (100 iterations): {:?}", duration);
        println!("Average: {:?}", duration / 100);
        assert!(
            duration.as_millis() < 10000,
            "Export too slow: {:?}",
            duration
        );
    }
    #[test]
    fn test_benchmark_batch_export() {
        let statutes: Vec<Statute> = (0..100)
            .map(|i| {
                Statute::new(
                    format!("statute-{}", i),
                    format!("Statute Number {}", i),
                    Effect::new(EffectType::Grant, format!("Effect {}", i)),
                )
                .with_precondition(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18 + (i % 10),
                })
            })
            .collect();
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let start = std::time::Instant::now();
        let output = exporter.export_batch(&statutes).unwrap();
        let duration = start.elapsed();
        println!("Batch export (100 statutes): {:?}", duration);
        println!("Per statute: {:?}", duration / 100);
        assert!(!output.is_empty());
        assert!(
            duration.as_millis() < 10000,
            "Batch export too slow: {:?}",
            duration
        );
    }
    #[test]
    fn test_benchmark_all_formats() {
        let statute = sample_statute();
        for format in RdfFormat::all_formats() {
            let exporter = LodExporter::new(format);
            let start = std::time::Instant::now();
            for _ in 0..50 {
                let _ = exporter.export(&statute);
            }
            let duration = start.elapsed();
            println!("{:?} format (50 iterations): {:?}", format, duration);
            println!("Average: {:?}", duration / 50);
            assert!(
                duration.as_millis() < 5000,
                "{:?} too slow: {:?}",
                format,
                duration
            );
        }
    }
    #[test]
    fn test_benchmark_validation() {
        let statute = sample_statute();
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let start = std::time::Instant::now();
        for _ in 0..100 {
            let _ = exporter.validate_statute(&statute);
        }
        let duration = start.elapsed();
        println!("Validation (100 iterations): {:?}", duration);
        println!("Average: {:?}", duration / 100);
        assert!(
            duration.as_millis() < 5000,
            "Validation too slow: {:?}",
            duration
        );
    }
    #[test]
    fn test_export_with_ontologies() {
        let exporter = LodExporter::new(RdfFormat::Turtle).with_ontologies(true);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();
        assert!(output.contains("fabio:"));
        assert!(output.contains("lkif:"));
        assert!(output.contains("lrml:"));
        assert!(output.contains("akn:"));
    }
    #[test]
    fn test_benchmark_streaming_export() {
        use std::io::Cursor;
        let statutes: Vec<Statute> = (0..50)
            .map(|i| {
                Statute::new(
                    format!("statute-{}", i),
                    format!("Statute {}", i),
                    Effect::new(EffectType::Grant, format!("Effect {}", i)),
                )
            })
            .collect();
        let mut buffer = Cursor::new(Vec::new());
        let ns = Namespaces::default();
        let start = std::time::Instant::now();
        {
            let mut serializer =
                crate::streaming::StreamingSerializer::new(&mut buffer, RdfFormat::Turtle, ns);
            serializer.write_header().unwrap();
            for statute in &statutes {
                let exporter = LodExporter::new(RdfFormat::Turtle);
                let triples = exporter.statute_to_triples(statute).unwrap();
                serializer.write_triples(&triples).unwrap();
            }
            serializer.finalize().unwrap();
        }
        let duration = start.elapsed();
        println!("Streaming export (50 statutes): {:?}", duration);
        println!("Per statute: {:?}", duration / 50);
        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(!output.is_empty());
        assert!(
            duration.as_millis() < 5000,
            "Streaming export too slow: {:?}",
            duration
        );
    }
}
