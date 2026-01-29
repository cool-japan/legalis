//! Islamic Family Law for Muslims in Malaysia
//!
//! Governed by state-level Islamic Family Law enactments.
//!
//! # Key Areas
//!
//! - **Marriage (Nikah)**: Requirements for valid Islamic marriage
//! - **Divorce (Talaq, Khuluk, etc.)**: Various forms of Islamic divorce
//! - **Custody (Hadhanah)**: Child custody rights
//! - **Maintenance (Nafkah)**: Financial support obligations
//! - **Inheritance (Faraid)**: Islamic inheritance distribution

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Islamic family law error types.
#[derive(Debug, Error)]
pub enum IslamicFamilyLawError {
    /// Invalid marriage.
    #[error("Invalid Islamic marriage: {reason}")]
    InvalidMarriage { reason: String },

    /// Invalid divorce.
    #[error("Invalid divorce: {reason}")]
    InvalidDivorce { reason: String },

    /// Maintenance issue.
    #[error("Maintenance issue: {reason}")]
    MaintenanceIssue { reason: String },
}

/// Result type for Islamic family law operations.
pub type Result<T> = std::result::Result<T, IslamicFamilyLawError>;

/// Islamic marriage (Nikah).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IslamicMarriage {
    /// Marriage ID.
    pub id: Uuid,
    /// Groom's name.
    pub groom_name: String,
    /// Groom's IC number.
    pub groom_ic: String,
    /// Bride's name.
    pub bride_name: String,
    /// Bride's IC number.
    pub bride_ic: String,
    /// Wali (guardian) for bride.
    pub wali_name: String,
    /// Mahr (dowry) amount in sen.
    pub mahr_sen: i64,
    /// Marriage date.
    pub marriage_date: DateTime<Utc>,
    /// Whether marriage is registered.
    pub registered: bool,
}

impl IslamicMarriage {
    /// Creates a new Islamic marriage.
    #[must_use]
    pub fn new(
        groom_name: impl Into<String>,
        groom_ic: impl Into<String>,
        bride_name: impl Into<String>,
        bride_ic: impl Into<String>,
        wali_name: impl Into<String>,
        mahr_sen: i64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            groom_name: groom_name.into(),
            groom_ic: groom_ic.into(),
            bride_name: bride_name.into(),
            bride_ic: bride_ic.into(),
            wali_name: wali_name.into(),
            mahr_sen,
            marriage_date: Utc::now(),
            registered: false,
        }
    }

    /// Validates the Islamic marriage.
    pub fn validate(&self) -> Result<ValidationReport> {
        validate_islamic_marriage(self)
    }
}

/// Types of Islamic divorce.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivorceType {
    /// Talaq (husband-initiated divorce).
    Talaq,
    /// Khuluk (divorce by mutual consent with return of mahr).
    Khuluk,
    /// Fasakh (divorce by court order).
    Fasakh,
    /// Taklik (conditional divorce).
    Taklik,
}

/// Islamic divorce.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IslamicDivorce {
    /// Divorce ID.
    pub id: Uuid,
    /// Marriage ID being dissolved.
    pub marriage_id: Uuid,
    /// Divorce type.
    pub divorce_type: DivorceType,
    /// Divorce date.
    pub divorce_date: DateTime<Utc>,
    /// Iddah period (waiting period) in days.
    pub iddah_days: u16,
    /// Whether divorce is registered.
    pub registered: bool,
}

impl IslamicDivorce {
    /// Creates a new Islamic divorce.
    #[must_use]
    pub fn new(marriage_id: Uuid, divorce_type: DivorceType, iddah_days: u16) -> Self {
        Self {
            id: Uuid::new_v4(),
            marriage_id,
            divorce_type,
            divorce_date: Utc::now(),
            iddah_days,
            registered: false,
        }
    }
}

/// Maintenance (Nafkah) obligation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Maintenance {
    /// Maintenance ID.
    pub id: Uuid,
    /// Obligor (person paying).
    pub obligor_name: String,
    /// Obligee (person receiving).
    pub obligee_name: String,
    /// Monthly amount in sen.
    pub monthly_amount_sen: i64,
    /// Maintenance type.
    pub maintenance_type: MaintenanceType,
}

/// Type of maintenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceType {
    /// Maintenance for wife.
    Wife,
    /// Maintenance for children.
    Children,
    /// Maintenance for parents.
    Parents,
    /// Iddah maintenance (during waiting period after divorce).
    Iddah,
    /// Mut'ah (consolatory gift upon divorce).
    Mutah,
}

/// Validation report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Whether marriage is valid under Syariah.
    pub valid: bool,
    /// Issues found.
    pub issues: Vec<String>,
}

/// Validates an Islamic marriage.
pub fn validate_islamic_marriage(marriage: &IslamicMarriage) -> Result<ValidationReport> {
    let mut issues = Vec::new();

    // Check wali requirement (bride must have wali)
    if marriage.wali_name.is_empty() {
        issues.push("Bride must have a wali (guardian) for valid nikah".to_string());
    }

    // Check mahr requirement (must have mahr)
    if marriage.mahr_sen <= 0 {
        issues.push("Marriage must have mahr (dowry)".to_string());
    }

    // Recommend registration
    if !marriage.registered {
        issues.push("Marriage should be registered with Syariah authorities".to_string());
    }

    let valid = issues.iter().filter(|i| !i.contains("should")).count() == 0;

    Ok(ValidationReport { valid, issues })
}

/// Validates Syariah compliance for family law matters.
pub fn validate_shariah_compliance(description: &str) -> bool {
    // Simplified validation - checks for basic Islamic principles
    let prohibited_keywords = ["riba", "gharar", "maysir"];
    !prohibited_keywords
        .iter()
        .any(|keyword| description.to_lowercase().contains(keyword))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_islamic_marriage() {
        let marriage = IslamicMarriage::new(
            "Ahmad bin Abdullah",
            "850123-01-5678",
            "Fatimah binti Hassan",
            "900214-02-1234",
            "Hassan bin Ali", // Wali
            50_000,           // RM 500 mahr
        );

        let report = marriage.validate().expect("Validation succeeds");
        // Will have registration warning but should be valid
        assert!(report.valid || report.issues.len() == 1);
    }

    #[test]
    fn test_invalid_marriage_no_wali() {
        let marriage = IslamicMarriage::new(
            "Ahmad bin Abdullah",
            "850123-01-5678",
            "Fatimah binti Hassan",
            "900214-02-1234",
            "", // No wali
            50_000,
        );

        let report = marriage.validate().expect("Validation succeeds");
        assert!(!report.valid);
    }

    #[test]
    fn test_divorce_creation() {
        let marriage_id = Uuid::new_v4();
        let divorce = IslamicDivorce::new(marriage_id, DivorceType::Talaq, 90); // 3 months iddah

        assert_eq!(divorce.divorce_type, DivorceType::Talaq);
        assert_eq!(divorce.iddah_days, 90);
    }
}
