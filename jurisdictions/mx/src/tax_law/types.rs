//! Tax law types

use crate::common::MexicanCurrency;
use serde::{Deserialize, Serialize};

/// Taxpayer (Contribuyente)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Taxpayer {
    /// RFC (Tax ID)
    pub rfc: String,
    /// Taxpayer name
    pub nombre: String,
    /// Taxpayer type
    pub tipo: TaxpayerType,
}

/// Taxpayer type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxpayerType {
    /// Individual (Persona física)
    Individual,
    /// Corporation (Persona moral)
    Corporation,
}

/// Tax obligation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxObligation {
    /// Tax type
    pub tipo_impuesto: TaxType,
    /// Tax amount
    pub monto: MexicanCurrency,
    /// Tax period
    pub periodo: TaxPeriod,
}

/// Tax type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxType {
    /// Income tax (ISR)
    ISR,
    /// Value added tax (IVA)
    IVA,
    /// Special production tax (IEPS)
    IEPS,
    /// Import tax (Impuesto de importación)
    Import,
}

/// Tax period
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxPeriod {
    /// Monthly
    Monthly,
    /// Quarterly
    Quarterly,
    /// Annual
    Annual,
}

impl Taxpayer {
    /// Create new taxpayer
    pub fn new(rfc: String, nombre: String, tipo: TaxpayerType) -> Self {
        Self { rfc, nombre, tipo }
    }

    /// Check if taxpayer is corporation
    pub fn is_corporation(&self) -> bool {
        matches!(self.tipo, TaxpayerType::Corporation)
    }
}
