//! PDPA Error Types

use crate::calendar::BuddhistYear;
use crate::citation::ThaiAct;
use thiserror::Error;

/// PDPA Error types
#[derive(Debug, Clone, Error)]
pub enum PdpaError {
    /// No legal basis for processing (Section 24)
    #[error("ไม่มีฐานทางกฎหมายในการประมวลผล (มาตรา 24): {description}")]
    NoLegalBasis {
        /// Description of the violation
        description: String,
    },

    /// Invalid consent (Section 19)
    #[error("ความยินยอมไม่ถูกต้อง (มาตรา 19): {reason}")]
    InvalidConsent {
        /// Reason consent is invalid
        reason: String,
    },

    /// Sensitive data processing violation (Section 26)
    #[error("การประมวลผลข้อมูลอ่อนไหวไม่ถูกต้อง (มาตรา 26): {description}")]
    SensitiveDataViolation {
        /// Description of the violation
        description: String,
    },

    /// Cross-border transfer violation (Section 28)
    #[error("การโอนข้อมูลข้ามประเทศไม่ถูกต้อง (มาตรา 28): ประเทศปลายทาง {destination}")]
    InvalidCrossBorderTransfer {
        /// Destination country
        destination: String,
    },

    /// Data subject right denied (Sections 30-36)
    #[error("ปฏิเสธสิทธิของเจ้าของข้อมูล: {right}")]
    RightDenied {
        /// Right that was denied
        right: String,
    },

    /// Breach notification failure (Section 37)
    #[error("ไม่ได้แจ้งเหตุการละเมิดข้อมูล (มาตรา 37): {description}")]
    BreachNotReported {
        /// Description of the failure
        description: String,
    },

    /// Excessive data collection
    #[error("เก็บข้อมูลเกินความจำเป็น (มาตรา 22): {description}")]
    ExcessiveDataCollection {
        /// Description of the issue
        description: String,
    },

    /// Retention period violation (Section 23)
    #[error("เก็บข้อมูลเกินระยะเวลาที่จำเป็น (มาตรา 23): {description}")]
    RetentionViolation {
        /// Description of the violation
        description: String,
    },

    /// Missing DPO (Section 41)
    #[error("ไม่มีเจ้าหน้าที่คุ้มครองข้อมูลส่วนบุคคล (มาตรา 41)")]
    MissingDpo,

    /// Security measures inadequate (Section 37)
    #[error("มาตรการรักษาความปลอดภัยไม่เพียงพอ (มาตรา 37): {description}")]
    InadequateSecurity {
        /// Description of the issue
        description: String,
    },
}

impl PdpaError {
    /// Get the relevant PDPA section citation
    pub fn citation(&self) -> String {
        let pdpa = ThaiAct::new(
            "คุ้มครองข้อมูลส่วนบุคคล",
            "Personal Data Protection Act",
            BuddhistYear::from_be(2562),
        );

        let section = match self {
            Self::NoLegalBasis { .. } => 24,
            Self::InvalidConsent { .. } => 19,
            Self::SensitiveDataViolation { .. } => 26,
            Self::InvalidCrossBorderTransfer { .. } => 28,
            Self::RightDenied { .. } => 30,
            Self::BreachNotReported { .. } => 37,
            Self::ExcessiveDataCollection { .. } => 22,
            Self::RetentionViolation { .. } => 23,
            Self::MissingDpo => 41,
            Self::InadequateSecurity { .. } => 37,
        };

        pdpa.section(section).format_th()
    }

    /// Get maximum administrative fine (in THB)
    pub fn max_administrative_fine(&self) -> u64 {
        match self {
            Self::NoLegalBasis { .. }
            | Self::InvalidConsent { .. }
            | Self::SensitiveDataViolation { .. } => 5_000_000, // 5M THB

            Self::InvalidCrossBorderTransfer { .. } => 5_000_000,
            Self::RightDenied { .. } => 1_000_000,
            Self::BreachNotReported { .. } => 3_000_000,
            Self::ExcessiveDataCollection { .. } => 3_000_000,
            Self::RetentionViolation { .. } => 3_000_000,
            Self::MissingDpo => 1_000_000,
            Self::InadequateSecurity { .. } => 5_000_000,
        }
    }

    /// Check if violation can result in criminal penalty
    pub fn has_criminal_penalty(&self) -> bool {
        matches!(
            self,
            Self::SensitiveDataViolation { .. } | Self::InvalidCrossBorderTransfer { .. }
        )
    }

    /// Get criminal penalty description if applicable
    pub fn criminal_penalty(&self) -> Option<&'static str> {
        match self {
            Self::SensitiveDataViolation { .. } => {
                Some("จำคุกไม่เกิน 1 ปี หรือปรับไม่เกิน 1,000,000 บาท หรือทั้งจำทั้งปรับ (มาตรา 90)")
            }
            Self::InvalidCrossBorderTransfer { .. } => {
                Some("จำคุกไม่เกิน 1 ปี หรือปรับไม่เกิน 1,000,000 บาท หรือทั้งจำทั้งปรับ (มาตรา 90)")
            }
            _ => None,
        }
    }

    /// Get remediation guidance
    pub fn remediation_th(&self) -> &'static str {
        match self {
            Self::NoLegalBasis { .. } => "กำหนดฐานทางกฎหมายที่เหมาะสมสำหรับการประมวลผลข้อมูล",
            Self::InvalidConsent { .. } => "ขอความยินยอมใหม่ที่ชัดเจนและเฉพาะเจาะจง",
            Self::SensitiveDataViolation { .. } => "หยุดการประมวลผลข้อมูลอ่อนไหวหรือขอความยินยอมโดยชัดแจ้ง",
            Self::InvalidCrossBorderTransfer { .. } => "ใช้มาตรการคุ้มครองที่เหมาะสมก่อนโอนข้อมูลข้ามประเทศ",
            Self::RightDenied { .. } => "ดำเนินการตามคำขอใช้สิทธิของเจ้าของข้อมูลภายในกำหนดเวลา",
            Self::BreachNotReported { .. } => "แจ้งเหตุการละเมิดต่อ สคส. ภายใน 72 ชั่วโมง",
            Self::ExcessiveDataCollection { .. } => "เก็บเฉพาะข้อมูลที่จำเป็นสำหรับวัตถุประสงค์ที่กำหนด",
            Self::RetentionViolation { .. } => "ลบหรือทำลายข้อมูลที่ไม่จำเป็นต้องเก็บรักษา",
            Self::MissingDpo => "แต่งตั้งเจ้าหน้าที่คุ้มครองข้อมูลส่วนบุคคล (DPO)",
            Self::InadequateSecurity { .. } => "ปรับปรุงมาตรการรักษาความปลอดภัยให้เหมาะสม",
        }
    }
}

/// Result type for PDPA operations
pub type PdpaResult<T> = Result<T, PdpaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_legal_basis_citation() {
        let error = PdpaError::NoLegalBasis {
            description: "ประมวลผลโดยไม่มีความยินยอม".to_string(),
        };
        let citation = error.citation();
        assert!(citation.contains("มาตรา 24"));
    }

    #[test]
    fn test_max_fine() {
        let error = PdpaError::SensitiveDataViolation {
            description: "ประมวลผลข้อมูลสุขภาพโดยไม่ได้รับอนุญาต".to_string(),
        };
        assert_eq!(error.max_administrative_fine(), 5_000_000);
    }

    #[test]
    fn test_criminal_penalty() {
        let error = PdpaError::SensitiveDataViolation {
            description: "test".to_string(),
        };
        assert!(error.has_criminal_penalty());
        assert!(error.criminal_penalty().is_some());
    }

    #[test]
    fn test_no_criminal_penalty() {
        let error = PdpaError::MissingDpo;
        assert!(!error.has_criminal_penalty());
    }

    #[test]
    fn test_remediation() {
        let error = PdpaError::BreachNotReported {
            description: "ไม่แจ้งภายใน 72 ชั่วโมง".to_string(),
        };
        let remediation = error.remediation_th();
        assert!(remediation.contains("72"));
    }
}
