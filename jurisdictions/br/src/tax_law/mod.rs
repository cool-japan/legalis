//! # Tax Law - Direito Tributário
//!
//! Brazilian tax system including federal, state, and municipal taxes.
//!
//! ## Overview
//!
//! Brazil has one of the world's most complex tax systems with three levels:
//!
//! | Level | Main Taxes | Authority |
//! |-------|-----------|-----------|
//! | Federal | IRPF, IRPJ, IPI, PIS/COFINS | Receita Federal |
//! | State | ICMS | SEFAZ (State Tax Secretariats) |
//! | Municipal | ISS, IPTU | Municipal Tax Offices |
//!
//! ## Major Taxes
//!
//! ### Federal Taxes
//!
//! | Tax | Description | Rate |
//! |-----|-------------|------|
//! | IRPF | Personal Income Tax | 0-27.5% progressive |
//! | IRPJ | Corporate Income Tax | 15% + 10% surcharge |
//! | IPI | Excise Tax | 0-300% (product-specific) |
//! | PIS/COFINS | Social contributions | 0.65%/3% or 1.65%/7.6% |
//!
//! ### State Taxes
//!
//! | Tax | Description | Rate |
//! |-----|-------------|------|
//! | ICMS | VAT on goods/services | 7-25% (state-dependent) |
//!
//! ### Municipal Taxes
//!
//! | Tax | Description | Rate |
//! |-----|-------------|------|
//! | ISS | Service Tax | 2-5% |
//! | IPTU | Property Tax | Variable |
//!
//! ## Tax Principles (CTN - Lei 5.172/1966)
//!
//! | Principle | Article | Description |
//! |-----------|---------|-------------|
//! | Legality | Art. 97 | No tax without law |
//! | Anteriority | Art. 150, III, b | Cannot collect in same year |
//! | Non-retroactivity | Art. 150, III, a | Law applies to future only |
//! | Equality | Art. 150, II | Equal treatment of taxpayers |

pub mod icms;
pub mod ipi;
pub mod irpf;
pub mod irpj;
pub mod iss;

pub use icms::*;
pub use ipi::*;
pub use irpf::*;
pub use irpj::*;
pub use iss::*;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Tax level (federative entity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxLevel {
    /// Federal (União)
    Federal,
    /// State (Estado)
    State,
    /// Municipal (Município)
    Municipal,
}

/// Tax errors
#[derive(Debug, Clone, Error)]
pub enum TaxError {
    /// Tax calculation error
    #[error("Erro no cálculo do tributo: {message}")]
    CalculationError { message: String },

    /// Invalid tax rate
    #[error("Alíquota inválida: {rate}%")]
    InvalidRate { rate: f64 },

    /// Tax evasion detected
    #[error("Sonegação fiscal detectada: {description}")]
    TaxEvasion { description: String },

    /// Principle violation
    #[error("Violação de princípio tributário: {principle}")]
    PrincipleViolation { principle: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for tax operations
pub type TaxResult<T> = Result<T, TaxError>;
