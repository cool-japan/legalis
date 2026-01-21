//! Book II: Property (ຊັບສິນ) - Articles 162-431
//!
//! This module implements property law provisions of the Lao Civil Code 2020,
//! covering real rights, ownership, possession, co-ownership, and servitudes.
//!
//! ## Structure
//! - Chapter 1: General Provisions on Property (Articles 162-180)
//! - Chapter 2: Ownership (Articles 181-250)
//! - Chapter 3: Possession (Articles 251-280)
//! - Chapter 4: Co-ownership (Articles 281-320)
//! - Chapter 5: Servitudes (Articles 321-380)
//! - Chapter 6: Security Rights (Articles 381-431)
//!
//! ## Comparative Law Notes
//! - Based on Japanese property law (物権法) with French droit réel influences
//! - Ownership structure follows Japanese Civil Code Book II
//! - Servitudes adapted from French Code civil Book II

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error types for property law
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PropertyError {
    #[error("Invalid ownership: {0}")]
    InvalidOwnership(String),

    #[error("Possession conflict: {0}")]
    PossessionConflict(String),

    #[error("Invalid property transaction: {0}")]
    InvalidTransaction(String),

    #[error("Servitude violation: {0}")]
    ServitudeViolation(String),

    #[error("Security right error: {0}")]
    SecurityRightError(String),
}

pub type Result<T> = std::result::Result<T, PropertyError>;

/// Type of property
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyType {
    /// Immovable property (land, buildings)
    Immovable,
    /// Movable property (chattels)
    Movable,
}

/// Article 162: Classification of Property
///
/// Property is classified as immovable or movable.
///
/// Comparative: Japanese Civil Code Article 86, French Code civil Article 516
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub id: String,
    pub property_type: PropertyType,
    pub description: String,
    pub location: Option<String>,
    pub value: Option<u64>,
}

/// Article 162: Validates property classification
pub fn article162(property: &Property) -> Result<PropertyType> {
    match property.property_type {
        PropertyType::Immovable => {
            if property.location.is_none() {
                return Err(PropertyError::InvalidOwnership(
                    "Immovable property must have location".to_string(),
                ));
            }
            Ok(PropertyType::Immovable)
        }
        PropertyType::Movable => Ok(PropertyType::Movable),
    }
}

/// Type of ownership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OwnershipType {
    /// Full ownership
    Full,
    /// Co-ownership
    CoOwnership { shares: Vec<(String, f64)> },
    /// Conditional ownership
    Conditional { condition: String },
}

/// Article 181-250: Ownership (ກຳມະສິດ)
///
/// Ownership is the right to use, enjoy, and dispose of property within the limits of law.
///
/// Comparative: Japanese Civil Code Articles 206-208 (所有権), French Code civil Article 544
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ownership {
    pub property: Property,
    pub owner: String,
    pub ownership_type: OwnershipType,
    pub acquired_at: DateTime<Utc>,
    pub restrictions: Vec<String>,
}

/// Article 163: Content of Ownership
///
/// The owner has the right to freely use, enjoy, and dispose of property
/// within the limits provided by law.
///
/// # Japanese Influence
/// Based on Japanese Civil Code Article 206: "所有者は、法令の制限内において、
/// 自由にその所有物の使用、収益及び処分をする権利を有する"
pub fn article163(ownership: &Ownership) -> Result<()> {
    // Verify ownership is properly established
    if ownership.owner.is_empty() {
        return Err(PropertyError::InvalidOwnership(
            "Owner must be specified".to_string(),
        ));
    }

    // Check co-ownership shares sum to 1.0
    if let OwnershipType::CoOwnership { shares } = &ownership.ownership_type {
        let total: f64 = shares.iter().map(|(_, share)| share).sum();
        if (total - 1.0).abs() > 0.001 {
            return Err(PropertyError::InvalidOwnership(
                "Co-ownership shares must sum to 1.0".to_string(),
            ));
        }
    }

    Ok(())
}

/// Validates ownership claim
pub fn validate_ownership(ownership: &Ownership) -> Result<()> {
    article162(&ownership.property)?;
    article163(ownership)?;
    Ok(())
}

/// Article 200: Acquisition by Accession
///
/// The owner of property acquires ownership of natural fruits and products.
///
/// Comparative: Japanese Civil Code Article 89, French Code civil Article 546
pub fn article200(ownership: &Ownership, fruits_value: u64) -> Result<u64> {
    // Natural fruits belong to owner
    if ownership.owner.is_empty() {
        return Err(PropertyError::InvalidOwnership(
            "Cannot determine owner for fruits".to_string(),
        ));
    }

    // In co-ownership, fruits divided by shares
    if let OwnershipType::CoOwnership { shares } = &ownership.ownership_type {
        // Return owner's proportional share
        if let Some((_, share)) = shares.first() {
            Ok((fruits_value as f64 * share) as u64)
        } else {
            Err(PropertyError::InvalidOwnership(
                "Invalid co-ownership structure".to_string(),
            ))
        }
    } else {
        Ok(fruits_value)
    }
}

/// Type of possession
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PossessionType {
    /// Possession with intent to own
    WithIntentToOwn,
    /// Possession without intent to own (e.g., lessee)
    WithoutIntentToOwn,
}

/// Article 251-280: Possession (ການຄອບຄອງ)
///
/// Possession is the factual control of property with or without intent to own.
///
/// Comparative: Japanese Civil Code Articles 180-205 (占有権), French Code civil Articles 2255-2279
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Possession {
    pub property: Property,
    pub possessor: String,
    pub possession_type: PossessionType,
    pub started_at: DateTime<Utc>,
    pub good_faith: bool,
    pub peaceful: bool,
    pub public: bool,
}

/// Validates possession claim
pub fn validate_possession(possession: &Possession) -> Result<()> {
    if possession.possessor.is_empty() {
        return Err(PropertyError::PossessionConflict(
            "Possessor must be specified".to_string(),
        ));
    }

    // Good faith possession requires peaceful, public possession
    if possession.good_faith && (!possession.peaceful || !possession.public) {
        return Err(PropertyError::PossessionConflict(
            "Good faith possession must be peaceful and public".to_string(),
        ));
    }

    Ok(())
}

/// Type of servitude
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServitudeType {
    /// Right of way
    RightOfWay { width_meters: f64 },
    /// Right to draw water
    WaterRight,
    /// Right to light and air
    LightAndAir,
    /// Other servitude
    Other(String),
}

/// Article 321-380: Servitudes (ພັນທະບໍລິການທີ່ດິນ)
///
/// A servitude is a burden on property for the benefit of another property.
///
/// Comparative: Japanese Civil Code Articles 280-294 (地役権), French Code civil Articles 637-710
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Servitude {
    pub dominant_property: Property,
    pub servient_property: Property,
    pub servitude_type: ServitudeType,
    pub created_at: DateTime<Utc>,
    pub perpetual: bool,
}

/// Validates servitude establishment
pub fn validate_servitude(servitude: &Servitude) -> Result<()> {
    // Dominant and servient properties must be different
    if servitude.dominant_property.id == servitude.servient_property.id {
        return Err(PropertyError::ServitudeViolation(
            "Cannot establish servitude on same property".to_string(),
        ));
    }

    // Both properties must be immovable for typical servitudes
    if servitude.dominant_property.property_type != PropertyType::Immovable
        || servitude.servient_property.property_type != PropertyType::Immovable
    {
        return Err(PropertyError::ServitudeViolation(
            "Servitudes typically require immovable property".to_string(),
        ));
    }

    Ok(())
}

/// Type of security right
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityRightType {
    /// Mortgage on immovable property
    Mortgage { amount: u64 },
    /// Pledge on movable property
    Pledge { amount: u64 },
    /// Lien
    Lien { amount: u64 },
}

/// Article 381-431: Security Rights
///
/// Security rights provide creditors with priority in satisfaction from property.
///
/// Comparative: Japanese Civil Code Articles 342-398 (担保物権)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRight {
    pub property: Property,
    pub creditor: String,
    pub debtor: String,
    pub security_type: SecurityRightType,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Validates security right establishment
pub fn validate_security_right(security: &SecurityRight) -> Result<()> {
    if security.creditor.is_empty() || security.debtor.is_empty() {
        return Err(PropertyError::SecurityRightError(
            "Creditor and debtor must be specified".to_string(),
        ));
    }

    // Mortgage requires immovable property
    if let SecurityRightType::Mortgage { .. } = security.security_type
        && security.property.property_type != PropertyType::Immovable
    {
        return Err(PropertyError::SecurityRightError(
            "Mortgage requires immovable property".to_string(),
        ));
    }

    // Pledge requires movable property
    if let SecurityRightType::Pledge { .. } = security.security_type
        && security.property.property_type != PropertyType::Movable
    {
        return Err(PropertyError::SecurityRightError(
            "Pledge requires movable property".to_string(),
        ));
    }

    Ok(())
}

/// Type of property transaction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Sale,
    Gift,
    Exchange,
    Lease,
}

/// Property transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyTransaction {
    pub property: Property,
    pub from: String,
    pub to: String,
    pub transaction_type: TransactionType,
    pub consideration: Option<u64>,
    pub executed_at: DateTime<Utc>,
}

/// Validates property transaction
pub fn validate_property_transaction(transaction: &PropertyTransaction) -> Result<()> {
    if transaction.from.is_empty() || transaction.to.is_empty() {
        return Err(PropertyError::InvalidTransaction(
            "Parties must be specified".to_string(),
        ));
    }

    // Sale requires consideration
    if transaction.transaction_type == TransactionType::Sale && transaction.consideration.is_none()
    {
        return Err(PropertyError::InvalidTransaction(
            "Sale requires consideration".to_string(),
        ));
    }

    // Gift must have no consideration or nominal consideration
    if transaction.transaction_type == TransactionType::Gift
        && let Some(consideration) = transaction.consideration
        && consideration > 0
    {
        return Err(PropertyError::InvalidTransaction(
            "Gift should not have substantial consideration".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article162_property_classification() {
        let land = Property {
            id: "LAND001".to_string(),
            property_type: PropertyType::Immovable,
            description: "Agricultural land".to_string(),
            location: Some("Vientiane Province".to_string()),
            value: Some(100_000_000),
        };
        assert_eq!(article162(&land).unwrap(), PropertyType::Immovable);

        let car = Property {
            id: "CAR001".to_string(),
            property_type: PropertyType::Movable,
            description: "Toyota sedan".to_string(),
            location: None,
            value: Some(50_000_000),
        };
        assert_eq!(article162(&car).unwrap(), PropertyType::Movable);
    }

    #[test]
    fn test_article163_ownership() {
        let ownership = Ownership {
            property: Property {
                id: "LAND001".to_string(),
                property_type: PropertyType::Immovable,
                description: "Land".to_string(),
                location: Some("Vientiane".to_string()),
                value: Some(100_000_000),
            },
            owner: "John Doe".to_string(),
            ownership_type: OwnershipType::Full,
            acquired_at: Utc::now(),
            restrictions: vec![],
        };
        assert!(article163(&ownership).is_ok());

        // Co-ownership with correct shares
        let co_ownership = Ownership {
            ownership_type: OwnershipType::CoOwnership {
                shares: vec![("Owner A".to_string(), 0.6), ("Owner B".to_string(), 0.4)],
            },
            ..ownership.clone()
        };
        assert!(article163(&co_ownership).is_ok());

        // Co-ownership with incorrect shares
        let bad_co_ownership = Ownership {
            ownership_type: OwnershipType::CoOwnership {
                shares: vec![("Owner A".to_string(), 0.6), ("Owner B".to_string(), 0.5)],
            },
            ..ownership.clone()
        };
        assert!(article163(&bad_co_ownership).is_err());
    }

    #[test]
    fn test_article200_accession() {
        let ownership = Ownership {
            property: Property {
                id: "FARM001".to_string(),
                property_type: PropertyType::Immovable,
                description: "Farm".to_string(),
                location: Some("Luang Prabang".to_string()),
                value: Some(200_000_000),
            },
            owner: "Farmer".to_string(),
            ownership_type: OwnershipType::Full,
            acquired_at: Utc::now(),
            restrictions: vec![],
        };

        // Full ownership receives all fruits
        let fruits = article200(&ownership, 10_000_000).unwrap();
        assert_eq!(fruits, 10_000_000);

        // Co-ownership divides fruits
        let co_ownership = Ownership {
            ownership_type: OwnershipType::CoOwnership {
                shares: vec![("Owner A".to_string(), 0.7), ("Owner B".to_string(), 0.3)],
            },
            ..ownership.clone()
        };
        let fruits = article200(&co_ownership, 10_000_000).unwrap();
        assert_eq!(fruits, 7_000_000); // Owner A's share
    }

    #[test]
    fn test_validate_possession() {
        let possession = Possession {
            property: Property {
                id: "HOUSE001".to_string(),
                property_type: PropertyType::Immovable,
                description: "House".to_string(),
                location: Some("Vientiane".to_string()),
                value: Some(80_000_000),
            },
            possessor: "Tenant".to_string(),
            possession_type: PossessionType::WithoutIntentToOwn,
            started_at: Utc::now(),
            good_faith: false,
            peaceful: true,
            public: true,
        };
        assert!(validate_possession(&possession).is_ok());

        // Good faith requires peaceful and public
        let good_faith_possession = Possession {
            good_faith: true,
            ..possession.clone()
        };
        assert!(validate_possession(&good_faith_possession).is_ok());

        let bad_possession = Possession {
            good_faith: true,
            peaceful: false,
            ..possession.clone()
        };
        assert!(validate_possession(&bad_possession).is_err());
    }

    #[test]
    fn test_validate_servitude() {
        let servitude = Servitude {
            dominant_property: Property {
                id: "LAND001".to_string(),
                property_type: PropertyType::Immovable,
                description: "Landlocked property".to_string(),
                location: Some("Vientiane".to_string()),
                value: Some(50_000_000),
            },
            servient_property: Property {
                id: "LAND002".to_string(),
                property_type: PropertyType::Immovable,
                description: "Adjacent property with road access".to_string(),
                location: Some("Vientiane".to_string()),
                value: Some(60_000_000),
            },
            servitude_type: ServitudeType::RightOfWay { width_meters: 3.0 },
            created_at: Utc::now(),
            perpetual: true,
        };
        assert!(validate_servitude(&servitude).is_ok());
    }

    #[test]
    fn test_validate_security_right() {
        let mortgage = SecurityRight {
            property: Property {
                id: "HOUSE001".to_string(),
                property_type: PropertyType::Immovable,
                description: "House".to_string(),
                location: Some("Vientiane".to_string()),
                value: Some(150_000_000),
            },
            creditor: "Bank".to_string(),
            debtor: "Borrower".to_string(),
            security_type: SecurityRightType::Mortgage {
                amount: 100_000_000,
            },
            created_at: Utc::now(),
            expires_at: None,
        };
        assert!(validate_security_right(&mortgage).is_ok());

        // Pledge requires movable property
        let invalid_pledge = SecurityRight {
            security_type: SecurityRightType::Pledge { amount: 10_000_000 },
            ..mortgage.clone()
        };
        assert!(validate_security_right(&invalid_pledge).is_err());
    }

    #[test]
    fn test_validate_property_transaction() {
        let sale = PropertyTransaction {
            property: Property {
                id: "LAND001".to_string(),
                property_type: PropertyType::Immovable,
                description: "Land".to_string(),
                location: Some("Vientiane".to_string()),
                value: Some(100_000_000),
            },
            from: "Seller".to_string(),
            to: "Buyer".to_string(),
            transaction_type: TransactionType::Sale,
            consideration: Some(100_000_000),
            executed_at: Utc::now(),
        };
        assert!(validate_property_transaction(&sale).is_ok());

        // Sale without consideration is invalid
        let invalid_sale = PropertyTransaction {
            consideration: None,
            ..sale.clone()
        };
        assert!(validate_property_transaction(&invalid_sale).is_err());

        // Gift should have no consideration
        let gift = PropertyTransaction {
            transaction_type: TransactionType::Gift,
            consideration: Some(0),
            ..sale.clone()
        };
        assert!(validate_property_transaction(&gift).is_ok());
    }
}
