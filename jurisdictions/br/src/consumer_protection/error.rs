//! CDC Error Types

use crate::citation::{RomanNumeral, format_cdc_citation};
use thiserror::Error;

/// CDC Error types
#[derive(Debug, Clone, Error)]
pub enum CdcError {
    /// Abusive clause detected (Art. 51)
    #[error("Cláusula abusiva detectada (Art. 51): {description}")]
    AbusiveClause {
        /// Clause description
        description: String,
        /// Specific inciso
        inciso: Option<u32>,
    },

    /// Product defect (Art. 12)
    #[error("Defeito do produto (Art. 12): {defect_type}")]
    ProductDefect {
        /// Type of defect
        defect_type: String,
        /// Product name
        product: String,
    },

    /// Service defect (Art. 14)
    #[error("Defeito do serviço (Art. 14): {defect_type}")]
    ServiceDefect {
        /// Type of defect
        defect_type: String,
        /// Service description
        service: String,
    },

    /// Misleading advertising (Art. 37)
    #[error("Publicidade enganosa (Art. 37): {description}")]
    MisleadingAdvertising {
        /// Description of misleading content
        description: String,
    },

    /// Abusive advertising (Art. 37, §2)
    #[error("Publicidade abusiva (Art. 37, §2º): {description}")]
    AbusiveAdvertising {
        /// Description of abusive content
        description: String,
    },

    /// Information inadequacy (Art. 6, III)
    #[error("Informação inadequada (Art. 6, III): {missing_info}")]
    InformationInadequacy {
        /// Missing or inadequate information
        missing_info: String,
    },

    /// Warranty violation (Art. 26)
    #[error("Violação de garantia (Art. 26): {description}")]
    WarrantyViolation {
        /// Description of violation
        description: String,
        /// Days remaining in warranty
        days_remaining: Option<u32>,
    },

    /// Withdrawal right denial (Art. 49)
    #[error("Negativa de arrependimento (Art. 49): {reason}")]
    WithdrawalDenied {
        /// Reason for denial
        reason: String,
        /// Days since purchase
        days_since_purchase: u32,
    },

    /// Excessive price (Art. 39, V)
    #[error("Preço excessivo (Art. 39, V): {description}")]
    ExcessivePrice {
        /// Price description
        description: String,
    },

    /// Tied sale (venda casada - Art. 39, I)
    #[error("Venda casada (Art. 39, I): {description}")]
    TiedSale {
        /// Description of tied products/services
        description: String,
    },

    /// Collection abuse (Art. 42)
    #[error("Cobrança abusiva (Art. 42): {description}")]
    CollectionAbuse {
        /// Description of abusive collection
        description: String,
    },

    /// Recall failure (Art. 10)
    #[error("Falha no recall (Art. 10): {description}")]
    RecallFailure {
        /// Description of recall issue
        description: String,
    },

    /// Contract interpretation error
    #[error("Erro de interpretação contratual: {description}")]
    ContractInterpretation {
        /// Description of interpretation issue
        description: String,
    },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError {
        /// Error message
        message: String,
    },
}

impl CdcError {
    /// Get the relevant CDC article citation
    pub fn citation(&self) -> String {
        match self {
            Self::AbusiveClause { inciso, .. } => {
                let roman = inciso.and_then(crate::citation::to_roman_numeral);
                format_cdc_citation(51, None, roman)
            }
            Self::ProductDefect { .. } => format_cdc_citation(12, None, None),
            Self::ServiceDefect { .. } => format_cdc_citation(14, None, None),
            Self::MisleadingAdvertising { .. } => format_cdc_citation(37, None, None),
            Self::AbusiveAdvertising { .. } => format_cdc_citation(37, Some(2), None),
            Self::InformationInadequacy { .. } => {
                format_cdc_citation(6, None, Some(RomanNumeral::III))
            }
            Self::WarrantyViolation { .. } => format_cdc_citation(26, None, None),
            Self::WithdrawalDenied { .. } => format_cdc_citation(49, None, None),
            Self::ExcessivePrice { .. } => format_cdc_citation(39, None, Some(RomanNumeral::V)),
            Self::TiedSale { .. } => format_cdc_citation(39, None, Some(RomanNumeral::I)),
            Self::CollectionAbuse { .. } => format_cdc_citation(42, None, None),
            Self::RecallFailure { .. } => format_cdc_citation(10, None, None),
            Self::ContractInterpretation { .. } => format_cdc_citation(47, None, None),
            Self::ValidationError { .. } => "CDC".to_string(),
        }
    }

    /// Get PROCON administrative penalty range (in BRL)
    pub fn procon_penalty_range(&self) -> (u64, u64) {
        match self {
            Self::AbusiveClause { .. } => (200, 3_000_000),
            Self::ProductDefect { .. } | Self::ServiceDefect { .. } => (200, 3_000_000),
            Self::MisleadingAdvertising { .. } | Self::AbusiveAdvertising { .. } => {
                (200, 10_000_000)
            }
            Self::TiedSale { .. } | Self::ExcessivePrice { .. } => (200, 3_000_000),
            Self::CollectionAbuse { .. } => (200, 3_000_000),
            Self::RecallFailure { .. } => (200, 10_000_000),
            _ => (200, 3_000_000),
        }
    }

    /// Check if violation can result in criminal penalty
    pub fn has_criminal_penalty(&self) -> bool {
        matches!(
            self,
            Self::MisleadingAdvertising { .. }
                | Self::AbusiveAdvertising { .. }
                | Self::RecallFailure { .. }
        )
    }

    /// Get criminal penalty description if applicable
    pub fn criminal_penalty(&self) -> Option<&'static str> {
        match self {
            Self::MisleadingAdvertising { .. } => {
                Some("Detenção de 3 meses a 1 ano e multa (Art. 67)")
            }
            Self::AbusiveAdvertising { .. } => {
                Some("Detenção de 3 meses a 1 ano e multa (Art. 67)")
            }
            Self::RecallFailure { .. } => Some("Detenção de 6 meses a 2 anos e multa (Art. 64)"),
            _ => None,
        }
    }

    /// Get consumer remedy
    pub fn remedy_pt(&self) -> &'static str {
        match self {
            Self::AbusiveClause { .. } => "Nulidade da cláusula, restante do contrato válido",
            Self::ProductDefect { .. } => "Substituição, restituição ou abatimento (Art. 18)",
            Self::ServiceDefect { .. } => "Reexecução, restituição ou abatimento (Art. 20)",
            Self::MisleadingAdvertising { .. } => "Contrapropaganda às expensas do infrator",
            Self::WithdrawalDenied { .. } => "Restituição imediata dos valores pagos",
            Self::WarrantyViolation { .. } => "Substituição ou restituição do valor",
            Self::TiedSale { .. } => "Nulidade da venda casada",
            Self::CollectionAbuse { .. } => "Repetição do indébito em dobro (Art. 42, §único)",
            _ => "Reparação integral dos danos",
        }
    }
}

/// Result type for CDC operations
pub type CdcResult<T> = Result<T, CdcError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abusive_clause_citation() {
        let error = CdcError::AbusiveClause {
            description: "Exclusão de responsabilidade".to_string(),
            inciso: Some(1),
        };
        let citation = error.citation();
        assert!(citation.contains("Art. 51"));
    }

    #[test]
    fn test_penalty_range() {
        let error = CdcError::MisleadingAdvertising {
            description: "Propaganda enganosa".to_string(),
        };
        let (min, max) = error.procon_penalty_range();
        assert_eq!(min, 200);
        assert_eq!(max, 10_000_000);
    }

    #[test]
    fn test_criminal_penalty() {
        let error = CdcError::RecallFailure {
            description: "Não notificou autoridades".to_string(),
        };
        assert!(error.has_criminal_penalty());
        assert!(error.criminal_penalty().is_some());
    }

    #[test]
    fn test_no_criminal_penalty() {
        let error = CdcError::AbusiveClause {
            description: "Multa excessiva".to_string(),
            inciso: None,
        };
        assert!(!error.has_criminal_penalty());
    }

    #[test]
    fn test_remedy() {
        let error = CdcError::CollectionAbuse {
            description: "Cobrou valor já pago".to_string(),
        };
        assert!(error.remedy_pt().contains("dobro"));
    }

    #[test]
    fn test_withdrawal_citation() {
        let error = CdcError::WithdrawalDenied {
            reason: "Prazo expirado".to_string(),
            days_since_purchase: 10,
        };
        let citation = error.citation();
        assert!(citation.contains("Art. 49"));
    }
}
