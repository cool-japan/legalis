//! Validation logic for EU AI Act compliance

use super::error::AiRegulationError;
use super::types::*;

/// AI Act validation result
#[derive(Debug, Clone, PartialEq)]
pub struct AiActValidationResult {
    /// Whether compliant with AI Act
    pub compliant: bool,
    /// List of violations
    pub violations: Vec<String>,
    /// List of warnings
    pub warnings: Vec<String>,
    /// Applicable requirements
    pub applicable_requirements: Vec<String>,
}

impl AiActValidationResult {
    /// Create new validation result
    pub fn new() -> Self {
        Self {
            compliant: true,
            violations: Vec::new(),
            warnings: Vec::new(),
            applicable_requirements: Vec::new(),
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

    /// Add applicable requirement
    pub fn add_requirement(&mut self, requirement: impl Into<String>) {
        self.applicable_requirements.push(requirement.into());
    }

    /// Check if compliant
    pub fn is_compliant(&self) -> bool {
        self.compliant
    }
}

impl Default for AiActValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate AI system classification
pub fn validate_ai_system(system: &AiSystem) -> Result<AiActValidationResult, AiRegulationError> {
    let mut result = AiActValidationResult::new();

    if system.system_id.is_empty() {
        return Err(AiRegulationError::missing_field("system_id"));
    }

    if system.name.is_empty() {
        return Err(AiRegulationError::missing_field("name"));
    }

    if system.provider.is_empty() {
        return Err(AiRegulationError::missing_field("provider"));
    }

    if system.intended_purpose.is_empty() {
        return Err(AiRegulationError::missing_field("intended_purpose"));
    }

    // Validate based on risk level
    match &system.risk_level {
        RiskLevel::Unacceptable {
            prohibited_practice,
        } => {
            return Err(AiRegulationError::prohibited_practice(format!(
                "{:?} - This AI system is prohibited under Article 5",
                prohibited_practice
            )));
        }
        RiskLevel::HighRisk { category } => {
            result.add_requirement("Conformity assessment (Article 43)");
            result.add_requirement("Risk management system (Article 9)");
            result.add_requirement("Data governance (Article 10)");
            result.add_requirement("Technical documentation (Article 11)");
            result.add_requirement("Record-keeping (Article 12)");
            result.add_requirement("Transparency and user information (Article 13)");
            result.add_requirement("Human oversight (Article 14)");
            result.add_requirement("Accuracy, robustness, cybersecurity (Article 15)");
            result.add_requirement("Registration in EU database (Article 71)");

            if system.market_placement_date.is_some() {
                // System already on market - check conformity
                match &system.conformity_status {
                    ConformityStatus::NotAssessed => {
                        result.add_violation(
                            "High-risk AI system on market without conformity assessment",
                        );
                    }
                    ConformityStatus::InProgress { .. } => {
                        result.add_warning(
                            "High-risk AI system with conformity assessment in progress",
                        );
                    }
                    ConformityStatus::Conformant { .. } => {
                        // OK
                    }
                    ConformityStatus::NonConformant { issues } => {
                        result.add_violation(format!(
                            "High-risk AI system non-conformant: {}",
                            issues.join(", ")
                        ));
                    }
                }
            } else {
                result.add_warning(
                    "High-risk AI system not yet placed on market - ensure conformity before placement",
                );
            }

            // Category-specific warnings
            match category {
                HighRiskCategory::BiometricIdentification => {
                    result.add_warning("Biometric identification systems subject to strict requirements and potential bans (Article 5)");
                }
                HighRiskCategory::LawEnforcement { .. } => {
                    result
                        .add_warning("Law enforcement AI systems subject to additional safeguards");
                }
                HighRiskCategory::Employment { .. } => {
                    result.add_warning("Employment AI systems must comply with labor law and anti-discrimination requirements");
                }
                _ => {}
            }
        }
        RiskLevel::LimitedRisk { system_type } => {
            result.add_requirement("Transparency obligation (Article 52)");
            result.add_requirement("Users must be informed of AI interaction");

            match system_type {
                LimitedRiskType::DeepFake => {
                    result.add_requirement(
                        "Content must be marked as artificially generated or manipulated",
                    );
                }
                LimitedRiskType::EmotionRecognition => {
                    result.add_warning(
                        "Emotion recognition in workplace/education is prohibited (Article 5)",
                    );
                }
                LimitedRiskType::BiometricCategorization => {
                    result.add_warning(
                        "Biometric categorization based on sensitive attributes is prohibited (Article 5)",
                    );
                }
                _ => {}
            }
        }
        RiskLevel::MinimalRisk => {
            result.add_requirement("Voluntary codes of conduct (Article 95)");
            result.add_warning("No specific legal obligations, but good practices encouraged");
        }
    }

    Ok(result)
}

/// Validate high-risk AI system requirements
pub fn validate_high_risk_requirements(
    requirements: &HighRiskRequirements,
) -> Result<AiActValidationResult, AiRegulationError> {
    let mut result = AiActValidationResult::new();

    // Validate risk management system (Article 9)
    validate_risk_management(&requirements.risk_management, &mut result)?;

    // Validate data governance (Article 10)
    validate_data_governance(&requirements.data_governance, &mut result)?;

    // Validate technical documentation (Article 11)
    validate_technical_documentation(&requirements.technical_documentation, &mut result)?;

    // Validate record-keeping (Article 12)
    validate_record_keeping(&requirements.record_keeping, &mut result)?;

    // Validate transparency (Article 13)
    validate_transparency_requirements(&requirements.transparency, &mut result)?;

    // Validate human oversight (Article 14)
    validate_human_oversight(&requirements.human_oversight, &mut result)?;

    // Validate accuracy and robustness (Article 15)
    validate_accuracy_robustness(&requirements.accuracy_robustness, &mut result)?;

    Ok(result)
}

fn validate_risk_management(
    risk_mgmt: &RiskManagementSystem,
    result: &mut AiActValidationResult,
) -> Result<(), AiRegulationError> {
    if !risk_mgmt.continuous_process {
        result.add_violation(
            "Risk management must be continuous iterative process throughout AI system lifecycle",
        );
    }

    if risk_mgmt.identified_risks.is_empty() {
        result.add_violation("Risk management system must identify and analyze risks");
    }

    if risk_mgmt.mitigation_measures.is_empty() {
        result.add_violation("Risk management system must include mitigation measures");
    }

    if !risk_mgmt.residual_risk_acceptable {
        result.add_violation("Residual risks after mitigation must be acceptable");
    }

    if risk_mgmt.testing_procedures.is_empty() {
        result.add_violation("Risk management system must include testing procedures");
    }

    // Check for critical unmitigated risks
    for risk in &risk_mgmt.identified_risks {
        if matches!(risk.severity, RiskSeverity::Critical | RiskSeverity::High) {
            result.add_warning(format!(
                "High/Critical severity risk identified: {}",
                risk.description
            ));
        }
    }

    Ok(())
}

fn validate_data_governance(
    data_gov: &DataGovernance,
    result: &mut AiActValidationResult,
) -> Result<(), AiRegulationError> {
    // Validate training data quality
    if !data_gov
        .training_data_quality
        .appropriate_statistical_properties
    {
        result.add_violation("Training data must have appropriate statistical properties");
    }
    if !data_gov.training_data_quality.complete_and_error_free {
        result.add_violation("Training data must be complete and free from errors");
    }
    if !data_gov.training_data_quality.representative {
        result.add_violation("Training data must be relevant, representative, and free of bias");
    }

    // Validate validation data quality
    if !data_gov.validation_data_quality.representative {
        result.add_violation("Validation data must be representative");
    }

    // Validate testing data quality
    if !data_gov.testing_data_quality.representative {
        result.add_violation("Testing data must be representative");
    }

    // Validate bias examination
    if !data_gov.bias_examination.conducted {
        result.add_violation("Data must be examined for possible biases (Article 10(2)(f))");
    } else {
        if !data_gov.bias_examination.identified_biases.is_empty()
            && data_gov.bias_examination.bias_mitigation.is_empty()
        {
            return Err(AiRegulationError::bias_not_mitigated(
                data_gov.bias_examination.identified_biases.join(", "),
            ));
        }
    }

    if data_gov.data_representativeness.is_empty() {
        result.add_violation("Must document data relevance and representativeness");
    }

    Ok(())
}

fn validate_technical_documentation(
    tech_doc: &TechnicalDocumentation,
    result: &mut AiActValidationResult,
) -> Result<(), AiRegulationError> {
    let mut missing = Vec::new();

    if tech_doc.general_description.is_empty() {
        missing.push("general description");
    }
    if tech_doc.detailed_description.is_empty() {
        missing.push("detailed description");
    }
    if tech_doc.data_description.is_empty() {
        missing.push("data description");
    }
    if tech_doc.monitoring_mechanisms.is_empty() {
        missing.push("monitoring mechanisms");
    }
    if tech_doc.validation_testing.is_empty() {
        missing.push("validation and testing procedures");
    }

    if !missing.is_empty() {
        return Err(AiRegulationError::missing_technical_documentation(
            missing.join(", "),
        ));
    }

    if tech_doc.modification_log.is_empty() {
        result.add_warning("Technical documentation should include modification log");
    }

    Ok(())
}

fn validate_record_keeping(
    record_keeping: &RecordKeeping,
    result: &mut AiActValidationResult,
) -> Result<(), AiRegulationError> {
    if !record_keeping.automatic_logging {
        return Err(AiRegulationError::record_keeping_violation(
            "High-risk AI systems must have automatic logging capability",
        ));
    }

    if record_keeping.retention_period_months < 6 {
        result.add_warning("Consider retention period of at least 6 months for logs");
    }

    if record_keeping.logged_events.is_empty() {
        return Err(AiRegulationError::record_keeping_violation(
            "Must specify events to be logged",
        ));
    }

    // Check for essential logged events
    let has_input = record_keeping
        .logged_events
        .iter()
        .any(|e| matches!(e, LoggedEvent::InputData));
    let has_output = record_keeping
        .logged_events
        .iter()
        .any(|e| matches!(e, LoggedEvent::Output));

    if !has_input || !has_output {
        result.add_violation("Logs should include at least input data and outputs");
    }

    Ok(())
}

fn validate_transparency_requirements(
    transparency: &TransparencyRequirements,
    result: &mut AiActValidationResult,
) -> Result<(), AiRegulationError> {
    if transparency.instructions_for_use.is_empty() {
        return Err(AiRegulationError::insufficient_transparency(
            "Instructions for use required",
        ));
    }

    if !transparency.information_quality {
        result.add_violation("Information must be concise, complete, correct, and clear");
    }

    if transparency.foreseeable_misuse.is_empty() {
        result.add_warning("Should identify reasonably foreseeable misuse");
    }

    if transparency.accuracy_metrics.is_empty() {
        result.add_violation("Must disclose level of accuracy and relevant metrics");
    }

    Ok(())
}

fn validate_human_oversight(
    oversight: &HumanOversight,
    result: &mut AiActValidationResult,
) -> Result<(), AiRegulationError> {
    if oversight.measures.is_empty() {
        return Err(AiRegulationError::MissingHumanOversight);
    }

    if !oversight.output_interpretable {
        result.add_violation("Humans must be able to interpret system outputs");
    }

    if !oversight.override_capability {
        result.add_violation("Humans must be able to override or stop the system");
    }

    Ok(())
}

fn validate_accuracy_robustness(
    accuracy: &AccuracyRobustness,
    result: &mut AiActValidationResult,
) -> Result<(), AiRegulationError> {
    if accuracy.accuracy_level < 0.0 || accuracy.accuracy_level > 1.0 {
        return Err(AiRegulationError::invalid_value(
            "accuracy_level",
            "Must be between 0.0 and 1.0",
        ));
    }

    if accuracy.accuracy_metrics.is_empty() {
        result.add_violation("Must specify accuracy metrics used");
    }

    if accuracy.robustness_measures.is_empty() {
        result
            .add_violation("Must implement robustness measures against errors and inconsistencies");
    }

    if accuracy.cybersecurity_measures.is_empty() {
        result.add_violation("Must implement cybersecurity measures (Article 15)");
    }

    if !accuracy.adversarial_robustness {
        result.add_warning("Consider resilience to adversarial attacks");
    }

    Ok(())
}

/// Validate transparency obligation for limited risk systems (Article 52)
pub fn validate_transparency_obligation(
    obligation: &TransparencyObligation,
) -> Result<AiActValidationResult, AiRegulationError> {
    let mut result = AiActValidationResult::new();

    if !obligation.users_informed {
        return Err(AiRegulationError::LimitedRiskTransparencyViolation);
    }

    if obligation.notification_method.is_empty() {
        result.add_violation("Must specify method of notifying users about AI interaction");
    }

    match obligation.system_type {
        LimitedRiskType::DeepFake => {
            if !obligation.content_marked {
                return Err(AiRegulationError::DeepFakeNotMarked);
            }
        }
        LimitedRiskType::EmotionRecognition => {
            result.add_warning(
                "Emotion recognition in workplace/education is prohibited (Article 5)",
            );
        }
        LimitedRiskType::BiometricCategorization => {
            result.add_warning(
                "Biometric categorization inferring sensitive attributes is prohibited (Article 5)",
            );
        }
        _ => {}
    }

    Ok(result)
}

/// Validate general-purpose AI model (Article 51)
pub fn validate_general_purpose_ai(
    model: &GeneralPurposeAiModel,
) -> Result<AiActValidationResult, AiRegulationError> {
    let mut result = AiActValidationResult::new();

    if model.name.is_empty() {
        return Err(AiRegulationError::missing_field("name"));
    }

    if model.provider.is_empty() {
        return Err(AiRegulationError::missing_field("provider"));
    }

    if !model.transparency_documentation {
        return Err(AiRegulationError::GpaiTransparencyViolation);
    }

    if model.training_data_summary.is_empty() {
        result.add_violation("Must provide summary of training data used");
    }

    if model.capabilities.is_empty() {
        result.add_violation("Must document capabilities and limitations");
    }

    if model.systemic_risk {
        result.add_requirement("Additional requirements for AI models with systemic risk");
        result.add_requirement("Model evaluation (Article 51)");
        result.add_requirement("Adversarial testing");
        result.add_requirement("Tracking and reporting serious incidents");
        result.add_requirement("Ensuring adequate level of cybersecurity protection");
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prohibited_practice() {
        let system = AiSystem {
            system_id: "AI001".to_string(),
            name: "Social Scoring System".to_string(),
            description: "Social credit system".to_string(),
            provider: "BadAI Corp".to_string(),
            deployer: None,
            intended_purpose: "Citizen scoring".to_string(),
            risk_level: RiskLevel::Unacceptable {
                prohibited_practice: ProhibitedPractice::SocialScoring,
            },
            adaptive: true,
            market_placement_date: None,
            conformity_status: ConformityStatus::NotAssessed,
        };

        let result = validate_ai_system(&system);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AiRegulationError::ProhibitedPractice { .. }
        ));
    }

    #[test]
    fn test_high_risk_requirements() {
        let system = AiSystem {
            system_id: "AI002".to_string(),
            name: "Hiring AI".to_string(),
            description: "AI for candidate screening".to_string(),
            provider: "HRTech Inc".to_string(),
            deployer: Some("BigCorp".to_string()),
            intended_purpose: "Resume screening and ranking".to_string(),
            risk_level: RiskLevel::HighRisk {
                category: HighRiskCategory::Employment {
                    use_case: "recruitment".to_string(),
                },
            },
            adaptive: true,
            market_placement_date: None,
            conformity_status: ConformityStatus::NotAssessed,
        };

        let result = validate_ai_system(&system).expect("validation failed");
        assert!(result.applicable_requirements.len() >= 8);
        assert!(
            result
                .applicable_requirements
                .iter()
                .any(|r| r.contains("Risk management"))
        );
    }

    #[test]
    fn test_limited_risk_transparency() {
        let obligation = TransparencyObligation {
            system_type: LimitedRiskType::Chatbot,
            users_informed: true,
            notification_method: "Disclosure message at start of conversation".to_string(),
            content_marked: false,
        };

        let result = validate_transparency_obligation(&obligation).expect("validation failed");
        assert!(result.is_compliant());
    }

    #[test]
    fn test_deepfake_not_marked() {
        let obligation = TransparencyObligation {
            system_type: LimitedRiskType::DeepFake,
            users_informed: true,
            notification_method: "Watermark".to_string(),
            content_marked: false, // Not marked!
        };

        let result = validate_transparency_obligation(&obligation);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AiRegulationError::DeepFakeNotMarked
        ));
    }
}
