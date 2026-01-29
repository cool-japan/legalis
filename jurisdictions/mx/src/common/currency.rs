//! Mexican currency utilities

use crate::common::types::MexicanCurrency;

/// Minimum wage constants (updated annually)
pub mod minimum_wage {
    use super::*;

    /// Minimum wage for 2024 in the Northern Border Zone (Zona Libre)
    pub const NORTHERN_BORDER_2024: MexicanCurrency = MexicanCurrency {
        centavos: 37470, // $374.70 MXN
    };

    /// General minimum wage for 2024
    pub const GENERAL_2024: MexicanCurrency = MexicanCurrency {
        centavos: 24873, // $248.73 MXN
    };

    /// Get minimum wage by year and zone
    pub fn get_minimum_wage(year: u32, is_northern_border: bool) -> Option<MexicanCurrency> {
        match (year, is_northern_border) {
            (2024, true) => Some(NORTHERN_BORDER_2024),
            (2024, false) => Some(GENERAL_2024),
            _ => None,
        }
    }
}

/// UMA (Unidad de Medida y Actualización) constants
pub mod uma {
    use super::*;

    /// UMA daily value for 2024
    pub const DAILY_2024: MexicanCurrency = MexicanCurrency {
        centavos: 10841, // $108.41 MXN
    };

    /// UMA monthly value for 2024
    pub const MONTHLY_2024: MexicanCurrency = MexicanCurrency {
        centavos: 325305, // $3,253.05 MXN
    };

    /// UMA annual value for 2024
    pub const ANNUAL_2024: MexicanCurrency = MexicanCurrency {
        centavos: 3965795, // $39,657.95 MXN
    };

    /// Get UMA value by year and period
    pub fn get_uma(year: u32, period: UmaPeriod) -> Option<MexicanCurrency> {
        match (year, period) {
            (2024, UmaPeriod::Daily) => Some(DAILY_2024),
            (2024, UmaPeriod::Monthly) => Some(MONTHLY_2024),
            (2024, UmaPeriod::Annual) => Some(ANNUAL_2024),
            _ => None,
        }
    }

    /// UMA period types
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum UmaPeriod {
        Daily,
        Monthly,
        Annual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_wage() {
        let wage = minimum_wage::get_minimum_wage(2024, false);
        assert!(wage.is_some());
        assert_eq!(wage.unwrap(), minimum_wage::GENERAL_2024);
    }

    #[test]
    fn test_uma_values() {
        let daily = uma::get_uma(2024, uma::UmaPeriod::Daily);
        assert!(daily.is_some());
        assert_eq!(daily.unwrap(), uma::DAILY_2024);
    }
}
