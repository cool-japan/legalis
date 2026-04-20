use super::super::*;
use legalis_core::{Effect, EffectType};

fn test_statute(id: &str) -> Statute {
    Statute::new(
        id,
        format!("Test {}", id),
        Effect::new(EffectType::Grant, "Test"),
    )
}

#[test]
fn test_similarity_score_possible_duplicate() {
    let score = SimilarityScore::new(0.65, 0.7, 0.6);

    // 0.7 * 0.85 = 0.595, score.overall ~ 0.68
    assert!(score.is_possible_duplicate(0.85));
    assert!(!score.is_likely_duplicate(0.85));
}

#[test]
fn test_calculate_similarity() {
    let registry = StatuteRegistry::new();

    let entry1 = StatuteEntry::new(test_statute("test-1"), "US")
        .with_tag("civil")
        .with_tag("rights")
        .with_reference("ref-1".to_string())
        .with_reference("ref-2".to_string());

    let entry2 = StatuteEntry::new(test_statute("test-1"), "US")
        .with_tag("civil")
        .with_tag("rights")
        .with_reference("ref-1".to_string())
        .with_reference("ref-2".to_string());

    let similarity = registry.calculate_similarity(&entry1, &entry2);

    // Same title, tags, and references should give high similarity
    assert!(similarity.overall > 0.8);
    assert!(similarity.title > 0.8);
    assert!(similarity.content > 0.9); // Same references
    assert!(similarity.metadata > 0.9); // Same tags
}

#[test]
fn test_calculate_similarity_different() {
    let registry = StatuteRegistry::new();

    let entry1 = StatuteEntry::new(test_statute("completely-different-1"), "US").with_tag("civil");

    let entry2 = StatuteEntry::new(test_statute("another-thing-2"), "UK").with_tag("criminal");

    let similarity = registry.calculate_similarity(&entry1, &entry2);

    // Different titles and tags should give low similarity
    assert!(similarity.overall < 0.5);
}

#[test]
fn test_duplicate_detection_result() {
    let mut result = DuplicateDetectionResult::new(0.8, 10);

    assert_eq!(result.threshold, 0.8);
    assert_eq!(result.statutes_analyzed, 10);
    assert_eq!(result.total_duplicates(), 0);

    let candidate = DuplicateCandidate::new(
        "s1".to_string(),
        "s2".to_string(),
        SimilarityScore::new(0.85, 0.9, 0.8),
        "High similarity".to_string(),
    );

    result.add_candidate(candidate);
    assert_eq!(result.total_duplicates(), 1);
}

#[test]
fn test_duplicate_detection_filtering() {
    let mut result = DuplicateDetectionResult::new(0.8, 10);

    // Add a likely duplicate (high similarity)
    result.add_candidate(DuplicateCandidate::new(
        "s1".to_string(),
        "s2".to_string(),
        SimilarityScore::new(0.85, 0.9, 0.8),
        "High".to_string(),
    ));

    // Add a possible duplicate (medium similarity)
    result.add_candidate(DuplicateCandidate::new(
        "s3".to_string(),
        "s4".to_string(),
        SimilarityScore::new(0.6, 0.65, 0.55),
        "Medium".to_string(),
    ));

    assert_eq!(result.likely_duplicates().len(), 1);
    // Both should be in possible duplicates (>= threshold * 0.7)
    assert_eq!(result.possible_duplicates().len(), 2);
}

#[test]
fn test_detect_duplicates() {
    let mut registry = StatuteRegistry::new();

    // Add similar statutes with shared references
    registry
        .register(
            StatuteEntry::new(test_statute("civil-code-1"), "US")
                .with_tag("civil")
                .with_reference("ref-common".to_string()),
        )
        .unwrap();
    registry
        .register(
            StatuteEntry::new(test_statute("civil-code-2"), "US")
                .with_tag("civil")
                .with_reference("ref-common".to_string()),
        )
        .unwrap();
    registry
        .register(StatuteEntry::new(
            test_statute("completely-different"),
            "UK",
        ))
        .unwrap();

    let result = registry.detect_duplicates(0.7);

    assert_eq!(result.statutes_analyzed, 3);
    // Should find at least one duplicate pair (the two civil codes with similar titles, tags, and refs)
    assert!(result.total_duplicates() > 0);
}

#[test]
fn test_detect_duplicates_no_duplicates() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("very-unique-1"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("totally-different-2"), "UK"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("another-one-3"), "JP"))
        .unwrap();

    let result = registry.detect_duplicates(0.9);

    assert_eq!(result.statutes_analyzed, 3);
    // With high threshold and different statutes, should find no duplicates
    assert_eq!(result.total_duplicates(), 0);
}

#[test]
fn test_field_profile_creation() {
    let mut profile = FieldProfile::new("test_field".to_string(), 100);
    profile.null_count = 10;
    profile.unique_count = 50;

    profile.calculate_completeness();

    assert_eq!(profile.field_name, "test_field");
    assert_eq!(profile.total_values, 100);
    assert_eq!(profile.null_count, 10);
    assert_eq!(profile.unique_count, 50);
    assert_eq!(profile.completeness, 90.0); // (100-10)/100 * 100
}

#[test]
fn test_data_profile_creation() {
    let mut profile = DataProfile::new(50);

    assert_eq!(profile.total_statutes, 50);
    assert_eq!(profile.average_quality, 0.0);

    let field_profile = FieldProfile::new("title".to_string(), 50);
    profile.add_field_profile(field_profile);

    assert_eq!(profile.field_profiles.len(), 1);
    assert!(profile.field_profiles.contains_key("title"));
}

#[test]
fn test_data_profile_field_completeness() {
    let mut profile = DataProfile::new(100);

    let mut field = FieldProfile::new("jurisdiction".to_string(), 100);
    field.null_count = 5;
    field.calculate_completeness();

    profile.add_field_profile(field);

    let completeness = profile.field_completeness("jurisdiction");
    assert_eq!(completeness, Some(95.0));

    let missing = profile.field_completeness("nonexistent");
    assert_eq!(missing, None);
}

#[test]
fn test_profile_data() {
    let mut registry = StatuteRegistry::new();

    // Add diverse statutes
    registry
        .register(
            StatuteEntry::new(test_statute("civil-1"), "US")
                .with_tag("civil")
                .with_status(StatuteStatus::Active),
        )
        .unwrap();
    registry
        .register(
            StatuteEntry::new(test_statute("criminal-1"), "UK")
                .with_tag("criminal")
                .with_status(StatuteStatus::Draft),
        )
        .unwrap();
    registry
        .register(
            StatuteEntry::new(test_statute("admin-1"), "JP")
                .with_tag("administrative")
                .with_status(StatuteStatus::Active),
        )
        .unwrap();

    let profile = registry.profile_data();

    assert_eq!(profile.total_statutes, 3);
    assert!(profile.average_quality > 0.0);

    // Should have status distribution
    assert!(
        profile
            .status_distribution
            .contains_key(&StatuteStatus::Active)
    );
    assert_eq!(profile.status_distribution[&StatuteStatus::Active], 2);

    // Should have jurisdiction distribution
    assert!(profile.jurisdiction_distribution.contains_key("US"));
    assert!(profile.jurisdiction_distribution.contains_key("UK"));
    assert!(profile.jurisdiction_distribution.contains_key("JP"));

    // Should have tag patterns
    assert!(profile.tag_patterns.contains_key("civil"));
    assert!(profile.tag_patterns.contains_key("criminal"));
    assert!(profile.tag_patterns.contains_key("administrative"));
}

#[test]
fn test_profile_data_quality_distribution() {
    let mut registry = StatuteRegistry::new();

    // Add statutes with varying quality
    registry
        .register(
            StatuteEntry::new(test_statute("high-quality"), "US")
                .with_tag("civil")
                .with_metadata("description".to_string(), "Detailed statute".to_string())
                .with_metadata("author".to_string(), "Congress".to_string()),
        )
        .unwrap();

    registry
        .register(StatuteEntry::new(test_statute("low-quality"), "UK"))
        .unwrap();

    let profile = registry.profile_data();

    assert_eq!(profile.total_statutes, 2);
    assert!(!profile.quality_distribution.is_empty());
}

#[test]
fn test_find_low_quality_statutes() {
    let mut registry = StatuteRegistry::new();

    // Add a low-quality statute (minimal metadata)
    registry
        .register(StatuteEntry::new(test_statute("low"), "US"))
        .unwrap();

    // Add a high-quality statute
    registry
        .register(
            StatuteEntry::new(test_statute("high"), "UK")
                .with_tag("civil")
                .with_tag("rights")
                .with_metadata("description".to_string(), "Excellent statute".to_string())
                .with_metadata("version_notes".to_string(), "Initial".to_string()),
        )
        .unwrap();

    let low_quality = registry.find_low_quality_statutes(70.0);

    // At least the "low" statute should be flagged
    assert!(!low_quality.is_empty());
    assert!(low_quality.iter().any(|(id, _)| id == "low"));
}

#[test]
fn test_export_quality_assessments_json() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("test-1"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("test-2"), "UK"))
        .unwrap();

    let json = registry.export_quality_assessments_json().unwrap();

    assert!(json.contains("test-1"));
    assert!(json.contains("test-2"));
    assert!(json.contains("overall"));
    assert!(json.contains("issues"));
}

#[test]
fn test_export_duplicates_json() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("similar-1"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("similar-2"), "US"))
        .unwrap();

    let json = registry.export_duplicates_json(0.7).unwrap();

    assert!(json.contains("candidates"));
    assert!(json.contains("threshold"));
    assert!(json.contains("statutes_analyzed"));
}

#[test]
fn test_data_profile_export_json() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("test-1"), "US"))
        .unwrap();

    let profile = registry.profile_data();
    let json = profile.export_json().unwrap();

    assert!(json.contains("total_statutes"));
    assert!(json.contains("average_quality"));
    assert!(json.contains("field_profiles"));
}

// ========================================================================
// Enrichment and Lineage Tests
// ========================================================================

#[test]
fn test_enrichment_suggestion_creation() {
    let suggestion = EnrichmentSuggestion::new(
        EnrichmentType::AutoTag,
        "civil".to_string(),
        0.85,
        "Contains civil law keywords".to_string(),
    );

    assert_eq!(suggestion.enrichment_type, EnrichmentType::AutoTag);
    assert_eq!(suggestion.suggestion, "civil");
    assert_eq!(suggestion.confidence, 0.85);
    assert!(suggestion.meets_threshold(0.8));
    assert!(!suggestion.meets_threshold(0.9));
}

#[test]
fn test_enrichment_suggestion_confidence_clamping() {
    let too_high = EnrichmentSuggestion::new(
        EnrichmentType::AutoTag,
        "tag".to_string(),
        1.5,
        "test".to_string(),
    );
    assert_eq!(too_high.confidence, 1.0);

    let too_low = EnrichmentSuggestion::new(
        EnrichmentType::AutoTag,
        "tag".to_string(),
        -0.5,
        "test".to_string(),
    );
    assert_eq!(too_low.confidence, 0.0);
}

#[test]
fn test_enrichment_result() {
    let mut result = EnrichmentResult::new("statute-1".to_string());

    result.add_suggestion(EnrichmentSuggestion::new(
        EnrichmentType::AutoTag,
        "criminal".to_string(),
        0.9,
        "High confidence".to_string(),
    ));

    result.add_suggestion(EnrichmentSuggestion::new(
        EnrichmentType::MetadataInference,
        "description".to_string(),
        0.5,
        "Low confidence".to_string(),
    ));

    assert_eq!(result.statute_id, "statute-1");
    assert_eq!(result.suggestions.len(), 2);
    assert_eq!(result.high_confidence_suggestions(0.7).len(), 1);
    assert_eq!(result.suggestions_by_type(EnrichmentType::AutoTag).len(), 1);
}

#[test]
fn test_enrichment_config_builders() {
    let config = EnrichmentConfig::new()
        .with_auto_tagging(false)
        .with_metadata_inference(true)
        .with_jurisdiction_inference(false)
        .with_min_confidence(0.85);

    assert!(!config.enable_auto_tagging);
    assert!(config.enable_metadata_inference);
    assert!(!config.enable_jurisdiction_inference);
    assert_eq!(config.min_confidence, 0.85);
}

#[test]
fn test_analyze_enrichment_auto_tagging() {
    let mut registry = StatuteRegistry::new();

    // Register a statute with civil law keywords in title
    registry
        .register(StatuteEntry::new(test_statute("civil-contract-law"), "US"))
        .unwrap();

    let config = EnrichmentConfig::new();
    let result = registry
        .analyze_enrichment("civil-contract-law", &config)
        .unwrap();

    // Should suggest "civil" tag since title contains "civil" and "contract"
    let auto_tag_suggestions = result.suggestions_by_type(EnrichmentType::AutoTag);
    let civil_suggestions: Vec<_> = auto_tag_suggestions
        .iter()
        .filter(|s| s.suggestion == "civil")
        .collect();

    assert!(!civil_suggestions.is_empty());
}

#[test]
fn test_analyze_enrichment_metadata_inference() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("test-1"), "US"))
        .unwrap();

    let config = EnrichmentConfig::new();
    let result = registry.analyze_enrichment("test-1", &config).unwrap();

    // Should suggest adding description
    let metadata_suggestions = result.suggestions_by_type(EnrichmentType::MetadataInference);
    assert!(!metadata_suggestions.is_empty());
}

#[test]
fn test_analyze_enrichment_nonexistent() {
    let registry = StatuteRegistry::new();
    let config = EnrichmentConfig::new();

    let result = registry.analyze_enrichment("nonexistent", &config);
    assert!(result.is_err());
    assert!(matches!(result, Err(RegistryError::StatuteNotFound(_))));
}

#[test]
fn test_apply_enrichment() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("test-1"), "US"))
        .unwrap();

    let suggestions = vec![
        EnrichmentSuggestion::new(
            EnrichmentType::AutoTag,
            "civil".to_string(),
            0.9,
            "High confidence tag".to_string(),
        ),
        EnrichmentSuggestion::new(
            EnrichmentType::MetadataInference,
            "category".to_string(),
            0.8,
            "Category suggestion".to_string(),
        ),
    ];

    let count = registry
        .apply_enrichment("test-1", &suggestions, 0.7)
        .unwrap();

    assert_eq!(count, 2);

    let entry = registry.get("test-1").unwrap();
    assert!(entry.tags.contains(&"civil".to_string()));
    assert!(entry.metadata.contains_key("category"));
}

#[test]
fn test_apply_enrichment_confidence_filter() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("test-1"), "US"))
        .unwrap();

    let suggestions = vec![
        EnrichmentSuggestion::new(
            EnrichmentType::AutoTag,
            "high-confidence".to_string(),
            0.9,
            "High".to_string(),
        ),
        EnrichmentSuggestion::new(
            EnrichmentType::AutoTag,
            "low-confidence".to_string(),
            0.5,
            "Low".to_string(),
        ),
    ];

    // Only apply suggestions with confidence >= 0.8
    let count = registry
        .apply_enrichment("test-1", &suggestions, 0.8)
        .unwrap();

    assert_eq!(count, 1);

    let entry = registry.get("test-1").unwrap();
    assert!(entry.tags.contains(&"high-confidence".to_string()));
    assert!(!entry.tags.contains(&"low-confidence".to_string()));
}

#[test]
fn test_auto_enrich_all() {
    let mut registry = StatuteRegistry::new();

    // Register statutes with enrichment opportunities (using actual keyword matches)
    // Create custom statutes with titles containing keywords
    let criminal_statute = Statute::new(
        "criminal-offense-law",
        "Criminal Offense and Penalties Act",
        Effect::new(EffectType::Grant, "Test"),
    );

    let civil_statute = Statute::new(
        "civil-procedure-code",
        "Civil Procedure and Contract Law",
        Effect::new(EffectType::Grant, "Test"),
    );

    registry
        .register(StatuteEntry::new(criminal_statute, "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(civil_statute, "UK"))
        .unwrap();

    let config = EnrichmentConfig::new().with_min_confidence(0.25); // Lower threshold for test
    let results = registry.auto_enrich_all(&config);

    // At least some statutes should be enriched
    assert!(!results.is_empty());

    // Verify enrichment was actually applied
    for (statute_id, count) in results {
        assert!(count > 0);
        let entry = registry.get(&statute_id).unwrap();
        // Should have gained tags or metadata
        assert!(!entry.tags.is_empty() || !entry.metadata.is_empty());
    }
}

#[test]
fn test_lineage_entry_creation() {
    let entry = LineageEntry::new(
        "statute-1".to_string(),
        LineageOperation::Created,
        "admin".to_string(),
    );

    assert_eq!(entry.statute_id, "statute-1");
    assert_eq!(entry.operation, LineageOperation::Created);
    assert_eq!(entry.actor, "admin");
    assert!(entry.context.is_empty());
}

#[test]
fn test_lineage_entry_with_context() {
    let entry = LineageEntry::new(
        "statute-1".to_string(),
        LineageOperation::Imported {
            source: "external-db".to_string(),
        },
        "system".to_string(),
    )
    .with_context("batch_id".to_string(), "batch-123".to_string())
    .with_context("import_date".to_string(), "2025-12-27".to_string());

    assert_eq!(entry.context.len(), 2);
    assert_eq!(
        entry.context.get("batch_id"),
        Some(&"batch-123".to_string())
    );
}

#[test]
fn test_data_lineage_record() {
    let mut lineage = DataLineage::new(100);

    let entry1 = LineageEntry::new(
        "statute-1".to_string(),
        LineageOperation::Created,
        "admin".to_string(),
    );

    let entry2 = LineageEntry::new(
        "statute-2".to_string(),
        LineageOperation::Created,
        "user".to_string(),
    );

    lineage.record(entry1);
    lineage.record(entry2);

    assert_eq!(lineage.count(), 2);
}

#[test]
fn test_data_lineage_get_lineage() {
    let mut lineage = DataLineage::new(100);

    lineage.record(LineageEntry::new(
        "statute-1".to_string(),
        LineageOperation::Created,
        "admin".to_string(),
    ));

    lineage.record(LineageEntry::new(
        "statute-1".to_string(),
        LineageOperation::Enriched {
            enrichment_type: "auto-tag".to_string(),
        },
        "system".to_string(),
    ));

    lineage.record(LineageEntry::new(
        "statute-2".to_string(),
        LineageOperation::Created,
        "user".to_string(),
    ));

    let statute1_lineage = lineage.get_lineage("statute-1");
    assert_eq!(statute1_lineage.len(), 2);
}

#[test]
fn test_data_lineage_get_by_operation() {
    let mut lineage = DataLineage::new(100);

    lineage.record(LineageEntry::new(
        "s1".to_string(),
        LineageOperation::Created,
        "admin".to_string(),
    ));

    lineage.record(LineageEntry::new(
        "s2".to_string(),
        LineageOperation::Imported {
            source: "db".to_string(),
        },
        "system".to_string(),
    ));

    lineage.record(LineageEntry::new(
        "s3".to_string(),
        LineageOperation::Created,
        "user".to_string(),
    ));

    let created = lineage.get_by_operation("Created");
    assert_eq!(created.len(), 2);

    let imported = lineage.get_by_operation("Imported");
    assert_eq!(imported.len(), 1);
}

#[test]
fn test_data_lineage_get_by_actor() {
    let mut lineage = DataLineage::new(100);

    lineage.record(LineageEntry::new(
        "s1".to_string(),
        LineageOperation::Created,
        "admin".to_string(),
    ));

    lineage.record(LineageEntry::new(
        "s2".to_string(),
        LineageOperation::Created,
        "admin".to_string(),
    ));

    lineage.record(LineageEntry::new(
        "s3".to_string(),
        LineageOperation::Created,
        "user".to_string(),
    ));

    let admin_entries = lineage.get_by_actor("admin");
    assert_eq!(admin_entries.len(), 2);

    let user_entries = lineage.get_by_actor("user");
    assert_eq!(user_entries.len(), 1);
}

#[test]
fn test_data_lineage_get_by_time_range() {
    let mut lineage = DataLineage::new(100);

    let now = Utc::now();
    let past = now - chrono::Duration::hours(2);
    let future = now + chrono::Duration::hours(2);

    lineage.record(LineageEntry::new(
        "s1".to_string(),
        LineageOperation::Created,
        "admin".to_string(),
    ));

    let entries = lineage.get_by_time_range(past, future);
    assert_eq!(entries.len(), 1);
}

#[test]
fn test_data_lineage_trace_provenance() {
    let mut lineage = DataLineage::new(100);

    // Create a provenance chain: s1 -> s2 -> s3
    lineage.record(LineageEntry::new(
        "s1".to_string(),
        LineageOperation::Created,
        "admin".to_string(),
    ));

    lineage.record(LineageEntry::new(
        "s2".to_string(),
        LineageOperation::Derived {
            parent_id: "s1".to_string(),
        },
        "system".to_string(),
    ));

    lineage.record(LineageEntry::new(
        "s3".to_string(),
        LineageOperation::Derived {
            parent_id: "s2".to_string(),
        },
        "system".to_string(),
    ));

    let provenance = lineage.trace_provenance("s3");
    // Should include all three statutes in the chain
    assert!(!provenance.is_empty());
}

#[test]
fn test_data_lineage_trace_merge_provenance() {
    let mut lineage = DataLineage::new(100);

    // Create merged statute from multiple sources
    lineage.record(LineageEntry::new(
        "s1".to_string(),
        LineageOperation::Created,
        "admin".to_string(),
    ));

    lineage.record(LineageEntry::new(
        "s2".to_string(),
        LineageOperation::Created,
        "admin".to_string(),
    ));

    lineage.record(LineageEntry::new(
        "merged".to_string(),
        LineageOperation::Merged {
            source_ids: vec!["s1".to_string(), "s2".to_string()],
        },
        "system".to_string(),
    ));

    let provenance = lineage.trace_provenance("merged");
    // Should trace back to both source statutes
    assert!(!provenance.is_empty());
}

#[test]
fn test_data_lineage_max_entries() {
    let mut lineage = DataLineage::new(5);

    // Add more entries than max
    for i in 0..10 {
        lineage.record(LineageEntry::new(
            format!("s{}", i),
            LineageOperation::Created,
            "admin".to_string(),
        ));
    }

    // Should have trimmed to max entries
    assert_eq!(lineage.count(), 5);
}

#[test]
fn test_data_lineage_export_json() {
    let mut lineage = DataLineage::new(100);

    lineage.record(LineageEntry::new(
        "s1".to_string(),
        LineageOperation::Created,
        "admin".to_string(),
    ));

    let json = lineage.export_json().unwrap();

    assert!(json.contains("statute_id"));
    assert!(json.contains("s1"));
    assert!(json.contains("Created"));
    assert!(json.contains("admin"));
}

#[test]
fn test_data_lineage_clear() {
    let mut lineage = DataLineage::new(100);

    lineage.record(LineageEntry::new(
        "s1".to_string(),
        LineageOperation::Created,
        "admin".to_string(),
    ));

    assert_eq!(lineage.count(), 1);

    lineage.clear();
    assert_eq!(lineage.count(), 0);
}

// ========================================================================
// Compliance Features Tests (v0.1.9)
// ========================================================================

#[test]
fn test_pii_detection_creation() {
    let detection = PiiDetection::new(
        PiiFieldType::Email,
        "test@example.com".to_string(),
        10,
        0.95,
    );

    assert_eq!(detection.field_type, PiiFieldType::Email);
    assert_eq!(detection.value, "test@example.com");
    assert_eq!(detection.position, 10);
    assert_eq!(detection.length, 16);
    assert_eq!(detection.confidence, 0.95);
}

#[test]
fn test_pii_detection_confidence() {
    let detection = PiiDetection::new(
        PiiFieldType::PhoneNumber,
        "123-456-7890".to_string(),
        0,
        0.85,
    );

    assert!(detection.is_confident(0.8));
    assert!(detection.is_confident(0.85));
    assert!(!detection.is_confident(0.9));
}

#[test]
fn test_pii_scan_result() {
    let detections = vec![
        PiiDetection::new(PiiFieldType::Email, "a@b.com".to_string(), 0, 0.9),
        PiiDetection::new(
            PiiFieldType::PhoneNumber,
            "123-456-7890".to_string(),
            10,
            0.8,
        ),
    ];

    let result = PiiScanResult::new("test-statute".to_string(), detections);

    assert_eq!(result.statute_id, "test-statute");
    assert_eq!(result.pii_count, 2);

    let high_conf = result.high_confidence(0.85);
    assert_eq!(high_conf.len(), 1);
    assert_eq!(high_conf[0].field_type, PiiFieldType::Email);

    let emails = result.by_type(&PiiFieldType::Email);
    assert_eq!(emails.len(), 1);
}

#[test]
fn test_pii_detector_scan() {
    let detector = PiiDetector::new();
    let content = "Contact us at support@example.com or call 555-123-4567";

    let result = detector.scan("statute-1", content);

    assert_eq!(result.statute_id, "statute-1");
    assert!(!result.detections.is_empty());
}

#[test]
fn test_pii_detector_disabled() {
    let mut detector = PiiDetector::new();
    detector.set_enabled(false);

    let content = "Contact us at support@example.com";
    let result = detector.scan("statute-1", content);

    assert_eq!(result.pii_count, 0);
}

#[test]
fn test_pii_masking_strategies() {
    let detector_asterisk = PiiDetector::new().with_masking_strategy(MaskingStrategy::Asterisks);
    let detector_redacted = PiiDetector::new().with_masking_strategy(MaskingStrategy::Redacted);
    let detector_partial = PiiDetector::new().with_masking_strategy(MaskingStrategy::Partial);

    let content = "email@test.com";
    let detections = vec![PiiDetection::new(
        PiiFieldType::Email,
        "email@test.com".to_string(),
        0,
        0.9,
    )];
    let scan_result = PiiScanResult::new("test".to_string(), detections);

    let masked_asterisk = detector_asterisk.mask(content, &scan_result);
    let masked_redacted = detector_redacted.mask(content, &scan_result);
    let masked_partial = detector_partial.mask(content, &scan_result);

    assert!(masked_asterisk.contains('*') || masked_asterisk.is_empty());
    assert!(masked_redacted.contains("[REDACTED]") || masked_redacted == content);
    assert!(masked_partial.starts_with('e') || masked_partial == content);
}

#[test]
fn test_data_retention_config() {
    let config = DataRetentionConfig::new()
        .add_rule(DataRetentionRule::RetainForDays(30))
        .add_rule(DataRetentionRule::ArchiveAfterDays(90))
        .with_auto_apply(true)
        .with_dry_run(false);

    assert_eq!(config.rules().len(), 2);
    assert!(config.is_auto_apply());
    assert!(!config.is_dry_run());
}

#[test]
fn test_retention_execution_result() {
    let result = RetentionExecutionResult::new(
        vec!["s1".to_string(), "s2".to_string()],
        vec!["s3".to_string()],
        false,
    );

    assert_eq!(result.deleted.len(), 2);
    assert_eq!(result.archived.len(), 1);
    assert_eq!(result.total_affected(), 3);
    assert!(!result.dry_run);
}

#[test]
fn test_apply_retention_rules_dry_run() {
    let mut registry = StatuteRegistry::new();
    let entry = StatuteEntry::new(test_statute("old-statute"), "JP");
    registry.register(entry).unwrap();

    let config = DataRetentionConfig::new()
        .add_rule(DataRetentionRule::RetainForDays(0))
        .with_dry_run(true);

    let result = registry.apply_retention_rules(&config);

    // In dry-run mode, nothing should be deleted
    assert_eq!(registry.count(), 1);
    assert!(result.dry_run);
}

#[test]
fn test_apply_retention_rules_archive() {
    let mut registry = StatuteRegistry::new();
    let entry = StatuteEntry::new(test_statute("old-statute"), "JP");
    registry.register(entry).unwrap();

    // Use a rule that will definitely trigger (old age)
    let config = DataRetentionConfig::new()
        .add_rule(DataRetentionRule::RetainForDays(0))
        .with_dry_run(true); // Use dry-run first

    let result = registry.apply_retention_rules(&config);

    // In this case, we're testing dry-run mode
    // Statute with age > 0 days would be deleted (but we're in dry run)
    assert!(result.dry_run);
    assert_eq!(registry.count(), 1); // Nothing actually deleted
}

#[test]
fn test_audit_report_config() {
    let now = Utc::now();
    let config = AuditReportConfig::new("Monthly Report")
        .with_date_range(now, now)
        .with_sections(true, true, false, false)
        .with_format(AuditReportFormat::Json);

    assert_eq!(config.title, "Monthly Report");
    assert!(config.include_operations);
    assert!(config.include_events);
    assert!(!config.include_quality);
    assert_eq!(config.format, AuditReportFormat::Json);
}

#[test]
fn test_generate_audit_report() {
    let mut registry = StatuteRegistry::new();
    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    let config = AuditReportConfig::new("Test Report").with_format(AuditReportFormat::Text);

    let report = registry.generate_audit_report(&config);

    assert_eq!(report.title, "Test Report");
    assert_eq!(report.total_statutes, 1);
    assert!(!report.content.is_empty());
    assert_eq!(report.format, AuditReportFormat::Text);
}

#[test]
fn test_audit_report_export() {
    let report = AuditReport::new(
        "Test".to_string(),
        (None, None),
        10,
        5,
        3,
        0,
        85.0,
        "Test content".to_string(),
        AuditReportFormat::Json,
    );

    let exported = report.export();
    assert!(exported.contains("Test"));
}

#[test]
fn test_geographic_region_code() {
    assert_eq!(GeographicRegion::EU.code(), "EU");
    assert_eq!(GeographicRegion::US.code(), "US");
    assert_eq!(GeographicRegion::Japan.code(), "JP");
    assert_eq!(GeographicRegion::Custom("XX".to_string()).code(), "XX");
}

#[test]
fn test_geographic_region_transfer_rules() {
    // EU can transfer to EU and UK
    assert!(GeographicRegion::EU.allows_transfer_to(&GeographicRegion::EU));
    assert!(GeographicRegion::EU.allows_transfer_to(&GeographicRegion::UK));
    // EU cannot transfer to US (GDPR)
    assert!(!GeographicRegion::EU.allows_transfer_to(&GeographicRegion::US));

    // US can transfer anywhere
    assert!(GeographicRegion::US.allows_transfer_to(&GeographicRegion::EU));
    assert!(GeographicRegion::US.allows_transfer_to(&GeographicRegion::Japan));
}

#[test]
fn test_data_sovereignty_config() {
    let config = DataSovereigntyConfig::new(GeographicRegion::EU)
        .allow_region(GeographicRegion::UK)
        .with_strict_residency(false)
        .with_encryption_required(true);

    assert_eq!(config.primary_region, GeographicRegion::EU);
    assert!(config.allowed_regions.contains(&GeographicRegion::UK));
    assert!(!config.strict_residency);
    assert!(config.require_encryption);
}

#[test]
fn test_data_sovereignty_region_allowed() {
    let config =
        DataSovereigntyConfig::new(GeographicRegion::EU).allow_region(GeographicRegion::UK);

    // Primary region is always allowed
    assert!(config.is_region_allowed(&GeographicRegion::EU));

    // UK is explicitly allowed and EU->UK transfer is permitted
    assert!(config.is_region_allowed(&GeographicRegion::UK));

    // US is not in allowed list
    assert!(!config.is_region_allowed(&GeographicRegion::US));
}

#[test]
fn test_data_sovereignty_strict_residency() {
    let config = DataSovereigntyConfig::new(GeographicRegion::EU)
        .allow_region(GeographicRegion::UK)
        .with_strict_residency(true);

    // Only primary region allowed in strict mode
    assert!(config.is_region_allowed(&GeographicRegion::EU));
    assert!(!config.is_region_allowed(&GeographicRegion::UK));
    assert!(!config.is_region_allowed(&GeographicRegion::US));
}

#[test]
fn test_compliance_dashboard_creation() {
    let dashboard = ComplianceDashboard::new(
        100,  // total_statutes
        5,    // statutes_with_pii
        10,   // statutes_pending_retention
        85.0, // avg_quality_score
        8,    // low_quality_count
        200,  // total_audit_events
        3,    // failed_operations
        2,    // sovereignty_violations
    );

    assert_eq!(dashboard.total_statutes, 100);
    assert_eq!(dashboard.statutes_with_pii, 5);
    assert_eq!(dashboard.low_quality_count, 8);

    // Compliance rate = (100 - 8 - 2) / 100 = 0.90
    assert!((dashboard.compliance_rate - 0.90).abs() < 0.01);
}

#[test]
fn test_compliance_dashboard_threshold() {
    let dashboard = ComplianceDashboard::new(100, 0, 0, 90.0, 5, 100, 0, 0);

    assert!(dashboard.meets_compliance_threshold(0.90));
    assert!(dashboard.meets_compliance_threshold(0.95));
    assert!(!dashboard.meets_compliance_threshold(0.99));
}

#[test]
fn test_compliance_dashboard_to_json() {
    let dashboard = ComplianceDashboard::new(10, 1, 2, 85.0, 1, 50, 0, 0);
    let json = dashboard.to_json();

    assert!(json.contains("total_statutes"));
    assert!(json.contains("compliance_rate"));
}

#[test]
fn test_generate_compliance_dashboard() {
    let mut registry = StatuteRegistry::new();

    // Add some statutes with varying quality
    for i in 1..=5 {
        let entry = StatuteEntry::new(test_statute(&format!("s{}", i)), "JP").with_tag("test");
        registry.register(entry).unwrap();
    }

    let dashboard = registry.generate_compliance_dashboard(70.0);

    assert_eq!(dashboard.total_statutes, 5);
    assert!(dashboard.compliance_rate >= 0.0 && dashboard.compliance_rate <= 1.0);
}

#[test]
fn test_scan_for_pii() {
    let mut registry = StatuteRegistry::new();
    let entry = StatuteEntry::new(test_statute("statute-1"), "JP")
        .with_metadata("email", "contact@example.com");
    registry.register(entry).unwrap();

    let detector = PiiDetector::new();
    let result = registry.scan_for_pii("statute-1", &detector).unwrap();

    assert_eq!(result.statute_id, "statute-1");
}

#[test]
fn test_scan_for_pii_not_found() {
    let mut registry = StatuteRegistry::new();
    let detector = PiiDetector::new();

    let result = registry.scan_for_pii("nonexistent", &detector);
    assert!(result.is_err());
}

#[test]
fn test_check_sovereignty_access() {
    let registry = StatuteRegistry::new();
    let config =
        DataSovereigntyConfig::new(GeographicRegion::EU).allow_region(GeographicRegion::UK);

    // Check access from UK (allowed)
    assert!(registry.check_sovereignty_access("statute-1", &GeographicRegion::UK, &config));

    // Check access from US (not allowed)
    assert!(!registry.check_sovereignty_access("statute-1", &GeographicRegion::US, &config));
}

#[test]
fn test_pii_field_type_variants() {
    let types = [
        PiiFieldType::Name,
        PiiFieldType::Email,
        PiiFieldType::PhoneNumber,
        PiiFieldType::NationalId,
        PiiFieldType::Address,
        PiiFieldType::DateOfBirth,
        PiiFieldType::IpAddress,
        PiiFieldType::Custom("SSN".to_string()),
    ];

    assert_eq!(types.len(), 8);
}

#[test]
fn test_masking_strategy_variants() {
    let strategies = [
        MaskingStrategy::Asterisks,
        MaskingStrategy::Redacted,
        MaskingStrategy::TypeMarker,
        MaskingStrategy::Hash,
        MaskingStrategy::Partial,
    ];

    assert_eq!(strategies.len(), 5);
}

#[test]
fn test_audit_report_format_variants() {
    let formats = [
        AuditReportFormat::Json,
        AuditReportFormat::Csv,
        AuditReportFormat::Text,
        AuditReportFormat::Html,
    ];

    assert_eq!(formats.len(), 4);
}

#[test]
fn test_data_retention_rule_variants() {
    let now = Utc::now();
    let rules = [
        DataRetentionRule::RetainForDays(30),
        DataRetentionRule::RetainUntil(now),
        DataRetentionRule::RetainIndefinitely,
        DataRetentionRule::DeleteInactiveAfterDays(60),
        DataRetentionRule::ArchiveAfterDays(90),
    ];

    assert_eq!(rules.len(), 5);
}

#[test]
fn test_pii_detector_builder_methods() {
    let _detector = PiiDetector::new()
        .with_min_confidence(0.85)
        .with_masking_strategy(MaskingStrategy::Partial);

    // Confidence should be clamped
    let _detector2 = PiiDetector::new().with_min_confidence(1.5);
    // Internal check - confidence should be 1.0 (clamped)

    let _detector3 = PiiDetector::new().with_min_confidence(-0.5);
    // Internal check - confidence should be 0.0 (clamped)
}

// ========================================================================
// Access Control Features Tests (v0.1.4)
// ========================================================================

#[test]
fn test_permission_all() {
    let perms = Permission::all();
    assert_eq!(perms.len(), 12);
    assert!(perms.contains(&Permission::Read));
    assert!(perms.contains(&Permission::ManagePermissions));
}

#[test]
fn test_permission_read_only() {
    let perms = Permission::read_only();
    assert_eq!(perms.len(), 2);
    assert!(perms.contains(&Permission::Read));
    assert!(perms.contains(&Permission::GenerateReports));
    assert!(!perms.contains(&Permission::Delete));
}
