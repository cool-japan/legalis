//! UK Nuisance Law
//!
//! This module implements the law of nuisance including:
//! - Private nuisance (Hunter v Canary Wharf [1997] AC 655)
//! - Public nuisance (AG v PYA Quarries [1957] 2 QB 169)
//! - Rylands v Fletcher (1868) LR 3 HL 330 (strict liability)
//!
//! Key cases:
//! - Sedleigh-Denfield v O'Callaghan [1940] AC 880 (adoption/continuation)
//! - Transco v Stockport [2004] 2 AC 1 (modern Rylands)

use serde::{Deserialize, Serialize};

use super::error::TortError;

// ============================================================================
// Core Types for Nuisance
// ============================================================================

/// Type of nuisance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NuisanceType {
    /// Private nuisance - unlawful interference with land
    Private,
    /// Public nuisance - interference with public rights
    Public,
    /// Rylands v Fletcher - escape of dangerous things
    RylandsVFletcher,
}

/// Interest in land for standing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandInterest {
    /// Freehold owner
    FreeholdOwner,
    /// Leasehold tenant
    LeaseholdTenant,
    /// Licensee with exclusive possession
    ExclusiveLicensee,
    /// Licensee without exclusive possession
    NonExclusiveLicensee,
    /// Reversioner
    Reversioner,
    /// Mere occupier (no proprietary interest)
    MereOccupier,
    /// Family member of owner/tenant
    FamilyMember,
}

/// Type of interference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterferenceType {
    /// Physical damage to property
    PhysicalDamage,
    /// Encroachment onto land
    Encroachment,
    /// Interference with comfort/amenity
    AmenityInterference,
    /// Noise
    Noise,
    /// Smell/odour
    Smell,
    /// Vibration
    Vibration,
    /// Dust/particles
    Dust,
    /// Smoke
    Smoke,
    /// Light (artificial)
    ArtificialLight,
    /// Blocking natural light
    BlockingLight,
    /// Flooding/water
    Water,
    /// Heat
    Heat,
    /// Other
    Other(String),
}

/// Duration of interference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterferenceDuration {
    /// One-time event
    SingleEvent,
    /// Temporary/short term
    Temporary,
    /// Recurring/regular
    Recurring,
    /// Continuous/ongoing
    Continuous,
    /// Permanent
    Permanent,
}

/// Severity of interference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterferenceSeverity {
    /// Trivial
    Trivial,
    /// Minor
    Minor,
    /// Moderate
    Moderate,
    /// Substantial
    Substantial,
    /// Severe
    Severe,
}

/// Character of locality
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalityCharacter {
    /// Residential area
    Residential,
    /// Mixed residential/commercial
    MixedUse,
    /// Commercial/business district
    Commercial,
    /// Industrial area
    Industrial,
    /// Rural/agricultural
    Rural,
    /// Urban centre
    UrbanCentre,
}

// ============================================================================
// Private Nuisance Analysis
// ============================================================================

/// Private nuisance claim analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateNuisanceAnalysis {
    /// Standing analysis (Hunter v Canary Wharf)
    pub standing: StandingAnalysis,
    /// Interference analysis
    pub interference: InterferenceAnalysis,
    /// Reasonableness analysis
    pub reasonableness: ReasonablenessAnalysis,
    /// Defendant liability
    pub defendant_liability: DefendantLiabilityAnalysis,
    /// Defences
    pub defences: Vec<NuisanceDefence>,
    /// Remedies
    pub remedies: Vec<NuisanceRemedy>,
    /// Claim succeeds?
    pub claim_succeeds: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Standing analysis for private nuisance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandingAnalysis {
    /// Claimant's interest in land
    pub interest: LandInterest,
    /// Does claimant have standing?
    pub has_standing: bool,
    /// Reasoning (Hunter v Canary Wharf)
    pub reasoning: String,
}

/// Interference analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterferenceAnalysis {
    /// Type of interference
    pub interference_type: InterferenceType,
    /// Duration
    pub duration: InterferenceDuration,
    /// Severity
    pub severity: InterferenceSeverity,
    /// Description
    pub description: String,
    /// Is there actionable interference?
    pub actionable: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Reasonableness analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReasonablenessAnalysis {
    /// Character of locality
    pub locality: LocalityCharacter,
    /// Duration and frequency
    pub duration: InterferenceDuration,
    /// Time of day/night
    pub time_sensitivity: TimeSensitivity,
    /// Sensitivity of claimant
    pub claimant_sensitivity: SensitivityAnalysis,
    /// Utility of defendant's conduct
    pub defendant_utility: DefendantUtility,
    /// Malice present?
    pub malice: Option<MaliceAnalysis>,
    /// Is use of land unreasonable?
    pub unreasonable: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Time sensitivity of interference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeSensitivity {
    /// Daytime only
    DaytimeOnly,
    /// Evening/night
    NightTime,
    /// 24 hours
    Continuous,
    /// Irregular/unpredictable
    Irregular,
}

/// Analysis of claimant sensitivity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensitivityAnalysis {
    /// Is claimant or their use abnormally sensitive?
    pub abnormally_sensitive: bool,
    /// Would ordinary person be affected?
    pub ordinary_person_affected: bool,
    /// Description of sensitivity
    pub description: String,
    /// Effect on claim (Robinson v Kilvert)
    pub effect: SensitivityEffect,
}

/// Effect of sensitivity on claim
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensitivityEffect {
    /// No effect - ordinary sensitivity
    NoEffect,
    /// Claim fails due to abnormal sensitivity
    ClaimFails,
    /// Damages limited to what ordinary person would suffer
    DamagesLimited,
}

/// Defendant's utility in their conduct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefendantUtility {
    /// Nature of defendant's activity
    pub activity: String,
    /// Public benefit?
    pub public_benefit: bool,
    /// Economic necessity?
    pub economic_necessity: bool,
    /// Level of utility
    pub utility_level: UtilityLevel,
}

/// Level of utility
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UtilityLevel {
    /// No utility/gratuitous
    None,
    /// Low utility
    Low,
    /// Moderate utility
    Moderate,
    /// High utility
    High,
    /// Essential/vital
    Essential,
}

/// Analysis of malice
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaliceAnalysis {
    /// Evidence of malice
    pub malice_present: bool,
    /// Description
    pub description: String,
    /// Effect on liability (Christie v Davey, Hollywood Silver Fox Farm)
    pub effect: String,
}

/// Analysis of defendant liability
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefendantLiabilityAnalysis {
    /// Defendant's role
    pub defendant_role: DefendantRole,
    /// Is defendant liable?
    pub liable: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Role of defendant in nuisance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefendantRole {
    /// Creator of nuisance
    Creator,
    /// Occupier who adopted nuisance (Sedleigh-Denfield)
    Adopter,
    /// Occupier who continued nuisance
    Continuer,
    /// Landlord (limited liability)
    Landlord,
    /// Not responsible
    NotResponsible,
}

/// Defence to nuisance claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NuisanceDefence {
    /// Type of defence
    pub defence_type: NuisanceDefenceType,
    /// Does defence apply?
    pub applies: bool,
    /// Effect
    pub effect: String,
    /// Reasoning
    pub reasoning: String,
}

/// Types of defence to nuisance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NuisanceDefenceType {
    /// Statutory authority
    StatutoryAuthority,
    /// Prescription (20 years)
    Prescription,
    /// Consent
    Consent,
    /// Contributory negligence
    ContributoryNegligence,
    /// Act of God
    ActOfGod,
    /// Act of stranger
    ActOfStranger,
    /// Necessity
    Necessity,
    /// Came to nuisance (NOT a defence but often raised)
    CameToNuisance,
}

/// Remedy for nuisance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NuisanceRemedy {
    /// Type of remedy
    pub remedy_type: NuisanceRemedyType,
    /// Appropriate?
    pub appropriate: bool,
    /// Quantum (for damages)
    pub quantum: Option<f64>,
    /// Reasoning
    pub reasoning: String,
}

/// Types of remedy for nuisance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NuisanceRemedyType {
    /// Damages
    Damages,
    /// Injunction (prohibitory)
    ProhibitoryInjunction,
    /// Injunction (mandatory)
    MandatoryInjunction,
    /// Damages in lieu of injunction (Shelfer v City of London)
    DamagesInLieu,
    /// Abatement (self-help)
    Abatement,
}

// ============================================================================
// Public Nuisance Analysis
// ============================================================================

/// Public nuisance analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicNuisanceAnalysis {
    /// Is this a public nuisance?
    pub is_public_nuisance: PublicNuisanceTest,
    /// Special damage (for civil claim)
    pub special_damage: SpecialDamageAnalysis,
    /// Defendant liable?
    pub defendant_liable: bool,
    /// Claim succeeds?
    pub claim_succeeds: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Test for public nuisance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicNuisanceTest {
    /// Does it affect a class of the public?
    pub affects_class: bool,
    /// Description of class affected
    pub class_description: String,
    /// Is class sufficiently large? (AG v PYA Quarries)
    pub class_large_enough: bool,
    /// Interference with public rights?
    pub interferes_with_public_rights: bool,
    /// Nature of public right affected
    pub public_right: PublicRight,
    /// Is it a public nuisance?
    pub is_public_nuisance: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of public right affected
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicRight {
    /// Highway use
    Highway,
    /// Navigation
    Navigation,
    /// Public health
    PublicHealth,
    /// Public safety
    PublicSafety,
    /// Environmental
    Environmental,
    /// Other public right
    Other(String),
}

/// Special damage analysis for civil claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialDamageAnalysis {
    /// Has claimant suffered special damage?
    pub suffered_special_damage: bool,
    /// Is damage different in kind?
    pub different_in_kind: bool,
    /// Is damage greater in degree?
    pub greater_in_degree: bool,
    /// Description of special damage
    pub description: String,
    /// Sufficient for civil claim?
    pub sufficient: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Rylands v Fletcher Analysis
// ============================================================================

/// Rylands v Fletcher analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RylandsVFletcherAnalysis {
    /// Standing
    pub standing: RylandsStanding,
    /// Thing brought onto land
    pub thing_brought: ThingBroughtAnalysis,
    /// Non-natural use
    pub non_natural_use: NonNaturalUseAnalysis,
    /// Escape
    pub escape: EscapeAnalysis,
    /// Mischief/damage
    pub mischief: MischiefAnalysis,
    /// Defences
    pub defences: Vec<RylandsDefence>,
    /// Claim succeeds?
    pub claim_succeeds: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Standing for Rylands v Fletcher
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RylandsStanding {
    /// Claimant's interest
    pub interest: LandInterest,
    /// Does claimant have standing?
    pub has_standing: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Analysis of thing brought onto land
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThingBroughtAnalysis {
    /// Description of thing
    pub description: String,
    /// Type of thing
    pub thing_type: DangerousThing,
    /// Was it brought onto land?
    pub brought_onto_land: bool,
    /// Likely to cause mischief if escapes?
    pub likely_to_cause_mischief: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of dangerous thing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DangerousThing {
    /// Water (large quantity)
    Water,
    /// Fire
    Fire,
    /// Gas
    Gas,
    /// Electricity
    Electricity,
    /// Chemicals
    Chemicals,
    /// Explosives
    Explosives,
    /// Sewage
    Sewage,
    /// Oil/petroleum
    Oil,
    /// Other
    Other(String),
}

/// Non-natural use analysis (Transco v Stockport)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NonNaturalUseAnalysis {
    /// Description of use
    pub use_description: String,
    /// Is it a non-natural use?
    pub non_natural: bool,
    /// Factors considered
    pub factors: Vec<NonNaturalFactor>,
    /// Reasoning (Transco test)
    pub reasoning: String,
}

/// Factors for non-natural use
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonNaturalFactor {
    /// Ordinary use of land
    OrdinaryUse,
    /// Common/expected in locality
    CommonInLocality,
    /// Benefit to community
    CommunityBenefit,
    /// Exceptional/unusual use
    ExceptionalUse,
    /// Industrial scale
    IndustrialScale,
    /// Quantity beyond normal
    ExcessiveQuantity,
}

/// Analysis of escape
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EscapeAnalysis {
    /// Did thing escape?
    pub escaped: bool,
    /// Description of escape
    pub description: String,
    /// From defendant's land to outside?
    pub from_land: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Analysis of mischief/damage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MischiefAnalysis {
    /// Type of damage
    pub damage_type: RylandsDamageType,
    /// Description
    pub description: String,
    /// Was damage foreseeable?
    pub foreseeable: bool,
    /// Actionable?
    pub actionable: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Type of damage for Rylands
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RylandsDamageType {
    /// Property damage
    PropertyDamage,
    /// Personal injury (historically excluded, now possible)
    PersonalInjury,
    /// Economic loss
    EconomicLoss,
}

/// Defence to Rylands v Fletcher
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RylandsDefence {
    /// Type of defence
    pub defence_type: RylandsDefenceType,
    /// Does defence apply?
    pub applies: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Types of defence to Rylands
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RylandsDefenceType {
    /// Act of God
    ActOfGod,
    /// Act of stranger
    ActOfStranger,
    /// Claimant's default
    ClaimantsDefault,
    /// Statutory authority
    StatutoryAuthority,
    /// Consent/common benefit
    Consent,
}

// ============================================================================
// Nuisance Analyzer
// ============================================================================

/// Analyzer for nuisance claims
#[derive(Debug, Clone)]
pub struct NuisanceAnalyzer;

impl NuisanceAnalyzer {
    /// Analyze private nuisance claim
    pub fn analyze_private_nuisance(
        facts: &PrivateNuisanceFacts,
    ) -> Result<PrivateNuisanceAnalysis, TortError> {
        // 1. Check standing (Hunter v Canary Wharf)
        let standing = Self::analyze_standing(&facts.claimant_interest);
        if !standing.has_standing {
            return Err(TortError::NoInterestInLand {
                status: format!("{:?}", facts.claimant_interest),
            });
        }

        // 2. Analyze interference
        let interference = Self::analyze_interference(facts);
        if !interference.actionable {
            return Err(TortError::ReasonableUseOfLand {
                reason: interference.reasoning.clone(),
            });
        }

        // 3. Analyze reasonableness
        let reasonableness = Self::analyze_reasonableness(facts);
        if !reasonableness.unreasonable {
            return Err(TortError::ReasonableUseOfLand {
                reason: reasonableness.reasoning.clone(),
            });
        }

        // 4. Check defendant liability
        let defendant_liability = Self::analyze_defendant_liability(facts);
        if !defendant_liability.liable {
            return Err(TortError::NotResponsibleForNuisance {
                reason: defendant_liability.reasoning.clone(),
            });
        }

        // 5. Check defences
        let defences = Self::analyze_private_nuisance_defences(facts);

        // 6. Determine remedies
        let remedies = Self::determine_remedies(facts, &reasonableness);

        // 7. Check if any complete defence applies
        let complete_defence = defences.iter().any(|d| {
            d.applies
                && matches!(
                    d.defence_type,
                    NuisanceDefenceType::StatutoryAuthority | NuisanceDefenceType::Prescription
                )
        });

        let claim_succeeds = !complete_defence;

        Ok(PrivateNuisanceAnalysis {
            standing,
            interference,
            reasonableness,
            defendant_liability,
            defences,
            remedies,
            claim_succeeds,
            reasoning: if claim_succeeds {
                "Private nuisance established".to_string()
            } else {
                "Defence applies to defeat claim".to_string()
            },
        })
    }

    /// Analyze public nuisance claim
    pub fn analyze_public_nuisance(
        facts: &PublicNuisanceFacts,
    ) -> Result<PublicNuisanceAnalysis, TortError> {
        // 1. Is this a public nuisance?
        let is_public = Self::analyze_public_nuisance_test(facts);
        if !is_public.is_public_nuisance {
            return Ok(PublicNuisanceAnalysis {
                is_public_nuisance: is_public,
                special_damage: SpecialDamageAnalysis {
                    suffered_special_damage: false,
                    different_in_kind: false,
                    greater_in_degree: false,
                    description: String::new(),
                    sufficient: false,
                    reasoning: "Not a public nuisance".to_string(),
                },
                defendant_liable: false,
                claim_succeeds: false,
                reasoning: "Not a public nuisance".to_string(),
            });
        }

        // 2. For civil claim, need special damage
        let special_damage = Self::analyze_special_damage(facts);

        let claim_succeeds = is_public.is_public_nuisance && special_damage.sufficient;

        let reasoning = if claim_succeeds {
            "Public nuisance with special damage - civil claim available".to_string()
        } else if !special_damage.sufficient {
            "Public nuisance but no special damage for civil claim".to_string()
        } else {
            "Not a public nuisance".to_string()
        };

        Ok(PublicNuisanceAnalysis {
            is_public_nuisance: is_public,
            special_damage,
            defendant_liable: claim_succeeds,
            claim_succeeds,
            reasoning,
        })
    }

    /// Analyze Rylands v Fletcher claim
    pub fn analyze_rylands_v_fletcher(
        facts: &RylandsFacts,
    ) -> Result<RylandsVFletcherAnalysis, TortError> {
        // 1. Check standing
        let standing = RylandsStanding {
            interest: facts.claimant_interest.clone(),
            has_standing: matches!(
                facts.claimant_interest,
                LandInterest::FreeholdOwner
                    | LandInterest::LeaseholdTenant
                    | LandInterest::ExclusiveLicensee
            ),
            reasoning: "Claimant must have proprietary interest in land".to_string(),
        };

        if !standing.has_standing {
            return Err(TortError::NoInterestInLand {
                status: format!("{:?}", facts.claimant_interest),
            });
        }

        // 2. Thing brought onto land
        let thing_brought = ThingBroughtAnalysis {
            description: facts.thing_description.clone(),
            thing_type: facts.thing_type.clone(),
            brought_onto_land: facts.brought_onto_land,
            likely_to_cause_mischief: facts.likely_to_cause_mischief,
            reasoning: if facts.brought_onto_land && facts.likely_to_cause_mischief {
                "Thing brought onto land likely to cause mischief if escapes".to_string()
            } else {
                "Requirement not satisfied".to_string()
            },
        };

        // 3. Non-natural use (Transco test)
        let non_natural_use = Self::analyze_non_natural_use(facts);
        if !non_natural_use.non_natural {
            return Err(TortError::NaturalUseOfLand {
                reason: non_natural_use.reasoning.clone(),
            });
        }

        // 4. Escape
        let escape = EscapeAnalysis {
            escaped: facts.escaped,
            description: facts.escape_description.clone(),
            from_land: facts.escaped,
            reasoning: if facts.escaped {
                "Thing escaped from defendant's land".to_string()
            } else {
                "No escape occurred".to_string()
            },
        };

        if !escape.escaped {
            return Err(TortError::NoEscape {
                thing: facts.thing_description.clone(),
            });
        }

        // 5. Mischief
        let mischief = MischiefAnalysis {
            damage_type: facts.damage_type.clone(),
            description: facts.damage_description.clone(),
            foreseeable: facts.damage_foreseeable,
            actionable: facts.damage_foreseeable,
            reasoning: if facts.damage_foreseeable {
                "Foreseeable type of damage".to_string()
            } else {
                "Damage not foreseeable".to_string()
            },
        };

        // 6. Defences
        let defences = Self::analyze_rylands_defences(facts);

        // 7. Check if any defence applies
        let defence_applies = defences.iter().any(|d| d.applies);

        let claim_succeeds = thing_brought.brought_onto_land
            && thing_brought.likely_to_cause_mischief
            && non_natural_use.non_natural
            && escape.escaped
            && mischief.actionable
            && !defence_applies;

        Ok(RylandsVFletcherAnalysis {
            standing,
            thing_brought,
            non_natural_use,
            escape,
            mischief,
            defences,
            claim_succeeds,
            reasoning: if claim_succeeds {
                "Rylands v Fletcher liability established".to_string()
            } else if defence_applies {
                "Defence applies to exclude liability".to_string()
            } else {
                "Requirements not satisfied".to_string()
            },
        })
    }

    fn analyze_standing(interest: &LandInterest) -> StandingAnalysis {
        let has_standing = matches!(
            interest,
            LandInterest::FreeholdOwner
                | LandInterest::LeaseholdTenant
                | LandInterest::ExclusiveLicensee
                | LandInterest::Reversioner
        );

        StandingAnalysis {
            interest: interest.clone(),
            has_standing,
            reasoning: if has_standing {
                "Claimant has proprietary or possessory interest in land (Hunter v Canary Wharf)"
                    .to_string()
            } else {
                "Claimant lacks proprietary interest - mere occupiers/family members cannot sue (Hunter v Canary Wharf)"
                    .to_string()
            },
        }
    }

    fn analyze_interference(facts: &PrivateNuisanceFacts) -> InterferenceAnalysis {
        let actionable = facts.severity != InterferenceSeverity::Trivial
            && facts.duration != InterferenceDuration::SingleEvent;

        InterferenceAnalysis {
            interference_type: facts.interference_type.clone(),
            duration: facts.duration.clone(),
            severity: facts.severity.clone(),
            description: facts.interference_description.clone(),
            actionable,
            reasoning: if actionable {
                "Substantial interference with enjoyment of land".to_string()
            } else {
                "Interference trivial or isolated - not actionable".to_string()
            },
        }
    }

    fn analyze_reasonableness(facts: &PrivateNuisanceFacts) -> ReasonablenessAnalysis {
        // Sensitivity analysis (Robinson v Kilvert)
        let sensitivity = SensitivityAnalysis {
            abnormally_sensitive: facts.abnormally_sensitive,
            ordinary_person_affected: facts.ordinary_person_affected,
            description: facts.sensitivity_description.clone().unwrap_or_default(),
            effect: if facts.abnormally_sensitive && !facts.ordinary_person_affected {
                SensitivityEffect::ClaimFails
            } else if facts.abnormally_sensitive {
                SensitivityEffect::DamagesLimited
            } else {
                SensitivityEffect::NoEffect
            },
        };

        // Check if sensitivity bars claim
        if sensitivity.effect == SensitivityEffect::ClaimFails {
            return ReasonablenessAnalysis {
                locality: facts.locality.clone(),
                duration: facts.duration.clone(),
                time_sensitivity: facts.time_sensitivity.clone(),
                claimant_sensitivity: sensitivity,
                defendant_utility: DefendantUtility {
                    activity: facts.defendant_activity.clone(),
                    public_benefit: facts.defendant_public_benefit,
                    economic_necessity: facts.defendant_economic_necessity,
                    utility_level: UtilityLevel::Moderate,
                },
                malice: facts.malice.as_ref().map(|m| MaliceAnalysis {
                    malice_present: true,
                    description: m.clone(),
                    effect: "Malice makes otherwise lawful activity actionable".to_string(),
                }),
                unreasonable: false,
                reasoning: "Abnormal sensitivity - Robinson v Kilvert".to_string(),
            };
        }

        // Balance factors
        let locality_factor = match (&facts.locality, &facts.interference_type) {
            (LocalityCharacter::Residential, InterferenceType::Noise) => true,
            (LocalityCharacter::Residential, InterferenceType::Smell) => true,
            (LocalityCharacter::Industrial, InterferenceType::Noise) => false,
            (LocalityCharacter::Industrial, InterferenceType::Smell) => false,
            _ => facts.severity != InterferenceSeverity::Minor,
        };

        let duration_factor = matches!(
            facts.duration,
            InterferenceDuration::Continuous | InterferenceDuration::Recurring
        );

        let severity_factor = matches!(
            facts.severity,
            InterferenceSeverity::Substantial | InterferenceSeverity::Severe
        );

        let malice_factor = facts.malice.is_some();

        // Malice can make otherwise lawful conduct actionable (Christie v Davey)
        let unreasonable = (locality_factor && duration_factor) || severity_factor || malice_factor;

        ReasonablenessAnalysis {
            locality: facts.locality.clone(),
            duration: facts.duration.clone(),
            time_sensitivity: facts.time_sensitivity.clone(),
            claimant_sensitivity: sensitivity,
            defendant_utility: DefendantUtility {
                activity: facts.defendant_activity.clone(),
                public_benefit: facts.defendant_public_benefit,
                economic_necessity: facts.defendant_economic_necessity,
                utility_level: if facts.defendant_public_benefit {
                    UtilityLevel::High
                } else if facts.defendant_economic_necessity {
                    UtilityLevel::Moderate
                } else {
                    UtilityLevel::Low
                },
            },
            malice: facts.malice.as_ref().map(|m| MaliceAnalysis {
                malice_present: true,
                description: m.clone(),
                effect: "Malice makes otherwise lawful activity actionable (Christie v Davey)"
                    .to_string(),
            }),
            unreasonable,
            reasoning: if unreasonable {
                "Unreasonable interference with enjoyment of land".to_string()
            } else {
                "Use of land reasonable in all circumstances".to_string()
            },
        }
    }

    fn analyze_defendant_liability(facts: &PrivateNuisanceFacts) -> DefendantLiabilityAnalysis {
        let role = if facts.defendant_created_nuisance {
            DefendantRole::Creator
        } else if facts.defendant_adopted_nuisance {
            DefendantRole::Adopter
        } else if facts.defendant_continued_nuisance {
            DefendantRole::Continuer
        } else if facts.defendant_is_landlord {
            DefendantRole::Landlord
        } else {
            DefendantRole::NotResponsible
        };

        let liable = matches!(
            role,
            DefendantRole::Creator | DefendantRole::Adopter | DefendantRole::Continuer
        ) || (matches!(role, DefendantRole::Landlord)
            && facts.landlord_knew_of_nuisance);

        DefendantLiabilityAnalysis {
            defendant_role: role.clone(),
            liable,
            reasoning: match role {
                DefendantRole::Creator => "Defendant created the nuisance - liable".to_string(),
                DefendantRole::Adopter => {
                    "Defendant adopted nuisance by using it for their purposes (Sedleigh-Denfield)"
                        .to_string()
                }
                DefendantRole::Continuer => {
                    "Defendant continued nuisance with knowledge (Sedleigh-Denfield)".to_string()
                }
                DefendantRole::Landlord => {
                    if facts.landlord_knew_of_nuisance {
                        "Landlord liable - knew of nuisance before letting".to_string()
                    } else {
                        "Landlord not liable - no knowledge of nuisance".to_string()
                    }
                }
                DefendantRole::NotResponsible => {
                    "Defendant not responsible for nuisance".to_string()
                }
            },
        }
    }

    fn analyze_private_nuisance_defences(facts: &PrivateNuisanceFacts) -> Vec<NuisanceDefence> {
        let mut defences = Vec::new();

        // Prescription (20 years)
        if facts.nuisance_duration_years >= 20 {
            defences.push(NuisanceDefence {
                defence_type: NuisanceDefenceType::Prescription,
                applies: true,
                effect: "Complete defence".to_string(),
                reasoning: "Nuisance existed for 20+ years - prescriptive right acquired"
                    .to_string(),
            });
        }

        // Statutory authority
        if facts.statutory_authority {
            defences.push(NuisanceDefence {
                defence_type: NuisanceDefenceType::StatutoryAuthority,
                applies: facts.nuisance_inevitable_from_authorized_act,
                effect: "Complete defence if nuisance inevitable".to_string(),
                reasoning: if facts.nuisance_inevitable_from_authorized_act {
                    "Nuisance inevitable consequence of statutory authorization".to_string()
                } else {
                    "Could have avoided nuisance despite statutory authority".to_string()
                },
            });
        }

        // Coming to nuisance (NOT a defence but often raised)
        if facts.claimant_came_to_nuisance {
            defences.push(NuisanceDefence {
                defence_type: NuisanceDefenceType::CameToNuisance,
                applies: false, // Never applies as a defence
                effect: "No effect".to_string(),
                reasoning: "Coming to nuisance is not a defence (Miller v Jackson)".to_string(),
            });
        }

        defences
    }

    fn determine_remedies(
        facts: &PrivateNuisanceFacts,
        reasonableness: &ReasonablenessAnalysis,
    ) -> Vec<NuisanceRemedy> {
        let mut remedies = Vec::new();

        // Damages always available
        remedies.push(NuisanceRemedy {
            remedy_type: NuisanceRemedyType::Damages,
            appropriate: true,
            quantum: facts.estimated_damages,
            reasoning: "Damages available for proven loss".to_string(),
        });

        // Injunction - consider Shelfer criteria
        let shelfer_applies = facts.estimated_damages.is_some_and(|d| d < 10000.0)
            && reasonableness.defendant_utility.utility_level == UtilityLevel::High;

        if !shelfer_applies {
            remedies.push(NuisanceRemedy {
                remedy_type: NuisanceRemedyType::ProhibitoryInjunction,
                appropriate: true,
                quantum: None,
                reasoning: "Injunction appropriate - Shelfer criteria not met".to_string(),
            });
        } else {
            remedies.push(NuisanceRemedy {
                remedy_type: NuisanceRemedyType::DamagesInLieu,
                appropriate: true,
                quantum: facts.estimated_damages,
                reasoning: "Damages in lieu of injunction - Shelfer criteria met".to_string(),
            });
        }

        remedies
    }

    fn analyze_public_nuisance_test(facts: &PublicNuisanceFacts) -> PublicNuisanceTest {
        let class_large_enough = facts.number_affected >= 10
            || facts.class_description.contains("public")
            || facts.class_description.contains("highway");

        let is_public_nuisance = facts.affects_public && class_large_enough;

        PublicNuisanceTest {
            affects_class: facts.affects_public,
            class_description: facts.class_description.clone(),
            class_large_enough,
            interferes_with_public_rights: facts.interferes_with_public_right,
            public_right: facts.public_right.clone(),
            is_public_nuisance,
            reasoning: if is_public_nuisance {
                format!(
                    "Affects class of Her Majesty's subjects ({}) - AG v PYA Quarries",
                    facts.class_description
                )
            } else {
                "Does not affect sufficient class of public".to_string()
            },
        }
    }

    fn analyze_special_damage(facts: &PublicNuisanceFacts) -> SpecialDamageAnalysis {
        let sufficient = facts.special_damage_suffered
            && (facts.damage_different_in_kind || facts.damage_greater_in_degree);

        SpecialDamageAnalysis {
            suffered_special_damage: facts.special_damage_suffered,
            different_in_kind: facts.damage_different_in_kind,
            greater_in_degree: facts.damage_greater_in_degree,
            description: facts.special_damage_description.clone(),
            sufficient,
            reasoning: if sufficient {
                "Special damage suffered - civil claim available".to_string()
            } else {
                "No special damage particular to claimant".to_string()
            },
        }
    }

    fn analyze_non_natural_use(facts: &RylandsFacts) -> NonNaturalUseAnalysis {
        // Transco v Stockport test
        let factors: Vec<NonNaturalFactor> = if facts.ordinary_use_of_land {
            vec![NonNaturalFactor::OrdinaryUse]
        } else if facts.common_in_locality {
            vec![NonNaturalFactor::CommonInLocality]
        } else if facts.exceptional_use {
            vec![
                NonNaturalFactor::ExceptionalUse,
                NonNaturalFactor::ExcessiveQuantity,
            ]
        } else {
            vec![]
        };

        let non_natural = facts.exceptional_use && !facts.ordinary_use_of_land;

        NonNaturalUseAnalysis {
            use_description: facts.use_description.clone(),
            non_natural,
            factors,
            reasoning: if non_natural {
                "Non-natural use per Transco v Stockport - exceptional use not for general benefit"
                    .to_string()
            } else {
                "Natural/ordinary use of land - Rylands not applicable".to_string()
            },
        }
    }

    fn analyze_rylands_defences(facts: &RylandsFacts) -> Vec<RylandsDefence> {
        let mut defences = Vec::new();

        // Act of God
        if facts.act_of_god {
            defences.push(RylandsDefence {
                defence_type: RylandsDefenceType::ActOfGod,
                applies: true,
                reasoning: format!(
                    "Act of God: {} (Nichols v Marsland)",
                    facts
                        .act_of_god_description
                        .as_deref()
                        .unwrap_or("natural event")
                ),
            });
        }

        // Act of stranger
        if facts.act_of_stranger {
            defences.push(RylandsDefence {
                defence_type: RylandsDefenceType::ActOfStranger,
                applies: true,
                reasoning: format!(
                    "Act of stranger: {} (Perry v Kendricks Transport)",
                    facts
                        .act_of_stranger_description
                        .as_deref()
                        .unwrap_or("third party")
                ),
            });
        }

        // Claimant's default
        if facts.claimants_default {
            defences.push(RylandsDefence {
                defence_type: RylandsDefenceType::ClaimantsDefault,
                applies: true,
                reasoning: "Claimant's own act caused or contributed to escape".to_string(),
            });
        }

        // Statutory authority
        if facts.statutory_authority {
            defences.push(RylandsDefence {
                defence_type: RylandsDefenceType::StatutoryAuthority,
                applies: facts.escape_inevitable_from_authorized_act,
                reasoning: if facts.escape_inevitable_from_authorized_act {
                    "Escape inevitable from statutorily authorized activity".to_string()
                } else {
                    "Could have prevented escape despite statutory authority".to_string()
                },
            });
        }

        // Consent/common benefit
        if facts.claimant_consented || facts.common_benefit {
            defences.push(RylandsDefence {
                defence_type: RylandsDefenceType::Consent,
                applies: true,
                reasoning: if facts.claimant_consented {
                    "Claimant consented to presence of thing".to_string()
                } else {
                    "Thing accumulated for common benefit of parties".to_string()
                },
            });
        }

        defences
    }
}

/// Facts for private nuisance analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateNuisanceFacts {
    /// Claimant's interest in land
    pub claimant_interest: LandInterest,
    /// Type of interference
    pub interference_type: InterferenceType,
    /// Duration of interference
    pub duration: InterferenceDuration,
    /// Severity
    pub severity: InterferenceSeverity,
    /// Description of interference
    pub interference_description: String,
    /// Character of locality
    pub locality: LocalityCharacter,
    /// Time sensitivity
    pub time_sensitivity: TimeSensitivity,
    /// Is claimant abnormally sensitive?
    pub abnormally_sensitive: bool,
    /// Would ordinary person be affected?
    pub ordinary_person_affected: bool,
    /// Description of sensitivity
    pub sensitivity_description: Option<String>,
    /// Defendant's activity
    pub defendant_activity: String,
    /// Does defendant's activity have public benefit?
    pub defendant_public_benefit: bool,
    /// Is defendant's activity economically necessary?
    pub defendant_economic_necessity: bool,
    /// Evidence of malice
    pub malice: Option<String>,
    /// Did defendant create the nuisance?
    pub defendant_created_nuisance: bool,
    /// Did defendant adopt the nuisance?
    pub defendant_adopted_nuisance: bool,
    /// Did defendant continue the nuisance?
    pub defendant_continued_nuisance: bool,
    /// Is defendant a landlord?
    pub defendant_is_landlord: bool,
    /// Did landlord know of nuisance?
    pub landlord_knew_of_nuisance: bool,
    /// How many years has nuisance existed?
    pub nuisance_duration_years: u32,
    /// Is there statutory authority?
    pub statutory_authority: bool,
    /// Is nuisance inevitable from authorized act?
    pub nuisance_inevitable_from_authorized_act: bool,
    /// Did claimant come to nuisance?
    pub claimant_came_to_nuisance: bool,
    /// Estimated damages
    pub estimated_damages: Option<f64>,
}

/// Facts for public nuisance analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicNuisanceFacts {
    /// Does nuisance affect the public?
    pub affects_public: bool,
    /// Description of class affected
    pub class_description: String,
    /// Number of people affected
    pub number_affected: usize,
    /// Does it interfere with public right?
    pub interferes_with_public_right: bool,
    /// Type of public right
    pub public_right: PublicRight,
    /// Did claimant suffer special damage?
    pub special_damage_suffered: bool,
    /// Is damage different in kind?
    pub damage_different_in_kind: bool,
    /// Is damage greater in degree?
    pub damage_greater_in_degree: bool,
    /// Description of special damage
    pub special_damage_description: String,
}

/// Facts for Rylands v Fletcher analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RylandsFacts {
    /// Claimant's interest
    pub claimant_interest: LandInterest,
    /// Description of thing
    pub thing_description: String,
    /// Type of thing
    pub thing_type: DangerousThing,
    /// Was it brought onto land?
    pub brought_onto_land: bool,
    /// Likely to cause mischief?
    pub likely_to_cause_mischief: bool,
    /// Description of use
    pub use_description: String,
    /// Is it ordinary use of land?
    pub ordinary_use_of_land: bool,
    /// Common in locality?
    pub common_in_locality: bool,
    /// Exceptional use?
    pub exceptional_use: bool,
    /// Did thing escape?
    pub escaped: bool,
    /// Description of escape
    pub escape_description: String,
    /// Type of damage
    pub damage_type: RylandsDamageType,
    /// Description of damage
    pub damage_description: String,
    /// Was damage foreseeable?
    pub damage_foreseeable: bool,
    /// Act of God?
    pub act_of_god: bool,
    /// Description of act of God
    pub act_of_god_description: Option<String>,
    /// Act of stranger?
    pub act_of_stranger: bool,
    /// Description of act of stranger
    pub act_of_stranger_description: Option<String>,
    /// Claimant's default?
    pub claimants_default: bool,
    /// Statutory authority?
    pub statutory_authority: bool,
    /// Escape inevitable from authorized act?
    pub escape_inevitable_from_authorized_act: bool,
    /// Claimant consented?
    pub claimant_consented: bool,
    /// Common benefit?
    pub common_benefit: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_nuisance_noise() {
        let facts = PrivateNuisanceFacts {
            claimant_interest: LandInterest::FreeholdOwner,
            interference_type: InterferenceType::Noise,
            duration: InterferenceDuration::Continuous,
            severity: InterferenceSeverity::Substantial,
            interference_description: "Loud music at night".to_string(),
            locality: LocalityCharacter::Residential,
            time_sensitivity: TimeSensitivity::NightTime,
            abnormally_sensitive: false,
            ordinary_person_affected: true,
            sensitivity_description: None,
            defendant_activity: "Playing loud music".to_string(),
            defendant_public_benefit: false,
            defendant_economic_necessity: false,
            malice: None,
            defendant_created_nuisance: true,
            defendant_adopted_nuisance: false,
            defendant_continued_nuisance: false,
            defendant_is_landlord: false,
            landlord_knew_of_nuisance: false,
            nuisance_duration_years: 1,
            statutory_authority: false,
            nuisance_inevitable_from_authorized_act: false,
            claimant_came_to_nuisance: false,
            estimated_damages: Some(5000.0),
        };

        let result = NuisanceAnalyzer::analyze_private_nuisance(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.claim_succeeds);
        assert!(analysis.standing.has_standing);
        assert!(analysis.reasonableness.unreasonable);
    }

    #[test]
    fn test_private_nuisance_no_standing() {
        let facts = PrivateNuisanceFacts {
            claimant_interest: LandInterest::FamilyMember, // No standing
            interference_type: InterferenceType::Noise,
            duration: InterferenceDuration::Continuous,
            severity: InterferenceSeverity::Substantial,
            interference_description: "Noise".to_string(),
            locality: LocalityCharacter::Residential,
            time_sensitivity: TimeSensitivity::Continuous,
            abnormally_sensitive: false,
            ordinary_person_affected: true,
            sensitivity_description: None,
            defendant_activity: "Activity".to_string(),
            defendant_public_benefit: false,
            defendant_economic_necessity: false,
            malice: None,
            defendant_created_nuisance: true,
            defendant_adopted_nuisance: false,
            defendant_continued_nuisance: false,
            defendant_is_landlord: false,
            landlord_knew_of_nuisance: false,
            nuisance_duration_years: 1,
            statutory_authority: false,
            nuisance_inevitable_from_authorized_act: false,
            claimant_came_to_nuisance: false,
            estimated_damages: None,
        };

        let result = NuisanceAnalyzer::analyze_private_nuisance(&facts);
        assert!(result.is_err());
        assert!(matches!(result, Err(TortError::NoInterestInLand { .. })));
    }

    #[test]
    fn test_private_nuisance_abnormal_sensitivity() {
        let facts = PrivateNuisanceFacts {
            claimant_interest: LandInterest::FreeholdOwner,
            interference_type: InterferenceType::Heat,
            duration: InterferenceDuration::Continuous,
            severity: InterferenceSeverity::Moderate,
            interference_description: "Heat from boiler".to_string(),
            locality: LocalityCharacter::Commercial,
            time_sensitivity: TimeSensitivity::Continuous,
            abnormally_sensitive: true, // Robinson v Kilvert
            ordinary_person_affected: false,
            sensitivity_description: Some("Stored sensitive paper".to_string()),
            defendant_activity: "Operating boiler".to_string(),
            defendant_public_benefit: false,
            defendant_economic_necessity: true,
            malice: None,
            defendant_created_nuisance: true,
            defendant_adopted_nuisance: false,
            defendant_continued_nuisance: false,
            defendant_is_landlord: false,
            landlord_knew_of_nuisance: false,
            nuisance_duration_years: 2,
            statutory_authority: false,
            nuisance_inevitable_from_authorized_act: false,
            claimant_came_to_nuisance: false,
            estimated_damages: Some(10000.0),
        };

        let result = NuisanceAnalyzer::analyze_private_nuisance(&facts);
        assert!(result.is_err());
        assert!(matches!(result, Err(TortError::ReasonableUseOfLand { .. })));
    }

    #[test]
    fn test_private_nuisance_malice() {
        let facts = PrivateNuisanceFacts {
            claimant_interest: LandInterest::LeaseholdTenant,
            interference_type: InterferenceType::Noise,
            duration: InterferenceDuration::Recurring,
            severity: InterferenceSeverity::Moderate,
            interference_description: "Deliberate noise to annoy".to_string(),
            locality: LocalityCharacter::Residential,
            time_sensitivity: TimeSensitivity::NightTime,
            abnormally_sensitive: false,
            ordinary_person_affected: true,
            sensitivity_description: None,
            defendant_activity: "Making noise".to_string(),
            defendant_public_benefit: false,
            defendant_economic_necessity: false,
            malice: Some("Deliberately done to annoy claimant".to_string()), // Christie v Davey
            defendant_created_nuisance: true,
            defendant_adopted_nuisance: false,
            defendant_continued_nuisance: false,
            defendant_is_landlord: false,
            landlord_knew_of_nuisance: false,
            nuisance_duration_years: 0,
            statutory_authority: false,
            nuisance_inevitable_from_authorized_act: false,
            claimant_came_to_nuisance: false,
            estimated_damages: Some(2000.0),
        };

        let result = NuisanceAnalyzer::analyze_private_nuisance(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.claim_succeeds);
        assert!(analysis.reasonableness.malice.is_some());
        assert!(analysis.reasonableness.unreasonable);
    }

    #[test]
    fn test_public_nuisance_highway() {
        let facts = PublicNuisanceFacts {
            affects_public: true,
            class_description: "Highway users".to_string(),
            number_affected: 100,
            interferes_with_public_right: true,
            public_right: PublicRight::Highway,
            special_damage_suffered: true,
            damage_different_in_kind: true,
            damage_greater_in_degree: false,
            special_damage_description: "Vehicle damaged by obstruction".to_string(),
        };

        let result = NuisanceAnalyzer::analyze_public_nuisance(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.claim_succeeds);
        assert!(analysis.is_public_nuisance.is_public_nuisance);
        assert!(analysis.special_damage.sufficient);
    }

    #[test]
    fn test_public_nuisance_no_special_damage() {
        let facts = PublicNuisanceFacts {
            affects_public: true,
            class_description: "Residents of the street".to_string(),
            number_affected: 50,
            interferes_with_public_right: true,
            public_right: PublicRight::PublicHealth,
            special_damage_suffered: false, // No special damage
            damage_different_in_kind: false,
            damage_greater_in_degree: false,
            special_damage_description: String::new(),
        };

        let result = NuisanceAnalyzer::analyze_public_nuisance(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.claim_succeeds); // No civil claim without special damage
        assert!(analysis.is_public_nuisance.is_public_nuisance);
    }

    #[test]
    fn test_rylands_v_fletcher_water_escape() {
        let facts = RylandsFacts {
            claimant_interest: LandInterest::FreeholdOwner,
            thing_description: "Large reservoir of water".to_string(),
            thing_type: DangerousThing::Water,
            brought_onto_land: true,
            likely_to_cause_mischief: true,
            use_description: "Industrial water storage".to_string(),
            ordinary_use_of_land: false,
            common_in_locality: false,
            exceptional_use: true, // Non-natural
            escaped: true,
            escape_description: "Water flooded neighbour's land".to_string(),
            damage_type: RylandsDamageType::PropertyDamage,
            damage_description: "Flood damage to property".to_string(),
            damage_foreseeable: true,
            act_of_god: false,
            act_of_god_description: None,
            act_of_stranger: false,
            act_of_stranger_description: None,
            claimants_default: false,
            statutory_authority: false,
            escape_inevitable_from_authorized_act: false,
            claimant_consented: false,
            common_benefit: false,
        };

        let result = NuisanceAnalyzer::analyze_rylands_v_fletcher(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.claim_succeeds);
        assert!(analysis.non_natural_use.non_natural);
        assert!(analysis.escape.escaped);
    }

    #[test]
    fn test_rylands_natural_use() {
        let facts = RylandsFacts {
            claimant_interest: LandInterest::FreeholdOwner,
            thing_description: "Domestic water pipes".to_string(),
            thing_type: DangerousThing::Water,
            brought_onto_land: true,
            likely_to_cause_mischief: true,
            use_description: "Domestic water supply".to_string(),
            ordinary_use_of_land: true, // Natural use (Transco)
            common_in_locality: true,
            exceptional_use: false,
            escaped: true,
            escape_description: "Pipe burst".to_string(),
            damage_type: RylandsDamageType::PropertyDamage,
            damage_description: "Water damage".to_string(),
            damage_foreseeable: true,
            act_of_god: false,
            act_of_god_description: None,
            act_of_stranger: false,
            act_of_stranger_description: None,
            claimants_default: false,
            statutory_authority: false,
            escape_inevitable_from_authorized_act: false,
            claimant_consented: false,
            common_benefit: false,
        };

        let result = NuisanceAnalyzer::analyze_rylands_v_fletcher(&facts);
        assert!(result.is_err());
        assert!(matches!(result, Err(TortError::NaturalUseOfLand { .. })));
    }

    #[test]
    fn test_rylands_act_of_god_defence() {
        let facts = RylandsFacts {
            claimant_interest: LandInterest::FreeholdOwner,
            thing_description: "Industrial chemicals".to_string(),
            thing_type: DangerousThing::Chemicals,
            brought_onto_land: true,
            likely_to_cause_mischief: true,
            use_description: "Chemical storage".to_string(),
            ordinary_use_of_land: false,
            common_in_locality: false,
            exceptional_use: true,
            escaped: true,
            escape_description: "Chemicals escaped during unprecedented flood".to_string(),
            damage_type: RylandsDamageType::PropertyDamage,
            damage_description: "Chemical contamination".to_string(),
            damage_foreseeable: true,
            act_of_god: true, // Defence
            act_of_god_description: Some("Unprecedented 1-in-1000-year flood".to_string()),
            act_of_stranger: false,
            act_of_stranger_description: None,
            claimants_default: false,
            statutory_authority: false,
            escape_inevitable_from_authorized_act: false,
            claimant_consented: false,
            common_benefit: false,
        };

        let result = NuisanceAnalyzer::analyze_rylands_v_fletcher(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.claim_succeeds); // Defence applies
        assert!(
            analysis
                .defences
                .iter()
                .any(|d| matches!(d.defence_type, RylandsDefenceType::ActOfGod) && d.applies)
        );
    }
}
