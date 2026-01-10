//! BGB Contract Law Types (Schuldrecht - Allgemeiner Teil)
//!
//! Type-safe representations of German contract law under the BGB
//! (Bürgerliches Gesetzbuch).
//!
//! # Legal Context
//!
//! The Schuldrecht (law of obligations) is Book 2 of the BGB, divided into:
//! - **General Part** (Allgemeiner Teil, §241-432): Applies to all obligations
//! - **Special Part** (Besonderer Teil, §433-853): Specific contract types
//!
//! This module focuses on the general principles applicable to all contracts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::gmbhg::Capital;

/// Contract formation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractStatus {
    /// Contract offer made but not yet accepted
    OfferPending,
    /// Contract validly formed
    Concluded,
    /// Contract void (nichtig) per §§105-144 BGB
    Void,
    /// Contract voidable (anfechtbar) per §§119-124 BGB
    Voidable,
    /// Contract terminated (beendet)
    Terminated,
    /// Contract rescinded (rückgängig gemacht) per §§346-354 BGB
    Rescinded,
}

/// Legal capacity status (Geschäftsfähigkeit)
///
/// Per §§104-115 BGB, determines ability to enter binding contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalCapacity {
    /// Full capacity (voll geschäftsfähig) - age 18+ with no restrictions
    Full,
    /// Limited capacity (beschränkt geschäftsfähig) - age 7-17 (§106 BGB)
    /// Requires legal representative consent for binding acts
    Limited,
    /// No capacity (geschäftsunfähig) - under age 7 or permanently incapacitated (§104 BGB)
    None,
}

/// Declaration of intent (Willenserklärung)
///
/// Fundamental building block of contracts per §§116-144 BGB.
/// A manifestation of will directed at producing legal effects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Declaration {
    /// Party making the declaration
    pub declarant: Party,
    /// Content of the declaration
    pub content: String,
    /// When the declaration was made
    pub declared_at: DateTime<Utc>,
    /// Whether declaration was received by intended recipient
    pub received: bool,
    /// When declaration was received (if applicable)
    pub received_at: Option<DateTime<Utc>>,
    /// Whether under mental reservation (Mentalreservation §116 BGB)
    pub mental_reservation: bool,
    /// Whether made under duress (Drohung §123 BGB)
    pub under_duress: bool,
    /// Whether made under mistake (Irrtum §§119-122 BGB)
    pub mistake_type: Option<MistakeType>,
}

/// Types of mistake (Irrtum) enabling voidability per §§119-122 BGB
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MistakeType {
    /// Mistake in declaration (Erklärungsirrtum §119 Abs. 1 Alt. 1 BGB)
    /// Error in expressing the declaration
    InDeclaration,
    /// Mistake in transmission (Übermittlungsirrtum §120 BGB)
    /// Error by messenger or transmission method
    InTransmission,
    /// Mistake about content (Inhaltsirrtum §119 Abs. 1 Alt. 2 BGB)
    /// Error about meaning of declaration
    AboutContent,
    /// Mistake about essential qualities (Eigenschaftsirrtum §119 Abs. 2 BGB)
    /// Error about characteristics of person or thing
    AboutEssentialQuality,
    /// Mistake about person (Irrtum über die Person)
    /// Specific case of essential quality mistake
    AboutPerson,
}

/// Contract offer (Angebot) per §145 BGB
///
/// A declaration of intent that is sufficiently specific and shows
/// intention to be bound upon acceptance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Offer {
    /// Offeror (Anbieter)
    pub offeror: Party,
    /// Offeree (Empfänger)
    pub offeree: Party,
    /// Terms of the offer
    pub terms: ContractTerms,
    /// When offer was made
    pub offered_at: DateTime<Utc>,
    /// Deadline for acceptance (if any)
    pub acceptance_deadline: Option<DateTime<Utc>>,
    /// Whether offer is binding (generally yes per §145 BGB)
    /// Non-binding = invitation to treat (invitatio ad offerendum)
    pub binding: bool,
    /// Whether offer has been revoked (Widerruf §130 Abs. 1 S. 2 BGB)
    pub revoked: bool,
}

/// Contract acceptance (Annahme) per §147 BGB
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Acceptance {
    /// Party accepting the offer
    pub acceptor: Party,
    /// When acceptance was declared
    pub accepted_at: DateTime<Utc>,
    /// Whether acceptance modifies offer terms (§150 BGB - rejection + counter-offer)
    pub modifications: Option<Vec<String>>,
    /// Whether acceptance was timely per §147-149 BGB
    pub timely: bool,
}

/// Party to a contract
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Party {
    /// Name of the party
    pub name: String,
    /// Address
    pub address: String,
    /// Legal capacity status
    pub legal_capacity: LegalCapacity,
    /// Legal representative (if limited/no capacity)
    pub legal_representative: Option<String>,
    /// Whether party is a natural person or legal entity
    pub party_type: PartyType,
}

/// Party type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyType {
    /// Natural person (natürliche Person)
    NaturalPerson,
    /// Legal entity (juristische Person)
    LegalEntity,
}

/// Contract terms (Vertragsbedingungen)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractTerms {
    /// Subject matter of the contract (Vertragsgegenstand)
    pub subject_matter: String,
    /// Price or consideration (Gegenleistung)
    pub consideration: Option<Capital>,
    /// Essential terms (essentialia negotii)
    pub essential_terms: Vec<String>,
    /// Additional terms
    pub additional_terms: Vec<String>,
    /// Whether terms include general terms and conditions (AGB)
    pub includes_gtc: bool,
}

/// Formed contract (concluded agreement)
///
/// Requires valid offer + acceptance per §§145-157 BGB.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contract {
    /// Contract ID
    pub contract_id: String,
    /// Parties to the contract
    pub parties: Vec<Party>,
    /// Contract terms
    pub terms: ContractTerms,
    /// When contract was concluded
    pub concluded_at: DateTime<Utc>,
    /// Contract status
    pub status: ContractStatus,
    /// Contract type
    pub contract_type: ContractType,
    /// Performance obligations (Leistungspflichten)
    pub obligations: Vec<Obligation>,
    /// Whether contract is in writing (Schriftform §126 BGB)
    pub in_writing: bool,
}

/// Types of contracts (Vertragstypen)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    /// Sales contract (Kaufvertrag §§433-479 BGB)
    Sale,
    /// Lease/rental (Mietvertrag §§535-580a BGB)
    Lease,
    /// Service contract (Dienstvertrag §§611-630 BGB)
    Service,
    /// Work contract (Werkvertrag §§631-651 BGB)
    Work,
    /// Loan contract (Darlehensvertrag §§488-505 BGB)
    Loan,
    /// Gift (Schenkung §§516-534 BGB)
    Gift,
    /// General/unspecified contract
    General,
}

/// Performance obligation (Leistungspflicht)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Obligation {
    /// Obligor (party who must perform)
    pub obligor: String,
    /// Obligee (party entitled to performance)
    pub obligee: String,
    /// Description of obligation
    pub description: String,
    /// Due date
    pub due_date: Option<DateTime<Utc>>,
    /// Whether obligation has been performed
    pub performed: bool,
    /// Performance date (if performed)
    pub performed_at: Option<DateTime<Utc>>,
}

/// Breach of contract (Pflichtverletzung §280 BGB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Breach {
    /// Contract that was breached
    pub contract_id: String,
    /// Breaching party
    pub breaching_party: String,
    /// Type of breach
    pub breach_type: BreachType,
    /// When breach occurred
    pub occurred_at: DateTime<Utc>,
    /// Whether breaching party was at fault (Verschulden §276 BGB)
    pub fault: FaultLevel,
    /// Description of the breach
    pub description: String,
}

/// Types of breach (Pflichtverletzung)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachType {
    /// Non-performance (Nichterfüllung)
    NonPerformance,
    /// Delayed performance (Verzug §§286-292 BGB)
    Delay,
    /// Defective performance (Schlechterfüllung)
    DefectivePerformance,
    /// Impossibility of performance (Unmöglichkeit §§275-276 BGB)
    Impossibility,
    /// Breach of ancillary duty (Verletzung einer Nebenpflicht)
    AncillaryDutyBreach,
    /// Culpa in contrahendo (precontractual fault §311 Abs. 2 BGB)
    CulpaInContrahendo,
}

/// Level of fault (Verschulden) per §276 BGB
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FaultLevel {
    /// Intent (Vorsatz)
    Intent,
    /// Gross negligence (grobe Fahrlässigkeit)
    GrossNegligence,
    /// Ordinary negligence (einfache Fahrlässigkeit)
    OrdinaryNegligence,
    /// Slight negligence (leichte Fahrlässigkeit)
    SlightNegligence,
    /// No fault (kein Verschulden)
    NoFault,
}

/// Remedy for breach (Rechtsbehelfe)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Remedy {
    /// Contract ID
    pub contract_id: String,
    /// Party seeking remedy
    pub claimant: String,
    /// Party against whom remedy is sought
    pub respondent: String,
    /// Type of remedy
    pub remedy_type: RemedyType,
    /// Amount of damages (if applicable)
    pub damages_amount: Option<Capital>,
    /// Grace period set (Nachfrist §281 BGB) if applicable
    pub grace_period_days: Option<u32>,
    /// Whether grace period has expired
    pub grace_period_expired: bool,
}

/// Types of remedies (Rechtsbehelfe)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemedyType {
    /// Right to performance (Erfüllungsanspruch §241 Abs. 1 BGB)
    Performance,
    /// Supplementary performance (Nacherfüllung §§437 Nr. 1, 439 BGB)
    SupplementaryPerformance,
    /// Damages (Schadensersatz §§280-283 BGB)
    Damages,
    /// Damages in lieu of performance (Schadensersatz statt der Leistung §281 BGB)
    DamagesInLieu,
    /// Reimbursement of expenses (Aufwendungsersatz §284 BGB)
    ExpenseReimbursement,
    /// Reduction of price (Minderung §§437 Nr. 2, 441 BGB)
    PriceReduction,
    /// Termination (Rücktritt §§323-326 BGB)
    Termination,
    /// Withdrawal (Widerruf - consumer right §355 BGB)
    ConsumerWithdrawal,
}

/// Termination (Rücktritt) per §§323-326 BGB
///
/// Right to rescind contract for non-performance or defective performance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Termination {
    /// Contract being terminated
    pub contract_id: String,
    /// Terminating party
    pub terminating_party: String,
    /// Grounds for termination
    pub grounds: TerminationGrounds,
    /// Whether grace period was set and expired (§323 Abs. 1 BGB)
    pub grace_period_set_and_expired: bool,
    /// When termination was declared
    pub declared_at: DateTime<Utc>,
    /// Whether termination is effective
    pub effective: bool,
}

/// Grounds for termination (Rücktrittsgrund)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminationGrounds {
    /// Non-performance after grace period (§323 Abs. 1 BGB)
    NonPerformanceAfterGracePeriod,
    /// Refusal to perform (§323 Abs. 2 Nr. 1 BGB)
    RefusalToPerform,
    /// Performance impossible (§323 Abs. 4 BGB)
    PerformanceImpossible,
    /// Minor breach (unerhebliche Pflichtverletzung) - termination excluded (§323 Abs. 5 S. 2 BGB)
    MinorBreach,
    /// Serious breach justifying immediate termination (§323 Abs. 2 Nr. 3 BGB)
    SeriousBreach,
}

/// Damages claim (Schadensersatzanspruch) per §§280-283 BGB
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DamagesClaim {
    /// Contract ID (if contract-based)
    pub contract_id: Option<String>,
    /// Claimant
    pub claimant: String,
    /// Respondent
    pub respondent: String,
    /// Legal basis for damages
    pub legal_basis: DamagesLegalBasis,
    /// Types of damages claimed
    pub damage_types: Vec<DamageType>,
    /// Total amount claimed
    pub amount_claimed: Capital,
    /// Whether fault (Verschulden) is proven
    pub fault_proven: bool,
    /// Whether causation is proven
    pub causation_proven: bool,
}

/// Legal basis for damages claim
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamagesLegalBasis {
    /// General damages for breach (§280 Abs. 1 BGB)
    GeneralBreach,
    /// Damages for delay (§280 Abs. 2, §286 BGB)
    Delay,
    /// Damages in lieu of performance (§281 BGB)
    InLieuOfPerformance,
    /// Damages after impossibility (§283 BGB)
    AfterImpossibility,
    /// Damages for breach of duty in performance (§282 BGB)
    BreachInPerformance,
    /// Culpa in contrahendo (§311 Abs. 2 BGB)
    CulpaInContrahendo,
}

/// Types of damages (Schadensarten)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    /// Positive damage (positives Interesse) - loss from breach
    Positive,
    /// Negative damage (negatives Interesse) - reliance loss
    Negative,
    /// Consequential damages (Folgeschäden)
    Consequential,
    /// Loss of profit (entgangener Gewinn §252 BGB)
    LostProfit,
}

impl Declaration {
    /// Check if declaration is valid (not void or voidable)
    pub fn is_valid(&self) -> bool {
        !self.mental_reservation && !self.under_duress && self.mistake_type.is_none()
    }

    /// Check if declaration is effective (zugegangen per §130 BGB)
    pub fn is_effective(&self) -> bool {
        self.received
    }
}

impl Offer {
    /// Check if offer is still valid and can be accepted
    pub fn is_valid(&self) -> bool {
        if self.revoked {
            return false;
        }

        if let Some(deadline) = self.acceptance_deadline {
            Utc::now() <= deadline
        } else {
            true
        }
    }

    /// Check if offer is binding (per §145 BGB, generally yes)
    pub fn is_binding(&self) -> bool {
        self.binding
    }
}

impl Contract {
    /// Check if contract is currently valid and enforceable
    pub fn is_enforceable(&self) -> bool {
        matches!(self.status, ContractStatus::Concluded)
    }

    /// Check if all obligations are performed
    pub fn is_fully_performed(&self) -> bool {
        self.obligations.iter().all(|o| o.performed)
    }

    /// Get unperformed obligations
    pub fn unperformed_obligations(&self) -> Vec<&Obligation> {
        self.obligations.iter().filter(|o| !o.performed).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_declaration_validity() {
        let valid_declaration = Declaration {
            declarant: Party {
                name: "Max Mustermann".to_string(),
                address: "Berlin".to_string(),
                legal_capacity: LegalCapacity::Full,
                legal_representative: None,
                party_type: PartyType::NaturalPerson,
            },
            content: "I offer to sell my car".to_string(),
            declared_at: Utc::now(),
            received: true,
            received_at: Some(Utc::now()),
            mental_reservation: false,
            under_duress: false,
            mistake_type: None,
        };

        assert!(valid_declaration.is_valid());
        assert!(valid_declaration.is_effective());
    }

    #[test]
    fn test_declaration_with_mistake_invalid() {
        let mut declaration = Declaration {
            declarant: Party {
                name: "Test".to_string(),
                address: "Test".to_string(),
                legal_capacity: LegalCapacity::Full,
                legal_representative: None,
                party_type: PartyType::NaturalPerson,
            },
            content: "Test".to_string(),
            declared_at: Utc::now(),
            received: true,
            received_at: Some(Utc::now()),
            mental_reservation: false,
            under_duress: false,
            mistake_type: Some(MistakeType::InDeclaration),
        };

        assert!(!declaration.is_valid());

        declaration.mistake_type = None;
        declaration.under_duress = true;
        assert!(!declaration.is_valid());
    }

    #[test]
    fn test_offer_validity() {
        let future_deadline = Utc::now() + chrono::Duration::days(7);

        let offer = Offer {
            offeror: Party {
                name: "Seller".to_string(),
                address: "Munich".to_string(),
                legal_capacity: LegalCapacity::Full,
                legal_representative: None,
                party_type: PartyType::NaturalPerson,
            },
            offeree: Party {
                name: "Buyer".to_string(),
                address: "Hamburg".to_string(),
                legal_capacity: LegalCapacity::Full,
                legal_representative: None,
                party_type: PartyType::NaturalPerson,
            },
            terms: ContractTerms {
                subject_matter: "Car".to_string(),
                consideration: Some(Capital::from_euros(10_000)),
                essential_terms: vec!["Price: €10,000".to_string()],
                additional_terms: vec![],
                includes_gtc: false,
            },
            offered_at: Utc::now(),
            acceptance_deadline: Some(future_deadline),
            binding: true,
            revoked: false,
        };

        assert!(offer.is_valid());
        assert!(offer.is_binding());
    }

    #[test]
    fn test_offer_revoked_invalid() {
        let mut offer = Offer {
            offeror: Party {
                name: "Test".to_string(),
                address: "Test".to_string(),
                legal_capacity: LegalCapacity::Full,
                legal_representative: None,
                party_type: PartyType::NaturalPerson,
            },
            offeree: Party {
                name: "Test2".to_string(),
                address: "Test2".to_string(),
                legal_capacity: LegalCapacity::Full,
                legal_representative: None,
                party_type: PartyType::NaturalPerson,
            },
            terms: ContractTerms {
                subject_matter: "Test".to_string(),
                consideration: None,
                essential_terms: vec![],
                additional_terms: vec![],
                includes_gtc: false,
            },
            offered_at: Utc::now(),
            acceptance_deadline: None,
            binding: true,
            revoked: false,
        };

        assert!(offer.is_valid());

        offer.revoked = true;
        assert!(!offer.is_valid());
    }

    #[test]
    fn test_contract_enforceability() {
        let contract = Contract {
            contract_id: "C001".to_string(),
            parties: vec![],
            terms: ContractTerms {
                subject_matter: "Services".to_string(),
                consideration: Some(Capital::from_euros(5_000)),
                essential_terms: vec!["Service provision".to_string()],
                additional_terms: vec![],
                includes_gtc: false,
            },
            concluded_at: Utc::now(),
            status: ContractStatus::Concluded,
            contract_type: ContractType::Service,
            obligations: vec![Obligation {
                obligor: "Party A".to_string(),
                obligee: "Party B".to_string(),
                description: "Provide services".to_string(),
                due_date: Some(Utc::now() + chrono::Duration::days(30)),
                performed: false,
                performed_at: None,
            }],
            in_writing: true,
        };

        assert!(contract.is_enforceable());
        assert!(!contract.is_fully_performed());
        assert_eq!(contract.unperformed_obligations().len(), 1);
    }

    #[test]
    fn test_legal_capacity_types() {
        let full_capacity = LegalCapacity::Full;
        let limited_capacity = LegalCapacity::Limited;
        let no_capacity = LegalCapacity::None;

        assert_ne!(full_capacity, limited_capacity);
        assert_ne!(limited_capacity, no_capacity);
        assert_ne!(full_capacity, no_capacity);
    }

    #[test]
    fn test_breach_types() {
        let non_performance = BreachType::NonPerformance;
        let delay = BreachType::Delay;
        let impossibility = BreachType::Impossibility;

        assert_ne!(non_performance, delay);
        assert_ne!(delay, impossibility);
    }

    #[test]
    fn test_remedy_types() {
        let performance = RemedyType::Performance;
        let damages = RemedyType::Damages;
        let termination = RemedyType::Termination;

        assert_ne!(performance, damages);
        assert_ne!(damages, termination);
    }
}
