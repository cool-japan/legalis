//! # IPI - Imposto sobre Produtos Industrializados
//!
//! Federal excise tax on industrialized products (Decreto 7.212/2010 - RIPI).
//!
//! ## Overview
//!
//! IPI is levied on:
//! - Domestic manufacturing
//! - Importation of industrialized products
//!
//! ## Rates
//!
//! Rates vary widely by product (TIPI table):
//! - Essential products: 0%
//! - Luxury goods: up to 300%

use crate::common::BrazilianCurrency;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// IPI transaction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IpiTransaction {
    /// Product description
    pub produto: String,
    /// NCM code (Nomenclatura Comum do Mercosul)
    pub ncm: String,
    /// Product value
    pub valor: BrazilianCurrency,
    /// IPI rate
    pub aliquota_percentual: u16,
    /// Transaction type
    pub tipo: IpiTransactionType,
}

/// IPI transaction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IpiTransactionType {
    /// Domestic manufacturing (saída do estabelecimento fabricante)
    DomesticManufacturing,
    /// Importation
    Import,
    /// Exempt (isenção)
    Exempt,
    /// Export (não-incidência)
    Export,
}

impl IpiTransaction {
    /// Create a new IPI transaction
    pub fn new(
        produto: impl Into<String>,
        ncm: impl Into<String>,
        valor: BrazilianCurrency,
        aliquota: u16,
        tipo: IpiTransactionType,
    ) -> Result<Self, IpiError> {
        if aliquota > 300 {
            return Err(IpiError::InvalidRate { rate: aliquota });
        }

        Ok(Self {
            produto: produto.into(),
            ncm: ncm.into(),
            valor,
            aliquota_percentual: aliquota,
            tipo,
        })
    }

    /// Calculate IPI amount
    pub fn calculate_ipi(&self) -> BrazilianCurrency {
        if matches!(
            self.tipo,
            IpiTransactionType::Exempt | IpiTransactionType::Export
        ) {
            return BrazilianCurrency::from_centavos(0);
        }

        let valor_centavos = self.valor.centavos;
        let ipi_centavos = (valor_centavos * self.aliquota_percentual as i64) / 100;
        BrazilianCurrency::from_centavos(ipi_centavos)
    }

    /// Check if product is in essential category (0% rate)
    pub fn is_essential(&self) -> bool {
        self.aliquota_percentual == 0
    }

    /// Check if product is luxury (high rate)
    pub fn is_luxury(&self) -> bool {
        self.aliquota_percentual >= 30
    }
}

/// IPI errors
#[derive(Debug, Clone, Error)]
pub enum IpiError {
    /// Invalid rate
    #[error("Alíquota de IPI inválida: {rate}% (máximo 300%)")]
    InvalidRate { rate: u16 },

    /// Invalid NCM code
    #[error("Código NCM inválido: {ncm}")]
    InvalidNcm { ncm: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for IPI operations
pub type IpiResult<T> = Result<T, IpiError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipi_calculation() {
        let transaction = IpiTransaction::new(
            "Bebida alcoólica",
            "2208.30.00",
            BrazilianCurrency::from_reais(1000),
            60,
            IpiTransactionType::DomesticManufacturing,
        )
        .expect("valid transaction");

        let ipi = transaction.calculate_ipi();
        assert_eq!(ipi.reais(), 600); // 60% of 1000
        assert!(transaction.is_luxury());
    }

    #[test]
    fn test_exempt_transaction() {
        let transaction = IpiTransaction::new(
            "Produto essencial",
            "1001.11.00",
            BrazilianCurrency::from_reais(1000),
            0,
            IpiTransactionType::Exempt,
        )
        .expect("valid transaction");

        let ipi = transaction.calculate_ipi();
        assert_eq!(ipi.reais(), 0);
        assert!(transaction.is_essential());
    }
}
