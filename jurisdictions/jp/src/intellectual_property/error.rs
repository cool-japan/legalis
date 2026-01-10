//! Intellectual Property Error Types (知的財産法のエラー型)
//!
//! Comprehensive error types for intellectual property law validation and operations.

use thiserror::Error;

/// Intellectual property errors (知的財産法のエラー)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum IntellectualPropertyError {
    // ========================================================================
    // Patent Errors (特許法エラー)
    // ========================================================================
    /// Lacks industrial applicability
    /// (産業上の利用可能性なし - Article 29 preamble)
    #[error("Invention lacks industrial applicability (Article 29)")]
    LacksIndustrialApplicability,

    /// Lacks novelty
    /// (新規性なし - Shinki-sei nashi - Article 29-1)
    #[error("Invention lacks novelty: {reason} (Article 29-1)")]
    LacksNovelty { reason: String },

    /// Lacks inventive step
    /// (進歩性なし - Shinpo-sei nashi - Article 29-2)
    #[error("Invention lacks inventive step: {reason} (Article 29-2)")]
    LacksInventiveStep { reason: String },

    /// Prior art exists
    /// (先行技術あり - Senkō gijutsu ari)
    #[error("Prior art exists: {description}")]
    PriorArtExists { description: String },

    /// Insufficient disclosure
    /// (開示不十分 - Kaiji fujūbun - Article 36)
    #[error("Patent application has insufficient disclosure (Article 36)")]
    InsufficientDisclosure,

    /// Claims not supported
    /// (請求項がサポートされていない - Article 36-6-1)
    #[error("Claims not supported by specification (Article 36-6-1)")]
    ClaimsNotSupported,

    /// Patent expired
    /// (特許権期間満了 - Tokkyo-ken kikan manryō)
    #[error("Patent protection expired on {expiry_date}")]
    PatentExpired { expiry_date: String },

    /// Annual fees not paid
    /// (年金未納 - Nenkin minō)
    #[error("Patent annual fees not paid for year {year}")]
    AnnualFeesNotPaid { year: u32 },

    /// Patent infringement detected
    /// (特許権侵害 - Tokkyo-ken shingai - Article 68)
    #[error("Patent infringement detected: {description}")]
    PatentInfringement { description: String },

    // ========================================================================
    // Copyright Errors (著作権法エラー)
    // ========================================================================
    /// Not a copyrightable work
    /// (著作物に該当しない - Article 2-1-1)
    #[error("Work does not qualify as copyrightable: {reason} (Article 2-1-1)")]
    NotCopyrightable { reason: String },

    /// Lacks originality
    /// (創作性なし - Sōsaku-sei nashi)
    #[error("Work lacks sufficient originality for copyright protection")]
    LacksOriginality,

    /// Copyright expired
    /// (著作権期間満了 - Chosakuken kikan manryō)
    #[error("Copyright protection expired on {expiry_date} (Article 51)")]
    CopyrightExpired { expiry_date: String },

    /// Copyright infringement
    /// (著作権侵害 - Chosakuken shingai)
    #[error("Copyright infringement: {description}")]
    CopyrightInfringement { description: String },

    /// Reproduction right violation
    /// (複製権侵害 - Fukusei-ken shingai - Article 21)
    #[error("Reproduction right violated: {description}")]
    ReproductionRightViolation { description: String },

    /// Public transmission right violation
    /// (公衆送信権侵害 - Kōshū sōshin-ken shingai - Article 23)
    #[error("Public transmission right violated: {description}")]
    PublicTransmissionViolation { description: String },

    /// Moral rights violation
    /// (著作者人格権侵害 - Chosakusha jinkaku-ken shingai)
    #[error("Moral rights violated: {right_type}")]
    MoralRightsViolation { right_type: String },

    /// Fair use not applicable
    /// (権利制限の適用不可 - Ken'ri seigen no tekiyō fuka)
    #[error("Fair use defense not applicable: {reason}")]
    FairUseNotApplicable { reason: String },

    /// Invalid quotation
    /// (不適切な引用 - Futekisetsu na in'yō - Article 32)
    #[error("Quotation does not meet fair use requirements: {reason}")]
    InvalidQuotation { reason: String },

    // ========================================================================
    // Trademark Errors (商標法エラー)
    // ========================================================================
    /// Not distinctive
    /// (識別力なし - Shikibetsu-ryoku nashi - Article 3)
    #[error("Trademark lacks distinctiveness (Article 3)")]
    LacksDistinctiveness,

    /// Descriptive mark
    /// (記述的商標 - Kijutsu-teki shōhyō - Article 3-1-3)
    #[error("Trademark is merely descriptive: {description}")]
    MerelyDescriptive { description: String },

    /// Generic term
    /// (普通名称 - Futsū meishō - Article 3-1-1)
    #[error("Trademark is a generic term: {term}")]
    GenericTerm { term: String },

    /// Confusingly similar
    /// (混同のおそれ - Kondō no osore - Article 4-1-11)
    #[error("Trademark is confusingly similar to existing mark: {existing_mark}")]
    ConfusinglySimilar { existing_mark: String },

    /// Trademark registration expired
    /// (商標権期間満了 - Shōhyō-ken kikan manryō)
    #[error("Trademark registration expired on {expiry_date}")]
    TrademarkExpired { expiry_date: String },

    /// Renewal not filed
    /// (更新未申請 - Kōshin mi-shinsei)
    #[error("Trademark renewal not filed before deadline")]
    RenewalNotFiled,

    /// Trademark infringement
    /// (商標権侵害 - Shōhyō-ken shingai - Article 25)
    #[error("Trademark infringement: {description}")]
    TrademarkInfringement { description: String },

    /// Invalid class designation
    /// (無効な区分指定 - Mukō na kubun shitei)
    #[error("Invalid Nice Classification: class {class} is out of range (1-45)")]
    InvalidClassDesignation { class: u8 },

    // ========================================================================
    // Design Errors (意匠法エラー)
    // ========================================================================
    /// Not a registrable design
    /// (登録意匠に該当しない - Article 3)
    #[error("Design is not registrable: {reason} (Article 3)")]
    NotRegistrableDesign { reason: String },

    /// Lacks novelty (design)
    /// (新規性なし - Article 3-1-1)
    #[error("Design lacks novelty: {reason}")]
    DesignLacksNovelty { reason: String },

    /// Design registration expired
    /// (意匠権期間満了 - Ishō-ken kikan manryō)
    #[error("Design protection expired on {expiry_date}")]
    DesignExpired { expiry_date: String },

    /// Design infringement
    /// (意匠権侵害 - Ishō-ken shingai - Article 23)
    #[error("Design infringement: {description}")]
    DesignInfringement { description: String },

    /// Similar design exists
    /// (類似意匠あり - Ruiji ishō ari)
    #[error("Similar design already registered: {existing_design}")]
    SimilarDesignExists { existing_design: String },

    // ========================================================================
    // General Errors (一般エラー)
    // ========================================================================
    /// Missing required field
    /// (必須フィールド未入力 - Hissu fīrudo mi-nyūryoku)
    #[error("Missing required field: {field_name}")]
    MissingRequiredField { field_name: String },

    /// Invalid application number
    /// (無効な出願番号 - Mukō na shutsugan bangō)
    #[error("Invalid application number format: {number}")]
    InvalidApplicationNumber { number: String },

    /// Invalid registration number
    /// (無効な登録番号 - Mukō na tōroku bangō)
    #[error("Invalid registration number format: {number}")]
    InvalidRegistrationNumber { number: String },

    /// Invalid date
    /// (無効な日付 - Mukō na hizuke)
    #[error("Invalid date: {reason}")]
    InvalidDate { reason: String },

    /// Filing deadline missed
    /// (出願期限超過 - Shutsugan kigen chōka)
    #[error("Filing deadline missed: {deadline}")]
    FilingDeadlineMissed { deadline: String },

    /// Priority claim invalid
    /// (優先権主張無効 - Yūsen-ken shuchō mukō)
    #[error("Priority claim invalid: {reason}")]
    InvalidPriorityClaim { reason: String },

    /// Generic validation error
    /// (汎用バリデーションエラー - Han'yō baridēshon erā)
    #[error("Intellectual property validation error: {message}")]
    ValidationError { message: String },
}

/// Result type for intellectual property operations
pub type Result<T> = std::result::Result<T, IntellectualPropertyError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = IntellectualPropertyError::LacksNovelty {
            reason: "Prior publication exists".to_string(),
        };
        assert!(error.to_string().contains("novelty"));
        assert!(error.to_string().contains("Article 29-1"));

        let error = IntellectualPropertyError::CopyrightInfringement {
            description: "Unauthorized reproduction".to_string(),
        };
        assert!(error.to_string().contains("Copyright infringement"));

        let error = IntellectualPropertyError::TrademarkExpired {
            expiry_date: "2020-01-01".to_string(),
        };
        assert!(error.to_string().contains("expired"));
    }

    #[test]
    fn test_error_equality() {
        let error1 = IntellectualPropertyError::LacksDistinctiveness;
        let error2 = IntellectualPropertyError::LacksDistinctiveness;
        assert_eq!(error1, error2);
    }
}
