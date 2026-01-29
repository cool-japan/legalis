//! # CVM - Comissão de Valores Mobiliários
//!
//! Brazilian Securities and Exchange Commission (Lei nº 6.385/1976).

use crate::common::BrazilianCurrency;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// CVM regulatory regime
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CvmRegulation {
    /// Regulated entity
    pub entidade: String,
    /// Registration category
    pub categoria: RegistrationCategory,
    /// Registration date
    pub data_registro: NaiveDate,
    /// Whether registration is active
    pub ativo: bool,
}

/// CVM registration categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistrationCategory {
    /// Category A: Public offerings
    CategoryA,
    /// Category B: Listed companies
    CategoryB,
    /// Investment funds
    InvestmentFunds,
    /// Securities intermediaries (brokers/dealers)
    Intermediaries,
    /// Portfolio administrators
    PortfolioAdministrators,
    /// Auditors
    Auditors,
}

/// Securities (valores mobiliários) - Art. 2
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Security {
    /// Security type
    pub tipo: SecurityType,
    /// Issuer
    pub emissor: String,
    /// Issue date
    pub data_emissao: NaiveDate,
    /// Value
    pub valor: BrazilianCurrency,
}

/// Security types (Art. 2)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityType {
    /// Shares (ações)
    Shares,
    /// Debentures (debêntures)
    Debentures,
    /// Commercial paper (notas promissórias)
    CommercialPaper,
    /// Investment fund quotas (quotas de fundos)
    FundQuotas,
    /// Derivatives contracts
    Derivatives,
    /// Real estate receivables certificates (CRI)
    CRI,
    /// Agribusiness receivables certificates (CRA)
    CRA,
}

/// Public offering (oferta pública)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicOffering {
    /// Offering type
    pub tipo: OfferingType,
    /// Issuer
    pub emissor: String,
    /// Securities offered
    pub valores_mobiliarios: Vec<Security>,
    /// Total offering value
    pub valor_total: BrazilianCurrency,
    /// Whether CVM registered
    pub registrada_cvm: bool,
}

/// Public offering types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfferingType {
    /// Initial public offering (IPO)
    IPO,
    /// Follow-on offering
    FollowOn,
    /// Debentures offering
    Debentures,
    /// Restricted offering (476 instruction)
    Restricted476,
}

impl PublicOffering {
    /// Check if offering requires CVM registration (Art. 19)
    /// Restricted offerings under Instruction 476 may be exempt
    pub fn requires_cvm_registration(&self) -> bool {
        !matches!(self.tipo, OfferingType::Restricted476)
    }
}

/// Market manipulation (Art. 27-D)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketAbuse {
    /// Insider trading (uso de informação privilegiada)
    InsiderTrading {
        /// Person with inside information
        person: String,
        /// Information used
        informacao: String,
    },
    /// Market manipulation (manipulação de mercado)
    MarketManipulation {
        /// Description of manipulation
        descricao: String,
    },
    /// Fraud (fraude)
    Fraud {
        /// Description of fraud
        descricao: String,
    },
}

/// CVM penalties (Art. 11)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CvmPenalty {
    /// Penalty type
    pub tipo: PenaltyType,
    /// Amount (if fine)
    pub valor: Option<BrazilianCurrency>,
    /// Description
    pub descricao: String,
}

/// CVM penalty types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PenaltyType {
    /// Warning (advertência)
    Warning,
    /// Fine (multa)
    Fine,
    /// Suspension (suspensão)
    Suspension,
    /// Disqualification (inabilitação temporária)
    Disqualification,
}

impl CvmPenalty {
    /// Get maximum fine amount (Art. 11, II)
    /// Up to R$ 500,000, 50% of benefit, or 3x damage
    pub fn maximum_fine() -> BrazilianCurrency {
        BrazilianCurrency::from_reais(500000)
    }

    /// Create fine penalty
    pub fn fine(valor: BrazilianCurrency, descricao: impl Into<String>) -> Self {
        Self {
            tipo: PenaltyType::Fine,
            valor: Some(valor),
            descricao: descricao.into(),
        }
    }
}

/// CVM errors
#[derive(Debug, Clone, Error)]
pub enum CvmError {
    /// Unregistered public offering
    #[error("Oferta pública não registrada na CVM (Art. 19)")]
    UnregisteredOffering,

    /// Market abuse
    #[error("Abuso de mercado (Art. 27-D): {abuse:?}")]
    MarketAbuse { abuse: MarketAbuse },

    /// Invalid registration
    #[error("Registro inválido na CVM: {reason}")]
    InvalidRegistration { reason: String },

    /// Disclosure violation
    #[error("Violação de dever de informação (Art. 22): {description}")]
    DisclosureViolation { description: String },

    /// Penalty error
    #[error("Erro na aplicação de penalidade: {reason}")]
    PenaltyError { reason: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for CVM operations
pub type CvmResult<T> = Result<T, CvmError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_offering() {
        let offering = PublicOffering {
            tipo: OfferingType::IPO,
            emissor: "ACME S.A.".to_string(),
            valores_mobiliarios: Vec::new(),
            valor_total: BrazilianCurrency::from_reais(100000000),
            registrada_cvm: true,
        };
        assert!(offering.requires_cvm_registration());
    }

    #[test]
    fn test_restricted_offering() {
        let offering = PublicOffering {
            tipo: OfferingType::Restricted476,
            emissor: "XYZ S.A.".to_string(),
            valores_mobiliarios: Vec::new(),
            valor_total: BrazilianCurrency::from_reais(10000000),
            registrada_cvm: false,
        };
        assert!(!offering.requires_cvm_registration());
    }

    #[test]
    fn test_penalty_maximum() {
        let max = CvmPenalty::maximum_fine();
        assert_eq!(max.reais(), 500000);
    }

    #[test]
    fn test_fine_penalty() {
        let penalty = CvmPenalty::fine(
            BrazilianCurrency::from_reais(100000),
            "Violação de informação privilegiada",
        );
        assert_eq!(penalty.tipo, PenaltyType::Fine);
        assert!(penalty.valor.is_some());
    }
}
