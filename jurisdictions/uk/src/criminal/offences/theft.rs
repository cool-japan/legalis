//! UK Theft and Related Offences
//!
//! This module covers property offences under the Theft Act 1968 and related legislation.
//!
//! # Statutory Framework
//!
//! - **Theft** (s.1 TA 1968) - Either way, max 7 years
//! - **Robbery** (s.8 TA 1968) - Indictable only, max life
//! - **Burglary** (s.9 TA 1968) - Either way / indictable, max 10/14 years
//! - **Aggravated burglary** (s.10 TA 1968) - Indictable only, max life
//! - **Handling stolen goods** (s.22 TA 1968) - Either way, max 14 years
//! - **Going equipped** (s.25 TA 1968) - Either way, max 3 years
//!
//! # Key Concepts
//!
//! ## Theft (s.1)
//! "A person is guilty of theft if he dishonestly appropriates property
//! belonging to another with the intention of permanently depriving the
//! other of it"
//!
//! ## Five Elements
//! 1. Appropriation (s.3) - R v Gomez, R v Hinks
//! 2. Property (s.4)
//! 3. Belonging to another (s.5)
//! 4. Dishonesty (Ivey v Genting Casinos)
//! 5. Intention to permanently deprive (s.6)

// Allow missing docs on enum variant struct fields - these are self-documenting in context
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::criminal::error::{CriminalError, CriminalResult};
use crate::criminal::types::{
    ActType, ActusReusElement, CaseCitation, DishonestyAnalysis, MaximumSentence, MensReaType,
    Offence, OffenceBuilder, OffenceCategory, OffenceClassification, OffenceSeverity,
};

// ============================================================================
// Theft Offence Types
// ============================================================================

/// Property offence type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyOffence {
    /// Basic theft (s.1 TA 1968)
    Theft,
    /// Robbery (s.8 TA 1968)
    Robbery,
    /// Burglary (s.9(1)(a) - entry with intent)
    BurglaryWithIntent,
    /// Burglary (s.9(1)(b) - entry then commits)
    BurglaryHavingEntered,
    /// Aggravated burglary (s.10)
    AggravatedBurglary,
    /// Handling stolen goods (s.22)
    HandlingStolen,
    /// Going equipped (s.25)
    GoingEquipped,
    /// Taking without consent (s.12)
    TWOC,
    /// Aggravated vehicle taking (s.12A)
    AggravatedTWOC,
    /// Making off without payment (s.3 TA 1978)
    MakingOff,
    /// Abstracting electricity (s.13 TA 1968)
    AbstractingElectricity,
}

// ============================================================================
// Theft Analysis
// ============================================================================

/// Facts for theft analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TheftFacts {
    /// Appropriation facts (s.3)
    pub appropriation: AppropriationFacts,
    /// Property facts (s.4)
    pub property: PropertyFacts,
    /// Belonging to another facts (s.5)
    pub belonging: BelongingFacts,
    /// Dishonesty facts
    pub dishonesty: TheftDishonestyFacts,
    /// Intention to permanently deprive (s.6)
    pub intention_deprive: IntentionDepriveFacts,
}

/// Appropriation facts (s.3 TA 1968)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppropriationFacts {
    /// Description of appropriation
    pub description: String,
    /// Type of appropriation
    pub appropriation_type: AppropriationType,
    /// Was there owner's consent? (R v Gomez - consent doesn't prevent appropriation)
    pub with_consent: bool,
    /// Was property given as valid gift? (R v Hinks - may still be appropriation)
    pub valid_gift: bool,
}

/// Types of appropriation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppropriationType {
    /// Taking/removing property
    Taking,
    /// Assuming rights (e.g., switching price labels)
    AssumingRights,
    /// Using property inconsistently with owner's rights
    InconsistentUse,
    /// Keeping property after innocent acquisition
    KeepingAfterInnocent,
    /// Dealing with property as owner
    DealingAsOwner,
}

/// Property facts (s.4 TA 1968)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyFacts {
    /// Description of property
    pub description: String,
    /// Type of property
    pub property_type: PropertyType,
    /// Value (relevant for sentencing)
    pub value: Option<f64>,
}

/// Types of property
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyType {
    /// Money
    Money,
    /// Physical/tangible property
    Physical,
    /// Real property (land) - generally cannot be stolen except in specific cases
    Land { exception: Option<LandException> },
    /// Chose in action (e.g., bank account)
    ChoseInAction,
    /// Intangible property (e.g., patents)
    Intangible,
    /// Wild creatures - special rules
    WildCreature { reduced_to_possession: bool },
    /// Electricity - cannot be stolen (s.13)
    Electricity,
    /// Confidential information - not property (Oxford v Moss)
    ConfidentialInfo,
}

/// Exceptions allowing theft of land
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandException {
    /// Trustee/PR disposing of land
    TrusteeDisposal,
    /// Not in possession - severing fixtures
    SeveringFixtures,
    /// Tenant removing fixtures
    TenantRemoving,
}

/// Belonging to another facts (s.5 TA 1968)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BelongingFacts {
    /// Who does property belong to?
    pub owner: Option<String>,
    /// Type of belonging
    pub belonging_type: BelongingType,
    /// Any s.5 special situations?
    pub special_situation: Option<SpecialBelongingSituation>,
}

/// Types of belonging
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BelongingType {
    /// Ownership
    Ownership,
    /// Possession
    Possession,
    /// Control
    Control,
    /// Proprietary right or interest
    ProprietaryInterest,
}

/// Special belonging situations under s.5
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecialBelongingSituation {
    /// s.5(3) - Obligation to deal in particular way
    ObligationToDeal,
    /// s.5(4) - Property received by mistake
    ReceivedByMistake,
    /// Abandoned property (generally not theft)
    Abandoned,
}

/// Dishonesty facts for theft
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TheftDishonestyFacts {
    /// s.2(1)(a) - Believed had right in law
    pub believed_right_in_law: bool,
    /// s.2(1)(b) - Believed owner would consent
    pub believed_owner_consent: bool,
    /// s.2(1)(c) - Believed owner cannot be found
    pub believed_owner_unknown: bool,
    /// For Ivey test: D's actual knowledge/belief
    pub actual_knowledge_belief: String,
    /// Would ordinary person consider it dishonest?
    pub dishonest_by_ordinary_standards: bool,
}

/// Intention to permanently deprive facts (s.6)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentionDepriveFacts {
    /// Did D intend permanent deprivation?
    pub intended_permanent: bool,
    /// s.6 deemed intention situations
    pub deemed_intention: Option<DeemedIntention>,
}

/// Deemed intention under s.6
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeemedIntention {
    /// s.6(1) - Treating as own to dispose of regardless of other's rights
    TreatAsOwn,
    /// s.6(1) - Borrowing/lending for period/circumstances making it equivalent to outright taking
    EquivalentToTaking { circumstances: String },
    /// s.6(2) - Parting with property under condition not fulfilled
    PartingUnderCondition,
}

/// Result of theft analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TheftAnalysisResult {
    /// Theft established?
    pub established: bool,
    /// Appropriation analysis
    pub appropriation: AppropriationAnalysis,
    /// Property analysis
    pub property: PropertyAnalysis,
    /// Belonging analysis
    pub belonging: BelongingAnalysis,
    /// Dishonesty analysis
    pub dishonesty: DishonestyAnalysis,
    /// Intention to permanently deprive analysis
    pub intention_deprive: IntentionDepriveAnalysis,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Appropriation analysis result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppropriationAnalysis {
    /// Appropriation established?
    pub established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Property analysis result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyAnalysis {
    /// Property established?
    pub established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Belonging analysis result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BelongingAnalysis {
    /// Belonging established?
    pub established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Intention to permanently deprive analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentionDepriveAnalysis {
    /// Intention established?
    pub established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Analyzer for theft
pub struct TheftAnalyzer;

impl TheftAnalyzer {
    /// Analyze theft
    pub fn analyze(facts: &TheftFacts) -> CriminalResult<TheftAnalysisResult> {
        let case_law = vec![
            CaseCitation::new(
                "R v Gomez",
                1993,
                "AC 442",
                "Appropriation occurs even with owner's consent",
            ),
            CaseCitation::new(
                "R v Hinks",
                2001,
                "2 AC 241",
                "Appropriation can occur even for valid gift if dishonest",
            ),
            CaseCitation::new(
                "Ivey v Genting Casinos",
                2017,
                "UKSC 67",
                "Two-stage dishonesty test: D's actual belief + ordinary standards",
            ),
            CaseCitation::new(
                "R v Lloyd",
                1985,
                "QB 829",
                "Borrowing is s.6 only if equivalent to outright taking",
            ),
        ];

        // Analyze each element
        let appropriation = Self::analyze_appropriation(&facts.appropriation);
        let property = Self::analyze_property(&facts.property);
        let belonging = Self::analyze_belonging(&facts.belonging);
        let dishonesty = Self::analyze_dishonesty(&facts.dishonesty);
        let intention_deprive = Self::analyze_intention(&facts.intention_deprive);

        let established = appropriation.established
            && property.established
            && belonging.established
            && dishonesty.dishonest_by_ordinary_standards
            && intention_deprive.established;

        Ok(TheftAnalysisResult {
            established,
            appropriation,
            property,
            belonging,
            dishonesty,
            intention_deprive,
            case_law,
        })
    }

    fn analyze_appropriation(facts: &AppropriationFacts) -> AppropriationAnalysis {
        // Per R v Gomez and R v Hinks, appropriation is very broad
        let established = true; // Almost any assumption of rights counts

        let reasoning = match &facts.appropriation_type {
            AppropriationType::Taking => "Appropriation by taking/removing property".to_string(),
            AppropriationType::AssumingRights => {
                "Appropriation by assuming owner's rights (s.3)".to_string()
            }
            AppropriationType::InconsistentUse => {
                "Appropriation by use inconsistent with owner's rights".to_string()
            }
            AppropriationType::KeepingAfterInnocent => {
                "Appropriation by keeping after innocent acquisition (s.3(1))".to_string()
            }
            AppropriationType::DealingAsOwner => {
                "Appropriation by dealing with property as owner".to_string()
            }
        };

        let mut reasoning = reasoning;
        if facts.with_consent {
            reasoning.push_str(" (R v Gomez: consent doesn't prevent appropriation)");
        }
        if facts.valid_gift {
            reasoning.push_str(" (R v Hinks: even valid gift can be appropriation)");
        }

        AppropriationAnalysis {
            established,
            reasoning,
        }
    }

    fn analyze_property(facts: &PropertyFacts) -> PropertyAnalysis {
        let (established, reasoning) = match &facts.property_type {
            PropertyType::Money => (true, "Money is property (s.4)".to_string()),
            PropertyType::Physical => (true, format!("Physical property: {}", facts.description)),
            PropertyType::ChoseInAction => (true, "Chose in action is property (s.4)".to_string()),
            PropertyType::Intangible => (true, "Intangible property (s.4)".to_string()),
            PropertyType::Land { exception } => {
                if exception.is_some() {
                    (true, "Land with s.4(2) exception".to_string())
                } else {
                    (
                        false,
                        "Land cannot generally be stolen (s.4(2))".to_string(),
                    )
                }
            }
            PropertyType::WildCreature {
                reduced_to_possession,
            } => {
                if *reduced_to_possession {
                    (
                        true,
                        "Wild creature reduced to possession - is property".to_string(),
                    )
                } else {
                    (false, "Wild creature not reduced to possession".to_string())
                }
            }
            PropertyType::Electricity => (
                false,
                "Electricity cannot be stolen - use s.13 instead".to_string(),
            ),
            PropertyType::ConfidentialInfo => (
                false,
                "Confidential information is not property (Oxford v Moss)".to_string(),
            ),
        };

        PropertyAnalysis {
            established,
            reasoning,
        }
    }

    fn analyze_belonging(facts: &BelongingFacts) -> BelongingAnalysis {
        let mut established = true;
        let mut reasoning = String::new();

        if let Some(owner) = &facts.owner {
            reasoning.push_str(&format!(
                "Property belongs to {} by {:?}",
                owner, facts.belonging_type
            ));
        }

        // Special situations under s.5
        if let Some(special) = &facts.special_situation {
            match special {
                SpecialBelongingSituation::ObligationToDeal => {
                    reasoning.push_str(" - s.5(3) obligation to deal in particular way");
                }
                SpecialBelongingSituation::ReceivedByMistake => {
                    reasoning.push_str(" - s.5(4) received by mistake, obligation to restore");
                }
                SpecialBelongingSituation::Abandoned => {
                    established = false;
                    reasoning = "Property abandoned - not 'belonging to another'".to_string();
                }
            }
        }

        BelongingAnalysis {
            established,
            reasoning,
        }
    }

    fn analyze_dishonesty(facts: &TheftDishonestyFacts) -> DishonestyAnalysis {
        // s.2(1) negatives
        if facts.believed_right_in_law {
            return DishonestyAnalysis {
                defendants_knowledge: facts.actual_knowledge_belief.clone(),
                dishonest_by_ordinary_standards: false,
                reasoning: "s.2(1)(a): D believed had right in law to deprive - not dishonest"
                    .into(),
            };
        }

        if facts.believed_owner_consent {
            return DishonestyAnalysis {
                defendants_knowledge: facts.actual_knowledge_belief.clone(),
                dishonest_by_ordinary_standards: false,
                reasoning: "s.2(1)(b): D believed owner would consent - not dishonest".into(),
            };
        }

        if facts.believed_owner_unknown {
            return DishonestyAnalysis {
                defendants_knowledge: facts.actual_knowledge_belief.clone(),
                dishonest_by_ordinary_standards: false,
                reasoning: "s.2(1)(c): D believed owner could not be found by reasonable steps \
                           - not dishonest"
                    .into(),
            };
        }

        // Ivey test
        DishonestyAnalysis {
            defendants_knowledge: facts.actual_knowledge_belief.clone(),
            dishonest_by_ordinary_standards: facts.dishonest_by_ordinary_standards,
            reasoning: if facts.dishonest_by_ordinary_standards {
                "Ivey test: given D's knowledge/belief, conduct dishonest by ordinary standards"
                    .into()
            } else {
                "Ivey test: given D's knowledge/belief, not dishonest by ordinary standards".into()
            },
        }
    }

    fn analyze_intention(facts: &IntentionDepriveFacts) -> IntentionDepriveAnalysis {
        if facts.intended_permanent {
            return IntentionDepriveAnalysis {
                established: true,
                reasoning: "D intended to permanently deprive owner".into(),
            };
        }

        // Check deemed intention under s.6
        if let Some(deemed) = &facts.deemed_intention {
            let (established, reasoning) = match deemed {
                DeemedIntention::TreatAsOwn => (
                    true,
                    "s.6(1): D treated property as own to dispose regardless of owner's rights"
                        .to_string(),
                ),
                DeemedIntention::EquivalentToTaking { circumstances } => (
                    true,
                    format!(
                        "s.6(1): Borrowing equivalent to outright taking - {}",
                        circumstances
                    ),
                ),
                DeemedIntention::PartingUnderCondition => (
                    true,
                    "s.6(2): D parted with property under unfulfilled condition".to_string(),
                ),
            };
            return IntentionDepriveAnalysis {
                established,
                reasoning,
            };
        }

        IntentionDepriveAnalysis {
            established: false,
            reasoning: "No intention to permanently deprive (mere borrowing - R v Lloyd)".into(),
        }
    }
}

// ============================================================================
// Robbery
// ============================================================================

/// Facts for robbery analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobberyFacts {
    /// Theft facts
    pub theft: TheftFacts,
    /// Force or putting in fear
    pub force: RobberyForceFacts,
}

/// Force facts for robbery
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobberyForceFacts {
    /// Was force used?
    pub force_used: bool,
    /// Was person put in fear of force?
    pub put_in_fear: bool,
    /// Description of force/threat
    pub description: String,
    /// Was force immediately before or at time of stealing?
    pub at_time_of_theft: bool,
    /// Was force used to steal?
    pub in_order_to_steal: bool,
}

/// Result of robbery analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobberyAnalysisResult {
    /// Robbery established?
    pub established: bool,
    /// Theft analysis
    pub theft_analysis: TheftAnalysisResult,
    /// Force/fear analysis
    pub force_analysis: ForceAnalysis,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Force analysis for robbery
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForceAnalysis {
    /// Force/fear established?
    pub established: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Analyzer for robbery
pub struct RobberyAnalyzer;

impl RobberyAnalyzer {
    /// Analyze robbery
    pub fn analyze(facts: &RobberyFacts) -> CriminalResult<RobberyAnalysisResult> {
        let case_law = vec![
            CaseCitation::new(
                "R v Dawson",
                1976,
                "64 Cr App R 170",
                "Force includes any physical force - nudge can be enough",
            ),
            CaseCitation::new(
                "R v Clouden",
                1987,
                "Crim LR 56",
                "Wrenching bag from victim's grasp is robbery",
            ),
        ];

        // Analyze theft component
        let theft_analysis = TheftAnalyzer::analyze(&facts.theft)?;

        // Analyze force component
        let force_analysis = Self::analyze_force(&facts.force);

        let established = theft_analysis.established && force_analysis.established;

        Ok(RobberyAnalysisResult {
            established,
            theft_analysis,
            force_analysis,
            case_law,
        })
    }

    fn analyze_force(facts: &RobberyForceFacts) -> ForceAnalysis {
        let mut reasoning_parts = Vec::new();

        if !facts.force_used && !facts.put_in_fear {
            return ForceAnalysis {
                established: false,
                reasoning: "No force used and no person put in fear".into(),
            };
        }

        if facts.force_used {
            reasoning_parts.push("Force used on person");
        }
        if facts.put_in_fear {
            reasoning_parts.push("Person put in fear of force");
        }

        if !facts.at_time_of_theft {
            return ForceAnalysis {
                established: false,
                reasoning: "Force not immediately before or at time of stealing".into(),
            };
        }
        reasoning_parts.push("Force at time of theft");

        if !facts.in_order_to_steal {
            return ForceAnalysis {
                established: false,
                reasoning: "Force not used in order to steal".into(),
            };
        }
        reasoning_parts.push("Force used in order to steal");

        ForceAnalysis {
            established: true,
            reasoning: reasoning_parts.join("; "),
        }
    }
}

// ============================================================================
// Burglary
// ============================================================================

/// Facts for burglary analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurglaryFacts {
    /// Entry facts
    pub entry: EntryFacts,
    /// Building facts
    pub building: BuildingFacts,
    /// Which section - s.9(1)(a) or s.9(1)(b)?
    pub section: BurglarySection,
}

/// Entry facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryFacts {
    /// Was there entry?
    pub entry_occurred: bool,
    /// Type of entry
    pub entry_type: EntryType,
    /// Was entry as trespasser?
    pub as_trespasser: bool,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Types of entry
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryType {
    /// Physical bodily entry
    Bodily,
    /// Partial entry (e.g., arm through window - R v Brown)
    Partial,
    /// Entry using instrument
    Instrument,
    /// Entry by innocent agent
    InnocentAgent,
}

/// Building facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildingFacts {
    /// Type of building/part
    pub building_type: BuildingType,
    /// Is it a dwelling?
    pub dwelling: bool,
}

/// Types of buildings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingType {
    /// Residential dwelling
    Dwelling,
    /// Commercial premises
    Commercial,
    /// Part of building
    PartOfBuilding { description: String },
    /// Inhabited vehicle or vessel
    InhabitedVehicle,
    /// Other structure
    Other { description: String },
}

/// Burglary section type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BurglarySection {
    /// s.9(1)(a) - Entry with intent
    Section91A {
        /// Intent to steal, GBH, or damage?
        intent: BurglaryIntent,
    },
    /// s.9(1)(b) - Entry then commits/attempts
    Section91B {
        /// Offence committed: theft, GBH, or attempt
        offence_committed: BurglarySubOffence,
    },
}

/// Intent for s.9(1)(a) burglary
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BurglaryIntent {
    /// Intent to steal
    Steal,
    /// Intent to inflict GBH
    GBH,
    /// Intent to cause criminal damage
    CriminalDamage,
}

/// Sub-offence for s.9(1)(b) burglary
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BurglarySubOffence {
    /// Committed or attempted theft
    Theft { attempt: bool },
    /// Committed or attempted GBH
    GBH { attempt: bool },
}

/// Result of burglary analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurglaryAnalysisResult {
    /// Burglary established?
    pub established: bool,
    /// Entry analysis
    pub entry_analysis: String,
    /// Building analysis
    pub building_analysis: String,
    /// Section analysis
    pub section_analysis: String,
    /// Maximum sentence
    pub maximum_sentence: String,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Analyzer for burglary
pub struct BurglaryAnalyzer;

impl BurglaryAnalyzer {
    /// Analyze burglary
    pub fn analyze(facts: &BurglaryFacts) -> CriminalResult<BurglaryAnalysisResult> {
        let case_law = vec![
            CaseCitation::new(
                "R v Collins",
                1973,
                "QB 100",
                "Entry must be effective and substantial (now just 'effective')",
            ),
            CaseCitation::new(
                "R v Brown",
                1985,
                "Crim LR 212",
                "Partial entry (leaning through window) can be sufficient",
            ),
            CaseCitation::new(
                "R v Smith and Jones",
                1976,
                "1 WLR 672",
                "Son entering parent's home to steal is trespasser",
            ),
        ];

        let mut established = true;

        // Entry analysis
        let entry_analysis = if facts.entry.entry_occurred {
            if facts.entry.as_trespasser {
                format!(
                    "Entry as trespasser established ({:?} entry)",
                    facts.entry.entry_type
                )
            } else {
                established = false;
                "Entry occurred but not as trespasser".to_string()
            }
        } else {
            established = false;
            "No entry occurred".to_string()
        };

        // Building analysis
        let building_analysis = format!(
            "{:?} - {}",
            facts.building.building_type,
            if facts.building.dwelling {
                "dwelling"
            } else {
                "non-dwelling"
            }
        );

        // Section analysis
        let section_analysis = match &facts.section {
            BurglarySection::Section91A { intent } => {
                format!("s.9(1)(a): Entry with intent to {:?}", intent)
            }
            BurglarySection::Section91B { offence_committed } => {
                format!(
                    "s.9(1)(b): Having entered, committed {:?}",
                    offence_committed
                )
            }
        };

        // Maximum sentence depends on dwelling
        let maximum_sentence = if facts.building.dwelling {
            "14 years (dwelling)".to_string()
        } else {
            "10 years (non-dwelling)".to_string()
        };

        Ok(BurglaryAnalysisResult {
            established,
            entry_analysis,
            building_analysis,
            section_analysis,
            maximum_sentence,
            case_law,
        })
    }
}

// ============================================================================
// Handling Stolen Goods
// ============================================================================

/// Facts for handling stolen goods analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandlingFacts {
    /// The goods
    pub goods: HandledGoodsDetails,
    /// Type of handling
    pub handling_type: HandlingType,
    /// Knowledge/belief
    pub knowledge: HandlingKnowledge,
    /// Benefit element
    pub for_benefit: HandlingBenefit,
}

/// Details of handled goods
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandledGoodsDetails {
    /// Description
    pub description: String,
    /// Were goods stolen?
    pub stolen: bool,
    /// When stolen (must still be stolen at time of handling)
    pub still_stolen: bool,
}

/// Types of handling
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandlingType {
    /// Receiving
    Receiving,
    /// Undertaking retention, removal, disposal, realisation
    Undertaking { activity: String },
    /// Assisting with retention, removal, disposal, realisation
    Assisting { activity: String },
    /// Arranging to receive
    ArrangingReceive,
    /// Arranging for undertaking/assisting
    ArrangingUndertake,
}

/// Knowledge element for handling
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandlingKnowledge {
    /// Knew goods were stolen?
    pub knew_stolen: bool,
    /// Believed goods were stolen?
    pub believed_stolen: bool,
    /// Evidence
    pub evidence: Vec<String>,
}

/// Benefit element for handling
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandlingBenefit {
    /// For benefit of another (not the thief)?
    pub for_another: bool,
    /// Details
    pub details: String,
}

/// Result of handling analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandlingAnalysisResult {
    /// Offence established?
    pub established: bool,
    /// Goods analysis
    pub goods_analysis: String,
    /// Handling analysis
    pub handling_analysis: String,
    /// Knowledge analysis
    pub knowledge_analysis: String,
    /// Key case law
    pub case_law: Vec<CaseCitation>,
}

/// Analyzer for handling
pub struct HandlingAnalyzer;

impl HandlingAnalyzer {
    /// Analyze handling stolen goods
    pub fn analyze(facts: &HandlingFacts) -> CriminalResult<HandlingAnalysisResult> {
        let case_law = vec![CaseCitation::new(
            "R v Bloxham",
            1983,
            "1 AC 109",
            "Purchaser cannot 'handle' for benefit of seller (the thief)",
        )];

        let mut established = true;

        // Goods analysis
        let goods_analysis = if facts.goods.stolen && facts.goods.still_stolen {
            format!(
                "Goods ({}) were stolen and remain stolen",
                facts.goods.description
            )
        } else if !facts.goods.stolen {
            established = false;
            "Goods were not stolen".to_string()
        } else {
            established = false;
            "Goods no longer stolen (restored to owner)".to_string()
        };

        // Handling analysis
        let handling_analysis = match &facts.handling_type {
            HandlingType::Receiving => "Receiving stolen goods".to_string(),
            HandlingType::Undertaking { activity } => {
                format!("Undertaking: {}", activity)
            }
            HandlingType::Assisting { activity } => {
                if facts.for_benefit.for_another {
                    format!(
                        "Assisting with {} for benefit of {}",
                        activity, facts.for_benefit.details
                    )
                } else {
                    established = false;
                    "Assisting/undertaking requires benefit of another (not thief - R v Bloxham)"
                        .to_string()
                }
            }
            HandlingType::ArrangingReceive => "Arranging to receive".to_string(),
            HandlingType::ArrangingUndertake => "Arranging for undertaking".to_string(),
        };

        // Knowledge analysis
        let knowledge_analysis = if facts.knowledge.knew_stolen {
            "D knew goods were stolen".to_string()
        } else if facts.knowledge.believed_stolen {
            "D believed goods were stolen".to_string()
        } else {
            established = false;
            "D did not know or believe goods were stolen".to_string()
        };

        Ok(HandlingAnalysisResult {
            established,
            goods_analysis,
            handling_analysis,
            knowledge_analysis,
            case_law,
        })
    }
}

// ============================================================================
// Offence Definitions
// ============================================================================

/// Get theft offence definition
pub fn theft_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Theft")
        .statutory_source("Theft Act 1968", "s.1")
        .classification(OffenceClassification::EitherWay)
        .category(OffenceCategory::Property)
        .severity(OffenceSeverity::Moderate)
        .maximum_sentence(MaximumSentence::Years(7))
        .mens_rea(MensReaType::Dishonesty)
        .mens_rea(MensReaType::DirectIntention)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get robbery offence definition
pub fn robbery_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Robbery")
        .statutory_source("Theft Act 1968", "s.8")
        .classification(OffenceClassification::IndictableOnly)
        .category(OffenceCategory::Property)
        .severity(OffenceSeverity::VerySerious)
        .maximum_sentence(MaximumSentence::Life)
        .mens_rea(MensReaType::Dishonesty)
        .mens_rea(MensReaType::DirectIntention)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get burglary offence definition
pub fn burglary_offence(dwelling: bool) -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name(if dwelling {
            "Burglary (dwelling)"
        } else {
            "Burglary (non-dwelling)"
        })
        .statutory_source("Theft Act 1968", "s.9")
        .classification(if dwelling {
            OffenceClassification::IndictableOnly
        } else {
            OffenceClassification::EitherWay
        })
        .category(OffenceCategory::Property)
        .severity(if dwelling {
            OffenceSeverity::Serious
        } else {
            OffenceSeverity::Moderate
        })
        .maximum_sentence(MaximumSentence::Years(if dwelling { 14 } else { 10 }))
        .mens_rea(MensReaType::DirectIntention)
        .actus_reus(ActusReusElement::Act(ActType::VoluntaryAct))
        .build()
        .map_err(CriminalError::InvalidInput)
}

/// Get handling stolen goods offence definition
pub fn handling_offence() -> CriminalResult<Offence> {
    OffenceBuilder::new()
        .name("Handling Stolen Goods")
        .statutory_source("Theft Act 1968", "s.22")
        .classification(OffenceClassification::EitherWay)
        .category(OffenceCategory::Property)
        .severity(OffenceSeverity::Serious)
        .maximum_sentence(MaximumSentence::Years(14))
        .mens_rea(MensReaType::Knowledge)
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

    fn create_basic_theft_facts() -> TheftFacts {
        TheftFacts {
            appropriation: AppropriationFacts {
                description: "Took laptop from desk".into(),
                appropriation_type: AppropriationType::Taking,
                with_consent: false,
                valid_gift: false,
            },
            property: PropertyFacts {
                description: "Laptop".into(),
                property_type: PropertyType::Physical,
                value: Some(1000.0),
            },
            belonging: BelongingFacts {
                owner: Some("Victim".into()),
                belonging_type: BelongingType::Ownership,
                special_situation: None,
            },
            dishonesty: TheftDishonestyFacts {
                believed_right_in_law: false,
                believed_owner_consent: false,
                believed_owner_unknown: false,
                actual_knowledge_belief: "Knew laptop belonged to victim".into(),
                dishonest_by_ordinary_standards: true,
            },
            intention_deprive: IntentionDepriveFacts {
                intended_permanent: true,
                deemed_intention: None,
            },
        }
    }

    #[test]
    fn test_basic_theft() {
        let facts = create_basic_theft_facts();
        let result = TheftAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_theft_with_consent_still_appropriation() {
        let mut facts = create_basic_theft_facts();
        facts.appropriation.with_consent = true;

        let result = TheftAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        // Per R v Gomez, consent doesn't prevent appropriation
        assert!(analysis.appropriation.established);
    }

    #[test]
    fn test_believed_right_in_law_negates_dishonesty() {
        let mut facts = create_basic_theft_facts();
        facts.dishonesty.believed_right_in_law = true;

        let result = TheftAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.established); // s.2(1)(a) defence
    }

    #[test]
    fn test_no_intention_to_permanently_deprive() {
        let mut facts = create_basic_theft_facts();
        facts.intention_deprive.intended_permanent = false;
        facts.intention_deprive.deemed_intention = None;

        let result = TheftAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.established);
    }

    #[test]
    fn test_deemed_intention_s6() {
        let mut facts = create_basic_theft_facts();
        facts.intention_deprive.intended_permanent = false;
        facts.intention_deprive.deemed_intention = Some(DeemedIntention::TreatAsOwn);

        let result = TheftAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.intention_deprive.established);
    }

    #[test]
    fn test_electricity_cannot_be_stolen() {
        let mut facts = create_basic_theft_facts();
        facts.property.property_type = PropertyType::Electricity;

        let result = TheftAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(!analysis.property.established);
    }

    #[test]
    fn test_robbery() {
        let facts = RobberyFacts {
            theft: create_basic_theft_facts(),
            force: RobberyForceFacts {
                force_used: true,
                put_in_fear: true,
                description: "Pushed victim to ground".into(),
                at_time_of_theft: true,
                in_order_to_steal: true,
            },
        };

        let result = RobberyAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_burglary_dwelling() {
        let facts = BurglaryFacts {
            entry: EntryFacts {
                entry_occurred: true,
                entry_type: EntryType::Bodily,
                as_trespasser: true,
                evidence: vec!["Broke window to enter".into()],
            },
            building: BuildingFacts {
                building_type: BuildingType::Dwelling,
                dwelling: true,
            },
            section: BurglarySection::Section91A {
                intent: BurglaryIntent::Steal,
            },
        };

        let result = BurglaryAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
        assert!(analysis.maximum_sentence.contains("14"));
    }

    #[test]
    fn test_handling_stolen_goods() {
        let facts = HandlingFacts {
            goods: HandledGoodsDetails {
                description: "Stolen television".into(),
                stolen: true,
                still_stolen: true,
            },
            handling_type: HandlingType::Receiving,
            knowledge: HandlingKnowledge {
                knew_stolen: true,
                believed_stolen: false,
                evidence: vec!["Purchased at fraction of value".into()],
            },
            for_benefit: HandlingBenefit {
                for_another: false,
                details: "For own use".into(),
            },
        };

        let result = HandlingAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let analysis = result.expect("should succeed");
        assert!(analysis.established);
    }

    #[test]
    fn test_offence_definitions() {
        assert!(theft_offence().is_ok());
        assert!(robbery_offence().is_ok());
        assert!(burglary_offence(true).is_ok());
        assert!(burglary_offence(false).is_ok());
        assert!(handling_offence().is_ok());
    }
}
