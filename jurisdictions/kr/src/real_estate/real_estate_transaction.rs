//! Real Estate Transaction Reporting Act (부동산 거래신고법)
//!
//! # 부동산 거래신고 등에 관한 법률 / Act on Report of Real Estate Transactions

use crate::common::KrwAmount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Real estate transaction errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RealEstateTransactionError {
    /// Invalid transaction
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),

    /// Reporting violation
    #[error("Reporting violation: {0}")]
    ReportingViolation(String),
}

/// Result type for real estate transaction operations
pub type RealEstateTransactionResult<T> = Result<T, RealEstateTransactionError>;

/// Real estate transaction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealEstateTransaction {
    /// Seller
    pub seller: String,
    /// Buyer
    pub buyer: String,
    /// Property address
    pub property_address: String,
    /// Transaction price
    pub transaction_price: KrwAmount,
    /// Contract date
    pub contract_date: NaiveDate,
    /// Reporting date
    pub reporting_date: Option<NaiveDate>,
}

/// Reporting deadline (30 days from contract)
pub const REPORTING_DEADLINE_DAYS: u32 = 30;

/// Validate reporting deadline
pub fn validate_reporting_deadline(
    contract_date: &NaiveDate,
    reporting_date: &NaiveDate,
) -> RealEstateTransactionResult<()> {
    let days_elapsed = (*reporting_date - *contract_date).num_days();

    if days_elapsed > REPORTING_DEADLINE_DAYS as i64 {
        return Err(RealEstateTransactionError::ReportingViolation(format!(
            "Reporting deadline exceeded: {} days",
            days_elapsed
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_estate_transaction() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let transaction = RealEstateTransaction {
                seller: "박영희".to_string(),
                buyer: "김철수".to_string(),
                property_address: "서울시 강남구".to_string(),
                transaction_price: KrwAmount::from_eok(10.0),
                contract_date: date,
                reporting_date: None,
            };
            assert_eq!(transaction.buyer, "김철수");
        }
    }

    #[test]
    fn test_validate_reporting_deadline() {
        if let (Some(contract), Some(reporting)) = (
            NaiveDate::from_ymd_opt(2024, 1, 1),
            NaiveDate::from_ymd_opt(2024, 1, 15),
        ) {
            let result = validate_reporting_deadline(&contract, &reporting);
            assert!(result.is_ok());
        }
    }
}
