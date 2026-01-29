//! # IRPF - Imposto de Renda Pessoa Física
//!
//! Personal income tax (Lei 7.713/1988 and subsequent amendments).
//!
//! ## Overview
//!
//! IRPF is a progressive tax on individual income with annual adjustment.
//!
//! ## 2024 Rates (Approximation)
//!
//! | Monthly Income (R$) | Rate | Deduction (R$) |
//! |---------------------|------|----------------|
//! | Up to 2,112.00 | Exempt | 0 |
//! | 2,112.01 - 2,826.65 | 7.5% | 158.40 |
//! | 2,826.66 - 3,751.05 | 15% | 370.40 |
//! | 3,751.06 - 4,664.68 | 22.5% | 651.73 |
//! | Above 4,664.68 | 27.5% | 884.96 |

use crate::common::BrazilianCurrency;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// IRPF calculation (monthly)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IrpfCalculation {
    /// Gross monthly income
    pub renda_bruta: BrazilianCurrency,
    /// INSS deduction
    pub desconto_inss: BrazilianCurrency,
    /// Number of dependents
    pub dependentes: u8,
    /// Other deductions (pension, etc.)
    pub outras_deducoes: BrazilianCurrency,
}

impl IrpfCalculation {
    /// Create a new IRPF calculation
    pub fn new(renda_bruta: BrazilianCurrency) -> Self {
        Self {
            renda_bruta,
            desconto_inss: BrazilianCurrency::from_centavos(0),
            dependentes: 0,
            outras_deducoes: BrazilianCurrency::from_centavos(0),
        }
    }

    /// Add INSS deduction
    pub fn with_inss(mut self, inss: BrazilianCurrency) -> Self {
        self.desconto_inss = inss;
        self
    }

    /// Add dependents
    pub fn with_dependents(mut self, count: u8) -> Self {
        self.dependentes = count;
        self
    }

    /// Add other deductions
    pub fn with_deductions(mut self, deductions: BrazilianCurrency) -> Self {
        self.outras_deducoes = deductions;
        self
    }

    /// Calculate taxable income (base de cálculo)
    pub fn calculate_taxable_income(&self) -> BrazilianCurrency {
        // Dependent deduction: R$ 189.59 per dependent (2024 approximation)
        let dependent_deduction = 18959 * self.dependentes as i64;

        let total_deductions =
            self.desconto_inss.centavos + dependent_deduction + self.outras_deducoes.centavos;

        let taxable = self.renda_bruta.centavos.saturating_sub(total_deductions);

        BrazilianCurrency::from_centavos(taxable)
    }

    /// Calculate IRPF amount
    pub fn calculate_irpf(&self) -> BrazilianCurrency {
        let taxable = self.calculate_taxable_income();
        let taxable_centavos = taxable.centavos;

        // 2024 brackets (approximation in centavos)
        let (rate, deduction) = if taxable_centavos <= 211200 {
            return BrazilianCurrency::from_centavos(0); // Exempt
        } else if taxable_centavos <= 282665 {
            (75, 15840) // 7.5%, R$ 158.40
        } else if taxable_centavos <= 375105 {
            (150, 37040) // 15%, R$ 370.40
        } else if taxable_centavos <= 466468 {
            (225, 65173) // 22.5%, R$ 651.73
        } else {
            (275, 88496) // 27.5%, R$ 884.96
        };

        let tax = (taxable_centavos * rate) / 1000;
        let final_tax = tax.saturating_sub(deduction);

        BrazilianCurrency::from_centavos(final_tax)
    }

    /// Get effective tax rate
    pub fn effective_rate(&self) -> f64 {
        let tax = self.calculate_irpf().centavos as f64;
        let gross = self.renda_bruta.centavos as f64;

        if gross == 0.0 {
            return 0.0;
        }

        (tax / gross) * 100.0
    }
}

/// Annual IRPF declaration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IrpfDeclaration {
    /// Tax year
    pub ano_calendario: u16,
    /// Taxpayer CPF
    pub cpf: String,
    /// Total annual income
    pub rendimentos_totais: BrazilianCurrency,
    /// Model (simplified or complete)
    pub modelo: DeclarationModel,
}

/// Declaration models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeclarationModel {
    /// Simplified (20% standard deduction)
    Simplificada,
    /// Complete (itemized deductions)
    Completa,
}

/// IRPF errors
#[derive(Debug, Clone, Error)]
pub enum IrpfError {
    /// Invalid income
    #[error("Renda inválida: {reason}")]
    InvalidIncome { reason: String },

    /// Invalid deduction
    #[error("Dedução inválida: {reason}")]
    InvalidDeduction { reason: String },

    /// Calculation error
    #[error("Erro no cálculo do IRPF: {message}")]
    CalculationError { message: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for IRPF operations
pub type IrpfResult<T> = Result<T, IrpfError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exempt_income() {
        let calc = IrpfCalculation::new(BrazilianCurrency::from_reais(2000));
        let irpf = calc.calculate_irpf();
        assert_eq!(irpf.reais(), 0); // Below exempt threshold
    }

    #[test]
    fn test_taxable_income() {
        let calc = IrpfCalculation::new(BrazilianCurrency::from_reais(5000))
            .with_inss(BrazilianCurrency::from_reais(550))
            .with_dependents(2);

        let taxable = calc.calculate_taxable_income();
        // 5000 - 550 - (189.59 * 2) = 4070.82 approx
        assert!(taxable.reais() > 4000 && taxable.reais() < 4100);
    }

    #[test]
    fn test_irpf_calculation() {
        let calc = IrpfCalculation::new(BrazilianCurrency::from_reais(5000));
        let irpf = calc.calculate_irpf();
        assert!(irpf.reais() > 0); // Should have some tax
    }

    #[test]
    fn test_effective_rate() {
        let calc = IrpfCalculation::new(BrazilianCurrency::from_reais(5000));
        let rate = calc.effective_rate();
        assert!(rate > 0.0 && rate < 27.5);
    }
}
