//! Australian Consumer Law
//!
//! Analysis under the Australian Consumer Law (Schedule 2 to the
//! Competition and Consumer Act 2010).

use serde::{Deserialize, Serialize};

use super::types::{ConsumerGuarantee, ProhibitedConduct, UnfairTermType};

// ============================================================================
// Consumer Definitions
// ============================================================================

/// Consumer status analysis
pub struct ConsumerAnalyzer;

impl ConsumerAnalyzer {
    /// Determine if person is a "consumer" under ACL
    pub fn is_consumer(facts: &ConsumerStatusFacts) -> ConsumerStatusResult {
        // ACL s.3: Consumer if:
        // 1. Goods/services acquired for personal/domestic/household use, OR
        // 2. Price does not exceed $100,000 (or prescribed amount), OR
        // 3. Goods are of a kind ordinarily acquired for personal use

        let is_consumer = if facts.goods_or_services_transaction {
            facts.personal_domestic_household_use
                || facts.price_under_threshold
                || facts.ordinarily_personal_use_goods
        } else {
            false
        };

        // Exceptions
        let excepted = facts.acquired_for_resupply
            || facts.acquired_for_manufacturing
            || facts.acquired_for_commercial_vehicle_transport;

        let final_status = is_consumer && !excepted;

        ConsumerStatusResult {
            is_consumer: final_status,
            basis: if final_status {
                Self::determine_basis(facts)
            } else {
                None
            },
            exceptions_applied: excepted,
            reasoning: Self::build_reasoning(facts, final_status, excepted),
        }
    }

    /// Determine the basis for consumer status
    fn determine_basis(facts: &ConsumerStatusFacts) -> Option<ConsumerBasis> {
        if facts.personal_domestic_household_use {
            Some(ConsumerBasis::PersonalUse)
        } else if facts.price_under_threshold {
            Some(ConsumerBasis::PriceThreshold)
        } else if facts.ordinarily_personal_use_goods {
            Some(ConsumerBasis::OrdinarilyPersonalUse)
        } else {
            None
        }
    }

    /// Build reasoning
    fn build_reasoning(facts: &ConsumerStatusFacts, is_consumer: bool, excepted: bool) -> String {
        let mut parts = Vec::new();

        parts.push("ACL consumer status analysis (s.3)".to_string());

        if is_consumer {
            parts.push("Person qualifies as consumer".to_string());
            if facts.personal_domestic_household_use {
                parts.push("Acquired for personal/domestic/household use".to_string());
            }
            if facts.price_under_threshold {
                parts.push("Price under $100,000 threshold".to_string());
            }
        } else if excepted {
            parts.push("Consumer status excluded by exception".to_string());
            if facts.acquired_for_resupply {
                parts.push("Goods acquired for resupply".to_string());
            }
            if facts.acquired_for_manufacturing {
                parts.push("Goods acquired for manufacturing".to_string());
            }
        } else {
            parts.push("Does not meet consumer definition".to_string());
        }

        parts.join(". ")
    }
}

/// Basis for consumer status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsumerBasis {
    /// Personal/domestic/household use
    PersonalUse,
    /// Price under threshold
    PriceThreshold,
    /// Goods ordinarily for personal use
    OrdinarilyPersonalUse,
}

/// Facts for consumer status
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsumerStatusFacts {
    /// Goods or services transaction
    pub goods_or_services_transaction: bool,
    /// For personal/domestic/household use
    pub personal_domestic_household_use: bool,
    /// Price under $100,000
    pub price_under_threshold: bool,
    /// Goods ordinarily for personal use
    pub ordinarily_personal_use_goods: bool,
    /// Acquired for resupply
    pub acquired_for_resupply: bool,
    /// Acquired for manufacturing
    pub acquired_for_manufacturing: bool,
    /// Acquired for commercial vehicle transport
    pub acquired_for_commercial_vehicle_transport: bool,
}

/// Result of consumer status analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerStatusResult {
    /// Is consumer under ACL
    pub is_consumer: bool,
    /// Basis for status
    pub basis: Option<ConsumerBasis>,
    /// Exceptions applied
    pub exceptions_applied: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Consumer Guarantees
// ============================================================================

/// Analyzer for consumer guarantees
pub struct GuaranteeAnalyzer;

impl GuaranteeAnalyzer {
    /// Analyze consumer guarantee breach
    pub fn analyze(guarantee: ConsumerGuarantee, facts: &GuaranteeFacts) -> GuaranteeResult {
        let breach = Self::check_breach(&guarantee, facts);
        let major_failure = breach && Self::is_major_failure(&guarantee, facts);
        let remedies = Self::determine_remedies(breach, major_failure);

        let reasoning = Self::build_reasoning(&guarantee, facts, breach, major_failure);

        GuaranteeResult {
            guarantee,
            breached: breach,
            major_failure,
            available_remedies: remedies,
            reasoning,
        }
    }

    /// Check if guarantee breached
    fn check_breach(guarantee: &ConsumerGuarantee, facts: &GuaranteeFacts) -> bool {
        match guarantee {
            ConsumerGuarantee::AcceptableQuality => {
                !facts.fit_for_normal_purpose
                    || !facts.acceptable_appearance_finish
                    || !facts.free_from_defects
                    || !facts.safe
                    || !facts.durable
            }
            ConsumerGuarantee::FitForPurpose => {
                facts.disclosed_purpose.is_some() && !facts.fit_for_disclosed_purpose
            }
            ConsumerGuarantee::MatchesDescription => {
                facts.described_goods && !facts.matches_description
            }
            ConsumerGuarantee::MatchesSample => facts.sample_provided && !facts.matches_sample,
            ConsumerGuarantee::DueCareAndSkill => {
                facts.services_provided && !facts.due_care_exercised
            }
            ConsumerGuarantee::ServicesFitForPurpose => {
                facts.services_provided
                    && facts.disclosed_purpose.is_some()
                    && !facts.services_fit_for_purpose
            }
            ConsumerGuarantee::ReasonableTime => {
                facts.services_provided && facts.no_time_agreed && !facts.reasonable_time_taken
            }
            ConsumerGuarantee::Title => !facts.has_clear_title,
            ConsumerGuarantee::UndisturbedPossession => facts.possession_disturbed,
            ConsumerGuarantee::FreeFromEncumbrances => facts.undisclosed_encumbrances,
            _ => false,
        }
    }

    /// Check if failure is major (for goods)
    fn is_major_failure(guarantee: &ConsumerGuarantee, facts: &GuaranteeFacts) -> bool {
        // s.260 Major failure if:
        // - Would not have acquired if known
        // - Significantly different from description/sample
        // - Substantially unfit for purpose and cannot easily be remedied
        // - Unsafe

        match guarantee {
            ConsumerGuarantee::AcceptableQuality
            | ConsumerGuarantee::FitForPurpose
            | ConsumerGuarantee::MatchesDescription => {
                !facts.safe
                    || facts.would_not_have_acquired_if_known
                    || (facts.cannot_be_remedied && facts.substantially_unfit)
            }
            ConsumerGuarantee::DueCareAndSkill | ConsumerGuarantee::ServicesFitForPurpose => {
                // s.268 Major failure for services
                facts.would_not_have_acquired_if_known || facts.cannot_be_remedied
            }
            _ => false,
        }
    }

    /// Determine available remedies
    fn determine_remedies(breach: bool, major_failure: bool) -> Vec<ConsumerRemedy> {
        if !breach {
            return Vec::new();
        }

        let mut remedies = Vec::new();

        if major_failure {
            // Consumer can reject goods/terminate services
            remedies.push(ConsumerRemedy::Reject);
            remedies.push(ConsumerRemedy::TerminateService);
            remedies.push(ConsumerRemedy::Refund);
        } else {
            // Supplier can choose: repair, replace, or refund
            remedies.push(ConsumerRemedy::Repair);
            remedies.push(ConsumerRemedy::Replace);
        }

        // Compensation always available for consequential loss
        remedies.push(ConsumerRemedy::Compensation);

        remedies
    }

    /// Build reasoning
    fn build_reasoning(
        guarantee: &ConsumerGuarantee,
        facts: &GuaranteeFacts,
        breach: bool,
        major: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push(format!(
            "Consumer guarantee analysis: ACL s.{}",
            guarantee.section()
        ));

        if breach {
            parts.push("Guarantee breached".to_string());

            match guarantee {
                ConsumerGuarantee::AcceptableQuality => {
                    if !facts.fit_for_normal_purpose {
                        parts.push("Not fit for normal purpose".to_string());
                    }
                    if !facts.safe {
                        parts.push("Not safe".to_string());
                    }
                    if !facts.durable {
                        parts.push("Not durable".to_string());
                    }
                }
                ConsumerGuarantee::DueCareAndSkill => {
                    parts.push("Services not provided with due care and skill".to_string());
                }
                _ => {}
            }

            if major {
                parts.push("Major failure - consumer can reject and seek refund".to_string());
            } else {
                parts.push("Minor failure - supplier can repair or replace".to_string());
            }
        } else {
            parts.push("No breach of guarantee".to_string());
        }

        parts.join(". ")
    }
}

/// Consumer remedy under ACL
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsumerRemedy {
    /// Repair
    Repair,
    /// Replace
    Replace,
    /// Refund
    Refund,
    /// Reject goods
    Reject,
    /// Terminate service
    TerminateService,
    /// Compensation for consequential loss
    Compensation,
}

/// Facts for guarantee analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GuaranteeFacts {
    // Acceptable quality factors
    /// Fit for normal purpose
    pub fit_for_normal_purpose: bool,
    /// Acceptable appearance and finish
    pub acceptable_appearance_finish: bool,
    /// Free from defects
    pub free_from_defects: bool,
    /// Safe
    pub safe: bool,
    /// Durable
    pub durable: bool,

    // Fit for purpose
    /// Disclosed purpose
    pub disclosed_purpose: Option<String>,
    /// Fit for disclosed purpose
    pub fit_for_disclosed_purpose: bool,

    // Description/sample
    /// Goods described
    pub described_goods: bool,
    /// Matches description
    pub matches_description: bool,
    /// Sample provided
    pub sample_provided: bool,
    /// Matches sample
    pub matches_sample: bool,

    // Services
    /// Services provided
    pub services_provided: bool,
    /// Due care exercised
    pub due_care_exercised: bool,
    /// Services fit for purpose
    pub services_fit_for_purpose: bool,
    /// No time agreed
    pub no_time_agreed: bool,
    /// Reasonable time taken
    pub reasonable_time_taken: bool,

    // Title
    /// Has clear title
    pub has_clear_title: bool,
    /// Possession disturbed
    pub possession_disturbed: bool,
    /// Undisclosed encumbrances
    pub undisclosed_encumbrances: bool,

    // Major failure indicators
    /// Would not have acquired if known
    pub would_not_have_acquired_if_known: bool,
    /// Cannot be remedied
    pub cannot_be_remedied: bool,
    /// Substantially unfit
    pub substantially_unfit: bool,
}

/// Result of guarantee analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuaranteeResult {
    /// Guarantee analyzed
    pub guarantee: ConsumerGuarantee,
    /// Whether breached
    pub breached: bool,
    /// Whether major failure
    pub major_failure: bool,
    /// Available remedies
    pub available_remedies: Vec<ConsumerRemedy>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Misleading or Deceptive Conduct
// ============================================================================

/// Analyzer for misleading or deceptive conduct (s.18)
pub struct MisleadingConductAnalyzer;

impl MisleadingConductAnalyzer {
    /// Analyze misleading or deceptive conduct claim
    pub fn analyze(facts: &MisleadingConductFacts) -> MisleadingConductResult {
        let in_trade_or_commerce = facts.in_trade_or_commerce;
        let conduct_occurred = facts.conduct_occurred;
        let misleading = Self::assess_misleading(facts);
        let reliance = facts.reliance_on_conduct;
        let loss = facts.loss_suffered;

        let liability = in_trade_or_commerce && conduct_occurred && misleading;
        let damages_available = liability && reliance && loss;

        let reasoning = Self::build_reasoning(facts, liability, damages_available);

        MisleadingConductResult {
            conduct_type: ProhibitedConduct::MisleadingDeceptive,
            in_trade_or_commerce,
            conduct_misleading: misleading,
            liability_established: liability,
            damages_available,
            reasoning,
        }
    }

    /// Assess if conduct is misleading
    fn assess_misleading(facts: &MisleadingConductFacts) -> bool {
        // Test: Would conduct lead or be likely to lead into error?
        // Assessed objectively based on whole conduct

        if facts.literal_truth && facts.no_material_omission {
            return false;
        }

        if facts.future_prediction && facts.reasonable_grounds_for_prediction {
            return false;
        }

        if facts.mere_puff_opinion && !facts.opinion_implies_factual_basis {
            return false;
        }

        facts.would_lead_into_error || facts.material_omission || facts.half_truth
    }

    /// Build reasoning
    fn build_reasoning(facts: &MisleadingConductFacts, liability: bool, damages: bool) -> String {
        let mut parts = Vec::new();

        parts.push("Misleading or deceptive conduct analysis (ACL s.18)".to_string());

        if !facts.in_trade_or_commerce {
            parts.push("Not in trade or commerce - s.18 does not apply".to_string());
            return parts.join(". ");
        }

        parts.push("Conduct in trade or commerce".to_string());

        if liability {
            parts.push("Conduct is misleading or deceptive".to_string());

            if facts.would_lead_into_error {
                parts.push("Conduct would lead reasonable person into error".to_string());
            }
            if facts.material_omission {
                parts.push("Material omission constitutes misleading conduct".to_string());
            }
            if facts.half_truth {
                parts.push("Half-truth that creates false impression".to_string());
            }
        } else {
            parts.push("Conduct not misleading or deceptive".to_string());
            if facts.mere_puff_opinion {
                parts.push("Mere puff or opinion - not actionable".to_string());
            }
        }

        if damages {
            parts.push("Damages recoverable: reliance and loss established".to_string());
        } else if liability {
            parts.push("Injunction available but no damages (no reliance/loss)".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for misleading conduct analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MisleadingConductFacts {
    /// In trade or commerce
    pub in_trade_or_commerce: bool,
    /// Conduct occurred
    pub conduct_occurred: bool,
    /// Would lead into error
    pub would_lead_into_error: bool,
    /// Literal truth
    pub literal_truth: bool,
    /// No material omission
    pub no_material_omission: bool,
    /// Material omission
    pub material_omission: bool,
    /// Half-truth
    pub half_truth: bool,
    /// Future prediction
    pub future_prediction: bool,
    /// Reasonable grounds for prediction
    pub reasonable_grounds_for_prediction: bool,
    /// Mere puff/opinion
    pub mere_puff_opinion: bool,
    /// Opinion implies factual basis
    pub opinion_implies_factual_basis: bool,
    /// Reliance on conduct
    pub reliance_on_conduct: bool,
    /// Loss suffered
    pub loss_suffered: bool,
}

/// Result of misleading conduct analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MisleadingConductResult {
    /// Conduct type
    pub conduct_type: ProhibitedConduct,
    /// In trade or commerce
    pub in_trade_or_commerce: bool,
    /// Conduct is misleading
    pub conduct_misleading: bool,
    /// Liability established
    pub liability_established: bool,
    /// Damages available
    pub damages_available: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Unfair Contract Terms
// ============================================================================

/// Analyzer for unfair contract terms
pub struct UnfairTermsAnalyzer;

impl UnfairTermsAnalyzer {
    /// Analyze whether term is unfair under ACL
    pub fn analyze(facts: &UnfairTermFacts) -> UnfairTermResult {
        // ACL Pt 2-3: Unfair if:
        // 1. Standard form contract
        // 2. Consumer or small business contract
        // 3. Term causes significant imbalance
        // 4. Not reasonably necessary to protect legitimate interests
        // 5. Would cause detriment if relied upon

        let applies = facts.standard_form_contract
            && (facts.consumer_contract || facts.small_business_contract);

        let unfair = if applies {
            facts.causes_significant_imbalance
                && !facts.reasonably_necessary_to_protect_interests
                && facts.would_cause_detriment
        } else {
            false
        };

        let reasoning = Self::build_reasoning(facts, applies, unfair);

        UnfairTermResult {
            term_description: facts.term_description.clone(),
            term_type: facts.term_type.clone(),
            applies_to_contract: applies,
            is_unfair: unfair,
            void: unfair,
            reasoning,
        }
    }

    /// Build reasoning
    fn build_reasoning(facts: &UnfairTermFacts, applies: bool, unfair: bool) -> String {
        let mut parts = Vec::new();

        parts.push("Unfair contract terms analysis (ACL Pt 2-3)".to_string());

        if !applies {
            if !facts.standard_form_contract {
                parts.push("Not a standard form contract - provisions do not apply".to_string());
            } else {
                parts.push("Not a consumer or small business contract".to_string());
            }
            return parts.join(". ");
        }

        parts.push("Standard form consumer/small business contract".to_string());

        if unfair {
            parts.push("Term is unfair".to_string());

            if facts.causes_significant_imbalance {
                parts.push("Causes significant imbalance in parties' rights".to_string());
            }

            if let Some(term_type) = &facts.term_type {
                parts.push(format!("Term type: {:?} - commonly unfair", term_type));
            }

            parts.push("Term is void".to_string());
        } else {
            parts.push("Term is not unfair".to_string());

            if facts.reasonably_necessary_to_protect_interests {
                parts.push("Term reasonably necessary to protect legitimate interests".to_string());
            }
        }

        parts.join(". ")
    }
}

/// Facts for unfair term analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnfairTermFacts {
    /// Term description
    pub term_description: String,
    /// Term type
    pub term_type: Option<UnfairTermType>,
    /// Standard form contract
    pub standard_form_contract: bool,
    /// Consumer contract
    pub consumer_contract: bool,
    /// Small business contract
    pub small_business_contract: bool,
    /// Causes significant imbalance
    pub causes_significant_imbalance: bool,
    /// Reasonably necessary to protect interests
    pub reasonably_necessary_to_protect_interests: bool,
    /// Would cause detriment
    pub would_cause_detriment: bool,
}

/// Result of unfair term analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfairTermResult {
    /// Term description
    pub term_description: String,
    /// Term type
    pub term_type: Option<UnfairTermType>,
    /// Whether provisions apply
    pub applies_to_contract: bool,
    /// Whether term is unfair
    pub is_unfair: bool,
    /// Whether term is void
    pub void: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumer_personal_use() {
        let facts = ConsumerStatusFacts {
            goods_or_services_transaction: true,
            personal_domestic_household_use: true,
            ..Default::default()
        };

        let result = ConsumerAnalyzer::is_consumer(&facts);
        assert!(result.is_consumer);
        assert_eq!(result.basis, Some(ConsumerBasis::PersonalUse));
    }

    #[test]
    fn test_consumer_price_threshold() {
        let facts = ConsumerStatusFacts {
            goods_or_services_transaction: true,
            price_under_threshold: true,
            ..Default::default()
        };

        let result = ConsumerAnalyzer::is_consumer(&facts);
        assert!(result.is_consumer);
    }

    #[test]
    fn test_consumer_resupply_exception() {
        let facts = ConsumerStatusFacts {
            goods_or_services_transaction: true,
            personal_domestic_household_use: true,
            acquired_for_resupply: true,
            ..Default::default()
        };

        let result = ConsumerAnalyzer::is_consumer(&facts);
        assert!(!result.is_consumer);
    }

    #[test]
    fn test_guarantee_acceptable_quality() {
        let facts = GuaranteeFacts {
            fit_for_normal_purpose: false,
            safe: true,
            durable: true,
            acceptable_appearance_finish: true,
            free_from_defects: true,
            ..Default::default()
        };

        let result = GuaranteeAnalyzer::analyze(ConsumerGuarantee::AcceptableQuality, &facts);
        assert!(result.breached);
    }

    #[test]
    fn test_guarantee_major_failure() {
        let facts = GuaranteeFacts {
            fit_for_normal_purpose: false,
            safe: false, // Unsafe = major failure
            ..Default::default()
        };

        let result = GuaranteeAnalyzer::analyze(ConsumerGuarantee::AcceptableQuality, &facts);
        assert!(result.breached);
        assert!(result.major_failure);
        assert!(result.available_remedies.contains(&ConsumerRemedy::Refund));
    }

    #[test]
    fn test_misleading_conduct() {
        let facts = MisleadingConductFacts {
            in_trade_or_commerce: true,
            conduct_occurred: true,
            would_lead_into_error: true,
            reliance_on_conduct: true,
            loss_suffered: true,
            ..Default::default()
        };

        let result = MisleadingConductAnalyzer::analyze(&facts);
        assert!(result.liability_established);
        assert!(result.damages_available);
    }

    #[test]
    fn test_unfair_term() {
        let facts = UnfairTermFacts {
            term_description: "Unilateral variation clause".to_string(),
            term_type: Some(UnfairTermType::UnilateralVariation),
            standard_form_contract: true,
            consumer_contract: true,
            causes_significant_imbalance: true,
            would_cause_detriment: true,
            ..Default::default()
        };

        let result = UnfairTermsAnalyzer::analyze(&facts);
        assert!(result.is_unfair);
        assert!(result.void);
    }
}
