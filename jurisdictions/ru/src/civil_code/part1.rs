//! Civil Code Part 1: General Provisions, Persons, and Property (1994).
//!
//! Federal Law No. 51-FZ of November 30, 1994
//!
//! This part covers:
//! - General provisions (Articles 1-208)
//! - Legal persons and individuals
//! - Objects of civil rights (Article 128-141)
//! - Transactions and representation
//! - Time limits and limitation periods

use serde::{Deserialize, Serialize};

use super::CivilCodeError;

/// Types of property under Russian Civil Code
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyType {
    /// Movable property (движимое имущество)
    Movable,
    /// Immovable property (недвижимое имущество)
    Immovable,
    /// Money (деньги)
    Money,
    /// Securities (ценные бумаги)
    Securities,
    /// Intellectual property (результаты интеллектуальной деятельности)
    IntellectualProperty,
    /// Other property rights
    OtherPropertyRights,
}

/// Property rights under Russian law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyRight {
    /// Ownership right (право собственности)
    Ownership,
    /// Limited property right (ограниченное вещное право)
    LimitedPropertyRight,
    /// Right of use (право пользования)
    UseRight,
    /// Right of possession (право владения)
    Possession,
    /// Easement (сервитут)
    Easement,
    /// Pledge (залог)
    Pledge,
}

/// Legal capacity status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalCapacity {
    /// Full capacity (полная дееспособность) - from 18 years
    Full,
    /// Limited capacity (ограниченная дееспособность) - 14-18 years
    Limited,
    /// Partial capacity (частичная дееспособность) - 6-14 years
    Partial,
    /// No capacity (недееспособность) - under 6 years or declared incapable
    None,
}

/// Article 128: Types of objects of civil rights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article128 {
    /// Type of property
    pub property_type: PropertyType,
    /// Description
    pub description: String,
}

impl Article128 {
    /// Creates a new Article 128 object
    pub fn new(property_type: PropertyType, description: impl Into<String>) -> Self {
        Self {
            property_type,
            description: description.into(),
        }
    }

    /// Validates the property type classification
    pub fn validate(&self) -> Result<(), CivilCodeError> {
        // All property types are valid under Article 128
        Ok(())
    }
}

/// Article 209: Content of ownership right
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article209 {
    /// The owner has the rights to possession, use, and disposition
    pub possession: bool,
    pub use_right: bool,
    pub disposition: bool,
}

impl Article209 {
    /// Creates full ownership rights
    pub fn full_ownership() -> Self {
        Self {
            possession: true,
            use_right: true,
            disposition: true,
        }
    }

    /// Creates limited ownership rights
    pub fn limited_ownership(possession: bool, use_right: bool, disposition: bool) -> Self {
        Self {
            possession,
            use_right,
            disposition,
        }
    }

    /// Checks if this represents full ownership
    pub fn is_full_ownership(&self) -> bool {
        self.possession && self.use_right && self.disposition
    }
}

/// Legal person representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalPerson {
    /// Name of the legal entity
    pub name: String,
    /// Registration number (OGRN)
    pub ogrn: String,
    /// Tax identification number (INN)
    pub inn: String,
    /// Legal address
    pub legal_address: String,
    /// Type of legal entity
    pub entity_type: LegalEntityType,
}

/// Types of legal entities in Russia
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalEntityType {
    /// Limited Liability Company (ООО)
    LLC,
    /// Joint Stock Company (АО)
    JSC,
    /// Public Joint Stock Company (ПАО)
    PublicJSC,
    /// Partnership (товарищество)
    Partnership,
    /// Cooperative (кооператив)
    Cooperative,
    /// Unitary Enterprise (унитарное предприятие)
    UnitaryEnterprise,
    /// Non-profit organization (некоммерческая организация)
    NonProfit,
}

/// Individual person (physical person)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Individual {
    /// Full name
    pub full_name: String,
    /// Birth date
    pub birth_date: chrono::NaiveDate,
    /// Legal capacity
    pub capacity: LegalCapacity,
    /// Passport or ID
    pub identification: String,
}

impl Individual {
    /// Determines legal capacity based on age
    pub fn determine_capacity(birth_date: &chrono::NaiveDate) -> LegalCapacity {
        let today = chrono::Local::now().naive_local().date();
        let age = today.years_since(*birth_date).unwrap_or(0);

        match age {
            0..=5 => LegalCapacity::None,
            6..=13 => LegalCapacity::Partial,
            14..=17 => LegalCapacity::Limited,
            _ => LegalCapacity::Full,
        }
    }
}

/// Validates property rights for a given property type
pub fn validate_property_right(
    property_type: &PropertyType,
    right: &PropertyRight,
) -> Result<(), CivilCodeError> {
    match (property_type, right) {
        // Ownership is valid for all property types
        (_, PropertyRight::Ownership) => Ok(()),

        // Intellectual property has special rules
        (PropertyType::IntellectualProperty, PropertyRight::LimitedPropertyRight) => {
            Err(CivilCodeError::InvalidPropertyRight(
                "Limited property rights do not apply to intellectual property".to_string(),
            ))
        }

        // Money and securities cannot have easements
        (PropertyType::Money | PropertyType::Securities, PropertyRight::Easement) => {
            Err(CivilCodeError::InvalidPropertyRight(
                "Easements cannot apply to money or securities".to_string(),
            ))
        }

        // All other combinations are valid
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_128() {
        let obj = Article128::new(PropertyType::Immovable, "Apartment in Moscow");
        assert_eq!(obj.property_type, PropertyType::Immovable);
        assert!(obj.validate().is_ok());
    }

    #[test]
    fn test_article_209() {
        let full = Article209::full_ownership();
        assert!(full.is_full_ownership());

        let limited = Article209::limited_ownership(true, true, false);
        assert!(!limited.is_full_ownership());
    }

    #[test]
    fn test_legal_capacity() {
        let birth_2020 = chrono::NaiveDate::from_ymd_opt(2020, 1, 1).expect("Valid date");
        let capacity = Individual::determine_capacity(&birth_2020);
        assert_eq!(capacity, LegalCapacity::Partial);

        let birth_1990 = chrono::NaiveDate::from_ymd_opt(1990, 1, 1).expect("Valid date");
        let capacity = Individual::determine_capacity(&birth_1990);
        assert_eq!(capacity, LegalCapacity::Full);
    }

    #[test]
    fn test_property_right_validation() {
        // Valid: Ownership of immovable property
        assert!(
            validate_property_right(&PropertyType::Immovable, &PropertyRight::Ownership).is_ok()
        );

        // Invalid: Easement on money
        assert!(validate_property_right(&PropertyType::Money, &PropertyRight::Easement).is_err());

        // Invalid: Limited property right on IP
        assert!(
            validate_property_right(
                &PropertyType::IntellectualProperty,
                &PropertyRight::LimitedPropertyRight
            )
            .is_err()
        );
    }
}
