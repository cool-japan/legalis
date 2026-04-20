//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

/// Translation quality score (0.0 to 1.0).
pub type QualityScore = f32;
#[cfg(test)]
mod mt_tests {
    use crate::*;
    use std::sync::{Arc, Mutex};
    #[test]
    fn test_neural_mt_basic() {
        let translator = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("ja").with_country("JP");
        let result = translator.translate("contract", &source, &target);
        assert!(result.is_ok());
        let translation = result.unwrap();
        assert!(!translation.text.is_empty());
        assert!(translation.quality_score >= 0.0 && translation.quality_score <= 1.0);
        assert_eq!(translation.engine, TranslationEngine::Generic);
    }
    #[test]
    fn test_neural_mt_legal_domain() {
        let translator = NeuralMachineTranslator::legal_domain();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("de").with_country("DE");
        let result = translator.translate("plaintiff", &source, &target);
        assert!(result.is_ok());
        let translation = result.unwrap();
        assert_eq!(translation.engine, TranslationEngine::LegalDomain);
        assert!(translation.quality_score >= 0.8);
    }
    #[test]
    fn test_neural_mt_with_dictionary() {
        let locale_en_us = Locale::new("en").with_country("US");
        let mut dict = LegalDictionary::new(locale_en_us.clone());
        dict.add_translation("contract", "Vertrag");
        let translator = NeuralMachineTranslator::new(TranslationEngine::Generic)
            .with_dictionary(Arc::new(dict));
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("de").with_country("DE");
        let result = translator.translate("contract", &source, &target).unwrap();
        assert_eq!(result.text, "Vertrag");
        assert!(result.quality_score > 0.9);
    }
    #[test]
    fn test_neural_mt_alternatives() {
        let translator = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");
        let result = translator.translate("statute", &source, &target).unwrap();
        assert!(!result.alternatives.is_empty());
        assert!(result.alternatives.len() >= 2);
    }
    #[test]
    fn test_terminology_aware_translator() {
        let mt = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let mut term_translator = TerminologyAwareTranslator::new(mt);
        term_translator.add_term("plaintiff", "demandeur");
        term_translator.add_term("defendant", "défendeur");
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");
        let result = term_translator
            .translate("The plaintiff sued the defendant", &source, &target)
            .unwrap();
        assert!(result.text.contains("demandeur"));
        assert!(result.text.contains("défendeur"));
        assert_eq!(term_translator.glossary_size(), 2);
    }
    #[test]
    fn test_terminology_aware_with_dictionary() {
        let locale_en_us = Locale::new("en").with_country("US");
        let mut dict = LegalDictionary::new(locale_en_us);
        dict.add_translation("tort", "responsabilité civile");
        dict.add_translation("contract", "contrat");
        let mt = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let mut term_translator = TerminologyAwareTranslator::new(mt);
        let target = Locale::new("fr").with_country("FR");
        term_translator.load_glossary_from_dictionary(&dict, &target);
        assert_eq!(term_translator.glossary_size(), 2);
    }
    #[test]
    fn test_mt_with_memory_exact_match() {
        let mt = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let mut memory = TranslationMemory::new();
        let source_locale = Locale::new("en").with_country("US");
        let target_locale = Locale::new("fr").with_country("FR");
        memory.add_translation(
            "contract".to_string(),
            source_locale.clone(),
            "contrat".to_string(),
            target_locale.clone(),
        );
        let mt_with_memory = MTWithMemory::new(mt, Arc::new(Mutex::new(memory)));
        let result = mt_with_memory
            .translate("contract", &source_locale, &target_locale)
            .unwrap();
        assert_eq!(result.text, "contrat");
        assert_eq!(result.quality_score, 1.0);
    }
    #[test]
    fn test_mt_with_memory_fallback() {
        let mt = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let memory = TranslationMemory::new();
        let mt_with_memory = MTWithMemory::new(mt, Arc::new(Mutex::new(memory)));
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("ja").with_country("JP");
        let result = mt_with_memory
            .translate("new term", &source, &target)
            .unwrap();
        assert!(!result.text.is_empty());
    }
    #[test]
    fn test_glossary_enforcer_mandatory_terms() {
        let mut enforcer = GlossaryEnforcer::new();
        enforcer.add_mandatory_term("plaintiff", "demandeur");
        enforcer.add_mandatory_term("defendant", "défendeur");
        let source = "The plaintiff sued the defendant";
        let translation = "Le demandeur a poursuivi la partie adverse";
        let (_, violations) = enforcer.enforce(source, translation);
        assert!(!violations.is_empty());
        assert!(
            violations
                .iter()
                .any(|v| v.violation_type == ViolationType::MissingMandatoryTerm)
        );
    }
    #[test]
    fn test_glossary_enforcer_forbidden_terms() {
        let mut enforcer = GlossaryEnforcer::new();
        enforcer.add_forbidden_term("bad word");
        enforcer.add_forbidden_term("inappropriate");
        let source = "This is a test";
        let translation = "This contains a bad word";
        let (enforced, violations) = enforcer.enforce(source, translation);
        assert!(!violations.is_empty());
        assert!(
            violations
                .iter()
                .any(|v| v.violation_type == ViolationType::ForbiddenTermUsed)
        );
        assert!(enforced.contains("[REMOVED]") || enforced.contains("[FORBIDDEN]"));
    }
    #[test]
    fn test_glossary_enforcer_counts() {
        let mut enforcer = GlossaryEnforcer::new();
        enforcer.add_mandatory_term("term1", "translation1");
        enforcer.add_mandatory_term("term2", "translation2");
        enforcer.add_forbidden_term("forbidden1");
        assert_eq!(enforcer.mandatory_term_count(), 2);
        assert_eq!(enforcer.forbidden_term_count(), 1);
    }
    #[test]
    fn test_post_editing_workflow() {
        let mut workflow = PostEditingWorkflow::new();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");
        let translation1 = MTTranslation {
            text: "contrat".to_string(),
            quality_score: 0.9,
            source_locale: source.clone(),
            target_locale: target.clone(),
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };
        let translation2 = MTTranslation {
            text: "accord".to_string(),
            quality_score: 0.7,
            source_locale: source.clone(),
            target_locale: target.clone(),
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };
        workflow.add_for_review("contract", translation1);
        workflow.add_for_review("agreement", translation2);
        assert_eq!(workflow.pending_count(), 2);
        assert_eq!(workflow.accepted_count(), 0);
        assert_eq!(workflow.rejected_count(), 0);
    }
    #[test]
    fn test_post_editing_accept() {
        let mut workflow = PostEditingWorkflow::new();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");
        let translation = MTTranslation {
            text: "contrat".to_string(),
            quality_score: 0.95,
            source_locale: source,
            target_locale: target,
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };
        workflow.add_for_review("contract", translation);
        let feedback = PostEditFeedback {
            original: "contrat".to_string(),
            edited: None,
            action: PostEditAction::Accept,
            quality_rating: Some(0.95),
            comments: vec![],
        };
        workflow.submit_feedback(0, feedback);
        assert_eq!(workflow.pending_count(), 0);
        assert_eq!(workflow.accepted_count(), 1);
        assert_eq!(workflow.rejected_count(), 0);
    }
    #[test]
    fn test_post_editing_reject() {
        let mut workflow = PostEditingWorkflow::new();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("ja").with_country("JP");
        let translation = MTTranslation {
            text: "bad translation".to_string(),
            quality_score: 0.4,
            source_locale: source,
            target_locale: target,
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };
        workflow.add_for_review("contract", translation);
        let feedback = PostEditFeedback {
            original: "bad translation".to_string(),
            edited: None,
            action: PostEditAction::Reject,
            quality_rating: Some(0.2),
            comments: vec!["Poor quality".to_string()],
        };
        workflow.submit_feedback(0, feedback);
        assert_eq!(workflow.pending_count(), 0);
        assert_eq!(workflow.accepted_count(), 0);
        assert_eq!(workflow.rejected_count(), 1);
    }
    #[test]
    fn test_post_editing_edit() {
        let mut workflow = PostEditingWorkflow::new();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("de").with_country("DE");
        let translation = MTTranslation {
            text: "Kontrakt".to_string(),
            quality_score: 0.7,
            source_locale: source,
            target_locale: target,
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };
        workflow.add_for_review("contract", translation);
        let feedback = PostEditFeedback {
            original: "Kontrakt".to_string(),
            edited: Some("Vertrag".to_string()),
            action: PostEditAction::Edit,
            quality_rating: Some(0.9),
            comments: vec!["Corrected to proper legal term".to_string()],
        };
        workflow.submit_feedback(0, feedback);
        assert_eq!(workflow.pending_count(), 0);
        assert_eq!(workflow.accepted_count(), 1);
    }
    #[test]
    fn test_post_editing_export_to_memory() {
        let mut workflow = PostEditingWorkflow::new();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");
        let translation = MTTranslation {
            text: "contrat".to_string(),
            quality_score: 0.95,
            source_locale: source.clone(),
            target_locale: target.clone(),
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };
        workflow.add_for_review("contract", translation);
        let feedback = PostEditFeedback {
            original: "contrat".to_string(),
            edited: None,
            action: PostEditAction::Accept,
            quality_rating: Some(0.95),
            comments: vec![],
        };
        workflow.submit_feedback(0, feedback);
        let mut memory = TranslationMemory::new();
        workflow.export_to_memory(&mut memory, &source, &target);
        let entries = memory.find_exact("contract", &source, &target);
        assert!(!entries.is_empty());
        assert_eq!(entries[0].target_text, "contrat");
    }
    #[test]
    fn test_translation_engine_display() {
        assert_eq!(TranslationEngine::Generic.to_string(), "Generic");
        assert_eq!(TranslationEngine::LegalDomain.to_string(), "Legal Domain");
        assert_eq!(TranslationEngine::Custom.to_string(), "Custom");
    }
    #[test]
    fn test_violation_type_display() {
        assert_eq!(
            ViolationType::MissingMandatoryTerm.to_string(),
            "Missing Mandatory Term"
        );
        assert_eq!(
            ViolationType::ForbiddenTermUsed.to_string(),
            "Forbidden Term Used"
        );
    }
    #[test]
    fn test_term_preservation_modes() {
        let mt = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let translator_exact = TerminologyAwareTranslator::new(mt.clone())
            .with_preservation_mode(TermPreservationMode::Exact);
        assert_eq!(
            translator_exact.preservation_mode,
            TermPreservationMode::Exact
        );
        let translator_glossary = TerminologyAwareTranslator::new(mt.clone())
            .with_preservation_mode(TermPreservationMode::GlossaryEnforced);
        assert_eq!(
            translator_glossary.preservation_mode,
            TermPreservationMode::GlossaryEnforced
        );
    }
    #[test]
    fn test_mt_quality_threshold() {
        let translator =
            NeuralMachineTranslator::new(TranslationEngine::Generic).with_quality_threshold(0.85);
        assert_eq!(translator.quality_threshold(), 0.85);
    }
    #[test]
    fn test_mt_with_memory_fuzzy_threshold() {
        let mt = NeuralMachineTranslator::new(TranslationEngine::Generic);
        let memory = TranslationMemory::new();
        let mt_with_memory =
            MTWithMemory::new(mt, Arc::new(Mutex::new(memory))).with_fuzzy_threshold(0.9);
        assert_eq!(mt_with_memory.fuzzy_threshold(), 0.9);
    }
    #[test]
    fn test_workflow_clear() {
        let mut workflow = PostEditingWorkflow::new();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");
        let translation = MTTranslation {
            text: "test".to_string(),
            quality_score: 0.8,
            source_locale: source,
            target_locale: target,
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };
        workflow.add_for_review("test", translation);
        assert_eq!(workflow.pending_count(), 1);
        workflow.clear();
        assert_eq!(workflow.pending_count(), 0);
        assert_eq!(workflow.accepted_count(), 0);
        assert_eq!(workflow.rejected_count(), 0);
    }
    #[test]
    fn test_workflow_get_pending() {
        let mut workflow = PostEditingWorkflow::new();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("ja").with_country("JP");
        let translation = MTTranslation {
            text: "test".to_string(),
            quality_score: 0.8,
            source_locale: source,
            target_locale: target,
            engine: TranslationEngine::Generic,
            alternatives: vec![],
        };
        workflow.add_for_review("original", translation);
        let pending = workflow.get_pending(0);
        assert!(pending.is_some());
        assert_eq!(pending.unwrap().0, "original");
    }
}
#[cfg(test)]
mod international_standards_tests {
    use crate::*;
    #[test]
    fn test_iso639_3_creation() {
        let code = ISO639_3::new(
            "eng",
            "English",
            LanguageType::Living,
            LanguageScope::Individual,
        );
        assert_eq!(code.code, "eng");
        assert_eq!(code.name, "English");
        assert_eq!(code.language_type, LanguageType::Living);
        assert_eq!(code.scope, LanguageScope::Individual);
    }
    #[test]
    fn test_iso639_3_to_iso639_1() {
        let eng = ISO639_3::new(
            "eng",
            "English",
            LanguageType::Living,
            LanguageScope::Individual,
        );
        assert_eq!(eng.to_iso639_1(), Some("en".to_string()));
        let jpn = ISO639_3::new(
            "jpn",
            "Japanese",
            LanguageType::Living,
            LanguageScope::Individual,
        );
        assert_eq!(jpn.to_iso639_1(), Some("ja".to_string()));
        let lat = ISO639_3::new(
            "lat",
            "Latin",
            LanguageType::Ancient,
            LanguageScope::Individual,
        );
        assert_eq!(lat.to_iso639_1(), Some("la".to_string()));
    }
    #[test]
    fn test_iso639_3_is_legal_language() {
        let eng = ISO639_3::new(
            "eng",
            "English",
            LanguageType::Living,
            LanguageScope::Individual,
        );
        assert!(eng.is_legal_language());
        let lat = ISO639_3::new(
            "lat",
            "Latin",
            LanguageType::Ancient,
            LanguageScope::Individual,
        );
        assert!(lat.is_legal_language());
        let swa = ISO639_3::new(
            "swa",
            "Swahili",
            LanguageType::Living,
            LanguageScope::Individual,
        );
        assert!(!swa.is_legal_language());
    }
    #[test]
    fn test_iso639_3_registry_defaults() {
        let registry = ISO639_3_Registry::with_defaults();
        assert!(registry.code_count() > 0);
        assert!(registry.get_code("eng").is_some());
        assert!(registry.get_code("jpn").is_some());
        assert!(registry.get_code("lat").is_some());
    }
    #[test]
    fn test_iso639_3_registry_legal_languages() {
        let registry = ISO639_3_Registry::with_defaults();
        let legal_langs = registry.get_legal_languages();
        assert!(!legal_langs.is_empty());
        assert!(legal_langs.iter().any(|l| l.code == "eng"));
        assert!(legal_langs.iter().any(|l| l.code == "fra"));
    }
    #[test]
    fn test_iso639_3_registry_historical_languages() {
        let registry = ISO639_3_Registry::with_defaults();
        let historical = registry.get_historical_languages();
        assert!(!historical.is_empty());
        assert!(historical.iter().any(|l| l.code == "lat"));
        assert!(historical.iter().any(|l| l.code == "ang"));
    }
    #[test]
    fn test_language_type_display() {
        assert_eq!(LanguageType::Living.to_string(), "Living");
        assert_eq!(LanguageType::Ancient.to_string(), "Ancient");
        assert_eq!(LanguageType::Historical.to_string(), "Historical");
    }
    #[test]
    fn test_language_scope_display() {
        assert_eq!(LanguageScope::Individual.to_string(), "Individual");
        assert_eq!(LanguageScope::Macrolanguage.to_string(), "Macrolanguage");
        assert_eq!(LanguageScope::Special.to_string(), "Special");
    }
    #[test]
    fn test_cldr_entry_creation() {
        let locale = Locale::new("en").with_country("US");
        let entry = CLDREntry::new(locale.clone(), CLDRFieldType::Languages, "ja", "Japanese");
        assert_eq!(entry.locale, locale);
        assert_eq!(entry.field_type, CLDRFieldType::Languages);
        assert_eq!(entry.key, "ja");
        assert_eq!(entry.value, "Japanese");
    }
    #[test]
    fn test_cldr_data_defaults() {
        let cldr = CLDRData::with_defaults();
        assert!(cldr.locale_count() > 0);
        assert!(cldr.entry_count() > 0);
    }
    #[test]
    fn test_cldr_data_get_value() {
        let cldr = CLDRData::with_defaults();
        let en_us = Locale::new("en").with_country("US");
        let value = cldr.get_value(&en_us, CLDRFieldType::Languages, "ja");
        assert_eq!(value, Some("Japanese".to_string()));
        let territory = cldr.get_value(&en_us, CLDRFieldType::Territories, "JP");
        assert_eq!(territory, Some("Japan".to_string()));
    }
    #[test]
    fn test_cldr_data_japanese_localization() {
        let cldr = CLDRData::with_defaults();
        let ja_jp = Locale::new("ja").with_country("JP");
        let value = cldr.get_value(&ja_jp, CLDRFieldType::Languages, "en");
        assert_eq!(value, Some("英語".to_string()));
        let territory = cldr.get_value(&ja_jp, CLDRFieldType::Territories, "US");
        assert_eq!(territory, Some("アメリカ合衆国".to_string()));
    }
    #[test]
    fn test_cldr_field_type_display() {
        assert_eq!(CLDRFieldType::Languages.to_string(), "Languages");
        assert_eq!(CLDRFieldType::Territories.to_string(), "Territories");
        assert_eq!(CLDRFieldType::TimeZones.to_string(), "Time Zones");
    }
    #[test]
    fn test_legal_extension_creation() {
        let ext = LegalExtension::legal_system("common");
        assert_eq!(ext.extension_type, LegalExtensionType::LegalSystem);
        assert_eq!(ext.value, "common");
    }
    #[test]
    fn test_legal_extension_to_bcp47() {
        let legal_system = LegalExtension::legal_system("common");
        assert_eq!(legal_system.to_bcp47_extension(), "u-legal-common");
        let cite_style = LegalExtension::citation_style("bluebook");
        assert_eq!(cite_style.to_bcp47_extension(), "u-cite-bluebook");
        let court = LegalExtension::court_type("supreme");
        assert_eq!(court.to_bcp47_extension(), "u-court-supreme");
        let formality = LegalExtension::formality_level("high");
        assert_eq!(formality.to_bcp47_extension(), "u-formality-high");
    }
    #[test]
    fn test_legal_extension_type_display() {
        assert_eq!(LegalExtensionType::LegalSystem.to_string(), "u-legal");
        assert_eq!(LegalExtensionType::CitationStyle.to_string(), "u-cite");
        assert_eq!(LegalExtensionType::CourtType.to_string(), "u-court");
        assert_eq!(
            LegalExtensionType::FormalityLevel.to_string(),
            "u-formality"
        );
    }
    #[test]
    fn test_w3c_compliance_valid_locale() {
        let locale = Locale::new("en").with_country("US");
        let checker = W3CComplianceChecker::new(locale);
        assert!(checker.has_valid_language_tag());
        assert!(checker.has_valid_country_code());
        assert_eq!(checker.get_text_direction(), "ltr");
    }
    #[test]
    fn test_w3c_compliance_rtl_locale() {
        let locale = Locale::new("ar").with_country("SA");
        let checker = W3CComplianceChecker::new(locale);
        assert!(checker.has_text_direction());
        assert_eq!(checker.get_text_direction(), "rtl");
    }
    #[test]
    fn test_w3c_compliance_html_attributes() {
        let locale = Locale::new("en").with_country("US");
        let checker = W3CComplianceChecker::new(locale);
        assert_eq!(checker.generate_html_lang_attribute(), "en-US");
        assert_eq!(checker.generate_html_dir_attribute(), "ltr");
    }
    #[test]
    fn test_w3c_compliance_report() {
        let locale = Locale::new("en").with_country("US");
        let checker = W3CComplianceChecker::new(locale);
        let report = checker.check_compliance();
        assert!(report.is_compliant);
        assert!(report.issues.is_empty());
        assert_eq!(report.lang_attribute, "en-US");
        assert_eq!(report.dir_attribute, "ltr");
    }
    #[test]
    fn test_w3c_compliance_report_summary() {
        let locale = Locale::new("en").with_country("US");
        let checker = W3CComplianceChecker::new(locale);
        let report = checker.check_compliance();
        let summary = report.summary();
        assert!(summary.contains("compliant"));
    }
    #[test]
    fn test_bcp47_creation() {
        let tag = BCP47LanguageTag::new("en");
        assert_eq!(tag.language, "en");
        assert!(tag.script.is_none());
        assert!(tag.region.is_none());
    }
    #[test]
    fn test_bcp47_with_script_and_region() {
        let tag = BCP47LanguageTag::new("zh")
            .with_script("Hans")
            .with_region("CN");
        assert_eq!(tag.language, "zh");
        assert_eq!(tag.script, Some("Hans".to_string()));
        assert_eq!(tag.region, Some("CN".to_string()));
        assert_eq!(tag.format_tag(), "zh-Hans-CN");
    }
    #[test]
    fn test_bcp47_with_variants() {
        let tag = BCP47LanguageTag::new("sl")
            .with_region("IT")
            .add_variant("nedis");
        assert_eq!(tag.format_tag(), "sl-IT-nedis");
    }
    #[test]
    fn test_bcp47_with_extensions() {
        let tag = BCP47LanguageTag::new("en")
            .with_region("US")
            .add_extension("u-ca-gregory");
        assert!(tag.format_tag().contains("u-ca-gregory"));
    }
    #[test]
    fn test_bcp47_with_private_use() {
        let tag = BCP47LanguageTag::new("en").add_private_use("legal");
        assert!(tag.format_tag().contains("x-legal"));
    }
    #[test]
    fn test_bcp47_parse_simple() {
        let tag = BCP47LanguageTag::parse("en-US").unwrap();
        assert_eq!(tag.language, "en");
        assert_eq!(tag.region, Some("US".to_string()));
    }
    #[test]
    fn test_bcp47_parse_with_script() {
        let tag = BCP47LanguageTag::parse("zh-Hans-CN").unwrap();
        assert_eq!(tag.language, "zh");
        assert_eq!(tag.script, Some("Hans".to_string()));
        assert_eq!(tag.region, Some("CN".to_string()));
    }
    #[test]
    fn test_bcp47_parse_invalid() {
        let result = BCP47LanguageTag::parse("x");
        assert!(result.is_err());
    }
    #[test]
    fn test_bcp47_to_locale() {
        let tag = BCP47LanguageTag::new("en")
            .with_script("Latn")
            .with_region("US");
        let locale = tag.to_locale();
        assert_eq!(locale.language, "en");
        assert_eq!(locale.script, Some("Latn".to_string()));
        assert_eq!(locale.country, Some("US".to_string()));
    }
    #[test]
    fn test_bcp47_from_locale() {
        let locale = Locale::new("ja").with_script("Jpan").with_country("JP");
        let tag = BCP47LanguageTag::from_locale(&locale);
        assert_eq!(tag.language, "ja");
        assert_eq!(tag.script, Some("Jpan".to_string()));
        assert_eq!(tag.region, Some("JP".to_string()));
    }
    #[test]
    fn test_bcp47_is_valid() {
        let valid = BCP47LanguageTag::new("en").with_region("US");
        assert!(valid.is_valid());
        let mut invalid = BCP47LanguageTag::new("x");
        assert!(!invalid.is_valid());
        invalid = BCP47LanguageTag::new("en");
        invalid.script = Some("AB".to_string());
        assert!(!invalid.is_valid());
    }
    #[test]
    fn test_bcp47_roundtrip() {
        let original = "en-Latn-US";
        let tag = BCP47LanguageTag::parse(original).unwrap();
        let reconstructed = tag.format_tag();
        assert_eq!(original, reconstructed);
    }
}
#[cfg(test)]
mod ai_translation_tests {
    use crate::*;
    #[test]
    fn test_llm_provider_display() {
        assert_eq!(LLMProvider::OpenAI.to_string(), "OpenAI");
        assert_eq!(LLMProvider::Anthropic.to_string(), "Anthropic");
        assert_eq!(LLMProvider::Google.to_string(), "Google");
    }
    #[test]
    fn test_legal_prompt_template_creation() {
        let template = LegalPromptTemplate::default_legal_translation();
        assert!(template.system_prompt.contains("legal translator"));
        assert!(template.include_legal_context);
        assert!(template.preserve_citations);
    }
    #[test]
    fn test_legal_prompt_template_render() {
        let template = LegalPromptTemplate::default_legal_translation();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");
        let rendered = template.render(
            "This is a contract.",
            &source,
            &target,
            Some("contract_law"),
        );
        assert!(rendered.contains("This is a contract."));
        assert!(rendered.contains("en-US"));
        assert!(rendered.contains("fr-FR"));
        assert!(rendered.contains("contract_law"));
    }
    #[test]
    fn test_llm_translator_creation() {
        let translator = LLMTranslator::new(LLMProvider::OpenAI, "gpt-4");
        assert_eq!(translator.provider, LLMProvider::OpenAI);
        assert_eq!(translator.model_name, "gpt-4");
        assert_eq!(translator.max_tokens, 2000);
        assert_eq!(translator.temperature, 0.3);
    }
    #[test]
    fn test_llm_translator_openai() {
        let translator = LLMTranslator::openai_gpt4();
        assert_eq!(translator.provider, LLMProvider::OpenAI);
        assert_eq!(translator.model_name, "gpt-4");
    }
    #[test]
    fn test_llm_translator_anthropic() {
        let translator = LLMTranslator::anthropic_claude();
        assert_eq!(translator.provider, LLMProvider::Anthropic);
        assert!(translator.model_name.contains("claude"));
    }
    #[test]
    fn test_llm_translator_generate_prompt() {
        let translator = LLMTranslator::openai_gpt4();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("ja").with_country("JP");
        let prompt =
            translator.generate_prompt("Contract law", &source, &target, Some("civil_law"));
        assert!(prompt.contains("Contract law"));
        assert!(prompt.contains("en-US"));
        assert!(prompt.contains("ja-JP"));
    }
    #[test]
    fn test_disambiguation_type_display() {
        assert_eq!(DisambiguationType::LegalDomain.to_string(), "Legal Domain");
        assert_eq!(DisambiguationType::Jurisdiction.to_string(), "Jurisdiction");
    }
    #[test]
    fn test_disambiguation_context_creation() {
        let context =
            DisambiguationContext::new(DisambiguationType::LegalDomain, "criminal_law", 0.9)
                .with_explanation("Criminal law context");
        assert_eq!(context.disambiguation_type, DisambiguationType::LegalDomain);
        assert_eq!(context.value, "criminal_law");
        assert_eq!(context.confidence, 0.9);
        assert!(context.explanation.is_some());
    }
    #[test]
    fn test_context_disambiguator_defaults() {
        let disambiguator = ContextDisambiguator::with_defaults();
        assert!(disambiguator.term_count() > 0);
        assert!(disambiguator.context_count() > 0);
    }
    #[test]
    fn test_context_disambiguator_get_contexts() {
        let disambiguator = ContextDisambiguator::with_defaults();
        let contexts = disambiguator.get_contexts("action");
        assert!(!contexts.is_empty());
    }
    #[test]
    fn test_context_disambiguator_best_context() {
        let disambiguator = ContextDisambiguator::with_defaults();
        let best = disambiguator.get_best_context("consideration", DisambiguationType::LegalDomain);
        assert!(best.is_some());
        assert_eq!(best.unwrap().value, "contract_law");
    }
    #[test]
    fn test_style_attribute_display() {
        assert_eq!(StyleAttribute::Formality.to_string(), "Formality");
        assert_eq!(StyleAttribute::Tone.to_string(), "Tone");
        assert_eq!(StyleAttribute::Voice.to_string(), "Voice");
    }
    #[test]
    fn test_style_profile_formal_legal() {
        let profile = StyleProfile::formal_legal();
        assert_eq!(
            profile.get_attribute(StyleAttribute::Formality),
            Some(&"formal".to_string())
        );
        assert_eq!(
            profile.get_attribute(StyleAttribute::Tone),
            Some(&"professional".to_string())
        );
        assert_eq!(
            profile.get_attribute(StyleAttribute::Voice),
            Some(&"passive".to_string())
        );
    }
    #[test]
    fn test_style_profile_informal_legal() {
        let profile = StyleProfile::informal_legal();
        assert_eq!(
            profile.get_attribute(StyleAttribute::Formality),
            Some(&"informal".to_string())
        );
        assert_eq!(
            profile.get_attribute(StyleAttribute::Tone),
            Some(&"conversational".to_string())
        );
    }
    #[test]
    fn test_style_profile_locale_preference() {
        let mut profile = StyleProfile::new();
        let ja_jp = Locale::new("ja").with_country("JP");
        profile.set_locale_preference(ja_jp.clone(), StyleAttribute::Voice, "passive");
        let voice = profile.get_attribute_for_locale(&ja_jp, StyleAttribute::Voice);
        assert_eq!(voice, Some(&"passive".to_string()));
    }
    #[test]
    fn test_style_preserving_translator() {
        let profile = StyleProfile::formal_legal();
        let target = Locale::new("fr").with_country("FR");
        let translator = StylePreservingTranslator::new(profile, target);
        assert!(!translator.adapt_to_target);
    }
    #[test]
    fn test_style_preserving_translator_instructions() {
        let profile = StyleProfile::formal_legal();
        let target = Locale::new("en").with_country("US");
        let translator = StylePreservingTranslator::new(profile, target);
        let instructions = translator.generate_style_instructions();
        assert!(instructions.contains("formal") || instructions.contains("professional"));
    }
    #[test]
    fn test_quality_metric_display() {
        assert_eq!(
            QualityMetric::SemanticAccuracy.to_string(),
            "Semantic Accuracy"
        );
        assert_eq!(
            QualityMetric::TerminologicalConsistency.to_string(),
            "Terminological Consistency"
        );
    }
    #[test]
    fn test_ai_quality_score_creation() {
        let score = AIQualityScore::new(QualityMetric::SemanticAccuracy, 0.85)
            .with_explanation("High semantic accuracy");
        assert_eq!(score.metric, QualityMetric::SemanticAccuracy);
        assert_eq!(score.score, 0.85);
        assert!(score.explanation.is_some());
    }
    #[test]
    fn test_quality_estimation_report_creation() {
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");
        let mut report = QualityEstimationReport::new(
            "This is a contract",
            "Ceci est un contrat",
            source,
            target,
        );
        report.add_score(AIQualityScore::new(QualityMetric::SemanticAccuracy, 0.9));
        report.add_score(AIQualityScore::new(QualityMetric::Fluency, 0.85));
        assert!(report.overall_score > 0.0);
        assert_eq!(report.metric_scores.len(), 2);
    }
    #[test]
    fn test_quality_estimation_report_quality_level() {
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");
        let mut report = QualityEstimationReport::new("Source", "Target", source, target);
        report.add_score(AIQualityScore::new(QualityMetric::SemanticAccuracy, 0.95));
        assert_eq!(report.get_quality_level(), "Excellent");
    }
    #[test]
    fn test_quality_estimation_report_threshold() {
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");
        let mut report = QualityEstimationReport::new("Source", "Target", source, target);
        report.add_score(AIQualityScore::new(QualityMetric::SemanticAccuracy, 0.8));
        assert!(report.meets_threshold(0.7));
        assert!(!report.meets_threshold(0.9));
    }
    #[test]
    fn test_quality_estimator_creation() {
        let estimator = QualityEstimator::new(0.75);
        assert_eq!(estimator.min_threshold, 0.75);
    }
    #[test]
    fn test_quality_estimator_defaults() {
        let estimator = QualityEstimator::with_defaults();
        assert_eq!(estimator.min_threshold, 0.7);
    }
    #[test]
    fn test_quality_estimator_estimate() {
        let estimator = QualityEstimator::with_defaults();
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");
        let report = estimator.estimate_quality(
            "This is a legal contract.",
            "Ceci est un contrat juridique.",
            source,
            target,
        );
        assert!(report.overall_score > 0.0);
        assert!(!report.metric_scores.is_empty());
    }
    #[test]
    fn test_quality_estimator_is_acceptable() {
        let estimator = QualityEstimator::new(0.6);
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");
        let report = estimator.estimate_quality(
            "This is a contract.",
            "Ceci est un contrat.",
            source,
            target,
        );
        assert!(estimator.is_acceptable(&report));
    }
}
#[cfg(test)]
mod cultural_tests {
    use crate::*;
    #[test]
    fn test_cultural_context_creation() {
        let locale = Locale::new("ja").with_country("JP");
        let context = CulturalContext::new(
            locale,
            ContextCategory::SocialHierarchy,
            "keigo",
            "Honorific language system",
        );
        assert_eq!(context.term, "keigo");
        assert_eq!(context.category, ContextCategory::SocialHierarchy);
        assert!(context.guidelines.is_empty());
    }
    #[test]
    fn test_cultural_context_with_guidelines() {
        let locale = Locale::new("ja").with_country("JP");
        let context = CulturalContext::new(
            locale,
            ContextCategory::BusinessEtiquette,
            "hanko",
            "Personal seal",
        )
        .with_guideline("Required for contracts")
        .with_equivalent("en-US", "signature");
        assert_eq!(context.guidelines.len(), 1);
        assert_eq!(context.cross_cultural_equivalents.len(), 1);
    }
    #[test]
    fn test_cultural_context_registry() {
        let registry = CulturalContextRegistry::with_defaults();
        let ja_jp = Locale::new("ja").with_country("JP");
        let contexts = registry.get_contexts(&ja_jp);
        assert!(!contexts.is_empty());
        let keigo = registry.find_term(&ja_jp, "keigo");
        assert!(keigo.is_some());
        assert_eq!(keigo.unwrap().term, "keigo");
    }
    #[test]
    fn test_cultural_context_by_category() {
        let registry = CulturalContextRegistry::with_defaults();
        let ja_jp = Locale::new("ja").with_country("JP");
        let hierarchy_contexts =
            registry.get_by_category(&ja_jp, &ContextCategory::SocialHierarchy);
        assert!(!hierarchy_contexts.is_empty());
    }
    #[test]
    fn test_context_category_display() {
        assert_eq!(
            ContextCategory::SocialHierarchy.to_string(),
            "Social Hierarchy"
        );
        assert_eq!(
            ContextCategory::ReligiousPractice.to_string(),
            "Religious Practice"
        );
    }
    #[test]
    fn test_local_custom_creation() {
        let locale = Locale::new("ja").with_country("JP");
        let custom = LocalCustom::new(
            "Miai marriage",
            "Japan",
            locale,
            CustomType::Marriage,
            "Traditional arranged marriage introduction",
        )
        .with_recognition_level(0.3);
        assert_eq!(custom.name, "Miai marriage");
        assert_eq!(custom.recognition_level, 0.3);
    }
    #[test]
    fn test_local_custom_registry() {
        let registry = LocalCustomRegistry::with_defaults();
        let japan_customs = registry.get_customs("Japan");
        assert!(!japan_customs.is_empty());
        let miai = registry.find_custom("Japan", "Miai marriage");
        assert!(miai.is_some());
    }
    #[test]
    fn test_local_custom_by_type() {
        let registry = LocalCustomRegistry::with_defaults();
        let marriage_customs = registry.get_by_type("Saudi Arabia", &CustomType::Marriage);
        assert!(!marriage_customs.is_empty());
    }
    #[test]
    fn test_custom_type_display() {
        assert_eq!(CustomType::Marriage.to_string(), "Marriage");
        assert_eq!(
            CustomType::DisputeResolution.to_string(),
            "Dispute Resolution"
        );
    }
    #[test]
    fn test_religious_law_islamic() {
        let islamic = ReligiousLawSystem::islamic();
        assert_eq!(islamic.law_type, ReligiousLawType::Islamic);
        assert!(!islamic.principles.is_empty());
        assert!(!islamic.sources.is_empty());
        assert!(islamic.civil_equivalents.contains_key("mahr"));
    }
    #[test]
    fn test_religious_law_jewish() {
        let jewish = ReligiousLawSystem::jewish();
        assert_eq!(jewish.law_type, ReligiousLawType::Jewish);
        assert!(jewish.civil_equivalents.contains_key("get"));
    }
    #[test]
    fn test_religious_law_registry() {
        let registry = ReligiousLawRegistry::with_defaults();
        let islamic = registry.get_system(ReligiousLawType::Islamic);
        assert!(islamic.is_some());
        let sa_systems = registry.get_by_jurisdiction("Saudi Arabia");
        assert!(!sa_systems.is_empty());
    }
    #[test]
    fn test_religious_law_type_display() {
        assert_eq!(
            ReligiousLawType::Islamic.to_string(),
            "Islamic Law (Sharia)"
        );
        assert_eq!(ReligiousLawType::Jewish.to_string(), "Jewish Law (Halakha)");
    }
    #[test]
    fn test_indigenous_law_creation() {
        let system = IndigenousLawSystem::new("Navajo Nation", "Southwestern United States")
            .with_principle("Hózhǫ́ (harmony)")
            .with_dispute_resolution("Peacemaking circles")
            .with_property_concept("Communal land ownership")
            .with_state_recognition(true);
        assert_eq!(system.people_name, "Navajo Nation");
        assert!(system.state_recognition);
        assert!(!system.principles.is_empty());
    }
    #[test]
    fn test_indigenous_law_registry() {
        let registry = IndigenousLawRegistry::with_defaults();
        let navajo = registry.get_system("Navajo Nation");
        assert!(navajo.is_some());
        let recognized = registry.get_recognized();
        assert_eq!(recognized.len(), 4);
    }
    #[test]
    fn test_indigenous_law_by_region() {
        let registry = IndigenousLawRegistry::with_defaults();
        let nz_systems = registry.get_by_region("New Zealand");
        assert!(!nz_systems.is_empty());
    }
    #[test]
    fn test_colonial_legacy_creation() {
        let legacy = ColonialLegacy::new(ColonialPower::British, "India")
            .with_retained_concept("Common law")
            .with_hybrid_concept("Anglo-Hindu law", "Hindu personal law")
            .with_reform("Constitution of India 1950");
        assert_eq!(legacy.colonial_power, ColonialPower::British);
        assert_eq!(legacy.jurisdiction, "India");
        assert!(!legacy.retained_concepts.is_empty());
        assert!(!legacy.hybrid_concepts.is_empty());
    }
    #[test]
    fn test_colonial_legacy_mapper() {
        let mapper = ColonialLegacyMapper::with_defaults();
        let india = mapper.get_legacy("India");
        assert!(india.is_some());
        assert_eq!(india.unwrap().colonial_power, ColonialPower::British);
        let british_legacies = mapper.get_by_colonial_power(ColonialPower::British);
        assert!(!british_legacies.is_empty());
    }
    #[test]
    fn test_colonial_power_display() {
        assert_eq!(ColonialPower::British.to_string(), "British");
        assert_eq!(ColonialPower::French.to_string(), "French");
    }
    #[test]
    fn test_registry_counts() {
        let cultural_registry = CulturalContextRegistry::with_defaults();
        assert!(cultural_registry.context_count() > 0);
        assert!(cultural_registry.locale_count() > 0);
        let custom_registry = LocalCustomRegistry::with_defaults();
        assert!(custom_registry.custom_count() > 0);
        assert!(custom_registry.region_count() > 0);
        let religious_registry = ReligiousLawRegistry::with_defaults();
        assert_eq!(religious_registry.system_count(), 3);
        let indigenous_registry = IndigenousLawRegistry::with_defaults();
        assert_eq!(indigenous_registry.system_count(), 4);
        let colonial_mapper = ColonialLegacyMapper::with_defaults();
        assert_eq!(colonial_mapper.legacy_count(), 7);
    }
}
#[cfg(test)]
mod accessibility_tests {
    use crate::*;
    #[test]
    fn test_plain_language_generator() {
        let locale = Locale::new("en").with_country("US");
        let generator = PlainLanguageGenerator::new(8.0, locale);
        let legal_text = "The party hereinafter referred to as the Plaintiff shall forthwith commence proceedings pursuant to the aforementioned statute.";
        let simplified = generator.simplify(legal_text);
        assert!(simplified.contains("from now on"));
        assert!(simplified.contains("immediately"));
        assert!(simplified.contains("start"));
    }
    #[test]
    fn test_plain_language_custom_jargon() {
        let locale = Locale::new("en").with_country("US");
        let generator = PlainLanguageGenerator::new(8.0, locale)
            .add_jargon_replacement("consideration", "payment");
        let text = "Valid consideration is required.";
        let simplified = generator.simplify(text);
        assert!(simplified.contains("payment"));
    }
    #[test]
    fn test_plain_language_meets_target() {
        let locale = Locale::new("en").with_country("US");
        let generator = PlainLanguageGenerator::new(8.0, locale);
        let simple_text = "The party must pay now.";
        assert!(generator.meets_target(simple_text));
    }
    #[test]
    fn test_simplification_strategy_display() {
        assert_eq!(
            SimplificationStrategy::ReplaceJargon.to_string(),
            "Replace Jargon"
        );
        assert_eq!(
            SimplificationStrategy::ShortenSentences.to_string(),
            "Shorten Sentences"
        );
        assert_eq!(
            SimplificationStrategy::ActiveVoice.to_string(),
            "Active Voice"
        );
    }
    #[test]
    fn test_reading_level_adjuster() {
        let locale = Locale::new("en").with_country("US");
        let adjuster = ReadingLevelAdjuster::new(TargetReadingLevel::MiddleSchool, locale);
        let legal_text = "The party shall forthwith pay consideration.";
        let adjusted = adjuster.adjust(legal_text);
        assert_eq!(adjusted.original, legal_text);
        assert!(adjusted.iterations > 0);
        assert_ne!(adjusted.adjusted, adjusted.original);
    }
    #[test]
    fn test_reading_level_improvement() {
        let locale = Locale::new("en").with_country("US");
        let adjuster = ReadingLevelAdjuster::new(TargetReadingLevel::MiddleSchool, locale);
        let legal_text = "The party hereinafter shall forthwith commence.";
        let adjusted = adjuster.adjust(legal_text);
        let improvement = adjusted.improvement();
        assert!(improvement >= 0.0);
    }
    #[test]
    fn test_target_reading_level_display() {
        assert_eq!(
            TargetReadingLevel::Elementary.to_string(),
            "Elementary (grades 3-5)"
        );
        assert_eq!(
            TargetReadingLevel::MiddleSchool.to_string(),
            "Middle School (grades 6-8)"
        );
        assert_eq!(
            TargetReadingLevel::HighSchool.to_string(),
            "High School (grades 9-12)"
        );
    }
    #[test]
    fn test_target_reading_level_grade() {
        assert_eq!(TargetReadingLevel::Elementary.grade_level(), 4.0);
        assert_eq!(TargetReadingLevel::MiddleSchool.grade_level(), 7.0);
        assert_eq!(TargetReadingLevel::College.grade_level(), 14.0);
    }
    #[test]
    fn test_screen_reader_optimizer() {
        let locale = Locale::new("en").with_country("US");
        let optimizer = ScreenReaderOptimizer::new(WCAGLevel::AA, locale);
        let html = "<nav>Menu</nav><main>Content</main>";
        let optimized = optimizer.optimize_html(html);
        assert!(optimized.contains("role=\"navigation\""));
        assert!(optimized.contains("role=\"main\""));
        assert!(optimized.contains("lang=\"en\""));
    }
    #[test]
    fn test_screen_reader_skip_links() {
        let locale = Locale::new("en").with_country("US");
        let optimizer = ScreenReaderOptimizer::new(WCAGLevel::AA, locale);
        let html = "<main>Content</main>";
        let optimized = optimizer.optimize_html(html);
        assert!(optimized.contains("Skip to main content"));
        assert!(optimized.contains("skip-link"));
    }
    #[test]
    fn test_screen_reader_skip_links_locale() {
        let locale = Locale::new("ja").with_country("JP");
        let optimizer = ScreenReaderOptimizer::new(WCAGLevel::AA, locale);
        let html = "<main>Content</main>";
        let optimized = optimizer.optimize_html(html);
        assert!(optimized.contains("メインコンテンツへスキップ"));
    }
    #[test]
    fn test_screen_reader_document_structure() {
        let locale = Locale::new("en").with_country("US");
        let optimizer = ScreenReaderOptimizer::new(WCAGLevel::AA, locale);
        let sections = vec![
            ("Introduction", "This is the introduction."),
            ("Terms", "These are the terms."),
        ];
        let html = optimizer.generate_document_structure("Legal Agreement", sections);
        assert!(html.contains("<h1>Legal Agreement</h1>"));
        assert!(html.contains("<h2>Introduction</h2>"));
        assert!(html.contains("<h2>Terms</h2>"));
        assert!(html.contains("lang=\"en\""));
    }
    #[test]
    fn test_screen_reader_compliance_check() {
        let locale = Locale::new("en").with_country("US");
        let optimizer = ScreenReaderOptimizer::new(WCAGLevel::AA, locale);
        let good_html = "<html lang=\"en\"><h1>Title</h1><main role=\"main\"><a href=\"#main\" class=\"skip-link\">Skip</a></main></html>";
        let report = optimizer.check_compliance(good_html);
        assert!(report.is_compliant);
        assert_eq!(report.issues.len(), 0);
    }
    #[test]
    fn test_screen_reader_compliance_issues() {
        let locale = Locale::new("en").with_country("US");
        let optimizer = ScreenReaderOptimizer::new(WCAGLevel::AA, locale);
        let bad_html = "<div>Content</div>";
        let report = optimizer.check_compliance(bad_html);
        assert!(!report.is_compliant);
        assert!(!report.issues.is_empty());
    }
    #[test]
    fn test_wcag_level_display() {
        assert_eq!(WCAGLevel::A.to_string(), "WCAG Level A");
        assert_eq!(WCAGLevel::AA.to_string(), "WCAG Level AA");
        assert_eq!(WCAGLevel::AAA.to_string(), "WCAG Level AAA");
    }
    #[test]
    fn test_audio_narration_ssml() {
        let locale = Locale::new("en").with_country("US");
        let narration = AudioNarrationSupport::new(locale);
        let text = "The party shall pay.";
        let ssml = narration.generate_ssml(text);
        assert!(ssml.contains("<speak"));
        assert!(ssml.contains("xml:lang=\"en-US\""));
        assert!(ssml.contains("<prosody"));
        assert!(ssml.contains("</speak>"));
    }
    #[test]
    fn test_audio_narration_legal_text() {
        let locale = Locale::new("en").with_country("US");
        let narration = AudioNarrationSupport::new(locale);
        let text = "The contract shall be valid.";
        let ssml = narration.generate_ssml(text);
        assert!(ssml.contains("<emphasis level=\"strong\">shall</emphasis>"));
    }
    #[test]
    fn test_audio_narration_section() {
        let locale = Locale::new("en").with_country("US");
        let narration = AudioNarrationSupport::new(locale);
        let ssml = narration.narrate_section("1", "Definitions", "Terms are defined here.");
        assert!(ssml.contains("Section") || ssml.contains("ordinal"));
        assert!(ssml.contains("Definitions"));
        assert!(ssml.contains("<break time=\"500ms\"/>"));
    }
    #[test]
    fn test_audio_narration_citation() {
        let locale = Locale::new("en").with_country("US");
        let narration = AudioNarrationSupport::new(locale);
        let ssml = narration.narrate_citation("Brown v. Board of Education, 347 U.S. 483 (1954)");
        assert!(ssml.contains("versus"));
        assert!(ssml.contains("United States"));
    }
    #[test]
    fn test_audio_narration_with_settings() {
        let locale = Locale::new("en").with_country("US");
        let narration = AudioNarrationSupport::new(locale)
            .with_speaking_rate(1.2)
            .with_pitch(1.1)
            .with_volume(0.9);
        let ssml = narration.generate_ssml("Test");
        assert!(ssml.contains("<prosody"));
    }
    #[test]
    fn test_emphasis_level_display() {
        assert_eq!(EmphasisLevel::None.to_string(), "none");
        assert_eq!(EmphasisLevel::Reduced.to_string(), "reduced");
        assert_eq!(EmphasisLevel::Moderate.to_string(), "moderate");
        assert_eq!(EmphasisLevel::Strong.to_string(), "strong");
    }
    #[test]
    fn test_sign_language_reference() {
        let locale = Locale::new("en").with_country("US");
        let reference = SignLanguageReference::new("contract", SignLanguageType::ASL, locale)
            .with_video("https://example.com/contract.mp4")
            .with_image("https://example.com/contract.jpg")
            .with_description("Hands form C-shape");
        assert_eq!(reference.term, "contract");
        assert_eq!(reference.sign_language, SignLanguageType::ASL);
        assert!(reference.video_url.is_some());
        assert!(reference.image_url.is_some());
        assert!(reference.description.is_some());
    }
    #[test]
    fn test_sign_language_referencer() {
        let referencer = SignLanguageReferencer::with_defaults();
        assert!(referencer.term_count() > 0);
        assert!(referencer.reference_count() > 0);
    }
    #[test]
    fn test_sign_language_get_references() {
        let referencer = SignLanguageReferencer::with_defaults();
        let refs = referencer.get_references("contract");
        assert!(!refs.is_empty());
    }
    #[test]
    fn test_sign_language_by_type() {
        let referencer = SignLanguageReferencer::with_defaults();
        let asl_refs =
            referencer.get_references_for_sign_language("contract", SignLanguageType::ASL);
        assert!(!asl_refs.is_empty());
        let bsl_refs =
            referencer.get_references_for_sign_language("solicitor", SignLanguageType::BSL);
        assert!(!bsl_refs.is_empty());
    }
    #[test]
    fn test_sign_language_html_generation() {
        let mut referencer = SignLanguageReferencer::new();
        referencer.add_reference(
            SignLanguageReference::new(
                "law",
                SignLanguageType::ASL,
                Locale::new("en").with_country("US"),
            )
            .with_video("https://example.com/law.mp4"),
        );
        let html = referencer.generate_accessible_html("This is about law.");
        assert!(html.contains("sign-language-link"));
        assert!(html.contains("aria-label"));
    }
    #[test]
    fn test_sign_language_type_display() {
        assert_eq!(
            SignLanguageType::ASL.to_string(),
            "American Sign Language (ASL)"
        );
        assert_eq!(
            SignLanguageType::BSL.to_string(),
            "British Sign Language (BSL)"
        );
        assert_eq!(
            SignLanguageType::JSL.to_string(),
            "Japanese Sign Language (JSL)"
        );
        assert_eq!(SignLanguageType::IS.to_string(), "International Sign (IS)");
    }
    #[test]
    fn test_sign_language_add_custom() {
        let mut referencer = SignLanguageReferencer::new();
        let locale = Locale::new("en").with_country("US");
        referencer.add_reference(
            SignLanguageReference::new("judge", SignLanguageType::ASL, locale)
                .with_description("Gavel motion"),
        );
        assert_eq!(referencer.term_count(), 1);
        assert_eq!(referencer.reference_count(), 1);
    }
}
#[cfg(test)]
mod historical_tests {
    use crate::*;
    #[test]
    fn test_archaic_term_creation() {
        let term = ArchaicTerm::new(
            "wergild",
            HistoricalPeriod::OldEnglish,
            "blood money",
            "Compensation paid to slain person's family",
            Locale::new("en").with_country("GB"),
        )
        .with_example("Wergild was 1200 shillings");
        assert_eq!(term.term, "wergild");
        assert_eq!(term.period, HistoricalPeriod::OldEnglish);
        assert_eq!(term.modern_equivalent, "blood money");
        assert!(term.example.is_some());
    }
    #[test]
    fn test_archaic_dictionary_defaults() {
        let dict = ArchaicTermDictionary::with_defaults();
        assert!(dict.term_count() > 0);
        assert!(dict.period_count() > 0);
    }
    #[test]
    fn test_archaic_dictionary_by_period() {
        let dict = ArchaicTermDictionary::with_defaults();
        let old_english_terms = dict.get_by_period(HistoricalPeriod::OldEnglish);
        assert!(!old_english_terms.is_empty());
        let latin_terms = dict.get_by_period(HistoricalPeriod::ClassicalLatin);
        assert!(!latin_terms.is_empty());
    }
    #[test]
    fn test_archaic_dictionary_translate() {
        let dict = ArchaicTermDictionary::with_defaults();
        let modern = dict.translate_to_modern("wergild");
        assert_eq!(modern, Some("blood money".to_string()));
        let modern = dict.translate_to_modern("feoffment");
        assert_eq!(modern, Some("grant of land".to_string()));
    }
    #[test]
    fn test_archaic_dictionary_by_name() {
        let dict = ArchaicTermDictionary::with_defaults();
        let terms = dict.get_by_name("moot");
        assert!(!terms.is_empty());
        assert_eq!(terms[0].modern_equivalent, "assembly");
    }
    #[test]
    fn test_historical_period_display() {
        assert_eq!(
            HistoricalPeriod::OldEnglish.to_string(),
            "Old English (450-1150)"
        );
        assert_eq!(
            HistoricalPeriod::MiddleEnglish.to_string(),
            "Middle English (1150-1500)"
        );
        assert_eq!(
            HistoricalPeriod::ClassicalLatin.to_string(),
            "Classical Latin (Roman Empire)"
        );
    }
    #[test]
    fn test_historical_calendar_display() {
        assert_eq!(HistoricalCalendar::Julian.to_string(), "Julian Calendar");
        assert_eq!(
            HistoricalCalendar::Gregorian.to_string(),
            "Gregorian Calendar"
        );
        assert_eq!(
            HistoricalCalendar::FrenchRevolutionary.to_string(),
            "French Revolutionary Calendar"
        );
    }
    #[test]
    fn test_julian_to_gregorian_conversion() {
        let converter = HistoricalCalendarConverter::new(HistoricalCalendar::Julian);
        let (year, month, day) = converter.julian_to_gregorian(1582, 10, 5);
        assert_eq!(year, 1582);
        assert_eq!(month, 10);
        assert!(day >= 5);
    }
    #[test]
    fn test_gregorian_to_julian_conversion() {
        let converter = HistoricalCalendarConverter::new(HistoricalCalendar::Gregorian);
        let (year, month, day) = converter.gregorian_to_julian(1700, 1, 1);
        assert!((1699..=1700).contains(&year));
        assert!((1..=12).contains(&month));
        assert!((1..=31).contains(&day));
    }
    #[test]
    fn test_julian_gregorian_offset() {
        let converter = HistoricalCalendarConverter::new(HistoricalCalendar::Julian);
        let offset_before = converter.julian_gregorian_offset(1500);
        assert_eq!(offset_before, 0);
        let offset_after = converter.julian_gregorian_offset(1700);
        assert!(offset_after > 0);
    }
    #[test]
    fn test_format_historical_date_julian() {
        let converter = HistoricalCalendarConverter::new(HistoricalCalendar::Julian);
        let formatted = converter.format_historical_date(1215, 6, 15);
        assert!(formatted.contains("15"));
        assert!(formatted.contains("1215"));
        assert!(formatted.contains("(O.S.)"));
    }
    #[test]
    fn test_format_historical_date_gregorian() {
        let converter = HistoricalCalendarConverter::new(HistoricalCalendar::Gregorian);
        let formatted = converter.format_historical_date(1789, 7, 14);
        assert!(formatted.contains("14"));
        assert!(formatted.contains("1789"));
        assert!(formatted.contains("(N.S.)"));
    }
    #[test]
    fn test_format_french_revolutionary_date() {
        let converter = HistoricalCalendarConverter::new(HistoricalCalendar::FrenchRevolutionary);
        let formatted = converter.format_french_revolutionary_date(2, 11, 9);
        assert!(formatted.contains("9"));
        assert!(formatted.contains("Thermidor"));
        assert!(formatted.contains("An 2"));
    }
    #[test]
    fn test_language_family_display() {
        assert_eq!(LanguageFamily::Germanic.to_string(), "Germanic");
        assert_eq!(LanguageFamily::Latin.to_string(), "Latin");
        assert_eq!(LanguageFamily::NormanFrench.to_string(), "Norman French");
    }
    #[test]
    fn test_etymology_creation() {
        let etymology = Etymology::new(
            "contract",
            "contractus",
            LanguageFamily::Latin,
            "Latin",
            "drawn together",
        )
        .with_first_usage(HistoricalPeriod::ClassicalLatin)
        .add_evolution("Latin → Old French → Middle English");
        assert_eq!(etymology.term, "contract");
        assert_eq!(etymology.root, "contractus");
        assert_eq!(etymology.language_family, LanguageFamily::Latin);
        assert!(etymology.first_usage.is_some());
        assert_eq!(etymology.evolution.len(), 1);
    }
    #[test]
    fn test_etymology_tracker_defaults() {
        let tracker = EtymologyTracker::with_defaults();
        assert!(tracker.etymology_count() > 0);
    }
    #[test]
    fn test_etymology_tracker_get() {
        let tracker = EtymologyTracker::with_defaults();
        let etymology = tracker.get_etymology("contract");
        assert!(etymology.is_some());
        assert_eq!(etymology.unwrap().root, "contractus");
    }
    #[test]
    fn test_etymology_by_language_family() {
        let tracker = EtymologyTracker::with_defaults();
        let latin_etymologies = tracker.get_by_language_family(LanguageFamily::Latin);
        assert!(!latin_etymologies.is_empty());
        let french_etymologies = tracker.get_by_language_family(LanguageFamily::OldFrench);
        assert!(!french_etymologies.is_empty());
    }
    #[test]
    fn test_historical_context_creation() {
        let context = HistoricalContext::new(
            "Magna Carta",
            HistoricalPeriod::MiddleEnglish,
            "Signed in 1215",
            "Established rule of law",
        )
        .with_modern_relevance("Foundation of constitutional law")
        .add_related_document("Bill of Rights 1689");
        assert_eq!(context.term, "Magna Carta");
        assert_eq!(context.period, HistoricalPeriod::MiddleEnglish);
        assert!(context.modern_relevance.is_some());
        assert_eq!(context.related_documents.len(), 1);
    }
    #[test]
    fn test_historical_context_annotator_defaults() {
        let annotator = HistoricalContextAnnotator::with_defaults();
        assert!(annotator.context_count() > 0);
    }
    #[test]
    fn test_historical_context_get() {
        let annotator = HistoricalContextAnnotator::with_defaults();
        let contexts = annotator.get_contexts("Magna Carta");
        assert!(!contexts.is_empty());
        assert!(contexts[0].modern_relevance.is_some());
    }
    #[test]
    fn test_historical_context_by_period() {
        let annotator = HistoricalContextAnnotator::with_defaults();
        let middle_english = annotator.get_by_period(HistoricalPeriod::MiddleEnglish);
        assert!(!middle_english.is_empty());
        let enlightenment = annotator.get_by_period(HistoricalPeriod::Enlightenment);
        assert!(!enlightenment.is_empty());
    }
    #[test]
    fn test_archaic_dictionary_add_custom() {
        let mut dict = ArchaicTermDictionary::new();
        dict.add_term(ArchaicTerm::new(
            "gavelkind",
            HistoricalPeriod::MiddleEnglish,
            "equal inheritance",
            "System of land inheritance divided equally among sons",
            Locale::new("en").with_country("GB"),
        ));
        assert_eq!(dict.term_count(), 1);
        assert!(dict.translate_to_modern("gavelkind").is_some());
    }
    #[test]
    fn test_etymology_tracker_add_custom() {
        let mut tracker = EtymologyTracker::new();
        tracker.add_etymology(Etymology::new(
            "judge",
            "iudex",
            LanguageFamily::Latin,
            "Latin",
            "one who declares law",
        ));
        assert_eq!(tracker.etymology_count(), 1);
        assert!(tracker.get_etymology("judge").is_some());
    }
    #[test]
    fn test_historical_context_add_custom() {
        let mut annotator = HistoricalContextAnnotator::new();
        annotator.add_context(HistoricalContext::new(
            "Common Law",
            HistoricalPeriod::MiddleEnglish,
            "Developed in medieval England",
            "Based on judicial precedent",
        ));
        assert_eq!(annotator.context_count(), 1);
        assert!(!annotator.get_contexts("Common Law").is_empty());
    }
}
