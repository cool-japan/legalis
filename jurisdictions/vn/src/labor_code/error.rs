//! Error types for Vietnamese Labor Code 2019

use thiserror::Error;

/// Result type for labor code operations
pub type LaborCodeResult<T> = Result<T, LaborCodeError>;

/// Errors related to Labor Code 2019 compliance
#[derive(Debug, Error)]
pub enum LaborCodeError {
    /// Working hours exceeded - Article 105
    #[error("Giờ làm việc vượt quá quy định (Điều 105 BLLĐ): {hours} giờ/tuần (tối đa 48 giờ)")]
    WorkingHoursExceeded { hours: u32 },

    /// Overtime exceeded - Article 107
    #[error("Giờ làm thêm vượt quá quy định (Điều 107 BLLĐ): {hours} giờ/tháng (tối đa 40 giờ)")]
    OvertimeExceeded { hours: u32 },

    /// Minimum wage violation - Article 90-91
    #[error(
        "Lương dưới mức lương tối thiểu vùng (Điều 90-91 BLLĐ): {actual} đ (tối thiểu {minimum} đ)"
    )]
    MinimumWageViolation { actual: i64, minimum: i64 },

    /// Invalid contract duration - Article 20
    #[error("Thời hạn hợp đồng không hợp lệ (Điều 20 BLLĐ): {months} tháng")]
    InvalidContractDuration { months: u32 },

    /// Contract not in Vietnamese - Article 14
    #[error("Hợp đồng lao động phải bằng tiếng Việt (Điều 14 BLLĐ)")]
    ContractNotInVietnamese,

    /// Contract not written - Article 14
    #[error("Hợp đồng lao động phải bằng văn bản (Điều 14 BLLĐ)")]
    ContractNotWritten,

    /// Probation period too long - Article 25
    #[error(
        "Thời gian thử việc vượt quá quy định (Điều 25 BLLĐ): {days} ngày (tối đa {max_days} ngày)"
    )]
    ProbationTooLong { days: u32, max_days: u32 },

    /// Notice period violation - Article 35-36
    #[error(
        "Vi phạm thời hạn báo trước (Điều 35-36 BLLĐ): {provided} ngày (yêu cầu {required} ngày)"
    )]
    NoticePeriodViolation { provided: u32, required: u32 },

    /// Unlawful termination - Article 36
    #[error("Chấm dứt hợp đồng trái pháp luật (Điều 36 BLLĐ): {reason}")]
    UnlawfulTermination { reason: String },

    /// Missing social insurance - Article 168
    #[error("Chưa đóng bảo hiểm xã hội bắt buộc (Điều 168 BLLĐ)")]
    MissingSocialInsurance,

    /// Child labor violation - Article 143-147
    #[error("Vi phạm quy định về lao động chưa thành niên (Điều 143-147 BLLĐ): {description}")]
    ChildLaborViolation { description: String },

    /// Rest period violation - Article 108-110
    #[error("Vi phạm thời gian nghỉ ngơi (Điều 108-110 BLLĐ): {description}")]
    RestPeriodViolation { description: String },

    /// Annual leave violation - Article 113
    #[error(
        "Vi phạm quy định về nghỉ phép năm (Điều 113 BLLĐ): {provided} ngày (tối thiểu {required} ngày)"
    )]
    AnnualLeaveViolation { provided: u32, required: u32 },

    /// Discrimination - Article 8
    #[error("Phân biệt đối xử trong lao động (Điều 8 BLLĐ): {description}")]
    Discrimination { description: String },
}

impl LaborCodeError {
    /// Get the statutory reference
    pub fn statutory_reference(&self) -> &'static str {
        match self {
            Self::WorkingHoursExceeded { .. } => "Bộ luật Lao động 2019, Điều 105",
            Self::OvertimeExceeded { .. } => "Bộ luật Lao động 2019, Điều 107",
            Self::MinimumWageViolation { .. } => "Bộ luật Lao động 2019, Điều 90-91",
            Self::InvalidContractDuration { .. } => "Bộ luật Lao động 2019, Điều 20",
            Self::ContractNotInVietnamese => "Bộ luật Lao động 2019, Điều 14",
            Self::ContractNotWritten => "Bộ luật Lao động 2019, Điều 14",
            Self::ProbationTooLong { .. } => "Bộ luật Lao động 2019, Điều 25",
            Self::NoticePeriodViolation { .. } => "Bộ luật Lao động 2019, Điều 35-36",
            Self::UnlawfulTermination { .. } => "Bộ luật Lao động 2019, Điều 36",
            Self::MissingSocialInsurance => "Bộ luật Lao động 2019, Điều 168",
            Self::ChildLaborViolation { .. } => "Bộ luật Lao động 2019, Điều 143-147",
            Self::RestPeriodViolation { .. } => "Bộ luật Lao động 2019, Điều 108-110",
            Self::AnnualLeaveViolation { .. } => "Bộ luật Lao động 2019, Điều 113",
            Self::Discrimination { .. } => "Bộ luật Lao động 2019, Điều 8",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        let error = LaborCodeError::MinimumWageViolation {
            actual: 3_000_000,
            minimum: 4_680_000,
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Điều 90-91"));
        assert!(msg.contains("BLLĐ"));
    }

    #[test]
    fn test_statutory_reference() {
        let error = LaborCodeError::WorkingHoursExceeded { hours: 50 };
        assert!(error.statutory_reference().contains("Điều 105"));
    }
}
