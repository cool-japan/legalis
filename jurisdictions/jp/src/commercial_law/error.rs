//! Commercial Law Error Types (商法・会社法のエラー型)
//!
//! Comprehensive error types for commercial law validation and operations.

use thiserror::Error;

/// Commercial law errors (商法・会社法のエラー)
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CommercialLawError {
    // ========================================================================
    // Capital Errors (資本金エラー)
    // ========================================================================
    /// Capital below minimum requirement
    /// (資本金が最低要件未満 - Shihon-kin ga saitei yōken miman)
    #[error("Capital amount ¥{actual} is below minimum requirement of ¥{minimum} (Article 27)")]
    CapitalBelowMinimum { actual: u64, minimum: u64 },

    // ========================================================================
    // Company Formation Errors (会社設立エラー)
    // ========================================================================
    /// Invalid company name
    /// (無効な商号 - Mukō na shōgō)
    #[error("Invalid company name: {reason}")]
    InvalidCompanyName { reason: String },

    /// Company name missing required suffix
    /// (商号に必須の表示がない - Shōgō ni hissu no hyōji ga nai)
    #[error("Company name '{name}' must include company type suffix (e.g., '株式会社')")]
    MissingCompanyTypeSuffix { name: String },

    /// No business purposes specified
    /// (事業目的が未指定 - Jigyō mokuteki ga mishitei)
    #[error("At least one business purpose must be specified (Article 27, Item 1)")]
    NoBusinessPurposes,

    /// Invalid business purpose
    /// (無効な事業目的 - Mukō na jigyō mokuteki)
    #[error("Invalid business purpose: {reason}")]
    InvalidBusinessPurpose { reason: String },

    /// No incorporators
    /// (発起人がいない - Hokki-nin ga inai)
    #[error("At least one incorporator is required (Article 25)")]
    NoIncorporators,

    /// Invalid incorporator investment
    /// (無効な発起人出資 - Mukō na hokki-nin shusshi)
    #[error("Incorporator '{name}' has invalid investment: {reason}")]
    InvalidIncorporatorInvestment { name: String, reason: String },

    /// Total investment mismatch
    /// (出資総額の不一致 - Shusshi sōgaku no fuicchi)
    #[error("Total incorporator investment ¥{total_investment} does not match capital ¥{capital}")]
    InvestmentCapitalMismatch { total_investment: u64, capital: u64 },

    /// Authorized shares not specified for stock company
    /// (株式会社の発行可能株式総数が未指定)
    #[error("Authorized shares must be specified for stock companies (Article 37)")]
    AuthorizedSharesNotSpecified,

    /// Authorized shares below issued shares
    /// (発行可能株式総数が発行済株式数未満)
    #[error("Authorized shares {authorized} must be at least issued shares {issued}")]
    AuthorizedSharesBelowIssued { authorized: u64, issued: u64 },

    /// Invalid fiscal year end
    /// (無効な決算期 - Mukō na kessan-ki)
    #[error("Invalid fiscal year end month: {month}. Must be between 1 and 12")]
    InvalidFiscalYearEnd { month: u8 },

    // ========================================================================
    // Corporate Governance Errors (コーポレートガバナンスエラー)
    // ========================================================================
    /// Insufficient directors
    /// (取締役数不足 - Torishimari-yaku-sū fusoku)
    #[error(
        "Minimum {required} directors required for board, but only {actual} provided (Article 331-5)"
    )]
    InsufficientDirectors { required: usize, actual: usize },

    /// Invalid director term
    /// (無効な取締役任期 - Mukō na torishimari-yaku ninki)
    #[error("Director term exceeds maximum of {max_years} years (Article 332-1)")]
    DirectorTermTooLong { max_years: u32 },

    /// Insufficient auditors
    /// (監査役数不足 - Kansa-yaku-sū fusoku)
    #[error(
        "Minimum {required} auditors required for audit board, but only {actual} provided (Article 335-3)"
    )]
    InsufficientAuditors { required: usize, actual: usize },

    /// Insufficient outside auditors
    /// (社外監査役数不足 - Shagai kansa-yaku-sū fusoku)
    #[error(
        "At least half of auditors must be outside auditors for large companies (Article 335-3)"
    )]
    InsufficientOutsideAuditors { outside: usize, total: usize },

    /// Invalid auditor term
    /// (無効な監査役任期 - Mukō na kansa-yaku ninki)
    #[error("Auditor term exceeds maximum of 4 years (Article 336-1)")]
    AuditorTermTooLong,

    /// Quorum not met
    /// (定足数未達 - Teisoku-sū mittatsu)
    #[error(
        "Quorum not met: {present} of {total} voting rights present (required: {required_percent}%)"
    )]
    QuorumNotMet {
        present: u64,
        total: u64,
        required_percent: u32,
    },

    /// Insufficient votes for approval
    /// (承認に必要な票数不足 - Shōnin ni hitsuyō na hyō-sū fusoku)
    #[error(
        "Resolution failed: {favor} favor, {against} against (required: {required_percent}% of {base} votes)"
    )]
    InsufficientVotes {
        favor: u64,
        against: u64,
        base: u64,
        required_percent: u32,
    },

    // ========================================================================
    // Share-related Errors (株式関連エラー)
    // ========================================================================
    /// Share transfer without required approval
    /// (必要な承認なしの株式譲渡 - Hitsuyō na shōnin nashi no kabushiki jōto)
    #[error("Share transfer requires board approval but approval not obtained")]
    ShareTransferApprovalRequired,

    /// Invalid share issuance
    /// (無効な株式発行 - Mukō na kabushiki hakkō)
    #[error("Invalid share issuance: {reason}")]
    InvalidShareIssuance { reason: String },

    /// Preemptive rights violation
    /// (優先引受権の侵害 - Yūsen hikiuke-ken no shingai)
    #[error("Preemptive rights of existing shareholders not offered (Article 202)")]
    PreemptiveRightsViolation,

    /// Share issuance exceeds authorized shares
    /// (発行可能株式総数超過 - Hakkō kanō kabushiki sōsū chōka)
    #[error("Share issuance would exceed authorized shares: {new_total} > {authorized}")]
    ExceedsAuthorizedShares { new_total: u64, authorized: u64 },

    // ========================================================================
    // Commercial Transaction Errors (商取引エラー)
    // ========================================================================
    /// Invalid commercial transaction
    /// (無効な商取引 - Mukō na shō torihiki)
    #[error("Invalid commercial transaction: {reason}")]
    InvalidCommercialTransaction { reason: String },

    /// Missing merchant registration
    /// (商人登記なし - Shōnin tōki nashi)
    #[error("Merchant registration required for this transaction type")]
    MerchantRegistrationRequired,

    // ========================================================================
    // General Errors (一般エラー)
    // ========================================================================
    /// Missing required field
    /// (必須フィールド未入力 - Hissu fīrudo mi nyūryoku)
    #[error("Missing required field: {field_name}")]
    MissingRequiredField { field_name: String },

    /// Invalid date
    /// (無効な日付 - Mukō na hizuke)
    #[error("Invalid date: {reason}")]
    InvalidDate { reason: String },

    /// Date out of range
    /// (日付が範囲外 - Hizuke ga han'i-gai)
    #[error("{field} date {date} is out of valid range")]
    DateOutOfRange { field: String, date: String },

    /// Generic validation error
    /// (汎用バリデーションエラー - Han'yō baridēshon erā)
    #[error("Validation error: {message}")]
    ValidationError { message: String },
}

/// Result type for commercial law operations
pub type Result<T> = std::result::Result<T, CommercialLawError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = CommercialLawError::CapitalBelowMinimum {
            actual: 0,
            minimum: 1,
        };
        assert!(error.to_string().contains("Capital amount"));

        let error = CommercialLawError::NoBusinessPurposes;
        assert!(error.to_string().contains("business purpose"));

        let error = CommercialLawError::InsufficientDirectors {
            required: 3,
            actual: 2,
        };
        assert!(error.to_string().contains("Minimum 3 directors"));
    }

    #[test]
    fn test_error_equality() {
        let error1 = CommercialLawError::NoIncorporators;
        let error2 = CommercialLawError::NoIncorporators;
        assert_eq!(error1, error2);
    }
}
