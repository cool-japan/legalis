//! Value Added Tax Law (Ley del Impuesto al Valor Agregado)

use crate::common::MexicanCurrency;

/// Standard IVA rate (Article 1)
pub const STANDARD_RATE: f64 = 0.16; // 16%

/// Border zone reduced rate
pub const BORDER_RATE: f64 = 0.08; // 8%

/// Zero rate (for exports and some food items)
pub const ZERO_RATE: f64 = 0.00;

/// IVA rate type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IVARate {
    /// Standard 16%
    Standard,
    /// Border zone 8%
    Border,
    /// Zero rate (exports, basic food)
    Zero,
    /// Exempt
    Exempt,
}

impl IVARate {
    /// Get numeric rate
    pub fn rate(&self) -> f64 {
        match self {
            IVARate::Standard => STANDARD_RATE,
            IVARate::Border => BORDER_RATE,
            IVARate::Zero => ZERO_RATE,
            IVARate::Exempt => 0.0,
        }
    }
}

/// Calculate IVA on a transaction
pub fn calculate_iva(base_amount: MexicanCurrency, rate: IVARate) -> MexicanCurrency {
    let iva_amount = (base_amount.centavos as f64 * rate.rate()) as i64;
    MexicanCurrency::from_centavos(iva_amount)
}

/// Calculate total amount including IVA
pub fn calculate_with_iva(base_amount: MexicanCurrency, rate: IVARate) -> MexicanCurrency {
    let iva = calculate_iva(base_amount, rate);
    MexicanCurrency::from_centavos(base_amount.centavos + iva.centavos)
}

/// Extract IVA from total amount (inverse calculation)
pub fn extract_iva_from_total(total_amount: MexicanCurrency, rate: IVARate) -> MexicanCurrency {
    if rate.rate() == 0.0 {
        return MexicanCurrency::from_centavos(0);
    }

    let base = (total_amount.centavos as f64 / (1.0 + rate.rate())) as i64;
    let iva = total_amount.centavos - base;
    MexicanCurrency::from_centavos(iva)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_iva_standard() {
        let base = MexicanCurrency::from_pesos(1000);
        let iva = calculate_iva(base, IVARate::Standard);

        // 16% of 1,000 = 160
        assert_eq!(iva.pesos(), 160);
    }

    #[test]
    fn test_calculate_with_iva() {
        let base = MexicanCurrency::from_pesos(1000);
        let total = calculate_with_iva(base, IVARate::Standard);

        // 1,000 + 160 = 1,160
        assert_eq!(total.pesos(), 1160);
    }

    #[test]
    fn test_extract_iva() {
        let total = MexicanCurrency::from_pesos(1160);
        let iva = extract_iva_from_total(total, IVARate::Standard);

        // Should extract approximately 160
        assert!(iva.pesos() >= 159 && iva.pesos() <= 161);
    }
}
