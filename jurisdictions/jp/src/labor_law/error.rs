//! Labor Law Error Types (労働法のエラー型)
//!
//! Comprehensive error types for labor law validation and compliance.

use thiserror::Error;

/// Labor law errors (労働法のエラー)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum LaborLawError {
    // ========================================================================
    // Working Hours Violations (労働時間違反)
    // ========================================================================
    /// Exceeds statutory working hours
    /// (法定労働時間超過 - Hōtei rōdō jikan chōka)
    #[error(
        "Working hours {actual} hours/day exceeds statutory limit of {statutory} hours (Article 32)"
    )]
    ExceedsStatutoryDailyHours { actual: f64, statutory: u32 },

    /// Exceeds statutory weekly hours
    /// (週法定労働時間超過 - Shū hōtei rōdō jikan chōka)
    #[error(
        "Weekly working hours {actual} hours exceeds statutory limit of {statutory} hours (Article 32)"
    )]
    ExceedsStatutoryWeeklyHours { actual: u32, statutory: u32 },

    /// Insufficient rest period
    /// (休憩時間不足 - Kyūkei jikan fusoku)
    #[error(
        "Rest period of {actual} minutes is insufficient for {working_hours} hour shift (required: {required} minutes, Article 34)"
    )]
    InsufficientRestPeriod {
        actual: u32,
        required: u32,
        working_hours: f64,
    },

    /// Insufficient days off
    /// (休日日数不足 - Kyūjitsu nissū fusoku)
    #[error("Days off {actual} per week is below statutory minimum of {required} (Article 35)")]
    InsufficientDaysOff { actual: u32, required: u32 },

    /// Overtime exceeds monthly limit
    /// (時間外労働上限超過 - Jikan-gai rōdō jōgen chōka)
    #[error(
        "Monthly overtime {actual} hours exceeds limit of {limit} hours (Article 36 Agreement required)"
    )]
    ExceedsMonthlyOvertimeLimit { actual: f64, limit: u32 },

    // ========================================================================
    // Wage Violations (賃金違反)
    // ========================================================================
    /// Below minimum wage
    /// (最低賃金未満 - Saitei chingin miman)
    #[error(
        "Hourly wage ¥{actual_hourly} is below minimum wage of ¥{required_minimum} in {prefecture:?} (Minimum Wage Act)"
    )]
    BelowMinimumWage {
        actual_hourly: u64,
        required_minimum: u64,
        prefecture: crate::labor_law::minimum_wage::Prefecture,
    },

    /// Invalid wage deduction
    /// (無効な賃金控除 - Mukō na chingin kōjo)
    #[error("Invalid wage deduction: {reason} (Article 24 - Full payment principle)")]
    InvalidWageDeduction { reason: String },

    /// Late wage payment
    /// (賃金支払遅延 - Chingin shiharai chien)
    #[error(
        "Wage payment delayed: payment date {payment_date} is after period end {period_end} (Article 24)"
    )]
    LateWagePayment {
        payment_date: String,
        period_end: String,
    },

    /// Incorrect overtime premium
    /// (時間外手当計算誤り - Jikan-gai teate keisan ayamari)
    #[error("Overtime premium ¥{actual} is less than required ¥{required} (Article 37)")]
    IncorrectOvertimePremium { actual: u64, required: u64 },

    // ========================================================================
    // Termination Violations (解雇・退職違反)
    // ========================================================================
    /// Insufficient notice period
    /// (解雇予告期間不足 - Kaiko yokoku kikan fusoku)
    #[error(
        "Termination notice period {actual} days is less than required {required} days (Article 20)"
    )]
    InsufficientNotice { actual: i64, required: u32 },

    /// Missing notice allowance
    /// (解雇予告手当未払い - Kaiko yokoku teate mibarai)
    #[error(
        "Insufficient notice period requires payment of ¥{required} notice allowance (Article 20)"
    )]
    MissingNoticeAllowance { required: u64 },

    /// Abusive dismissal
    /// (解雇権濫用 - Kaiko-ken ran'yō)
    #[error(
        "Dismissal may constitute abuse of dismissal rights: {reason} (Labor Contract Act Article 16)"
    )]
    AbusiveDismissal { reason: String },

    /// Invalid dismissal during protected period
    /// (保護期間中の解雇 - Hogo kikan-chū no kaiko)
    #[error("Dismissal during protected period is prohibited: {reason} (Article 19)")]
    DismissalDuringProtectedPeriod { reason: String },

    // ========================================================================
    // Contract Violations (契約違反)
    // ========================================================================
    /// Fixed-term contract exceeds limit
    /// (有期契約期間超過 - Yūki keiyaku kikan chōka)
    #[error(
        "Fixed-term contract duration {duration_years} years exceeds maximum (Labor Contract Act Article 17)"
    )]
    FixedTermExceedsLimit { duration_years: f64 },

    /// Missing required contract terms
    /// (必須契約条項欠如 - Hissu keiyaku jōkō ketsujo)
    #[error("Employment contract missing required terms: {missing_terms} (Article 15)")]
    MissingContractTerms { missing_terms: String },

    /// Invalid probation period
    /// (無効な試用期間 - Mukō na shiyō kikan)
    #[error("Probation period {actual} days exceeds reasonable limit (typically 90-180 days)")]
    InvalidProbationPeriod { actual: u32 },

    /// Contract renewal expectation violation
    /// (契約更新期待権侵害 - Keiyaku kōshin kitai-ken shingai)
    #[error(
        "Non-renewal violates legitimate expectation after {renewal_count} renewals (Article 19)"
    )]
    RenewalExpectationViolation { renewal_count: u32 },

    // ========================================================================
    // Harassment Violations (ハラスメント)
    // ========================================================================
    /// Power harassment detected
    /// (パワーハラスメント検出 - Pawā harasumento kenshutsu)
    #[error("Potential power harassment detected: {description}")]
    PowerHarassmentDetected { description: String },

    /// Sexual harassment detected
    /// (セクシャルハラスメント検出 - Sekusharu harasumento kenshutsu)
    #[error("Potential sexual harassment detected: {description}")]
    SexualHarassmentDetected { description: String },

    /// Maternity harassment detected
    /// (マタニティハラスメント検出 - Mataniti harasumento kenshutsu)
    #[error("Potential maternity harassment detected: {description}")]
    MaternityHarassmentDetected { description: String },

    // ========================================================================
    // Discrimination Violations (差別禁止違反)
    // ========================================================================
    /// Gender discrimination
    /// (性別差別 - Seibetsu sabetsu)
    #[error("Potential gender discrimination: {reason} (Equal Employment Opportunity Act)")]
    GenderDiscrimination { reason: String },

    /// Age discrimination
    /// (年齢差別 - Nenrei sabetsu)
    #[error("Potential age discrimination: {reason}")]
    AgeDiscrimination { reason: String },

    // ========================================================================
    // Work Rules Violations (就業規則違反)
    // ========================================================================
    /// Work rules not properly established
    /// (就業規則未整備 - Shūgyō kisoku mi-seibi)
    #[error(
        "Work rules required for employers with 10+ employees but not established (Article 89)"
    )]
    WorkRulesNotEstablished,

    /// Work rules not reported to authorities
    /// (就業規則未届 - Shūgyō kisoku mi-todoke)
    #[error("Work rules must be reported to labor standards office (Article 89)")]
    WorkRulesNotReported,

    // ========================================================================
    // General Errors (一般エラー)
    // ========================================================================
    /// Missing required field
    /// (必須フィールド未入力 - Hissu fīrudo mi-nyūryoku)
    #[error("Missing required field: {field_name}")]
    MissingRequiredField { field_name: String },

    /// Invalid date
    /// (無効な日付 - Mukō na hizuke)
    #[error("Invalid date: {reason}")]
    InvalidDate { reason: String },

    /// Date out of range
    /// (日付範囲外 - Hizuke han'i-gai)
    #[error("{field} date is out of valid range")]
    DateOutOfRange { field: String },

    /// Good faith principle violation
    /// (信義誠実の原則違反 - Shin'gi seijitsu no gensoku ihan)
    #[error("Action violates good faith principle: {reason} (Labor Contract Act Article 3)")]
    GoodFaithViolation { reason: String },

    /// Invalid calculation
    /// (無効な計算 - Mukō na keisan)
    #[error("Invalid calculation: {reason}")]
    InvalidCalculation { reason: String },

    /// Invalid contract type
    /// (無効な契約種別 - Mukō na keiyaku shubetsu)
    #[error("Invalid contract type: expected {expected}, got {actual}")]
    InvalidContractType { expected: String, actual: String },

    /// Not eligible for indefinite conversion
    /// (無期転換権なし - Muki tenkan-ken nashi)
    #[error(
        "Not eligible for indefinite conversion: worked {years_worked} years, requires {required_years} years (Labor Contract Act Article 18)"
    )]
    NotEligibleForConversion {
        years_worked: f64,
        required_years: u32,
    },

    /// Adverse change to employment terms
    /// (不利益変更 - Furieki henkō)
    #[error("Adverse change prohibited: {field} - {reason}")]
    AdverseChange { field: String, reason: String },

    /// Generic validation error
    /// (汎用バリデーションエラー - Han'yō baridēshon erā)
    #[error("Labor law validation error: {message}")]
    ValidationError { message: String },
}

/// Result type for labor law operations
pub type Result<T> = std::result::Result<T, LaborLawError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = LaborLawError::ExceedsStatutoryDailyHours {
            actual: 10.0,
            statutory: 8,
        };
        assert!(error.to_string().contains("10"));
        assert!(error.to_string().contains("Article 32"));

        let error = LaborLawError::BelowMinimumWage {
            actual_hourly: 800,
            required_minimum: 1000,
            prefecture: crate::labor_law::minimum_wage::Prefecture::Tokyo,
        };
        assert!(error.to_string().contains("minimum wage"));

        let error = LaborLawError::InsufficientNotice {
            actual: 15,
            required: 30,
        };
        assert!(error.to_string().contains("Article 20"));
    }

    #[test]
    fn test_error_equality() {
        let error1 = LaborLawError::WorkRulesNotEstablished;
        let error2 = LaborLawError::WorkRulesNotEstablished;
        assert_eq!(error1, error2);
    }
}
