//! Labor Law Types (労働法型定義)
//!
//! Type definitions for Japanese labor law, including:
//! - Labor Standards Act (労働基準法 - Rōdō Kijun-hō)
//! - Labor Contract Act (労働契約法 - Rōdō Keiyaku-hō)
//! - Employment regulations and contracts
//!
//! # Legal References
//! - Labor Standards Act (Act No. 49 of 1947) - 労働基準法
//! - Labor Contract Act (Act No. 128 of 2007) - 労働契約法
//! - Minimum Wage Act (Act No. 137 of 1959) - 最低賃金法

use chrono::{DateTime, Duration, Timelike, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Labor Standards Act (労働基準法)
// ============================================================================

/// Statutory working hours per day (Article 32)
/// 法定労働時間（1日） - 8時間
pub const STATUTORY_HOURS_PER_DAY: u32 = 8;

/// Statutory working hours per week (Article 32)
/// 法定労働時間（1週） - 40時間
pub const STATUTORY_HOURS_PER_WEEK: u32 = 40;

/// Minimum rest period for 6+ hour shifts (Article 34)
/// 最低休憩時間（6時間超） - 45分
pub const MIN_REST_6_HOURS_MINUTES: u32 = 45;

/// Minimum rest period for 8+ hour shifts (Article 34)
/// 最低休憩時間（8時間超） - 60分
pub const MIN_REST_8_HOURS_MINUTES: u32 = 60;

/// Minimum days off per week (Article 35)
/// 週最低休日数 - 1日
pub const MIN_DAYS_OFF_PER_WEEK: u32 = 1;

/// Advance notice period for termination (Article 20)
/// 解雇予告期間 - 30日
pub const TERMINATION_NOTICE_DAYS: u32 = 30;

/// Overtime premium rate (Article 37)
/// 時間外割増率 - 25%
pub const OVERTIME_PREMIUM_RATE: f64 = 0.25;

/// Late night premium rate (Article 37)
/// 深夜割増率（22:00-5:00） - 25%
pub const LATE_NIGHT_PREMIUM_RATE: f64 = 0.25;

/// Holiday work premium rate (Article 37)
/// 休日労働割増率 - 35%
pub const HOLIDAY_WORK_PREMIUM_RATE: f64 = 0.35;

/// Monthly overtime limit (60 hours)
/// 月間時間外労働上限 - 60時間（超過は50%割増）
pub const MONTHLY_OVERTIME_LIMIT: u32 = 60;

/// Overtime over limit premium rate (Article 37)
/// 時間外労働超過割増率（60時間超） - 50%
pub const OVERTIME_OVER_LIMIT_RATE: f64 = 0.50;

/// Fixed-term contract conversion threshold (Labor Contract Act Article 18)
/// 無期転換ルール - 5年
pub const FIXED_TERM_CONVERSION_YEARS: u32 = 5;

// ============================================================================
// Employment Contract Types (雇用契約型)
// ============================================================================

/// Employment contract type (雇用形態)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EmploymentType {
    /// Indefinite-term contract (無期雇用 - Muki koyō)
    IndefiniteTerm,

    /// Fixed-term contract (有期雇用 - Yūki koyō)
    FixedTerm,

    /// Part-time (パートタイム - Pāto taimu)
    PartTime,

    /// Temporary/Dispatch (派遣 - Haken)
    Temporary,

    /// Contract worker (契約社員 - Keiyaku shain)
    ContractWorker,
}

/// Work pattern (勤務形態)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WorkPattern {
    /// Regular hours (通常勤務 - Tsūjō kinmu)
    Regular,

    /// Flextime (フレックスタイム - Furrekkusu taimu)
    Flextime,

    /// Shift work (シフト制 - Shifuto-sei)
    Shift,

    /// Discretionary work (裁量労働制 - Sairyō rōdō-sei)
    Discretionary,
}

/// Employment contract (雇用契約 - Koyō keiyaku)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EmploymentContract {
    /// Employee name (従業員名 - Jūgyō-in mei)
    pub employee_name: String,

    /// Employer name (使用者名 - Shiyō-sha mei)
    pub employer_name: String,

    /// Employment type (雇用形態 - Koyō keitai)
    pub employment_type: EmploymentType,

    /// Work pattern (勤務形態 - Kinmu keitai)
    pub work_pattern: WorkPattern,

    /// Start date (契約開始日 - Keiyaku kaishi-bi)
    pub start_date: DateTime<Utc>,

    /// End date for fixed-term contracts (契約終了日 - Keiyaku shūryō-bi)
    pub end_date: Option<DateTime<Utc>>,

    /// Base wage in JPY (基本給 - Kihon-kyū)
    pub base_wage_jpy: u64,

    /// Working hours per day (1日の労働時間 - Ichinichi no rōdō jikan)
    pub hours_per_day: u32,

    /// Working days per week (週の労働日数 - Shū no rōdō nissū)
    pub days_per_week: u32,

    /// Job description (職務内容 - Shokumu naiyō)
    pub job_description: String,

    /// Work location (勤務場所 - Kinmu basho)
    pub work_location: String,

    /// Probation period in days (試用期間 - Shiyō kikan)
    pub probation_period_days: Option<u32>,

    /// Contract renewal history (契約更新回数 - Keiyaku kōshin kaisū)
    pub renewal_count: u32,
}

impl EmploymentContract {
    /// Check if contract is eligible for indefinite-term conversion (Article 18)
    /// 無期転換ルール適用判定
    pub fn is_eligible_for_indefinite_conversion(&self) -> bool {
        if self.employment_type != EmploymentType::FixedTerm {
            return false;
        }

        let years_since_start = (Utc::now() - self.start_date).num_days() / 365;
        years_since_start >= FIXED_TERM_CONVERSION_YEARS as i64
    }

    /// Calculate weekly working hours (週の労働時間)
    pub fn weekly_hours(&self) -> u32 {
        self.hours_per_day * self.days_per_week
    }
}

// ============================================================================
// Working Hours & Overtime (労働時間・残業)
// ============================================================================

/// Time period for work calculation (労働時間帯)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TimePeriod {
    /// Regular hours (通常時間 - Tsūjō jikan)
    Regular,

    /// Overtime (時間外 - Jikan-gai)
    Overtime,

    /// Late night (22:00-5:00) (深夜 - Shin'ya)
    LateNight,

    /// Holiday (休日 - Kyūjitsu)
    Holiday,

    /// Overtime + Late night (時間外深夜 - Jikan-gai shin'ya)
    OvertimeLateNight,

    /// Holiday + Late night (休日深夜 - Kyūjitsu shin'ya)
    HolidayLateNight,
}

impl TimePeriod {
    /// Get premium rate for this time period (割増率取得)
    pub fn premium_rate(&self) -> f64 {
        match self {
            TimePeriod::Regular => 0.0,
            TimePeriod::Overtime => OVERTIME_PREMIUM_RATE,
            TimePeriod::LateNight => LATE_NIGHT_PREMIUM_RATE,
            TimePeriod::Holiday => HOLIDAY_WORK_PREMIUM_RATE,
            TimePeriod::OvertimeLateNight => OVERTIME_PREMIUM_RATE + LATE_NIGHT_PREMIUM_RATE,
            TimePeriod::HolidayLateNight => HOLIDAY_WORK_PREMIUM_RATE + LATE_NIGHT_PREMIUM_RATE,
        }
    }
}

/// Working time record (労働時間記録 - Rōdō jikan kiroku)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WorkingTimeRecord {
    /// Date (日付 - Hizuke)
    pub date: DateTime<Utc>,

    /// Start time (開始時刻 - Kaishi jikoku)
    pub start_time: DateTime<Utc>,

    /// End time (終了時刻 - Shūryō jikoku)
    pub end_time: DateTime<Utc>,

    /// Rest period in minutes (休憩時間 - Kyūkei jikan)
    pub rest_minutes: u32,

    /// Is holiday work (休日労働フラグ - Kyūjitsu rōdō furaggu)
    pub is_holiday: bool,
}

impl WorkingTimeRecord {
    /// Calculate actual working hours (実労働時間計算)
    pub fn actual_working_hours(&self) -> f64 {
        let total_minutes = (self.end_time - self.start_time).num_minutes() as f64;
        let working_minutes = total_minutes - self.rest_minutes as f64;
        working_minutes / 60.0
    }

    /// Calculate overtime hours (時間外労働時間計算)
    pub fn overtime_hours(&self, statutory_daily_hours: u32) -> f64 {
        let actual = self.actual_working_hours();
        let statutory = statutory_daily_hours as f64;
        if actual > statutory {
            actual - statutory
        } else {
            0.0
        }
    }

    /// Calculate late night hours (深夜労働時間計算)
    pub fn late_night_hours(&self) -> f64 {
        // Late night is 22:00-5:00
        let mut late_night_minutes = 0.0;
        let mut current = self.start_time;

        while current < self.end_time {
            let hour = current.hour();
            if !(5..22).contains(&hour) {
                late_night_minutes += 1.0;
            }
            current += Duration::minutes(1);
        }

        late_night_minutes / 60.0
    }
}

/// Monthly working time summary (月間労働時間集計)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MonthlyWorkingSummary {
    /// Year and month (年月 - Nen-getsu)
    pub year: i32,
    pub month: u32,

    /// Total working hours (総労働時間 - Sō rōdō jikan)
    pub total_hours: f64,

    /// Overtime hours (時間外労働時間 - Jikan-gai rōdō jikan)
    pub overtime_hours: f64,

    /// Late night hours (深夜労働時間 - Shin'ya rōdō jikan)
    pub late_night_hours: f64,

    /// Holiday work hours (休日労働時間 - Kyūjitsu rōdō jikan)
    pub holiday_hours: f64,

    /// Days worked (労働日数 - Rōdō nissū)
    pub days_worked: u32,
}

impl MonthlyWorkingSummary {
    /// Check if overtime exceeds legal limit (時間外労働上限超過判定)
    pub fn exceeds_overtime_limit(&self) -> bool {
        self.overtime_hours > MONTHLY_OVERTIME_LIMIT as f64
    }

    /// Calculate base wage for the month (月間基本給計算)
    pub fn calculate_base_wage(&self, hourly_rate_jpy: u64) -> u64 {
        (self.total_hours * hourly_rate_jpy as f64) as u64
    }

    /// Calculate overtime premium pay (時間外手当計算)
    pub fn calculate_overtime_premium(&self, hourly_rate_jpy: u64) -> u64 {
        let standard_overtime = self.overtime_hours.min(MONTHLY_OVERTIME_LIMIT as f64);
        let excess_overtime = self.overtime_hours - standard_overtime;

        let standard_premium =
            standard_overtime * hourly_rate_jpy as f64 * (1.0 + OVERTIME_PREMIUM_RATE);
        let excess_premium =
            excess_overtime * hourly_rate_jpy as f64 * (1.0 + OVERTIME_OVER_LIMIT_RATE);

        (standard_premium + excess_premium) as u64
    }

    /// Calculate late night premium pay (深夜手当計算)
    pub fn calculate_late_night_premium(&self, hourly_rate_jpy: u64) -> u64 {
        (self.late_night_hours * hourly_rate_jpy as f64 * (1.0 + LATE_NIGHT_PREMIUM_RATE)) as u64
    }

    /// Calculate holiday premium pay (休日手当計算)
    pub fn calculate_holiday_premium(&self, hourly_rate_jpy: u64) -> u64 {
        (self.holiday_hours * hourly_rate_jpy as f64 * (1.0 + HOLIDAY_WORK_PREMIUM_RATE)) as u64
    }

    /// Calculate total wage including all premiums (総賃金計算)
    pub fn calculate_total_wage(&self, hourly_rate_jpy: u64) -> u64 {
        let base = self.calculate_base_wage(hourly_rate_jpy);
        let overtime_premium = self.calculate_overtime_premium(hourly_rate_jpy);
        let late_night_premium = self.calculate_late_night_premium(hourly_rate_jpy);
        let holiday_premium = self.calculate_holiday_premium(hourly_rate_jpy);

        base + overtime_premium + late_night_premium + holiday_premium
    }
}

// ============================================================================
// Termination & Dismissal (解雇・退職)
// ============================================================================

/// Termination type (解雇・退職種別)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TerminationType {
    /// Ordinary dismissal (普通解雇 - Futsū kaiko)
    OrdinaryDismissal,

    /// Disciplinary dismissal (懲戒解雇 - Chōkai kaiko)
    DisciplinaryDismissal,

    /// Voluntary resignation (自己都合退職 - Jiko tsugō taishoku)
    VoluntaryResignation,

    /// Mutual agreement (合意退職 - Gōi taishoku)
    MutualAgreement,

    /// Contract expiration (契約期間満了 - Keiyaku kikan manryō)
    ContractExpiration,

    /// Retirement (定年退職 - Teinen taishoku)
    Retirement,
}

/// Termination notice (解雇予告・退職通知)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TerminationNotice {
    /// Employee name (従業員名 - Jūgyō-in mei)
    pub employee_name: String,

    /// Termination type (解雇・退職種別 - Kaiko/taishoku shubetsu)
    pub termination_type: TerminationType,

    /// Notice date (通知日 - Tsūchi-bi)
    pub notice_date: DateTime<Utc>,

    /// Effective date (発効日 - Hakkō-bi)
    pub effective_date: DateTime<Utc>,

    /// Reason (理由 - Riyū)
    pub reason: String,

    /// Severance pay in JPY (退職金 - Taishoku-kin)
    pub severance_pay_jpy: Option<u64>,

    /// Advance notice allowance (解雇予告手当 - Kaiko yokoku teate)
    pub notice_allowance_jpy: Option<u64>,
}

impl TerminationNotice {
    /// Calculate days between notice and effective date (予告日数計算)
    pub fn notice_period_days(&self) -> i64 {
        (self.effective_date - self.notice_date).num_days()
    }

    /// Check if advance notice period is sufficient (Article 20)
    /// 解雇予告期間の充足判定
    pub fn has_sufficient_notice_period(&self) -> bool {
        match self.termination_type {
            TerminationType::OrdinaryDismissal | TerminationType::DisciplinaryDismissal => {
                // Must provide 30 days notice or pay in lieu
                self.notice_period_days() >= TERMINATION_NOTICE_DAYS as i64
                    || self.notice_allowance_jpy.is_some()
            }
            _ => true, // Other types don't require advance notice
        }
    }

    /// Calculate required notice allowance if insufficient notice (予告手当計算)
    pub fn calculate_required_notice_allowance(&self, average_daily_wage_jpy: u64) -> Option<u64> {
        match self.termination_type {
            TerminationType::OrdinaryDismissal | TerminationType::DisciplinaryDismissal => {
                let notice_days = self.notice_period_days();
                if notice_days < TERMINATION_NOTICE_DAYS as i64 {
                    let shortage_days = TERMINATION_NOTICE_DAYS as i64 - notice_days;
                    Some((shortage_days as u64) * average_daily_wage_jpy)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

// ============================================================================
// Harassment Types (ハラスメント種別)
// ============================================================================

/// Harassment type (ハラスメント種別)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HarassmentType {
    /// Power harassment (パワーハラスメント - Pawā harasumento)
    PowerHarassment,

    /// Sexual harassment (セクシャルハラスメント - Sekusharu harasumento)
    SexualHarassment,

    /// Maternity harassment (マタニティハラスメント - Mataniti harasumento)
    MaternityHarassment,

    /// Paternity harassment (パタニティハラスメント - Pataniti harasumento)
    PaternityHarassment,
}

/// Harassment incident report (ハラスメント報告)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HarassmentReport {
    /// Incident date (発生日 - Hassei-bi)
    pub incident_date: DateTime<Utc>,

    /// Harassment type (ハラスメント種別 - Harasumento shubetsu)
    pub harassment_type: HarassmentType,

    /// Description (内容 - Naiyō)
    pub description: String,

    /// Perpetrator position (加害者役職 - Kagai-sha yakushoku)
    pub perpetrator_position: Option<String>,

    /// Victim position (被害者役職 - Higai-sha yakushoku)
    pub victim_position: Option<String>,

    /// Witnesses (目撃者数 - Mokugeki-sha sū)
    pub witness_count: u32,
}

// ============================================================================
// Wage Payment (賃金支払い)
// ============================================================================

/// Wage payment record (賃金支払記録 - Chingin shiharai kiroku)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WagePayment {
    /// Employee name (従業員名 - Jūgyō-in mei)
    pub employee_name: String,

    /// Payment period start (支払期間開始日 - Shiharai kikan kaishi-bi)
    pub period_start: DateTime<Utc>,

    /// Payment period end (支払期間終了日 - Shiharai kikan shūryō-bi)
    pub period_end: DateTime<Utc>,

    /// Payment date (支払日 - Shiharai-bi)
    pub payment_date: DateTime<Utc>,

    /// Base wage (基本給 - Kihon-kyū)
    pub base_wage_jpy: u64,

    /// Overtime pay (時間外手当 - Jikan-gai teate)
    pub overtime_pay_jpy: u64,

    /// Other allowances (その他手当 - Sonota teate)
    pub other_allowances_jpy: u64,

    /// Deductions (控除 - Kōjo)
    pub deductions_jpy: u64,

    /// Net payment (差引支給額 - Sashibiki shikyū-gaku)
    pub net_payment_jpy: u64,
}

impl WagePayment {
    /// Calculate gross wage (総支給額計算)
    pub fn gross_wage(&self) -> u64 {
        self.base_wage_jpy + self.overtime_pay_jpy + self.other_allowances_jpy
    }

    /// Validate net payment calculation (差引支給額検証)
    pub fn validate_net_payment(&self) -> bool {
        self.gross_wage() - self.deductions_jpy == self.net_payment_jpy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_period_premium_rates() {
        assert_eq!(TimePeriod::Regular.premium_rate(), 0.0);
        assert_eq!(TimePeriod::Overtime.premium_rate(), 0.25);
        assert_eq!(TimePeriod::LateNight.premium_rate(), 0.25);
        assert_eq!(TimePeriod::Holiday.premium_rate(), 0.35);
        assert_eq!(TimePeriod::OvertimeLateNight.premium_rate(), 0.50);
        assert_eq!(TimePeriod::HolidayLateNight.premium_rate(), 0.60);
    }

    #[test]
    fn test_employment_contract_weekly_hours() {
        let contract = EmploymentContract {
            employee_name: "Test Employee".to_string(),
            employer_name: "Test Company".to_string(),
            employment_type: EmploymentType::IndefiniteTerm,
            work_pattern: WorkPattern::Regular,
            start_date: Utc::now(),
            end_date: None,
            base_wage_jpy: 300_000,
            hours_per_day: 8,
            days_per_week: 5,
            job_description: "Test".to_string(),
            work_location: "Tokyo".to_string(),
            probation_period_days: Some(90),
            renewal_count: 0,
        };

        assert_eq!(contract.weekly_hours(), 40);
    }

    #[test]
    fn test_termination_notice_period() {
        let notice = TerminationNotice {
            employee_name: "Test Employee".to_string(),
            termination_type: TerminationType::OrdinaryDismissal,
            notice_date: Utc::now(),
            effective_date: Utc::now() + Duration::days(30),
            reason: "Test reason".to_string(),
            severance_pay_jpy: None,
            notice_allowance_jpy: None,
        };

        assert!(notice.has_sufficient_notice_period());
    }

    #[test]
    fn test_wage_payment_validation() {
        let payment = WagePayment {
            employee_name: "Test Employee".to_string(),
            period_start: Utc::now(),
            period_end: Utc::now() + Duration::days(30),
            payment_date: Utc::now() + Duration::days(31),
            base_wage_jpy: 300_000,
            overtime_pay_jpy: 50_000,
            other_allowances_jpy: 10_000,
            deductions_jpy: 60_000,
            net_payment_jpy: 300_000,
        };

        assert_eq!(payment.gross_wage(), 360_000);
        assert!(payment.validate_net_payment());
    }

    #[test]
    fn test_monthly_summary_overtime_limit() {
        let summary = MonthlyWorkingSummary {
            year: 2026,
            month: 1,
            total_hours: 200.0,
            overtime_hours: 65.0,
            late_night_hours: 10.0,
            holiday_hours: 8.0,
            days_worked: 22,
        };

        assert!(summary.exceeds_overtime_limit());
    }
}

// ============================================================================
// Article 36 Agreement (36協定 - Sanroku Kyōtei)
// ============================================================================

/// Article 36 Agreement for overtime work (36協定)
///
/// Labor-management agreement required under Labor Standards Act Article 36
/// to allow overtime work beyond statutory limits.
///
/// # Legal Basis
///
/// Article 36 of the Labor Standards Act requires an agreement between
/// employer and employee representatives to permit overtime work.
///
/// # Standard Limits (告示第316号)
///
/// - Monthly: 45 hours (特別条項なし)
/// - Yearly: 360 hours (特別条項なし)
///
/// # Special Circumstances (特別条項)
///
/// - Monthly: Up to 100 hours (including holidays, 2-6 months average < 80h)
/// - Yearly: Up to 720 hours
/// - Special months per year: Maximum 6 months
///
/// # Example
///
/// ```
/// use legalis_jp::labor_law::{Article36Agreement};
/// use chrono::{Utc, Duration};
///
/// let agreement = Article36Agreement {
///     employer_name: "株式会社ABC".to_string(),
///     labor_representative: "労働組合ABC支部".to_string(),
///     effective_date: Utc::now().date_naive(),
///     expiration_date: (Utc::now() + Duration::days(365)).date_naive(),
///     max_overtime_per_day: 3,
///     max_overtime_per_month: 45,
///     max_overtime_per_year: 360,
///     has_special_circumstances: false,
///     special_max_per_month: None,
///     special_months_per_year: None,
///     permitted_reasons: vec!["臨時の業務増加".to_string()],
/// };
///
/// assert!(agreement.is_within_standard_limits());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Article36Agreement {
    /// Employer name (使用者名)
    pub employer_name: String,

    /// Labor union or employee representative (労働組合・労働者代表)
    pub labor_representative: String,

    /// Effective date (効力発生日)
    pub effective_date: chrono::NaiveDate,

    /// Expiration date (有効期限)
    pub expiration_date: chrono::NaiveDate,

    /// Maximum overtime per day (1日の時間外労働上限)
    pub max_overtime_per_day: u32,

    /// Maximum overtime per month in hours (1ヶ月の時間外労働上限)
    /// Standard limit: 45 hours (原則上限: 45時間)
    pub max_overtime_per_month: u32,

    /// Maximum overtime per year in hours (1年の時間外労働上限)
    /// Standard limit: 360 hours (原則上限: 360時間)
    pub max_overtime_per_year: u32,

    /// Has special circumstances clause (特別条項あり)
    pub has_special_circumstances: bool,

    /// Special maximum per month (特別条項時の月間上限)
    /// Absolute limit: 100 hours including holidays (絶対上限: 100時間)
    pub special_max_per_month: Option<u32>,

    /// Number of special months allowed per year (特別条項適用月数)
    /// Maximum: 6 months per year (上限: 年6回)
    pub special_months_per_year: Option<u32>,

    /// Permitted reasons for overtime (時間外労働の理由)
    pub permitted_reasons: Vec<String>,
}

impl Article36Agreement {
    /// Check if agreement is within standard limits (標準範囲内か)
    ///
    /// Standard limits without special circumstances:
    /// - Monthly: 45 hours
    /// - Yearly: 360 hours
    pub fn is_within_standard_limits(&self) -> bool {
        self.max_overtime_per_month <= 45 && self.max_overtime_per_year <= 360
    }

    /// Check if special circumstances are properly configured (特別条項が適切か)
    ///
    /// When special circumstances are enabled:
    /// - Monthly special limit must not exceed 100 hours
    /// - Special months must not exceed 6 per year
    /// - 2-6 month average must not exceed 80 hours (checked separately)
    pub fn is_special_circumstances_valid(&self) -> bool {
        if !self.has_special_circumstances {
            return true;
        }

        match (self.special_max_per_month, self.special_months_per_year) {
            (Some(monthly), Some(months)) => monthly <= 100 && months <= 6,
            _ => false, // Both fields required if special circumstances enabled
        }
    }

    /// Check if currently valid (有効期間内か)
    pub fn is_currently_valid(&self, date: chrono::NaiveDate) -> bool {
        date >= self.effective_date && date <= self.expiration_date
    }

    /// Validate that agreement meets all legal requirements (法的要件を満たすか)
    pub fn validate(&self) -> Result<(), String> {
        // Check standard limits
        if self.max_overtime_per_month > 45 && !self.has_special_circumstances {
            return Err(
                "Monthly overtime exceeds 45 hours without special circumstances".to_string(),
            );
        }

        if self.max_overtime_per_year > 360 && !self.has_special_circumstances {
            return Err(
                "Yearly overtime exceeds 360 hours without special circumstances".to_string(),
            );
        }

        // Check special circumstances if enabled
        if self.has_special_circumstances && !self.is_special_circumstances_valid() {
            return Err("Special circumstances configuration is invalid".to_string());
        }

        // Check that expiration is after effective date
        if self.expiration_date <= self.effective_date {
            return Err("Expiration date must be after effective date".to_string());
        }

        // Check that permitted reasons are specified
        if self.permitted_reasons.is_empty() {
            return Err("At least one permitted reason must be specified".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod article36_tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_standard_agreement() {
        let agreement = Article36Agreement {
            employer_name: "株式会社ABC".to_string(),
            labor_representative: "労働組合ABC支部".to_string(),
            effective_date: Utc::now().date_naive(),
            expiration_date: (Utc::now() + Duration::days(365)).date_naive(),
            max_overtime_per_day: 3,
            max_overtime_per_month: 45,
            max_overtime_per_year: 360,
            has_special_circumstances: false,
            special_max_per_month: None,
            special_months_per_year: None,
            permitted_reasons: vec!["臨時の業務増加".to_string()],
        };

        assert!(agreement.is_within_standard_limits());
        assert!(agreement.is_special_circumstances_valid());
        assert!(agreement.validate().is_ok());
    }

    #[test]
    fn test_special_circumstances_agreement() {
        let agreement = Article36Agreement {
            employer_name: "株式会社XYZ".to_string(),
            labor_representative: "従業員代表 山田太郎".to_string(),
            effective_date: Utc::now().date_naive(),
            expiration_date: (Utc::now() + Duration::days(365)).date_naive(),
            max_overtime_per_day: 5,
            max_overtime_per_month: 45,
            max_overtime_per_year: 360,
            has_special_circumstances: true,
            special_max_per_month: Some(80),
            special_months_per_year: Some(6),
            permitted_reasons: vec![
                "突発的な設備トラブル".to_string(),
                "納期の短縮要請".to_string(),
            ],
        };

        assert!(agreement.is_within_standard_limits()); // Standard limits are still 45/360
        assert!(agreement.is_special_circumstances_valid()); // Special circumstances properly configured
        assert!(agreement.validate().is_ok());
    }

    #[test]
    fn test_invalid_special_circumstances() {
        let agreement = Article36Agreement {
            employer_name: "株式会社違法".to_string(),
            labor_representative: "労働者代表".to_string(),
            effective_date: Utc::now().date_naive(),
            expiration_date: (Utc::now() + Duration::days(365)).date_naive(),
            max_overtime_per_day: 5,
            max_overtime_per_month: 45,
            max_overtime_per_year: 360,
            has_special_circumstances: true,
            special_max_per_month: Some(120), // Exceeds 100 hour limit!
            special_months_per_year: Some(6),
            permitted_reasons: vec!["業務増加".to_string()],
        };

        assert!(!agreement.is_special_circumstances_valid());
        assert!(agreement.validate().is_err());
    }

    #[test]
    fn test_missing_special_circumstances_config() {
        let agreement = Article36Agreement {
            employer_name: "株式会社不備".to_string(),
            labor_representative: "労働者代表".to_string(),
            effective_date: Utc::now().date_naive(),
            expiration_date: (Utc::now() + Duration::days(365)).date_naive(),
            max_overtime_per_day: 5,
            max_overtime_per_month: 45,
            max_overtime_per_year: 360,
            has_special_circumstances: true,
            special_max_per_month: None,   // Missing!
            special_months_per_year: None, // Missing!
            permitted_reasons: vec!["業務増加".to_string()],
        };

        assert!(!agreement.is_special_circumstances_valid());
    }

    #[test]
    fn test_currently_valid() {
        let now = Utc::now().date_naive();
        let agreement = Article36Agreement {
            employer_name: "株式会社ABC".to_string(),
            labor_representative: "労働組合".to_string(),
            effective_date: now - Duration::days(30),
            expiration_date: now + Duration::days(335),
            max_overtime_per_day: 3,
            max_overtime_per_month: 45,
            max_overtime_per_year: 360,
            has_special_circumstances: false,
            special_max_per_month: None,
            special_months_per_year: None,
            permitted_reasons: vec!["臨時業務".to_string()],
        };

        assert!(agreement.is_currently_valid(now));
        assert!(!agreement.is_currently_valid(now - Duration::days(60)));
        assert!(!agreement.is_currently_valid(now + Duration::days(365)));
    }
}
