//! Personal Data Protection Act 2012 - Error Types
//!
//! This module defines error types for Singapore PDPA violations with quadrilingual messages
//! in Singapore's four official languages: English, Chinese (中文), Malay (Bahasa Melayu), Tamil (தமிழ்).

use thiserror::Error;

/// Result type for PDPA operations
pub type Result<T> = std::result::Result<T, PdpaError>;

/// PDPA error types
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PdpaError {
    /// Missing or invalid consent (s. 13)
    #[error(
        "Missing consent for data collection (PDPA s. 13)\n\
         数据收集缺少同意 (个人资料保护法第13条)"
    )]
    MissingConsent,

    /// Consent withdrawn
    #[error(
        "Consent has been withdrawn\n\
         同意已被撤回"
    )]
    ConsentWithdrawn,

    /// Purpose limitation violation (s. 18)
    #[error(
        "Data used beyond stated purpose (PDPA s. 18)\n\
         数据使用超出声明目的 (个人资料保护法第18条)"
    )]
    PurposeLimitationViolation,

    /// Data breach notification late (s. 26C)
    #[error(
        "Breach notification exceeded 3-day deadline (PDPA s. 26C)\n\
         违反通知超过3天期限 (个人资料保护法第26C条)\n\
         Pemberitahuan pelanggaran melebihi had masa 3 hari (PDPA s. 26C)"
    )]
    LateBreachNotification,

    /// DNC registry violation (Part IX)
    #[error(
        "Phone number {phone} is on DNC Registry for {dnc_type:?} (PDPA Part IX)\n\
         电话号码 {phone} 在拒收讯息登记册 {dnc_type:?} (个人资料保护法第九部分)"
    )]
    DncViolation { phone: String, dnc_type: String },

    /// Cross-border transfer without adequate protection
    #[error(
        "Cross-border transfer to {country} lacks adequate protection\n\
         跨境转移到 {country} 缺乏足够保护"
    )]
    InadequateTransferProtection { country: String },

    /// Access request not fulfilled (s. 21)
    #[error(
        "Access request response exceeded 30-day deadline (PDPA s. 21)\n\
         访问请求响应超过30天期限 (个人资料保护法第21条)"
    )]
    AccessRequestDelayed,

    /// Generic validation error
    #[error(
        "PDPA validation error: {message}\n\
         个人资料保护法验证错误: {message}"
    )]
    ValidationError { message: String },
}

impl PdpaError {
    /// Returns the PDPA section reference
    pub fn statute_reference(&self) -> Option<&'static str> {
        match self {
            PdpaError::MissingConsent | PdpaError::ConsentWithdrawn => Some("PDPA s. 13"),
            PdpaError::PurposeLimitationViolation => Some("PDPA s. 18"),
            PdpaError::LateBreachNotification => Some("PDPA s. 26C"),
            PdpaError::DncViolation { .. } => Some("PDPA Part IX"),
            PdpaError::AccessRequestDelayed => Some("PDPA s. 21"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_statute_reference() {
        let error = PdpaError::MissingConsent;
        assert_eq!(error.statute_reference(), Some("PDPA s. 13"));

        let error2 = PdpaError::LateBreachNotification;
        assert_eq!(error2.statute_reference(), Some("PDPA s. 26C"));
    }

    #[test]
    fn test_error_display() {
        let error = PdpaError::DncViolation {
            phone: "+6598765432".to_string(),
            dnc_type: "Voice".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("+6598765432"));
        assert!(display.contains("DNC Registry"));
    }
}
