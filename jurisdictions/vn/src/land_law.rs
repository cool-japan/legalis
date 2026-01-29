//! Vietnamese Land Law 2013 (Luật Đất đai 2013) - Law No. 45/2013/QH13
//!
//! Vietnam's land law, effective from July 1, 2014. Amended by Law 25/2024.
//!
//! ## Key Principles
//!
//! - **State ownership** (Điều 4): All land belongs to the people, State manages
//! - **Land use rights** (Quyền sử dụng đất): Individuals and organizations can use land
//! - **Red Book** (Sổ đỏ): Certificate for residential land use rights
//! - **Pink Book** (Sổ hồng): Certificate for apartment ownership
//!
//! ## Land Use Purposes
//!
//! - Agricultural land (Đất nông nghiệp)
//! - Non-agricultural land (Đất phi nông nghiệp)
//! - Unused land (Đất chưa sử dụng)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Land classification (Phân loại đất) - Article 10
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LandCategory {
    /// Agricultural land (Đất nông nghiệp) - Article 10.1
    Agricultural(AgriculturalLandType),
    /// Non-agricultural land (Đất phi nông nghiệp) - Article 10.2
    NonAgricultural(NonAgriculturalLandType),
    /// Unused land (Đất chưa sử dụng) - Article 10.3
    Unused,
}

impl LandCategory {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> String {
        match self {
            Self::Agricultural(atype) => atype.name_vi(),
            Self::NonAgricultural(ntype) => ntype.name_vi(),
            Self::Unused => "Đất chưa sử dụng".to_string(),
        }
    }

    /// Get English name
    pub fn name_en(&self) -> String {
        match self {
            Self::Agricultural(atype) => atype.name_en(),
            Self::NonAgricultural(ntype) => ntype.name_en(),
            Self::Unused => "Unused land".to_string(),
        }
    }
}

/// Agricultural land types (Đất nông nghiệp) - Article 11-18
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgriculturalLandType {
    /// Rice cultivation (Đất trồng lúa)
    RiceCultivation,
    /// Annual crop cultivation (Đất trồng cây hàng năm khác)
    AnnualCrops,
    /// Perennial crop cultivation (Đất trồng cây lâu năm)
    PerennialCrops,
    /// Forestry (Đất rừng)
    Forestry,
    /// Aquaculture (Đất nuôi trồng thủy sản)
    Aquaculture,
    /// Salt production (Đất làm muối)
    SaltProduction,
    /// Other agricultural (Đất nông nghiệp khác)
    Other,
}

impl AgriculturalLandType {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> String {
        match self {
            Self::RiceCultivation => "Đất trồng lúa".to_string(),
            Self::AnnualCrops => "Đất trồng cây hàng năm khác".to_string(),
            Self::PerennialCrops => "Đất trồng cây lâu năm".to_string(),
            Self::Forestry => "Đất rừng sản xuất".to_string(),
            Self::Aquaculture => "Đất nuôi trồng thủy sản".to_string(),
            Self::SaltProduction => "Đất làm muối".to_string(),
            Self::Other => "Đất nông nghiệp khác".to_string(),
        }
    }

    /// Get English name
    pub fn name_en(&self) -> String {
        match self {
            Self::RiceCultivation => "Rice cultivation land".to_string(),
            Self::AnnualCrops => "Annual crop cultivation land".to_string(),
            Self::PerennialCrops => "Perennial crop cultivation land".to_string(),
            Self::Forestry => "Forestry land".to_string(),
            Self::Aquaculture => "Aquaculture land".to_string(),
            Self::SaltProduction => "Salt production land".to_string(),
            Self::Other => "Other agricultural land".to_string(),
        }
    }
}

/// Non-agricultural land types (Đất phi nông nghiệp) - Article 19-29
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NonAgriculturalLandType {
    /// Residential (Đất ở)
    Residential,
    /// Commercial (Đất thương mại, dịch vụ)
    Commercial,
    /// Industrial (Đất sản xuất, kinh doanh phi nông nghiệp)
    Industrial,
    /// Public facilities (Đất công trình công cộng)
    PublicFacilities,
    /// Defense and security (Đất quốc phòng, an ninh)
    DefenseSecurity,
    /// Religious (Đất tín ngưỡng, tôn giáo)
    Religious,
    /// Cemetery (Đất nghĩa trang, nghĩa địa)
    Cemetery,
    /// Rivers and water bodies (Đất sông, ngòi, kênh, rạch, suối)
    WaterBodies,
    /// Transportation (Đất giao thông)
    Transportation,
    /// Other non-agricultural (Đất phi nông nghiệp khác)
    Other,
}

impl NonAgriculturalLandType {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> String {
        match self {
            Self::Residential => "Đất ở".to_string(),
            Self::Commercial => "Đất thương mại, dịch vụ".to_string(),
            Self::Industrial => "Đất sản xuất, kinh doanh phi nông nghiệp".to_string(),
            Self::PublicFacilities => "Đất công trình công cộng".to_string(),
            Self::DefenseSecurity => "Đất quốc phòng, an ninh".to_string(),
            Self::Religious => "Đất tín ngưỡng, tôn giáo".to_string(),
            Self::Cemetery => "Đất nghĩa trang, nghĩa địa".to_string(),
            Self::WaterBodies => "Đất sông, ngòi, kênh, rạch".to_string(),
            Self::Transportation => "Đất giao thông".to_string(),
            Self::Other => "Đất phi nông nghiệp khác".to_string(),
        }
    }

    /// Get English name
    pub fn name_en(&self) -> String {
        match self {
            Self::Residential => "Residential land".to_string(),
            Self::Commercial => "Commercial and service land".to_string(),
            Self::Industrial => "Industrial and business land".to_string(),
            Self::PublicFacilities => "Public facility land".to_string(),
            Self::DefenseSecurity => "Defense and security land".to_string(),
            Self::Religious => "Religious land".to_string(),
            Self::Cemetery => "Cemetery land".to_string(),
            Self::WaterBodies => "Rivers and water bodies".to_string(),
            Self::Transportation => "Transportation land".to_string(),
            Self::Other => "Other non-agricultural land".to_string(),
        }
    }

    /// Check if transferable to foreigners
    pub fn foreigners_can_own(&self) -> bool {
        matches!(self, Self::Residential) // Only residential under certain conditions
    }
}

/// Land use certificate types (Giấy chứng nhận quyền sử dụng đất)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LandCertificateType {
    /// Red Book (Sổ đỏ) - residential land use right certificate
    RedBook,
    /// Pink Book (Sổ hồng) - apartment/condo ownership certificate
    PinkBook,
    /// General land use right certificate
    GeneralCertificate,
}

impl LandCertificateType {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> &'static str {
        match self {
            Self::RedBook => "Sổ đỏ (Giấy chứng nhận quyền sử dụng đất ở)",
            Self::PinkBook => "Sổ hồng (Giấy chứng nhận quyền sở hữu căn hộ)",
            Self::GeneralCertificate => "Giấy chứng nhận quyền sử dụng đất",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::RedBook => "Red Book (Residential land use right certificate)",
            Self::PinkBook => "Pink Book (Apartment ownership certificate)",
            Self::GeneralCertificate => "Land use right certificate",
        }
    }
}

/// Land use duration types (Thời hạn sử dụng đất) - Article 126-128
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandUseDuration {
    /// Stable and long-term use (Sử dụng ổn định lâu dài) - Residential
    StableLongTerm,
    /// Fixed-term use (Sử dụng có thời hạn)
    FixedTerm {
        /// Years of use right
        years: u16,
    },
}

impl LandUseDuration {
    /// Maximum fixed term for different land types
    pub fn max_term_years(category: &LandCategory) -> u16 {
        match category {
            LandCategory::Agricultural(_) => 50,
            LandCategory::NonAgricultural(NonAgriculturalLandType::Residential) => 70,
            LandCategory::NonAgricultural(_) => 50,
            LandCategory::Unused => 0,
        }
    }

    /// Get Vietnamese description
    pub fn description_vi(&self) -> String {
        match self {
            Self::StableLongTerm => "Sử dụng ổn định lâu dài".to_string(),
            Self::FixedTerm { years } => format!("Sử dụng có thời hạn {} năm", years),
        }
    }
}

/// Land use rights (Quyền của người sử dụng đất) - Article 166-175
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LandUseRight {
    /// Transfer (Chuyển nhượng)
    Transfer,
    /// Lease/Sublease (Cho thuê, cho thuê lại)
    Lease,
    /// Mortgage (Thế chấp)
    Mortgage,
    /// Capital contribution (Góp vốn)
    CapitalContribution,
    /// Inheritance (Thừa kế)
    Inheritance,
    /// Gift (Tặng cho)
    Gift,
}

impl LandUseRight {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> &'static str {
        match self {
            Self::Transfer => "Chuyển nhượng quyền sử dụng đất",
            Self::Lease => "Cho thuê quyền sử dụng đất",
            Self::Mortgage => "Thế chấp quyền sử dụng đất",
            Self::CapitalContribution => "Góp vốn bằng quyền sử dụng đất",
            Self::Inheritance => "Thừa kế quyền sử dụng đất",
            Self::Gift => "Tặng cho quyền sử dụng đất",
        }
    }

    /// Check if requires land use fee payment
    pub fn requires_fee_payment(&self) -> bool {
        matches!(self, Self::Transfer | Self::CapitalContribution)
    }
}

/// Land compensation (Bồi thường khi Nhà nước thu hồi đất) - Article 74-91
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandCompensation {
    /// Land area being recovered (m²)
    pub land_area_sqm: f64,
    /// Land price per m² (VND)
    pub land_price_per_sqm: i64,
    /// Property/building value on land (VND)
    pub property_value: i64,
    /// Relocation support (VND)
    pub relocation_support: i64,
}

impl LandCompensation {
    /// Calculate total compensation (Tổng tiền bồi thường)
    pub fn calculate_total_compensation(&self) -> i64 {
        let land_compensation =
            (self.land_area_sqm * f64::from(self.land_price_per_sqm as i32)) as i64;
        land_compensation + self.property_value + self.relocation_support
    }
}

/// Result type for land law operations
pub type LandResult<T> = Result<T, LandError>;

/// Errors related to Land Law
#[derive(Debug, Error)]
pub enum LandError {
    /// Invalid land use purpose
    #[error("Sử dụng đất sai mục đích (Điều 57): {reason}")]
    InvalidPurpose { reason: String },

    /// Exceeded land use term
    #[error("Vượt quá thời hạn sử dụng đất (Điều 126): {years} năm")]
    ExceededTerm { years: u16 },

    /// Unauthorized land transfer
    #[error("Chuyển nhượng đất không được phép (Điều 186): {reason}")]
    UnauthorizedTransfer { reason: String },

    /// Missing land use certificate
    #[error("Chưa có giấy chứng nhận quyền sử dụng đất (Điều 100)")]
    MissingCertificate,

    /// Other land law violation
    #[error("Vi phạm Luật Đất đai: {reason}")]
    LandViolation { reason: String },
}

/// Validate land use duration
pub fn validate_land_use_duration(
    category: &LandCategory,
    duration: &LandUseDuration,
) -> LandResult<()> {
    if let LandUseDuration::FixedTerm { years } = duration {
        let max_years = LandUseDuration::max_term_years(category);
        if *years > max_years {
            return Err(LandError::ExceededTerm { years: *years });
        }
    }
    Ok(())
}

/// Check if foreigners can acquire land use rights
pub fn can_foreigners_acquire(
    land_type: &NonAgriculturalLandType,
    is_married_to_vietnamese: bool,
    has_investment_certificate: bool,
) -> bool {
    match land_type {
        NonAgriculturalLandType::Residential => {
            // Foreigners can own residential land if:
            // 1. Married to Vietnamese citizen, or
            // 2. Have valid investment certificate
            is_married_to_vietnamese || has_investment_certificate
        }
        _ => false, // Foreigners generally cannot own other land types
    }
}

/// Get Land Law checklist
pub fn get_land_law_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Giấy chứng nhận quyền sử dụng đất",
            "Land use right certificate",
            "Điều 100-105",
        ),
        ("Mục đích sử dụng đất", "Land use purpose", "Điều 57"),
        ("Thời hạn sử dụng đất", "Land use duration", "Điều 126-128"),
        (
            "Chuyển nhượng quyền sử dụng đất",
            "Transfer of land use rights",
            "Điều 186-191",
        ),
        (
            "Thế chấp quyền sử dụng đất",
            "Mortgage of land use rights",
            "Điều 192-195",
        ),
        (
            "Bồi thường khi thu hồi đất",
            "Compensation for land recovery",
            "Điều 74-91",
        ),
        (
            "Nghĩa vụ tài chính về đất đai",
            "Financial obligations on land",
            "Điều 54-56",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_land_categories() {
        let residential = LandCategory::NonAgricultural(NonAgriculturalLandType::Residential);
        assert!(residential.name_vi().contains("Đất ở"));

        let rice = LandCategory::Agricultural(AgriculturalLandType::RiceCultivation);
        assert!(rice.name_vi().contains("lúa"));
    }

    #[test]
    fn test_land_use_duration() {
        let stable = LandUseDuration::StableLongTerm;
        assert!(stable.description_vi().contains("ổn định lâu dài"));

        let fixed = LandUseDuration::FixedTerm { years: 50 };
        assert!(fixed.description_vi().contains("50 năm"));
    }

    #[test]
    fn test_max_term_years() {
        let residential = LandCategory::NonAgricultural(NonAgriculturalLandType::Residential);
        assert_eq!(LandUseDuration::max_term_years(&residential), 70);

        let commercial = LandCategory::NonAgricultural(NonAgriculturalLandType::Commercial);
        assert_eq!(LandUseDuration::max_term_years(&commercial), 50);

        let agricultural = LandCategory::Agricultural(AgriculturalLandType::RiceCultivation);
        assert_eq!(LandUseDuration::max_term_years(&agricultural), 50);
    }

    #[test]
    fn test_validate_duration() {
        let residential = LandCategory::NonAgricultural(NonAgriculturalLandType::Residential);

        // Valid duration
        let valid = LandUseDuration::FixedTerm { years: 50 };
        assert!(validate_land_use_duration(&residential, &valid).is_ok());

        // Exceeded duration
        let invalid = LandUseDuration::FixedTerm { years: 100 };
        assert!(validate_land_use_duration(&residential, &invalid).is_err());
    }

    #[test]
    fn test_foreigners_acquisition() {
        let residential = NonAgriculturalLandType::Residential;

        // Married to Vietnamese
        assert!(can_foreigners_acquire(&residential, true, false));

        // Has investment certificate
        assert!(can_foreigners_acquire(&residential, false, true));

        // Neither condition
        assert!(!can_foreigners_acquire(&residential, false, false));

        // Non-residential land
        let commercial = NonAgriculturalLandType::Commercial;
        assert!(!can_foreigners_acquire(&commercial, true, true));
    }

    #[test]
    fn test_land_compensation() {
        let compensation = LandCompensation {
            land_area_sqm: 100.0,
            land_price_per_sqm: 50_000_000,  // 50M VND/m²
            property_value: 500_000_000,     // 500M VND for building
            relocation_support: 100_000_000, // 100M VND relocation
        };

        let total = compensation.calculate_total_compensation();
        // 100m² * 50M + 500M + 100M = 5,000M + 500M + 100M = 5,600M
        assert_eq!(total, 5_600_000_000);
    }

    #[test]
    fn test_certificate_types() {
        let red_book = LandCertificateType::RedBook;
        assert!(red_book.name_vi().contains("Sổ đỏ"));

        let pink_book = LandCertificateType::PinkBook;
        assert!(pink_book.name_vi().contains("Sổ hồng"));
    }

    #[test]
    fn test_land_use_rights() {
        assert!(LandUseRight::Transfer.requires_fee_payment());
        assert!(LandUseRight::CapitalContribution.requires_fee_payment());
        assert!(!LandUseRight::Lease.requires_fee_payment());
        assert!(!LandUseRight::Inheritance.requires_fee_payment());
    }

    #[test]
    fn test_foreigners_ownership() {
        assert!(NonAgriculturalLandType::Residential.foreigners_can_own());
        assert!(!NonAgriculturalLandType::Commercial.foreigners_can_own());
        assert!(!NonAgriculturalLandType::Industrial.foreigners_can_own());
    }

    #[test]
    fn test_land_law_checklist() {
        let checklist = get_land_law_checklist();
        assert!(!checklist.is_empty());
        assert_eq!(checklist.len(), 7);
    }
}
