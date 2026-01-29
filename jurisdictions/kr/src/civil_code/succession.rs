//! Succession Law (상속법)
//!
//! # 상속법 / Law of Succession
//!
//! Articles 980-1118 (제980조 - 제1118조)
//!
//! Covers:
//! - Inheritance (상속)
//! - Wills (유언)
//! - Statutory succession order
//! - Renunciation and acceptance of inheritance

use crate::common::KrwAmount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Succession errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SuccessionError {
    /// Invalid will
    #[error("Invalid will: {0}")]
    InvalidWill(String),

    /// Invalid heir
    #[error("Invalid heir: {0}")]
    InvalidHeir(String),

    /// Calculation error
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

/// Result type for succession operations
pub type SuccessionResult<T> = Result<T, SuccessionError>;

/// Heir class (상속 순위)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeirClass {
    /// First class: Direct descendants (직계비속)
    FirstClass,
    /// Second class: Direct ascendants (직계존속)
    SecondClass,
    /// Third class: Siblings (형제자매)
    ThirdClass,
    /// Fourth class: Collateral relatives within 4th degree (4촌 이내의 방계혈족)
    FourthClass,
}

/// Heir (상속인)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Heir {
    /// Name
    pub name: String,
    /// Heir class
    pub heir_class: HeirClass,
    /// Relationship to deceased
    pub relationship: String,
}

impl Heir {
    /// Create new heir
    pub fn new(
        name: impl Into<String>,
        heir_class: HeirClass,
        relationship: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            heir_class,
            relationship: relationship.into(),
        }
    }
}

/// Estate (상속재산)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Estate {
    /// Deceased
    pub deceased: String,
    /// Date of death
    pub date_of_death: NaiveDate,
    /// Total value
    pub total_value: KrwAmount,
    /// Debts
    pub debts: KrwAmount,
}

impl Estate {
    /// Create new estate
    pub fn new(
        deceased: impl Into<String>,
        date_of_death: NaiveDate,
        total_value: KrwAmount,
        debts: KrwAmount,
    ) -> Self {
        Self {
            deceased: deceased.into(),
            date_of_death,
            total_value,
            debts,
        }
    }

    /// Calculate net estate value
    pub fn net_value(&self) -> KrwAmount {
        self.total_value.subtract(&self.debts)
    }
}

/// Will type (유언의 방식)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WillType {
    /// Holographic will (자필증서)
    Holographic,
    /// Notarial will (공정증서)
    Notarial,
    /// Secret will (비밀증서)
    Secret,
    /// Oral will (구수증서)
    Oral,
}

/// Will (유언)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Will {
    /// Testator (유언자)
    pub testator: String,
    /// Will type
    pub will_type: WillType,
    /// Date of will
    pub date: NaiveDate,
    /// Beneficiaries
    pub beneficiaries: Vec<String>,
}

impl Will {
    /// Create new will
    pub fn new(testator: impl Into<String>, will_type: WillType, date: NaiveDate) -> Self {
        Self {
            testator: testator.into(),
            will_type,
            date,
            beneficiaries: Vec::new(),
        }
    }

    /// Add beneficiary
    pub fn add_beneficiary(mut self, beneficiary: impl Into<String>) -> Self {
        self.beneficiaries.push(beneficiary.into());
        self
    }
}

/// Validate will (basic validation)
pub fn validate_will(will: &Will) -> SuccessionResult<()> {
    if will.testator.is_empty() {
        return Err(SuccessionError::InvalidWill(
            "Testator cannot be empty".to_string(),
        ));
    }

    if will.beneficiaries.is_empty() {
        return Err(SuccessionError::InvalidWill(
            "Will must have at least one beneficiary".to_string(),
        ));
    }

    Ok(())
}

/// Calculate statutory share for spouse
/// Article 1009: Spouse receives 1.5x the share of each child
pub fn calculate_spouse_share(
    estate_value: &KrwAmount,
    number_of_children: u32,
) -> SuccessionResult<KrwAmount> {
    if number_of_children == 0 {
        return Err(SuccessionError::CalculationError(
            "No children specified".to_string(),
        ));
    }

    // Spouse gets 1.5 shares, each child gets 1 share
    // Total shares = 1.5 + number_of_children
    let total_shares = 1.5 + number_of_children as f64;
    let spouse_shares = 1.5;

    let spouse_portion = spouse_shares / total_shares;
    Ok(estate_value.multiply(spouse_portion))
}

/// Calculate statutory share for each child
pub fn calculate_child_share(
    estate_value: &KrwAmount,
    number_of_children: u32,
) -> SuccessionResult<KrwAmount> {
    if number_of_children == 0 {
        return Err(SuccessionError::CalculationError(
            "No children specified".to_string(),
        ));
    }

    // Spouse gets 1.5 shares, each child gets 1 share
    let total_shares = 1.5 + number_of_children as f64;
    let child_shares = 1.0;

    let child_portion = child_shares / total_shares;
    Ok(estate_value.multiply(child_portion))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heir_creation() {
        let heir = Heir::new("김철수", HeirClass::FirstClass, "아들");
        assert_eq!(heir.name, "김철수");
        assert_eq!(heir.heir_class, HeirClass::FirstClass);
    }

    #[test]
    fn test_estate_creation() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let estate = Estate::new(
                "김철수",
                date,
                KrwAmount::from_eok(10.0),
                KrwAmount::from_eok(2.0),
            );

            let net = estate.net_value();
            assert!((net.won - 800_000_000.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_will_creation() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let will = Will::new("김철수", WillType::Holographic, date)
                .add_beneficiary("박영희")
                .add_beneficiary("이민호");

            assert_eq!(will.beneficiaries.len(), 2);
        }
    }

    #[test]
    fn test_validate_will() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let will = Will::new("김철수", WillType::Notarial, date).add_beneficiary("박영희");

            let result = validate_will(&will);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_calculate_spouse_share() {
        let estate = KrwAmount::from_eok(10.0);
        let result = calculate_spouse_share(&estate, 2);
        assert!(result.is_ok());

        // Spouse gets 1.5 / (1.5 + 2) = 1.5/3.5 = 42.857%
        if let Ok(share) = result {
            let expected = 1_000_000_000.0 * (1.5 / 3.5);
            assert!((share.won - expected).abs() < 1.0);
        }
    }

    #[test]
    fn test_calculate_child_share() {
        let estate = KrwAmount::from_eok(10.0);
        let result = calculate_child_share(&estate, 2);
        assert!(result.is_ok());

        // Each child gets 1.0 / (1.5 + 2) = 1.0/3.5 = 28.571%
        if let Ok(share) = result {
            let expected = 1_000_000_000.0 * (1.0 / 3.5);
            assert!((share.won - expected).abs() < 1.0);
        }
    }
}
