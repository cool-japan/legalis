//! Vietnamese Enterprise Law 2020 (Luật Doanh nghiệp 2020) - Law No. 59/2020/QH14
//!
//! Vietnam's comprehensive company law, governing business establishment and operation.
//!
//! ## Enterprise Types (Article 74-117)
//!
//! - Single-member Limited Liability Company (Công ty TNHH một thành viên)
//! - Multi-member Limited Liability Company (Công ty TNHH hai thành viên trở lên)
//! - Joint Stock Company (Công ty cổ phần)
//! - Partnership (Công ty hợp danh)
//! - Private Enterprise (Doanh nghiệp tư nhân)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Enterprise type - Article 74-117
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnterpriseType {
    /// Công ty TNHH một thành viên (Single-member LLC)
    SingleMemberLlc,
    /// Công ty TNHH hai thành viên trở lên (Multi-member LLC)
    MultiMemberLlc,
    /// Công ty cổ phần (Joint Stock Company)
    JointStockCompany,
    /// Công ty hợp danh (Partnership)
    Partnership,
    /// Doanh nghiệp tư nhân (Private Enterprise)
    PrivateEnterprise,
    /// Chi nhánh (Branch office of foreign company)
    BranchOffice,
    /// Văn phòng đại diện (Representative office)
    RepresentativeOffice,
}

impl EnterpriseType {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> &'static str {
        match self {
            Self::SingleMemberLlc => "Công ty TNHH một thành viên",
            Self::MultiMemberLlc => "Công ty TNHH hai thành viên trở lên",
            Self::JointStockCompany => "Công ty cổ phần",
            Self::Partnership => "Công ty hợp danh",
            Self::PrivateEnterprise => "Doanh nghiệp tư nhân",
            Self::BranchOffice => "Chi nhánh",
            Self::RepresentativeOffice => "Văn phòng đại diện",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::SingleMemberLlc => "Single-member Limited Liability Company",
            Self::MultiMemberLlc => "Multi-member Limited Liability Company",
            Self::JointStockCompany => "Joint Stock Company",
            Self::Partnership => "Partnership",
            Self::PrivateEnterprise => "Private Enterprise",
            Self::BranchOffice => "Branch Office",
            Self::RepresentativeOffice => "Representative Office",
        }
    }

    /// Check if enterprise type has limited liability
    pub fn has_limited_liability(&self) -> bool {
        matches!(
            self,
            Self::SingleMemberLlc | Self::MultiMemberLlc | Self::JointStockCompany
        )
    }

    /// Get minimum members/shareholders
    pub fn minimum_members(&self) -> Option<u32> {
        match self {
            Self::SingleMemberLlc => Some(1),
            Self::MultiMemberLlc => Some(2),
            Self::JointStockCompany => Some(3), // 3 founding shareholders
            Self::Partnership => Some(2),
            Self::PrivateEnterprise => Some(1),
            _ => None,
        }
    }

    /// Get maximum members (if applicable)
    pub fn maximum_members(&self) -> Option<u32> {
        match self {
            Self::SingleMemberLlc => Some(1),
            Self::MultiMemberLlc => Some(50),
            Self::JointStockCompany => None, // Unlimited
            Self::Partnership => None,
            Self::PrivateEnterprise => Some(1),
            _ => None,
        }
    }
}

/// Enterprise registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseRegistration {
    /// Enterprise name
    pub name: String,
    /// Enterprise type
    pub enterprise_type: EnterpriseType,
    /// Charter capital (VND)
    pub charter_capital: i64,
    /// Head office address
    pub head_office: String,
    /// Business lines (VSIC codes)
    pub business_lines: Vec<String>,
    /// Legal representative
    pub legal_representative: String,
    /// Founding members/shareholders count
    pub member_count: u32,
    /// Foreign ownership percentage (if applicable)
    pub foreign_ownership_percent: Option<u32>,
}

impl EnterpriseRegistration {
    /// Check if registration meets basic requirements
    pub fn is_valid(&self) -> bool {
        // Check member count
        if let Some(min) = self.enterprise_type.minimum_members()
            && self.member_count < min
        {
            return false;
        }
        if let Some(max) = self.enterprise_type.maximum_members()
            && self.member_count > max
        {
            return false;
        }

        // Check charter capital (must be positive)
        if self.charter_capital <= 0 {
            return false;
        }

        // Check name not empty
        if self.name.is_empty() {
            return false;
        }

        true
    }
}

/// Result type for enterprise operations
pub type EnterpriseResult<T> = Result<T, EnterpriseError>;

/// Errors related to Enterprise Law
#[derive(Debug, Error)]
pub enum EnterpriseError {
    /// Invalid enterprise name - Article 37-39
    #[error("Tên doanh nghiệp không hợp lệ (Điều 37-39 LDN): {reason}")]
    InvalidName { reason: String },

    /// Member count violation - Article 74+
    #[error("Số thành viên không hợp lệ (LDN): {actual} (yêu cầu {min}-{max})")]
    InvalidMemberCount { actual: u32, min: u32, max: u32 },

    /// Charter capital violation
    #[error("Vốn điều lệ không hợp lệ: {amount} VND")]
    InvalidCharterCapital { amount: i64 },

    /// Business line restriction
    #[error("Ngành nghề kinh doanh bị hạn chế: {line}")]
    RestrictedBusinessLine { line: String },

    /// Foreign ownership restriction
    #[error("Tỷ lệ sở hữu nước ngoài vượt quá quy định: {actual}% (tối đa {limit}%)")]
    ForeignOwnershipExceeded { actual: u32, limit: u32 },
}

/// Validate enterprise registration
pub fn validate_registration(reg: &EnterpriseRegistration) -> EnterpriseResult<()> {
    // Check member count
    let min = reg.enterprise_type.minimum_members().unwrap_or(1);
    let max = reg.enterprise_type.maximum_members().unwrap_or(u32::MAX);

    if reg.member_count < min || reg.member_count > max {
        return Err(EnterpriseError::InvalidMemberCount {
            actual: reg.member_count,
            min,
            max,
        });
    }

    // Check charter capital
    if reg.charter_capital <= 0 {
        return Err(EnterpriseError::InvalidCharterCapital {
            amount: reg.charter_capital,
        });
    }

    // Check name
    if reg.name.is_empty() || reg.name.len() < 3 {
        return Err(EnterpriseError::InvalidName {
            reason: "Tên quá ngắn".to_string(),
        });
    }

    Ok(())
}

/// Get enterprise law checklist
pub fn get_enterprise_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Tên doanh nghiệp hợp lệ",
            "Valid enterprise name",
            "Điều 37-39",
        ),
        ("Địa chỉ trụ sở chính", "Head office address", "Điều 42"),
        (
            "Vốn điều lệ đăng ký",
            "Registered charter capital",
            "Điều 47",
        ),
        (
            "Người đại diện theo pháp luật",
            "Legal representative",
            "Điều 12",
        ),
        (
            "Danh mục ngành nghề kinh doanh",
            "Business lines registered",
            "Điều 7",
        ),
        (
            "Điều lệ công ty (TNHH, Cổ phần)",
            "Company charter",
            "Điều 24-25",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enterprise_types() {
        assert!(EnterpriseType::SingleMemberLlc.has_limited_liability());
        assert!(!EnterpriseType::Partnership.has_limited_liability());
        assert_eq!(EnterpriseType::JointStockCompany.minimum_members(), Some(3));
    }

    #[test]
    fn test_registration_valid() {
        let reg = EnterpriseRegistration {
            name: "Công ty ABC".to_string(),
            enterprise_type: EnterpriseType::MultiMemberLlc,
            charter_capital: 1_000_000_000,
            head_office: "Hà Nội".to_string(),
            business_lines: vec!["6201".to_string()],
            legal_representative: "Nguyễn Văn A".to_string(),
            member_count: 2,
            foreign_ownership_percent: None,
        };

        assert!(reg.is_valid());
        assert!(validate_registration(&reg).is_ok());
    }

    #[test]
    fn test_registration_invalid_member_count() {
        let reg = EnterpriseRegistration {
            name: "Công ty ABC".to_string(),
            enterprise_type: EnterpriseType::SingleMemberLlc,
            charter_capital: 1_000_000_000,
            head_office: "Hà Nội".to_string(),
            business_lines: vec!["6201".to_string()],
            legal_representative: "Nguyễn Văn A".to_string(),
            member_count: 5, // Invalid for single-member LLC
            foreign_ownership_percent: None,
        };

        assert!(!reg.is_valid());
    }

    #[test]
    fn test_enterprise_checklist() {
        let checklist = get_enterprise_checklist();
        assert!(!checklist.is_empty());
    }
}
