//! Breach of Trust and Remedies
//!
//! This module implements breach of trust law, including:
//! - Personal remedies against trustees
//! - Proprietary remedies (tracing)
//! - Third party liability (dishonest assistance, knowing receipt)
//!
//! ## Breach of Trust
//!
//! A breach of trust occurs when a trustee:
//! - Acts outside their powers
//! - Fails to perform their duties
//! - Makes an unauthorized profit
//! - Causes loss to the trust
//!
//! ## Personal Remedies
//!
//! Beneficiaries can claim:
//! - **Compensation** for loss caused (Target Holdings v Redferns [1996])
//! - **Account of profits** for unauthorized gains (Boardman v Phipps)
//! - **Equitable compensation** (AIB v Mark Redler [2014])
//!
//! ## Proprietary Remedies - Tracing
//!
//! ### Common Law Tracing
//! - Follows legal title
//! - Fails if property mixed (Taylor v Plumer [1815])
//!
//! ### Equitable Tracing
//! - Can trace through mixtures (Re Hallett's Estate [1880])
//! - Clayton's Case - first in, first out (but often disapplied)
//! - Re Oatway [1903] - trustee deemed to spend own money first if dissipated
//! - Foskett v McKeown [2001] - can trace into substitutes
//!
//! ## Third Party Liability
//!
//! ### Dishonest Assistance (Royal Brunei Airlines v Tan [1995])
//! - Assistance in breach of trust
//! - Dishonesty (objective standard - Ivey v Genting Casinos [2017])
//!
//! ### Knowing Receipt (BCCI v Akindele [2001])
//! - Receipt of trust property
//! - Beneficial receipt
//! - Knowledge making retention unconscionable
//!
//! ## Defences
//!
//! - **s.61 TA 1925** - Court may relieve honest and reasonable trustee
//! - **Beneficiary consent** - Fully informed, sui juris
//! - **Acquiescence** - Beneficiary accepted breach
//! - **Limitation** - 6 years (but not for fraud or trustee benefiting)

use serde::{Deserialize, Serialize};

use super::error::{TrustError, TrustResult};
use super::types::PropertyType;

// ============================================================================
// Breach of Trust
// ============================================================================

/// Category of breach of trust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BreachCategory {
    /// Acting outside powers
    ExcessOfPower {
        /// The power claimed by trustee
        power_claimed: String,
    },
    /// Failure to perform duty
    FailureOfDuty {
        /// The duty that was breached
        duty: String,
    },
    /// Unauthorized profit
    UnauthorizedProfit {
        /// Amount of unauthorized profit
        profit_amount: Option<f64>,
    },
    /// Misapplication of trust property
    Misapplication {
        /// Description of misapplication
        description: String,
    },
    /// Improper investment
    ImproperInvestment {
        /// The investment made
        investment: String,
    },
    /// Failure to distribute
    FailureToDistribute,
    /// Self-dealing
    SelfDealing {
        /// The transaction involved
        transaction: String,
    },
    /// Negligent management
    NegligentManagement {
        /// Description of negligent act
        description: String,
    },
}

/// A breach of trust occurrence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachOfTrust {
    /// Category of breach
    pub category: BreachCategory,
    /// Description
    pub description: String,
    /// Severity
    pub severity: BreachSeverity,
    /// Trustees involved
    pub trustees_involved: Vec<String>,
    /// Was breach dishonest? (affects limitation)
    pub dishonest: bool,
    /// Did trustee benefit personally?
    pub trustee_benefited: bool,
    /// Loss caused (if quantifiable)
    pub loss_caused: Option<f64>,
    /// Profit made by trustee (if any)
    pub profit_made: Option<f64>,
    /// Available defences
    pub potential_defences: Vec<BreachDefence>,
}

/// Severity of breach
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BreachSeverity {
    /// Minor technical breach
    Minor,
    /// Moderate breach causing some loss
    Moderate,
    /// Serious breach causing significant loss
    Serious,
    /// Fraudulent or grossly negligent breach
    Gross,
}

impl BreachSeverity {
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Minor => "Minor technical breach with minimal impact",
            Self::Moderate => "Moderate breach causing recoverable loss",
            Self::Serious => "Serious breach causing significant harm to trust",
            Self::Gross => "Gross breach involving dishonesty or recklessness",
        }
    }

    /// Does severity affect limitation period?
    pub fn affects_limitation(&self) -> bool {
        // Fraudulent breach - no limitation (LA 1980 s.21)
        matches!(self, Self::Gross)
    }
}

/// Available defences to breach of trust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BreachDefence {
    /// Court relief under s.61 TA 1925
    Section61Relief,
    /// Beneficiary consent (fully informed, sui juris)
    BeneficiaryConsent,
    /// Beneficiary acquiescence
    Acquiescence,
    /// Limitation (6 years - LA 1980 s.21)
    Limitation {
        /// Years since the breach occurred
        years_since_breach: u32,
    },
    /// Laches (delay making claim inequitable)
    Laches,
    /// Exemption clause in trust deed
    ExemptionClause {
        /// Text of the exemption clause
        clause_text: String,
    },
    /// Trustee acted on legal advice
    LegalAdvice,
    /// Impounding beneficiary's interest (TA 1925 s.62)
    Impounding,
}

impl BreachDefence {
    /// Check if defence is available
    pub fn is_available(&self, breach: &BreachOfTrust) -> bool {
        match self {
            Self::Section61Relief => {
                // Available if trustee acted honestly and reasonably
                !breach.dishonest
            }
            Self::BeneficiaryConsent => true, // Depends on facts
            Self::Acquiescence => true,       // Depends on facts
            Self::Limitation { years_since_breach } => {
                // LA 1980 s.21 - 6 years, but not for:
                // - Fraud (s.21(1)(a))
                // - Trustee benefiting (s.21(1)(b))
                if breach.dishonest || breach.trustee_benefited {
                    false
                } else {
                    *years_since_breach >= 6
                }
            }
            Self::Laches => !breach.dishonest, // Not available for fraud
            Self::ExemptionClause { .. } => {
                // Per Armitage v Nurse, cannot exclude liability for:
                // - Fraud
                // - Dishonesty
                // - Recklessness (debatable)
                !breach.dishonest && !matches!(breach.severity, BreachSeverity::Gross)
            }
            Self::LegalAdvice => true, // Mitigating factor
            Self::Impounding => true,  // If beneficiary instigated breach
        }
    }
}

// ============================================================================
// Breach Remedies
// ============================================================================

/// Type of remedy for breach of trust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BreachRemedy {
    /// Compensation for loss (personal remedy)
    Compensation(CompensationClaim),
    /// Account of profits (personal remedy)
    AccountOfProfits(AccountOfProfits),
    /// Equitable compensation (AIB v Mark Redler approach)
    EquitableCompensation(EquitableCompensation),
    /// Proprietary remedy through tracing
    ProprietaryRemedy(ProprietaryRemedy),
    /// Removal of trustee
    RemovalOfTrustee,
    /// Injunction to prevent further breach
    Injunction,
}

/// Compensation claim for breach
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompensationClaim {
    /// Amount claimed
    pub amount_gbp: f64,
    /// Basis of calculation
    pub calculation_basis: CompensationBasis,
    /// Causation analysis
    pub causation: CausationAnalysis,
    /// Interest claimed
    pub interest: Option<InterestClaim>,
}

/// Basis for calculating compensation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompensationBasis {
    /// Restore trust fund to position pre-breach
    RestorationOfTrustFund,
    /// Substitute performance basis (Target Holdings)
    SubstitutePerformance,
    /// Reflective loss
    ReflectiveLoss,
}

/// Causation analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CausationAnalysis {
    /// Did breach cause loss?
    pub breach_caused_loss: bool,
    /// Would loss have occurred anyway?
    pub loss_would_have_occurred: bool,
    /// But-for test satisfied?
    pub but_for_satisfied: bool,
    /// Analysis notes
    pub analysis: String,
}

/// Interest claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterestClaim {
    /// Rate of interest
    pub rate_percent: f64,
    /// Simple or compound
    pub compound: bool,
    /// Period
    pub period_months: u32,
    /// Calculated amount
    pub calculated_amount: f64,
}

impl InterestClaim {
    /// Calculate interest
    pub fn calculate(principal: f64, rate_percent: f64, months: u32, compound: bool) -> Self {
        let years = months as f64 / 12.0;
        let calculated_amount = if compound {
            principal * (1.0 + rate_percent / 100.0).powf(years) - principal
        } else {
            principal * (rate_percent / 100.0) * years
        };

        Self {
            rate_percent,
            compound,
            period_months: months,
            calculated_amount,
        }
    }
}

/// Account of profits claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountOfProfits {
    /// Profits to be disgorged
    pub profit_amount: f64,
    /// Source of profit
    pub profit_source: String,
    /// Is profit attributable to breach?
    pub attributable: bool,
    /// Trustee entitled to allowance?
    pub allowance_for_skill: Option<f64>,
}

/// Equitable compensation (AIB v Mark Redler approach)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquitableCompensation {
    /// Traditional "falsification" of account
    pub falsification_appropriate: bool,
    /// Or "surcharging" the account
    pub surcharge_appropriate: bool,
    /// Calculated amount
    pub amount_gbp: f64,
    /// Approach to causation (Target Holdings vs traditional)
    pub causation_approach: CausationApproach,
    /// Analysis
    pub analysis: String,
}

/// Approach to causation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CausationApproach {
    /// Traditional - strict (no but-for requirement)
    Traditional,
    /// Target Holdings - commercial but-for causation
    TargetHoldings,
    /// AIB v Mark Redler - depends on transaction type
    AIBMarkRedler,
}

// ============================================================================
// Proprietary Remedies - Tracing
// ============================================================================

/// Proprietary remedy through tracing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProprietaryRemedy {
    /// Method of tracing
    pub tracing_method: TracingMethod,
    /// Property traced to
    pub traced_property: TracedProperty,
    /// Type of claim
    pub claim_type: ProprietaryClaim,
    /// Is proprietary remedy available?
    pub available: bool,
    /// Reasons if not available
    pub unavailability_reasons: Vec<String>,
}

/// Method of tracing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TracingMethod {
    /// Common law tracing (follows legal title)
    CommonLaw,
    /// Equitable tracing (can trace through mixtures)
    Equitable,
}

impl TracingMethod {
    /// Get applicable rules
    pub fn rules(&self) -> &'static str {
        match self {
            Self::CommonLaw => {
                "Common law tracing follows legal title. Fails if property mixed with \
                 other property (Taylor v Plumer [1815]). Claimant must show property \
                 was their property or held for them."
            }
            Self::Equitable => {
                "Equitable tracing can follow property through mixtures. Requires \
                 fiduciary relationship. Key rules: Re Hallett's Estate (trustee \
                 deemed to spend own money first), Re Oatway (unless trust money \
                 dissipated), Clayton's Case (FIFO - often disapplied)."
            }
        }
    }

    /// Can trace through mixture?
    pub fn can_trace_through_mixture(&self) -> bool {
        matches!(self, Self::Equitable)
    }
}

/// Property traced to
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TracedProperty {
    /// Description of property
    pub description: String,
    /// Type of property
    pub property_type: PropertyType,
    /// Value
    pub value_gbp: f64,
    /// Is property mixed?
    pub is_mixed: bool,
    /// If mixed, what is claimant's share?
    pub claimant_share: Option<f64>,
    /// Has property been dissipated?
    pub dissipated: bool,
    /// Has property increased in value?
    pub value_increased: bool,
    /// Original contribution
    pub original_contribution: f64,
}

/// Type of proprietary claim
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProprietaryClaim {
    /// Beneficial ownership of specific property
    BeneficialOwnership,
    /// Equitable charge over property
    EquitableCharge,
    /// Equitable lien
    EquitableLien,
    /// Constructive trust over property
    ConstructiveTrust,
    /// Subrogation
    Subrogation,
}

/// Calculate tracing remedy
pub fn calculate_tracing_remedy(
    traced: &TracedProperty,
    method: TracingMethod,
) -> TrustResult<f64> {
    // Check if tracing is possible
    if traced.dissipated {
        return Err(TrustError::BreachOfTrust {
            description: "Property dissipated - proprietary remedy unavailable".to_string(),
        });
    }

    if traced.is_mixed && method == TracingMethod::CommonLaw {
        return Err(TrustError::BreachOfTrust {
            description: "Common law tracing fails when property is mixed".to_string(),
        });
    }

    // Calculate value claimant entitled to
    if traced.value_increased {
        // Foskett v McKeown - claimant can share in increase proportionately
        let proportion = traced.original_contribution / traced.value_gbp;
        let current_value = traced.value_gbp;
        Ok(current_value * proportion)
    } else if let Some(share) = traced.claimant_share {
        // Mixed property - claimant gets their share
        Ok(traced.value_gbp * share)
    } else {
        // Unmixed or sole ownership
        Ok(traced.value_gbp.min(traced.original_contribution))
    }
}

// ============================================================================
// Third Party Liability
// ============================================================================

/// Dishonest assistance claim (Royal Brunei Airlines v Tan)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DishonestAssistance {
    /// The assister
    pub assister: String,
    /// The breach assisted
    pub breach_assisted: String,
    /// Type of assistance
    pub assistance_type: AssistanceType,
    /// Dishonesty analysis
    pub dishonesty: DishonestyAnalysis,
    /// Is claim established?
    pub established: bool,
    /// Analysis
    pub analysis: String,
}

/// Type of assistance provided
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssistanceType {
    /// Actively procured breach
    Procurement,
    /// Facilitated breach
    Facilitation,
    /// Helped conceal breach
    Concealment,
    /// Received and dealt with trust property
    DealingWithProperty,
}

/// Dishonesty analysis (Ivey v Genting Casinos [2017])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DishonestyAnalysis {
    /// Assister's subjective knowledge/belief
    pub subjective_knowledge: String,
    /// Objective assessment - would honest person consider conduct dishonest?
    pub objectively_dishonest: bool,
    /// Combined Ivey test result
    pub dishonest: bool,
    /// Analysis notes
    pub analysis: String,
}

impl DishonestyAnalysis {
    /// Apply Ivey v Genting test
    pub fn apply_ivey_test(subjective_knowledge: &str, objectively_dishonest: bool) -> Self {
        // Ivey test: What did D subjectively know/believe?
        // Then: Would ordinary honest person consider that conduct dishonest?
        // (No longer ask whether D realized it was dishonest - overruled Ghosh)

        let analysis = if objectively_dishonest {
            format!(
                "Applying Ivey v Genting Casinos [2017]: Given D's knowledge that '{}', \
                 an ordinary honest person would consider the conduct dishonest.",
                subjective_knowledge
            )
        } else {
            format!(
                "Applying Ivey v Genting Casinos [2017]: Given D's knowledge that '{}', \
                 an ordinary honest person would not consider the conduct dishonest.",
                subjective_knowledge
            )
        };

        Self {
            subjective_knowledge: subjective_knowledge.to_string(),
            objectively_dishonest,
            dishonest: objectively_dishonest,
            analysis,
        }
    }
}

/// Validate dishonest assistance claim
pub fn validate_dishonest_assistance(
    assister: &str,
    breach_assisted: &str,
    assistance_type: AssistanceType,
    subjective_knowledge: &str,
    objectively_dishonest: bool,
) -> DishonestAssistance {
    let dishonesty =
        DishonestyAnalysis::apply_ivey_test(subjective_knowledge, objectively_dishonest);
    let established = dishonesty.dishonest;

    let analysis = if established {
        format!(
            "Dishonest assistance claim established against {}. Following Royal Brunei Airlines \
             v Tan [1995], liable for assistance in breach. The breach assisted: '{}'. \
             Assistance type: {:?}. {}",
            assister, breach_assisted, assistance_type, dishonesty.analysis
        )
    } else {
        format!(
            "Dishonest assistance claim against {} not established - dishonesty element not \
             satisfied. {}",
            assister, dishonesty.analysis
        )
    };

    DishonestAssistance {
        assister: assister.to_string(),
        breach_assisted: breach_assisted.to_string(),
        assistance_type,
        dishonesty,
        established,
        analysis,
    }
}

/// Knowing receipt claim (BCCI v Akindele [2001])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowingReceipt {
    /// The recipient
    pub recipient: String,
    /// Property received
    pub property_received: String,
    /// Value received
    pub value_gbp: f64,
    /// Was receipt beneficial?
    pub beneficial_receipt: bool,
    /// Knowledge analysis
    pub knowledge: KnowledgeAnalysis,
    /// Is claim established?
    pub established: bool,
    /// Analysis
    pub analysis: String,
}

/// Knowledge analysis for knowing receipt
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeAnalysis {
    /// Type of knowledge
    pub knowledge_type: KnowledgeType,
    /// Would retention be unconscionable?
    pub unconscionable: bool,
    /// Analysis notes
    pub notes: String,
}

/// Baden scale of knowledge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KnowledgeType {
    /// (i) Actual knowledge
    ActualKnowledge,
    /// (ii) Wilfully shutting eyes (Nelsonian knowledge)
    WilfulBlindness,
    /// (iii) Wilfully and recklessly failing to inquire
    WilfulFailureToInquire,
    /// (iv) Knowledge of circumstances putting on inquiry
    ConstructiveNotice,
    /// (v) Knowledge of circumstances that would have put reasonable person on inquiry
    ImputtedNotice,
    /// No knowledge
    None,
}

impl KnowledgeType {
    /// Is this level of knowledge sufficient for knowing receipt?
    /// BCCI v Akindele says threshold is "unconscionability" not Baden scale
    pub fn sufficient_for_receipt(&self) -> bool {
        matches!(
            self,
            Self::ActualKnowledge | Self::WilfulBlindness | Self::WilfulFailureToInquire
        )
    }

    /// Description
    pub fn description(&self) -> &'static str {
        match self {
            Self::ActualKnowledge => "Actual knowledge of trust and breach",
            Self::WilfulBlindness => "Wilfully shut eyes to obvious (Nelsonian knowledge)",
            Self::WilfulFailureToInquire => "Wilfully and recklessly failed to make inquiries",
            Self::ConstructiveNotice => "Knowledge of facts putting on inquiry",
            Self::ImputtedNotice => "Knowledge a reasonable person would have acquired",
            Self::None => "No knowledge of breach or trust",
        }
    }
}

/// Validate knowing receipt claim
pub fn validate_knowing_receipt(
    recipient: &str,
    property: &str,
    value: f64,
    beneficial: bool,
    knowledge_type: KnowledgeType,
) -> KnowingReceipt {
    let knowledge = KnowledgeAnalysis {
        knowledge_type,
        unconscionable: knowledge_type.sufficient_for_receipt(),
        notes: format!(
            "Knowledge level: {}. Following BCCI v Akindele [2001], the test is whether \
             recipient's state of knowledge makes it unconscionable to retain the benefit.",
            knowledge_type.description()
        ),
    };

    let established = beneficial && knowledge.unconscionable;

    let analysis = if established {
        format!(
            "Knowing receipt claim established against {}. Received {} (£{:.2}) beneficially \
             with {}. Retention unconscionable - liable as constructive trustee.",
            recipient,
            property,
            value,
            knowledge_type.description().to_lowercase()
        )
    } else if !beneficial {
        format!(
            "Knowing receipt claim against {} fails - receipt was not beneficial (e.g., \
             agent receiving as agent).",
            recipient
        )
    } else {
        format!(
            "Knowing receipt claim against {} not established - knowledge level ({}) \
             insufficient to make retention unconscionable.",
            recipient,
            knowledge_type.description()
        )
    };

    KnowingReceipt {
        recipient: recipient.to_string(),
        property_received: property.to_string(),
        value_gbp: value,
        beneficial_receipt: beneficial,
        knowledge,
        established,
        analysis,
    }
}

// ============================================================================
// Breach Assessment
// ============================================================================

/// Assess a breach of trust
pub fn assess_breach_of_trust(
    category: BreachCategory,
    description: &str,
    trustees: Vec<String>,
    dishonest: bool,
    trustee_benefited: bool,
    loss: Option<f64>,
    profit: Option<f64>,
) -> BreachOfTrust {
    // Determine severity
    let severity = if dishonest {
        BreachSeverity::Gross
    } else if trustee_benefited || loss.is_some_and(|l| l > 100_000.0) {
        BreachSeverity::Serious
    } else if loss.is_some() {
        BreachSeverity::Moderate
    } else {
        BreachSeverity::Minor
    };

    // Identify potential defences
    let mut defences = Vec::new();

    if !dishonest {
        defences.push(BreachDefence::Section61Relief);
    }
    defences.push(BreachDefence::BeneficiaryConsent);
    if !dishonest && !trustee_benefited {
        defences.push(BreachDefence::Limitation {
            years_since_breach: 0,
        });
    }

    BreachOfTrust {
        category,
        description: description.to_string(),
        severity,
        trustees_involved: trustees,
        dishonest,
        trustee_benefited,
        loss_caused: loss,
        profit_made: profit,
        potential_defences: defences,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breach_severity_affects_defences() {
        let breach = assess_breach_of_trust(
            BreachCategory::SelfDealing {
                transaction: "Purchase of trust property".to_string(),
            },
            "Trustee purchased property from trust",
            vec!["John Smith".to_string()],
            true, // Dishonest
            true, // Benefited
            Some(50_000.0),
            Some(10_000.0),
        );

        assert_eq!(breach.severity, BreachSeverity::Gross);
        // Limitation not available for dishonest breach
        let limitation = BreachDefence::Limitation {
            years_since_breach: 10,
        };
        assert!(!limitation.is_available(&breach));
    }

    #[test]
    fn test_tracing_fails_common_law_mixed() {
        let traced = TracedProperty {
            description: "Bank account balance".to_string(),
            property_type: PropertyType::Money,
            value_gbp: 50_000.0,
            is_mixed: true,
            claimant_share: Some(0.5),
            dissipated: false,
            value_increased: false,
            original_contribution: 25_000.0,
        };

        let result = calculate_tracing_remedy(&traced, TracingMethod::CommonLaw);
        assert!(result.is_err());
    }

    #[test]
    fn test_tracing_equitable_mixed_succeeds() {
        let traced = TracedProperty {
            description: "Bank account balance".to_string(),
            property_type: PropertyType::Money,
            value_gbp: 50_000.0,
            is_mixed: true,
            claimant_share: Some(0.5),
            dissipated: false,
            value_increased: false,
            original_contribution: 25_000.0,
        };

        let result = calculate_tracing_remedy(&traced, TracingMethod::Equitable);
        assert!(result.is_ok());
        assert!((result.unwrap() - 25_000.0).abs() < 0.01);
    }

    #[test]
    fn test_dishonest_assistance_ivey() {
        let claim = validate_dishonest_assistance(
            "Solicitor X",
            "Transfer of trust funds to offshore account",
            AssistanceType::Facilitation,
            "Knew client was trustee and transfer was unauthorized",
            true,
        );

        assert!(claim.established);
        assert!(claim.analysis.contains("Royal Brunei"));
    }

    #[test]
    fn test_knowing_receipt_unconscionable() {
        let claim = validate_knowing_receipt(
            "Company Y",
            "Trust funds",
            100_000.0,
            true,
            KnowledgeType::ActualKnowledge,
        );

        assert!(claim.established);
        assert!(claim.knowledge.unconscionable);
    }

    #[test]
    fn test_knowing_receipt_constructive_notice_fails() {
        let claim = validate_knowing_receipt(
            "Innocent Bank",
            "Wire transfer",
            50_000.0,
            true,
            KnowledgeType::ConstructiveNotice,
        );

        // Mere constructive notice not enough post-Akindele
        assert!(!claim.established);
    }
}
