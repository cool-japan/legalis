//! Value Added Tax (VAT) - ضريبة القيمة المضافة
//!
//! Royal Decree No. M/113 dated 2/11/1438H (2017)
//!
//! VAT was introduced at 5% in 2018 and increased to 15% in 2020.
//! Mandatory registration threshold: SAR 375,000 annual revenue.

use crate::common::Sar;
use serde::{Deserialize, Serialize};

/// VAT rates in Saudi Arabia
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VatRate {
    /// Standard rate (15%)
    Standard,
    /// Zero-rated (0%)
    Zero,
    /// Exempt (no VAT)
    Exempt,
}

impl VatRate {
    /// Get rate as percentage
    pub fn rate(&self) -> f64 {
        match self {
            Self::Standard => 15.0,
            Self::Zero => 0.0,
            Self::Exempt => 0.0,
        }
    }

    /// Get description
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::Standard => "Standard rate applicable to most goods and services",
            Self::Zero => "Zero-rated supplies (exports, international transport)",
            Self::Exempt => "Exempt supplies (residential property, some financial services)",
        }
    }
}

/// VAT registration status
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VatRegistrationStatus {
    /// Mandatory registration (revenue >= 375,000 SAR)
    Mandatory,
    /// Voluntary registration (revenue >= 187,500 SAR)
    Voluntary,
    /// Not required
    NotRequired,
}

/// VAT registration details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatRegistration {
    /// Annual revenue
    pub annual_revenue: Sar,
    /// VAT registration number
    pub vat_number: Option<String>,
    /// Registration status
    pub status: VatRegistrationStatus,
    /// Is e-invoicing enabled
    pub e_invoicing_enabled: bool,
}

impl VatRegistration {
    /// Create new VAT registration
    pub fn new(annual_revenue: Sar) -> Self {
        let status = if annual_revenue >= Sar::from_riyals(375_000) {
            VatRegistrationStatus::Mandatory
        } else if annual_revenue >= Sar::from_riyals(187_500) {
            VatRegistrationStatus::Voluntary
        } else {
            VatRegistrationStatus::NotRequired
        };

        Self {
            annual_revenue,
            vat_number: None,
            status,
            e_invoicing_enabled: false,
        }
    }

    /// Set VAT number
    pub fn with_vat_number(mut self, number: impl Into<String>) -> Self {
        self.vat_number = Some(number.into());
        self
    }

    /// Enable e-invoicing
    pub fn with_e_invoicing(mut self) -> Self {
        self.e_invoicing_enabled = true;
        self
    }

    /// Check if registration is required
    pub fn is_registration_required(&self) -> bool {
        matches!(self.status, VatRegistrationStatus::Mandatory)
    }
}

/// Calculate VAT amount
pub fn calculate_vat(amount: Sar, rate: VatRate) -> Sar {
    let rate_decimal = rate.rate() / 100.0;
    let vat_halalas = (amount.halalas() as f64 * rate_decimal).round() as i64;
    Sar::from_halalas(vat_halalas)
}

/// Calculate total including VAT
pub fn calculate_total_with_vat(amount: Sar, rate: VatRate) -> Sar {
    amount + calculate_vat(amount, rate)
}

/// Extract VAT from gross amount (reverse calculation)
pub fn extract_vat_from_gross(gross: Sar, rate: VatRate) -> Sar {
    let rate_decimal = rate.rate() / 100.0;
    let vat_halalas = (gross.halalas() as f64 * rate_decimal / (1.0 + rate_decimal)).round() as i64;
    Sar::from_halalas(vat_halalas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vat_rates() {
        assert_eq!(VatRate::Standard.rate(), 15.0);
        assert_eq!(VatRate::Zero.rate(), 0.0);
        assert_eq!(VatRate::Exempt.rate(), 0.0);
    }

    #[test]
    fn test_vat_calculation() {
        let amount = Sar::from_riyals(1000);
        let vat = calculate_vat(amount, VatRate::Standard);
        assert_eq!(vat.riyals(), 150); // 15% of 1000
    }

    #[test]
    fn test_total_with_vat() {
        let amount = Sar::from_riyals(1000);
        let total = calculate_total_with_vat(amount, VatRate::Standard);
        assert_eq!(total.riyals(), 1150); // 1000 + 150
    }

    #[test]
    fn test_extract_vat() {
        let gross = Sar::from_riyals(1150);
        let vat = extract_vat_from_gross(gross, VatRate::Standard);
        assert_eq!(vat.riyals(), 150);
    }

    #[test]
    fn test_vat_registration_mandatory() {
        let reg = VatRegistration::new(Sar::from_riyals(500_000));
        assert_eq!(reg.status, VatRegistrationStatus::Mandatory);
        assert!(reg.is_registration_required());
    }

    #[test]
    fn test_vat_registration_voluntary() {
        let reg = VatRegistration::new(Sar::from_riyals(200_000));
        assert_eq!(reg.status, VatRegistrationStatus::Voluntary);
        assert!(!reg.is_registration_required());
    }

    #[test]
    fn test_vat_registration_not_required() {
        let reg = VatRegistration::new(Sar::from_riyals(100_000));
        assert_eq!(reg.status, VatRegistrationStatus::NotRequired);
        assert!(!reg.is_registration_required());
    }
}
