//! Indonesian Agrarian Law (Land Law) - UU No. 5/1960 (UUPA)
//!
//! ## Overview
//!
//! Basic Agrarian Law (Undang-Undang Pokok Agraria - UUPA) regulates:
//! - Land rights in Indonesia
//! - Land registration
//! - Land use and ownership
//!
//! ## National Land Agency
//!
//! - **Badan Pertanahan Nasional (BPN)**: Now Ministry of Agrarian Affairs and Spatial Planning/BPN
//! - **Kementerian Agraria dan Tata Ruang/BPN (ATR/BPN)**
//!
//! ## Key Principles
//!
//! - All land in Indonesia is under state control
//! - Only Indonesian citizens and certain legal entities can own land (Hak Milik)
//! - Foreigners can only hold land use rights (Hak Pakai, Hak Guna Bangunan)

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Land rights type - Pasal 16 UUPA
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LandRightType {
    /// Freehold title (Hak Milik) - strongest right, indefinite duration
    HakMilik,
    /// Right to cultivate (Hak Guna Usaha - HGU) - max 35 years + extensions
    HakGunaUsaha { duration_years: u32 },
    /// Building use right (Hak Guna Bangunan - HGB) - max 30 years + extensions
    HakGunaBangunan { duration_years: u32 },
    /// Right to use (Hak Pakai) - varying duration
    HakPakai { duration_years: Option<u32> },
    /// Lease (Sewa) - rental agreement
    Sewa { duration_years: u32 },
    /// Opening of land (Hak Membuka Tanah)
    HakMembukaTanah,
    /// Forest collection (Hak Memungut Hasil Hutan)
    HakMemungutHasilHutan,
}

impl LandRightType {
    /// Get land right type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::HakMilik => "Hak Milik",
            Self::HakGunaUsaha { .. } => "Hak Guna Usaha (HGU)",
            Self::HakGunaBangunan { .. } => "Hak Guna Bangunan (HGB)",
            Self::HakPakai { .. } => "Hak Pakai",
            Self::Sewa { .. } => "Sewa",
            Self::HakMembukaTanah => "Hak Membuka Tanah",
            Self::HakMemungutHasilHutan => "Hak Memungut Hasil Hutan",
        }
    }

    /// Get land right type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::HakMilik => "Freehold Title",
            Self::HakGunaUsaha { .. } => "Right to Cultivate (HGU)",
            Self::HakGunaBangunan { .. } => "Building Use Right (HGB)",
            Self::HakPakai { .. } => "Right to Use",
            Self::Sewa { .. } => "Lease",
            Self::HakMembukaTanah => "Right to Open Land",
            Self::HakMemungutHasilHutan => "Forest Collection Right",
        }
    }

    /// Check if right is perpetual (no expiry)
    pub fn is_perpetual(&self) -> bool {
        matches!(self, Self::HakMilik)
    }

    /// Check if foreign entities can hold this right
    pub fn can_foreign_hold(&self) -> bool {
        matches!(
            self,
            Self::HakPakai { .. } | Self::Sewa { .. } | Self::HakGunaBangunan { .. }
        )
    }

    /// Get maximum duration in years
    pub fn max_duration_years(&self) -> Option<u32> {
        match self {
            Self::HakMilik => None,                   // Perpetual
            Self::HakGunaUsaha { .. } => Some(35),    // 35 + 25 extension
            Self::HakGunaBangunan { .. } => Some(30), // 30 + 20 extension
            Self::HakPakai { duration_years } => *duration_years,
            Self::Sewa { duration_years } => Some(*duration_years),
            Self::HakMembukaTanah => None,
            Self::HakMemungutHasilHutan => None,
        }
    }
}

/// Land use purpose for HGU
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HguPurpose {
    /// Plantation (perkebunan)
    Plantation,
    /// Agriculture (pertanian)
    Agriculture,
    /// Animal husbandry (peternakan)
    AnimalHusbandry,
    /// Fisheries (perikanan)
    Fisheries,
}

impl HguPurpose {
    /// Get purpose name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Plantation => "Perkebunan",
            Self::Agriculture => "Pertanian",
            Self::AnimalHusbandry => "Peternakan",
            Self::Fisheries => "Perikanan",
        }
    }

    /// Get purpose name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Plantation => "Plantation",
            Self::Agriculture => "Agriculture",
            Self::AnimalHusbandry => "Animal Husbandry",
            Self::Fisheries => "Fisheries",
        }
    }
}

/// Minimum and maximum land area for HGU
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HguAreaLimits {
    /// Minimum area in hectares
    pub minimum_ha: f64,
    /// Maximum area in hectares (varies by province and crop type)
    pub maximum_ha: Option<f64>,
}

impl HguAreaLimits {
    /// General minimum area for HGU (5 hectares)
    pub fn general_minimum_ha() -> f64 {
        5.0
    }

    /// Example maximum for oil palm in certain provinces (can be up to 100,000 ha)
    pub fn typical_maximum_ha() -> f64 {
        100_000.0
    }
}

/// Land certificate status
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CertificateStatus {
    /// Certified (sertipikat)
    Certified,
    /// Letter C (Surat C) - village land registry
    LetterC,
    /// Girik - old Dutch land registry
    Girik,
    /// Petok D
    PetokD,
    /// Uncertified (belum bersertipikat)
    Uncertified,
}

impl CertificateStatus {
    /// Get status name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Certified => "Bersertipikat",
            Self::LetterC => "Surat C (Letter C)",
            Self::Girik => "Girik",
            Self::PetokD => "Petok D",
            Self::Uncertified => "Belum Bersertipikat",
        }
    }

    /// Get status name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Certified => "Certified",
            Self::LetterC => "Letter C",
            Self::Girik => "Girik",
            Self::PetokD => "Petok D",
            Self::Uncertified => "Uncertified",
        }
    }

    /// Check if status provides strong legal evidence
    pub fn is_strong_evidence(&self) -> bool {
        matches!(self, Self::Certified)
    }
}

/// Land registration record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandRegistration {
    /// Certificate number
    pub certificate_number: Option<String>,
    /// Land right type
    pub right_type: LandRightType,
    /// Certificate status
    pub certificate_status: CertificateStatus,
    /// Land area in square meters
    pub area_sqm: f64,
    /// Land location (kelurahan/desa)
    pub location: String,
    /// Province
    pub province: String,
    /// Owner name
    pub owner_name: String,
    /// Owner citizenship (for Hak Milik eligibility)
    pub owner_is_indonesian: bool,
    /// Issue date
    pub issue_date: Option<NaiveDate>,
    /// Expiry date (if applicable)
    pub expiry_date: Option<NaiveDate>,
    /// Whether land is mortgaged (Hak Tanggungan)
    pub is_mortgaged: bool,
}

impl LandRegistration {
    /// Check if owner can hold this land right type
    pub fn is_valid_ownership(&self) -> bool {
        match self.right_type {
            LandRightType::HakMilik => {
                // Only Indonesian citizens can hold Hak Milik
                self.owner_is_indonesian
            }
            LandRightType::HakGunaUsaha { .. } => {
                // Indonesian citizens or certain legal entities
                true
            }
            _ => true,
        }
    }

    /// Check if certificate is valid on given date
    pub fn is_valid_on(&self, date: NaiveDate) -> bool {
        if let Some(expiry) = self.expiry_date {
            date <= expiry
        } else {
            true // No expiry
        }
    }

    /// Convert area to hectares
    pub fn area_hectares(&self) -> f64 {
        self.area_sqm / 10_000.0
    }
}

/// Land acquisition purpose - for state acquisition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LandAcquisitionPurpose {
    /// Public infrastructure (roads, bridges)
    PublicInfrastructure,
    /// Public facilities (schools, hospitals)
    PublicFacilities,
    /// Public interest projects
    PublicInterest,
    /// Defense and security
    DefenseAndSecurity,
}

impl LandAcquisitionPurpose {
    /// Get purpose name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::PublicInfrastructure => "Infrastruktur Publik",
            Self::PublicFacilities => "Fasilitas Umum",
            Self::PublicInterest => "Kepentingan Umum",
            Self::DefenseAndSecurity => "Pertahanan dan Keamanan",
        }
    }

    /// Get purpose name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::PublicInfrastructure => "Public Infrastructure",
            Self::PublicFacilities => "Public Facilities",
            Self::PublicInterest => "Public Interest",
            Self::DefenseAndSecurity => "Defense and Security",
        }
    }
}

/// Land dispute type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LandDisputeType {
    /// Boundary dispute (sengketa batas)
    BoundaryDispute,
    /// Ownership dispute (sengketa kepemilikan)
    OwnershipDispute,
    /// Double certificate (sertipikat ganda)
    DoubleCertificate,
    /// Land grabbing (perampasan tanah)
    LandGrabbing,
    /// Inheritance dispute (sengketa warisan)
    InheritanceDispute,
    /// Customary land dispute (sengketa tanah adat)
    CustomaryLandDispute,
}

impl LandDisputeType {
    /// Get dispute type description in Indonesian
    pub fn description_id(&self) -> &str {
        match self {
            Self::BoundaryDispute => "Sengketa Batas Tanah",
            Self::OwnershipDispute => "Sengketa Kepemilikan",
            Self::DoubleCertificate => "Sertipikat Ganda",
            Self::LandGrabbing => "Perampasan Tanah",
            Self::InheritanceDispute => "Sengketa Warisan Tanah",
            Self::CustomaryLandDispute => "Sengketa Tanah Adat",
        }
    }

    /// Get dispute type description in English
    pub fn description_en(&self) -> &str {
        match self {
            Self::BoundaryDispute => "Boundary Dispute",
            Self::OwnershipDispute => "Ownership Dispute",
            Self::DoubleCertificate => "Double Certificate",
            Self::LandGrabbing => "Land Grabbing",
            Self::InheritanceDispute => "Inheritance Dispute",
            Self::CustomaryLandDispute => "Customary Land Dispute",
        }
    }
}

/// Mortgage right (Hak Tanggungan) - UU 4/1996
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HakTanggungan {
    /// Certificate number
    pub certificate_number: String,
    /// Debtor name
    pub debtor_name: String,
    /// Creditor name (bank or financial institution)
    pub creditor_name: String,
    /// Debt amount secured
    pub debt_amount: i64,
    /// Registration date
    pub registration_date: NaiveDate,
    /// Whether mortgage is first-ranking (peringkat pertama)
    pub is_first_ranking: bool,
    /// Underlying land right
    pub underlying_land_right: LandRightType,
}

impl HakTanggungan {
    /// Check if mortgage can be created on this land right type
    pub fn can_create_on(right_type: &LandRightType) -> bool {
        matches!(
            right_type,
            LandRightType::HakMilik
                | LandRightType::HakGunaBangunan { .. }
                | LandRightType::HakGunaUsaha { .. }
                | LandRightType::HakPakai { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_land_right_type() {
        let hak_milik = LandRightType::HakMilik;
        assert!(hak_milik.is_perpetual());
        assert!(!hak_milik.can_foreign_hold());
        assert_eq!(hak_milik.name_id(), "Hak Milik");

        let hgb = LandRightType::HakGunaBangunan { duration_years: 30 };
        assert!(!hgb.is_perpetual());
        assert!(hgb.can_foreign_hold());
        assert_eq!(hgb.max_duration_years(), Some(30));
    }

    #[test]
    fn test_hgu_purpose() {
        let plantation = HguPurpose::Plantation;
        assert_eq!(plantation.name_id(), "Perkebunan");
        assert_eq!(plantation.name_en(), "Plantation");
    }

    #[test]
    fn test_certificate_status() {
        let certified = CertificateStatus::Certified;
        assert!(certified.is_strong_evidence());

        let girik = CertificateStatus::Girik;
        assert!(!girik.is_strong_evidence());
    }

    #[test]
    fn test_land_registration_validity() {
        let registration = LandRegistration {
            certificate_number: Some("12345".to_string()),
            right_type: LandRightType::HakMilik,
            certificate_status: CertificateStatus::Certified,
            area_sqm: 100_000.0,
            location: "Jakarta".to_string(),
            province: "DKI Jakarta".to_string(),
            owner_name: "Test Owner".to_string(),
            owner_is_indonesian: true,
            issue_date: Some(NaiveDate::from_ymd_opt(2020, 1, 1).expect("Valid date")),
            expiry_date: None,
            is_mortgaged: false,
        };

        assert!(registration.is_valid_ownership());
        assert_eq!(registration.area_hectares(), 10.0);
    }

    #[test]
    fn test_land_registration_foreign_hak_milik() {
        let registration = LandRegistration {
            certificate_number: Some("12345".to_string()),
            right_type: LandRightType::HakMilik,
            certificate_status: CertificateStatus::Certified,
            area_sqm: 100_000.0,
            location: "Jakarta".to_string(),
            province: "DKI Jakarta".to_string(),
            owner_name: "Foreign Owner".to_string(),
            owner_is_indonesian: false,
            issue_date: Some(NaiveDate::from_ymd_opt(2020, 1, 1).expect("Valid date")),
            expiry_date: None,
            is_mortgaged: false,
        };

        assert!(!registration.is_valid_ownership());
    }

    #[test]
    fn test_hak_tanggungan() {
        let hak_milik = LandRightType::HakMilik;
        assert!(HakTanggungan::can_create_on(&hak_milik));

        let sewa = LandRightType::Sewa { duration_years: 5 };
        assert!(!HakTanggungan::can_create_on(&sewa));
    }

    #[test]
    fn test_hgu_area_limits() {
        assert_eq!(HguAreaLimits::general_minimum_ha(), 5.0);
        assert_eq!(HguAreaLimits::typical_maximum_ha(), 100_000.0);
    }
}
