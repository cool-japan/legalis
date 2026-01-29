//! Vietnamese Intellectual Property Law (Luật Sở hữu trí tuệ)
//!
//! Law on Intellectual Property No. 50/2005/QH11, amended by Laws 36/2009, 42/2019.
//!
//! ## IP Rights Protected
//!
//! - **Copyright** (Quyền tác giả): Literary and artistic works
//! - **Industrial Property Rights** (Quyền sở hữu công nghiệp):
//!   - Patents (Bằng sáng chế)
//!   - Utility solutions (Giải pháp hữu ích)
//!   - Industrial designs (Kiểu dáng công nghiệp)
//!   - Trademarks (Nhãn hiệu)
//!   - Trade names (Tên thương mại)
//!   - Geographical indications (Chỉ dẫn địa lý)
//! - **Plant variety rights** (Quyền đối với giống cây trồng)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Types of intellectual property rights (Quyền sở hữu trí tuệ)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IpRightType {
    /// Copyright (Quyền tác giả) - Part VI
    Copyright,
    /// Patent (Bằng sáng chế) - Part II, Chapter XIV
    Patent,
    /// Utility solution (Giải pháp hữu ích) - Part II, Chapter XV
    UtilitySolution,
    /// Industrial design (Kiểu dáng công nghiệp) - Part II, Chapter XVI
    IndustrialDesign,
    /// Trademark (Nhãn hiệu) - Part II, Chapter XVII
    Trademark,
    /// Trade name (Tên thương mại) - Part II, Chapter XVIII
    TradeName,
    /// Geographical indication (Chỉ dẫn địa lý) - Part II, Chapter XIX
    GeographicalIndication,
    /// Trade secret (Bí mật kinh doanh) - Part II, Chapter XX
    TradeSecret,
    /// Plant variety (Giống cây trồng) - Part IV
    PlantVariety,
}

impl IpRightType {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> &'static str {
        match self {
            Self::Copyright => "Quyền tác giả",
            Self::Patent => "Bằng sáng chế",
            Self::UtilitySolution => "Giải pháp hữu ích",
            Self::IndustrialDesign => "Kiểu dáng công nghiệp",
            Self::Trademark => "Nhãn hiệu",
            Self::TradeName => "Tên thương mại",
            Self::GeographicalIndication => "Chỉ dẫn địa lý",
            Self::TradeSecret => "Bí mật kinh doanh",
            Self::PlantVariety => "Giống cây trồng",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Copyright => "Copyright",
            Self::Patent => "Patent",
            Self::UtilitySolution => "Utility solution",
            Self::IndustrialDesign => "Industrial design",
            Self::Trademark => "Trademark",
            Self::TradeName => "Trade name",
            Self::GeographicalIndication => "Geographical indication",
            Self::TradeSecret => "Trade secret",
            Self::PlantVariety => "Plant variety",
        }
    }

    /// Check if requires registration
    pub fn requires_registration(&self) -> bool {
        matches!(
            self,
            Self::Patent
                | Self::UtilitySolution
                | Self::IndustrialDesign
                | Self::Trademark
                | Self::GeographicalIndication
                | Self::PlantVariety
        )
    }

    /// Get protection duration in years (if fixed)
    pub fn protection_duration_years(&self) -> Option<u16> {
        match self {
            Self::Copyright => Some(50), // Author's life + 50 years (or 50 years from publication)
            Self::Patent => Some(20),    // 20 years from filing date
            Self::UtilitySolution => Some(10), // 10 years from filing date
            Self::IndustrialDesign => Some(5), // 5 years, renewable up to 15 years
            Self::Trademark => Some(10), // 10 years, renewable indefinitely
            Self::PlantVariety => Some(20), // 20 years (25 for trees and vines)
            Self::TradeName | Self::TradeSecret => None, // No fixed duration
            Self::GeographicalIndication => None, // No fixed duration
        }
    }

    /// Check if renewable
    pub fn is_renewable(&self) -> bool {
        matches!(self, Self::Trademark | Self::IndustrialDesign)
    }
}

/// Patent requirements (Điều kiện bảo hộ bằng sáng chế) - Article 58
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatentRequirements;

impl PatentRequirements {
    /// Check if invention meets patentability requirements
    /// - Novelty (Tính mới)
    /// - Inventive step (Trình độ sáng tạo)
    /// - Industrial applicability (Khả năng áp dụng công nghiệp)
    pub fn is_patentable(
        is_novel: bool,
        has_inventive_step: bool,
        is_industrially_applicable: bool,
    ) -> bool {
        is_novel && has_inventive_step && is_industrially_applicable
    }
}

/// Trademark classification (Loại nhãn hiệu) - Article 72
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrademarkType {
    /// Word mark (Nhãn hiệu chữ)
    Word(String),
    /// Figurative mark (Nhãn hiệu hình)
    Figurative,
    /// Combined mark (Nhãn hiệu kết hợp)
    Combined { words: String },
    /// Three-dimensional mark (Nhãn hiệu ba chiều)
    ThreeDimensional,
    /// Sound mark (Nhãn hiệu âm thanh)
    Sound,
    /// Color mark (Nhãn hiệu màu sắc)
    Color,
}

impl TrademarkType {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> String {
        match self {
            Self::Word(text) => format!("Nhãn hiệu chữ: {}", text),
            Self::Figurative => "Nhãn hiệu hình".to_string(),
            Self::Combined { words } => format!("Nhãn hiệu kết hợp: {}", words),
            Self::ThreeDimensional => "Nhãn hiệu ba chiều".to_string(),
            Self::Sound => "Nhãn hiệu âm thanh".to_string(),
            Self::Color => "Nhãn hiệu màu sắc".to_string(),
        }
    }
}

/// Copyright subject matter (Đối tượng quyền tác giả) - Article 14
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CopyrightSubject {
    /// Literary works (Tác phẩm văn học)
    Literary,
    /// Artistic works (Tác phẩm nghệ thuật)
    Artistic,
    /// Musical works (Tác phẩm âm nhạc)
    Musical,
    /// Cinematographic works (Tác phẩm điện ảnh)
    Cinematographic,
    /// Photographic works (Tác phẩm nhiếp ảnh)
    Photographic,
    /// Computer programs (Chương trình máy tính)
    Software,
    /// Databases (Cơ sở dữ liệu)
    Database,
    /// Other works
    Other(String),
}

impl CopyrightSubject {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> String {
        match self {
            Self::Literary => "Tác phẩm văn học, khoa học".to_string(),
            Self::Artistic => "Tác phẩm nghệ thuật".to_string(),
            Self::Musical => "Tác phẩm âm nhạc".to_string(),
            Self::Cinematographic => "Tác phẩm điện ảnh, video".to_string(),
            Self::Photographic => "Tác phẩm nhiếp ảnh".to_string(),
            Self::Software => "Chương trình máy tính".to_string(),
            Self::Database => "Cơ sở dữ liệu".to_string(),
            Self::Other(desc) => desc.clone(),
        }
    }

    /// Check if registration is recommended (not required but helpful)
    pub fn registration_recommended(&self) -> bool {
        matches!(self, Self::Software | Self::Database)
    }
}

/// IP infringement types (Vi phạm quyền sở hữu trí tuệ) - Article 198-202
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IpInfringement {
    /// Counterfeiting (Hàng giả)
    Counterfeiting,
    /// Piracy (Sao chép trái phép)
    Piracy,
    /// Unauthorized use (Sử dụng trái phép)
    UnauthorizedUse,
    /// Importation of infringing goods (Nhập khẩu hàng xâm phạm)
    InfringingImports,
    /// False designation of origin (Gắn sai xuất xứ)
    FalseOrigin,
    /// Trade dress infringement (Xâm phạm hình thức bên ngoài)
    TradeDressInfringement,
    /// Other infringement
    Other(String),
}

impl IpInfringement {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> String {
        match self {
            Self::Counterfeiting => "Sản xuất, buôn bán hàng giả mạo".to_string(),
            Self::Piracy => "Sao chép, phổ biến trái phép".to_string(),
            Self::UnauthorizedUse => "Sử dụng không được phép".to_string(),
            Self::InfringingImports => "Nhập khẩu hàng hóa xâm phạm quyền".to_string(),
            Self::FalseOrigin => "Gắn sai xuất xứ, nguồn gốc".to_string(),
            Self::TradeDressInfringement => "Xâm phạm hình thức bên ngoài sản phẩm".to_string(),
            Self::Other(desc) => desc.clone(),
        }
    }

    /// Check if criminal prosecution possible (Article 224-226)
    pub fn allows_criminal_prosecution(&self) -> bool {
        matches!(
            self,
            Self::Counterfeiting | Self::Piracy | Self::InfringingImports
        )
    }
}

/// IP registration application (Đơn đăng ký bảo hộ)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRegistration {
    /// Type of IP right
    pub ip_type: IpRightType,
    /// Applicant name
    pub applicant: String,
    /// Title/description
    pub title: String,
    /// Filing date
    pub filing_date: String,
    /// Priority claimed (if any)
    pub priority_claim: Option<String>,
}

impl IpRegistration {
    /// Check if registration is required for this IP type
    pub fn registration_required(&self) -> bool {
        self.ip_type.requires_registration()
    }

    /// Get expected protection duration
    pub fn protection_duration(&self) -> Option<u16> {
        self.ip_type.protection_duration_years()
    }
}

/// Result type for IP operations
pub type IpResult<T> = Result<T, IpError>;

/// Errors related to Intellectual Property Law
#[derive(Debug, Error)]
pub enum IpError {
    /// Infringement detected
    #[error("Xâm phạm quyền sở hữu trí tuệ: {infringement_type}")]
    Infringement { infringement_type: String },

    /// Invalid registration
    #[error("Đăng ký không hợp lệ: {reason}")]
    InvalidRegistration { reason: String },

    /// Not patentable
    #[error("Không đủ điều kiện bảo hộ bằng sáng chế (Điều 58): {reason}")]
    NotPatentable { reason: String },

    /// Expired protection
    #[error("Quyền bảo hộ đã hết hạn: {ip_type}")]
    ExpiredProtection { ip_type: String },

    /// Other IP violation
    #[error("Vi phạm Luật Sở hữu trí tuệ: {reason}")]
    IpViolation { reason: String },
}

/// Validate patent requirements
pub fn validate_patent_requirements(
    is_novel: bool,
    has_inventive_step: bool,
    is_industrially_applicable: bool,
) -> IpResult<()> {
    if !is_novel {
        return Err(IpError::NotPatentable {
            reason: "Không có tính mới".to_string(),
        });
    }

    if !has_inventive_step {
        return Err(IpError::NotPatentable {
            reason: "Không có trình độ sáng tạo".to_string(),
        });
    }

    if !is_industrially_applicable {
        return Err(IpError::NotPatentable {
            reason: "Không có khả năng áp dụng công nghiệp".to_string(),
        });
    }

    Ok(())
}

/// Check if IP protection has expired
pub fn check_protection_expired(ip_type: &IpRightType, years_since_grant: u16) -> IpResult<()> {
    if let Some(duration) = ip_type.protection_duration_years()
        && years_since_grant >= duration
    {
        return Err(IpError::ExpiredProtection {
            ip_type: ip_type.name_vi().to_string(),
        });
    }
    Ok(())
}

/// Get IP Law checklist
pub fn get_ip_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Đăng ký quyền sở hữu công nghiệp",
            "Register industrial property rights",
            "Điều 86-108",
        ),
        (
            "Kiểm tra tính mới của phát minh",
            "Check novelty of invention",
            "Điều 59",
        ),
        ("Đăng ký nhãn hiệu", "Trademark registration", "Điều 95-100"),
        ("Bảo vệ quyền tác giả", "Copyright protection", "Điều 19-37"),
        (
            "Duy trì hiệu lực bằng bảo hộ",
            "Maintain protection validity",
            "Điều 93-94",
        ),
        (
            "Xử lý xâm phạm quyền",
            "Handle infringement",
            "Điều 198-226",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_right_types() {
        assert!(IpRightType::Patent.requires_registration());
        assert!(IpRightType::Trademark.requires_registration());
        assert!(!IpRightType::Copyright.requires_registration());
        assert!(!IpRightType::TradeSecret.requires_registration());

        assert_eq!(IpRightType::Patent.protection_duration_years(), Some(20));
        assert_eq!(IpRightType::Trademark.protection_duration_years(), Some(10));
        assert_eq!(IpRightType::Copyright.protection_duration_years(), Some(50));

        assert!(IpRightType::Trademark.is_renewable());
        assert!(!IpRightType::Patent.is_renewable());
    }

    #[test]
    fn test_patent_requirements() {
        // All requirements met
        assert!(PatentRequirements::is_patentable(true, true, true));

        // Missing novelty
        assert!(!PatentRequirements::is_patentable(false, true, true));

        // Missing inventive step
        assert!(!PatentRequirements::is_patentable(true, false, true));

        // Missing industrial applicability
        assert!(!PatentRequirements::is_patentable(true, true, false));
    }

    #[test]
    fn test_validate_patent() {
        assert!(validate_patent_requirements(true, true, true).is_ok());
        assert!(validate_patent_requirements(false, true, true).is_err());
        assert!(validate_patent_requirements(true, false, true).is_err());
    }

    #[test]
    fn test_trademark_types() {
        let word_mark = TrademarkType::Word("ACME".to_string());
        assert!(word_mark.description_vi().contains("ACME"));

        let combined = TrademarkType::Combined {
            words: "ACME Corp".to_string(),
        };
        assert!(combined.description_vi().contains("kết hợp"));
    }

    #[test]
    fn test_copyright_subjects() {
        assert!(CopyrightSubject::Software.registration_recommended());
        assert!(CopyrightSubject::Database.registration_recommended());
        assert!(!CopyrightSubject::Literary.registration_recommended());
    }

    #[test]
    fn test_infringement_types() {
        assert!(IpInfringement::Counterfeiting.allows_criminal_prosecution());
        assert!(IpInfringement::Piracy.allows_criminal_prosecution());
        assert!(!IpInfringement::UnauthorizedUse.allows_criminal_prosecution());
    }

    #[test]
    fn test_protection_expiry() {
        // Patent after 15 years - still valid
        assert!(check_protection_expired(&IpRightType::Patent, 15).is_ok());

        // Patent after 21 years - expired
        assert!(check_protection_expired(&IpRightType::Patent, 21).is_err());

        // Trademark after 9 years - still valid
        assert!(check_protection_expired(&IpRightType::Trademark, 9).is_ok());

        // Trademark after 11 years - expired (unless renewed)
        assert!(check_protection_expired(&IpRightType::Trademark, 11).is_err());
    }

    #[test]
    fn test_ip_registration() {
        let registration = IpRegistration {
            ip_type: IpRightType::Patent,
            applicant: "Inventor X".to_string(),
            title: "New widget".to_string(),
            filing_date: "2024-01-15".to_string(),
            priority_claim: None,
        };

        assert!(registration.registration_required());
        assert_eq!(registration.protection_duration(), Some(20));
    }

    #[test]
    fn test_ip_checklist() {
        let checklist = get_ip_checklist();
        assert!(!checklist.is_empty());
        assert_eq!(checklist.len(), 6);
    }
}
