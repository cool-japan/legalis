//! Property Law - Conveyancing
//!
//! Models the conveyancing of land in Singapore:
//!
//! - **Formalities.** A contract for the sale or other disposition of immovable
//!   property must be evidenced in writing and signed (Civil Law Act s. 6(d) -
//!   the Singapore equivalent of the Statute of Frauds).
//! - **The Option to Purchase (OTP).** The standard mechanism in Singapore
//!   private-property conveyancing: the vendor grants the purchaser an option,
//!   for an option fee (conventionally 1% of the price), exercisable within the
//!   option period (conventionally 14 days for a private resale) by signing the
//!   acceptance and paying the balance deposit (conventionally a further 4%, to
//!   a 5% total).
//! - **Completion.** Payment of the balance of the purchase price against
//!   delivery of the executed instrument of transfer and vacant possession,
//!   followed by registration under the Land Titles Act.
//! - **Buyer's Stamp Duty (BSD).** Computed on the higher of the price or market
//!   value under the Stamp Duties Act 1929 (rates as at 2023).
//!
//! Monetary values are stored as **SGD cents** (`u64`). The OTP figures below are
//! market conventions, not statutory requirements.

use super::types::PropertyType;
use serde::{Deserialize, Serialize};

// ============================================================================
// Conventions (market practice, not statute)
// ============================================================================

/// Conventional option fee for a private resale, as a percentage of the price
/// (market practice, not a statutory requirement).
pub const OPTION_FEE_PERCENT_PRIVATE_RESALE: u64 = 1;

/// Conventional balance deposit paid on exercise of the option, as a percentage
/// of the price, taking the total deposit to 5% (market practice).
pub const BALANCE_DEPOSIT_PERCENT: u64 = 4;

/// Conventional option period for a private resale, in days (market practice).
pub const DEFAULT_OPTION_PERIOD_DAYS: u32 = 14;

/// Conventional completion period for a private resale, in weeks (market
/// practice).
pub const TYPICAL_COMPLETION_WEEKS: u32 = 12;

// ============================================================================
// Buyer's Stamp Duty (Stamp Duties Act 1929)
// ============================================================================

/// A marginal Buyer's Stamp Duty band: a band width in SGD cents and the rate
/// (percent) applied to consideration within the band. The final band uses
/// [`u64::MAX`] as its width to absorb any remainder.
type BsdBand = (u64, u64);

/// Residential Buyer's Stamp Duty bands (Stamp Duties Act 1929; rates as at
/// 2023): 1% on the first SGD 180,000; 2% on the next SGD 180,000; 3% on the
/// next SGD 640,000; 4% on the next SGD 500,000; 5% on the next SGD 1,500,000;
/// 6% on the remainder above SGD 3,000,000.
pub const BSD_RESIDENTIAL_BANDS: [BsdBand; 6] = [
    (18_000_000, 1),  // first SGD 180,000
    (18_000_000, 2),  // next SGD 180,000  -> SGD 360,000
    (64_000_000, 3),  // next SGD 640,000  -> SGD 1,000,000
    (50_000_000, 4),  // next SGD 500,000  -> SGD 1,500,000
    (150_000_000, 5), // next SGD 1,500,000 -> SGD 3,000,000
    (u64::MAX, 6),    // remainder above SGD 3,000,000
];

/// Non-residential Buyer's Stamp Duty bands (Stamp Duties Act 1929; rates as at
/// 2023): 1% on the first SGD 180,000; 2% on the next SGD 180,000; 3% on the
/// next SGD 640,000; 4% on the next SGD 500,000; 5% on the remainder above
/// SGD 1,500,000.
pub const BSD_NON_RESIDENTIAL_BANDS: [BsdBand; 5] = [
    (18_000_000, 1), // first SGD 180,000
    (18_000_000, 2), // next SGD 180,000  -> SGD 360,000
    (64_000_000, 3), // next SGD 640,000  -> SGD 1,000,000
    (50_000_000, 4), // next SGD 500,000  -> SGD 1,500,000
    (u64::MAX, 5),   // remainder above SGD 1,500,000
];

/// Computes the Buyer's Stamp Duty (BSD) payable on a purchase, in SGD cents,
/// applying the marginal-rate bands of the Stamp Duties Act 1929 (rates as at
/// 2023).
///
/// BSD is charged on the higher of the consideration and the market value; the
/// caller should pass that higher figure as `consideration_cents`. The
/// residential scale applies to residential property; the non-residential scale
/// applies to all other property.
///
/// # Examples
///
/// ```
/// use legalis_sg::property::compute_buyers_stamp_duty_cents;
/// use legalis_sg::property::PropertyType;
///
/// // SGD 1,000,000 residential purchase -> BSD of SGD 24,600.
/// let bsd = compute_buyers_stamp_duty_cents(100_000_000, PropertyType::Residential);
/// assert_eq!(bsd, 2_460_000);
/// ```
pub fn compute_buyers_stamp_duty_cents(
    consideration_cents: u64,
    property_type: PropertyType,
) -> u64 {
    let bands: &[BsdBand] = if property_type.is_residential() {
        &BSD_RESIDENTIAL_BANDS
    } else {
        &BSD_NON_RESIDENTIAL_BANDS
    };

    let mut remaining = consideration_cents;
    let mut duty = 0u64;
    for &(width, rate) in bands {
        if remaining == 0 {
            break;
        }
        let in_band = remaining.min(width);
        duty = duty.saturating_add(in_band.saturating_mul(rate) / 100);
        remaining = remaining.saturating_sub(in_band);
    }
    duty
}

// ============================================================================
// Option to Purchase
// ============================================================================

/// An Option to Purchase (OTP) granted by a vendor to a prospective purchaser.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionToPurchase {
    /// Description / reference of the property.
    pub property: String,
    /// The agreed price, in SGD cents.
    pub price_cents: u64,
    /// The option fee paid for the grant of the option, in SGD cents.
    pub option_fee_cents: u64,
    /// The option period, in days, within which the option must be exercised.
    pub option_period_days: u32,
    /// Whether the option has been exercised.
    pub exercised: bool,
    /// If exercised, the number of days after grant on which exercise occurred.
    pub exercised_on_day: Option<u32>,
}

impl OptionToPurchase {
    /// Creates an OTP on the standard private-resale terms: a 1% option fee and a
    /// 14-day option period (market conventions).
    pub fn private_resale(property: impl Into<String>, price_cents: u64) -> Self {
        Self {
            property: property.into(),
            price_cents,
            option_fee_cents: price_cents.saturating_mul(OPTION_FEE_PERCENT_PRIVATE_RESALE) / 100,
            option_period_days: DEFAULT_OPTION_PERIOD_DAYS,
            exercised: false,
            exercised_on_day: None,
        }
    }

    /// Creates an OTP with bespoke terms.
    pub fn new(
        property: impl Into<String>,
        price_cents: u64,
        option_fee_cents: u64,
        option_period_days: u32,
    ) -> Self {
        Self {
            property: property.into(),
            price_cents,
            option_fee_cents,
            option_period_days,
            exercised: false,
            exercised_on_day: None,
        }
    }

    /// Records exercise of the option on the given day after grant.
    pub fn exercise_on_day(mut self, day: u32) -> Self {
        self.exercised = true;
        self.exercised_on_day = Some(day);
        self
    }

    /// The conventional balance deposit payable on exercise (a further 4% of the
    /// price, to a 5% total), in SGD cents.
    pub fn balance_deposit_cents(&self) -> u64 {
        self.price_cents.saturating_mul(BALANCE_DEPOSIT_PERCENT) / 100
    }

    /// Whether the option was validly exercised: exercised, and within the option
    /// period.
    pub fn validly_exercised(&self) -> bool {
        match self.exercised_on_day {
            Some(day) => self.exercised && day <= self.option_period_days,
            None => false,
        }
    }
}

// ============================================================================
// Sale and purchase
// ============================================================================

/// A contract for the sale and purchase of land.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaleAndPurchase {
    /// The vendor (seller).
    pub vendor: String,
    /// The purchaser (buyer).
    pub purchaser: String,
    /// Description / reference of the property.
    pub property: String,
    /// Use class of the property (relevant to stamp duty).
    pub property_type: PropertyType,
    /// The purchase price, in SGD cents.
    pub price_cents: u64,
    /// The deposit paid, in SGD cents.
    pub deposit_cents: u64,
    /// The completion period, in weeks.
    pub completion_weeks: u32,
    /// Whether the contract is evidenced in writing and signed (Civil Law Act
    /// s. 6(d)).
    pub in_writing_and_signed: bool,
}

impl SaleAndPurchase {
    /// Creates a new, written sale and purchase contract on the conventional
    /// completion period.
    pub fn new(
        vendor: impl Into<String>,
        purchaser: impl Into<String>,
        property: impl Into<String>,
        property_type: PropertyType,
        price_cents: u64,
    ) -> Self {
        Self {
            vendor: vendor.into(),
            purchaser: purchaser.into(),
            property: property.into(),
            property_type,
            price_cents,
            deposit_cents: price_cents.saturating_mul(5) / 100,
            completion_weeks: TYPICAL_COMPLETION_WEEKS,
            in_writing_and_signed: true,
        }
    }

    /// Records that the contract is not evidenced in writing and signed.
    pub fn oral(mut self) -> Self {
        self.in_writing_and_signed = false;
        self
    }

    /// Sets the deposit paid.
    pub fn with_deposit_cents(mut self, deposit_cents: u64) -> Self {
        self.deposit_cents = deposit_cents;
        self
    }

    /// The Buyer's Stamp Duty payable on this purchase, in SGD cents.
    pub fn buyers_stamp_duty_cents(&self) -> u64 {
        compute_buyers_stamp_duty_cents(self.price_cents, self.property_type)
    }

    /// The balance of the purchase price payable on completion, in SGD cents.
    pub fn balance_on_completion_cents(&self) -> u64 {
        self.price_cents.saturating_sub(self.deposit_cents)
    }
}

/// The state of completion of a sale and purchase.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Completion {
    /// Whether the balance of the purchase price has been paid.
    pub balance_paid: bool,
    /// Whether the instrument of transfer has been duly executed and delivered.
    pub transfer_executed: bool,
    /// Whether vacant possession has been given.
    pub vacant_possession_given: bool,
}

impl Completion {
    /// Creates a pending completion (nothing done yet).
    pub fn pending() -> Self {
        Self {
            balance_paid: false,
            transfer_executed: false,
            vacant_possession_given: false,
        }
    }

    /// Records payment of the balance of the purchase price.
    pub fn with_balance_paid(mut self) -> Self {
        self.balance_paid = true;
        self
    }

    /// Records execution and delivery of the instrument of transfer.
    pub fn with_transfer_executed(mut self) -> Self {
        self.transfer_executed = true;
        self
    }

    /// Records that vacant possession has been given.
    pub fn with_vacant_possession(mut self) -> Self {
        self.vacant_possession_given = true;
        self
    }

    /// Whether completion has occurred: the balance is paid, the transfer
    /// executed and delivered, and vacant possession given.
    pub fn is_complete(&self) -> bool {
        self.balance_paid && self.transfer_executed && self.vacant_possession_given
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsd_residential_one_million() {
        // SGD 1,000,000 -> SGD 24,600 = 2,460,000 cents.
        let bsd = compute_buyers_stamp_duty_cents(100_000_000, PropertyType::Residential);
        assert_eq!(bsd, 2_460_000);
    }

    #[test]
    fn test_bsd_residential_higher_tiers() {
        // SGD 1,500,000 -> SGD 44,600.
        assert_eq!(
            compute_buyers_stamp_duty_cents(150_000_000, PropertyType::Residential),
            4_460_000
        );
        // SGD 3,000,000 -> SGD 119,600.
        assert_eq!(
            compute_buyers_stamp_duty_cents(300_000_000, PropertyType::Residential),
            11_960_000
        );
        // SGD 5,000,000 -> SGD 239,600.
        assert_eq!(
            compute_buyers_stamp_duty_cents(500_000_000, PropertyType::Residential),
            23_960_000
        );
    }

    #[test]
    fn test_bsd_non_residential() {
        // SGD 2,000,000 non-residential -> SGD 69,600.
        assert_eq!(
            compute_buyers_stamp_duty_cents(200_000_000, PropertyType::Commercial),
            6_960_000
        );
    }

    #[test]
    fn test_bsd_small_purchase() {
        // SGD 100,000 residential -> 1% = SGD 1,000.
        assert_eq!(
            compute_buyers_stamp_duty_cents(10_000_000, PropertyType::Residential),
            100_000
        );
    }

    #[test]
    fn test_otp_private_resale_fee_and_deposit() {
        // SGD 2,000,000 price -> 1% option fee = SGD 20,000; 4% balance = SGD 80,000.
        let otp = OptionToPurchase::private_resale("Condo #10-11", 200_000_000);
        assert_eq!(otp.option_fee_cents, 2_000_000);
        assert_eq!(otp.balance_deposit_cents(), 8_000_000);
        assert_eq!(otp.option_period_days, 14);
    }

    #[test]
    fn test_otp_valid_exercise() {
        let otp = OptionToPurchase::private_resale("Condo", 100_000_000).exercise_on_day(10);
        assert!(otp.validly_exercised());

        // Exercised after the option period lapsed.
        let late = OptionToPurchase::private_resale("Condo", 100_000_000).exercise_on_day(20);
        assert!(!late.validly_exercised());

        // Never exercised.
        let unexercised = OptionToPurchase::private_resale("Condo", 100_000_000);
        assert!(!unexercised.validly_exercised());
    }

    #[test]
    fn test_sale_and_purchase_balance_and_bsd() {
        let sap = SaleAndPurchase::new(
            "Vendor",
            "Purchaser",
            "Condo #10-11",
            PropertyType::Residential,
            200_000_000,
        );
        // 5% deposit -> SGD 100,000; balance SGD 1,900,000.
        assert_eq!(sap.deposit_cents, 10_000_000);
        assert_eq!(sap.balance_on_completion_cents(), 190_000_000);
        // BSD for SGD 2,000,000 residential = SGD 69,600.
        assert_eq!(sap.buyers_stamp_duty_cents(), 6_960_000);
    }

    #[test]
    fn test_completion_state() {
        let completion = Completion::pending()
            .with_balance_paid()
            .with_transfer_executed();
        assert!(!completion.is_complete());

        let done = completion.with_vacant_possession();
        assert!(done.is_complete());
    }

    #[test]
    fn test_conveyancing_serde_roundtrip() {
        let otp = OptionToPurchase::private_resale("Condo #10-11", 150_000_000).exercise_on_day(7);
        let json = serde_json::to_string(&otp).expect("serialize");
        let back: OptionToPurchase = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(otp, back);
    }
}
