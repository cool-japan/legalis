//! UK Land Law - Interests Module
//!
//! This module provides analysis of interests in land:
//! - Easements (Re Ellenborough Park requirements)
//! - Restrictive covenants (Tulk v Moxhay)
//! - Mortgages (legal charges, remedies)
//!
//! Key statutes:
//! - Law of Property Act 1925 (s.1 - legal interests, s.62 - implied grant)
//! - Land Charges Act 1972 (unregistered land)
//!
//! Key cases:
//! - Re Ellenborough Park [1956] Ch 131 (easement requirements)
//! - Tulk v Moxhay (1848) 2 Ph 774 (restrictive covenants in equity)
//! - Wheeldon v Burrows (1879) 12 Ch D 31 (implied easements)
//! - Etridge (No 2) [2001] UKHL 44 (undue influence in mortgages)

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    Covenant, CovenantNature, Easement, EasementCreation, EasementType, LandLawCase, Mortgage,
    MortgageRemedy,
};

// ============================================================================
// Easement Analyzer (Re Ellenborough Park)
// ============================================================================

/// Facts for easement analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EasementFacts {
    /// Proposed easement
    pub easement: Easement,
    /// Dominant tenement description
    pub dominant_description: String,
    /// Servient tenement description
    pub servient_description: String,
    /// Same owner of both tenements
    pub common_ownership: bool,
    /// Type of benefit claimed
    pub benefit_type: EasementBenefit,
    /// How right was created/claimed
    pub creation_facts: CreationFacts,
}

/// Type of benefit from easement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasementBenefit {
    /// Necessary for access
    AccessNecessity,
    /// Convenience/enhancement
    Convenience,
    /// Light and air
    LightAndAir,
    /// Support of buildings
    Support,
    /// Drainage
    Drainage,
    /// Services (utilities)
    Services,
    /// Purely recreational
    Recreational,
}

/// Facts about creation of easement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreationFacts {
    /// Method of creation claimed
    pub method: EasementCreation,
    /// For express grant: deed executed
    pub deed_executed: bool,
    /// For implied: necessity/common intention
    pub necessity_or_common_intention: Option<String>,
    /// For s.62: diversity of occupation before conveyance
    pub diversity_of_occupation: bool,
    /// For prescription: years of use
    pub years_of_use: Option<u32>,
    /// Use as of right (nec vi, nec clam, nec precario)
    pub use_as_of_right: bool,
    /// For Wheeldon v Burrows: continuous and apparent
    pub continuous_and_apparent: bool,
    /// For Wheeldon v Burrows: necessary for reasonable enjoyment
    pub necessary_for_enjoyment: bool,
}

/// Result of easement analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EasementAnalysisResult {
    pub valid_easement: bool,
    pub legal_or_equitable: LegalOrEquitable,
    pub re_ellenborough_satisfied: bool,
    pub dominant_accommodated: bool,
    pub capable_of_grant: bool,
    pub creation_valid: bool,
    pub issues: Vec<String>,
    pub reasoning: String,
    pub key_cases: Vec<LandLawCase>,
}

/// Whether interest is legal or equitable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalOrEquitable {
    /// Legal interest
    Legal,
    /// Equitable interest
    Equitable,
    /// Cannot exist (failed requirements)
    CannotExist,
}

/// Analyzes easements against Re Ellenborough Park requirements
pub struct EasementAnalyzer;

impl EasementAnalyzer {
    /// Analyze whether claimed right constitutes valid easement
    pub fn analyze(facts: &EasementFacts) -> EasementAnalysisResult {
        let mut key_cases = vec![LandLawCase::re_ellenborough_park()];
        let mut issues = Vec::new();

        // Requirement 1: Dominant and servient tenement
        let has_both_tenements =
            !facts.dominant_description.is_empty() && !facts.servient_description.is_empty();

        if !has_both_tenements {
            issues.push("Easement in gross not permitted - must have dominant tenement".into());
        }

        // Requirement 2: Different owners
        if facts.common_ownership {
            issues.push("Same owner cannot have easement over own land".into());
        }

        // Requirement 3: Easement must accommodate dominant tenement
        let dominant_accommodated = Self::check_accommodation(&facts.benefit_type);
        if !dominant_accommodated {
            issues.push(
                "Right does not accommodate dominant tenement - mere personal advantage".into(),
            );
        }

        // Requirement 4: Capable of forming subject matter of grant
        let capable_of_grant = Self::check_capable_of_grant(&facts.easement.easement_type);
        if !capable_of_grant {
            issues.push("Right not capable of forming subject matter of grant".into());
        }

        let re_ellenborough_satisfied = has_both_tenements
            && !facts.common_ownership
            && dominant_accommodated
            && capable_of_grant;

        // Check creation method
        let (creation_valid, creation_reasoning) = Self::check_creation(&facts.creation_facts);
        if !creation_valid {
            issues.push(creation_reasoning.clone());
        }

        // Add relevant case law
        match &facts.creation_facts.method {
            EasementCreation::WheeldonVBurrows => {
                key_cases.push(LandLawCase::wheeldon_v_burrows());
            }
            EasementCreation::Section62Lpa => {
                key_cases.push(LandLawCase {
                    name: "Wright v Macadam".into(),
                    citation: "[1949] 2 KB 744".into(),
                    year: 1949,
                    principle: "Section 62 LPA 1925 converts licence into easement on \
                        conveyance where there was prior diversity of occupation."
                        .into(),
                });
            }
            EasementCreation::PrescriptionAct1832 | EasementCreation::PrescriptionCommonLaw => {
                key_cases.push(LandLawCase {
                    name: "Tehidy Minerals v Norman".into(),
                    citation: "[1971] 2 QB 528".into(),
                    year: 1971,
                    principle: "Use must be as of right: nec vi, nec clam, nec precario \
                        (without force, secrecy, or permission)."
                        .into(),
                });
            }
            _ => {}
        }

        // Determine legal or equitable
        let legal_or_equitable = Self::determine_legal_or_equitable(facts, creation_valid);

        let valid_easement = re_ellenborough_satisfied && creation_valid;

        let reasoning = if valid_easement {
            format!(
                "Valid {} easement established. All four Re Ellenborough Park requirements \
                 satisfied and creation method valid.",
                if legal_or_equitable == LegalOrEquitable::Legal {
                    "legal"
                } else {
                    "equitable"
                }
            )
        } else {
            format!("Claimed easement fails. Issues: {}", issues.join("; "))
        };

        EasementAnalysisResult {
            valid_easement,
            legal_or_equitable,
            re_ellenborough_satisfied,
            dominant_accommodated,
            capable_of_grant,
            creation_valid,
            issues,
            reasoning,
            key_cases,
        }
    }

    fn check_accommodation(benefit_type: &EasementBenefit) -> bool {
        // Must benefit land, not just owner personally
        match benefit_type {
            EasementBenefit::AccessNecessity
            | EasementBenefit::LightAndAir
            | EasementBenefit::Support
            | EasementBenefit::Drainage
            | EasementBenefit::Services
            | EasementBenefit::Convenience => true,
            EasementBenefit::Recreational => {
                // Re Ellenborough Park allowed recreational use
                true
            }
        }
    }

    fn check_capable_of_grant(easement_type: &EasementType) -> bool {
        // Most easement types are capable of grant
        // Exclude vague/personal rights
        match easement_type {
            EasementType::RightOfWay { .. }
            | EasementType::RightOfLight
            | EasementType::RightOfSupport
            | EasementType::RightOfDrainage
            | EasementType::Services { .. }
            | EasementType::Parking
            | EasementType::Storage => true,
            EasementType::Other { description } => {
                // Check if too wide/vague (like "right to a view" - not an easement)
                !description.to_lowercase().contains("view")
                    && !description.to_lowercase().contains("privacy")
            }
        }
    }

    fn check_creation(facts: &CreationFacts) -> (bool, String) {
        match &facts.method {
            EasementCreation::ExpressGrant | EasementCreation::ExpressReservation => {
                if facts.deed_executed {
                    (true, "Express creation by deed - valid".into())
                } else {
                    (
                        false,
                        "Express easement requires deed (s.52 LPA 1925)".into(),
                    )
                }
            }
            EasementCreation::ImpliedNecessity => {
                if facts.necessity_or_common_intention.is_some() {
                    (true, "Implied by necessity - valid".into())
                } else {
                    (false, "Necessity not established".into())
                }
            }
            EasementCreation::WheeldonVBurrows => {
                if facts.continuous_and_apparent && facts.necessary_for_enjoyment {
                    (true, "Wheeldon v Burrows requirements satisfied".into())
                } else {
                    (
                        false,
                        "Wheeldon v Burrows: must be continuous, apparent, and necessary".into(),
                    )
                }
            }
            EasementCreation::Section62Lpa => {
                if facts.diversity_of_occupation {
                    (
                        true,
                        "Section 62 LPA 1925 operates - prior diversity shown".into(),
                    )
                } else {
                    (
                        false,
                        "Section 62: requires prior diversity of occupation".into(),
                    )
                }
            }
            EasementCreation::PrescriptionCommonLaw
            | EasementCreation::PrescriptionAct1832
            | EasementCreation::LostModernGrant => {
                let years_required = match &facts.method {
                    EasementCreation::PrescriptionCommonLaw => 20,
                    EasementCreation::PrescriptionAct1832 => 20, // 20 for easements, 40 for light
                    EasementCreation::LostModernGrant => 20,
                    _ => 20,
                };

                if let Some(years) = facts.years_of_use {
                    if years >= years_required && facts.use_as_of_right {
                        (
                            true,
                            format!("Prescription established ({} years use as of right)", years),
                        )
                    } else if !facts.use_as_of_right {
                        (false, "Use not as of right (vi, clam, or precario)".into())
                    } else {
                        (
                            false,
                            format!(
                                "Insufficient period: {} years (need {})",
                                years, years_required
                            ),
                        )
                    }
                } else {
                    (false, "No use period established".into())
                }
            }
            EasementCreation::ImpliedCommonIntention => {
                if facts.necessity_or_common_intention.is_some() {
                    (true, "Implied by common intention - valid".into())
                } else {
                    (false, "Common intention not established".into())
                }
            }
        }
    }

    fn determine_legal_or_equitable(
        facts: &EasementFacts,
        creation_valid: bool,
    ) -> LegalOrEquitable {
        if !creation_valid {
            return LegalOrEquitable::CannotExist;
        }

        match &facts.creation_facts.method {
            EasementCreation::ExpressGrant | EasementCreation::ExpressReservation => {
                if facts.creation_facts.deed_executed {
                    LegalOrEquitable::Legal
                } else {
                    LegalOrEquitable::Equitable
                }
            }
            EasementCreation::Section62Lpa
            | EasementCreation::PrescriptionCommonLaw
            | EasementCreation::PrescriptionAct1832
            | EasementCreation::LostModernGrant => LegalOrEquitable::Legal,
            EasementCreation::ImpliedNecessity
            | EasementCreation::ImpliedCommonIntention
            | EasementCreation::WheeldonVBurrows => {
                // Implied easements are legal if lease/freehold was by deed
                if facts.creation_facts.deed_executed {
                    LegalOrEquitable::Legal
                } else {
                    LegalOrEquitable::Equitable
                }
            }
        }
    }
}

// ============================================================================
// Covenant Analyzer (Tulk v Moxhay)
// ============================================================================

/// Facts for covenant analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovenantFacts {
    /// Covenant details
    pub covenant: Covenant,
    /// Original parties still involved
    pub original_parties: bool,
    /// Land has been sold
    pub land_sold: bool,
    /// Annexation clause exists
    pub annexation_clause: bool,
    /// Express assignment of benefit
    pub assignment: bool,
    /// Building scheme claimed
    pub building_scheme_claimed: bool,
    /// Covenantee retained land
    pub covenantee_retained_land: bool,
    /// Registered or protected
    pub registered: bool,
}

/// Result of covenant analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovenantAnalysisResult {
    pub enforceable: bool,
    pub burden_runs: bool,
    pub benefit_runs: bool,
    pub enforcement_method: EnforcementMethod,
    pub tulk_v_moxhay_satisfied: bool,
    pub building_scheme: bool,
    pub issues: Vec<String>,
    pub reasoning: String,
    pub key_cases: Vec<LandLawCase>,
}

/// Method of enforcement available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementMethod {
    /// At common law (original parties)
    AtLaw,
    /// In equity (Tulk v Moxhay)
    InEquity,
    /// By statute (LRA, LCA)
    ByStatute,
    /// Via building scheme
    BuildingScheme,
    /// Not enforceable
    NotEnforceable,
}

/// Analyzes covenant running with land
pub struct CovenantAnalyzer;

impl CovenantAnalyzer {
    /// Analyze whether covenant is enforceable
    pub fn analyze(facts: &CovenantFacts) -> CovenantAnalysisResult {
        let mut key_cases = Vec::new();
        let mut issues = Vec::new();

        // Check if original parties - privity of contract
        if facts.original_parties {
            return CovenantAnalysisResult {
                enforceable: true,
                burden_runs: false,
                benefit_runs: false,
                enforcement_method: EnforcementMethod::AtLaw,
                tulk_v_moxhay_satisfied: false,
                building_scheme: false,
                issues: vec![],
                reasoning: "Original covenanting parties - enforceable at law by privity \
                    of contract. No need to consider running of burden/benefit."
                    .into(),
                key_cases: vec![],
            };
        }

        // For successors, analyze running of burden and benefit
        let (burden_runs, burden_reasoning) = Self::analyze_burden(&facts.covenant, facts);
        let (benefit_runs, benefit_reasoning) = Self::analyze_benefit(facts);

        // Check building scheme
        let building_scheme = facts.building_scheme_claimed && Self::check_building_scheme(facts);
        if facts.building_scheme_claimed && !building_scheme {
            issues.push("Building scheme claimed but requirements not met".into());
        }

        // Tulk v Moxhay requirements
        let tulk_v_moxhay_satisfied = facts.covenant.covenant_nature == CovenantNature::Restrictive
            && facts.covenantee_retained_land
            && facts.registered;

        if tulk_v_moxhay_satisfied {
            key_cases.push(LandLawCase::tulk_v_moxhay());
        }

        // Determine enforcement method
        let enforcement_method = if building_scheme {
            key_cases.push(LandLawCase {
                name: "Elliston v Reacher".into(),
                citation: "[1908] 2 Ch 374".into(),
                year: 1908,
                principle: "Building scheme requirements: common vendor, defined area, \
                    intention of mutual enforceability, purchasers derive title from common vendor."
                    .into(),
            });
            EnforcementMethod::BuildingScheme
        } else if burden_runs && benefit_runs {
            EnforcementMethod::InEquity
        } else {
            EnforcementMethod::NotEnforceable
        };

        let enforceable = burden_runs && benefit_runs;

        if !burden_runs {
            issues.push(burden_reasoning.clone());
        }
        if !benefit_runs {
            issues.push(benefit_reasoning.clone());
        }

        let reasoning = if enforceable {
            format!(
                "Covenant enforceable in equity. {}. {}",
                burden_reasoning, benefit_reasoning
            )
        } else if facts.covenant.covenant_nature == CovenantNature::Positive {
            key_cases.push(LandLawCase {
                name: "Rhone v Stephens".into(),
                citation: "[1994] 2 AC 310".into(),
                year: 1994,
                principle: "Positive covenants do not run with freehold land at law \
                    or in equity. Only restrictive covenants bind successors."
                    .into(),
            });
            "Positive covenant - burden does not run with freehold land (Rhone v Stephens). \
             Consider indemnity chain or other workarounds."
                .into()
        } else {
            format!(
                "Covenant not enforceable against successor. {}",
                issues.join("; ")
            )
        };

        CovenantAnalysisResult {
            enforceable,
            burden_runs,
            benefit_runs,
            enforcement_method,
            tulk_v_moxhay_satisfied,
            building_scheme,
            issues,
            reasoning,
            key_cases,
        }
    }

    fn analyze_burden(covenant: &Covenant, facts: &CovenantFacts) -> (bool, String) {
        // Burden runs in equity if: (1) restrictive, (2) touches and concerns, (3) intended, (4) protected
        if covenant.covenant_nature == CovenantNature::Positive {
            return (
                false,
                "Positive covenant - burden does not run (Rhone v Stephens)".into(),
            );
        }

        if !facts.covenantee_retained_land {
            return (
                false,
                "Covenantee must retain benefited land for burden to run".into(),
            );
        }

        if !facts.registered {
            return (
                false,
                "Covenant not protected by registration - void against purchaser".into(),
            );
        }

        (
            true,
            "Burden runs in equity - restrictive, touches and concerns, and protected".into(),
        )
    }

    fn analyze_benefit(facts: &CovenantFacts) -> (bool, String) {
        // Benefit runs by: (1) annexation, (2) assignment, (3) building scheme
        if facts.annexation_clause {
            return (true, "Benefit runs by express annexation".into());
        }

        if facts.assignment {
            return (true, "Benefit runs by express assignment".into());
        }

        if facts.building_scheme_claimed && facts.covenant.building_scheme {
            return (true, "Benefit runs via building scheme".into());
        }

        // Implied annexation under s.78 LPA 1925 (Federated Homes)
        (
            true,
            "Benefit presumed annexed under s.78 LPA 1925 (Federated Homes v Mill Lodge)".into(),
        )
    }

    fn check_building_scheme(facts: &CovenantFacts) -> bool {
        // Elliston v Reacher requirements
        facts.covenant.building_scheme && facts.covenantee_retained_land
    }
}

// ============================================================================
// Mortgage Analyzer
// ============================================================================

/// Facts for mortgage analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MortgageFacts {
    /// Mortgage details
    pub mortgage: Mortgage,
    /// Borrower in default
    pub in_default: bool,
    /// Default type
    pub default_type: Option<DefaultType>,
    /// Months of default
    pub months_in_default: u32,
    /// Property value (pence)
    pub property_value_pence: u64,
    /// Surety involved
    pub surety_involved: bool,
    /// Undue influence concerns
    pub undue_influence_concerns: bool,
    /// Independent legal advice obtained
    pub independent_advice: bool,
}

/// Type of mortgage default
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefaultType {
    /// Non-payment of instalments
    NonPayment,
    /// Breach of covenant in mortgage
    BreachOfCovenant,
    /// Insolvency of borrower
    Insolvency,
}

/// Result of mortgage analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MortgageAnalysisResult {
    pub valid_charge: bool,
    pub remedies_available: Vec<MortgageRemedy>,
    pub possession_available: bool,
    pub sale_available: bool,
    pub undue_influence_risk: UndueInfluenceRisk,
    pub equity_of_redemption_protected: bool,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub key_cases: Vec<LandLawCase>,
}

/// Risk of undue influence defence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UndueInfluenceRisk {
    /// No risk
    None,
    /// Low risk
    Low,
    /// Medium risk - bank should have made inquiries
    Medium,
    /// High risk - likely defence
    High,
}

/// Analyzes mortgage remedies and issues
pub struct MortgageAnalyzer;

impl MortgageAnalyzer {
    /// Analyze mortgage situation
    pub fn analyze(facts: &MortgageFacts) -> MortgageAnalysisResult {
        let mut key_cases = Vec::new();
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut remedies = Vec::new();

        // Check validity of charge
        let valid_charge = facts.mortgage.legal_charge;

        // Analyze remedies if in default
        if facts.in_default {
            // Action for debt - always available
            remedies.push(MortgageRemedy::ActionForDebt);

            // Possession - available but court discretion (residential)
            let possession_available = true;
            if possession_available {
                remedies.push(MortgageRemedy::Possession);
                key_cases.push(LandLawCase {
                    name: "Four-Maids Ltd v Dudley Marshall".into(),
                    citation: "[1957] Ch 317".into(),
                    year: 1957,
                    principle: "Mortgagee's right to possession arises from the mortgage \
                        itself, not from default."
                        .into(),
                });
            }

            // Power of sale - arises after default on specific terms
            if facts.months_in_default >= 2 {
                remedies.push(MortgageRemedy::PowerOfSale);
            } else {
                recommendations.push(
                    "Power of sale not yet exercisable - s.103 LPA requirements not met".into(),
                );
            }

            // Appointment of receiver
            remedies.push(MortgageRemedy::AppointmentOfReceiver);
        }

        // Analyze undue influence risk
        let undue_influence_risk = Self::assess_undue_influence(facts);
        if undue_influence_risk == UndueInfluenceRisk::High {
            issues.push("High risk of undue influence defence succeeding".into());
            key_cases.push(LandLawCase {
                name: "Royal Bank of Scotland v Etridge (No 2)".into(),
                citation: "[2001] UKHL 44".into(),
                year: 2001,
                principle: "Bank put on inquiry where wife stands surety for husband's debt. \
                    Must take steps to ensure wife received independent legal advice."
                    .into(),
            });
        }

        // Check negative equity
        if facts.mortgage.amount_pence > facts.property_value_pence {
            issues.push("Negative equity - debt exceeds property value".into());
            recommendations.push("Consider voluntary sale to minimize loss".into());
        }

        // Equity of redemption
        let equity_protected = true; // Always protected in equity

        let sale_available = remedies.contains(&MortgageRemedy::PowerOfSale);
        let possession_available = remedies.contains(&MortgageRemedy::Possession);

        MortgageAnalysisResult {
            valid_charge,
            remedies_available: remedies,
            possession_available,
            sale_available,
            undue_influence_risk,
            equity_of_redemption_protected: equity_protected,
            issues,
            recommendations,
            key_cases,
        }
    }

    fn assess_undue_influence(facts: &MortgageFacts) -> UndueInfluenceRisk {
        if !facts.surety_involved {
            return UndueInfluenceRisk::None;
        }

        if facts.undue_influence_concerns {
            if facts.independent_advice {
                UndueInfluenceRisk::Low
            } else {
                UndueInfluenceRisk::High
            }
        } else if facts.independent_advice {
            UndueInfluenceRisk::None
        } else {
            UndueInfluenceRisk::Medium
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
    fn test_easement_valid() {
        let facts = EasementFacts {
            easement: Easement {
                easement_type: EasementType::RightOfWay {
                    foot: true,
                    vehicle: false,
                },
                dominant_tenement: "Plot A".into(),
                servient_tenement: "Plot B".into(),
                creation_method: EasementCreation::ExpressGrant,
                legal: true,
                route: Some("Path along north boundary".into()),
            },
            dominant_description: "Residential house on Plot A".into(),
            servient_description: "Access land Plot B".into(),
            common_ownership: false,
            benefit_type: EasementBenefit::AccessNecessity,
            creation_facts: CreationFacts {
                method: EasementCreation::ExpressGrant,
                deed_executed: true,
                necessity_or_common_intention: None,
                diversity_of_occupation: false,
                years_of_use: None,
                use_as_of_right: false,
                continuous_and_apparent: false,
                necessary_for_enjoyment: false,
            },
        };
        let result = EasementAnalyzer::analyze(&facts);
        assert!(result.valid_easement);
        assert!(result.re_ellenborough_satisfied);
        assert_eq!(result.legal_or_equitable, LegalOrEquitable::Legal);
    }

    #[test]
    fn test_easement_common_ownership_fails() {
        let facts = EasementFacts {
            easement: Easement {
                easement_type: EasementType::RightOfWay {
                    foot: true,
                    vehicle: false,
                },
                dominant_tenement: "Plot A".into(),
                servient_tenement: "Plot B".into(),
                creation_method: EasementCreation::ExpressGrant,
                legal: false,
                route: None,
            },
            dominant_description: "Plot A".into(),
            servient_description: "Plot B".into(),
            common_ownership: true, // Same owner
            benefit_type: EasementBenefit::Convenience,
            creation_facts: CreationFacts {
                method: EasementCreation::ExpressGrant,
                deed_executed: true,
                necessity_or_common_intention: None,
                diversity_of_occupation: false,
                years_of_use: None,
                use_as_of_right: false,
                continuous_and_apparent: false,
                necessary_for_enjoyment: false,
            },
        };
        let result = EasementAnalyzer::analyze(&facts);
        assert!(!result.valid_easement);
        assert!(!result.re_ellenborough_satisfied);
    }

    #[test]
    fn test_prescription() {
        let facts = EasementFacts {
            easement: Easement {
                easement_type: EasementType::RightOfWay {
                    foot: true,
                    vehicle: true,
                },
                dominant_tenement: "Farm A".into(),
                servient_tenement: "Farm B".into(),
                creation_method: EasementCreation::PrescriptionCommonLaw,
                legal: true,
                route: Some("Track across field".into()),
            },
            dominant_description: "Farm A".into(),
            servient_description: "Farm B".into(),
            common_ownership: false,
            benefit_type: EasementBenefit::AccessNecessity,
            creation_facts: CreationFacts {
                method: EasementCreation::PrescriptionCommonLaw,
                deed_executed: false,
                necessity_or_common_intention: None,
                diversity_of_occupation: false,
                years_of_use: Some(25),
                use_as_of_right: true,
                continuous_and_apparent: false,
                necessary_for_enjoyment: false,
            },
        };
        let result = EasementAnalyzer::analyze(&facts);
        assert!(result.valid_easement);
        assert!(result.creation_valid);
    }

    #[test]
    fn test_restrictive_covenant_runs() {
        let facts = CovenantFacts {
            covenant: Covenant {
                wording: "Not to use the property for trade or business".into(),
                covenant_nature: CovenantNature::Restrictive,
                benefited_land: Some("Plot A".into()),
                burdened_land: "Plot B".into(),
                burden_runs: true,
                benefit_runs: true,
                building_scheme: false,
            },
            original_parties: false,
            land_sold: true,
            annexation_clause: true,
            assignment: false,
            building_scheme_claimed: false,
            covenantee_retained_land: true,
            registered: true,
        };
        let result = CovenantAnalyzer::analyze(&facts);
        assert!(result.enforceable);
        assert!(result.burden_runs);
        assert!(result.benefit_runs);
    }

    #[test]
    fn test_positive_covenant_fails() {
        let facts = CovenantFacts {
            covenant: Covenant {
                wording: "To maintain the boundary fence".into(),
                covenant_nature: CovenantNature::Positive,
                benefited_land: Some("Plot A".into()),
                burdened_land: "Plot B".into(),
                burden_runs: false,
                benefit_runs: false,
                building_scheme: false,
            },
            original_parties: false,
            land_sold: true,
            annexation_clause: false,
            assignment: false,
            building_scheme_claimed: false,
            covenantee_retained_land: true,
            registered: true,
        };
        let result = CovenantAnalyzer::analyze(&facts);
        assert!(!result.enforceable);
        assert!(!result.burden_runs);
        assert!(result.reasoning.contains("Positive covenant"));
    }

    #[test]
    fn test_mortgage_remedies() {
        let facts = MortgageFacts {
            mortgage: Mortgage {
                lender: "Test Bank".into(),
                property: "1 Test Street".into(),
                amount_pence: 30_000_000,
                legal_charge: true,
                priority: 1,
                registered: true,
            },
            in_default: true,
            default_type: Some(DefaultType::NonPayment),
            months_in_default: 3,
            property_value_pence: 50_000_000,
            surety_involved: false,
            undue_influence_concerns: false,
            independent_advice: false,
        };
        let result = MortgageAnalyzer::analyze(&facts);
        assert!(result.valid_charge);
        assert!(result.possession_available);
        assert!(result.sale_available);
        assert!(
            result
                .remedies_available
                .contains(&MortgageRemedy::PowerOfSale)
        );
    }

    #[test]
    fn test_undue_influence_risk() {
        let facts = MortgageFacts {
            mortgage: Mortgage {
                lender: "Test Bank".into(),
                property: "Family Home".into(),
                amount_pence: 20_000_000,
                legal_charge: true,
                priority: 1,
                registered: true,
            },
            in_default: false,
            default_type: None,
            months_in_default: 0,
            property_value_pence: 40_000_000,
            surety_involved: true,
            undue_influence_concerns: true,
            independent_advice: false,
        };
        let result = MortgageAnalyzer::analyze(&facts);
        assert_eq!(result.undue_influence_risk, UndueInfluenceRisk::High);
    }
}
