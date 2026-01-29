//! Sales and Service Tax (SST)
//!
//! Implemented in 2018, replacing GST.
//!
//! # Sales Tax
//!
//! - Standard rate: 10% (on manufactured goods)
//! - Reduced rate: 5% (on certain goods)
//! - Exemptions: Basic necessities, exported goods
//!
//! # Service Tax
//!
//! - Standard rate: 6% (on prescribed services)
//! - Applies to: F&B, telecommunications, professional services, etc.

use serde::{Deserialize, Serialize};

/// Sales tax configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SalesTax {
    /// Description of goods.
    pub goods_description: String,
    /// Value of goods in sen.
    pub value_sen: i64,
    /// Sales tax rate (5% or 10%).
    pub rate: f64,
    /// Whether goods are exempt.
    pub exempt: bool,
}

impl SalesTax {
    /// Creates a new sales tax entry.
    #[must_use]
    pub fn new(goods_description: impl Into<String>, value_sen: i64) -> Self {
        Self {
            goods_description: goods_description.into(),
            value_sen,
            rate: 10.0, // Default to standard rate
            exempt: false,
        }
    }

    /// Sets the tax rate.
    #[must_use]
    pub fn with_rate(mut self, rate: f64) -> Self {
        self.rate = rate;
        self
    }

    /// Sets exemption status.
    #[must_use]
    pub fn with_exemption(mut self, exempt: bool) -> Self {
        self.exempt = exempt;
        self
    }

    /// Calculates sales tax amount.
    #[must_use]
    pub fn calculate(&self) -> i64 {
        if self.exempt {
            return 0;
        }

        ((self.value_sen as f64) * (self.rate / 100.0)).round() as i64
    }
}

/// Service tax configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceTax {
    /// Description of service.
    pub service_description: String,
    /// Value of service in sen.
    pub value_sen: i64,
    /// Service tax rate (6%).
    pub rate: f64,
    /// Whether service is exempt.
    pub exempt: bool,
    /// Service type.
    pub service_type: ServiceType,
}

/// Type of prescribed service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceType {
    /// Food and beverage.
    FoodAndBeverage,
    /// Telecommunications.
    Telecommunications,
    /// Professional services (legal, accounting, etc.).
    Professional,
    /// Logistics and freight forwarding.
    Logistics,
    /// Other prescribed services.
    Other,
}

impl ServiceTax {
    /// Creates a new service tax entry.
    #[must_use]
    pub fn new(
        service_description: impl Into<String>,
        value_sen: i64,
        service_type: ServiceType,
    ) -> Self {
        Self {
            service_description: service_description.into(),
            value_sen,
            rate: 6.0, // Standard service tax rate
            exempt: false,
            service_type,
        }
    }

    /// Sets exemption status.
    #[must_use]
    pub fn with_exemption(mut self, exempt: bool) -> Self {
        self.exempt = exempt;
        self
    }

    /// Calculates service tax amount.
    #[must_use]
    pub fn calculate(&self) -> i64 {
        if self.exempt {
            return 0;
        }

        ((self.value_sen as f64) * (self.rate / 100.0)).round() as i64
    }
}

/// Calculates sales tax.
#[must_use]
pub fn calculate_sales_tax(value_sen: i64, rate: f64, exempt: bool) -> i64 {
    if exempt {
        return 0;
    }

    ((value_sen as f64) * (rate / 100.0)).round() as i64
}

/// Calculates service tax.
#[must_use]
pub fn calculate_service_tax(value_sen: i64, exempt: bool) -> i64 {
    if exempt {
        return 0;
    }

    ((value_sen as f64) * 0.06).round() as i64 // 6% service tax
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sales_tax_standard_rate() {
        let sales_tax = SalesTax::new("Electronics", 100_000); // RM 1,000
        let tax = sales_tax.calculate();
        assert_eq!(tax, 10_000); // 10% = RM 100
    }

    #[test]
    fn test_sales_tax_reduced_rate() {
        let sales_tax = SalesTax::new("Certain goods", 100_000).with_rate(5.0); // 5%
        let tax = sales_tax.calculate();
        assert_eq!(tax, 5_000); // 5% = RM 50
    }

    #[test]
    fn test_sales_tax_exempt() {
        let sales_tax = SalesTax::new("Basic necessities", 100_000).with_exemption(true);
        let tax = sales_tax.calculate();
        assert_eq!(tax, 0);
    }

    #[test]
    fn test_service_tax() {
        let service_tax = ServiceTax::new(
            "Legal services",
            200_000, // RM 2,000
            ServiceType::Professional,
        );
        let tax = service_tax.calculate();
        assert_eq!(tax, 12_000); // 6% = RM 120
    }

    #[test]
    fn test_service_tax_exempt() {
        let service_tax =
            ServiceTax::new("Exempt service", 200_000, ServiceType::Other).with_exemption(true);
        let tax = service_tax.calculate();
        assert_eq!(tax, 0);
    }
}
