//! Civil Procedure Act (민사소송법)
//!
//! # 민사소송법 / Civil Procedure Act

use crate::common::KrwAmount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Civil procedure errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CivilProcedureError {
    /// Invalid filing
    #[error("Invalid filing: {0}")]
    InvalidFiling(String),
}

/// Result type for civil procedure operations
pub type CivilProcedureResult<T> = Result<T, CivilProcedureError>;

/// Civil lawsuit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CivilLawsuit {
    /// Case number
    pub case_number: String,
    /// Plaintiff
    pub plaintiff: String,
    /// Defendant
    pub defendant: String,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Claim amount
    pub claim_amount: Option<KrwAmount>,
}

/// Court filing fees (based on claim amount)
pub fn calculate_filing_fee(claim_amount: &KrwAmount) -> CivilProcedureResult<KrwAmount> {
    // Simplified calculation: approximately 0.5% of claim amount
    let fee = claim_amount.multiply(0.005);
    Ok(fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_lawsuit() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let lawsuit = CivilLawsuit {
                case_number: "2024가합12345".to_string(),
                plaintiff: "김철수".to_string(),
                defendant: "박영희".to_string(),
                filing_date: date,
                claim_amount: Some(KrwAmount::from_eok(1.0)),
            };
            assert_eq!(lawsuit.case_number, "2024가합12345");
        }
    }

    #[test]
    fn test_calculate_filing_fee() {
        let claim = KrwAmount::from_eok(1.0);
        let result = calculate_filing_fee(&claim);
        assert!(result.is_ok());
    }
}
