//! Unconscionable Conduct
//!
//! Analysis of unconscionable conduct under:
//! - Australian Consumer Law ss.20-22 (ACL)
//! - Common law unconscionable dealing
//! - Equitable unconscionability
//!
//! Key principles from:
//! - Blomley v Ryan (1956) - Classic unconscionability
//! - Commercial Bank of Australia v Amadio (1983) - Special disadvantage
//! - ACCC v CG Berbatis Holdings (2003) - Statutory unconscionability
//! - Kakavas v Crown Melbourne (2013) - Problem gambling case

use serde::{Deserialize, Serialize};

// ============================================================================
// Unconscionability Types
// ============================================================================

/// Type of unconscionable conduct
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnconscionabilityType {
    /// Common law unconscionable dealing
    CommonLaw,
    /// Equitable unconscionability
    Equitable,
    /// Statutory (ACL s.20) - general
    StatutoryGeneral,
    /// Statutory (ACL s.21) - in connection with goods/services
    StatutoryGoodsServices,
    /// Statutory (ACL s.22) - small business
    StatutorySmallBusiness,
}

/// Elements of unconscionable conduct
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnconscionableElement {
    /// Special disadvantage
    SpecialDisadvantage,
    /// Inequality of bargaining power
    InequalityOfBargainingPower,
    /// Unconscientious exploitation
    UnsonscientiousExploitation,
    /// Procedural unfairness
    ProceduralUnfairness,
    /// Substantive unfairness
    SubstantiveUnfairness,
    /// Lack of independent advice
    LackOfIndependentAdvice,
    /// Undue pressure or influence
    UnduePressureOrInfluence,
    /// Vulnerable party
    VulnerableParty,
}

/// Type of special disadvantage
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecialDisadvantage {
    /// Age (elderly)
    Age,
    /// Illness (physical or mental)
    Illness,
    /// Intellectual impairment
    IntellectualImpairment,
    /// Poverty or financial distress
    FinancialDistress,
    /// Illiteracy or language barrier
    LiteracyLanguage,
    /// Emotional dependence
    EmotionalDependence,
    /// Addiction or compulsion
    Addiction,
    /// Lack of business experience
    LackOfBusinessExperience,
    /// Other vulnerability
    Other(String),
}

/// Power imbalance factors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerImbalance {
    /// Relative bargaining strength disparity
    BargainingStrength,
    /// Information asymmetry
    InformationAsymmetry,
    /// Economic dependence
    EconomicDependence,
    /// Monopoly or market power
    MarketPower,
    /// Professional vs layperson
    ProfessionalAdvantage,
    /// Sophistication disparity
    SophisticationDisparity,
}

/// Procedural unconscionability factors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProceduralFactor {
    /// High pressure tactics
    HighPressureTactics,
    /// Lack of time to consider
    NoTimeToConsider,
    /// Complex or technical language
    ComplexLanguage,
    /// Hidden or buried terms
    HiddenTerms,
    /// No opportunity to negotiate
    NoNegotiation,
    /// Refused independent advice
    RefusedAdvice,
    /// Misrepresentation of terms
    MisrepresentedTerms,
    /// Unconscionable system of conduct
    SystemOfConduct,
}

/// Substantive unconscionability factors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubstantiveFactor {
    /// Grossly excessive price
    ExcessivePrice,
    /// Extremely one-sided terms
    OneSidedTerms,
    /// Absence of meaningful choice
    NoMeaningfulChoice,
    /// Oppressive terms
    OppressiveTerms,
    /// Harsh or unjust outcome
    HarshOutcome,
    /// Manifestly unfair advantage
    UnfairAdvantage,
    /// Exploitation of vulnerability
    ExploitationOfVulnerability,
}

/// ACL s.22 considerations for small business
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SmallBusinessConsideration {
    /// Relative bargaining power
    RelativeBargainingPower,
    /// Whether conditions were reasonably necessary
    ConditionsNecessity,
    /// Whether party could understand documents
    AbilityToUnderstand,
    /// Whether undue influence or pressure
    UnduePressure,
    /// Amount for which services could be supplied
    MarketPrice,
    /// Extent to which party's conduct was consistent with good faith
    GoodFaithConduct,
    /// Availability of equivalent goods/services
    AlternativesAvailable,
    /// Whether party was required to comply with conditions
    RequiredCompliance,
}

// ============================================================================
// Common Law Unconscionability
// ============================================================================

/// Common law unconscionable dealing analyzer
pub struct CommonLawUnconscionabilityAnalyzer;

impl CommonLawUnconscionabilityAnalyzer {
    /// Analyze common law unconscionable dealing
    ///
    /// Based on Blomley v Ryan and Amadio:
    /// 1. Special disadvantage of one party
    /// 2. Other party knew or ought to have known of disadvantage
    /// 3. Unconscientious exploitation of disadvantage
    pub fn analyze(facts: &CommonLawUnconscionabilityFacts) -> CommonLawUnconscionabilityResult {
        let special_disadvantage = Self::assess_special_disadvantage(facts);
        let knowledge = Self::assess_knowledge(facts);
        let exploitation = special_disadvantage && knowledge && Self::assess_exploitation(facts);

        let equitable_fraud = exploitation && facts.unconscientious_conduct;

        let available_remedies = if exploitation {
            Self::determine_remedies(facts)
        } else {
            Vec::new()
        };

        let reasoning = Self::build_reasoning(facts, special_disadvantage, knowledge, exploitation);

        CommonLawUnconscionabilityResult {
            unconscionability_type: UnconscionabilityType::CommonLaw,
            special_disadvantage_established: special_disadvantage,
            disadvantage_types: facts.disadvantage_types.clone(),
            knowledge_established: knowledge,
            exploitation_established: exploitation,
            equitable_fraud,
            available_remedies,
            reasoning,
        }
    }

    /// Assess special disadvantage (Amadio test)
    fn assess_special_disadvantage(facts: &CommonLawUnconscionabilityFacts) -> bool {
        // Must be serious disability or condition affecting capacity
        // to make judgment or protect own interests
        !facts.disadvantage_types.is_empty()
            && (facts.seriously_impaired_judgment
                || facts.unable_to_protect_interests
                || facts.vulnerable_to_exploitation)
    }

    /// Assess knowledge (actual or constructive)
    fn assess_knowledge(facts: &CommonLawUnconscionabilityFacts) -> bool {
        // Other party knew or ought to have known
        facts.actual_knowledge_of_disadvantage
            || (facts.circumstances_putting_on_notice && !facts.made_reasonable_inquiries)
            || facts.willful_blindness
    }

    /// Assess unconscientious exploitation
    fn assess_exploitation(facts: &CommonLawUnconscionabilityFacts) -> bool {
        // Taking advantage of special disadvantage in unconscientious manner
        facts.unconscientious_conduct
            && (facts.procured_substantially_unfair_advantage
                || facts.transaction_manifestly_disadvantageous
                || facts.terms_oppressive_or_harsh)
    }

    /// Determine available remedies
    fn determine_remedies(facts: &CommonLawUnconscionabilityFacts) -> Vec<UnconscionableRemedy> {
        let mut remedies = Vec::new();

        // Rescission generally available
        remedies.push(UnconscionableRemedy::Rescission);

        // Set aside transaction
        remedies.push(UnconscionableRemedy::SetAside);

        // Restitution
        if facts.benefits_transferred {
            remedies.push(UnconscionableRemedy::Restitution);
        }

        // Equitable compensation
        if facts.loss_suffered {
            remedies.push(UnconscionableRemedy::EquitableCompensation);
        }

        // Account of profits
        if facts.profits_made_by_wrongdoer {
            remedies.push(UnconscionableRemedy::AccountOfProfits);
        }

        remedies
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &CommonLawUnconscionabilityFacts,
        disadvantage: bool,
        knowledge: bool,
        exploitation: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push(
            "Common law unconscionable dealing analysis (Blomley v Ryan; Amadio)".to_string(),
        );

        // Element 1: Special disadvantage
        if disadvantage {
            parts.push("Special disadvantage established".to_string());

            for dt in &facts.disadvantage_types {
                match dt {
                    SpecialDisadvantage::Age => {
                        parts.push("Disadvantage from age/elderly status".to_string());
                    }
                    SpecialDisadvantage::Illness => {
                        parts.push("Disadvantage from illness affecting judgment".to_string());
                    }
                    SpecialDisadvantage::FinancialDistress => {
                        parts.push("Disadvantage from financial distress".to_string());
                    }
                    SpecialDisadvantage::LiteracyLanguage => {
                        parts.push("Disadvantage from illiteracy or language barrier".to_string());
                    }
                    SpecialDisadvantage::Addiction => {
                        parts.push("Disadvantage from addiction or compulsion".to_string());
                    }
                    _ => {}
                }
            }

            if facts.unable_to_protect_interests {
                parts.push("Party unable to protect own interests".to_string());
            }
        } else {
            parts.push("No special disadvantage established".to_string());
            return parts.join(". ");
        }

        // Element 2: Knowledge
        if knowledge {
            parts.push("Other party knew or ought to have known of disadvantage".to_string());

            if facts.actual_knowledge_of_disadvantage {
                parts.push("Actual knowledge of special disadvantage".to_string());
            } else if facts.circumstances_putting_on_notice {
                parts
                    .push("Circumstances put party on notice - constructive knowledge".to_string());
            }
        } else {
            parts.push("No knowledge of disadvantage established".to_string());
            return parts.join(". ");
        }

        // Element 3: Unconscientious exploitation
        if exploitation {
            parts.push("Unconscientious exploitation established".to_string());

            if facts.procured_substantially_unfair_advantage {
                parts.push("Procured substantially unfair advantage".to_string());
            }

            if facts.transaction_manifestly_disadvantageous {
                parts.push("Transaction manifestly disadvantageous".to_string());
            }

            parts.push("Transaction liable to be set aside in equity".to_string());
        } else {
            parts.push("No unconscientious exploitation".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for common law unconscionability
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommonLawUnconscionabilityFacts {
    // Special disadvantage
    /// Types of disadvantage
    pub disadvantage_types: Vec<SpecialDisadvantage>,
    /// Seriously impaired judgment
    pub seriously_impaired_judgment: bool,
    /// Unable to protect own interests
    pub unable_to_protect_interests: bool,
    /// Vulnerable to exploitation
    pub vulnerable_to_exploitation: bool,

    // Knowledge
    /// Actual knowledge of disadvantage
    pub actual_knowledge_of_disadvantage: bool,
    /// Circumstances putting on notice
    pub circumstances_putting_on_notice: bool,
    /// Made reasonable inquiries
    pub made_reasonable_inquiries: bool,
    /// Willful blindness
    pub willful_blindness: bool,

    // Exploitation
    /// Unconscientious conduct
    pub unconscientious_conduct: bool,
    /// Procured substantially unfair advantage
    pub procured_substantially_unfair_advantage: bool,
    /// Transaction manifestly disadvantageous
    pub transaction_manifestly_disadvantageous: bool,
    /// Terms oppressive or harsh
    pub terms_oppressive_or_harsh: bool,

    // Remedies factors
    /// Benefits transferred
    pub benefits_transferred: bool,
    /// Loss suffered
    pub loss_suffered: bool,
    /// Profits made by wrongdoer
    pub profits_made_by_wrongdoer: bool,
}

/// Result of common law unconscionability analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonLawUnconscionabilityResult {
    /// Type of unconscionability
    pub unconscionability_type: UnconscionabilityType,
    /// Special disadvantage established
    pub special_disadvantage_established: bool,
    /// Types of disadvantage
    pub disadvantage_types: Vec<SpecialDisadvantage>,
    /// Knowledge established
    pub knowledge_established: bool,
    /// Exploitation established
    pub exploitation_established: bool,
    /// Equitable fraud
    pub equitable_fraud: bool,
    /// Available remedies
    pub available_remedies: Vec<UnconscionableRemedy>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Statutory Unconscionability (ACL s.20-22)
// ============================================================================

/// Statutory unconscionability analyzer
pub struct StatutoryUnconscionabilityAnalyzer;

impl StatutoryUnconscionabilityAnalyzer {
    /// Analyze statutory unconscionability under ACL
    pub fn analyze(facts: &StatutoryUnconscionabilityFacts) -> StatutoryUnconscionabilityResult {
        let section_applies = Self::determine_applicable_section(facts);

        let unconscionable = match section_applies {
            Some(UnconscionabilityType::StatutoryGeneral) => Self::analyze_s20(facts),
            Some(UnconscionabilityType::StatutoryGoodsServices) => Self::analyze_s21(facts),
            Some(UnconscionabilityType::StatutorySmallBusiness) => Self::analyze_s22(facts),
            _ => false,
        };

        let procedural_factors = Self::assess_procedural_factors(facts);
        let substantive_factors = Self::assess_substantive_factors(facts);
        let power_imbalances = Self::assess_power_imbalances(facts);

        let remedies = if unconscionable {
            Self::determine_statutory_remedies(facts)
        } else {
            Vec::new()
        };

        let reasoning = Self::build_statutory_reasoning(
            facts,
            section_applies.as_ref(),
            unconscionable,
            &procedural_factors,
            &substantive_factors,
            &power_imbalances,
        );

        StatutoryUnconscionabilityResult {
            unconscionability_type: section_applies,
            unconscionable_conduct_established: unconscionable,
            procedural_factors,
            substantive_factors,
            power_imbalances,
            system_of_conduct: facts.system_of_conduct,
            available_remedies: remedies,
            reasoning,
        }
    }

    /// Determine which ACL section applies
    fn determine_applicable_section(
        facts: &StatutoryUnconscionabilityFacts,
    ) -> Option<UnconscionabilityType> {
        if facts.small_business_contract {
            Some(UnconscionabilityType::StatutorySmallBusiness)
        } else if facts.goods_or_services_transaction {
            Some(UnconscionabilityType::StatutoryGoodsServices)
        } else if facts.in_trade_or_commerce {
            Some(UnconscionabilityType::StatutoryGeneral)
        } else {
            None
        }
    }

    /// Analyze under s.20 (general)
    fn analyze_s20(facts: &StatutoryUnconscionabilityFacts) -> bool {
        // Must be in trade or commerce
        if !facts.in_trade_or_commerce {
            return false;
        }

        // Broader than common law - system of conduct can be unconscionable
        facts.system_of_conduct
            || (facts.procedural_unfairness && facts.substantive_unfairness)
            || facts.unconscionable_system_or_pattern
    }

    /// Analyze under s.21 (goods/services)
    fn analyze_s21(facts: &StatutoryUnconscionabilityFacts) -> bool {
        if !facts.goods_or_services_transaction {
            return false;
        }

        // Consider factors from s.21:
        // - relative bargaining strength
        // - whether conditions were reasonably necessary
        // - whether party could understand documents
        // - whether undue influence or pressure
        // - amount for which services could be supplied

        let factors_count = [
            facts.inequality_of_bargaining_power,
            facts.conditions_not_reasonably_necessary,
            facts.party_could_not_understand_documents,
            facts.undue_influence_or_pressure,
            facts.price_grossly_excessive,
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        // Multiple factors indicate unconscionability
        factors_count >= 3 || (factors_count >= 2 && facts.substantive_unfairness)
    }

    /// Analyze under s.22 (small business)
    fn analyze_s22(facts: &StatutoryUnconscionabilityFacts) -> bool {
        if !facts.small_business_contract {
            return false;
        }

        // Similar to s.21 but specific to small business
        // All s.22 considerations apply
        let consideration_count = facts.small_business_considerations.len();

        consideration_count >= 3
            || (consideration_count >= 2
                && (facts.inequality_of_bargaining_power || facts.substantive_unfairness))
    }

    /// Assess procedural unconscionability factors
    fn assess_procedural_factors(facts: &StatutoryUnconscionabilityFacts) -> Vec<ProceduralFactor> {
        let mut factors = Vec::new();

        if facts.high_pressure_tactics {
            factors.push(ProceduralFactor::HighPressureTactics);
        }
        if facts.no_time_to_consider {
            factors.push(ProceduralFactor::NoTimeToConsider);
        }
        if facts.party_could_not_understand_documents {
            factors.push(ProceduralFactor::ComplexLanguage);
        }
        if facts.hidden_or_buried_terms {
            factors.push(ProceduralFactor::HiddenTerms);
        }
        if facts.no_opportunity_to_negotiate {
            factors.push(ProceduralFactor::NoNegotiation);
        }
        if facts.refused_independent_advice {
            factors.push(ProceduralFactor::RefusedAdvice);
        }
        if facts.misrepresented_terms {
            factors.push(ProceduralFactor::MisrepresentedTerms);
        }
        if facts.system_of_conduct {
            factors.push(ProceduralFactor::SystemOfConduct);
        }

        factors
    }

    /// Assess substantive unconscionability factors
    fn assess_substantive_factors(
        facts: &StatutoryUnconscionabilityFacts,
    ) -> Vec<SubstantiveFactor> {
        let mut factors = Vec::new();

        if facts.price_grossly_excessive {
            factors.push(SubstantiveFactor::ExcessivePrice);
        }
        if facts.one_sided_terms {
            factors.push(SubstantiveFactor::OneSidedTerms);
        }
        if facts.no_meaningful_choice {
            factors.push(SubstantiveFactor::NoMeaningfulChoice);
        }
        if facts.oppressive_terms {
            factors.push(SubstantiveFactor::OppressiveTerms);
        }
        if facts.harsh_or_unjust_outcome {
            factors.push(SubstantiveFactor::HarshOutcome);
        }
        if facts.manifestly_unfair_advantage {
            factors.push(SubstantiveFactor::UnfairAdvantage);
        }
        if facts.exploitation_of_vulnerability {
            factors.push(SubstantiveFactor::ExploitationOfVulnerability);
        }

        factors
    }

    /// Assess power imbalances
    fn assess_power_imbalances(facts: &StatutoryUnconscionabilityFacts) -> Vec<PowerImbalance> {
        let mut imbalances = Vec::new();

        if facts.inequality_of_bargaining_power {
            imbalances.push(PowerImbalance::BargainingStrength);
        }
        if facts.information_asymmetry {
            imbalances.push(PowerImbalance::InformationAsymmetry);
        }
        if facts.economic_dependence {
            imbalances.push(PowerImbalance::EconomicDependence);
        }
        if facts.monopoly_or_market_power {
            imbalances.push(PowerImbalance::MarketPower);
        }
        if facts.professional_vs_layperson {
            imbalances.push(PowerImbalance::ProfessionalAdvantage);
        }
        if facts.sophistication_disparity {
            imbalances.push(PowerImbalance::SophisticationDisparity);
        }

        imbalances
    }

    /// Determine statutory remedies
    fn determine_statutory_remedies(
        facts: &StatutoryUnconscionabilityFacts,
    ) -> Vec<UnconscionableRemedy> {
        let mut remedies = Vec::new();

        // ACCC or private action remedies

        // Declarations
        remedies.push(UnconscionableRemedy::Declaration);

        // Injunctions
        remedies.push(UnconscionableRemedy::Injunction);

        // Damages
        if facts.loss_suffered {
            remedies.push(UnconscionableRemedy::Damages);
        }

        // Vary or void contract
        remedies.push(UnconscionableRemedy::VaryContract);
        remedies.push(UnconscionableRemedy::VoidContract);

        // Refund
        if facts.payment_made {
            remedies.push(UnconscionableRemedy::Refund);
        }

        // Other orders
        remedies.push(UnconscionableRemedy::OtherOrders);

        remedies
    }

    /// Build statutory reasoning
    fn build_statutory_reasoning(
        facts: &StatutoryUnconscionabilityFacts,
        section: Option<&UnconscionabilityType>,
        unconscionable: bool,
        procedural: &[ProceduralFactor],
        substantive: &[SubstantiveFactor],
        power: &[PowerImbalance],
    ) -> String {
        let mut parts = Vec::new();

        match section {
            Some(UnconscionabilityType::StatutoryGeneral) => {
                parts.push("Statutory unconscionability analysis: ACL s.20 (general)".to_string());
            }
            Some(UnconscionabilityType::StatutoryGoodsServices) => {
                parts.push(
                    "Statutory unconscionability analysis: ACL s.21 (goods/services)".to_string(),
                );
            }
            Some(UnconscionabilityType::StatutorySmallBusiness) => {
                parts.push(
                    "Statutory unconscionability analysis: ACL s.22 (small business)".to_string(),
                );
            }
            _ => {
                parts.push("No applicable ACL unconscionability provision".to_string());
                return parts.join(". ");
            }
        }

        if !procedural.is_empty() {
            parts.push(format!(
                "Procedural unconscionability factors present: {}",
                procedural.len()
            ));
        }

        if !substantive.is_empty() {
            parts.push(format!(
                "Substantive unconscionability factors present: {}",
                substantive.len()
            ));
        }

        if !power.is_empty() {
            parts.push(format!("Power imbalances identified: {}", power.len()));
        }

        if unconscionable {
            parts.push("Unconscionable conduct established".to_string());

            if facts.system_of_conduct {
                parts.push("System or pattern of unconscionable conduct".to_string());
            }

            if facts.inequality_of_bargaining_power {
                parts.push("Significant inequality of bargaining power".to_string());
            }

            if facts.price_grossly_excessive {
                parts.push("Price grossly excessive compared to market".to_string());
            }

            if facts.undue_influence_or_pressure {
                parts.push("Undue influence or pressure applied".to_string());
            }

            parts.push("Remedies available under ACL Part 5-2".to_string());
        } else {
            parts.push("Unconscionable conduct not established".to_string());

            if facts.procedural_unfairness && !facts.substantive_unfairness {
                parts.push("Procedural issues present but no substantive unfairness".to_string());
            }
        }

        parts.join(". ")
    }
}

/// Facts for statutory unconscionability
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatutoryUnconscionabilityFacts {
    // Threshold requirements
    /// In trade or commerce (s.20)
    pub in_trade_or_commerce: bool,
    /// Goods or services transaction (s.21)
    pub goods_or_services_transaction: bool,
    /// Small business contract (s.22)
    pub small_business_contract: bool,

    // Procedural factors
    /// High pressure tactics
    pub high_pressure_tactics: bool,
    /// No time to consider
    pub no_time_to_consider: bool,
    /// Hidden or buried terms
    pub hidden_or_buried_terms: bool,
    /// No opportunity to negotiate
    pub no_opportunity_to_negotiate: bool,
    /// Refused independent advice
    pub refused_independent_advice: bool,
    /// Misrepresented terms
    pub misrepresented_terms: bool,
    /// System of conduct
    pub system_of_conduct: bool,
    /// Procedural unfairness
    pub procedural_unfairness: bool,

    // Substantive factors
    /// Price grossly excessive
    pub price_grossly_excessive: bool,
    /// One-sided terms
    pub one_sided_terms: bool,
    /// No meaningful choice
    pub no_meaningful_choice: bool,
    /// Oppressive terms
    pub oppressive_terms: bool,
    /// Harsh or unjust outcome
    pub harsh_or_unjust_outcome: bool,
    /// Manifestly unfair advantage
    pub manifestly_unfair_advantage: bool,
    /// Exploitation of vulnerability
    pub exploitation_of_vulnerability: bool,
    /// Substantive unfairness
    pub substantive_unfairness: bool,

    // Power imbalance
    /// Inequality of bargaining power
    pub inequality_of_bargaining_power: bool,
    /// Information asymmetry
    pub information_asymmetry: bool,
    /// Economic dependence
    pub economic_dependence: bool,
    /// Monopoly or market power
    pub monopoly_or_market_power: bool,
    /// Professional vs layperson
    pub professional_vs_layperson: bool,
    /// Sophistication disparity
    pub sophistication_disparity: bool,

    // s.21/22 specific factors
    /// Conditions not reasonably necessary
    pub conditions_not_reasonably_necessary: bool,
    /// Party could not understand documents
    pub party_could_not_understand_documents: bool,
    /// Undue influence or pressure
    pub undue_influence_or_pressure: bool,
    /// Small business considerations (s.22)
    pub small_business_considerations: Vec<SmallBusinessConsideration>,

    // System of conduct
    /// Unconscionable system or pattern
    pub unconscionable_system_or_pattern: bool,

    // Remedies factors
    /// Loss suffered
    pub loss_suffered: bool,
    /// Payment made
    pub payment_made: bool,
}

/// Result of statutory unconscionability analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatutoryUnconscionabilityResult {
    /// Type of unconscionability
    pub unconscionability_type: Option<UnconscionabilityType>,
    /// Unconscionable conduct established
    pub unconscionable_conduct_established: bool,
    /// Procedural factors
    pub procedural_factors: Vec<ProceduralFactor>,
    /// Substantive factors
    pub substantive_factors: Vec<SubstantiveFactor>,
    /// Power imbalances
    pub power_imbalances: Vec<PowerImbalance>,
    /// System of conduct
    pub system_of_conduct: bool,
    /// Available remedies
    pub available_remedies: Vec<UnconscionableRemedy>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Combined Unconscionability Analysis
// ============================================================================

/// Combined unconscionability analyzer
pub struct UnconscionabilityAnalyzer;

impl UnconscionabilityAnalyzer {
    /// Comprehensive unconscionability analysis
    pub fn analyze(
        common_law_facts: &CommonLawUnconscionabilityFacts,
        statutory_facts: &StatutoryUnconscionabilityFacts,
    ) -> UnconscionabilityAnalysisResult {
        let common_law = CommonLawUnconscionabilityAnalyzer::analyze(common_law_facts);
        let statutory = StatutoryUnconscionabilityAnalyzer::analyze(statutory_facts);

        let unconscionable =
            common_law.exploitation_established || statutory.unconscionable_conduct_established;

        let primary_basis = if common_law.exploitation_established {
            Some(UnconscionabilityType::CommonLaw)
        } else if statutory.unconscionable_conduct_established {
            statutory.unconscionability_type.clone()
        } else {
            None
        };

        let all_remedies = Self::combine_remedies(
            &common_law.available_remedies,
            &statutory.available_remedies,
        );

        let reasoning = Self::build_combined_reasoning(&common_law, &statutory, unconscionable);

        UnconscionabilityAnalysisResult {
            unconscionable_conduct_found: unconscionable,
            primary_basis,
            common_law_result: common_law,
            statutory_result: statutory,
            available_remedies: all_remedies,
            reasoning,
        }
    }

    /// Combine remedies from both analyses
    fn combine_remedies(
        common_law: &[UnconscionableRemedy],
        statutory: &[UnconscionableRemedy],
    ) -> Vec<UnconscionableRemedy> {
        let mut remedies = Vec::new();

        for remedy in common_law {
            if !remedies.contains(remedy) {
                remedies.push(remedy.clone());
            }
        }

        for remedy in statutory {
            if !remedies.contains(remedy) {
                remedies.push(remedy.clone());
            }
        }

        remedies
    }

    /// Build combined reasoning
    fn build_combined_reasoning(
        common_law: &CommonLawUnconscionabilityResult,
        statutory: &StatutoryUnconscionabilityResult,
        unconscionable: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Comprehensive unconscionability analysis".to_string());

        if unconscionable {
            parts.push("Unconscionable conduct established".to_string());

            if common_law.exploitation_established {
                parts.push("Common law unconscionable dealing satisfied".to_string());
                parts.push(format!(
                    "Special disadvantage types: {}",
                    common_law.disadvantage_types.len()
                ));
            }

            if statutory.unconscionable_conduct_established {
                match &statutory.unconscionability_type {
                    Some(UnconscionabilityType::StatutoryGeneral) => {
                        parts.push("ACL s.20 unconscionability satisfied".to_string());
                    }
                    Some(UnconscionabilityType::StatutoryGoodsServices) => {
                        parts.push("ACL s.21 unconscionability satisfied".to_string());
                    }
                    Some(UnconscionabilityType::StatutorySmallBusiness) => {
                        parts.push("ACL s.22 unconscionability satisfied".to_string());
                    }
                    _ => {}
                }
            }
        } else {
            parts.push("No unconscionable conduct established".to_string());
        }

        parts.join(". ")
    }
}

/// Result of combined unconscionability analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnconscionabilityAnalysisResult {
    /// Unconscionable conduct found
    pub unconscionable_conduct_found: bool,
    /// Primary basis
    pub primary_basis: Option<UnconscionabilityType>,
    /// Common law result
    pub common_law_result: CommonLawUnconscionabilityResult,
    /// Statutory result
    pub statutory_result: StatutoryUnconscionabilityResult,
    /// Available remedies
    pub available_remedies: Vec<UnconscionableRemedy>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Remedies
// ============================================================================

/// Remedies for unconscionable conduct
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnconscionableRemedy {
    /// Rescission (common law)
    Rescission,
    /// Set aside transaction (common law)
    SetAside,
    /// Restitution (common law)
    Restitution,
    /// Equitable compensation (common law)
    EquitableCompensation,
    /// Account of profits (common law)
    AccountOfProfits,
    /// Declaration (statutory)
    Declaration,
    /// Injunction (statutory)
    Injunction,
    /// Damages (statutory)
    Damages,
    /// Vary contract (statutory)
    VaryContract,
    /// Void contract (statutory)
    VoidContract,
    /// Refund (statutory)
    Refund,
    /// Other orders (statutory)
    OtherOrders,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_law_unconscionability_amadio() {
        // Based on Commercial Bank of Australia v Amadio (1983)
        let facts = CommonLawUnconscionabilityFacts {
            disadvantage_types: vec![
                SpecialDisadvantage::Age,
                SpecialDisadvantage::LiteracyLanguage,
            ],
            seriously_impaired_judgment: true,
            unable_to_protect_interests: true,
            vulnerable_to_exploitation: true,
            actual_knowledge_of_disadvantage: false,
            circumstances_putting_on_notice: true,
            made_reasonable_inquiries: false,
            unconscientious_conduct: true,
            procured_substantially_unfair_advantage: true,
            transaction_manifestly_disadvantageous: true,
            ..Default::default()
        };

        let result = CommonLawUnconscionabilityAnalyzer::analyze(&facts);
        assert!(result.special_disadvantage_established);
        assert!(result.knowledge_established);
        assert!(result.exploitation_established);
        assert!(!result.available_remedies.is_empty());
    }

    #[test]
    fn test_common_law_no_knowledge() {
        let facts = CommonLawUnconscionabilityFacts {
            disadvantage_types: vec![SpecialDisadvantage::Illness],
            seriously_impaired_judgment: true,
            unable_to_protect_interests: true,
            actual_knowledge_of_disadvantage: false,
            circumstances_putting_on_notice: false,
            made_reasonable_inquiries: true,
            ..Default::default()
        };

        let result = CommonLawUnconscionabilityAnalyzer::analyze(&facts);
        assert!(result.special_disadvantage_established);
        assert!(!result.knowledge_established);
        assert!(!result.exploitation_established);
    }

    #[test]
    fn test_statutory_s20_system_of_conduct() {
        let facts = StatutoryUnconscionabilityFacts {
            in_trade_or_commerce: true,
            system_of_conduct: true,
            unconscionable_system_or_pattern: true,
            procedural_unfairness: true,
            substantive_unfairness: true,
            ..Default::default()
        };

        let result = StatutoryUnconscionabilityAnalyzer::analyze(&facts);
        assert_eq!(
            result.unconscionability_type,
            Some(UnconscionabilityType::StatutoryGeneral)
        );
        assert!(result.unconscionable_conduct_established);
    }

    #[test]
    fn test_statutory_s21_goods_services() {
        let facts = StatutoryUnconscionabilityFacts {
            in_trade_or_commerce: true,
            goods_or_services_transaction: true,
            inequality_of_bargaining_power: true,
            conditions_not_reasonably_necessary: true,
            party_could_not_understand_documents: true,
            undue_influence_or_pressure: true,
            substantive_unfairness: true,
            ..Default::default()
        };

        let result = StatutoryUnconscionabilityAnalyzer::analyze(&facts);
        assert_eq!(
            result.unconscionability_type,
            Some(UnconscionabilityType::StatutoryGoodsServices)
        );
        assert!(result.unconscionable_conduct_established);
    }

    #[test]
    fn test_statutory_s22_small_business() {
        let facts = StatutoryUnconscionabilityFacts {
            in_trade_or_commerce: true,
            small_business_contract: true,
            inequality_of_bargaining_power: true,
            small_business_considerations: vec![
                SmallBusinessConsideration::RelativeBargainingPower,
                SmallBusinessConsideration::UnduePressure,
                SmallBusinessConsideration::AbilityToUnderstand,
            ],
            substantive_unfairness: true,
            ..Default::default()
        };

        let result = StatutoryUnconscionabilityAnalyzer::analyze(&facts);
        assert_eq!(
            result.unconscionability_type,
            Some(UnconscionabilityType::StatutorySmallBusiness)
        );
        assert!(result.unconscionable_conduct_established);
    }

    #[test]
    fn test_procedural_vs_substantive() {
        // Procedural factors alone may not be enough
        let facts = StatutoryUnconscionabilityFacts {
            in_trade_or_commerce: true,
            goods_or_services_transaction: true,
            high_pressure_tactics: true,
            no_time_to_consider: true,
            procedural_unfairness: true,
            substantive_unfairness: false,
            ..Default::default()
        };

        let result = StatutoryUnconscionabilityAnalyzer::analyze(&facts);
        // May or may not be unconscionable depending on severity
        assert!(!result.procedural_factors.is_empty());
    }

    #[test]
    fn test_combined_analysis() {
        let common_law_facts = CommonLawUnconscionabilityFacts {
            disadvantage_types: vec![SpecialDisadvantage::FinancialDistress],
            seriously_impaired_judgment: true,
            unable_to_protect_interests: true,
            actual_knowledge_of_disadvantage: true,
            unconscientious_conduct: true,
            procured_substantially_unfair_advantage: true,
            ..Default::default()
        };

        let statutory_facts = StatutoryUnconscionabilityFacts {
            in_trade_or_commerce: true,
            goods_or_services_transaction: true,
            inequality_of_bargaining_power: true,
            price_grossly_excessive: true,
            undue_influence_or_pressure: true,
            substantive_unfairness: true,
            ..Default::default()
        };

        let result = UnconscionabilityAnalyzer::analyze(&common_law_facts, &statutory_facts);
        assert!(result.unconscionable_conduct_found);
        assert!(result.primary_basis.is_some());
        assert!(!result.available_remedies.is_empty());
    }

    #[test]
    fn test_no_unconscionability() {
        let common_law_facts = CommonLawUnconscionabilityFacts::default();
        let statutory_facts = StatutoryUnconscionabilityFacts {
            in_trade_or_commerce: false,
            ..Default::default()
        };

        let result = UnconscionabilityAnalyzer::analyze(&common_law_facts, &statutory_facts);
        assert!(!result.unconscionable_conduct_found);
        assert!(result.available_remedies.is_empty());
    }

    #[test]
    fn test_kakavas_gambling_case() {
        // Based on Kakavas v Crown Melbourne (2013)
        // Problem gambler - court found no unconscionability
        let facts = CommonLawUnconscionabilityFacts {
            disadvantage_types: vec![SpecialDisadvantage::Addiction],
            seriously_impaired_judgment: true,
            unable_to_protect_interests: true,
            vulnerable_to_exploitation: true,
            actual_knowledge_of_disadvantage: true,
            unconscientious_conduct: false, // Key difference
            ..Default::default()
        };

        let result = CommonLawUnconscionabilityAnalyzer::analyze(&facts);
        assert!(result.special_disadvantage_established);
        assert!(result.knowledge_established);
        // But no exploitation without unconscientious conduct
        assert!(!result.exploitation_established);
    }

    #[test]
    fn test_multiple_disadvantage_types() {
        let facts = CommonLawUnconscionabilityFacts {
            disadvantage_types: vec![
                SpecialDisadvantage::Age,
                SpecialDisadvantage::Illness,
                SpecialDisadvantage::LiteracyLanguage,
            ],
            seriously_impaired_judgment: true,
            unable_to_protect_interests: true,
            circumstances_putting_on_notice: true,
            made_reasonable_inquiries: false,
            unconscientious_conduct: true,
            procured_substantially_unfair_advantage: true,
            ..Default::default()
        };

        let result = CommonLawUnconscionabilityAnalyzer::analyze(&facts);
        assert_eq!(result.disadvantage_types.len(), 3);
        assert!(result.exploitation_established);
    }

    #[test]
    fn test_statutory_procedural_only() {
        // Procedural factors alone without substantive unfairness
        let facts = StatutoryUnconscionabilityFacts {
            in_trade_or_commerce: true,
            goods_or_services_transaction: true,
            high_pressure_tactics: true,
            no_time_to_consider: true,
            hidden_or_buried_terms: true,
            procedural_unfairness: true,
            substantive_unfairness: false,
            ..Default::default()
        };

        let result = StatutoryUnconscionabilityAnalyzer::analyze(&facts);
        assert!(!result.procedural_factors.is_empty());
        assert!(result.substantive_factors.is_empty());
        // May not be unconscionable without substantive unfairness
    }

    #[test]
    fn test_statutory_substantive_only() {
        // Substantive factors alone without procedural issues
        let facts = StatutoryUnconscionabilityFacts {
            in_trade_or_commerce: true,
            goods_or_services_transaction: true,
            price_grossly_excessive: true,
            one_sided_terms: true,
            substantive_unfairness: true,
            procedural_unfairness: false,
            ..Default::default()
        };

        let result = StatutoryUnconscionabilityAnalyzer::analyze(&facts);
        assert!(result.procedural_factors.is_empty());
        assert!(!result.substantive_factors.is_empty());
    }

    #[test]
    fn test_small_business_multiple_considerations() {
        let facts = StatutoryUnconscionabilityFacts {
            in_trade_or_commerce: true,
            small_business_contract: true,
            inequality_of_bargaining_power: true,
            small_business_considerations: vec![
                SmallBusinessConsideration::RelativeBargainingPower,
                SmallBusinessConsideration::UnduePressure,
                SmallBusinessConsideration::AbilityToUnderstand,
                SmallBusinessConsideration::GoodFaithConduct,
            ],
            substantive_unfairness: true,
            ..Default::default()
        };

        let result = StatutoryUnconscionabilityAnalyzer::analyze(&facts);
        assert_eq!(
            result.unconscionability_type,
            Some(UnconscionabilityType::StatutorySmallBusiness)
        );
        assert!(result.unconscionable_conduct_established);
        assert_eq!(result.power_imbalances.len(), 1);
    }

    #[test]
    fn test_information_asymmetry() {
        let facts = StatutoryUnconscionabilityFacts {
            in_trade_or_commerce: true,
            goods_or_services_transaction: true,
            information_asymmetry: true,
            professional_vs_layperson: true,
            sophistication_disparity: true,
            party_could_not_understand_documents: true,
            substantive_unfairness: true,
            inequality_of_bargaining_power: true,
            ..Default::default()
        };

        let result = StatutoryUnconscionabilityAnalyzer::analyze(&facts);
        assert!(result.power_imbalances.len() >= 3);
        assert!(
            result
                .power_imbalances
                .contains(&PowerImbalance::InformationAsymmetry)
        );
        assert!(
            result
                .power_imbalances
                .contains(&PowerImbalance::ProfessionalAdvantage)
        );
        assert!(
            result
                .power_imbalances
                .contains(&PowerImbalance::SophisticationDisparity)
        );
    }

    #[test]
    fn test_willful_blindness_constructive_knowledge() {
        let facts = CommonLawUnconscionabilityFacts {
            disadvantage_types: vec![SpecialDisadvantage::FinancialDistress],
            seriously_impaired_judgment: true,
            unable_to_protect_interests: true,
            actual_knowledge_of_disadvantage: false,
            willful_blindness: true,
            unconscientious_conduct: true,
            procured_substantially_unfair_advantage: true,
            ..Default::default()
        };

        let result = CommonLawUnconscionabilityAnalyzer::analyze(&facts);
        assert!(result.knowledge_established); // Willful blindness = knowledge
        assert!(result.exploitation_established);
    }

    #[test]
    fn test_remedies_account_of_profits() {
        let facts = CommonLawUnconscionabilityFacts {
            disadvantage_types: vec![SpecialDisadvantage::IntellectualImpairment],
            seriously_impaired_judgment: true,
            unable_to_protect_interests: true,
            actual_knowledge_of_disadvantage: true,
            unconscientious_conduct: true,
            procured_substantially_unfair_advantage: true,
            transaction_manifestly_disadvantageous: true,
            profits_made_by_wrongdoer: true,
            ..Default::default()
        };

        let result = CommonLawUnconscionabilityAnalyzer::analyze(&facts);
        assert!(result.exploitation_established);
        assert!(
            result
                .available_remedies
                .contains(&UnconscionableRemedy::AccountOfProfits)
        );
    }

    #[test]
    fn test_emotional_dependence_disadvantage() {
        let facts = CommonLawUnconscionabilityFacts {
            disadvantage_types: vec![SpecialDisadvantage::EmotionalDependence],
            seriously_impaired_judgment: true,
            unable_to_protect_interests: true,
            vulnerable_to_exploitation: true,
            actual_knowledge_of_disadvantage: true,
            unconscientious_conduct: true,
            procured_substantially_unfair_advantage: true,
            ..Default::default()
        };

        let result = CommonLawUnconscionabilityAnalyzer::analyze(&facts);
        assert!(result.special_disadvantage_established);
        assert!(
            result
                .disadvantage_types
                .contains(&SpecialDisadvantage::EmotionalDependence)
        );
    }

    #[test]
    fn test_lack_business_experience() {
        let facts = CommonLawUnconscionabilityFacts {
            disadvantage_types: vec![SpecialDisadvantage::LackOfBusinessExperience],
            seriously_impaired_judgment: true,
            unable_to_protect_interests: true,
            circumstances_putting_on_notice: true,
            made_reasonable_inquiries: false,
            unconscientious_conduct: true,
            procured_substantially_unfair_advantage: true,
            ..Default::default()
        };

        let result = CommonLawUnconscionabilityAnalyzer::analyze(&facts);
        assert!(result.special_disadvantage_established);
        assert!(result.knowledge_established);
        assert!(result.exploitation_established);
    }
}

// ============================================================================
// Case-Specific Analysis
// ============================================================================

/// Amadio-specific analysis for guarantees and surety relationships
pub struct AmadioAnalyzer;

impl AmadioAnalyzer {
    /// Analyze under Amadio principles (guarantor/surety cases)
    pub fn analyze_guarantee(facts: &AmadioGuaranteeFacts) -> AmadioGuaranteeResult {
        // Commercial Bank v Amadio elements:
        // 1. Special disability/disadvantage
        // 2. Party under disability unable to make judgment
        // 3. Other party knew or ought to have known
        // 4. Unconscionable to procure/accept transaction

        let special_disadvantage = Self::assess_guarantor_disadvantage(facts);
        let knowledge = Self::assess_creditor_knowledge(facts);
        let unconscionable_to_accept =
            special_disadvantage && knowledge && facts.guarantee_manifestly_disadvantageous;

        let reasoning = Self::build_amadio_reasoning(
            facts,
            special_disadvantage,
            knowledge,
            unconscionable_to_accept,
        );

        AmadioGuaranteeResult {
            special_disadvantage,
            knowledge_of_disadvantage: knowledge,
            unconscionable_to_accept,
            guarantee_set_aside: unconscionable_to_accept,
            reasoning,
        }
    }

    /// Assess guarantor's special disadvantage
    fn assess_guarantor_disadvantage(facts: &AmadioGuaranteeFacts) -> bool {
        // Amadio: elderly, non-English speaking, lack of business experience
        (facts.elderly_guarantor
            || facts.language_barrier
            || facts.lack_business_acumen
            || facts.emotional_relationship_debtor)
            && facts.unable_to_understand_transaction
    }

    /// Assess creditor's knowledge
    fn assess_creditor_knowledge(facts: &AmadioGuaranteeFacts) -> bool {
        // Bank/creditor knew or ought to have known
        facts.creditor_knew_of_disadvantage
            || (facts.circumstances_apparent && !facts.creditor_made_inquiries)
            || facts.creditor_failed_explain_effect
    }

    /// Build Amadio-specific reasoning
    fn build_amadio_reasoning(
        facts: &AmadioGuaranteeFacts,
        disadvantage: bool,
        knowledge: bool,
        unconscionable: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Amadio guarantee analysis (Commercial Bank v Amadio)".to_string());

        if disadvantage {
            parts.push("Guarantor under special disadvantage".to_string());

            if facts.elderly_guarantor {
                parts.push("Elderly guarantor with impaired judgment".to_string());
            }
            if facts.language_barrier {
                parts.push("Language barrier affecting understanding".to_string());
            }
            if facts.emotional_relationship_debtor {
                parts.push("Emotional relationship with debtor".to_string());
            }
        }

        if knowledge {
            parts.push("Creditor knew or ought to have known of disadvantage".to_string());

            if facts.creditor_failed_explain_effect {
                parts.push("Creditor failed to explain effect of guarantee".to_string());
            }
        }

        if unconscionable {
            parts.push("Unconscionable for creditor to accept guarantee".to_string());
            parts.push("Guarantee liable to be set aside".to_string());
        } else if !disadvantage {
            parts.push("No special disadvantage established".to_string());
        } else if !knowledge {
            parts.push("No knowledge of disadvantage".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for Amadio guarantee analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AmadioGuaranteeFacts {
    /// Elderly guarantor
    pub elderly_guarantor: bool,
    /// Language barrier
    pub language_barrier: bool,
    /// Lack business acumen
    pub lack_business_acumen: bool,
    /// Emotional relationship with debtor
    pub emotional_relationship_debtor: bool,
    /// Unable to understand transaction
    pub unable_to_understand_transaction: bool,
    /// Creditor knew of disadvantage
    pub creditor_knew_of_disadvantage: bool,
    /// Circumstances apparent
    pub circumstances_apparent: bool,
    /// Creditor made inquiries
    pub creditor_made_inquiries: bool,
    /// Creditor failed to explain effect
    pub creditor_failed_explain_effect: bool,
    /// Guarantee manifestly disadvantageous
    pub guarantee_manifestly_disadvantageous: bool,
}

/// Result of Amadio guarantee analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmadioGuaranteeResult {
    /// Special disadvantage
    pub special_disadvantage: bool,
    /// Knowledge of disadvantage
    pub knowledge_of_disadvantage: bool,
    /// Unconscionable to accept
    pub unconscionable_to_accept: bool,
    /// Guarantee set aside
    pub guarantee_set_aside: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Blomley-specific analysis for improvident bargains
pub struct BlomleyAnalyzer;

impl BlomleyAnalyzer {
    /// Analyze under Blomley v Ryan principles
    pub fn analyze_improvident_bargain(facts: &BlomleyBargainFacts) -> BlomleyBargainResult {
        // Blomley v Ryan: drunk vendor selling property at undervalue
        // Broader than Amadio - doesn't require other party to know

        let impaired_capacity = Self::assess_impairment(facts);
        let grossly_inadequate = Self::assess_consideration(facts);
        let unconscientious = impaired_capacity && grossly_inadequate && !facts.fair_dealing;

        let reasoning = Self::build_blomley_reasoning(
            facts,
            impaired_capacity,
            grossly_inadequate,
            unconscientious,
        );

        BlomleyBargainResult {
            impaired_capacity,
            grossly_inadequate_consideration: grossly_inadequate,
            unconscientious_bargain: unconscientious,
            transaction_set_aside: unconscientious,
            reasoning,
        }
    }

    /// Assess impairment of capacity
    fn assess_impairment(facts: &BlomleyBargainFacts) -> bool {
        facts.intoxication
            || facts.mental_impairment
            || facts.severe_emotional_distress
            || facts.desperately_in_need
    }

    /// Assess adequacy of consideration
    fn assess_consideration(facts: &BlomleyBargainFacts) -> bool {
        facts.grossly_undervalue || facts.consideration_derisory
    }

    /// Build Blomley-specific reasoning
    fn build_blomley_reasoning(
        facts: &BlomleyBargainFacts,
        impaired: bool,
        inadequate: bool,
        unconscientious: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Blomley improvident bargain analysis (Blomley v Ryan)".to_string());

        if impaired {
            parts.push("Impaired capacity to contract".to_string());

            if facts.intoxication {
                parts.push("Intoxication affecting judgment".to_string());
            }
            if facts.desperately_in_need {
                parts.push("Party desperately in need".to_string());
            }
        }

        if inadequate {
            parts.push("Grossly inadequate consideration".to_string());

            if facts.grossly_undervalue {
                parts.push("Property sold at gross undervalue".to_string());
            }
        }

        if unconscientious {
            parts.push("Unconscientious bargain - transaction liable to be set aside".to_string());
        } else if facts.fair_dealing {
            parts.push("Fair dealing by other party".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for Blomley bargain analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlomleyBargainFacts {
    /// Intoxication
    pub intoxication: bool,
    /// Mental impairment
    pub mental_impairment: bool,
    /// Severe emotional distress
    pub severe_emotional_distress: bool,
    /// Desperately in need
    pub desperately_in_need: bool,
    /// Grossly undervalue
    pub grossly_undervalue: bool,
    /// Consideration derisory
    pub consideration_derisory: bool,
    /// Fair dealing
    pub fair_dealing: bool,
}

/// Result of Blomley bargain analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlomleyBargainResult {
    /// Impaired capacity
    pub impaired_capacity: bool,
    /// Grossly inadequate consideration
    pub grossly_inadequate_consideration: bool,
    /// Unconscientious bargain
    pub unconscientious_bargain: bool,
    /// Transaction set aside
    pub transaction_set_aside: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Defences and Exceptions
// ============================================================================

/// Defences to unconscionability claims
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnconscionabilityDefence {
    /// Independent legal advice obtained
    IndependentAdvice,
    /// Full disclosure made
    FullDisclosure,
    /// Fair dealing
    FairDealing,
    /// Adequate time to consider
    AdequateTime,
    /// Reasonable inquiries made
    ReasonableInquiries,
    /// Market rate/normal commercial terms
    MarketTerms,
    /// Party's own commercial judgment
    CommercialJudgment,
    /// Delay/laches
    Delay,
}

/// Defence analyzer
pub struct DefenceAnalyzer;

impl DefenceAnalyzer {
    /// Analyze defences to unconscionability
    pub fn analyze(facts: &DefenceFacts) -> DefenceResult {
        let applicable_defences = Self::identify_defences(facts);
        let defence_successful = Self::assess_success(&applicable_defences, facts);

        let reasoning =
            Self::build_defence_reasoning(facts, &applicable_defences, defence_successful);

        DefenceResult {
            applicable_defences,
            defence_successful,
            reasoning,
        }
    }

    /// Identify applicable defences
    fn identify_defences(facts: &DefenceFacts) -> Vec<UnconscionabilityDefence> {
        let mut defences = Vec::new();

        if facts.independent_advice_obtained {
            defences.push(UnconscionabilityDefence::IndependentAdvice);
        }
        if facts.full_disclosure_made {
            defences.push(UnconscionabilityDefence::FullDisclosure);
        }
        if facts.fair_dealing_shown {
            defences.push(UnconscionabilityDefence::FairDealing);
        }
        if facts.adequate_time_given {
            defences.push(UnconscionabilityDefence::AdequateTime);
        }
        if facts.reasonable_inquiries_made {
            defences.push(UnconscionabilityDefence::ReasonableInquiries);
        }
        if facts.market_rate_terms {
            defences.push(UnconscionabilityDefence::MarketTerms);
        }
        if facts.partys_commercial_judgment {
            defences.push(UnconscionabilityDefence::CommercialJudgment);
        }
        if facts.delay_in_bringing_claim {
            defences.push(UnconscionabilityDefence::Delay);
        }

        defences
    }

    /// Assess success of defences
    fn assess_success(defences: &[UnconscionabilityDefence], facts: &DefenceFacts) -> bool {
        // Independent advice is strong defence
        if defences.contains(&UnconscionabilityDefence::IndependentAdvice)
            && facts.advice_genuinely_independent
            && facts.advice_competent
        {
            return true;
        }

        // Full disclosure + fair dealing
        if defences.contains(&UnconscionabilityDefence::FullDisclosure)
            && defences.contains(&UnconscionabilityDefence::FairDealing)
            && facts.disclosure_complete
            && facts.no_pressure_tactics
        {
            return true;
        }

        // Multiple procedural safeguards
        if defences.len() >= 3 {
            let procedural_count = [
                facts.adequate_time_given,
                facts.reasonable_inquiries_made,
                facts.full_disclosure_made,
                facts.no_pressure_tactics,
            ]
            .iter()
            .filter(|&&x| x)
            .count();

            if procedural_count >= 3 {
                return true;
            }
        }

        false
    }

    /// Build defence reasoning
    fn build_defence_reasoning(
        facts: &DefenceFacts,
        defences: &[UnconscionabilityDefence],
        successful: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Unconscionability defence analysis".to_string());

        if defences.is_empty() {
            parts.push("No defences available".to_string());
            return parts.join(". ");
        }

        parts.push(format!("Defences raised: {}", defences.len()));

        for defence in defences {
            match defence {
                UnconscionabilityDefence::IndependentAdvice => {
                    if facts.advice_genuinely_independent && facts.advice_competent {
                        parts
                            .push("Independent legal advice obtained - strong defence".to_string());
                    } else {
                        parts.push("Advice not genuinely independent".to_string());
                    }
                }
                UnconscionabilityDefence::FullDisclosure => {
                    if facts.disclosure_complete {
                        parts.push("Full disclosure made".to_string());
                    }
                }
                UnconscionabilityDefence::FairDealing => {
                    if facts.no_pressure_tactics {
                        parts.push("Fair dealing demonstrated".to_string());
                    }
                }
                UnconscionabilityDefence::AdequateTime => {
                    parts.push("Adequate time provided to consider".to_string());
                }
                _ => {}
            }
        }

        if successful {
            parts.push("Defence successful - unconscionability claim defeated".to_string());
        } else {
            parts.push("Defence unsuccessful".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for defence analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DefenceFacts {
    /// Independent advice obtained
    pub independent_advice_obtained: bool,
    /// Advice genuinely independent
    pub advice_genuinely_independent: bool,
    /// Advice competent
    pub advice_competent: bool,
    /// Full disclosure made
    pub full_disclosure_made: bool,
    /// Disclosure complete
    pub disclosure_complete: bool,
    /// Fair dealing shown
    pub fair_dealing_shown: bool,
    /// No pressure tactics
    pub no_pressure_tactics: bool,
    /// Adequate time given
    pub adequate_time_given: bool,
    /// Reasonable inquiries made
    pub reasonable_inquiries_made: bool,
    /// Market rate terms
    pub market_rate_terms: bool,
    /// Party's commercial judgment
    pub partys_commercial_judgment: bool,
    /// Delay in bringing claim
    pub delay_in_bringing_claim: bool,
}

/// Result of defence analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenceResult {
    /// Applicable defences
    pub applicable_defences: Vec<UnconscionabilityDefence>,
    /// Defence successful
    pub defence_successful: bool,
    /// Reasoning
    pub reasoning: String,
}
