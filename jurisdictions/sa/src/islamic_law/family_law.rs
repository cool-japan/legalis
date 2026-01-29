//! Islamic Family Law (الأحوال الشخصية)
//!
//! Family law in Saudi Arabia is governed entirely by Islamic Sharia,
//! following the Hanbali school of jurisprudence.

use crate::common::Sar;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for family law operations
pub type FamilyLawResult<T> = Result<T, FamilyLawError>;

/// Family law errors
#[derive(Debug, Error)]
pub enum FamilyLawError {
    /// Invalid marriage contract
    #[error("عقد زواج غير صالح: {reason}")]
    InvalidMarriage { reason: String },

    /// Invalid divorce
    #[error("طلاق غير صالح: {reason}")]
    InvalidDivorce { reason: String },

    /// Guardianship error
    #[error("خطأ في الولاية: {reason}")]
    GuardianshipError { reason: String },
}

/// Types of divorce in Islamic law
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DivorceType {
    /// Talaq (طلاق) - Husband-initiated divorce
    Talaq,
    /// Khul' (خلع) - Wife-initiated divorce with compensation
    Khul,
    /// Faskh (فسخ) - Judicial dissolution
    Faskh,
    /// Mubarat (مبارأة) - Mutual consent divorce
    Mubarat,
}

impl DivorceType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Talaq => "طلاق",
            Self::Khul => "خلع",
            Self::Faskh => "فسخ",
            Self::Mubarat => "مبارأة",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Talaq => "Talaq (Unilateral Divorce)",
            Self::Khul => "Khul' (Divorce by Wife with Compensation)",
            Self::Faskh => "Faskh (Judicial Dissolution)",
            Self::Mubarat => "Mubarat (Mutual Consent Divorce)",
        }
    }
}

/// Types of guardianship
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GuardianshipType {
    /// Guardian of person (الولاية على النفس)
    PersonGuardianship,
    /// Guardian of property (الولاية على المال)
    PropertyGuardianship,
    /// Both person and property
    FullGuardianship,
}

/// Marriage contract details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarriageContract {
    /// Mahr (dowry) amount
    pub mahr_amount: Sar,
    /// Conditions specified in contract
    pub conditions: Vec<String>,
    /// Guardian consent obtained
    pub guardian_consent: bool,
    /// Witnesses present (minimum 2)
    pub witnesses_count: u32,
}

impl MarriageContract {
    /// Create a new marriage contract
    pub fn new(mahr_amount: Sar) -> Self {
        Self {
            mahr_amount,
            conditions: Vec::new(),
            guardian_consent: false,
            witnesses_count: 0,
        }
    }

    /// Add condition to contract
    pub fn add_condition(mut self, condition: impl Into<String>) -> Self {
        self.conditions.push(condition.into());
        self
    }

    /// Set guardian consent
    pub fn with_guardian_consent(mut self) -> Self {
        self.guardian_consent = true;
        self
    }

    /// Set number of witnesses
    pub fn with_witnesses(mut self, count: u32) -> Self {
        self.witnesses_count = count;
        self
    }

    /// Validate marriage contract under Hanbali school
    pub fn validate(&self) -> FamilyLawResult<()> {
        // Mahr must be specified
        if self.mahr_amount.is_zero() || self.mahr_amount.is_negative() {
            return Err(FamilyLawError::InvalidMarriage {
                reason: "Mahr must be positive amount".to_string(),
            });
        }

        // Guardian consent required for bride
        if !self.guardian_consent {
            return Err(FamilyLawError::InvalidMarriage {
                reason: "Guardian consent (wali) is required".to_string(),
            });
        }

        // Minimum 2 witnesses required
        if self.witnesses_count < 2 {
            return Err(FamilyLawError::InvalidMarriage {
                reason: "At least 2 witnesses required".to_string(),
            });
        }

        Ok(())
    }
}

/// Family law structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyLaw {
    /// Marriage contract
    pub marriage: Option<MarriageContract>,
    /// Children count
    pub children_count: u32,
    /// Guardianship arrangements
    pub guardianship: Option<GuardianshipType>,
}

impl FamilyLaw {
    /// Create new family law instance
    pub fn new() -> Self {
        Self {
            marriage: None,
            children_count: 0,
            guardianship: None,
        }
    }

    /// Set marriage contract
    pub fn with_marriage(mut self, marriage: MarriageContract) -> Self {
        self.marriage = Some(marriage);
        self
    }

    /// Set children count
    pub fn with_children(mut self, count: u32) -> Self {
        self.children_count = count;
        self
    }
}

impl Default for FamilyLaw {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divorce_types() {
        assert_eq!(DivorceType::Talaq.name_ar(), "طلاق");
        assert_eq!(
            DivorceType::Khul.name_en(),
            "Khul' (Divorce by Wife with Compensation)"
        );
    }

    #[test]
    fn test_valid_marriage_contract() {
        let contract = MarriageContract::new(Sar::from_riyals(50_000))
            .with_guardian_consent()
            .with_witnesses(2);

        assert!(contract.validate().is_ok());
    }

    #[test]
    fn test_invalid_marriage_no_mahr() {
        let contract = MarriageContract::new(Sar::from_riyals(0))
            .with_guardian_consent()
            .with_witnesses(2);

        assert!(contract.validate().is_err());
    }

    #[test]
    fn test_invalid_marriage_no_guardian() {
        let contract = MarriageContract::new(Sar::from_riyals(50_000)).with_witnesses(2);

        assert!(contract.validate().is_err());
    }

    #[test]
    fn test_invalid_marriage_insufficient_witnesses() {
        let contract = MarriageContract::new(Sar::from_riyals(50_000))
            .with_guardian_consent()
            .with_witnesses(1);

        assert!(contract.validate().is_err());
    }

    #[test]
    fn test_marriage_with_conditions() {
        let contract = MarriageContract::new(Sar::from_riyals(100_000))
            .add_condition("Wife can work")
            .add_condition("No polygamy")
            .with_guardian_consent()
            .with_witnesses(3);

        assert_eq!(contract.conditions.len(), 2);
        assert!(contract.validate().is_ok());
    }
}
