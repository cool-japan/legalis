//! Consumer Protection Error Types (消費者保護法のエラー型)
//!
//! Comprehensive error types for consumer protection law validation.

use thiserror::Error;

/// Consumer protection errors (消費者保護法のエラー)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ConsumerProtectionError {
    // ========================================================================
    // Unfair Terms Errors (不当条項エラー)
    // ========================================================================
    /// Full exemption clause detected
    /// (全部免責条項 - Zenbu menjo jōkō - Article 8-1-1)
    #[error("Full exemption from liability clause is invalid (Article 8-1-1): {description}")]
    FullExemptionClause { description: String },

    /// Partial exemption clause detected
    /// (一部免責条項 - Ichibu menjo jōkō - Article 8-1-2/3)
    #[error("Partial exemption clause may be invalid (Article 8-1-2/3): {description}")]
    PartialExemptionClause { description: String },

    /// Excessive penalty clause
    /// (過大な損害賠償予定 - Kadai na songai baishō yotei - Article 9-1)
    #[error("Penalty clause ¥{penalty} exceeds average damages ¥{average} (Article 9-1)")]
    ExcessivePenalty { penalty: u64, average: u64 },

    /// Excessive cancellation fee
    /// (過大な解除料 - Kadai na kaijo-ryō - Article 9-1)
    #[error("Cancellation fee ¥{fee} ({percentage:.1}%) is excessive (Article 9-1)")]
    ExcessiveCancellationFee { fee: u64, percentage: f64 },

    /// Consumer disadvantage clause
    /// (消費者の利益を一方的に害する条項 - Article 10)
    #[error("Clause unfairly disadvantages consumer (Article 10): {description}")]
    ConsumerDisadvantageClause { description: String },

    /// Unreasonable burden on consumer
    /// (不当に重い義務 - Futō ni omoi gimu)
    #[error("Clause imposes unreasonable burden on consumer: {description}")]
    UnreasonableBurden { description: String },

    // ========================================================================
    // Rescission Errors (取消権エラー)
    // ========================================================================
    /// Misrepresentation detected
    /// (不実告知 - Fujitsu kokuchi - Article 4-1-1)
    #[error("Misrepresentation of important facts (Article 4-1-1): {description}")]
    Misrepresentation { description: String },

    /// Definite judgment provided
    /// (断定的判断の提供 - Danteい-teki handan no teikyō - Article 4-1-2)
    #[error("Definite judgment regarding uncertain matters (Article 4-1-2): {description}")]
    DefiniteJudgment { description: String },

    /// Non-disclosure of disadvantages
    /// (不利益事実の不告知 - Furieki jijitsu no fukokuchi - Article 4-2)
    #[error("Failure to disclose disadvantageous facts (Article 4-2): {description}")]
    NonDisclosure { description: String },

    /// Undue influence detected
    /// (過量な契約 - Karyō na keiyaku - Article 4-3)
    #[error("Contract induced by undue influence (Article 4-3): {description}")]
    UndueInfluence { description: String },

    /// Threatening behavior
    /// (威迫 - Ihaku - Article 4-3-3)
    #[error("Contract induced by threatening behavior: {description}")]
    Threat { description: String },

    /// Rescission period expired
    /// (取消期間経過 - Torikeshi kikan keika - Article 7)
    #[error(
        "Rescission period expired: {months_since_contract} months since contract (max: 60 months)"
    )]
    RescissionPeriodExpired { months_since_contract: i64 },

    // ========================================================================
    // Cooling-Off Errors (クーリング・オフエラー)
    // ========================================================================
    /// Cooling-off period expired
    /// (クーリング・オフ期間経過 - Kūringu ofu kikan keika)
    #[error("Cooling-off period expired: deadline was {deadline}")]
    CoolingOffExpired { deadline: String },

    /// Cooling-off not applicable
    /// (クーリング・オフ適用外 - Kūringu ofu tekiyō-gai)
    #[error("Cooling-off not applicable for {transaction_type:?}")]
    CoolingOffNotApplicable { transaction_type: String },

    /// Cooling-off notice not provided
    /// (クーリング・オフ告知義務違反 - Kokuchi gimu ihan)
    #[error("Required cooling-off notice was not provided to consumer")]
    CoolingOffNoticeNotProvided,

    /// Improper cooling-off exercise
    /// (不適切なクーリング・オフ行使 - Futekisetsu na kōshi)
    #[error("Cooling-off exercise invalid: {reason}")]
    ImproperCoolingOffExercise { reason: String },

    // ========================================================================
    // SCTA Violations (特定商取引法違反)
    // ========================================================================
    /// Required disclosure not made
    /// (法定書面不交付 - Hōtei shomen fu-kōfu)
    #[error("Required contract documents not provided (Specified Commercial Transactions Act)")]
    RequiredDisclosureNotMade,

    /// Prohibited sales method
    /// (禁止販売方法 - Kinshi hanbai hōhō)
    #[error("Prohibited sales method used: {method}")]
    ProhibitedSalesMethod { method: String },

    /// False/exaggerated advertising
    /// (誇大広告 - Kodai kōkoku)
    #[error("False or exaggerated advertising detected: {description}")]
    FalseAdvertising { description: String },

    // ========================================================================
    // Contract Errors (契約エラー)
    // ========================================================================
    /// Missing required field
    /// (必須フィールド未入力 - Hissu fīrudo mi-nyūryoku)
    #[error("Missing required field: {field_name}")]
    MissingRequiredField { field_name: String },

    /// Invalid contract terms
    /// (無効な契約条項 - Mukō na keiyaku jōkō)
    #[error("Invalid contract terms: {reason}")]
    InvalidContractTerms { reason: String },

    /// Invalid date
    /// (無効な日付 - Mukō na hizuke)
    #[error("Invalid date: {reason}")]
    InvalidDate { reason: String },

    /// Insufficient contract information
    /// (契約情報不足 - Keiyaku jōhō fusoku)
    #[error("Insufficient contract information: {description}")]
    InsufficientInformation { description: String },

    /// Generic validation error
    /// (汎用バリデーションエラー - Han'yō baridēshon erā)
    #[error("Consumer protection validation error: {message}")]
    ValidationError { message: String },
}

/// Result type for consumer protection operations
pub type Result<T> = std::result::Result<T, ConsumerProtectionError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = ConsumerProtectionError::ExcessivePenalty {
            penalty: 200_000,
            average: 100_000,
        };
        assert!(error.to_string().contains("200000"));
        assert!(error.to_string().contains("Article 9-1"));

        let error = ConsumerProtectionError::Misrepresentation {
            description: "False claims".to_string(),
        };
        assert!(error.to_string().contains("Misrepresentation"));
        assert!(error.to_string().contains("Article 4-1-1"));

        let error = ConsumerProtectionError::CoolingOffExpired {
            deadline: "2024-01-01".to_string(),
        };
        assert!(error.to_string().contains("expired"));
    }

    #[test]
    fn test_error_equality() {
        let error1 = ConsumerProtectionError::CoolingOffNoticeNotProvided;
        let error2 = ConsumerProtectionError::CoolingOffNoticeNotProvided;
        assert_eq!(error1, error2);
    }
}
