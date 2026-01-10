//! Types for French family law.
//!
//! This module provides structured representations of:
//! - Marriage (Mariage)
//! - Divorce
//! - Property Regimes (Régimes matrimoniaux)
//! - PACS (Civil solidarity pacts)
//! - Parental Relations

use chrono::NaiveDate;
use std::fmt;

/// Nationality of a person.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Nationality {
    French,
    Foreign(String),
}

impl fmt::Display for Nationality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::French => write!(f, "French"),
            Self::Foreign(country) => write!(f, "{}", country),
        }
    }
}

/// Marital status of a person.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MaritalStatus {
    Single,
    Married,
    Divorced,
    Widowed,
    PACS,
}

/// Relationship between two persons (for consanguinity checks).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Relationship {
    /// Direct ancestors (parent, grandparent, etc.)
    DirectAscendant,
    /// Direct descendants (child, grandchild, etc.)
    DirectDescendant,
    /// Siblings (brother, sister)
    Sibling,
    /// Uncle/Aunt
    UncleAunt,
    /// Nephew/Niece
    NephewNiece,
    /// First cousins
    FirstCousin,
    /// No known relationship
    None,
    /// Other specified relationship
    Other(String),
}

impl fmt::Display for Relationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DirectAscendant => write!(f, "Direct ascendant"),
            Self::DirectDescendant => write!(f, "Direct descendant"),
            Self::Sibling => write!(f, "Sibling"),
            Self::UncleAunt => write!(f, "Uncle/Aunt"),
            Self::NephewNiece => write!(f, "Nephew/Niece"),
            Self::FirstCousin => write!(f, "First cousin"),
            Self::None => write!(f, "No relationship"),
            Self::Other(rel) => write!(f, "{}", rel),
        }
    }
}

/// A person involved in family law matters.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub nationality: Nationality,
    pub marital_status: MaritalStatus,
    pub relationship_to_other: Option<Relationship>,
}

impl Person {
    /// Create a new person.
    #[must_use]
    pub fn new(
        name: String,
        age: u32,
        nationality: Nationality,
        marital_status: MaritalStatus,
    ) -> Self {
        Self {
            name,
            age,
            nationality,
            marital_status,
            relationship_to_other: None,
        }
    }

    /// Set the relationship to another person.
    #[must_use]
    pub fn with_relationship(mut self, relationship: Relationship) -> Self {
        self.relationship_to_other = Some(relationship);
        self
    }
}

/// Grounds for opposing a marriage (Article 165).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OppositionGround {
    /// Existing marriage (bigamy)
    ExistingMarriage,
    /// Insufficient age
    InsufficientAge,
    /// Lack of consent
    LackOfConsent,
    /// Consanguinity
    Consanguinity,
    /// Mental incapacity
    MentalIncapacity,
    /// Other ground
    Other(String),
}

impl fmt::Display for OppositionGround {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExistingMarriage => write!(f, "Existing marriage"),
            Self::InsufficientAge => write!(f, "Insufficient age"),
            Self::LackOfConsent => write!(f, "Lack of consent"),
            Self::Consanguinity => write!(f, "Consanguinity"),
            Self::MentalIncapacity => write!(f, "Mental incapacity"),
            Self::Other(reason) => write!(f, "{}", reason),
        }
    }
}

/// Opposition to a marriage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarriageOpposition {
    pub ground: OppositionGround,
    pub filed_by: String,
    pub filed_date: NaiveDate,
}

/// Marriage (Mariage).
///
/// Represents a marriage under French law, including requirements from Articles 143-180.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Marriage {
    pub parties: [Person; 2],
    pub marriage_date: Option<NaiveDate>,
    pub banns_published: bool,
    pub banns_publication_date: Option<NaiveDate>,
    pub oppositions: Vec<MarriageOpposition>,
    pub consent_given: [bool; 2],
    pub proxy_used: bool,
}

impl Marriage {
    /// Create a new marriage.
    #[must_use]
    pub fn new(party1: Person, party2: Person) -> Self {
        Self {
            parties: [party1, party2],
            marriage_date: None,
            banns_published: false,
            banns_publication_date: None,
            oppositions: Vec::new(),
            consent_given: [false, false],
            proxy_used: false,
        }
    }

    /// Set marriage date.
    #[must_use]
    pub fn with_marriage_date(mut self, date: NaiveDate) -> Self {
        self.marriage_date = Some(date);
        self
    }

    /// Set banns publication status.
    #[must_use]
    pub fn with_banns_published(mut self, published: bool) -> Self {
        self.banns_published = published;
        self
    }

    /// Set banns publication date.
    #[must_use]
    pub fn with_banns_publication_date(mut self, date: NaiveDate) -> Self {
        self.banns_publication_date = Some(date);
        self
    }

    /// Add an opposition.
    #[must_use]
    pub fn with_opposition(mut self, opposition: MarriageOpposition) -> Self {
        self.oppositions.push(opposition);
        self
    }

    /// Set consent status for both parties.
    #[must_use]
    pub fn with_consent(mut self, consent: [bool; 2]) -> Self {
        self.consent_given = consent;
        self
    }

    /// Set proxy usage status.
    #[must_use]
    pub fn with_proxy_used(mut self, proxy_used: bool) -> Self {
        self.proxy_used = proxy_used;
        self
    }
}

/// Type of fault in a fault-based divorce (Article 242).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FaultType {
    Violence,
    Adultery,
    SevereBreachOfMaritalDuties,
    Other(String),
}

impl fmt::Display for FaultType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Violence => write!(f, "Violence"),
            Self::Adultery => write!(f, "Adultery"),
            Self::SevereBreachOfMaritalDuties => write!(f, "Severe breach of marital duties"),
            Self::Other(reason) => write!(f, "{}", reason),
        }
    }
}

/// Type of divorce (Article 229).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DivorceType {
    /// Mutual consent divorce (Article 230).
    ///
    /// Simplified procedure with notary filing since 2017 reform.
    MutualConsent {
        agreement_signed: bool,
        notary_filing_date: Option<NaiveDate>,
        children_heard: bool,
    },

    /// Divorce by acceptance of the principle (Article 233).
    ///
    /// Both parties accept the divorce principle but disagree on consequences.
    AcceptancePrinciple {
        both_accept_principle: bool,
        disagreement_on_consequences: bool,
    },

    /// Divorce for definitive alteration of the marriage bond (Article 237).
    ///
    /// Requires separation of at least 24 months.
    DefinitiveAlteration { separation_duration_months: u32 },

    /// Fault-based divorce (Article 242).
    ///
    /// Divorce due to serious breach by one spouse.
    Fault {
        fault_type: FaultType,
        evidence: Vec<String>,
    },
}

/// Child in a divorce.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Child {
    pub name: String,
    pub age: u32,
    pub heard: bool,
}

impl Child {
    /// Create a new child.
    #[must_use]
    pub fn new(name: String, age: u32) -> Self {
        Self {
            name,
            age,
            heard: false,
        }
    }

    /// Set whether the child has been heard.
    #[must_use]
    pub fn with_heard(mut self, heard: bool) -> Self {
        self.heard = heard;
        self
    }
}

/// Asset in property division.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Asset {
    pub description: String,
    pub value: u64,
    pub acquisition_date: NaiveDate,
}

impl Asset {
    /// Create a new asset.
    #[must_use]
    pub fn new(description: String, value: u64, acquisition_date: NaiveDate) -> Self {
        Self {
            description,
            value,
            acquisition_date,
        }
    }
}

/// Property regime (Régime matrimonial).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PropertyRegime {
    /// Communauté réduite aux acquêts (default since 1966, Article 1400).
    ///
    /// Property acquired during marriage is jointly owned.
    CommunauteReduite {
        marriage_contract: bool,
        acquets: Vec<Asset>,
        biens_propres: Vec<(String, Asset)>,
    },

    /// Séparation de biens (Article 1536).
    ///
    /// Each spouse retains ownership of their own property.
    SeparationDeBiens { marriage_contract: bool },

    /// Communauté universelle.
    ///
    /// All property (past and future) is jointly owned.
    CommunauteUniverselle { marriage_contract: bool },

    /// Custom regime.
    Custom {
        description: String,
        marriage_contract: bool,
    },
}

/// Divorce proceeding.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Divorce {
    pub divorce_type: DivorceType,
    pub marriage_date: NaiveDate,
    pub parties: [String; 2],
    pub children: Vec<Child>,
    pub property_regime: PropertyRegime,
    pub separation_date: Option<NaiveDate>,
}

impl Divorce {
    /// Create a new divorce.
    #[must_use]
    pub fn new(
        divorce_type: DivorceType,
        marriage_date: NaiveDate,
        party1: String,
        party2: String,
        property_regime: PropertyRegime,
    ) -> Self {
        Self {
            divorce_type,
            marriage_date,
            parties: [party1, party2],
            children: Vec::new(),
            property_regime,
            separation_date: None,
        }
    }

    /// Add a child.
    #[must_use]
    pub fn with_child(mut self, child: Child) -> Self {
        self.children.push(child);
        self
    }

    /// Set separation date.
    #[must_use]
    pub fn with_separation_date(mut self, date: NaiveDate) -> Self {
        self.separation_date = Some(date);
        self
    }

    /// Get separation duration in months.
    #[must_use]
    pub fn separation_duration_months(&self) -> Option<u32> {
        if let Some(sep_date) = self.separation_date {
            let now = chrono::Utc::now().naive_utc().date();
            let duration = now.signed_duration_since(sep_date);
            Some((duration.num_days() / 30) as u32)
        } else {
            None
        }
    }
}

/// PACS (Pacte civil de solidarité) - Civil solidarity pact (Article 515-1).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PACS {
    pub parties: [String; 2],
    pub registration_date: Option<NaiveDate>,
    pub property_regime: PACSPropertyRegime,
    pub dissolution_date: Option<NaiveDate>,
    pub dissolution_notice_date: Option<NaiveDate>,
}

/// Property regime for PACS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PACSPropertyRegime {
    /// Separation of property (default).
    Separation,
    /// Joint ownership.
    Joint,
}

impl PACS {
    /// Create a new PACS.
    #[must_use]
    pub fn new(party1: String, party2: String) -> Self {
        Self {
            parties: [party1, party2],
            registration_date: None,
            property_regime: PACSPropertyRegime::Separation,
            dissolution_date: None,
            dissolution_notice_date: None,
        }
    }

    /// Set registration date.
    #[must_use]
    pub fn with_registration_date(mut self, date: NaiveDate) -> Self {
        self.registration_date = Some(date);
        self
    }

    /// Set property regime.
    #[must_use]
    pub fn with_property_regime(mut self, regime: PACSPropertyRegime) -> Self {
        self.property_regime = regime;
        self
    }

    /// Set dissolution date.
    #[must_use]
    pub fn with_dissolution_date(mut self, date: NaiveDate) -> Self {
        self.dissolution_date = Some(date);
        self
    }

    /// Set dissolution notice date.
    #[must_use]
    pub fn with_dissolution_notice_date(mut self, date: NaiveDate) -> Self {
        self.dissolution_notice_date = Some(date);
        self
    }

    /// Get dissolution notice period in days.
    #[must_use]
    pub fn dissolution_notice_days(&self) -> Option<u32> {
        if let (Some(notice), Some(dissolution)) =
            (self.dissolution_notice_date, self.dissolution_date)
        {
            let duration = dissolution.signed_duration_since(notice);
            Some(duration.num_days() as u32)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_creation() {
        let person = Person::new(
            "Alice Dupont".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        assert_eq!(person.name, "Alice Dupont");
        assert_eq!(person.age, 25);
        assert_eq!(person.nationality, Nationality::French);
        assert_eq!(person.marital_status, MaritalStatus::Single);
        assert!(person.relationship_to_other.is_none());
    }

    #[test]
    fn test_person_with_relationship() {
        let person = Person::new(
            "Bob Martin".to_string(),
            30,
            Nationality::Foreign("Belgium".to_string()),
            MaritalStatus::Single,
        )
        .with_relationship(Relationship::Sibling);

        assert_eq!(person.relationship_to_other, Some(Relationship::Sibling));
    }

    #[test]
    fn test_marriage_creation() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2)
            .with_consent([true, true])
            .with_banns_published(true);

        assert_eq!(marriage.consent_given, [true, true]);
        assert!(marriage.banns_published);
        assert!(!marriage.proxy_used);
    }

    #[test]
    fn test_divorce_mutual_consent() {
        let divorce_type = DivorceType::MutualConsent {
            agreement_signed: true,
            notary_filing_date: Some(chrono::Utc::now().naive_utc().date()),
            children_heard: true,
        };

        let divorce = Divorce::new(
            divorce_type,
            chrono::Utc::now().naive_utc().date(),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );

        assert_eq!(divorce.parties[0], "Alice");
        assert_eq!(divorce.parties[1], "Bob");
    }

    #[test]
    fn test_divorce_with_children() {
        let child = Child::new("Charlie".to_string(), 10).with_heard(true);

        let divorce = Divorce::new(
            DivorceType::Fault {
                fault_type: FaultType::Violence,
                evidence: vec!["Police report".to_string()],
            },
            chrono::Utc::now().naive_utc().date(),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::SeparationDeBiens {
                marriage_contract: true,
            },
        )
        .with_child(child);

        assert_eq!(divorce.children.len(), 1);
        assert_eq!(divorce.children[0].name, "Charlie");
        assert!(divorce.children[0].heard);
    }

    #[test]
    fn test_pacs_creation() {
        let pacs = PACS::new("Alice".to_string(), "Bob".to_string())
            .with_registration_date(chrono::Utc::now().naive_utc().date())
            .with_property_regime(PACSPropertyRegime::Joint);

        assert_eq!(pacs.parties[0], "Alice");
        assert_eq!(pacs.parties[1], "Bob");
        assert_eq!(pacs.property_regime, PACSPropertyRegime::Joint);
    }

    #[test]
    fn test_asset_creation() {
        let asset = Asset::new(
            "Apartment".to_string(),
            250_000,
            chrono::Utc::now().naive_utc().date(),
        );

        assert_eq!(asset.description, "Apartment");
        assert_eq!(asset.value, 250_000);
    }

    #[test]
    fn test_separation_duration_calculation() {
        use chrono::Duration;
        let sep_date = chrono::Utc::now().naive_utc().date() - Duration::days(750);

        let divorce = Divorce::new(
            DivorceType::DefinitiveAlteration {
                separation_duration_months: 25,
            },
            chrono::Utc::now().naive_utc().date(),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        )
        .with_separation_date(sep_date);

        let duration = divorce.separation_duration_months();
        assert!(duration.is_some());
        assert!(duration.unwrap() >= 24);
    }

    #[test]
    fn test_relationship_display() {
        assert_eq!(Relationship::Sibling.to_string(), "Sibling");
        assert_eq!(
            Relationship::DirectAscendant.to_string(),
            "Direct ascendant"
        );
        assert_eq!(Relationship::FirstCousin.to_string(), "First cousin");
    }

    #[test]
    fn test_fault_type_display() {
        assert_eq!(FaultType::Violence.to_string(), "Violence");
        assert_eq!(FaultType::Adultery.to_string(), "Adultery");
        assert_eq!(
            FaultType::SevereBreachOfMaritalDuties.to_string(),
            "Severe breach of marital duties"
        );
    }
}
