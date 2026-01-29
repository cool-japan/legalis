//! Criminal Procedure Act (형사소송법)
//!
//! # 형사소송법 / Criminal Procedure Act

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Criminal procedure errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CriminalProcedureError {
    /// Invalid procedure
    #[error("Invalid procedure: {0}")]
    InvalidProcedure(String),
}

/// Result type for criminal procedure operations
pub type CriminalProcedureResult<T> = Result<T, CriminalProcedureError>;

/// Criminal case
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriminalCase {
    /// Case number
    pub case_number: String,
    /// Defendant
    pub defendant: String,
    /// Charge
    pub charge: String,
    /// Indictment date
    pub indictment_date: NaiveDate,
}

/// Detention warrant validity (10 days initial)
pub const DETENTION_WARRANT_VALIDITY_DAYS: u32 = 10;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criminal_case() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let case = CriminalCase {
                case_number: "2024고단1234".to_string(),
                defendant: "김철수".to_string(),
                charge: "절도".to_string(),
                indictment_date: date,
            };
            assert_eq!(case.case_number, "2024고단1234");
        }
    }
}
