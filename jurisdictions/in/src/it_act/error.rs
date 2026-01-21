//! Information Technology Act 2000 Error Types
//!
//! Error types for cyber law compliance

use crate::citation::{Citation, cite};
use thiserror::Error;

/// IT Act 2000 errors
#[derive(Debug, Clone, Error)]
pub enum ItActError {
    // Digital signature errors
    /// Invalid digital signature (Section 3)
    #[error("IT Act Section 3: Invalid digital signature")]
    InvalidDigitalSignature,

    /// Expired certificate (Section 35)
    #[error("IT Act Section 35: Digital signature certificate has expired")]
    ExpiredCertificate,

    /// Revoked certificate (Section 38)
    #[error("IT Act Section 38: Digital signature certificate has been revoked")]
    RevokedCertificate,

    /// Suspended certificate (Section 37)
    #[error("IT Act Section 37: Digital signature certificate has been suspended")]
    SuspendedCertificate,

    // Cyber crime errors
    /// Tampering with source code (Section 65)
    #[error("IT Act Section 65: Tampering with computer source documents")]
    TamperingSourceCode,

    /// Hacking (Section 66)
    #[error("IT Act Section 66: Computer related offence (hacking)")]
    Hacking,

    /// Identity theft (Section 66C)
    #[error("IT Act Section 66C: Identity theft using computer resource")]
    IdentityTheft,

    /// Cheating by personation (Section 66D)
    #[error("IT Act Section 66D: Cheating by personation using computer resource")]
    CheatingByPersonation,

    /// Privacy violation (Section 66E)
    #[error("IT Act Section 66E: Violation of privacy")]
    PrivacyViolation,

    /// Cyber terrorism (Section 66F)
    #[error("IT Act Section 66F: Cyber terrorism")]
    CyberTerrorism,

    /// Obscene material (Section 67)
    #[error("IT Act Section 67: Publishing or transmitting obscene material")]
    ObsceneMaterial,

    /// Sexually explicit material (Section 67A)
    #[error("IT Act Section 67A: Publishing or transmitting sexually explicit material")]
    SexuallyExplicitMaterial,

    /// Child pornography (Section 67B)
    #[error(
        "IT Act Section 67B: Publishing or transmitting material depicting children in sexually explicit act"
    )]
    ChildPornography,

    // Computer offence errors (Section 43)
    /// Unauthorized access (Section 43(a))
    #[error("IT Act Section 43(a): Unauthorized access to computer system")]
    UnauthorizedAccess,

    /// Unauthorized download (Section 43(b))
    #[error("IT Act Section 43(b): Unauthorized downloading or copying of data")]
    UnauthorizedDownload,

    /// Introducing malware (Section 43(c))
    #[error("IT Act Section 43(c): Introducing computer contaminant or virus")]
    IntroducingMalware,

    /// Damaging computer system (Section 43(d))
    #[error("IT Act Section 43(d): Damaging computer or computer system")]
    DamagingSystem,

    /// Disrupting service (Section 43(e))
    #[error("IT Act Section 43(e): Disrupting or denying access to computer")]
    DisruptingService,

    /// Denying access (Section 43(f))
    #[error("IT Act Section 43(f): Denying access to authorized person")]
    DenyingAccess,

    /// Destroying evidence (Section 43(i))
    #[error("IT Act Section 43(i): Destroying, deleting or altering computer source code")]
    DestroyingEvidence,

    /// Breach of confidentiality (Section 43(j))
    #[error("IT Act Section 43(j): Breach of confidentiality and privacy")]
    BreachOfConfidentiality,

    // Data protection errors
    /// Failure to protect data (Section 43A)
    #[error("IT Act Section 43A: Failure to protect sensitive personal data")]
    FailureToProtectData { details: String },

    /// Unlawful disclosure of personal data
    #[error("IT Act Section 72A: Disclosure of personal information in breach of contract")]
    UnlawfulDisclosure,

    // Intermediary errors
    /// Safe harbor not available (Section 79)
    #[error("IT Act Section 79: Intermediary safe harbor conditions not met")]
    NoSafeHarbor { reason: String },

    /// Due diligence failure
    #[error("IT Act Section 79: Intermediary failed to exercise due diligence")]
    DueDiligenceFailure,

    /// Takedown non-compliance
    #[error("IT Act Section 79: Failed to remove unlawful content within prescribed time")]
    TakedownNonCompliance,

    /// SSMI compliance failure (IT Rules 2021)
    #[error("IT Rules 2021: Significant Social Media Intermediary compliance failure")]
    SsmiComplianceFailure { requirement: String },

    // Government direction errors
    /// Non-compliance with interception direction (Section 69)
    #[error("IT Act Section 69: Non-compliance with interception direction")]
    InterceptionNonCompliance,

    /// Non-compliance with blocking direction (Section 69A)
    #[error("IT Act Section 69A: Non-compliance with blocking direction")]
    BlockingNonCompliance,

    /// Non-compliance with monitoring direction (Section 69B)
    #[error("IT Act Section 69B: Non-compliance with monitoring/decryption direction")]
    MonitoringNonCompliance,

    // Certifying Authority errors
    /// CA license violation (Section 21)
    #[error("IT Act Section 21: Certifying Authority license violation")]
    CaLicenseViolation,

    /// False certificate (Section 73)
    #[error("IT Act Section 73: Publishing false digital signature certificate")]
    FalseCertificate,

    // E-commerce errors
    /// E-commerce rules violation
    #[error("Consumer Protection (E-Commerce) Rules 2020: {violation}")]
    EcommerceViolation { violation: String },

    // General errors
    /// Validation error
    #[error("IT Act validation error: {message}")]
    ValidationError { message: String },
}

impl ItActError {
    /// Get relevant citation
    pub fn citation(&self) -> Option<Citation> {
        match self {
            Self::InvalidDigitalSignature => Some(cite::it_act(3)),
            Self::ExpiredCertificate => Some(cite::it_act(35)),
            Self::RevokedCertificate => Some(cite::it_act(38)),
            Self::SuspendedCertificate => Some(cite::it_act(37)),
            Self::TamperingSourceCode => Some(cite::it_act(65)),
            Self::Hacking => Some(cite::it_act(66)),
            Self::IdentityTheft => Some(cite::it_act(66)),
            Self::CheatingByPersonation => Some(cite::it_act(66)),
            Self::PrivacyViolation => Some(cite::it_act(66)),
            Self::CyberTerrorism => Some(cite::it_act(66)),
            Self::ObsceneMaterial => Some(cite::it_act(67)),
            Self::SexuallyExplicitMaterial => Some(cite::it_act(67)),
            Self::ChildPornography => Some(cite::it_act(67)),
            Self::UnauthorizedAccess
            | Self::UnauthorizedDownload
            | Self::IntroducingMalware
            | Self::DamagingSystem
            | Self::DisruptingService
            | Self::DenyingAccess
            | Self::DestroyingEvidence
            | Self::BreachOfConfidentiality => Some(cite::it_act(43)),
            Self::FailureToProtectData { .. } => Some(cite::it_act(43)),
            Self::UnlawfulDisclosure => Some(cite::it_act(72)),
            Self::NoSafeHarbor { .. } | Self::DueDiligenceFailure | Self::TakedownNonCompliance => {
                Some(cite::it_act(79))
            }
            Self::InterceptionNonCompliance => Some(cite::it_act(69)),
            Self::BlockingNonCompliance => Some(cite::it_act(69)),
            Self::MonitoringNonCompliance => Some(cite::it_act(69)),
            Self::CaLicenseViolation => Some(cite::it_act(21)),
            Self::FalseCertificate => Some(cite::it_act(73)),
            Self::SsmiComplianceFailure { .. }
            | Self::EcommerceViolation { .. }
            | Self::ValidationError { .. } => None,
        }
    }

    /// Get penalty/compensation info
    pub fn penalty_info(&self) -> PenaltyInfo {
        match self {
            Self::TamperingSourceCode => PenaltyInfo {
                imprisonment_years: Some(3),
                fine_rupees: Some(200_000),
                compensation: None,
                bail_status: "Bailable",
            },
            Self::Hacking => PenaltyInfo {
                imprisonment_years: Some(3),
                fine_rupees: Some(500_000),
                compensation: None,
                bail_status: "Bailable",
            },
            Self::CyberTerrorism => PenaltyInfo {
                imprisonment_years: None, // Life
                fine_rupees: None,
                compensation: None,
                bail_status: "Non-bailable",
            },
            Self::ChildPornography => PenaltyInfo {
                imprisonment_years: Some(7),
                fine_rupees: Some(1_000_000),
                compensation: None,
                bail_status: "Non-bailable",
            },
            Self::UnauthorizedAccess
            | Self::UnauthorizedDownload
            | Self::IntroducingMalware
            | Self::DamagingSystem => PenaltyInfo {
                imprisonment_years: None,
                fine_rupees: None,
                compensation: Some("Compensation as determined by Adjudicating Officer"),
                bail_status: "N/A (Civil)",
            },
            Self::FailureToProtectData { .. } => PenaltyInfo {
                imprisonment_years: None,
                fine_rupees: None,
                compensation: Some("Compensation to affected persons"),
                bail_status: "N/A (Civil)",
            },
            _ => PenaltyInfo {
                imprisonment_years: Some(3),
                fine_rupees: Some(500_000),
                compensation: None,
                bail_status: "Bailable",
            },
        }
    }

    /// Check if criminal offence
    pub fn is_criminal_offence(&self) -> bool {
        matches!(
            self,
            Self::TamperingSourceCode
                | Self::Hacking
                | Self::IdentityTheft
                | Self::CheatingByPersonation
                | Self::PrivacyViolation
                | Self::CyberTerrorism
                | Self::ObsceneMaterial
                | Self::SexuallyExplicitMaterial
                | Self::ChildPornography
                | Self::FalseCertificate
        )
    }
}

/// Penalty information
#[derive(Debug, Clone)]
pub struct PenaltyInfo {
    /// Maximum imprisonment in years (None = life)
    pub imprisonment_years: Option<u32>,
    /// Maximum fine in rupees
    pub fine_rupees: Option<u64>,
    /// Compensation note
    pub compensation: Option<&'static str>,
    /// Bail status
    pub bail_status: &'static str,
}

/// Result type for IT Act operations
pub type ItActResult<T> = Result<T, ItActError>;

/// IT compliance report
#[derive(Debug, Clone, Default)]
pub struct ItComplianceReport {
    /// Overall compliance
    pub compliant: bool,
    /// Violations
    pub violations: Vec<ItActError>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_citations() {
        let error = ItActError::Hacking;
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.number, 66);

        let error = ItActError::UnauthorizedAccess;
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.number, 43);
    }

    #[test]
    fn test_criminal_offence() {
        assert!(ItActError::CyberTerrorism.is_criminal_offence());
        assert!(ItActError::ChildPornography.is_criminal_offence());
        assert!(!ItActError::UnauthorizedAccess.is_criminal_offence());
    }

    #[test]
    fn test_cyber_terrorism_penalty() {
        let penalty = ItActError::CyberTerrorism.penalty_info();
        assert!(penalty.imprisonment_years.is_none()); // Life imprisonment
        assert_eq!(penalty.bail_status, "Non-bailable");
    }
}
