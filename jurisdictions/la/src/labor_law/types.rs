//! Labor Law Types (ກົດໝາຍແຮງງານ)
//!
//! Type definitions for Lao labor law based on:
//! - **Labor Law 2013** (Law No. 43/NA, dated December 24, 2013)
//! - National Assembly of the Lao PDR
//!
//! # Legal References
//! - Labor Law 2013 (Law No. 43/NA) - ກົດໝາຍແຮງງານ ປີ 2013
//! - Minimum Wage Decree - ດີກຣີວ່າດ້ວຍຄ່າແຮງງານຂັ້ນຕ່ຳ
//! - Social Security Law - ກົດໝາຍປະກັນສັງຄົມ
//!
//! # Bilingual Support
//! All types include both Lao (ລາວ) and English field names where applicable.

use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Labor Law 2013 (ກົດໝາຍແຮງງານ ປີ 2013)
// ============================================================================

/// Statutory working hours per day (Article 51)
/// ຊົ່ວໂມງເຮັດວຽກຕາມກົດໝາຍ (ຕໍ່ມື້) - 8 ຊົ່ວໂມງ
pub const STATUTORY_HOURS_PER_DAY: u32 = 8;

/// Statutory working hours per week (Article 51)
/// ຊົ່ວໂມງເຮັດວຽກຕາມກົດໝາຍ (ຕໍ່ອາທິດ) - 48 ຊົ່ວໂມງ
pub const STATUTORY_HOURS_PER_WEEK: u32 = 48;

/// Maximum working days per week (Article 51)
/// ມື້ເຮັດວຽກສູງສຸດຕໍ່ອາທິດ - 6 ມື້
pub const MAX_WORKING_DAYS_PER_WEEK: u32 = 6;

/// Minimum rest period for 6+ hour shifts (Article 54)
/// ເວລາພັກຜ່ອນຂັ້ນຕ່ຳ (ເຮັດວຽກ 6 ຊົ່ວໂມງຂຶ້ນໄປ) - 1 ຊົ່ວໂມງ
pub const MIN_REST_PERIOD_MINUTES: u32 = 60;

/// Minimum weekly rest days (Article 55)
/// ມື້ພັກຜ່ອນຂັ້ນຕ່ຳຕໍ່ອາທິດ - 1 ມື້
pub const MIN_WEEKLY_REST_DAYS: u32 = 1;

/// Minimum annual leave days (Article 58)
/// ມື້ພັກຜ່ອນປະຈຳປີຂັ້ນຕ່ຳ - 15 ມື້
pub const MIN_ANNUAL_LEAVE_DAYS: u32 = 15;

/// Sick leave days per year (Article 61)
/// ມື້ລາປ່ວຍຕໍ່ປີ - 30 ມື້
pub const SICK_LEAVE_DAYS_PER_YEAR: u32 = 30;

/// Maternity leave days (Article 62)
/// ມື້ລາຄອດ - 105 ມື້ (15 ອາທິດ)
pub const MATERNITY_LEAVE_DAYS: u32 = 105;

/// Paternity leave days (Article 62)
/// ມື້ລາບິດາ - 3 ມື້
pub const PATERNITY_LEAVE_DAYS: u32 = 3;

/// Maximum probation period days (Article 20)
/// ໄລຍະທົດລອງສູງສຸດ - 60 ມື້
pub const MAX_PROBATION_PERIOD_DAYS: u32 = 60;

/// Termination notice period days (Article 74)
/// ໄລຍະແຈ້ງການເລີກຈ້າງ - 30 ມື້
pub const TERMINATION_NOTICE_DAYS: u32 = 30;

/// Overtime premium rate (Article 53)
/// ອັດຕາຄ່າລ່ວງເວລາ - 50%
pub const OVERTIME_PREMIUM_RATE: f64 = 0.50;

/// Night shift premium rate (22:00-06:00) (Article 53)
/// ອັດຕາຄ່າກະກາງຄືນ - 20%
pub const NIGHT_SHIFT_PREMIUM_RATE: f64 = 0.20;

/// Holiday work premium rate (Article 53)
/// ອັດຕາຄ່າເຮັດວຽກວັນພັກ - 100%
pub const HOLIDAY_WORK_PREMIUM_RATE: f64 = 1.00;

/// Maximum overtime hours per day (Article 52)
/// ຊົ່ວໂມງລ່ວງເວລາສູງສຸດຕໍ່ມື້ - 4 ຊົ່ວໂມງ
pub const MAX_OVERTIME_HOURS_PER_DAY: u32 = 4;

/// Maximum overtime hours per month (Article 52)
/// ຊົ່ວໂມງລ່ວງເວລາສູງສຸດຕໍ່ເດືອນ - 45 ຊົ່ວໂມງ
pub const MAX_OVERTIME_HOURS_PER_MONTH: u32 = 45;

/// Maximum fixed-term contract duration (years) (Article 17)
/// ໄລຍະສັນຍາຈ້າງມີກຳນົດສູງສຸດ - 3 ປີ
pub const MAX_FIXED_TERM_DURATION_YEARS: u32 = 3;

// ============================================================================
// Employment Contract Types (ປະເພດສັນຍາຈ້າງ)
// ============================================================================

/// Employment contract type (ປະເພດການຈ້າງງານ)
///
/// Article 16: Employment contracts shall be classified as:
/// - Indefinite-term contract (ສັນຍາຈ້າງບໍ່ມີກຳນົດ)
/// - Fixed-term contract (ສັນຍາຈ້າງມີກຳນົດ)
/// - Task-based contract (ສັນຍາຈ້າງຕາມວຽກງານ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EmploymentType {
    /// Indefinite-term contract (ສັນຍາຈ້າງບໍ່ມີກຳນົດ)
    /// No specified end date, continues until termination
    IndefiniteTerm,

    /// Fixed-term contract (ສັນຍາຈ້າງມີກຳນົດ)
    /// Maximum 3 years, can be renewed (Article 17)
    FixedTerm,

    /// Task-based contract (ສັນຍາຈ້າງຕາມວຽກງານ)
    /// Employment for specific task or project
    TaskBased,

    /// Probation (ໄລຍະທົດລອງ)
    /// Maximum 60 days probation period (Article 20)
    Probation,

    /// Seasonal work (ວຽກຕາມລະດູການ)
    /// Agricultural or seasonal employment
    Seasonal,
}

/// Work schedule type (ປະເພດຕາຕະລາງການເຮັດວຽກ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WorkSchedule {
    /// Regular schedule (ຕາຕະລາງປົກກະຕິ)
    /// Fixed daily schedule, 8 hours/day
    Regular,

    /// Shift work (ເຮັດວຽກເປັນກະ)
    /// Rotating shifts including night shifts
    Shift,

    /// Flexible (ເວລາຍືດຫຍຸ່ນ)
    /// Flexible working hours within limits
    Flexible,

    /// Part-time (ເຮັດວຽກບາງເວລາ)
    /// Less than statutory hours
    PartTime,

    /// Continuous operation (ດຳເນີນງານຕໍ່ເນື່ອງ)
    /// 24-hour operation with rotating shifts
    ContinuousOperation,
}

/// Employment contract (ສັນຍາຈ້າງງານ)
///
/// Article 15: Essential terms of employment contract:
/// 1. Names and addresses of parties
/// 2. Job description and workplace
/// 3. Working hours and rest periods
/// 4. Wages and payment terms
/// 5. Contract duration
/// 6. Other agreed terms
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EmploymentContract {
    // ========================================================================
    // Parties (ຄູ່ສັນຍາ)
    // ========================================================================
    /// Employee name (ຊື່ລູກຈ້າງ)
    pub employee_name: String,

    /// Employee name in Lao (ຊື່ລູກຈ້າງເປັນພາສາລາວ)
    pub employee_name_lao: Option<String>,

    /// Employee ID/Passport (ບັດປະຈຳໂຕ/ໜັງສືຜ່ານແດນ)
    pub employee_id: String,

    /// Employer name (ຊື່ນາຍຈ້າງ)
    pub employer_name: String,

    /// Employer name in Lao (ຊື່ນາຍຈ້າງເປັນພາສາລາວ)
    pub employer_name_lao: Option<String>,

    /// Enterprise registration number (ເລກທະບຽນວິສາຫະກິດ)
    pub employer_registration: String,

    // ========================================================================
    // Contract Terms (ເງື່ອນໄຂສັນຍາ)
    // ========================================================================
    /// Employment type (ປະເພດການຈ້າງ)
    pub employment_type: EmploymentType,

    /// Work schedule (ຕາຕະລາງການເຮັດວຽກ)
    pub work_schedule: WorkSchedule,

    /// Start date (ວັນທີເລີ່ມຕົ້ນ)
    pub start_date: DateTime<Utc>,

    /// End date for fixed-term contracts (ວັນທີສິ້ນສຸດ)
    pub end_date: Option<DateTime<Utc>>,

    /// Probation period in days (ໄລຍະທົດລອງ)
    /// Maximum 60 days (Article 20)
    pub probation_period_days: Option<u32>,

    // ========================================================================
    // Job Details (ລາຍລະອຽດວຽກງານ)
    // ========================================================================
    /// Job title (ຕຳແໜ່ງ)
    pub job_title: String,

    /// Job title in Lao (ຕຳແໜ່ງເປັນພາສາລາວ)
    pub job_title_lao: Option<String>,

    /// Job description (ລາຍລະອຽດວຽກງານ)
    pub job_description: String,

    /// Work location (ສະຖານທີ່ເຮັດວຽກ)
    pub work_location: String,

    /// Work location in Lao (ສະຖານທີ່ເຮັດວຽກເປັນພາສາລາວ)
    pub work_location_lao: Option<String>,

    // ========================================================================
    // Working Hours (ເວລາເຮັດວຽກ)
    // ========================================================================
    /// Working hours per day (ຊົ່ວໂມງເຮັດວຽກຕໍ່ມື້)
    /// Maximum 8 hours (Article 51)
    pub hours_per_day: u32,

    /// Working days per week (ມື້ເຮັດວຽກຕໍ່ອາທິດ)
    /// Maximum 6 days (Article 51)
    pub days_per_week: u32,

    /// Start time (ເວລາເລີ່ມວຽກ)
    pub start_time: String,

    /// End time (ເວລາເລີກວຽກ)
    pub end_time: String,

    /// Rest period minutes (ເວລາພັກຜ່ອນ ນາທີ)
    /// Minimum 60 minutes for 6+ hour shifts (Article 54)
    pub rest_period_minutes: u32,

    // ========================================================================
    // Compensation (ຄ່າຕອບແທນ)
    // ========================================================================
    /// Base monthly wage in LAK (ເງິນເດືອນພື້ນຖານຕໍ່ເດືອນ)
    pub base_wage_lak: u64,

    /// Hourly rate for part-time (ອັດຕາຕໍ່ຊົ່ວໂມງ)
    pub hourly_rate_lak: Option<u64>,

    /// Allowances (ເບີ້ຍເລີ້ຍງຕ່າງໆ)
    pub allowances: Vec<Allowance>,

    /// Payment frequency (ຄວາມຖີ່ການຈ່າຍເງິນ)
    pub payment_frequency: PaymentFrequency,

    /// Payment method (ວິທີການຈ່າຍເງິນ)
    pub payment_method: PaymentMethod,

    // ========================================================================
    // Leave Entitlements (ສິດການລາພັກ)
    // ========================================================================
    /// Annual leave days (ມື້ລາພັກປະຈຳປີ)
    /// Minimum 15 days (Article 58)
    pub annual_leave_days: u32,

    /// Sick leave days (ມື້ລາປ່ວຍ)
    /// 30 days per year (Article 61)
    pub sick_leave_days: u32,

    // ========================================================================
    // Social Security (ປະກັນສັງຄົມ)
    // ========================================================================
    /// Social security enrollment (ການຂຶ້ນທະບຽນປະກັນສັງຄົມ)
    pub social_security_enrolled: bool,

    /// Social security number (ເລກບັດປະກັນສັງຄົມ)
    pub social_security_number: Option<String>,

    // ========================================================================
    // Other Terms (ເງື່ອນໄຂອື່ນໆ)
    // ========================================================================
    /// Special conditions (ເງື່ອນໄຂພິເສດ)
    pub special_conditions: Vec<String>,

    /// Renewal count (ຈຳນວນຄັ້ງຕໍ່ສັນຍາ)
    pub renewal_count: u32,
}

impl EmploymentContract {
    /// Calculate weekly working hours (ຊົ່ວໂມງເຮັດວຽກຕໍ່ອາທິດ)
    pub fn weekly_hours(&self) -> u32 {
        self.hours_per_day * self.days_per_week
    }

    /// Check if contract exceeds statutory hours (ກວດສອບເກີນຊົ່ວໂມງກົດໝາຍ)
    pub fn exceeds_statutory_hours(&self) -> bool {
        self.hours_per_day > STATUTORY_HOURS_PER_DAY
            || self.weekly_hours() > STATUTORY_HOURS_PER_WEEK
    }

    /// Check if probation period is valid (ກວດສອບໄລຍະທົດລອງຖືກຕ້ອງ)
    pub fn is_probation_valid(&self) -> bool {
        match self.probation_period_days {
            Some(days) => days <= MAX_PROBATION_PERIOD_DAYS,
            None => true,
        }
    }

    /// Calculate minimum hourly rate (ຄຳນວນອັດຕາຕໍ່ຊົ່ວໂມງຂັ້ນຕ່ຳ)
    pub fn calculate_hourly_rate(&self) -> u64 {
        if let Some(rate) = self.hourly_rate_lak {
            rate
        } else {
            // Monthly wage / (hours per day * days per week * 4.33 weeks)
            let monthly_hours = self.hours_per_day as f64 * self.days_per_week as f64 * 4.33; // Average weeks per month
            (self.base_wage_lak as f64 / monthly_hours) as u64
        }
    }
}

// ============================================================================
// Allowances (ເບີ້ຍເລີ້ຍງ)
// ============================================================================

/// Allowance type (ປະເພດເບີ້ຍເລີ້ຍງ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AllowanceType {
    /// Transportation (ຄ່າພາຫະນະ)
    Transportation,

    /// Housing (ຄ່າທີ່ພັກອາໄສ)
    Housing,

    /// Meal (ຄ່າອາຫານ)
    Meal,

    /// Position (ເບີ້ຍຕຳແໜ່ງ)
    Position,

    /// Hazard pay (ເບີ້ຍຄວາມສ່ຽງ)
    Hazard,

    /// Remote work (ເບີ້ຍຂ້າງທ່ອງ)
    Remote,

    /// Night shift (ເບີ້ຍກະກາງຄືນ)
    NightShift,

    /// Other (ອື່ນໆ)
    Other,
}

/// Allowance (ເບີ້ຍເລີ້ຍງ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Allowance {
    /// Allowance type (ປະເພດເບີ້ຍເລີ້ຍງ)
    pub allowance_type: AllowanceType,

    /// Amount in LAK (ຈຳນວນເງິນ)
    pub amount_lak: u64,

    /// Description (ລາຍລະອຽດ)
    pub description: String,

    /// Description in Lao (ລາຍລະອຽດເປັນພາສາລາວ)
    pub description_lao: Option<String>,
}

// ============================================================================
// Payment (ການຈ່າຍເງິນ)
// ============================================================================

/// Payment frequency (ຄວາມຖີ່ການຈ່າຍເງິນ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PaymentFrequency {
    /// Monthly (ລາຍເດືອນ)
    Monthly,

    /// Bi-weekly (ທຸກ 2 ອາທິດ)
    BiWeekly,

    /// Weekly (ລາຍອາທິດ)
    Weekly,

    /// Daily (ລາຍມື້)
    Daily,

    /// Hourly (ລາຍຊົ່ວໂມງ)
    Hourly,
}

/// Payment method (ວິທີການຈ່າຍເງິນ)
///
/// Article 79: Wages shall be paid in Lao Kip currency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PaymentMethod {
    /// Bank transfer (ໂອນເງິນຜ່ານທະນາຄານ)
    BankTransfer,

    /// Cash (ເງິນສົດ)
    Cash,

    /// Check (ເຊັກ)
    Check,

    /// Mobile payment (ຈ່າຍຜ່ານມືຖື)
    MobilePayment,
}

// ============================================================================
// Working Hours (ຊົ່ວໂມງເຮັດວຽກ)
// ============================================================================

/// Working hours record (ບັນທຶກຊົ່ວໂມງເຮັດວຽກ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WorkingHoursRecord {
    /// Employee name (ຊື່ລູກຈ້າງ)
    pub employee_name: String,

    /// Date (ວັນທີ)
    pub date: DateTime<Utc>,

    /// Start time (ເວລາເຂົ້າວຽກ)
    pub start_time: DateTime<Utc>,

    /// End time (ເວລາອອກວຽກ)
    pub end_time: DateTime<Utc>,

    /// Regular hours (ຊົ່ວໂມງປົກກະຕິ)
    pub regular_hours: f64,

    /// Overtime hours (ຊົ່ວໂມງລ່ວງເວລາ)
    pub overtime_hours: f64,

    /// Night shift hours (ຊົ່ວໂມງກະກາງຄືນ)
    pub night_shift_hours: f64,

    /// Holiday work hours (ຊົ່ວໂມງເຮັດວຽກວັນພັກ)
    pub holiday_work_hours: f64,

    /// Rest periods (ເວລາພັກຜ່ອນ)
    pub rest_periods_minutes: u32,
}

impl WorkingHoursRecord {
    /// Calculate total hours (ຄຳນວນຊົ່ວໂມງທັງໝົດ)
    pub fn total_hours(&self) -> f64 {
        self.regular_hours + self.overtime_hours + self.night_shift_hours + self.holiday_work_hours
    }

    /// Check if overtime exceeds daily limit (ກວດລ່ວງເວລາເກີນຈຳກັດມື້)
    pub fn exceeds_daily_overtime_limit(&self) -> bool {
        self.overtime_hours > MAX_OVERTIME_HOURS_PER_DAY as f64
    }
}

/// Monthly working summary (ສະຫຼຸບການເຮັດວຽກລາຍເດືອນ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MonthlyWorkingSummary {
    /// Year (ປີ)
    pub year: u32,

    /// Month (ເດືອນ)
    pub month: u32,

    /// Employee name (ຊື່ລູກຈ້າງ)
    pub employee_name: String,

    /// Total regular hours (ຊົ່ວໂມງປົກກະຕິທັງໝົດ)
    pub total_regular_hours: f64,

    /// Total overtime hours (ຊົ່ວໂມງລ່ວງເວລາທັງໝົດ)
    pub total_overtime_hours: f64,

    /// Total night shift hours (ຊົ່ວໂມງກະກາງຄືນທັງໝົດ)
    pub total_night_shift_hours: f64,

    /// Total holiday work hours (ຊົ່ວໂມງເຮັດວຽກວັນພັກທັງໝົດ)
    pub total_holiday_work_hours: f64,

    /// Days worked (ມື້ເຮັດວຽກ)
    pub days_worked: u32,

    /// Days absent (ມື້ຂາດວຽກ)
    pub days_absent: u32,

    /// Days on leave (ມື້ລາພັກ)
    pub days_on_leave: u32,
}

impl MonthlyWorkingSummary {
    /// Calculate total hours (ຄຳນວນຊົ່ວໂມງທັງໝົດ)
    pub fn total_hours(&self) -> f64 {
        self.total_regular_hours
            + self.total_overtime_hours
            + self.total_night_shift_hours
            + self.total_holiday_work_hours
    }

    /// Check if overtime exceeds monthly limit (ກວດລ່ວງເວລາເກີນຈຳກັດເດືອນ)
    pub fn exceeds_monthly_overtime_limit(&self) -> bool {
        self.total_overtime_hours > MAX_OVERTIME_HOURS_PER_MONTH as f64
    }

    /// Calculate total wage (ຄຳນວນເງິນເດືອນທັງໝົດ)
    ///
    /// # Arguments
    /// * `hourly_rate` - Base hourly rate in LAK
    ///
    /// # Returns
    /// Total wage including premiums
    pub fn calculate_total_wage(&self, hourly_rate: u64) -> u64 {
        let base_rate = hourly_rate as f64;

        let regular_pay = self.total_regular_hours * base_rate;
        let overtime_pay = self.total_overtime_hours * base_rate * (1.0 + OVERTIME_PREMIUM_RATE);
        let night_pay = self.total_night_shift_hours * base_rate * (1.0 + NIGHT_SHIFT_PREMIUM_RATE);
        let holiday_pay =
            self.total_holiday_work_hours * base_rate * (1.0 + HOLIDAY_WORK_PREMIUM_RATE);

        (regular_pay + overtime_pay + night_pay + holiday_pay) as u64
    }
}

// ============================================================================
// Leave (ການລາພັກ)
// ============================================================================

/// Leave type (ປະເພດການລາພັກ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LeaveType {
    /// Annual leave (ລາພັກປະຈຳປີ)
    /// Article 58: Minimum 15 days per year
    AnnualLeave,

    /// Sick leave (ລາປ່ວຍ)
    /// Article 61: 30 days per year with full pay
    SickLeave,

    /// Maternity leave (ລາຄອດ)
    /// Article 62: 105 days (15 weeks)
    MaternityLeave,

    /// Paternity leave (ລາບິດາ)
    /// Article 62: 3 days
    PaternityLeave,

    /// Bereavement leave (ລາງານສົບ)
    BereavementLeave,

    /// Marriage leave (ລາແຕ່ງງານ)
    MarriageLeave,

    /// Study leave (ລາສຶກສາ)
    StudyLeave,

    /// Unpaid leave (ລາບໍ່ໄດ້ເງິນ)
    UnpaidLeave,
}

/// Leave request (ການຂໍລາພັກ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LeaveRequest {
    /// Employee name (ຊື່ລູກຈ້າງ)
    pub employee_name: String,

    /// Leave type (ປະເພດການລາ)
    pub leave_type: LeaveType,

    /// Start date (ວັນທີເລີ່ມ)
    pub start_date: DateTime<Utc>,

    /// End date (ວັນທີສິ້ນສຸດ)
    pub end_date: DateTime<Utc>,

    /// Number of days (ຈຳນວນມື້)
    pub days: u32,

    /// Reason (ເຫດຜົນ)
    pub reason: String,

    /// Reason in Lao (ເຫດຜົນເປັນພາສາລາວ)
    pub reason_lao: Option<String>,

    /// Supporting documents (ເອກະສານປະກອບ)
    pub supporting_documents: Vec<String>,

    /// Approved (ອະນຸມັດແລ້ວ)
    pub approved: bool,

    /// Approved by (ອະນຸມັດໂດຍ)
    pub approved_by: Option<String>,
}

// ============================================================================
// Termination (ການເລີກຈ້າງ)
// ============================================================================

/// Termination type (ປະເພດການເລີກຈ້າງ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TerminationType {
    /// Voluntary resignation (ລາອອກດ້ວຍຄວາມສະໝັກໃຈ)
    /// Article 73: Employee may resign with notice
    VoluntaryResignation,

    /// Termination by employer (ນາຍຈ້າງເລີກຈ້າງ)
    /// Article 74: Employer may terminate with notice
    EmployerTermination,

    /// Termination for cause (ເລີກຈ້າງເພາະມີຄວາມຜິດ)
    /// Article 75: Immediate termination for serious misconduct
    TerminationForCause,

    /// Mutual agreement (ຕົກລົງກັນ)
    /// Article 76: Termination by mutual agreement
    MutualAgreement,

    /// Contract expiration (ສັນຍາໝົດກຳນົດ)
    /// Natural end of fixed-term contract
    ContractExpiration,

    /// Redundancy (ປັບໂຄງສ້າງ)
    /// Economic or restructuring reasons
    Redundancy,

    /// Retirement (ອອກບຳນານ)
    /// Reaching retirement age
    Retirement,

    /// Death (ເສຍຊີວິດ)
    /// Employee death
    Death,
}

/// Termination notice (ແຈ້ງການເລີກຈ້າງ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TerminationNotice {
    /// Employee name (ຊື່ລູກຈ້າງ)
    pub employee_name: String,

    /// Termination type (ປະເພດການເລີກຈ້າງ)
    pub termination_type: TerminationType,

    /// Notice date (ວັນທີແຈ້ງ)
    pub notice_date: DateTime<Utc>,

    /// Effective date (ວັນທີມີຜົນ)
    pub effective_date: DateTime<Utc>,

    /// Reason (ເຫດຜົນ)
    pub reason: String,

    /// Reason in Lao (ເຫດຜົນເປັນພາສາລາວ)
    pub reason_lao: Option<String>,

    /// Severance pay amount (ຄ່າຊົດເຊີຍ)
    pub severance_pay_lak: Option<u64>,

    /// Notice allowance (ເງິນແທນການແຈ້ງການ)
    pub notice_allowance_lak: Option<u64>,

    /// Years of service (ຈຳນວນປີທີ່ເຮັດວຽກ)
    pub years_of_service: f64,
}

impl TerminationNotice {
    /// Calculate notice period in days (ຄຳນວນໄລຍະແຈ້ງການເປັນມື້)
    pub fn notice_period_days(&self) -> i64 {
        (self.effective_date - self.notice_date).num_days()
    }

    /// Check if notice period is sufficient (ກວດໄລຍະແຈ້ງການພຽງພໍ)
    pub fn is_notice_period_sufficient(&self) -> bool {
        match self.termination_type {
            TerminationType::TerminationForCause
            | TerminationType::Death
            | TerminationType::ContractExpiration => true,
            _ => self.notice_period_days() >= TERMINATION_NOTICE_DAYS as i64,
        }
    }

    /// Calculate required severance pay (ຄຳນວນຄ່າຊົດເຊີຍທີ່ຕ້ອງຈ່າຍ)
    ///
    /// Article 77: Severance pay calculation:
    /// - 1-3 years: 1 month salary
    /// - 3-5 years: 2 months salary
    /// - 5-10 years: 3 months salary
    /// - 10+ years: 6 months salary
    ///
    /// # Arguments
    /// * `monthly_wage` - Employee's monthly wage in LAK
    ///
    /// # Returns
    /// Required severance pay amount
    pub fn calculate_severance_pay(&self, monthly_wage: u64) -> u64 {
        match self.termination_type {
            TerminationType::TerminationForCause
            | TerminationType::VoluntaryResignation
            | TerminationType::Death => 0,
            _ => {
                let months = if self.years_of_service < 1.0 {
                    0
                } else if self.years_of_service < 3.0 {
                    1
                } else if self.years_of_service < 5.0 {
                    2
                } else if self.years_of_service < 10.0 {
                    3
                } else {
                    6
                };
                monthly_wage * months
            }
        }
    }
}

// ============================================================================
// Labor Disputes (ຂໍ້ຂັດແຍ່ງດ້ານແຮງງານ)
// ============================================================================

/// Labor dispute type (ປະເພດຂໍ້ຂັດແຍ່ງ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DisputeType {
    /// Individual dispute (ຂໍ້ຂັດແຍ່ງລະຫວ່າງບຸກຄົນ)
    /// Between individual employee and employer
    Individual,

    /// Collective dispute (ຂໍ້ຂັດແຍ່ງລະຫວ່າງກຸ່ມ)
    /// Between labor union and employer
    Collective,

    /// Rights dispute (ຂໍ້ຂັດແຍ່ງກ່ຽວກັບສິດ)
    /// Concerning legal rights and entitlements
    Rights,

    /// Interest dispute (ຂໍ້ຂັດແຍ່ງກ່ຽວກັບຜົນປະໂຫຍດ)
    /// Concerning terms and conditions
    Interest,
}

/// Labor dispute (ຂໍ້ຂັດແຍ່ງດ້ານແຮງງານ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LaborDispute {
    /// Dispute type (ປະເພດຂໍ້ຂັດແຍ່ງ)
    pub dispute_type: DisputeType,

    /// Employee/Union name (ຊື່ລູກຈ້າງ/ສະຫະພັນ)
    pub complainant: String,

    /// Employer name (ຊື່ນາຍຈ້າງ)
    pub respondent: String,

    /// Filed date (ວັນທີຍື່ນ)
    pub filed_date: DateTime<Utc>,

    /// Issue description (ລາຍລະອຽດບັນຫາ)
    pub issue: String,

    /// Issue in Lao (ລາຍລະອຽດບັນຫາເປັນພາສາລາວ)
    pub issue_lao: Option<String>,

    /// Resolution sought (ວິທີແກ້ໄຂທີ່ຕ້ອງການ)
    pub resolution_sought: String,

    /// Status (ສະຖານະ)
    pub status: DisputeStatus,
}

/// Dispute status (ສະຖານະຂໍ້ຂັດແຍ່ງ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DisputeStatus {
    /// Filed (ຍື່ນແລ້ວ)
    Filed,

    /// Under mediation (ກຳລັງໄກ່ເກ່ຍ)
    UnderMediation,

    /// Under arbitration (ກຳລັງອານຸຍາໂຕ)
    UnderArbitration,

    /// In court (ຢູ່ໃນສານ)
    InCourt,

    /// Resolved (ແກ້ໄຂແລ້ວ)
    Resolved,

    /// Dismissed (ຍົກເລີກ)
    Dismissed,
}

// ============================================================================
// Social Security (ປະກັນສັງຄົມ)
// ============================================================================

/// Social security type (ປະເພດປະກັນສັງຄົມ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SocialSecurityType {
    /// Work injury (ອຸບັດຕິເຫດຈາກການເຮັດວຽກ)
    WorkInjury,

    /// Sickness (ເຈັບປ່ວຍ)
    Sickness,

    /// Maternity (ຄອດລູກ)
    Maternity,

    /// Old age pension (ບຳນານ)
    OldAge,

    /// Disability (ພິການ)
    Disability,

    /// Death benefit (ຜົນປະໂຫຍດເສຍຊີວິດ)
    Death,
}

/// Social security contribution (ການປະກອບສ່ວນປະກັນສັງຄົມ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SocialSecurityContribution {
    /// Employee name (ຊື່ລູກຈ້າງ)
    pub employee_name: String,

    /// Social security number (ເລກບັດປະກັນສັງຄົມ)
    pub social_security_number: String,

    /// Base wage for calculation (ເງິນເດືອນພື້ນຖານສຳລັບຄຳນວນ)
    pub base_wage_lak: u64,

    /// Employee contribution rate (ອັດຕາປະກອບສ່ວນຂອງລູກຈ້າງ)
    /// Typically 5.5% of base wage
    pub employee_rate: f64,

    /// Employer contribution rate (ອັດຕາປະກອບສ່ວນຂອງນາຍຈ້າງ)
    /// Typically 6.0% of base wage
    pub employer_rate: f64,

    /// Coverage types (ປະເພດການຄຸ້ມຄອງ)
    pub coverage_types: Vec<SocialSecurityType>,
}

impl SocialSecurityContribution {
    /// Calculate employee contribution (ຄຳນວນປະກອບສ່ວນຂອງລູກຈ້າງ)
    pub fn employee_contribution(&self) -> u64 {
        (self.base_wage_lak as f64 * self.employee_rate) as u64
    }

    /// Calculate employer contribution (ຄຳນວນປະກອບສ່ວນຂອງນາຍຈ້າງ)
    pub fn employer_contribution(&self) -> u64 {
        (self.base_wage_lak as f64 * self.employer_rate) as u64
    }

    /// Calculate total contribution (ຄຳນວນປະກອບສ່ວນທັງໝົດ)
    pub fn total_contribution(&self) -> u64 {
        self.employee_contribution() + self.employer_contribution()
    }
}
