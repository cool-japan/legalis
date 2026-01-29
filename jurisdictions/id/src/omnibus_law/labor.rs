//! Labor Law Amendments under Omnibus Law
//!
//! ## Key Changes to UU 13/2003 (Manpower Law)
//!
//! - **PKWT (Fixed-term contracts)**: Extended to max 5 years (from 2 years)
//! - **Severance calculation**: Simplified formula
//! - **Outsourcing**: Expanded permitted activities
//! - **Working hours**: Flexibility in arrangements
//! - **Wages**: Provincial minimum wage (UMP) becomes primary benchmark
//! - **Job loss guarantee (JKP)**: New social security program

use serde::{Deserialize, Serialize};

/// Contract types under Omnibus Law amendments
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OmnibusContractType {
    /// PKWTT - Permanent contract (unchanged)
    Pkwtt,
    /// PKWT - Fixed-term contract (extended to max 5 years including renewals)
    PkwtExtended {
        /// Duration in years
        duration_years: u32,
    },
    /// Outsourcing - expanded activities allowed
    OutsourcingExpanded,
}

impl OmnibusContractType {
    /// Maximum PKWT duration under Omnibus Law (5 years total)
    pub fn max_pkwt_duration_years() -> u32 {
        5
    }

    /// Check if PKWT duration is valid under Omnibus Law
    pub fn is_valid_pkwt_duration(duration_years: u32) -> bool {
        duration_years <= Self::max_pkwt_duration_years()
    }

    /// Get contract type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Pkwtt => "PKWTT (Perjanjian Kerja Waktu Tidak Tertentu)",
            Self::PkwtExtended { .. } => "PKWT Diperpanjang (maksimal 5 tahun)",
            Self::OutsourcingExpanded => "Outsourcing dengan Kegiatan Diperluas",
        }
    }

    /// Get contract type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Pkwtt => "Permanent Contract (PKWTT)",
            Self::PkwtExtended { .. } => "Extended Fixed-Term Contract (PKWT)",
            Self::OutsourcingExpanded => "Expanded Outsourcing",
        }
    }
}

/// Severance calculation under Omnibus Law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmnibusSeverance {
    /// Years of service
    pub years_of_service: u32,
    /// Monthly salary (Rupiah)
    pub monthly_salary: i64,
    /// Termination reason
    pub termination_reason: OmnibusTerminationReason,
}

/// Termination reasons under Omnibus Law
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OmnibusTerminationReason {
    /// Company efficiency/restructuring
    Efficiency,
    /// Company closure
    Closure,
    /// Company bankruptcy
    Bankruptcy,
    /// Merger/acquisition/separation
    MergerAcquisition,
    /// Employee misconduct (serious)
    SeriousMisconduct,
    /// Mutual agreement
    MutualAgreement,
    /// Resignation
    Resignation,
    /// Death
    Death,
    /// Retirement
    Retirement,
}

impl OmnibusSeverance {
    /// Calculate severance pay multiplier under Omnibus Law PP 35/2021
    /// Returns (severance_months, service_appreciation_months, compensation_months)
    pub fn calculate_multiplier(&self) -> (u32, u32, u32) {
        let severance = match self.years_of_service {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            5 => 5,
            6 => 6,
            7 => 7,
            8..=23 => 8,
            _ => 9, // 24+ years
        };

        let service_appreciation = match self.years_of_service {
            0..=2 => 0,
            3..=5 => 1,
            6..=8 => 2,
            9..=11 => 3,
            12..=14 => 4,
            15..=17 => 5,
            18..=20 => 6,
            21..=23 => 7,
            _ => 8, // 24+ years
        };

        let compensation = match self.termination_reason {
            OmnibusTerminationReason::Efficiency
            | OmnibusTerminationReason::Closure
            | OmnibusTerminationReason::Bankruptcy
            | OmnibusTerminationReason::MergerAcquisition => {
                // Rights compensation for these reasons
                15 // 15% of severance + service appreciation
            }
            _ => 0,
        };

        (severance, service_appreciation, compensation)
    }

    /// Calculate total severance amount
    pub fn calculate_total_severance(&self) -> i64 {
        let (sev, service, comp_percent) = self.calculate_multiplier();

        let severance_amount = self.monthly_salary * sev as i64;
        let service_amount = self.monthly_salary * service as i64;
        let base_amount = severance_amount + service_amount;
        let compensation_amount = (base_amount as f64 * comp_percent as f64 / 100.0).round() as i64;

        severance_amount + service_amount + compensation_amount
    }
}

/// Outsourcing regulations under Omnibus Law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmnibusOutsourcing {
    /// Type of work outsourced
    pub work_type: OutsourcingWorkType,
    /// Whether work is permitted to be outsourced under Omnibus Law
    pub is_permitted: bool,
    /// Contractor company name
    pub contractor_name: String,
    /// Client company name
    pub client_name: String,
    /// Number of workers
    pub num_workers: u32,
}

/// Types of work that can be outsourced
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutsourcingWorkType {
    /// Supporting activities (kegiatan penunjang) - always allowed
    SupportingActivities,
    /// Core activities (kegiatan inti) - now allowed under Omnibus Law with conditions
    CoreActivities,
    /// Seasonal work
    SeasonalWork,
    /// New product/service activities
    NewProductService,
    /// Uncertain/intermittent products
    UncertainProducts,
}

impl OutsourcingWorkType {
    /// Check if outsourcing is permitted under Omnibus Law
    pub fn is_permitted_under_omnibus(&self) -> bool {
        // Under Omnibus Law, most types are now permitted with proper conditions
        true
    }

    /// Get work type description in Indonesian
    pub fn description_id(&self) -> &str {
        match self {
            Self::SupportingActivities => "Kegiatan Penunjang",
            Self::CoreActivities => "Kegiatan Inti (dengan syarat)",
            Self::SeasonalWork => "Pekerjaan Musiman",
            Self::NewProductService => "Kegiatan Produk/Jasa Baru",
            Self::UncertainProducts => "Produk yang Belum Pasti Pemasarannya",
        }
    }

    /// Get work type description in English
    pub fn description_en(&self) -> &str {
        match self {
            Self::SupportingActivities => "Supporting Activities",
            Self::CoreActivities => "Core Activities (with conditions)",
            Self::SeasonalWork => "Seasonal Work",
            Self::NewProductService => "New Product/Service Activities",
            Self::UncertainProducts => "Products with Uncertain Market",
        }
    }
}

/// Working hours flexibility under Omnibus Law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmnibusWorkingHours {
    /// Standard hours per day
    pub hours_per_day: u32,
    /// Standard days per week
    pub days_per_week: u32,
    /// Whether compressed work week is used
    pub is_compressed_week: bool,
}

impl OmnibusWorkingHours {
    /// Standard 5-day work week (8 hours/day, 40 hours/week)
    pub fn standard_5_day() -> Self {
        Self {
            hours_per_day: 8,
            days_per_week: 5,
            is_compressed_week: false,
        }
    }

    /// Compressed 4-day work week (10 hours/day, 40 hours/week) - allowed under Omnibus
    pub fn compressed_4_day() -> Self {
        Self {
            hours_per_day: 10,
            days_per_week: 4,
            is_compressed_week: true,
        }
    }

    /// Calculate total weekly hours
    pub fn total_weekly_hours(&self) -> u32 {
        self.hours_per_day * self.days_per_week
    }

    /// Check if within legal limits (40 hours/week maximum)
    pub fn is_within_limits(&self) -> bool {
        self.total_weekly_hours() <= 40
    }
}

/// Job Loss Guarantee (JKP - Jaminan Kehilangan Pekerjaan)
/// New social security program under Omnibus Law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobLossGuarantee {
    /// Whether employee is registered for JKP
    pub is_registered: bool,
    /// Monthly salary for JKP calculation
    pub monthly_salary: i64,
    /// Months of unemployment benefit
    pub benefit_months: u32,
}

impl JobLossGuarantee {
    /// Maximum JKP benefit duration (6 months)
    pub fn max_benefit_months() -> u32 {
        6
    }

    /// JKP benefit percentage of salary (45%)
    pub fn benefit_percentage() -> f64 {
        0.45
    }

    /// Calculate monthly JKP benefit
    pub fn calculate_monthly_benefit(&self) -> i64 {
        (self.monthly_salary as f64 * Self::benefit_percentage()).round() as i64
    }

    /// Calculate total JKP benefit
    pub fn calculate_total_benefit(&self) -> i64 {
        let monthly_benefit = self.calculate_monthly_benefit();
        let months = self.benefit_months.min(Self::max_benefit_months());
        monthly_benefit * months as i64
    }
}

/// Minimum wage setting under Omnibus Law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmnibusMinimumWage {
    /// Province name
    pub province: String,
    /// Previous year's minimum wage
    pub previous_ump: i64,
    /// Economic growth rate (%)
    pub economic_growth_percent: f64,
    /// Inflation rate (%)
    pub inflation_percent: f64,
    /// New minimum wage (UMP)
    pub new_ump: i64,
}

impl OmnibusMinimumWage {
    /// Calculate new minimum wage under Omnibus formula
    /// UMP(n) = UMP(n-1) + (UMP(n-1) × (inflation + economic growth))
    pub fn calculate_new_ump(
        previous_ump: i64,
        economic_growth_percent: f64,
        inflation_percent: f64,
    ) -> i64 {
        let growth_rate = (inflation_percent + economic_growth_percent) / 100.0;
        let increase = (previous_ump as f64 * growth_rate).round() as i64;
        previous_ump + increase
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omnibus_contract_type() {
        assert_eq!(OmnibusContractType::max_pkwt_duration_years(), 5);
        assert!(OmnibusContractType::is_valid_pkwt_duration(5));
        assert!(!OmnibusContractType::is_valid_pkwt_duration(6));
    }

    #[test]
    fn test_omnibus_severance_calculation() {
        let severance = OmnibusSeverance {
            years_of_service: 10,
            monthly_salary: 10_000_000,
            termination_reason: OmnibusTerminationReason::Efficiency,
        };

        let (sev, service, comp) = severance.calculate_multiplier();
        assert_eq!(sev, 8); // 10 years service = 8 months severance
        assert_eq!(service, 3); // 10 years = 3 months service appreciation
        assert_eq!(comp, 15); // 15% compensation for efficiency

        let total = severance.calculate_total_severance();
        // (8 + 3) * 10M + 15% of (8+3)*10M = 110M + 16.5M = 126.5M
        assert_eq!(total, 126_500_000);
    }

    #[test]
    fn test_outsourcing_work_type() {
        let core = OutsourcingWorkType::CoreActivities;
        assert!(core.is_permitted_under_omnibus());
        assert_eq!(core.description_id(), "Kegiatan Inti (dengan syarat)");
    }

    #[test]
    fn test_omnibus_working_hours() {
        let standard = OmnibusWorkingHours::standard_5_day();
        assert_eq!(standard.total_weekly_hours(), 40);
        assert!(standard.is_within_limits());

        let compressed = OmnibusWorkingHours::compressed_4_day();
        assert_eq!(compressed.total_weekly_hours(), 40);
        assert!(compressed.is_within_limits());
        assert!(compressed.is_compressed_week);
    }

    #[test]
    fn test_job_loss_guarantee() {
        let jkp = JobLossGuarantee {
            is_registered: true,
            monthly_salary: 10_000_000,
            benefit_months: 6,
        };

        assert_eq!(jkp.calculate_monthly_benefit(), 4_500_000); // 45% of 10M
        assert_eq!(jkp.calculate_total_benefit(), 27_000_000); // 4.5M * 6 months
    }

    #[test]
    fn test_minimum_wage_calculation() {
        let new_ump = OmnibusMinimumWage::calculate_new_ump(
            5_000_000, // Previous UMP
            5.0,       // 5% economic growth
            3.0,       // 3% inflation
        );
        // 5M + (5M * 8%) = 5M + 400k = 5.4M
        assert_eq!(new_ump, 5_400_000);
    }
}
