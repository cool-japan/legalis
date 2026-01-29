//! Housing Lease Protection Act (주택임대차보호법)
//!
//! # 주택임대차보호법 / Housing Lease Protection Act

use crate::common::KrwAmount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Housing lease errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum HousingLeaseError {
    /// Invalid lease
    #[error("Invalid lease: {0}")]
    InvalidLease(String),
}

/// Result type for housing lease operations
pub type HousingLeaseResult<T> = Result<T, HousingLeaseError>;

/// Housing lease
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HousingLease {
    /// Landlord
    pub landlord: String,
    /// Tenant
    pub tenant: String,
    /// Property address
    pub property_address: String,
    /// Deposit
    pub deposit: KrwAmount,
    /// Monthly rent
    pub monthly_rent: Option<KrwAmount>,
    /// Start date
    pub start_date: NaiveDate,
    /// End date
    pub end_date: NaiveDate,
}

/// Lease term (2 years minimum)
pub const MINIMUM_LEASE_TERM_YEARS: u32 = 2;

/// Deposit return deadline (after lease end)
pub const DEPOSIT_RETURN_DEADLINE_DAYS: u32 = 30;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_housing_lease() {
        if let (Some(start), Some(end)) = (
            NaiveDate::from_ymd_opt(2024, 1, 1),
            NaiveDate::from_ymd_opt(2026, 1, 1),
        ) {
            let lease = HousingLease {
                landlord: "박영희".to_string(),
                tenant: "김철수".to_string(),
                property_address: "서울시 강남구".to_string(),
                deposit: KrwAmount::from_eok(5.0),
                monthly_rent: Some(KrwAmount::from_man(100.0)),
                start_date: start,
                end_date: end,
            };
            assert_eq!(lease.landlord, "박영희");
        }
    }
}
