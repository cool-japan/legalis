//! Common Law Contract Types
//!
//! Core data structures for English contract law (common law system).
//!
//! # Contract Formation Elements
//!
//! 1. **Offer** - Definite promise to be bound on specific terms
//! 2. **Acceptance** - Unqualified agreement to offer terms (mirror image rule)
//! 3. **Consideration** - Something of value exchanged
//! 4. **Intention to create legal relations** - Intent to be legally bound
//! 5. **Capacity** - Legal ability to contract
//!
//! # Key Case Law
//!
//! - **Carlill v Carbolic Smoke Ball Co [1893]**: Unilateral contracts
//! - **Adams v Lindsell [1818]**: Postal rule
//! - **Hyde v Wrench [1840]**: Counter-offer destroys original offer
//! - **Hadley v Baxendale [1854]**: Remoteness of damages
//! - **Williams v Roffey Bros [1991]**: Practical benefit as consideration

#![allow(missing_docs)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Contract formation under common law
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractFormation {
    /// Offer made
    pub offer: Offer,

    /// Acceptance of offer
    pub acceptance: Option<Acceptance>,

    /// Consideration provided
    pub consideration: Consideration,

    /// Intention to create legal relations
    pub intention: IntentionToCreateLegalRelations,

    /// Contractual capacity
    pub capacity: ContractualCapacity,

    /// Whether contract is formed
    pub is_formed: bool,
}

/// Offer under common law
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Offer {
    /// Offeror (person making offer)
    pub offeror: Party,

    /// Offeree (person to whom offer made)
    pub offeree: Party,

    /// Terms of offer
    pub terms: Vec<String>,

    /// Date offer made
    pub offer_date: DateTime<Utc>,

    /// Expiry date (if specified)
    pub expiry_date: Option<DateTime<Utc>>,

    /// Whether offer is still open
    pub still_open: bool,

    /// Type of offer
    pub offer_type: OfferType,
}

/// Type of offer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfferType {
    /// Bilateral offer (promise for promise)
    Bilateral,

    /// Unilateral offer (promise for act)
    /// See: Carlill v Carbolic Smoke Ball Co [1893]
    Unilateral,

    /// Invitation to treat (not an offer)
    InvitationToTreat,
}

/// Acceptance of offer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Acceptance {
    /// Date of acceptance
    pub acceptance_date: DateTime<Utc>,

    /// Method of acceptance
    pub method: AcceptanceMethod,

    /// Whether acceptance is unqualified
    /// Mirror image rule: acceptance must match offer exactly
    pub unqualified: bool,

    /// Any modifications (creates counter-offer if present)
    pub modifications: Vec<String>,
}

/// Method of acceptance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcceptanceMethod {
    /// Oral acceptance
    Oral,

    /// Written acceptance
    Written,

    /// Acceptance by post
    /// Postal rule applies: Adams v Lindsell [1818]
    /// Acceptance complete when posted, not when received
    Post,

    /// Acceptance by email/electronic
    Email,

    /// Acceptance by conduct
    Conduct,
}

/// Consideration (something of value exchanged)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consideration {
    /// Description of consideration
    pub description: String,

    /// Who provides consideration
    pub provided_by: Party,

    /// Type of consideration
    pub consideration_type: ConsiderationType,

    /// Whether consideration is sufficient
    /// Chappell v Nestlé [1960]: Must be sufficient but need not be adequate
    pub sufficient: bool,

    /// Whether consideration is past
    /// Re McArdle [1951]: Past consideration is not valid
    pub is_past: bool,
}

/// Type of consideration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsiderationType {
    /// Payment of money
    Money,

    /// Promise to do something
    Promise,

    /// Performance of an act
    Act,

    /// Forbearance (promise not to do something)
    Forbearance,

    /// Practical benefit
    /// Williams v Roffey Bros [1991]
    PracticalBenefit,
}

/// Intention to create legal relations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentionToCreateLegalRelations {
    /// Context of agreement
    pub context: AgreementContext,

    /// Presumption (rebuttable)
    pub presumption: IntentionPresumption,

    /// Evidence rebutting presumption
    pub rebuttal_evidence: Vec<String>,

    /// Whether intention exists
    pub intention_exists: bool,
}

/// Context of agreement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgreementContext {
    /// Commercial/business context
    /// Presumption: Intention exists (Esso v Commissioners [1976])
    Commercial,

    /// Domestic/social context
    /// Presumption: No intention (Balfour v Balfour [1919])
    Domestic,

    /// Social context
    Social,
}

/// Presumption of intention
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentionPresumption {
    /// Presumed to intend legal relations (commercial)
    IntentionPresumed,

    /// Presumed not to intend legal relations (domestic/social)
    NoIntentionPresumed,
}

/// Contractual capacity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractualCapacity {
    /// Party whose capacity is assessed
    pub party: Party,

    /// Type of incapacity (if any)
    pub incapacity: Option<IncapacityType>,

    /// Whether party has capacity
    pub has_capacity: bool,
}

/// Type of incapacity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncapacityType {
    /// Minor (under 18)
    /// Minors' Contracts Act 1987
    Minor,

    /// Mental incapacity
    /// Mental Capacity Act 2005
    MentalIncapacity,

    /// Intoxication (drunk/drugs)
    Intoxication,

    /// Company acting ultra vires
    /// Companies Act 2006
    UltraVires,
}

/// Party to contract
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Party {
    /// Name
    pub name: String,

    /// Type of party
    pub party_type: PartyType,

    /// Age (if individual)
    pub age: Option<u8>,
}

/// Type of party
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyType {
    /// Individual person
    Individual,

    /// Company/corporation
    Company,

    /// Partnership
    Partnership,

    /// Other legal entity
    Other,
}

/// Contract term classification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractTerm {
    /// Term text
    pub text: String,

    /// Classification
    pub classification: TermClassification,

    /// Whether term is express or implied
    pub term_source: TermSource,
}

/// Term classification under common law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TermClassification {
    /// Condition: Essential term, breach allows termination
    /// Poussard v Spiers [1876]
    Condition,

    /// Warranty: Minor term, breach allows damages only
    /// Bettini v Gye [1876]
    Warranty,

    /// Innominate term: Depends on consequences of breach
    /// Hong Kong Fir Shipping v Kawasaki [1962]
    InnominateTerm,
}

/// Source of term
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TermSource {
    /// Express term (explicitly stated)
    Express,

    /// Implied by fact (necessary to give business efficacy)
    /// The Moorcock [1889]
    ImpliedInFact,

    /// Implied by law (statutory)
    ImpliedInLaw,

    /// Implied by custom/trade usage
    ImpliedByCustom,
}

/// Contract breach
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractBreach {
    /// Breaching party
    pub breaching_party: Party,

    /// Term breached
    pub term_breached: ContractTerm,

    /// Type of breach
    pub breach_type: BreachType,

    /// Date of breach
    pub breach_date: DateTime<Utc>,

    /// Description
    pub description: String,
}

/// Type of breach
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachType {
    /// Fundamental breach (repudiatory)
    /// Allows innocent party to terminate
    Fundamental,

    /// Minor breach
    /// Allows damages only
    Minor,

    /// Anticipatory breach
    /// Hochster v De La Tour [1853]
    Anticipatory,
}

/// Contract remedy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractRemedy {
    /// Type of remedy
    pub remedy_type: RemedyType,

    /// Amount (if damages)
    pub amount: Option<f64>,

    /// Basis for remedy
    pub basis: String,
}

/// Type of remedy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemedyType {
    /// Damages (monetary compensation)
    Damages(DamagesType),

    /// Specific performance (order to perform contract)
    /// Equitable remedy - discretionary
    SpecificPerformance,

    /// Injunction (order not to do something)
    /// Equitable remedy
    Injunction,

    /// Rescission (set aside contract)
    Rescission,

    /// Rectification (correct written terms)
    Rectification,
}

/// Type of damages
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamagesType {
    /// Expectation damages (put in position as if contract performed)
    /// Robinson v Harman [1848]
    Expectation,

    /// Reliance damages (restore to pre-contract position)
    Reliance,

    /// Restitutionary damages (prevent unjust enrichment)
    Restitutionary,

    /// Liquidated damages (pre-agreed amount)
    Liquidated,

    /// Nominal damages (technical breach, no loss)
    Nominal,
}

/// Remoteness test for damages
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemotenessTest {
    /// Loss claimed
    pub loss_description: String,

    /// Amount of loss
    pub loss_amount: f64,

    /// Whether loss passes remoteness test
    /// Hadley v Baxendale [1854] two limbs:
    /// 1. Arising naturally from breach
    /// 2. Reasonably in contemplation of both parties
    pub passes_test: bool,

    /// Which limb applies
    pub limb: HadleyLimb,
}

/// Hadley v Baxendale [1854] limbs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HadleyLimb {
    /// First limb: Loss arising naturally in ordinary course
    FirstLimb,

    /// Second limb: Loss reasonably in contemplation of parties
    /// (special circumstances communicated)
    SecondLimb,
}

impl Offer {
    /// Check if offer is still open
    pub fn is_open(&self) -> bool {
        if !self.still_open {
            return false;
        }

        if let Some(expiry) = self.expiry_date {
            if Utc::now() > expiry {
                return false;
            }
        }

        true
    }

    /// Revoke offer (before acceptance)
    pub fn revoke(&mut self) {
        self.still_open = false;
    }
}

impl Acceptance {
    /// Check if acceptance is valid (mirror image rule)
    pub fn is_valid_acceptance(&self) -> bool {
        self.unqualified && self.modifications.is_empty()
    }

    /// Check if acceptance creates counter-offer
    /// Hyde v Wrench [1840]
    pub fn is_counter_offer(&self) -> bool {
        !self.modifications.is_empty()
    }
}

impl Consideration {
    /// Check if consideration is valid
    pub fn is_valid(&self) -> bool {
        // Must be sufficient (not necessarily adequate)
        // Must not be past
        self.sufficient && !self.is_past
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offer_is_open() {
        let offer = Offer {
            offeror: Party {
                name: "Alice".to_string(),
                party_type: PartyType::Individual,
                age: Some(30),
            },
            offeree: Party {
                name: "Bob".to_string(),
                party_type: PartyType::Individual,
                age: Some(25),
            },
            terms: vec!["Sell car for £5000".to_string()],
            offer_date: Utc::now(),
            expiry_date: None,
            still_open: true,
            offer_type: OfferType::Bilateral,
        };

        assert!(offer.is_open());
    }

    #[test]
    fn test_acceptance_mirror_image_rule() {
        let acceptance = Acceptance {
            acceptance_date: Utc::now(),
            method: AcceptanceMethod::Written,
            unqualified: true,
            modifications: vec![],
        };

        assert!(acceptance.is_valid_acceptance());
        assert!(!acceptance.is_counter_offer());
    }

    #[test]
    fn test_acceptance_counter_offer() {
        let acceptance = Acceptance {
            acceptance_date: Utc::now(),
            method: AcceptanceMethod::Written,
            unqualified: false,
            modifications: vec!["I accept but will pay £4500".to_string()],
        };

        assert!(!acceptance.is_valid_acceptance());
        assert!(acceptance.is_counter_offer());
    }

    #[test]
    fn test_consideration_validity() {
        let valid = Consideration {
            description: "Payment of £100".to_string(),
            provided_by: Party {
                name: "Buyer".to_string(),
                party_type: PartyType::Individual,
                age: Some(30),
            },
            consideration_type: ConsiderationType::Money,
            sufficient: true,
            is_past: false,
        };

        assert!(valid.is_valid());

        let past = Consideration {
            description: "Work already completed".to_string(),
            provided_by: Party {
                name: "Worker".to_string(),
                party_type: PartyType::Individual,
                age: Some(30),
            },
            consideration_type: ConsiderationType::Act,
            sufficient: true,
            is_past: true,
        };

        assert!(!past.is_valid());
    }
}
