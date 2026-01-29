//! Special Production and Services Tax (Impuesto Especial sobre Producción y Servicios)

use crate::common::MexicanCurrency;

/// IEPS categories and rates
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IEPSCategory {
    /// Alcoholic beverages (14-53% depending on alcohol content)
    AlcoholicBeverages(f64),
    /// Beer (26.5%)
    Beer,
    /// Tobacco products (160%)
    Tobacco,
    /// Sugary drinks (1 peso per liter)
    SugaryDrinks,
    /// High-calorie foods (8%)
    HighCalorieFoods,
    /// Fossil fuels (variable)
    FossilFuels(f64),
}

impl IEPSCategory {
    /// Get the IEPS rate for this category
    pub fn rate(&self) -> f64 {
        match self {
            IEPSCategory::AlcoholicBeverages(rate) => *rate,
            IEPSCategory::Beer => 0.265,
            IEPSCategory::Tobacco => 1.60,
            IEPSCategory::SugaryDrinks => 0.0, // Fixed per liter, not percentage
            IEPSCategory::HighCalorieFoods => 0.08,
            IEPSCategory::FossilFuels(rate) => *rate,
        }
    }
}

/// Calculate IEPS tax
pub fn calculate_ieps(base_amount: MexicanCurrency, category: IEPSCategory) -> MexicanCurrency {
    let rate = category.rate();
    let ieps_amount = (base_amount.centavos as f64 * rate) as i64;
    MexicanCurrency::from_centavos(ieps_amount)
}

/// Calculate IEPS for sugary drinks (per liter)
pub fn calculate_sugary_drinks_ieps(liters: f64) -> MexicanCurrency {
    // 1 peso per liter
    MexicanCurrency::from_centavos((liters * 100.0) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_ieps_beer() {
        let base = MexicanCurrency::from_pesos(100);
        let ieps = calculate_ieps(base, IEPSCategory::Beer);

        // 26.5% of 100 = 26.5
        assert_eq!(ieps.pesos(), 26);
    }

    #[test]
    fn test_calculate_ieps_tobacco() {
        let base = MexicanCurrency::from_pesos(100);
        let ieps = calculate_ieps(base, IEPSCategory::Tobacco);

        // 160% of 100 = 160
        assert_eq!(ieps.pesos(), 160);
    }

    #[test]
    fn test_sugary_drinks_ieps() {
        let ieps = calculate_sugary_drinks_ieps(2.0); // 2 liters
        assert_eq!(ieps.pesos(), 2);
    }
}
