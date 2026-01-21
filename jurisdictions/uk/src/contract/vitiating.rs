//! Vitiating Factors in Contract Law
//!
//! This module implements the law relating to factors that may render a contract
//! void, voidable, or unenforceable under English law.
//!
//! ## Misrepresentation
//!
//! A false statement of fact that induces the other party to enter the contract.
//!
//! ### Types (Misrepresentation Act 1967)
//! - **Fraudulent**: Derry v Peek \[1889\] - knowing falsity or reckless disregard
//! - **Negligent**: s.2(1) MA 1967 - burden on representor to prove reasonable belief
//! - **Innocent**: Genuine and reasonable belief in truth
//!
//! ### Remedies
//! - Rescission (subject to bars)
//! - Damages (fraudulent - deceit; negligent - s.2(1); innocent - s.2(2))
//!
//! ## Mistake
//!
//! ### Common Mistake (Bell v Lever Bros \[1932\])
//! Both parties make same mistake. Void only if:
//! - Res extincta (subject matter does not exist)
//! - Res sua (ownership already acquired)
//! - Fundamental quality (very limited - Great Peace Shipping)
//!
//! ### Mutual Mistake (Raffles v Wichelhaus \[1864\])
//! Parties at cross-purposes. No genuine agreement.
//!
//! ### Unilateral Mistake
//! One party knows other is mistaken:
//! - Identity (Shogun Finance v Hudson \[2004\])
//! - Terms (Hartog v Colin & Shields \[1939\])
//!
//! ## Duress
//!
//! ### Common Law Duress
//! - Duress to person
//! - Duress to goods
//!
//! ### Economic Duress (DSND Subsea v Petroleum Geo-Services \[2000\])
//! - Illegitimate pressure
//! - Pressure must be decisive cause
//! - No reasonable alternative
//!
//! ## Undue Influence (Royal Bank of Scotland v Etridge \[2002\])
//!
//! ### Class 1 - Actual Undue Influence
//! Overt acts of improper pressure proved.
//!
//! ### Class 2 - Presumed Undue Influence
//! - Class 2A: Automatic relationships (solicitor-client, parent-child)
//! - Class 2B: Relationships of trust and confidence proved on facts
//!
//! ## Illegality (Patel v Mirza \[2016\])
//!
//! Modern approach considers:
//! - Underlying purpose of prohibition
//! - Other relevant public policies
//! - Whether denial proportionate

use serde::{Deserialize, Serialize};

// ============================================================================
// Misrepresentation
// ============================================================================

/// Type of misrepresentation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MisrepresentationType {
    /// Fraudulent - knowing falsity or reckless disregard (Derry v Peek)
    Fraudulent,
    /// Negligent - failure to prove reasonable belief (s.2(1) MA 1967)
    Negligent,
    /// Innocent - genuine and reasonable belief in truth
    Innocent,
}

impl MisrepresentationType {
    /// Get description with case/statute reference
    pub fn description(&self) -> &'static str {
        match self {
            Self::Fraudulent => {
                "Made knowingly, without belief in truth, or recklessly careless \
                 whether true or false (Derry v Peek [1889])"
            }
            Self::Negligent => {
                "Made without reasonable grounds for believing statement true. \
                 Burden on representor to prove reasonable belief (s.2(1) MA 1967)"
            }
            Self::Innocent => {
                "Made with genuine and reasonable belief in truth. \
                 Representor had reasonable grounds for belief."
            }
        }
    }

    /// Get available damages measure
    pub fn damages_measure(&self) -> &'static str {
        match self {
            Self::Fraudulent => "Tort of deceit - all direct losses (no remoteness)",
            Self::Negligent => "s.2(1) MA 1967 - fiction of fraud (Royscot Trust v Rogerson)",
            Self::Innocent => "s.2(2) MA 1967 - damages in lieu of rescission (court discretion)",
        }
    }
}

/// Elements of misrepresentation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MisrepresentationElements {
    /// The statement made
    pub statement: String,
    /// Was it a statement of fact (not opinion/intention/law)?
    pub is_fact: bool,
    /// Was the statement false?
    pub is_false: bool,
    /// Did representor intend statement to be relied upon?
    pub intended_reliance: bool,
    /// Did representee actually rely on statement?
    pub actual_reliance: bool,
    /// Did reliance induce entry into contract?
    pub induced_contract: bool,
}

impl MisrepresentationElements {
    /// Are all elements satisfied?
    pub fn is_actionable(&self) -> bool {
        self.is_fact
            && self.is_false
            && self.intended_reliance
            && self.actual_reliance
            && self.induced_contract
    }
}

/// Misrepresentation analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MisrepresentationAnalysis {
    /// The misrepresentation elements
    pub elements: MisrepresentationElements,
    /// Type of misrepresentation
    pub misrep_type: MisrepresentationType,
    /// Are elements satisfied?
    pub actionable: bool,
    /// Is rescission available?
    pub rescission_available: bool,
    /// Bars to rescission (if any)
    pub rescission_bars: Vec<RescissionBar>,
    /// Are damages available?
    pub damages_available: bool,
    /// Analysis
    pub analysis: String,
}

impl MisrepresentationAnalysis {
    /// Analyze misrepresentation claim
    pub fn analyze(
        elements: MisrepresentationElements,
        misrep_type: MisrepresentationType,
        rescission_bars: Vec<RescissionBar>,
    ) -> Self {
        let actionable = elements.is_actionable();
        let rescission_available = actionable && rescission_bars.is_empty();
        let damages_available = actionable;

        let analysis = if !actionable {
            Self::generate_failure_analysis(&elements)
        } else {
            Self::generate_success_analysis(&elements, misrep_type, &rescission_bars)
        };

        Self {
            elements,
            misrep_type,
            actionable,
            rescission_available,
            rescission_bars,
            damages_available,
            analysis,
        }
    }

    fn generate_failure_analysis(elements: &MisrepresentationElements) -> String {
        let mut reasons = Vec::new();

        if !elements.is_fact {
            reasons
                .push("Statement was not a statement of fact (may be opinion, intention, or law)");
        }
        if !elements.is_false {
            reasons.push("Statement was not false");
        }
        if !elements.actual_reliance {
            reasons.push(
                "Representee did not actually rely on statement (JEB Fasteners v Marks Bloom)",
            );
        }
        if !elements.induced_contract {
            reasons.push("Statement did not induce entry into contract");
        }

        format!(
            "Misrepresentation claim FAILS. Reason(s): {}",
            reasons.join("; ")
        )
    }

    fn generate_success_analysis(
        elements: &MisrepresentationElements,
        misrep_type: MisrepresentationType,
        bars: &[RescissionBar],
    ) -> String {
        let mut analysis = format!(
            "{:?} misrepresentation established. Statement '{}' was false statement of \
             fact that induced contract. ",
            misrep_type,
            if elements.statement.len() > 50 {
                format!("{}...", &elements.statement[..50])
            } else {
                elements.statement.clone()
            }
        );

        if bars.is_empty() {
            analysis.push_str("Rescission AVAILABLE. ");
        } else {
            analysis.push_str(&format!("Rescission BARRED by: {:?}. ", bars));
        }

        analysis.push_str(&format!(
            "Damages measure: {}",
            misrep_type.damages_measure()
        ));

        analysis
    }
}

/// Bars to rescission
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RescissionBar {
    /// Affirmation after discovering misrepresentation
    Affirmation,
    /// Lapse of time (Leaf v International Galleries \[1950\])
    LapseOfTime,
    /// Third party acquired rights
    ThirdPartyRights,
    /// Restitutio in integrum impossible
    RestitutioImpossible,
    /// Contract fully executed (for innocent misrep only)
    ExecutedContract,
}

// ============================================================================
// Mistake
// ============================================================================

/// Type of mistake
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MistakeType {
    /// Common mistake - both parties make same mistake
    Common,
    /// Mutual mistake - parties at cross-purposes
    Mutual,
    /// Unilateral mistake - one party knows other mistaken
    Unilateral,
}

/// Category of common mistake
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommonMistakeCategory {
    /// Res extincta - subject matter does not exist
    ResExtincta,
    /// Res sua - claimant already owns subject matter
    ResSua,
    /// Quality - mistake as to fundamental quality (very limited)
    Quality,
}

/// Category of unilateral mistake
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnilateralMistakeCategory {
    /// Mistake as to identity
    Identity,
    /// Mistake as to terms
    Terms,
}

/// Mistake analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MistakeAnalysis {
    /// Type of mistake
    pub mistake_type: MistakeType,
    /// Description of the mistake
    pub description: String,
    /// Specific category (for common/unilateral)
    pub category: Option<String>,
    /// Does mistake make contract void?
    pub makes_void: bool,
    /// Does mistake make contract voidable?
    pub makes_voidable: bool,
    /// Analysis
    pub analysis: String,
}

impl MistakeAnalysis {
    /// Analyze common mistake
    pub fn analyze_common(
        description: &str,
        category: CommonMistakeCategory,
        is_fundamental: bool,
    ) -> Self {
        let makes_void = match category {
            CommonMistakeCategory::ResExtincta => true,
            CommonMistakeCategory::ResSua => true,
            CommonMistakeCategory::Quality => is_fundamental,
        };

        let analysis = match category {
            CommonMistakeCategory::ResExtincta => {
                format!(
                    "Common mistake as to existence: '{}'. Subject matter does not exist. \
                     Following Couturier v Hastie [1856], contract is VOID.",
                    description
                )
            }
            CommonMistakeCategory::ResSua => {
                format!(
                    "Common mistake - res sua: '{}'. Buyer already owns the property. \
                     Following Cooper v Phibbs [1867], contract VOID.",
                    description
                )
            }
            CommonMistakeCategory::Quality => {
                if is_fundamental {
                    format!(
                        "Common mistake as to quality: '{}'. This is sufficiently fundamental \
                         to render subject matter essentially different. Following Bell v \
                         Lever Bros [1932] and Great Peace Shipping [2002], contract VOID. \
                         Note: this is extremely rare - quality mistake almost never voids.",
                        description
                    )
                } else {
                    format!(
                        "Common mistake as to quality: '{}'. Following Bell v Lever Bros [1932], \
                         mistake as to quality rarely makes contract void. Subject matter must \
                         be 'essentially and radically different'. Here, mistake is not \
                         sufficiently fundamental. Contract VALID.",
                        description
                    )
                }
            }
        };

        Self {
            mistake_type: MistakeType::Common,
            description: description.to_string(),
            category: Some(format!("{:?}", category)),
            makes_void,
            makes_voidable: false,
            analysis,
        }
    }

    /// Analyze mutual mistake
    pub fn analyze_mutual(description: &str, is_genuinely_ambiguous: bool) -> Self {
        let makes_void = is_genuinely_ambiguous;

        let analysis = if is_genuinely_ambiguous {
            format!(
                "Mutual mistake: '{}'. Parties were genuinely at cross-purposes with no \
                 objective meaning ascertainable. Following Raffles v Wichelhaus [1864], \
                 no genuine agreement - contract VOID.",
                description
            )
        } else {
            format!(
                "Alleged mutual mistake: '{}'. However, an objective reasonable meaning \
                 can be ascertained. Court will apply objective meaning. Contract VALID.",
                description
            )
        };

        Self {
            mistake_type: MistakeType::Mutual,
            description: description.to_string(),
            category: None,
            makes_void,
            makes_voidable: false,
            analysis,
        }
    }

    /// Analyze unilateral mistake
    pub fn analyze_unilateral(
        description: &str,
        category: UnilateralMistakeCategory,
        other_party_knew: bool,
        face_to_face: bool,
    ) -> Self {
        let (makes_void, makes_voidable, analysis) = match category {
            UnilateralMistakeCategory::Identity => {
                if face_to_face {
                    // Face to face: contract with person present, voidable for fraud
                    (
                        false,
                        true,
                        format!(
                            "Unilateral mistake as to identity: '{}'. Transaction was \
                             face-to-face. Following Shogun Finance v Hudson [2004], contract \
                             is with person present - identity mistake makes contract VOIDABLE \
                             (for fraud) not void.",
                            description
                        ),
                    )
                } else if other_party_knew {
                    // Inter absentes with knowledge: may be void
                    (
                        true,
                        false,
                        format!(
                            "Unilateral mistake as to identity: '{}'. Transaction was \
                             inter absentes and other party knew of mistake. Following Cundy v \
                             Lindsay [1878] and Shogun Finance v Hudson [2004], contract \
                             VOID - no contract with rogue, title cannot pass to third party.",
                            description
                        ),
                    )
                } else {
                    (
                        false,
                        false,
                        format!(
                            "Alleged identity mistake: '{}'. Other party did not know of \
                             mistake. Contract VALID.",
                            description
                        ),
                    )
                }
            }
            UnilateralMistakeCategory::Terms => {
                if other_party_knew {
                    (
                        true,
                        false,
                        format!(
                            "Unilateral mistake as to terms: '{}'. Other party knew of \
                             mistake and sought to take advantage. Following Hartog v Colin & \
                             Shields [1939], contract VOID - cannot snap up offer made by mistake.",
                            description
                        ),
                    )
                } else {
                    (
                        false,
                        false,
                        format!(
                            "Alleged mistake as to terms: '{}'. Other party did not know \
                             of mistake. Contract VALID - mistaken party bound to objective \
                             meaning.",
                            description
                        ),
                    )
                }
            }
        };

        Self {
            mistake_type: MistakeType::Unilateral,
            description: description.to_string(),
            category: Some(format!("{:?}", category)),
            makes_void,
            makes_voidable,
            analysis,
        }
    }
}

// ============================================================================
// Duress
// ============================================================================

/// Type of duress
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DuressType {
    /// Duress to person (threats of violence)
    ToPerson,
    /// Duress to goods (wrongful detention)
    ToGoods,
    /// Economic duress
    Economic,
}

impl DuressType {
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::ToPerson => "Threat of physical violence to person",
            Self::ToGoods => "Wrongful detention of or threat to goods",
            Self::Economic => "Illegitimate commercial/economic pressure",
        }
    }
}

/// Elements of economic duress (DSND Subsea v Petroleum Geo-Services \[2000\])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EconomicDuressElements {
    /// Description of the pressure
    pub pressure_description: String,
    /// Was the pressure illegitimate?
    pub illegitimate_pressure: bool,
    /// Factors making pressure illegitimate
    pub illegitimacy_factors: Vec<IllegitimacyFactor>,
    /// Was pressure a significant cause of entering contract?
    pub significant_cause: bool,
    /// Did victim have reasonable alternative?
    pub had_reasonable_alternative: bool,
    /// Did victim protest at the time?
    pub protested: bool,
}

/// Factors that make pressure illegitimate
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IllegitimacyFactor {
    /// Threat to breach contract
    ThreatToBreachContract,
    /// Bad faith demand
    BadFaithDemand,
    /// Exploitation of victim's vulnerability
    ExploitationOfVulnerability,
    /// Threat to commit unlawful act
    ThreatOfUnlawfulAct,
    /// Withholding what is lawfully owed
    WithholdingWhatOwed,
}

/// Duress analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuressAnalysis {
    /// Type of duress
    pub duress_type: DuressType,
    /// For economic duress: elements analysis
    pub economic_elements: Option<EconomicDuressElements>,
    /// Is duress established?
    pub duress_established: bool,
    /// Effect on contract
    pub effect: ContractEffect,
    /// Analysis
    pub analysis: String,
}

/// Effect of vitiating factor on contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractEffect {
    /// Contract void ab initio
    Void,
    /// Contract voidable at option of victim
    Voidable,
    /// Contract valid
    Valid,
    /// Contract unenforceable
    Unenforceable,
}

impl DuressAnalysis {
    /// Analyze duress to person
    pub fn analyze_duress_to_person(description: &str, threat_proved: bool) -> Self {
        let duress_established = threat_proved;
        let effect = if duress_established {
            ContractEffect::Voidable
        } else {
            ContractEffect::Valid
        };

        let analysis = if duress_established {
            format!(
                "Duress to person: '{}'. Threat of physical violence established. \
                 Contract VOIDABLE at victim's option. Barton v Armstrong [1976].",
                description
            )
        } else {
            format!(
                "Alleged duress to person: '{}'. Threat not proved. Contract VALID.",
                description
            )
        };

        Self {
            duress_type: DuressType::ToPerson,
            economic_elements: None,
            duress_established,
            effect,
            analysis,
        }
    }

    /// Analyze economic duress
    pub fn analyze_economic(elements: EconomicDuressElements) -> Self {
        // DSND Subsea requirements
        let duress_established = elements.illegitimate_pressure
            && elements.significant_cause
            && !elements.had_reasonable_alternative;

        let effect = if duress_established {
            ContractEffect::Voidable
        } else {
            ContractEffect::Valid
        };

        let analysis = if duress_established {
            format!(
                "Economic duress established. Pressure: '{}'. Following DSND Subsea v \
                 Petroleum Geo-Services [2000]: (1) Pressure was illegitimate - {}; \
                 (2) Pressure was significant cause of entering contract; \
                 (3) Victim had no reasonable alternative. Contract VOIDABLE.",
                elements.pressure_description,
                elements
                    .illegitimacy_factors
                    .iter()
                    .map(|f| format!("{:?}", f))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            let mut reasons = Vec::new();
            if !elements.illegitimate_pressure {
                reasons
                    .push("pressure was not illegitimate - lawful to threaten what entitled to do");
            }
            if !elements.significant_cause {
                reasons.push("pressure was not significant cause of entry");
            }
            if elements.had_reasonable_alternative {
                reasons.push("victim had reasonable alternative available");
            }

            format!(
                "Economic duress NOT established. Pressure: '{}'. Failed element(s): {}. \
                 Following DSND Subsea v Petroleum Geo-Services [2000], all elements \
                 must be satisfied. Contract VALID.",
                elements.pressure_description,
                reasons.join("; ")
            )
        };

        Self {
            duress_type: DuressType::Economic,
            economic_elements: Some(elements),
            duress_established,
            effect,
            analysis,
        }
    }
}

// ============================================================================
// Undue Influence
// ============================================================================

/// Class of undue influence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UndueInfluenceClass {
    /// Class 1: Actual undue influence (overt acts proved)
    Class1Actual,
    /// Class 2A: Presumed - recognized relationships
    Class2ARecognized,
    /// Class 2B: Presumed - relationship of trust proved
    Class2BProved,
}

/// Recognized relationships for Class 2A
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecognizedRelationship {
    /// Solicitor and client
    SolicitorClient,
    /// Doctor and patient
    DoctorPatient,
    /// Parent and child (child as victim)
    ParentChild,
    /// Trustee and beneficiary
    TrusteeBeneficiary,
    /// Religious adviser and follower
    ReligiousAdviser,
    /// Guardian and ward
    GuardianWard,
}

impl RecognizedRelationship {
    /// Does this relationship give rise to presumption?
    pub fn presumption_applies(&self) -> bool {
        // All recognized relationships give rise to presumption
        true
    }
}

/// Undue influence analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UndueInfluenceAnalysis {
    /// Class of undue influence claimed
    pub class: UndueInfluenceClass,
    /// For Class 2A: recognized relationship
    pub recognized_relationship: Option<RecognizedRelationship>,
    /// Description of the influence
    pub description: String,
    /// Is transaction calling for explanation?
    pub calls_for_explanation: bool,
    /// Has influence been rebutted?
    pub rebutted: bool,
    /// Rebuttal evidence (e.g., independent advice)
    pub rebuttal_evidence: Vec<String>,
    /// Is undue influence established?
    pub established: bool,
    /// Third party involvement (banks, etc.)
    pub third_party_notice: Option<ThirdPartyNotice>,
    /// Effect on contract
    pub effect: ContractEffect,
    /// Analysis
    pub analysis: String,
}

/// Third party notice (relevant for bank cases)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThirdPartyNotice {
    /// Type of third party
    pub party_type: String,
    /// Was third party on notice of potential undue influence?
    pub on_notice: bool,
    /// Did third party take reasonable steps (Etridge guidelines)?
    pub reasonable_steps_taken: bool,
    /// Steps taken
    pub steps_description: Vec<String>,
}

impl UndueInfluenceAnalysis {
    /// Analyze Class 1 (actual) undue influence
    pub fn analyze_actual(description: &str, overt_acts_proved: bool) -> Self {
        let established = overt_acts_proved;
        let effect = if established {
            ContractEffect::Voidable
        } else {
            ContractEffect::Valid
        };

        let analysis = if established {
            format!(
                "Class 1 actual undue influence established: '{}'. Overt acts of \
                 improper pressure have been proved. Contract VOIDABLE.",
                description
            )
        } else {
            format!(
                "Actual undue influence claimed but NOT established: '{}'. \
                 Claimant must prove overt acts of improper pressure. Contract VALID.",
                description
            )
        };

        Self {
            class: UndueInfluenceClass::Class1Actual,
            recognized_relationship: None,
            description: description.to_string(),
            calls_for_explanation: false,
            rebutted: false,
            rebuttal_evidence: vec![],
            established,
            third_party_notice: None,
            effect,
            analysis,
        }
    }

    /// Analyze Class 2A (presumed - recognized relationship)
    pub fn analyze_class_2a(
        relationship: RecognizedRelationship,
        description: &str,
        calls_for_explanation: bool,
        independent_advice_given: bool,
    ) -> Self {
        // Presumption arises if relationship + transaction calls for explanation
        let presumption_arises = calls_for_explanation;
        let rebutted = independent_advice_given;
        let established = presumption_arises && !rebutted;

        let effect = if established {
            ContractEffect::Voidable
        } else {
            ContractEffect::Valid
        };

        let rebuttal_evidence = if independent_advice_given {
            vec!["Independent legal advice obtained".to_string()]
        } else {
            vec![]
        };

        let analysis = if !calls_for_explanation {
            format!(
                "Recognized relationship ({:?}) but transaction does not call for \
                 explanation - not manifestly disadvantageous. No presumption of undue \
                 influence. Contract VALID.",
                relationship
            )
        } else if rebutted {
            format!(
                "Class 2A: Recognized relationship ({:?}). Transaction calls for explanation. \
                 Presumption of undue influence arises. However, presumption REBUTTED by \
                 independent legal advice. Following Royal Bank of Scotland v Etridge [2002], \
                 contract VALID.",
                relationship
            )
        } else {
            format!(
                "Class 2A undue influence established. Recognized relationship ({:?}). \
                 Transaction '{}' calls for explanation. Presumption of undue influence \
                 arises (Royal Bank of Scotland v Etridge [2002]). Not rebutted. \
                 Contract VOIDABLE.",
                relationship, description
            )
        };

        Self {
            class: UndueInfluenceClass::Class2ARecognized,
            recognized_relationship: Some(relationship),
            description: description.to_string(),
            calls_for_explanation,
            rebutted,
            rebuttal_evidence,
            established,
            third_party_notice: None,
            effect,
            analysis,
        }
    }

    /// Analyze Class 2B (relationship of trust proved)
    pub fn analyze_class_2b(
        description: &str,
        relationship_of_trust_proved: bool,
        calls_for_explanation: bool,
        rebutted: bool,
        rebuttal_evidence: Vec<String>,
    ) -> Self {
        let established = relationship_of_trust_proved && calls_for_explanation && !rebutted;

        let effect = if established {
            ContractEffect::Voidable
        } else {
            ContractEffect::Valid
        };

        let analysis = if !relationship_of_trust_proved {
            format!(
                "Class 2B: No relationship of trust and confidence proved for '{}'. \
                 Following Royal Bank of Scotland v Etridge [2002], claimant must prove \
                 such relationship exists. Contract VALID.",
                description
            )
        } else if !calls_for_explanation {
            "Relationship of trust proved but transaction does not call for explanation. \
             No presumption arises. Contract VALID."
                .to_string()
        } else if rebutted {
            format!(
                "Class 2B: Relationship of trust proved. Transaction '{}' calls for \
                 explanation. Presumption arises but REBUTTED by: {}. Contract VALID.",
                description,
                rebuttal_evidence.join("; ")
            )
        } else {
            format!(
                "Class 2B undue influence established. Relationship of trust proved. \
                 Transaction '{}' calls for explanation. Presumption not rebutted. \
                 Contract VOIDABLE.",
                description
            )
        };

        Self {
            class: UndueInfluenceClass::Class2BProved,
            recognized_relationship: None,
            description: description.to_string(),
            calls_for_explanation,
            rebutted,
            rebuttal_evidence,
            established,
            third_party_notice: None,
            effect,
            analysis,
        }
    }
}

// ============================================================================
// Illegality
// ============================================================================

/// Type of illegality
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IllegalityType {
    /// Illegal at formation (e.g., contract to commit crime)
    IllegalAtFormation,
    /// Illegal in performance
    IllegalInPerformance,
    /// Contract contrary to public policy
    ContraryToPublicPolicy,
    /// Statutory illegality
    Statutory,
}

/// Illegality analysis (Patel v Mirza \[2016\] approach)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IllegalityAnalysis {
    /// Type of illegality
    pub illegality_type: IllegalityType,
    /// Description of the illegality
    pub description: String,
    /// Patel v Mirza factors
    pub patel_factors: PatelVMirzaFactors,
    /// Effect on contract
    pub effect: ContractEffect,
    /// Can unjust enrichment claim proceed?
    pub restitution_available: bool,
    /// Analysis
    pub analysis: String,
}

/// Patel v Mirza \[2016\] factors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatelVMirzaFactors {
    /// What is the underlying purpose of the prohibition?
    pub underlying_purpose: String,
    /// Would denial of claim enhance or undermine purpose?
    pub enhances_purpose: bool,
    /// Other relevant public policies
    pub other_policies: Vec<String>,
    /// Is denial of claim proportionate response?
    pub denial_proportionate: bool,
}

impl IllegalityAnalysis {
    /// Analyze illegality using Patel v Mirza approach
    pub fn analyze(
        illegality_type: IllegalityType,
        description: &str,
        factors: PatelVMirzaFactors,
    ) -> Self {
        // Under Patel v Mirza, court considers range of factors
        // No automatic bar - proportionate response required
        let contract_unenforceable = factors.enhances_purpose && factors.denial_proportionate;

        // Restitution may still be available even if contract unenforceable
        let restitution_available = !factors.denial_proportionate
            || factors
                .other_policies
                .iter()
                .any(|p| p.contains("unjust enrichment"));

        let effect = if contract_unenforceable {
            ContractEffect::Unenforceable
        } else {
            ContractEffect::Valid
        };

        let analysis = format!(
            "Illegality analysis (Patel v Mirza [2016]): '{}'. Type: {:?}. \
             Underlying purpose of prohibition: '{}'. \
             Would denial enhance purpose: {}. Denial proportionate: {}. \
             Other policies: {}. \
             Contract is {}. Restitution {}available.",
            description,
            illegality_type,
            factors.underlying_purpose,
            if factors.enhances_purpose {
                "Yes"
            } else {
                "No"
            },
            if factors.denial_proportionate {
                "Yes"
            } else {
                "No"
            },
            if factors.other_policies.is_empty() {
                "None".to_string()
            } else {
                factors.other_policies.join("; ")
            },
            match effect {
                ContractEffect::Unenforceable => "UNENFORCEABLE",
                ContractEffect::Valid => "ENFORCEABLE (illegality no bar)",
                _ => "affected",
            },
            if restitution_available { "" } else { "NOT " }
        );

        Self {
            illegality_type,
            description: description.to_string(),
            patel_factors: factors,
            effect,
            restitution_available,
            analysis,
        }
    }
}

// ============================================================================
// Comprehensive Vitiating Factor Analysis
// ============================================================================

/// Result of vitiating factor analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VitiatingFactorResult {
    /// Factor analyzed
    pub factor: VitiatingFactorType,
    /// Effect on contract
    pub effect: ContractEffect,
    /// Remedies available
    pub remedies: Vec<AvailableRemedy>,
    /// Summary
    pub summary: String,
}

/// Type of vitiating factor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VitiatingFactorType {
    /// Misrepresentation
    Misrepresentation,
    /// Mistake
    Mistake,
    /// Duress
    Duress,
    /// Undue influence
    UndueInfluence,
    /// Illegality
    Illegality,
}

/// Remedies for vitiating factors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AvailableRemedy {
    /// Rescission
    Rescission,
    /// Damages
    Damages,
    /// Restitution/unjust enrichment
    Restitution,
    /// No remedy (contract valid)
    None,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraudulent_misrepresentation() {
        let elements = MisrepresentationElements {
            statement: "Car has only 30,000 miles".to_string(),
            is_fact: true,
            is_false: true,
            intended_reliance: true,
            actual_reliance: true,
            induced_contract: true,
        };
        let analysis =
            MisrepresentationAnalysis::analyze(elements, MisrepresentationType::Fraudulent, vec![]);
        assert!(analysis.actionable);
        assert!(analysis.rescission_available);
        assert!(analysis.damages_available);
    }

    #[test]
    fn test_misrepresentation_no_reliance() {
        let elements = MisrepresentationElements {
            statement: "Excellent condition".to_string(),
            is_fact: true,
            is_false: true,
            intended_reliance: true,
            actual_reliance: false, // Did not rely
            induced_contract: false,
        };
        let analysis =
            MisrepresentationAnalysis::analyze(elements, MisrepresentationType::Negligent, vec![]);
        assert!(!analysis.actionable);
    }

    #[test]
    fn test_common_mistake_res_extincta() {
        let analysis = MistakeAnalysis::analyze_common(
            "Ship had already sunk before contract",
            CommonMistakeCategory::ResExtincta,
            true,
        );
        assert!(analysis.makes_void);
    }

    #[test]
    fn test_common_mistake_quality_not_fundamental() {
        let analysis = MistakeAnalysis::analyze_common(
            "Painting turned out to be copy not original",
            CommonMistakeCategory::Quality,
            false, // Not fundamental
        );
        assert!(!analysis.makes_void);
        assert!(analysis.analysis.contains("Bell v Lever Bros"));
    }

    #[test]
    fn test_unilateral_mistake_identity_face_to_face() {
        let analysis = MistakeAnalysis::analyze_unilateral(
            "Rogue impersonated known buyer",
            UnilateralMistakeCategory::Identity,
            true,
            true, // Face to face
        );
        assert!(!analysis.makes_void);
        assert!(analysis.makes_voidable);
        assert!(analysis.analysis.contains("Shogun Finance"));
    }

    #[test]
    fn test_economic_duress_established() {
        let elements = EconomicDuressElements {
            pressure_description: "Threatened to terminate unless price increased".to_string(),
            illegitimate_pressure: true,
            illegitimacy_factors: vec![
                IllegitimacyFactor::ThreatToBreachContract,
                IllegitimacyFactor::BadFaithDemand,
            ],
            significant_cause: true,
            had_reasonable_alternative: false,
            protested: true,
        };
        let analysis = DuressAnalysis::analyze_economic(elements);
        assert!(analysis.duress_established);
        assert_eq!(analysis.effect, ContractEffect::Voidable);
    }

    #[test]
    fn test_economic_duress_reasonable_alternative() {
        let elements = EconomicDuressElements {
            pressure_description: "Demanded higher price".to_string(),
            illegitimate_pressure: true,
            illegitimacy_factors: vec![IllegitimacyFactor::BadFaithDemand],
            significant_cause: true,
            had_reasonable_alternative: true, // Could have gone elsewhere
            protested: false,
        };
        let analysis = DuressAnalysis::analyze_economic(elements);
        assert!(!analysis.duress_established);
    }

    #[test]
    fn test_undue_influence_class_2a() {
        let analysis = UndueInfluenceAnalysis::analyze_class_2a(
            RecognizedRelationship::SolicitorClient,
            "Gift of £500,000 to solicitor",
            true,  // Calls for explanation
            false, // No independent advice
        );
        assert!(analysis.established);
        assert_eq!(analysis.effect, ContractEffect::Voidable);
    }

    #[test]
    fn test_undue_influence_rebutted() {
        let analysis = UndueInfluenceAnalysis::analyze_class_2a(
            RecognizedRelationship::ParentChild,
            "Gift to parent",
            true,
            true, // Independent advice given
        );
        assert!(!analysis.established);
        assert!(analysis.rebutted);
    }

    #[test]
    fn test_illegality_patel_v_mirza() {
        let factors = PatelVMirzaFactors {
            underlying_purpose: "Prevent insider trading".to_string(),
            enhances_purpose: false, // Contract already void, no need to deny restitution
            other_policies: vec!["Prevent unjust enrichment".to_string()],
            denial_proportionate: false,
        };
        let analysis = IllegalityAnalysis::analyze(
            IllegalityType::Statutory,
            "Payment for insider information",
            factors,
        );
        assert!(analysis.restitution_available);
    }
}
