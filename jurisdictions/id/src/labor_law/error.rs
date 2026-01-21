//! Error types for Indonesian Labor Law

use thiserror::Error;

/// Result type for labor law operations
pub type LaborResult<T> = Result<T, LaborError>;

/// Errors related to UU Ketenagakerjaan compliance
#[derive(Debug, Error)]
pub enum LaborError {
    /// Working hours exceeded - Pasal 77
    #[error(
        "Jam kerja melebihi batas (UU Ketenagakerjaan Pasal 77): {hours} jam/minggu (maksimum 40 jam)"
    )]
    WorkingHoursExceeded { hours: u32 },

    /// Overtime exceeded - PP 35/2021 Pasal 26
    #[error("Lembur melebihi batas (PP 35/2021 Pasal 26): {hours} jam/minggu (maksimum 18 jam)")]
    OvertimeExceeded { hours: u32 },

    /// Minimum wage violation - Pasal 88
    #[error(
        "Upah di bawah UMP/UMK (UU Ketenagakerjaan Pasal 88): Rp {actual} (minimum Rp {minimum})"
    )]
    MinimumWageViolation { actual: i64, minimum: i64 },

    /// Invalid PKWT duration - Pasal 59 as amended by Omnibus Law
    #[error("Durasi PKWT tidak valid (Omnibus Law): {months} bulan (maksimum 60 bulan)")]
    InvalidPkwtDuration { months: u32 },

    /// PKWT for permanent work - Pasal 59
    #[error("PKWT tidak boleh untuk pekerjaan tetap (UU Ketenagakerjaan Pasal 59)")]
    PkwtForPermanentWork,

    /// Probation in PKWT - Pasal 58
    #[error("PKWT tidak boleh memiliki masa percobaan (UU Ketenagakerjaan Pasal 58)")]
    ProbationInPkwt,

    /// Contract not in Indonesian - Pasal 57
    #[error("Perjanjian kerja harus dalam Bahasa Indonesia (UU Ketenagakerjaan Pasal 57)")]
    ContractNotInIndonesian,

    /// Contract not written for PKWT - Pasal 57
    #[error("PKWT harus dibuat secara tertulis (UU Ketenagakerjaan Pasal 57)")]
    PkwtNotWritten,

    /// Child labor violation - Pasal 68-75
    #[error("Pelanggaran ketentuan pekerja anak (UU Ketenagakerjaan Pasal 68-75): {description}")]
    ChildLaborViolation { description: String },

    /// Discrimination - Pasal 5-6
    #[error("Diskriminasi dalam ketenagakerjaan (UU Ketenagakerjaan Pasal 5-6): {description}")]
    Discrimination { description: String },

    /// Improper termination - Pasal 151-172
    #[error(
        "Pemutusan hubungan kerja tidak sesuai prosedur (UU Ketenagakerjaan Pasal 151-172): {description}"
    )]
    ImproperTermination { description: String },

    /// Missing BPJS registration - UU BPJS
    #[error("Wajib mendaftarkan pekerja ke BPJS (UU BPJS): {description}")]
    MissingBpjs { description: String },

    /// Rest period violation - Pasal 79
    #[error("Pelanggaran waktu istirahat (UU Ketenagakerjaan Pasal 79): {description}")]
    RestPeriodViolation { description: String },

    /// Leave entitlement violation - Pasal 79-84
    #[error("Pelanggaran hak cuti (UU Ketenagakerjaan Pasal 79-84): {leave_type} - {description}")]
    LeaveViolation {
        leave_type: String,
        description: String,
    },

    /// Outsourcing violation - Pasal 64-66
    #[error("Pelanggaran ketentuan outsourcing (UU Ketenagakerjaan Pasal 64-66): {description}")]
    OutsourcingViolation { description: String },
}

impl LaborError {
    /// Get the administrative sanction
    pub fn administrative_sanction(&self) -> AdminSanction {
        match self {
            Self::MinimumWageViolation { .. } => AdminSanction {
                fine_range: (1_000_000, 100_000_000),
                business_suspension: true,
            },
            Self::WorkingHoursExceeded { .. } | Self::OvertimeExceeded { .. } => AdminSanction {
                fine_range: (5_000_000, 50_000_000),
                business_suspension: false,
            },
            Self::ChildLaborViolation { .. } => AdminSanction {
                fine_range: (10_000_000, 500_000_000),
                business_suspension: true,
            },
            Self::MissingBpjs { .. } => AdminSanction {
                fine_range: (1_000_000, 50_000_000),
                business_suspension: false,
            },
            _ => AdminSanction {
                fine_range: (1_000_000, 50_000_000),
                business_suspension: false,
            },
        }
    }

    /// Check if violation may result in criminal sanction
    pub fn may_result_in_criminal(&self) -> bool {
        matches!(
            self,
            Self::ChildLaborViolation { .. }
                | Self::MinimumWageViolation { .. }
                | Self::Discrimination { .. }
        )
    }
}

/// Administrative sanction details
#[derive(Debug, Clone)]
pub struct AdminSanction {
    /// Fine range in Rupiah (min, max)
    pub fine_range: (i64, i64),
    /// Whether business may be suspended
    pub business_suspension: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        let error = LaborError::MinimumWageViolation {
            actual: 3_000_000,
            minimum: 5_000_000,
        };
        let msg = format!("{}", error);
        assert!(msg.contains("UU Ketenagakerjaan Pasal 88"));
        assert!(msg.contains("3.000.000") || msg.contains("3000000"));
    }

    #[test]
    fn test_child_labor_criminal() {
        let error = LaborError::ChildLaborViolation {
            description: "Under 15".to_string(),
        };
        assert!(error.may_result_in_criminal());
    }

    #[test]
    fn test_admin_sanction() {
        let error = LaborError::MissingBpjs {
            description: "Not registered".to_string(),
        };
        let sanction = error.administrative_sanction();
        assert!(!sanction.business_suspension);
    }
}
