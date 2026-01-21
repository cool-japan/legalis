//! LPA Types and Structures

use crate::calendar::BuddhistYear;
use crate::citation::ThaiAct;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Employment types under Thai law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentType {
    /// Indefinite term (ไม่มีกำหนดระยะเวลา)
    Indefinite,

    /// Fixed term (มีกำหนดระยะเวลา) - max 2 years
    FixedTerm,

    /// Seasonal/Project-based (งานตามฤดูกาลหรือโครงการ)
    Seasonal,

    /// Part-time (ทำงานบางเวลา)
    PartTime,

    /// Probationary (ทดลองงาน) - max 120 days
    Probationary,

    /// Trainee/Apprentice (ฝึกงาน)
    Trainee,
}

impl EmploymentType {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Indefinite => "พนักงานประจำไม่มีกำหนดระยะเวลา",
            Self::FixedTerm => "พนักงานสัญญาจ้างมีกำหนดระยะเวลา",
            Self::Seasonal => "พนักงานตามฤดูกาลหรือโครงการ",
            Self::PartTime => "พนักงานบางเวลา",
            Self::Probationary => "พนักงานทดลองงาน",
            Self::Trainee => "พนักงานฝึกงาน",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Indefinite => "Indefinite Employment",
            Self::FixedTerm => "Fixed-Term Contract",
            Self::Seasonal => "Seasonal/Project-Based",
            Self::PartTime => "Part-Time",
            Self::Probationary => "Probationary",
            Self::Trainee => "Trainee/Apprentice",
        }
    }

    /// Check if entitled to severance pay
    pub fn entitled_to_severance(&self) -> bool {
        matches!(self, Self::Indefinite | Self::FixedTerm)
    }

    /// Check if covered by Social Security Fund
    pub fn ssf_covered(&self) -> bool {
        !matches!(self, Self::Trainee)
    }
}

/// Termination types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminationType {
    /// Termination without cause (เลิกจ้างโดยไม่มีความผิด)
    WithoutCause,

    /// Termination for cause (เลิกจ้างโดยมีความผิดร้ายแรง - Section 119)
    ForCause,

    /// Resignation (ลาออก)
    Resignation,

    /// Mutual agreement (ตกลงยุติสัญญา)
    MutualAgreement,

    /// End of fixed term (สิ้นสุดระยะเวลาสัญญา)
    EndOfTerm,

    /// Retirement (เกษียณอายุ)
    Retirement,

    /// Death (เสียชีวิต)
    Death,
}

impl TerminationType {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::WithoutCause => "เลิกจ้างโดยไม่มีความผิด",
            Self::ForCause => "เลิกจ้างโดยมีความผิดร้ายแรง",
            Self::Resignation => "ลาออก",
            Self::MutualAgreement => "ตกลงยุติสัญญา",
            Self::EndOfTerm => "สิ้นสุดระยะเวลาสัญญา",
            Self::Retirement => "เกษียณอายุ",
            Self::Death => "เสียชีวิต",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::WithoutCause => "Termination Without Cause",
            Self::ForCause => "Termination For Cause",
            Self::Resignation => "Resignation",
            Self::MutualAgreement => "Mutual Agreement",
            Self::EndOfTerm => "End of Fixed Term",
            Self::Retirement => "Retirement",
            Self::Death => "Death",
        }
    }

    /// Check if severance pay is required
    pub fn requires_severance(&self) -> bool {
        matches!(
            self,
            Self::WithoutCause | Self::Retirement | Self::EndOfTerm
        )
    }

    /// Check if notice is required
    pub fn requires_notice(&self) -> bool {
        matches!(
            self,
            Self::WithoutCause | Self::Resignation | Self::Retirement
        )
    }
}

/// Just causes for termination (Section 119)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JustCause {
    /// Dishonest acts or intentional criminal acts against employer (119(1))
    DishonestActs,

    /// Willful damage to employer's property (119(2))
    WillfulDamage,

    /// Violation causing serious damage to employer (119(3))
    SeriousViolation,

    /// Absence without justification for 3+ consecutive days (119(4))
    Abandonment,

    /// Imprisonment for non-negligence offense (119(5))
    Imprisonment,

    /// Gross negligence (119(6))
    GrossNegligence,
}

impl JustCause {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::DishonestActs => "ทุจริตต่อหน้าที่หรือกระทำความผิดอาญาโดยเจตนาแก่นายจ้าง",
            Self::WillfulDamage => "จงใจทำให้นายจ้างได้รับความเสียหาย",
            Self::SeriousViolation => "ประมาทเลินเล่อเป็นเหตุให้นายจ้างได้รับความเสียหายอย่างร้ายแรง",
            Self::Abandonment => "ละทิ้งหน้าที่โดยไม่มีเหตุอันสมควรเกิน 3 วันทำงานติดต่อกัน",
            Self::Imprisonment => "ได้รับโทษจำคุกตามคำพิพากษาถึงที่สุดในความผิดที่กระทำโดยเจตนา",
            Self::GrossNegligence => "ฝ่าฝืนข้อบังคับหรือระเบียบการทำงานอย่างร้ายแรง",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::DishonestActs => "Dishonest acts or intentional crimes against employer",
            Self::WillfulDamage => "Willful damage to employer's property",
            Self::SeriousViolation => "Serious negligence causing damage to employer",
            Self::Abandonment => "Abandonment for 3+ consecutive workdays",
            Self::Imprisonment => "Imprisonment for intentional offense",
            Self::GrossNegligence => "Serious violation of work rules",
        }
    }

    /// Get LPA citation
    pub fn citation(&self) -> String {
        let lpa = ThaiAct::new(
            "คุ้มครองแรงงาน",
            "Labour Protection Act",
            BuddhistYear::from_be(2541),
        );

        let paragraph = match self {
            Self::DishonestActs => "1",
            Self::WillfulDamage => "2",
            Self::SeriousViolation => "3",
            Self::Abandonment => "4",
            Self::Imprisonment => "5",
            Self::GrossNegligence => "6",
        };

        format!("{} ({}) ", lpa.section(119).format_th(), paragraph)
    }
}

/// Working hours configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingHours {
    /// Daily working hours (max 8)
    pub daily_hours: u32,

    /// Weekly working hours (max 48)
    pub weekly_hours: u32,

    /// Weekly overtime hours (max 36)
    pub overtime_hours: u32,

    /// Schedule type
    pub schedule_type: ScheduleType,
}

impl WorkingHours {
    /// Create standard 8h/day, 6 days/week schedule
    pub fn standard() -> Self {
        Self {
            daily_hours: 8,
            weekly_hours: 48,
            overtime_hours: 0,
            schedule_type: ScheduleType::Standard,
        }
    }

    /// Check if hours exceed legal maximum
    pub fn exceeds_legal_limit(&self) -> bool {
        self.daily_hours > 8 || self.weekly_hours > 48 || self.overtime_hours > 36
    }

    /// Calculate weekly overtime
    pub fn weekly_overtime(&self) -> u32 {
        self.weekly_hours.saturating_sub(48)
    }
}

/// Work schedule types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleType {
    /// Standard 8h/day, 6 days
    Standard,

    /// Office 8h/day, 5 days (40h/week)
    Office,

    /// Part-time
    PartTime,

    /// Shift work
    Shift,

    /// Flexible hours
    Flexible,
}

impl ScheduleType {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Standard => "ทำงานปกติ 8 ชม./วัน 6 วัน/สัปดาห์",
            Self::Office => "ทำงานสำนักงาน 8 ชม./วัน 5 วัน/สัปดาห์",
            Self::PartTime => "ทำงานบางเวลา",
            Self::Shift => "ทำงานเป็นกะ",
            Self::Flexible => "ชั่วโมงทำงานยืดหยุ่น",
        }
    }
}

/// Severance calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Severance {
    /// Years of service
    pub years_of_service: u32,

    /// Days of severance pay entitled
    pub severance_days: u32,

    /// Monthly wage for calculation
    pub monthly_wage: u32,

    /// Total severance amount (THB)
    pub total_amount: u32,

    /// LPA citation
    pub citation: String,
}

impl Severance {
    /// Calculate severance based on tenure (Section 118)
    pub fn calculate(years_of_service: u32, monthly_wage: u32) -> Self {
        let severance_days = match years_of_service {
            0 => 0,
            y if y < 1 => 30,   // 120 days - 1 year: 30 days
            y if y < 3 => 90,   // 1-3 years: 90 days
            y if y < 6 => 180,  // 3-6 years: 180 days
            y if y < 10 => 240, // 6-10 years: 240 days
            y if y < 20 => 300, // 10-20 years: 300 days
            _ => 400,           // 20+ years: 400 days
        };

        let daily_wage = monthly_wage / 30;
        let total_amount = daily_wage * severance_days;

        let lpa = ThaiAct::new(
            "คุ้มครองแรงงาน",
            "Labour Protection Act",
            BuddhistYear::from_be(2541),
        );

        Self {
            years_of_service,
            severance_days,
            monthly_wage,
            total_amount,
            citation: lpa.section(118).format_th(),
        }
    }
}

/// Employment contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentContract {
    /// Employee name
    pub employee_name: String,

    /// Employer name
    pub employer_name: String,

    /// Employment type
    pub employment_type: EmploymentType,

    /// Start date
    pub start_date: NaiveDate,

    /// End date (for fixed-term)
    pub end_date: Option<NaiveDate>,

    /// Monthly wage (THB)
    pub monthly_wage: u32,

    /// Working hours configuration
    pub working_hours: WorkingHours,

    /// Probation period days (max 120)
    pub probation_days: Option<u32>,
}

impl EmploymentContract {
    /// Calculate years of service
    pub fn years_of_service(&self, as_of: NaiveDate) -> u32 {
        let days = (as_of - self.start_date).num_days();
        if days < 0 {
            return 0;
        }
        (days / 365) as u32
    }

    /// Calculate severance if terminated
    pub fn calculate_severance(&self, termination_date: NaiveDate) -> Severance {
        let years = self.years_of_service(termination_date);
        Severance::calculate(years, self.monthly_wage)
    }

    /// Get notice period days required
    pub fn notice_period_days(&self) -> u32 {
        // Standard is 1 pay period or 30 days
        30
    }
}

/// Labor rights under LPA
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaborRight {
    /// 8-hour workday (Section 23)
    EightHourDay,

    /// 48-hour workweek (Section 23)
    FourtyEightHourWeek,

    /// Overtime pay (Section 61)
    OvertimePay,

    /// Rest period (Section 27)
    RestPeriod,

    /// Weekly rest day (Section 28)
    WeeklyRest,

    /// Public holidays (Section 29)
    PublicHolidays,

    /// Annual leave (Section 30)
    AnnualLeave,

    /// Sick leave (Section 32)
    SickLeave,

    /// Maternity leave (Section 41)
    MaternityLeave,

    /// Severance pay (Section 118)
    SeverancePay,

    /// Minimum wage (Section 90)
    MinimumWage,
}

impl LaborRight {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::EightHourDay => "ทำงานไม่เกิน 8 ชั่วโมงต่อวัน",
            Self::FourtyEightHourWeek => "ทำงานไม่เกิน 48 ชั่วโมงต่อสัปดาห์",
            Self::OvertimePay => "ค่าล่วงเวลา",
            Self::RestPeriod => "เวลาพักระหว่างทำงาน",
            Self::WeeklyRest => "วันหยุดประจำสัปดาห์",
            Self::PublicHolidays => "วันหยุดตามประเพณี",
            Self::AnnualLeave => "วันลาพักผ่อนประจำปี",
            Self::SickLeave => "วันลาป่วย",
            Self::MaternityLeave => "ลาคลอด",
            Self::SeverancePay => "ค่าชดเชย",
            Self::MinimumWage => "ค่าจ้างขั้นต่ำ",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::EightHourDay => "8-Hour Workday",
            Self::FourtyEightHourWeek => "48-Hour Workweek",
            Self::OvertimePay => "Overtime Pay",
            Self::RestPeriod => "Rest Period",
            Self::WeeklyRest => "Weekly Rest Day",
            Self::PublicHolidays => "Public Holidays",
            Self::AnnualLeave => "Annual Leave",
            Self::SickLeave => "Sick Leave",
            Self::MaternityLeave => "Maternity Leave",
            Self::SeverancePay => "Severance Pay",
            Self::MinimumWage => "Minimum Wage",
        }
    }

    /// Get LPA section number
    pub fn section(&self) -> u32 {
        match self {
            Self::EightHourDay | Self::FourtyEightHourWeek => 23,
            Self::OvertimePay => 61,
            Self::RestPeriod => 27,
            Self::WeeklyRest => 28,
            Self::PublicHolidays => 29,
            Self::AnnualLeave => 30,
            Self::SickLeave => 32,
            Self::MaternityLeave => 41,
            Self::SeverancePay => 118,
            Self::MinimumWage => 90,
        }
    }

    /// Get entitlement description
    pub fn entitlement_th(&self) -> &'static str {
        match self {
            Self::EightHourDay => "ไม่เกิน 8 ชั่วโมงต่อวัน",
            Self::FourtyEightHourWeek => "ไม่เกิน 48 ชั่วโมงต่อสัปดาห์",
            Self::OvertimePay => "1.5x วันปกติ, 3x วันหยุด",
            Self::RestPeriod => "ไม่น้อยกว่า 1 ชั่วโมงหลังทำงาน 5 ชั่วโมง",
            Self::WeeklyRest => "ไม่น้อยกว่า 1 วันต่อสัปดาห์",
            Self::PublicHolidays => "ไม่น้อยกว่า 13 วันต่อปี",
            Self::AnnualLeave => "ไม่น้อยกว่า 6 วันต่อปี (หลังทำงาน 1 ปี)",
            Self::SickLeave => "ไม่เกิน 30 วันต่อปี (จ่ายค่าจ้าง)",
            Self::MaternityLeave => "98 วัน (จ่ายค่าจ้าง 45 วัน)",
            Self::SeverancePay => "30-400 วัน ตามอายุงาน",
            Self::MinimumWage => "ตามประกาศกระทรวงแรงงาน",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employment_type() {
        assert!(EmploymentType::Indefinite.entitled_to_severance());
        assert!(!EmploymentType::Trainee.ssf_covered());
    }

    #[test]
    fn test_termination_type() {
        assert!(TerminationType::WithoutCause.requires_severance());
        assert!(!TerminationType::ForCause.requires_severance());
        assert!(TerminationType::Resignation.requires_notice());
    }

    #[test]
    fn test_just_cause_citation() {
        let cause = JustCause::Abandonment;
        let citation = cause.citation();
        assert!(citation.contains("มาตรา 119"));
    }

    #[test]
    fn test_working_hours() {
        let hours = WorkingHours::standard();
        assert_eq!(hours.daily_hours, 8);
        assert_eq!(hours.weekly_hours, 48);
        assert!(!hours.exceeds_legal_limit());

        let excessive = WorkingHours {
            daily_hours: 10,
            weekly_hours: 60,
            overtime_hours: 40,
            schedule_type: ScheduleType::Standard,
        };
        assert!(excessive.exceeds_legal_limit());
    }

    #[test]
    fn test_severance_calculation() {
        // Less than 1 year: 30 days
        let sev = Severance::calculate(0, 30_000);
        assert_eq!(sev.severance_days, 0);

        // 1-3 years: 90 days
        let sev = Severance::calculate(2, 30_000);
        assert_eq!(sev.severance_days, 90);

        // 3-6 years: 180 days
        let sev = Severance::calculate(5, 30_000);
        assert_eq!(sev.severance_days, 180);

        // 6-10 years: 240 days
        let sev = Severance::calculate(8, 30_000);
        assert_eq!(sev.severance_days, 240);

        // 10-20 years: 300 days
        let sev = Severance::calculate(15, 30_000);
        assert_eq!(sev.severance_days, 300);

        // 20+ years: 400 days
        let sev = Severance::calculate(25, 30_000);
        assert_eq!(sev.severance_days, 400);
    }

    #[test]
    fn test_labor_right_section() {
        assert_eq!(LaborRight::EightHourDay.section(), 23);
        assert_eq!(LaborRight::SeverancePay.section(), 118);
    }

    #[test]
    fn test_employment_contract() {
        let contract = EmploymentContract {
            employee_name: "สมชาย".to_string(),
            employer_name: "บริษัท ทดสอบ จำกัด".to_string(),
            employment_type: EmploymentType::Indefinite,
            start_date: NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
            end_date: None,
            monthly_wage: 30_000,
            working_hours: WorkingHours::standard(),
            probation_days: Some(120),
        };

        let years = contract.years_of_service(NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid"));
        assert_eq!(years, 4);

        let severance =
            contract.calculate_severance(NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid"));
        assert_eq!(severance.severance_days, 180); // 3-6 years
    }
}
