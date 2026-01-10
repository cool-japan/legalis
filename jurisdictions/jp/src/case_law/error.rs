//! Case Law Error Types (判例エラー型)
//!
//! Error types for case law database operations.

use thiserror::Error;

/// Case law database errors (判例データベースエラー)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum CaseLawError {
    /// Case not found
    /// (判例が見つかりません - Hanrei ga mitsukarimasen)
    #[error("Case not found: {case_id}")]
    CaseNotFound { case_id: String },

    /// Invalid case number format
    /// (無効な事件番号 - Mukō na jiken bangō)
    #[error("Invalid case number format: {case_number}")]
    InvalidCaseNumber { case_number: String },

    /// Search query error
    /// (検索クエリエラー - Kensaku kueri erā)
    #[error("Invalid search query: {reason}")]
    InvalidSearchQuery { reason: String },

    /// No results found
    /// (検索結果なし - Kensaku kekka nashi)
    #[error("No cases found matching the search criteria")]
    NoResultsFound,

    /// Database error
    /// (データベースエラー - Dētabēsu erā)
    #[error("Database error: {message}")]
    DatabaseError { message: String },

    /// Citation format error
    /// (引用形式エラー - In'yō keishiki erā)
    #[error("Invalid citation format: {reason}")]
    InvalidCitationFormat { reason: String },

    /// Parsing error
    /// (パースエラー - Pāsu erā)
    #[error("Failed to parse case data: {reason}")]
    ParsingError { reason: String },

    /// Missing required field
    /// (必須フィールド未入力 - Hissu fīrudo mi-nyūryoku)
    #[error("Missing required field: {field_name}")]
    MissingRequiredField { field_name: String },

    /// Invalid date range
    /// (無効な日付範囲 - Mukō na hizuke han'i)
    #[error("Invalid date range: start date must be before end date")]
    InvalidDateRange,

    /// Too many results
    /// (結果が多すぎます - Kekka ga ōsugimasu)
    #[error("Query returned too many results: {count} (limit: {limit})")]
    TooManyResults { count: usize, limit: usize },

    /// External API error
    /// (外部APIエラー - Gaibu API erā)
    #[error("External API error: {service} - {message}")]
    ExternalApiError { service: String, message: String },

    /// Serialization error
    /// (シリアライズエラー - Shiriaraizu erā)
    #[error("Serialization error: {reason}")]
    SerializationError { reason: String },

    /// Permission denied
    /// (アクセス拒否 - Akusesu kyohi)
    #[error("Permission denied: {resource}")]
    PermissionDenied { resource: String },

    /// Generic error
    /// (汎用エラー - Han'yō erā)
    #[error("Case law error: {message}")]
    GenericError { message: String },
}

/// Result type for case law operations
pub type Result<T> = std::result::Result<T, CaseLawError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = CaseLawError::CaseNotFound {
            case_id: "case-001".to_string(),
        };
        assert!(error.to_string().contains("case-001"));

        let error = CaseLawError::InvalidSearchQuery {
            reason: "Empty keywords".to_string(),
        };
        assert!(error.to_string().contains("Empty keywords"));
    }

    #[test]
    fn test_error_equality() {
        let error1 = CaseLawError::NoResultsFound;
        let error2 = CaseLawError::NoResultsFound;
        assert_eq!(error1, error2);

        let error3 = CaseLawError::InvalidDateRange;
        assert_ne!(error1, error3);
    }
}
