//! Intellectual Property Law
//!
//! # 지식재산권법 / Intellectual Property Law
//!
//! Covers patents, trademarks, copyrights

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// IP errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum IpError {
    /// Invalid registration
    #[error("Invalid registration: {0}")]
    InvalidRegistration(String),
}

/// Result type for intellectual property operations
pub type IpResult<T> = Result<T, IpError>;

/// IP right type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IpRightType {
    /// Patent (특허)
    Patent,
    /// Trademark (상표)
    Trademark,
    /// Copyright (저작권)
    Copyright,
    /// Design (디자인)
    Design,
}

/// Patent term (20 years from filing date)
pub const PATENT_TERM_YEARS: u32 = 20;

/// Trademark term (10 years, renewable)
pub const TRADEMARK_TERM_YEARS: u32 = 10;

/// IP registration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IpRegistration {
    /// Right type
    pub right_type: IpRightType,
    /// Registration number
    pub registration_number: String,
    /// Registration date
    pub registration_date: NaiveDate,
    /// Owner
    pub owner: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_registration() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let registration = IpRegistration {
                right_type: IpRightType::Patent,
                registration_number: "10-2024-0000001".to_string(),
                registration_date: date,
                owner: "김철수".to_string(),
            };

            assert_eq!(registration.right_type, IpRightType::Patent);
        }
    }
}
