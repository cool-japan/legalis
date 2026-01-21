//! LPA Error Types

use crate::calendar::BuddhistYear;
use crate::citation::ThaiAct;
use thiserror::Error;

/// LPA Error types
#[derive(Debug, Clone, Error)]
pub enum LpaError {
    /// Working hours violation (Section 23)
    #[error("ชั่วโมงทำงานเกินกำหนด (มาตรา 23): {description}")]
    WorkingHoursViolation {
        /// Description of the violation
        description: String,
    },

    /// Overtime violation (Section 24)
    #[error("ล่วงเวลาเกินกำหนด (มาตรา 24): {description}")]
    OvertimeViolation {
        /// Description of the violation
        description: String,
    },

    /// Rest period violation (Section 27)
    #[error("เวลาพักไม่เพียงพอ (มาตรา 27): {description}")]
    RestPeriodViolation {
        /// Description of the violation
        description: String,
    },

    /// Holiday violation (Section 29)
    #[error("วันหยุดไม่ครบตามกฎหมาย (มาตรา 29): {description}")]
    HolidayViolation {
        /// Description of the violation
        description: String,
    },

    /// Annual leave violation (Section 30)
    #[error("วันลาพักร้อนไม่ครบ (มาตรา 30): {description}")]
    AnnualLeaveViolation {
        /// Description of the violation
        description: String,
    },

    /// Minimum wage violation (Section 90)
    #[error("ค่าจ้างต่ำกว่าอัตราขั้นต่ำ (มาตรา 90): ค่าจ้าง {wage} บาท ขั้นต่ำ {minimum} บาท")]
    MinimumWageViolation {
        /// Actual wage
        wage: u32,
        /// Minimum wage
        minimum: u32,
    },

    /// Severance pay violation (Section 118)
    #[error("ค่าชดเชยไม่ครบตามกฎหมาย (มาตรา 118): {description}")]
    SeveranceViolation {
        /// Description of the violation
        description: String,
    },

    /// Child labor violation (Section 44-52)
    #[error("การจ้างแรงงานเด็กไม่ถูกต้อง (มาตรา 44-52): {description}")]
    ChildLaborViolation {
        /// Description of the violation
        description: String,
    },

    /// Wrongful termination (Section 119)
    #[error("การเลิกจ้างไม่เป็นธรรม (มาตรา 119): {description}")]
    WrongfulTermination {
        /// Description of the violation
        description: String,
    },

    /// Discrimination (Section 15)
    #[error("การเลือกปฏิบัติในการจ้างงาน (มาตรา 15): {description}")]
    Discrimination {
        /// Description of the violation
        description: String,
    },
}

impl LpaError {
    /// Get the relevant LPA section citation
    pub fn citation(&self) -> String {
        let lpa = ThaiAct::new(
            "คุ้มครองแรงงาน",
            "Labour Protection Act",
            BuddhistYear::from_be(2541),
        );

        let section = match self {
            Self::WorkingHoursViolation { .. } => 23,
            Self::OvertimeViolation { .. } => 24,
            Self::RestPeriodViolation { .. } => 27,
            Self::HolidayViolation { .. } => 29,
            Self::AnnualLeaveViolation { .. } => 30,
            Self::MinimumWageViolation { .. } => 90,
            Self::SeveranceViolation { .. } => 118,
            Self::ChildLaborViolation { .. } => 44,
            Self::WrongfulTermination { .. } => 119,
            Self::Discrimination { .. } => 15,
        };

        lpa.section(section).format_th()
    }

    /// Get maximum administrative fine (in THB)
    pub fn max_fine(&self) -> u64 {
        match self {
            Self::WorkingHoursViolation { .. } => 20_000,
            Self::OvertimeViolation { .. } => 20_000,
            Self::RestPeriodViolation { .. } => 20_000,
            Self::HolidayViolation { .. } => 20_000,
            Self::AnnualLeaveViolation { .. } => 20_000,
            Self::MinimumWageViolation { .. } => 100_000,
            Self::SeveranceViolation { .. } => 200_000,
            Self::ChildLaborViolation { .. } => 400_000,
            Self::WrongfulTermination { .. } => 100_000,
            Self::Discrimination { .. } => 100_000,
        }
    }

    /// Check if violation can result in criminal penalty
    pub fn has_criminal_penalty(&self) -> bool {
        matches!(
            self,
            Self::ChildLaborViolation { .. }
                | Self::MinimumWageViolation { .. }
                | Self::SeveranceViolation { .. }
        )
    }

    /// Get criminal penalty description if applicable
    pub fn criminal_penalty(&self) -> Option<&'static str> {
        match self {
            Self::ChildLaborViolation { .. } => Some("จำคุกไม่เกิน 2 ปี หรือปรับไม่เกิน 400,000 บาท"),
            Self::MinimumWageViolation { .. } => Some("จำคุกไม่เกิน 6 เดือน หรือปรับไม่เกิน 100,000 บาท"),
            Self::SeveranceViolation { .. } => Some("จำคุกไม่เกิน 6 เดือน หรือปรับไม่เกิน 200,000 บาท"),
            _ => None,
        }
    }

    /// Get remediation guidance in Thai
    pub fn remediation_th(&self) -> &'static str {
        match self {
            Self::WorkingHoursViolation { .. } => {
                "ปรับชั่วโมงทำงานให้ไม่เกิน 8 ชั่วโมงต่อวัน หรือ 48 ชั่วโมงต่อสัปดาห์"
            }
            Self::OvertimeViolation { .. } => "ขอความยินยอมและจ่ายค่าล่วงเวลาตามอัตราที่กฎหมายกำหนด",
            Self::RestPeriodViolation { .. } => "จัดให้มีเวลาพักอย่างน้อย 1 ชั่วโมงหลังทำงาน 5 ชั่วโมง",
            Self::HolidayViolation { .. } => "จัดให้มีวันหยุดตามประเพณีไม่น้อยกว่า 13 วันต่อปี",
            Self::AnnualLeaveViolation { .. } => "จัดให้มีวันลาพักร้อนไม่น้อยกว่า 6 วันต่อปี",
            Self::MinimumWageViolation { .. } => "ปรับค่าจ้างให้ไม่ต่ำกว่าอัตราค่าจ้างขั้นต่ำ",
            Self::SeveranceViolation { .. } => "จ่ายค่าชดเชยตามอัตราที่กฎหมายกำหนด",
            Self::ChildLaborViolation { .. } => "หยุดการจ้างแรงงานเด็กหรือปฏิบัติตามข้อกำหนดพิเศษ",
            Self::WrongfulTermination { .. } => "จ่ายค่าเสียหายและค่าชดเชยตามคำสั่งศาลแรงงาน",
            Self::Discrimination { .. } => "ยุติการเลือกปฏิบัติและชดใช้ค่าเสียหาย",
        }
    }
}

/// Result type for LPA operations
pub type LpaResult<T> = Result<T, LpaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_hours_citation() {
        let error = LpaError::WorkingHoursViolation {
            description: "ทำงานเกิน 48 ชั่วโมงต่อสัปดาห์".to_string(),
        };
        let citation = error.citation();
        assert!(citation.contains("มาตรา 23"));
    }

    #[test]
    fn test_max_fine() {
        let error = LpaError::ChildLaborViolation {
            description: "จ้างเด็กอายุต่ำกว่า 15 ปี".to_string(),
        };
        assert_eq!(error.max_fine(), 400_000);
    }

    #[test]
    fn test_criminal_penalty() {
        let error = LpaError::ChildLaborViolation {
            description: "test".to_string(),
        };
        assert!(error.has_criminal_penalty());
        assert!(error.criminal_penalty().is_some());
    }

    #[test]
    fn test_no_criminal_penalty() {
        let error = LpaError::RestPeriodViolation {
            description: "test".to_string(),
        };
        assert!(!error.has_criminal_penalty());
    }

    #[test]
    fn test_minimum_wage_error() {
        let error = LpaError::MinimumWageViolation {
            wage: 300,
            minimum: 363,
        };
        let msg = error.to_string();
        assert!(msg.contains("300"));
        assert!(msg.contains("363"));
    }
}
