//! Personal Data Protection Act 2012 — error types.
//!
//! Error messages are provided in Singapore's four official languages: English,
//! Chinese (中文), Malay (Bahasa Melayu) and Tamil (தமிழ்), following the
//! convention used across the `legalis-sg` crate. Each error carries an accurate
//! PDPA section reference via [`PdpaError::statute_reference`].

use thiserror::Error;

/// Result type for PDPA operations.
pub type Result<T> = std::result::Result<T, PdpaError>;

/// PDPA validation and compliance errors.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PdpaError {
    /// No valid consent (express or deemed) for the collection, use or
    /// disclosure of personal data (s. 13).
    #[error(
        "No valid consent for the collection, use or disclosure of personal data (PDPA s. 13)\n\
         未取得收集、使用或披露个人资料的有效同意 (个人资料保护法第13条)\n\
         Tiada persetujuan sah untuk pengumpulan, penggunaan atau pendedahan data peribadi (PDPA s. 13)\n\
         தனிப்பட்ட தரவைச் சேகரிக்க, பயன்படுத்த அல்லது வெளியிட செல்லுபடியான ஒப்புதல் இல்லை (PDPA s. 13)"
    )]
    MissingConsent,

    /// Consent has been withdrawn under s. 16; processing must cease (s. 16(4)).
    #[error(
        "Consent has been withdrawn; collection, use or disclosure must cease (PDPA s. 16)\n\
         同意已被撤回；必须停止收集、使用或披露 (个人资料保护法第16条)"
    )]
    ConsentWithdrawn,

    /// Withdrawal of consent was processed without informing the individual of
    /// the likely consequences (s. 16(2)).
    #[error(
        "Consequences of withdrawing consent were not explained to the individual (PDPA s. 16(2))\n\
         未向个人说明撤回同意的后果 (个人资料保护法第16(2)条)"
    )]
    WithdrawalConsequencesNotExplained,

    /// A deemed-consent-by-notification flow (s. 15A) is missing a precondition
    /// (the adverse-effect assessment or the opt-out window).
    #[error(
        "Deemed consent by notification is invalid: {reason} (PDPA s. 15A)\n\
         通知推定同意无效: {reason} (个人资料保护法第15A条)"
    )]
    InvalidDeemedConsent { reason: String },

    /// Personal data used or disclosed for a purpose incompatible with the
    /// purpose for which it was collected (s. 18).
    #[error(
        "Personal data used beyond the purpose for which it was collected (PDPA s. 18)\n\
         个人资料的使用超出其收集目的 (个人资料保护法第18条)\n\
         Data peribadi digunakan melebihi tujuan pengumpulannya (PDPA s. 18)"
    )]
    PurposeLimitationViolation,

    /// A notifiable data breach has not been (timely) notified to the PDPC
    /// (s. 26D(1)).
    #[error(
        "Notifiable data breach not reported to the PDPC within 3 calendar days (PDPA s. 26D(1))\n\
         应通报的数据泄露未在3个日历日内通知个人资料保护委员会 (个人资料保护法第26D(1)条)\n\
         Pelanggaran data yang perlu dilaporkan tidak dimaklumkan kepada PDPC dalam 3 hari kalendar (PDPA s. 26D(1))"
    )]
    LateBreachNotification,

    /// A notifiable significant-harm breach has not been notified to affected
    /// individuals and no exemption applies (s. 26D(2)).
    #[error(
        "Affected individuals not notified of a significant-harm data breach (PDPA s. 26D(2))\n\
         未通知受影响个人发生可能造成重大损害的数据泄露 (个人资料保护法第26D(2)条)"
    )]
    IndividualsNotNotified,

    /// A marketing message was (or would be) sent to a number listed on the
    /// relevant DNC register without a valid prior check (Part 9, s. 43).
    #[error(
        "Number {phone} is listed on the {register} (PDPA Part 9, s. 43)\n\
         号码 {phone} 已登记在 {register} (个人资料保护法第9部分第43条)\n\
         Nombor {phone} disenaraikan dalam {register} (PDPA Bahagian 9, s. 43)"
    )]
    DncViolation { phone: String, register: String },

    /// A marketing message was sent without a valid, in-date DNC non-registration
    /// confirmation (Part 9, s. 43(2) — confirmation valid for 21 days).
    #[error(
        "No valid DNC check (within 21 days) before sending to {phone} (PDPA s. 43(2))\n\
         在向 {phone} 发送讯息前没有有效的拒收讯息查询(21天内) (个人资料保护法第43(2)条)"
    )]
    MissingDncCheck { phone: String },

    /// A cross-border transfer does not satisfy the Transfer Limitation
    /// Obligation (s. 26).
    #[error(
        "Cross-border transfer to {country} does not ensure comparable protection (PDPA s. 26)\n\
         向 {country} 的跨境转移未确保可比的保护标准 (个人资料保护法第26条)"
    )]
    InadequateTransferProtection { country: String },

    /// An access request was not responded to (or extended) within 30 days
    /// (s. 21 read with reg. 5 of the PDP Regulations 2021).
    #[error(
        "Access request not handled within 30 days (PDPA s. 21)\n\
         查阅要求未在30天内处理 (个人资料保护法第21条)\n\
         Permintaan akses tidak dikendalikan dalam 30 hari (PDPA s. 21)"
    )]
    AccessRequestOverdue,

    /// A correction request was not actioned as soon as practicable (s. 22).
    #[error(
        "Correction request not actioned as soon as practicable (PDPA s. 22)\n\
         更正要求未尽快处理 (个人资料保护法第22条)"
    )]
    CorrectionRequestOverdue,

    /// The organisation has not designated a Data Protection Officer, contrary to
    /// the mandatory duty in s. 11(3).
    #[error(
        "Organisation has not designated a Data Protection Officer (PDPA s. 11(3))\n\
         机构未指定数据保护主任 (个人资料保护法第11(3)条)\n\
         Organisasi tidak melantik Pegawai Perlindungan Data (PDPA s. 11(3))"
    )]
    NoDataProtectionOfficer,

    /// The DPO's business contact information has not been made available to the
    /// public (s. 11(5)).
    #[error(
        "DPO business contact information not made available to the public (PDPA s. 11(5))\n\
         未向公众提供数据保护主任的业务联系信息 (个人资料保护法第11(5)条)"
    )]
    DpoContactNotPublished,

    /// Generic validation error with a descriptive message.
    #[error(
        "PDPA validation error: {message}\n\
         个人资料保护法验证错误: {message}"
    )]
    ValidationError { message: String },
}

impl PdpaError {
    /// Returns the governing PDPA section (or Part) reference for this error.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            PdpaError::MissingConsent => "PDPA s. 13",
            PdpaError::ConsentWithdrawn => "PDPA s. 16",
            PdpaError::WithdrawalConsequencesNotExplained => "PDPA s. 16(2)",
            PdpaError::InvalidDeemedConsent { .. } => "PDPA s. 15A",
            PdpaError::PurposeLimitationViolation => "PDPA s. 18",
            PdpaError::LateBreachNotification => "PDPA s. 26D(1)",
            PdpaError::IndividualsNotNotified => "PDPA s. 26D(2)",
            PdpaError::DncViolation { .. } => "PDPA s. 43",
            PdpaError::MissingDncCheck { .. } => "PDPA s. 43(2)",
            PdpaError::InadequateTransferProtection { .. } => "PDPA s. 26",
            PdpaError::AccessRequestOverdue => "PDPA s. 21",
            PdpaError::CorrectionRequestOverdue => "PDPA s. 22",
            PdpaError::NoDataProtectionOfficer => "PDPA s. 11(3)",
            PdpaError::DpoContactNotPublished => "PDPA s. 11(5)",
            PdpaError::ValidationError { .. } => "PDPA",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statute_references_are_accurate() {
        assert_eq!(PdpaError::MissingConsent.statute_reference(), "PDPA s. 13");
        assert_eq!(
            PdpaError::LateBreachNotification.statute_reference(),
            "PDPA s. 26D(1)"
        );
        assert_eq!(
            PdpaError::NoDataProtectionOfficer.statute_reference(),
            "PDPA s. 11(3)"
        );
        assert_eq!(
            PdpaError::AccessRequestOverdue.statute_reference(),
            "PDPA s. 21"
        );
    }

    #[test]
    fn error_display_includes_details() {
        let e = PdpaError::DncViolation {
            phone: "+6591234567".to_string(),
            register: "No Voice Call Register".to_string(),
        };
        let display = format!("{}", e);
        assert!(display.contains("+6591234567"));
        assert!(display.contains("No Voice Call Register"));
        assert!(display.contains("s. 43"));
    }
}
