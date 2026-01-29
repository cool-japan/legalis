//! Saudi Labor Law (نظام العمل)
//!
//! Royal Decree No. M/51 dated 23/8/1426H (2005)
//!
//! Regulates employment relationships, working conditions, and labor rights
//! in Saudi Arabia. Includes Nitaqat (Saudization) program requirements.

use crate::common::Sar;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for labor law operations
pub type LaborResult<T> = Result<T, LaborError>;

/// Labor law errors
#[derive(Debug, Error)]
pub enum LaborError {
    /// Invalid working hours
    #[error("ساعات عمل غير صالحة: {reason}")]
    InvalidWorkingHours { reason: String },

    /// Invalid contract
    #[error("عقد عمل غير صالح: {reason}")]
    InvalidContract { reason: String },

    /// Calculation error
    #[error("خطأ في الحساب: {reason}")]
    CalculationError { reason: String },
}

/// Types of employment contracts
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    /// Fixed-term contract (عقد محدد المدة)
    FixedTerm,
    /// Indefinite contract (عقد غير محدد المدة)
    Indefinite,
    /// Part-time contract (عقد دوام جزئي)
    PartTime,
    /// Temporary/Seasonal (عقد موسمي)
    Temporary,
}

impl ContractType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::FixedTerm => "عقد محدد المدة",
            Self::Indefinite => "عقد غير محدد المدة",
            Self::PartTime => "عقد دوام جزئي",
            Self::Temporary => "عقد موسمي",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::FixedTerm => "Fixed-Term Contract",
            Self::Indefinite => "Indefinite Contract",
            Self::PartTime => "Part-Time Contract",
            Self::Temporary => "Temporary/Seasonal Contract",
        }
    }
}

/// Types of leave
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeaveType {
    /// Annual leave (الإجازة السنوية)
    Annual,
    /// Sick leave (الإجازة المرضية)
    Sick,
    /// Maternity leave (إجازة الولادة)
    Maternity,
    /// Hajj leave (إجازة الحج)
    Hajj,
    /// Eid leave (إجازة العيد)
    Eid,
    /// Bereavement leave (إجازة الوفاة)
    Bereavement,
}

impl LeaveType {
    /// Get statutory days
    pub fn statutory_days(&self) -> u32 {
        match self {
            Self::Annual => 21,     // 21 days minimum
            Self::Sick => 120,      // Up to 120 days (with varying pay)
            Self::Maternity => 70,  // 10 weeks total
            Self::Hajj => 10,       // Once in employment
            Self::Eid => 4,         // Eid al-Fitr and Eid al-Adha combined
            Self::Bereavement => 5, // Varies by relationship
        }
    }

    /// Check if paid leave
    pub fn is_paid(&self) -> bool {
        matches!(
            self,
            Self::Annual | Self::Maternity | Self::Hajj | Self::Eid
        )
    }
}

/// Working hours structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingHours {
    /// Daily hours
    pub daily_hours: u32,
    /// Weekly hours
    pub weekly_hours: u32,
    /// Is Ramadan schedule
    pub is_ramadan: bool,
}

impl WorkingHours {
    /// Create standard working hours
    pub fn standard() -> Self {
        Self {
            daily_hours: 8,
            weekly_hours: 48,
            is_ramadan: false,
        }
    }

    /// Create Ramadan working hours (reduced)
    pub fn ramadan() -> Self {
        Self {
            daily_hours: 6,
            weekly_hours: 36,
            is_ramadan: true,
        }
    }

    /// Check if within legal limits
    pub fn is_within_limits(&self) -> bool {
        if self.is_ramadan {
            self.daily_hours <= 6 && self.weekly_hours <= 36
        } else {
            self.daily_hours <= 8 && self.weekly_hours <= 48
        }
    }

    /// Get total weekly hours
    pub fn total_weekly_hours(&self) -> u32 {
        self.weekly_hours
    }
}

/// Reasons for termination
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerminationReason {
    /// Contract expiry
    ContractExpiry,
    /// Resignation
    Resignation,
    /// Dismissal with cause
    DismissalWithCause,
    /// Dismissal without cause
    DismissalWithoutCause,
    /// Mutual agreement
    MutualAgreement,
    /// Retirement
    Retirement,
}

/// Nitaqat (Saudization) categories
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NitaqatCategory {
    /// Platinum (highest Saudization)
    Platinum,
    /// Green (compliant)
    Green,
    /// Yellow (warning)
    Yellow,
    /// Red (non-compliant)
    Red,
}

impl NitaqatCategory {
    /// Get category name in Arabic
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Platinum => "بلاتيني",
            Self::Green => "أخضر",
            Self::Yellow => "أصفر",
            Self::Red => "أحمر",
        }
    }

    /// Get benefits description
    pub fn benefits_en(&self) -> &'static str {
        match self {
            Self::Platinum => "Full benefits, priority services, unlimited foreign worker visas",
            Self::Green => "Compliant, can hire foreign workers, standard services",
            Self::Yellow => "Warning status, limited foreign worker hiring",
            Self::Red => "Non-compliant, cannot hire foreign workers, penalties apply",
        }
    }
}

/// End of Service Award details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndOfServiceAward {
    /// Years of service
    pub years_of_service: u32,
    /// Last monthly salary
    pub last_salary: Sar,
    /// Termination reason
    pub termination_reason: TerminationReason,
    /// Calculated award amount
    pub award_amount: Sar,
}

/// Calculate End of Service Award (مكافأة نهاية الخدمة)
///
/// Calculation:
/// - First 5 years: half month salary per year
/// - After 5 years: one month salary per year
pub fn calculate_eosa(years_of_service: u32, last_salary: Sar) -> EndOfServiceAward {
    let mut award = Sar::from_halalas(0);

    // First 5 years: half month per year
    let first_period = years_of_service.min(5);
    award = award + (last_salary / 2) * (first_period as i64);

    // After 5 years: full month per year
    if years_of_service > 5 {
        let second_period = years_of_service - 5;
        award = award + last_salary * (second_period as i64);
    }

    EndOfServiceAward {
        years_of_service,
        last_salary,
        termination_reason: TerminationReason::ContractExpiry,
        award_amount: award,
    }
}

/// Validate working hours
pub fn validate_working_hours(hours: &WorkingHours) -> LaborResult<()> {
    if !hours.is_within_limits() {
        return Err(LaborError::InvalidWorkingHours {
            reason: format!(
                "Exceeds legal limits: {} hours/day, {} hours/week",
                hours.daily_hours, hours.weekly_hours
            ),
        });
    }
    Ok(())
}

/// Get labor law compliance checklist
pub fn get_labor_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("عقد عمل مكتوب", "Written employment contract"),
        ("الحد الأدنى للأجور", "Minimum wage compliance"),
        ("ساعات العمل", "Working hours (8h/day, 48h/week max)"),
        ("الإجازة السنوية", "Annual leave (21 days minimum)"),
        ("نظام حماية الأجور", "Wage Protection System (WPS)"),
        ("التأمينات الاجتماعية", "GOSI registration"),
        ("السعودة (نطاقات)", "Nitaqat (Saudization) compliance"),
        ("مكافأة نهاية الخدمة", "End of Service Award"),
        ("بيئة عمل آمنة", "Safe working environment"),
        ("عدم التمييز", "Non-discrimination"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_hours() {
        let standard = WorkingHours::standard();
        assert!(standard.is_within_limits());
        assert_eq!(standard.total_weekly_hours(), 48);

        let ramadan = WorkingHours::ramadan();
        assert!(ramadan.is_within_limits());
        assert_eq!(ramadan.total_weekly_hours(), 36);
    }

    #[test]
    fn test_leave_types() {
        assert_eq!(LeaveType::Annual.statutory_days(), 21);
        assert_eq!(LeaveType::Maternity.statutory_days(), 70);
        assert!(LeaveType::Annual.is_paid());
        assert!(!LeaveType::Sick.is_paid());
    }

    #[test]
    fn test_eosa_calculation() {
        // 3 years service
        let eosa3 = calculate_eosa(3, Sar::from_riyals(10_000));
        assert_eq!(eosa3.award_amount.riyals(), 15_000); // 3 * 5000

        // 7 years service
        let eosa7 = calculate_eosa(7, Sar::from_riyals(10_000));
        // First 5 years: 5 * 5000 = 25000
        // Next 2 years: 2 * 10000 = 20000
        // Total: 45000
        assert_eq!(eosa7.award_amount.riyals(), 45_000);
    }

    #[test]
    fn test_contract_types() {
        assert_eq!(ContractType::FixedTerm.name_ar(), "عقد محدد المدة");
        assert_eq!(ContractType::Indefinite.name_en(), "Indefinite Contract");
    }

    #[test]
    fn test_nitaqat_categories() {
        assert_eq!(NitaqatCategory::Platinum.name_ar(), "بلاتيني");
        assert!(NitaqatCategory::Green.benefits_en().contains("Compliant"));
    }

    #[test]
    fn test_invalid_working_hours() {
        let excessive = WorkingHours {
            daily_hours: 10,
            weekly_hours: 60,
            is_ramadan: false,
        };
        assert!(!excessive.is_within_limits());
        assert!(validate_working_hours(&excessive).is_err());
    }
}
