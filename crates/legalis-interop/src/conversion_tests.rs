//! Crate-root integration tests for the universal converter and per-format
//! round-trips. Extracted verbatim from the former inline `mod tests` in
//! `lib.rs` to keep that file within the 2000-line limit; as a sibling file
//! module its `super` resolves to the crate root exactly as before.

use super::*;
use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

#[test]
fn test_format_extension() {
    assert_eq!(LegalFormat::Catala.extension(), "catala_en");
    assert_eq!(LegalFormat::Stipula.extension(), "stipula");
    assert_eq!(LegalFormat::L4.extension(), "l4");
}

#[test]
fn test_format_from_extension() {
    assert_eq!(
        LegalFormat::from_extension("catala_en"),
        Some(LegalFormat::Catala)
    );
    assert_eq!(
        LegalFormat::from_extension("stipula"),
        Some(LegalFormat::Stipula)
    );
    assert_eq!(LegalFormat::from_extension("l4"), Some(LegalFormat::L4));
    assert_eq!(LegalFormat::from_extension("unknown"), None);
}

#[test]
fn test_conversion_report() {
    let mut report = ConversionReport::new(LegalFormat::Catala, LegalFormat::Legalis);
    assert_eq!(report.confidence, 1.0);

    report.add_unsupported("scopes");
    assert!(report.confidence < 1.0);

    report.add_warning("Date format normalized");
    assert!(report.unsupported_features.contains(&"scopes".to_string()));
}

#[test]
fn test_converter_supported_formats() {
    let converter = LegalConverter::new();
    let imports = converter.supported_imports();
    let exports = converter.supported_exports();

    assert!(imports.contains(&LegalFormat::Catala));
    assert!(imports.contains(&LegalFormat::Stipula));
    assert!(imports.contains(&LegalFormat::L4));
    assert!(imports.contains(&LegalFormat::AkomaNtoso));

    assert!(exports.contains(&LegalFormat::Catala));
    assert!(exports.contains(&LegalFormat::Stipula));
    assert!(exports.contains(&LegalFormat::L4));
    assert!(exports.contains(&LegalFormat::AkomaNtoso));
}

#[test]
fn test_catala_export_import_roundtrip() {
    let mut converter = LegalConverter::new();

    // Create a statute
    let statute = Statute::new(
        "voting-rights",
        "Voting Rights",
        Effect::new(EffectType::Grant, "vote"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    // Export to Catala
    let (catala_output, export_report) = converter.export(&[statute], LegalFormat::Catala).unwrap();
    assert_eq!(export_report.statutes_converted, 1);
    assert!(catala_output.contains("declaration scope VotingRights"));
    assert!(catala_output.contains("input.age >= 18"));

    // Import from Catala
    let (imported, import_report) = converter
        .import(&catala_output, LegalFormat::Catala)
        .unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert_eq!(imported.len(), 1);
    assert_eq!(imported[0].id, "votingrights");
}

#[test]
fn test_stipula_export_import_roundtrip() {
    let mut converter = LegalConverter::new();

    // Create a statute
    let statute = Statute::new(
        "simple-contract",
        "Simple Contract",
        Effect::new(EffectType::Grant, "execute"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 21,
    });

    // Export to Stipula
    let (stipula_output, export_report) =
        converter.export(&[statute], LegalFormat::Stipula).unwrap();
    assert_eq!(export_report.statutes_converted, 1);
    assert!(stipula_output.contains("agreement SimpleContract"));
    assert!(stipula_output.contains("age >= 21"));

    // Import from Stipula
    let (imported, import_report) = converter
        .import(&stipula_output, LegalFormat::Stipula)
        .unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert_eq!(imported.len(), 1);
    assert_eq!(imported[0].id, "simplecontract");
}

#[test]
fn test_l4_export_import_roundtrip() {
    let mut converter = LegalConverter::new();

    // Create a statute
    let statute = Statute::new(
        "adult-rights",
        "Adult Rights",
        Effect::new(EffectType::Grant, "full_capacity"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    // Export to L4
    let (l4_output, export_report) = converter.export(&[statute], LegalFormat::L4).unwrap();
    assert_eq!(export_report.statutes_converted, 1);
    assert!(l4_output.contains("RULE AdultRights"));
    assert!(l4_output.contains("age >= 18"));
    assert!(l4_output.contains("MAY"));

    // Import from L4
    let (imported, import_report) = converter.import(&l4_output, LegalFormat::L4).unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert_eq!(imported.len(), 1);
}

#[test]
fn test_catala_to_l4_conversion() {
    let mut converter = LegalConverter::new();

    let catala_source = r#"
```catala
declaration scope TaxBenefit:
  context input content Input
  context output content Output
```

```catala
scope TaxBenefit:
  definition output.eligible equals
    input.age >= 65
```
"#;

    // Convert Catala to L4
    let (l4_output, report) = converter
        .convert(catala_source, LegalFormat::Catala, LegalFormat::L4)
        .unwrap();

    assert!(report.statutes_converted >= 1);
    assert!(l4_output.contains("RULE"));
}

#[test]
fn test_auto_detect_catala() {
    let mut converter = LegalConverter::new();

    let catala_source = r#"
declaration scope Test:
  context input content integer
"#;

    let (statutes, report) = converter.auto_import(catala_source).unwrap();
    assert_eq!(report.source_format, Some(LegalFormat::Catala));
    assert!(!statutes.is_empty());
}

#[test]
fn test_auto_detect_stipula() {
    let mut converter = LegalConverter::new();

    let stipula_source = "agreement TestContract(Alice, Bob) { }";

    let (statutes, report) = converter.auto_import(stipula_source).unwrap();
    assert_eq!(report.source_format, Some(LegalFormat::Stipula));
    assert!(!statutes.is_empty());
}

#[test]
fn test_auto_detect_l4() {
    let mut converter = LegalConverter::new();

    let l4_source = "RULE TestRule WHEN age >= 18 THEN Person MAY vote";

    let (statutes, report) = converter.auto_import(l4_source).unwrap();
    assert_eq!(report.source_format, Some(LegalFormat::L4));
    assert!(!statutes.is_empty());
}

#[test]
fn test_akoma_ntoso_export_import_roundtrip() {
    let mut converter = LegalConverter::new();

    // Create a statute
    let statute = Statute::new(
        "adult-capacity",
        "Adult Capacity Act",
        Effect::new(EffectType::Grant, "Full legal capacity"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    // Export to Akoma Ntoso
    let (akn_output, export_report) = converter
        .export(&[statute], LegalFormat::AkomaNtoso)
        .unwrap();
    assert_eq!(export_report.statutes_converted, 1);
    assert!(akn_output.contains("<akomaNtoso"));
    assert!(akn_output.contains("Adult Capacity Act"));

    // Import from Akoma Ntoso
    let (imported, import_report) = converter
        .import(&akn_output, LegalFormat::AkomaNtoso)
        .unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert_eq!(imported.len(), 1);
    assert_eq!(imported[0].title, "Adult Capacity Act");
}

#[test]
fn test_auto_detect_akoma_ntoso() {
    let mut converter = LegalConverter::new();

    let akn_source = r#"
        <akomaNtoso>
            <act>
                <body>
                    <article eId="art_1">
                        <heading>Test Article</heading>
                    </article>
                </body>
            </act>
        </akomaNtoso>
        "#;

    let (statutes, report) = converter.auto_import(akn_source).unwrap();
    assert_eq!(report.source_format, Some(LegalFormat::AkomaNtoso));
    assert!(!statutes.is_empty());
}

#[test]
fn test_catala_to_akoma_ntoso_conversion() {
    let mut converter = LegalConverter::new();

    let catala_source = r#"
declaration scope AdultRights:
  context input content integer
"#;

    // Convert Catala to Akoma Ntoso
    let (akn_output, report) = converter
        .convert(catala_source, LegalFormat::Catala, LegalFormat::AkomaNtoso)
        .unwrap();

    assert!(report.statutes_converted >= 1);
    assert!(akn_output.contains("<akomaNtoso"));
    assert!(akn_output.contains("<article"));
}

#[test]
fn test_legalruleml_export_import_roundtrip() {
    let mut converter = LegalConverter::new();

    // Create a statute
    let statute = Statute::new(
        "legal-rule",
        "Legal Rule Example",
        Effect::new(EffectType::Grant, "Legal capacity"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    // Export to LegalRuleML
    let (lrml_output, export_report) = converter
        .export(&[statute], LegalFormat::LegalRuleML)
        .unwrap();
    assert_eq!(export_report.statutes_converted, 1);
    assert!(lrml_output.contains("<legalruleml"));
    assert!(lrml_output.contains("Legal Rule Example"));

    // Import from LegalRuleML
    let (imported, import_report) = converter
        .import(&lrml_output, LegalFormat::LegalRuleML)
        .unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert_eq!(imported.len(), 1);
    assert_eq!(imported[0].title, "Legal Rule Example");
}

#[test]
fn test_auto_detect_legalruleml() {
    let mut converter = LegalConverter::new();

    let lrml_source = r#"
        <legalruleml>
            <Statements>
                <LegalRule key="test">
                    <Name>Test</Name>
                    <if><Premise>age >= 18</Premise></if>
                    <then><Conclusion>Grant</Conclusion></then>
                </LegalRule>
            </Statements>
        </legalruleml>
        "#;

    let (statutes, report) = converter.auto_import(lrml_source).unwrap();
    assert_eq!(report.source_format, Some(LegalFormat::LegalRuleML));
    assert!(!statutes.is_empty());
}

#[test]
fn test_catala_to_legalruleml_conversion() {
    let mut converter = LegalConverter::new();

    let catala_source = r#"
declaration scope TaxRule:
  context input content integer
"#;

    // Convert Catala to LegalRuleML
    let (lrml_output, report) = converter
        .convert(catala_source, LegalFormat::Catala, LegalFormat::LegalRuleML)
        .unwrap();

    assert!(report.statutes_converted >= 1);
    assert!(lrml_output.contains("<legalruleml"));
    assert!(lrml_output.contains("<LegalRule"));
}

#[test]
fn test_batch_convert() {
    let mut converter = LegalConverter::new();

    let sources = vec![
        (
            "declaration scope Test1:\n  context input content integer".to_string(),
            LegalFormat::Catala,
        ),
        (
            "agreement Test2(A, B) { }".to_string(),
            LegalFormat::Stipula,
        ),
    ];

    let results = converter.batch_convert(&sources, LegalFormat::L4).unwrap();

    assert_eq!(results.len(), 2);
    assert!(results[0].0.contains("RULE"));
    assert!(results[1].0.contains("RULE"));
}

#[test]
fn test_batch_import() {
    let mut converter = LegalConverter::new();

    let sources = vec![
        (
            "declaration scope Test1:\n  context input content integer".to_string(),
            LegalFormat::Catala,
        ),
        (
            "agreement Test2(A, B) { }".to_string(),
            LegalFormat::Stipula,
        ),
    ];

    let results = converter.batch_import(&sources).unwrap();

    assert_eq!(results.len(), 2);
    assert!(!results[0].0.is_empty());
    assert!(!results[1].0.is_empty());
}

#[test]
fn test_batch_export() {
    let mut converter = LegalConverter::new();

    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    );

    let formats = vec![LegalFormat::Catala, LegalFormat::L4, LegalFormat::Stipula];

    let results = converter.batch_export(&[statute], &formats).unwrap();

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].0, LegalFormat::Catala);
    assert_eq!(results[1].0, LegalFormat::L4);
    assert_eq!(results[2].0, LegalFormat::Stipula);
}

#[test]
fn test_conversion_caching() {
    let mut converter = LegalConverter::with_cache(10);

    let catala_source = "declaration scope Test:\n  context input content integer";

    // First conversion - cache miss
    let (output1, report1) = converter
        .convert(catala_source, LegalFormat::Catala, LegalFormat::L4)
        .unwrap();

    // Second conversion - cache hit
    let (output2, report2) = converter
        .convert(catala_source, LegalFormat::Catala, LegalFormat::L4)
        .unwrap();

    // Results should be identical
    assert_eq!(output1, output2);
    assert_eq!(report1.statutes_converted, report2.statutes_converted);

    // Check cache stats
    // Note: We cache both import and conversion, so first run creates 2 entries
    // Second run is a cache hit on conversion
    let stats = converter.cache_stats().unwrap();
    assert_eq!(stats.entries, 2); // import + conversion cached
    assert!(stats.access_count >= 3); // Multiple puts and gets
}

#[test]
fn test_cache_enable_disable() {
    let mut converter = LegalConverter::new();

    // Initially no cache
    assert!(converter.cache_stats().is_none());

    // Enable cache
    converter.enable_cache(5);
    assert!(converter.cache_stats().is_some());

    // Disable cache
    converter.disable_cache();
    assert!(converter.cache_stats().is_none());
}

#[test]
fn test_semantic_validation_roundtrip() {
    let mut converter = LegalConverter::new();

    let l4_source = "RULE VotingAge WHEN age >= 18 THEN Person MAY vote";

    let validation = converter
        .validate_roundtrip(l4_source, LegalFormat::L4, LegalFormat::Catala)
        .unwrap();

    // Should preserve basic structure
    assert!(validation.confidence > 0.0);
    assert!(!validation.issues.is_empty() || validation.passed());
}

#[test]
fn test_conversion_report_quality() {
    let mut report = ConversionReport::new(LegalFormat::Catala, LegalFormat::L4);

    assert!(report.is_lossless());
    assert!(report.is_high_quality());

    report.add_warning("Minor issue");
    assert!(!report.is_lossless());
    assert!(report.is_high_quality());

    report.add_unsupported("Major feature");
    report.add_unsupported("Another feature");
    report.add_unsupported("Yet another");
    assert!(!report.is_high_quality());
}

#[test]
fn test_semantic_validation_structure() {
    let mut converter = LegalConverter::new();

    let catala_source = r#"
declaration scope AdultRights:
  context input content integer
"#;

    let validation = converter
        .validate_roundtrip(catala_source, LegalFormat::Catala, LegalFormat::L4)
        .unwrap();

    // Validation structure should be populated
    assert_eq!(validation.source_format, LegalFormat::Catala);
    assert_eq!(validation.target_format, LegalFormat::L4);
    assert!(validation.confidence <= 1.0);
}

// Tests for new formats (v0.1.5)

#[test]
fn test_legalcite_export_import_roundtrip() {
    let mut converter = LegalConverter::new();

    let statute = Statute::new(
        "test-statute",
        "Test Statute",
        Effect::new(EffectType::Grant, "legal_reference"),
    )
    .with_jurisdiction("US");

    let (legalcite_output, export_report) = converter
        .export(&[statute], LegalFormat::LegalCite)
        .unwrap();
    assert_eq!(export_report.statutes_converted, 1);
    assert!(legalcite_output.contains("legalCite"));

    let (imported, import_report) = converter
        .import(&legalcite_output, LegalFormat::LegalCite)
        .unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert_eq!(imported.len(), 1);
}

#[test]
fn test_metalex_export_import_roundtrip() {
    let mut converter = LegalConverter::new();

    let statute = Statute::new(
        "article-1",
        "Article 1",
        Effect::new(EffectType::Grant, "provision"),
    );

    let (metalex_output, export_report) =
        converter.export(&[statute], LegalFormat::MetaLex).unwrap();
    assert_eq!(export_report.statutes_converted, 1);
    assert!(metalex_output.contains("metalex"));

    let (imported, import_report) = converter
        .import(&metalex_output, LegalFormat::MetaLex)
        .unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert_eq!(imported.len(), 1);
}

#[test]
fn test_mpeg21_rel_export_import_roundtrip() {
    let mut converter = LegalConverter::new();

    let statute = Statute::new(
        "play-right",
        "Play Right",
        Effect::new(EffectType::Grant, "play"),
    );

    let (mpeg21_output, export_report) = converter
        .export(&[statute], LegalFormat::Mpeg21Rel)
        .unwrap();
    assert_eq!(export_report.statutes_converted, 1);

    let (imported, import_report) = converter
        .import(&mpeg21_output, LegalFormat::Mpeg21Rel)
        .unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert_eq!(imported.len(), 1);
}

#[test]
fn test_creative_commons_export_import_roundtrip() {
    let mut converter = LegalConverter::new();

    let statute = Statute::new(
        "cc-permit",
        "Permit Reproduction",
        Effect::new(EffectType::Grant, "Reproduction"),
    );

    let (cc_output, export_report) = converter
        .export(&[statute], LegalFormat::CreativeCommons)
        .unwrap();
    assert_eq!(export_report.statutes_converted, 1);

    let (imported, import_report) = converter
        .import(&cc_output, LegalFormat::CreativeCommons)
        .unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert!(!imported.is_empty());
}

#[test]
fn test_spdx_export_import_roundtrip() {
    let mut converter = LegalConverter::new();

    let mut effect = Effect::new(EffectType::Grant, "use");
    effect
        .parameters
        .insert("spdx_id".to_string(), "MIT".to_string());
    let statute = Statute::new("mit_license", "License: MIT", effect);

    let (spdx_output, export_report) = converter.export(&[statute], LegalFormat::Spdx).unwrap();
    assert_eq!(export_report.statutes_converted, 1);
    assert_eq!(spdx_output, "MIT");

    let (imported, import_report) = converter.import(&spdx_output, LegalFormat::Spdx).unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert_eq!(imported.len(), 1);
}

#[test]
fn test_auto_detect_legalcite() {
    let mut converter = LegalConverter::new();

    let legalcite_source = r#"<LegalCiteDocument>
            <legalCite>
                <citations>
                    <id>test-1</id>
                    <title>Test Citation</title>
                    <citation_type>statute</citation_type>
                </citations>
            </legalCite>
        </LegalCiteDocument>"#;

    let (statutes, report) = converter.auto_import(legalcite_source).unwrap();
    assert_eq!(report.source_format, Some(LegalFormat::LegalCite));
    assert!(!statutes.is_empty());
}

#[test]
fn test_auto_detect_metalex() {
    let mut converter = LegalConverter::new();

    let metalex_source = r#"<MetaLexDocument>
            <metalex>
                <Body>
                    <Article id="art-1">
                        <Title>Test Article</Title>
                    </Article>
                </Body>
            </metalex>
        </MetaLexDocument>"#;

    let (statutes, report) = converter.auto_import(metalex_source).unwrap();
    assert_eq!(report.source_format, Some(LegalFormat::MetaLex));
    assert!(!statutes.is_empty());
}

#[test]
fn test_auto_detect_mpeg21_rel() {
    let mut converter = LegalConverter::new();

    let mpeg21_source = r#"<Mpeg21RelDocument>
            <license>
                <grant>
                    <right>play</right>
                </grant>
            </license>
        </Mpeg21RelDocument>"#;

    let (statutes, report) = converter.auto_import(mpeg21_source).unwrap();
    assert_eq!(report.source_format, Some(LegalFormat::Mpeg21Rel));
    assert!(!statutes.is_empty());
}

#[test]
fn test_auto_detect_creative_commons() {
    let mut converter = LegalConverter::new();

    let cc_source = "https://creativecommons.org/licenses/by/4.0/";

    let (statutes, report) = converter.auto_import(cc_source).unwrap();
    assert_eq!(report.source_format, Some(LegalFormat::CreativeCommons));
    assert!(!statutes.is_empty());
}

#[test]
fn test_auto_detect_spdx() {
    let mut converter = LegalConverter::new();

    let spdx_source = "MIT AND Apache-2.0";

    let (statutes, report) = converter.auto_import(spdx_source).unwrap();
    assert_eq!(report.source_format, Some(LegalFormat::Spdx));
    assert!(!statutes.is_empty());
}

#[test]
fn test_new_formats_in_converter() {
    let converter = LegalConverter::new();
    let imports = converter.supported_imports();
    let exports = converter.supported_exports();

    // Check all new formats are registered
    assert!(imports.contains(&LegalFormat::LegalCite));
    assert!(imports.contains(&LegalFormat::MetaLex));
    assert!(imports.contains(&LegalFormat::Mpeg21Rel));
    assert!(imports.contains(&LegalFormat::CreativeCommons));
    assert!(imports.contains(&LegalFormat::Spdx));

    assert!(exports.contains(&LegalFormat::LegalCite));
    assert!(exports.contains(&LegalFormat::MetaLex));
    assert!(exports.contains(&LegalFormat::Mpeg21Rel));
    assert!(exports.contains(&LegalFormat::CreativeCommons));
    assert!(exports.contains(&LegalFormat::Spdx));
}

// Blockchain format tests (v0.2.9)

#[test]
fn test_blockchain_formats_registered() {
    let converter = LegalConverter::new();
    let imports = converter.supported_imports();
    let exports = converter.supported_exports();

    // Check all blockchain formats are registered
    assert!(imports.contains(&LegalFormat::Solidity));
    assert!(imports.contains(&LegalFormat::Vyper));
    assert!(imports.contains(&LegalFormat::Cadence));
    assert!(imports.contains(&LegalFormat::Move));

    assert!(exports.contains(&LegalFormat::Solidity));
    assert!(exports.contains(&LegalFormat::Vyper));
    assert!(exports.contains(&LegalFormat::Cadence));
    assert!(exports.contains(&LegalFormat::Move));
}

#[test]
fn test_solidity_import_export_roundtrip() {
    let mut converter = LegalConverter::new();

    let mut statute = Statute::new(
        "token_transfer",
        "Token Transfer",
        Effect::new(EffectType::MonetaryTransfer, "Transfer tokens"),
    );
    statute
        .effect
        .parameters
        .insert("contract".to_string(), "ERC20".to_string());
    statute
        .effect
        .parameters
        .insert("function".to_string(), "transfer".to_string());
    statute
        .effect
        .parameters
        .insert("license".to_string(), "MIT".to_string());

    let (solidity_output, export_report) =
        converter.export(&[statute], LegalFormat::Solidity).unwrap();
    assert_eq!(export_report.statutes_converted, 1);
    assert!(solidity_output.contains("contract ERC20"));
    assert!(solidity_output.contains("function transfer()"));

    let (imported, import_report) = converter
        .import(&solidity_output, LegalFormat::Solidity)
        .unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert!(!imported.is_empty());
}

#[test]
fn test_vyper_import_export_roundtrip() {
    let mut converter = LegalConverter::new();

    let mut statute = Statute::new(
        "vote",
        "Vote Function",
        Effect::new(EffectType::Grant, "Cast a vote"),
    );
    statute
        .effect
        .parameters
        .insert("contract".to_string(), "Voting".to_string());
    statute
        .effect
        .parameters
        .insert("function".to_string(), "vote".to_string());
    statute
        .effect
        .parameters
        .insert("license".to_string(), "MIT".to_string());

    let (vyper_output, export_report) = converter.export(&[statute], LegalFormat::Vyper).unwrap();
    assert_eq!(export_report.statutes_converted, 1);
    assert!(vyper_output.contains("# Voting"));
    assert!(vyper_output.contains("def vote()"));

    let (imported, import_report) = converter.import(&vyper_output, LegalFormat::Vyper).unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert!(!imported.is_empty());
}

#[test]
fn test_cadence_import_export_roundtrip() {
    let mut converter = LegalConverter::new();

    let mut statute = Statute::new(
        "mint_nft",
        "Mint NFT",
        Effect::new(EffectType::Grant, "Mint new NFT"),
    );
    statute
        .effect
        .parameters
        .insert("contract".to_string(), "NFT".to_string());
    statute
        .effect
        .parameters
        .insert("function".to_string(), "mintNFT".to_string());

    let (cadence_output, export_report) =
        converter.export(&[statute], LegalFormat::Cadence).unwrap();
    assert_eq!(export_report.statutes_converted, 1);
    assert!(cadence_output.contains("pub contract NFT"));
    assert!(cadence_output.contains("pub fun mintNFT()"));

    let (imported, import_report) = converter
        .import(&cadence_output, LegalFormat::Cadence)
        .unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert!(!imported.is_empty());
}

#[test]
fn test_move_import_export_roundtrip() {
    let mut converter = LegalConverter::new();

    let mut statute = Statute::new(
        "transfer_coin",
        "Transfer Coin",
        Effect::new(EffectType::MonetaryTransfer, "Transfer coins"),
    );
    statute
        .effect
        .parameters
        .insert("module_address".to_string(), "0x1".to_string());
    statute
        .effect
        .parameters
        .insert("module_name".to_string(), "Coin".to_string());
    statute
        .effect
        .parameters
        .insert("function".to_string(), "transfer".to_string());
    statute
        .effect
        .parameters
        .insert("entry".to_string(), "true".to_string());

    let (move_output, export_report) = converter.export(&[statute], LegalFormat::Move).unwrap();
    assert_eq!(export_report.statutes_converted, 1);
    assert!(move_output.contains("module 0x1::Coin"));
    assert!(move_output.contains("public entry fun transfer()"));

    let (imported, import_report) = converter.import(&move_output, LegalFormat::Move).unwrap();
    assert_eq!(import_report.statutes_converted, 1);
    assert!(!imported.is_empty());
}

#[test]
fn test_blockchain_format_extensions() {
    assert_eq!(LegalFormat::Solidity.extension(), "sol");
    assert_eq!(LegalFormat::Vyper.extension(), "vy");
    assert_eq!(LegalFormat::Cadence.extension(), "cdc");
    assert_eq!(LegalFormat::Move.extension(), "move");
}

#[test]
fn test_blockchain_format_from_extension() {
    assert_eq!(
        LegalFormat::from_extension("sol"),
        Some(LegalFormat::Solidity)
    );
    assert_eq!(
        LegalFormat::from_extension("solidity"),
        Some(LegalFormat::Solidity)
    );
    assert_eq!(LegalFormat::from_extension("vy"), Some(LegalFormat::Vyper));
    assert_eq!(
        LegalFormat::from_extension("vyper"),
        Some(LegalFormat::Vyper)
    );
    assert_eq!(
        LegalFormat::from_extension("cdc"),
        Some(LegalFormat::Cadence)
    );
    assert_eq!(
        LegalFormat::from_extension("cadence"),
        Some(LegalFormat::Cadence)
    );
    assert_eq!(LegalFormat::from_extension("move"), Some(LegalFormat::Move));
}

#[test]
fn test_solidity_source_import() {
    let mut converter = LegalConverter::new();

    let source = r#"
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.0;

        contract SimpleStorage {
            function store(uint256 value) public {
            }
        }
        "#;

    let (statutes, report) = converter.import(source, LegalFormat::Solidity).unwrap();
    assert_eq!(report.statutes_converted, 1);
    assert!(!statutes.is_empty());
    assert_eq!(
        statutes[0].effect.parameters.get("contract"),
        Some(&"SimpleStorage".to_string())
    );
}

#[test]
fn test_vyper_source_import() {
    let mut converter = LegalConverter::new();

    let source = r#"
        # @version 0.3.0
        # @license MIT

        @external
        def transfer():
            pass
        "#;

    let (statutes, report) = converter.import(source, LegalFormat::Vyper).unwrap();
    assert_eq!(report.statutes_converted, 1);
    assert!(!statutes.is_empty());
}

#[test]
fn test_cadence_source_import() {
    let mut converter = LegalConverter::new();

    let source = r#"
        pub contract FlowToken {
            pub fun transfer() {
            }
        }
        "#;

    let (statutes, report) = converter.import(source, LegalFormat::Cadence).unwrap();
    assert_eq!(report.statutes_converted, 1);
    assert!(!statutes.is_empty());
    assert_eq!(
        statutes[0].effect.parameters.get("blockchain"),
        Some(&"Flow".to_string())
    );
}

#[test]
fn test_move_source_import() {
    let mut converter = LegalConverter::new();

    let source = r#"
        module 0x1::Coin {
            public entry fun mint() {
            }
        }
        "#;

    let (statutes, report) = converter.import(source, LegalFormat::Move).unwrap();
    assert_eq!(report.statutes_converted, 1);
    assert!(!statutes.is_empty());
    assert_eq!(
        statutes[0].effect.parameters.get("blockchain"),
        Some(&"Move".to_string())
    );
}

#[test]
fn test_cross_blockchain_conversion() {
    let mut converter = LegalConverter::new();

    // Import from Solidity
    let solidity_source = r#"
        pragma solidity ^0.8.0;
        contract Token {
            function transfer() public {
            }
        }
        "#;

    let (statutes, _) = converter
        .import(solidity_source, LegalFormat::Solidity)
        .unwrap();

    // Export to Vyper
    let (vyper_output, report) = converter.export(&statutes, LegalFormat::Vyper).unwrap();
    assert_eq!(report.statutes_converted, 1);
    assert!(vyper_output.contains("def transfer()"));

    // Export to Cadence
    let (cadence_output, report) = converter.export(&statutes, LegalFormat::Cadence).unwrap();
    assert_eq!(report.statutes_converted, 1);
    assert!(cadence_output.contains("pub fun transfer()"));

    // Export to Move
    let (move_output, report) = converter.export(&statutes, LegalFormat::Move).unwrap();
    assert_eq!(report.statutes_converted, 1);
    assert!(move_output.contains("fun transfer()"));
}

// Cross-reality (immersive / spatial) format tests (v0.3.4)

const CROSS_REALITY_FORMATS: [LegalFormat; 5] = [
    LegalFormat::VrArAnnotation,
    LegalFormat::SpatialDocument3D,
    LegalFormat::Holographic,
    LegalFormat::SpatialMarkup,
    LegalFormat::MetaverseLegal,
];

#[test]
fn test_cross_reality_formats_registered() {
    let converter = LegalConverter::new();
    let imports = converter.supported_imports();
    let exports = converter.supported_exports();
    for format in CROSS_REALITY_FORMATS {
        assert!(imports.contains(&format), "missing importer {format:?}");
        assert!(exports.contains(&format), "missing exporter {format:?}");
    }
}

#[test]
fn test_cross_reality_extensions() {
    assert_eq!(LegalFormat::VrArAnnotation.extension(), "var.json");
    assert_eq!(LegalFormat::SpatialDocument3D.extension(), "l3d.json");
    assert_eq!(LegalFormat::Holographic.extension(), "holo.json");
    assert_eq!(LegalFormat::SpatialMarkup.extension(), "slm");
    assert_eq!(LegalFormat::MetaverseLegal.extension(), "mvl.json");
}

#[test]
fn test_cross_reality_from_extension() {
    assert_eq!(
        LegalFormat::from_extension("var.json"),
        Some(LegalFormat::VrArAnnotation)
    );
    assert_eq!(
        LegalFormat::from_extension("l3d"),
        Some(LegalFormat::SpatialDocument3D)
    );
    assert_eq!(
        LegalFormat::from_extension("holo.json"),
        Some(LegalFormat::Holographic)
    );
    assert_eq!(
        LegalFormat::from_extension("slm"),
        Some(LegalFormat::SpatialMarkup)
    );
    assert_eq!(
        LegalFormat::from_extension("metaverse.json"),
        Some(LegalFormat::MetaverseLegal)
    );
}

#[test]
fn test_cross_reality_roundtrip_via_converter() {
    let mut converter = LegalConverter::new();
    let statute = Statute::new(
        "spatial-rights",
        "Spatial Rights",
        Effect::new(EffectType::Grant, "Grant spatial access"),
    )
    .with_jurisdiction("US")
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 21,
    });

    for format in CROSS_REALITY_FORMATS {
        let (output, export_report) = converter
            .export(std::slice::from_ref(&statute), format)
            .unwrap();
        assert_eq!(export_report.statutes_converted, 1, "export {format:?}");

        let (imported, import_report) = converter.import(&output, format).unwrap();
        assert_eq!(import_report.statutes_converted, 1, "import {format:?}");
        assert_eq!(imported.len(), 1);
        assert_eq!(imported[0].id, "spatial-rights", "id lost for {format:?}");
        assert_eq!(imported[0].jurisdiction.as_deref(), Some("US"));
        assert_eq!(imported[0].preconditions.len(), 1, "cond lost {format:?}");
    }
}

#[test]
fn test_l4_to_cross_reality_conversion() {
    let mut converter = LegalConverter::new();
    let l4_source = "RULE VotingAge WHEN age >= 18 THEN Person MAY vote";

    for format in CROSS_REALITY_FORMATS {
        let (output, report) = converter
            .convert(l4_source, LegalFormat::L4, format)
            .unwrap();
        assert!(report.statutes_converted >= 1, "convert L4 -> {format:?}");
        assert!(!output.is_empty(), "empty output for {format:?}");
    }
}

#[test]
fn test_auto_detect_cross_reality() {
    let mut converter = LegalConverter::new();
    let statute = Statute::new(
        "auto-detect",
        "Auto Detect",
        Effect::new(EffectType::Obligation, "Detect me"),
    );

    for format in CROSS_REALITY_FORMATS {
        let (output, _) = converter
            .export(std::slice::from_ref(&statute), format)
            .unwrap();
        let (statutes, report) = converter.auto_import(&output).unwrap();
        assert_eq!(report.source_format, Some(format), "auto-detect {format:?}");
        assert_eq!(statutes.len(), 1);
    }
}
