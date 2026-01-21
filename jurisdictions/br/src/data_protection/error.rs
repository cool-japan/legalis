//! LGPD Error Types

use crate::citation::{RomanNumeral, format_lgpd_citation};
use thiserror::Error;

/// LGPD Error types
#[derive(Debug, Clone, Error)]
pub enum LgpdError {
    /// No legal basis for processing (Art. 7)
    #[error("Tratamento sem base legal (Art. 7): {description}")]
    NoLegalBasis {
        /// Description
        description: String,
    },

    /// Invalid consent (Art. 8)
    #[error("Consentimento inválido (Art. 8): {reason}")]
    InvalidConsent {
        /// Reason
        reason: String,
    },

    /// Sensitive data violation (Art. 11)
    #[error("Violação de dados sensíveis (Art. 11): {description}")]
    SensitiveDataViolation {
        /// Description
        description: String,
    },

    /// Children's data violation (Art. 14)
    #[error("Violação de dados de crianças (Art. 14): {description}")]
    ChildrensDataViolation {
        /// Description
        description: String,
    },

    /// Data subject right denied (Art. 18)
    #[error("Direito do titular negado (Art. 18): {right}")]
    RightDenied {
        /// Right denied
        right: String,
    },

    /// Security incident not reported (Art. 48)
    #[error("Incidente de segurança não reportado (Art. 48): {description}")]
    IncidentNotReported {
        /// Description
        description: String,
    },

    /// International transfer violation (Art. 33)
    #[error("Transferência internacional irregular (Art. 33): {destination}")]
    InvalidInternationalTransfer {
        /// Destination country
        destination: String,
    },

    /// No DPO appointed (Art. 41)
    #[error("Encarregado (DPO) não nomeado (Art. 41)")]
    NoDpoAppointed,

    /// Excessive data collection (Art. 6, III)
    #[error("Coleta excessiva de dados (Art. 6, III): {description}")]
    ExcessiveDataCollection {
        /// Description
        description: String,
    },

    /// Purpose limitation violation (Art. 6, II)
    #[error("Desvio de finalidade (Art. 6, II): {description}")]
    PurposeLimitationViolation {
        /// Description
        description: String,
    },

    /// Data retention violation (Art. 16)
    #[error("Retenção indevida de dados (Art. 16): {description}")]
    RetentionViolation {
        /// Description
        description: String,
    },

    /// Automated decision without review (Art. 20)
    #[error("Decisão automatizada sem revisão (Art. 20)")]
    AutomatedDecisionNoReview,

    /// Validation error
    #[error("Erro de validação LGPD: {message}")]
    ValidationError {
        /// Message
        message: String,
    },
}

impl LgpdError {
    /// Get LGPD citation
    pub fn citation(&self) -> String {
        match self {
            Self::NoLegalBasis { .. } => format_lgpd_citation(7, None, None),
            Self::InvalidConsent { .. } => format_lgpd_citation(8, None, None),
            Self::SensitiveDataViolation { .. } => format_lgpd_citation(11, None, None),
            Self::ChildrensDataViolation { .. } => format_lgpd_citation(14, None, None),
            Self::RightDenied { .. } => format_lgpd_citation(18, None, None),
            Self::IncidentNotReported { .. } => format_lgpd_citation(48, None, None),
            Self::InvalidInternationalTransfer { .. } => format_lgpd_citation(33, None, None),
            Self::NoDpoAppointed => format_lgpd_citation(41, None, None),
            Self::ExcessiveDataCollection { .. } => {
                format_lgpd_citation(6, None, Some(RomanNumeral::III))
            }
            Self::PurposeLimitationViolation { .. } => {
                format_lgpd_citation(6, None, Some(RomanNumeral::II))
            }
            Self::RetentionViolation { .. } => format_lgpd_citation(16, None, None),
            Self::AutomatedDecisionNoReview => format_lgpd_citation(20, None, None),
            Self::ValidationError { .. } => "LGPD".to_string(),
        }
    }

    /// Get ANPD penalty type (Art. 52)
    pub fn penalty_type(&self) -> &'static str {
        match self {
            Self::IncidentNotReported { .. }
            | Self::SensitiveDataViolation { .. }
            | Self::ChildrensDataViolation { .. } => "Multa simples ou diária",
            Self::NoDpoAppointed | Self::InvalidConsent { .. } => "Advertência com prazo",
            Self::InvalidInternationalTransfer { .. } => "Bloqueio dos dados + multa",
            _ => "Advertência ou multa simples",
        }
    }

    /// Calculate maximum fine (2% of revenue, max R$ 50M)
    pub fn max_fine_per_violation() -> i64 {
        5_000_000_000 // R$ 50,000,000.00 in centavos
    }

    /// Calculate fine based on revenue
    pub fn calculate_fine(annual_revenue_centavos: i64) -> i64 {
        let two_percent = (annual_revenue_centavos * 2) / 100;
        two_percent.min(Self::max_fine_per_violation())
    }

    /// Get recommended remediation
    pub fn remediation_pt(&self) -> &'static str {
        match self {
            Self::NoLegalBasis { .. } => "Identificar e documentar base legal apropriada",
            Self::InvalidConsent { .. } => {
                "Obter consentimento válido, livre, específico e informado"
            }
            Self::SensitiveDataViolation { .. } => {
                "Implementar salvaguardas adicionais para dados sensíveis"
            }
            Self::ChildrensDataViolation { .. } => "Obter consentimento parental específico",
            Self::RightDenied { .. } => "Implementar processo de atendimento a direitos do titular",
            Self::IncidentNotReported { .. } => "Comunicar ANPD e titulares em prazo razoável",
            Self::InvalidInternationalTransfer { .. } => {
                "Garantir adequação do país ou cláusulas contratuais padrão"
            }
            Self::NoDpoAppointed => "Nomear Encarregado (DPO) e divulgar contato",
            Self::ExcessiveDataCollection { .. } => "Limitar coleta ao mínimo necessário",
            Self::PurposeLimitationViolation { .. } => {
                "Tratar dados apenas para finalidade original"
            }
            Self::RetentionViolation { .. } => "Eliminar dados após término da finalidade",
            Self::AutomatedDecisionNoReview => {
                "Garantir direito de revisão humana de decisões automatizadas"
            }
            Self::ValidationError { .. } => "Corrigir o erro identificado",
        }
    }
}

/// Result type for LGPD operations
pub type LgpdResult<T> = Result<T, LgpdError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_legal_basis_citation() {
        let error = LgpdError::NoLegalBasis {
            description: "Tratamento sem base".to_string(),
        };
        let citation = error.citation();
        assert!(citation.contains("Art. 7"));
    }

    #[test]
    #[allow(clippy::inconsistent_digit_grouping)] // Brazilian Real with centavos: R$ 100M = 100_000_000_00
    fn test_calculate_fine_small() {
        let revenue = 100_000_000_00_i64; // R$ 100M (in centavos)
        let fine = LgpdError::calculate_fine(revenue);
        assert_eq!(fine, 2_000_000_00); // 2% = R$ 2M
    }

    #[test]
    #[allow(clippy::inconsistent_digit_grouping)] // Brazilian Real with centavos: R$ 10B = 10_000_000_000_00
    fn test_calculate_fine_large() {
        let revenue = 10_000_000_000_00_i64; // R$ 10B (in centavos)
        let fine = LgpdError::calculate_fine(revenue);
        assert_eq!(fine, LgpdError::max_fine_per_violation()); // Capped at R$ 50M
    }

    #[test]
    fn test_penalty_type() {
        let error = LgpdError::IncidentNotReported {
            description: "Não reportado".to_string(),
        };
        assert!(error.penalty_type().contains("Multa"));

        let error = LgpdError::NoDpoAppointed;
        assert!(error.penalty_type().contains("Advertência"));
    }

    #[test]
    fn test_remediation() {
        let error = LgpdError::InvalidConsent {
            reason: "Vício de consentimento".to_string(),
        };
        assert!(error.remediation_pt().contains("consentimento"));
    }
}
