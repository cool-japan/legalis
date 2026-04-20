//! Auto-generated module: tests for legalis-porting.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[cfg(test)]
mod tests {
    use crate::*;
    use legalis_core::{Effect, EffectType, Statute};
    use legalis_i18n::{CulturalParams, Jurisdiction, LegalSystem, Locale};
    fn test_jurisdiction_jp() -> Jurisdiction {
        Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
            .with_legal_system(LegalSystem::CivilLaw)
            .with_cultural_params(CulturalParams::japan())
    }
    fn test_jurisdiction_us() -> Jurisdiction {
        Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
            .with_legal_system(LegalSystem::CommonLaw)
            .with_cultural_params(CulturalParams::for_country("US"))
    }
    #[test]
    fn test_port_statute() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new(
            "adult-rights",
            "成人権法",
            Effect::new(EffectType::Grant, "Complete legal capacity"),
        );
        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };
        let result = engine.port_statute(&statute, &options).unwrap();
        assert!(result.statute.id.starts_with("us-"));
    }
    #[test]
    fn test_compatibility_report() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statutes = [Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        )];
        let report = engine.generate_report(&statutes);
        assert!(report.compatibility_score > 0.0);
    }
    #[test]
    fn test_conflict_detection() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );
        let conflicts = engine.detect_conflicts(&statute);
        assert!(!conflicts.is_empty());
    }
    #[test]
    fn test_semantic_validation() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();
        let validation = engine.validate_semantics(&statute, &ported);
        assert!(validation.preservation_score >= 0.0);
        assert!(validation.preservation_score <= 1.0);
    }
    #[test]
    fn test_risk_assessment() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();
        let assessment = engine.assess_risks(&ported);
        assert!(assessment.risk_score >= 0.0);
        assert!(assessment.risk_score <= 1.0);
    }
    #[test]
    fn test_partial_porting() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let section_ids = vec!["section1".to_string(), "section2".to_string()];
        let result = engine
            .port_sections(&statute, &section_ids, &options)
            .unwrap();
        assert!(result.statute.id.starts_with("us-"));
        assert!(!result.changes.is_empty());
    }
    #[test]
    fn test_reverse_porting() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let _changes = engine.reverse_port_analysis(&statute).unwrap();
    }
    #[tokio::test]
    async fn test_batch_port() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statutes = [
            Statute::new("test1", "Test 1", Effect::new(EffectType::Grant, "Test 1")),
            Statute::new("test2", "Test 2", Effect::new(EffectType::Grant, "Test 2")),
        ];
        let options = PortingOptions {
            generate_report: true,
            detect_conflicts: true,
            validate_semantics: true,
            ..Default::default()
        };
        let result = engine.batch_port(&statutes, &options).await.unwrap();
        assert_eq!(result.statutes.len(), 2);
        assert!(result.report.is_some());
        assert!(!result.conflicts.is_empty());
        assert!(result.semantic_validation.is_some());
        assert!(result.risk_assessment.is_some());
    }
    #[test]
    fn test_bilateral_agreement() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let agreement = engine.create_bilateral_agreement(AgreementType::MutualRecognition);
        assert_eq!(agreement.source_jurisdiction, "JP");
        assert_eq!(agreement.target_jurisdiction, "US");
        assert!(!agreement.mutual_recognition.is_empty());
        assert!(!agreement.adaptation_protocols.is_empty());
    }
    #[test]
    fn test_regulatory_equivalence() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us())
            .with_equivalence_mappings(vec![EquivalenceMapping {
                source_regulation: "test".to_string(),
                target_regulation: "us-test".to_string(),
                equivalence_score: 0.9,
                differences: vec!["Minor terminology differences".to_string()],
                notes: "Highly equivalent".to_string(),
            }]);
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let mappings = engine.find_regulatory_equivalence(&statute);
        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].equivalence_score, 0.9);
    }
    #[tokio::test]
    async fn test_similar_statutes() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new(
            "test",
            "Adult Rights Law",
            Effect::new(EffectType::Grant, "Test"),
        );
        let candidates = vec![
            Statute::new(
                "c1",
                "Adult Rights Statute",
                Effect::new(EffectType::Grant, "C1"),
            ),
            Statute::new(
                "c2",
                "Child Protection Law",
                Effect::new(EffectType::Grant, "C2"),
            ),
            Statute::new(
                "c3",
                "Adult Legal Capacity",
                Effect::new(EffectType::Grant, "C3"),
            ),
        ];
        let similar = engine.find_similar_statutes(&statute, &candidates).await;
        assert!(!similar.is_empty());
        assert!(similar[0].1 >= 0.3);
    }
    #[test]
    fn test_term_replacement() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us())
            .with_term_replacements(vec![TermReplacement {
                source_term: "成人".to_string(),
                target_term: "adult".to_string(),
                context: None,
                confidence: 0.95,
            }]);
        let mut statute = Statute::new(
            "test",
            "成人 Rights Law",
            Effect::new(EffectType::Grant, "Test"),
        );
        let replacements = engine.apply_term_replacement(&mut statute);
        assert_eq!(replacements.len(), 1);
        assert!(statute.title.contains("adult"));
    }
    #[test]
    fn test_contextual_adjustment() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new(
            "test",
            "Fine Payment Law",
            Effect::new(EffectType::Grant, "Test"),
        );
        let adjustments = engine.adjust_parameters_contextually(&statute);
        assert!(!adjustments.is_empty());
    }
    #[test]
    fn test_workflow_creation() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let workflow = engine.create_workflow("test-statute".to_string());
        assert_eq!(workflow.state, WorkflowState::Initiated);
        assert_eq!(workflow.pending_steps.len(), 4);
        assert_eq!(workflow.approvals.len(), 2);
    }
    #[test]
    fn test_workflow_advancement() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let mut workflow = engine.create_workflow("test-statute".to_string());
        engine.advance_workflow(&mut workflow).unwrap();
        assert_eq!(workflow.state, WorkflowState::InProgress);
        assert_eq!(workflow.completed_steps.len(), 1);
        assert_eq!(workflow.pending_steps.len(), 3);
    }
    #[test]
    fn test_versioned_statute() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();
        let versioned = engine.create_versioned_statute(
            ported,
            1,
            "test_user".to_string(),
            "Initial version".to_string(),
        );
        assert_eq!(versioned.version, 1);
        assert!(versioned.previous_hash.is_none());
        assert!(!versioned.hash.is_empty());
    }
    #[test]
    fn test_version_comparison() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute1 = Statute::new("test", "Test V1", Effect::new(EffectType::Grant, "Test"));
        let statute2 = Statute::new("test", "Test V2", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported1 = engine.port_statute(&statute1, &options).unwrap();
        let ported2 = engine.port_statute(&statute2, &options).unwrap();
        let v1 = engine.create_versioned_statute(ported1, 1, "user".to_string(), "V1".to_string());
        let v2 = engine.create_versioned_statute(ported2, 2, "user".to_string(), "V2".to_string());
        let differences = engine.compare_versions(&v1, &v2);
        assert!(!differences.is_empty());
    }
    #[test]
    fn test_submit_for_review() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();
        let review_request = engine.submit_for_review(ported);
        assert_eq!(review_request.status, ReviewStatus::Pending);
        assert_eq!(review_request.source_jurisdiction, "JP");
        assert_eq!(review_request.target_jurisdiction, "US");
        assert!(review_request.assigned_expert.is_none());
        assert!(review_request.reviews.is_empty());
    }
    #[test]
    fn test_assign_expert() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();
        let mut review_request = engine.submit_for_review(ported);
        engine.assign_expert(&mut review_request, "expert-001".to_string());
        assert_eq!(review_request.status, ReviewStatus::Assigned);
        assert_eq!(
            review_request.assigned_expert,
            Some("expert-001".to_string())
        );
    }
    #[test]
    fn test_add_expert_review_approve() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();
        let mut review_request = engine.submit_for_review(ported);
        let expert_review = ExpertReview {
            id: "review-001".to_string(),
            expert_id: "expert-001".to_string(),
            expert_name: "Dr. Legal Expert".to_string(),
            qualifications: vec!["Bar License".to_string(), "PhD in Law".to_string()],
            reviewed_at: chrono::Utc::now().to_rfc3339(),
            recommendation: ReviewRecommendation::Approve,
            comments: Vec::new(),
            confidence: 0.95,
            concerns: Vec::new(),
            suggested_modifications: Vec::new(),
        };
        engine
            .add_expert_review(&mut review_request, expert_review)
            .unwrap();
        assert_eq!(review_request.status, ReviewStatus::Approved);
        assert_eq!(review_request.reviews.len(), 1);
    }
    #[test]
    fn test_add_expert_review_reject() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();
        let mut review_request = engine.submit_for_review(ported);
        let expert_review = ExpertReview {
            id: "review-001".to_string(),
            expert_id: "expert-001".to_string(),
            expert_name: "Dr. Legal Expert".to_string(),
            qualifications: vec!["Bar License".to_string()],
            reviewed_at: chrono::Utc::now().to_rfc3339(),
            recommendation: ReviewRecommendation::Reject,
            comments: Vec::new(),
            confidence: 0.9,
            concerns: vec!["Major legal incompatibility".to_string()],
            suggested_modifications: vec!["Complete revision required".to_string()],
        };
        engine
            .add_expert_review(&mut review_request, expert_review)
            .unwrap();
        assert_eq!(review_request.status, ReviewStatus::Rejected);
    }
    #[test]
    fn test_create_review_comment() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let comment = engine.create_review_comment(
            Some("section-1".to_string()),
            "This section needs clarification".to_string(),
            Severity::Warning,
            "Clarity".to_string(),
        );
        assert!(comment.section.is_some());
        assert_eq!(comment.text, "This section needs clarification");
        assert_eq!(comment.severity, Severity::Warning);
        assert_eq!(comment.category, "Clarity");
    }
    #[test]
    fn test_compliance_check() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );
        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };
        let ported = engine.port_statute(&statute, &options).unwrap();
        let result = engine.check_compliance(&ported);
        assert!(!result.checks.is_empty());
        assert!(result.compliance_score >= 0.0);
        assert!(result.compliance_score <= 1.0);
        assert!(!result.recommendations.is_empty());
    }
    #[test]
    fn test_compliance_check_detects_issues() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();
        let result = engine.check_compliance(&ported);
        assert!(!result.violations.is_empty());
        assert_eq!(result.status, ComplianceStatus::RequiresReview);
    }
    #[test]
    fn test_batch_compliance_check() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statutes = [
            Statute::new("test1", "Test 1", Effect::new(EffectType::Grant, "Test 1")),
            Statute::new("test2", "Test 2", Effect::new(EffectType::Grant, "Test 2")),
        ];
        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };
        let ported: Vec<PortedStatute> = statutes
            .iter()
            .map(|s| engine.port_statute(s, &options).unwrap())
            .collect();
        let results = engine.batch_check_compliance(&ported);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.compliance_score >= 0.0));
    }
    #[test]
    fn test_compliance_summary() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statutes = [
            Statute::new("test1", "Test 1", Effect::new(EffectType::Grant, "Test 1")),
            Statute::new("test2", "Test 2", Effect::new(EffectType::Grant, "Test 2")),
        ];
        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };
        let ported: Vec<PortedStatute> = statutes
            .iter()
            .map(|s| engine.port_statute(s, &options).unwrap())
            .collect();
        let results = engine.batch_check_compliance(&ported);
        let summary = engine.generate_compliance_summary(&results);
        assert_eq!(summary.total_statutes, 2);
        assert!(summary.average_compliance_score >= 0.0);
        assert!(summary.average_compliance_score <= 1.0);
    }
    #[test]
    fn test_export_compatibility_report_json() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let report = engine.generate_report(&[statute]);
        let json = engine
            .export_compatibility_report(&report, ExportFormat::Json)
            .unwrap();
        assert!(json.contains("compatibility_score"));
        assert!(json.contains("findings"));
    }
    #[test]
    fn test_export_compatibility_report_markdown() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let report = engine.generate_report(&[statute]);
        let md = engine
            .export_compatibility_report(&report, ExportFormat::Markdown)
            .unwrap();
        assert!(md.contains("# Compatibility Report"));
        assert!(md.contains("Compatibility Score"));
    }
    #[tokio::test]
    async fn test_export_porting_output() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statutes = [Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        )];
        let options = PortingOptions::default();
        let output = engine.batch_port(&statutes, &options).await.unwrap();
        let json = engine
            .export_porting_output(&output, ExportFormat::Json)
            .unwrap();
        assert!(json.contains("statutes"));
        let md = engine
            .export_porting_output(&output, ExportFormat::Markdown)
            .unwrap();
        assert!(md.contains("# Porting Output"));
    }
    #[test]
    fn test_tfidf_similarity() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute1 = Statute::new(
            "test1",
            "Adult Rights Law",
            Effect::new(EffectType::Grant, "Test"),
        );
        let statute2 = Statute::new(
            "test2",
            "Adult Rights Act",
            Effect::new(EffectType::Grant, "Test"),
        );
        let statute3 = Statute::new(
            "test3",
            "Child Protection Law",
            Effect::new(EffectType::Grant, "Test"),
        );
        let sim12 = engine.calculate_tfidf_similarity(&statute1, &statute2);
        let sim13 = engine.calculate_tfidf_similarity(&statute1, &statute3);
        assert!(sim12 > sim13);
        assert!((0.0..=1.0).contains(&sim12));
        assert!((0.0..=1.0).contains(&sim13));
    }
    #[test]
    fn test_create_template() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let template = engine.create_template(
            "Civil Law Template".to_string(),
            "Template for civil law statutes".to_string(),
            vec!["civil".to_string(), "commercial".to_string()],
        );
        assert_eq!(template.name, "Civil Law Template");
        assert_eq!(template.statute_types.len(), 2);
        assert!(!template.contextual_rules.is_empty());
    }
    #[test]
    fn test_apply_template() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let template = engine.create_template(
            "Test Template".to_string(),
            "Test".to_string(),
            vec!["test".to_string()],
        );
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let ported = engine.apply_template(&statute, &template).unwrap();
        assert!(ported.statute.id.starts_with("us-"));
    }
    #[test]
    fn test_generate_conflict_resolutions() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let conflicts = engine.detect_conflicts(&statute);
        let resolutions = engine.generate_conflict_resolutions(&conflicts);
        assert!(!resolutions.is_empty());
        for resolution in &resolutions {
            assert!(resolution.priority >= 1 && resolution.priority <= 10);
        }
        for i in 1..resolutions.len() {
            assert!(resolutions[i - 1].priority >= resolutions[i].priority);
        }
    }
    #[test]
    fn test_conflict_precedent_database() {
        let mut db = ConflictPrecedentDatabase::new();
        let precedent1 = ConflictPrecedent {
            id: "prec-1".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            conflict_type: ConflictType::SystemMismatch,
            description: "Legal system mismatch resolved".to_string(),
            resolution_used: "Adapt procedural elements".to_string(),
            effectiveness: 0.85,
            resolved_by: Some("Expert A".to_string()),
            resolved_at: "2024-01-01T00:00:00Z".to_string(),
            lessons_learned: vec!["Focus on procedural adaptation".to_string()],
            applicable_statute_types: vec!["commercial".to_string()],
            tags: vec!["system-mismatch".to_string()],
        };
        let precedent2 = ConflictPrecedent {
            id: "prec-2".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            conflict_type: ConflictType::CulturalIncompatibility,
            description: "Cultural conflict resolved".to_string(),
            resolution_used: "Local adaptation with consultation".to_string(),
            effectiveness: 0.75,
            resolved_by: Some("Expert B".to_string()),
            resolved_at: "2024-01-02T00:00:00Z".to_string(),
            lessons_learned: vec!["Involve local stakeholders".to_string()],
            applicable_statute_types: vec!["social".to_string()],
            tags: vec!["cultural".to_string()],
        };
        db.add_precedent(precedent1);
        db.add_precedent(precedent2);
        assert_eq!(db.all_precedents().len(), 2);
        let relevant = db.find_relevant_precedents("JP", "US", &ConflictType::SystemMismatch);
        assert_eq!(relevant.len(), 1);
        assert_eq!(relevant[0].id, "prec-1");
        let effective = db.get_effective_precedents();
        assert_eq!(effective.len(), 2);
    }
    #[test]
    fn test_conflict_detector_severity_analysis() {
        let mut detector = ConflictDetector::new();
        let precedent = ConflictPrecedent {
            id: "prec-1".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            conflict_type: ConflictType::Contradiction,
            description: "Test conflict".to_string(),
            resolution_used: "Test resolution".to_string(),
            effectiveness: 0.9,
            resolved_by: None,
            resolved_at: "2024-01-01T00:00:00Z".to_string(),
            lessons_learned: vec![],
            applicable_statute_types: vec![],
            tags: vec![],
        };
        detector.precedent_db.add_precedent(precedent);
        let jp = test_jurisdiction_jp();
        let us = test_jurisdiction_us();
        let conflict = ConflictReport {
            statute_id: "test".to_string(),
            conflict_type: ConflictType::Contradiction,
            description: "Test conflict".to_string(),
            severity: Severity::Warning,
            resolutions: vec!["Test resolution".to_string()],
        };
        let severity = detector.analyze_severity(&conflict, &jp, &us);
        assert!(matches!(
            severity,
            Severity::Warning | Severity::Error | Severity::Critical
        ));
    }
    #[test]
    fn test_conflict_detector_recommend_strategies() {
        let mut detector = ConflictDetector::new();
        let precedent = ConflictPrecedent {
            id: "prec-1".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            conflict_type: ConflictType::SystemMismatch,
            description: "Legal system mismatch".to_string(),
            resolution_used: "Gradual adaptation with expert review".to_string(),
            effectiveness: 0.85,
            resolved_by: Some("Expert A".to_string()),
            resolved_at: "2024-01-01T00:00:00Z".to_string(),
            lessons_learned: vec![],
            applicable_statute_types: vec![],
            tags: vec![],
        };
        detector.precedent_db.add_precedent(precedent);
        let template = NegotiatedResolutionTemplate {
            id: "template-1".to_string(),
            name: "System Mismatch Template".to_string(),
            conflict_types: vec![ConflictType::SystemMismatch],
            source_patterns: vec!["JP".to_string()],
            target_patterns: vec!["US".to_string()],
            approach: "Bilateral adaptation protocol".to_string(),
            negotiation_steps: vec![],
            fallback_strategies: vec![],
            success_rate: 0.8,
            stakeholders: vec![],
            required_approvals: vec![],
        };
        detector.add_template(template);
        let jp = test_jurisdiction_jp();
        let us = test_jurisdiction_us();
        let conflict = ConflictReport {
            statute_id: "test".to_string(),
            conflict_type: ConflictType::SystemMismatch,
            description: "System mismatch".to_string(),
            severity: Severity::Warning,
            resolutions: vec!["Default resolution".to_string()],
        };
        let strategies = detector.recommend_strategies(&conflict, &jp, &us);
        assert!(!strategies.is_empty());
        assert!(strategies.iter().any(|s| s.contains("effective")));
        assert!(strategies.iter().any(|s| s.contains("template")));
    }
    #[test]
    fn test_conflict_resolution_workflow_creation() {
        let detector = ConflictDetector::new();
        let conflict = ConflictReport {
            statute_id: "test".to_string(),
            conflict_type: ConflictType::Contradiction,
            description: "Critical conflict".to_string(),
            severity: Severity::Critical,
            resolutions: vec!["Manual review required".to_string()],
        };
        let workflow = detector.create_resolution_workflow(conflict);
        assert_eq!(workflow.state, ResolutionWorkflowState::InitialAssessment);
        assert_eq!(workflow.escalation_level, EscalationLevel::Critical);
        assert!(workflow.stakeholder_reviews.is_empty());
        assert!(workflow.expert_consultations.is_empty());
        assert!(workflow.proposed_resolution.is_none());
        assert!(workflow.final_decision.is_none());
    }
    #[test]
    fn test_negotiated_resolution_template() {
        let template = NegotiatedResolutionTemplate {
            id: "template-1".to_string(),
            name: "Cultural Adaptation Template".to_string(),
            conflict_types: vec![ConflictType::CulturalIncompatibility],
            source_patterns: vec!["CivilLaw".to_string()],
            target_patterns: vec!["CommonLaw".to_string()],
            approach: "Phased adaptation with stakeholder consultation".to_string(),
            negotiation_steps: vec![
                NegotiationStep {
                    step_number: 1,
                    description: "Initial stakeholder meeting".to_string(),
                    involved_parties: vec![
                        "Legal experts".to_string(),
                        "Cultural advisors".to_string(),
                    ],
                    expected_outcome: "Agreement on adaptation scope".to_string(),
                    estimated_days: 5,
                },
                NegotiationStep {
                    step_number: 2,
                    description: "Draft adaptation proposal".to_string(),
                    involved_parties: vec!["Legal drafters".to_string()],
                    expected_outcome: "Initial proposal document".to_string(),
                    estimated_days: 10,
                },
            ],
            fallback_strategies: vec![
                "Escalate to bilateral commission".to_string(),
                "Seek international arbitration".to_string(),
            ],
            success_rate: 0.75,
            stakeholders: vec![
                "Source jurisdiction legal authority".to_string(),
                "Target jurisdiction legal authority".to_string(),
                "Cultural representatives".to_string(),
            ],
            required_approvals: vec![
                "Legal committee".to_string(),
                "Cultural affairs ministry".to_string(),
            ],
        };
        assert_eq!(template.negotiation_steps.len(), 2);
        assert_eq!(template.fallback_strategies.len(), 2);
        assert_eq!(template.stakeholders.len(), 3);
        assert!(template.success_rate > 0.5);
        assert!(
            template
                .conflict_types
                .contains(&ConflictType::CulturalIncompatibility)
        );
    }
    #[test]
    fn test_escalation_level_ordering() {
        assert!(EscalationLevel::Routine < EscalationLevel::Elevated);
        assert!(EscalationLevel::Elevated < EscalationLevel::High);
        assert!(EscalationLevel::High < EscalationLevel::Critical);
    }
    #[test]
    fn test_stakeholder_review() {
        let review = StakeholderReview {
            reviewer_id: "reviewer-1".to_string(),
            reviewer_name: "Jane Smith".to_string(),
            role: "Legal Counsel".to_string(),
            reviewed_at: "2024-01-01T00:00:00Z".to_string(),
            recommendation: StakeholderRecommendation::ApproveWithModifications,
            comments: "Approve with minor adjustments to cultural references".to_string(),
            concerns: vec!["Potential cultural sensitivity issue in section 3".to_string()],
            modifications: vec![
                "Adjust terminology in section 3".to_string(),
                "Add explanatory note for cultural context".to_string(),
            ],
        };
        assert_eq!(
            review.recommendation,
            StakeholderRecommendation::ApproveWithModifications
        );
        assert_eq!(review.concerns.len(), 1);
        assert_eq!(review.modifications.len(), 2);
    }
    #[test]
    fn test_expert_consultation() {
        let consultation = ExpertConsultation {
            id: "consult-1".to_string(),
            expert_id: "expert-123".to_string(),
            expert_name: "Dr. John Doe".to_string(),
            expertise_area: "International Legal Systems".to_string(),
            consulted_at: "2024-01-01T00:00:00Z".to_string(),
            opinion: "The proposed adaptation is sound but requires additional safeguards"
                .to_string(),
            recommended_approach: "Implement with monitoring period".to_string(),
            confidence: 0.9,
            legal_references: vec![
                "Treaty on Legal Harmonization, Art. 12".to_string(),
                "Case Law: Smith v. State (2020)".to_string(),
            ],
        };
        assert_eq!(consultation.confidence, 0.9);
        assert_eq!(consultation.legal_references.len(), 2);
        assert!(consultation.opinion.contains("safeguards"));
    }
    #[test]
    fn test_resolution_decision() {
        let decision = ResolutionDecision {
            id: "decision-1".to_string(),
            decision_maker_id: "dm-123".to_string(),
            decision_maker_role: "Chief Legal Officer".to_string(),
            decided_at: "2024-01-01T00:00:00Z".to_string(),
            chosen_strategy: "Gradual implementation with monitoring".to_string(),
            rationale: "Balances legal requirements with practical concerns".to_string(),
            implementation_plan: vec![
                "Phase 1: Pilot program in limited jurisdictions".to_string(),
                "Phase 2: Full implementation with review checkpoints".to_string(),
                "Phase 3: Final assessment and adjustments".to_string(),
            ],
            monitoring_requirements: vec![
                "Monthly compliance reports".to_string(),
                "Quarterly stakeholder reviews".to_string(),
            ],
            accepted_risks: vec!["Potential initial resistance from local authorities".to_string()],
        };
        assert_eq!(decision.implementation_plan.len(), 3);
        assert_eq!(decision.monitoring_requirements.len(), 2);
        assert_eq!(decision.accepted_risks.len(), 1);
    }
    #[tokio::test]
    async fn test_ai_assistant_creation() {
        let assistant = AiPortingAssistant::new();
        assert!(assistant.generator.is_none());
        let assistant_default = AiPortingAssistant::default();
        assert!(assistant_default.generator.is_none());
    }
    #[tokio::test]
    async fn test_llm_adaptation_suggestions() {
        let assistant = AiPortingAssistant::new();
        let jp = test_jurisdiction_jp();
        let us = test_jurisdiction_us();
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );
        let suggestions = assistant
            .generate_adaptation_suggestions(&statute, &jp, &us)
            .await
            .unwrap();
        assert!(!suggestions.is_empty());
        let first = &suggestions[0];
        assert_eq!(first.statute_id, "test");
        assert!(first.confidence > 0.0 && first.confidence <= 1.0);
        assert!(!first.suggestion.is_empty());
        assert!(matches!(
            first.category,
            AdaptationCategory::Procedural | AdaptationCategory::Cultural
        ));
    }
    #[tokio::test]
    async fn test_similar_statute_discovery() {
        let assistant = AiPortingAssistant::new();
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Rights"));
        let jurisdictions = vec![test_jurisdiction_jp(), test_jurisdiction_us()];
        let similar = assistant
            .discover_similar_statutes(&statute, &jurisdictions)
            .await
            .unwrap();
        assert!(!similar.is_empty());
        for sim in &similar {
            assert!(sim.similarity_score > 0.0 && sim.similarity_score <= 1.0);
            assert!(!sim.matching_features.is_empty());
        }
        for i in 1..similar.len() {
            assert!(similar[i - 1].similarity_score >= similar[i].similarity_score);
        }
    }
    #[tokio::test]
    async fn test_gap_analysis() {
        let assistant = AiPortingAssistant::new();
        let jp = test_jurisdiction_jp();
        let us = test_jurisdiction_us();
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Rights"));
        let gap_analysis = assistant.analyze_gaps(&statute, &jp, &us).await.unwrap();
        assert_eq!(gap_analysis.source_statute_id, "test");
        assert!(gap_analysis.coverage_score >= 0.0 && gap_analysis.coverage_score <= 1.0);
        assert!(!gap_analysis.gaps.is_empty());
        assert!(!gap_analysis.recommendations.is_empty());
        for gap in &gap_analysis.gaps {
            assert!(!gap.description.is_empty());
            assert!(!gap.missing_element.is_empty());
            assert!(!gap.solutions.is_empty());
        }
    }
    #[tokio::test]
    async fn test_cultural_sensitivity_analysis() {
        let assistant = AiPortingAssistant::new();
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Rights"));
        let mut params = CulturalParams::for_country("US");
        params.prohibitions.push("alcohol".to_string());
        let jurisdiction = Jurisdiction::new("TEST", "Test", Locale::new("en").with_country("US"))
            .with_legal_system(LegalSystem::CommonLaw)
            .with_cultural_params(params);
        let analysis = assistant
            .check_cultural_sensitivity(&statute, &jurisdiction)
            .await
            .unwrap();
        assert_eq!(analysis.statute_id, "test");
        assert!(analysis.sensitivity_score >= 0.0 && analysis.sensitivity_score <= 1.0);
        assert!(!analysis.issues.is_empty());
        assert!(!analysis.assessment.is_empty());
        for issue in &analysis.issues {
            assert!(!issue.description.is_empty());
            assert!(!issue.explanation.is_empty());
        }
    }
    #[tokio::test]
    async fn test_plain_language_explanation() {
        let assistant = AiPortingAssistant::new();
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );
        for audience_level in [
            AudienceLevel::GeneralPublic,
            AudienceLevel::Business,
            AudienceLevel::Government,
            AudienceLevel::Legal,
            AudienceLevel::Academic,
        ] {
            let explanation = assistant
                .generate_plain_explanation(&statute, audience_level)
                .await
                .unwrap();
            assert_eq!(explanation.statute_id, "test");
            assert_eq!(explanation.audience_level, audience_level);
            assert!(!explanation.summary.is_empty());
            assert!(!explanation.explanation.is_empty());
            assert!(!explanation.key_points.is_empty());
            assert!(explanation.readability_score > 0.0 && explanation.readability_score <= 1.0);
        }
    }
    #[test]
    fn test_adaptation_category() {
        let categories = vec![
            AdaptationCategory::Terminology,
            AdaptationCategory::Procedural,
            AdaptationCategory::Cultural,
            AdaptationCategory::Numerical,
            AdaptationCategory::Structural,
            AdaptationCategory::LegalPrinciple,
            AdaptationCategory::Compliance,
        ];
        for category in categories {
            assert!(matches!(
                category,
                AdaptationCategory::Terminology
                    | AdaptationCategory::Procedural
                    | AdaptationCategory::Cultural
                    | AdaptationCategory::Numerical
                    | AdaptationCategory::Structural
                    | AdaptationCategory::LegalPrinciple
                    | AdaptationCategory::Compliance
            ));
        }
    }
    #[test]
    fn test_gap_types() {
        let gap_types = vec![
            GapType::MissingConcept,
            GapType::MissingProcedure,
            GapType::MissingEnforcement,
            GapType::MissingSafeguard,
            GapType::InsufficientSpecificity,
            GapType::MissingCulturalElement,
        ];
        for gap_type in gap_types {
            assert!(matches!(
                gap_type,
                GapType::MissingConcept
                    | GapType::MissingProcedure
                    | GapType::MissingEnforcement
                    | GapType::MissingSafeguard
                    | GapType::InsufficientSpecificity
                    | GapType::MissingCulturalElement
            ));
        }
    }
    #[test]
    fn test_cultural_issue_types() {
        let issue_types = vec![
            CulturalIssueType::Religious,
            CulturalIssueType::Traditional,
            CulturalIssueType::SocialNorm,
            CulturalIssueType::Gender,
            CulturalIssueType::Family,
            CulturalIssueType::Language,
            CulturalIssueType::Historical,
        ];
        for issue_type in issue_types {
            assert!(matches!(
                issue_type,
                CulturalIssueType::Religious
                    | CulturalIssueType::Traditional
                    | CulturalIssueType::SocialNorm
                    | CulturalIssueType::Gender
                    | CulturalIssueType::Family
                    | CulturalIssueType::Language
                    | CulturalIssueType::Historical
            ));
        }
    }
    #[test]
    fn test_feature_types() {
        let feature_types = vec![
            FeatureType::LegalEffect,
            FeatureType::Structure,
            FeatureType::Terminology,
            FeatureType::Scope,
            FeatureType::Conditions,
            FeatureType::Remedies,
        ];
        for feature_type in feature_types {
            assert!(matches!(
                feature_type,
                FeatureType::LegalEffect
                    | FeatureType::Structure
                    | FeatureType::Terminology
                    | FeatureType::Scope
                    | FeatureType::Conditions
                    | FeatureType::Remedies
            ));
        }
    }
    #[test]
    fn test_audience_levels() {
        let levels = [
            AudienceLevel::GeneralPublic,
            AudienceLevel::Business,
            AudienceLevel::Government,
            AudienceLevel::Legal,
            AudienceLevel::Academic,
        ];
        for level in levels {
            assert!(matches!(
                level,
                AudienceLevel::GeneralPublic
                    | AudienceLevel::Business
                    | AudienceLevel::Government
                    | AudienceLevel::Legal
                    | AudienceLevel::Academic
            ));
        }
    }
    #[tokio::test]
    async fn test_multi_hop_port() {
        let jp = test_jurisdiction_jp();
        let us = test_jurisdiction_us();
        let uk = Jurisdiction::new("UK", "United Kingdom", Locale::new("en").with_country("GB"))
            .with_legal_system(LegalSystem::CommonLaw)
            .with_cultural_params(CulturalParams::for_country("GB"));
        let engine = PortingEngine::new(jp, us);
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };
        let chain = engine
            .multi_hop_port(&statute, &[uk], &options)
            .await
            .unwrap();
        assert_eq!(chain.hop_results.len(), 2);
        assert!(chain.chain_score >= 0.0 && chain.chain_score <= 1.0);
        assert_eq!(chain.source_jurisdiction, "JP");
        assert_eq!(chain.target_jurisdiction, "US");
        assert_eq!(chain.intermediate_hops.len(), 1);
    }
    #[test]
    fn test_record_history() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let options = PortingOptions::default();
        let history = engine.record_history(
            "test-statute".to_string(),
            "user-001".to_string(),
            &options,
            true,
            None,
        );
        assert_eq!(history.statute_id, "test-statute");
        assert_eq!(history.user, "user-001");
        assert!(history.success);
        assert!(history.error.is_none());
    }
    #[test]
    fn test_build_lineage() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let options = PortingOptions::default();
        let history = vec![
            engine.record_history(
                "statute-1".to_string(),
                "user".to_string(),
                &options,
                true,
                None,
            ),
            engine.record_history(
                "statute-2".to_string(),
                "user".to_string(),
                &options,
                true,
                None,
            ),
        ];
        let lineage = engine.build_lineage("original-id".to_string(), "JP".to_string(), &history);
        assert_eq!(lineage.original_id, "original-id");
        assert_eq!(lineage.original_jurisdiction, "JP");
        assert!(lineage.total_ports <= 2);
    }
    #[test]
    fn test_generate_diff() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new(
            "test",
            "Original Title",
            Effect::new(EffectType::Grant, "Test"),
        );
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();
        let diff = engine.generate_diff(&statute, &ported);
        assert_eq!(diff.original_id, "test");
        assert!(diff.similarity_score >= 0.0 && diff.similarity_score <= 1.0);
        assert!(!diff.differences.is_empty());
    }
    #[test]
    fn test_export_diff_markdown() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Original", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();
        let diff = engine.generate_diff(&statute, &ported);
        let md = engine.export_diff_markdown(&diff);
        assert!(md.contains("# Statute Diff"));
        assert!(md.contains("Similarity Score"));
        assert!(md.contains("```diff"));
    }
    #[test]
    fn test_jurisdiction_profile_creation() {
        let profile = JurisdictionProfile::new(
            String::from("US"),
            String::from("United States"),
            LegalSystemType::CommonLaw,
        );
        assert_eq!(profile.code, "US");
        assert_eq!(profile.name, "United States");
        assert_eq!(profile.legal_system, LegalSystemType::CommonLaw);
        assert!(profile.official_languages.is_empty());
    }
    #[test]
    fn test_court_hierarchy() {
        let mut hierarchy = CourtHierarchy::new();
        hierarchy.add_court(Court {
            name: String::from("Supreme Court"),
            level: CourtLevel::Supreme,
            jurisdiction: String::from("Federal"),
            precedent_setting: true,
            judges: Some(9),
            url: None,
        });
        hierarchy.add_court(Court {
            name: String::from("District Court"),
            level: CourtLevel::District,
            jurisdiction: String::from("Regional"),
            precedent_setting: false,
            judges: Some(100),
            url: None,
        });
        assert_eq!(hierarchy.courts.len(), 2);
        assert_eq!(hierarchy.courts_by_level(CourtLevel::Supreme).len(), 1);
        assert_eq!(hierarchy.courts_by_level(CourtLevel::District).len(), 1);
    }
    #[test]
    fn test_legislative_process() {
        let process = LegislativeProcess::new(String::from("Congress"), String::from("House"))
            .with_upper_house(String::from("Senate"));
        assert!(process.is_bicameral);
        assert_eq!(process.upper_house, Some(String::from("Senate")));
        assert!(process.stages.contains(&LegislativeStage::UpperHouse));
    }
    #[test]
    fn test_constitutional_framework() {
        let mut framework = ConstitutionalFramework::new();
        framework.add_feature(ConstitutionalFeature::WrittenConstitution);
        framework.add_feature(ConstitutionalFeature::BillOfRights);
        framework.add_feature(ConstitutionalFeature::Federalism);
        assert!(framework.has_feature(ConstitutionalFeature::WrittenConstitution));
        assert!(framework.has_feature(ConstitutionalFeature::BillOfRights));
        assert!(framework.has_feature(ConstitutionalFeature::Federalism));
        assert!(!framework.has_feature(ConstitutionalFeature::ParliamentarySovereignty));
        assert_eq!(framework.features.len(), 3);
    }
    #[test]
    fn test_jurisdiction_compatibility_score() {
        let us = JurisdictionProfile::new(
            String::from("US"),
            String::from("United States"),
            LegalSystemType::CommonLaw,
        );
        let gb = JurisdictionProfile::new(
            String::from("GB"),
            String::from("United Kingdom"),
            LegalSystemType::CommonLaw,
        );
        let jp = JurisdictionProfile::new(
            String::from("JP"),
            String::from("Japan"),
            LegalSystemType::CivilLaw,
        );
        let us_gb_score = us.compatibility_score(&gb);
        let us_jp_score = us.compatibility_score(&jp);
        assert!(us_gb_score > us_jp_score);
        assert!((0.0..=1.0).contains(&us_gb_score));
        assert!((0.0..=1.0).contains(&us_jp_score));
    }
    #[test]
    fn test_jurisdiction_database() {
        let mut db = JurisdictionDatabase::new();
        let us = JurisdictionProfile::new(
            String::from("US"),
            String::from("United States"),
            LegalSystemType::CommonLaw,
        );
        let jp = JurisdictionProfile::new(
            String::from("JP"),
            String::from("Japan"),
            LegalSystemType::CivilLaw,
        );
        db.add_profile(us);
        db.add_profile(jp);
        assert!(db.get_profile("US").is_some());
        assert!(db.get_profile("JP").is_some());
        assert!(db.get_profile("FR").is_none());
        assert_eq!(db.list_codes().len(), 2);
    }
    #[test]
    fn test_find_by_legal_system() {
        let db = JurisdictionDatabase::with_major_jurisdictions();
        let common_law = db.find_by_legal_system(LegalSystemType::CommonLaw);
        let civil_law = db.find_by_legal_system(LegalSystemType::CivilLaw);
        assert!(common_law.len() >= 2);
        assert!(civil_law.len() >= 3);
    }
    #[test]
    fn test_find_compatible_jurisdictions() {
        let db = JurisdictionDatabase::with_major_jurisdictions();
        let compatible = db.find_compatible("US", 0.5);
        assert!(!compatible.is_empty());
        for i in 0..compatible.len().saturating_sub(1) {
            assert!(compatible[i].1 >= compatible[i + 1].1);
        }
        for (_, score) in &compatible {
            assert!(*score >= 0.5);
        }
    }
    #[test]
    fn test_major_jurisdictions_database() {
        let db = JurisdictionDatabase::with_major_jurisdictions();
        let us = db.get_profile("US").expect("US profile should exist");
        assert_eq!(us.name, "United States");
        assert_eq!(us.legal_system, LegalSystemType::CommonLaw);
        assert!(
            us.constitutional_framework
                .has_feature(ConstitutionalFeature::Federalism)
        );
        assert!(us.legislative_process.is_bicameral);
        assert!(us.court_hierarchy.has_jury_trials);
        let jp = db.get_profile("JP").expect("JP profile should exist");
        assert_eq!(jp.name, "Japan");
        assert_eq!(jp.legal_system, LegalSystemType::CivilLaw);
        assert!(
            jp.constitutional_framework
                .has_feature(ConstitutionalFeature::ParliamentarySystem)
        );
        assert!(!jp.court_hierarchy.has_jury_trials);
        let de = db.get_profile("DE").expect("DE profile should exist");
        assert_eq!(de.name, "Germany");
        assert!(
            de.constitutional_framework
                .has_feature(ConstitutionalFeature::Federalism)
        );
        assert!(de.court_hierarchy.constitutional_court.is_some());
        let gb = db.get_profile("GB").expect("GB profile should exist");
        assert_eq!(gb.name, "United Kingdom");
        assert!(!gb.constitutional_framework.has_written_constitution);
        assert!(
            gb.constitutional_framework
                .has_feature(ConstitutionalFeature::ParliamentarySovereignty)
        );
        let fr = db.get_profile("FR").expect("FR profile should exist");
        assert_eq!(fr.name, "France");
        assert!(
            fr.constitutional_framework
                .has_feature(ConstitutionalFeature::SemiPresidentialSystem)
        );
    }
    #[test]
    fn test_court_level_ordering() {
        assert!(CourtLevel::Local < CourtLevel::District);
        assert!(CourtLevel::District < CourtLevel::Appellate);
        assert!(CourtLevel::Appellate < CourtLevel::Supreme);
        assert!(CourtLevel::Supreme < CourtLevel::International);
    }
    #[test]
    fn test_legislative_stage_ordering() {
        assert!(LegislativeStage::Drafting < LegislativeStage::Committee);
        assert!(LegislativeStage::Committee < LegislativeStage::FirstReading);
        assert!(LegislativeStage::FirstReading < LegislativeStage::SecondReading);
        assert!(LegislativeStage::SecondReading < LegislativeStage::ThirdReading);
        assert!(LegislativeStage::ThirdReading < LegislativeStage::UpperHouse);
        assert!(LegislativeStage::UpperHouse < LegislativeStage::Executive);
        assert!(LegislativeStage::Executive < LegislativeStage::Publication);
    }
    #[test]
    fn test_concept_equivalence() {
        let equiv = ConceptEquivalence::new(String::from("contract"), String::from("契約"), 0.95)
            .with_context(String::from("civil law"))
            .with_notes(String::from("Direct translation"));
        assert_eq!(equiv.source_concept, "contract");
        assert_eq!(equiv.target_concept, "契約");
        assert_eq!(equiv.equivalence_score, 0.95);
        assert!((equiv.semantic_distance - 0.05).abs() < 0.0001);
        assert_eq!(equiv.context.len(), 1);
        assert!(equiv.notes.is_some());
    }
    #[test]
    fn test_concept_equivalence_database() {
        let mut db = ConceptEquivalenceDatabase::new();
        db.add_equivalence(
            String::from("US->JP"),
            ConceptEquivalence::new(String::from("contract"), String::from("契約"), 0.95),
        );
        db.add_equivalence(
            String::from("US->JP"),
            ConceptEquivalence::new(String::from("tort"), String::from("不法行為"), 0.9),
        );
        let matches = db.find_equivalences("US", "JP", "contract");
        assert_eq!(matches.len(), 1);
        let best = db.best_match("US", "JP", "contract");
        assert!(best.is_some());
        assert_eq!(best.unwrap().target_concept, "契約");
    }
    #[test]
    fn test_term_translation() {
        let translation = TermTranslation::new(
            String::from("felony"),
            String::from("US"),
            String::from("重罪"),
            String::from("JP"),
            0.9,
            true,
        );
        assert_eq!(translation.source_term, "felony");
        assert_eq!(translation.target_term, "重罪");
        assert_eq!(translation.accuracy, 0.9);
        assert!(translation.is_direct);
    }
    #[test]
    fn test_term_translation_matrix() {
        let matrix = TermTranslationMatrix::with_common_translations();
        let translations = matrix.find_translations("US", "JP", "felony");
        assert!(!translations.is_empty());
        let best = matrix.best_translation("US", "JP", "felony", None);
        assert!(best.is_some());
        assert_eq!(best.unwrap().target_term, "重罪");
    }
    #[test]
    fn test_term_translation_context() {
        let mut matrix = TermTranslationMatrix::new();
        let mut criminal_trans = TermTranslation::new(
            String::from("charge"),
            String::from("US"),
            String::from("起訴"),
            String::from("JP"),
            0.9,
            true,
        );
        criminal_trans.valid_contexts = vec![String::from("criminal")];
        let mut civil_trans = TermTranslation::new(
            String::from("charge"),
            String::from("US"),
            String::from("料金"),
            String::from("JP"),
            0.8,
            true,
        );
        civil_trans.valid_contexts = vec![String::from("civil"), String::from("contract")];
        matrix.add_translation(criminal_trans);
        matrix.add_translation(civil_trans);
        let criminal_best = matrix.best_translation("US", "JP", "charge", Some("criminal"));
        assert_eq!(criminal_best.unwrap().target_term, "起訴");
        let civil_best = matrix.best_translation("US", "JP", "charge", Some("civil"));
        assert_eq!(civil_best.unwrap().target_term, "料金");
    }
    #[test]
    fn test_semantic_distance_calculator() {
        let mut concept_db = ConceptEquivalenceDatabase::new();
        concept_db.add_equivalence(
            String::from("US->JP"),
            ConceptEquivalence::new(String::from("contract"), String::from("契約"), 0.95),
        );
        let calculator = SemanticDistanceCalculator::new(concept_db);
        let distance = calculator.calculate_distance("US", "JP", "contract", "契約");
        assert!((0.0..=1.0).contains(&distance));
        assert!(distance < 0.1);
    }
    #[test]
    fn test_levenshtein_distance() {
        let concept_db = ConceptEquivalenceDatabase::new();
        let calculator = SemanticDistanceCalculator::new(concept_db);
        let dist1 = calculator.calculate_distance("US", "JP", "test", "test");
        assert_eq!(dist1, 0.0);
        let dist2 = calculator.calculate_distance("US", "JP", "contract", "compact");
        assert!(dist2 > 0.0 && dist2 < 1.0);
    }
    #[test]
    fn test_context_aware_term_mapper() {
        let matrix = TermTranslationMatrix::with_common_translations();
        let mut mapper = ContextAwareTermMapper::new(matrix);
        mapper.add_context_rule(
            String::from("criminal"),
            vec![String::from("crime"), String::from("offense")],
        );
        let mapped = mapper.map_term("US", "JP", "felony", "serious crime");
        assert!(mapped.is_some());
        assert_eq!(mapped.unwrap(), "重罪");
    }
    #[test]
    fn test_legal_dictionary() {
        let dict = LegalDictionary::us_dictionary();
        assert_eq!(dict.jurisdiction, "US");
        assert!(!dict.terms.is_empty());
        let felony = dict.find_term("felony");
        assert!(felony.is_some());
        assert_eq!(felony.unwrap().domain, "criminal");
        let criminal_terms = dict.get_by_domain("criminal");
        assert!(criminal_terms.len() >= 2);
    }
    #[test]
    fn test_japan_dictionary() {
        let dict = LegalDictionary::japan_dictionary();
        assert_eq!(dict.jurisdiction, "JP");
        assert!(!dict.terms.is_empty());
        let felony = dict.find_term("重罪");
        assert!(felony.is_some());
        let criminal_terms = dict.get_by_domain("criminal");
        assert!(criminal_terms.len() >= 2);
    }
    #[test]
    fn test_legal_term_creation() {
        let term = LegalTerm::new(
            String::from("contract"),
            String::from("An agreement between parties"),
            String::from("US"),
            String::from("civil"),
        );
        assert_eq!(term.term, "contract");
        assert_eq!(term.jurisdiction, "US");
        assert_eq!(term.domain, "civil");
        assert!(term.related_terms.is_empty());
    }
    #[test]
    fn test_term_translation_matrix_get_terms() {
        let mut matrix = TermTranslationMatrix::new();
        matrix.add_term(LegalTerm::new(
            String::from("felony"),
            String::from("Serious crime"),
            String::from("US"),
            String::from("criminal"),
        ));
        matrix.add_term(LegalTerm::new(
            String::from("tort"),
            String::from("Civil wrong"),
            String::from("US"),
            String::from("civil"),
        ));
        let us_terms = matrix.get_terms("US");
        assert_eq!(us_terms.len(), 2);
        let criminal_terms = matrix.get_terms_by_domain("US", "criminal");
        assert_eq!(criminal_terms.len(), 1);
        assert_eq!(criminal_terms[0].term, "felony");
    }
    #[test]
    fn test_cultural_exception() {
        let exception = CulturalException::new(
            CulturalExceptionType::Religious,
            String::from("US"),
            String::from("Religious accommodation"),
        )
        .with_legal_basis(String::from("Title VII"))
        .with_domain(String::from("employment"));
        assert_eq!(exception.exception_type, CulturalExceptionType::Religious);
        assert_eq!(exception.jurisdiction, "US");
        assert!(exception.legal_basis.is_some());
        assert_eq!(exception.applicable_domains.len(), 1);
    }
    #[test]
    fn test_cultural_exception_registry() {
        let registry = CulturalExceptionRegistry::with_common_exceptions();
        let us_exceptions = registry.get_exceptions("US");
        assert!(!us_exceptions.is_empty());
        let jp_religious = registry.get_by_type("JP", CulturalExceptionType::Religious);
        assert!(!jp_religious.is_empty());
    }
    #[test]
    fn test_holiday_calendar() {
        let mut calendar = HolidayCalendar::new(String::from("US"), CalendarSystem::Gregorian);
        let holiday = Holiday::new(
            String::from("Independence Day"),
            HolidayType::National,
            String::from("US"),
        )
        .with_fixed_date(7, 4)
        .as_legal_holiday();
        calendar.add_holiday(holiday);
        assert_eq!(calendar.holidays.len(), 1);
        assert_eq!(calendar.calendar_system, CalendarSystem::Gregorian);
    }
    #[test]
    fn test_us_calendar() {
        let calendar = HolidayCalendar::us_calendar();
        assert_eq!(calendar.jurisdiction, "US");
        assert_eq!(calendar.calendar_system, CalendarSystem::Gregorian);
        assert!(calendar.holidays.len() >= 2);
        let national_holidays = calendar.get_by_type(HolidayType::National);
        assert!(national_holidays.len() >= 2);
    }
    #[test]
    fn test_japan_calendar() {
        let calendar = HolidayCalendar::japan_calendar();
        assert_eq!(calendar.jurisdiction, "JP");
        assert_eq!(calendar.calendar_system, CalendarSystem::Japanese);
        assert!(calendar.holidays.len() >= 2);
    }
}
