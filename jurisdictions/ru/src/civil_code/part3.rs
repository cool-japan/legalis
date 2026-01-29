//! Civil Code Part 3: Succession Law (2001).
//!
//! Federal Law No. 146-FZ of November 26, 2001
//!
//! This part covers:
//! - General provisions on succession (Articles 1110-1117)
//! - Succession by will (Articles 1118-1140)
//! - Succession by law (Articles 1141-1151)
//! - Acquisition of inheritance (Articles 1152-1175)

use serde::{Deserialize, Serialize};

use super::CivilCodeError;

/// Order of succession by law (очередность наследования по закону)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SuccessionOrder {
    /// First order: children, spouse, parents (Article 1142)
    First = 1,
    /// Second order: siblings, grandparents (Article 1143)
    Second = 2,
    /// Third order: uncles and aunts (Article 1144)
    Third = 3,
    /// Fourth order: great-grandparents (Article 1145)
    Fourth = 4,
    /// Fifth order: great-great-grandparents, children of grandchildren (Article 1145)
    Fifth = 5,
    /// Sixth order: great-great-great-grandparents, children of great-grandchildren (Article 1145)
    Sixth = 6,
    /// Seventh order: stepchildren, stepparents (Article 1145)
    Seventh = 7,
    /// Eighth order: disabled dependents (Article 1148)
    Eighth = 8,
}

impl SuccessionOrder {
    /// Gets the description in Russian
    pub fn description_ru(&self) -> &'static str {
        match self {
            Self::First => "Дети, супруг, родители",
            Self::Second => "Братья, сестры, дедушки, бабушки",
            Self::Third => "Дяди и тети",
            Self::Fourth => "Прадедушки и прабабушки",
            Self::Fifth => "Дети племянников и племянниц, двоюродные внуки",
            Self::Sixth => "Двоюродные правнуки, двоюродные племянники",
            Self::Seventh => "Пасынки, падчерицы, отчим, мачеха",
            Self::Eighth => "Нетрудоспособные иждивенцы",
        }
    }

    /// Gets the description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::First => "Children, spouse, parents",
            Self::Second => "Siblings, grandparents",
            Self::Third => "Uncles and aunts",
            Self::Fourth => "Great-grandparents",
            Self::Fifth => "Children of nieces/nephews, cousins",
            Self::Sixth => "Great-grandchildren of siblings, second cousins",
            Self::Seventh => "Stepchildren, stepparents",
            Self::Eighth => "Disabled dependents",
        }
    }
}

/// Succession rights representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessionRights {
    /// Heir information
    pub heir: Heir,
    /// Order of succession (if by law)
    pub succession_order: Option<SuccessionOrder>,
    /// Share of inheritance (fraction, e.g., 0.5 for half)
    pub share: f64,
    /// Is successor by will (завещание)
    pub by_will: bool,
    /// Is mandatory heir (обязательный наследник)
    pub mandatory_heir: bool,
}

impl SuccessionRights {
    /// Creates succession rights by law
    pub fn by_law(heir: Heir, order: SuccessionOrder, share: f64) -> Self {
        Self {
            heir,
            succession_order: Some(order),
            share,
            by_will: false,
            mandatory_heir: false,
        }
    }

    /// Creates succession rights by will
    pub fn by_will(heir: Heir, share: f64) -> Self {
        Self {
            heir,
            succession_order: None,
            share,
            by_will: true,
            mandatory_heir: false,
        }
    }

    /// Creates mandatory succession rights (Article 1149)
    pub fn mandatory(heir: Heir, share: f64) -> Self {
        Self {
            heir,
            succession_order: Some(SuccessionOrder::First),
            share,
            by_will: false,
            mandatory_heir: true,
        }
    }

    /// Validates the succession rights
    pub fn validate(&self) -> Result<(), CivilCodeError> {
        // Share must be between 0 and 1
        if self.share <= 0.0 || self.share > 1.0 {
            return Err(CivilCodeError::InvalidSuccession(
                "Succession share must be between 0 and 1".to_string(),
            ));
        }

        // Mandatory heirs must have at least half the share they would get by law
        if self.mandatory_heir && self.share < 0.5 {
            return Err(CivilCodeError::InvalidSuccession(
                "Mandatory heir must receive at least half of lawful share".to_string(),
            ));
        }

        Ok(())
    }
}

/// Heir information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heir {
    /// Full name
    pub name: String,
    /// Relationship to deceased
    pub relationship: Relationship,
    /// Birth date
    pub birth_date: Option<chrono::NaiveDate>,
    /// Is disabled or minor
    pub is_disabled_or_minor: bool,
}

/// Relationship to the deceased
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Relationship {
    /// Child
    Child,
    /// Spouse
    Spouse,
    /// Parent
    Parent,
    /// Sibling
    Sibling,
    /// Grandparent
    Grandparent,
    /// Grandchild
    Grandchild,
    /// Uncle/Aunt
    UncleAunt,
    /// Nephew/Niece
    NephewNiece,
    /// Stepchild
    Stepchild,
    /// Stepparent
    Stepparent,
    /// Other
    Other(String),
}

impl Relationship {
    /// Determines the succession order for this relationship
    pub fn succession_order(&self) -> Option<SuccessionOrder> {
        match self {
            Self::Child | Self::Spouse | Self::Parent => Some(SuccessionOrder::First),
            Self::Sibling | Self::Grandparent => Some(SuccessionOrder::Second),
            Self::UncleAunt => Some(SuccessionOrder::Third),
            Self::Grandchild => Some(SuccessionOrder::First), // By representation
            Self::NephewNiece => Some(SuccessionOrder::Second), // By representation
            Self::Stepchild | Self::Stepparent => Some(SuccessionOrder::Seventh),
            Self::Other(_) => None,
        }
    }
}

/// Will (завещание) representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Will {
    /// Testator (завещатель)
    pub testator: String,
    /// Date of will
    pub date: chrono::NaiveDate,
    /// Is notarized
    pub notarized: bool,
    /// Beneficiaries
    pub beneficiaries: Vec<Beneficiary>,
    /// Executor (душеприказчик)
    pub executor: Option<String>,
}

/// Beneficiary in a will
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beneficiary {
    /// Name
    pub name: String,
    /// Share of inheritance
    pub share: f64,
    /// Specific property (if any)
    pub specific_property: Option<String>,
}

impl Will {
    /// Validates the will according to Civil Code requirements
    pub fn validate(&self) -> Result<(), CivilCodeError> {
        // Will must be notarized (Article 1124)
        if !self.notarized {
            return Err(CivilCodeError::InvalidSuccession(
                "Will must be notarized".to_string(),
            ));
        }

        // Total shares cannot exceed 100%
        let total_share: f64 = self.beneficiaries.iter().map(|b| b.share).sum();
        if total_share > 1.0 {
            return Err(CivilCodeError::InvalidSuccession(
                "Total shares in will cannot exceed 100%".to_string(),
            ));
        }

        Ok(())
    }
}

/// Article 1149: Right to mandatory share
pub fn calculate_mandatory_share(heir: &Heir, lawful_share: f64) -> Result<f64, CivilCodeError> {
    // Mandatory heirs: disabled children, parents, spouse, and dependents
    if !heir.is_disabled_or_minor {
        return Err(CivilCodeError::InvalidSuccession(
            "Only disabled or minor heirs have right to mandatory share".to_string(),
        ));
    }

    // Mandatory share is at least half of what they would receive by law
    Ok(lawful_share * 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_succession_order() {
        assert_eq!(SuccessionOrder::First, SuccessionOrder::First);
        assert!(SuccessionOrder::First < SuccessionOrder::Second);
        assert_eq!(
            SuccessionOrder::First.description_en(),
            "Children, spouse, parents"
        );
    }

    #[test]
    fn test_relationship_to_succession_order() {
        assert_eq!(
            Relationship::Child.succession_order(),
            Some(SuccessionOrder::First)
        );
        assert_eq!(
            Relationship::Sibling.succession_order(),
            Some(SuccessionOrder::Second)
        );
        assert_eq!(
            Relationship::UncleAunt.succession_order(),
            Some(SuccessionOrder::Third)
        );
    }

    #[test]
    fn test_succession_rights_validation() {
        let heir = Heir {
            name: "Иванов Иван".to_string(),
            relationship: Relationship::Child,
            birth_date: Some(chrono::NaiveDate::from_ymd_opt(2000, 1, 1).expect("Valid date")),
            is_disabled_or_minor: false,
        };

        let rights = SuccessionRights::by_law(heir, SuccessionOrder::First, 0.5);
        assert!(rights.validate().is_ok());

        // Invalid share
        let heir2 = Heir {
            name: "Петров Петр".to_string(),
            relationship: Relationship::Spouse,
            birth_date: None,
            is_disabled_or_minor: false,
        };
        let invalid_rights = SuccessionRights::by_law(heir2, SuccessionOrder::First, 1.5);
        assert!(invalid_rights.validate().is_err());
    }

    #[test]
    fn test_will_validation() {
        let will = Will {
            testator: "Иванов Иван Иванович".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2023, 1, 1).expect("Valid date"),
            notarized: true,
            beneficiaries: vec![
                Beneficiary {
                    name: "Иванова Мария".to_string(),
                    share: 0.6,
                    specific_property: None,
                },
                Beneficiary {
                    name: "Иванов Петр".to_string(),
                    share: 0.4,
                    specific_property: None,
                },
            ],
            executor: None,
        };

        assert!(will.validate().is_ok());

        // Will without notarization should fail
        let mut invalid_will = will.clone();
        invalid_will.notarized = false;
        assert!(invalid_will.validate().is_err());
    }

    #[test]
    fn test_mandatory_share() {
        let heir = Heir {
            name: "Иванов Иван".to_string(),
            relationship: Relationship::Child,
            birth_date: Some(chrono::NaiveDate::from_ymd_opt(2020, 1, 1).expect("Valid date")),
            is_disabled_or_minor: true,
        };

        let mandatory = calculate_mandatory_share(&heir, 0.5).expect("Should succeed");
        assert_eq!(mandatory, 0.25); // Half of lawful share
    }
}
