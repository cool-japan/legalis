//! # ISS - Imposto sobre Serviços
//!
//! Municipal service tax (Lei Complementar 116/2003).
//!
//! ## Overview
//!
//! ISS is levied by municipalities on services listed in LC 116/2003.
//!
//! ## Rates
//!
//! - Minimum: 2%
//! - Maximum: 5%
//! - Most common: 2-5% depending on municipality and service type

use crate::common::BrazilianCurrency;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// ISS transaction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssTransaction {
    /// Service description
    pub servico: String,
    /// Service value
    pub valor: BrazilianCurrency,
    /// Service code (LC 116/2003 list)
    pub codigo_servico: String,
    /// ISS rate
    pub aliquota_percentual: u8,
    /// Taxing municipality
    pub municipio: String,
}

impl IssTransaction {
    /// Create a new ISS transaction
    pub fn new(
        servico: impl Into<String>,
        valor: BrazilianCurrency,
        codigo: impl Into<String>,
        aliquota: u8,
        municipio: impl Into<String>,
    ) -> Result<Self, IssError> {
        if !(2..=5).contains(&aliquota) {
            return Err(IssError::InvalidRate { rate: aliquota });
        }

        Ok(Self {
            servico: servico.into(),
            valor,
            codigo_servico: codigo.into(),
            aliquota_percentual: aliquota,
            municipio: municipio.into(),
        })
    }

    /// Calculate ISS amount
    pub fn calculate_iss(&self) -> BrazilianCurrency {
        let valor_centavos = self.valor.centavos;
        let iss_centavos = (valor_centavos * self.aliquota_percentual as i64) / 100;
        BrazilianCurrency::from_centavos(iss_centavos)
    }
}

/// ISS errors
#[derive(Debug, Clone, Error)]
pub enum IssError {
    /// Invalid rate (must be 2-5%)
    #[error("Alíquota de ISS inválida: {rate}% (mínimo 2%, máximo 5%)")]
    InvalidRate { rate: u8 },

    /// Invalid service code
    #[error("Código de serviço inválido: {code}")]
    InvalidServiceCode { code: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for ISS operations
pub type IssResult<T> = Result<T, IssError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iss_calculation() {
        let transaction = IssTransaction::new(
            "Consultoria",
            BrazilianCurrency::from_reais(10000),
            "17.01",
            5,
            "São Paulo",
        )
        .expect("valid transaction");

        let iss = transaction.calculate_iss();
        assert_eq!(iss.reais(), 500); // 5% of 10000
    }

    #[test]
    fn test_invalid_rate() {
        let result = IssTransaction::new(
            "Serviço",
            BrazilianCurrency::from_reais(1000),
            "01.01",
            10, // Invalid: > 5%
            "São Paulo",
        );
        assert!(result.is_err());
    }
}
