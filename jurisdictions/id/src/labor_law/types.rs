//! Types for Indonesian Labor Law (UU Ketenagakerjaan)

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Type of employment contract - Pasal 56-59
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    /// PKWTT - Perjanjian Kerja Waktu Tidak Tertentu (Permanent)
    Pkwtt,
    /// PKWT - Perjanjian Kerja Waktu Tertentu (Fixed-term)
    /// Under Omnibus Law, max 5 years including extensions
    Pkwt {
        /// Duration in months
        duration_months: u32,
        /// Number of extensions
        extensions: u32,
    },
    /// Outsourcing contract
    Outsourcing,
    /// Apprenticeship/Internship
    Magang,
    /// Daily worker (pekerja harian lepas)
    DailyWorker,
}

impl ContractType {
    /// Check if this is a permanent contract
    pub fn is_permanent(&self) -> bool {
        matches!(self, Self::Pkwtt)
    }

    /// Get maximum duration in months for PKWT
    pub fn max_pkwt_duration_months() -> u32 {
        60 // 5 years under Omnibus Law
    }

    /// Check if PKWT duration is valid
    pub fn is_valid_pkwt_duration(&self) -> bool {
        match self {
            Self::Pkwt {
                duration_months,
                extensions,
            } => {
                // Total duration including extensions must not exceed 5 years
                let total_months = duration_months * (extensions + 1);
                total_months <= Self::max_pkwt_duration_months()
            }
            _ => true,
        }
    }
}

/// Employment contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentContract {
    /// Contract type
    pub contract_type: ContractType,
    /// Employee name
    pub employee_name: String,
    /// Employee NIK (ID number)
    pub employee_nik: Option<String>,
    /// Employer/company name
    pub employer_name: String,
    /// Job position
    pub position: String,
    /// Contract start date
    pub start_date: NaiveDate,
    /// Contract end date (None for PKWTT)
    pub end_date: Option<NaiveDate>,
    /// Monthly base salary (Rupiah)
    pub monthly_salary: i64,
    /// Province for minimum wage calculation
    pub province: String,
    /// Working hours schedule
    pub working_hours: WorkingHours,
    /// Whether contract is in Bahasa Indonesia
    pub in_indonesian: bool,
    /// Whether contract is written
    pub is_written: bool,
    /// Probation period in months (max 3 for PKWTT, not allowed for PKWT)
    pub probation_months: Option<u32>,
    /// Date contract was signed
    pub signed_date: DateTime<Utc>,
}

/// Working hours configuration - Pasal 77
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingHours {
    /// Hours per day
    pub hours_per_day: u32,
    /// Days per week
    pub days_per_week: u32,
    /// Overtime hours per week
    pub overtime_hours_per_week: u32,
}

impl WorkingHours {
    /// Standard 5-day work week (8 hours/day, 40 hours/week)
    pub fn standard_5_day() -> Self {
        Self {
            hours_per_day: 8,
            days_per_week: 5,
            overtime_hours_per_week: 0,
        }
    }

    /// Standard 6-day work week (7 hours/day, 42 hours/week)
    pub fn standard_6_day() -> Self {
        Self {
            hours_per_day: 7,
            days_per_week: 6,
            overtime_hours_per_week: 0,
        }
    }

    /// Calculate total weekly hours
    pub fn total_weekly_hours(&self) -> u32 {
        (self.hours_per_day * self.days_per_week) + self.overtime_hours_per_week
    }

    /// Check if working hours are within legal limits
    pub fn is_within_limits(&self) -> bool {
        let regular_hours = self.hours_per_day * self.days_per_week;

        // Regular hours must not exceed 40 per week - Pasal 77
        if regular_hours > 40 {
            return false;
        }

        // Overtime must not exceed 4 hours/day and 18 hours/week
        // Under Omnibus Law PP 35/2021
        if self.overtime_hours_per_week > 18 {
            return false;
        }

        true
    }
}

/// Termination type - Pasal 154A-172 (Omnibus Law)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerminationType {
    /// Resignation by employee (pengunduran diri)
    Resignation,
    /// End of PKWT contract period
    ContractExpiry,
    /// Termination for cause (kesalahan berat)
    ForCause,
    /// Mutual agreement (kesepakatan bersama)
    MutualAgreement,
    /// Redundancy/efficiency (efisiensi)
    Redundancy,
    /// Company closure (perusahaan tutup)
    CompanyClosure,
    /// Company bankruptcy (pailit)
    Bankruptcy,
    /// Retirement (pensiun)
    Retirement,
    /// Employee death (meninggal dunia)
    Death,
    /// Prolonged illness (sakit berkepanjangan)
    ProlongedIllness,
    /// Detention (ditahan pihak berwajib)
    Detention,
    /// Violation of contract/regulations
    Violation,
}

impl TerminationType {
    /// Get severance multiplier based on termination type (Omnibus Law)
    pub fn severance_multiplier(&self) -> f64 {
        match self {
            Self::Resignation => 0.0,
            Self::ContractExpiry => 0.0, // Compensation handled separately
            Self::ForCause => 0.0,
            Self::MutualAgreement => 1.0,
            Self::Redundancy => 0.5,       // 0.5x under Omnibus Law
            Self::CompanyClosure => 1.0,   // Full severance
            Self::Bankruptcy => 0.5,       // 0.5x
            Self::Retirement => 1.0,       // Full
            Self::Death => 1.0,            // Full (to heirs)
            Self::ProlongedIllness => 1.0, // Full
            Self::Detention => 0.5,        // 0.5x
            Self::Violation => 0.5,        // 0.5x
        }
    }

    /// Check if termination requires prior warning letter
    pub fn requires_warning_letter(&self) -> bool {
        matches!(self, Self::Violation | Self::ForCause | Self::Redundancy)
    }
}

/// Severance calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Severance {
    /// Years of service
    pub years_of_service: u32,
    /// Monthly wage for calculation
    pub monthly_wage: i64,
    /// Uang Pesangon (severance pay) in months
    pub pesangon_months: u32,
    /// Uang Penghargaan Masa Kerja (service appreciation) in months
    pub upmk_months: u32,
    /// Uang Penggantian Hak (entitlement replacement) in months
    pub uph_months: f64,
    /// Total severance amount
    pub total_amount: i64,
    /// Termination type
    pub termination_type: TerminationType,
}

impl Severance {
    /// Calculate severance based on tenure and termination type
    /// Based on PP 35/2021 (Omnibus Law implementation)
    pub fn calculate(
        years_of_service: u32,
        monthly_wage: i64,
        termination_type: TerminationType,
    ) -> Self {
        // Uang Pesangon - Pasal 40 PP 35/2021
        let pesangon_months = match years_of_service {
            0 => 0,
            1..=2 => 1,
            3..=4 => 2,
            5..=6 => 3,
            7..=8 => 4,
            9..=12 => 5,
            13..=16 => 6,
            17..=20 => 7,
            21..=24 => 8,
            _ => 9, // Max 9 months
        };

        // Uang Penghargaan Masa Kerja (UPMK) - Pasal 40 PP 35/2021
        let upmk_months = match years_of_service {
            0..=3 => 0,
            4..=6 => 2,
            7..=9 => 3,
            10..=12 => 4,
            13..=15 => 5,
            16..=18 => 6,
            19..=21 => 7,
            22..=24 => 8,
            _ => 10, // Max 10 months
        };

        // Apply termination type multiplier
        let multiplier = termination_type.severance_multiplier();

        // Uang Penggantian Hak (UPH) - estimated at 15%, only if severance applies
        let uph_months = if multiplier > 0.0 {
            0.15 * (pesangon_months + upmk_months) as f64
        } else {
            0.0
        };

        let base_amount = monthly_wage * (pesangon_months + upmk_months) as i64;
        let uph_amount = (monthly_wage as f64 * uph_months * multiplier) as i64;
        let total_amount = ((base_amount as f64 * multiplier) as i64) + uph_amount;

        Self {
            years_of_service,
            monthly_wage,
            pesangon_months,
            upmk_months,
            uph_months,
            total_amount,
            termination_type,
        }
    }
}

/// Types of leave - Pasal 79-84
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeaveType {
    /// Annual leave (cuti tahunan) - 12 days after 1 year
    Annual,
    /// Sick leave (cuti sakit)
    Sick,
    /// Maternity leave (cuti melahirkan) - 3 months
    Maternity,
    /// Paternity leave (cuti ayah) - 2 days
    Paternity,
    /// Marriage leave (cuti menikah) - 3 days
    Marriage,
    /// Child marriage (anak menikah) - 2 days
    ChildMarriage,
    /// Circumcision/baptism of child - 2 days
    ChildCircumcisionBaptism,
    /// Family death (kematian keluarga) - 2 days
    FamilyDeath,
    /// Household member death - 1 day
    HouseholdMemberDeath,
    /// Menstrual leave (cuti haid) - 2 days
    Menstrual,
    /// Hajj leave (ibadah haji) - once during employment
    Hajj,
    /// Public holiday (libur nasional)
    PublicHoliday,
}

impl LeaveType {
    /// Get default days for this leave type
    pub fn default_days(&self) -> u32 {
        match self {
            Self::Annual => 12,
            Self::Sick => 30,      // Paid for first 12 months
            Self::Maternity => 90, // 3 months (1.5 before, 1.5 after)
            Self::Paternity => 2,
            Self::Marriage => 3,
            Self::ChildMarriage => 2,
            Self::ChildCircumcisionBaptism => 2,
            Self::FamilyDeath => 2,
            Self::HouseholdMemberDeath => 1,
            Self::Menstrual => 2,
            Self::Hajj => 40, // Once during employment
            Self::PublicHoliday => 14,
        }
    }

    /// Check if leave is paid
    pub fn is_paid(&self) -> bool {
        match self {
            Self::Sick => true, // Graduated reduction after 4 months
            Self::Maternity => true,
            Self::Annual => true,
            Self::Marriage => true,
            Self::ChildMarriage => true,
            Self::ChildCircumcisionBaptism => true,
            Self::FamilyDeath => true,
            Self::HouseholdMemberDeath => true,
            Self::Paternity => true,
            Self::Menstrual => true,
            Self::Hajj => true, // Once per employment
            Self::PublicHoliday => true,
        }
    }
}

/// BPJS contribution breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpjsContribution {
    /// BPJS Kesehatan (Health) - 5% total (4% employer, 1% employee)
    pub health_employer: i64,
    pub health_employee: i64,
    /// BPJS Ketenagakerjaan - JKK (Work Accident) - 0.24%-1.74% employer
    pub jkk: i64,
    /// BPJS Ketenagakerjaan - JKM (Death) - 0.30% employer
    pub jkm: i64,
    /// BPJS Ketenagakerjaan - JHT (Old Age) - 5.7% (3.7% employer, 2% employee)
    pub jht_employer: i64,
    pub jht_employee: i64,
    /// BPJS Ketenagakerjaan - JP (Pension) - 3% (2% employer, 1% employee)
    pub jp_employer: i64,
    pub jp_employee: i64,
    /// Total employer contribution
    pub total_employer: i64,
    /// Total employee contribution
    pub total_employee: i64,
    /// Grand total
    pub total: i64,
}

impl BpjsContribution {
    /// Calculate BPJS contributions based on monthly wage
    pub fn calculate(monthly_wage: i64, jkk_rate: f64) -> Self {
        // BPJS Kesehatan (capped at certain amount)
        let health_cap = 12_000_000i64; // Cap for calculation
        let health_base = monthly_wage.min(health_cap);
        let health_employer = (health_base as f64 * 0.04) as i64;
        let health_employee = (health_base as f64 * 0.01) as i64;

        // BPJS Ketenagakerjaan
        let jkk = (monthly_wage as f64 * jkk_rate) as i64;
        let jkm = (monthly_wage as f64 * 0.003) as i64;

        // JHT (Old Age Savings) - capped
        let jht_cap = 9_559_600i64; // 2024 cap
        let jht_base = monthly_wage.min(jht_cap);
        let jht_employer = (jht_base as f64 * 0.037) as i64;
        let jht_employee = (jht_base as f64 * 0.02) as i64;

        // JP (Pension) - capped
        let jp_cap = 9_559_600i64;
        let jp_base = monthly_wage.min(jp_cap);
        let jp_employer = (jp_base as f64 * 0.02) as i64;
        let jp_employee = (jp_base as f64 * 0.01) as i64;

        let total_employer = health_employer + jkk + jkm + jht_employer + jp_employer;
        let total_employee = health_employee + jht_employee + jp_employee;

        Self {
            health_employer,
            health_employee,
            jkk,
            jkm,
            jht_employer,
            jht_employee,
            jp_employer,
            jp_employee,
            total_employer,
            total_employee,
            total: total_employer + total_employee,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_hours_limits() {
        let standard = WorkingHours::standard_5_day();
        assert!(standard.is_within_limits());
        assert_eq!(standard.total_weekly_hours(), 40);

        let excessive = WorkingHours {
            hours_per_day: 10,
            days_per_week: 6,
            overtime_hours_per_week: 0,
        };
        assert!(!excessive.is_within_limits());
    }

    #[test]
    fn test_severance_calculation() {
        let severance = Severance::calculate(10, 10_000_000, TerminationType::MutualAgreement);

        assert_eq!(severance.pesangon_months, 5); // 9-12 years = 5 months
        assert_eq!(severance.upmk_months, 4); // 10-12 years = 4 months
        assert!(severance.total_amount > 0);
    }

    #[test]
    fn test_severance_resignation() {
        let severance = Severance::calculate(5, 10_000_000, TerminationType::Resignation);
        assert_eq!(severance.total_amount, 0);
    }

    #[test]
    fn test_bpjs_contribution() {
        let contribution = BpjsContribution::calculate(10_000_000, 0.0054); // Low risk rate

        assert!(contribution.health_employer > 0);
        assert!(contribution.health_employee > 0);
        assert!(contribution.total_employer > contribution.total_employee);
    }

    #[test]
    fn test_leave_types() {
        assert_eq!(LeaveType::Annual.default_days(), 12);
        assert_eq!(LeaveType::Maternity.default_days(), 90);
        assert!(LeaveType::Annual.is_paid());
    }

    #[test]
    fn test_pkwt_duration() {
        let valid_pkwt = ContractType::Pkwt {
            duration_months: 24,
            extensions: 1,
        };
        assert!(valid_pkwt.is_valid_pkwt_duration());

        let invalid_pkwt = ContractType::Pkwt {
            duration_months: 36,
            extensions: 2,
        };
        assert!(!invalid_pkwt.is_valid_pkwt_duration()); // 108 months > 60
    }
}
