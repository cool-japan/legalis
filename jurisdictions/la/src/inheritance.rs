//! Book V: Inheritance (ມໍລະດົກ) - Articles 910-1078
//!
//! This module implements inheritance law provisions of the Lao Civil Code 2020,
//! covering succession, wills, and forced heirship.
//!
//! ## Structure
//! - Chapter 1: General Provisions (Articles 910-940)
//! - Chapter 2: Intestate Succession (Articles 941-980)
//! - Chapter 3: Wills (Articles 981-1030)
//! - Chapter 4: Forced Heirship (Articles 1031-1060)
//! - Chapter 5: Estate Administration (Articles 1061-1078)
//!
//! ## Comparative Law Notes
//! - Based on Japanese inheritance law (相続法) with Lao cultural adaptations
//! - Forced heirship system influenced by French réserve héréditaire

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InheritanceError {
    #[error("Invalid succession: {0}")]
    InvalidSuccession(String),

    #[error("Invalid will: {0}")]
    InvalidWill(String),

    #[error("Forced heirship violation: {0}")]
    ForcedHeirshipViolation(String),
}

pub type Result<T> = std::result::Result<T, InheritanceError>;

/// Order of heirs under intestate succession
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeirOrder {
    /// First order: Children and spouse
    First,
    /// Second order: Parents and spouse
    Second,
    /// Third order: Siblings and spouse
    Third,
    /// Fourth order: Other relatives
    Fourth,
}

/// Article 910: Succession
///
/// Upon death, the estate passes to heirs according to law or will.
///
/// Comparative: Japanese Civil Code Articles 882-1044 (相続),
///              French Code civil Articles 720-892
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Succession {
    pub deceased: String,
    pub date_of_death: DateTime<Utc>,
    pub estate_value: u64,
    pub heirs: Vec<(String, f64)>, // (heir, share)
    pub has_will: bool,
}

/// Article 910: Validates succession
pub fn article910(succession: &Succession) -> Result<()> {
    if succession.deceased.is_empty() {
        return Err(InheritanceError::InvalidSuccession(
            "Deceased must be identified".to_string(),
        ));
    }

    if succession.heirs.is_empty() {
        return Err(InheritanceError::InvalidSuccession(
            "Succession requires at least one heir".to_string(),
        ));
    }

    // Verify shares sum to 1.0
    let total: f64 = succession.heirs.iter().map(|(_, share)| share).sum();
    if (total - 1.0).abs() > 0.001 {
        return Err(InheritanceError::InvalidSuccession(
            "Heir shares must sum to 1.0".to_string(),
        ));
    }

    Ok(())
}

/// Validates succession
pub fn validate_succession(succession: &Succession) -> Result<()> {
    article910(succession)
}

/// Type of will
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WillType {
    /// Holographic will (handwritten)
    Holographic,
    /// Notarial will
    Notarial,
    /// Secret will
    Secret,
}

/// Article 950: Wills
///
/// A person may dispose of their property by will.
///
/// Comparative: Japanese Civil Code Articles 960-1027 (遺言),
///              French Code civil Articles 967-1034
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Will {
    pub testator: String,
    pub testator_age: u32,
    pub will_type: WillType,
    pub dated: bool,
    pub signed: bool,
    pub dispositions: Vec<(String, u64)>, // (beneficiary, amount)
    pub created_at: DateTime<Utc>,
    pub witnessed: bool,
}

/// Article 950: Validates will
///
/// # Requirements
/// - Testator must be at least 18 years old
/// - Will must be dated and signed
/// - Notarial wills require witnesses
pub fn article950(will: &Will) -> Result<()> {
    // Testator must have capacity (18+)
    if will.testator_age < 18 {
        return Err(InheritanceError::InvalidWill(
            "Testator must be at least 18 years old".to_string(),
        ));
    }

    // Will must be dated
    if !will.dated {
        return Err(InheritanceError::InvalidWill(
            "Will must be dated".to_string(),
        ));
    }

    // Will must be signed
    if !will.signed {
        return Err(InheritanceError::InvalidWill(
            "Will must be signed by testator".to_string(),
        ));
    }

    // Notarial wills require witnesses
    if will.will_type == WillType::Notarial && !will.witnessed {
        return Err(InheritanceError::InvalidWill(
            "Notarial will requires witnesses".to_string(),
        ));
    }

    Ok(())
}

/// Validates will
pub fn validate_will(will: &Will) -> Result<()> {
    article950(will)
}

/// Article 1000: Forced Heirship
///
/// Certain heirs are entitled to a reserved portion of the estate.
///
/// Comparative: Japanese Civil Code Article 1042 (遺留分),
///              French Code civil Articles 912-917 (réserve héréditaire)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForcedHeirship {
    pub estate_value: u64,
    pub forced_heirs: Vec<(String, f64)>, // (heir, reserved share)
    pub testamentary_dispositions: u64,
}

/// Article 1000: Validates forced heirship
///
/// # Lao Law
/// - Spouse and children are forced heirs
/// - Reserved portion: 1/2 for one child, 2/3 for two or more children
pub fn article1000(forced: &ForcedHeirship) -> Result<u64> {
    if forced.forced_heirs.is_empty() {
        return Ok(forced.estate_value); // No forced heirs, full freedom
    }

    // Calculate total reserved portion
    let reserved_share: f64 = forced.forced_heirs.iter().map(|(_, share)| share).sum();
    let reserved_amount = (forced.estate_value as f64 * reserved_share) as u64;

    // Available portion for testamentary dispositions
    let available = forced.estate_value.saturating_sub(reserved_amount);

    // Check if testamentary dispositions exceed available portion
    if forced.testamentary_dispositions > available {
        return Err(InheritanceError::ForcedHeirshipViolation(format!(
            "Testamentary dispositions ({}) exceed available portion ({})",
            forced.testamentary_dispositions, available
        )));
    }

    Ok(available)
}

/// Validates forced heirship compliance
pub fn validate_forced_heirship(forced: &ForcedHeirship) -> Result<()> {
    article1000(forced)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article910_succession() {
        let succession = Succession {
            deceased: "Decedent".to_string(),
            date_of_death: Utc::now(),
            estate_value: 100_000_000,
            heirs: vec![
                ("Spouse".to_string(), 0.5),
                ("Child 1".to_string(), 0.25),
                ("Child 2".to_string(), 0.25),
            ],
            has_will: false,
        };
        assert!(article910(&succession).is_ok());

        // Invalid shares (don't sum to 1.0)
        let invalid = Succession {
            heirs: vec![("Spouse".to_string(), 0.5), ("Child 1".to_string(), 0.3)],
            ..succession.clone()
        };
        assert!(article910(&invalid).is_err());
    }

    #[test]
    fn test_article950_will() {
        let will = Will {
            testator: "Testator".to_string(),
            testator_age: 65,
            will_type: WillType::Holographic,
            dated: true,
            signed: true,
            dispositions: vec![
                ("Beneficiary 1".to_string(), 50_000_000),
                ("Beneficiary 2".to_string(), 30_000_000),
            ],
            created_at: Utc::now(),
            witnessed: false,
        };
        assert!(article950(&will).is_ok());

        // Testator too young
        let underage = Will {
            testator_age: 17,
            ..will.clone()
        };
        assert!(article950(&underage).is_err());

        // Not signed
        let unsigned = Will {
            signed: false,
            ..will.clone()
        };
        assert!(article950(&unsigned).is_err());

        // Notarial will without witnesses
        let notarial = Will {
            will_type: WillType::Notarial,
            witnessed: false,
            ..will.clone()
        };
        assert!(article950(&notarial).is_err());
    }

    #[test]
    fn test_article1000_forced_heirship() {
        // Estate of 100M with one child (reserved 1/2 = 50M)
        let forced = ForcedHeirship {
            estate_value: 100_000_000,
            forced_heirs: vec![("Child".to_string(), 0.5)],
            testamentary_dispositions: 40_000_000,
        };
        let available = article1000(&forced).unwrap();
        assert_eq!(available, 50_000_000);

        // Excessive testamentary dispositions
        let excessive = ForcedHeirship {
            testamentary_dispositions: 60_000_000,
            ..forced.clone()
        };
        assert!(article1000(&excessive).is_err());

        // Two children (reserved 2/3)
        let two_children = ForcedHeirship {
            estate_value: 90_000_000,
            forced_heirs: vec![
                ("Child 1".to_string(), 1.0 / 3.0),
                ("Child 2".to_string(), 1.0 / 3.0),
            ],
            testamentary_dispositions: 30_000_000,
        };
        let available = article1000(&two_children).unwrap();
        assert_eq!(available, 30_000_000);
    }
}
