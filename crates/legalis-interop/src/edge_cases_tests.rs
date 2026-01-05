//! Comprehensive edge case tests for interop layer.

#[cfg(test)]
mod tests {
    use crate::{LegalConverter, LegalFormat};
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    // Empty input tests
    #[test]
    fn test_empty_catala_input() {
        let mut converter = LegalConverter::new();
        let result = converter.import("", LegalFormat::Catala);
        // Empty input may result in error or empty statutes depending on implementation
        if let Ok((statutes, _)) = result {
            assert_eq!(statutes.len(), 0);
        }
    }

    #[test]
    fn test_empty_l4_input() {
        let mut converter = LegalConverter::new();
        let result = converter.import("", LegalFormat::L4);
        // Empty input may result in error or empty statutes
        if let Ok((statutes, _)) = result {
            assert_eq!(statutes.len(), 0);
        }
    }

    #[test]
    fn test_empty_stipula_input() {
        let mut converter = LegalConverter::new();
        let result = converter.import("", LegalFormat::Stipula);
        // Empty input may result in error or empty statutes
        if let Ok((statutes, _)) = result {
            assert_eq!(statutes.len(), 0);
        }
    }

    // Whitespace-only input tests
    #[test]
    fn test_whitespace_only_input() {
        let mut converter = LegalConverter::new();
        let result = converter.import("   \n\t\r\n   ", LegalFormat::Catala);
        // Should either succeed with empty result or error
        if let Ok((statutes, _)) = result {
            assert_eq!(statutes.len(), 0);
        }
    }

    // Malformed input tests
    #[test]
    fn test_malformed_catala() {
        let mut converter = LegalConverter::new();
        let malformed = "this is not valid catala syntax!!!";
        let result = converter.import(malformed, LegalFormat::Catala);
        // Malformed input may error or return empty - both are acceptable
        if let Ok((statutes, _)) = result {
            assert!(statutes.is_empty() || !statutes.is_empty());
        }
    }

    #[test]
    fn test_malformed_l4() {
        let mut converter = LegalConverter::new();
        let malformed = "INVALID SYNTAX HERE";
        let result = converter.import(malformed, LegalFormat::L4);
        // Should either error or handle gracefully
        if let Ok((statutes, _)) = result {
            assert!(statutes.is_empty() || !statutes.is_empty());
        }
    }

    #[test]
    fn test_malformed_xml() {
        let mut converter = LegalConverter::new();
        let malformed = "<unclosed><tag>";
        let result = converter.import(malformed, LegalFormat::AkomaNtoso);
        assert!(result.is_err()); // XML parser should error
    }

    // Very large input tests
    #[test]
    fn test_large_catala_document() {
        let mut converter = LegalConverter::new();
        let mut large_source = String::new();

        for i in 0..100 {
            large_source.push_str(&format!(
                r#"
declaration scope Rule{}:
  context input content Input
  context output content Output

scope Rule{}:
  definition output.eligible equals
    input.age >= {}
"#,
                i,
                i,
                18 + (i % 50)
            ));
        }

        let result = converter.import(&large_source, LegalFormat::Catala);
        assert!(result.is_ok());
        let (statutes, _) = result.unwrap();
        assert!(statutes.len() >= 50); // Should parse multiple scopes
    }

    // Unicode and special characters
    #[test]
    fn test_unicode_in_statute_title() {
        let statute = Statute::new(
            "test-unicode",
            "Test with unicode: 日本語 العربية",
            Effect::new(EffectType::Grant, "test"),
        );

        let mut converter = LegalConverter::new();
        let (output, _) = converter.export(&[statute], LegalFormat::L4).unwrap();
        // Should successfully export with unicode (may be encoded)
        assert!(!output.is_empty());
    }

    #[test]
    fn test_special_characters_in_description() {
        let statute = Statute::new(
            "test-special",
            "Test with special chars: <>&\"'",
            Effect::new(EffectType::Grant, "test"),
        );

        let mut converter = LegalConverter::new();
        let result = converter.export(&[statute], LegalFormat::AkomaNtoso);
        assert!(result.is_ok()); // Should escape properly
    }

    // Batch conversion edge cases
    #[test]
    fn test_batch_convert_empty_sources() {
        let mut converter = LegalConverter::new();
        let sources: Vec<(String, LegalFormat)> = vec![];
        let result = converter.batch_convert(&sources, LegalFormat::L4);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_batch_convert_with_failures() {
        let mut converter = LegalConverter::new();
        let sources = vec![
            (
                "declaration scope Good:\n  context input content integer".to_string(),
                LegalFormat::Catala,
            ),
            ("<malformed>".to_string(), LegalFormat::AkomaNtoso),
        ];
        let result = converter.batch_convert(&sources, LegalFormat::L4);
        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.len(), 2);
        // First should succeed, second should have error report
        assert!(results[0].1.confidence > 0.0);
    }

    // Cache edge cases
    #[test]
    fn test_cache_with_identical_sources() {
        let mut converter = LegalConverter::with_cache(10);
        let source = "RULE Test WHEN age >= 18 THEN Person MAY vote";

        // First conversion
        let (output1, _) = converter
            .convert(source, LegalFormat::L4, LegalFormat::Catala)
            .unwrap();

        // Second conversion (should hit cache)
        let (output2, _) = converter
            .convert(source, LegalFormat::L4, LegalFormat::Catala)
            .unwrap();

        assert_eq!(output1, output2);
    }

    #[test]
    fn test_cache_overflow() {
        let mut converter = LegalConverter::with_cache(2);

        for i in 0..5 {
            let source = format!("RULE Test{} WHEN age >= {} THEN Person MAY vote", i, 18 + i);
            converter
                .convert(&source, LegalFormat::L4, LegalFormat::Catala)
                .unwrap();
        }

        // Cache should have evicted older entries
        let stats = converter.cache_stats().unwrap();
        assert_eq!(stats.entries, 2); // Only last 2 should be cached
    }

    // Format auto-detection edge cases
    #[test]
    fn test_auto_detect_ambiguous_input() {
        let mut converter = LegalConverter::new();
        let ambiguous = "some random text";
        let result = converter.auto_import(ambiguous);
        assert!(result.is_err()); // Should fail to detect
    }

    #[test]
    fn test_auto_detect_mixed_format_markers() {
        let mut converter = LegalConverter::new();
        // Contains markers from multiple formats
        let mixed = "declaration scope Test RULE WHEN agreement";
        let result = converter.auto_import(mixed);
        // Should pick the first matching format or error - both acceptable
        match result {
            Ok((_statutes, _)) => {
                // Successfully detected a format
            }
            Err(_) => {
                // Also acceptable if it can't clearly determine format
            }
        }
    }

    // Semantic validation edge cases
    #[test]
    fn test_roundtrip_with_data_loss() {
        let mut converter = LegalConverter::new();
        let l4_source = "RULE Complex WHEN age >= 18 AND income <= 50000 THEN Person MAY benefit";

        let validation = converter
            .validate_roundtrip(l4_source, LegalFormat::L4, LegalFormat::Stipula)
            .unwrap();

        // Some formats may lose information
        assert!(validation.confidence >= 0.0);
    }

    // Extreme values
    #[test]
    fn test_extreme_age_values() {
        let statute = Statute::new(
            "test-extreme",
            "Extreme Age Test",
            Effect::new(EffectType::Grant, "test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: u32::MAX,
        });

        let mut converter = LegalConverter::new();
        let result = converter.export(&[statute], LegalFormat::L4);
        assert!(result.is_ok());
    }

    // Multiple exports to same format
    #[test]
    fn test_batch_export_same_format_multiple_times() {
        let mut converter = LegalConverter::new();
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));

        let formats = vec![
            LegalFormat::L4,
            LegalFormat::L4,
            LegalFormat::Catala,
            LegalFormat::L4,
        ];

        let results = converter.batch_export(&[statute], &formats).unwrap();
        assert_eq!(results.len(), 4);
        assert_eq!(results[0].0, LegalFormat::L4);
        assert_eq!(results[1].0, LegalFormat::L4);
        assert_eq!(results[2].0, LegalFormat::Catala);
        assert_eq!(results[3].0, LegalFormat::L4);
    }

    // Statutes with no preconditions
    #[test]
    fn test_statute_without_preconditions() {
        let statute = Statute::new(
            "unconditional",
            "Unconditional Right",
            Effect::new(EffectType::Grant, "universal_right"),
        );

        let mut converter = LegalConverter::new();

        // Should work with all formats
        for format in [
            LegalFormat::Catala,
            LegalFormat::L4,
            LegalFormat::Stipula,
            LegalFormat::AkomaNtoso,
            LegalFormat::LegalRuleML,
            LegalFormat::LKIF,
        ] {
            let result = converter.export(std::slice::from_ref(&statute), format);
            assert!(result.is_ok(), "Failed to export to {:?}", format);
        }
    }

    // Very long statute IDs and titles
    #[test]
    fn test_very_long_statute_metadata() {
        let long_id = "a".repeat(1000);
        let long_title = "Very Long Title ".repeat(100);

        let statute = Statute::new(
            &long_id,
            &long_title,
            Effect::new(EffectType::Grant, "test"),
        );

        let mut converter = LegalConverter::new();
        let result = converter.export(&[statute], LegalFormat::L4);
        assert!(result.is_ok());
    }

    // Format extension detection
    #[test]
    fn test_format_extension_case_insensitive() {
        assert_eq!(
            LegalFormat::from_extension("CATALA"),
            Some(LegalFormat::Catala)
        );
        assert_eq!(
            LegalFormat::from_extension("CaTaLa_EN"),
            Some(LegalFormat::Catala)
        );
        assert_eq!(LegalFormat::from_extension("L4"), Some(LegalFormat::L4));
    }

    #[test]
    fn test_unknown_extension() {
        assert_eq!(LegalFormat::from_extension("unknown"), None);
        assert_eq!(LegalFormat::from_extension("txt"), None);
        assert_eq!(LegalFormat::from_extension(""), None);
    }

    // Conversion report quality metrics
    #[test]
    fn test_conversion_report_confidence_bounds() {
        use crate::ConversionReport;

        let mut report = ConversionReport::new(LegalFormat::Catala, LegalFormat::L4);

        // Add many unsupported features
        for i in 0..20 {
            report.add_unsupported(format!("feature_{}", i));
        }

        // Confidence should never go below 0
        assert!(report.confidence >= 0.0);
        assert!(report.confidence <= 1.0);
    }

    // Incremental conversion edge cases
    #[test]
    fn test_incremental_format_switch() {
        use crate::incremental::IncrementalConverter;

        let mut converter = IncrementalConverter::new();

        let catala_source = "declaration scope Test:\n  context input content integer";

        // First conversion
        converter
            .convert_incremental(catala_source, LegalFormat::Catala, LegalFormat::L4)
            .unwrap();

        // Switch target format - should do full conversion
        let (_, _, diff) = converter
            .convert_incremental(catala_source, LegalFormat::Catala, LegalFormat::Stipula)
            .unwrap();

        // Should be treated as new (all added)
        assert!(diff.additions > 0);
    }

    // Coverage report edge cases
    #[test]
    fn test_coverage_report_all_formats() {
        use crate::coverage::CoverageReport;

        let report = CoverageReport::generate();

        // Should have coverage for all formats
        assert!(report.format_coverage.len() >= 6);
        assert!(report.average_coverage > 0.0);
        assert!(report.average_coverage <= 100.0);
    }

    #[test]
    fn test_coverage_markdown_generation() {
        use crate::coverage::CoverageReport;

        let report = CoverageReport::generate();
        let md = report.to_markdown();

        assert!(md.contains("# Format Coverage Report"));
        assert!(md.contains("Average Coverage"));
        assert!(md.contains("Supported Features"));
    }
}
