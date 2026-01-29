//! Value Added Tax (VAT / НДС) implementation.
//!
//! Chapter 21 of Tax Code Part 2
//!
//! Standard rate: 20%
//! Reduced rate: 10% (food, children's goods, medical supplies)
//! Zero rate: 0% (exports, international transport)

use serde::{Deserialize, Serialize};

use super::TaxCodeError;

/// VAT rates under Russian Tax Code
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VatRate {
    /// Standard rate (20%)
    Standard,
    /// Reduced rate (10%)
    Reduced,
    /// Zero rate (0%)
    Zero,
    /// Exempt from VAT
    Exempt,
}

impl VatRate {
    /// Gets the percentage for this VAT rate
    pub fn percentage(&self) -> f64 {
        match self {
            Self::Standard => 20.0,
            Self::Reduced => 10.0,
            Self::Zero => 0.0,
            Self::Exempt => 0.0,
        }
    }

    /// Gets the description in Russian
    pub fn description_ru(&self) -> &'static str {
        match self {
            Self::Standard => "Основная ставка НДС 20%",
            Self::Reduced => "Пониженная ставка НДС 10%",
            Self::Zero => "Нулевая ставка НДС 0%",
            Self::Exempt => "Освобождение от НДС",
        }
    }

    /// Gets the description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::Standard => "Standard VAT rate 20%",
            Self::Reduced => "Reduced VAT rate 10%",
            Self::Zero => "Zero VAT rate 0%",
            Self::Exempt => "VAT exempt",
        }
    }
}

/// VAT calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatCalculation {
    /// Base amount (without VAT)
    pub base_amount: crate::common::Currency,
    /// VAT rate applied
    pub vat_rate: VatRate,
    /// VAT amount
    pub vat_amount: crate::common::Currency,
    /// Total amount (base + VAT)
    pub total_amount: crate::common::Currency,
}

impl VatCalculation {
    /// Creates a new VAT calculation
    pub fn new(base_amount: crate::common::Currency, vat_rate: VatRate) -> Self {
        let vat_amount = base_amount.multiply_percentage(vat_rate.percentage());
        let total_amount = base_amount.add(&vat_amount);

        Self {
            base_amount,
            vat_rate,
            vat_amount,
            total_amount,
        }
    }

    /// Calculates VAT from total amount (extracts VAT)
    pub fn from_total(total_amount: crate::common::Currency, vat_rate: VatRate) -> Self {
        let rate = vat_rate.percentage();
        let divisor = 100.0 + rate;
        let base_kopecks = ((total_amount.kopecks as f64) * 100.0 / divisor).round() as i64;
        let base_amount = crate::common::Currency {
            kopecks: base_kopecks,
        };
        let vat_amount = total_amount.subtract(&base_amount);

        Self {
            base_amount,
            vat_rate,
            vat_amount,
            total_amount,
        }
    }

    /// Validates the VAT calculation
    pub fn validate(&self) -> Result<(), TaxCodeError> {
        if !self.base_amount.is_positive() {
            return Err(TaxCodeError::InvalidCalculation(
                "VAT base amount must be positive".to_string(),
            ));
        }

        // Verify calculation
        let expected_vat = self
            .base_amount
            .multiply_percentage(self.vat_rate.percentage());
        let expected_total = self.base_amount.add(&expected_vat);

        if (self.vat_amount.kopecks - expected_vat.kopecks).abs() > 1 {
            return Err(TaxCodeError::InvalidCalculation(
                "VAT amount calculation mismatch".to_string(),
            ));
        }

        if (self.total_amount.kopecks - expected_total.kopecks).abs() > 1 {
            return Err(TaxCodeError::InvalidCalculation(
                "Total amount calculation mismatch".to_string(),
            ));
        }

        Ok(())
    }
}

/// Calculates VAT on a given amount
pub fn calculate_vat(base_amount: crate::common::Currency, vat_rate: VatRate) -> VatCalculation {
    VatCalculation::new(base_amount, vat_rate)
}

/// Article 164: VAT rates - determines appropriate rate for goods/services
pub fn determine_vat_rate(is_food: bool, is_export: bool, is_medical: bool) -> VatRate {
    if is_export {
        VatRate::Zero
    } else if is_food || is_medical {
        VatRate::Reduced
    } else {
        VatRate::Standard
    }
}

/// Article 171: VAT deduction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatDeduction {
    /// Input VAT amount
    pub input_vat: crate::common::Currency,
    /// Is eligible for deduction
    pub eligible: bool,
    /// Reason for ineligibility (if any)
    pub ineligibility_reason: Option<String>,
}

impl VatDeduction {
    /// Creates a new VAT deduction
    pub fn new(input_vat: crate::common::Currency) -> Self {
        Self {
            input_vat,
            eligible: true,
            ineligibility_reason: None,
        }
    }

    /// Marks as ineligible with reason
    pub fn ineligible(mut self, reason: impl Into<String>) -> Self {
        self.eligible = false;
        self.ineligibility_reason = Some(reason.into());
        self
    }

    /// Gets the deductible amount
    pub fn deductible_amount(&self) -> crate::common::Currency {
        if self.eligible {
            self.input_vat
        } else {
            crate::common::Currency::from_rubles(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vat_rates() {
        assert_eq!(VatRate::Standard.percentage(), 20.0);
        assert_eq!(VatRate::Reduced.percentage(), 10.0);
        assert_eq!(VatRate::Zero.percentage(), 0.0);
    }

    #[test]
    fn test_vat_calculation() {
        let base = crate::common::Currency::from_rubles(1000);
        let calc = calculate_vat(base, VatRate::Standard);

        assert_eq!(calc.base_amount.rubles(), 1000);
        assert_eq!(calc.vat_amount.rubles(), 200); // 20% of 1000
        assert_eq!(calc.total_amount.rubles(), 1200);
        assert!(calc.validate().is_ok());
    }

    #[test]
    fn test_vat_from_total() {
        let total = crate::common::Currency::from_rubles(1200);
        let calc = VatCalculation::from_total(total, VatRate::Standard);

        assert_eq!(calc.total_amount.rubles(), 1200);
        assert_eq!(calc.base_amount.rubles(), 1000);
        assert_eq!(calc.vat_amount.rubles(), 200);
    }

    #[test]
    fn test_determine_vat_rate() {
        assert_eq!(determine_vat_rate(false, false, false), VatRate::Standard);
        assert_eq!(determine_vat_rate(true, false, false), VatRate::Reduced);
        assert_eq!(determine_vat_rate(false, true, false), VatRate::Zero);
        assert_eq!(determine_vat_rate(false, false, true), VatRate::Reduced);
    }

    #[test]
    fn test_vat_deduction() {
        let input_vat = crate::common::Currency::from_rubles(200);
        let deduction = VatDeduction::new(input_vat);

        assert!(deduction.eligible);
        assert_eq!(deduction.deductible_amount().rubles(), 200);

        let ineligible = VatDeduction::new(input_vat).ineligible("Not used for taxable operations");
        assert!(!ineligible.eligible);
        assert_eq!(ineligible.deductible_amount().rubles(), 0);
    }
}
