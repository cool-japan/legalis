//! Types for French inheritance law
//!
//! This module defines the core types for representing successions, wills,
//! heirs, and estate distributions under French law.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Represents a person in inheritance context
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub date_of_birth: Option<NaiveDate>,
    pub date_of_death: Option<NaiveDate>,
}

impl Person {
    /// Creates a new person with name and age
    pub fn new(name: String, age: u32) -> Self {
        Self {
            name,
            age,
            date_of_birth: None,
            date_of_death: None,
        }
    }

    /// Sets the date of birth
    pub fn with_date_of_birth(mut self, date: NaiveDate) -> Self {
        self.date_of_birth = Some(date);
        self
    }

    /// Sets the date of death
    pub fn with_date_of_death(mut self, date: NaiveDate) -> Self {
        self.date_of_death = Some(date);
        self
    }
}

/// Represents a succession (succession)
///
/// A succession opens at the moment of death and at the last domicile
/// of the deceased (Article 720)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Succession {
    pub deceased: Person,
    pub death_date: NaiveDate,
    pub last_domicile: String,
    pub heirs: Vec<Heir>,
    pub estate: Vec<Asset>,
    pub debts: Vec<Debt>,
    pub will: Option<Will>,
    pub opened: bool,
}

impl Succession {
    /// Creates a new succession
    pub fn new(deceased: Person, death_date: NaiveDate) -> Self {
        Self {
            deceased,
            death_date,
            last_domicile: String::new(),
            heirs: Vec::new(),
            estate: Vec::new(),
            debts: Vec::new(),
            will: None,
            opened: false,
        }
    }

    /// Sets the last domicile of the deceased
    pub fn with_last_domicile(mut self, domicile: String) -> Self {
        self.last_domicile = domicile;
        self
    }

    /// Adds an heir to the succession
    pub fn with_heir(mut self, heir: Heir) -> Self {
        self.heirs.push(heir);
        self
    }

    /// Adds multiple heirs to the succession
    pub fn with_heirs(mut self, heirs: Vec<Heir>) -> Self {
        self.heirs = heirs;
        self
    }

    /// Adds an asset to the estate
    pub fn with_asset(mut self, asset: Asset) -> Self {
        self.estate.push(asset);
        self
    }

    /// Sets the estate assets
    pub fn with_estate(mut self, estate: Vec<Asset>) -> Self {
        self.estate = estate;
        self
    }

    /// Adds a debt to the succession
    pub fn with_debt(mut self, debt: Debt) -> Self {
        self.debts.push(debt);
        self
    }

    /// Sets the debts
    pub fn with_debts(mut self, debts: Vec<Debt>) -> Self {
        self.debts = debts;
        self
    }

    /// Sets the will
    pub fn with_will(mut self, will: Will) -> Self {
        self.will = Some(will);
        self
    }

    /// Marks the succession as opened
    pub fn with_opened(mut self, opened: bool) -> Self {
        self.opened = opened;
        self
    }

    /// Calculates the total value of the estate
    pub fn total_estate_value(&self) -> u64 {
        self.estate.iter().map(|a| a.value).sum()
    }

    /// Calculates the total debts
    pub fn total_debts(&self) -> u64 {
        self.debts.iter().map(|d| d.amount).sum()
    }

    /// Calculates the net estate value (assets - debts)
    pub fn net_estate_value(&self) -> i64 {
        self.total_estate_value() as i64 - self.total_debts() as i64
    }
}

/// Represents an heir in a succession
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Heir {
    pub person: Person,
    pub relationship: Relationship,
    pub reserved_portion: Option<f64>, // réserve héréditaire (0.0-1.0)
    pub actual_share: f64,             // actual inheritance share (0.0-1.0)
    pub renounced: bool,               // has renounced the succession
}

impl Heir {
    /// Creates a new heir
    pub fn new(person: Person, relationship: Relationship) -> Self {
        Self {
            person,
            relationship,
            reserved_portion: None,
            actual_share: 0.0,
            renounced: false,
        }
    }

    /// Sets the reserved portion (réserve héréditaire)
    pub fn with_reserved_portion(mut self, portion: f64) -> Self {
        self.reserved_portion = Some(portion);
        self
    }

    /// Sets the actual inheritance share
    pub fn with_actual_share(mut self, share: f64) -> Self {
        self.actual_share = share;
        self
    }

    /// Marks the heir as having renounced the succession
    pub fn with_renounced(mut self, renounced: bool) -> Self {
        self.renounced = renounced;
        self
    }
}

/// Relationship of heir to deceased
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Relationship {
    Child,
    Grandchild,
    GreatGrandchild,
    Spouse,
    Parent,
    Grandparent,
    Sibling,
    HalfSibling,
    NieceNephew,
    UncleAunt,
    Cousin,
    Other,
}

impl Relationship {
    /// Returns the order of succession (1 = first order, etc.)
    pub fn succession_order(&self) -> u32 {
        match self {
            Self::Child | Self::Grandchild | Self::GreatGrandchild => 1,
            Self::Parent | Self::Sibling | Self::HalfSibling => 2,
            Self::Grandparent => 3,
            Self::UncleAunt | Self::NieceNephew | Self::Cousin => 4,
            Self::Spouse => 0, // spouse has special rules
            Self::Other => 5,
        }
    }
}

/// Represents a will (testament)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Will {
    pub will_type: WillType,
    pub testator: String,
    pub date: NaiveDate,
    pub dispositions: Vec<Disposition>,
    pub revoked: bool,
}

impl Will {
    /// Creates a new will
    pub fn new(will_type: WillType, testator: String, date: NaiveDate) -> Self {
        Self {
            will_type,
            testator,
            date,
            dispositions: Vec::new(),
            revoked: false,
        }
    }

    /// Adds a disposition to the will
    pub fn with_disposition(mut self, disposition: Disposition) -> Self {
        self.dispositions.push(disposition);
        self
    }

    /// Sets the dispositions
    pub fn with_dispositions(mut self, dispositions: Vec<Disposition>) -> Self {
        self.dispositions = dispositions;
        self
    }

    /// Marks the will as revoked
    pub fn with_revoked(mut self, revoked: bool) -> Self {
        self.revoked = revoked;
        self
    }
}

/// Types of wills recognized under French law
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WillType {
    /// Authentic will (testament authentique) - Article 971
    /// Made before a notary and two witnesses
    Authentic {
        notary: String,
        witnesses: Vec<String>,
    },

    /// Holographic will (testament olographe) - Article 970
    /// Entirely handwritten, dated, and signed by testator
    Holographic {
        handwritten: bool,
        dated: bool,
        signed: bool,
    },

    /// Mystic will (testament mystique) - Article 976
    /// Sealed and presented to notary
    Mystic { sealed: bool, notary: String },
}

/// Represents a testamentary disposition (legs)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disposition {
    pub disposition_type: DispositionType,
    pub beneficiary: String,
    pub description: String,
    pub value: Option<u64>,
}

impl Disposition {
    /// Creates a new disposition
    pub fn new(disposition_type: DispositionType, beneficiary: String) -> Self {
        Self {
            disposition_type,
            beneficiary,
            description: String::new(),
            value: None,
        }
    }

    /// Sets the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Sets the value
    pub fn with_value(mut self, value: u64) -> Self {
        self.value = Some(value);
        self
    }
}

/// Types of testamentary dispositions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DispositionType {
    /// Universal legacy (legs universel) - all estate
    Universal,

    /// General legacy (legs à titre universel) - portion of estate
    General,

    /// Specific legacy (legs particulier) - specific items
    Specific,
}

/// Represents an asset in the estate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub asset_type: AssetType,
    pub description: String,
    pub value: u64,
    pub acquisition_date: Option<NaiveDate>,
}

impl Asset {
    /// Creates a new asset
    pub fn new(asset_type: AssetType, description: String, value: u64) -> Self {
        Self {
            asset_type,
            description,
            value,
            acquisition_date: None,
        }
    }

    /// Sets the acquisition date
    pub fn with_acquisition_date(mut self, date: NaiveDate) -> Self {
        self.acquisition_date = Some(date);
        self
    }
}

/// Types of assets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    RealEstate,
    BankAccount,
    Investment,
    PersonalProperty,
    IntellectualProperty,
    BusinessInterest,
    Other,
}

/// Represents a debt of the estate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Debt {
    pub creditor: String,
    pub amount: u64,
    pub description: String,
    pub due_date: Option<NaiveDate>,
}

impl Debt {
    /// Creates a new debt
    pub fn new(creditor: String, amount: u64) -> Self {
        Self {
            creditor,
            amount,
            description: String::new(),
            due_date: None,
        }
    }

    /// Sets the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Sets the due date
    pub fn with_due_date(mut self, date: NaiveDate) -> Self {
        self.due_date = Some(date);
        self
    }
}

/// Reserved portion calculation result (réserve héréditaire)
#[derive(Debug, Clone, PartialEq)]
pub struct ReservedPortion {
    pub total_children: u32,
    pub reserved_portion: f64,  // 0.0-1.0
    pub available_portion: f64, // 0.0-1.0 (quotité disponible)
}

impl ReservedPortion {
    /// Calculates reserved portion based on number of children (Article 913)
    pub fn calculate(total_children: u32) -> Self {
        let reserved_portion = match total_children {
            0 => 0.0,
            1 => 0.5,       // 1/2
            2 => 2.0 / 3.0, // 2/3
            _ => 0.75,      // 3/4 for 3 or more children
        };

        Self {
            total_children,
            reserved_portion,
            available_portion: 1.0 - reserved_portion,
        }
    }

    /// Returns the share per child
    pub fn share_per_child(&self) -> f64 {
        if self.total_children == 0 {
            0.0
        } else {
            self.reserved_portion / self.total_children as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_creation() {
        let person = Person::new("Jean Dupont".to_string(), 75);
        assert_eq!(person.name, "Jean Dupont");
        assert_eq!(person.age, 75);
        assert!(person.date_of_birth.is_none());
    }

    #[test]
    fn test_succession_builder() {
        let deceased = Person::new("Jean Dupont".to_string(), 75);
        let succession = Succession::new(deceased, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
            .with_last_domicile("Paris, France".to_string())
            .with_opened(true);

        assert_eq!(succession.last_domicile, "Paris, France");
        assert!(succession.opened);
    }

    #[test]
    fn test_heir_with_reserved_portion() {
        let person = Person::new("Marie Dupont".to_string(), 45);
        let heir = Heir::new(person, Relationship::Child).with_reserved_portion(0.5);

        assert_eq!(heir.reserved_portion, Some(0.5));
        assert_eq!(heir.relationship, Relationship::Child);
    }

    #[test]
    fn test_will_creation() {
        let will_type = WillType::Holographic {
            handwritten: true,
            dated: true,
            signed: true,
        };
        let will = Will::new(
            will_type,
            "Jean Dupont".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        );

        assert!(!will.revoked);
        assert!(will.dispositions.is_empty());
    }

    #[test]
    fn test_reserved_portion_calculation() {
        // 1 child = 1/2 reserved
        let portion1 = ReservedPortion::calculate(1);
        assert_eq!(portion1.reserved_portion, 0.5);
        assert_eq!(portion1.available_portion, 0.5);

        // 2 children = 2/3 reserved
        let portion2 = ReservedPortion::calculate(2);
        assert_eq!(portion2.reserved_portion, 2.0 / 3.0);

        // 3+ children = 3/4 reserved
        let portion3 = ReservedPortion::calculate(3);
        assert_eq!(portion3.reserved_portion, 0.75);
        assert_eq!(portion3.available_portion, 0.25);
    }

    #[test]
    fn test_relationship_succession_order() {
        assert_eq!(Relationship::Child.succession_order(), 1);
        assert_eq!(Relationship::Parent.succession_order(), 2);
        assert_eq!(Relationship::Grandparent.succession_order(), 3);
        assert_eq!(Relationship::UncleAunt.succession_order(), 4);
        assert_eq!(Relationship::Spouse.succession_order(), 0);
    }

    #[test]
    fn test_succession_estate_calculations() {
        let deceased = Person::new("Jean Dupont".to_string(), 75);
        let mut succession =
            Succession::new(deceased, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());

        succession = succession
            .with_asset(Asset::new(
                AssetType::RealEstate,
                "Apartment".to_string(),
                300_000,
            ))
            .with_asset(Asset::new(
                AssetType::BankAccount,
                "Savings".to_string(),
                50_000,
            ))
            .with_debt(Debt::new("Bank".to_string(), 100_000));

        assert_eq!(succession.total_estate_value(), 350_000);
        assert_eq!(succession.total_debts(), 100_000);
        assert_eq!(succession.net_estate_value(), 250_000);
    }

    #[test]
    fn test_disposition_creation() {
        let disposition = Disposition::new(DispositionType::Specific, "Marie Dupont".to_string())
            .with_description("Family home".to_string())
            .with_value(300_000);

        assert_eq!(disposition.beneficiary, "Marie Dupont");
        assert_eq!(disposition.value, Some(300_000));
    }

    #[test]
    fn test_asset_types() {
        let asset1 = Asset::new(AssetType::RealEstate, "House".to_string(), 500_000);
        let asset2 = Asset::new(AssetType::BankAccount, "Checking".to_string(), 10_000);

        assert_eq!(asset1.asset_type, AssetType::RealEstate);
        assert_eq!(asset2.asset_type, AssetType::BankAccount);
    }
}
