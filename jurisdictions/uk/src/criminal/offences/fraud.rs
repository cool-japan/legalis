//! UK Fraud Offences
//!
//! This module covers fraud offences under the Fraud Act 2006.
//!
//! # Statutory Framework
//!
//! The Fraud Act 2006 created three ways of committing fraud:
//! - **Fraud by false representation** (s.2) - max 10 years
//! - **Fraud by failing to disclose** (s.3) - max 10 years
//! - **Fraud by abuse of position** (s.4) - max 10 years
//!
//! Related offences:
//! - **Obtaining services dishonestly** (s.11) - max 5 years
//! - **Making or supplying articles for fraud** (s.7) - max 10 years
//! - **Possession of articles for fraud** (s.6) - max 5 years
//!
//! # Key Features
//!
//! - Conduct crime: no need to prove actual loss or gain
//! - Dishonesty test: Ivey v Genting Casinos \[2017\]
//! - Replaced old deception offences under Theft Acts

// Allow missing docs on enum variant struct fields - these are self-documenting in context
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::criminal::error::{CriminalError, CriminalResult};
use crate::criminal::types::{
    ActType, ActusReusElement, CaseCitation, DishonestyAnalysis, MaximumSentence, MensReaType,
    Offence, OffenceBuilder, OffenceCategory, OffenceClassification, OffenceSeverity,
};

// ============================================================================
// Fraud Types
// ============================================================================

/// Type of fraud under Fraud Act 2006
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FraudType {
    /// s.2 - False representation
    FalseRepresentation,
    /// s.3 - Failing to disclose information
    FailingToDisclose,
    /// s.4 - Abuse of position
    AbuseOfPosition,
}

/// Related fraud offences
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelatedFraudOffence {
    /// s.6 - Possession of articles for fraud
    PossessionForFraud,
    /// s.7 - Making/supplying articles for fraud
    MakingOrSupplying,
    /// s.11 - Obtaining services dishonestly
    ObtainingServices,
}

// ============================================================================
// Fraud by False Representation (s.2)
// ============================================================================

/// Facts for fraud by false representation analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FalseRepresentationFacts {
    /// The representation made
    pub representation: RepresentationDetails,
    /// Dishonesty facts
    pub dishonesty: FraudDishonestyFacts,
    /// Intent to make gain or cause loss
    pub intent: GainLossIntent,
}

/// Details of representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepresentationDetails {
    /// Description of representation
    pub description: String,
    /// Type of representation
    pub representation_type: RepresentationType,
    /// Was representation false?
    pub false_representation: bool,
    /// Did D know it was or might be untrue/misleading?
    pub knew_untrue_misleading: bool,
}

/// Types of representation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepresentationType {
    /// Representation as to fact
    Fact,
    /// Representation as to law
    Law,
    /// Representation as to state of mind
    StateOfMind,
    /// Express representation
    Express { words: String },
    /// Implied representation
    Implied { conduct: String },
    /// Representation to machine/system (s.2(5))
    ToMachine,
}

/// Dishonesty facts for fraud
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FraudDishonestyFacts {
    /// D's actual knowledge/belief
    pub actual_knowledge_belief: String,
    /// Would ordinary person consider it dishonest?
    pub dishonest_by_ordinary_standards: bool,
}

/// Intent to make gain or cause loss (s.5)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GainLossIntent {
    /// Intent to make gain for self
    pub intent_gain_self: bool,
    /// Intent to make gain for another
    pub intent_gain_other: bool,
    /// Intent to cause loss to another
    pub intent_cause_loss: bool,
    /// Intent to expose another to risk of loss
    pub intent_expose_to_loss: bool,
    /// Evidence of intent
    pub evidence: Vec<String>,
}

/// Result of false representation fraud analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FalseRepresentationResult {
    /// Offence established?
    pub established: bool,
    /// Representation analysis
    pub representation_analysis: RepresentationAnalysis,
    /// Dishonesty analysis
    pub dishonesty_analysis: DishonestyAnalysis,
    /// Intent analysis
    pub intent_analysis: IntentAnalysis,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Representation analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepresentationAnalysis {
    /// Representation established?
    pub established: bool,
    /// Was it false?
    pub false_established: bool,
    /// Did D know?
    pub knowledge_established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Intent analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentAnalysis {
    /// Intent established?
    pub established: bool,
    /// Type of intent
    pub intent_type: String,
    /// Reasoning
    pub reasoning: String,
}

/// Analyzer for false representation fraud
pub struct FalseRepresentationAnalyzer;

impl FalseRepresentationAnalyzer {
    /// Analyze fraud by false representation
    pub fn analyze(facts: &FalseRepresentationFacts) -> CriminalResult<FalseRepresentationResult> {
        let case_law = vec![
            CaseCitation::new(
                "R v Silverman",
                1988,
                "86 Cr App R 213",
                "Excessive quotation is false representation",
            ),
            CaseCitation::new(
                "Ivey v Genting Casinos",
                2017,
                "UKSC 67",
                "Two-stage dishonesty test",
            ),
        ];

        // Analyze representation
        let representation_analysis = Self::analyze_representation(&facts.representation);

        // Analyze dishonesty (Ivey test)
        let dishonesty_analysis = DishonestyAnalysis {
            defendants_knowledge: facts.dishonesty.actual_knowledge_belief.clone(),
            dishonest_by_ordinary_standards: facts.dishonesty.dishonest_by_ordinary_standards,
            reasoning: if facts.dishonesty.dishonest_by_ordinary_standards {
                "Given D's knowledge/belief, ordinary person would consider conduct dishonest"
                    .into()
            } else {
                "Not dishonest by ordinary standards given D's state of knowledge".into()
            },
        };

        // Analyze intent
        let intent_analysis = Self::analyze_intent(&facts.intent);

        let established = representation_analysis.established
            && representation_analysis.false_established
            && representation_analysis.knowledge_established
            && dishonesty_analysis.dishonest_by_ordinary_standards
            && intent_analysis.established;

        Ok(FalseRepresentationResult {
            established,
            representation_analysis,
            dishonesty_analysis,
            intent_analysis,
            case_law,
        })
    }

    fn analyze_representation(facts: &RepresentationDetails) -> RepresentationAnalysis {
        let mut reasoning_parts = Vec::new();

        // Check representation exists
        reasoning_parts.push(format!(
            "Representation: {} ({:?})",
            facts.description, facts.representation_type
        ));

        // Check if false
        let false_established = facts.false_representation;
        if false_established {
            reasoning_parts.push("Representation was untrue or misleading".into());
        } else {
            reasoning_parts.push("Representation was not false".into());
        }

        // Check knowledge
        let knowledge_established = facts.knew_untrue_misleading;
        if knowledge_established {
            reasoning_parts.push("D knew representation was/might be untrue or misleading".into());
        } else {
            reasoning_parts.push("D did not know representation was false".into());
        }

        RepresentationAnalysis {
            established: true, // A representation was made
            false_established,
            knowledge_established,
            reasoning: reasoning_parts.join("; "),
        }
    }

    fn analyze_intent(facts: &GainLossIntent) -> IntentAnalysis {
        let has_intent = facts.intent_gain_self
            || facts.intent_gain_other
            || facts.intent_cause_loss
            || facts.intent_expose_to_loss;

        let intent_type = if facts.intent_gain_self {
            "Intent to make gain for self"
        } else if facts.intent_gain_other {
            "Intent to make gain for another"
        } else if facts.intent_cause_loss {
            "Intent to cause loss to another"
        } else if facts.intent_expose_to_loss {
            "Intent to expose another to risk of loss"
        } else {
            "No relevant intent"
        };

        IntentAnalysis {
            established: has_intent,
            intent_type: intent_type.into(),
            reasoning: if has_intent {
                format!("{} - gain/loss in money or property (s.5)", intent_type)
            } else {
                "No intent to make gain or cause loss established".into()
            },
        }
    }
}

// ============================================================================
// Fraud by Failing to Disclose (s.3)
// ============================================================================

/// Facts for fraud by failing to disclose analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailingToDiscloseFacts {
    /// The legal duty to disclose
    pub duty: DisclosureDuty,
    /// The information not disclosed
    pub information: UndisclosedInformation,
    /// Dishonesty facts
    pub dishonesty: FraudDishonestyFacts,
    /// Intent to make gain or cause loss
    pub intent: GainLossIntent,
}

/// Disclosure duty details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisclosureDuty {
    /// Was there a legal duty to disclose?
    pub duty_exists: bool,
    /// Source of duty
    pub duty_source: DutySource,
    /// Description of duty
    pub duty_description: String,
}

/// Sources of disclosure duty
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DutySource {
    /// Statutory duty
    Statute { act: String },
    /// Contractual duty (including uberrimae fidei)
    Contract { contract_type: String },
    /// Fiduciary duty
    Fiduciary { relationship: String },
    /// Professional duty
    Professional { profession: String },
    /// Custom or trade usage
    Custom,
}

/// Undisclosed information details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UndisclosedInformation {
    /// What information was not disclosed?
    pub information: String,
    /// Was failure to disclose deliberate?
    pub deliberate: bool,
}

/// Result of failing to disclose fraud analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailingToDiscloseResult {
    /// Offence established?
    pub established: bool,
    /// Duty analysis
    pub duty_analysis: DutyAnalysis,
    /// Failure analysis
    pub failure_analysis: FailureAnalysis,
    /// Dishonesty analysis
    pub dishonesty_analysis: DishonestyAnalysis,
    /// Intent analysis
    pub intent_analysis: IntentAnalysis,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Duty analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DutyAnalysis {
    /// Duty established?
    pub established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Failure analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailureAnalysis {
    /// Failure established?
    pub established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Analyzer for failing to disclose fraud
pub struct FailingToDiscloseAnalyzer;

impl FailingToDiscloseAnalyzer {
    /// Analyze fraud by failing to disclose
    pub fn analyze(facts: &FailingToDiscloseFacts) -> CriminalResult<FailingToDiscloseResult> {
        let case_law = vec![CaseCitation::new(
            "R v Razaq",
            2012,
            "EWCA Crim 674",
            "s.3 fraud can apply to solicitor failing to disclose conflict",
        )];

        // Analyze duty
        let duty_analysis = DutyAnalysis {
            established: facts.duty.duty_exists,
            reasoning: if facts.duty.duty_exists {
                format!(
                    "Legal duty to disclose from {:?}: {}",
                    facts.duty.duty_source, facts.duty.duty_description
                )
            } else {
                "No legal duty to disclose established".into()
            },
        };

        // Analyze failure
        let failure_analysis = FailureAnalysis {
            established: facts.information.deliberate,
            reasoning: format!(
                "Failed to disclose: {} ({})",
                facts.information.information,
                if facts.information.deliberate {
                    "deliberate"
                } else {
                    "not deliberate"
                }
            ),
        };

        // Dishonesty
        let dishonesty_analysis = DishonestyAnalysis {
            defendants_knowledge: facts.dishonesty.actual_knowledge_belief.clone(),
            dishonest_by_ordinary_standards: facts.dishonesty.dishonest_by_ordinary_standards,
            reasoning: "Ivey test applied to non-disclosure".into(),
        };

        // Intent
        let intent_analysis = FalseRepresentationAnalyzer::analyze_intent(&facts.intent);

        let established = duty_analysis.established
            && failure_analysis.established
            && dishonesty_analysis.dishonest_by_ordinary_standards
            && intent_analysis.established;

        Ok(FailingToDiscloseResult {
            established,
            duty_analysis,
            failure_analysis,
            dishonesty_analysis,
            intent_analysis,
            case_law,
        })
    }
}

// ============================================================================
// Fraud by Abuse of Position (s.4)
// ============================================================================

/// Facts for fraud by abuse of position analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbuseOfPositionFacts {
    /// The position occupied
    pub position: PositionDetails,
    /// The abuse committed
    pub abuse: AbuseDetails,
    /// Dishonesty facts
    pub dishonesty: FraudDishonestyFacts,
    /// Intent to make gain or cause loss
    pub intent: GainLossIntent,
}

/// Position details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionDetails {
    /// Description of position
    pub description: String,
    /// Type of position
    pub position_type: PositionType,
    /// Expected to safeguard financial interests?
    pub safeguard_expected: bool,
    /// Whose interests?
    pub whose_interests: String,
}

/// Types of position for s.4
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionType {
    /// Trustee
    Trustee,
    /// Director
    Director,
    /// Employee
    Employee,
    /// Agent
    Agent,
    /// Family member
    FamilyMember,
    /// Carer
    Carer,
    /// Professional advisor
    Professional { profession: String },
    /// Other fiduciary
    OtherFiduciary { description: String },
}

/// Abuse details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbuseDetails {
    /// Description of abuse
    pub description: String,
    /// Type of abuse (act or omission)
    pub abuse_type: AbuseType,
}

/// Type of abuse
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbuseType {
    /// Positive act
    Act,
    /// Omission
    Omission,
}

/// Result of abuse of position fraud analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbuseOfPositionResult {
    /// Offence established?
    pub established: bool,
    /// Position analysis
    pub position_analysis: PositionAnalysis,
    /// Abuse analysis
    pub abuse_analysis: AbuseAnalysisResult,
    /// Dishonesty analysis
    pub dishonesty_analysis: DishonestyAnalysis,
    /// Intent analysis
    pub intent_analysis: IntentAnalysis,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Position analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionAnalysis {
    /// Position established?
    pub established: bool,
    /// Expected to safeguard?
    pub safeguard: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Abuse analysis result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbuseAnalysisResult {
    /// Abuse established?
    pub established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Analyzer for abuse of position fraud
pub struct AbuseOfPositionAnalyzer;

impl AbuseOfPositionAnalyzer {
    /// Analyze fraud by abuse of position
    pub fn analyze(facts: &AbuseOfPositionFacts) -> CriminalResult<AbuseOfPositionResult> {
        let case_law = vec![CaseCitation::new(
            "R v Valujevs",
            2014,
            "EWCA Crim 2888",
            "Position of trust can arise in informal relationships",
        )];

        // Analyze position
        let position_analysis = PositionAnalysis {
            established: true, // Position exists
            safeguard: facts.position.safeguard_expected,
            reasoning: format!(
                "Position: {} ({:?}) - expected to safeguard {} interests: {}",
                facts.position.description,
                facts.position.position_type,
                facts.position.whose_interests,
                if facts.position.safeguard_expected {
                    "yes"
                } else {
                    "no"
                }
            ),
        };

        // Analyze abuse
        let abuse_analysis = AbuseAnalysisResult {
            established: true, // Assume abuse if facts provided
            reasoning: format!(
                "Abuse by {:?}: {}",
                facts.abuse.abuse_type, facts.abuse.description
            ),
        };

        // Dishonesty
        let dishonesty_analysis = DishonestyAnalysis {
            defendants_knowledge: facts.dishonesty.actual_knowledge_belief.clone(),
            dishonest_by_ordinary_standards: facts.dishonesty.dishonest_by_ordinary_standards,
            reasoning: "Ivey test applied to abuse of position".into(),
        };

        // Intent
        let intent_analysis = FalseRepresentationAnalyzer::analyze_intent(&facts.intent);

        let established = position_analysis.established
            && position_analysis.safeguard
            && abuse_analysis.established
            && dishonesty_analysis.dishonest_by_ordinary_standards
            && intent_analysis.established;

        Ok(AbuseOfPositionResult {
            established,
            position_analysis,
            abuse_analysis,
            dishonesty_analysis,
            intent_analysis,
            case_law,
        })
    }
}

// ============================================================================
// Obtaining Services Dishonestly (s.11)
// ============================================================================

/// Facts for obtaining services dishonestly analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObtainingServicesFacts {
    /// Service obtained
    pub service: ServiceDetails,
    /// Dishonesty facts
    pub dishonesty: FraudDishonestyFacts,
    /// Knowledge that service requires payment
    pub knew_payment_required: bool,
    /// Intent regarding payment
    pub payment_intent: PaymentIntent,
}

/// Service details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceDetails {
    /// Description of service
    pub description: String,
    /// Was service obtained?
    pub obtained: bool,
    /// Was service available on basis of payment?
    pub payment_required: bool,
    /// Value of service
    pub value: Option<f64>,
}

/// Payment intent
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentIntent {
    /// Intended not to pay
    IntendedNotToPay,
    /// Intended not to pay full amount
    IntendedPartialPayment,
    /// Intended to pay later (but knew wouldn't be provided without payment)
    IntendedPayLater,
}

/// Result of obtaining services analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObtainingServicesResult {
    /// Offence established?
    pub established: bool,
    /// Service analysis
    pub service_analysis: String,
    /// Dishonesty analysis
    pub dishonesty_analysis: DishonestyAnalysis,
    /// Payment analysis
    pub payment_analysis: String,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Analyzer for obtaining services dishonestly
pub struct ObtainingServicesAnalyzer;

impl ObtainingServicesAnalyzer {
    /// Analyze obtaining services dishonestly
    pub fn analyze(facts: &ObtainingServicesFacts) -> CriminalResult<ObtainingServicesResult> {
        let case_law = vec![CaseCitation::new(
            "R v Sofroniou",
            2003,
            "EWCA Crim 3681",
            "Under predecessor offence - service must actually be obtained",
        )];

        let mut established = true;

        // Service analysis
        let service_analysis = if facts.service.obtained && facts.service.payment_required {
            format!(
                "Service ({}) obtained; payment of {} was required",
                facts.service.description,
                facts
                    .service
                    .value
                    .map(|v| format!("£{:.2}", v))
                    .unwrap_or_else(|| "unknown amount".into())
            )
        } else if !facts.service.obtained {
            established = false;
            "Service not obtained - s.11 requires actual obtaining".to_string()
        } else {
            established = false;
            "Service not provided on basis of payment".to_string()
        };

        // Dishonesty
        let dishonesty_analysis = DishonestyAnalysis {
            defendants_knowledge: facts.dishonesty.actual_knowledge_belief.clone(),
            dishonest_by_ordinary_standards: facts.dishonesty.dishonest_by_ordinary_standards,
            reasoning: "Ivey test applied to obtaining services".into(),
        };

        if !dishonesty_analysis.dishonest_by_ordinary_standards {
            established = false;
        }

        // Payment analysis
        let payment_analysis = if !facts.knew_payment_required {
            established = false;
            "D did not know service required payment".to_string()
        } else {
            match facts.payment_intent {
                PaymentIntent::IntendedNotToPay => {
                    "D knew payment required and intended not to pay".to_string()
                }
                PaymentIntent::IntendedPartialPayment => {
                    "D knew payment required and intended only partial payment".to_string()
                }
                PaymentIntent::IntendedPayLater => {
                    "D intended to pay later but knew service wouldn't be provided without payment"
                        .to_string()
                }
            }
        };

        Ok(ObtainingServicesResult {
            established,
            service_analysis,
            dishonesty_analysis,
            payment_analysis,
            case_law,
        })
    }
}

// ============================================================================
// Offence Definitions
// ============================================================================

/// Get fraud by false representation offence definition
pub fn false_representation_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Fraud by False Representation")
        .statutory_source("Fraud Act 2006", "s.2")
        .classification(OffenceClassification::EitherWay)
        .category(OffenceCategory::FraudDishonesty)
        .severity(OffenceSeverity::Serious)
        .maximum_sentence(MaximumSentence::Years(10))
        .mens_rea(MensReaType::Dishonesty)
        .mens_rea(MensReaType::Knowledge)
        .mens_rea(MensReaType::DirectIntention)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get fraud by failing to disclose offence definition
pub fn failing_to_disclose_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Fraud by Failing to Disclose Information")
        .statutory_source("Fraud Act 2006", "s.3")
        .classification(OffenceClassification::EitherWay)
        .category(OffenceCategory::FraudDishonesty)
        .severity(OffenceSeverity::Serious)
        .maximum_sentence(MaximumSentence::Years(10))
        .mens_rea(MensReaType::Dishonesty)
        .mens_rea(MensReaType::DirectIntention)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get fraud by abuse of position offence definition
pub fn abuse_of_position_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Fraud by Abuse of Position")
        .statutory_source("Fraud Act 2006", "s.4")
        .classification(OffenceClassification::EitherWay)
        .category(OffenceCategory::FraudDishonesty)
        .severity(OffenceSeverity::Serious)
        .maximum_sentence(MaximumSentence::Years(10))
        .mens_rea(MensReaType::Dishonesty)
        .mens_rea(MensReaType::DirectIntention)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get obtaining services dishonestly offence definition
pub fn obtaining_services_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Obtaining Services Dishonestly")
        .statutory_source("Fraud Act 2006", "s.11")
        .classification(OffenceClassification::EitherWay)
        .category(OffenceCategory::FraudDishonesty)
        .severity(OffenceSeverity::Moderate)
        .maximum_sentence(MaximumSentence::Years(5))
        .mens_rea(MensReaType::Dishonesty)
        .mens_rea(MensReaType::DirectIntention)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .build()
        .map_err(CriminalError::InvalidInput)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_false_representation_fraud() {
        let facts = FalseRepresentationFacts {
            representation: RepresentationDetails {
                description: "Claimed goods were genuine designer items".into(),
                representation_type: RepresentationType::Fact,
                false_representation: true,
                knew_untrue_misleading: true,
            },
            dishonesty: FraudDishonestyFacts {
                actual_knowledge_belief: "Knew goods were counterfeit".into(),
                dishonest_by_ordinary_standards: true,
            },
            intent: GainLossIntent {
                intent_gain_self: true,
                intent_gain_other: false,
                intent_cause_loss: false,
                intent_expose_to_loss: false,
                evidence: vec!["Sold items for profit".into()],
            },
        };

        let result = FalseRepresentationAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_representation_to_machine() {
        let facts = FalseRepresentationFacts {
            representation: RepresentationDetails {
                description: "Used stolen card at ATM".into(),
                representation_type: RepresentationType::ToMachine,
                false_representation: true,
                knew_untrue_misleading: true,
            },
            dishonesty: FraudDishonestyFacts {
                actual_knowledge_belief: "Knew card was stolen".into(),
                dishonest_by_ordinary_standards: true,
            },
            intent: GainLossIntent {
                intent_gain_self: true,
                intent_gain_other: false,
                intent_cause_loss: false,
                intent_expose_to_loss: false,
                evidence: vec!["Withdrew cash".into()],
            },
        };

        let result = FalseRepresentationAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_failing_to_disclose() {
        let facts = FailingToDiscloseFacts {
            duty: DisclosureDuty {
                duty_exists: true,
                duty_source: DutySource::Contract {
                    contract_type: "Insurance uberrimae fidei".into(),
                },
                duty_description: "Duty to disclose material facts to insurer".into(),
            },
            information: UndisclosedInformation {
                information: "Previous insurance claims".into(),
                deliberate: true,
            },
            dishonesty: FraudDishonestyFacts {
                actual_knowledge_belief: "Knew claims history was material".into(),
                dishonest_by_ordinary_standards: true,
            },
            intent: GainLossIntent {
                intent_gain_self: true,
                intent_gain_other: false,
                intent_cause_loss: false,
                intent_expose_to_loss: false,
                evidence: vec!["Obtained lower premium".into()],
            },
        };

        let result = FailingToDiscloseAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_abuse_of_position() {
        let facts = AbuseOfPositionFacts {
            position: PositionDetails {
                description: "Finance director".into(),
                position_type: PositionType::Director,
                safeguard_expected: true,
                whose_interests: "company".into(),
            },
            abuse: AbuseDetails {
                description: "Transferred company funds to personal account".into(),
                abuse_type: AbuseType::Act,
            },
            dishonesty: FraudDishonestyFacts {
                actual_knowledge_belief: "Knew funds belonged to company".into(),
                dishonest_by_ordinary_standards: true,
            },
            intent: GainLossIntent {
                intent_gain_self: true,
                intent_gain_other: false,
                intent_cause_loss: true,
                intent_expose_to_loss: false,
                evidence: vec!["Personal bank transfers".into()],
            },
        };

        let result = AbuseOfPositionAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_obtaining_services() {
        let facts = ObtainingServicesFacts {
            service: ServiceDetails {
                description: "Hotel accommodation".into(),
                obtained: true,
                payment_required: true,
                value: Some(500.0),
            },
            dishonesty: FraudDishonestyFacts {
                actual_knowledge_belief: "Knew had no means to pay".into(),
                dishonest_by_ordinary_standards: true,
            },
            knew_payment_required: true,
            payment_intent: PaymentIntent::IntendedNotToPay,
        };

        let result = ObtainingServicesAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_no_dishonesty() {
        let facts = FalseRepresentationFacts {
            representation: RepresentationDetails {
                description: "Stated item was valuable antique".into(),
                representation_type: RepresentationType::Fact,
                false_representation: true,
                knew_untrue_misleading: false, // Honestly believed it was true
            },
            dishonesty: FraudDishonestyFacts {
                actual_knowledge_belief: "Genuinely believed item was antique".into(),
                dishonest_by_ordinary_standards: false,
            },
            intent: GainLossIntent {
                intent_gain_self: true,
                intent_gain_other: false,
                intent_cause_loss: false,
                intent_expose_to_loss: false,
                evidence: vec![],
            },
        };

        let result = FalseRepresentationAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // Not dishonest, so fraud not established
        assert!(!analysis.established);
    }

    #[test]
    fn test_offence_definitions() {
        assert!(false_representation_offence().is_ok());
        assert!(failing_to_disclose_offence().is_ok());
        assert!(abuse_of_position_offence().is_ok());
        assert!(obtaining_services_offence().is_ok());
    }
}
