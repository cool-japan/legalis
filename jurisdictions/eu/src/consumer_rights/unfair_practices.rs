//! Unfair Commercial Practices Directive (2005/29/EC)
//!
//! This module implements the EU's rules against unfair business-to-consumer
//! commercial practices.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

/// Unfair commercial practice classification
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum UnfairCommercialPractice {
    /// Misleading actions (Article 6)
    MisleadingAction(MisleadingAction),

    /// Misleading omissions (Article 7)
    MisleadingOmission(MisleadingOmission),

    /// Aggressive practices (Article 8-9)
    AggressivePractice(AggressivePractice),

    /// Prohibited practices (Annex I - "blacklist")
    ProhibitedPractice(ProhibitedPractice),
}

/// Misleading actions (Article 6)
///
/// Commercial practices that contain false information or deceive the average consumer
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum MisleadingAction {
    /// False information about existence/nature of product
    FalseProductInformation,

    /// False information about main characteristics
    /// (availability, benefits, risks, composition, accessories, etc.)
    FalseCharacteristics {
        /// What characteristic was misrepresented
        characteristic: String,
    },

    /// False information about extent of trader's commitments
    FalseCommitments,

    /// False information about price or manner of calculation
    FalsePrice {
        /// Description of the misleading pricing
        description: String,
    },

    /// False information about trader's identity, qualifications, status
    FalseTraderIdentity,

    /// False information about consumer rights
    /// (e.g., right of withdrawal, guarantee)
    FalseConsumerRights {
        /// Which right was misrepresented
        right: String,
    },

    /// False endorsements, approval marks, or awards
    FalseEndorsements,

    /// Confusion with competitor's products/trademarks
    ConfusionWithCompetitor,

    /// Non-compliance with code of conduct
    CodeOfConductViolation {
        /// Which code was violated
        code: String,
    },
}

/// Misleading omissions (Article 7)
///
/// Omitting or hiding material information that the average consumer needs
/// to make an informed transactional decision
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum MisleadingOmission {
    /// Hiding material information needed for informed decision
    HiddenMaterialInformation {
        /// What information was hidden
        information: String,
    },

    /// Providing material information in unclear/ambiguous manner
    UnclearInformation {
        /// Description of the unclear information
        description: String,
    },

    /// Failing to identify commercial intent (hidden advertising)
    HiddenAdvertising,

    /// Omitting information required by EU law
    /// (e.g., right of withdrawal, total price)
    OmittedLegalInformation {
        /// Which legal requirement was omitted
        requirement: String,
    },
}

/// Aggressive commercial practices (Articles 8-9)
///
/// Practices that significantly impair the average consumer's freedom of choice
/// through harassment, coercion, or undue influence
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum AggressivePractice {
    /// Harassment (repeated/unwanted solicitations)
    Harassment {
        /// Description of harassing behavior
        description: String,
    },

    /// Coercion (physical or non-physical)
    Coercion {
        /// Type of coercion
        coercion_type: String,
    },

    /// Undue influence (exploiting position of power)
    UndueInfluence {
        /// Description of influence
        description: String,
    },

    /// Obstacles to exercising contractual rights
    /// (e.g., claiming insurance, switching suppliers)
    ObstaclesToRights {
        /// Which right was obstructed
        right: String,
    },

    /// Threats of action that cannot legally be taken
    IllegalThreats {
        /// Description of threat
        threat: String,
    },
}

/// Prohibited practices - "Blacklist" (Annex I)
///
/// These 31 practices are ALWAYS considered unfair
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ProhibitedPractice {
    /// #1 - Claiming adherence to code of conduct without actually adhering
    FalseCodeAdherence,

    /// #2 - Displaying trust mark/quality mark without authorization
    UnauthorizedTrustMark,

    /// #3 - Claiming code of conduct has endorsement of public/private body
    FalseCodeEndorsement,

    /// #4 - Claiming trader/product approved by public/private body (when not)
    FalseApproval,

    /// #5 - Making invitation to purchase without revealing true commercial intent (advertorial)
    HiddenAdvertorial,

    /// #6 - Claiming product cures illnesses, dysfunctions without evidence
    FalseHealthClaims,

    /// #7 - Materially inaccurate information on market conditions to induce purchase
    FalseMarketConditions,

    /// #8 - Prize promotion where no prize exists or claiming prize without conditions
    FalsePrizes,

    /// #9 - Describing product as "free" when consumer must pay anything other than unavoidable costs
    FalseFree,

    /// #10 - Including invoice demanding payment for unsolicited products (inertia selling)
    InertiaSelling,

    /// #11 - Falsely stating product available only for limited time to elicit immediate decision
    FalseScarcity,

    /// #12 - Undertaking to provide after-sales service in language other than official EU language of Member State
    LanguageMisrepresentation,

    /// #13 - Stating purchase necessary for trader's job/livelihood
    FalseJobNecessity,

    /// #14 - Creating impression consumer cannot leave premises until contract formed
    PhysicalDetention,

    /// #15 - Conducting personal visits to consumer's home ignoring request to leave
    UnwantedHomeVisits,

    /// #16 - Making persistent/unwanted solicitations by phone/fax/email (spam)
    PersistentSolicitation,

    /// #17 - Requiring consumer to pay disproportionately high charges for non-performance of contractual obligations
    DisproportionatePenalties,

    /// #18 - Informing consumer that trader's job/livelihood threatened if consumer doesn't buy
    FalseJobThreat,

    /// #19 - Falsely stating product can legally be sold
    FalseLegality,

    /// #20 - Presenting rights as distinctive feature of offer when it's actually a legal right
    PresentingLegalRightsAsFeatures,

    /// #21 - Using editorial content in media for promotion, paying for it without making it clear (advertorial)
    PaidEditorialContent,

    /// #22 - Making materially inaccurate claim about nature/extent of risk to consumer's security
    FalseRiskClaims,

    /// #23 - Promoting product similar to another trader's product to deliberately confuse
    DeliberateConfusion,

    /// #24 - Creating/promoting pyramid promotional scheme
    PyramidScheme,

    /// #25 - Claiming product increases chances of winning games of chance
    FalseGamblingClaims,

    /// #26 - Falsely claiming product can facilitate winning in games of chance
    FalseGamblingFacilitation,

    /// #27 - Falsely claiming testimonial about product is from satisfied consumer
    FakeTestimonials,

    /// #28 - Claiming trader about to cease trading/move premises when not
    FalseClosure,

    /// #29 - Claiming product can facilitate winning in games of chance (repeated for emphasis in directive)
    FalseGamblingWinning,

    /// #30 - Describing product as "free" when not (duplicate of #9)
    FalseFreeProduct,

    /// #31 - Including in marketing material invoice/similar document seeking payment giving impression already ordered
    FalseInvoice,
}

/// Assessment criteria for unfair practices (Article 5)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct UnfairPracticeAssessment {
    /// The practice being assessed
    pub practice_description: String,

    /// Whether practice is contrary to professional diligence
    pub contrary_to_professional_diligence: bool,

    /// Whether practice materially distorts economic behavior of average consumer
    pub materially_distorts_behavior: bool,

    /// Target audience (if practice targets specific group)
    pub target_audience: Option<TargetAudience>,

    /// Whether practice is on the blacklist (Annex I)
    pub on_blacklist: bool,
}

/// Target audience for commercial practice
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum TargetAudience {
    /// Average consumer
    AverageConsumer,

    /// Vulnerable group (children, elderly, disabled)
    VulnerableGroup {
        /// Description of vulnerability
        vulnerability: String,
    },

    /// Specific identifiable group with particular characteristics
    SpecificGroup {
        /// Group description
        group: String,
    },
}

/// Average consumer standard
///
/// The directive uses the "average consumer" as the benchmark - reasonably well-informed,
/// observant and circumspect, taking into account social, cultural and linguistic factors.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct AverageConsumerStandard {
    /// Reasonably well-informed
    pub reasonably_informed: bool,

    /// Reasonably observant
    pub reasonably_observant: bool,

    /// Reasonably circumspect
    pub reasonably_circumspect: bool,

    /// Social factors considered
    pub social_factors: Vec<String>,

    /// Cultural factors considered
    pub cultural_factors: Vec<String>,

    /// Linguistic factors considered
    pub linguistic_factors: Vec<String>,
}

/// Transactional decision
///
/// Any decision by a consumer whether to act or refrain from acting concerning:
/// - Whether to purchase a product
/// - How to purchase (terms, conditions)
/// - Whether to pay in full or in part
/// - Whether to keep or dispose of product
/// - Whether to exercise contractual right
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum TransactionalDecision {
    /// Whether to purchase
    WhetherToPurchase,

    /// How to purchase (terms/conditions)
    HowToPurchase,

    /// Whether to pay (in full/in part)
    WhetherToPay,

    /// Whether to keep or dispose
    WhetherToKeep,

    /// Whether to exercise contractual right
    WhetherToExerciseRight,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_misleading_action() {
        let practice = UnfairCommercialPractice::MisleadingAction(MisleadingAction::FalsePrice {
            description: "Showing crossed-out 'original price' that was never charged".to_string(),
        });

        assert!(matches!(
            practice,
            UnfairCommercialPractice::MisleadingAction(_)
        ));
    }

    #[test]
    fn test_prohibited_practice() {
        let practice = UnfairCommercialPractice::ProhibitedPractice(ProhibitedPractice::FalseFree);

        assert!(matches!(
            practice,
            UnfairCommercialPractice::ProhibitedPractice(_)
        ));
    }

    #[test]
    fn test_unfair_assessment() {
        let assessment = UnfairPracticeAssessment {
            practice_description: "Hidden subscription charges".to_string(),
            contrary_to_professional_diligence: true,
            materially_distorts_behavior: true,
            target_audience: Some(TargetAudience::AverageConsumer),
            on_blacklist: false,
        };

        // Practice is unfair if contrary to professional diligence AND materially distorts behavior
        assert!(
            assessment.contrary_to_professional_diligence
                && assessment.materially_distorts_behavior
        );
    }
}
