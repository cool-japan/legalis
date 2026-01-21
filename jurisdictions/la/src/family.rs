//! Book IV: Family Law (ກົດໝາຍຄອບຄົວ) - Articles 673-909
//!
//! This module implements family law provisions of the Lao Civil Code 2020,
//! covering marriage, divorce, parent-child relations, and adoption.
//!
//! ## Structure
//! - Chapter 1: Marriage (Articles 673-730)
//! - Chapter 2: Divorce (Articles 731-780)
//! - Chapter 3: Parent-Child Relations (Articles 781-850)
//! - Chapter 4: Adoption (Articles 851-890)
//! - Chapter 5: Guardianship (Articles 891-909)
//!
//! ## Comparative Law Notes
//! - Influenced by Japanese family law with Lao cultural adaptations
//! - Marriage age and formalities adapted to Lao context

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum FamilyLawError {
    #[error("Invalid marriage: {0}")]
    InvalidMarriage(String),

    #[error("Invalid divorce: {0}")]
    InvalidDivorce(String),

    #[error("Adoption error: {0}")]
    AdoptionError(String),
}

pub type Result<T> = std::result::Result<T, FamilyLawError>;

/// Article 673: Marriage Requirements
///
/// Marriage requires mutual consent of both parties and compliance with legal requirements.
///
/// Comparative: Japanese Civil Code Articles 731-739 (婚姻), French Code civil Articles 143-147
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marriage {
    pub spouse1: String,
    pub spouse2: String,
    pub spouse1_age: u32,
    pub spouse2_age: u32,
    pub mutual_consent: bool,
    pub registered: bool,
    pub marriage_date: DateTime<Utc>,
}

/// Article 673: Validates marriage requirements
///
/// # Lao Law Requirements
/// - Minimum age: 18 for both spouses
/// - Mutual consent required
/// - Registration with authorities
pub fn article673(marriage: &Marriage) -> Result<()> {
    // Minimum age requirement (18 years)
    if marriage.spouse1_age < 18 || marriage.spouse2_age < 18 {
        return Err(FamilyLawError::InvalidMarriage(
            "Both spouses must be at least 18 years old".to_string(),
        ));
    }

    // Mutual consent required
    if !marriage.mutual_consent {
        return Err(FamilyLawError::InvalidMarriage(
            "Marriage requires mutual consent".to_string(),
        ));
    }

    // Must be registered
    if !marriage.registered {
        return Err(FamilyLawError::InvalidMarriage(
            "Marriage must be registered with authorities".to_string(),
        ));
    }

    Ok(())
}

/// Validates marriage
pub fn validate_marriage(marriage: &Marriage) -> Result<()> {
    article673(marriage)
}

/// Type of divorce
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivorceType {
    /// Divorce by mutual consent
    MutualConsent,
    /// Divorce by judicial decision
    Judicial,
}

/// Article 700: Divorce
///
/// Divorce may be granted by mutual consent or by judicial decision.
///
/// Comparative: Japanese Civil Code Articles 763-771 (離婚), French Code civil Articles 229-247
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Divorce {
    pub marriage: Marriage,
    pub divorce_type: DivorceType,
    pub grounds: Option<String>,
    pub filed_date: DateTime<Utc>,
    pub approved: bool,
}

/// Article 700: Validates divorce
pub fn article700(divorce: &Divorce) -> Result<()> {
    // Marriage must be valid
    validate_marriage(&divorce.marriage)?;

    // Judicial divorce requires grounds
    if divorce.divorce_type == DivorceType::Judicial && divorce.grounds.is_none() {
        return Err(FamilyLawError::InvalidDivorce(
            "Judicial divorce requires grounds".to_string(),
        ));
    }

    Ok(())
}

/// Validates divorce
pub fn validate_divorce(divorce: &Divorce) -> Result<()> {
    article700(divorce)
}

/// Article 800: Parent-Child Relations
///
/// Parents have duty to care for and educate their children.
///
/// Comparative: Japanese Civil Code Articles 818-837 (親権)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentChild {
    pub parent: String,
    pub child: String,
    pub biological: bool,
    pub parental_authority: bool,
    pub duty_of_care: bool,
    pub duty_of_education: bool,
}

/// Type of adoption
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdoptionType {
    /// Full adoption (severs ties with biological parents)
    Full,
    /// Simple adoption (maintains ties with biological parents)
    Simple,
}

/// Article 851: Adoption
///
/// Adoption creates a parent-child relationship between adopter and adoptee.
///
/// Comparative: Japanese Civil Code Articles 792-817 (養子縁組),
///              French Code civil Articles 343-370
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adoption {
    pub adopter: String,
    pub adoptee: String,
    pub adoptee_age: u32,
    pub adoption_type: AdoptionType,
    pub consent_given: bool,
    pub court_approved: bool,
    pub adoption_date: DateTime<Utc>,
}

/// Validates adoption
pub fn validate_adoption(adoption: &Adoption) -> Result<()> {
    // Adoptee must be minor (under 18)
    if adoption.adoptee_age >= 18 {
        return Err(FamilyLawError::AdoptionError(
            "Adoptee must be under 18 years old".to_string(),
        ));
    }

    // Consent required
    if !adoption.consent_given {
        return Err(FamilyLawError::AdoptionError(
            "Adoption requires consent".to_string(),
        ));
    }

    // Court approval required
    if !adoption.court_approved {
        return Err(FamilyLawError::AdoptionError(
            "Adoption requires court approval".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article673_marriage() {
        let valid_marriage = Marriage {
            spouse1: "Spouse A".to_string(),
            spouse2: "Spouse B".to_string(),
            spouse1_age: 25,
            spouse2_age: 23,
            mutual_consent: true,
            registered: true,
            marriage_date: Utc::now(),
        };
        assert!(article673(&valid_marriage).is_ok());

        // Underage marriage
        let underage = Marriage {
            spouse1_age: 17,
            ..valid_marriage.clone()
        };
        assert!(article673(&underage).is_err());

        // No mutual consent
        let no_consent = Marriage {
            mutual_consent: false,
            ..valid_marriage.clone()
        };
        assert!(article673(&no_consent).is_err());
    }

    #[test]
    fn test_article700_divorce() {
        let marriage = Marriage {
            spouse1: "Spouse A".to_string(),
            spouse2: "Spouse B".to_string(),
            spouse1_age: 25,
            spouse2_age: 23,
            mutual_consent: true,
            registered: true,
            marriage_date: Utc::now(),
        };

        let mutual_divorce = Divorce {
            marriage: marriage.clone(),
            divorce_type: DivorceType::MutualConsent,
            grounds: None,
            filed_date: Utc::now(),
            approved: true,
        };
        assert!(article700(&mutual_divorce).is_ok());

        // Judicial divorce without grounds
        let no_grounds = Divorce {
            divorce_type: DivorceType::Judicial,
            grounds: None,
            ..mutual_divorce.clone()
        };
        assert!(article700(&no_grounds).is_err());

        // Judicial divorce with grounds
        let with_grounds = Divorce {
            divorce_type: DivorceType::Judicial,
            grounds: Some("Irreconcilable differences".to_string()),
            ..mutual_divorce.clone()
        };
        assert!(article700(&with_grounds).is_ok());
    }

    #[test]
    fn test_validate_adoption() {
        let adoption = Adoption {
            adopter: "Adopter".to_string(),
            adoptee: "Child".to_string(),
            adoptee_age: 10,
            adoption_type: AdoptionType::Full,
            consent_given: true,
            court_approved: true,
            adoption_date: Utc::now(),
        };
        assert!(validate_adoption(&adoption).is_ok());

        // Adult adoptee
        let adult = Adoption {
            adoptee_age: 18,
            ..adoption.clone()
        };
        assert!(validate_adoption(&adult).is_err());

        // No court approval
        let no_approval = Adoption {
            court_approved: false,
            ..adoption.clone()
        };
        assert!(validate_adoption(&no_approval).is_err());
    }
}
