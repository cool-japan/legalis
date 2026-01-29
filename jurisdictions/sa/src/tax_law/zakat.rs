//! Zakat (الزكاة) - Islamic Wealth Tax
//!
//! Zakat is an Islamic obligation (one of the Five Pillars) levied on:
//! - Saudi nationals and GCC nationals
//! - Companies owned by Saudi/GCC nationals
//!
//! Rate: 2.5% of Zakat-able base (الوعاء الزكوي)

use crate::common::Sar;
use serde::{Deserialize, Serialize};

/// Zakat rates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZakatRate {
    /// Standard rate (2.5%)
    Standard,
}

impl ZakatRate {
    /// Get rate as percentage
    pub fn rate(&self) -> f64 {
        match self {
            Self::Standard => 2.5,
        }
    }

    /// Get description
    pub fn description_en(&self) -> &'static str {
        "2.5% of Zakat-able base (Islamic wealth purification)"
    }

    /// Get Arabic description
    pub fn description_ar(&self) -> &'static str {
        "2.5% من الوعاء الزكوي (تطهير المال)"
    }
}

/// Calculate Zakat amount
///
/// Zakat base typically includes:
/// - Cash and equivalents
/// - Inventory
/// - Accounts receivable
/// - Investments
/// - Minus current liabilities
pub fn calculate_zakat(zakat_base: Sar) -> Sar {
    let rate = ZakatRate::Standard.rate() / 100.0;
    let zakat_halalas = (zakat_base.halalas() as f64 * rate).round() as i64;
    Sar::from_halalas(zakat_halalas)
}

/// Calculate Zakat base from company financials (simplified)
pub fn calculate_zakat_base(
    cash: Sar,
    inventory: Sar,
    receivables: Sar,
    investments: Sar,
    liabilities: Sar,
) -> Sar {
    let total_assets = cash + inventory + receivables + investments;
    if total_assets > liabilities {
        total_assets - liabilities
    } else {
        Sar::from_halalas(0)
    }
}

/// Nisab threshold for Zakat (minimum wealth)
///
/// Nisab is equivalent to:
/// - 85 grams of gold, OR
/// - 595 grams of silver
///
/// Approximate in SAR (varies with gold/silver prices)
pub fn nisab_threshold_sar_approximate() -> Sar {
    // Approximate: 85g gold @ ~230 SAR/g = ~19,550 SAR
    Sar::from_riyals(19_550)
}

/// Check if Zakat is due (wealth >= Nisab threshold)
pub fn is_zakat_due(wealth: Sar) -> bool {
    wealth >= nisab_threshold_sar_approximate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zakat_rate() {
        assert_eq!(ZakatRate::Standard.rate(), 2.5);
    }

    #[test]
    fn test_zakat_calculation() {
        let base = Sar::from_riyals(100_000);
        let zakat = calculate_zakat(base);
        assert_eq!(zakat.riyals(), 2_500); // 2.5% of 100,000
    }

    #[test]
    fn test_zakat_base_calculation() {
        let cash = Sar::from_riyals(50_000);
        let inventory = Sar::from_riyals(30_000);
        let receivables = Sar::from_riyals(20_000);
        let investments = Sar::from_riyals(10_000);
        let liabilities = Sar::from_riyals(40_000);

        let base = calculate_zakat_base(cash, inventory, receivables, investments, liabilities);
        assert_eq!(base.riyals(), 70_000); // 110,000 - 40,000
    }

    #[test]
    fn test_zakat_base_with_high_liabilities() {
        let cash = Sar::from_riyals(10_000);
        let inventory = Sar::from_riyals(5_000);
        let receivables = Sar::from_riyals(3_000);
        let investments = Sar::from_riyals(2_000);
        let liabilities = Sar::from_riyals(25_000);

        let base = calculate_zakat_base(cash, inventory, receivables, investments, liabilities);
        assert_eq!(base.riyals(), 0); // Cannot be negative
    }

    #[test]
    fn test_nisab_threshold() {
        let nisab = nisab_threshold_sar_approximate();
        assert!(nisab.riyals() > 0);
        assert!(nisab.riyals() < 25_000); // Reasonable range
    }

    #[test]
    fn test_is_zakat_due() {
        let above_nisab = Sar::from_riyals(50_000);
        assert!(is_zakat_due(above_nisab));

        let below_nisab = Sar::from_riyals(10_000);
        assert!(!is_zakat_due(below_nisab));
    }
}
