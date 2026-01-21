//! Indian Contract Act 1872 Types
//!
//! Types for contract law under the Indian Contract Act, 1872

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Contract status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractStatus {
    /// Valid contract (Section 2(h))
    Valid,
    /// Void contract (Section 2(g))
    Void,
    /// Voidable contract (Section 2(i))
    Voidable,
    /// Illegal contract
    Illegal,
    /// Unenforceable contract
    Unenforceable,
    /// Executed contract
    Executed,
    /// Executory contract
    Executory,
    /// Discharged contract
    Discharged,
    /// Breached contract
    Breached,
}

impl ContractStatus {
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Valid => "Legally binding agreement",
            Self::Void => "Agreement without legal effect from beginning",
            Self::Voidable => "Valid but may be set aside at option of aggrieved party",
            Self::Illegal => "Agreement prohibited by law",
            Self::Unenforceable => "Valid but cannot be enforced in court",
            Self::Executed => "Fully performed by both parties",
            Self::Executory => "Obligations yet to be performed",
            Self::Discharged => "Obligations terminated",
            Self::Breached => "Terms violated by one or more parties",
        }
    }
}

/// Contract type by formation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    /// Express contract (explicit terms)
    Express,
    /// Implied contract (conduct)
    Implied,
    /// Quasi-contract (Sections 68-72)
    QuasiContract,
    /// Unilateral contract
    Unilateral,
    /// Bilateral contract
    Bilateral,
    /// Contingent contract (Section 31)
    Contingent,
    /// Wagering agreement (Section 30)
    Wagering,
}

impl ContractType {
    /// Get section reference
    pub fn section(&self) -> Option<u32> {
        match self {
            Self::QuasiContract => Some(68),
            Self::Contingent => Some(31),
            Self::Wagering => Some(30),
            _ => None,
        }
    }
}

/// Essential elements of a valid contract (Section 10)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractEssentials {
    /// Free consent obtained (Section 13-14)
    pub free_consent: bool,
    /// Parties competent to contract (Section 11)
    pub competent_parties: bool,
    /// Lawful consideration (Section 23)
    pub lawful_consideration: bool,
    /// Lawful object (Section 23)
    pub lawful_object: bool,
    /// Agreement not expressly declared void (Sections 24-30)
    pub not_void: bool,
}

impl ContractEssentials {
    /// Check if all essentials are met
    pub fn is_valid(&self) -> bool {
        self.free_consent
            && self.competent_parties
            && self.lawful_consideration
            && self.lawful_object
            && self.not_void
    }

    /// Get missing essentials
    pub fn missing_essentials(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if !self.free_consent {
            missing.push("Free consent (Sections 13-14)");
        }
        if !self.competent_parties {
            missing.push("Competent parties (Section 11)");
        }
        if !self.lawful_consideration {
            missing.push("Lawful consideration (Section 23)");
        }
        if !self.lawful_object {
            missing.push("Lawful object (Section 23)");
        }
        if !self.not_void {
            missing.push("Agreement not expressly declared void (Sections 24-30)");
        }
        missing
    }
}

/// Factors vitiating free consent (Section 14)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsentVitiator {
    /// Coercion (Section 15)
    Coercion,
    /// Undue influence (Section 16)
    UndueInfluence,
    /// Fraud (Section 17)
    Fraud,
    /// Misrepresentation (Section 18)
    Misrepresentation,
    /// Mistake of fact (Section 20)
    MistakeOfFact,
    /// Mistake of law (Section 21)
    MistakeOfLaw,
}

impl ConsentVitiator {
    /// Get section reference
    pub fn section(&self) -> u32 {
        match self {
            Self::Coercion => 15,
            Self::UndueInfluence => 16,
            Self::Fraud => 17,
            Self::Misrepresentation => 18,
            Self::MistakeOfFact => 20,
            Self::MistakeOfLaw => 21,
        }
    }

    /// Get the effect on contract
    pub fn contract_effect(&self) -> ContractStatus {
        match self {
            Self::Coercion | Self::UndueInfluence | Self::Fraud | Self::Misrepresentation => {
                ContractStatus::Voidable
            }
            Self::MistakeOfFact => ContractStatus::Void, // Bilateral mistake
            Self::MistakeOfLaw => ContractStatus::Valid, // Generally valid (ignorance of law no excuse)
        }
    }

    /// Description of the vitiating factor
    pub fn description(&self) -> &'static str {
        match self {
            Self::Coercion => "Committing or threatening criminal acts to compel consent",
            Self::UndueInfluence => "Domination of will through position of authority",
            Self::Fraud => "Intentional deception to induce consent",
            Self::Misrepresentation => "Innocent but false statement of fact",
            Self::MistakeOfFact => "Erroneous belief about matter of fact essential to agreement",
            Self::MistakeOfLaw => "Misunderstanding of legal position",
        }
    }
}

/// Competency to contract (Section 11)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncompetentParty {
    /// Minor (below 18 years)
    Minor,
    /// Person of unsound mind (Section 12)
    UnsoundMind,
    /// Person disqualified by law
    DisqualifiedByLaw,
    /// Alien enemy
    AlienEnemy,
    /// Insolvent
    Insolvent,
    /// Convict
    Convict,
}

impl IncompetentParty {
    /// Get section reference
    pub fn section(&self) -> u32 {
        match self {
            Self::Minor => 11,
            Self::UnsoundMind => 12,
            _ => 11,
        }
    }

    /// Effect on contract
    pub fn contract_effect(&self) -> ContractStatus {
        match self {
            Self::Minor => ContractStatus::Void, // Mohori Bibee v. Dharmodas Ghose
            Self::UnsoundMind => ContractStatus::Void,
            Self::DisqualifiedByLaw | Self::AlienEnemy | Self::Insolvent | Self::Convict => {
                ContractStatus::Void
            }
        }
    }
}

/// Types of void agreements (Sections 24-30)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VoidAgreementType {
    /// Agreement without consideration (Section 25)
    NoConsideration,
    /// Agreement in restraint of marriage (Section 26)
    RestraintOfMarriage,
    /// Agreement in restraint of trade (Section 27)
    RestraintOfTrade,
    /// Agreement in restraint of legal proceedings (Section 28)
    RestraintOfLegalProceedings,
    /// Uncertain agreements (Section 29)
    UncertainAgreement,
    /// Wagering agreements (Section 30)
    WageringAgreement,
    /// Agreement to do impossible act (Section 56)
    ImpossibleAct,
}

impl VoidAgreementType {
    /// Get section reference
    pub fn section(&self) -> u32 {
        match self {
            Self::NoConsideration => 25,
            Self::RestraintOfMarriage => 26,
            Self::RestraintOfTrade => 27,
            Self::RestraintOfLegalProceedings => 28,
            Self::UncertainAgreement => 29,
            Self::WageringAgreement => 30,
            Self::ImpossibleAct => 56,
        }
    }

    /// Exceptions to the rule (if any)
    pub fn exceptions(&self) -> Vec<&'static str> {
        match self {
            Self::NoConsideration => vec![
                "Natural love and affection (writing, near relations)",
                "Compensation for past voluntary service",
                "Promise to pay time-barred debt (writing)",
            ],
            Self::RestraintOfTrade => vec![
                "Sale of goodwill (reasonable restriction)",
                "Partner not to carry on similar business",
            ],
            Self::RestraintOfLegalProceedings => vec!["Contract to refer disputes to arbitration"],
            _ => vec![],
        }
    }
}

/// Consideration (Section 2(d))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consideration {
    /// Type of consideration
    pub consideration_type: ConsiderationType,
    /// Value in rupees (if monetary)
    pub monetary_value: Option<f64>,
    /// Description
    pub description: String,
    /// Past, present, or future
    pub timing: ConsiderationTiming,
}

/// Type of consideration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsiderationType {
    /// Monetary payment
    Money,
    /// Goods or services
    GoodsServices,
    /// Promise (executory)
    Promise,
    /// Forbearance
    Forbearance,
    /// Settlement of dispute
    Settlement,
}

/// Timing of consideration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsiderationTiming {
    /// Past consideration (valid in India under Section 2(d))
    Past,
    /// Present/executed consideration
    Present,
    /// Future/executory consideration
    Future,
}

impl ConsiderationTiming {
    /// Check if valid in India
    pub fn is_valid_in_india(&self) -> bool {
        // Unlike English law, past consideration is valid in India
        true
    }
}

/// Discharge of contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DischargeMode {
    /// By performance (Section 37-38)
    Performance,
    /// By mutual agreement (Section 62-63)
    MutualAgreement,
    /// By impossibility (Section 56)
    Impossibility,
    /// By lapse of time (Limitation Act)
    LapseOfTime,
    /// By operation of law
    OperationOfLaw,
    /// By breach
    Breach,
}

impl DischargeMode {
    /// Get section reference
    pub fn section(&self) -> Option<&'static str> {
        match self {
            Self::Performance => Some("Sections 37-38"),
            Self::MutualAgreement => Some("Sections 62-63"),
            Self::Impossibility => Some("Section 56"),
            Self::LapseOfTime => Some("Limitation Act, 1963"),
            Self::OperationOfLaw => None,
            Self::Breach => Some("Section 39"),
        }
    }
}

/// Types of breach of contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BreachType {
    /// Actual breach at due date
    ActualBreach,
    /// Anticipatory breach (Section 39)
    AnticipatoryBreach,
    /// Fundamental breach
    FundamentalBreach,
    /// Minor breach
    MinorBreach,
}

impl BreachType {
    /// Get section reference
    pub fn section(&self) -> Option<u32> {
        match self {
            Self::AnticipatoryBreach => Some(39),
            _ => None,
        }
    }
}

/// Remedies for breach of contract (Sections 73-75)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Remedy {
    /// Damages (Section 73)
    Damages,
    /// Liquidated damages (Section 74)
    LiquidatedDamages,
    /// Penalty (Section 74)
    Penalty,
    /// Quantum meruit
    QuantumMeruit,
    /// Specific performance (Specific Relief Act)
    SpecificPerformance,
    /// Injunction (Specific Relief Act)
    Injunction,
    /// Rescission
    Rescission,
}

impl Remedy {
    /// Get section reference
    pub fn section(&self) -> Option<&'static str> {
        match self {
            Self::Damages => Some("Section 73"),
            Self::LiquidatedDamages | Self::Penalty => Some("Section 74"),
            Self::QuantumMeruit => Some("Section 70"),
            Self::SpecificPerformance | Self::Injunction => Some("Specific Relief Act, 1963"),
            Self::Rescission => Some("Section 64"),
        }
    }

    /// Description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Damages => "Monetary compensation for loss suffered",
            Self::LiquidatedDamages => "Pre-agreed amount as compensation",
            Self::Penalty => "Amount stipulated in terrorem (only reasonable compensation awarded)",
            Self::QuantumMeruit => "Reasonable value for services rendered",
            Self::SpecificPerformance => "Court order to perform contractual obligations",
            Self::Injunction => "Court order restraining party from doing something",
            Self::Rescission => "Setting aside the contract",
        }
    }
}

/// Damages types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamagesType {
    /// Ordinary/general damages
    Ordinary,
    /// Special damages
    Special,
    /// Exemplary damages
    Exemplary,
    /// Nominal damages
    Nominal,
}

impl DamagesType {
    /// Description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Ordinary => "Natural consequence of breach in usual course of things",
            Self::Special => "Consequence of special circumstances known to parties",
            Self::Exemplary => "Punishment for willful breach (rare in contract)",
            Self::Nominal => "Breach proven but no actual loss",
        }
    }
}

/// Quasi-contractual obligation (Sections 68-72)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuasiContractType {
    /// Necessaries supplied to incapable (Section 68)
    NecessariesToIncapable,
    /// Payment of another's debt (Section 69)
    PaymentOfAnothersDebt,
    /// Non-gratuitous act benefit (Section 70)
    NonGratuitousAct,
    /// Finder of goods (Section 71)
    FinderOfGoods,
    /// Mistake or coercion payment (Section 72)
    MistakePayment,
}

impl QuasiContractType {
    /// Get section reference
    pub fn section(&self) -> u32 {
        match self {
            Self::NecessariesToIncapable => 68,
            Self::PaymentOfAnothersDebt => 69,
            Self::NonGratuitousAct => 70,
            Self::FinderOfGoods => 71,
            Self::MistakePayment => 72,
        }
    }
}

/// Contingent contract conditions (Section 31)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContingentContract {
    /// The contingent event
    pub event: String,
    /// Whether event must happen or not happen
    pub on_happening: bool,
    /// Event is possible
    pub event_possible: bool,
    /// Whether event occurred
    pub event_occurred: Option<bool>,
}

impl ContingentContract {
    /// Check if contract can be enforced
    pub fn is_enforceable(&self) -> bool {
        if !self.event_possible {
            return false; // Void under Section 36
        }

        match self.event_occurred {
            Some(true) if self.on_happening => true,
            Some(false) if !self.on_happening => true,
            None => false, // Event hasn't occurred yet
            _ => false,
        }
    }
}

/// Agency types (Chapter X)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentType {
    /// General agent
    General,
    /// Special agent
    Special,
    /// Universal agent
    Universal,
    /// Factor
    Factor,
    /// Broker
    Broker,
    /// Commission agent
    CommissionAgent,
    /// Del credere agent
    DelCredere,
    /// Auctioneer
    Auctioneer,
}

impl AgentType {
    /// Section reference
    pub fn section(&self) -> &'static str {
        "Chapter X (Sections 182-238)"
    }
}

/// Agent authority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentAuthority {
    /// Express authority
    Express,
    /// Implied authority (Section 187)
    Implied,
    /// Apparent/ostensible authority
    Apparent,
    /// Agency by ratification (Section 196)
    Ratification,
    /// Agency by necessity
    Necessity,
    /// Agency by estoppel
    Estoppel,
}

impl AgentAuthority {
    /// Section reference
    pub fn section(&self) -> Option<u32> {
        match self {
            Self::Implied => Some(187),
            Self::Ratification => Some(196),
            _ => None,
        }
    }
}

/// Contract party
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractParty {
    /// Party name
    pub name: String,
    /// Party type (individual, company, etc.)
    pub party_type: PartyType,
    /// Age (for competency check)
    pub age: Option<u32>,
    /// Whether party is of sound mind
    pub sound_mind: bool,
    /// Any legal disqualification
    pub disqualification: Option<IncompetentParty>,
}

impl ContractParty {
    /// Check if party is competent to contract
    pub fn is_competent(&self) -> bool {
        // Must be 18+ (major)
        if let Some(age) = self.age
            && age < 18
        {
            return false;
        }
        // Must be of sound mind
        if !self.sound_mind {
            return false;
        }
        // Must not be disqualified
        self.disqualification.is_none()
    }
}

/// Party type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartyType {
    /// Individual person
    Individual,
    /// Company
    Company,
    /// Partnership firm
    Partnership,
    /// LLP
    Llp,
    /// Government
    Government,
    /// Trust
    Trust,
    /// Society
    Society,
}

/// Contract details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    /// Contract ID
    pub id: String,
    /// Contract type
    pub contract_type: ContractType,
    /// Status
    pub status: ContractStatus,
    /// Parties
    pub parties: Vec<ContractParty>,
    /// Date of agreement
    pub agreement_date: NaiveDate,
    /// Consideration
    pub consideration: Consideration,
    /// Essential elements
    pub essentials: ContractEssentials,
    /// Subject matter
    pub subject_matter: String,
    /// Performance due date
    pub performance_due: Option<NaiveDate>,
    /// Written or oral
    pub is_written: bool,
    /// Stamp duty paid
    pub stamp_duty_paid: bool,
    /// Registered (if required)
    pub registered: Option<bool>,
}

impl Contract {
    /// Check if contract is valid
    pub fn is_valid(&self) -> bool {
        self.essentials.is_valid()
            && self.parties.iter().all(|p| p.is_competent())
            && matches!(self.status, ContractStatus::Valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_essentials() {
        let essentials = ContractEssentials {
            free_consent: true,
            competent_parties: true,
            lawful_consideration: true,
            lawful_object: true,
            not_void: true,
        };
        assert!(essentials.is_valid());

        let invalid = ContractEssentials {
            free_consent: false,
            ..essentials
        };
        assert!(!invalid.is_valid());
        assert_eq!(invalid.missing_essentials().len(), 1);
    }

    #[test]
    fn test_consent_vitiator_effect() {
        assert_eq!(
            ConsentVitiator::Coercion.contract_effect(),
            ContractStatus::Voidable
        );
        assert_eq!(
            ConsentVitiator::MistakeOfFact.contract_effect(),
            ContractStatus::Void
        );
    }

    #[test]
    fn test_void_agreement_sections() {
        assert_eq!(VoidAgreementType::WageringAgreement.section(), 30);
        assert_eq!(VoidAgreementType::RestraintOfTrade.section(), 27);
    }

    #[test]
    fn test_party_competency() {
        let adult = ContractParty {
            name: "Test Person".to_string(),
            party_type: PartyType::Individual,
            age: Some(25),
            sound_mind: true,
            disqualification: None,
        };
        assert!(adult.is_competent());

        let minor = ContractParty {
            age: Some(16),
            ..adult.clone()
        };
        assert!(!minor.is_competent());
    }

    #[test]
    fn test_contingent_contract() {
        let contract = ContingentContract {
            event: "Rain tomorrow".to_string(),
            on_happening: true,
            event_possible: true,
            event_occurred: Some(true),
        };
        assert!(contract.is_enforceable());

        let impossible = ContingentContract {
            event_possible: false,
            ..contract
        };
        assert!(!impossible.is_enforceable());
    }

    #[test]
    fn test_consideration_timing() {
        // Past consideration is valid in India
        assert!(ConsiderationTiming::Past.is_valid_in_india());
        assert!(ConsiderationTiming::Present.is_valid_in_india());
        assert!(ConsiderationTiming::Future.is_valid_in_india());
    }

    #[test]
    fn test_remedy_sections() {
        assert_eq!(Remedy::Damages.section(), Some("Section 73"));
        assert_eq!(Remedy::LiquidatedDamages.section(), Some("Section 74"));
    }

    #[test]
    fn test_quasi_contract_sections() {
        assert_eq!(QuasiContractType::NecessariesToIncapable.section(), 68);
        assert_eq!(QuasiContractType::FinderOfGoods.section(), 71);
    }
}
