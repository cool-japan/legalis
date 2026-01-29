//! Stamp Duty Act 1949
//!
//! Stamp duty on instruments (documents).
//!
//! # Common Instruments
//!
//! - **Transfer of property**: Progressive rates up to 4%
//! - **Loan agreements**: 0.5% on loan amount
//! - **Share transfer**: RM 1 per RM 1,000 (0.3%)
//! - **Tenancy agreements**: Varies based on rent and duration

use serde::{Deserialize, Serialize};

/// Type of instrument subject to stamp duty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StampDutyType {
    /// Transfer of real property.
    PropertyTransfer,
    /// Loan or financing agreement.
    LoanAgreement,
    /// Share transfer.
    ShareTransfer,
    /// Tenancy agreement.
    TenancyAgreement,
    /// Other instruments.
    Other,
}

/// Stamp duty calculator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StampDuty {
    /// Type of instrument.
    pub duty_type: StampDutyType,
    /// Value of instrument in sen.
    pub value_sen: i64,
    /// Whether first-time home buyer exemption applies.
    pub first_time_buyer: bool,
}

impl StampDuty {
    /// Creates a new stamp duty calculator.
    #[must_use]
    pub fn new(duty_type: StampDutyType, value_sen: i64) -> Self {
        Self {
            duty_type,
            value_sen,
            first_time_buyer: false,
        }
    }

    /// Sets first-time buyer status (for property transfer).
    #[must_use]
    pub fn with_first_time_buyer(mut self, is_first_time: bool) -> Self {
        self.first_time_buyer = is_first_time;
        self
    }

    /// Calculates stamp duty.
    #[must_use]
    pub fn calculate(&self) -> i64 {
        match self.duty_type {
            StampDutyType::PropertyTransfer => self.calculate_property_transfer(),
            StampDutyType::LoanAgreement => self.calculate_loan_agreement(),
            StampDutyType::ShareTransfer => self.calculate_share_transfer(),
            StampDutyType::TenancyAgreement => self.calculate_tenancy_agreement(),
            StampDutyType::Other => 0,
        }
    }

    /// Calculates stamp duty for property transfer (progressive rates).
    ///
    /// Rates:
    /// - First RM 100,000: 1%
    /// - RM 100,001 - 500,000: 2%
    /// - RM 500,001 - 1,000,000: 3%
    /// - Above RM 1,000,000: 4%
    fn calculate_property_transfer(&self) -> i64 {
        if self.first_time_buyer && self.value_sen <= 50000000 {
            // First-time buyer exemption for properties up to RM 500,000
            return 0;
        }

        let mut duty = 0;
        let mut remaining = self.value_sen;

        // First RM 100,000 at 1%
        let tier1 = remaining.min(10000000);
        duty += ((tier1 as f64) * 0.01).round() as i64;
        remaining -= tier1;

        if remaining > 0 {
            // RM 100,001 - 500,000 at 2%
            let tier2 = remaining.min(40000000);
            duty += ((tier2 as f64) * 0.02).round() as i64;
            remaining -= tier2;
        }

        if remaining > 0 {
            // RM 500,001 - 1,000,000 at 3%
            let tier3 = remaining.min(50000000);
            duty += ((tier3 as f64) * 0.03).round() as i64;
            remaining -= tier3;
        }

        if remaining > 0 {
            // Above RM 1,000,000 at 4%
            duty += ((remaining as f64) * 0.04).round() as i64;
        }

        duty
    }

    /// Calculates stamp duty for loan agreement (0.5%).
    fn calculate_loan_agreement(&self) -> i64 {
        ((self.value_sen as f64) * 0.005).round() as i64
    }

    /// Calculates stamp duty for share transfer (RM 1 per RM 1,000 = 0.3%).
    fn calculate_share_transfer(&self) -> i64 {
        ((self.value_sen as f64) * 0.003).round() as i64
    }

    /// Calculates stamp duty for tenancy agreement.
    ///
    /// Rates based on annual rent:
    /// - Up to RM 2,400: RM 1 per RM 250
    /// - Above RM 2,400: Progressive rates
    fn calculate_tenancy_agreement(&self) -> i64 {
        // Simplified calculation: RM 1 per RM 250 of annual rent
        let annual_rent = self.value_sen;
        ((annual_rent as f64) / 25_000.0).round() as i64 * 100 // RM 1 = 100 sen
    }
}

/// Calculates stamp duty for a given instrument type and value.
#[must_use]
pub fn calculate_stamp_duty(duty_type: StampDutyType, value_sen: i64) -> i64 {
    let duty = StampDuty::new(duty_type, value_sen);
    duty.calculate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_transfer_stamp_duty() {
        // RM 300,000 property
        let duty = StampDuty::new(StampDutyType::PropertyTransfer, 30000000);
        let amount = duty.calculate();
        // First RM 100,000 at 1% = RM 1,000
        // Next RM 200,000 at 2% = RM 4,000
        // Total = RM 5,000
        assert_eq!(amount, 500_000);
    }

    #[test]
    fn test_first_time_buyer_exemption() {
        // RM 400,000 property, first-time buyer
        let duty =
            StampDuty::new(StampDutyType::PropertyTransfer, 40000000).with_first_time_buyer(true);
        let amount = duty.calculate();
        assert_eq!(amount, 0); // Exempt
    }

    #[test]
    fn test_loan_agreement_stamp_duty() {
        // RM 500,000 loan
        let duty = StampDuty::new(StampDutyType::LoanAgreement, 50000000);
        let amount = duty.calculate();
        // 0.5% of RM 500,000 = RM 2,500
        assert_eq!(amount, 250_000);
    }

    #[test]
    fn test_share_transfer_stamp_duty() {
        // RM 100,000 share transfer
        let duty = StampDuty::new(StampDutyType::ShareTransfer, 10000000);
        let amount = duty.calculate();
        // 0.3% of RM 100,000 = RM 300
        assert_eq!(amount, 30_000);
    }

    #[test]
    fn test_tenancy_agreement_stamp_duty() {
        // RM 24,000 annual rent
        let duty = StampDuty::new(StampDutyType::TenancyAgreement, 2400000);
        let amount = duty.calculate();
        // RM 1 per RM 250 = RM 96
        assert_eq!(amount, 9_600);
    }
}
