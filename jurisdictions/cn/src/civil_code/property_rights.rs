//! Book II: Property Rights (物权编)
//!
//! Articles 205-462 of the Civil Code
//!
//! Covers:
//! - Ownership (所有权)
//! - Usufruct (用益物权)
//! - Security interests (担保物权)
//! - Possession (占有)

use crate::i18n::BilingualText;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Types
// ============================================================================

/// Types of property rights (物权类型)
///
/// Article 114-116
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyRightType {
    /// Ownership (所有权)
    Ownership,
    /// Usufruct (用益物权)
    Usufruct,
    /// Security interest (担保物权)
    SecurityInterest,
}

/// Ownership type (所有权类型)
///
/// Articles 240-267
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OwnershipType {
    /// State ownership (国家所有权) - Article 247
    State,
    /// Collective ownership (集体所有权) - Article 260
    Collective,
    /// Private ownership (私人所有权) - Article 266
    Private,
}

impl OwnershipType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::State => BilingualText::new("国家所有权", "State ownership"),
            Self::Collective => BilingualText::new("集体所有权", "Collective ownership"),
            Self::Private => BilingualText::new("私人所有权", "Private ownership"),
        }
    }
}

/// Property (物)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    /// Description
    pub description: BilingualText,
    /// Owner
    pub owner: String,
    /// Ownership type
    pub ownership_type: OwnershipType,
    /// Location (if real property)
    pub location: Option<String>,
    /// Is movable property (动产)
    pub is_movable: bool,
}

/// Construction land use right (建设用地使用权)
///
/// Articles 347-363
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionLandUseRight {
    /// Land description
    pub land_description: BilingualText,
    /// Right holder
    pub right_holder: String,
    /// Location
    pub location: String,
    /// Area (square meters)
    pub area_sqm: f64,
    /// Permitted use (用途)
    pub permitted_use: BilingualText,
    /// Term of use (years)
    pub term_years: u32,
    /// Start date
    pub start_date: DateTime<Utc>,
    /// Is transferable
    pub is_transferable: bool,
}

impl ConstructionLandUseRight {
    /// Get expiration date
    pub fn expiration_date(&self) -> DateTime<Utc> {
        self.start_date + chrono::Duration::days(365 * i64::from(self.term_years))
    }

    /// Check if expired
    pub fn is_expired(&self, current_date: DateTime<Utc>) -> bool {
        current_date > self.expiration_date()
    }
}

/// Standard term for construction land use rights
///
/// Based on Article 359 and implementing regulations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstructionLandTerm {
    /// Residential (住宅用地) - 70 years
    Residential,
    /// Industrial (工业用地) - 50 years
    Industrial,
    /// Education/Science/Culture/Health/Sports (教科文卫体用地) - 50 years
    PublicService,
    /// Commercial/Tourism/Entertainment (商业旅游娱乐用地) - 40 years
    Commercial,
    /// Comprehensive or other (综合或其他用地) - 50 years
    Comprehensive,
}

impl ConstructionLandTerm {
    /// Get standard term in years
    pub fn years(&self) -> u32 {
        match self {
            Self::Residential => 70,
            Self::Industrial | Self::PublicService | Self::Comprehensive => 50,
            Self::Commercial => 40,
        }
    }

    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Residential => BilingualText::new("住宅用地", "Residential land"),
            Self::Industrial => BilingualText::new("工业用地", "Industrial land"),
            Self::PublicService => BilingualText::new("教科文卫体用地", "Public service land"),
            Self::Commercial => BilingualText::new("商业旅游娱乐用地", "Commercial/tourism land"),
            Self::Comprehensive => BilingualText::new("综合或其他用地", "Comprehensive land"),
        }
    }
}

/// Residential land use right (宅基地使用权)
///
/// Articles 362-364
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResidentialLandUseRight {
    /// Right holder (must be member of rural collective)
    pub right_holder: String,
    /// Location
    pub location: String,
    /// Area (square meters)
    pub area_sqm: f64,
    /// Is transferable (generally not, except specific cases)
    pub is_transferable: bool,
}

/// Land contractual management right (土地承包经营权)
///
/// Articles 330-346
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandContractualManagementRight {
    /// Right holder (承包人)
    pub right_holder: String,
    /// Contractor (发包人)
    pub contractor: String,
    /// Land description
    pub land_description: BilingualText,
    /// Location
    pub location: String,
    /// Area (square meters or mu)
    pub area: f64,
    /// Contract term (years)
    pub term_years: u32,
    /// Start date
    pub start_date: DateTime<Utc>,
}

/// Mortgage (抵押权)
///
/// Articles 394-419
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mortgage {
    /// Mortgagor (抵押人)
    pub mortgagor: String,
    /// Mortgagee (抵押权人)
    pub mortgagee: String,
    /// Mortgaged property description
    pub property: BilingualText,
    /// Secured debt amount
    pub secured_debt_amount: f64,
    /// Currency
    pub currency: String,
    /// Registration date (if registered)
    pub registration_date: Option<DateTime<Utc>>,
    /// Is registered
    pub is_registered: bool,
}

impl Mortgage {
    /// Check if mortgage takes effect
    ///
    /// Article 402: Mortgage on real property takes effect upon registration
    pub fn is_effective(&self, property_is_real: bool) -> bool {
        if property_is_real {
            self.is_registered
        } else {
            // For movable property, effective upon creation (Article 403)
            true
        }
    }
}

/// Pledge (质权)
///
/// Articles 425-447
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pledge {
    /// Pledgor (出质人)
    pub pledgor: String,
    /// Pledgee (质权人)
    pub pledgee: String,
    /// Pledged property description
    pub property: BilingualText,
    /// Secured debt amount
    pub secured_debt_amount: f64,
    /// Currency
    pub currency: String,
    /// Delivery date
    pub delivery_date: DateTime<Utc>,
}

impl Pledge {
    /// Check if pledge takes effect
    ///
    /// Article 429: Pledge takes effect upon delivery of pledged property
    pub fn is_effective(&self) -> bool {
        // Effective upon delivery (已交付)
        true
    }
}

/// Lien (留置权)
///
/// Articles 447-457
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lien {
    /// Lien holder (留置权人)
    pub lien_holder: String,
    /// Debtor
    pub debtor: String,
    /// Retained property description
    pub property: BilingualText,
    /// Debt amount
    pub debt_amount: f64,
    /// Currency
    pub currency: String,
    /// Grace period given (days)
    pub grace_period_days: u32,
}

// ============================================================================
// Validators
// ============================================================================

/// Validate construction land use right
///
/// Articles 347-363
pub fn validate_construction_land_use_right(
    right: &ConstructionLandUseRight,
) -> Result<(), PropertyRightsError> {
    // Check term is reasonable
    if right.term_years == 0 || right.term_years > 70 {
        return Err(PropertyRightsError::InvalidLandUseTerm {
            term: right.term_years,
            max_term: 70,
        });
    }

    // Check area is positive
    if right.area_sqm <= 0.0 {
        return Err(PropertyRightsError::InvalidArea {
            area: right.area_sqm,
        });
    }

    Ok(())
}

/// Validate mortgage
///
/// Articles 394-419
pub fn validate_mortgage(
    mortgage: &Mortgage,
    property_is_real: bool,
) -> Result<(), PropertyRightsError> {
    // Article 402: Real property mortgage must be registered
    if property_is_real && !mortgage.is_registered {
        return Err(PropertyRightsError::MortgageNotRegistered {
            property: mortgage.property.clone(),
        });
    }

    // Check debt amount is positive
    if mortgage.secured_debt_amount <= 0.0 {
        return Err(PropertyRightsError::InvalidDebtAmount {
            amount: mortgage.secured_debt_amount,
        });
    }

    Ok(())
}

/// Validate pledge
///
/// Articles 425-447
pub fn validate_pledge(pledge: &Pledge) -> Result<(), PropertyRightsError> {
    // Check debt amount is positive
    if pledge.secured_debt_amount <= 0.0 {
        return Err(PropertyRightsError::InvalidDebtAmount {
            amount: pledge.secured_debt_amount,
        });
    }

    Ok(())
}

/// Calculate priority among security interests
///
/// Articles 414, 415
pub fn calculate_security_interest_priority(mortgages: &[Mortgage]) -> Vec<(usize, BilingualText)> {
    let mut priority_list = Vec::new();

    for (idx, mortgage) in mortgages.iter().enumerate() {
        let priority_text = if let Some(reg_date) = mortgage.registration_date {
            BilingualText::new(
                format!(
                    "第{}顺位 (登记日期: {})",
                    idx + 1,
                    reg_date.format("%Y-%m-%d")
                ),
                format!(
                    "Priority {} (Registration: {})",
                    idx + 1,
                    reg_date.format("%Y-%m-%d")
                ),
            )
        } else {
            BilingualText::new(
                "未登记 (无优先权)".to_string(),
                "Unregistered (no priority)".to_string(),
            )
        };
        priority_list.push((idx, priority_text));
    }

    // Sort by registration date
    priority_list.sort_by(|a, b| {
        let date_a = mortgages[a.0].registration_date;
        let date_b = mortgages[b.0].registration_date;
        date_a.cmp(&date_b)
    });

    priority_list
}

// ============================================================================
// Errors
// ============================================================================

/// Errors for Property Rights
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum PropertyRightsError {
    /// Invalid land use term
    #[error("Invalid land use term: {term} years (max: {max_term})")]
    InvalidLandUseTerm {
        /// Term in years
        term: u32,
        /// Maximum allowed term
        max_term: u32,
    },

    /// Invalid area
    #[error("Invalid area: {area} sqm")]
    InvalidArea {
        /// Area
        area: f64,
    },

    /// Mortgage not registered
    #[error("Real property mortgage must be registered: {property}")]
    MortgageNotRegistered {
        /// Property description
        property: BilingualText,
    },

    /// Invalid debt amount
    #[error("Invalid debt amount: {amount}")]
    InvalidDebtAmount {
        /// Debt amount
        amount: f64,
    },

    /// Property right conflict
    #[error("Property right conflict: {description}")]
    PropertyRightConflict {
        /// Description
        description: BilingualText,
    },
}

/// Result type for Property Rights operations
pub type PropertyRightsResult<T> = Result<T, PropertyRightsError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction_land_term() {
        assert_eq!(ConstructionLandTerm::Residential.years(), 70);
        assert_eq!(ConstructionLandTerm::Industrial.years(), 50);
        assert_eq!(ConstructionLandTerm::Commercial.years(), 40);
    }

    #[test]
    fn test_construction_land_use_right() {
        let right = ConstructionLandUseRight {
            land_description: BilingualText::new("住宅用地", "Residential land"),
            right_holder: "张三".to_string(),
            location: "北京市朝阳区".to_string(),
            area_sqm: 1000.0,
            permitted_use: BilingualText::new("住宅", "Residential"),
            term_years: 70,
            start_date: Utc::now(),
            is_transferable: true,
        };

        assert!(validate_construction_land_use_right(&right).is_ok());
    }

    #[test]
    fn test_invalid_land_use_term() {
        let right = ConstructionLandUseRight {
            land_description: BilingualText::new("住宅用地", "Residential land"),
            right_holder: "张三".to_string(),
            location: "北京市朝阳区".to_string(),
            area_sqm: 1000.0,
            permitted_use: BilingualText::new("住宅", "Residential"),
            term_years: 100, // Invalid: exceeds 70 years
            start_date: Utc::now(),
            is_transferable: true,
        };

        assert!(validate_construction_land_use_right(&right).is_err());
    }

    #[test]
    fn test_mortgage_registration() {
        let mortgage = Mortgage {
            mortgagor: "借款人".to_string(),
            mortgagee: "银行".to_string(),
            property: BilingualText::new("房产", "Real property"),
            secured_debt_amount: 1_000_000.0,
            currency: "CNY".to_string(),
            registration_date: Some(Utc::now()),
            is_registered: true,
        };

        // Real property mortgage must be registered
        assert!(validate_mortgage(&mortgage, true).is_ok());
        assert!(mortgage.is_effective(true));
    }

    #[test]
    fn test_unregistered_real_property_mortgage() {
        let mortgage = Mortgage {
            mortgagor: "借款人".to_string(),
            mortgagee: "银行".to_string(),
            property: BilingualText::new("房产", "Real property"),
            secured_debt_amount: 1_000_000.0,
            currency: "CNY".to_string(),
            registration_date: None,
            is_registered: false,
        };

        // Should fail validation for real property
        assert!(validate_mortgage(&mortgage, true).is_err());
        assert!(!mortgage.is_effective(true));
    }

    #[test]
    fn test_pledge_effectiveness() {
        let pledge = Pledge {
            pledgor: "借款人".to_string(),
            pledgee: "贷款人".to_string(),
            property: BilingualText::new("珠宝", "Jewelry"),
            secured_debt_amount: 100_000.0,
            currency: "CNY".to_string(),
            delivery_date: Utc::now(),
        };

        assert!(pledge.is_effective());
        assert!(validate_pledge(&pledge).is_ok());
    }
}
