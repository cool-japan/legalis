//! Administrative Procedure Act (행정절차법)
//!
//! # 행정절차법 / Administrative Procedure Act

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Administrative procedure errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AdminProcedureError {
    /// Invalid procedure
    #[error("Invalid procedure: {0}")]
    InvalidProcedure(String),
}

/// Result type for administrative procedure operations
pub type AdminProcedureResult<T> = Result<T, AdminProcedureError>;

/// Administrative disposition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdministrativeDisposition {
    /// Agency name
    pub agency: String,
    /// Date
    pub date: NaiveDate,
    /// Description
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_disposition() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let disposition = AdministrativeDisposition {
                agency: "행정청".to_string(),
                date,
                description: "허가".to_string(),
            };
            assert_eq!(disposition.agency, "행정청");
        }
    }
}
