//! FBA Error Types

use crate::calendar::BuddhistYear;
use crate::citation::ThaiAct;
use thiserror::Error;

/// FBA Error types
#[derive(Debug, Clone, Error)]
pub enum FbaError {
    /// Prohibited business activity (List 1)
    #[error("ธุรกิจต้องห้าม (บัญชี 1): {activity}")]
    ProhibitedActivity {
        /// Activity description
        activity: String,
    },

    /// Requires cabinet approval (List 2)
    #[error("ต้องได้รับอนุมัติจากคณะรัฐมนตรี (บัญชี 2): {activity}")]
    RequiresCabinetApproval {
        /// Activity description
        activity: String,
    },

    /// Requires FBA license (List 3)
    #[error("ต้องขอใบอนุญาต (บัญชี 3): {activity}")]
    RequiresLicense {
        /// Activity description
        activity: String,
    },

    /// Foreign ownership exceeds limit
    #[error("สัดส่วนการถือหุ้นของคนต่างด้าวเกินกำหนด: {percentage}% (สูงสุด {limit}%)")]
    ExcessiveForeignOwnership {
        /// Actual percentage
        percentage: f64,
        /// Maximum allowed
        limit: f64,
    },

    /// Invalid nominee structure detected
    #[error("ตรวจพบโครงสร้างนอมินี: {description}")]
    NomineeStructure {
        /// Description of the issue
        description: String,
    },

    /// License application rejected
    #[error("คำขอใบอนุญาตถูกปฏิเสธ: {reason}")]
    LicenseRejected {
        /// Rejection reason
        reason: String,
    },

    /// License expired
    #[error("ใบอนุญาตหมดอายุ")]
    LicenseExpired,

    /// Treaty exemption not applicable
    #[error("ไม่สามารถใช้สิทธิตามสนธิสัญญา: {reason}")]
    TreatyExemptionInvalid {
        /// Reason exemption is invalid
        reason: String,
    },

    /// BOI exemption not applicable
    #[error("ไม่สามารถใช้สิทธิ BOI: {reason}")]
    BoiExemptionInvalid {
        /// Reason exemption is invalid
        reason: String,
    },

    /// Insufficient Thai shareholders
    #[error("จำนวนผู้ถือหุ้นไทยไม่เพียงพอ: {count} คน (ต้องมีอย่างน้อย {required} คน)")]
    InsufficientThaiShareholders {
        /// Actual count
        count: u32,
        /// Required count
        required: u32,
    },
}

impl FbaError {
    /// Get the relevant FBA citation
    pub fn citation(&self) -> String {
        let fba = ThaiAct::new(
            "ประกอบธุรกิจของคนต่างด้าว",
            "Foreign Business Act",
            BuddhistYear::from_be(2542),
        );

        let section = match self {
            Self::ProhibitedActivity { .. } => 8,
            Self::RequiresCabinetApproval { .. } => 8,
            Self::RequiresLicense { .. } => 17,
            Self::ExcessiveForeignOwnership { .. } => 4,
            Self::NomineeStructure { .. } => 36,
            Self::LicenseRejected { .. } => 17,
            Self::LicenseExpired => 20,
            Self::TreatyExemptionInvalid { .. } => 10,
            Self::BoiExemptionInvalid { .. } => 12,
            Self::InsufficientThaiShareholders { .. } => 4,
        };

        fba.section(section).format_th()
    }

    /// Get maximum penalty (in THB)
    pub fn max_penalty(&self) -> u64 {
        match self {
            Self::ProhibitedActivity { .. } => 1_000_000, // + up to 3 years imprisonment
            Self::RequiresCabinetApproval { .. } => 1_000_000,
            Self::RequiresLicense { .. } => 1_000_000,
            Self::ExcessiveForeignOwnership { .. } => 500_000,
            Self::NomineeStructure { .. } => 1_000_000, // + up to 3 years imprisonment
            Self::LicenseRejected { .. } => 0,
            Self::LicenseExpired => 500_000,
            Self::TreatyExemptionInvalid { .. } => 500_000,
            Self::BoiExemptionInvalid { .. } => 500_000,
            Self::InsufficientThaiShareholders { .. } => 500_000,
        }
    }

    /// Check if violation can result in criminal penalty
    pub fn has_criminal_penalty(&self) -> bool {
        matches!(
            self,
            Self::ProhibitedActivity { .. }
                | Self::NomineeStructure { .. }
                | Self::RequiresCabinetApproval { .. }
                | Self::RequiresLicense { .. }
        )
    }

    /// Get criminal penalty description if applicable
    pub fn criminal_penalty(&self) -> Option<&'static str> {
        match self {
            Self::ProhibitedActivity { .. }
            | Self::RequiresCabinetApproval { .. }
            | Self::RequiresLicense { .. } => {
                Some("จำคุกไม่เกิน 3 ปี หรือปรับไม่เกิน 1,000,000 บาท หรือทั้งจำทั้งปรับ")
            }
            Self::NomineeStructure { .. } => {
                Some("จำคุกไม่เกิน 3 ปี หรือปรับไม่เกิน 1,000,000 บาท หรือทั้งจำทั้งปรับ (มาตรา 36)")
            }
            _ => None,
        }
    }

    /// Get remediation guidance
    pub fn remediation_th(&self) -> &'static str {
        match self {
            Self::ProhibitedActivity { .. } => "ยุติการดำเนินธุรกิจที่ต้องห้าม",
            Self::RequiresCabinetApproval { .. } => "ยื่นคำขออนุมัติต่อคณะรัฐมนตรี",
            Self::RequiresLicense { .. } => "ยื่นคำขอใบอนุญาตต่อกรมพัฒนาธุรกิจการค้า",
            Self::ExcessiveForeignOwnership { .. } => "ปรับโครงสร้างผู้ถือหุ้นให้เป็นไปตามกฎหมาย",
            Self::NomineeStructure { .. } => "ยุติการใช้นอมินีและปรับโครงสร้างใหม่",
            Self::LicenseRejected { .. } => "แก้ไขเอกสารและยื่นคำขอใหม่",
            Self::LicenseExpired => "ยื่นคำขอต่ออายุใบอนุญาต",
            Self::TreatyExemptionInvalid { .. } => "ตรวจสอบคุณสมบัติและยื่นขอใบอนุญาตปกติ",
            Self::BoiExemptionInvalid { .. } => "ติดต่อ BOI เพื่อตรวจสอบสิทธิประโยชน์",
            Self::InsufficientThaiShareholders { .. } => "เพิ่มจำนวนผู้ถือหุ้นไทยให้เป็นไปตามกฎหมาย",
        }
    }
}

/// Result type for FBA operations
pub type FbaResult<T> = Result<T, FbaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prohibited_activity_citation() {
        let error = FbaError::ProhibitedActivity {
            activity: "การค้าที่ดิน".to_string(),
        };
        let citation = error.citation();
        assert!(citation.contains("มาตรา 8"));
    }

    #[test]
    fn test_max_penalty() {
        let error = FbaError::NomineeStructure {
            description: "ใช้บุคคลธรรมดาถือหุ้นแทน".to_string(),
        };
        assert_eq!(error.max_penalty(), 1_000_000);
    }

    #[test]
    fn test_criminal_penalty() {
        let error = FbaError::NomineeStructure {
            description: "test".to_string(),
        };
        assert!(error.has_criminal_penalty());
        assert!(error.criminal_penalty().is_some());
    }

    #[test]
    fn test_no_criminal_penalty() {
        let error = FbaError::LicenseExpired;
        assert!(!error.has_criminal_penalty());
    }

    #[test]
    fn test_ownership_error() {
        let error = FbaError::ExcessiveForeignOwnership {
            percentage: 60.0,
            limit: 49.0,
        };
        let msg = error.to_string();
        assert!(msg.contains("60"));
        assert!(msg.contains("49"));
    }
}
