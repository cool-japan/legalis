//! Canada Contract Law - Types
//!
//! Core types for Canadian contract law (common law provinces + Quebec civil law).

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::common::{CaseCitation, Court};

// ============================================================================
// Contract Formation
// ============================================================================

/// Elements of contract formation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormationElement {
    /// Offer
    Offer,
    /// Acceptance
    Acceptance,
    /// Consideration (common law only)
    Consideration,
    /// Intention to create legal relations
    Intention,
    /// Capacity to contract
    Capacity,
    /// Legality of purpose
    Legality,
    /// Certainty of terms
    Certainty,
    /// Consent (Quebec civil law - no consideration required)
    Consent,
}

/// Offer in contract formation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offer {
    /// Description of the offer
    pub description: String,
    /// Offeror (party making offer)
    pub offeror: String,
    /// Offeree (party receiving offer)
    pub offeree: String,
    /// Terms of the offer
    pub terms: Vec<String>,
    /// Whether offer is definite and certain
    pub is_definite: bool,
    /// Communication method
    pub communication: CommunicationMethod,
    /// Whether offer includes time limit
    pub time_limit: Option<String>,
    /// Whether offer has lapsed
    pub lapsed: bool,
    /// Whether offer has been revoked
    pub revoked: bool,
}

/// Method of communication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommunicationMethod {
    /// In person
    InPerson,
    /// Written (mail, courier)
    Written,
    /// Email
    Email,
    /// Telephone
    Telephone,
    /// Fax
    Fax,
    /// Website/online
    Online,
    /// Conduct
    Conduct,
}

/// Acceptance of offer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Acceptance {
    /// Description of acceptance
    pub description: String,
    /// Whether acceptance mirrors offer (no counter-offer)
    pub mirrors_offer: bool,
    /// Method of acceptance
    pub method: CommunicationMethod,
    /// Whether acceptance was communicated
    pub communicated: bool,
    /// Whether acceptance complied with prescribed method
    pub prescribed_method_followed: bool,
}

/// Consideration in common law contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consideration {
    /// What the promisor receives
    pub benefit_to_promisor: String,
    /// What the promisee gives
    pub detriment_to_promisee: String,
    /// Whether consideration is sufficient (need not be adequate)
    pub is_sufficient: bool,
    /// Whether consideration is past
    pub is_past: bool,
    /// Whether consideration moves from promisee
    pub moves_from_promisee: bool,
}

// ============================================================================
// Contract Terms
// ============================================================================

/// Classification of contract term
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TermClassification {
    /// Condition (essential term - breach entitles termination)
    Condition,
    /// Warranty (minor term - damages only)
    Warranty,
    /// Innominate term (depends on consequences of breach)
    Innominate,
}

/// Type of term incorporation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TermType {
    /// Express term
    Express,
    /// Implied by statute
    ImpliedByStatute { statute: String },
    /// Implied by custom or trade usage
    ImpliedByCustom,
    /// Implied by fact (business efficacy test)
    ImpliedByFact,
    /// Implied by law
    ImpliedByLaw,
}

/// Contract term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTerm {
    /// Description of the term
    pub description: String,
    /// Classification
    pub classification: TermClassification,
    /// How term was incorporated
    pub term_type: TermType,
    /// Whether term is an exclusion/limitation clause
    pub is_exclusion_clause: bool,
}

/// Exclusion clause analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExclusionClause {
    /// The clause text
    pub clause: String,
    /// Whether properly incorporated
    pub incorporated: bool,
    /// Whether clause covers the breach
    pub covers_breach: bool,
    /// Whether void for unconscionability
    pub unconscionable: bool,
    /// Applicable consumer protection legislation
    pub consumer_protection: Option<String>,
}

// ============================================================================
// Breach and Remedies
// ============================================================================

/// Type of breach
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachType {
    /// Breach of condition
    BreachOfCondition,
    /// Breach of warranty
    BreachOfWarranty,
    /// Breach of innominate term (serious consequences)
    SeriousBreachOfInnominate,
    /// Breach of innominate term (minor consequences)
    MinorBreachOfInnominate,
    /// Anticipatory breach
    AnticipatoryBreach,
    /// Repudiation
    Repudiation,
    /// Fundamental breach
    FundamentalBreach,
}

/// Remedy for breach of contract
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractRemedy {
    /// Expectation damages (put in position if contract performed)
    ExpectationDamages,
    /// Reliance damages (put in position before contract)
    RelianceDamages,
    /// Restitution (return of benefits conferred)
    Restitution,
    /// Specific performance
    SpecificPerformance,
    /// Injunction
    Injunction,
    /// Rescission
    Rescission,
    /// Rectification
    Rectification,
    /// Account of profits
    AccountOfProfits,
    /// Nominal damages
    NominalDamages,
    /// Punitive damages (exceptional cases)
    PunitiveDamages,
}

/// Damages calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamagesCalculation {
    /// Type of damages
    pub damages_type: ContractRemedy,
    /// Quantum (amount in cents)
    pub quantum_cents: Option<i64>,
    /// Whether damages are remote (Hadley v Baxendale)
    pub remoteness_satisfied: bool,
    /// Whether damages are mitigated
    pub mitigation_satisfied: bool,
    /// Explanation of calculation
    pub calculation: String,
}

// ============================================================================
// Vitiating Factors
// ============================================================================

/// Vitiating factor (renders contract void or voidable)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VitiatingFactor {
    /// Mistake (common, mutual, or unilateral)
    Mistake(MistakeType),
    /// Misrepresentation
    Misrepresentation(MisrepresentationType),
    /// Duress
    Duress(DuressType),
    /// Undue influence
    UndueInfluence,
    /// Unconscionability
    Unconscionability,
    /// Illegality
    Illegality,
    /// Incapacity
    Incapacity,
}

/// Types of mistake
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MistakeType {
    /// Common mistake (both parties same mistake)
    CommonMistake,
    /// Mutual mistake (parties at cross-purposes)
    MutualMistake,
    /// Unilateral mistake (one party mistaken)
    UnilateralMistake,
    /// Non est factum (document not what signed)
    NonEstFactum,
}

/// Types of misrepresentation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MisrepresentationType {
    /// Fraudulent misrepresentation
    Fraudulent,
    /// Negligent misrepresentation
    Negligent,
    /// Innocent misrepresentation
    Innocent,
}

/// Types of duress
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DuressType {
    /// Duress to person
    ToPerson,
    /// Duress to goods
    ToGoods,
    /// Economic duress
    Economic,
}

// ============================================================================
// Quebec Civil Law
// ============================================================================

/// Quebec Civil Code contract concepts (Book Five: Obligations)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CcqConcept {
    /// Consent (art. 1386)
    Consent,
    /// Capacity (art. 1398)
    Capacity,
    /// Cause (object/purpose) (art. 1410)
    Cause,
    /// Object of contract (art. 1412)
    Object,
    /// Error (art. 1400)
    Error,
    /// Fear (crainte) (art. 1402)
    Fear,
    /// Lesion (art. 1405)
    Lesion,
    /// Warranty of quality (art. 1726)
    WarrantyOfQuality,
    /// Good faith (art. 1375)
    GoodFaith,
}

/// Quebec-specific contract type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CcqContractType {
    /// Contract of sale (art. 1708)
    Sale,
    /// Contract of lease (art. 1851)
    Lease,
    /// Contract of enterprise (art. 2098)
    Enterprise,
    /// Contract of mandate (art. 2130)
    Mandate,
    /// Contract of employment (art. 2085)
    Employment,
    /// Contract of insurance (art. 2389)
    Insurance,
    /// Suretyship (art. 2333)
    Suretyship,
    /// Hypothec (art. 2660)
    Hypothec,
    /// Nominate contract (named in CCQ)
    Nominate { name: String },
    /// Innominate contract
    Innominate,
}

// ============================================================================
// Key Cases
// ============================================================================

/// Key Canadian contract law case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractCase {
    /// Citation
    pub citation: CaseCitation,
    /// Legal principle
    pub principle: String,
    /// Area of contract law
    pub area: ContractArea,
}

/// Area of contract law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractArea {
    /// Formation
    Formation,
    /// Terms
    Terms,
    /// Breach
    Breach,
    /// Remedies
    Remedies,
    /// Vitiating factors
    VitiatingFactors,
    /// Privity
    Privity,
}

impl ContractCase {
    /// Hunter Engineering v Syncrude \[1989\] - exclusion clauses
    pub fn hunter_engineering() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Hunter Engineering Co v Syncrude Canada Ltd",
                1989,
                21,
                "Fundamental breach doctrine rejected; exclusion clauses enforced \
                 unless unconscionable",
            ),
            principle: "Exclusion clauses should be enforced according to their terms \
                unless unconscionable; fundamental breach is not a rule of law"
                .to_string(),
            area: ContractArea::Terms,
        }
    }

    /// Tercon Contractors v BC \[2010\] - exclusion clauses
    pub fn tercon() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Tercon Contractors Ltd v British Columbia (Transportation)",
                2010,
                4,
                "Three-step framework for exclusion clauses",
            ),
            principle: "1) Does clause apply to circumstances? 2) Was clause unconscionable? \
                3) Should court refuse enforcement as matter of public policy?"
                .to_string(),
            area: ContractArea::Terms,
        }
    }

    /// Bhasin v Hrynew \[2014\] - good faith
    pub fn bhasin() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Bhasin v Hrynew",
                2014,
                71,
                "Recognized duty of honest contractual performance",
            ),
            principle: "Organizing principle of good faith in contract law; \
                parties must not lie or mislead about matters related to contract"
                .to_string(),
            area: ContractArea::Formation,
        }
    }

    /// C.M. Callow Inc v Zollinger \[2020\] - good faith
    pub fn callow() -> Self {
        Self {
            citation: CaseCitation::scc(
                "C.M. Callow Inc v Zollinger",
                2020,
                45,
                "Expanded duty of honest performance to active deception",
            ),
            principle: "Duty of honest performance breached by active deception \
                (knowingly misleading), not just direct lies"
                .to_string(),
            area: ContractArea::Formation,
        }
    }

    /// Hadley v Baxendale - remoteness (English case applied in Canada)
    pub fn hadley_v_baxendale() -> Self {
        Self {
            citation: CaseCitation {
                name: "Hadley v Baxendale".to_string(),
                year: 1854,
                neutral_citation: None,
                report_citation: Some("(1854) 9 Ex 341".to_string()),
                court: Court::Tribunal {
                    name: "English Court of Exchequer".to_string(),
                },
                principle: "Remoteness test for contract damages".to_string(),
            },
            principle: "Damages recoverable if: 1) arising naturally from breach, or \
                2) in contemplation of parties at time of contract"
                .to_string(),
            area: ContractArea::Remedies,
        }
    }

    /// Fidler v Sun Life \[2006\] - mental distress damages
    pub fn fidler() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Fidler v Sun Life Assurance Co of Canada",
                2006,
                30,
                "Mental distress damages available in contract",
            ),
            principle: "Mental distress damages available where object of contract \
                was to secure psychological benefit (peace of mind contract)"
                .to_string(),
            area: ContractArea::Remedies,
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
    fn test_formation_elements() {
        let offer = FormationElement::Offer;
        let consideration = FormationElement::Consideration;
        assert_ne!(offer, consideration);
    }

    #[test]
    fn test_term_classification() {
        let condition = TermClassification::Condition;
        let warranty = TermClassification::Warranty;
        assert_ne!(condition, warranty);
    }

    #[test]
    fn test_breach_type() {
        let anticipatory = BreachType::AnticipatoryBreach;
        assert_eq!(anticipatory, BreachType::AnticipatoryBreach);
    }

    #[test]
    fn test_contract_remedy() {
        let specific = ContractRemedy::SpecificPerformance;
        let damages = ContractRemedy::ExpectationDamages;
        assert_ne!(specific, damages);
    }

    #[test]
    fn test_key_cases() {
        let bhasin = ContractCase::bhasin();
        assert_eq!(bhasin.citation.year, 2014);
        assert_eq!(bhasin.area, ContractArea::Formation);
    }

    #[test]
    fn test_quebec_concepts() {
        let consent = CcqConcept::Consent;
        let good_faith = CcqConcept::GoodFaith;
        assert_ne!(consent, good_faith);
    }
}
