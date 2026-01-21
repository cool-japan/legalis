//! UK Occupiers' Liability
//!
//! This module implements occupiers' liability under:
//! - Occupiers' Liability Act 1957 (visitors)
//! - Occupiers' Liability Act 1984 (trespassers and others)
//!
//! Key cases:
//! - Wheat v E Lacon & Co Ltd \[1966\] AC 552 (multiple occupiers)
//! - Roles v Nathan \[1963\] 1 WLR 1117 (skilled visitors)
//! - Tomlinson v Congleton BC \[2003\] UKHL 47 (obvious risks)

use serde::{Deserialize, Serialize};

use super::error::TortError;
use super::types::{ContributoryNegligence, DefenceEffect, PartyType, TortParty};

// ============================================================================
// Core Types for Occupiers' Liability
// ============================================================================

/// Status of person on premises
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntrantStatus {
    /// Visitor with express permission (OLA 1957)
    ExpressVisitor,
    /// Visitor with implied permission (OLA 1957)
    ImpliedVisitor,
    /// Contractual entrant (OLA 1957 via contract)
    ContractualEntrant,
    /// Person with statutory right of entry (e.g., police, firefighters)
    StatutoryEntrant,
    /// Trespasser (OLA 1984)
    Trespasser,
    /// Person exercising private right of way (OLA 1984)
    RightOfWay,
    /// Person exercising access rights under CROW Act 2000 (OLA 1984)
    CROWAccessRight,
    /// Child (may have different treatment)
    Child(u8), // age
}

/// Type of premises
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PremisesType {
    /// Residential property
    Residential,
    /// Commercial premises
    Commercial,
    /// Industrial premises
    Industrial,
    /// Agricultural land
    Agricultural,
    /// Public park or open space
    PublicOpenSpace,
    /// Construction site
    ConstructionSite,
    /// Educational institution
    Educational,
    /// Sports/leisure facility
    SportsLeisure,
    /// Transport infrastructure
    Transport,
    /// Other
    Other(String),
}

/// Type of danger on premises
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PremisesDanger {
    /// Description of danger
    pub description: String,
    /// Type of danger
    pub danger_type: DangerType,
    /// Was the danger obvious?
    pub obvious: bool,
    /// Was the danger concealed?
    pub concealed: bool,
    /// Risk level
    pub risk_level: RiskSeverity,
    /// Was there a warning?
    pub warning: Option<Warning>,
    /// Has the danger been there for long?
    pub longstanding: bool,
    /// Is this a known danger?
    pub known_to_occupier: bool,
}

/// Type of danger
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DangerType {
    /// Structural defect
    Structural,
    /// Slippery surface
    SlipperySurface,
    /// Uneven surface
    UnevenSurface,
    /// Inadequate lighting
    InadequateLighting,
    /// Hazardous substance
    HazardousSubstance,
    /// Dangerous machinery
    Machinery,
    /// Height/falling risk
    FallingRisk,
    /// Water hazard
    WaterHazard,
    /// Electrical hazard
    Electrical,
    /// Animal hazard
    Animal,
    /// Other
    Other(String),
}

/// Severity of risk
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskSeverity {
    /// Minor risk
    Minor,
    /// Moderate risk
    Moderate,
    /// Serious risk
    Serious,
    /// Severe risk
    Severe,
}

/// Warning given about danger
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Warning {
    /// Description of warning
    pub description: String,
    /// Type of warning
    pub warning_type: WarningType,
    /// Was warning adequate? (s.2(4)(a) OLA 1957)
    pub adequate: bool,
    /// Was warning visible?
    pub visible: bool,
    /// Was warning comprehensible?
    pub comprehensible: bool,
}

/// Type of warning
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarningType {
    /// Written sign
    WrittenSign,
    /// Verbal warning
    Verbal,
    /// Physical barrier
    PhysicalBarrier,
    /// Lighting/visual indication
    Visual,
    /// Fencing
    Fencing,
    /// Other
    Other(String),
}

/// Occupier details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Occupier {
    /// Name
    pub name: String,
    /// Type of occupier
    pub party_type: PartyType,
    /// Basis of occupation
    pub occupation_basis: OccupationBasis,
    /// Degree of control
    pub control_degree: ControlDegree,
}

/// Basis on which person is an occupier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OccupationBasis {
    /// Freehold owner
    FreeholdOwner,
    /// Leasehold tenant
    LeaseholdTenant,
    /// Licensee (not a visitor but occupier)
    Licensee,
    /// Contractor with control
    Contractor,
    /// Management company
    ManagementCompany,
    /// De facto control
    DeFactoControl,
}

/// Degree of control over premises
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlDegree {
    /// Full control
    Full,
    /// Partial control (shared occupier)
    Partial,
    /// Control over specific area only
    SpecificArea,
    /// Minimal control
    Minimal,
}

// ============================================================================
// OLA 1957 Analysis (Visitors)
// ============================================================================

/// Analysis under Occupiers' Liability Act 1957
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OLA1957Analysis {
    /// Is defendant an occupier?
    pub is_occupier: OccupierAnalysis,
    /// Is claimant a visitor?
    pub is_visitor: VisitorAnalysis,
    /// Common duty of care (s.2)
    pub common_duty: CommonDutyAnalysis,
    /// Any applicable defences
    pub defences: Vec<OLADefence>,
    /// Result of analysis
    pub liability_established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Analysis of occupier status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OccupierAnalysis {
    /// Potential occupiers
    pub occupiers: Vec<Occupier>,
    /// Is defendant an occupier?
    pub defendant_is_occupier: bool,
    /// Reasoning (Wheat v Lacon)
    pub reasoning: String,
}

/// Analysis of visitor status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisitorAnalysis {
    /// Status of claimant
    pub status: EntrantStatus,
    /// Is claimant a visitor under OLA 1957?
    pub is_visitor: bool,
    /// Did claimant exceed permission?
    pub exceeded_permission: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Analysis of common duty of care (s.2 OLA 1957)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonDutyAnalysis {
    /// The danger that caused harm
    pub danger: PremisesDanger,
    /// Did occupier take reasonable care?
    pub took_reasonable_care: bool,
    /// Special considerations for children (s.2(3)(a))
    pub child_considerations: Option<ChildConsiderations>,
    /// Skilled visitor considerations (s.2(3)(b))
    pub skilled_visitor: Option<SkilledVisitorAnalysis>,
    /// Warning analysis (s.2(4)(a))
    pub warning_analysis: Option<WarningAnalysis>,
    /// Independent contractor (s.2(4)(b))
    pub independent_contractor: Option<IndependentContractorAnalysis>,
    /// Was duty breached?
    pub duty_breached: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Considerations for child visitors under s.2(3)(a)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildConsiderations {
    /// Age of child
    pub age: u8,
    /// Was there an allurement?
    pub allurement: bool,
    /// Description of allurement
    pub allurement_description: Option<String>,
    /// Could occupier expect children to be less careful?
    pub expected_less_careful: bool,
    /// Higher duty owed?
    pub higher_duty: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Analysis of skilled visitors under s.2(3)(b)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkilledVisitorAnalysis {
    /// Skill/profession of visitor
    pub skill: String,
    /// Was danger within their professional competence?
    pub danger_within_competence: bool,
    /// Could visitor have been expected to guard against it?
    pub expected_to_guard: bool,
    /// Reduced duty applies?
    pub reduced_duty: bool,
    /// Reasoning (Roles v Nathan)
    pub reasoning: String,
}

/// Warning analysis under s.2(4)(a)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarningAnalysis {
    /// Warning given
    pub warning: Warning,
    /// Was warning enough to enable visitor to be reasonably safe?
    pub enables_reasonable_safety: bool,
    /// Would warning alone discharge duty?
    pub discharges_duty: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Independent contractor analysis under s.2(4)(b)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndependentContractorAnalysis {
    /// Was work of construction, maintenance or repair?
    pub construction_maintenance_repair: bool,
    /// Was it reasonable to entrust to contractor?
    pub reasonable_to_entrust: bool,
    /// Did occupier take reasonable steps to check competence?
    pub checked_competence: bool,
    /// Did occupier take reasonable steps to check work done?
    pub checked_work: bool,
    /// Is occupier absolved?
    pub occupier_absolved: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Defence to OLA claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OLADefence {
    /// Type of defence
    pub defence_type: OLADefenceType,
    /// Does defence apply?
    pub applies: bool,
    /// Effect of defence
    pub effect: DefenceEffect,
    /// Reasoning
    pub reasoning: String,
}

/// Types of defence under OLA
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OLADefenceType {
    /// Volenti (s.2(5) OLA 1957)
    Volenti,
    /// Contributory negligence
    ContributoryNegligence,
    /// Adequate warning
    AdequateWarning,
    /// Exclusion of liability (subject to UCTA 1977)
    ExclusionOfLiability,
    /// Visitor exceeded permission
    ExceededPermission,
    /// Independent contractor defence
    IndependentContractor,
}

// ============================================================================
// OLA 1984 Analysis (Non-Visitors)
// ============================================================================

/// Analysis under Occupiers' Liability Act 1984
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OLA1984Analysis {
    /// Is defendant an occupier?
    pub is_occupier: OccupierAnalysis,
    /// Type of non-visitor
    pub non_visitor_type: NonVisitorType,
    /// s.1(3) conditions
    pub section_1_3: Section1_3Analysis,
    /// s.1(4) duty
    pub section_1_4: Section1_4Analysis,
    /// Any applicable defences
    pub defences: Vec<OLA1984Defence>,
    /// Result
    pub liability_established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of non-visitor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonVisitorType {
    /// Trespasser
    Trespasser,
    /// Person using right of way
    RightOfWay,
    /// Person exercising CROW 2000 access right
    CROWAccess,
    /// Person who exceeded permission
    ExceededPermission,
}

/// Analysis of s.1(3) OLA 1984 conditions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section1_3Analysis {
    /// s.1(3)(a) - aware of danger or has reasonable grounds to believe it exists
    pub aware_of_danger: AwarenessAnalysis,
    /// s.1(3)(b) - knows or has reasonable grounds to believe other is in vicinity
    pub knows_other_in_vicinity: PresenceKnowledgeAnalysis,
    /// s.1(3)(c) - risk is one against which occupier may reasonably be expected to offer protection
    pub reasonable_protection_expected: ReasonableProtectionAnalysis,
    /// All conditions satisfied?
    pub all_conditions_met: bool,
}

/// Analysis of occupier's awareness of danger
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwarenessAnalysis {
    /// Actual knowledge of danger?
    pub actual_knowledge: bool,
    /// Constructive knowledge (reasonable grounds to believe)?
    pub constructive_knowledge: bool,
    /// Condition satisfied?
    pub satisfied: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Analysis of occupier's knowledge of presence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PresenceKnowledgeAnalysis {
    /// Actual knowledge of presence?
    pub actual_knowledge: bool,
    /// Constructive knowledge?
    pub constructive_knowledge: bool,
    /// History of trespassers on premises?
    pub history_of_trespassers: bool,
    /// Condition satisfied?
    pub satisfied: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Analysis of reasonable protection expectation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReasonableProtectionAnalysis {
    /// Nature of risk
    pub risk_severity: RiskSeverity,
    /// Cost of protection
    pub cost_of_protection: CostOfProtection,
    /// Social utility of activity
    pub social_utility: bool,
    /// Is protection reasonably expected?
    pub protection_expected: bool,
    /// Reasoning (Tomlinson v Congleton)
    pub reasoning: String,
}

/// Cost of providing protection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CostOfProtection {
    /// Minimal cost
    Minimal,
    /// Moderate cost
    Moderate,
    /// Significant cost
    Significant,
    /// Prohibitive cost
    Prohibitive,
}

/// Analysis of s.1(4) duty under OLA 1984
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section1_4Analysis {
    /// Did occupier take such care as reasonable in all circumstances?
    pub took_reasonable_care: bool,
    /// Warning given? (s.1(5))
    pub warning: Option<Warning>,
    /// Was warning adequate to discourage entry?
    pub warning_adequate: bool,
    /// Was risk willingly accepted? (s.1(6))
    pub risk_willingly_accepted: bool,
    /// Duty breached?
    pub duty_breached: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Defence under OLA 1984
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OLA1984Defence {
    /// Type of defence
    pub defence_type: OLA1984DefenceType,
    /// Does defence apply?
    pub applies: bool,
    /// Effect
    pub effect: DefenceEffect,
    /// Reasoning
    pub reasoning: String,
}

/// Types of defence under OLA 1984
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OLA1984DefenceType {
    /// s.1(5) - obvious risk
    ObviousRisk,
    /// s.1(6) - willing acceptance of risk
    WillingAcceptance,
    /// Contributory negligence
    ContributoryNegligence,
    /// No duty arises (s.1(3) not satisfied)
    NoDutyArises,
}

// ============================================================================
// Occupiers' Liability Analyzer
// ============================================================================

/// Analyzer for occupiers' liability claims
#[derive(Debug, Clone)]
pub struct OccupiersLiabilityAnalyzer {
    /// Premises information
    premises: PremisesInfo,
}

/// Information about premises
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PremisesInfo {
    /// Type of premises
    pub premises_type: PremisesType,
    /// Description
    pub description: String,
    /// Potential occupiers
    pub occupiers: Vec<Occupier>,
}

impl OccupiersLiabilityAnalyzer {
    /// Create new analyzer
    pub fn new(premises: PremisesInfo) -> Self {
        Self { premises }
    }

    /// Analyze claim under OLA 1957 (visitors)
    pub fn analyze_ola1957(&self, facts: &OLA1957Facts) -> Result<OLA1957Analysis, TortError> {
        // Check if defendant is an occupier
        let occupier_analysis = self.analyze_occupier_status(&facts.defendant);
        if !occupier_analysis.defendant_is_occupier {
            return Err(TortError::NotAnOccupier {
                reason: occupier_analysis.reasoning.clone(),
            });
        }

        // Check if claimant is a visitor
        let visitor_analysis = self.analyze_visitor_status(facts);
        if !visitor_analysis.is_visitor {
            return Err(TortError::NotAVisitor {
                status: format!("{:?}", visitor_analysis.status),
            });
        }

        // Analyze common duty of care
        let common_duty = self.analyze_common_duty(facts);

        // Check defences
        let defences = self.analyze_ola1957_defences(facts, &common_duty);

        // Determine liability
        let complete_defence = defences
            .iter()
            .any(|d| d.applies && matches!(d.effect, DefenceEffect::CompleteDefence));

        let duty_breached = common_duty.duty_breached;
        let liability_established = duty_breached && !complete_defence;

        let reasoning = if liability_established {
            "Liability established under s.2 OLA 1957".to_string()
        } else if !duty_breached {
            "No breach of common duty of care".to_string()
        } else {
            "Defence applies to exclude or reduce liability".to_string()
        };

        Ok(OLA1957Analysis {
            is_occupier: occupier_analysis,
            is_visitor: visitor_analysis,
            common_duty,
            defences,
            liability_established,
            reasoning,
        })
    }

    /// Analyze claim under OLA 1984 (non-visitors)
    pub fn analyze_ola1984(&self, facts: &OLA1984Facts) -> Result<OLA1984Analysis, TortError> {
        // Check if defendant is an occupier
        let occupier_analysis = self.analyze_occupier_status(&facts.defendant);
        if !occupier_analysis.defendant_is_occupier {
            return Err(TortError::NotAnOccupier {
                reason: occupier_analysis.reasoning.clone(),
            });
        }

        // Determine non-visitor type
        let non_visitor_type = self.determine_non_visitor_type(&facts.claimant_status);

        // Analyze s.1(3) conditions
        let section_1_3 = self.analyze_section_1_3(facts);
        if !section_1_3.all_conditions_met {
            return Ok(OLA1984Analysis {
                is_occupier: occupier_analysis,
                non_visitor_type,
                section_1_3,
                section_1_4: Section1_4Analysis {
                    took_reasonable_care: true,
                    warning: None,
                    warning_adequate: false,
                    risk_willingly_accepted: false,
                    duty_breached: false,
                    reasoning: "No duty arises as s.1(3) not satisfied".to_string(),
                },
                defences: vec![OLA1984Defence {
                    defence_type: OLA1984DefenceType::NoDutyArises,
                    applies: true,
                    effect: DefenceEffect::CompleteDefence,
                    reasoning: "s.1(3) conditions not met".to_string(),
                }],
                liability_established: false,
                reasoning: "No duty under OLA 1984 - s.1(3) conditions not satisfied".to_string(),
            });
        }

        // Analyze s.1(4) duty
        let section_1_4 = self.analyze_section_1_4(facts);

        // Check defences
        let defences = self.analyze_ola1984_defences(facts, &section_1_4);

        // Determine liability
        let complete_defence = defences
            .iter()
            .any(|d| d.applies && matches!(d.effect, DefenceEffect::CompleteDefence));

        let duty_breached = section_1_4.duty_breached;
        let liability_established = duty_breached && !complete_defence;

        let reasoning = if liability_established {
            "Liability established under OLA 1984".to_string()
        } else if !duty_breached {
            "Occupier took reasonable care under s.1(4)".to_string()
        } else {
            "Defence applies".to_string()
        };

        Ok(OLA1984Analysis {
            is_occupier: occupier_analysis,
            non_visitor_type,
            section_1_3,
            section_1_4,
            defences,
            liability_established,
            reasoning,
        })
    }

    fn analyze_occupier_status(&self, defendant: &TortParty) -> OccupierAnalysis {
        // Wheat v E Lacon - can be multiple occupiers with sufficient control
        let defendant_is_occupier = self
            .premises
            .occupiers
            .iter()
            .any(|o| o.name == defendant.name);

        OccupierAnalysis {
            occupiers: self.premises.occupiers.clone(),
            defendant_is_occupier,
            reasoning: if defendant_is_occupier {
                format!(
                    "{} is an occupier with {} control (Wheat v Lacon)",
                    defendant.name,
                    self.premises
                        .occupiers
                        .iter()
                        .find(|o| o.name == defendant.name)
                        .map(|o| format!("{:?}", o.control_degree))
                        .unwrap_or_else(|| "unknown".to_string())
                )
            } else {
                format!(
                    "{} does not have sufficient control to be an occupier",
                    defendant.name
                )
            },
        }
    }

    fn analyze_visitor_status(&self, facts: &OLA1957Facts) -> VisitorAnalysis {
        let is_visitor = match &facts.claimant_status {
            EntrantStatus::ExpressVisitor
            | EntrantStatus::ImpliedVisitor
            | EntrantStatus::ContractualEntrant
            | EntrantStatus::StatutoryEntrant => true,
            EntrantStatus::Child(age) => {
                // Children can be implied visitors (allurements)
                *age < 18 && facts.child_allurement.unwrap_or(false)
            }
            EntrantStatus::Trespasser
            | EntrantStatus::RightOfWay
            | EntrantStatus::CROWAccessRight => false,
        };

        let exceeded_permission = facts.exceeded_permission;

        VisitorAnalysis {
            status: facts.claimant_status.clone(),
            is_visitor: is_visitor && !exceeded_permission,
            exceeded_permission,
            reasoning: if exceeded_permission {
                "Claimant exceeded scope of permission - becomes trespasser".to_string()
            } else if is_visitor {
                format!(
                    "Claimant is a visitor under OLA 1957 ({:?})",
                    facts.claimant_status
                )
            } else {
                "Claimant is not a visitor - OLA 1984 applies".to_string()
            },
        }
    }

    fn analyze_common_duty(&self, facts: &OLA1957Facts) -> CommonDutyAnalysis {
        let took_reasonable_care = facts.reasonable_care_taken;

        // Child considerations
        let child_considerations = if let EntrantStatus::Child(age) = &facts.claimant_status {
            Some(ChildConsiderations {
                age: *age,
                allurement: facts.child_allurement.unwrap_or(false),
                allurement_description: facts.allurement_description.clone(),
                expected_less_careful: *age < 12,
                higher_duty: *age < 16,
                reasoning: format!(
                    "Child aged {} - occupier must be prepared for children to be less careful than adults (s.2(3)(a))",
                    age
                ),
            })
        } else {
            None
        };

        // Skilled visitor analysis
        let skilled_visitor = facts.visitor_skill.as_ref().map(|skill| {
            let danger_within_competence = facts.danger_within_skill.unwrap_or(false);
            SkilledVisitorAnalysis {
                skill: skill.clone(),
                danger_within_competence,
                expected_to_guard: danger_within_competence,
                reduced_duty: danger_within_competence,
                reasoning: if danger_within_competence {
                    format!(
                        "Danger within {} competence - occupier may expect them to appreciate and guard against it (s.2(3)(b), Roles v Nathan)",
                        skill
                    )
                } else {
                    format!(
                        "Danger outside {} professional competence - normal duty applies",
                        skill
                    )
                },
            }
        });

        // Warning analysis
        let warning_analysis = facts.warning.as_ref().map(|w| {
            let enables_safety = w.adequate && w.visible && w.comprehensible;
            WarningAnalysis {
                warning: w.clone(),
                enables_reasonable_safety: enables_safety,
                discharges_duty: enables_safety && facts.warning_sufficient_alone.unwrap_or(false),
                reasoning: if enables_safety {
                    "Warning adequate to enable visitor to be reasonably safe (s.2(4)(a))"
                        .to_string()
                } else {
                    "Warning not adequate to discharge duty".to_string()
                },
            }
        });

        // Independent contractor analysis
        let independent_contractor = if facts.work_by_contractor {
            Some(IndependentContractorAnalysis {
                construction_maintenance_repair: facts.cmr_work.unwrap_or(false),
                reasonable_to_entrust: facts.reasonable_to_entrust.unwrap_or(true),
                checked_competence: facts.checked_contractor_competence.unwrap_or(false),
                checked_work: facts.checked_contractor_work.unwrap_or(false),
                occupier_absolved: facts.cmr_work.unwrap_or(false)
                    && facts.reasonable_to_entrust.unwrap_or(true)
                    && facts.checked_contractor_competence.unwrap_or(false)
                    && facts.checked_contractor_work.unwrap_or(false),
                reasoning: "s.2(4)(b) analysis for independent contractor work".to_string(),
            })
        } else {
            None
        };

        // Determine if duty breached
        let duty_breached = !took_reasonable_care
            && !skilled_visitor
                .as_ref()
                .is_some_and(|sv| sv.reduced_duty && sv.danger_within_competence)
            && !warning_analysis
                .as_ref()
                .is_some_and(|wa| wa.discharges_duty)
            && !independent_contractor
                .as_ref()
                .is_some_and(|ic| ic.occupier_absolved);

        CommonDutyAnalysis {
            danger: facts.danger.clone(),
            took_reasonable_care,
            child_considerations,
            skilled_visitor,
            warning_analysis,
            independent_contractor,
            duty_breached,
            reasoning: if duty_breached {
                "Common duty of care under s.2(2) breached".to_string()
            } else {
                "Occupier satisfied common duty of care".to_string()
            },
        }
    }

    fn analyze_ola1957_defences(
        &self,
        facts: &OLA1957Facts,
        common_duty: &CommonDutyAnalysis,
    ) -> Vec<OLADefence> {
        let mut defences = Vec::new();

        // Volenti (s.2(5))
        if let Some(vf) = &facts.volenti_facts {
            let applies = vf.knowledge && vf.consent && vf.freely_given;
            defences.push(OLADefence {
                defence_type: OLADefenceType::Volenti,
                applies,
                effect: if applies {
                    DefenceEffect::CompleteDefence
                } else {
                    DefenceEffect::NoEffect
                },
                reasoning: if applies {
                    "Visitor willingly accepted risk (s.2(5))".to_string()
                } else {
                    "Volenti not established".to_string()
                },
            });
        }

        // Contributory negligence
        if let Some(ref cn) = facts.contributory_negligence {
            let applies = cn.failed_reasonable_care && cn.contributed_to_damage;
            defences.push(OLADefence {
                defence_type: OLADefenceType::ContributoryNegligence,
                applies,
                effect: if applies {
                    DefenceEffect::ReducesDamages(f64::from(cn.reduction_percentage) / 100.0)
                } else {
                    DefenceEffect::NoEffect
                },
                reasoning: format!(
                    "Contributory negligence: {}% reduction",
                    cn.reduction_percentage
                ),
            });
        }

        // Adequate warning
        if let Some(ref warning) = common_duty.warning_analysis
            && warning.discharges_duty
        {
            defences.push(OLADefence {
                defence_type: OLADefenceType::AdequateWarning,
                applies: true,
                effect: DefenceEffect::CompleteDefence,
                reasoning: "Adequate warning discharged duty (s.2(4)(a))".to_string(),
            });
        }

        // Exclusion of liability
        if let Some(ref exclusion) = facts.exclusion_clause {
            let applies = exclusion.valid_under_ucta;
            // UCTA s.2(1) - cannot exclude liability for personal injury
            let effect = if applies && !exclusion.personal_injury {
                DefenceEffect::CompleteDefence
            } else {
                DefenceEffect::NoEffect
            };
            defences.push(OLADefence {
                defence_type: OLADefenceType::ExclusionOfLiability,
                applies,
                effect,
                reasoning: if exclusion.personal_injury {
                    "Cannot exclude liability for personal injury (UCTA 1977 s.2(1))".to_string()
                } else if applies {
                    "Exclusion clause valid under UCTA 1977".to_string()
                } else {
                    "Exclusion clause invalid under UCTA 1977".to_string()
                },
            });
        }

        defences
    }

    fn determine_non_visitor_type(&self, status: &EntrantStatus) -> NonVisitorType {
        match status {
            EntrantStatus::Trespasser => NonVisitorType::Trespasser,
            EntrantStatus::RightOfWay => NonVisitorType::RightOfWay,
            EntrantStatus::CROWAccessRight => NonVisitorType::CROWAccess,
            _ => NonVisitorType::ExceededPermission,
        }
    }

    fn analyze_section_1_3(&self, facts: &OLA1984Facts) -> Section1_3Analysis {
        // s.1(3)(a) - awareness of danger
        let aware = AwarenessAnalysis {
            actual_knowledge: facts.occupier_knew_of_danger,
            constructive_knowledge: facts.occupier_should_have_known,
            satisfied: facts.occupier_knew_of_danger || facts.occupier_should_have_known,
            reasoning: if facts.occupier_knew_of_danger {
                "Occupier had actual knowledge of danger".to_string()
            } else if facts.occupier_should_have_known {
                "Occupier had reasonable grounds to believe danger existed".to_string()
            } else {
                "Occupier neither knew nor had grounds to believe danger existed".to_string()
            },
        };

        // s.1(3)(b) - knowledge of presence
        let presence = PresenceKnowledgeAnalysis {
            actual_knowledge: facts.occupier_knew_of_presence,
            constructive_knowledge: facts.occupier_should_have_known_presence,
            history_of_trespassers: facts.history_of_trespassers,
            satisfied: facts.occupier_knew_of_presence
                || facts.occupier_should_have_known_presence
                || facts.history_of_trespassers,
            reasoning: if facts.occupier_knew_of_presence {
                "Occupier knew claimant was in vicinity".to_string()
            } else if facts.history_of_trespassers {
                "History of trespassers provides constructive knowledge".to_string()
            } else {
                "No grounds to believe anyone in vicinity".to_string()
            },
        };

        // s.1(3)(c) - reasonable protection expected
        let protection = self.analyze_reasonable_protection(facts);

        let all_met = aware.satisfied && presence.satisfied && protection.protection_expected;

        Section1_3Analysis {
            aware_of_danger: aware,
            knows_other_in_vicinity: presence,
            reasonable_protection_expected: protection,
            all_conditions_met: all_met,
        }
    }

    fn analyze_reasonable_protection(&self, facts: &OLA1984Facts) -> ReasonableProtectionAnalysis {
        // Tomlinson v Congleton - obvious risks may not require protection
        let obvious = facts.danger.obvious;
        let cost = match facts.cost_of_protection {
            CostOfProtection::Minimal => true,
            CostOfProtection::Moderate => !obvious,
            CostOfProtection::Significant => {
                !obvious && matches!(facts.danger.risk_level, RiskSeverity::Severe)
            }
            CostOfProtection::Prohibitive => false,
        };

        let social_utility = facts.premises_has_social_utility;

        let protection_expected = !obvious && cost && !social_utility;

        ReasonableProtectionAnalysis {
            risk_severity: facts.danger.risk_level.clone(),
            cost_of_protection: facts.cost_of_protection.clone(),
            social_utility,
            protection_expected,
            reasoning: if obvious {
                "Risk was obvious - no protection expected (Tomlinson v Congleton)".to_string()
            } else if !cost {
                "Cost of protection disproportionate to risk".to_string()
            } else if social_utility {
                "Activity has social utility - limits expected protection".to_string()
            } else {
                "Protection reasonably expected in circumstances".to_string()
            },
        }
    }

    fn analyze_section_1_4(&self, facts: &OLA1984Facts) -> Section1_4Analysis {
        let warning_adequate = facts.warning.as_ref().is_some_and(|w| w.adequate);
        let risk_willingly_accepted = facts.risk_willingly_accepted;

        let took_reasonable_care = facts.took_reasonable_care
            || warning_adequate
            || risk_willingly_accepted
            || facts.danger.obvious;

        Section1_4Analysis {
            took_reasonable_care,
            warning: facts.warning.clone(),
            warning_adequate,
            risk_willingly_accepted,
            duty_breached: !took_reasonable_care,
            reasoning: if took_reasonable_care {
                "Occupier took reasonable care in all circumstances".to_string()
            } else {
                "Occupier failed to take reasonable care under s.1(4)".to_string()
            },
        }
    }

    fn analyze_ola1984_defences(
        &self,
        facts: &OLA1984Facts,
        section_1_4: &Section1_4Analysis,
    ) -> Vec<OLA1984Defence> {
        let mut defences = Vec::new();

        // s.1(5) - obvious risk
        if facts.danger.obvious {
            defences.push(OLA1984Defence {
                defence_type: OLA1984DefenceType::ObviousRisk,
                applies: true,
                effect: DefenceEffect::CompleteDefence,
                reasoning: format!(
                    "Risk was obvious: {} (s.1(5), Tomlinson v Congleton)",
                    facts.danger.description
                ),
            });
        }

        // s.1(6) - willing acceptance
        if section_1_4.risk_willingly_accepted {
            defences.push(OLA1984Defence {
                defence_type: OLA1984DefenceType::WillingAcceptance,
                applies: true,
                effect: DefenceEffect::CompleteDefence,
                reasoning: "Claimant willingly accepted the risk (s.1(6))".to_string(),
            });
        }

        // Contributory negligence
        if let Some(ref cn) = facts.contributory_negligence {
            let applies = cn.failed_reasonable_care && cn.contributed_to_damage;
            defences.push(OLA1984Defence {
                defence_type: OLA1984DefenceType::ContributoryNegligence,
                applies,
                effect: if applies {
                    DefenceEffect::ReducesDamages(f64::from(cn.reduction_percentage) / 100.0)
                } else {
                    DefenceEffect::NoEffect
                },
                reasoning: format!(
                    "Contributory negligence: {}% reduction",
                    cn.reduction_percentage
                ),
            });
        }

        defences
    }
}

/// Facts for OLA 1957 analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OLA1957Facts {
    /// Defendant details
    pub defendant: TortParty,
    /// Claimant details
    pub claimant: TortParty,
    /// Claimant's status on premises
    pub claimant_status: EntrantStatus,
    /// Did claimant exceed permission?
    pub exceeded_permission: bool,
    /// The danger that caused harm
    pub danger: PremisesDanger,
    /// Did occupier take reasonable care?
    pub reasonable_care_taken: bool,
    /// Warning given
    pub warning: Option<Warning>,
    /// Was warning alone sufficient?
    pub warning_sufficient_alone: Option<bool>,
    /// Child allurement?
    pub child_allurement: Option<bool>,
    /// Description of allurement
    pub allurement_description: Option<String>,
    /// Visitor's special skill
    pub visitor_skill: Option<String>,
    /// Was danger within that skill?
    pub danger_within_skill: Option<bool>,
    /// Work done by independent contractor?
    pub work_by_contractor: bool,
    /// Construction, maintenance or repair work?
    pub cmr_work: Option<bool>,
    /// Reasonable to entrust to contractor?
    pub reasonable_to_entrust: Option<bool>,
    /// Checked contractor competence?
    pub checked_contractor_competence: Option<bool>,
    /// Checked contractor work?
    pub checked_contractor_work: Option<bool>,
    /// Volenti facts
    pub volenti_facts: Option<VolentiFacts1957>,
    /// Contributory negligence
    pub contributory_negligence: Option<ContributoryNegligence>,
    /// Exclusion clause
    pub exclusion_clause: Option<ExclusionClause>,
}

/// Volenti facts for OLA 1957
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolentiFacts1957 {
    /// Knowledge of risk
    pub knowledge: bool,
    /// Consent to risk
    pub consent: bool,
    /// Freely given
    pub freely_given: bool,
}

/// Exclusion clause analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExclusionClause {
    /// Text of clause
    pub text: String,
    /// Valid under UCTA 1977?
    pub valid_under_ucta: bool,
    /// Is claim for personal injury?
    pub personal_injury: bool,
}

/// Facts for OLA 1984 analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OLA1984Facts {
    /// Defendant details
    pub defendant: TortParty,
    /// Claimant details
    pub claimant: TortParty,
    /// Claimant's status
    pub claimant_status: EntrantStatus,
    /// The danger
    pub danger: PremisesDanger,
    /// Did occupier know of danger?
    pub occupier_knew_of_danger: bool,
    /// Should occupier have known?
    pub occupier_should_have_known: bool,
    /// Did occupier know claimant in vicinity?
    pub occupier_knew_of_presence: bool,
    /// Should have known of presence?
    pub occupier_should_have_known_presence: bool,
    /// History of trespassers?
    pub history_of_trespassers: bool,
    /// Cost of providing protection
    pub cost_of_protection: CostOfProtection,
    /// Does premises/activity have social utility?
    pub premises_has_social_utility: bool,
    /// Did occupier take reasonable care?
    pub took_reasonable_care: bool,
    /// Warning given
    pub warning: Option<Warning>,
    /// Was risk willingly accepted?
    pub risk_willingly_accepted: bool,
    /// Contributory negligence
    pub contributory_negligence: Option<ContributoryNegligence>,
}

#[cfg(test)]
mod tests {
    use super::super::types::PartyRole;
    use super::*;

    fn create_test_occupier() -> Occupier {
        Occupier {
            name: "ABC Ltd".to_string(),
            party_type: PartyType::Company,
            occupation_basis: OccupationBasis::LeaseholdTenant,
            control_degree: ControlDegree::Full,
        }
    }

    fn create_test_defendant() -> TortParty {
        TortParty {
            name: "ABC Ltd".to_string(),
            role: PartyRole::Defendant,
            party_type: PartyType::Company,
            vulnerable: false,
            professional_capacity: None,
        }
    }

    fn create_test_claimant() -> TortParty {
        TortParty {
            name: "John Smith".to_string(),
            role: PartyRole::Claimant,
            party_type: PartyType::Individual,
            vulnerable: false,
            professional_capacity: None,
        }
    }

    #[test]
    fn test_ola1957_visitor_slip() {
        let premises = PremisesInfo {
            premises_type: PremisesType::Commercial,
            description: "Supermarket".to_string(),
            occupiers: vec![create_test_occupier()],
        };

        let analyzer = OccupiersLiabilityAnalyzer::new(premises);

        let facts = OLA1957Facts {
            defendant: create_test_defendant(),
            claimant: create_test_claimant(),
            claimant_status: EntrantStatus::ImpliedVisitor,
            exceeded_permission: false,
            danger: PremisesDanger {
                description: "Wet floor".to_string(),
                danger_type: DangerType::SlipperySurface,
                obvious: false,
                concealed: true,
                risk_level: RiskSeverity::Moderate,
                warning: None,
                longstanding: true,
                known_to_occupier: true,
            },
            reasonable_care_taken: false,
            warning: None,
            warning_sufficient_alone: None,
            child_allurement: None,
            allurement_description: None,
            visitor_skill: None,
            danger_within_skill: None,
            work_by_contractor: false,
            cmr_work: None,
            reasonable_to_entrust: None,
            checked_contractor_competence: None,
            checked_contractor_work: None,
            volenti_facts: None,
            contributory_negligence: None,
            exclusion_clause: None,
        };

        let result = analyzer.analyze_ola1957(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.liability_established);
        assert!(analysis.common_duty.duty_breached);
    }

    #[test]
    fn test_ola1957_adequate_warning() {
        let premises = PremisesInfo {
            premises_type: PremisesType::Commercial,
            description: "Shop".to_string(),
            occupiers: vec![create_test_occupier()],
        };

        let analyzer = OccupiersLiabilityAnalyzer::new(premises);

        let warning = Warning {
            description: "Caution: Wet Floor".to_string(),
            warning_type: WarningType::WrittenSign,
            adequate: true,
            visible: true,
            comprehensible: true,
        };

        let facts = OLA1957Facts {
            defendant: create_test_defendant(),
            claimant: create_test_claimant(),
            claimant_status: EntrantStatus::ImpliedVisitor,
            exceeded_permission: false,
            danger: PremisesDanger {
                description: "Wet floor".to_string(),
                danger_type: DangerType::SlipperySurface,
                obvious: false,
                concealed: false,
                risk_level: RiskSeverity::Moderate,
                warning: Some(warning.clone()),
                longstanding: false,
                known_to_occupier: true,
            },
            reasonable_care_taken: true,
            warning: Some(warning),
            warning_sufficient_alone: Some(true),
            child_allurement: None,
            allurement_description: None,
            visitor_skill: None,
            danger_within_skill: None,
            work_by_contractor: false,
            cmr_work: None,
            reasonable_to_entrust: None,
            checked_contractor_competence: None,
            checked_contractor_work: None,
            volenti_facts: None,
            contributory_negligence: None,
            exclusion_clause: None,
        };

        let result = analyzer.analyze_ola1957(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.liability_established);
        assert!(analysis.common_duty.warning_analysis.is_some());
    }

    #[test]
    fn test_ola1957_skilled_visitor_roles_v_nathan() {
        let premises = PremisesInfo {
            premises_type: PremisesType::Commercial,
            description: "Building with boiler".to_string(),
            occupiers: vec![create_test_occupier()],
        };

        let analyzer = OccupiersLiabilityAnalyzer::new(premises);

        let facts = OLA1957Facts {
            defendant: create_test_defendant(),
            claimant: create_test_claimant(),
            claimant_status: EntrantStatus::ContractualEntrant,
            exceeded_permission: false,
            danger: PremisesDanger {
                description: "Carbon monoxide from boiler".to_string(),
                danger_type: DangerType::HazardousSubstance,
                obvious: false,
                concealed: false,
                risk_level: RiskSeverity::Severe,
                warning: None,
                longstanding: false,
                known_to_occupier: true,
            },
            reasonable_care_taken: true,
            warning: None,
            warning_sufficient_alone: None,
            child_allurement: None,
            allurement_description: None,
            visitor_skill: Some("chimney sweep".to_string()),
            danger_within_skill: Some(true), // Danger within their expertise
            work_by_contractor: false,
            cmr_work: None,
            reasonable_to_entrust: None,
            checked_contractor_competence: None,
            checked_contractor_work: None,
            volenti_facts: None,
            contributory_negligence: None,
            exclusion_clause: None,
        };

        let result = analyzer.analyze_ola1957(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // Skilled visitor expected to guard against risks within their competence
        assert!(!analysis.liability_established);
        assert!(analysis.common_duty.skilled_visitor.is_some());
        assert!(
            analysis
                .common_duty
                .skilled_visitor
                .as_ref()
                .expect("exists")
                .reduced_duty
        );
    }

    #[test]
    fn test_ola1957_child_allurement() {
        let premises = PremisesInfo {
            premises_type: PremisesType::Residential,
            description: "House with pond".to_string(),
            occupiers: vec![create_test_occupier()],
        };

        let analyzer = OccupiersLiabilityAnalyzer::new(premises);

        let facts = OLA1957Facts {
            defendant: create_test_defendant(),
            claimant: create_test_claimant(),
            claimant_status: EntrantStatus::Child(6),
            exceeded_permission: false,
            danger: PremisesDanger {
                description: "Garden pond".to_string(),
                danger_type: DangerType::WaterHazard,
                obvious: false,
                concealed: false,
                risk_level: RiskSeverity::Serious,
                warning: None,
                longstanding: true,
                known_to_occupier: true,
            },
            reasonable_care_taken: false,
            warning: None,
            warning_sufficient_alone: None,
            child_allurement: Some(true),
            allurement_description: Some("Attractive pond with fish".to_string()),
            visitor_skill: None,
            danger_within_skill: None,
            work_by_contractor: false,
            cmr_work: None,
            reasonable_to_entrust: None,
            checked_contractor_competence: None,
            checked_contractor_work: None,
            volenti_facts: None,
            contributory_negligence: None,
            exclusion_clause: None,
        };

        let result = analyzer.analyze_ola1957(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.liability_established);
        assert!(analysis.common_duty.child_considerations.is_some());
        assert!(
            analysis
                .common_duty
                .child_considerations
                .as_ref()
                .expect("exists")
                .higher_duty
        );
    }

    #[test]
    fn test_ola1984_trespasser_obvious_risk() {
        let premises = PremisesInfo {
            premises_type: PremisesType::PublicOpenSpace,
            description: "Lake".to_string(),
            occupiers: vec![create_test_occupier()],
        };

        let analyzer = OccupiersLiabilityAnalyzer::new(premises);

        let facts = OLA1984Facts {
            defendant: create_test_defendant(),
            claimant: create_test_claimant(),
            claimant_status: EntrantStatus::Trespasser,
            danger: PremisesDanger {
                description: "Deep water in lake".to_string(),
                danger_type: DangerType::WaterHazard,
                obvious: true, // Tomlinson v Congleton
                concealed: false,
                risk_level: RiskSeverity::Severe,
                warning: None,
                longstanding: true,
                known_to_occupier: true,
            },
            occupier_knew_of_danger: true,
            occupier_should_have_known: true,
            occupier_knew_of_presence: false,
            occupier_should_have_known_presence: true,
            history_of_trespassers: true,
            cost_of_protection: CostOfProtection::Significant,
            premises_has_social_utility: true,
            took_reasonable_care: true,
            warning: None,
            risk_willingly_accepted: false,
            contributory_negligence: None,
        };

        let result = analyzer.analyze_ola1984(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.liability_established);
        // When risk is obvious, s.1(3)(c) protection_expected is false, so NoDutyArises applies
        // This reflects Tomlinson v Congleton - no duty for obvious risks
        assert!(
            analysis
                .defences
                .iter()
                .any(|d| matches!(d.defence_type, OLA1984DefenceType::NoDutyArises) && d.applies)
        );
    }

    #[test]
    fn test_ola1984_section_1_3_not_met() {
        let premises = PremisesInfo {
            premises_type: PremisesType::Agricultural,
            description: "Remote farmland".to_string(),
            occupiers: vec![create_test_occupier()],
        };

        let analyzer = OccupiersLiabilityAnalyzer::new(premises);

        let facts = OLA1984Facts {
            defendant: create_test_defendant(),
            claimant: create_test_claimant(),
            claimant_status: EntrantStatus::Trespasser,
            danger: PremisesDanger {
                description: "Concealed well".to_string(),
                danger_type: DangerType::FallingRisk,
                obvious: false,
                concealed: true,
                risk_level: RiskSeverity::Severe,
                warning: None,
                longstanding: true,
                known_to_occupier: false, // Didn't know
            },
            occupier_knew_of_danger: false,
            occupier_should_have_known: false, // No reasonable grounds
            occupier_knew_of_presence: false,
            occupier_should_have_known_presence: false,
            history_of_trespassers: false, // No history
            cost_of_protection: CostOfProtection::Moderate,
            premises_has_social_utility: false,
            took_reasonable_care: true,
            warning: None,
            risk_willingly_accepted: false,
            contributory_negligence: None,
        };

        let result = analyzer.analyze_ola1984(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.liability_established);
        assert!(!analysis.section_1_3.all_conditions_met);
    }

    #[test]
    fn test_ola1984_duty_breached() {
        let premises = PremisesInfo {
            premises_type: PremisesType::Industrial,
            description: "Factory site".to_string(),
            occupiers: vec![create_test_occupier()],
        };

        let analyzer = OccupiersLiabilityAnalyzer::new(premises);

        let facts = OLA1984Facts {
            defendant: create_test_defendant(),
            claimant: create_test_claimant(),
            claimant_status: EntrantStatus::Trespasser,
            danger: PremisesDanger {
                description: "Unfenced machinery".to_string(),
                danger_type: DangerType::Machinery,
                obvious: false,
                concealed: true,
                risk_level: RiskSeverity::Severe,
                warning: None,
                longstanding: true,
                known_to_occupier: true,
            },
            occupier_knew_of_danger: true,
            occupier_should_have_known: true,
            occupier_knew_of_presence: true,
            occupier_should_have_known_presence: true,
            history_of_trespassers: true,
            cost_of_protection: CostOfProtection::Minimal, // Easy to fix
            premises_has_social_utility: false,
            took_reasonable_care: false, // Did nothing
            warning: None,
            risk_willingly_accepted: false,
            contributory_negligence: None,
        };

        let result = analyzer.analyze_ola1984(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.liability_established);
        assert!(analysis.section_1_3.all_conditions_met);
        assert!(analysis.section_1_4.duty_breached);
    }

    #[test]
    fn test_ola1957_not_a_visitor() {
        let premises = PremisesInfo {
            premises_type: PremisesType::Commercial,
            description: "Shop".to_string(),
            occupiers: vec![create_test_occupier()],
        };

        let analyzer = OccupiersLiabilityAnalyzer::new(premises);

        let facts = OLA1957Facts {
            defendant: create_test_defendant(),
            claimant: create_test_claimant(),
            claimant_status: EntrantStatus::Trespasser, // Trespasser
            exceeded_permission: false,
            danger: PremisesDanger {
                description: "Hazard".to_string(),
                danger_type: DangerType::Other("test".to_string()),
                obvious: false,
                concealed: false,
                risk_level: RiskSeverity::Moderate,
                warning: None,
                longstanding: false,
                known_to_occupier: true,
            },
            reasonable_care_taken: false,
            warning: None,
            warning_sufficient_alone: None,
            child_allurement: None,
            allurement_description: None,
            visitor_skill: None,
            danger_within_skill: None,
            work_by_contractor: false,
            cmr_work: None,
            reasonable_to_entrust: None,
            checked_contractor_competence: None,
            checked_contractor_work: None,
            volenti_facts: None,
            contributory_negligence: None,
            exclusion_clause: None,
        };

        let result = analyzer.analyze_ola1957(&facts);
        assert!(result.is_err());
        assert!(matches!(result, Err(TortError::NotAVisitor { .. })));
    }
}
