//! # IRPJ - Imposto de Renda Pessoa Jurídica
//!
//! Corporate income tax (Lei 9.249/1995).
//!
//! ## Overview
//!
//! IRPJ is levied on corporate profits with two calculation regimes:
//!
//! | Regime | Description | Rate |
//! |--------|-------------|------|
//! | Lucro Real | Actual profit | 15% + 10% surcharge |
//! | Lucro Presumido | Presumed profit | 15% + 10% surcharge |
//! | Simples Nacional | Simplified (SMEs) | 4-19.5% unified |
//!
//! ## Rates
//!
//! - Base rate: 15% on taxable profit
//! - Surcharge: 10% on profit exceeding R$ 20,000/month (R$ 240,000/year)

use crate::common::BrazilianCurrency;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// IRPJ calculation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IrpjCalculation {
    /// Tax regime
    pub regime: IrpjRegime,
    /// Annual revenue or profit
    pub base_calculo: BrazilianCurrency,
    /// Tax year
    pub ano: u16,
}

/// IRPJ tax regimes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrpjRegime {
    /// Real profit (lucro real)
    /// Mandatory for companies with revenue > R$ 78M or financial institutions
    LucroReal,
    /// Presumed profit (lucro presumido)
    /// Optional for companies with revenue ≤ R$ 78M
    LucroPresumido {
        /// Presumption percentage (varies by activity: 8%, 16%, 32%)
        presuncao_percentual: u8,
    },
    /// Simples Nacional (simplified regime for SMEs)
    SimplesNacional,
}

impl IrpjCalculation {
    /// Create a new IRPJ calculation
    pub fn new(regime: IrpjRegime, base_calculo: BrazilianCurrency, ano: u16) -> Self {
        Self {
            regime,
            base_calculo,
            ano,
        }
    }

    /// Calculate taxable profit
    pub fn calculate_taxable_profit(&self) -> BrazilianCurrency {
        match self.regime {
            IrpjRegime::LucroReal => {
                // Base is already the actual profit
                self.base_calculo
            }
            IrpjRegime::LucroPresumido {
                presuncao_percentual,
            } => {
                // Presumed profit = Revenue * Presumption %
                let revenue = self.base_calculo.centavos;
                let profit = (revenue * presuncao_percentual as i64) / 100;
                BrazilianCurrency::from_centavos(profit)
            }
            IrpjRegime::SimplesNacional => {
                // Simples Nacional has unified calculation
                BrazilianCurrency::from_centavos(0)
            }
        }
    }

    /// Calculate IRPJ amount
    pub fn calculate_irpj(&self) -> BrazilianCurrency {
        if matches!(self.regime, IrpjRegime::SimplesNacional) {
            // Simples Nacional has different calculation
            return self.calculate_simples_nacional();
        }

        let taxable_profit = self.calculate_taxable_profit();
        let profit_centavos = taxable_profit.centavos;

        // Base rate: 15%
        let base_tax = (profit_centavos * 15) / 100;

        // Surcharge: 10% on profit exceeding R$ 240,000/year
        let threshold = 24000000; // R$ 240,000 in centavos
        let surcharge = if profit_centavos > threshold {
            ((profit_centavos - threshold) * 10) / 100
        } else {
            0
        };

        BrazilianCurrency::from_centavos(base_tax + surcharge)
    }

    /// Calculate Simples Nacional (simplified)
    fn calculate_simples_nacional(&self) -> BrazilianCurrency {
        let revenue = self.base_calculo.centavos;

        // Simplified calculation using average 8% rate
        // Actual rate varies by revenue bracket and activity (Annexes I-V)
        let tax = (revenue * 8) / 100;

        BrazilianCurrency::from_centavos(tax)
    }

    /// Calculate effective tax rate
    pub fn effective_rate(&self) -> f64 {
        let tax = self.calculate_irpj().centavos as f64;
        let base = self.base_calculo.centavos as f64;

        if base == 0.0 {
            return 0.0;
        }

        (tax / base) * 100.0
    }
}

/// CSLL - Contribuição Social sobre o Lucro Líquido
/// Social contribution on net profit (companion tax to IRPJ)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CsllCalculation {
    /// Tax regime (same as IRPJ)
    pub regime: IrpjRegime,
    /// Tax base (same as IRPJ)
    pub base_calculo: BrazilianCurrency,
}

impl CsllCalculation {
    /// CSLL rate: 9% (general) or 15% (financial institutions)
    const RATE_GENERAL: u8 = 9;
    const RATE_FINANCIAL: u8 = 15;

    /// Create a new CSLL calculation
    pub fn new(regime: IrpjRegime, base_calculo: BrazilianCurrency) -> Self {
        Self {
            regime,
            base_calculo,
        }
    }

    /// Calculate CSLL amount
    pub fn calculate_csll(&self, is_financial: bool) -> BrazilianCurrency {
        if matches!(self.regime, IrpjRegime::SimplesNacional) {
            // Included in Simples Nacional unified rate
            return BrazilianCurrency::from_centavos(0);
        }

        let base = self.base_calculo.centavos;
        let rate = if is_financial {
            Self::RATE_FINANCIAL
        } else {
            Self::RATE_GENERAL
        };

        let csll = (base * rate as i64) / 100;
        BrazilianCurrency::from_centavos(csll)
    }
}

/// IRPJ errors
#[derive(Debug, Clone, Error)]
pub enum IrpjError {
    /// Invalid regime
    #[error("Regime tributário inválido: {reason}")]
    InvalidRegime { reason: String },

    /// Invalid calculation base
    #[error("Base de cálculo inválida: {reason}")]
    InvalidBase { reason: String },

    /// Calculation error
    #[error("Erro no cálculo do IRPJ: {message}")]
    CalculationError { message: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for IRPJ operations
pub type IrpjResult<T> = Result<T, IrpjError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lucro_real() {
        let calc = IrpjCalculation::new(
            IrpjRegime::LucroReal,
            BrazilianCurrency::from_reais(1000000), // R$ 1M profit
            2024,
        );

        let irpj = calc.calculate_irpj();
        // 15% on 1M + 10% on (1M - 240k) = 150k + 76k = 226k
        assert!(irpj.reais() > 220000 && irpj.reais() < 230000);
    }

    #[test]
    fn test_lucro_presumido() {
        let calc = IrpjCalculation::new(
            IrpjRegime::LucroPresumido {
                presuncao_percentual: 32, // Services
            },
            BrazilianCurrency::from_reais(1000000), // R$ 1M revenue
            2024,
        );

        let taxable = calc.calculate_taxable_profit();
        assert_eq!(taxable.reais(), 320000); // 32% of 1M

        let irpj = calc.calculate_irpj();
        // 15% on 320k + 10% on (320k - 240k) = 48k + 8k = 56k
        assert!(irpj.reais() > 55000 && irpj.reais() < 57000);
    }

    #[test]
    fn test_csll() {
        let csll = CsllCalculation::new(
            IrpjRegime::LucroReal,
            BrazilianCurrency::from_reais(1000000),
        );

        let tax = csll.calculate_csll(false);
        assert_eq!(tax.reais(), 90000); // 9% of 1M
    }

    #[test]
    fn test_effective_rate() {
        let calc = IrpjCalculation::new(
            IrpjRegime::LucroReal,
            BrazilianCurrency::from_reais(1000000),
            2024,
        );

        let rate = calc.effective_rate();
        assert!(rate > 20.0 && rate < 25.0); // Around 22.6%
    }
}
