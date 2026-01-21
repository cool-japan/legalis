//! Error types for Indonesian Personal Data Protection (UU PDP)

use thiserror::Error;

/// Result type for PDP operations
pub type PdpResult<T> = Result<T, PdpError>;

/// Errors related to UU PDP compliance
#[derive(Debug, Error)]
pub enum PdpError {
    /// Missing legal basis for processing - Pasal 20
    #[error("Tidak ada dasar hukum pemrosesan yang valid (UU PDP Pasal 20): {description}")]
    MissingLegalBasis { description: String },

    /// Invalid consent - Pasal 20-22
    #[error("Persetujuan tidak valid (UU PDP Pasal 20-22): {description}")]
    InvalidConsent { description: String },

    /// Consent not explicit for specific data - Pasal 25
    #[error(
        "Data Pribadi Spesifik memerlukan persetujuan eksplisit (UU PDP Pasal 25): {data_type}"
    )]
    SpecificDataRequiresExplicitConsent { data_type: String },

    /// Data retention violation - Pasal 44
    #[error(
        "Pelanggaran penyimpanan data (UU PDP Pasal 44): data disimpan melebihi {retained_days} hari (batas: {limit_days} hari)"
    )]
    DataRetentionViolation { retained_days: u32, limit_days: u32 },

    /// Cross-border transfer violation - Pasal 55-56
    #[error("Transfer lintas batas tidak sah (UU PDP Pasal 55-56): {destination} - {reason}")]
    CrossBorderTransferViolation { destination: String, reason: String },

    /// Breach notification overdue - Pasal 46
    #[error(
        "Pemberitahuan pelanggaran data terlambat (UU PDP Pasal 46): {hours_elapsed} jam (batas: 72 jam)"
    )]
    BreachNotificationOverdue { hours_elapsed: i64 },

    /// Missing Data Protection Officer - Pasal 53
    #[error("Wajib menunjuk Pejabat Pelindung Data Pribadi (UU PDP Pasal 53)")]
    MissingDpo,

    /// Missing DPIA for high-risk processing - Pasal 34
    #[error(
        "Analisis dampak pelindungan data pribadi diperlukan untuk pemrosesan berisiko tinggi (UU PDP Pasal 34)"
    )]
    DpiaRequired,

    /// Data subject right violation - Pasal 5-13
    #[error(
        "Pelanggaran hak Subjek Data Pribadi (UU PDP Pasal {article}): {right} - {description}"
    )]
    DataSubjectRightViolation {
        right: String,
        article: String,
        description: String,
    },

    /// Processing purpose not specified - Pasal 21
    #[error("Tujuan pemrosesan tidak ditentukan secara jelas (UU PDP Pasal 21)")]
    PurposeNotSpecified,

    /// Processing beyond consent scope - Pasal 22
    #[error("Pemrosesan melebihi cakupan persetujuan (UU PDP Pasal 22): {description}")]
    ProcessingBeyondConsent { description: String },

    /// Child data without parental consent - Pasal 25(2)
    #[error(
        "Pemrosesan data anak memerlukan persetujuan orang tua/wali (UU PDP Pasal 25 ayat (2))"
    )]
    ChildDataWithoutParentalConsent,

    /// Security measures inadequate - Pasal 35
    #[error("Langkah keamanan tidak memadai (UU PDP Pasal 35): {description}")]
    InadequateSecurityMeasures { description: String },

    /// Language not Indonesian - Pasal 21(3)
    #[error("Persetujuan harus dalam Bahasa Indonesia (UU PDP Pasal 21 ayat (3))")]
    ConsentNotInIndonesian,
}

impl PdpError {
    /// Get the criminal sanction for this violation
    pub fn criminal_sanction(&self) -> Option<CriminalSanction> {
        match self {
            Self::CrossBorderTransferViolation { .. } => Some(CriminalSanction {
                article: "Pasal 67".to_string(),
                max_imprisonment_years: 5,
                max_fine_billion_idr: 5,
            }),
            Self::BreachNotificationOverdue { .. } => Some(CriminalSanction {
                article: "Pasal 68".to_string(),
                max_imprisonment_years: 4,
                max_fine_billion_idr: 4,
            }),
            Self::SpecificDataRequiresExplicitConsent { .. } => Some(CriminalSanction {
                article: "Pasal 67".to_string(),
                max_imprisonment_years: 5,
                max_fine_billion_idr: 5,
            }),
            Self::ProcessingBeyondConsent { .. } => Some(CriminalSanction {
                article: "Pasal 67".to_string(),
                max_imprisonment_years: 5,
                max_fine_billion_idr: 5,
            }),
            Self::ChildDataWithoutParentalConsent => Some(CriminalSanction {
                article: "Pasal 67".to_string(),
                max_imprisonment_years: 5,
                max_fine_billion_idr: 5,
            }),
            _ => None,
        }
    }

    /// Get the administrative sanction for this violation
    pub fn administrative_sanction(&self) -> AdministrativeSanction {
        match self {
            Self::MissingLegalBasis { .. }
            | Self::InvalidConsent { .. }
            | Self::PurposeNotSpecified
            | Self::ConsentNotInIndonesian => AdministrativeSanction {
                warning: true,
                suspension: true,
                deletion_order: false,
                fine_percentage: Some(2.0),
            },
            Self::DataRetentionViolation { .. }
            | Self::InadequateSecurityMeasures { .. }
            | Self::MissingDpo
            | Self::DpiaRequired => AdministrativeSanction {
                warning: true,
                suspension: true,
                deletion_order: true,
                fine_percentage: Some(2.0),
            },
            Self::BreachNotificationOverdue { .. } | Self::CrossBorderTransferViolation { .. } => {
                AdministrativeSanction {
                    warning: true,
                    suspension: true,
                    deletion_order: true,
                    fine_percentage: Some(2.0),
                }
            }
            _ => AdministrativeSanction {
                warning: true,
                suspension: false,
                deletion_order: false,
                fine_percentage: None,
            },
        }
    }
}

/// Criminal sanction under UU PDP Chapter XIV
#[derive(Debug, Clone)]
pub struct CriminalSanction {
    /// Article reference
    pub article: String,
    /// Maximum imprisonment in years
    pub max_imprisonment_years: u32,
    /// Maximum fine in billion IDR
    pub max_fine_billion_idr: u32,
}

/// Administrative sanction under UU PDP Pasal 57
#[derive(Debug, Clone)]
pub struct AdministrativeSanction {
    /// Written warning (peringatan tertulis)
    pub warning: bool,
    /// Temporary suspension of processing (penghentian sementara)
    pub suspension: bool,
    /// Data deletion order (penghapusan data)
    pub deletion_order: bool,
    /// Administrative fine as percentage of annual revenue (max 2%)
    pub fine_percentage: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages_indonesian() {
        let error = PdpError::MissingLegalBasis {
            description: "Tidak ada persetujuan".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("UU PDP Pasal 20"));
        assert!(msg.contains("dasar hukum"));
    }

    #[test]
    fn test_criminal_sanction() {
        let error = PdpError::CrossBorderTransferViolation {
            destination: "Country X".to_string(),
            reason: "No adequacy".to_string(),
        };
        let sanction = error.criminal_sanction();
        assert!(sanction.is_some());
        let s = sanction.expect("should exist");
        assert_eq!(s.max_imprisonment_years, 5);
    }

    #[test]
    fn test_administrative_sanction() {
        let error = PdpError::MissingDpo;
        let sanction = error.administrative_sanction();
        assert!(sanction.warning);
        assert!(sanction.suspension);
    }
}
