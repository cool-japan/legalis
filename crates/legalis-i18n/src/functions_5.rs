//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[cfg(test)]
mod emerging_markets_tests {
    use crate::*;
    #[test]
    fn test_extended_language_creation() {
        let lang = ExtendedLanguage::new("sw", "Swahili", "Kiswahili", "Bantu", "Latin")
            .with_speakers(200.0)
            .add_official_country("TZ")
            .add_official_country("KE")
            .as_low_resource();
        assert_eq!(lang.code, "sw");
        assert_eq!(lang.name, "Swahili");
        assert_eq!(lang.native_name, "Kiswahili");
        assert_eq!(lang.family, "Bantu");
        assert_eq!(lang.script, "Latin");
        assert!(lang.low_resource);
        assert_eq!(lang.speakers_millions, 200.0);
        assert_eq!(lang.official_in.len(), 2);
    }
    #[test]
    fn test_extended_language_registry_defaults() {
        let registry = ExtendedLanguageRegistry::with_extended_set();
        assert!(registry.language_count() >= 60);
        assert!(registry.get_language("en").is_some());
        assert!(registry.get_language("sw").is_some());
        assert!(registry.get_language("ha").is_some());
    }
    #[test]
    fn test_extended_language_registry_get_language() {
        let registry = ExtendedLanguageRegistry::with_extended_set();
        let english = registry.get_language("en").unwrap();
        assert_eq!(english.name, "English");
        assert_eq!(english.family, "Germanic");
        let swahili = registry.get_language("sw").unwrap();
        assert_eq!(swahili.name, "Swahili");
        assert_eq!(swahili.native_name, "Kiswahili");
    }
    #[test]
    fn test_extended_language_registry_by_family() {
        let registry = ExtendedLanguageRegistry::with_extended_set();
        let romance_langs = registry.get_by_family("Romance");
        assert!(!romance_langs.is_empty());
        assert!(romance_langs.iter().any(|l| l.code == "es"));
        assert!(romance_langs.iter().any(|l| l.code == "fr"));
    }
    #[test]
    fn test_extended_language_registry_low_resource() {
        let registry = ExtendedLanguageRegistry::with_extended_set();
        let low_resource = registry.get_low_resource_languages();
        assert!(!low_resource.is_empty());
        assert!(low_resource.iter().any(|l| l.code == "ha"));
        assert!(low_resource.iter().any(|l| l.code == "yo"));
    }
    #[test]
    fn test_extended_language_registry_add_custom() {
        let mut registry = ExtendedLanguageRegistry::new();
        registry.add_language(
            ExtendedLanguage::new("test", "Test Lang", "Test", "Test Family", "Latin")
                .with_speakers(10.0),
        );
        assert_eq!(registry.language_count(), 1);
        assert!(registry.get_language("test").is_some());
    }
    #[test]
    fn test_low_resource_strategy_display() {
        assert_eq!(
            LowResourceStrategy::FallbackToRelated.to_string(),
            "Fallback to Related Language"
        );
        assert_eq!(
            LowResourceStrategy::TransferLearning.to_string(),
            "Transfer Learning"
        );
        assert_eq!(
            LowResourceStrategy::MultilingualModel.to_string(),
            "Multilingual Model"
        );
        assert_eq!(
            LowResourceStrategy::CommunityDriven.to_string(),
            "Community-Driven"
        );
    }
    #[test]
    fn test_low_resource_config_creation() {
        let config = LowResourceConfig::new("ha", LowResourceStrategy::FallbackToRelated)
            .add_fallback("sw")
            .add_fallback("en")
            .with_transfer_from("sw")
            .with_min_confidence(0.7);
        assert_eq!(config.language_code, "ha");
        assert_eq!(config.strategy, LowResourceStrategy::FallbackToRelated);
        assert_eq!(config.fallback_chain.len(), 2);
        assert_eq!(config.transfer_from, Some("sw".to_string()));
        assert_eq!(config.min_confidence, 0.7);
    }
    #[test]
    fn test_low_resource_support_defaults() {
        let support = LowResourceSupport::with_defaults();
        assert!(support.config_count() > 0);
        assert!(support.get_config("ha").is_some());
        assert!(support.get_config("yo").is_some());
    }
    #[test]
    fn test_low_resource_support_get_config() {
        let support = LowResourceSupport::with_defaults();
        let ha_config = support.get_config("ha").unwrap();
        assert_eq!(ha_config.language_code, "ha");
        assert_eq!(ha_config.strategy, LowResourceStrategy::FallbackToRelated);
    }
    #[test]
    fn test_low_resource_support_fallback_chain() {
        let support = LowResourceSupport::with_defaults();
        let chain = support.get_fallback_chain("ha");
        assert!(!chain.is_empty());
        assert!(chain.contains(&"sw".to_string()));
        assert!(chain.contains(&"en".to_string()));
    }
    #[test]
    fn test_low_resource_support_is_low_resource() {
        let support = LowResourceSupport::with_defaults();
        assert!(support.is_low_resource("ha"));
        assert!(support.is_low_resource("yo"));
        assert!(!support.is_low_resource("en"));
    }
    #[test]
    fn test_low_resource_support_add_custom() {
        let registry = ExtendedLanguageRegistry::with_extended_set();
        let mut support = LowResourceSupport::new(registry);
        support.add_config(
            LowResourceConfig::new("test", LowResourceStrategy::CommunityDriven).add_fallback("en"),
        );
        assert_eq!(support.config_count(), 1);
        assert!(support.get_config("test").is_some());
    }
    #[test]
    fn test_dialect_type_display() {
        assert_eq!(DialectType::Regional.to_string(), "Regional");
        assert_eq!(DialectType::Social.to_string(), "Social");
        assert_eq!(DialectType::Occupational.to_string(), "Occupational");
        assert_eq!(DialectType::Historical.to_string(), "Historical");
    }
    #[test]
    fn test_dialect_creation() {
        let dialect = Dialect::new(
            "en-GB-legal",
            "en",
            "British Legal English",
            DialectType::Occupational,
        )
        .with_region("GB")
        .add_variation("attorney", "solicitor")
        .add_variation("lawsuit", "legal action");
        assert_eq!(dialect.dialect_id, "en-GB-legal");
        assert_eq!(dialect.base_language, "en");
        assert_eq!(dialect.name, "British Legal English");
        assert_eq!(dialect.dialect_type, DialectType::Occupational);
        assert_eq!(dialect.region, Some("GB".to_string()));
        assert_eq!(dialect.variations.len(), 2);
    }
    #[test]
    fn test_dialect_to_dialect() {
        let dialect = Dialect::new(
            "en-GB-legal",
            "en",
            "British Legal English",
            DialectType::Occupational,
        )
        .add_variation("attorney", "solicitor");
        let result = dialect.to_dialect("attorney");
        assert_eq!(result, Some("solicitor".to_string()));
        let no_result = dialect.to_dialect("unknown");
        assert_eq!(no_result, None);
    }
    #[test]
    fn test_dialect_to_standard() {
        let dialect = Dialect::new(
            "en-GB-legal",
            "en",
            "British Legal English",
            DialectType::Occupational,
        )
        .add_variation("attorney", "solicitor");
        let result = dialect.to_standard("solicitor");
        assert_eq!(result, Some("attorney".to_string()));
        let no_result = dialect.to_standard("unknown");
        assert_eq!(no_result, None);
    }
    #[test]
    fn test_dialect_handler_defaults() {
        let handler = DialectHandler::with_defaults();
        assert!(handler.dialect_count() > 0);
        assert!(handler.get_dialect("en-GB-legal").is_some());
        assert!(handler.get_dialect("en-US-legal").is_some());
    }
    #[test]
    fn test_dialect_handler_get_dialect() {
        let handler = DialectHandler::with_defaults();
        let gb_legal = handler.get_dialect("en-GB-legal").unwrap();
        assert_eq!(gb_legal.name, "British Legal English");
        assert_eq!(gb_legal.base_language, "en");
    }
    #[test]
    fn test_dialect_handler_get_by_language() {
        let handler = DialectHandler::with_defaults();
        let en_dialects = handler.get_by_language("en");
        assert!(!en_dialects.is_empty());
        assert!(en_dialects.iter().any(|d| d.dialect_id == "en-GB-legal"));
        assert!(en_dialects.iter().any(|d| d.dialect_id == "en-US-legal"));
    }
    #[test]
    fn test_dialect_handler_normalize() {
        let handler = DialectHandler::with_defaults();
        let result = handler.normalize("en-GB-legal", "solicitor");
        assert_eq!(result, Some("attorney".to_string()));
    }
    #[test]
    fn test_dialect_handler_to_dialect() {
        let handler = DialectHandler::with_defaults();
        let result = handler.to_dialect("en-GB-legal", "attorney");
        assert_eq!(result, Some("solicitor".to_string()));
    }
    #[test]
    fn test_dialect_handler_add_custom() {
        let mut handler = DialectHandler::new();
        handler.add_dialect(
            Dialect::new(
                "test-dialect",
                "test",
                "Test Dialect",
                DialectType::Regional,
            )
            .add_variation("word1", "word2"),
        );
        assert_eq!(handler.dialect_count(), 1);
        assert!(handler.get_dialect("test-dialect").is_some());
    }
    #[test]
    fn test_local_law_term_creation() {
        let term = LocalLawTerm::new("憲法", "Constitution", "Civil Law", "JP", "国の最高法規")
            .add_example("憲法第9条")
            .add_statute("日本国憲法");
        assert_eq!(term.local_term, "憲法");
        assert_eq!(term.english_equiv, "Constitution");
        assert_eq!(term.legal_system, "Civil Law");
        assert_eq!(term.jurisdiction, "JP");
        assert_eq!(term.definition, "国の最高法規");
        assert_eq!(term.examples.len(), 1);
        assert_eq!(term.related_statutes.len(), 1);
    }
    #[test]
    fn test_local_law_database_defaults() {
        let db = LocalLawDatabase::with_samples();
        assert!(db.term_count() > 0);
        assert!(db.get_term("憲法").is_some());
        assert!(db.get_term("Grundgesetz").is_some());
    }
    #[test]
    fn test_local_law_database_get_term() {
        let db = LocalLawDatabase::with_samples();
        let term = db.get_term("憲法").unwrap();
        assert_eq!(term.english_equiv, "Constitution");
        assert_eq!(term.jurisdiction, "JP");
    }
    #[test]
    fn test_local_law_database_by_jurisdiction() {
        let db = LocalLawDatabase::with_samples();
        let jp_terms = db.get_by_jurisdiction("JP");
        assert!(!jp_terms.is_empty());
        assert!(jp_terms.iter().any(|t| t.local_term == "憲法"));
    }
    #[test]
    fn test_local_law_database_by_system() {
        let db = LocalLawDatabase::with_samples();
        let civil_law_terms = db.get_by_system("Civil Law");
        assert!(!civil_law_terms.is_empty());
    }
    #[test]
    fn test_local_law_database_to_english() {
        let db = LocalLawDatabase::with_samples();
        let translation = db.to_english("憲法");
        assert_eq!(translation, Some("Constitution".to_string()));
        let no_translation = db.to_english("unknown");
        assert_eq!(no_translation, None);
    }
    #[test]
    fn test_local_law_database_add_custom() {
        let mut db = LocalLawDatabase::new();
        db.add_term(LocalLawTerm::new(
            "test",
            "Test Term",
            "Test System",
            "TEST",
            "Test definition",
        ));
        assert_eq!(db.term_count(), 1);
        assert!(db.get_term("test").is_some());
    }
    #[test]
    fn test_contribution_status_display() {
        assert_eq!(ContributionStatus::Pending.to_string(), "Pending");
        assert_eq!(ContributionStatus::InReview.to_string(), "In Review");
        assert_eq!(ContributionStatus::Approved.to_string(), "Approved");
        assert_eq!(ContributionStatus::Rejected.to_string(), "Rejected");
    }
    #[test]
    fn test_contribution_creation() {
        let content = LocalLawTerm::new("test", "Test", "System", "JUR", "Definition");
        let contrib = Contribution::new("contrib-1", "user123", "en", content)
            .with_timestamp("2024-01-15T10:00:00Z")
            .add_comment("Looks good");
        assert_eq!(contrib.contribution_id, "contrib-1");
        assert_eq!(contrib.contributor, "user123");
        assert_eq!(contrib.language_code, "en");
        assert_eq!(contrib.status, ContributionStatus::Pending);
        assert_eq!(contrib.submitted_at, "2024-01-15T10:00:00Z");
        assert_eq!(contrib.comments.len(), 1);
    }
    #[test]
    fn test_contribution_approve() {
        let content = LocalLawTerm::new("test", "Test", "System", "JUR", "Definition");
        let mut contrib = Contribution::new("contrib-1", "user123", "en", content);
        contrib.approve();
        assert_eq!(contrib.status, ContributionStatus::Approved);
    }
    #[test]
    fn test_contribution_reject() {
        let content = LocalLawTerm::new("test", "Test", "System", "JUR", "Definition");
        let mut contrib = Contribution::new("contrib-1", "user123", "en", content);
        contrib.reject("Needs more details");
        assert_eq!(contrib.status, ContributionStatus::Rejected);
        assert_eq!(
            contrib.rejection_reason,
            Some("Needs more details".to_string())
        );
    }
    #[test]
    fn test_contribution_start_review() {
        let content = LocalLawTerm::new("test", "Test", "System", "JUR", "Definition");
        let mut contrib = Contribution::new("contrib-1", "user123", "en", content);
        contrib.start_review();
        assert_eq!(contrib.status, ContributionStatus::InReview);
    }
    #[test]
    fn test_contribution_workflow_new() {
        let workflow = ContributionWorkflow::new();
        assert_eq!(workflow.contribution_count(), 0);
    }
    #[test]
    fn test_contribution_workflow_submit() {
        let mut workflow = ContributionWorkflow::new();
        let content = LocalLawTerm::new("test", "Test", "System", "JUR", "Definition");
        let contrib = Contribution::new("contrib-1", "user123", "en", content);
        workflow.submit(contrib);
        assert_eq!(workflow.contribution_count(), 1);
    }
    #[test]
    fn test_contribution_workflow_get_contribution() {
        let mut workflow = ContributionWorkflow::new();
        let content = LocalLawTerm::new("test", "Test", "System", "JUR", "Definition");
        let contrib = Contribution::new("contrib-1", "user123", "en", content);
        workflow.submit(contrib);
        let retrieved = workflow.get_contribution("contrib-1");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().contribution_id, "contrib-1");
    }
    #[test]
    fn test_contribution_workflow_get_by_status() {
        let mut workflow = ContributionWorkflow::new();
        let content1 = LocalLawTerm::new("test1", "Test1", "System", "JUR", "Definition");
        let content2 = LocalLawTerm::new("test2", "Test2", "System", "JUR", "Definition");
        workflow.submit(Contribution::new("contrib-1", "user1", "en", content1));
        workflow.submit(Contribution::new("contrib-2", "user2", "en", content2));
        let pending = workflow.get_by_status(ContributionStatus::Pending);
        assert_eq!(pending.len(), 2);
    }
    #[test]
    fn test_contribution_workflow_get_by_language() {
        let mut workflow = ContributionWorkflow::new();
        let content1 = LocalLawTerm::new("test1", "Test1", "System", "JUR", "Definition");
        let content2 = LocalLawTerm::new("test2", "Test2", "System", "JUR", "Definition");
        workflow.submit(Contribution::new("contrib-1", "user1", "en", content1));
        workflow.submit(Contribution::new("contrib-2", "user2", "fr", content2));
        let en_contribs = workflow.get_by_language("en");
        assert_eq!(en_contribs.len(), 1);
        assert_eq!(en_contribs[0].contribution_id, "contrib-1");
    }
    #[test]
    fn test_contribution_workflow_approve() {
        let mut workflow = ContributionWorkflow::new();
        let content = LocalLawTerm::new("test", "Test", "System", "JUR", "Definition");
        workflow.submit(Contribution::new("contrib-1", "user123", "en", content));
        let result = workflow.approve("contrib-1");
        assert!(result.is_ok());
        let contrib = workflow.get_contribution("contrib-1").unwrap();
        assert_eq!(contrib.status, ContributionStatus::Approved);
    }
    #[test]
    fn test_contribution_workflow_reject() {
        let mut workflow = ContributionWorkflow::new();
        let content = LocalLawTerm::new("test", "Test", "System", "JUR", "Definition");
        workflow.submit(Contribution::new("contrib-1", "user123", "en", content));
        let result = workflow.reject("contrib-1", "Insufficient detail");
        assert!(result.is_ok());
        let contrib = workflow.get_contribution("contrib-1").unwrap();
        assert_eq!(contrib.status, ContributionStatus::Rejected);
        assert_eq!(
            contrib.rejection_reason,
            Some("Insufficient detail".to_string())
        );
    }
    #[test]
    fn test_contribution_workflow_count_by_status() {
        let mut workflow = ContributionWorkflow::new();
        let content1 = LocalLawTerm::new("test1", "Test1", "System", "JUR", "Definition");
        let content2 = LocalLawTerm::new("test2", "Test2", "System", "JUR", "Definition");
        workflow.submit(Contribution::new("contrib-1", "user1", "en", content1));
        workflow.submit(Contribution::new("contrib-2", "user2", "en", content2));
        workflow.approve("contrib-1").ok();
        assert_eq!(workflow.count_by_status(ContributionStatus::Pending), 1);
        assert_eq!(workflow.count_by_status(ContributionStatus::Approved), 1);
    }
    #[test]
    fn test_emerging_markets_integration() {
        let registry = ExtendedLanguageRegistry::with_extended_set();
        let support = LowResourceSupport::with_defaults();
        let handler = DialectHandler::with_defaults();
        assert!(registry.language_count() >= 60);
        assert!(support.is_low_resource("ha"));
        let fallback = support.get_fallback_chain("ha");
        assert!(!fallback.is_empty());
        let en_dialects = handler.get_by_language("en");
        assert!(!en_dialects.is_empty());
    }
    #[test]
    fn test_low_resource_with_dialects() {
        let support = LowResourceSupport::with_defaults();
        let handler = DialectHandler::with_defaults();
        assert!(support.is_low_resource("yo"));
        let result = handler.to_dialect("en-GB-legal", "attorney");
        assert_eq!(result, Some("solicitor".to_string()));
    }
    #[test]
    fn test_local_law_with_contribution() {
        let db = LocalLawDatabase::with_samples();
        let mut workflow = ContributionWorkflow::new();
        assert!(db.get_term("憲法").is_some());
        let new_term =
            LocalLawTerm::new("新用語", "New Term", "Civil Law", "JP", "新しい用語の定義");
        let contrib = Contribution::new("contrib-1", "user123", "ja", new_term);
        workflow.submit(contrib);
        assert_eq!(workflow.contribution_count(), 1);
        workflow.approve("contrib-1").ok();
        let approved = workflow.get_by_status(ContributionStatus::Approved);
        assert_eq!(approved.len(), 1);
    }
}
#[cfg(test)]
mod legal_nlp_tests {
    use crate::*;
    #[test]
    fn test_legal_entity_type_display() {
        assert_eq!(LegalEntityType::Court.to_string(), "Court");
        assert_eq!(LegalEntityType::Company.to_string(), "Company");
        assert_eq!(LegalEntityType::Statute.to_string(), "Statute");
        assert_eq!(LegalEntityType::Person.to_string(), "Person");
        assert_eq!(
            LegalEntityType::GovernmentAgency.to_string(),
            "Government Agency"
        );
        assert_eq!(LegalEntityType::LawFirm.to_string(), "Law Firm");
    }
    #[test]
    fn test_legal_entity_creation() {
        let entity = LegalEntity::new("Supreme Court", LegalEntityType::Court, 0)
            .with_confidence(0.95)
            .with_normalized("US Supreme Court");
        assert_eq!(entity.text, "Supreme Court");
        assert_eq!(entity.entity_type, LegalEntityType::Court);
        assert_eq!(entity.confidence, 0.95);
        assert_eq!(entity.position, 0);
        assert_eq!(entity.normalized, Some("US Supreme Court".to_string()));
    }
    #[test]
    fn test_entity_recognizer_court() {
        let recognizer = LegalEntityRecognizer::new();
        let text = "The Supreme Court ruled in favor.";
        let entities = recognizer.recognize(text);
        assert!(!entities.is_empty());
        let court_count = recognizer.count_by_type(&entities, &LegalEntityType::Court);
        assert!(court_count > 0);
    }
    #[test]
    fn test_entity_recognizer_company() {
        let recognizer = LegalEntityRecognizer::new();
        let text = "Apple Inc. signed the agreement.";
        let entities = recognizer.recognize(text);
        let company_count = recognizer.count_by_type(&entities, &LegalEntityType::Company);
        assert!(company_count > 0);
    }
    #[test]
    fn test_entity_recognizer_custom_patterns() {
        let mut recognizer = LegalEntityRecognizer::new();
        recognizer.add_court_pattern("High Court");
        recognizer.add_company_suffix("Plc.");
        assert!(
            recognizer
                .court_patterns
                .contains(&"High Court".to_string())
        );
        assert!(recognizer.company_suffixes.contains(&"Plc.".to_string()));
    }
    #[test]
    fn test_clause_class_display() {
        assert_eq!(ClauseClass::Payment.to_string(), "Payment");
        assert_eq!(ClauseClass::Termination.to_string(), "Termination");
        assert_eq!(ClauseClass::Confidentiality.to_string(), "Confidentiality");
        assert_eq!(
            ClauseClass::LiabilityLimitation.to_string(),
            "Liability Limitation"
        );
        assert_eq!(ClauseClass::Indemnification.to_string(), "Indemnification");
        assert_eq!(ClauseClass::ForceMajeure.to_string(), "Force Majeure");
    }
    #[test]
    fn test_classified_clause_creation() {
        let clause = ClassifiedClause::new(
            "This is a confidentiality clause.",
            ClauseClass::Confidentiality,
            0.9,
        )
        .add_alternative(ClauseClass::Payment, 0.3);
        assert_eq!(clause.text, "This is a confidentiality clause.");
        assert_eq!(clause.class, ClauseClass::Confidentiality);
        assert_eq!(clause.confidence, 0.9);
        assert_eq!(clause.alternatives.len(), 1);
    }
    #[test]
    fn test_clause_classifier_payment() {
        let classifier = ClauseClassifier::new();
        let clause = "The client shall make payment of fees upon receipt of invoice.";
        let result = classifier.classify(clause);
        assert!(result.is_some());
        let classified = result.unwrap();
        assert_eq!(classified.class, ClauseClass::Payment);
        assert!(classified.confidence > 0.0);
    }
    #[test]
    fn test_clause_classifier_confidentiality() {
        let classifier = ClauseClassifier::new();
        let clause = "All confidential information shall remain proprietary.";
        let result = classifier.classify(clause);
        assert!(result.is_some());
        let classified = result.unwrap();
        assert_eq!(classified.class, ClauseClass::Confidentiality);
    }
    #[test]
    fn test_clause_classifier_threshold() {
        let classifier = ClauseClassifier::new().with_threshold(0.8);
        let clause = "Random text without keywords.";
        let result = classifier.classify(clause);
        assert!(result.is_none());
    }
    #[test]
    fn test_clause_classifier_custom_pattern() {
        let mut classifier = ClauseClassifier::new();
        classifier.add_pattern(
            ClauseClass::Custom("Test".to_string()),
            vec!["custom".to_string(), "test".to_string()],
        );
        let clause = "This is a custom test clause.";
        let result = classifier.classify(clause);
        assert!(result.is_some());
    }
    #[test]
    fn test_legal_topic_creation() {
        let topic = LegalTopic::new("contract", "Contract Law")
            .add_term("contract")
            .add_term("agreement")
            .with_weight(0.75);
        assert_eq!(topic.id, "contract");
        assert_eq!(topic.name, "Contract Law");
        assert_eq!(topic.key_terms.len(), 2);
        assert_eq!(topic.weight, 0.75);
    }
    #[test]
    fn test_topic_modeler_extract() {
        let modeler = LegalTopicModeler::new();
        let text = "This contract agreement creates obligations between the parties.";
        let topics = modeler.extract_topics(text);
        assert!(!topics.is_empty());
        assert_eq!(topics[0].id, "contract");
        assert!(topics[0].weight > 0.0);
    }
    #[test]
    fn test_topic_modeler_multiple_topics() {
        let modeler = LegalTopicModeler::new();
        let text = "The corporation's shareholders approved the merger. The patent for this invention is pending.";
        let topics = modeler.extract_topics(text);
        assert!(topics.len() >= 2);
    }
    #[test]
    fn test_topic_modeler_custom_topic() {
        let mut modeler = LegalTopicModeler::new();
        let custom_topic = LegalTopic::new("custom", "Custom Topic")
            .add_term("custom")
            .add_term("specific");
        modeler.add_topic(custom_topic);
        assert_eq!(modeler.topic_count(), 7);
    }
    #[test]
    fn test_similarity_score_creation() {
        let score = SimilarityScore::new("doc1", "doc2", 0.85, "cosine");
        assert_eq!(score.doc1_id, "doc1");
        assert_eq!(score.doc2_id, "doc2");
        assert_eq!(score.score, 0.85);
        assert_eq!(score.method, "cosine");
    }
    #[test]
    fn test_similarity_score_highly_similar() {
        let score = SimilarityScore::new("doc1", "doc2", 0.9, "cosine");
        assert!(score.is_highly_similar());
        assert!(score.is_moderately_similar());
    }
    #[test]
    fn test_similarity_score_moderately_similar() {
        let score = SimilarityScore::new("doc1", "doc2", 0.6, "cosine");
        assert!(!score.is_highly_similar());
        assert!(score.is_moderately_similar());
    }
    #[test]
    fn test_similarity_calculator_jaccard() {
        let calc = DocumentSimilarityCalculator::new();
        let doc1 = "contract agreement terms conditions";
        let doc2 = "contract terms conditions legal";
        let similarity = calc.jaccard_similarity(doc1, doc2);
        assert!(similarity > 0.0);
        assert!(similarity <= 1.0);
    }
    #[test]
    fn test_similarity_calculator_cosine() {
        let calc = DocumentSimilarityCalculator::new();
        let doc1 = "legal contract agreement";
        let doc2 = "legal contract agreement";
        let similarity = calc.cosine_similarity(doc1, doc2);
        assert!((similarity - 1.0).abs() < 0.01);
    }
    #[test]
    fn test_similarity_calculator_compare() {
        let calc = DocumentSimilarityCalculator::new();
        let doc1 = "This is a legal contract.";
        let doc2 = "This is a legal agreement.";
        let score = calc.compare("doc1", doc1, "doc2", doc2);
        assert_eq!(score.doc1_id, "doc1");
        assert_eq!(score.doc2_id, "doc2");
        assert_eq!(score.method, "cosine");
        assert!(score.score > 0.0);
    }
    #[test]
    fn test_similarity_calculator_threshold() {
        let calc = DocumentSimilarityCalculator::new().with_threshold(0.7);
        assert_eq!(calc.threshold, 0.7);
    }
    #[test]
    fn test_tfidf_score_creation() {
        let score = TfIdfScore::new("contract", 0.5, 2.0);
        assert_eq!(score.term, "contract");
        assert_eq!(score.term_frequency, 0.5);
        assert_eq!(score.idf, 2.0);
        assert_eq!(score.score, 1.0);
    }
    #[test]
    fn test_key_term_extractor_creation() {
        let extractor = KeyTermExtractor::new();
        assert_eq!(extractor.corpus_size(), 0);
        assert!(extractor.stop_words.contains("the"));
    }
    #[test]
    fn test_key_term_extractor_add_document() {
        let mut extractor = KeyTermExtractor::new();
        extractor.add_document("This is a legal contract.");
        extractor.add_document("This is a legal agreement.");
        assert_eq!(extractor.corpus_size(), 2);
    }
    #[test]
    fn test_key_term_extractor_extract() {
        let mut extractor = KeyTermExtractor::new();
        extractor.add_document("Legal contract with legal terms.");
        extractor.add_document("Another legal document.");
        let text = "Legal contract agreement with specific terms.";
        let key_terms = extractor.extract_key_terms(text, 3);
        assert!(key_terms.len() <= 3);
        assert!(!key_terms.is_empty());
        if key_terms.len() >= 2 {
            assert!(key_terms[0].score >= key_terms[1].score);
        }
    }
    #[test]
    fn test_key_term_extractor_custom_stop_word() {
        let mut extractor = KeyTermExtractor::new();
        extractor.add_stop_word("legal");
        assert!(extractor.stop_words.contains("legal"));
    }
    #[test]
    fn test_nlp_pipeline_integration() {
        let recognizer = LegalEntityRecognizer::new();
        let text = "Apple Inc. sued Samsung Corp. in the Supreme Court.";
        let entities = recognizer.recognize(text);
        assert!(!entities.is_empty());
        let classifier = ClauseClassifier::new();
        let clause = "The parties agree to maintain confidentiality.";
        let classified = classifier.classify(clause);
        assert!(classified.is_some());
        let modeler = LegalTopicModeler::new();
        let _topics = modeler.extract_topics(text);
    }
    #[test]
    fn test_document_analysis_integration() {
        let calc = DocumentSimilarityCalculator::new();
        let doc1 = "This contract establishes the terms and conditions.";
        let doc2 = "This agreement sets forth the terms and conditions.";
        let similarity = calc.compare("d1", doc1, "d2", doc2);
        assert!(similarity.score > 0.5);
        let mut extractor = KeyTermExtractor::new();
        extractor.add_document(doc1);
        extractor.add_document(doc2);
        let key_terms = extractor.extract_key_terms(doc1, 5);
        assert!(!key_terms.is_empty());
    }
}
