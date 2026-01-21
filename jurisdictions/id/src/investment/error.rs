//! Error types for Indonesian Investment Law

use thiserror::Error;

/// Result type for investment operations
pub type InvestmentResult<T> = Result<T, InvestmentError>;

/// Errors related to investment law compliance
#[derive(Debug, Error)]
pub enum InvestmentError {
    /// Sector closed to foreign investment
    #[error("Sektor tertutup untuk penanaman modal asing (DNI): {sector}")]
    SectorClosed { sector: String },

    /// Foreign ownership exceeds limit
    #[error("Kepemilikan asing melebihi batas (DNI): {actual}% (maksimum {limit}% untuk {sector})")]
    OwnershipExceedsLimit {
        sector: String,
        actual: u32,
        limit: u32,
    },

    /// Minimum capital not met - PP 5/2021
    #[error("Modal minimum tidak terpenuhi (PP 5/2021): Rp {actual} (minimum Rp {required})")]
    MinimumCapitalNotMet { actual: i64, required: i64 },

    /// Local partnership required
    #[error("Kemitraan dengan UMKM diperlukan untuk sektor {sector} (PP 7/2021)")]
    PartnershipRequired { sector: String },

    /// Missing NIB
    #[error("NIB (Nomor Induk Berusaha) diperlukan untuk kegiatan usaha (OSS)")]
    MissingNib,

    /// Invalid KBLI code
    #[error("Kode KBLI tidak valid: {code}")]
    InvalidKbliCode { code: String },

    /// License type mismatch for risk level
    #[error("Jenis izin tidak sesuai tingkat risiko: diperlukan {required}, diperoleh {actual}")]
    LicenseTypeMismatch { required: String, actual: String },

    /// Missing certificate for risk level
    #[error("Sertifikat standar diperlukan untuk risiko {risk_level}")]
    MissingCertificate { risk_level: String },

    /// Foreign worker quota exceeded
    #[error("Kuota TKA melebihi batas: {actual} (maksimum {limit} untuk perusahaan ini)")]
    ForeignWorkerQuotaExceeded { actual: u32, limit: u32 },

    /// MSME sector restriction
    #[error("Sektor dicadangkan untuk UMKM (Usaha Mikro, Kecil, Menengah): {sector}")]
    MsmeReserved { sector: String },

    /// SEZ requirement not met
    #[error("Persyaratan KEK (Kawasan Ekonomi Khusus) tidak terpenuhi: {requirement}")]
    SezRequirementNotMet { requirement: String },

    /// Export obligation not met
    #[error("Kewajiban ekspor tidak terpenuhi: {percentage}% ekspor diperlukan")]
    ExportObligationNotMet { percentage: u32 },
}

impl InvestmentError {
    /// Get recommended action for the error
    pub fn recommended_action(&self) -> &'static str {
        match self {
            Self::SectorClosed { .. } => {
                "Pertimbangkan sektor alternatif atau investasi melalui perusahaan lokal"
            }
            Self::OwnershipExceedsLimit { .. } => "Kurangi kepemilikan asing atau cari mitra lokal",
            Self::MinimumCapitalNotMet { .. } => {
                "Tingkatkan modal investasi sesuai ketentuan PP 5/2021"
            }
            Self::PartnershipRequired { .. } => {
                "Bentuk kemitraan dengan UMKM sesuai ketentuan PP 7/2021"
            }
            Self::MissingNib => "Ajukan NIB melalui sistem OSS (oss.go.id)",
            Self::InvalidKbliCode { .. } => "Periksa kode KBLI yang benar di oss.go.id",
            Self::LicenseTypeMismatch { .. } => "Ajukan izin sesuai tingkat risiko usaha",
            Self::MissingCertificate { .. } => "Ajukan sertifikat standar melalui OSS",
            Self::ForeignWorkerQuotaExceeded { .. } => "Kurangi TKA atau ajukan RPTKA baru",
            Self::MsmeReserved { .. } => "Pertimbangkan kemitraan atau sektor alternatif",
            Self::SezRequirementNotMet { .. } => {
                "Lengkapi persyaratan KEK atau investasi di luar KEK"
            }
            Self::ExportObligationNotMet { .. } => "Tingkatkan ekspor sesuai kewajiban",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        let error = InvestmentError::OwnershipExceedsLimit {
            sector: "Retail".to_string(),
            actual: 100,
            limit: 67,
        };
        let msg = format!("{}", error);
        assert!(msg.contains("100%"));
        assert!(msg.contains("67%"));
    }

    #[test]
    fn test_recommended_action() {
        let error = InvestmentError::MissingNib;
        let action = error.recommended_action();
        assert!(action.contains("OSS"));
    }
}
