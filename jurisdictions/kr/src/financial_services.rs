//! Financial Services Law
//!
//! # 금융 서비스 관련 법률 / Financial Services Laws
//!
//! Covers financial consumer protection and capital markets

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Financial services errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum FinancialServicesError {
    /// Compliance violation
    #[error("Compliance violation: {0}")]
    ComplianceViolation(String),
}

/// Result type for financial services operations
pub type FinancialServicesResult<T> = Result<T, FinancialServicesError>;

/// Financial product type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FinancialProductType {
    /// Securities (증권)
    Securities,
    /// Insurance (보험)
    Insurance,
    /// Loan (대출)
    Loan,
    /// Deposit (예금)
    Deposit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_financial_product_type() {
        let product = FinancialProductType::Securities;
        assert_eq!(product, FinancialProductType::Securities);
    }
}
