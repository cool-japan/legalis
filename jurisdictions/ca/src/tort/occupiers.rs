//! Canada Tort Law - Occupiers' Liability
//!
//! Analyzers for occupiers' liability claims under provincial statutes and common law.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{CommonLawEntrantStatus, HazardType, OlaDuty, OlaStatute};
use crate::common::Province;

// ============================================================================
// Occupiers' Liability Analysis
// ============================================================================

/// Facts for occupiers' liability analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccupiersLiabilityFacts {
    /// Province where incident occurred
    pub province: Province,
    /// Description of premises
    pub premises: String,
    /// Defendant's relationship to premises
    pub defendant_relationship: OccupierStatus,
    /// Claimant's status
    pub entrant_status: EntrantStatus,
    /// Hazard involved
    pub hazard: HazardDescription,
    /// Whether hazard was known to occupier
    pub hazard_known: bool,
    /// Whether warning given
    pub warning_given: bool,
    /// Whether reasonable care taken
    pub reasonable_care_taken: bool,
    /// Activity or condition
    pub is_activity: bool,
}

/// Status of defendant as occupier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OccupierStatus {
    /// Owner in possession
    OwnerInPossession,
    /// Tenant
    Tenant,
    /// Landlord (retained control)
    LandlordRetainedControl,
    /// Contractor
    Contractor,
    /// Multiple occupiers
    MultipleOccupiers,
}

/// Status of entrant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntrantStatus {
    /// Purpose of entry
    pub purpose: EntryPurpose,
    /// Whether permission given
    pub permission: bool,
    /// Whether trespasser
    pub trespasser: bool,
    /// Whether child
    pub is_child: bool,
    /// Age (if child)
    pub age: Option<u32>,
}

/// Purpose of entry
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryPurpose {
    /// Business purpose (customer, client)
    Business,
    /// Social guest
    Social,
    /// Lawful right (utility worker, firefighter)
    LawfulRight,
    /// Recreational
    Recreational,
    /// No purpose (trespasser)
    None,
}

/// Description of hazard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HazardDescription {
    /// Type of hazard
    pub hazard_type: HazardType,
    /// Description
    pub description: String,
    /// Whether created by occupier
    pub created_by_occupier: bool,
    /// Duration of hazard
    pub duration: Option<String>,
}

/// Result of occupiers' liability analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccupiersLiabilityResult {
    /// Applicable law (statute or common law)
    pub applicable_law: ApplicableLaw,
    /// Duty owed
    pub duty_owed: OlaDuty,
    /// Common law status (if applicable)
    pub common_law_status: Option<CommonLawEntrantStatus>,
    /// Whether duty breached
    pub duty_breached: bool,
    /// Whether defence available
    pub defences_available: Vec<OlaDefence>,
    /// Reasoning
    pub reasoning: String,
}

/// Applicable law for occupiers' liability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicableLaw {
    /// Provincial statute
    Statute(OlaStatute),
    /// Common law
    CommonLaw,
    /// Civil Code (Quebec)
    CivilCode,
}

/// Defences to occupiers' liability
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OlaDefence {
    /// Voluntary assumption of risk
    VoluntaryAssumption,
    /// Warning sufficient
    WarningSufficient,
    /// Obvious danger
    ObviousDanger,
    /// Reasonable care taken
    ReasonableCare,
    /// Excluded by contract
    ContractualExclusion,
    /// Recreational use immunity
    RecreationalUseImmunity,
    /// Contributory negligence
    ContributoryNegligence { percentage: u8 },
}

// ============================================================================
// Occupiers' Liability Analyzer
// ============================================================================

/// Analyzer for occupiers' liability claims
pub struct OccupiersLiabilityAnalyzer;

impl OccupiersLiabilityAnalyzer {
    /// Analyze occupiers' liability claim
    pub fn analyze(facts: &OccupiersLiabilityFacts) -> OccupiersLiabilityResult {
        // Determine applicable law
        let applicable_law = Self::determine_applicable_law(&facts.province);

        // Determine duty and common law status
        let (duty_owed, common_law_status) =
            Self::determine_duty(&applicable_law, &facts.entrant_status);

        // Check if duty breached
        let duty_breached = Self::is_duty_breached(facts, &duty_owed);

        // Check available defences
        let defences_available = Self::check_defences(facts);

        let reasoning = Self::build_reasoning(
            &applicable_law,
            &duty_owed,
            duty_breached,
            &defences_available,
        );

        OccupiersLiabilityResult {
            applicable_law,
            duty_owed,
            common_law_status,
            duty_breached,
            defences_available,
            reasoning,
        }
    }

    /// Determine applicable law for province
    fn determine_applicable_law(province: &Province) -> ApplicableLaw {
        // Quebec uses civil code
        if province.is_civil_law() {
            return ApplicableLaw::CivilCode;
        }

        // Check for provincial OLA statute
        match OlaStatute::for_province(province) {
            Some(statute) => ApplicableLaw::Statute(statute),
            None => ApplicableLaw::CommonLaw,
        }
    }

    /// Determine duty owed based on applicable law and entrant status
    fn determine_duty(
        law: &ApplicableLaw,
        status: &EntrantStatus,
    ) -> (OlaDuty, Option<CommonLawEntrantStatus>) {
        match law {
            ApplicableLaw::Statute(_) => {
                // Statutory regime - common duty to all visitors
                let duty = if status.trespasser {
                    OlaDuty::TrespasserDuty
                } else {
                    OlaDuty::CommonDuty
                };
                (duty, None)
            }
            ApplicableLaw::CommonLaw => {
                // Common law categories
                let common_law_status = Self::determine_common_law_status(status);
                let duty = match &common_law_status {
                    CommonLawEntrantStatus::Invitee => OlaDuty::CommonDuty,
                    CommonLawEntrantStatus::Licensee => OlaDuty::CommonDuty,
                    CommonLawEntrantStatus::Trespasser => OlaDuty::TrespasserDuty,
                    CommonLawEntrantStatus::ChildTrespasser => OlaDuty::TrespasserDuty,
                };
                (duty, Some(common_law_status))
            }
            ApplicableLaw::CivilCode => {
                // Quebec civil code - general duty of care
                (OlaDuty::CommonDuty, None)
            }
        }
    }

    /// Determine common law entrant status
    fn determine_common_law_status(status: &EntrantStatus) -> CommonLawEntrantStatus {
        if status.trespasser {
            if status.is_child {
                CommonLawEntrantStatus::ChildTrespasser
            } else {
                CommonLawEntrantStatus::Trespasser
            }
        } else {
            match status.purpose {
                EntryPurpose::Business => CommonLawEntrantStatus::Invitee,
                EntryPurpose::LawfulRight => CommonLawEntrantStatus::Invitee,
                EntryPurpose::Social => CommonLawEntrantStatus::Licensee,
                EntryPurpose::Recreational => CommonLawEntrantStatus::Licensee,
                EntryPurpose::None => CommonLawEntrantStatus::Trespasser,
            }
        }
    }

    /// Check if duty breached
    fn is_duty_breached(facts: &OccupiersLiabilityFacts, duty: &OlaDuty) -> bool {
        match duty {
            OlaDuty::CommonDuty => {
                // Duty to take reasonable care
                !facts.reasonable_care_taken
                    && (facts.hazard_known || Self::should_have_known(&facts.hazard))
            }
            OlaDuty::TrespasserDuty => {
                // Limited duty - no willful/reckless harm
                facts.hazard.created_by_occupier
                    && matches!(facts.hazard.hazard_type, HazardType::ConcealedDanger)
            }
            OlaDuty::ActivityDuty => {
                // Higher standard for activities
                !facts.reasonable_care_taken
            }
            OlaDuty::ContractorDuty => {
                // Must ensure premises safe for contractor
                !facts.reasonable_care_taken && !facts.warning_given
            }
        }
    }

    /// Check if occupier should have known
    fn should_have_known(hazard: &HazardDescription) -> bool {
        matches!(
            hazard.hazard_type,
            HazardType::UnusualDanger | HazardType::ConcealedDanger
        ) && hazard.duration.is_some()
    }

    /// Check available defences
    fn check_defences(facts: &OccupiersLiabilityFacts) -> Vec<OlaDefence> {
        let mut defences = Vec::new();

        // Obvious danger
        if matches!(facts.hazard.hazard_type, HazardType::ObviousDanger) {
            defences.push(OlaDefence::ObviousDanger);
        }

        // Warning given
        if facts.warning_given && !matches!(facts.hazard.hazard_type, HazardType::ConcealedDanger) {
            defences.push(OlaDefence::WarningSufficient);
        }

        // Reasonable care
        if facts.reasonable_care_taken {
            defences.push(OlaDefence::ReasonableCare);
        }

        // Recreational use (some provinces)
        if matches!(facts.entrant_status.purpose, EntryPurpose::Recreational)
            && !facts.hazard.created_by_occupier
        {
            defences.push(OlaDefence::RecreationalUseImmunity);
        }

        defences
    }

    /// Build reasoning
    fn build_reasoning(
        law: &ApplicableLaw,
        duty: &OlaDuty,
        breached: bool,
        defences: &[OlaDefence],
    ) -> String {
        let law_desc = match law {
            ApplicableLaw::Statute(s) => format!("Under {} (statutory regime)", s.statute),
            ApplicableLaw::CommonLaw => "Under common law".to_string(),
            ApplicableLaw::CivilCode => "Under Quebec Civil Code (art. 1457)".to_string(),
        };

        let duty_desc = match duty {
            OlaDuty::CommonDuty => "common duty of care to take reasonable care",
            OlaDuty::TrespasserDuty => "limited duty to trespassers (no willful harm)",
            OlaDuty::ActivityDuty => "duty regarding activities on premises",
            OlaDuty::ContractorDuty => "duty to independent contractors",
        };

        let breach_desc = if breached {
            "Duty appears to be breached."
        } else {
            "Duty does not appear to be breached."
        };

        let defence_desc = if defences.is_empty() {
            String::new()
        } else {
            format!(" Available defences: {:?}.", defences)
        };

        format!(
            "{}: occupier owes {}. {}{}",
            law_desc, duty_desc, breach_desc, defence_desc
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ontario_statutory_regime() {
        let facts = OccupiersLiabilityFacts {
            province: Province::Ontario,
            premises: "Retail store".to_string(),
            defendant_relationship: OccupierStatus::OwnerInPossession,
            entrant_status: EntrantStatus {
                purpose: EntryPurpose::Business,
                permission: true,
                trespasser: false,
                is_child: false,
                age: None,
            },
            hazard: HazardDescription {
                hazard_type: HazardType::ConcealedDanger,
                description: "Wet floor".to_string(),
                created_by_occupier: true,
                duration: Some("30 minutes".to_string()),
            },
            hazard_known: true,
            warning_given: false,
            reasonable_care_taken: false,
            is_activity: false,
        };

        let result = OccupiersLiabilityAnalyzer::analyze(&facts);
        assert!(matches!(result.applicable_law, ApplicableLaw::Statute(_)));
        assert!(result.duty_breached);
    }

    #[test]
    fn test_quebec_civil_code() {
        let facts = OccupiersLiabilityFacts {
            province: Province::Quebec,
            premises: "Apartment building".to_string(),
            defendant_relationship: OccupierStatus::LandlordRetainedControl,
            entrant_status: EntrantStatus {
                purpose: EntryPurpose::Social,
                permission: true,
                trespasser: false,
                is_child: false,
                age: None,
            },
            hazard: HazardDescription {
                hazard_type: HazardType::KnownDanger,
                description: "Broken step".to_string(),
                created_by_occupier: false,
                duration: Some("1 week".to_string()),
            },
            hazard_known: true,
            warning_given: true,
            reasonable_care_taken: false,
            is_activity: false,
        };

        let result = OccupiersLiabilityAnalyzer::analyze(&facts);
        assert!(matches!(result.applicable_law, ApplicableLaw::CivilCode));
    }

    #[test]
    fn test_obvious_danger_defence() {
        let facts = OccupiersLiabilityFacts {
            province: Province::BritishColumbia,
            premises: "Construction site".to_string(),
            defendant_relationship: OccupierStatus::Contractor,
            entrant_status: EntrantStatus {
                purpose: EntryPurpose::LawfulRight,
                permission: true,
                trespasser: false,
                is_child: false,
                age: None,
            },
            hazard: HazardDescription {
                hazard_type: HazardType::ObviousDanger,
                description: "Open excavation".to_string(),
                created_by_occupier: true,
                duration: None,
            },
            hazard_known: true,
            warning_given: true,
            reasonable_care_taken: true,
            is_activity: true,
        };

        let result = OccupiersLiabilityAnalyzer::analyze(&facts);
        assert!(
            result
                .defences_available
                .contains(&OlaDefence::ObviousDanger)
        );
    }

    #[test]
    fn test_trespasser_limited_duty() {
        let facts = OccupiersLiabilityFacts {
            province: Province::Alberta,
            premises: "Industrial facility".to_string(),
            defendant_relationship: OccupierStatus::OwnerInPossession,
            entrant_status: EntrantStatus {
                purpose: EntryPurpose::None,
                permission: false,
                trespasser: true,
                is_child: false,
                age: None,
            },
            hazard: HazardDescription {
                hazard_type: HazardType::KnownDanger,
                description: "Machinery".to_string(),
                created_by_occupier: false,
                duration: None,
            },
            hazard_known: true,
            warning_given: false,
            reasonable_care_taken: true,
            is_activity: false,
        };

        let result = OccupiersLiabilityAnalyzer::analyze(&facts);
        assert!(matches!(result.duty_owed, OlaDuty::TrespasserDuty));
        assert!(!result.duty_breached);
    }
}
