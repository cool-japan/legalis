//! UK Land Law - Registration Module
//!
//! This module provides analysis of land registration under:
//! - Land Registration Act 2002 (registered land)
//! - Land Charges Act 1972 (unregistered land)
//!
//! Key concepts:
//! - First registration triggers (LRA 2002 s.4)
//! - Priority rules (LRA 2002 ss.28-30)
//! - Overriding interests (LRA 2002 Sch 3)
//! - Alteration and indemnity (LRA 2002 Sch 4, 8)
//!
//! Key cases:
//! - Williams & Glyn's Bank v Boland \[1981\] (actual occupation)
//! - City of London BS v Flegg \[1988\] (overreaching)

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    FirstRegistrationTrigger, LandChargeClass, LandLawCase, OverridingInterest, RegistrationStatus,
    TitleClass,
};

// ============================================================================
// First Registration Analyzer
// ============================================================================

/// Facts for first registration analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstRegistrationFacts {
    /// Current registration status
    pub status: RegistrationStatus,
    /// Triggering event
    pub trigger: Option<FirstRegistrationTrigger>,
    /// Date of triggering event (YYYY-MM-DD)
    pub trigger_date: Option<String>,
    /// Days since trigger
    pub days_since_trigger: u32,
    /// Voluntary application
    pub voluntary: bool,
    /// Property description
    pub property: String,
}

/// Result of first registration analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstRegistrationResult {
    pub registration_required: bool,
    pub registration_compulsory: bool,
    pub deadline_days: u32,
    pub days_remaining: Option<i32>,
    pub consequences_if_missed: Vec<String>,
    pub class_of_title_likely: TitleClass,
    pub recommendations: Vec<String>,
}

/// Analyzes first registration requirements
pub struct FirstRegistrationAnalyzer;

impl FirstRegistrationAnalyzer {
    /// Analyze first registration requirement
    pub fn analyze(facts: &FirstRegistrationFacts) -> FirstRegistrationResult {
        // Check if already registered
        if matches!(facts.status, RegistrationStatus::Registered { .. }) {
            return FirstRegistrationResult {
                registration_required: false,
                registration_compulsory: false,
                deadline_days: 0,
                days_remaining: None,
                consequences_if_missed: vec![],
                class_of_title_likely: TitleClass::Absolute,
                recommendations: vec![],
            };
        }

        let mut recommendations = Vec::new();
        let mut consequences = Vec::new();

        // Compulsory first registration under s.4 LRA 2002
        let (compulsory, deadline_days) = if let Some(trigger) = &facts.trigger {
            match trigger {
                FirstRegistrationTrigger::TransferOfFreehold => {
                    consequences
                        .push("Legal estate reverts to transferor until registration".into());
                    (true, 60) // 2 months
                }
                FirstRegistrationTrigger::GrantOfLeaseOver7Years => {
                    consequences.push("Lease takes effect as equitable lease only".into());
                    (true, 60)
                }
                FirstRegistrationTrigger::AssignmentOfLeaseOver7Years => {
                    consequences.push("Legal estate reverts to assignor".into());
                    (true, 60)
                }
                FirstRegistrationTrigger::FirstLegalMortgage => {
                    consequences.push("Mortgage takes effect as equitable charge only".into());
                    (true, 60)
                }
                FirstRegistrationTrigger::ProtectedMortgageOfLeasehold => (true, 60),
            }
        } else {
            (false, 0)
        };

        // Calculate days remaining
        let days_remaining = if compulsory {
            Some(deadline_days as i32 - facts.days_since_trigger as i32)
        } else {
            None
        };

        // Check if deadline missed
        if let Some(remaining) = days_remaining {
            if remaining < 0 {
                consequences.push(format!(
                    "Deadline missed by {} days - legal consequences apply",
                    -remaining
                ));
                recommendations.push("Apply for late registration immediately".into());
                recommendations
                    .push("Consider applying to Chief Land Registrar for extension".into());
            } else if remaining < 14 {
                recommendations.push("Deadline approaching - expedite application".into());
            }
        }

        // Voluntary registration always available
        if !compulsory {
            recommendations.push("Voluntary first registration recommended for protection".into());
        }

        // Likely class of title
        let class_of_title_likely = TitleClass::Absolute; // Usually absolute for conveyance

        FirstRegistrationResult {
            registration_required: compulsory || facts.voluntary,
            registration_compulsory: compulsory,
            deadline_days,
            days_remaining,
            consequences_if_missed: consequences,
            class_of_title_likely,
            recommendations,
        }
    }
}

// ============================================================================
// Priority Analyzer (LRA 2002 ss.28-30)
// ============================================================================

/// Facts for priority analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityFacts {
    /// Interest being analyzed
    pub interest_description: String,
    /// Type of interest
    pub interest_type: InterestCategory,
    /// Date of creation
    pub creation_date: String,
    /// Protected by notice/restriction
    pub protected_on_register: bool,
    /// Overriding interest claimed
    pub overriding_claimed: Option<OverridingInterest>,
    /// Actual occupation (for Schedule 3)
    pub actual_occupation: bool,
    /// Obvious on inspection
    pub obvious_on_inspection: bool,
    /// Competing interest date
    pub competing_interest_date: Option<String>,
}

/// Category of interest for priority
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterestCategory {
    /// Registered disposition (s.29)
    RegisteredDisposition,
    /// Minor interest (equitable)
    MinorInterest,
    /// Overriding interest
    OverridingInterest,
    /// Unregistered interest
    UnregisteredInterest,
}

/// Result of priority analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityResult {
    pub has_priority: bool,
    pub priority_basis: PriorityBasis,
    pub vulnerable_to: Vec<String>,
    pub protected_against: Vec<String>,
    pub reasoning: String,
    pub key_cases: Vec<LandLawCase>,
}

/// Basis for priority
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PriorityBasis {
    /// First in time (basic rule s.28)
    FirstInTime,
    /// Registered disposition for value (s.29)
    RegisteredDisposition,
    /// Overriding interest (Sch 3)
    OverridingInterest,
    /// Protected by notice
    ProtectedByNotice,
    /// No priority
    NoPriority,
}

/// Analyzes priority of interests
pub struct PriorityAnalyzer;

impl PriorityAnalyzer {
    /// Analyze priority of interest
    pub fn analyze(facts: &PriorityFacts) -> PriorityResult {
        let mut key_cases = Vec::new();
        let mut vulnerable_to = Vec::new();
        let mut protected_against = Vec::new();

        // Determine priority basis
        let (has_priority, priority_basis) = match &facts.interest_type {
            InterestCategory::RegisteredDisposition => {
                // s.29 - takes free of unprotected interests
                protected_against.push("Unprotected minor interests".into());
                if facts.overriding_claimed.is_some() {
                    vulnerable_to.push("Overriding interests".into());
                }
                (true, PriorityBasis::RegisteredDisposition)
            }
            InterestCategory::OverridingInterest => {
                // Sch 3 - overriding interests bind purchaser
                if facts.actual_occupation && facts.obvious_on_inspection {
                    key_cases.push(LandLawCase::williams_and_glyns_v_boland());
                    (true, PriorityBasis::OverridingInterest)
                } else if facts.actual_occupation {
                    // Actual occupation but not obvious - still binds unless purchaser
                    // made inquiry and not disclosed
                    vulnerable_to.push("Purchaser who inquired and got no response".into());
                    (true, PriorityBasis::OverridingInterest)
                } else {
                    (false, PriorityBasis::NoPriority)
                }
            }
            InterestCategory::MinorInterest => {
                if facts.protected_on_register {
                    (true, PriorityBasis::ProtectedByNotice)
                } else {
                    vulnerable_to.push("Registered disposition for value".into());
                    (false, PriorityBasis::FirstInTime)
                }
            }
            InterestCategory::UnregisteredInterest => {
                vulnerable_to.push("Any registered disposition".into());
                (false, PriorityBasis::NoPriority)
            }
        };

        let reasoning = match &priority_basis {
            PriorityBasis::RegisteredDisposition => {
                "Under s.29 LRA 2002, a registered disposition for value takes free \
                 of interests not protected on the register, subject only to overriding \
                 interests under Schedule 3."
                    .into()
            }
            PriorityBasis::OverridingInterest => {
                format!(
                    "Interest qualifies as overriding under Schedule 3 LRA 2002 \
                     ({:?}). Binds purchaser without need for registration.",
                    facts.overriding_claimed
                )
            }
            PriorityBasis::ProtectedByNotice => {
                "Interest protected by notice on the register. Has priority against \
                 subsequent registered dispositions."
                    .into()
            }
            PriorityBasis::FirstInTime => {
                "Basic priority rule (s.28 LRA 2002): first in time prevails. \
                 However, vulnerable to registered disposition for value."
                    .into()
            }
            PriorityBasis::NoPriority => {
                "Interest does not have priority. Unprotected minor interest loses \
                 to registered disposition for value."
                    .into()
            }
        };

        PriorityResult {
            has_priority,
            priority_basis,
            vulnerable_to,
            protected_against,
            reasoning,
            key_cases,
        }
    }
}

// ============================================================================
// Overriding Interest Analyzer
// ============================================================================

/// Facts for overriding interest analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverridingInterestFacts {
    /// Type of interest claimed
    pub interest_type: OverridingInterest,
    /// Actual occupation facts
    pub occupation_facts: Option<OccupationFacts>,
    /// Short lease facts
    pub lease_facts: Option<ShortLeaseFacts>,
    /// Legal easement facts
    pub easement_facts: Option<LegalEasementFacts>,
}

/// Facts about actual occupation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccupationFacts {
    /// Person in actual occupation
    pub occupier: String,
    /// Nature of occupation
    pub occupation_nature: String,
    /// Obvious on reasonably careful inspection
    pub obvious_on_inspection: bool,
    /// Purchaser made inquiry
    pub inquiry_made: bool,
    /// Interest disclosed on inquiry
    pub interest_disclosed: bool,
    /// Beneficial interest held
    pub beneficial_interest: bool,
}

/// Facts about short lease
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortLeaseFacts {
    /// Lease term (years)
    pub term_years: u32,
    /// Tenant in occupation
    pub in_occupation: bool,
}

/// Facts about legal easement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalEasementFacts {
    /// Obvious on inspection
    pub obvious_on_inspection: bool,
    /// Known to purchaser
    pub known_to_purchaser: bool,
    /// Registered against servient title
    pub registered: bool,
}

/// Result of overriding interest analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverridingInterestResult {
    pub is_overriding: bool,
    pub schedule_paragraph: String,
    pub exceptions_apply: Vec<String>,
    pub reasoning: String,
    pub key_cases: Vec<LandLawCase>,
}

/// Analyzes overriding interests under Schedule 3
pub struct OverridingInterestAnalyzer;

impl OverridingInterestAnalyzer {
    /// Analyze whether interest overrides
    pub fn analyze(facts: &OverridingInterestFacts) -> OverridingInterestResult {
        let mut key_cases = Vec::new();
        let mut exceptions = Vec::new();

        let (is_overriding, schedule_paragraph, reasoning) = match &facts.interest_type {
            OverridingInterest::ActualOccupation => {
                Self::analyze_actual_occupation(facts, &mut key_cases, &mut exceptions)
            }
            OverridingInterest::ShortLease => Self::analyze_short_lease(facts, &mut exceptions),
            OverridingInterest::LegalEasement => {
                Self::analyze_legal_easement(facts, &mut exceptions)
            }
            OverridingInterest::LocalLandCharge => (
                true,
                "Schedule 3, para 6".into(),
                "Local land charges always override".into(),
            ),
            OverridingInterest::MinesAndMinerals => (
                true,
                "Schedule 3, para 7".into(),
                "Mining interests may override".into(),
            ),
            OverridingInterest::ChancelRepair => (
                true,
                "Schedule 3, para 16".into(),
                "Chancel repair liability overrides".into(),
            ),
        };

        OverridingInterestResult {
            is_overriding,
            schedule_paragraph,
            exceptions_apply: exceptions,
            reasoning,
            key_cases,
        }
    }

    fn analyze_actual_occupation(
        facts: &OverridingInterestFacts,
        key_cases: &mut Vec<LandLawCase>,
        exceptions: &mut Vec<String>,
    ) -> (bool, String, String) {
        let paragraph = "Schedule 3, paragraph 2".to_string();

        if let Some(occ) = &facts.occupation_facts {
            // Must be in actual occupation
            if !occ.beneficial_interest {
                return (
                    false,
                    paragraph,
                    "No beneficial interest held - mere occupation insufficient".into(),
                );
            }

            key_cases.push(LandLawCase::williams_and_glyns_v_boland());

            // Check exceptions under para 2(b) and (c)
            if !occ.obvious_on_inspection {
                exceptions.push(
                    "Occupation not obvious on reasonably careful inspection (para 2(c)(i))".into(),
                );
            }

            if occ.inquiry_made && !occ.interest_disclosed {
                exceptions.push("Inquiry made but interest not disclosed (para 2(c)(ii))".into());
                return (
                    false,
                    paragraph,
                    "Interest not disclosed on inquiry - exception applies".into(),
                );
            }

            let is_overriding = occ.obvious_on_inspection || !occ.inquiry_made;

            let reasoning = if is_overriding {
                format!(
                    "Interest of person ({}) in actual occupation overrides \
                     under Schedule 3, para 2. {} holds beneficial interest and \
                     occupation was {}.",
                    occ.occupier,
                    occ.occupier,
                    if occ.obvious_on_inspection {
                        "obvious on inspection"
                    } else {
                        "present (no inquiry made)"
                    }
                )
            } else {
                "Interest does not override - exception applies".into()
            };

            (is_overriding, paragraph, reasoning)
        } else {
            (false, paragraph, "No occupation facts provided".into())
        }
    }

    fn analyze_short_lease(
        facts: &OverridingInterestFacts,
        _exceptions: &mut Vec<String>,
    ) -> (bool, String, String) {
        let paragraph = "Schedule 3, paragraph 1".to_string();

        if let Some(lease) = &facts.lease_facts {
            if lease.term_years <= 7 {
                (
                    true,
                    paragraph,
                    format!(
                        "Lease of {} years (≤7 years) is an overriding interest \
                         under Schedule 3, para 1.",
                        lease.term_years
                    ),
                )
            } else {
                (
                    false,
                    paragraph,
                    format!(
                        "Lease of {} years exceeds 7 years - not an overriding interest. \
                         Must be registered.",
                        lease.term_years
                    ),
                )
            }
        } else {
            (false, paragraph, "No lease facts provided".into())
        }
    }

    fn analyze_legal_easement(
        facts: &OverridingInterestFacts,
        exceptions: &mut Vec<String>,
    ) -> (bool, String, String) {
        let paragraph = "Schedule 3, paragraph 3".to_string();

        if let Some(easement) = &facts.easement_facts {
            // Legal easement overrides if:
            // (a) registered, or
            // (b) obvious on inspection, or
            // (c) known to purchaser
            let overrides = easement.registered
                || easement.obvious_on_inspection
                || easement.known_to_purchaser;

            if !overrides {
                exceptions.push("Easement not obvious, not known, and not registered".into());
            }

            let reasoning = if overrides {
                "Legal easement overrides under Schedule 3, para 3. Easement is \
                 either registered, obvious on inspection, or known to purchaser."
                    .into()
            } else {
                "Legal easement does not override - none of the conditions met".into()
            };

            (overrides, paragraph, reasoning)
        } else {
            (false, paragraph, "No easement facts provided".into())
        }
    }
}

// ============================================================================
// Unregistered Land Analyzer
// ============================================================================

/// Facts for unregistered land analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnregisteredLandFacts {
    /// Interest type
    pub interest_type: LandChargeClass,
    /// Registered as land charge
    pub registered_as_land_charge: bool,
    /// Purchaser for value
    pub purchaser_for_value: bool,
    /// Legal or equitable
    pub legal_interest: bool,
    /// Actual notice of interest
    pub actual_notice: bool,
}

/// Result of unregistered land analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnregisteredLandResult {
    pub binding_on_purchaser: bool,
    pub registration_required: bool,
    pub consequence_of_non_registration: String,
    pub reasoning: String,
}

/// Analyzes priority in unregistered land
pub struct UnregisteredLandAnalyzer;

impl UnregisteredLandAnalyzer {
    /// Analyze priority in unregistered land
    pub fn analyze(facts: &UnregisteredLandFacts) -> UnregisteredLandResult {
        // Legal interests bind the world
        if facts.legal_interest {
            return UnregisteredLandResult {
                binding_on_purchaser: true,
                registration_required: false,
                consequence_of_non_registration: "N/A - legal interest binds regardless".into(),
                reasoning: "Legal interests bind all persons, including purchasers \
                    for value without notice."
                    .into(),
            };
        }

        // Equitable interests - registration regime under LCA 1972
        let registration_required = matches!(
            facts.interest_type,
            LandChargeClass::ClassCiv // Estate contract
                | LandChargeClass::ClassDii // Restrictive covenant
                | LandChargeClass::ClassDiii // Equitable easement
                | LandChargeClass::ClassF // Home rights
        );

        if registration_required {
            if facts.registered_as_land_charge {
                return UnregisteredLandResult {
                    binding_on_purchaser: true,
                    registration_required: true,
                    consequence_of_non_registration: "N/A - properly registered".into(),
                    reasoning: "Land charge properly registered under LCA 1972 - \
                        constitutes actual notice to all."
                        .into(),
                };
            } else {
                // Not registered - void against purchaser (s.4 LCA 1972)
                let consequence = match facts.interest_type {
                    LandChargeClass::ClassCiv
                    | LandChargeClass::ClassDii
                    | LandChargeClass::ClassDiii => {
                        "Void against purchaser of legal estate for money or money's worth"
                    }
                    LandChargeClass::ClassF => "Void against purchaser of any interest",
                    _ => "Void against purchaser",
                };

                return UnregisteredLandResult {
                    binding_on_purchaser: false,
                    registration_required: true,
                    consequence_of_non_registration: consequence.into(),
                    reasoning: "Registrable land charge not registered - void against \
                        purchaser for value regardless of actual notice (Midland Bank v \
                        Green)."
                        .into(),
                };
            }
        }

        // Non-registrable equitable interests - doctrine of notice applies
        let binding = !facts.purchaser_for_value || facts.actual_notice;

        UnregisteredLandResult {
            binding_on_purchaser: binding,
            registration_required: false,
            consequence_of_non_registration: "N/A - not registrable".into(),
            reasoning: if binding {
                "Equitable interest binds - either not a purchaser for value, \
                    or purchaser had notice."
                    .into()
            } else {
                "Equity's darling - bona fide purchaser for value of legal estate \
                    without notice takes free."
                    .into()
            },
        }
    }
}

// ============================================================================
// Alteration and Indemnity Analyzer
// ============================================================================

/// Facts for alteration/indemnity analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlterationFacts {
    /// Type of alteration sought
    pub alteration_type: AlterationType,
    /// Register is incorrect
    pub register_incorrect: bool,
    /// Applicant is registered proprietor
    pub is_proprietor: bool,
    /// Proprietor caused or contributed to error
    pub proprietor_contributed: bool,
    /// Proprietor in possession
    pub proprietor_in_possession: bool,
    /// Exceptional circumstances
    pub exceptional_circumstances: Option<String>,
    /// Loss suffered (pence)
    pub loss_pence: u64,
}

/// Type of alteration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlterationType {
    /// Correction of mistake (rectification)
    Rectification,
    /// Updating register
    Updating,
    /// Removal of superfluous entry
    RemoveSuperfluous,
}

/// Result of alteration/indemnity analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlterationResult {
    pub alteration_available: bool,
    pub is_rectification: bool,
    pub requires_court_order: bool,
    pub indemnity_available: bool,
    pub indemnity_amount_pence: u64,
    pub reasoning: String,
    pub schedule_4_paragraph: Option<String>,
}

/// Analyzes alteration and indemnity
pub struct AlterationAnalyzer;

impl AlterationAnalyzer {
    /// Analyze alteration claim
    pub fn analyze(facts: &AlterationFacts) -> AlterationResult {
        // Determine if this is rectification (prejudicial to proprietor)
        let is_rectification =
            facts.is_proprietor && facts.alteration_type == AlterationType::Rectification;

        // Schedule 4 analysis
        let (alteration_available, schedule_para) = if is_rectification {
            // Para 3: Rectification against proprietor in possession
            if facts.proprietor_in_possession {
                // Only available if proprietor contributed OR exceptional circumstances
                let available =
                    facts.proprietor_contributed || facts.exceptional_circumstances.is_some();
                (available, Some("Schedule 4, paragraph 3(2)".into()))
            } else {
                // Not in possession - normal rules apply
                (
                    facts.register_incorrect,
                    Some("Schedule 4, paragraph 2".into()),
                )
            }
        } else {
            // Non-rectification alterations - available if register incorrect
            (
                facts.register_incorrect,
                Some("Schedule 4, paragraph 2".into()),
            )
        };

        // Indemnity analysis (Schedule 8)
        let indemnity_available = !alteration_available && facts.loss_pence > 0;
        let indemnity_amount = if indemnity_available {
            // Reduced if proprietor contributed (para 5)
            if facts.proprietor_contributed {
                facts.loss_pence / 2 // Simplified - actual reduction may vary
            } else {
                facts.loss_pence
            }
        } else {
            0
        };

        let reasoning = if alteration_available {
            if is_rectification {
                "Rectification available under Schedule 4. Despite prejudice to \
                 proprietor, conditions met (proprietor contributed to error or \
                 exceptional circumstances)."
                    .into()
            } else {
                "Alteration available under Schedule 4 to correct register.".into()
            }
        } else if indemnity_available {
            format!(
                "Alteration not available (proprietor protected). Indemnity of \
                 {} pence available under Schedule 8.",
                indemnity_amount
            )
        } else {
            "Neither alteration nor indemnity available.".into()
        };

        AlterationResult {
            alteration_available,
            is_rectification,
            requires_court_order: is_rectification && facts.proprietor_in_possession,
            indemnity_available,
            indemnity_amount_pence: indemnity_amount,
            reasoning,
            schedule_4_paragraph: schedule_para,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_registration_required() {
        let facts = FirstRegistrationFacts {
            status: RegistrationStatus::Unregistered,
            trigger: Some(FirstRegistrationTrigger::TransferOfFreehold),
            trigger_date: Some("2024-01-01".into()),
            days_since_trigger: 30,
            voluntary: false,
            property: "1 Test Street".into(),
        };
        let result = FirstRegistrationAnalyzer::analyze(&facts);
        assert!(result.registration_required);
        assert!(result.registration_compulsory);
        assert_eq!(result.deadline_days, 60);
        assert!(result.days_remaining.is_some_and(|d| d > 0));
    }

    #[test]
    fn test_first_registration_deadline_missed() {
        let facts = FirstRegistrationFacts {
            status: RegistrationStatus::Unregistered,
            trigger: Some(FirstRegistrationTrigger::TransferOfFreehold),
            trigger_date: Some("2023-01-01".into()),
            days_since_trigger: 90, // Past deadline
            voluntary: false,
            property: "1 Test Street".into(),
        };
        let result = FirstRegistrationAnalyzer::analyze(&facts);
        assert!(result.days_remaining.is_some_and(|d| d < 0));
        assert!(
            result
                .consequences_if_missed
                .iter()
                .any(|c| c.contains("missed"))
        );
    }

    #[test]
    fn test_priority_registered_disposition() {
        let facts = PriorityFacts {
            interest_description: "Purchase of freehold".into(),
            interest_type: InterestCategory::RegisteredDisposition,
            creation_date: "2024-01-15".into(),
            protected_on_register: true,
            overriding_claimed: None,
            actual_occupation: false,
            obvious_on_inspection: false,
            competing_interest_date: Some("2024-01-01".into()),
        };
        let result = PriorityAnalyzer::analyze(&facts);
        assert!(result.has_priority);
        assert_eq!(result.priority_basis, PriorityBasis::RegisteredDisposition);
    }

    #[test]
    fn test_overriding_actual_occupation() {
        let facts = OverridingInterestFacts {
            interest_type: OverridingInterest::ActualOccupation,
            occupation_facts: Some(OccupationFacts {
                occupier: "Spouse".into(),
                occupation_nature: "Living in property".into(),
                obvious_on_inspection: true,
                inquiry_made: false,
                interest_disclosed: false,
                beneficial_interest: true,
            }),
            lease_facts: None,
            easement_facts: None,
        };
        let result = OverridingInterestAnalyzer::analyze(&facts);
        assert!(result.is_overriding);
        assert!(result.key_cases.iter().any(|c| c.name.contains("Boland")));
    }

    #[test]
    fn test_overriding_short_lease() {
        let facts = OverridingInterestFacts {
            interest_type: OverridingInterest::ShortLease,
            occupation_facts: None,
            lease_facts: Some(ShortLeaseFacts {
                term_years: 5,
                in_occupation: true,
            }),
            easement_facts: None,
        };
        let result = OverridingInterestAnalyzer::analyze(&facts);
        assert!(result.is_overriding);
        assert!(result.schedule_paragraph.contains("paragraph 1"));
    }

    #[test]
    fn test_unregistered_land_charge_void() {
        let facts = UnregisteredLandFacts {
            interest_type: LandChargeClass::ClassDii,
            registered_as_land_charge: false,
            purchaser_for_value: true,
            legal_interest: false,
            actual_notice: true, // Has actual notice but still void
        };
        let result = UnregisteredLandAnalyzer::analyze(&facts);
        assert!(!result.binding_on_purchaser);
        assert!(result.reasoning.contains("Midland Bank"));
    }

    #[test]
    fn test_alteration_rectification() {
        let facts = AlterationFacts {
            alteration_type: AlterationType::Rectification,
            register_incorrect: true,
            is_proprietor: true,
            proprietor_contributed: true,
            proprietor_in_possession: true,
            exceptional_circumstances: None,
            loss_pence: 0,
        };
        let result = AlterationAnalyzer::analyze(&facts);
        assert!(result.alteration_available);
        assert!(result.is_rectification);
        assert!(result.requires_court_order);
    }

    #[test]
    fn test_indemnity_available() {
        let facts = AlterationFacts {
            alteration_type: AlterationType::Rectification,
            register_incorrect: true,
            is_proprietor: true,
            proprietor_contributed: false,
            proprietor_in_possession: true,
            exceptional_circumstances: None,
            loss_pence: 10_000_000,
        };
        let result = AlterationAnalyzer::analyze(&facts);
        // Rectification not available (proprietor in possession, didn't contribute)
        assert!(!result.alteration_available);
        assert!(result.indemnity_available);
        assert_eq!(result.indemnity_amount_pence, 10_000_000);
    }
}
