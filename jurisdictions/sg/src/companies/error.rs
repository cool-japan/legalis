//! Companies Act (Cap. 50) - Error Types
//!
//! This module defines error types for Singapore Companies Act violations and validation failures.
//!
//! All errors include:
//! - Trilingual messages (English, Chinese, Malay)
//! - Statute references (e.g., "CA s. 145")
//! - Contextual information for debugging
//!
//! ## Examples
//!
//! ```
//! use legalis_sg::companies::error::*;
//!
//! let error = CompaniesError::NoResidentDirector;
//! println!("{}", error);
//! // Output: No resident director appointed (Companies Act s. 145(1))
//! //         未委任本地董事 (公司法第145(1)条)
//! //         Tiada pengarah pemastautin dilantik (Akta Syarikat s. 145(1))
//! ```

use thiserror::Error;

/// Result type for Companies Act operations
pub type Result<T> = std::result::Result<T, CompaniesError>;

/// Companies Act error types
///
/// Each error variant includes:
/// - English error message
/// - Chinese translation (新加坡华文)
/// - Malay translation (Bahasa Melayu)
/// - Statute reference (Companies Act section)
#[derive(Error, Debug, Clone, PartialEq)]
pub enum CompaniesError {
    /// Company name already exists or is reserved
    #[error(
        "Company name already exists or reserved: '{name}'\n\
         公司名称已存在或已被保留: '{name}'"
    )]
    NameNotAvailable {
        /// The conflicting name
        name: String,
    },

    /// Company name missing required legal suffix
    #[error(
        "Company name '{name}' missing required suffix '{suffix}' (Companies Act s. 27)\n\
         公司名称 '{name}' 缺少必需的后缀 '{suffix}' (公司法第27条)\n\
         Nama syarikat '{name}' tiada akhiran '{suffix}' yang diperlukan (Akta Syarikat s. 27)"
    )]
    MissingLegalSuffix {
        /// The company name
        name: String,
        /// Required suffix (e.g., "Pte Ltd")
        suffix: String,
    },

    /// No resident director appointed (s. 145 violation)
    #[error(
        "No resident director appointed (Companies Act s. 145(1))\n\
         未委任本地董事 (公司法第145(1)条)\n\
         Tiada pengarah pemastautin dilantik (Akta Syarikat s. 145(1))"
    )]
    NoResidentDirector,

    /// Director is disqualified from being appointed
    #[error(
        "Director '{name}' is disqualified: {reason} (Companies Act s. 148/149/155)\n\
         董事 '{name}' 被取消资格: {reason} (公司法第148/149/155条)"
    )]
    DirectorDisqualified {
        /// Director name
        name: String,
        /// Reason for disqualification
        reason: String,
    },

    /// Director is underage (must be at least 18 years old)
    #[error(
        "Director must be at least 18 years old (Companies Act s. 145(2))\n\
         董事必须年满18岁 (公司法第145(2)条)\n\
         Pengarah mestilah berumur sekurang-kurangnya 18 tahun (Akta Syarikat s. 145(2))"
    )]
    DirectorUnderage,

    /// Insufficient paid-up capital
    #[error(
        "Insufficient paid-up capital: SGD {actual:.2} (minimum: SGD {required:.2})\n\
         实收资本不足: SGD {actual:.2} (最低要求: SGD {required:.2})"
    )]
    InsufficientCapital {
        /// Actual paid-up capital
        actual: f64,
        /// Required minimum capital
        required: f64,
    },

    /// Company has no shareholders
    #[error(
        "Company must have at least one shareholder\n\
         公司必须至少有一名股东"
    )]
    NoShareholders,

    /// Too many shareholders for private limited company
    #[error(
        "Private limited company cannot have more than {max} shareholders (actual: {actual}) (Companies Act s. 18(1))\n\
         私人有限公司股东人数不得超过{max}人 (实际: {actual}) (公司法第18(1)条)"
    )]
    TooManyShareholders {
        /// Actual number of shareholders
        actual: usize,
        /// Maximum allowed
        max: usize,
    },

    /// Share capital mismatch
    #[error(
        "Share capital mismatch: total paid-up capital SGD {paid_up:.2} does not match sum of shareholder contributions SGD {shareholder_total:.2}\n\
         股本不匹配: 总实收资本 SGD {paid_up:.2} 与股东出资总额 SGD {shareholder_total:.2} 不符"
    )]
    ShareCapitalMismatch {
        /// Declared paid-up capital
        paid_up: f64,
        /// Sum of shareholder contributions
        shareholder_total: f64,
    },

    /// No company secretary appointed
    #[error(
        "No company secretary appointed (Companies Act s. 171 - required within 6 months)\n\
         未委任公司秘书 (公司法第171条 - 须于6个月内委任)\n\
         Tiada setiausaha syarikat dilantik (Akta Syarikat s. 171 - perlu dalam 6 bulan)"
    )]
    NoCompanySecretary,

    /// Company secretary not resident in Singapore
    #[error(
        "Company secretary must be ordinarily resident in Singapore (Companies Act s. 171(1A))\n\
         公司秘书必须长居新加坡 (公司法第171(1A)条)"
    )]
    SecretaryNotResident,

    /// Annual General Meeting (AGM) overdue
    #[error(
        "Annual General Meeting overdue by {days} days (Companies Act s. 175)\n\
         年度股东大会逾期{days}天 (公司法第175条)\n\
         Mesyuarat Agung Tahunan lewat {days} hari (Akta Syarikat s. 175)"
    )]
    AgmOverdue {
        /// Number of days overdue
        days: i64,
    },

    /// Annual return filing overdue
    #[error(
        "Annual return filing overdue (Companies Act s. 197 - due within 7 months of FYE)\n\
         年度申报逾期 (公司法第197条 - 须于财政年度结束后7个月内提交)\n\
         Pemfailan penyata tahunan lewat (Akta Syarikat s. 197 - perlu dalam 7 bulan)"
    )]
    AnnualReturnOverdue,

    /// Invalid UEN (Unique Entity Number) format
    #[error(
        "Invalid UEN format: '{uen}' (expected 9-10 alphanumeric characters)\n\
         统一实体编号格式无效: '{uen}' (应为9-10位字母数字字符)"
    )]
    InvalidUen {
        /// The invalid UEN
        uen: String,
    },

    /// Registered office not in Singapore
    #[error(
        "Registered office must be in Singapore (Companies Act s. 142)\n\
         注册办事处必须位于新加坡 (公司法第142条)"
    )]
    RegisteredOfficeNotInSingapore,

    /// Invalid share allocation
    #[error(
        "Invalid share allocation: {reason}\n\
         股份分配无效: {reason}"
    )]
    InvalidShareAllocation {
        /// Reason for invalidity
        reason: String,
    },

    /// Share allotment exceeds authorized capital
    #[error(
        "Share allotment of SGD {allotment:.2} exceeds authorized capital of SGD {authorized:.2}\n\
         股份配发额 SGD {allotment:.2} 超过核定资本 SGD {authorized:.2}"
    )]
    AllotmentExceedsAuthorized {
        /// Amount being allotted
        allotment: f64,
        /// Authorized capital
        authorized: f64,
    },

    /// Invalid financial year end date
    #[error(
        "Invalid financial year end: month {month}, day {day}\n\
         财政年度结束日期无效: 月 {month}, 日 {day}"
    )]
    InvalidFinancialYearEnd {
        /// Month (1-12)
        month: u8,
        /// Day (1-31)
        day: u8,
    },

    /// Foreign company not properly registered
    #[error(
        "Foreign company must be registered within 2 months of establishing place of business in Singapore (Companies Act s. 368)\n\
         外国公司必须在新加坡设立营业地点后2个月内注册 (公司法第368条)"
    )]
    ForeignCompanyNotRegistered,

    /// Multiple directors with same identification
    #[error(
        "Duplicate director identification: '{id}'\n\
         重复的董事身份证明: '{id}'"
    )]
    DuplicateDirector {
        /// Duplicate identification (NRIC/passport)
        id: String,
    },

    /// Shareholder ownership exceeds 100%
    #[error(
        "Total shareholder ownership {total_percent:.2}% exceeds 100%\n\
         股东持股总额 {total_percent:.2}% 超过100%"
    )]
    OwnershipExceeds100Percent {
        /// Total ownership percentage
        total_percent: f64,
    },

    /// Validation failed with multiple errors
    #[error(
        "Validation failed with {count} errors\n\
         验证失败，共{count}个错误"
    )]
    MultipleValidationErrors {
        /// Number of errors
        count: usize,
        /// List of error messages
        errors: Vec<String>,
    },

    /// Generic validation error
    #[error(
        "Validation error: {message}\n\
         验证错误: {message}"
    )]
    ValidationError {
        /// Error message
        message: String,
    },
}

impl CompaniesError {
    /// Returns the Companies Act section reference for this error (if applicable)
    pub fn statute_reference(&self) -> Option<&'static str> {
        match self {
            CompaniesError::MissingLegalSuffix { .. } => Some("CA s. 27"),
            CompaniesError::NoResidentDirector => Some("CA s. 145(1)"),
            CompaniesError::DirectorDisqualified { .. } => Some("CA s. 148/149/155"),
            CompaniesError::DirectorUnderage => Some("CA s. 145(2)"),
            CompaniesError::TooManyShareholders { .. } => Some("CA s. 18(1)"),
            CompaniesError::NoCompanySecretary => Some("CA s. 171"),
            CompaniesError::SecretaryNotResident => Some("CA s. 171(1A)"),
            CompaniesError::AgmOverdue { .. } => Some("CA s. 175"),
            CompaniesError::AnnualReturnOverdue => Some("CA s. 197"),
            CompaniesError::RegisteredOfficeNotInSingapore => Some("CA s. 142"),
            CompaniesError::ForeignCompanyNotRegistered => Some("CA s. 368"),
            _ => None,
        }
    }

    /// Returns the severity of this error
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            CompaniesError::NoResidentDirector
            | CompaniesError::DirectorDisqualified { .. }
            | CompaniesError::AgmOverdue { .. }
            | CompaniesError::AnnualReturnOverdue
            | CompaniesError::RegisteredOfficeNotInSingapore => ErrorSeverity::High,

            CompaniesError::MissingLegalSuffix { .. }
            | CompaniesError::NoShareholders
            | CompaniesError::NoCompanySecretary
            | CompaniesError::InsufficientCapital { .. }
            | CompaniesError::ShareCapitalMismatch { .. } => ErrorSeverity::Medium,

            CompaniesError::InvalidUen { .. }
            | CompaniesError::InvalidFinancialYearEnd { .. }
            | CompaniesError::InvalidShareAllocation { .. } => ErrorSeverity::Low,

            _ => ErrorSeverity::Medium,
        }
    }

    /// Returns whether this error would prevent company registration
    pub fn blocks_registration(&self) -> bool {
        matches!(
            self,
            CompaniesError::NoResidentDirector
                | CompaniesError::DirectorDisqualified { .. }
                | CompaniesError::MissingLegalSuffix { .. }
                | CompaniesError::NoShareholders
                | CompaniesError::RegisteredOfficeNotInSingapore
                | CompaniesError::InvalidUen { .. }
        )
    }
}

/// Error severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Low severity (informational, does not block operations)
    Low,
    /// Medium severity (should be addressed but not immediately blocking)
    Medium,
    /// High severity (blocks operations, must be resolved immediately)
    High,
}

impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSeverity::Low => write!(f, "LOW"),
            ErrorSeverity::Medium => write!(f, "MEDIUM"),
            ErrorSeverity::High => write!(f, "HIGH"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_statute_reference() {
        let error = CompaniesError::NoResidentDirector;
        assert_eq!(error.statute_reference(), Some("CA s. 145(1)"));

        let error2 = CompaniesError::AgmOverdue { days: 30 };
        assert_eq!(error2.statute_reference(), Some("CA s. 175"));
    }

    #[test]
    fn test_error_severity() {
        let high = CompaniesError::NoResidentDirector;
        assert_eq!(high.severity(), ErrorSeverity::High);

        let medium = CompaniesError::NoCompanySecretary;
        assert_eq!(medium.severity(), ErrorSeverity::Medium);

        let low = CompaniesError::InvalidUen {
            uen: "test".to_string(),
        };
        assert_eq!(low.severity(), ErrorSeverity::Low);
    }

    #[test]
    fn test_blocks_registration() {
        let blocking = CompaniesError::NoResidentDirector;
        assert!(blocking.blocks_registration());

        let non_blocking = CompaniesError::AgmOverdue { days: 10 };
        assert!(!non_blocking.blocks_registration());
    }

    #[test]
    fn test_error_display() {
        let error = CompaniesError::NoResidentDirector;
        let display = format!("{}", error);
        assert!(display.contains("No resident director"));
        assert!(display.contains("未委任本地董事"));
        assert!(display.contains("s. 145"));
    }
}
