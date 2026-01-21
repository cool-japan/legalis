//! Types for Indonesian Investment Law

use serde::{Deserialize, Serialize};

/// Investment sector classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvestmentSector {
    /// Priority sectors with tax incentives
    Priority(PrioritySector),
    /// Open sectors (no restrictions)
    Open,
    /// Restricted sectors (ownership limits apply)
    Restricted(RestrictedType),
    /// Closed to foreign investment
    Closed,
}

/// Priority investment sectors (PP 7/2021)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrioritySector {
    /// Manufacturing (industri manufaktur)
    Manufacturing,
    /// Digital economy (ekonomi digital)
    DigitalEconomy,
    /// Healthcare/Pharmaceutical
    Healthcare,
    /// Education
    Education,
    /// Tourism
    Tourism,
    /// Energy/Renewable energy
    Energy,
    /// Research and development
    ResearchDevelopment,
    /// Infrastructure
    Infrastructure,
    /// Agriculture/Food security
    Agriculture,
    /// Fisheries/Marine
    MarineAndFisheries,
    /// Logistics/Transportation
    Logistics,
    /// Defense industry
    Defense,
}

impl PrioritySector {
    /// Get potential tax holiday years
    pub fn potential_tax_holiday_years(&self) -> Option<(u32, u32)> {
        match self {
            Self::Manufacturing => Some((5, 20)),
            Self::DigitalEconomy => Some((5, 15)),
            Self::Healthcare => Some((5, 15)),
            Self::Energy => Some((5, 20)),
            Self::Infrastructure => Some((5, 20)),
            Self::ResearchDevelopment => Some((5, 15)),
            _ => Some((5, 10)),
        }
    }
}

/// Types of restricted investment
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RestrictedType {
    /// MSME reserved (Usaha Mikro, Kecil, Menengah)
    MsmeReserved,
    /// Partnership with MSME required (kemitraan)
    PartnershipRequired,
    /// Foreign ownership capped at percentage
    OwnershipCapped(u32),
    /// Requires special permit (izin khusus)
    SpecialPermit(String),
    /// Only for ASEAN investors
    AseanOnly,
    /// Specific conditions apply
    ConditionalAccess(String),
}

/// Foreign ownership limit for a sector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipLimit {
    /// Sector KBLI code
    pub kbli_code: String,
    /// Sector name
    pub sector_name: String,
    /// Maximum foreign ownership percentage
    pub max_foreign_percentage: u32,
    /// Conditions for ownership
    pub conditions: Vec<String>,
    /// Legal basis
    pub legal_basis: String,
}

/// Business risk classification (OSS-RBA)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BusinessRisk {
    /// Low risk (risiko rendah) - NIB only
    Low,
    /// Medium-Low risk (risiko menengah rendah) - NIB + standard certificate
    MediumLow,
    /// Medium-High risk (risiko menengah tinggi) - NIB + verified certificate
    MediumHigh,
    /// High risk (risiko tinggi) - full permit required
    High,
}

impl BusinessRisk {
    /// Get required license type
    pub fn required_license(&self) -> LicenseType {
        match self {
            Self::Low => LicenseType::NibOnly,
            Self::MediumLow => LicenseType::NibWithStandardCertificate,
            Self::MediumHigh => LicenseType::NibWithVerifiedCertificate,
            Self::High => LicenseType::FullPermit,
        }
    }

    /// Get description in Indonesian
    pub fn description_id(&self) -> &'static str {
        match self {
            Self::Low => "Risiko Rendah - Cukup NIB",
            Self::MediumLow => "Risiko Menengah Rendah - NIB + Sertifikat Standar",
            Self::MediumHigh => "Risiko Menengah Tinggi - NIB + Sertifikat Standar Terverifikasi",
            Self::High => "Risiko Tinggi - Izin Lengkap",
        }
    }
}

/// Business license type under OSS-RBA
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LicenseType {
    /// NIB only (Nomor Induk Berusaha)
    NibOnly,
    /// NIB with standard certificate (sertifikat standar)
    NibWithStandardCertificate,
    /// NIB with verified standard certificate
    NibWithVerifiedCertificate,
    /// Full permit (izin)
    FullPermit,
}

/// Business license
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessLicense {
    /// NIB (Nomor Induk Berusaha)
    pub nib: String,
    /// Company name
    pub company_name: String,
    /// KBLI codes (business classifications)
    pub kbli_codes: Vec<String>,
    /// License type
    pub license_type: LicenseType,
    /// Business risk level
    pub risk_level: BusinessRisk,
    /// Is foreign investment (PMA)
    pub is_pma: bool,
    /// Foreign ownership percentage
    pub foreign_ownership_percent: Option<u32>,
    /// Investment value (Rupiah)
    pub investment_value: i64,
    /// Location (province)
    pub location: String,
    /// Special economic zone (if applicable)
    pub sez_location: Option<String>,
    /// Certificates obtained
    pub certificates: Vec<String>,
}

/// Foreign investment (PMA - Penanaman Modal Asing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignInvestment {
    /// Company name (PT PMA)
    pub company_name: String,
    /// Foreign investor country
    pub investor_country: String,
    /// Investment value (USD)
    pub investment_usd: i64,
    /// Foreign ownership percentage
    pub foreign_ownership_percent: u32,
    /// Local partner (if required)
    pub local_partner: Option<String>,
    /// KBLI business codes
    pub kbli_codes: Vec<String>,
    /// Location
    pub location: String,
    /// In priority sector
    pub is_priority_sector: bool,
    /// In special economic zone
    pub in_sez: bool,
    /// Planned employment (Indonesian workers)
    pub planned_local_employment: u32,
    /// Planned employment (Foreign workers)
    pub planned_foreign_employment: u32,
}

impl ForeignInvestment {
    /// Calculate minimum capital requirement
    /// PP 5/2021: Minimum Rp 10 billion total, Rp 2.5 billion paid-up
    pub fn minimum_capital_idr() -> i64 {
        10_000_000_000 // Rp 10 billion
    }

    /// Calculate minimum paid-up capital
    pub fn minimum_paid_up_idr() -> i64 {
        2_500_000_000 // Rp 2.5 billion
    }

    /// Check if investment meets minimum requirements
    pub fn meets_minimum_capital(&self, exchange_rate: f64) -> bool {
        let investment_idr = (self.investment_usd as f64 * exchange_rate) as i64;
        investment_idr >= Self::minimum_capital_idr()
    }
}

/// Priority investment incentives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityInvestment {
    /// Sector
    pub sector: PrioritySector,
    /// Tax holiday available
    pub tax_holiday_available: bool,
    /// Tax holiday years
    pub tax_holiday_years: Option<u32>,
    /// Investment allowance
    pub investment_allowance: bool,
    /// Accelerated depreciation
    pub accelerated_depreciation: bool,
    /// Import duty exemption
    pub import_duty_exemption: bool,
    /// Super deduction for R&D
    pub super_deduction_rd: bool,
    /// Super deduction for vocational training
    pub super_deduction_vocational: bool,
}

impl PriorityInvestment {
    /// Get incentives for a priority sector
    pub fn for_sector(sector: PrioritySector, investment_value_idr: i64) -> Self {
        let (_, max_years) = sector.potential_tax_holiday_years().unwrap_or((0, 0));

        // Tax holiday based on investment value
        let tax_holiday_years = if investment_value_idr >= 500_000_000_000_000 {
            // >= 500 trillion: 20 years
            Some(20)
        } else if investment_value_idr >= 100_000_000_000_000 {
            // >= 100 trillion: 15 years
            Some(15)
        } else if investment_value_idr >= 1_000_000_000_000 {
            // >= 1 trillion: 10 years
            Some(10.min(max_years))
        } else if investment_value_idr >= 500_000_000_000 {
            // >= 500 billion: 5 years
            Some(5.min(max_years))
        } else {
            None
        };

        Self {
            sector: sector.clone(),
            tax_holiday_available: tax_holiday_years.is_some(),
            tax_holiday_years,
            investment_allowance: true,
            accelerated_depreciation: true,
            import_duty_exemption: matches!(
                sector,
                PrioritySector::Manufacturing
                    | PrioritySector::Energy
                    | PrioritySector::Infrastructure
            ),
            super_deduction_rd: matches!(sector, PrioritySector::ResearchDevelopment),
            super_deduction_vocational: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_business_risk_license() {
        assert_eq!(BusinessRisk::Low.required_license(), LicenseType::NibOnly);
        assert_eq!(
            BusinessRisk::High.required_license(),
            LicenseType::FullPermit
        );
    }

    #[test]
    fn test_foreign_investment_minimum() {
        let investment = ForeignInvestment {
            company_name: "PT Example Indonesia".to_string(),
            investor_country: "Singapore".to_string(),
            investment_usd: 1_000_000,
            foreign_ownership_percent: 100,
            local_partner: None,
            kbli_codes: vec!["62011".to_string()],
            location: "DKI Jakarta".to_string(),
            is_priority_sector: true,
            in_sez: false,
            planned_local_employment: 50,
            planned_foreign_employment: 2,
        };

        // At 15,000 IDR/USD, $1M = 15 billion IDR > 10 billion minimum
        assert!(investment.meets_minimum_capital(15_000.0));

        // At lower exchange or lower investment, might not meet minimum
        assert!(!investment.meets_minimum_capital(5_000.0)); // $1M * 5000 = 5B < 10B
    }

    #[test]
    fn test_priority_sector_tax_holiday() {
        let manufacturing = PrioritySector::Manufacturing;
        let (min, max) = manufacturing.potential_tax_holiday_years().unwrap();
        assert_eq!(min, 5);
        assert_eq!(max, 20);
    }

    #[test]
    fn test_priority_investment_incentives() {
        let incentives = PriorityInvestment::for_sector(
            PrioritySector::Manufacturing,
            1_000_000_000_000, // 1 trillion
        );

        assert!(incentives.tax_holiday_available);
        assert_eq!(incentives.tax_holiday_years, Some(10));
        assert!(incentives.import_duty_exemption);
    }
}
