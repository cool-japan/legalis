//! Validation logic for Digital Services Act (DSA) and Digital Markets Act (DMA)

use super::error::DigitalServicesError;
use super::types::*;
use chrono::{Duration, Utc};

/// DSA validation result
#[derive(Debug, Clone, PartialEq)]
pub struct DsaValidationResult {
    /// Whether compliant with DSA
    pub compliant: bool,
    /// List of violations
    pub violations: Vec<String>,
    /// List of warnings
    pub warnings: Vec<String>,
    /// Applicable platform obligations
    pub applicable_obligations: Vec<String>,
}

impl DsaValidationResult {
    /// Create new validation result
    pub fn new() -> Self {
        Self {
            compliant: true,
            violations: Vec::new(),
            warnings: Vec::new(),
            applicable_obligations: Vec::new(),
        }
    }

    /// Add violation
    pub fn add_violation(&mut self, violation: impl Into<String>) {
        self.compliant = false;
        self.violations.push(violation.into());
    }

    /// Add warning
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    /// Add applicable obligation
    pub fn add_obligation(&mut self, obligation: impl Into<String>) {
        self.applicable_obligations.push(obligation.into());
    }

    /// Check if compliant
    pub fn is_compliant(&self) -> bool {
        self.compliant
    }
}

impl Default for DsaValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate platform type classification
pub fn validate_platform_classification(
    platform_type: &PlatformType,
) -> Result<DsaValidationResult, DigitalServicesError> {
    let mut result = DsaValidationResult::new();

    match platform_type {
        PlatformType::IntermediaryService => {
            result.add_obligation("Notice and action mechanism (Article 16)");
            result.add_obligation("Transparency reporting (Article 15)");
        }
        PlatformType::HostingService {
            monthly_active_recipients,
        } => {
            result.add_obligation("Notice and action mechanism (Article 16)");
            result.add_obligation("Transparency reporting (Article 15)");
            result.add_obligation("Statement of reasons (Article 17)");

            if *monthly_active_recipients >= 45_000_000 {
                result.add_warning("Platform may meet VLOP threshold - consider VLOP designation");
            }
        }
        PlatformType::OnlinePlatform {
            monthly_active_recipients,
            ..
        } => {
            result.add_obligation("Notice and action mechanism (Article 16)");
            result.add_obligation("Transparency reporting (Article 15, 24)");
            result.add_obligation("Statement of reasons (Article 17)");
            result.add_obligation("Internal complaint system (Article 20)");
            result.add_obligation("Out-of-court dispute settlement (Article 21)");
            result.add_obligation("Trusted flagger framework (Article 22)");
            result.add_obligation("Traceability of traders (Article 30)");

            if *monthly_active_recipients >= 45_000_000 {
                result.add_violation("Platform meets VLOP threshold - must be designated as VLOP");
            }
        }
        PlatformType::VeryLargeOnlinePlatform {
            monthly_active_recipients,
            systemic_risk_designation,
            ..
        } => {
            if *monthly_active_recipients < 45_000_000 {
                return Err(DigitalServicesError::BelowVlopThreshold {
                    recipients: monthly_active_recipients / 1_000_000,
                });
            }

            // All online platform obligations
            result.add_obligation("Notice and action mechanism (Article 16)");
            result.add_obligation("Transparency reporting (Article 15, 24, 42)");
            result.add_obligation("Statement of reasons (Article 17)");
            result.add_obligation("Internal complaint system (Article 20)");
            result.add_obligation("Out-of-court dispute settlement (Article 21)");
            result.add_obligation("Trusted flagger framework (Article 22)");
            result.add_obligation("Traceability of traders (Article 30)");

            // Additional VLOP obligations
            result.add_obligation("Systemic risk assessment (Article 34)");
            result.add_obligation("Risk mitigation measures (Article 35)");
            result.add_obligation("Independent audit (Article 37)");
            result.add_obligation("Algorithmic transparency (Article 27)");
            result.add_obligation("Recommender system transparency (Article 27)");
            result.add_obligation("Data access for researchers (Article 40)");
            result.add_obligation("Crisis response mechanism (Article 36)");

            if !systemic_risk_designation {
                result.add_warning("Platform should be designated for systemic risk assessment");
            }
        }
        PlatformType::VeryLargeOnlineSearchEngine {
            monthly_active_recipients,
            systemic_risk_designation,
            ..
        } => {
            if *monthly_active_recipients < 45_000_000 {
                return Err(DigitalServicesError::BelowVloseThreshold {
                    recipients: monthly_active_recipients / 1_000_000,
                });
            }

            // Basic obligations
            result.add_obligation("Transparency reporting (Article 15, 42)");

            // VLOSE-specific obligations
            result.add_obligation("Systemic risk assessment (Article 34)");
            result.add_obligation("Risk mitigation measures (Article 35)");
            result.add_obligation("Independent audit (Article 37)");
            result.add_obligation("Algorithmic transparency (Article 27)");
            result.add_obligation("Recommender system transparency (Article 27)");
            result.add_obligation("Data access for researchers (Article 40)");
            result.add_obligation("Crisis response mechanism (Article 36)");

            if !systemic_risk_designation {
                result.add_warning("Platform should be designated for systemic risk assessment");
            }
        }
    }

    Ok(result)
}

/// Validate illegal content notice
pub fn validate_illegal_content_notice(
    notice: &IllegalContentNotice,
) -> Result<DsaValidationResult, DigitalServicesError> {
    let mut result = DsaValidationResult::new();

    if notice.notice_id.is_empty() {
        return Err(DigitalServicesError::missing_field("notice_id"));
    }

    if notice.content_location.is_empty() {
        return Err(DigitalServicesError::missing_field("content_location"));
    }

    if notice.explanation.is_empty() {
        result
            .add_violation("Notice must include sufficient explanation of why content is illegal");
    }

    if notice.notifier_contact.is_empty() {
        result.add_violation("Notice must include contact information of notifier");
    }

    if notice.is_trusted_flagger {
        result.add_obligation(
            "Platform must process trusted flagger notices with priority (Article 22)",
        );
    }

    Ok(result)
}

/// Validate notice response
pub fn validate_notice_response(
    notice: &IllegalContentNotice,
    response: &NoticeResponse,
) -> Result<DsaValidationResult, DigitalServicesError> {
    let mut result = DsaValidationResult::new();

    if notice.notice_id != response.notice_id {
        return Err(DigitalServicesError::invalid_value(
            "notice_id",
            "Response notice_id does not match original notice",
        ));
    }

    // Check response timeliness
    let response_time = response.response_date - notice.submission_date;
    if response_time > Duration::days(7) {
        result.add_warning(
            "Platform should respond to notices without undue delay (typically within days)",
        );
    }

    if notice.is_trusted_flagger && response_time > Duration::days(2) {
        result.add_violation("Trusted flagger notices should be processed with priority");
    }

    if response.reasoning.is_empty() {
        result.add_violation("Response must include clear reasoning for decision");
    }

    if response.redress_information.is_empty() {
        result.add_violation("Response must include information about redress mechanisms");
    }

    match &response.decision {
        NoticeDecision::ContentRemoved { removal_date } => {
            if *removal_date > Utc::now() {
                result.add_violation("Removal date cannot be in the future");
            }
        }
        NoticeDecision::ContentRestricted { restriction_type } => {
            if restriction_type.is_empty() {
                result.add_violation("Must specify type of restriction applied");
            }
        }
        NoticeDecision::NoticeRejected { reason } => {
            if reason.is_empty() {
                result.add_violation("Rejection must include clear reason");
            }
        }
        NoticeDecision::UnderReview { .. } => {
            if response_time > Duration::days(14) {
                result.add_warning(
                    "Content under review for extended period - ensure timely resolution",
                );
            }
        }
    }

    Ok(result)
}

/// Validate statement of reasons (Article 17)
pub fn validate_statement_of_reasons(
    statement: &StatementOfReasons,
) -> Result<DsaValidationResult, DigitalServicesError> {
    let mut result = DsaValidationResult::new();

    if statement.statement_id.is_empty() {
        return Err(DigitalServicesError::missing_field("statement_id"));
    }

    if statement.facts_and_circumstances.is_empty() {
        result.add_violation(
            "Statement must include facts and circumstances relied upon for decision",
        );
    }

    if let Some(auto_info) = &statement.automated_decision_info
        && auto_info.solely_automated
        && auto_info.human_review.is_none()
    {
        result.add_warning(
            "Solely automated decisions should include information about human review availability",
        );
    }

    if statement.redress_mechanisms.is_empty() {
        result.add_violation("Statement must include information about redress mechanisms");
    }

    // Validate each redress mechanism
    for mechanism in &statement.redress_mechanisms {
        match mechanism {
            RedressMechanism::InternalComplaintSystem { complaint_contact } => {
                if complaint_contact.is_empty() {
                    result.add_violation(
                        "Internal complaint system must provide contact information",
                    );
                }
            }
            RedressMechanism::OutOfCourtSettlement { settlement_body } => {
                if settlement_body.is_empty() {
                    result.add_violation("Out-of-court settlement must identify certified body");
                }
            }
            RedressMechanism::JudicialRedress { court_information } => {
                if court_information.is_empty() {
                    result.add_violation("Judicial redress must provide court information");
                }
            }
        }
    }

    Ok(result)
}

/// Validate transparency report (Article 15, 24, 42)
pub fn validate_transparency_report(
    report: &TransparencyReport,
    is_vlop_vlose: bool,
) -> Result<DsaValidationResult, DigitalServicesError> {
    let mut result = DsaValidationResult::new();

    if report.period_end <= report.period_start {
        return Err(DigitalServicesError::invalid_value(
            "period_end",
            "Period end must be after period start",
        ));
    }

    // Validate reporting period is reasonable (typically 6-12 months)
    let period_duration = report.period_end - report.period_start;
    if period_duration > Duration::days(400) {
        result.add_warning("Reporting period exceeds 12 months");
    }
    if period_duration < Duration::days(150) {
        result.add_warning("Reporting period less than 6 months - consider longer periods");
    }

    // Validate moderation statistics
    if report.moderation_statistics.automated_decisions_percentage > 100.0 {
        result.add_violation("Automated decisions percentage cannot exceed 100%");
    }

    // Validate notice statistics
    if report.notice_statistics.notices_acted_upon > report.notice_statistics.total_notices {
        result.add_violation("Notices acted upon cannot exceed total notices");
    }

    if report.notice_statistics.trusted_flagger_notices > report.notice_statistics.total_notices {
        result.add_violation("Trusted flagger notices cannot exceed total notices");
    }

    // VLOP/VLOSE additional requirements
    if is_vlop_vlose {
        if report.algorithmic_transparency.is_none() {
            result.add_violation(
                "VLOP/VLOSE must include algorithmic transparency in transparency reports (Article 27, 42)",
            );
        } else if let Some(algo_trans) = &report.algorithmic_transparency {
            if algo_trans.recommendation_parameters.is_empty() {
                result.add_violation(
                    "Algorithmic transparency must include main recommendation parameters",
                );
            }
            if algo_trans.user_control_options.is_empty() {
                result.add_violation("Algorithmic transparency must describe user control options");
            }
        }
    } else if report.algorithmic_transparency.is_some() {
        result.add_warning("Algorithmic transparency reporting only required for VLOP/VLOSE");
    }

    Ok(result)
}

/// Validate gatekeeper designation (DMA Article 3)
pub fn validate_gatekeeper_designation(
    designation: &GatekeeperDesignation,
) -> Result<DsaValidationResult, DigitalServicesError> {
    let mut result = DsaValidationResult::new();

    if designation.company_name.is_empty() {
        return Err(DigitalServicesError::missing_field("company_name"));
    }

    if designation.designated_services.is_empty() {
        return Err(DigitalServicesError::missing_field("designated_services"));
    }

    let thresholds = &designation.meets_quantitative_thresholds;

    let mut missing_criteria = Vec::new();

    if !thresholds.significant_impact_on_internal_market {
        missing_criteria
            .push("significant impact on internal market (€7.5B turnover or €75B market cap)");
    }

    if !thresholds.operates_in_multiple_member_states {
        missing_criteria.push("operates in at least 3 Member States");
    }

    if !thresholds.substantial_user_base {
        missing_criteria.push(
            "substantial user base (45M+ monthly active end users AND 10k+ yearly business users)",
        );
    }

    if !thresholds.entrenched_and_durable_position {
        missing_criteria.push("entrenched and durable position (met thresholds for 3 years)");
    }

    if !missing_criteria.is_empty() {
        return Err(DigitalServicesError::not_gatekeeper(
            missing_criteria.join("; "),
        ));
    }

    result.add_obligation("Gatekeeper must comply with all DMA obligations (Articles 5-7)");
    result.add_obligation("Submit compliance report every 6 months");
    result.add_obligation("Notify Commission of intended concentrations");
    result.add_obligation("Provide access to data for market monitoring");

    if designation.contested {
        result.add_warning("Designation is contested - pending Commission review");
    }

    Ok(result)
}

/// Validate DMA compliance report
pub fn validate_dma_compliance_report(
    report: &DmaComplianceReport,
) -> Result<DsaValidationResult, DigitalServicesError> {
    let mut result = DsaValidationResult::new();

    if report.period_end <= report.period_start {
        return Err(DigitalServicesError::invalid_value(
            "period_end",
            "Period end must be after period start",
        ));
    }

    // DMA requires 6-monthly reports
    let period_duration = report.period_end - report.period_start;
    if period_duration > Duration::days(200) {
        result.add_violation("DMA compliance reports must be submitted every 6 months");
    }

    if report.obligation_compliance.is_empty() {
        result.add_violation("Compliance report must address all applicable obligations");
    }

    if report.compliance_measures.is_empty() {
        result.add_violation("Compliance report must describe measures taken to ensure compliance");
    }

    // Check for violations in obligations
    let mut violation_count = 0;
    for obligation in &report.obligation_compliance {
        if !obligation.compliant {
            violation_count += 1;
            result.add_violation(format!(
                "Non-compliant with {:?}: {}",
                obligation.obligation, obligation.explanation
            ));
        }

        if obligation.explanation.is_empty() {
            result.add_violation(format!(
                "Missing explanation for {:?} compliance status",
                obligation.obligation
            ));
        }
    }

    if violation_count > 0 {
        result.add_warning(format!(
            "Gatekeeper non-compliant with {} DMA obligations - Commission may impose fines up to 10% of global turnover",
            violation_count
        ));
    }

    Ok(result)
}

/// Validate interoperability requirement (DMA Article 7)
pub fn validate_interoperability_requirement(
    requirement: &InteroperabilityRequirement,
) -> Result<DsaValidationResult, DigitalServicesError> {
    let mut result = DsaValidationResult::new();

    if requirement.description.is_empty() {
        return Err(DigitalServicesError::missing_field("description"));
    }

    if requirement.implementation_deadline < Utc::now() {
        result.add_violation("Interoperability implementation deadline has passed");
    }

    match &requirement.access_terms {
        InteroperabilityAccessTerms::Free => {
            result.add_obligation("Provide free interoperability access");
        }
        InteroperabilityAccessTerms::Frand { fee_structure } => {
            result.add_obligation("Provide FRAND (Fair, Reasonable, Non-Discriminatory) access");
            if fee_structure.is_none() {
                result.add_warning("FRAND terms should include clear fee structure");
            }
        }
    }

    // Specific requirements based on service type
    match requirement.service_type {
        CorePlatformService::InterpersonalCommunications => {
            result.add_obligation(
                "Gatekeepers must ensure interoperability for messaging services (Article 7)",
            );
        }
        CorePlatformService::OnlineSocialNetworking => {
            result.add_obligation(
                "Consider interoperability requirements for social networking features",
            );
        }
        _ => {}
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_classification_vlop() {
        let platform = PlatformType::VeryLargeOnlinePlatform {
            monthly_active_recipients: 50_000_000,
            designation_date: Utc::now(),
            systemic_risk_designation: true,
        };

        let result = validate_platform_classification(&platform).expect("validation failed");
        assert!(result.is_compliant());
        assert!(result.applicable_obligations.len() > 10);
        assert!(
            result
                .applicable_obligations
                .iter()
                .any(|o| o.contains("Systemic risk assessment"))
        );
    }

    #[test]
    fn test_platform_classification_below_threshold() {
        let platform = PlatformType::VeryLargeOnlinePlatform {
            monthly_active_recipients: 40_000_000, // Below 45M threshold
            designation_date: Utc::now(),
            systemic_risk_designation: false,
        };

        let result = validate_platform_classification(&platform);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DigitalServicesError::BelowVlopThreshold { .. }
        ));
    }

    #[test]
    fn test_illegal_content_notice_validation() {
        let notice = IllegalContentNotice {
            notice_id: "N12345".to_string(),
            submission_date: Utc::now(),
            content_type: IllegalContent::TerroristContent,
            content_location: "https://example.com/bad-content".to_string(),
            explanation: "This content promotes terrorism".to_string(),
            notifier_contact: "user@example.com".to_string(),
            is_trusted_flagger: false,
        };

        let result = validate_illegal_content_notice(&notice).expect("validation failed");
        assert!(result.is_compliant());
    }

    #[test]
    fn test_gatekeeper_designation_all_criteria_met() {
        let designation = GatekeeperDesignation {
            company_name: "BigTech Corp".to_string(),
            designated_services: vec![CorePlatformService::OnlineSearchEngines],
            designation_date: Utc::now(),
            meets_quantitative_thresholds: QuantitativeThresholds {
                significant_impact_on_internal_market: true,
                operates_in_multiple_member_states: true,
                substantial_user_base: true,
                entrenched_and_durable_position: true,
            },
            contested: false,
        };

        let result = validate_gatekeeper_designation(&designation).expect("validation failed");
        assert!(result.is_compliant());
        assert!(!result.applicable_obligations.is_empty());
    }

    #[test]
    fn test_gatekeeper_designation_missing_criteria() {
        let designation = GatekeeperDesignation {
            company_name: "SmallTech Inc".to_string(),
            designated_services: vec![CorePlatformService::OnlineSearchEngines],
            designation_date: Utc::now(),
            meets_quantitative_thresholds: QuantitativeThresholds {
                significant_impact_on_internal_market: false, // Missing
                operates_in_multiple_member_states: true,
                substantial_user_base: false, // Missing
                entrenched_and_durable_position: true,
            },
            contested: false,
        };

        let result = validate_gatekeeper_designation(&designation);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DigitalServicesError::NotGatekeeper { .. }
        ));
    }
}
