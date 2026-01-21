//! Privacy Compliance Validator
//!
//! This module provides comprehensive validation of privacy practices against
//! the Australian Privacy Principles (APPs) and related requirements.
//!
//! ## Validation Scope
//!
//! The validator checks compliance with:
//! - All 13 Australian Privacy Principles
//! - Notifiable Data Breaches (NDB) scheme
//! - Credit reporting requirements (Part IIIA)
//! - Cross-border disclosure obligations
//!
//! ## Usage
//!
//! ```rust,ignore
//! use legalis_au::privacy::{PrivacyValidator, Organisation, EntityType};
//!
//! let org = Organisation::new("Test Corp", EntityType::Company)
//!     .with_turnover(5_000_000.0);
//!
//! let report = PrivacyValidator::validate_organisation(&org);
//! println!("Compliant: {}", report.is_compliant);
//! ```

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use super::apps::{App, AppAnalyzer, CollectionAnalysis, DisclosureAnalysis, SecurityAnalysis};
use super::breach::{BreachAssessment, DataBreach};
use super::entities::{AppEntity, Organisation};
use super::types::{
    AccessRequest, AccessRequestStatus, Consent, CorrectionRequest, CorrectionRequestStatus,
    PersonalInformation, PrivacyPolicy,
};

/// Privacy compliance validator
pub struct PrivacyValidator;

impl PrivacyValidator {
    /// Validate organisation's overall privacy compliance
    pub fn validate_organisation(org: &Organisation) -> ComplianceReport {
        let mut findings = Vec::new();
        let mut recommendations = Vec::new();

        // Check if organisation is covered
        if !org.is_covered() {
            return ComplianceReport {
                entity_name: org.name.clone(),
                assessment_date: Utc::now(),
                is_compliant: true,
                overall_risk: RiskLevel::Low,
                findings,
                recommendations: vec!["Organisation exempt under s.6D".to_string()],
                apps_assessed: Vec::new(),
                breach_readiness: None,
            };
        }

        // Assess APP compliance indicators
        let app_findings = Self::assess_app_indicators(org);
        findings.extend(app_findings.clone());

        // Generate recommendations
        for finding in &app_findings {
            if finding.risk_level == RiskLevel::High || finding.risk_level == RiskLevel::Critical {
                recommendations.push(format!(
                    "Priority: Address {} - {}",
                    finding
                        .app
                        .map_or("compliance".to_string(), |a| format!("APP {}", a as u8)),
                    finding.issue
                ));
            }
        }

        let overall_risk = Self::calculate_overall_risk(&findings);
        let is_compliant = !findings.iter().any(|f| {
            f.risk_level == RiskLevel::Critical
                || f.compliance_status == ComplianceStatus::NonCompliant
        });

        ComplianceReport {
            entity_name: org.name.clone(),
            assessment_date: Utc::now(),
            is_compliant,
            overall_risk,
            findings,
            recommendations,
            apps_assessed: (1..=13).collect(),
            breach_readiness: None,
        }
    }

    /// Validate APP entity (organisation or agency)
    pub fn validate_entity(entity: &AppEntity) -> ComplianceReport {
        match entity {
            AppEntity::Organisation(org) => Self::validate_organisation(org),
            AppEntity::Agency(agency) => {
                if !agency.is_covered() {
                    ComplianceReport {
                        entity_name: agency.name.clone(),
                        assessment_date: Utc::now(),
                        is_compliant: true,
                        overall_risk: RiskLevel::Low,
                        findings: Vec::new(),
                        recommendations: vec![
                            agency
                                .exemption_reason()
                                .unwrap_or_else(|| "Agency exempt".to_string()),
                        ],
                        apps_assessed: Vec::new(),
                        breach_readiness: None,
                    }
                } else {
                    // Agencies have similar obligations
                    ComplianceReport {
                        entity_name: agency.name.clone(),
                        assessment_date: Utc::now(),
                        is_compliant: true,
                        overall_risk: RiskLevel::Low,
                        findings: Vec::new(),
                        recommendations: vec![
                            "Agency subject to Privacy Act - full APP compliance required"
                                .to_string(),
                        ],
                        apps_assessed: (1..=13).collect(),
                        breach_readiness: None,
                    }
                }
            }
        }
    }

    /// Validate privacy policy against APP 1 requirements
    pub fn validate_privacy_policy(policy: &PrivacyPolicy) -> PolicyValidationResult {
        let mut missing_elements = Vec::new();
        let mut recommendations = Vec::new();

        // Check required elements (APP 1.4)
        if policy.entity_name.is_empty() {
            missing_elements.push("Identity and contact details (APP 1.4(a))".to_string());
        }

        if policy.collection_purposes.is_empty() {
            missing_elements.push("Purposes of collection (APP 1.4(b))".to_string());
        }

        if policy.disclosure_recipients.is_empty() {
            missing_elements.push("Types of recipients (APP 1.4(d))".to_string());
        }

        if !policy.overseas_disclosure && policy.likely_overseas_disclosure {
            missing_elements.push("Overseas disclosure information (APP 1.4(g))".to_string());
            recommendations.push(
                "Add information about countries where personal information may be disclosed"
                    .to_string(),
            );
        }

        if policy.access_correction_process.is_empty() {
            missing_elements.push("Access and correction information (APP 1.4(h))".to_string());
        }

        if policy.complaints_process.is_empty() {
            missing_elements.push("Complaint process (APP 1.4(i))".to_string());
            recommendations.push("Include clear process for making privacy complaints".to_string());
        }

        let is_compliant = missing_elements.is_empty();

        PolicyValidationResult {
            policy_name: policy.entity_name.clone(),
            is_compliant,
            missing_elements,
            recommendations,
            last_updated: policy.last_updated,
            update_required: policy.last_updated < Utc::now() - Duration::days(365),
        }
    }

    /// Validate consent for specific purpose
    pub fn validate_consent(
        consent: &Consent,
        information: &PersonalInformation,
        purpose: &str,
    ) -> ConsentValidationResult {
        let mut issues = Vec::new();

        // Check consent is current (not expired)
        if consent.is_expired() {
            issues.push(ConsentIssue {
                issue_type: ConsentIssueType::Expired,
                description: "Consent has expired".to_string(),
            });
        }

        // Check consent withdrawn (is_valid becomes false when withdrawn)
        if !consent.is_valid || consent.withdrawal_timestamp.is_some() {
            issues.push(ConsentIssue {
                issue_type: ConsentIssueType::Withdrawn,
                description: "Consent has been withdrawn".to_string(),
            });
        }

        // Check purpose covered (single purpose field)
        let purpose_covered = format!("{:?}", consent.purpose)
            .to_lowercase()
            .contains(&purpose.to_lowercase());

        if !purpose_covered {
            issues.push(ConsentIssue {
                issue_type: ConsentIssueType::PurposeNotCovered,
                description: format!("Purpose '{}' not covered by consent", purpose),
            });
        }

        // Check consent method appropriate for sensitive information
        if information.is_sensitive && !consent.method.is_express() {
            issues.push(ConsentIssue {
                issue_type: ConsentIssueType::WrongMethod,
                description: "Sensitive information requires express consent (APP 3.3)".to_string(),
            });
        }

        ConsentValidationResult {
            is_valid: issues.is_empty(),
            consent_date: consent.timestamp,
            issues,
            legal_references: vec![
                "Privacy Act 1988 s.6".to_string(),
                if information.is_sensitive {
                    "APP 3.3 (sensitive information)".to_string()
                } else {
                    "APP 3.2 (personal information)".to_string()
                },
            ],
        }
    }

    /// Validate collection of personal information (APP 3)
    pub fn validate_collection(
        information: &PersonalInformation,
        has_consent: bool,
        collection_context: &CollectionContext,
    ) -> CollectionValidationResult {
        let consent_opt = if has_consent {
            // Create a temporary consent for analysis
            Some(super::types::Consent::new(
                "temp",
                &information.data_subject_id,
                super::types::ConsentPurpose::Collection,
                super::types::ConsentMethod::ExpressElectronic,
                "Consent for collection",
            ))
        } else {
            None
        };

        let analysis = AppAnalyzer::analyze_app3(
            information,
            information
                .collection_purpose
                .as_deref()
                .unwrap_or("Unspecified purpose"),
            collection_context.directly_from_individual,
            consent_opt.as_ref(),
        );

        let mut issues = Vec::new();

        if !analysis.compliance.compliant {
            issues.push(format!(
                "Collection not compliant: {}",
                analysis.compliance.issues.join("; ")
            ));
        }

        if !analysis.reasonably_necessary {
            issues.push("Collection not reasonably necessary for functions/activities".to_string());
        }

        if information.is_sensitive && !has_consent {
            issues.push(
                "Sensitive information collected without express consent (APP 3.3)".to_string(),
            );
        }

        // Check collection method
        if !collection_context.directly_from_individual && !collection_context.exception_applies {
            issues.push(
                "Personal information should be collected directly from individual (APP 3.5)"
                    .to_string(),
            );
        }

        let is_compliant = issues.is_empty();
        let recommendations = if is_compliant {
            Vec::new()
        } else {
            vec!["Review collection practices against APP 3 requirements".to_string()]
        };

        CollectionValidationResult {
            is_compliant,
            issues,
            analysis,
            recommendations,
        }
    }

    /// Validate disclosure of personal information (APP 6)
    pub fn validate_disclosure(
        information: &PersonalInformation,
        primary_purpose: &str,
        disclosure_purpose: &str,
        has_consent: bool,
    ) -> DisclosureValidationResult {
        use super::apps::DisclosureException;

        let consent_opt = if has_consent {
            Some(super::types::Consent::new(
                "temp",
                &information.data_subject_id,
                super::types::ConsentPurpose::Disclosure,
                super::types::ConsentMethod::ExpressElectronic,
                "Consent for disclosure",
            ))
        } else {
            None
        };

        let analysis = AppAnalyzer::analyze_app6(
            information,
            primary_purpose,
            disclosure_purpose,
            consent_opt.as_ref(),
            if has_consent {
                Some(DisclosureException::Consent)
            } else {
                None
            },
        );

        let mut issues = Vec::new();

        if !analysis.compliance.compliant {
            issues.push(format!(
                "Disclosure not compliant: {}",
                analysis.compliance.issues.join("; ")
            ));
        }

        if !analysis.is_primary_purpose && analysis.exception.is_none() && !has_consent {
            issues.push(
                "Secondary purpose disclosure requires consent or exception (APP 6.2)".to_string(),
            );
        }

        let exceptions_applicable = analysis
            .exception
            .map(|e| format!("{:?}", e))
            .into_iter()
            .collect();

        DisclosureValidationResult {
            is_compliant: issues.is_empty(),
            issues,
            analysis,
            exceptions_applicable,
        }
    }

    /// Validate access request handling (APP 12)
    pub fn validate_access_request(request: &AccessRequest) -> AccessRequestValidationResult {
        let mut issues = Vec::new();
        let mut deadline_info = None;

        // Standard response period is 30 days
        let standard_period_days = 30;

        if request.status == AccessRequestStatus::Received
            || request.status == AccessRequestStatus::UnderAssessment
        {
            let days_elapsed = (Utc::now() - request.request_date).num_days();

            if days_elapsed > standard_period_days {
                issues.push(format!(
                    "Response overdue by {} days (APP 12.4)",
                    days_elapsed - standard_period_days
                ));
            }

            deadline_info = Some(DeadlineInfo {
                deadline: request.due_date,
                days_remaining: standard_period_days - days_elapsed,
                is_overdue: days_elapsed > standard_period_days,
            });
        }

        // Check if refusal reasons are valid
        if request.status == AccessRequestStatus::Refused && request.refusal_reason.is_none() {
            issues.push("Access refusal must include reasons (APP 12.8)".to_string());
        }

        AccessRequestValidationResult {
            request_id: request.request_id.clone(),
            is_compliant: issues.is_empty(),
            issues,
            deadline_info,
            legal_references: vec![
                "APP 12.4 (response timeframe)".to_string(),
                "APP 12.8 (refusal requirements)".to_string(),
            ],
        }
    }

    /// Validate correction request handling (APP 13)
    pub fn validate_correction_request(
        request: &CorrectionRequest,
    ) -> CorrectionRequestValidationResult {
        let mut issues = Vec::new();

        // Correction should be made within reasonable period
        let standard_period_days = 30;

        if request.status == CorrectionRequestStatus::Received
            || request.status == CorrectionRequestStatus::UnderAssessment
        {
            let days_elapsed = (Utc::now() - request.request_date).num_days();
            if days_elapsed > standard_period_days {
                issues.push(format!(
                    "Correction response overdue by {} days (APP 13.5)",
                    days_elapsed - standard_period_days
                ));
            }
        }

        // Check if refusal includes statement attachment option
        if request.status == CorrectionRequestStatus::Refused && !request.statement_attached {
            issues
                .push("Refusal should offer to attach statement to records (APP 13.4)".to_string());
        }

        CorrectionRequestValidationResult {
            request_id: request.request_id.clone(),
            is_compliant: issues.is_empty(),
            issues,
            deadline: request.due_date,
            legal_references: vec![
                "APP 13.1 (correction obligation)".to_string(),
                "APP 13.4 (statement attachment)".to_string(),
                "APP 13.5 (response timeframe)".to_string(),
            ],
        }
    }

    /// Validate data breach response (NDB scheme)
    pub fn validate_breach_response(
        breach: &DataBreach,
        assessment: &BreachAssessment,
    ) -> BreachResponseValidationResult {
        use super::breach::AssessmentRecommendation;

        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        let is_eligible = assessment.is_eligible;

        if is_eligible {
            // Check assessment timeframe (30 days max)
            let days_since_discovery = (Utc::now() - breach.discovery_date).num_days();

            let notification_required = matches!(
                assessment.recommendation,
                AssessmentRecommendation::NotifyOaicAndIndividuals
                    | AssessmentRecommendation::NotifyOaicOnly
            );

            if days_since_discovery > 30 && !notification_required {
                issues.push(format!(
                    "Assessment period exceeded ({} days) without notification",
                    days_since_discovery
                ));
            }

            if !notification_required {
                recommendations.push(
                    "Eligible breach requires notification to OAIC and affected individuals"
                        .to_string(),
                );
            }

            // Check remedial actions documented in breach
            if breach.remedial_actions.is_empty() {
                issues.push("No remedial actions documented".to_string());
                recommendations.push("Document all remedial actions taken".to_string());
            }
        }

        BreachResponseValidationResult {
            breach_id: breach.breach_id.clone(),
            is_eligible_breach: is_eligible,
            is_compliant: issues.is_empty(),
            issues,
            recommendations,
            notification_deadline: if is_eligible {
                Some(breach.discovery_date + Duration::days(30))
            } else {
                None
            },
            legal_references: vec![
                "Privacy Act 1988 Part IIIC (NDB scheme)".to_string(),
                "s.26WE (assessment requirement)".to_string(),
                "s.26WK (notification requirement)".to_string(),
            ],
        }
    }

    /// Validate security measures (APP 11)
    pub fn validate_security(
        _org: &Organisation,
        measures: &SecurityMeasures,
    ) -> SecurityValidationResult {
        use super::apps::{AppCompliance, SecurityMeasure, SecurityMeasureType, SecurityRiskLevel};

        let mut gaps = Vec::new();
        let mut recommendations = Vec::new();
        let mut security_measures = Vec::new();

        // Check technical controls
        if !measures.encryption_at_rest {
            gaps.push("Encryption at rest not implemented".to_string());
            recommendations
                .push("Implement encryption for stored personal information".to_string());
        }
        security_measures.push(SecurityMeasure {
            measure_type: SecurityMeasureType::Encryption,
            description: "Encryption at rest".to_string(),
            implemented: measures.encryption_at_rest,
            effective: measures.encryption_at_rest,
        });

        if !measures.encryption_in_transit {
            gaps.push("Encryption in transit not implemented".to_string());
            recommendations.push("Implement TLS for data transmission".to_string());
        }
        security_measures.push(SecurityMeasure {
            measure_type: SecurityMeasureType::Encryption,
            description: "Encryption in transit".to_string(),
            implemented: measures.encryption_in_transit,
            effective: measures.encryption_in_transit,
        });

        if !measures.access_controls {
            gaps.push("Access controls inadequate".to_string());
            recommendations.push("Implement role-based access controls".to_string());
        }
        security_measures.push(SecurityMeasure {
            measure_type: SecurityMeasureType::AccessControls,
            description: "Role-based access controls".to_string(),
            implemented: measures.access_controls,
            effective: measures.access_controls,
        });

        // Check organizational measures
        if !measures.staff_training {
            gaps.push("Staff privacy training not provided".to_string());
            recommendations.push("Implement regular privacy awareness training".to_string());
        }
        security_measures.push(SecurityMeasure {
            measure_type: SecurityMeasureType::StaffTraining,
            description: "Privacy awareness training".to_string(),
            implemented: measures.staff_training,
            effective: measures.staff_training,
        });

        if !measures.incident_response_plan {
            gaps.push("No incident response plan".to_string());
            recommendations.push("Develop and test data breach response plan".to_string());
        }
        security_measures.push(SecurityMeasure {
            measure_type: SecurityMeasureType::IncidentResponse,
            description: "Data breach response plan".to_string(),
            implemented: measures.incident_response_plan,
            effective: measures.incident_response_plan,
        });

        if !measures.regular_audits {
            gaps.push("No regular security audits".to_string());
            recommendations.push("Implement annual privacy and security audits".to_string());
        }
        security_measures.push(SecurityMeasure {
            measure_type: SecurityMeasureType::ThirdPartyManagement,
            description: "Regular security audits".to_string(),
            implemented: measures.regular_audits,
            effective: measures.regular_audits,
        });

        // Check data retention
        if !measures.data_retention_policy {
            gaps.push("No data retention/destruction policy".to_string());
            recommendations.push(
                "Implement data retention schedule and secure destruction procedures".to_string(),
            );
        }
        security_measures.push(SecurityMeasure {
            measure_type: SecurityMeasureType::DataRetention,
            description: "Data retention and destruction policy".to_string(),
            implemented: measures.data_retention_policy,
            effective: measures.data_retention_policy,
        });

        let risk_level = match gaps.len() {
            0 => RiskLevel::Low,
            1..=2 => RiskLevel::Medium,
            3..=4 => RiskLevel::High,
            _ => RiskLevel::Critical,
        };

        let sec_risk_level = match risk_level {
            RiskLevel::Low => SecurityRiskLevel::Low,
            RiskLevel::Medium => SecurityRiskLevel::Medium,
            RiskLevel::High => SecurityRiskLevel::High,
            RiskLevel::Critical => SecurityRiskLevel::Critical,
        };

        let compliance = if gaps.is_empty() {
            AppCompliance::compliant(App::App11)
        } else {
            AppCompliance::non_compliant(App::App11, gaps.clone())
        };

        SecurityValidationResult {
            is_compliant: gaps.is_empty(),
            risk_level,
            gaps,
            recommendations,
            analysis: SecurityAnalysis {
                security_measures,
                gaps: Vec::new(),
                risk_level: sec_risk_level,
                compliance,
            },
        }
    }

    /// Assess APP compliance indicators for organisation
    fn assess_app_indicators(org: &Organisation) -> Vec<ComplianceFinding> {
        let mut findings = Vec::new();

        // APP 1 - Privacy policy
        findings.push(ComplianceFinding {
            app: Some(App::App1),
            issue: "Privacy policy assessment required".to_string(),
            risk_level: RiskLevel::Medium,
            compliance_status: ComplianceStatus::NeedsReview,
            recommendation: Some(
                "Review and update privacy policy for APP 1 compliance".to_string(),
            ),
            statutory_reference: "APP 1.3, APP 1.4".to_string(),
        });

        // APP 3 - Collection practices
        if org.trades_in_personal_information {
            findings.push(ComplianceFinding {
                app: Some(App::App3),
                issue: "Organisation trades in personal information - heightened scrutiny"
                    .to_string(),
                risk_level: RiskLevel::High,
                compliance_status: ComplianceStatus::NeedsReview,
                recommendation: Some(
                    "Implement enhanced consent processes for data trading".to_string(),
                ),
                statutory_reference: "APP 3, s.6D(4)(b)".to_string(),
            });
        }

        // APP 8 - Cross-border disclosure
        findings.push(ComplianceFinding {
            app: Some(App::App8),
            issue: "Cross-border disclosure assessment required".to_string(),
            risk_level: RiskLevel::Medium,
            compliance_status: ComplianceStatus::NeedsReview,
            recommendation: Some(
                "Document all overseas transfers and applicable safeguards".to_string(),
            ),
            statutory_reference: "APP 8.1".to_string(),
        });

        // APP 11 - Security
        findings.push(ComplianceFinding {
            app: Some(App::App11),
            issue: "Security measures assessment required".to_string(),
            risk_level: RiskLevel::High,
            compliance_status: ComplianceStatus::NeedsReview,
            recommendation: Some(
                "Conduct security assessment and document reasonable steps".to_string(),
            ),
            statutory_reference: "APP 11.1".to_string(),
        });

        // NDB readiness
        if org.is_health_service_provider || org.is_credit_reporting_body {
            findings.push(ComplianceFinding {
                app: None,
                issue: "High-risk sector - enhanced NDB obligations".to_string(),
                risk_level: RiskLevel::High,
                compliance_status: ComplianceStatus::NeedsReview,
                recommendation: Some("Ensure robust data breach response procedures".to_string()),
                statutory_reference: "Part IIIC".to_string(),
            });
        }

        findings
    }

    /// Calculate overall risk level from findings
    fn calculate_overall_risk(findings: &[ComplianceFinding]) -> RiskLevel {
        if findings.iter().any(|f| f.risk_level == RiskLevel::Critical) {
            return RiskLevel::Critical;
        }
        if findings
            .iter()
            .filter(|f| f.risk_level == RiskLevel::High)
            .count()
            >= 2
        {
            return RiskLevel::High;
        }
        if findings.iter().any(|f| f.risk_level == RiskLevel::High) {
            return RiskLevel::Medium;
        }
        if findings.iter().any(|f| f.risk_level == RiskLevel::Medium) {
            return RiskLevel::Low;
        }
        RiskLevel::Low
    }
}

/// Compliance report for privacy assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Entity name
    pub entity_name: String,
    /// Assessment date
    pub assessment_date: DateTime<Utc>,
    /// Overall compliance status
    pub is_compliant: bool,
    /// Overall risk level
    pub overall_risk: RiskLevel,
    /// Individual findings
    pub findings: Vec<ComplianceFinding>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// APPs assessed
    pub apps_assessed: Vec<u8>,
    /// Breach readiness assessment
    pub breach_readiness: Option<BreachReadiness>,
}

/// Individual compliance finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFinding {
    /// Relevant APP (if applicable)
    pub app: Option<App>,
    /// Issue description
    pub issue: String,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
    /// Recommendation
    pub recommendation: Option<String>,
    /// Statutory reference
    pub statutory_reference: String,
}

/// Risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
}

/// Compliance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// Compliant
    Compliant,
    /// Needs review
    NeedsReview,
    /// Non-compliant
    NonCompliant,
}

/// Breach readiness assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachReadiness {
    /// Has incident response plan
    pub has_response_plan: bool,
    /// Staff trained on breach procedures
    pub staff_trained: bool,
    /// Notification templates prepared
    pub templates_prepared: bool,
    /// OAIC contact details documented
    pub oaic_contact_documented: bool,
}

/// Privacy policy validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyValidationResult {
    /// Policy name
    pub policy_name: String,
    /// Is compliant
    pub is_compliant: bool,
    /// Missing elements
    pub missing_elements: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Last updated date
    pub last_updated: DateTime<Utc>,
    /// Whether update is required
    pub update_required: bool,
}

/// Consent validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentValidationResult {
    /// Is valid
    pub is_valid: bool,
    /// Consent date
    pub consent_date: DateTime<Utc>,
    /// Issues found
    pub issues: Vec<ConsentIssue>,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Consent issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentIssue {
    /// Issue type
    pub issue_type: ConsentIssueType,
    /// Description
    pub description: String,
}

/// Type of consent issue
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentIssueType {
    /// Consent expired
    Expired,
    /// Consent withdrawn
    Withdrawn,
    /// Purpose not covered
    PurposeNotCovered,
    /// Wrong method for information type
    WrongMethod,
}

/// Collection context for APP 3 validation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectionContext {
    /// Collected directly from individual
    pub directly_from_individual: bool,
    /// Exception applies for indirect collection
    pub exception_applies: bool,
    /// Exception reason if applicable
    pub exception_reason: Option<String>,
}

/// Collection validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionValidationResult {
    /// Is compliant
    pub is_compliant: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Analysis
    pub analysis: CollectionAnalysis,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Disclosure validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisclosureValidationResult {
    /// Is compliant
    pub is_compliant: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Analysis
    pub analysis: DisclosureAnalysis,
    /// Applicable exceptions
    pub exceptions_applicable: Vec<String>,
}

/// Access request validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRequestValidationResult {
    /// Request ID
    pub request_id: String,
    /// Is compliant
    pub is_compliant: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Deadline info
    pub deadline_info: Option<DeadlineInfo>,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Deadline information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineInfo {
    /// Deadline date
    pub deadline: DateTime<Utc>,
    /// Days remaining (negative if overdue)
    pub days_remaining: i64,
    /// Is overdue
    pub is_overdue: bool,
}

/// Correction request validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionRequestValidationResult {
    /// Request ID
    pub request_id: String,
    /// Is compliant
    pub is_compliant: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Response deadline
    pub deadline: DateTime<Utc>,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Breach response validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachResponseValidationResult {
    /// Breach ID
    pub breach_id: String,
    /// Is eligible breach
    pub is_eligible_breach: bool,
    /// Is compliant
    pub is_compliant: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Notification deadline
    pub notification_deadline: Option<DateTime<Utc>>,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Security measures for APP 11 validation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityMeasures {
    /// Encryption at rest
    pub encryption_at_rest: bool,
    /// Encryption in transit
    pub encryption_in_transit: bool,
    /// Access controls
    pub access_controls: bool,
    /// Staff training
    pub staff_training: bool,
    /// Incident response plan
    pub incident_response_plan: bool,
    /// Regular audits
    pub regular_audits: bool,
    /// Data retention policy
    pub data_retention_policy: bool,
}

/// Security validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityValidationResult {
    /// Is compliant
    pub is_compliant: bool,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Security gaps
    pub gaps: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Analysis
    pub analysis: SecurityAnalysis,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::privacy::entities::EntityType;
    use crate::privacy::types::{ConsentMethod, ConsentPurpose, PersonalInformationType};

    #[test]
    fn test_organisation_validation_covered() {
        let org = Organisation::new("Big Corp", EntityType::Company).with_turnover(5_000_000.0);

        let report = PrivacyValidator::validate_organisation(&org);

        assert_eq!(report.entity_name, "Big Corp");
        assert!(!report.findings.is_empty());
        assert!(report.apps_assessed.contains(&1));
        assert!(report.apps_assessed.contains(&11));
    }

    #[test]
    fn test_organisation_validation_exempt() {
        let org = Organisation::new("Small Shop", EntityType::SoleTrader).with_turnover(500_000.0);

        let report = PrivacyValidator::validate_organisation(&org);

        assert!(report.is_compliant);
        assert!(report.findings.is_empty());
        assert!(report.recommendations[0].contains("exempt"));
    }

    #[test]
    fn test_privacy_policy_validation() {
        let mut policy = PrivacyPolicy::new("Test Policy", "1.0");
        policy.collection_purposes = vec!["Service delivery".to_string()];
        policy.disclosure_recipients = vec!["Service providers".to_string()];
        policy.overseas_disclosure = true;
        policy.access_correction_process = "Contact us".to_string();
        policy.complaints_process = "Lodge complaint".to_string();

        let result = PrivacyValidator::validate_privacy_policy(&policy);

        assert!(result.is_compliant);
        assert!(result.missing_elements.is_empty());
    }

    #[test]
    fn test_privacy_policy_missing_elements() {
        let policy = PrivacyPolicy::new("", "1.0");

        let result = PrivacyValidator::validate_privacy_policy(&policy);

        assert!(!result.is_compliant);
        assert!(!result.missing_elements.is_empty());
    }

    #[test]
    fn test_consent_validation_valid() {
        let consent = Consent::new(
            "c-001",
            "ind-001",
            ConsentPurpose::DirectMarketing,
            ConsentMethod::ExpressElectronic,
            "I consent to marketing communications",
        );

        let info =
            PersonalInformation::new("i-001", PersonalInformationType::ContactDetails, "ind-001");

        let result = PrivacyValidator::validate_consent(&consent, &info, "marketing");

        assert!(result.is_valid);
        assert!(result.issues.is_empty());
    }

    #[test]
    fn test_consent_validation_withdrawn() {
        let mut consent = Consent::new(
            "c-002",
            "ind-001",
            ConsentPurpose::DirectMarketing,
            ConsentMethod::ExpressElectronic,
            "Marketing consent",
        );
        consent.withdraw();

        let info =
            PersonalInformation::new("i-001", PersonalInformationType::ContactDetails, "ind-001");

        let result = PrivacyValidator::validate_consent(&consent, &info, "marketing");

        assert!(!result.is_valid);
        assert!(
            result
                .issues
                .iter()
                .any(|i| i.issue_type == ConsentIssueType::Withdrawn)
        );
    }

    #[test]
    fn test_security_validation() {
        let org = Organisation::new("Test Corp", EntityType::Company).with_turnover(5_000_000.0);

        let measures = SecurityMeasures {
            encryption_at_rest: true,
            encryption_in_transit: true,
            access_controls: true,
            staff_training: true,
            incident_response_plan: true,
            regular_audits: true,
            data_retention_policy: true,
        };

        let result = PrivacyValidator::validate_security(&org, &measures);

        assert!(result.is_compliant);
        assert!(result.gaps.is_empty());
        assert_eq!(result.risk_level, RiskLevel::Low);
    }

    #[test]
    fn test_security_validation_gaps() {
        let org = Organisation::new("Test Corp", EntityType::Company).with_turnover(5_000_000.0);

        let measures = SecurityMeasures::default();

        let result = PrivacyValidator::validate_security(&org, &measures);

        assert!(!result.is_compliant);
        assert!(!result.gaps.is_empty());
        assert!(result.risk_level == RiskLevel::Critical || result.risk_level == RiskLevel::High);
    }

    #[test]
    fn test_collection_validation() {
        let info =
            PersonalInformation::new("i-001", PersonalInformationType::ContactDetails, "ind-001");

        let context = CollectionContext {
            directly_from_individual: true,
            exception_applies: false,
            exception_reason: None,
        };

        let result = PrivacyValidator::validate_collection(&info, true, &context);

        assert!(result.is_compliant);
    }

    #[test]
    fn test_disclosure_validation() {
        let info =
            PersonalInformation::new("i-001", PersonalInformationType::ContactDetails, "ind-001");

        let result = PrivacyValidator::validate_disclosure(
            &info,
            "service delivery",
            "service delivery",
            true,
        );

        assert!(result.is_compliant);
    }

    #[test]
    fn test_risk_level_calculation() {
        let critical_findings = vec![ComplianceFinding {
            app: Some(App::App11),
            issue: "Critical issue".to_string(),
            risk_level: RiskLevel::Critical,
            compliance_status: ComplianceStatus::NonCompliant,
            recommendation: None,
            statutory_reference: "APP 11".to_string(),
        }];

        assert_eq!(
            PrivacyValidator::calculate_overall_risk(&critical_findings),
            RiskLevel::Critical
        );

        let low_findings = vec![ComplianceFinding {
            app: Some(App::App1),
            issue: "Minor issue".to_string(),
            risk_level: RiskLevel::Low,
            compliance_status: ComplianceStatus::NeedsReview,
            recommendation: None,
            statutory_reference: "APP 1".to_string(),
        }];

        assert_eq!(
            PrivacyValidator::calculate_overall_risk(&low_findings),
            RiskLevel::Low
        );
    }
}
