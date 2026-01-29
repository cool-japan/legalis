//! Family Law (가족법)
//!
//! # 가족법 / Family Law
//!
//! Articles 767-979 (제767조 - 제979조)
//!
//! Covers:
//! - Marriage (혼인)
//! - Parent-child relationship (친자)
//! - Adoption (입양)
//! - Parental authority (친권)
//! - Support obligations (부양)

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Family law errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum FamilyError {
    /// Invalid marriage
    #[error("Invalid marriage: {0}")]
    InvalidMarriage(String),

    /// Invalid adoption
    #[error("Invalid adoption: {0}")]
    InvalidAdoption(String),

    /// Support obligation error
    #[error("Support obligation error: {0}")]
    SupportError(String),
}

/// Result type for family law operations
pub type FamilyResult<T> = Result<T, FamilyError>;

/// Marriage (혼인)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Marriage {
    /// Husband
    pub husband: String,
    /// Wife
    pub wife: String,
    /// Marriage date
    pub marriage_date: NaiveDate,
    /// Registration date
    pub registration_date: Option<NaiveDate>,
}

impl Marriage {
    /// Create new marriage
    pub fn new(
        husband: impl Into<String>,
        wife: impl Into<String>,
        marriage_date: NaiveDate,
    ) -> Self {
        Self {
            husband: husband.into(),
            wife: wife.into(),
            marriage_date,
            registration_date: None,
        }
    }

    /// Set registration date
    pub fn with_registration(mut self, registration_date: NaiveDate) -> Self {
        self.registration_date = Some(registration_date);
        self
    }
}

/// Validate marriage eligibility
/// Article 807: Age requirement is 18 years old
pub fn validate_marriage_eligibility(
    person1_dob: &NaiveDate,
    person2_dob: &NaiveDate,
) -> FamilyResult<()> {
    let today = chrono::Utc::now().date_naive();

    let age1 = today.years_since(*person1_dob).unwrap_or(0);
    let age2 = today.years_since(*person2_dob).unwrap_or(0);

    if age1 < 18 {
        return Err(FamilyError::InvalidMarriage(
            "Person 1 must be at least 18 years old".to_string(),
        ));
    }

    if age2 < 18 {
        return Err(FamilyError::InvalidMarriage(
            "Person 2 must be at least 18 years old".to_string(),
        ));
    }

    Ok(())
}

/// Adoption (입양)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Adoption {
    /// Adoptive parent
    pub adoptive_parent: String,
    /// Adopted child
    pub adopted_child: String,
    /// Adoption date
    pub adoption_date: NaiveDate,
    /// Court approval date
    pub court_approval_date: Option<NaiveDate>,
}

impl Adoption {
    /// Create new adoption
    pub fn new(
        adoptive_parent: impl Into<String>,
        adopted_child: impl Into<String>,
        adoption_date: NaiveDate,
    ) -> Self {
        Self {
            adoptive_parent: adoptive_parent.into(),
            adopted_child: adopted_child.into(),
            adoption_date,
            court_approval_date: None,
        }
    }

    /// Set court approval date
    pub fn with_court_approval(mut self, approval_date: NaiveDate) -> Self {
        self.court_approval_date = Some(approval_date);
        self
    }
}

/// Support obligation (부양의무)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportObligation {
    /// Obligor (부양의무자)
    pub obligor: String,
    /// Obligee (부양권리자)
    pub obligee: String,
    /// Relationship
    pub relationship: String,
}

impl SupportObligation {
    /// Create new support obligation
    pub fn new(
        obligor: impl Into<String>,
        obligee: impl Into<String>,
        relationship: impl Into<String>,
    ) -> Self {
        Self {
            obligor: obligor.into(),
            obligee: obligee.into(),
            relationship: relationship.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marriage_creation() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let marriage = Marriage::new("김철수", "박영희", date);
            assert_eq!(marriage.husband, "김철수");
            assert_eq!(marriage.wife, "박영희");
        }
    }

    #[test]
    fn test_validate_marriage_eligibility() {
        if let (Some(dob1), Some(dob2)) = (
            NaiveDate::from_ymd_opt(2000, 1, 1),
            NaiveDate::from_ymd_opt(2001, 1, 1),
        ) {
            let result = validate_marriage_eligibility(&dob1, &dob2);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_validate_marriage_eligibility_underage() {
        if let (Some(dob1), Some(dob2)) = (
            NaiveDate::from_ymd_opt(2010, 1, 1),
            NaiveDate::from_ymd_opt(2011, 1, 1),
        ) {
            let result = validate_marriage_eligibility(&dob1, &dob2);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_adoption_creation() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let adoption = Adoption::new("김철수", "이민호", date);
            assert_eq!(adoption.adoptive_parent, "김철수");
            assert_eq!(adoption.adopted_child, "이민호");
        }
    }

    #[test]
    fn test_support_obligation() {
        let support = SupportObligation::new("김철수", "부모", "자녀");
        assert_eq!(support.obligor, "김철수");
        assert_eq!(support.relationship, "자녀");
    }
}
