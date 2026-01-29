//! Investment Reforms under Omnibus Law
//!
//! ## Key Changes
//!
//! - **Positive Investment List (DIL)**: Replaces Negative Investment List (DNI)
//! - **Risk-based licensing (OSS-RBA)**: Online Single Submission - Risk Based Approach
//! - **Sectoral opening**: More sectors open to 100% foreign ownership
//! - **Streamlined approvals**: Reduced bureaucratic requirements

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// OSS risk level classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OssRiskLevel {
    /// Low risk (Risiko Rendah) - self-certification
    Low,
    /// Medium-low risk (Risiko Menengah Rendah) - standard certificate
    MediumLow,
    /// Medium-high risk (Risiko Menengah Tinggi) - compliance verification
    MediumHigh,
    /// High risk (Risiko Tinggi) - full permit and inspection
    High,
}

impl OssRiskLevel {
    /// Get risk level name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Low => "Risiko Rendah",
            Self::MediumLow => "Risiko Menengah Rendah",
            Self::MediumHigh => "Risiko Menengah Tinggi",
            Self::High => "Risiko Tinggi",
        }
    }

    /// Get risk level name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Low => "Low Risk",
            Self::MediumLow => "Medium-Low Risk",
            Self::MediumHigh => "Medium-High Risk",
            Self::High => "High Risk",
        }
    }

    /// Check if business activity inspection is required
    pub fn requires_inspection(&self) -> bool {
        matches!(self, Self::MediumHigh | Self::High)
    }

    /// Check if prior licensing is required (vs self-certification)
    pub fn requires_prior_license(&self) -> bool {
        matches!(self, Self::High)
    }
}

/// License category under OSS
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LicenseCategory {
    /// NIB (Nomor Induk Berusaha) - Business Identification Number
    Nib,
    /// Certificate of standards (Sertifikat Standar)
    StandardCertificate,
    /// Business license (Izin Usaha)
    BusinessLicense,
    /// Commercial/operational license (Izin Komersial/Operasional)
    OperationalLicense,
}

impl LicenseCategory {
    /// Get license category name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Nib => "Nomor Induk Berusaha (NIB)",
            Self::StandardCertificate => "Sertifikat Standar",
            Self::BusinessLicense => "Izin Usaha",
            Self::OperationalLicense => "Izin Komersial/Operasional",
        }
    }

    /// Get license category name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Nib => "Business Identification Number (NIB)",
            Self::StandardCertificate => "Certificate of Standards",
            Self::BusinessLicense => "Business License",
            Self::OperationalLicense => "Operational License",
        }
    }
}

/// Risk-based license record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskBasedLicense {
    /// NIB (Business Identification Number)
    pub nib: String,
    /// Company name
    pub company_name: String,
    /// Business activity (KBLI code)
    pub kbli_code: String,
    /// Risk level
    pub risk_level: OssRiskLevel,
    /// License categories obtained
    pub licenses: Vec<LicenseCategory>,
    /// Issue date
    pub issue_date: NaiveDate,
    /// Whether operational (has completed all requirements)
    pub is_operational: bool,
}

impl RiskBasedLicense {
    /// Check if business can commence operations
    pub fn can_commence_operations(&self) -> bool {
        // Low risk can start with NIB only
        // Others need operational license
        match self.risk_level {
            OssRiskLevel::Low => self.licenses.contains(&LicenseCategory::Nib),
            _ => {
                self.licenses.contains(&LicenseCategory::OperationalLicense) && self.is_operational
            }
        }
    }
}

/// Sectors with simplified investment procedures under Omnibus Law
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SimplifiedSectorEligibility {
    /// Labor-intensive industries
    LaborIntensive,
    /// Export-oriented industries
    ExportOriented,
    /// High-tech/innovation industries (R&D-intensive)
    HighTech,
    /// Pioneer industries (newly developed sectors)
    Pioneer,
    /// Economic zone/special economic zone (KEK/SEZ)
    EconomicZone,
    /// Digital economy
    DigitalEconomy,
    /// Green economy/renewable energy
    GreenEconomy,
}

impl SimplifiedSectorEligibility {
    /// Get sector name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::LaborIntensive => "Industri Padat Karya",
            Self::ExportOriented => "Industri Berorientasi Ekspor",
            Self::HighTech => "Industri Teknologi Tinggi/Inovasi",
            Self::Pioneer => "Industri Pionir",
            Self::EconomicZone => "Kawasan Ekonomi Khusus (KEK)",
            Self::DigitalEconomy => "Ekonomi Digital",
            Self::GreenEconomy => "Ekonomi Hijau/Energi Terbarukan",
        }
    }

    /// Get sector name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::LaborIntensive => "Labor-Intensive Industries",
            Self::ExportOriented => "Export-Oriented Industries",
            Self::HighTech => "High-Tech/Innovation Industries",
            Self::Pioneer => "Pioneer Industries",
            Self::EconomicZone => "Economic Zone/Special Economic Zone",
            Self::DigitalEconomy => "Digital Economy",
            Self::GreenEconomy => "Green Economy/Renewable Energy",
        }
    }
}

/// Investment incentive types under Omnibus Law
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvestmentIncentive {
    /// Tax holiday (full tax exemption for period)
    TaxHoliday { duration_years: u32 },
    /// Tax allowance (investment tax reduction)
    TaxAllowance { percentage: u8 },
    /// Import duty facility
    ImportDutyFacility,
    /// VAT exemption on capital goods
    VatExemption,
    /// Accelerated depreciation
    AcceleratedDepreciation,
    /// Super deduction for R&D
    SuperDeductionRnd { multiplier: u8 },
    /// Super deduction for vocational training
    SuperDeductionVocational { multiplier: u8 },
    /// Land facility (discounted land rights)
    LandFacility,
}

impl InvestmentIncentive {
    /// Get incentive description in Indonesian
    pub fn description_id(&self) -> &str {
        match self {
            Self::TaxHoliday { .. } => "Tax Holiday",
            Self::TaxAllowance { .. } => "Tax Allowance",
            Self::ImportDutyFacility => "Fasilitas Bea Masuk",
            Self::VatExemption => "Pembebasan PPN Barang Modal",
            Self::AcceleratedDepreciation => "Penyusutan Dipercepat",
            Self::SuperDeductionRnd { .. } => "Super Deduction R&D",
            Self::SuperDeductionVocational { .. } => "Super Deduction Pelatihan Vokasi",
            Self::LandFacility => "Fasilitas Pertanahan",
        }
    }

    /// Get incentive description in English
    pub fn description_en(&self) -> &str {
        match self {
            Self::TaxHoliday { .. } => "Tax Holiday",
            Self::TaxAllowance { .. } => "Tax Allowance",
            Self::ImportDutyFacility => "Import Duty Facility",
            Self::VatExemption => "VAT Exemption on Capital Goods",
            Self::AcceleratedDepreciation => "Accelerated Depreciation",
            Self::SuperDeductionRnd { .. } => "Super Deduction for R&D",
            Self::SuperDeductionVocational { .. } => "Super Deduction for Vocational Training",
            Self::LandFacility => "Land Facility",
        }
    }
}

/// Foreign ownership limit changes under Omnibus Law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipLiberalization {
    /// Sector name
    pub sector_name: String,
    /// KBLI code
    pub kbli_code: String,
    /// Previous foreign ownership limit (%)
    pub previous_limit_percent: u8,
    /// New foreign ownership limit under Omnibus Law (%)
    pub new_limit_percent: u8,
    /// Whether sector is now fully open (100%)
    pub is_fully_open: bool,
    /// Conditions (if any)
    pub conditions: Vec<String>,
}

impl OwnershipLiberalization {
    /// Calculate percentage increase in foreign ownership allowed
    pub fn percentage_increase(&self) -> i16 {
        self.new_limit_percent as i16 - self.previous_limit_percent as i16
    }

    /// Check if sector was closed and is now open
    pub fn was_closed_now_open(&self) -> bool {
        self.previous_limit_percent == 0 && self.new_limit_percent > 0
    }
}

/// Land rights extension under Omnibus Law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandRightsExtension {
    /// Type of land right
    pub right_type: String,
    /// Previous maximum duration (years)
    pub previous_max_years: u32,
    /// New maximum duration under Omnibus Law (years)
    pub new_max_years: u32,
    /// Asset type (e.g., apartment, office building)
    pub asset_type: String,
}

impl LandRightsExtension {
    /// Example: HGB for apartments extended from 30 to 80 years
    pub fn apartment_hgb_extension() -> Self {
        Self {
            right_type: "Hak Guna Bangunan (HGB)".to_string(),
            previous_max_years: 30,
            new_max_years: 80,
            asset_type: "Satuan Rumah Susun (Apartment Unit)".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oss_risk_level() {
        let low = OssRiskLevel::Low;
        assert!(!low.requires_inspection());
        assert!(!low.requires_prior_license());

        let high = OssRiskLevel::High;
        assert!(high.requires_inspection());
        assert!(high.requires_prior_license());
    }

    #[test]
    fn test_risk_based_license() {
        let license_low = RiskBasedLicense {
            nib: "1234567890123".to_string(),
            company_name: "Test Company".to_string(),
            kbli_code: "12345".to_string(),
            risk_level: OssRiskLevel::Low,
            licenses: vec![LicenseCategory::Nib],
            issue_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date"),
            is_operational: false,
        };
        assert!(license_low.can_commence_operations());

        let license_high = RiskBasedLicense {
            nib: "1234567890124".to_string(),
            company_name: "Test Company 2".to_string(),
            kbli_code: "54321".to_string(),
            risk_level: OssRiskLevel::High,
            licenses: vec![LicenseCategory::Nib, LicenseCategory::BusinessLicense],
            issue_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date"),
            is_operational: false,
        };
        assert!(!license_high.can_commence_operations()); // Missing operational license
    }

    #[test]
    fn test_ownership_liberalization() {
        let lib = OwnershipLiberalization {
            sector_name: "Telecommunications".to_string(),
            kbli_code: "61100".to_string(),
            previous_limit_percent: 49,
            new_limit_percent: 67,
            is_fully_open: false,
            conditions: vec![],
        };

        assert_eq!(lib.percentage_increase(), 18);
        assert!(!lib.was_closed_now_open());

        let lib2 = OwnershipLiberalization {
            sector_name: "Retail".to_string(),
            kbli_code: "47110".to_string(),
            previous_limit_percent: 0,
            new_limit_percent: 100,
            is_fully_open: true,
            conditions: vec![],
        };
        assert!(lib2.was_closed_now_open());
    }

    #[test]
    fn test_land_rights_extension() {
        let extension = LandRightsExtension::apartment_hgb_extension();
        assert_eq!(extension.previous_max_years, 30);
        assert_eq!(extension.new_max_years, 80);
    }

    #[test]
    fn test_simplified_sector() {
        let labor_intensive = SimplifiedSectorEligibility::LaborIntensive;
        assert_eq!(labor_intensive.name_id(), "Industri Padat Karya");
        assert_eq!(labor_intensive.name_en(), "Labor-Intensive Industries");
    }
}
