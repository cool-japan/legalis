//! Constitution of India Validation
//!
//! Validation logic for constitutional compliance.

use super::error::{ConstitutionalComplianceReport, ConstitutionalError, ConstitutionalResult};
use super::types::*;

/// Validate fundamental right violation
pub fn validate_fundamental_right_claim(
    right: FundamentalRight,
    state_action: bool,
    emergency_active: bool,
) -> ConstitutionalResult<()> {
    // Fundamental rights only against state action (Article 12)
    if !state_action {
        return Err(ConstitutionalError::ValidationError {
            message: "Fundamental rights enforceable only against State (Article 12)".to_string(),
        });
    }

    // Check if right is suspended during emergency
    if emergency_active && right.suspendable_during_emergency() {
        return Err(ConstitutionalError::ValidationError {
            message: format!(
                "Article {} rights suspended during emergency under Article 359",
                right.article()
            ),
        });
    }

    Ok(())
}

/// Validate Article 19 restriction
pub fn validate_article19_restriction(
    freedom: Article19Freedom,
    restriction_ground: &str,
    reasonable: bool,
) -> ConstitutionalResult<()> {
    let valid_restrictions = freedom.restrictions();

    // Check if restriction ground is valid
    if !valid_restrictions
        .iter()
        .any(|r| restriction_ground.contains(r))
    {
        return Err(ConstitutionalError::FreedomViolation {
            freedom: format!("{:?}", freedom),
            restriction_invalid: true,
        });
    }

    // Restriction must be reasonable
    if !reasonable {
        return Err(ConstitutionalError::FreedomViolation {
            freedom: format!("{:?}", freedom),
            restriction_invalid: true,
        });
    }

    Ok(())
}

/// Validate writ petition maintainability
pub fn validate_writ_petition(petition: &WritPetition) -> ConstitutionalResult<()> {
    // Article 32 only for fundamental rights
    if petition.article == 32 && petition.right_violated.is_none() {
        return Err(ConstitutionalError::WritRejected {
            article: 32,
            reason: "Article 32 available only for enforcement of fundamental rights".to_string(),
        });
    }

    // Validate writ type against respondent
    match petition.writ_type {
        WritType::HabeasCorpus => {
            // Can be against anyone
            Ok(())
        }
        WritType::Mandamus => {
            // Cannot be against private person (except public duty)
            if petition.respondent.to_lowercase().contains("private") {
                return Err(ConstitutionalError::WritRejected {
                    article: petition.article,
                    reason: "Mandamus not available against purely private person".to_string(),
                });
            }
            Ok(())
        }
        WritType::Prohibition | WritType::Certiorari => {
            // Only against judicial/quasi-judicial bodies
            Ok(())
        }
        WritType::QuoWarranto => {
            // Only against holder of public office
            Ok(())
        }
    }
}

/// Validate emergency proclamation
pub fn validate_emergency_proclamation(
    emergency_type: EmergencyType,
    cabinet_approval: bool,
    parliament_approval: bool,
    days_since_proclamation: u32,
) -> ConstitutionalResult<()> {
    // 44th Amendment: Cabinet advice in writing required for Article 352
    if matches!(emergency_type, EmergencyType::NationalEmergency) && !cabinet_approval {
        return Err(ConstitutionalError::InvalidEmergency {
            article: emergency_type.article(),
            emergency_type,
            reason: "Cabinet approval in writing required (44th Amendment)".to_string(),
        });
    }

    // Parliament must approve within 1 month
    if days_since_proclamation > 30 && !parliament_approval {
        return Err(ConstitutionalError::InvalidEmergency {
            article: emergency_type.article(),
            emergency_type,
            reason: "Parliament approval required within 1 month".to_string(),
        });
    }

    Ok(())
}

/// House voting data for constitutional amendments
#[derive(Debug, Clone)]
pub struct HouseVotes {
    /// Votes in favor
    pub votes_for: u32,
    /// Members present and voting
    pub present: u32,
    /// Total membership of house
    pub total: u32,
}

/// Amendment vote check parameters
#[derive(Debug, Clone)]
pub struct AmendmentVoteCheck {
    /// Lok Sabha (House of the People) votes
    pub lok_sabha: HouseVotes,
    /// Rajya Sabha (Council of States) votes
    pub rajya_sabha: HouseVotes,
    /// Amendment procedure type
    pub procedure_type: AmendmentProcedure,
    /// Number of states that have ratified (if required)
    pub states_ratified: Option<u32>,
}

/// Validate constitutional amendment procedure
pub fn validate_amendment_procedure(check: &AmendmentVoteCheck) -> ConstitutionalResult<()> {
    // Check special majority in both houses
    // Requires: majority of total membership AND 2/3 of present and voting

    // Lok Sabha
    if check.lok_sabha.votes_for <= check.lok_sabha.total / 2 {
        return Err(ConstitutionalError::SpecialMajorityFailed {
            required: check.lok_sabha.total / 2 + 1,
            obtained: check.lok_sabha.votes_for,
        });
    }
    if check.lok_sabha.votes_for * 3 < check.lok_sabha.present * 2 {
        return Err(ConstitutionalError::SpecialMajorityFailed {
            required: (check.lok_sabha.present * 2).div_ceil(3),
            obtained: check.lok_sabha.votes_for,
        });
    }

    // Rajya Sabha
    if check.rajya_sabha.votes_for <= check.rajya_sabha.total / 2 {
        return Err(ConstitutionalError::SpecialMajorityFailed {
            required: check.rajya_sabha.total / 2 + 1,
            obtained: check.rajya_sabha.votes_for,
        });
    }
    if check.rajya_sabha.votes_for * 3 < check.rajya_sabha.present * 2 {
        return Err(ConstitutionalError::SpecialMajorityFailed {
            required: (check.rajya_sabha.present * 2).div_ceil(3),
            obtained: check.rajya_sabha.votes_for,
        });
    }

    // Check state ratification if required
    if matches!(
        check.procedure_type,
        AmendmentProcedure::SpecialMajorityWithRatification
    ) {
        let total_states = 28u32; // Current number of states
        let required_states = total_states.div_ceil(2); // Not less than half

        match check.states_ratified {
            None => {
                return Err(ConstitutionalError::RatificationNotObtained {
                    states_required: required_states,
                    states_obtained: 0,
                });
            }
            Some(ratified) if ratified < required_states => {
                return Err(ConstitutionalError::RatificationNotObtained {
                    states_required: required_states,
                    states_obtained: ratified,
                });
            }
            _ => {}
        }
    }

    Ok(())
}

/// Check if amendment violates basic structure
pub fn check_basic_structure_violation(amendment_subject: &str) -> Option<ConstitutionalError> {
    let basic_features = BasicStructure::recognized_features();

    for feature in basic_features {
        // Simple check - in production would need more sophisticated analysis
        if amendment_subject
            .to_lowercase()
            .contains(&feature.feature.to_lowercase())
        {
            return Some(ConstitutionalError::BasicStructureViolation {
                amendment_number: 0, // Would be provided
                feature_violated: feature.feature,
            });
        }
    }

    None
}

/// Validate legislative competence
pub fn validate_legislative_competence(
    legislature: Legislature,
    subject: &str,
    schedule_7_list: Schedule7List,
) -> ConstitutionalResult<()> {
    match (legislature, schedule_7_list) {
        // Parliament can legislate on Union List and Concurrent List
        (Legislature::Parliament, Schedule7List::Union) => Ok(()),
        (Legislature::Parliament, Schedule7List::Concurrent) => Ok(()),

        // State can legislate on State List and Concurrent List
        (Legislature::StateAssembly, Schedule7List::State) => Ok(()),
        (Legislature::StateAssembly, Schedule7List::Concurrent) => Ok(()),

        // Parliament cannot legislate on State List (normally)
        (Legislature::Parliament, Schedule7List::State) => {
            Err(ConstitutionalError::LegislativeIncompetence {
                list: "State List".to_string(),
                subject: subject.to_string(),
            })
        }

        // State cannot legislate on Union List
        (Legislature::StateAssembly, Schedule7List::Union) => {
            Err(ConstitutionalError::LegislativeIncompetence {
                list: "Union List".to_string(),
                subject: subject.to_string(),
            })
        }
    }
}

/// Legislature type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Legislature {
    /// Union Parliament
    Parliament,
    /// State Legislative Assembly
    StateAssembly,
}

/// Seventh Schedule Lists
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Schedule7List {
    /// Union List (List I)
    Union,
    /// State List (List II)
    State,
    /// Concurrent List (List III)
    Concurrent,
}

/// Validate due process for Article 21
pub fn validate_due_process(
    procedure_established_by_law: bool,
    procedure_fair: bool,
    procedure_non_arbitrary: bool,
) -> ConstitutionalResult<()> {
    // Post Maneka Gandhi: procedure must be fair, just and reasonable

    if !procedure_established_by_law {
        return Err(ConstitutionalError::DueProcessViolation {
            stage: "No procedure established by law".to_string(),
        });
    }

    if !procedure_fair {
        return Err(ConstitutionalError::DueProcessViolation {
            stage: "Procedure not fair and just".to_string(),
        });
    }

    if !procedure_non_arbitrary {
        return Err(ConstitutionalError::ArbitraryAction {
            action: "Arbitrary procedure".to_string(),
        });
    }

    Ok(())
}

/// Validate equality under Article 14
pub fn validate_equality(
    classification_made: bool,
    intelligible_differentia: bool,
    rational_nexus: bool,
) -> ConstitutionalResult<()> {
    if !classification_made {
        return Ok(()); // No classification, no issue
    }

    // Classification must have:
    // 1. Intelligible differentia - distinguishes grouped persons from others
    // 2. Rational nexus - to the object of the legislation

    if !intelligible_differentia {
        return Err(ConstitutionalError::EqualityViolation {
            description: "Classification lacks intelligible differentia".to_string(),
        });
    }

    if !rational_nexus {
        return Err(ConstitutionalError::EqualityViolation {
            description: "Classification lacks rational nexus to object".to_string(),
        });
    }

    Ok(())
}

/// Validate PIL maintainability
pub fn validate_pil_maintainability(pil: &PublicInterestLitigation) -> ConstitutionalResult<()> {
    // PIL cannot be for personal interest
    // Must be bona fide and for public good

    match pil.petitioner_type {
        PilPetitionerType::Individual => {
            // Individual must not have personal interest
            // (This would need more context to validate properly)
            Ok(())
        }
        PilPetitionerType::SuoMotu => {
            // Court's own motion - always maintainable
            Ok(())
        }
        PilPetitionerType::Ngo
        | PilPetitionerType::Journalist
        | PilPetitionerType::SocialActivist
        | PilPetitionerType::Lawyer => {
            // Generally maintainable if bona fide
            Ok(())
        }
    }
}

/// Get appropriate writ for situation
pub fn get_appropriate_writ(situation: &str) -> WritType {
    let situation_lower = situation.to_lowercase();

    if situation_lower.contains("detention")
        || situation_lower.contains("arrest")
        || situation_lower.contains("custody")
    {
        WritType::HabeasCorpus
    } else if situation_lower.contains("duty")
        || situation_lower.contains("perform")
        || situation_lower.contains("refusal")
    {
        WritType::Mandamus
    } else if situation_lower.contains("exceed")
        || situation_lower.contains("jurisdiction")
        || situation_lower.contains("stop")
    {
        WritType::Prohibition
    } else if situation_lower.contains("quash")
        || situation_lower.contains("error")
        || situation_lower.contains("void")
    {
        WritType::Certiorari
    } else if situation_lower.contains("office")
        || situation_lower.contains("authority")
        || situation_lower.contains("usurp")
    {
        WritType::QuoWarranto
    } else {
        // Default to certiorari for review
        WritType::Certiorari
    }
}

/// Comprehensive constitutional compliance check
pub fn validate_constitutional_compliance(
    state_action: bool,
    emergency_active: bool,
    fundamental_right_affected: Option<FundamentalRight>,
    procedure_fair: bool,
    classification_reasonable: bool,
) -> ConstitutionalComplianceReport {
    let mut report = ConstitutionalComplianceReport {
        compliant: true,
        fundamental_rights_compliant: true,
        procedure_compliant: true,
        federal_compliant: true,
        violations: Vec::new(),
        warnings: Vec::new(),
        recommendations: Vec::new(),
    };

    // Check fundamental rights
    if let Some(right) = fundamental_right_affected
        && state_action
        && let Err(e) = validate_fundamental_right_claim(right, state_action, emergency_active)
    {
        report.fundamental_rights_compliant = false;
        report.compliant = false;
        report.violations.push(e);
    }

    // Check due process (Article 21)
    if !procedure_fair {
        report.procedure_compliant = false;
        report.compliant = false;
        report
            .violations
            .push(ConstitutionalError::DueProcessViolation {
                stage: "Procedure not fair, just and reasonable".to_string(),
            });
    }

    // Check equality (Article 14)
    if !classification_reasonable {
        report.fundamental_rights_compliant = false;
        report.compliant = false;
        report
            .violations
            .push(ConstitutionalError::EqualityViolation {
                description: "Classification is unreasonable".to_string(),
            });
    }

    // Add warnings and recommendations
    if emergency_active {
        report
            .warnings
            .push("Emergency in force - some rights may be suspended".to_string());
    }

    if state_action && fundamental_right_affected.is_some() {
        report
            .recommendations
            .push("Writ petition under Article 32/226 may be filed".to_string());
    }

    report
}

/// Get limitation period for constitutional remedies
pub fn get_constitutional_limitation() -> &'static str {
    "No limitation period for constitutional remedies, but doctrine of laches may apply"
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_fundamental_right_claim() {
        // Valid claim - state action, no emergency
        let result =
            validate_fundamental_right_claim(FundamentalRight::EqualityBeforeLaw, true, false);
        assert!(result.is_ok());

        // Invalid - not state action
        let result =
            validate_fundamental_right_claim(FundamentalRight::EqualityBeforeLaw, false, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_article19_restriction() {
        // Valid restriction
        let result = validate_article19_restriction(
            Article19Freedom::SpeechExpression,
            "Public order",
            true,
        );
        assert!(result.is_ok());

        // Invalid ground
        let result = validate_article19_restriction(
            Article19Freedom::SpeechExpression,
            "Economic reasons",
            true,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_writ_petition_validation() {
        let petition = WritPetition {
            writ_type: WritType::HabeasCorpus,
            article: 32,
            court: ConstitutionalCourt::SupremeCourt,
            right_violated: Some(FundamentalRight::RightToLife),
            petitioner: "Petitioner".to_string(),
            respondent: "State".to_string(),
            prayer: "Release from illegal detention".to_string(),
            filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            case_number: None,
        };

        let result = validate_writ_petition(&petition);
        assert!(result.is_ok());
    }

    #[test]
    fn test_emergency_validation() {
        // Valid emergency
        let result = validate_emergency_proclamation(
            EmergencyType::NationalEmergency,
            true, // Cabinet approval
            true, // Parliament approval
            15,   // Days since proclamation
        );
        assert!(result.is_ok());

        // Invalid - no cabinet approval
        let result =
            validate_emergency_proclamation(EmergencyType::NationalEmergency, false, true, 15);
        assert!(result.is_err());
    }

    #[test]
    fn test_amendment_procedure() {
        // Valid special majority
        let check = AmendmentVoteCheck {
            lok_sabha: HouseVotes {
                votes_for: 350,
                present: 500,
                total: 545,
            },
            rajya_sabha: HouseVotes {
                votes_for: 150,
                present: 200,
                total: 245,
            },
            procedure_type: AmendmentProcedure::SpecialMajority,
            states_ratified: None,
        };
        let result = validate_amendment_procedure(&check);
        assert!(result.is_ok());

        // Need state ratification - 28 states / 2 = 14 required
        let check = AmendmentVoteCheck {
            lok_sabha: HouseVotes {
                votes_for: 350,
                present: 500,
                total: 545,
            },
            rajya_sabha: HouseVotes {
                votes_for: 150,
                present: 200,
                total: 245,
            },
            procedure_type: AmendmentProcedure::SpecialMajorityWithRatification,
            states_ratified: Some(10), // Not enough states (need 14)
        };
        let result = validate_amendment_procedure(&check);
        assert!(result.is_err());
    }

    #[test]
    fn test_legislative_competence() {
        // Parliament on Union List - valid
        let result = validate_legislative_competence(
            Legislature::Parliament,
            "Defence",
            Schedule7List::Union,
        );
        assert!(result.is_ok());

        // Parliament on State List - invalid
        let result = validate_legislative_competence(
            Legislature::Parliament,
            "Police",
            Schedule7List::State,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_due_process() {
        // Valid due process
        let result = validate_due_process(true, true, true);
        assert!(result.is_ok());

        // Unfair procedure
        let result = validate_due_process(true, false, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_equality_validation() {
        // Reasonable classification
        let result = validate_equality(true, true, true);
        assert!(result.is_ok());

        // Lacks rational nexus
        let result = validate_equality(true, true, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_appropriate_writ() {
        assert_eq!(
            get_appropriate_writ("illegal detention"),
            WritType::HabeasCorpus
        );
        assert_eq!(
            get_appropriate_writ("refusal to perform duty"),
            WritType::Mandamus
        );
        assert_eq!(
            get_appropriate_writ("exceed jurisdiction"),
            WritType::Prohibition
        );
    }

    #[test]
    fn test_comprehensive_compliance() {
        let report = validate_constitutional_compliance(
            true,  // State action
            false, // No emergency
            Some(FundamentalRight::EqualityBeforeLaw),
            true, // Procedure fair
            true, // Classification reasonable
        );

        assert!(report.compliant);
    }
}
