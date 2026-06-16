//! Securities and Futures Act 2001 - Market Conduct (Part 12)
//!
//! Models the **market misconduct** prohibitions in **Part 12** of the
//! Securities and Futures Act 2001, enforced by MAS through criminal prosecution
//! and the civil penalty regime (s. 232):
//!
//! - **Insider trading** - prohibited conduct by a connected person (s. 218) and
//!   by any other person (s. 219) in possession of inside information. "Inside
//!   information" is information that is not generally available (s. 215) and
//!   that, if it were, a reasonable person would expect to have a material effect
//!   on price (s. 216).
//! - **False trading and market rigging** (s. 197) - transactions creating a
//!   false or misleading appearance of active trading or of the market price.
//! - **Employment of manipulative or deceptive devices** (s. 201) - the general
//!   anti-manipulation / securities-fraud prohibition.
//! - **False or misleading statements** (s. 199).
//! - **Fraudulent inducement to deal** (s. 200).
//!
//! Monetary values are stored as **SGD cents** (`u64`).

use serde::{Deserialize, Serialize};

// ============================================================================
// Enforcement thresholds (SFA Part 12 Division 4; s. 232 / s. 204 / s. 221)
// ============================================================================

/// Civil penalty multiple: the penalty must not exceed this multiple of the
/// profit gained or loss avoided (SFA s. 232(2)).
pub const CIVIL_PENALTY_PROFIT_MULTIPLE: u64 = 3;

/// Minimum civil penalty for an individual (SFA s. 232). SGD 50,000, in cents.
pub const CIVIL_PENALTY_MIN_INDIVIDUAL_CENTS: u64 = 5_000_000;

/// Minimum civil penalty for a body corporate / non-individual (SFA s. 232).
/// SGD 100,000, in cents.
pub const CIVIL_PENALTY_MIN_CORPORATION_CENTS: u64 = 10_000_000;

/// Maximum civil penalty where the contravention produced no profit/loss avoided
/// (SFA s. 232). SGD 2,000,000, in cents.
pub const CIVIL_PENALTY_NO_PROFIT_MAX_CENTS: u64 = 200_000_000;

/// Maximum criminal fine for a market-misconduct offence (SFA s. 204/s. 221).
/// SGD 250,000, in cents.
pub const CRIMINAL_FINE_MAX_CENTS: u64 = 25_000_000;

/// Maximum term of imprisonment, in years, for a market-misconduct offence
/// (SFA s. 204/s. 221).
pub const CRIMINAL_IMPRISONMENT_MAX_YEARS: u32 = 7;

/// Computes the maximum civil penalty under SFA s. 232.
///
/// Where the contravention produced a profit gained or loss avoided, the cap is
/// the greater of three times that amount and the statutory minimum (SGD 50,000
/// for an individual, SGD 100,000 for a body corporate). Where no profit was
/// gained or loss avoided, the cap is [`CIVIL_PENALTY_NO_PROFIT_MAX_CENTS`]
/// (SGD 2 million).
///
/// All amounts are in SGD cents. The arithmetic is saturating to avoid overflow.
///
/// # Examples
///
/// ```
/// use legalis_sg::securities::max_civil_penalty_cents;
///
/// // Profit of SGD 1,000,000 by an individual -> cap is 3x = SGD 3,000,000.
/// assert_eq!(max_civil_penalty_cents(100_000_000, true), 300_000_000);
///
/// // No profit by a corporation -> cap is SGD 2,000,000.
/// assert_eq!(max_civil_penalty_cents(0, false), 200_000_000);
/// ```
pub fn max_civil_penalty_cents(profit_or_loss_avoided_cents: u64, is_individual: bool) -> u64 {
    let minimum = if is_individual {
        CIVIL_PENALTY_MIN_INDIVIDUAL_CENTS
    } else {
        CIVIL_PENALTY_MIN_CORPORATION_CENTS
    };
    if profit_or_loss_avoided_cents == 0 {
        return CIVIL_PENALTY_NO_PROFIT_MAX_CENTS.max(minimum);
    }
    let triple = profit_or_loss_avoided_cents.saturating_mul(CIVIL_PENALTY_PROFIT_MULTIPLE);
    triple.max(minimum)
}

// ============================================================================
// Insider trading (SFA Division 3; s. 214-219)
// ============================================================================

/// The prohibited conduct elements of insider trading (SFA s. 218(2)/s. 219(2)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsiderConduct {
    /// Dealing in the capital markets products to which the information relates.
    Dealt,
    /// Procuring another person to deal in those products.
    ProcuredDealing,
    /// Communicating ("tipping") the information to another person likely to deal.
    Communicated,
}

impl InsiderConduct {
    /// Returns a plain-language description of the conduct.
    pub fn description(&self) -> &'static str {
        match self {
            InsiderConduct::Dealt => "Dealing in the capital markets products",
            InsiderConduct::ProcuredDealing => "Procuring another person to deal",
            InsiderConduct::Communicated => "Communicating (tipping) the inside information",
        }
    }
}

/// A claim of insider trading under SFA s. 218 (connected person) or s. 219 (any
/// other person).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsiderTradingClaim {
    /// Identifier for the claim.
    pub claim_id: String,
    /// Whether the person is a "connected person" (officer/employee/etc.), which
    /// engages s. 218 rather than s. 219.
    pub connected_person: bool,
    /// Whether the person is in possession of the information.
    pub in_possession: bool,
    /// Whether the information is generally available (s. 215). If it is, it is
    /// not inside information.
    pub generally_available: bool,
    /// Whether a reasonable person would expect the information, if generally
    /// available, to have a material effect on price or value (s. 216).
    pub material_effect_on_price: bool,
    /// Whether the person knew (or ought reasonably to have known) that the
    /// information was inside information.
    pub knew_information_was_inside: bool,
    /// The prohibited conduct engaged in.
    pub conduct: InsiderConduct,
}

impl InsiderTradingClaim {
    /// Creates a new insider trading claim with the elements made out by default
    /// (in possession of inside information, dealing).
    pub fn new(claim_id: impl Into<String>, connected_person: bool) -> Self {
        Self {
            claim_id: claim_id.into(),
            connected_person,
            in_possession: true,
            generally_available: false,
            material_effect_on_price: true,
            knew_information_was_inside: true,
            conduct: InsiderConduct::Dealt,
        }
    }

    /// Sets the prohibited conduct.
    pub fn with_conduct(mut self, conduct: InsiderConduct) -> Self {
        self.conduct = conduct;
        self
    }

    /// Marks the information as generally available (s. 215), which takes it
    /// outside the definition of inside information.
    pub fn generally_available(mut self) -> Self {
        self.generally_available = true;
        self
    }

    /// Sets whether a reasonable person would expect a material effect on price
    /// (s. 216).
    pub fn with_material_effect(mut self, value: bool) -> Self {
        self.material_effect_on_price = value;
        self
    }

    /// Whether the information held is "inside information": not generally
    /// available (s. 215) and price-material (s. 216).
    pub fn is_inside_information(&self) -> bool {
        self.in_possession && !self.generally_available && self.material_effect_on_price
    }

    /// Returns the applicable section: s. 218 for a connected person, otherwise
    /// s. 219.
    pub fn applicable_section(&self) -> &'static str {
        if self.connected_person {
            "s. 218"
        } else {
            "s. 219"
        }
    }

    /// Whether insider trading is made out on the recorded facts.
    pub fn is_made_out(&self) -> bool {
        self.is_inside_information() && self.knew_information_was_inside
    }
}

// ============================================================================
// False trading and market rigging (SFA s. 197)
// ============================================================================

/// A claim of false trading or market rigging transactions under SFA s. 197.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FalseTradingClaim {
    /// Identifier for the claim.
    pub claim_id: String,
    /// Whether the conduct creates, or is likely to create, a false or
    /// misleading appearance of active trading (s. 197(1)(a)).
    pub false_appearance_of_active_trading: bool,
    /// Whether the conduct involves a wash trade (a transaction that does not
    /// change beneficial ownership) or matched orders (s. 197(3)).
    pub wash_trade_or_matched_orders: bool,
    /// Whether the conduct creates, or is likely to create, a false or
    /// misleading appearance with respect to the market for, or price of, the
    /// products (s. 197(1)(b)).
    pub false_appearance_of_market_or_price: bool,
}

impl FalseTradingClaim {
    /// Creates a new false-trading claim (false appearance of active trading by
    /// default).
    pub fn new(claim_id: impl Into<String>) -> Self {
        Self {
            claim_id: claim_id.into(),
            false_appearance_of_active_trading: true,
            wash_trade_or_matched_orders: false,
            false_appearance_of_market_or_price: false,
        }
    }

    /// Records that the conduct involves a wash trade or matched orders.
    pub fn with_wash_trade(mut self) -> Self {
        self.wash_trade_or_matched_orders = true;
        self.false_appearance_of_active_trading = true;
        self
    }

    /// Records a false or misleading appearance as to the market or price.
    pub fn with_false_market_or_price(mut self) -> Self {
        self.false_appearance_of_market_or_price = true;
        self
    }

    /// Whether false trading / market rigging is made out (any false appearance
    /// of active trading or of the market/price).
    pub fn is_made_out(&self) -> bool {
        self.false_appearance_of_active_trading
            || self.wash_trade_or_matched_orders
            || self.false_appearance_of_market_or_price
    }
}

// ============================================================================
// Employment of manipulative or deceptive devices (SFA s. 201)
// ============================================================================

/// A claim that a person employed a manipulative or deceptive device in
/// connection with capital markets products (SFA s. 201) - the general
/// anti-manipulation / securities-fraud prohibition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketManipulationClaim {
    /// Identifier for the claim.
    pub claim_id: String,
    /// Whether a manipulative or deceptive device, scheme or artifice was used
    /// (s. 201(b)).
    pub manipulative_or_deceptive_device: bool,
    /// Whether the conduct was in connection with the subscription, purchase or
    /// sale of capital markets products.
    pub in_connection_with_products: bool,
}

impl MarketManipulationClaim {
    /// Creates a new market-manipulation claim with the elements made out by
    /// default.
    pub fn new(claim_id: impl Into<String>) -> Self {
        Self {
            claim_id: claim_id.into(),
            manipulative_or_deceptive_device: true,
            in_connection_with_products: true,
        }
    }

    /// Sets whether a manipulative or deceptive device was employed.
    pub fn with_device(mut self, value: bool) -> Self {
        self.manipulative_or_deceptive_device = value;
        self
    }

    /// Whether s. 201 is made out: a manipulative/deceptive device employed in
    /// connection with capital markets products.
    pub fn is_made_out(&self) -> bool {
        self.manipulative_or_deceptive_device && self.in_connection_with_products
    }
}

// ============================================================================
// False/misleading statements (s. 199) and fraudulent inducement (s. 200)
// ============================================================================

/// A claim of making a false or misleading statement likely to induce dealing or
/// to affect the price of capital markets products (SFA s. 199).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MisleadingStatementClaim {
    /// Identifier for the claim.
    pub claim_id: String,
    /// Whether the statement (or disseminated information) is false or
    /// misleading in a material particular.
    pub statement_false_or_misleading: bool,
    /// Whether the statement is likely to induce dealing or to have a material
    /// effect on price (s. 199(a)/(b)).
    pub likely_to_induce_or_affect_price: bool,
    /// Whether the maker knew, or ought reasonably to have known, the statement
    /// was false or misleading, or was reckless or negligent (s. 199).
    pub knew_or_ought_to_have_known: bool,
}

impl MisleadingStatementClaim {
    /// Creates a new misleading-statement claim with the elements made out by
    /// default.
    pub fn new(claim_id: impl Into<String>) -> Self {
        Self {
            claim_id: claim_id.into(),
            statement_false_or_misleading: true,
            likely_to_induce_or_affect_price: true,
            knew_or_ought_to_have_known: true,
        }
    }

    /// Whether s. 199 is made out.
    pub fn is_made_out(&self) -> bool {
        self.statement_false_or_misleading
            && self.likely_to_induce_or_affect_price
            && self.knew_or_ought_to_have_known
    }
}

/// A claim of fraudulently or deceptively inducing another person to deal in
/// capital markets products (SFA s. 200).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FraudulentInducementClaim {
    /// Identifier for the claim.
    pub claim_id: String,
    /// Whether the inducement was by a dishonest concealment of material facts,
    /// a reckless statement, or a fraudulent device (s. 200(a)-(c)).
    pub dishonest_or_fraudulent_means: bool,
    /// Whether another person was induced (or attempted to be induced) to deal.
    pub induced_dealing: bool,
}

impl FraudulentInducementClaim {
    /// Creates a new fraudulent-inducement claim with the elements made out by
    /// default.
    pub fn new(claim_id: impl Into<String>) -> Self {
        Self {
            claim_id: claim_id.into(),
            dishonest_or_fraudulent_means: true,
            induced_dealing: true,
        }
    }

    /// Whether s. 200 is made out.
    pub fn is_made_out(&self) -> bool {
        self.dishonest_or_fraudulent_means && self.induced_dealing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_civil_penalty_with_profit() {
        // Individual, profit SGD 1m -> 3x = SGD 3m (above the SGD 50k minimum).
        assert_eq!(max_civil_penalty_cents(100_000_000, true), 300_000_000);
    }

    #[test]
    fn test_max_civil_penalty_minimum_floor() {
        // Tiny profit, individual: 3x would be below the SGD 50k minimum.
        assert_eq!(
            max_civil_penalty_cents(100, true),
            CIVIL_PENALTY_MIN_INDIVIDUAL_CENTS
        );
        // Tiny profit, corporation: SGD 100k minimum.
        assert_eq!(
            max_civil_penalty_cents(100, false),
            CIVIL_PENALTY_MIN_CORPORATION_CENTS
        );
    }

    #[test]
    fn test_max_civil_penalty_no_profit() {
        assert_eq!(
            max_civil_penalty_cents(0, true),
            CIVIL_PENALTY_NO_PROFIT_MAX_CENTS
        );
        assert_eq!(
            max_civil_penalty_cents(0, false),
            CIVIL_PENALTY_NO_PROFIT_MAX_CENTS
        );
    }

    #[test]
    fn test_insider_inside_information() {
        let claim = InsiderTradingClaim::new("it-1", true);
        assert!(claim.is_inside_information());
        assert!(claim.is_made_out());
        assert_eq!(claim.applicable_section(), "s. 218");

        // Generally available information is not inside information.
        let public = InsiderTradingClaim::new("it-2", true).generally_available();
        assert!(!public.is_inside_information());
        assert!(!public.is_made_out());
    }

    #[test]
    fn test_insider_non_connected_uses_s219() {
        let claim =
            InsiderTradingClaim::new("it-3", false).with_conduct(InsiderConduct::Communicated);
        assert_eq!(claim.applicable_section(), "s. 219");
        assert!(claim.is_made_out());
    }

    #[test]
    fn test_insider_not_material_not_made_out() {
        let claim = InsiderTradingClaim::new("it-4", true).with_material_effect(false);
        assert!(!claim.is_made_out());
    }

    #[test]
    fn test_false_trading_wash_trade() {
        let claim = FalseTradingClaim::new("ft-1").with_wash_trade();
        assert!(claim.is_made_out());
        assert!(claim.wash_trade_or_matched_orders);
    }

    #[test]
    fn test_market_manipulation() {
        let claim = MarketManipulationClaim::new("mm-1");
        assert!(claim.is_made_out());
        assert!(
            !MarketManipulationClaim::new("mm-2")
                .with_device(false)
                .is_made_out()
        );
    }

    #[test]
    fn test_misleading_statement_and_inducement() {
        assert!(MisleadingStatementClaim::new("ms-1").is_made_out());
        assert!(FraudulentInducementClaim::new("fi-1").is_made_out());
    }

    #[test]
    fn test_misconduct_serde_roundtrip() {
        let claim =
            InsiderTradingClaim::new("it-rt", true).with_conduct(InsiderConduct::ProcuredDealing);
        let json = serde_json::to_string(&claim).expect("serialize");
        let back: InsiderTradingClaim = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(claim, back);
    }
}
