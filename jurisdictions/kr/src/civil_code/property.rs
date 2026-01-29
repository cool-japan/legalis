//! Property Law (물권법)
//!
//! # 물권법 / Property Rights
//!
//! Articles 185-372 (제185조 - 제372조)
//!
//! Covers:
//! - Ownership (소유권)
//! - Superficies (지상권)
//! - Servitudes (지역권)
//! - Right of lease on a deposit basis (전세권)
//! - Liens and mortgages (유치권, 저당권)

use crate::common::KrwAmount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Property rights errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PropertyError {
    /// Invalid property right
    #[error("Invalid property right: {0}")]
    InvalidRight(String),

    /// Ownership error
    #[error("Ownership error: {0}")]
    OwnershipError(String),

    /// Security interest error
    #[error("Security interest error: {0}")]
    SecurityInterestError(String),
}

/// Result type for property law operations
pub type PropertyResult<T> = Result<T, PropertyError>;

/// Property rights type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyRight {
    /// Ownership (소유권)
    Ownership,
    /// Superficies (지상권)
    Superficies,
    /// Servitude (지역권)
    Servitude,
    /// Right of lease on deposit basis (전세권)
    Jeonse,
    /// Lien (유치권)
    Lien,
    /// Mortgage (저당권)
    Mortgage,
    /// Pledge (질권)
    Pledge,
}

/// Property ownership
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Property {
    /// Property identifier
    pub property_id: String,
    /// Description
    pub description: String,
    /// Owner
    pub owner: String,
    /// Registration date
    pub registration_date: Option<NaiveDate>,
}

/// Mortgage (저당권)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mortgage {
    /// Mortgagor (저당권 설정자)
    pub mortgagor: String,
    /// Mortgagee (저당권자)
    pub mortgagee: String,
    /// Property identifier
    pub property_id: String,
    /// Secured amount (피담보채권액)
    pub secured_amount: KrwAmount,
    /// Registration date
    pub registration_date: NaiveDate,
    /// Priority rank (순위)
    pub priority_rank: u32,
}

impl Mortgage {
    /// Create new mortgage
    pub fn new(
        mortgagor: impl Into<String>,
        mortgagee: impl Into<String>,
        property_id: impl Into<String>,
        secured_amount: KrwAmount,
        registration_date: NaiveDate,
        priority_rank: u32,
    ) -> Self {
        Self {
            mortgagor: mortgagor.into(),
            mortgagee: mortgagee.into(),
            property_id: property_id.into(),
            secured_amount,
            registration_date,
            priority_rank,
        }
    }
}

/// Validate mortgage
pub fn validate_mortgage(mortgage: &Mortgage) -> PropertyResult<()> {
    if mortgage.mortgagor.is_empty() {
        return Err(PropertyError::SecurityInterestError(
            "Mortgagor cannot be empty".to_string(),
        ));
    }

    if mortgage.mortgagee.is_empty() {
        return Err(PropertyError::SecurityInterestError(
            "Mortgagee cannot be empty".to_string(),
        ));
    }

    if mortgage.secured_amount.won <= 0.0 {
        return Err(PropertyError::SecurityInterestError(
            "Secured amount must be positive".to_string(),
        ));
    }

    Ok(())
}

/// Jeonse (전세권) - Unique Korean housing lease system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Jeonse {
    /// Lessor (임대인)
    pub lessor: String,
    /// Lessee (임차인)
    pub lessee: String,
    /// Property identifier
    pub property_id: String,
    /// Deposit amount (전세금)
    pub deposit: KrwAmount,
    /// Start date
    pub start_date: NaiveDate,
    /// End date
    pub end_date: NaiveDate,
}

impl Jeonse {
    /// Create new jeonse
    pub fn new(
        lessor: impl Into<String>,
        lessee: impl Into<String>,
        property_id: impl Into<String>,
        deposit: KrwAmount,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        Self {
            lessor: lessor.into(),
            lessee: lessee.into(),
            property_id: property_id.into(),
            deposit,
            start_date,
            end_date,
        }
    }

    /// Check if jeonse is currently active
    pub fn is_active(&self) -> bool {
        let today = chrono::Utc::now().date_naive();
        today >= self.start_date && today <= self.end_date
    }

    /// Get remaining days
    pub fn remaining_days(&self) -> i64 {
        let today = chrono::Utc::now().date_naive();
        if today > self.end_date {
            0
        } else {
            (self.end_date - today).num_days()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mortgage_creation() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let mortgage = Mortgage::new(
                "김철수",
                "은행",
                "서울-123",
                KrwAmount::from_man(5_000.0),
                date,
                1,
            );

            assert_eq!(mortgage.mortgagor, "김철수");
            assert_eq!(mortgage.priority_rank, 1);
        }
    }

    #[test]
    fn test_validate_mortgage() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let mortgage = Mortgage::new(
                "김철수",
                "은행",
                "서울-123",
                KrwAmount::from_man(5_000.0),
                date,
                1,
            );

            let result = validate_mortgage(&mortgage);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_jeonse_creation() {
        if let (Some(start), Some(end)) = (
            NaiveDate::from_ymd_opt(2024, 1, 1),
            NaiveDate::from_ymd_opt(2026, 1, 1),
        ) {
            let jeonse = Jeonse::new(
                "박영희",
                "이민호",
                "서울-456",
                KrwAmount::from_eok(5.0),
                start,
                end,
            );

            assert_eq!(jeonse.lessor, "박영희");
            assert_eq!(jeonse.lessee, "이민호");
        }
    }

    #[test]
    fn test_jeonse_is_active() {
        if let (Some(start), Some(end)) = (
            NaiveDate::from_ymd_opt(2020, 1, 1),
            NaiveDate::from_ymd_opt(2030, 1, 1),
        ) {
            let jeonse = Jeonse::new(
                "박영희",
                "이민호",
                "서울-456",
                KrwAmount::from_eok(5.0),
                start,
                end,
            );

            assert!(jeonse.is_active());
        }
    }
}
