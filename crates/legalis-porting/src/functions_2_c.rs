//! Auto-generated module: tests for legalis-porting.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[cfg(test)]
mod tests {
    use crate::*;
    use legalis_core::{Effect, EffectType, Statute};
    use legalis_i18n::{CulturalParams, Jurisdiction, LegalSystem, Locale};
    use std::collections::HashMap;
    #[allow(dead_code)]
    fn test_jurisdiction_jp() -> Jurisdiction {
        Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
            .with_legal_system(LegalSystem::CivilLaw)
            .with_cultural_params(CulturalParams::japan())
    }
    #[allow(dead_code)]
    fn test_jurisdiction_us() -> Jurisdiction {
        Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
            .with_legal_system(LegalSystem::CommonLaw)
            .with_cultural_params(CulturalParams::for_country("US"))
    }
    fn create_test_project() -> PortingProject {
        PortingProject {
            id: "test-project-1".to_string(),
            name: "Test Porting Project".to_string(),
            description: "A test project".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            status: ProjectStatus::InProgress,
            statute_ids: vec!["statute-1".to_string(), "statute-2".to_string()],
            stakeholders: vec![Stakeholder {
                id: "stakeholder-1".to_string(),
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                role: StakeholderRole::ProjectManager,
                notification_preferences: NotificationPreferences {
                    on_status_change: true,
                    on_deadline_approaching: true,
                    on_assignment: false,
                    on_review_request: true,
                    channels: vec![NotificationChannel::Email, NotificationChannel::InApp],
                },
            }],
            timeline: ProjectTimeline {
                start_date: chrono::Utc::now().to_rfc3339(),
                end_date: (chrono::Utc::now() + chrono::Duration::days(180)).to_rfc3339(),
                milestones: vec![],
                current_phase: "Implementation".to_string(),
            },
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            metadata: HashMap::new(),
        }
    }
    fn create_test_ported_statutes(count: usize) -> Vec<PortedStatute> {
        (0..count)
            .map(|i| PortedStatute {
                original_id: format!("statute-{}", i),
                statute: {
                    let id = format!("ported-{}", i);
                    let title = format!("Test Statute {}", i);
                    Statute::new(&id, &title, Effect::new(EffectType::Grant, "Test effect"))
                },
                changes: vec![],
                locale: Locale::new("en").with_country("US"),
                compatibility_score: 0.85,
            })
            .collect()
    }
    fn create_test_ported_statute_with_score(score: f64) -> PortedStatute {
        PortedStatute {
            original_id: "test-statute".to_string(),
            statute: Statute::new(
                "ported-statute",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: score,
        }
    }
    fn create_test_risk_assessment() -> RiskAssessment {
        RiskAssessment {
            risk_score: 0.5,
            risk_level: RiskLevel::Medium,
            risks: vec![Risk {
                id: "risk-1".to_string(),
                category: RiskCategory::Legal,
                description: "Legal system mismatch".to_string(),
                likelihood: RiskLevel::Medium,
                impact: 0.6,
                severity: RiskLevel::Medium,
            }],
            mitigations: vec!["Consult with legal experts".to_string()],
        }
    }
    fn create_test_validation_result(score: f64) -> ValidationResult {
        ValidationResult {
            id: uuid::Uuid::new_v4().to_string(),
            passed: score >= 0.75,
            overall_score: score,
            compliance: TargetJurisdictionComplianceCheck {
                id: uuid::Uuid::new_v4().to_string(),
                is_compliant: true,
                compliance_score: score,
                issues: vec![],
                recommendations: vec![],
                checked_regulations: vec![],
            },
            constitutional: ConstitutionalAnalysis {
                id: uuid::Uuid::new_v4().to_string(),
                is_compatible: true,
                compatibility_score: score,
                issues: vec![],
                relevant_provisions: vec![],
                recommended_amendments: vec![],
            },
            treaty_compliance: TreatyComplianceResult {
                id: uuid::Uuid::new_v4().to_string(),
                is_compliant: true,
                compliance_score: score,
                conflicts: vec![],
                checked_treaties: vec![],
                recommendations: vec![],
            },
            human_rights: HumanRightsAssessment {
                id: uuid::Uuid::new_v4().to_string(),
                impact_score: 0.0,
                affected_rights: vec![],
                vulnerable_groups: vec![],
                mitigation_measures: vec![],
                summary: "No human rights concerns identified".to_string(),
            },
            enforceability: EnforceabilityPrediction {
                id: uuid::Uuid::new_v4().to_string(),
                is_enforceable: true,
                enforceability_score: score,
                challenges: vec![],
                required_mechanisms: vec![],
                estimated_cost: None,
                recommendations: vec![],
            },
            summary: format!("Validation passed with score {:.2}", score),
        }
    }
    #[test]
    fn test_list_projects_by_status() {
        let mut manager = PortingProjectManager::new();
        manager.create_project(
            "P1".to_string(),
            "D1".to_string(),
            "JP".to_string(),
            "US".to_string(),
        );
        let p2 = manager.create_project(
            "P2".to_string(),
            "D2".to_string(),
            "JP".to_string(),
            "US".to_string(),
        );
        manager.update_status(&p2.id, ProjectStatus::InProgress);
        let in_progress = manager.list_projects_by_status(ProjectStatus::InProgress);
        assert_eq!(in_progress.len(), 1);
        let planning = manager.list_projects_by_status(ProjectStatus::Planning);
        assert_eq!(planning.len(), 1);
    }
    #[test]
    fn test_review_workflow_creation() {
        let mut workflow = StakeholderReviewWorkflow::new();
        let step = ReviewWorkflowStep {
            id: "step-1".to_string(),
            name: "Legal Review".to_string(),
            order: 1,
            required_reviewers: vec!["reviewer-1".to_string()],
            optional_reviewers: Vec::new(),
            min_approvals: 1,
            status: ReviewStepStatus::Pending,
            reviews: Vec::new(),
        };
        workflow.create_workflow("project-1".to_string(), vec![step]);
        let status = workflow.get_workflow_status("project-1");
        assert!(status.is_some());
        assert_eq!(status.unwrap().len(), 1);
    }
    #[test]
    fn test_submit_review() {
        let mut workflow = StakeholderReviewWorkflow::new();
        let step = ReviewWorkflowStep {
            id: "step-1".to_string(),
            name: "Legal Review".to_string(),
            order: 1,
            required_reviewers: vec!["reviewer-1".to_string()],
            optional_reviewers: Vec::new(),
            min_approvals: 1,
            status: ReviewStepStatus::Pending,
            reviews: Vec::new(),
        };
        workflow.create_workflow("project-1".to_string(), vec![step]);
        let review = WorkflowReview {
            id: "review-1".to_string(),
            reviewer_id: "reviewer-1".to_string(),
            decision: ReviewDecision::Approve,
            comments: "Looks good".to_string(),
            reviewed_at: chrono::Utc::now().to_rfc3339(),
            recommended_changes: Vec::new(),
        };
        workflow.submit_review("project-1", "step-1", review);
        let status = workflow.get_workflow_status("project-1").unwrap();
        assert_eq!(status[0].reviews.len(), 1);
        assert_eq!(status[0].status, ReviewStepStatus::Approved);
    }
    #[test]
    fn test_version_control_iteration() {
        let mut vc = PortingVersionControl::new();
        let iteration = vc.create_iteration(
            "project-1".to_string(),
            "statute snapshot v1".to_string(),
            "user-1".to_string(),
            "Initial version".to_string(),
        );
        assert_eq!(iteration.iteration_number, 1);
        assert_eq!(iteration.statute_snapshot, "statute snapshot v1");
        assert_eq!(iteration.project_id, "project-1");
    }
    #[test]
    fn test_multiple_iterations() {
        let mut vc = PortingVersionControl::new();
        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "First".to_string(),
        );
        vc.create_iteration(
            "project-1".to_string(),
            "v2".to_string(),
            "user-1".to_string(),
            "Second".to_string(),
        );
        let iterations = vc.get_iterations("project-1").unwrap();
        assert_eq!(iterations.len(), 2);
        assert_eq!(iterations[0].iteration_number, 1);
        assert_eq!(iterations[1].iteration_number, 2);
    }
    #[test]
    fn test_get_specific_iteration() {
        let mut vc = PortingVersionControl::new();
        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "First".to_string(),
        );
        vc.create_iteration(
            "project-1".to_string(),
            "v2".to_string(),
            "user-1".to_string(),
            "Second".to_string(),
        );
        let iteration = vc.get_iteration("project-1", 2).unwrap();
        assert_eq!(iteration.statute_snapshot, "v2");
    }
    #[test]
    fn test_revert_iteration() {
        let mut vc = PortingVersionControl::new();
        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "First".to_string(),
        );
        vc.create_iteration(
            "project-1".to_string(),
            "v2".to_string(),
            "user-1".to_string(),
            "Second".to_string(),
        );
        let reverted = vc.revert_to_iteration("project-1", 1, "user-2".to_string());
        assert!(reverted.is_some());
        let iterations = vc.get_iterations("project-1").unwrap();
        assert_eq!(iterations.len(), 3);
        assert_eq!(iterations[2].statute_snapshot, "v1");
    }
    #[test]
    fn test_create_branch() {
        let mut vc = PortingVersionControl::new();
        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "Version 1".to_string(),
        );
        let branch = vc
            .create_branch(
                "project-1".to_string(),
                "feature-x".to_string(),
                1,
                "user-1".to_string(),
                "Working on feature X".to_string(),
            )
            .unwrap();
        assert_eq!(branch.branch, Some("feature-x".to_string()));
        assert_eq!(branch.statute_snapshot, "v1");
        assert!(branch.tags.contains(&"branch".to_string()));
        let branches = vc.get_branches("project-1");
        assert_eq!(branches.len(), 1);
        assert!(branches.contains(&"feature-x".to_string()));
    }
    #[test]
    fn test_branch_iterations() {
        let mut vc = PortingVersionControl::new();
        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "Version 1".to_string(),
        );
        vc.create_branch(
            "project-1".to_string(),
            "feature-a".to_string(),
            1,
            "user-1".to_string(),
            "Branch A".to_string(),
        );
        vc.create_branch(
            "project-1".to_string(),
            "feature-b".to_string(),
            1,
            "user-1".to_string(),
            "Branch B".to_string(),
        );
        let branch_a_iterations = vc.get_branch_iterations("project-1", "feature-a");
        assert_eq!(branch_a_iterations.len(), 1);
        assert_eq!(branch_a_iterations[0].branch, Some("feature-a".to_string()));
        let branches = vc.get_branches("project-1");
        assert_eq!(branches.len(), 2);
    }
    #[test]
    fn test_merge_branch() {
        let mut vc = PortingVersionControl::new();
        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "Version 1".to_string(),
        );
        vc.create_branch(
            "project-1".to_string(),
            "feature-x".to_string(),
            1,
            "user-1".to_string(),
            "Feature X".to_string(),
        );
        let merged = vc
            .merge_branch(
                "project-1".to_string(),
                "feature-x".to_string(),
                None,
                "user-1".to_string(),
                "Merged feature X".to_string(),
            )
            .unwrap();
        assert_eq!(merged.branch, None);
        assert!(merged.notes.contains("Merged feature-x"));
        assert!(merged.tags.contains(&"merge".to_string()));
    }
    #[test]
    fn test_generate_changelog() {
        let mut vc = PortingVersionControl::new();
        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "Initial version".to_string(),
        );
        vc.create_iteration(
            "project-1".to_string(),
            "v2".to_string(),
            "user-2".to_string(),
            "Updated statute".to_string(),
        );
        let changelog = vc.generate_changelog("project-1").unwrap();
        assert_eq!(changelog.project_id, "project-1");
        assert_eq!(changelog.total_iterations, 2);
        assert_eq!(changelog.entries.len(), 2);
        assert_eq!(changelog.entries[0].iteration_number, 1);
        assert_eq!(changelog.entries[1].iteration_number, 2);
    }
    #[test]
    fn test_changelog_export_markdown() {
        let mut vc = PortingVersionControl::new();
        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "Initial version".to_string(),
        );
        let changelog = vc.generate_changelog("project-1").unwrap();
        let markdown = changelog.to_markdown();
        assert!(markdown.contains("# Porting Changelog"));
        assert!(markdown.contains("project-1"));
        assert!(markdown.contains("## Iteration 1"));
        assert!(markdown.contains("user-1"));
    }
    #[test]
    fn test_changelog_export_json() {
        let mut vc = PortingVersionControl::new();
        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "Initial version".to_string(),
        );
        let changelog = vc.generate_changelog("project-1").unwrap();
        let json = changelog.to_json().unwrap();
        assert!(json.contains("project-1"));
        assert!(json.contains("user-1"));
    }
    #[test]
    fn test_approval_chain_creation() {
        let mut manager = ApprovalChainManager::new();
        let step = ApprovalStep {
            id: "step-1".to_string(),
            name: "Manager Approval".to_string(),
            order: 1,
            approvers: vec!["manager-1".to_string()],
            approval_mode: ApprovalMode::Any,
            status: ApprovalStepStatus::Pending,
            approvals: Vec::new(),
            auto_approve_after: None,
        };
        let chain = manager.create_chain("Test Chain".to_string(), vec![step]);
        assert!(!chain.id.is_empty());
        assert_eq!(chain.name, "Test Chain");
        assert_eq!(chain.status, ApprovalChainStatus::NotStarted);
        assert_eq!(chain.steps.len(), 1);
    }
    #[test]
    fn test_submit_approval() {
        let mut manager = ApprovalChainManager::new();
        let step = ApprovalStep {
            id: "step-1".to_string(),
            name: "Manager Approval".to_string(),
            order: 1,
            approvers: vec!["manager-1".to_string()],
            approval_mode: ApprovalMode::Any,
            status: ApprovalStepStatus::Pending,
            approvals: Vec::new(),
            auto_approve_after: None,
        };
        let chain = manager.create_chain("Test Chain".to_string(), vec![step]);
        let approval = ApprovalRecord {
            id: "approval-1".to_string(),
            approver_id: "manager-1".to_string(),
            approved: true,
            comments: "Approved".to_string(),
            approved_at: chrono::Utc::now().to_rfc3339(),
        };
        manager.submit_approval(&chain.id, "step-1", approval);
        let updated = manager.get_chain(&chain.id).unwrap();
        assert_eq!(updated.steps[0].approvals.len(), 1);
        assert_eq!(updated.steps[0].status, ApprovalStepStatus::Approved);
    }
    #[test]
    fn test_approval_mode_all() {
        let mut manager = ApprovalChainManager::new();
        let step = ApprovalStep {
            id: "step-1".to_string(),
            name: "Multi Approval".to_string(),
            order: 1,
            approvers: vec!["approver-1".to_string(), "approver-2".to_string()],
            approval_mode: ApprovalMode::All,
            status: ApprovalStepStatus::Pending,
            approvals: Vec::new(),
            auto_approve_after: None,
        };
        let chain = manager.create_chain("Test Chain".to_string(), vec![step]);
        let approval1 = ApprovalRecord {
            id: "approval-1".to_string(),
            approver_id: "approver-1".to_string(),
            approved: true,
            comments: "OK".to_string(),
            approved_at: chrono::Utc::now().to_rfc3339(),
        };
        manager.submit_approval(&chain.id, "step-1", approval1);
        let updated = manager.get_chain(&chain.id).unwrap();
        assert_eq!(updated.steps[0].status, ApprovalStepStatus::Pending);
        let approval2 = ApprovalRecord {
            id: "approval-2".to_string(),
            approver_id: "approver-2".to_string(),
            approved: true,
            comments: "OK".to_string(),
            approved_at: chrono::Utc::now().to_rfc3339(),
        };
        manager.submit_approval(&chain.id, "step-1", approval2);
        let updated = manager.get_chain(&chain.id).unwrap();
        assert_eq!(updated.steps[0].status, ApprovalStepStatus::Approved);
    }
    #[test]
    fn test_notification_manager() {
        let mut manager = NotificationManager::new();
        let notification = Notification {
            id: "notif-1".to_string(),
            recipient_id: "user-1".to_string(),
            notification_type: NotificationType::StatusChange,
            title: "Status Changed".to_string(),
            message: "Project status changed to InProgress".to_string(),
            project_id: Some("project-1".to_string()),
            priority: NotificationPriority::Normal,
            created_at: chrono::Utc::now().to_rfc3339(),
            read: false,
            channels: vec![NotificationChannel::Email],
        };
        manager.send_notification(notification);
        let notifications = manager.get_notifications("user-1");
        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].title, "Status Changed");
        assert!(!notifications[0].read);
    }
    #[test]
    fn test_mark_notification_as_read() {
        let mut manager = NotificationManager::new();
        let notification = Notification {
            id: "notif-1".to_string(),
            recipient_id: "user-1".to_string(),
            notification_type: NotificationType::StatusChange,
            title: "Test".to_string(),
            message: "Test message".to_string(),
            project_id: None,
            priority: NotificationPriority::Normal,
            created_at: chrono::Utc::now().to_rfc3339(),
            read: false,
            channels: vec![NotificationChannel::Email],
        };
        manager.send_notification(notification);
        manager.mark_as_read("user-1", "notif-1");
        let notifications = manager.get_notifications("user-1");
        assert!(notifications[0].read);
    }
    #[test]
    fn test_deadline_tracker() {
        let mut manager = NotificationManager::new();
        let deadline = DeadlineTracker {
            id: "deadline-1".to_string(),
            project_id: "project-1".to_string(),
            name: "Final Review".to_string(),
            deadline: "2026-01-15T00:00:00Z".to_string(),
            warning_days: 7,
            status: DeadlineStatus::OnTrack,
            assigned_to: vec!["user-1".to_string()],
        };
        manager.add_deadline(deadline);
        let deadlines = manager.get_deadlines("project-1");
        assert_eq!(deadlines.len(), 1);
        assert_eq!(deadlines[0].name, "Final Review");
    }
    #[test]
    fn test_check_approaching_deadlines() {
        let mut manager = NotificationManager::new();
        let now = chrono::Utc::now();
        let deadline_date = now + chrono::Duration::days(5);
        let deadline = DeadlineTracker {
            id: "deadline-1".to_string(),
            project_id: "project-1".to_string(),
            name: "Urgent Deadline".to_string(),
            deadline: deadline_date.to_rfc3339(),
            warning_days: 7,
            status: DeadlineStatus::Approaching,
            assigned_to: vec!["user-1".to_string()],
        };
        manager.add_deadline(deadline);
        let notifications = manager.check_deadlines();
        assert!(!notifications.is_empty());
        assert_eq!(
            notifications[0].notification_type,
            NotificationType::DeadlineApproaching
        );
    }
    #[test]
    fn test_project_status_enum() {
        assert!(matches!(ProjectStatus::Planning, ProjectStatus::Planning));
        assert!(matches!(
            ProjectStatus::InProgress,
            ProjectStatus::InProgress
        ));
        assert!(matches!(ProjectStatus::Completed, ProjectStatus::Completed));
    }
    #[test]
    fn test_stakeholder_roles() {
        let role = StakeholderRole::LegalExpert;
        assert_eq!(role, StakeholderRole::LegalExpert);
        let roles = [
            StakeholderRole::ProjectManager,
            StakeholderRole::LegalExpert,
            StakeholderRole::TechnicalReviewer,
            StakeholderRole::Approver,
            StakeholderRole::Observer,
            StakeholderRole::Contributor,
        ];
        assert_eq!(roles.len(), 6);
    }
    #[test]
    fn test_notification_channels() {
        let channels = [
            NotificationChannel::Email,
            NotificationChannel::InApp,
            NotificationChannel::Sms,
            NotificationChannel::Webhook,
        ];
        assert_eq!(channels.len(), 4);
    }
    #[test]
    fn test_iteration_change_types() {
        assert!(matches!(
            IterationChangeType::Addition,
            IterationChangeType::Addition
        ));
        assert!(matches!(
            IterationChangeType::Modification,
            IterationChangeType::Modification
        ));
        assert!(matches!(
            IterationChangeType::Deletion,
            IterationChangeType::Deletion
        ));
        assert!(matches!(
            IterationChangeType::Restructure,
            IterationChangeType::Restructure
        ));
    }
    #[test]
    fn test_executive_summary_generator() {
        let generator = ExecutiveSummaryGenerator::new();
        let project = create_test_project();
        let statutes = create_test_ported_statutes(3);
        let summary = generator.generate(&project, &statutes);
        assert_eq!(summary.project_id, project.id);
        assert_eq!(summary.statutes_count, 3);
        assert!(summary.compatibility_score >= 0.0 && summary.compatibility_score <= 1.0);
        assert!(!summary.key_findings.is_empty());
        assert!(!summary.recommendations.is_empty());
        assert!(!summary.stakeholders.is_empty());
    }
    #[test]
    fn test_executive_summary_risk_levels() {
        let generator = ExecutiveSummaryGenerator::new();
        let project = create_test_project();
        let high_compat_statutes = vec![create_test_ported_statute_with_score(0.9)];
        let summary = generator.generate(&project, &high_compat_statutes);
        assert_eq!(summary.risk_level, RiskLevel::Low);
        let low_compat_statutes = vec![create_test_ported_statute_with_score(0.3)];
        let summary = generator.generate(&project, &low_compat_statutes);
        assert_eq!(summary.risk_level, RiskLevel::High);
    }
    #[test]
    fn test_risk_assessment_report_generator() {
        let generator = RiskAssessmentReportGenerator::new();
        let project = create_test_project();
        let risk_assessments = vec![create_test_risk_assessment()];
        let report = generator.generate(&project, &risk_assessments);
        assert_eq!(report.project_id, project.id);
        assert!(report.overall_risk_score >= 0.0 && report.overall_risk_score <= 1.0);
        assert!(!report.risks_by_category.is_empty());
        assert!(!report.mitigation_strategies.is_empty());
    }
    #[test]
    fn test_risk_matrix_categorization() {
        let generator = RiskAssessmentReportGenerator::new();
        let _project = create_test_project();
        let mut risks_by_category: HashMap<RiskCategory, Vec<Risk>> = HashMap::new();
        risks_by_category.insert(
            RiskCategory::Legal,
            vec![
                Risk {
                    id: "risk-1".to_string(),
                    category: RiskCategory::Legal,
                    description: "High-high risk".to_string(),
                    likelihood: RiskLevel::High,
                    impact: 0.9,
                    severity: RiskLevel::High,
                },
                Risk {
                    id: "risk-2".to_string(),
                    category: RiskCategory::Legal,
                    description: "Low-low risk".to_string(),
                    likelihood: RiskLevel::Low,
                    impact: 0.2,
                    severity: RiskLevel::Low,
                },
            ],
        );
        let matrix = generator.build_risk_matrix(&risks_by_category);
        assert!(!matrix.critical.is_empty());
        assert!(!matrix.low.is_empty());
    }
    #[test]
    fn test_implementation_roadmap_generator() {
        let generator = ImplementationRoadmapGenerator::new();
        let project = create_test_project();
        let statutes = create_test_ported_statutes(5);
        let roadmap = generator.generate(&project, &statutes);
        assert_eq!(roadmap.project_id, project.id);
        assert_eq!(roadmap.phases.len(), 4);
        assert!(!roadmap.critical_path.is_empty());
        assert!(!roadmap.resource_requirements.personnel.is_empty());
        assert!(roadmap.estimated_duration_days > 0);
    }
    #[test]
    fn test_implementation_phases_dependencies() {
        let generator = ImplementationRoadmapGenerator::new();
        let project = create_test_project();
        let statutes = create_test_ported_statutes(2);
        let roadmap = generator.generate(&project, &statutes);
        assert!(roadmap.phases[0].dependencies.is_empty());
        assert!(!roadmap.phases[1].dependencies.is_empty());
        assert!(!roadmap.phases[2].dependencies.is_empty());
        assert!(!roadmap.phases[3].dependencies.is_empty());
    }
    #[test]
    fn test_cost_benefit_analyzer() {
        let analyzer = CostBenefitAnalyzer::new();
        let project = create_test_project();
        let roadmap = ImplementationRoadmapGenerator::new()
            .generate(&project, &create_test_ported_statutes(3));
        let statutes = create_test_ported_statutes(3);
        let analysis = analyzer.analyze(&project, &roadmap, &statutes);
        assert_eq!(analysis.project_id, project.id);
        assert!(analysis.total_costs.total_five_year > 0.0);
        assert!(analysis.total_benefits.quantifiable_benefits >= 0.0);
        assert!(analysis.net_present_value.is_finite());
        assert!(!analysis.total_benefits.qualitative_benefits.is_empty());
    }
    #[test]
    fn test_cost_benefit_recommendations() {
        let analyzer = CostBenefitAnalyzer::new();
        let project = create_test_project();
        let high_compat_statutes = vec![
            create_test_ported_statute_with_score(0.95),
            create_test_ported_statute_with_score(0.92),
            create_test_ported_statute_with_score(0.90),
        ];
        let roadmap =
            ImplementationRoadmapGenerator::new().generate(&project, &high_compat_statutes);
        let analysis = analyzer.analyze(&project, &roadmap, &high_compat_statutes);
        assert!(matches!(
            analysis.recommendation,
            CBARecommendation::StronglyRecommend | CBARecommendation::RecommendWithConditions
        ));
    }
    #[test]
    fn test_compliance_certification_manager() {
        let mut manager = ComplianceCertificationManager::new();
        let project_id = "test-project".to_string();
        let validation_results = vec![create_test_validation_result(0.85)];
        let certifier = CertifierInfo {
            name: "John Doe".to_string(),
            organization: "Legal Standards Board".to_string(),
            credentials: vec!["Licensed Attorney".to_string()],
            contact: "john@example.com".to_string(),
        };
        let cert = manager.issue_certification(project_id.clone(), validation_results, certifier);
        assert_eq!(cert.project_id, project_id);
        assert_eq!(cert.certification_level, CertificationLevel::Enhanced);
        assert_eq!(cert.status, CertificationStatus::Certified);
        assert!(cert.signature.is_some());
        assert!(cert.expiration_date.is_some());
    }
    #[test]
    fn test_certification_levels() {
        let mut manager = ComplianceCertificationManager::new();
        let certifier = CertifierInfo {
            name: "Jane Smith".to_string(),
            organization: "Compliance Authority".to_string(),
            credentials: vec!["Certified Auditor".to_string()],
            contact: "jane@example.com".to_string(),
        };
        let full_cert = manager.issue_certification(
            "proj1".to_string(),
            vec![create_test_validation_result(0.96)],
            certifier.clone(),
        );
        assert_eq!(full_cert.certification_level, CertificationLevel::Full);
        let enhanced_cert = manager.issue_certification(
            "proj2".to_string(),
            vec![create_test_validation_result(0.88)],
            certifier.clone(),
        );
        assert_eq!(
            enhanced_cert.certification_level,
            CertificationLevel::Enhanced
        );
        let standard_cert = manager.issue_certification(
            "proj3".to_string(),
            vec![create_test_validation_result(0.78)],
            certifier.clone(),
        );
        assert_eq!(
            standard_cert.certification_level,
            CertificationLevel::Standard
        );
        let provisional_cert = manager.issue_certification(
            "proj4".to_string(),
            vec![create_test_validation_result(0.65)],
            certifier,
        );
        assert_eq!(
            provisional_cert.certification_level,
            CertificationLevel::Provisional
        );
    }
    #[test]
    fn test_certification_revocation() {
        let mut manager = ComplianceCertificationManager::new();
        let certifier = CertifierInfo {
            name: "Test Certifier".to_string(),
            organization: "Test Org".to_string(),
            credentials: vec!["Test Credential".to_string()],
            contact: "test@example.com".to_string(),
        };
        let cert = manager.issue_certification(
            "test-proj".to_string(),
            vec![create_test_validation_result(0.85)],
            certifier,
        );
        let cert_id = cert.id.clone();
        assert!(manager.revoke_certification(&cert_id).is_some());
        let revoked_cert = manager.get_certification(&cert_id).unwrap();
        assert_eq!(revoked_cert.status, CertificationStatus::Revoked);
    }
    #[test]
    fn test_bilateral_agreement_template_library() {
        let library = BilateralAgreementTemplateLibrary::new();
        let templates = library.list_templates();
        assert!(!templates.is_empty());
        let template = library.get_template("general-bilateral").unwrap();
        assert_eq!(template.id, "general-bilateral");
        assert!(!template.sections.is_empty());
        assert!(!template.required_parameters.is_empty());
    }
    #[test]
    fn test_template_agreement_generation() {
        let library = BilateralAgreementTemplateLibrary::new();
        let mut parameters = HashMap::new();
        parameters.insert(
            "source_jurisdiction".to_string(),
            "United States".to_string(),
        );
        parameters.insert("target_jurisdiction".to_string(), "Japan".to_string());
        parameters.insert("purpose".to_string(), "legal cooperation".to_string());
        let agreement = library.generate_agreement("general-bilateral", &parameters);
        assert!(agreement.is_some());
        let text = agreement.unwrap();
        assert!(text.contains("United States"));
        assert!(text.contains("Japan"));
        assert!(text.contains("legal cooperation"));
    }
    #[test]
    fn test_add_custom_template() {
        let mut library = BilateralAgreementTemplateLibrary::new();
        let custom_template = BilateralAgreementTemplate {
            id: "custom-test".to_string(),
            name: "Custom Test Template".to_string(),
            description: "A custom template for testing".to_string(),
            applicable_systems: vec![LegalSystem::CivilLaw],
            sections: vec![TemplateSection {
                section_number: 1,
                title: "Test Section".to_string(),
                content_template: "Test content for {{param1}}".to_string(),
                required: true,
            }],
            required_parameters: vec![TemplateParameter {
                name: "param1".to_string(),
                description: "Test parameter".to_string(),
                parameter_type: ParameterType::String,
                default_value: None,
            }],
            optional_parameters: vec![],
        };
        library.add_template(custom_template);
        assert!(library.get_template("custom-test").is_some());
    }
    #[test]
    fn test_regulatory_sandbox_manager() {
        let mut manager = RegulatorySandboxManager::new();
        let sandbox = manager.create_sandbox(
            "Test Sandbox".to_string(),
            "Testing ported statutes".to_string(),
            vec!["statute-1".to_string(), "statute-2".to_string()],
        );
        assert_eq!(sandbox.status, SandboxStatus::Planning);
        assert_eq!(sandbox.test_statutes.len(), 2);
        assert!(sandbox.scenarios.is_empty());
        assert!(sandbox.results.is_empty());
    }
    #[test]
    fn test_sandbox_scenario_and_results() {
        let mut manager = RegulatorySandboxManager::new();
        let sandbox = manager.create_sandbox(
            "Test Sandbox".to_string(),
            "Testing".to_string(),
            vec!["statute-1".to_string()],
        );
        let sandbox_id = sandbox.id.clone();
        let scenario = TestScenario {
            id: "scenario-1".to_string(),
            name: "Basic Test".to_string(),
            description: "Test basic functionality".to_string(),
            parameters: HashMap::new(),
            expected_outcomes: vec!["Outcome 1".to_string()],
        };
        assert!(manager.add_scenario(&sandbox_id, scenario).is_some());
        assert!(manager.activate_sandbox(&sandbox_id).is_some());
        let sandbox = manager.get_sandbox(&sandbox_id).unwrap();
        assert_eq!(sandbox.status, SandboxStatus::Active);
        let result = SandboxTestResult {
            scenario_id: "scenario-1".to_string(),
            status: TestStatus::Passed,
            actual_outcomes: vec!["Outcome 1".to_string()],
            issues: vec![],
            recommendations: vec![],
            test_date: chrono::Utc::now().to_rfc3339(),
        };
        assert!(manager.record_result(&sandbox_id, result).is_some());
        assert!(manager.complete_sandbox(&sandbox_id).is_some());
        let sandbox = manager.get_sandbox(&sandbox_id).unwrap();
        assert_eq!(sandbox.status, SandboxStatus::Completed);
        assert!(sandbox.end_date.is_some());
    }
    #[test]
    fn test_affected_party_notification_manager() {
        let mut manager = AffectedPartyNotificationManager::new();
        let notification = manager.send_notification(
            "proj-1".to_string(),
            "New Porting Initiative".to_string(),
            "We are porting statutes from jurisdiction A to B".to_string(),
            vec![
                AffectedPartyCategory::GeneralPublic,
                AffectedPartyCategory::LegalProfessionals,
            ],
            Some(30),
        );
        assert_eq!(notification.project_id, "proj-1");
        assert_eq!(notification.affected_categories.len(), 2);
        assert!(notification.response_deadline.is_some());
        assert!(notification.channels.contains(&NotificationChannel::Email));
    }
    #[test]
    fn test_notification_feedback() {
        let mut manager = AffectedPartyNotificationManager::new();
        let notification = manager.send_notification(
            "proj-1".to_string(),
            "Test".to_string(),
            "Content".to_string(),
            vec![AffectedPartyCategory::GeneralPublic],
            None,
        );
        let notif_id = notification.id.clone();
        let feedback = PublicFeedback {
            id: uuid::Uuid::new_v4().to_string(),
            submitter: Some("John Citizen".to_string()),
            category: FeedbackCategory::Support,
            content: "I support this initiative".to_string(),
            submitted_at: chrono::Utc::now().to_rfc3339(),
        };
        assert!(manager.record_feedback(&notif_id, feedback).is_some());
        let feedback_list = manager.list_feedback(&notif_id).unwrap();
        assert_eq!(feedback_list.len(), 1);
    }
    #[test]
    fn test_public_comment_period_manager() {
        let mut manager = PublicCommentPeriodManager::new();
        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Public Comment Period".to_string(),
            "Comments on proposed statute porting".to_string(),
            60,
        );
        assert_eq!(period.status, CommentPeriodStatus::Open);
        assert_eq!(period.project_id, "proj-1");
        assert!(period.comments.is_empty());
        assert!(period.documents.is_empty());
    }
    #[test]
    fn test_comment_period_document_management() {
        let mut manager = PublicCommentPeriodManager::new();
        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Test Period".to_string(),
            "Description".to_string(),
            30,
        );
        let period_id = period.id.clone();
        let document = CommentDocument {
            id: "doc-1".to_string(),
            title: "Draft Statute".to_string(),
            document_type: DocumentType::DraftStatute,
            description: "Draft version for review".to_string(),
            url: "https://example.com/draft.pdf".to_string(),
        };
        assert!(manager.add_document(&period_id, document).is_some());
        let period = manager.get_period(&period_id).unwrap();
        assert_eq!(period.documents.len(), 1);
    }
    #[test]
    fn test_comment_submission() {
        let mut manager = PublicCommentPeriodManager::new();
        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Test Period".to_string(),
            "Description".to_string(),
            30,
        );
        let period_id = period.id.clone();
        let comment = PublicComment {
            id: uuid::Uuid::new_v4().to_string(),
            commenter: CommenterInfo {
                name: Some("Jane Doe".to_string()),
                organization: Some("Citizens Alliance".to_string()),
                email: Some("jane@example.com".to_string()),
                affiliation: AffectedPartyCategory::GeneralPublic,
            },
            comment_text: "I have concerns about section 3".to_string(),
            document_id: None,
            section_reference: Some("Section 3".to_string()),
            submitted_at: chrono::Utc::now().to_rfc3339(),
            category: FeedbackCategory::Concern,
        };
        assert!(manager.submit_comment(&period_id, comment).is_some());
        let comments = manager.list_comments(&period_id).unwrap();
        assert_eq!(comments.len(), 1);
    }
    #[test]
    fn test_comment_period_extension() {
        let mut manager = PublicCommentPeriodManager::new();
        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Test Period".to_string(),
            "Description".to_string(),
            30,
        );
        let period_id = period.id.clone();
        let original_end = period.end_date.clone();
        assert!(manager.extend_period(&period_id, 15).is_some());
        let period = manager.get_period(&period_id).unwrap();
        assert_eq!(period.status, CommentPeriodStatus::Extended);
        assert_ne!(period.end_date, original_end);
    }
    #[test]
    fn test_comment_period_closure() {
        let mut manager = PublicCommentPeriodManager::new();
        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Test Period".to_string(),
            "Description".to_string(),
            30,
        );
        let period_id = period.id.clone();
        assert!(manager.close_period(&period_id).is_some());
        let period = manager.get_period(&period_id).unwrap();
        assert_eq!(period.status, CommentPeriodStatus::Closed);
        let comment = PublicComment {
            id: uuid::Uuid::new_v4().to_string(),
            commenter: CommenterInfo {
                name: None,
                organization: None,
                email: None,
                affiliation: AffectedPartyCategory::GeneralPublic,
            },
            comment_text: "Late comment".to_string(),
            document_id: None,
            section_reference: None,
            submitted_at: chrono::Utc::now().to_rfc3339(),
            category: FeedbackCategory::Question,
        };
        assert!(manager.submit_comment(&period_id, comment).is_none());
    }
    #[test]
    fn test_comment_summary_generation() {
        let mut manager = PublicCommentPeriodManager::new();
        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Test Period".to_string(),
            "Description".to_string(),
            30,
        );
        let period_id = period.id.clone();
        for i in 0..5 {
            let comment = PublicComment {
                id: format!("comment-{}", i),
                commenter: CommenterInfo {
                    name: Some(format!("Commenter {}", i)),
                    organization: None,
                    email: None,
                    affiliation: if i % 2 == 0 {
                        AffectedPartyCategory::GeneralPublic
                    } else {
                        AffectedPartyCategory::Businesses
                    },
                },
                comment_text: format!("Comment {}", i),
                document_id: None,
                section_reference: None,
                submitted_at: chrono::Utc::now().to_rfc3339(),
                category: if i % 2 == 0 {
                    FeedbackCategory::Support
                } else {
                    FeedbackCategory::Concern
                },
            };
            manager.submit_comment(&period_id, comment).unwrap();
        }
        let summary = manager.generate_comment_summary(&period_id).unwrap();
        assert_eq!(summary.total_comments, 5);
        assert!(!summary.category_breakdown.is_empty());
        assert!(!summary.affiliation_breakdown.is_empty());
        assert!(!summary.key_themes.is_empty());
    }
    #[test]
    fn test_discussion_thread() {
        let mut manager = DiscussionThreadManager::new();
        let thread = manager.create_thread(
            "project-1".to_string(),
            "Section 5 Discussion".to_string(),
            "Discuss changes to section 5".to_string(),
            "user-1".to_string(),
            vec!["section-5".to_string()],
        );
        assert!(!thread.id.is_empty());
        assert_eq!(thread.status, ThreadStatus::Open);
        assert_eq!(thread.project_id, "project-1");
    }
    #[test]
    fn test_discussion_thread_comments() {
        let mut manager = DiscussionThreadManager::new();
        let thread = manager.create_thread(
            "project-1".to_string(),
            "Test Thread".to_string(),
            "Context".to_string(),
            "user-1".to_string(),
            vec![],
        );
        let comment1 = manager
            .add_comment(
                &thread.id,
                "user-1".to_string(),
                "First comment".to_string(),
                None,
            )
            .unwrap();
        let _reply = manager
            .add_comment(
                &thread.id,
                "user-2".to_string(),
                "Reply to first".to_string(),
                Some(comment1.id.clone()),
            )
            .unwrap();
        let thread_after = manager.get_thread(&thread.id).unwrap();
        assert_eq!(thread_after.comments.len(), 1);
        assert_eq!(thread_after.comments[0].replies.len(), 1);
    }
    #[test]
    fn test_upvote_comment() {
        let mut manager = DiscussionThreadManager::new();
        let thread = manager.create_thread(
            "project-1".to_string(),
            "Test".to_string(),
            "Context".to_string(),
            "user-1".to_string(),
            vec![],
        );
        let comment = manager
            .add_comment(
                &thread.id,
                "user-1".to_string(),
                "Comment".to_string(),
                None,
            )
            .unwrap();
        manager
            .upvote_comment(&thread.id, &comment.id, "user-2".to_string())
            .unwrap();
        let thread_after = manager.get_thread(&thread.id).unwrap();
        assert_eq!(thread_after.comments[0].upvotes, 1);
    }
    #[test]
    fn test_resolve_thread() {
        let mut manager = DiscussionThreadManager::new();
        let thread = manager.create_thread(
            "project-1".to_string(),
            "Test".to_string(),
            "Context".to_string(),
            "user-1".to_string(),
            vec![],
        );
        manager
            .resolve_thread(&thread.id, "user-1".to_string())
            .unwrap();
        let thread_after = manager.get_thread(&thread.id).unwrap();
        assert_eq!(thread_after.status, ThreadStatus::Resolved);
        assert_eq!(thread_after.resolved_by, Some("user-1".to_string()));
    }
    #[test]
    fn test_voting_creation() {
        let mut manager = VotingManager::new();
        let options = vec![
            VoteOption {
                id: "opt-1".to_string(),
                text: "Option 1".to_string(),
                description: "First option".to_string(),
                vote_count: 0,
            },
            VoteOption {
                id: "opt-2".to_string(),
                text: "Option 2".to_string(),
                description: "Second option".to_string(),
                vote_count: 0,
            },
        ];
        let vote = manager.create_vote(
            "project-1".to_string(),
            "Test Vote".to_string(),
            "Vote on approach".to_string(),
            VoteType::SingleChoice,
            options,
            vec!["user-1".to_string(), "user-2".to_string()],
            24,
        );
        assert!(!vote.id.is_empty());
        assert_eq!(vote.status, VoteStatus::Active);
    }
    #[test]
    fn test_cast_vote() {
        let mut manager = VotingManager::new();
        let options = vec![VoteOption {
            id: "opt-1".to_string(),
            text: "Option 1".to_string(),
            description: "First option".to_string(),
            vote_count: 0,
        }];
        let vote = manager.create_vote(
            "project-1".to_string(),
            "Test".to_string(),
            "Description".to_string(),
            VoteType::SingleChoice,
            options,
            vec!["user-1".to_string()],
            24,
        );
        manager
            .cast_vote(&vote.id, "user-1".to_string(), vec!["opt-1".to_string()])
            .unwrap();
        let vote_after = manager.get_vote(&vote.id).unwrap();
        assert_eq!(vote_after.votes_cast.len(), 1);
    }
    #[test]
    fn test_close_vote() {
        let mut manager = VotingManager::new();
        let options = vec![
            VoteOption {
                id: "opt-1".to_string(),
                text: "Option 1".to_string(),
                description: "First".to_string(),
                vote_count: 0,
            },
            VoteOption {
                id: "opt-2".to_string(),
                text: "Option 2".to_string(),
                description: "Second".to_string(),
                vote_count: 0,
            },
        ];
        let vote = manager.create_vote(
            "project-1".to_string(),
            "Test".to_string(),
            "Desc".to_string(),
            VoteType::SingleChoice,
            options,
            vec!["user-1".to_string(), "user-2".to_string()],
            24,
        );
        manager
            .cast_vote(&vote.id, "user-1".to_string(), vec!["opt-1".to_string()])
            .unwrap();
        let result = manager.close_vote(&vote.id).unwrap();
        assert_eq!(result.total_eligible, 2);
        assert_eq!(result.total_votes, 1);
        assert_eq!(result.participation_rate, 0.5);
    }
    #[test]
    fn test_stakeholder_impact_tracker() {
        let mut tracker = StakeholderImpactTracker::new();
        let impact = tracker.record_impact(
            "project-1".to_string(),
            "stakeholder-1".to_string(),
            StakeholderImpactLevel::High,
            StakeholderImpactCategory::Economic,
            "Significant cost increase".to_string(),
            0.8,
            ImpactTimeframe::ShortTerm,
            vec!["Budget allocation".to_string()],
        );
        assert!(!impact.id.is_empty());
        assert_eq!(impact.impact_level, StakeholderImpactLevel::High);
        assert!(!impact.notification_sent);
    }
    #[test]
    fn test_stakeholder_impact_notifications() {
        let mut tracker = StakeholderImpactTracker::new();
        let impact = tracker.record_impact(
            "project-1".to_string(),
            "stakeholder-1".to_string(),
            StakeholderImpactLevel::Critical,
            StakeholderImpactCategory::Legal,
            "Critical legal issue".to_string(),
            0.9,
            ImpactTimeframe::Immediate,
            vec![],
        );
        let unnotified = tracker.get_unnotified_critical_impacts("project-1");
        assert_eq!(unnotified.len(), 1);
        tracker.mark_notified("project-1", &impact.id).unwrap();
        let unnotified_after = tracker.get_unnotified_critical_impacts("project-1");
        assert_eq!(unnotified_after.len(), 0);
    }
    #[test]
    fn test_stakeholder_impact_summary() {
        let mut tracker = StakeholderImpactTracker::new();
        tracker.record_impact(
            "project-1".to_string(),
            "stakeholder-1".to_string(),
            StakeholderImpactLevel::High,
            StakeholderImpactCategory::Economic,
            "Impact 1".to_string(),
            0.8,
            ImpactTimeframe::ShortTerm,
            vec![],
        );
        tracker.record_impact(
            "project-1".to_string(),
            "stakeholder-2".to_string(),
            StakeholderImpactLevel::Medium,
            StakeholderImpactCategory::Operational,
            "Impact 2".to_string(),
            0.5,
            ImpactTimeframe::MediumTerm,
            vec![],
        );
        let summary = tracker.get_impact_summary("project-1");
        assert_eq!(*summary.get(&StakeholderImpactLevel::High).unwrap(), 1);
        assert_eq!(*summary.get(&StakeholderImpactLevel::Medium).unwrap(), 1);
    }
    #[test]
    fn test_public_hearing_scheduling() {
        let mut manager = PublicCommentPeriodManager::new();
        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Test Period".to_string(),
            "Description".to_string(),
            30,
        );
        let period_id = period.id.clone();
        let hearing = PublicHearing {
            id: "hearing-1".to_string(),
            title: "Public Hearing on Statute Porting".to_string(),
            datetime: "2025-02-15T10:00:00Z".to_string(),
            location: "City Hall, Room 101".to_string(),
            virtual_link: Some("https://meeting.example.com/hearing1".to_string()),
            agenda: vec![
                "Opening remarks".to_string(),
                "Presentation of ported statutes".to_string(),
                "Public questions and comments".to_string(),
            ],
            registration_required: true,
        };
        assert!(manager.schedule_hearing(&period_id, hearing).is_some());
        let period = manager.get_period(&period_id).unwrap();
        assert_eq!(period.hearings.len(), 1);
        assert_eq!(period.hearings[0].agenda.len(), 3);
    }
    #[test]
    fn test_quality_scorer_creation() {
        let scorer = QualityScorer::new();
        assert_eq!(scorer.min_quality_threshold, 0.6);
        let scorer_custom = QualityScorer::new().with_threshold(0.8);
        assert_eq!(scorer_custom.min_quality_threshold, 0.8);
    }
    #[test]
    fn test_quality_scoring_with_changes() {
        let scorer = QualityScorer::new();
        let mut statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );
        statute.id = "test-statute".to_string();
        let ported = PortedStatute {
            original_id: "original-1".to_string(),
            statute,
            changes: vec![
                PortingChange {
                    change_type: ChangeType::CulturalAdaptation,
                    description: "Adapted age parameter".to_string(),
                    original: Some("20".to_string()),
                    adapted: Some("18".to_string()),
                    reason: "Age of majority differs between jurisdictions".to_string(),
                },
                PortingChange {
                    change_type: ChangeType::Translation,
                    description: "Translated legal term".to_string(),
                    original: Some("契約".to_string()),
                    adapted: Some("contract".to_string()),
                    reason: "Translation to target language".to_string(),
                },
            ],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.85,
        };
        let quality = scorer.score_porting(&ported);
        assert!(quality.overall >= 0.0 && quality.overall <= 1.0);
        assert!(quality.semantic_preservation >= 0.0);
        assert!(quality.legal_correctness >= 0.0);
        assert!(quality.cultural_adaptation >= 0.0);
        assert!(quality.completeness >= 0.0);
        assert!(quality.consistency >= 0.0);
    }
    #[test]
    fn test_quality_scoring_empty_statute() {
        let scorer = QualityScorer::new();
        let statute = Statute::new("", "", Effect::new(EffectType::Grant, "Test"));
        let ported = PortedStatute {
            original_id: "original-1".to_string(),
            statute,
            changes: vec![],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.5,
        };
        let quality = scorer.score_porting(&ported);
        assert!(
            quality.overall < 0.9,
            "Quality score is {}",
            quality.overall
        );
        assert!(
            (quality.completeness - 0.4).abs() < 0.01,
            "Completeness score is {}",
            quality.completeness
        );
        assert!(!quality.issues.is_empty());
        assert!(
            quality
                .issues
                .iter()
                .any(|i| matches!(i.issue_type, QualityIssueType::Incompleteness))
        );
    }
    #[test]
    fn test_quality_grade_classification() {
        let scorer = QualityScorer::new();
        let excellent = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Test".to_string(),
                original: None,
                adapted: None,
                reason: "Test reason".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 1.0,
        };
        let quality = scorer.score_porting(&excellent);
        assert!(matches!(
            quality.grade,
            QualityGrade::Good | QualityGrade::Excellent
        ));
    }
    #[test]
    fn test_quality_scorer_meets_threshold() {
        let scorer = QualityScorer::new().with_threshold(0.7);
        let score = QualityScore {
            overall: 0.8,
            semantic_preservation: 0.8,
            legal_correctness: 0.8,
            cultural_adaptation: 0.8,
            completeness: 0.8,
            consistency: 0.8,
            grade: QualityGrade::Good,
            issues: vec![],
            recommendations: vec![],
        };
        assert!(scorer.meets_threshold(&score));
        let low_score = QualityScore {
            overall: 0.5,
            semantic_preservation: 0.5,
            legal_correctness: 0.5,
            cultural_adaptation: 0.5,
            completeness: 0.5,
            consistency: 0.5,
            grade: QualityGrade::Poor,
            issues: vec![],
            recommendations: vec![],
        };
        assert!(!scorer.meets_threshold(&low_score));
    }
    #[test]
    fn test_consistency_verifier_creation() {
        let verifier = ConsistencyVerifier::new();
        let _ = verifier;
    }
    #[test]
    fn test_consistency_verification_consistent() {
        let verifier = ConsistencyVerifier::new();
        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 1.0,
        };
        let result = verifier.verify(&ported);
        assert!(result.is_consistent);
        assert_eq!(result.consistency_score, 1.0);
        assert!(result.inconsistencies.is_empty());
    }
    #[test]
    fn test_consistency_verification_with_many_changes() {
        let verifier = ConsistencyVerifier::new();
        let mut changes = vec![];
        for i in 0..15 {
            changes.push(PortingChange {
                change_type: ChangeType::Translation,
                description: format!("Translation {}", i),
                original: Some(format!("old-{}", i)),
                adapted: Some(format!("new-{}", i)),
                reason: format!("Translation reason {}", i),
            });
        }
        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes,
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.8,
        };
        let result = verifier.verify(&ported);
        assert!(!result.inconsistencies.is_empty());
        assert!(result.inconsistencies.iter().any(|i| matches!(
            i.inconsistency_type,
            InconsistencyType::TerminologyInconsistency
        )));
    }
    #[test]
    fn test_consistency_verification_logical_inconsistency() {
        let verifier = ConsistencyVerifier::new();
        let mut changes = vec![];
        for i in 0..4 {
            changes.push(PortingChange {
                change_type: ChangeType::ValueAdaptation,
                description: format!("Value adaptation {}", i),
                original: Some(format!("old-{}", i)),
                adapted: Some(format!("new-{}", i)),
                reason: "Value adaptation".to_string(),
            });
        }
        changes.push(PortingChange {
            change_type: ChangeType::Removal,
            description: "Removed incompatible clause".to_string(),
            original: Some("incompatible".to_string()),
            adapted: None,
            reason: "Incompatible with target jurisdiction".to_string(),
        });
        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes,
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.7,
        };
        let result = verifier.verify(&ported);
        assert!(!result.inconsistencies.is_empty());
        assert!(result.inconsistencies.iter().any(|i| matches!(
            i.inconsistency_type,
            InconsistencyType::LogicalInconsistency
        )));
    }
    #[test]
    fn test_completeness_checker_creation() {
        let checker = CompletenessChecker::new();
        assert!(!checker.check_optional);
        let checker_with_optional = CompletenessChecker::new().with_optional_check(true);
        assert!(checker_with_optional.check_optional);
    }
    #[test]
    fn test_completeness_check_complete() {
        let checker = CompletenessChecker::new();
        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Test change".to_string(),
                original: None,
                adapted: None,
                reason: "Cultural adaptation test".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 1.0,
        };
        let result = checker.check(&ported);
        assert!(result.is_complete);
        assert_eq!(result.completeness_score, 1.0);
        assert!(result.missing_elements.is_empty());
    }
    #[test]
    fn test_completeness_check_missing_required() {
        let checker = CompletenessChecker::new();
        let statute = Statute::new("", "", Effect::new(EffectType::Grant, "Test"));
        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute,
            changes: vec![],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.5,
        };
        let result = checker.check(&ported);
        assert!(!result.is_complete);
        assert_eq!(result.completeness_score, 0.0);
        assert!(
            result
                .missing_elements
                .iter()
                .any(|e| matches!(e.importance, ElementImportance::Required))
        );
    }
    #[test]
    fn test_completeness_check_missing_recommended() {
        let checker = CompletenessChecker::new();
        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.8,
        };
        let result = checker.check(&ported);
        assert!(!result.is_complete);
        assert!(result.completeness_score > 0.0 && result.completeness_score < 1.0);
        assert!(
            result
                .missing_elements
                .iter()
                .any(|e| matches!(e.importance, ElementImportance::Recommended))
        );
    }
    #[test]
    fn test_regression_test_manager_creation() {
        let manager = RegressionTestManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total, 0);
        assert_eq!(stats.pass_rate, 0.0);
    }
    #[test]
    fn test_regression_test_add() {
        let mut manager = RegressionTestManager::new();
        let test = RegressionTest {
            test_id: "test-1".to_string(),
            name: "Test Porting".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            input_statute: "{}".to_string(),
            expected_output: "{}".to_string(),
            quality_baseline: 0.8,
            created_at: chrono::Utc::now(),
            last_run: None,
            status: RegressionTestStatus::Pending,
        };
        manager.add_test(test);
        let stats = manager.get_statistics();
        assert_eq!(stats.total, 1);
        assert_eq!(stats.pending, 1);
    }
    #[test]
    fn test_regression_test_run() {
        let mut manager = RegressionTestManager::new();
        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Test".to_string(),
                original: None,
                adapted: None,
                reason: "Test reason".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.9,
        };
        let test = RegressionTest {
            test_id: "test-1".to_string(),
            name: "Test Porting".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            input_statute: "{}".to_string(),
            expected_output: "{}".to_string(),
            quality_baseline: 0.8,
            created_at: chrono::Utc::now(),
            last_run: None,
            status: RegressionTestStatus::Pending,
        };
        manager.add_test(test);
        let result = manager.run_test("test-1", &ported);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.passed);
        assert!(result.quality_score >= 0.0);
    }
    #[test]
    fn test_regression_test_statistics() {
        let mut manager = RegressionTestManager::new();
        for i in 0..5 {
            let test = RegressionTest {
                test_id: format!("test-{}", i),
                name: format!("Test {}", i),
                source_jurisdiction: "JP".to_string(),
                target_jurisdiction: "US".to_string(),
                input_statute: "{}".to_string(),
                expected_output: "{}".to_string(),
                quality_baseline: 0.8,
                created_at: chrono::Utc::now(),
                last_run: None,
                status: if i % 2 == 0 {
                    RegressionTestStatus::Passed
                } else {
                    RegressionTestStatus::Failed
                },
            };
            manager.add_test(test);
        }
        let stats = manager.get_statistics();
        assert_eq!(stats.total, 5);
        assert_eq!(stats.passed, 3);
        assert_eq!(stats.failed, 2);
        assert_eq!(stats.pass_rate, 0.6);
    }
    #[test]
    fn test_drift_monitor_creation() {
        let monitor = DriftMonitor::new();
        assert_eq!(monitor.drift_threshold, 0.1);
        let monitor_custom = DriftMonitor::new().with_threshold(0.2);
        assert_eq!(monitor_custom.drift_threshold, 0.2);
    }
}
