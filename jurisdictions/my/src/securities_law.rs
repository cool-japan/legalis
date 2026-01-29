//! Capital Markets and Services Act 2007
//!
//! Malaysian securities and capital markets regulation.
//!
//! # Key Provisions
//!
//! - **Licensing**: Requirement for capital market intermediaries
//! - **Disclosure**: Prospectus requirements for IPOs
//! - **Market conduct**: Insider trading, market manipulation prohibitions
//! - **Corporate governance**: Listed company requirements
//!
//! # Administration
//!
//! - **SC**: Securities Commission Malaysia
//! - **Bursa Malaysia**: Stock exchange

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Securities law error types.
#[derive(Debug, Error)]
pub enum SecuritiesError {
    /// Unlicensed activity.
    #[error("Unlicensed capital market activity: {activity}")]
    UnlicensedActivity { activity: String },

    /// Insider trading.
    #[error("Insider trading violation")]
    InsiderTrading,

    /// Market manipulation.
    #[error("Market manipulation: {description}")]
    MarketManipulation { description: String },

    /// Disclosure violation.
    #[error("Disclosure violation: {requirement}")]
    DisclosureViolation { requirement: String },
}

/// Result type for securities law operations.
pub type Result<T> = std::result::Result<T, SecuritiesError>;

/// Type of capital market license.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseType {
    /// Dealing in securities.
    DealingSecurities,
    /// Fund management.
    FundManagement,
    /// Investment advice.
    InvestmentAdvice,
    /// Financial planning.
    FinancialPlanning,
    /// Underwriting.
    Underwriting,
}

/// Licensed capital market intermediary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LicensedIntermediary {
    /// Entity ID.
    pub id: Uuid,
    /// Entity name.
    pub name: String,
    /// License number.
    pub license_number: String,
    /// License types held.
    pub licenses: Vec<LicenseType>,
    /// Whether license is active.
    pub active: bool,
    /// Issue date.
    pub issue_date: DateTime<Utc>,
}

impl LicensedIntermediary {
    /// Creates a new licensed intermediary.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        license_number: impl Into<String>,
        licenses: Vec<LicenseType>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            license_number: license_number.into(),
            licenses,
            active: true,
            issue_date: Utc::now(),
        }
    }

    /// Checks if intermediary is licensed for an activity.
    #[must_use]
    pub fn is_licensed_for(&self, license_type: LicenseType) -> bool {
        self.active && self.licenses.contains(&license_type)
    }
}

/// Listed company on Bursa Malaysia.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListedCompany {
    /// Company ID.
    pub id: Uuid,
    /// Company name.
    pub name: String,
    /// Stock code.
    pub stock_code: String,
    /// Market (Main Market, ACE Market, LEAP Market).
    pub market: Market,
    /// Market capitalization in sen.
    pub market_cap_sen: i64,
    /// Whether company is compliant with listing requirements.
    pub compliant: bool,
}

/// Bursa Malaysia market.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Market {
    /// Main Market (large cap).
    MainMarket,
    /// ACE Market (growth companies).
    AceMarket,
    /// LEAP Market (sophisticated investors).
    LeapMarket,
}

impl ListedCompany {
    /// Creates a new listed company.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        stock_code: impl Into<String>,
        market: Market,
        market_cap_sen: i64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            stock_code: stock_code.into(),
            market,
            market_cap_sen,
            compliant: true,
        }
    }
}

/// IPO (Initial Public Offering) prospectus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Prospectus {
    /// Prospectus ID.
    pub id: Uuid,
    /// Issuing company.
    pub company_name: String,
    /// Offer price in sen per share.
    pub offer_price_sen: i64,
    /// Number of shares offered.
    pub shares_offered: u64,
    /// Whether prospectus has been approved by SC.
    pub sc_approved: bool,
    /// Issue date.
    pub issue_date: DateTime<Utc>,
}

impl Prospectus {
    /// Creates a new prospectus.
    #[must_use]
    pub fn new(company_name: impl Into<String>, offer_price_sen: i64, shares_offered: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_name: company_name.into(),
            offer_price_sen,
            shares_offered,
            sc_approved: false,
            issue_date: Utc::now(),
        }
    }

    /// Sets SC approval status.
    #[must_use]
    pub fn with_sc_approval(mut self, approved: bool) -> Self {
        self.sc_approved = approved;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_licensed_intermediary() {
        let intermediary = LicensedIntermediary::new(
            "Securities Firm Sdn Bhd",
            "SC-12345",
            vec![
                LicenseType::DealingSecurities,
                LicenseType::InvestmentAdvice,
            ],
        );

        assert!(intermediary.is_licensed_for(LicenseType::DealingSecurities));
        assert!(intermediary.is_licensed_for(LicenseType::InvestmentAdvice));
        assert!(!intermediary.is_licensed_for(LicenseType::FundManagement));
    }

    #[test]
    fn test_listed_company() {
        let company = ListedCompany::new(
            "Tech Corporation Bhd",
            "TECH",
            Market::MainMarket,
            50_000_000_000, // RM 5 billion
        );

        assert_eq!(company.stock_code, "TECH");
        assert_eq!(company.market, Market::MainMarket);
        assert!(company.compliant);
    }

    #[test]
    fn test_prospectus() {
        let prospectus = Prospectus::new(
            "NewCo Bhd",
            10_000,   // RM 1.00 per share
            10000000, // 10 million shares
        )
        .with_sc_approval(true);

        assert_eq!(prospectus.offer_price_sen, 10_000);
        assert!(prospectus.sc_approved);
    }
}
