//! Canada Corporate Law - Oppression Remedy Analysis
//!
//! Analysis of oppression remedy under CBCA s.241.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{ComplainantType, OppressionConduct};

// ============================================================================
// Oppression Remedy Analysis
// ============================================================================

/// Facts for oppression remedy analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OppressionFacts {
    /// Complainant information
    pub complainant: ComplainantInfo,
    /// Corporation information
    pub corporation: String,
    /// Alleged oppressive conduct
    pub conduct: AllegedConduct,
    /// Reasonable expectations claimed
    pub expectations: Vec<ReasonableExpectation>,
    /// Context factors
    pub context: OppressionContext,
}

/// Complainant information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplainantInfo {
    /// Name
    pub name: String,
    /// Type of complainant
    pub complainant_type: ComplainantType,
    /// Shareholding percentage (if applicable)
    pub shareholding: Option<f64>,
    /// Duration of relationship
    pub relationship_duration_years: Option<u32>,
}

/// Alleged conduct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllegedConduct {
    /// Description of conduct
    pub description: String,
    /// Type of conduct
    pub conduct_type: ConductType,
    /// Who engaged in conduct
    pub actors: Vec<String>,
    /// When conduct occurred
    pub date: String,
    /// Is conduct ongoing
    pub ongoing: bool,
}

/// Type of allegedly oppressive conduct
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConductType {
    /// Exclusion from management
    ExclusionFromManagement,
    /// Withholding dividends
    WithholdingDividends,
    /// Excessive remuneration to majority
    ExcessiveRemuneration,
    /// Dilution of shareholding
    Dilution,
    /// Misappropriation of assets
    Misappropriation,
    /// Breach of shareholders agreement
    BreachOfAgreement,
    /// Squeeze out/freeze out
    SqueezeOut,
    /// Related party transactions
    RelatedPartyTransactions,
    /// Failure to provide information
    InformationDenial,
    /// Improper amendment of articles
    ImproperAmendment,
    /// Other
    Other(String),
}

/// Reasonable expectation claimed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonableExpectation {
    /// Description of expectation
    pub expectation: String,
    /// Source of expectation
    pub source: ExpectationSource,
    /// Strength of expectation
    pub strength: ExpectationStrength,
}

/// Source of reasonable expectation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpectationSource {
    /// Shareholders agreement
    ShareholdersAgreement,
    /// Course of dealing
    CourseOfDealing,
    /// Representations made
    Representations,
    /// Nature of corporation (quasi-partnership)
    QuasiPartnership,
    /// Corporate law principles
    CorporateLawPrinciples,
    /// Industry practice
    IndustryPractice,
    /// Promotional materials
    PromotionalMaterials,
}

/// Strength of expectation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ExpectationStrength {
    /// Weak/speculative
    Weak,
    /// Moderate
    Moderate,
    /// Strong
    Strong,
    /// Very strong (documentary evidence)
    VeryStrong,
}

/// Context for oppression analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OppressionContext {
    /// Is this a closely held corporation
    pub closely_held: bool,
    /// Is there a shareholders agreement
    pub shareholders_agreement: bool,
    /// Are there equal shareholders
    pub equal_shareholders: bool,
    /// Is this a quasi-partnership
    pub quasi_partnership: bool,
    /// Has there been an exit offer
    pub exit_offer: Option<ExitOffer>,
}

/// Exit offer made
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitOffer {
    /// Offer amount
    pub amount: f64,
    /// Valuation basis
    pub valuation_basis: ValuationBasis,
    /// Discount applied
    pub minority_discount: bool,
}

/// Valuation basis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValuationBasis {
    /// Fair market value
    FairMarketValue,
    /// Book value
    BookValue,
    /// Liquidation value
    LiquidationValue,
    /// Earnings multiple
    EarningsMultiple,
    /// Arbitrary
    Arbitrary,
}

/// Result of oppression analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OppressionResult {
    /// Is complainant status established
    pub complainant_status: bool,
    /// Are reasonable expectations established
    pub expectations_established: bool,
    /// Expectations found reasonable
    pub reasonable_expectations: Vec<String>,
    /// Is conduct oppressive/unfairly prejudicial/unfairly disregards
    pub oppression_found: bool,
    /// Type of oppression
    pub oppression_type: Option<OppressionConduct>,
    /// Recommended remedies
    pub remedies: Vec<OppressionRemedy>,
    /// Reasoning
    pub reasoning: String,
}

/// Oppression remedy type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OppressionRemedy {
    /// Order to purchase shares
    SharePurchase { by_whom: SharePurchaser },
    /// Order to pay compensation
    Compensation { amount_type: CompensationType },
    /// Order rectifying matters
    Rectification,
    /// Order restraining conduct
    Restraining,
    /// Order varying transaction
    VaryingTransaction,
    /// Appointment of receiver
    Receiver,
    /// Winding up order
    WindingUp,
    /// Other order
    Other(String),
}

/// Who purchases shares
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharePurchaser {
    /// Corporation
    Corporation,
    /// Other shareholders
    Shareholders,
    /// Third party
    ThirdParty,
}

/// Compensation type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompensationType {
    /// Fair value of shares
    FairValue,
    /// Lost dividends
    LostDividends,
    /// Lost remuneration
    LostRemuneration,
    /// General damages
    GeneralDamages,
}

/// Oppression remedy analyzer
pub struct OppressionAnalyzer;

impl OppressionAnalyzer {
    /// Analyze oppression remedy claim
    pub fn analyze(facts: &OppressionFacts) -> OppressionResult {
        // Step 1: Establish complainant status
        let complainant_status = Self::check_complainant_status(&facts.complainant);

        // Step 2: Identify reasonable expectations
        let (expectations_established, reasonable_expectations) =
            Self::analyze_expectations(&facts.expectations, &facts.context);

        // Step 3: Determine if conduct is oppressive
        let (oppression_found, oppression_type) =
            Self::analyze_conduct(&facts.conduct, &facts.context, expectations_established);

        // Step 4: Determine appropriate remedies
        let remedies = if oppression_found {
            Self::determine_remedies(facts)
        } else {
            vec![]
        };

        let reasoning = Self::build_reasoning(
            facts,
            complainant_status,
            expectations_established,
            oppression_found,
        );

        OppressionResult {
            complainant_status,
            expectations_established,
            reasonable_expectations,
            oppression_found,
            oppression_type,
            remedies,
            reasoning,
        }
    }

    /// Check complainant status (s.238 CBCA)
    fn check_complainant_status(complainant: &ComplainantInfo) -> bool {
        matches!(
            complainant.complainant_type,
            ComplainantType::RegisteredShareholder
                | ComplainantType::BeneficialShareholder
                | ComplainantType::FormerShareholder
                | ComplainantType::Director
                | ComplainantType::Officer
        )
    }

    /// Analyze reasonable expectations (BCE framework)
    fn analyze_expectations(
        expectations: &[ReasonableExpectation],
        context: &OppressionContext,
    ) -> (bool, Vec<String>) {
        let mut reasonable = Vec::new();

        for exp in expectations {
            let is_reasonable = Self::is_expectation_reasonable(exp, context);
            if is_reasonable {
                reasonable.push(exp.expectation.clone());
            }
        }

        (!reasonable.is_empty(), reasonable)
    }

    /// Determine if expectation is reasonable
    fn is_expectation_reasonable(exp: &ReasonableExpectation, context: &OppressionContext) -> bool {
        // Strong or very strong expectations generally reasonable
        if exp.strength >= ExpectationStrength::Strong {
            return true;
        }

        // Documentary sources carry more weight
        if matches!(
            exp.source,
            ExpectationSource::ShareholdersAgreement | ExpectationSource::Representations
        ) {
            return true;
        }

        // Quasi-partnership context enhances expectations
        if context.quasi_partnership
            && matches!(
                exp.source,
                ExpectationSource::CourseOfDealing | ExpectationSource::QuasiPartnership
            )
        {
            return true;
        }

        // Closely held corporation context
        if context.closely_held && exp.strength >= ExpectationStrength::Moderate {
            return true;
        }

        false
    }

    /// Analyze conduct for oppression
    fn analyze_conduct(
        conduct: &AllegedConduct,
        context: &OppressionContext,
        expectations_exist: bool,
    ) -> (bool, Option<OppressionConduct>) {
        if !expectations_exist {
            return (false, None);
        }

        // Certain conduct types are inherently more likely to be oppressive
        let is_oppressive = match &conduct.conduct_type {
            ConductType::ExclusionFromManagement if context.quasi_partnership => true,
            ConductType::SqueezeOut => true,
            ConductType::Misappropriation => true,
            ConductType::BreachOfAgreement => true,
            ConductType::RelatedPartyTransactions => true,
            ConductType::WithholdingDividends if context.closely_held => true,
            ConductType::ExcessiveRemuneration if context.closely_held => true,
            ConductType::Dilution if context.equal_shareholders => true,
            _ => false,
        };

        if is_oppressive {
            // Determine type based on severity
            let oppression_type = if matches!(
                conduct.conduct_type,
                ConductType::Misappropriation | ConductType::SqueezeOut
            ) {
                OppressionConduct::Oppressive
            } else if matches!(
                conduct.conduct_type,
                ConductType::RelatedPartyTransactions | ConductType::BreachOfAgreement
            ) {
                OppressionConduct::UnfairlyPrejudicial
            } else {
                OppressionConduct::UnfairlyDisregards
            };

            (true, Some(oppression_type))
        } else {
            (false, None)
        }
    }

    /// Determine appropriate remedies
    fn determine_remedies(facts: &OppressionFacts) -> Vec<OppressionRemedy> {
        let mut remedies = Vec::new();

        // Primary remedy for shareholders is usually buyout
        if matches!(
            facts.complainant.complainant_type,
            ComplainantType::RegisteredShareholder | ComplainantType::BeneficialShareholder
        ) {
            remedies.push(OppressionRemedy::SharePurchase {
                by_whom: if facts.context.closely_held {
                    SharePurchaser::Shareholders
                } else {
                    SharePurchaser::Corporation
                },
            });
        }

        // Add compensation if ongoing harm
        if facts.conduct.ongoing {
            remedies.push(OppressionRemedy::Restraining);
        }

        // Add rectification for specific wrongful acts
        match facts.conduct.conduct_type {
            ConductType::ImproperAmendment => {
                remedies.push(OppressionRemedy::VaryingTransaction);
            }
            ConductType::ExclusionFromManagement => {
                remedies.push(OppressionRemedy::Rectification);
            }
            ConductType::WithholdingDividends => {
                remedies.push(OppressionRemedy::Compensation {
                    amount_type: CompensationType::LostDividends,
                });
            }
            _ => {}
        }

        remedies
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &OppressionFacts,
        complainant: bool,
        expectations: bool,
        oppression: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push(
            "BCE Inc v 1976 Debentureholders [2008] SCC 69 - Oppression remedy framework"
                .to_string(),
        );

        // Complainant status
        if complainant {
            parts.push(format!(
                "Complainant status (s.238): {} is {:?}",
                facts.complainant.name, facts.complainant.complainant_type
            ));
        } else {
            parts.push("Complainant status not established".to_string());
        }

        // Reasonable expectations
        if expectations {
            parts.push("Reasonable expectations established".to_string());
        } else {
            parts.push("No reasonable expectations established".to_string());
        }

        // Oppression finding
        if oppression {
            parts.push(format!(
                "Conduct {:?} is oppressive/unfairly prejudicial/unfairly disregards interests",
                facts.conduct.conduct_type
            ));
        } else {
            parts.push("Conduct does not meet oppression threshold".to_string());
        }

        // Quasi-partnership context
        if facts.context.quasi_partnership {
            parts.push(
                "Ebrahimi v Westbourne Galleries [1973] - Quasi-partnership principles apply"
                    .to_string(),
            );
        }

        parts.join(". ")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oppression_found() {
        let facts = OppressionFacts {
            complainant: ComplainantInfo {
                name: "Minority Shareholder".to_string(),
                complainant_type: ComplainantType::RegisteredShareholder,
                shareholding: Some(25.0),
                relationship_duration_years: Some(10),
            },
            corporation: "Test Corp".to_string(),
            conduct: AllegedConduct {
                description: "Excluded from board and management".to_string(),
                conduct_type: ConductType::ExclusionFromManagement,
                actors: vec!["Majority Shareholder".to_string()],
                date: "2024-01-01".to_string(),
                ongoing: true,
            },
            expectations: vec![ReasonableExpectation {
                expectation: "Participation in management".to_string(),
                source: ExpectationSource::QuasiPartnership,
                strength: ExpectationStrength::Strong,
            }],
            context: OppressionContext {
                closely_held: true,
                shareholders_agreement: false,
                equal_shareholders: false,
                quasi_partnership: true,
                exit_offer: None,
            },
        };

        let result = OppressionAnalyzer::analyze(&facts);

        assert!(result.complainant_status);
        assert!(result.expectations_established);
        assert!(result.oppression_found);
        assert!(!result.remedies.is_empty());
    }

    #[test]
    fn test_no_reasonable_expectations() {
        let facts = OppressionFacts {
            complainant: ComplainantInfo {
                name: "Recent Investor".to_string(),
                complainant_type: ComplainantType::RegisteredShareholder,
                shareholding: Some(5.0),
                relationship_duration_years: Some(1),
            },
            corporation: "Public Corp".to_string(),
            conduct: AllegedConduct {
                description: "No dividends declared".to_string(),
                conduct_type: ConductType::WithholdingDividends,
                actors: vec!["Board".to_string()],
                date: "2024-01-01".to_string(),
                ongoing: false,
            },
            expectations: vec![ReasonableExpectation {
                expectation: "Regular dividends".to_string(),
                source: ExpectationSource::IndustryPractice,
                strength: ExpectationStrength::Weak,
            }],
            context: OppressionContext {
                closely_held: false,
                shareholders_agreement: false,
                equal_shareholders: false,
                quasi_partnership: false,
                exit_offer: None,
            },
        };

        let result = OppressionAnalyzer::analyze(&facts);

        assert!(result.complainant_status);
        assert!(!result.expectations_established);
        assert!(!result.oppression_found);
    }

    #[test]
    fn test_shareholders_agreement_expectation() {
        let facts = OppressionFacts {
            complainant: ComplainantInfo {
                name: "Partner".to_string(),
                complainant_type: ComplainantType::RegisteredShareholder,
                shareholding: Some(50.0),
                relationship_duration_years: Some(5),
            },
            corporation: "Joint Venture Corp".to_string(),
            conduct: AllegedConduct {
                description: "Breach of shareholders agreement".to_string(),
                conduct_type: ConductType::BreachOfAgreement,
                actors: vec!["Other Partner".to_string()],
                date: "2024-01-01".to_string(),
                ongoing: true,
            },
            expectations: vec![ReasonableExpectation {
                expectation: "Compliance with shareholders agreement".to_string(),
                source: ExpectationSource::ShareholdersAgreement,
                strength: ExpectationStrength::VeryStrong,
            }],
            context: OppressionContext {
                closely_held: true,
                shareholders_agreement: true,
                equal_shareholders: true,
                quasi_partnership: true,
                exit_offer: None,
            },
        };

        let result = OppressionAnalyzer::analyze(&facts);

        assert!(result.expectations_established);
        assert!(result.oppression_found);
    }

    #[test]
    fn test_creditor_not_automatic_complainant() {
        let facts = OppressionFacts {
            complainant: ComplainantInfo {
                name: "Creditor".to_string(),
                complainant_type: ComplainantType::Creditor,
                shareholding: None,
                relationship_duration_years: Some(3),
            },
            corporation: "Debtor Corp".to_string(),
            conduct: AllegedConduct {
                description: "Asset stripping".to_string(),
                conduct_type: ConductType::Misappropriation,
                actors: vec!["Shareholders".to_string()],
                date: "2024-01-01".to_string(),
                ongoing: false,
            },
            expectations: vec![],
            context: OppressionContext {
                closely_held: true,
                shareholders_agreement: false,
                equal_shareholders: false,
                quasi_partnership: false,
                exit_offer: None,
            },
        };

        let result = OppressionAnalyzer::analyze(&facts);

        // Creditors need court approval for complainant status
        assert!(!result.complainant_status);
    }
}
