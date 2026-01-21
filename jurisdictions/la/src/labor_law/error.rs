//! Labor Law Error Types (ປະເພດຄວາມຜິດພາດກົດໝາຍແຮງງານ)
//!
//! Comprehensive error types for Lao labor law validation and compliance.
//! All errors include bilingual messages (Lao/English) where applicable.

use thiserror::Error;

/// Result type for labor law operations
pub type Result<T> = std::result::Result<T, LaborLawError>;

/// Labor law errors (ຄວາມຜິດພາດກົດໝາຍແຮງງານ)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum LaborLawError {
    // ========================================================================
    // Working Hours Violations (ການລະເມີດເວລາເຮັດວຽກ)
    // ========================================================================
    /// Exceeds statutory daily working hours (Article 51)
    /// ເກີນຊົ່ວໂມງເຮັດວຽກຕາມກົດໝາຍ (ມາດຕາ 51)
    #[error(
        "Working hours {actual} hours/day exceeds statutory limit of {statutory} hours (Article 51)\nເກີນຊົ່ວໂມງເຮັດວຽກຕາມກົດໝາຍ: {actual} ຊົ່ວໂມງ/ມື້ > {statutory} ຊົ່ວໂມງ (ມາດຕາ 51)"
    )]
    ExceedsStatutoryDailyHours { actual: u32, statutory: u32 },

    /// Exceeds statutory weekly working hours (Article 51)
    /// ເກີນຊົ່ວໂມງເຮັດວຽກຕໍ່ອາທິດຕາມກົດໝາຍ (ມາດຕາ 51)
    #[error(
        "Weekly working hours {actual} hours exceeds statutory limit of {statutory} hours (Article 51)\nເກີນຊົ່ວໂມງເຮັດວຽກຕໍ່ອາທິດ: {actual} ຊົ່ວໂມງ > {statutory} ຊົ່ວໂມງ (ມາດຕາ 51)"
    )]
    ExceedsStatutoryWeeklyHours { actual: u32, statutory: u32 },

    /// Exceeds maximum working days per week (Article 51)
    /// ເກີນມື້ເຮັດວຽກສູງສຸດຕໍ່ອາທິດ (ມາດຕາ 51)
    #[error(
        "Working days {actual} days/week exceeds maximum of {max} days (Article 51)\nເກີນມື້ເຮັດວຽກສູງສຸດ: {actual} ມື້/ອາທິດ > {max} ມື້ (ມາດຕາ 51)"
    )]
    ExceedsMaxWorkingDays { actual: u32, max: u32 },

    /// Insufficient rest period (Article 54)
    /// ເວລາພັກຜ່ອນບໍ່ພຽງພໍ (ມາດຕາ 54)
    #[error(
        "Rest period of {actual} minutes is insufficient for {working_hours} hour shift (required: {required} minutes, Article 54)\nເວລາພັກຜ່ອນບໍ່ພຽງພໍ: {actual} ນາທີ < {required} ນາທີ ສຳລັບກະ {working_hours} ຊົ່ວໂມງ (ມາດຕາ 54)"
    )]
    InsufficientRestPeriod {
        actual: u32,
        required: u32,
        working_hours: u32,
    },

    /// Insufficient weekly rest days (Article 55)
    /// ມື້ພັກຜ່ອນຕໍ່ອາທິດບໍ່ພຽງພໍ (ມາດຕາ 55)
    #[error(
        "Weekly rest days {actual} is below minimum of {required} days (Article 55)\nມື້ພັກຜ່ອນບໍ່ພຽງພໍ: {actual} ມື້ < {required} ມື້ຕໍ່ອາທິດ (ມາດຕາ 55)"
    )]
    InsufficientWeeklyRest { actual: u32, required: u32 },

    /// Exceeds daily overtime limit (Article 52)
    /// ເກີນຊົ່ວໂມງລ່ວງເວລາສູງສຸດຕໍ່ມື້ (ມາດຕາ 52)
    #[error(
        "Daily overtime {actual} hours exceeds limit of {limit} hours (Article 52)\nເກີນຊົ່ວໂມງລ່ວງເວລາຕໍ່ມື້: {actual} ຊົ່ວໂມງ > {limit} ຊົ່ວໂມງ (ມາດຕາ 52)"
    )]
    ExceedsDailyOvertimeLimit { actual: f64, limit: u32 },

    /// Exceeds monthly overtime limit (Article 52)
    /// ເກີນຊົ່ວໂມງລ່ວງເວລາສູງສຸດຕໍ່ເດືອນ (ມາດຕາ 52)
    #[error(
        "Monthly overtime {actual} hours exceeds limit of {limit} hours (Article 52)\nເກີນຊົ່ວໂມງລ່ວງເວລາຕໍ່ເດືອນ: {actual} ຊົ່ວໂມງ > {limit} ຊົ່ວໂມງ (ມາດຕາ 52)"
    )]
    ExceedsMonthlyOvertimeLimit { actual: f64, limit: u32 },

    // ========================================================================
    // Wage Violations (ການລະເມີດຄ່າຈ້າງ)
    // ========================================================================
    /// Below minimum wage
    /// ຕ່ຳກວ່າຄ່າແຮງງານຂັ້ນຕ່ຳ
    #[error(
        "Wage {actual_lak} LAK is below minimum wage of {minimum_lak} LAK\nຄ່າຈ້າງຕ່ຳກວ່າຂັ້ນຕ່ຳ: {actual_lak} ກີບ < {minimum_lak} ກີບ"
    )]
    BelowMinimumWage { actual_lak: u64, minimum_lak: u64 },

    /// Hourly rate below minimum
    /// ອັດຕາຕໍ່ຊົ່ວໂມງຕ່ຳກວ່າຂັ້ນຕ່ຳ
    #[error(
        "Hourly rate {actual_lak} LAK/hour is below minimum {minimum_lak} LAK/hour\nອັດຕາຕໍ່ຊົ່ວໂມງຕ່ຳກວ່າຂັ້ນຕ່ຳ: {actual_lak} ກີບ/ຊົ່ວໂມງ < {minimum_lak} ກີບ/ຊົ່ວໂມງ"
    )]
    HourlyRateBelowMinimum { actual_lak: u64, minimum_lak: u64 },

    /// Incorrect overtime premium (Article 53)
    /// ການຄິດໄລ່ຄ່າລ່ວງເວລາບໍ່ຖືກຕ້ອງ (ມາດຕາ 53)
    #[error(
        "Overtime premium {actual_lak} LAK is less than required {required_lak} LAK (Article 53 - 50% premium)\nຄ່າລ່ວງເວລາບໍ່ພຽງພໍ: {actual_lak} ກີບ < {required_lak} ກີບ (ມາດຕາ 53 - ເພີ່ມ 50%)"
    )]
    IncorrectOvertimePremium { actual_lak: u64, required_lak: u64 },

    /// Incorrect night shift premium (Article 53)
    /// ການຄິດໄລ່ຄ່າກະກາງຄືນບໍ່ຖືກຕ້ອງ (ມາດຕາ 53)
    #[error(
        "Night shift premium {actual_lak} LAK is less than required {required_lak} LAK (Article 53 - 20% premium)\nຄ່າກະກາງຄືນບໍ່ພຽງພໍ: {actual_lak} ກີບ < {required_lak} ກີບ (ມາດຕາ 53 - ເພີ່ມ 20%)"
    )]
    IncorrectNightShiftPremium { actual_lak: u64, required_lak: u64 },

    /// Incorrect holiday work premium (Article 53)
    /// ການຄິດໄລ່ຄ່າເຮັດວຽກວັນພັກບໍ່ຖືກຕ້ອງ (ມາດຕາ 53)
    #[error(
        "Holiday work premium {actual_lak} LAK is less than required {required_lak} LAK (Article 53 - 100% premium)\nຄ່າເຮັດວຽກວັນພັກບໍ່ພຽງພໍ: {actual_lak} ກີບ < {required_lak} ກີບ (ມາດຕາ 53 - ເພີ່ມ 100%)"
    )]
    IncorrectHolidayWorkPremium { actual_lak: u64, required_lak: u64 },

    /// Late wage payment (Article 79)
    /// ການຈ່າຍເງິນເດືອນຊັກຊ້າ (ມາດຕາ 79)
    #[error(
        "Wage payment delayed: due date {due_date}, actual payment {payment_date} (Article 79)\nການຈ່າຍເງິນເດືອນຊັກຊ້າ: ກຳນົດ {due_date}, ຈ່າຍແທ້ {payment_date} (ມາດຕາ 79)"
    )]
    LateWagePayment {
        due_date: String,
        payment_date: String,
    },

    /// Invalid wage deduction (Article 79)
    /// ການຫັກເງິນເດືອນທີ່ບໍ່ຖືກຕ້ອງ (ມາດຕາ 79)
    #[error(
        "Invalid wage deduction: {reason} (Article 79)\nການຫັກເງິນເດືອນບໍ່ຖືກຕ້ອງ: {reason} (ມາດຕາ 79)"
    )]
    InvalidWageDeduction { reason: String },

    // ========================================================================
    // Contract Violations (ການລະເມີດສັນຍາຈ້າງ)
    // ========================================================================
    /// Missing required contract terms (Article 15)
    /// ຂາດເງື່ອນໄຂສັນຍາທີ່ຈຳເປັນ (ມາດຕາ 15)
    #[error(
        "Missing required contract terms: {missing_terms} (Article 15)\nຂາດເງື່ອນໄຂສັນຍາທີ່ຈຳເປັນ: {missing_terms} (ມາດຕາ 15)"
    )]
    MissingContractTerms { missing_terms: String },

    /// Missing required field
    /// ຂາດຊ່ອງຂໍ້ມູນທີ່ຈຳເປັນ
    #[error("Missing required field: {field_name}\nຂາດຊ່ອງຂໍ້ມູນທີ່ຈຳເປັນ: {field_name}")]
    MissingRequiredField { field_name: String },

    /// Fixed-term contract exceeds maximum duration (Article 17)
    /// ສັນຍາຈ້າງມີກຳນົດເກີນໄລຍະສູງສຸດ (ມາດຕາ 17)
    #[error(
        "Fixed-term contract duration {years} years exceeds maximum of {max_years} years (Article 17)\nສັນຍາຈ້າງມີກຳນົດເກີນກຳນົດ: {years} ປີ > {max_years} ປີ (ມາດຕາ 17)"
    )]
    FixedTermExceedsLimit { years: f64, max_years: u32 },

    /// Probation period exceeds maximum (Article 20)
    /// ໄລຍະທົດລອງເກີນກຳນົດສູງສຸດ (ມາດຕາ 20)
    #[error(
        "Probation period {actual_days} days exceeds maximum of {max_days} days (Article 20)\nໄລຍະທົດລອງເກີນກຳນົດ: {actual_days} ມື້ > {max_days} ມື້ (ມາດຕາ 20)"
    )]
    ProbationExceedsLimit { actual_days: u32, max_days: u32 },

    /// Invalid contract dates
    /// ວັນທີສັນຍາບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid contract dates: end date must be after start date\nວັນທີສັນຍາບໍ່ຖືກຕ້ອງ: ວັນທີສິ້ນສຸດຕ້ອງຫຼັງວັນທີເລີ່ມຕົ້ນ"
    )]
    InvalidContractDates,

    // ========================================================================
    // Leave Violations (ການລະເມີດການລາພັກ)
    // ========================================================================
    /// Insufficient annual leave (Article 58)
    /// ມື້ລາພັກປະຈຳປີບໍ່ພຽງພໍ (ມາດຕາ 58)
    #[error(
        "Annual leave {actual_days} days is below minimum of {min_days} days (Article 58)\nມື້ລາພັກປະຈຳປີບໍ່ພຽງພໍ: {actual_days} ມື້ < {min_days} ມື້ (ມາດຕາ 58)"
    )]
    InsufficientAnnualLeave { actual_days: u32, min_days: u32 },

    /// Insufficient sick leave (Article 61)
    /// ມື້ລາປ່ວຍບໍ່ພຽງພໍ (ມາດຕາ 61)
    #[error(
        "Sick leave {actual_days} days is below statutory {statutory_days} days (Article 61)\nມື້ລາປ່ວຍບໍ່ພຽງພໍ: {actual_days} ມື້ < {statutory_days} ມື້ (ມາດຕາ 61)"
    )]
    InsufficientSickLeave {
        actual_days: u32,
        statutory_days: u32,
    },

    /// Maternity leave violation (Article 62)
    /// ການລະເມີດການລາຄອດ (ມາດຕາ 62)
    #[error(
        "Maternity leave {actual_days} days is below statutory {statutory_days} days (Article 62 - 105 days)\nມື້ລາຄອດບໍ່ພຽງພໍ: {actual_days} ມື້ < {statutory_days} ມື້ (ມາດຕາ 62 - 105 ມື້)"
    )]
    InsufficientMaternityLeave {
        actual_days: u32,
        statutory_days: u32,
    },

    /// Invalid leave dates
    /// ວັນທີລາພັກບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid leave dates: end date must be after start date\nວັນທີລາພັກບໍ່ຖືກຕ້ອງ: ວັນທີສິ້ນສຸດຕ້ອງຫຼັງວັນທີເລີ່ມຕົ້ນ"
    )]
    InvalidLeaveDates,

    // ========================================================================
    // Termination Violations (ການລະເມີດການເລີກຈ້າງ)
    // ========================================================================
    /// Insufficient notice period (Article 74)
    /// ໄລຍະແຈ້ງການເລີກຈ້າງບໍ່ພຽງພໍ (ມາດຕາ 74)
    #[error(
        "Termination notice period {actual_days} days is less than required {required_days} days (Article 74)\nໄລຍະແຈ້ງການບໍ່ພຽງພໍ: {actual_days} ມື້ < {required_days} ມື້ (ມາດຕາ 74)"
    )]
    InsufficientNotice {
        actual_days: i64,
        required_days: u32,
    },

    /// Missing notice allowance (Article 74)
    /// ຂາດເງິນແທນການແຈ້ງການ (ມາດຕາ 74)
    #[error(
        "Insufficient notice period requires notice allowance of {required_lak} LAK (Article 74)\nຕ້ອງຈ່າຍເງິນແທນການແຈ້ງການ: {required_lak} ກີບ (ມາດຕາ 74)"
    )]
    MissingNoticeAllowance { required_lak: u64 },

    /// Insufficient severance pay (Article 77)
    /// ຄ່າຊົດເຊີຍບໍ່ພຽງພໍ (ມາດຕາ 77)
    #[error(
        "Severance pay {actual_lak} LAK is less than required {required_lak} LAK (Article 77 - {months} months salary)\nຄ່າຊົດເຊີຍບໍ່ພຽງພໍ: {actual_lak} ກີບ < {required_lak} ກີບ (ມາດຕາ 77 - {months} ເດືອນ)"
    )]
    InsufficientSeverancePay {
        actual_lak: u64,
        required_lak: u64,
        months: u64,
    },

    /// Unfair dismissal (Article 75)
    /// ການເລີກຈ້າງທີ່ບໍ່ຍຸດຕິທຳ (ມາດຕາ 75)
    #[error("Unfair dismissal: {reason} (Article 75)\nການເລີກຈ້າງບໍ່ຍຸດຕິທຳ: {reason} (ມາດຕາ 75)")]
    UnfairDismissal { reason: String },

    /// Termination during protected period
    /// ເລີກຈ້າງໃນໄລຍະຄຸ້ມຄອງ
    #[error(
        "Cannot terminate during protected period: {period}\nບໍ່ສາມາດເລີກຈ້າງໃນໄລຍະຄຸ້ມຄອງ: {period}"
    )]
    TerminationDuringProtectedPeriod { period: String },

    // ========================================================================
    // Social Security Violations (ການລະເມີດປະກັນສັງຄົມ)
    // ========================================================================
    /// Not enrolled in social security
    /// ບໍ່ໄດ້ຂຶ້ນທະບຽນປະກັນສັງຄົມ
    #[error(
        "Employee not enrolled in social security (mandatory for all employees)\nລູກຈ້າງບໍ່ໄດ້ຂຶ້ນທະບຽນປະກັນສັງຄົມ (ບັງຄັບສຳລັບລູກຈ້າງທຸກຄົນ)"
    )]
    NotEnrolledInSocialSecurity,

    /// Invalid social security number
    /// ເລກບັດປະກັນສັງຄົມບໍ່ຖືກຕ້ອງ
    #[error("Invalid social security number: {number}\nເລກບັດປະກັນສັງຄົມບໍ່ຖືກຕ້ອງ: {number}")]
    InvalidSocialSecurityNumber { number: String },

    /// Incorrect social security contribution
    /// ການປະກອບສ່ວນປະກັນສັງຄົມບໍ່ຖືກຕ້ອງ
    #[error(
        "Social security contribution {actual_lak} LAK is less than required {required_lak} LAK\nການປະກອບສ່ວນປະກັນສັງຄົມບໍ່ພຽງພໍ: {actual_lak} ກີບ < {required_lak} ກີບ"
    )]
    IncorrectSocialSecurityContribution { actual_lak: u64, required_lak: u64 },

    // ========================================================================
    // General Violations (ການລະເມີດທົ່ວໄປ)
    // ========================================================================
    /// Validation error
    /// ຄວາມຜິດພາດການກວດສອບ
    #[error("Validation error: {message}\nຄວາມຜິດພາດການກວດສອບ: {message}")]
    ValidationError { message: String },

    /// General labor law violation
    /// ການລະເມີດກົດໝາຍແຮງງານ
    #[error(
        "Labor law violation: {violation} (Article {article})\nການລະເມີດກົດໝາຍແຮງງານ: {violation} (ມາດຕາ {article})"
    )]
    LaborLawViolation { violation: String, article: u32 },

    /// Discrimination violation
    /// ການລະເມີດການຈຳແນກ
    #[error(
        "Employment discrimination: {reason} (prohibited under Labor Law 2013)\nການຈຳແນກໃນການຈ້າງງານ: {reason} (ຫ້າມຕາມກົດໝາຍແຮງງານ ປີ 2013)"
    )]
    Discrimination { reason: String },

    /// Child labor violation
    /// ການລະເມີດການໃຊ້ແຮງງານເດັກ
    #[error(
        "Child labor violation: employee age {age} is below minimum age {min_age} (Article 38)\nການລະເມີດການໃຊ້ແຮງງານເດັກ: ອາຍຸລູກຈ້າງ {age} ຕ່ຳກວ່າອາຍຸຂັ້ນຕ່ຳ {min_age} (ມາດຕາ 38)"
    )]
    ChildLabor { age: u32, min_age: u32 },

    /// Forced labor violation
    /// ການລະເມີດແຮງງານບັງຄັບ
    #[error(
        "Forced labor violation: {description} (prohibited under Labor Law 2013)\nການລະເມີດແຮງງານບັງຄັບ: {description} (ຫ້າມຕາມກົດໝາຍແຮງງານ ປີ 2013)"
    )]
    ForcedLabor { description: String },

    /// Unsafe working conditions
    /// ສະພາບການເຮັດວຽກບໍ່ປອດໄພ
    #[error(
        "Unsafe working conditions: {hazard} (Article 94 - occupational safety and health)\nສະພາບການເຮັດວຽກບໍ່ປອດໄພ: {hazard} (ມາດຕາ 94 - ຄວາມປອດໄພແລະສຸຂະພາບໃນການເຮັດວຽກ)"
    )]
    UnsafeWorkingConditions { hazard: String },
}

impl LaborLawError {
    /// Get the error message in Lao language
    /// ຮັບຂໍ້ຄວາມຄວາມຜິດພາດເປັນພາສາລາວ
    pub fn lao_message(&self) -> String {
        let full_msg = format!("{}", self);
        // Extract the Lao part after the newline
        if let Some((_english, lao)) = full_msg.split_once('\n') {
            lao.to_string()
        } else {
            full_msg
        }
    }

    /// Get the error message in English language
    /// ຮັບຂໍ້ຄວາມຄວາມຜິດພາດເປັນພາສາອັງກິດ
    pub fn english_message(&self) -> String {
        let full_msg = format!("{}", self);
        // Extract the English part before the newline
        if let Some((english, _lao)) = full_msg.split_once('\n') {
            english.to_string()
        } else {
            full_msg
        }
    }

    /// Check if this is a critical violation requiring immediate action
    /// ກວດສອບວ່າເປັນການລະເມີດຮ້າຍແຮງທີ່ຕ້ອງແກ້ໄຂທັນທີ
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            LaborLawError::ChildLabor { .. }
                | LaborLawError::ForcedLabor { .. }
                | LaborLawError::UnsafeWorkingConditions { .. }
                | LaborLawError::UnfairDismissal { .. }
                | LaborLawError::Discrimination { .. }
        )
    }

    /// Get the article number referenced in this error, if any
    /// ຮັບເລກມາດຕາທີ່ອ້າງອິງໃນຄວາມຜິດພາດນີ້
    pub fn article_number(&self) -> Option<u32> {
        match self {
            LaborLawError::ExceedsStatutoryDailyHours { .. } => Some(51),
            LaborLawError::ExceedsStatutoryWeeklyHours { .. } => Some(51),
            LaborLawError::ExceedsMaxWorkingDays { .. } => Some(51),
            LaborLawError::InsufficientRestPeriod { .. } => Some(54),
            LaborLawError::InsufficientWeeklyRest { .. } => Some(55),
            LaborLawError::ExceedsDailyOvertimeLimit { .. } => Some(52),
            LaborLawError::ExceedsMonthlyOvertimeLimit { .. } => Some(52),
            LaborLawError::IncorrectOvertimePremium { .. } => Some(53),
            LaborLawError::IncorrectNightShiftPremium { .. } => Some(53),
            LaborLawError::IncorrectHolidayWorkPremium { .. } => Some(53),
            LaborLawError::LateWagePayment { .. } => Some(79),
            LaborLawError::InvalidWageDeduction { .. } => Some(79),
            LaborLawError::MissingContractTerms { .. } => Some(15),
            LaborLawError::FixedTermExceedsLimit { .. } => Some(17),
            LaborLawError::ProbationExceedsLimit { .. } => Some(20),
            LaborLawError::InsufficientAnnualLeave { .. } => Some(58),
            LaborLawError::InsufficientSickLeave { .. } => Some(61),
            LaborLawError::InsufficientMaternityLeave { .. } => Some(62),
            LaborLawError::InsufficientNotice { .. } => Some(74),
            LaborLawError::MissingNoticeAllowance { .. } => Some(74),
            LaborLawError::InsufficientSeverancePay { .. } => Some(77),
            LaborLawError::UnfairDismissal { .. } => Some(75),
            LaborLawError::ChildLabor { .. } => Some(38),
            LaborLawError::UnsafeWorkingConditions { .. } => Some(94),
            LaborLawError::LaborLawViolation { article, .. } => Some(*article),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilingual_error_messages() {
        let error = LaborLawError::ExceedsStatutoryDailyHours {
            actual: 10,
            statutory: 8,
        };

        let english = error.english_message();
        let lao = error.lao_message();

        assert!(english.contains("Working hours"));
        assert!(lao.contains("ຊົ່ວໂມງ"));
    }

    #[test]
    fn test_critical_violations() {
        let child_labor = LaborLawError::ChildLabor {
            age: 14,
            min_age: 15,
        };
        assert!(child_labor.is_critical());

        let overtime = LaborLawError::ExceedsDailyOvertimeLimit {
            actual: 5.0,
            limit: 4,
        };
        assert!(!overtime.is_critical());
    }

    #[test]
    fn test_article_numbers() {
        let error = LaborLawError::ExceedsStatutoryDailyHours {
            actual: 10,
            statutory: 8,
        };
        assert_eq!(error.article_number(), Some(51));

        let error = LaborLawError::InsufficientAnnualLeave {
            actual_days: 10,
            min_days: 15,
        };
        assert_eq!(error.article_number(), Some(58));
    }
}
