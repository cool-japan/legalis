//! UAE Labour Law - Federal Decree-Law No. 33/2021
//!
//! The UAE Labour Relations Law (effective February 2, 2022) governs
//! employment relationships in the private sector.
//!
//! ## Key Features
//!
//! - 8 hours/day, 48 hours/week maximum
//! - Minimum 30 days annual leave after 1 year
//! - End of Service Gratuity (EOSG) system
//! - Wage Protection System (WPS)
//! - Friday is now a working day (workweek Mon-Fri since 2022)
//!
//! ## Exclusions
//!
//! This law does not apply to:
//! - Federal and local government employees
//! - Armed forces members
//! - Domestic workers (separate law)
//! - Agricultural workers (partially excluded)

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for labor law operations
pub type LaborResult<T> = Result<T, LaborError>;

/// Employment contract types - Article 8
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    /// Limited term (fixed-term) contract - max 3 years, renewable
    LimitedTerm {
        /// Duration in months (max 36)
        duration_months: u32,
    },
    /// Part-time contract
    PartTime {
        /// Hours per week
        hours_per_week: u32,
    },
    /// Temporary/Casual contract
    Temporary {
        /// Duration in days (max 6 months typically)
        duration_days: u32,
    },
    /// Flexible work contract
    Flexible,
    /// Remote work contract (added 2022)
    Remote,
    /// Job sharing contract (added 2022)
    JobSharing,
}

impl ContractType {
    /// Check if contract duration is valid
    pub fn is_valid_duration(&self) -> bool {
        match self {
            Self::LimitedTerm { duration_months } => *duration_months > 0 && *duration_months <= 36,
            Self::PartTime { hours_per_week } => *hours_per_week > 0 && *hours_per_week < 48,
            Self::Temporary { duration_days } => *duration_days > 0 && *duration_days <= 180,
            _ => true,
        }
    }

    /// Get name in Arabic
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::LimitedTerm { .. } => "عقد محدد المدة",
            Self::PartTime { .. } => "عقد عمل جزئي",
            Self::Temporary { .. } => "عقد عمل مؤقت",
            Self::Flexible => "عقد عمل مرن",
            Self::Remote => "عقد عمل عن بعد",
            Self::JobSharing => "عقد تقاسم العمل",
        }
    }

    /// Get name in English
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::LimitedTerm { .. } => "Limited Term Contract",
            Self::PartTime { .. } => "Part-Time Contract",
            Self::Temporary { .. } => "Temporary Contract",
            Self::Flexible => "Flexible Work Contract",
            Self::Remote => "Remote Work Contract",
            Self::JobSharing => "Job Sharing Contract",
        }
    }
}

/// Working hours - Article 17-21
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingHours {
    /// Regular hours per day (max 8)
    pub hours_per_day: u32,
    /// Working days per week
    pub days_per_week: u32,
    /// Overtime hours per day
    pub overtime_per_day: u32,
    /// Is Ramadan reduced hours (2 hours less per day)
    pub is_ramadan: bool,
}

impl WorkingHours {
    /// Standard working hours (8 hours/day, 5 days/week)
    pub fn standard() -> Self {
        Self {
            hours_per_day: 8,
            days_per_week: 5,
            overtime_per_day: 0,
            is_ramadan: false,
        }
    }

    /// Total weekly hours (including overtime)
    pub fn total_weekly_hours(&self) -> u32 {
        let daily_hours = if self.is_ramadan {
            self.hours_per_day.saturating_sub(2)
        } else {
            self.hours_per_day
        };
        (daily_hours + self.overtime_per_day) * self.days_per_week
    }

    /// Check if within legal limits
    pub fn is_within_limits(&self) -> bool {
        let effective_hours = if self.is_ramadan {
            self.hours_per_day.saturating_sub(2)
        } else {
            self.hours_per_day
        };
        effective_hours <= 8 && self.total_weekly_hours() <= 48
    }
}

/// Termination reasons - Article 42-44
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerminationReason {
    /// End of contract term
    ContractExpiry,
    /// Mutual agreement (Article 42(1))
    MutualAgreement,
    /// Employee resignation with notice (Article 43)
    EmployeeResignation,
    /// Employer termination with notice (Article 44)
    EmployerTermination,
    /// Summary dismissal - Article 44 grounds
    SummaryDismissal,
    /// Probation period termination
    ProbationEnd,
    /// Company closure/bankruptcy
    CompanyInsolvency,
    /// Employee death/permanent disability
    DeathOrDisability,
}

impl TerminationReason {
    /// Required notice period in days (30-90 days)
    pub fn notice_period_days(&self) -> u32 {
        match self {
            Self::ContractExpiry | Self::MutualAgreement => 0,
            Self::SummaryDismissal | Self::DeathOrDisability => 0,
            Self::ProbationEnd => 14, // Minimum 14 days during probation
            Self::EmployeeResignation | Self::EmployerTermination | Self::CompanyInsolvency => 30,
        }
    }

    /// Check if employee is entitled to EOSG
    pub fn entitled_to_eosg(&self) -> bool {
        !matches!(self, Self::SummaryDismissal)
    }
}

/// End of Service Gratuity (EOSG) calculation - Article 51
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndOfServiceGratuity {
    /// Years of service (completed)
    pub years_of_service: u32,
    /// Basic salary (monthly)
    pub basic_salary: Aed,
    /// Total gratuity amount
    pub gratuity_amount: Aed,
    /// Daily wage used for calculation
    pub daily_wage: Aed,
    /// Termination reason
    pub termination_reason: TerminationReason,
}

impl EndOfServiceGratuity {
    /// Calculate EOSG based on UAE labor law
    ///
    /// - First 5 years: 21 days basic salary per year
    /// - After 5 years: 30 days basic salary per year
    /// - Maximum total: 2 years of salary
    pub fn calculate(
        years_of_service: u32,
        basic_salary: Aed,
        termination_reason: TerminationReason,
    ) -> Self {
        // Daily wage = monthly salary / 30
        let daily_wage = Aed::from_fils(basic_salary.fils() / 30);

        let gratuity_amount = if years_of_service == 0 || !termination_reason.entitled_to_eosg() {
            Aed::from_fils(0)
        } else {
            // Calculate gratuity
            let years_first_five = years_of_service.min(5);
            let years_after_five = years_of_service.saturating_sub(5);

            // 21 days for first 5 years
            let gratuity_first_five = daily_wage.fils() * 21 * years_first_five as i64;

            // 30 days for years after 5
            let gratuity_after_five = daily_wage.fils() * 30 * years_after_five as i64;

            let total_gratuity = gratuity_first_five + gratuity_after_five;

            // Maximum is 2 years (24 months) of basic salary
            let max_gratuity = basic_salary.fils() * 24;

            Aed::from_fils(total_gratuity.min(max_gratuity))
        };

        Self {
            years_of_service,
            basic_salary,
            gratuity_amount,
            daily_wage,
            termination_reason,
        }
    }
}

/// Leave entitlements - Article 29-33
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeaveType {
    /// Annual leave (30 days after 1 year)
    Annual,
    /// Sick leave (90 days per year)
    Sick,
    /// Maternity leave (60 days, 45 full pay, 15 half pay)
    Maternity,
    /// Paternity leave (5 working days)
    Paternity,
    /// Bereavement leave (3-5 days)
    Bereavement { relationship: String },
    /// Hajj leave (30 days once per career, unpaid)
    Hajj,
    /// Study leave (10 days per year for 2+ years service)
    Study,
    /// National service leave (as required)
    NationalService,
}

impl LeaveType {
    /// Get statutory leave days
    pub fn statutory_days(&self) -> u32 {
        match self {
            Self::Annual => 30, // 30 calendar days after 1 year
            Self::Sick => 90,   // Total per year (45 full, 15 half, 30 unpaid)
            Self::Maternity => 60,
            Self::Paternity => 5,
            Self::Bereavement { .. } => 3, // 3-5 depending on relationship
            Self::Hajj => 30,              // Once per career, unpaid
            Self::Study => 10,             // For 2+ years service
            Self::NationalService => 0,    // As required by law
        }
    }
}

/// Labor Law errors
#[derive(Debug, Error)]
pub enum LaborError {
    /// Working hours exceeded - Article 17
    #[error("تجاوز ساعات العمل (المادة 17): {hours} ساعة/أسبوع (الحد الأقصى 48)")]
    WorkingHoursExceeded { hours: u32 },

    /// Invalid contract duration - Article 8
    #[error("مدة العقد غير صالحة (المادة 8): {months} شهر (الحد الأقصى 36)")]
    InvalidContractDuration { months: u32 },

    /// Minimum leave not provided - Article 29
    #[error(
        "لم يتم توفير الإجازة السنوية المطلوبة (المادة 29): {provided} يوم (المطلوب {required})"
    )]
    InsufficientAnnualLeave { provided: u32, required: u32 },

    /// Probation period exceeded - Article 9
    #[error("فترة الاختبار تجاوزت الحد الأقصى (المادة 9): {days} يوم (الحد الأقصى 180)")]
    ProbationExceeded { days: u32 },

    /// Notice period violation - Article 43
    #[error("انتهاك فترة الإشعار (المادة 43): {provided} يوم (المطلوب {required})")]
    InsufficientNoticePeriod { provided: u32, required: u32 },

    /// Wage protection system violation
    #[error("انتهاك نظام حماية الأجور (WPS): {description}")]
    WpsViolation { description: String },

    /// Contract language requirement
    #[error("يجب أن يكون العقد باللغة العربية (المادة 6)")]
    ContractNotInArabic,
}

/// Validate working hours
pub fn validate_working_hours(hours: &WorkingHours) -> LaborResult<()> {
    let weekly_hours = hours.total_weekly_hours();

    if weekly_hours > 48 {
        return Err(LaborError::WorkingHoursExceeded {
            hours: weekly_hours,
        });
    }

    Ok(())
}

/// Validate contract
pub fn validate_contract(contract_type: &ContractType) -> LaborResult<()> {
    if !contract_type.is_valid_duration()
        && let ContractType::LimitedTerm { duration_months } = contract_type
    {
        return Err(LaborError::InvalidContractDuration {
            months: *duration_months,
        });
    }
    Ok(())
}

/// Calculate EOSG
pub fn calculate_eosg(
    years_of_service: u32,
    basic_salary: Aed,
    termination_reason: TerminationReason,
) -> EndOfServiceGratuity {
    EndOfServiceGratuity::calculate(years_of_service, basic_salary, termination_reason)
}

/// Get labor law compliance checklist
pub fn get_labor_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("عقد العمل بالعربية", "Contract in Arabic", "Article 6"),
        (
            "ساعات العمل 8/يوم، 48/أسبوع",
            "Working hours 8/day, 48/week",
            "Article 17",
        ),
        ("إجازة سنوية 30 يوم", "Annual leave 30 days", "Article 29"),
        (
            "نظام حماية الأجور (WPS)",
            "Wage Protection System",
            "MoHRE Regulation",
        ),
        (
            "إجازة أمومة 60 يوم",
            "Maternity leave 60 days",
            "Article 30",
        ),
        ("إجازة أبوة 5 أيام", "Paternity leave 5 days", "Article 32"),
        (
            "فترة اختبار حد أقصى 6 أشهر",
            "Probation max 6 months",
            "Article 9",
        ),
        (
            "فترة إشعار 30-90 يوم",
            "Notice period 30-90 days",
            "Article 43",
        ),
        (
            "مكافأة نهاية الخدمة",
            "End of Service Gratuity",
            "Article 51",
        ),
        ("حظر التمييز", "Non-discrimination", "Article 4"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_types() {
        let limited = ContractType::LimitedTerm {
            duration_months: 24,
        };
        assert!(limited.is_valid_duration());

        let invalid = ContractType::LimitedTerm {
            duration_months: 48,
        };
        assert!(!invalid.is_valid_duration());
    }

    #[test]
    fn test_working_hours() {
        let hours = WorkingHours::standard();
        assert!(hours.is_within_limits());
        assert_eq!(hours.total_weekly_hours(), 40);
    }

    #[test]
    fn test_working_hours_ramadan() {
        let hours = WorkingHours {
            hours_per_day: 8,
            days_per_week: 5,
            overtime_per_day: 0,
            is_ramadan: true,
        };
        // Ramadan: 6 hours/day instead of 8
        assert_eq!(hours.total_weekly_hours(), 30);
    }

    #[test]
    fn test_eosg_calculation() {
        let salary = Aed::from_dirhams(10000);
        let eosg = EndOfServiceGratuity::calculate(3, salary, TerminationReason::ContractExpiry);

        // 3 years * 21 days * (10000/30) daily wage
        // = 3 * 21 * 333.33 = 20,999.67 ~ 21,000 AED
        assert!(eosg.gratuity_amount.dirhams() > 0);
    }

    #[test]
    fn test_eosg_five_plus_years() {
        let salary = Aed::from_dirhams(10000);
        let eosg = EndOfServiceGratuity::calculate(7, salary, TerminationReason::MutualAgreement);

        // 5 years * 21 days + 2 years * 30 days
        // Higher amount expected
        assert!(eosg.gratuity_amount.dirhams() > 40000);
    }

    #[test]
    fn test_eosg_summary_dismissal() {
        let salary = Aed::from_dirhams(10000);
        let eosg = EndOfServiceGratuity::calculate(5, salary, TerminationReason::SummaryDismissal);

        // No EOSG for summary dismissal
        assert_eq!(eosg.gratuity_amount.dirhams(), 0);
    }

    #[test]
    fn test_leave_entitlements() {
        assert_eq!(LeaveType::Annual.statutory_days(), 30);
        assert_eq!(LeaveType::Maternity.statutory_days(), 60);
        assert_eq!(LeaveType::Paternity.statutory_days(), 5);
    }

    #[test]
    fn test_termination_notice() {
        assert_eq!(
            TerminationReason::EmployeeResignation.notice_period_days(),
            30
        );
        assert_eq!(TerminationReason::SummaryDismissal.notice_period_days(), 0);
    }

    #[test]
    fn test_labor_checklist() {
        let checklist = get_labor_checklist();
        assert!(!checklist.is_empty());
    }

    #[test]
    fn test_validate_working_hours() {
        let valid = WorkingHours::standard();
        assert!(validate_working_hours(&valid).is_ok());

        let invalid = WorkingHours {
            hours_per_day: 10,
            days_per_week: 6,
            overtime_per_day: 0,
            is_ramadan: false,
        };
        assert!(validate_working_hours(&invalid).is_err());
    }
}
