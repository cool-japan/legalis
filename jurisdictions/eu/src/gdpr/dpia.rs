//! GDPR Article 35 - Data Protection Impact Assessment (DPIA)
//!
//! This module implements the DPIA framework required under Article 35 GDPR.
//!
//! ## When DPIA is Required (Article 35(1))
//!
//! A DPIA is required when processing is "likely to result in a high risk to the rights
//! and freedoms of natural persons," particularly for:
//!
//! 1. Systematic and extensive evaluation based on automated processing (including profiling)
//! 2. Large-scale processing of special categories (Article 9) or criminal data (Article 10)
//! 3. Systematic monitoring of publicly accessible areas on a large scale
//!
//! ## DPIA Process
//!
//! A DPIA must contain (Article 35(7)):
//! - Description of processing operations and purposes
//! - Assessment of necessity and proportionality
//! - Assessment of risks to data subjects' rights
//! - Measures to address risks (including safeguards, security, and mechanisms)
//!
//! ## Example
//!
//! ```rust
//! use legalis_eu::gdpr::dpia::*;
//!
//! let dpia = DataProtectionImpactAssessment::new()
//!     .with_processing_description("AI-powered recruitment screening")
//!     .with_purpose("Automated candidate evaluation")
//!     .add_trigger(DpiaTrigger::AutomatedDecisionMaking {
//!         produces_legal_effects: true,
//!         systematic: true,
//!         extensive: true,
//!     })
//!     .with_necessity_assessment("Required to handle 10,000+ applications/month")
//!     .add_risk(RiskAssessment {
//!         risk_type: RiskType::Discrimination,
//!         likelihood: Likelihood::High,
//!         severity: Severity::High,
//!         description: "AI may exhibit bias against protected groups".to_string(),
//!     })
//!     .add_mitigation(Mitigation {
//!         risk_addressed: RiskType::Discrimination,
//!         measure: "Regular algorithmic fairness audits".to_string(),
//!         effectiveness: Effectiveness::High,
//!     });
//!
//! match dpia.validate() {
//!     Ok(result) => {
//!         if result.prior_consultation_required {
//!             println!("⚠️ Must consult supervisory authority before processing");
//!         }
//!     }
//!     Err(e) => println!("DPIA incomplete: {}", e),
//! }
//! ```

use crate::gdpr::error::GdprError;
use crate::gdpr::types::{PersonalDataCategory, ProcessingOperation, SpecialCategory};
use chrono::{DateTime, Utc};
use legalis_core::LegalResult;
use uuid::Uuid;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Triggers requiring a DPIA under Article 35(3)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DpiaTrigger {
    /// Article 35(3)(a): Systematic and extensive automated decision-making
    AutomatedDecisionMaking {
        /// Whether processing produces legal or similarly significant effects
        produces_legal_effects: bool,
        /// Whether evaluation is systematic
        systematic: bool,
        /// Whether evaluation is extensive
        extensive: bool,
    },

    /// Article 35(3)(b): Large-scale processing of special categories
    LargeScaleSpecialCategories {
        /// Type of special category data
        categories: Vec<SpecialCategory>,
        /// Number of data subjects affected
        scale: u64,
    },

    /// Article 35(3)(c): Systematic monitoring of publicly accessible areas
    SystematicMonitoring {
        /// Type of monitoring (CCTV, facial recognition, etc.)
        monitoring_type: String,
        /// Whether monitoring is large-scale
        large_scale: bool,
        /// Geographic scope
        scope: String,
    },

    /// WP29 Guidelines: New technologies with high risk
    NewTechnology {
        /// Description of technology
        technology: String,
        /// Why it poses high risk
        risk_rationale: String,
    },

    /// WP29 Guidelines: Profiling or scoring
    ProfilingOrScoring {
        /// Type of profiling
        profiling_type: String,
        /// Whether creates legal/significant effects
        significant_effects: bool,
    },

    /// WP29 Guidelines: Data matching/combining from multiple sources
    DataMatching {
        /// Description of data sources
        sources: Vec<String>,
        /// Purpose of matching
        purpose: String,
    },

    /// WP29 Guidelines: Processing vulnerable data subjects
    VulnerableDataSubjects {
        /// Type of vulnerability (children, elderly, employees, etc.)
        vulnerability_type: String,
        /// Number affected
        affected_count: u64,
    },

    /// WP29 Guidelines: Innovative use or new organizational/technological solutions
    InnovativeUse {
        /// Description of innovation
        innovation: String,
    },

    /// WP29 Guidelines: Transfer outside EU without adequate safeguards
    CrossBorderTransferHighRisk {
        /// Destination country
        destination: String,
        /// Why high risk
        risk_description: String,
    },
}

impl DpiaTrigger {
    /// Check if trigger mandates DPIA
    pub fn is_mandatory(&self) -> bool {
        match self {
            Self::AutomatedDecisionMaking {
                produces_legal_effects,
                systematic,
                extensive,
            } => *produces_legal_effects && *systematic && *extensive,

            Self::LargeScaleSpecialCategories { scale, .. } => {
                // WP29: "Large-scale" not precisely defined, but >1000 subjects generally considered large-scale
                *scale >= 1000
            }

            Self::SystematicMonitoring { large_scale, .. } => *large_scale,

            Self::VulnerableDataSubjects { affected_count, .. } => *affected_count >= 100,

            // Other triggers are indicative, not mandatory
            _ => false,
        }
    }
}

/// Risk types identified in DPIA
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RiskType {
    /// Unlawful or accidental data loss/destruction
    DataLoss,

    /// Unauthorized access or disclosure
    UnauthorizedAccess,

    /// Discrimination or unfair treatment
    Discrimination,

    /// Identity theft or fraud
    IdentityTheft,

    /// Financial loss
    FinancialLoss,

    /// Reputational damage
    ReputationalDamage,

    /// Physical harm
    PhysicalHarm,

    /// Loss of confidentiality
    LossOfConfidentiality,

    /// Loss of availability
    LossOfAvailability,

    /// Loss of integrity
    LossOfIntegrity,

    /// Inability to exercise rights
    RightsViolation,

    /// Other risk
    Other(String),
}

/// Likelihood of risk occurring
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Likelihood {
    /// Risk is unlikely to occur
    Low,
    /// Risk may occur
    Medium,
    /// Risk is likely to occur
    High,
}

/// Severity of impact if risk occurs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Severity {
    /// Limited impact
    Low,
    /// Moderate impact
    Medium,
    /// Severe impact
    High,
}

/// Risk assessment for DPIA
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RiskAssessment {
    /// Type of risk
    pub risk_type: RiskType,

    /// Likelihood of occurrence
    pub likelihood: Likelihood,

    /// Severity of impact
    pub severity: Severity,

    /// Description of risk
    pub description: String,
}

impl RiskAssessment {
    /// Calculate overall risk level (likelihood × severity)
    pub fn risk_level(&self) -> RiskLevel {
        match (self.likelihood, self.severity) {
            (Likelihood::High, Severity::High) => RiskLevel::VeryHigh,
            (Likelihood::High, Severity::Medium) | (Likelihood::Medium, Severity::High) => {
                RiskLevel::High
            }
            (Likelihood::High, Severity::Low)
            | (Likelihood::Medium, Severity::Medium)
            | (Likelihood::Low, Severity::High) => RiskLevel::Medium,
            (Likelihood::Medium, Severity::Low) | (Likelihood::Low, Severity::Medium) => {
                RiskLevel::Low
            }
            (Likelihood::Low, Severity::Low) => RiskLevel::VeryLow,
        }
    }
}

/// Overall risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RiskLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Effectiveness of mitigation measure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Effectiveness {
    /// Mitigation has limited effectiveness
    Low,
    /// Mitigation partially addresses risk
    Medium,
    /// Mitigation substantially addresses risk
    High,
}

/// Mitigation measure to address risk
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mitigation {
    /// Risk being addressed
    pub risk_addressed: RiskType,

    /// Description of mitigation measure
    pub measure: String,

    /// Effectiveness of measure
    pub effectiveness: Effectiveness,
}

/// Data Protection Impact Assessment (DPIA) - Article 35
#[derive(Debug, Clone)]
pub struct DataProtectionImpactAssessment {
    /// Controller conducting DPIA
    pub controller: Option<String>,

    /// Date DPIA conducted
    pub conducted_date: Option<DateTime<Utc>>,

    /// Description of processing operations
    pub processing_description: Option<String>,

    /// Purpose of processing
    pub purpose: Option<String>,

    /// Data categories being processed
    pub data_categories: Vec<PersonalDataCategory>,

    /// Processing operations
    pub operations: Vec<ProcessingOperation>,

    /// Triggers requiring DPIA
    pub triggers: Vec<DpiaTrigger>,

    /// Assessment of necessity
    pub necessity_assessment: Option<String>,

    /// Assessment of proportionality
    pub proportionality_assessment: Option<String>,

    /// Identified risks
    pub risks: Vec<RiskAssessment>,

    /// Mitigation measures
    pub mitigations: Vec<Mitigation>,

    /// Whether data subjects were consulted
    pub data_subjects_consulted: bool,

    /// Whether DPO was consulted
    pub dpo_consulted: bool,

    /// DPO's opinion (if consulted)
    pub dpo_opinion: Option<String>,
}

impl DataProtectionImpactAssessment {
    pub fn new() -> Self {
        Self {
            controller: None,
            conducted_date: None,
            processing_description: None,
            purpose: None,
            data_categories: Vec::new(),
            operations: Vec::new(),
            triggers: Vec::new(),
            necessity_assessment: None,
            proportionality_assessment: None,
            risks: Vec::new(),
            mitigations: Vec::new(),
            data_subjects_consulted: false,
            dpo_consulted: false,
            dpo_opinion: None,
        }
    }

    pub fn with_controller(mut self, controller: impl Into<String>) -> Self {
        self.controller = Some(controller.into());
        self
    }

    pub fn with_conducted_date(mut self, date: DateTime<Utc>) -> Self {
        self.conducted_date = Some(date);
        self
    }

    pub fn with_processing_description(mut self, description: impl Into<String>) -> Self {
        self.processing_description = Some(description.into());
        self
    }

    pub fn with_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = Some(purpose.into());
        self
    }

    pub fn add_data_category(mut self, category: PersonalDataCategory) -> Self {
        self.data_categories.push(category);
        self
    }

    pub fn add_operation(mut self, operation: ProcessingOperation) -> Self {
        self.operations.push(operation);
        self
    }

    pub fn add_trigger(mut self, trigger: DpiaTrigger) -> Self {
        self.triggers.push(trigger);
        self
    }

    pub fn with_necessity_assessment(mut self, assessment: impl Into<String>) -> Self {
        self.necessity_assessment = Some(assessment.into());
        self
    }

    pub fn with_proportionality_assessment(mut self, assessment: impl Into<String>) -> Self {
        self.proportionality_assessment = Some(assessment.into());
        self
    }

    pub fn add_risk(mut self, risk: RiskAssessment) -> Self {
        self.risks.push(risk);
        self
    }

    pub fn add_mitigation(mut self, mitigation: Mitigation) -> Self {
        self.mitigations.push(mitigation);
        self
    }

    pub fn with_data_subjects_consulted(mut self, consulted: bool) -> Self {
        self.data_subjects_consulted = consulted;
        self
    }

    pub fn with_dpo_consulted(mut self, consulted: bool) -> Self {
        self.dpo_consulted = consulted;
        self
    }

    pub fn with_dpo_opinion(mut self, opinion: impl Into<String>) -> Self {
        self.dpo_opinion = Some(opinion.into());
        self
    }

    /// Validate DPIA completeness and determine if processing can proceed
    ///
    /// Returns `DpiaValidation` indicating:
    /// - Whether DPIA is complete (Article 35(7) requirements)
    /// - Whether residual risk remains high
    /// - Whether prior consultation with supervisory authority is required (Article 36)
    pub fn validate(&self) -> Result<DpiaValidation, GdprError> {
        // Check required fields (Article 35(7))
        if self.processing_description.is_none() {
            return Err(GdprError::missing_field(
                "processing_description (Article 35(7)(a))",
            ));
        }

        if self.purpose.is_none() {
            return Err(GdprError::missing_field("purpose (Article 35(7)(a))"));
        }

        if self.necessity_assessment.is_none() {
            return Err(GdprError::missing_field(
                "necessity_assessment (Article 35(7)(b))",
            ));
        }

        if self.proportionality_assessment.is_none() {
            return Err(GdprError::missing_field(
                "proportionality_assessment (Article 35(7)(b))",
            ));
        }

        if self.risks.is_empty() {
            return Err(GdprError::missing_field(
                "risks (Article 35(7)(c) - must assess risks)",
            ));
        }

        if self.mitigations.is_empty() {
            return Err(GdprError::missing_field(
                "mitigations (Article 35(7)(d) - must propose measures)",
            ));
        }

        // Calculate residual risk (highest remaining risk after mitigations)
        let residual_risk = self.calculate_residual_risk();

        // Article 36(1): Prior consultation required if DPIA shows high residual risk
        let prior_consultation_required =
            matches!(residual_risk, RiskLevel::High | RiskLevel::VeryHigh);

        // Check if any mandatory triggers present
        let has_mandatory_trigger = self.triggers.iter().any(|t| t.is_mandatory());

        Ok(DpiaValidation {
            dpia_complete: LegalResult::Deterministic(true),
            residual_risk_level: residual_risk,
            prior_consultation_required,
            processing_may_proceed: if prior_consultation_required {
                LegalResult::JudicialDiscretion {
                    issue: "High residual risk identified - supervisory authority consultation required under Article 36(1)".to_string(),
                    context_id: Uuid::new_v4(),
                    narrative_hint: Some(
                        "Controller must consult supervisory authority before processing. \
                         Consultation must describe: (1) processing operations, (2) DPIA findings, \
                         (3) measures to address risk. Authority has 8 weeks to respond (extendable to 14 weeks)."
                            .to_string(),
                    ),
                }
            } else {
                LegalResult::Deterministic(true)
            },
            has_mandatory_trigger,
            recommendations: self.generate_recommendations(),
        })
    }

    /// Calculate residual risk after mitigations
    fn calculate_residual_risk(&self) -> RiskLevel {
        // Find highest risk level among unmitigated or partially mitigated risks
        self.risks
            .iter()
            .map(|risk| {
                // Check if risk has effective mitigation
                let has_high_mitigation = self.mitigations.iter().any(|m| {
                    m.risk_addressed == risk.risk_type
                        && matches!(m.effectiveness, Effectiveness::High)
                });

                if has_high_mitigation {
                    // Reduce risk level by one step if highly mitigated
                    match risk.risk_level() {
                        RiskLevel::VeryHigh => RiskLevel::High,
                        RiskLevel::High => RiskLevel::Medium,
                        RiskLevel::Medium => RiskLevel::Low,
                        RiskLevel::Low => RiskLevel::VeryLow,
                        RiskLevel::VeryLow => RiskLevel::VeryLow,
                    }
                } else {
                    risk.risk_level()
                }
            })
            .max()
            .unwrap_or(RiskLevel::VeryLow)
    }

    /// Generate recommendations based on DPIA findings
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Check for unmitigated high risks
        for risk in &self.risks {
            if risk.risk_level() >= RiskLevel::High {
                let has_mitigation = self
                    .mitigations
                    .iter()
                    .any(|m| m.risk_addressed == risk.risk_type);

                if !has_mitigation {
                    recommendations.push(format!(
                        "Implement mitigation for high risk: {:?} - {}",
                        risk.risk_type, risk.description
                    ));
                }
            }
        }

        // Recommend DPO consultation if not done
        if !self.dpo_consulted {
            recommendations
                .push("Consider consulting Data Protection Officer (Article 35(2))".to_string());
        }

        // Recommend data subject consultation for high-risk processing
        if !self.data_subjects_consulted && self.calculate_residual_risk() >= RiskLevel::High {
            recommendations.push(
                "Consider consulting data subjects or their representatives (Article 35(9))"
                    .to_string(),
            );
        }

        recommendations
    }
}

impl Default for DataProtectionImpactAssessment {
    fn default() -> Self {
        Self::new()
    }
}

/// DPIA validation result
#[derive(Debug, Clone)]
pub struct DpiaValidation {
    /// Whether DPIA meets Article 35(7) requirements
    pub dpia_complete: LegalResult<bool>,

    /// Residual risk level after mitigations
    pub residual_risk_level: RiskLevel,

    /// Whether prior consultation with supervisory authority required (Article 36)
    pub prior_consultation_required: bool,

    /// Whether processing may proceed
    pub processing_may_proceed: LegalResult<bool>,

    /// Whether DPIA has mandatory triggers (Article 35(3))
    pub has_mandatory_trigger: bool,

    /// Recommendations for improvement
    pub recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpia_complete() {
        let dpia = DataProtectionImpactAssessment::new()
            .with_controller("Acme Corp")
            .with_processing_description("AI recruitment screening")
            .with_purpose("Automated candidate evaluation")
            .with_necessity_assessment("Required for scale")
            .with_proportionality_assessment("Measures are proportionate")
            .add_risk(RiskAssessment {
                risk_type: RiskType::Discrimination,
                likelihood: Likelihood::Medium,
                severity: Severity::Medium,
                description: "Potential AI bias".to_string(),
            })
            .add_mitigation(Mitigation {
                risk_addressed: RiskType::Discrimination,
                measure: "Fairness audits".to_string(),
                effectiveness: Effectiveness::High,
            });

        let result = dpia.validate().unwrap();
        assert!(matches!(
            result.dpia_complete,
            LegalResult::Deterministic(true)
        ));
        assert!(!result.prior_consultation_required);
    }

    #[test]
    fn test_dpia_missing_necessity() {
        let dpia = DataProtectionImpactAssessment::new()
            .with_processing_description("Test processing")
            .with_purpose("Test");

        let result = dpia.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_high_residual_risk_requires_consultation() {
        let dpia = DataProtectionImpactAssessment::new()
            .with_controller("Acme Corp")
            .with_processing_description("Facial recognition")
            .with_purpose("Public surveillance")
            .with_necessity_assessment("Security needs")
            .with_proportionality_assessment("Proportionate")
            .add_risk(RiskAssessment {
                risk_type: RiskType::RightsViolation,
                likelihood: Likelihood::High,
                severity: Severity::High,
                description: "Mass surveillance risk".to_string(),
            })
            .add_mitigation(Mitigation {
                risk_addressed: RiskType::RightsViolation,
                measure: "Limited retention".to_string(),
                effectiveness: Effectiveness::Low, // Low effectiveness = high residual risk
            });

        let result = dpia.validate().unwrap();
        assert!(result.prior_consultation_required);
        assert!(matches!(
            result.processing_may_proceed,
            LegalResult::JudicialDiscretion { .. }
        ));
    }

    #[test]
    fn test_mandatory_trigger_large_scale_special_categories() {
        let trigger = DpiaTrigger::LargeScaleSpecialCategories {
            categories: vec![SpecialCategory::HealthData],
            scale: 5000,
        };

        assert!(trigger.is_mandatory());
    }

    #[test]
    fn test_mandatory_trigger_automated_decision_making() {
        let trigger = DpiaTrigger::AutomatedDecisionMaking {
            produces_legal_effects: true,
            systematic: true,
            extensive: true,
        };

        assert!(trigger.is_mandatory());
    }

    #[test]
    fn test_risk_level_calculation() {
        let risk = RiskAssessment {
            risk_type: RiskType::DataLoss,
            likelihood: Likelihood::High,
            severity: Severity::High,
            description: "Severe data loss risk".to_string(),
        };

        assert_eq!(risk.risk_level(), RiskLevel::VeryHigh);
    }

    #[test]
    fn test_recommendations_generated() {
        let dpia = DataProtectionImpactAssessment::new()
            .with_processing_description("Test")
            .with_purpose("Test")
            .with_necessity_assessment("Required")
            .with_proportionality_assessment("Proportionate")
            .add_risk(RiskAssessment {
                risk_type: RiskType::UnauthorizedAccess,
                likelihood: Likelihood::High,
                severity: Severity::High,
                description: "High access risk".to_string(),
            })
            .add_mitigation(Mitigation {
                risk_addressed: RiskType::DataLoss, // Different risk - leaves UnauthorizedAccess unmitigated
                measure: "Backups".to_string(),
                effectiveness: Effectiveness::High,
            });

        let result = dpia.validate().unwrap();
        assert!(!result.recommendations.is_empty());
        assert!(
            result
                .recommendations
                .iter()
                .any(|r| r.contains("UnauthorizedAccess"))
        );
    }
}
