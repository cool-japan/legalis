//! # ICMS - Imposto sobre Circulação de Mercadorias e Serviços
//!
//! State VAT on goods and services (Lei Complementar 87/1996 - Kandir Law).
//!
//! ## Overview
//!
//! ICMS is Brazil's main indirect tax, levied by states on:
//! - Circulation of goods
//! - Interstate and intercity transportation
//! - Communication services
//! - Importation
//!
//! ## Rates by State (2024)
//!
//! | State | Standard Rate | Interstate |
//! |-------|---------------|------------|
//! | SP | 18% | 7% or 12% |
//! | RJ | 20% | 7% or 12% |
//! | MG | 18% | 7% or 12% |
//! | RS | 17% | 7% or 12% |

use crate::common::{BrazilianCurrency, BrazilianState};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// ICMS transaction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IcmsTransaction {
    /// Transaction value
    pub valor: BrazilianCurrency,
    /// Origin state
    pub origem: BrazilianState,
    /// Destination state
    pub destino: BrazilianState,
    /// Transaction type
    pub tipo: IcmsTransactionType,
    /// ICMS rate applied
    pub aliquota: IcmsRate,
}

/// ICMS transaction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IcmsTransactionType {
    /// Internal (within state)
    Internal,
    /// Interstate
    Interstate,
    /// Import
    Import,
    /// Export (exempt)
    Export,
}

/// ICMS rates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IcmsRate {
    /// Rate percentage
    pub percentual: u8,
    /// Whether rate is reduced
    pub reduzida: bool,
}

impl IcmsRate {
    /// Standard interstate rate for South/Southeast
    pub const INTERSTATE_12: Self = Self {
        percentual: 12,
        reduzida: false,
    };

    /// Standard interstate rate for North/Northeast/Center-West
    pub const INTERSTATE_7: Self = Self {
        percentual: 7,
        reduzida: false,
    };

    /// Get standard internal rate for state
    pub fn internal_rate(state: BrazilianState) -> Self {
        let percentual = match state {
            BrazilianState::SP | BrazilianState::MG | BrazilianState::SC => 18,
            BrazilianState::RJ | BrazilianState::CE => 20,
            BrazilianState::RS | BrazilianState::PR => 17,
            _ => 17, // Default rate
        };
        Self {
            percentual,
            reduzida: false,
        }
    }

    /// Get interstate rate based on destination
    pub fn interstate_rate(destino: BrazilianState) -> Self {
        match destino {
            BrazilianState::SP
            | BrazilianState::RJ
            | BrazilianState::MG
            | BrazilianState::RS
            | BrazilianState::PR
            | BrazilianState::SC
            | BrazilianState::ES => Self::INTERSTATE_12,
            _ => Self::INTERSTATE_7,
        }
    }
}

impl IcmsTransaction {
    /// Create a new ICMS transaction
    pub fn new(valor: BrazilianCurrency, origem: BrazilianState, destino: BrazilianState) -> Self {
        let tipo = if origem == destino {
            IcmsTransactionType::Internal
        } else {
            IcmsTransactionType::Interstate
        };

        let aliquota = if tipo == IcmsTransactionType::Internal {
            IcmsRate::internal_rate(origem)
        } else {
            IcmsRate::interstate_rate(destino)
        };

        Self {
            valor,
            origem,
            destino,
            tipo,
            aliquota,
        }
    }

    /// Calculate ICMS amount
    pub fn calculate_icms(&self) -> BrazilianCurrency {
        if matches!(self.tipo, IcmsTransactionType::Export) {
            // Exports are exempt (LC 87/1996, Art. 3, II)
            return BrazilianCurrency::from_centavos(0);
        }

        let taxa = self.aliquota.percentual as i64;
        let valor_centavos = self.valor.centavos;
        let icms_centavos = (valor_centavos * taxa) / 100;

        BrazilianCurrency::from_centavos(icms_centavos)
    }

    /// Check if differential rate (DIFAL) applies
    /// Applies to interstate consumer sales since 2016
    pub fn has_differential_rate(&self) -> bool {
        matches!(self.tipo, IcmsTransactionType::Interstate)
    }
}

/// ICMS substitution (substituição tributária)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IcmsSubstitution {
    /// Product description
    pub produto: String,
    /// Presumed margin (MVA - Margem de Valor Agregado)
    pub mva_percentual: u8,
    /// Whether subject to substitution
    pub sujeito_st: bool,
}

impl IcmsSubstitution {
    /// Calculate ICMS-ST amount
    pub fn calculate_st(
        &self,
        base_value: BrazilianCurrency,
        internal_rate: u8,
    ) -> BrazilianCurrency {
        if !self.sujeito_st {
            return BrazilianCurrency::from_centavos(0);
        }

        // ST = (Base * (1 + MVA) * Internal_Rate) - Normal_ICMS
        let base = base_value.centavos;
        let adjusted_base = (base * (100 + self.mva_percentual as i64)) / 100;
        let st_total = (adjusted_base * internal_rate as i64) / 100;
        let normal_icms = (base * internal_rate as i64) / 100;

        BrazilianCurrency::from_centavos(st_total - normal_icms)
    }
}

/// ICMS errors
#[derive(Debug, Clone, Error)]
pub enum IcmsError {
    /// Invalid rate
    #[error("Alíquota de ICMS inválida: {rate}%")]
    InvalidRate { rate: u8 },

    /// Invalid transaction
    #[error("Transação de ICMS inválida: {reason}")]
    InvalidTransaction { reason: String },

    /// Substitution error
    #[error("Erro na substituição tributária: {reason}")]
    SubstitutionError { reason: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for ICMS operations
pub type IcmsResult<T> = Result<T, IcmsError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_transaction() {
        let transaction = IcmsTransaction::new(
            BrazilianCurrency::from_reais(1000),
            BrazilianState::SP,
            BrazilianState::SP,
        );
        assert_eq!(transaction.tipo, IcmsTransactionType::Internal);
        assert_eq!(transaction.aliquota.percentual, 18);

        let icms = transaction.calculate_icms();
        assert_eq!(icms.reais(), 180); // 18% of 1000
    }

    #[test]
    fn test_interstate_transaction() {
        let transaction = IcmsTransaction::new(
            BrazilianCurrency::from_reais(1000),
            BrazilianState::SP,
            BrazilianState::BA,
        );
        assert_eq!(transaction.tipo, IcmsTransactionType::Interstate);
        assert!(transaction.has_differential_rate());
    }

    #[test]
    fn test_interstate_rates() {
        let rate_sp = IcmsRate::interstate_rate(BrazilianState::SP);
        assert_eq!(rate_sp.percentual, 12);

        let rate_ba = IcmsRate::interstate_rate(BrazilianState::BA);
        assert_eq!(rate_ba.percentual, 7);
    }

    #[test]
    fn test_icms_substitution() {
        let st = IcmsSubstitution {
            produto: "Bebida".to_string(),
            mva_percentual: 30,
            sujeito_st: true,
        };

        let st_value = st.calculate_st(BrazilianCurrency::from_reais(100), 18);
        // Base: 100, Adjusted: 130, ST_Total: 23.40, Normal: 18, ST: 5.40
        assert_eq!(st_value.reais(), 5);
    }
}
