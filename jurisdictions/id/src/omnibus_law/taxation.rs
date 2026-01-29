//! Tax Incentives and Changes under Omnibus Law
//!
//! ## Key Tax Provisions
//!
//! - **Super deduction**: 200%-300% tax deduction for R&D and vocational training
//! - **Tax holiday extension**: Up to 20-25 years for pioneer industries
//! - **Tax allowance enhancements**: Additional benefits for qualifying investments
//! - **Accelerated depreciation**: Faster write-offs for certain assets
//!
//! ## Objectives
//!
//! - Attract investment in strategic sectors
//! - Encourage R&D and innovation
//! - Develop skilled workforce through vocational training
//! - Support labor-intensive and export-oriented industries

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type of tax incentive under Omnibus Law
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaxIncentiveType {
    /// Super deduction for R&D
    SuperDeductionRnd,
    /// Super deduction for vocational training
    SuperDeductionVocational,
    /// Tax holiday
    TaxHoliday,
    /// Tax allowance
    TaxAllowance,
    /// Accelerated depreciation
    AcceleratedDepreciation,
    /// Investment allowance
    InvestmentAllowance,
}

impl TaxIncentiveType {
    /// Get incentive type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::SuperDeductionRnd => "Super Deduction Penelitian dan Pengembangan (R&D)",
            Self::SuperDeductionVocational => "Super Deduction Pelatihan Vokasi",
            Self::TaxHoliday => "Tax Holiday",
            Self::TaxAllowance => "Tax Allowance",
            Self::AcceleratedDepreciation => "Penyusutan Dipercepat",
            Self::InvestmentAllowance => "Investment Allowance",
        }
    }

    /// Get incentive type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::SuperDeductionRnd => "Super Deduction for R&D",
            Self::SuperDeductionVocational => "Super Deduction for Vocational Training",
            Self::TaxHoliday => "Tax Holiday",
            Self::TaxAllowance => "Tax Allowance",
            Self::AcceleratedDepreciation => "Accelerated Depreciation",
            Self::InvestmentAllowance => "Investment Allowance",
        }
    }
}

/// Super deduction incentive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperDeduction {
    /// Type (R&D or Vocational)
    pub deduction_type: SuperDeductionType,
    /// Actual expense incurred
    pub actual_expense: i64,
    /// Deduction multiplier (e.g., 300 for 300%)
    pub multiplier_percent: u16,
    /// Tax year
    pub tax_year: u32,
}

/// Type of super deduction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SuperDeductionType {
    /// R&D activities (200%-300% deduction)
    Rnd,
    /// Vocational training (200% deduction)
    VocationalTraining,
}

impl SuperDeduction {
    /// Calculate deductible amount
    pub fn calculate_deduction(&self) -> i64 {
        (self.actual_expense as f64 * self.multiplier_percent as f64 / 100.0).round() as i64
    }

    /// Calculate tax savings (assuming 22% corporate tax rate)
    pub fn calculate_tax_savings(&self, corporate_tax_rate_percent: f64) -> i64 {
        let additional_deduction = self.calculate_deduction() - self.actual_expense;
        (additional_deduction as f64 * corporate_tax_rate_percent / 100.0).round() as i64
    }

    /// R&D super deduction (300%)
    pub fn rnd_300_percent(actual_expense: i64, tax_year: u32) -> Self {
        Self {
            deduction_type: SuperDeductionType::Rnd,
            actual_expense,
            multiplier_percent: 300,
            tax_year,
        }
    }

    /// Vocational training super deduction (200%)
    pub fn vocational_200_percent(actual_expense: i64, tax_year: u32) -> Self {
        Self {
            deduction_type: SuperDeductionType::VocationalTraining,
            actual_expense,
            multiplier_percent: 200,
            tax_year,
        }
    }
}

/// Tax holiday under Omnibus Law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxHoliday {
    /// Company name
    pub company_name: String,
    /// Industry sector
    pub sector: PioneerIndustrySector,
    /// Investment value (Rupiah)
    pub investment_value: i64,
    /// Tax holiday duration (years) - up to 20-25 years
    pub duration_years: u32,
    /// Exemption percentage during holiday period
    pub exemption_percent: u8,
    /// Start date
    pub start_date: NaiveDate,
    /// End date
    pub end_date: NaiveDate,
}

/// Pioneer industry sectors eligible for extended tax holiday
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PioneerIndustrySector {
    /// Upstream metal industry
    UpstreamMetal,
    /// Oil and gas refining
    OilGasRefining,
    /// Petrochemicals derived from oil and gas
    Petrochemicals,
    /// Renewable energy
    RenewableEnergy,
    /// Telecommunications equipment
    TelecommunicationsEquipment,
    /// Main component manufacturing for various industries
    MainComponentManufacturing,
    /// Semiconductor industry
    Semiconductor,
    /// Data centers
    DataCenters,
    /// Electric vehicle manufacturing
    ElectricVehicle,
    /// Battery manufacturing
    BatteryManufacturing,
}

impl PioneerIndustrySector {
    /// Get sector name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::UpstreamMetal => "Industri Logam Hulu",
            Self::OilGasRefining => "Pengilangan Minyak dan Gas Bumi",
            Self::Petrochemicals => "Petrokimia dari Minyak dan Gas Bumi",
            Self::RenewableEnergy => "Energi Terbarukan",
            Self::TelecommunicationsEquipment => "Peralatan Telekomunikasi",
            Self::MainComponentManufacturing => "Manufaktur Komponen Utama",
            Self::Semiconductor => "Industri Semikonduktor",
            Self::DataCenters => "Pusat Data",
            Self::ElectricVehicle => "Manufaktur Kendaraan Listrik",
            Self::BatteryManufacturing => "Manufaktur Baterai",
        }
    }

    /// Get sector name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::UpstreamMetal => "Upstream Metal Industry",
            Self::OilGasRefining => "Oil and Gas Refining",
            Self::Petrochemicals => "Petrochemicals",
            Self::RenewableEnergy => "Renewable Energy",
            Self::TelecommunicationsEquipment => "Telecommunications Equipment",
            Self::MainComponentManufacturing => "Main Component Manufacturing",
            Self::Semiconductor => "Semiconductor Industry",
            Self::DataCenters => "Data Centers",
            Self::ElectricVehicle => "Electric Vehicle Manufacturing",
            Self::BatteryManufacturing => "Battery Manufacturing",
        }
    }

    /// Get maximum tax holiday duration for this sector
    pub fn max_tax_holiday_years(&self) -> u32 {
        match self {
            Self::UpstreamMetal
            | Self::OilGasRefining
            | Self::Petrochemicals
            | Self::Semiconductor
            | Self::DataCenters => 25,
            _ => 20,
        }
    }
}

impl TaxHoliday {
    /// Calculate annual tax savings
    pub fn calculate_annual_tax_savings(&self, annual_taxable_income: i64) -> i64 {
        let standard_tax = (annual_taxable_income as f64 * 0.22).round() as i64; // 22% corporate rate

        (standard_tax as f64 * self.exemption_percent as f64 / 100.0).round() as i64
    }

    /// Calculate total tax savings over holiday period
    pub fn calculate_total_tax_savings(&self, annual_taxable_income: i64) -> i64 {
        let annual_savings = self.calculate_annual_tax_savings(annual_taxable_income);
        annual_savings * self.duration_years as i64
    }
}

/// Tax allowance (investment tax deduction)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxAllowance {
    /// Company name
    pub company_name: String,
    /// Qualifying investment amount
    pub investment_amount: i64,
    /// Investment allowance percentage (30% of investment over 6 years)
    pub allowance_percent: u8,
    /// Distribution period (years) - typically 6 years
    pub distribution_years: u32,
    /// Additional benefits
    pub additional_benefits: Vec<TaxAllowanceBenefit>,
}

/// Additional tax allowance benefits
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaxAllowanceBenefit {
    /// Accelerated depreciation
    AcceleratedDepreciation,
    /// Extended loss carry-forward (5-10 years)
    ExtendedLossCarryForward { years: u32 },
    /// Reduced withholding tax on dividends to non-residents (10% or less)
    ReducedWithholdingTax { rate_percent: u8 },
}

impl TaxAllowance {
    /// Calculate annual tax allowance deduction
    pub fn calculate_annual_deduction(&self) -> i64 {
        let total_allowance =
            (self.investment_amount as f64 * self.allowance_percent as f64 / 100.0).round() as i64;
        total_allowance / self.distribution_years as i64
    }

    /// Calculate annual tax savings (assuming 22% corporate tax rate)
    pub fn calculate_annual_tax_savings(&self) -> i64 {
        let annual_deduction = self.calculate_annual_deduction();
        (annual_deduction as f64 * 0.22).round() as i64
    }

    /// Calculate total tax savings over distribution period
    pub fn calculate_total_tax_savings(&self) -> i64 {
        self.calculate_annual_tax_savings() * self.distribution_years as i64
    }
}

/// Accelerated depreciation under Omnibus Law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceleratedDepreciation {
    /// Asset category
    pub asset_category: AssetCategory,
    /// Asset value
    pub asset_value: i64,
    /// Standard depreciation years
    pub standard_years: u32,
    /// Accelerated depreciation years
    pub accelerated_years: u32,
}

/// Asset categories eligible for accelerated depreciation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetCategory {
    /// Machinery and equipment for labor-intensive industries
    LaborIntensiveMachinery,
    /// Equipment for export-oriented industries
    ExportOrientedEquipment,
    /// R&D facilities and equipment
    RndFacilities,
    /// Environmental protection equipment
    EnvironmentalProtection,
    /// Energy-efficient equipment
    EnergyEfficient,
}

impl AcceleratedDepreciation {
    /// Calculate annual depreciation under standard method
    pub fn standard_annual_depreciation(&self) -> i64 {
        self.asset_value / self.standard_years as i64
    }

    /// Calculate annual depreciation under accelerated method
    pub fn accelerated_annual_depreciation(&self) -> i64 {
        self.asset_value / self.accelerated_years as i64
    }

    /// Calculate additional first-year tax savings from acceleration
    pub fn calculate_first_year_tax_benefit(&self) -> i64 {
        let additional_depreciation =
            self.accelerated_annual_depreciation() - self.standard_annual_depreciation();
        (additional_depreciation as f64 * 0.22).round() as i64 // 22% corporate tax rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_super_deduction_rnd() {
        let super_ded = SuperDeduction::rnd_300_percent(100_000_000, 2024);
        assert_eq!(super_ded.calculate_deduction(), 300_000_000);

        // Tax savings: (300M - 100M) * 22% = 200M * 22% = 44M
        assert_eq!(super_ded.calculate_tax_savings(22.0), 44_000_000);
    }

    #[test]
    fn test_super_deduction_vocational() {
        let super_ded = SuperDeduction::vocational_200_percent(50_000_000, 2024);
        assert_eq!(super_ded.calculate_deduction(), 100_000_000);

        // Tax savings: (100M - 50M) * 22% = 50M * 22% = 11M
        assert_eq!(super_ded.calculate_tax_savings(22.0), 11_000_000);
    }

    #[test]
    fn test_pioneer_industry_sector() {
        let semiconductor = PioneerIndustrySector::Semiconductor;
        assert_eq!(semiconductor.max_tax_holiday_years(), 25);

        let ev = PioneerIndustrySector::ElectricVehicle;
        assert_eq!(ev.max_tax_holiday_years(), 20);
    }

    #[test]
    fn test_tax_holiday_savings() {
        let tax_holiday = TaxHoliday {
            company_name: "Test Company".to_string(),
            sector: PioneerIndustrySector::Semiconductor,
            investment_value: 10_000_000_000,
            duration_years: 20,
            exemption_percent: 100,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date"),
            end_date: NaiveDate::from_ymd_opt(2044, 1, 1).expect("Valid date"),
        };

        let annual_income = 1_000_000_000;
        // Standard tax: 1B * 22% = 220M
        // 100% exemption = 220M savings per year
        assert_eq!(
            tax_holiday.calculate_annual_tax_savings(annual_income),
            220_000_000
        );

        // Total over 20 years = 220M * 20 = 4.4B
        assert_eq!(
            tax_holiday.calculate_total_tax_savings(annual_income),
            4_400_000_000
        );
    }

    #[test]
    fn test_tax_allowance() {
        let allowance = TaxAllowance {
            company_name: "Test Company".to_string(),
            investment_amount: 1_000_000_000,
            allowance_percent: 30,
            distribution_years: 6,
            additional_benefits: vec![],
        };

        // Total allowance: 1B * 30% = 300M
        // Annual deduction: 300M / 6 = 50M
        assert_eq!(allowance.calculate_annual_deduction(), 50_000_000);

        // Annual tax savings: 50M * 22% = 11M
        assert_eq!(allowance.calculate_annual_tax_savings(), 11_000_000);

        // Total over 6 years: 11M * 6 = 66M
        assert_eq!(allowance.calculate_total_tax_savings(), 66_000_000);
    }

    #[test]
    fn test_accelerated_depreciation() {
        let accel_dep = AcceleratedDepreciation {
            asset_category: AssetCategory::LaborIntensiveMachinery,
            asset_value: 100_000_000,
            standard_years: 8,
            accelerated_years: 4,
        };

        assert_eq!(accel_dep.standard_annual_depreciation(), 12_500_000);
        assert_eq!(accel_dep.accelerated_annual_depreciation(), 25_000_000);

        // Additional depreciation: 25M - 12.5M = 12.5M
        // Tax benefit: 12.5M * 22% = 2.75M
        assert_eq!(accel_dep.calculate_first_year_tax_benefit(), 2_750_000);
    }
}
