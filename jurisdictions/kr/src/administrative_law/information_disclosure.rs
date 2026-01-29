//! Information Disclosure Act (정보공개법)
//!
//! # 정보공개법 / Information Disclosure Act

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Information disclosure errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InfoDisclosureError {
    /// Disclosure denied
    #[error("Disclosure denied: {0}")]
    DisclosureDenied(String),
}

/// Result type for information disclosure operations
pub type InfoDisclosureResult<T> = Result<T, InfoDisclosureError>;

/// Information disclosure request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisclosureRequest {
    /// Requester
    pub requester: String,
    /// Request date
    pub request_date: NaiveDate,
    /// Information requested
    pub information_requested: String,
}
/// Deadline for responding to information disclosure requests (days)
/// Response deadline (10 days)
pub const RESPONSE_DEADLINE_DAYS: u32 = 10;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disclosure_request() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let request = DisclosureRequest {
                requester: "김철수".to_string(),
                request_date: date,
                information_requested: "예산 집행 내역".to_string(),
            };
            assert_eq!(request.requester, "김철수");
        }
    }
}
