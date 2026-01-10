//! Banking Act (Cap. 19) - Type Definitions
//!
//! This module provides type-safe representations of banking entities and structures
//! regulated under Singapore's Banking Act (Cap. 19) and MAS Notice 637 (Basel III).
//!
//! # Key Concepts
//!
//! ## Banking Licenses
//! - **Full Bank**: Can accept deposits of any size, full range of banking services
//! - **Wholesale Bank**: Cannot take deposits < SGD 250,000
//! - **Merchant Bank**: Investment banking, cannot take retail deposits
//!
//! ## Basel III Capital Adequacy
//! - **CET1 (Common Equity Tier 1)**: ≥ 6.5% minimum
//! - **Tier 1 Capital**: CET1 + AT1 ≥ 8.0% minimum
//! - **Total Capital**: Tier 1 + Tier 2 ≥ 10.0% minimum
//! - **RWA (Risk-Weighted Assets)**: Assets weighted by credit, market, operational risk
//!
//! ## AML/CFT Requirements
//! - Customer Due Diligence (CDD) - MAS Notice 626
//! - Enhanced Due Diligence (EDD) for high-risk customers
//! - Suspicious Transaction Reporting (STR) to STRO
//!
//! # References
//! - Banking Act (Cap. 19): <https://sso.agc.gov.sg/Act/BA1970>
//! - MAS Notice 637 (Risk Based Capital Adequacy): <https://www.mas.gov.sg/regulation/notices/notice-637>
//! - MAS Notice 626 (Prevention of Money Laundering): <https://www.mas.gov.sg/regulation/notices/notice-626>

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Type of banking license issued by MAS
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BankLicenseType {
    /// Full bank license - can accept deposits of any size (Banking Act s. 4)
    FullBank,
    /// Wholesale bank license - minimum deposit SGD 250,000 (Banking Act s. 4)
    WholesaleBank,
    /// Merchant bank - investment banking only, no retail deposits (s. 28)
    MerchantBank,
}

/// Status of a banking license
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseStatus {
    /// License is active and in good standing
    Active,
    /// License is under suspension by MAS
    Suspended,
    /// License has been revoked
    Revoked,
    /// License is under review or investigation
    UnderReview,
}

/// Risk category for customer due diligence (MAS Notice 626)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomerRiskCategory {
    /// Low risk - standard CDD required
    Low,
    /// Medium risk - enhanced monitoring
    Medium,
    /// High risk - Enhanced Due Diligence (EDD) required
    High,
    /// Politically Exposed Person (PEP) - EDD mandatory
    PoliticallyExposed,
}

/// Type of capital under Basel III framework
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapitalType {
    /// Common Equity Tier 1 - highest quality capital
    CommonEquityTier1,
    /// Additional Tier 1 - e.g., perpetual preferred stock
    AdditionalTier1,
    /// Tier 2 - subordinated debt, hybrid instruments
    Tier2,
}

/// A banking institution licensed under the Banking Act
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bank {
    /// Unique Entity Number (UEN) assigned by ACRA
    pub uen: String,
    /// Registered name of the bank
    pub name: String,
    /// Type of banking license held
    pub license_type: BankLicenseType,
    /// Current status of the license
    pub license_status: LicenseStatus,
    /// Date the license was granted
    pub license_date: DateTime<Utc>,
    /// Date of license expiry (if applicable)
    pub license_expiry: Option<DateTime<Utc>>,
    /// Whether the bank is locally incorporated
    pub is_locally_incorporated: bool,
    /// Parent company name (if foreign bank)
    pub parent_company: Option<String>,
    /// Country of incorporation
    pub country_of_incorporation: String,
    /// Capital adequacy information
    pub capital_adequacy: CapitalAdequacy,
    /// AML/CFT compliance officer details
    pub aml_officer: Option<ComplianceOfficer>,
    /// Total assets in SGD cents
    pub total_assets_sgd: u64,
    /// Total deposits in SGD cents
    pub total_deposits_sgd: u64,
}

/// Capital adequacy ratio (CAR) information - Basel III framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalAdequacy {
    /// Common Equity Tier 1 capital in SGD cents
    pub cet1_capital_sgd: u64,
    /// Additional Tier 1 capital in SGD cents
    pub at1_capital_sgd: u64,
    /// Tier 2 capital in SGD cents
    pub tier2_capital_sgd: u64,
    /// Risk-Weighted Assets in SGD cents
    pub risk_weighted_assets_sgd: u64,
    /// Date of calculation
    pub calculation_date: DateTime<Utc>,
}

impl CapitalAdequacy {
    /// Calculate Common Equity Tier 1 (CET1) ratio
    ///
    /// # Formula
    /// CET1 Ratio = (CET1 Capital / RWA) × 100
    ///
    /// # Regulatory Minimum
    /// - MAS Notice 637: 6.5% minimum (includes capital conservation buffer)
    pub fn cet1_ratio(&self) -> f64 {
        if self.risk_weighted_assets_sgd == 0 {
            return 0.0;
        }
        (self.cet1_capital_sgd as f64 / self.risk_weighted_assets_sgd as f64) * 100.0
    }

    /// Calculate Tier 1 capital ratio
    ///
    /// # Formula
    /// Tier 1 Ratio = (CET1 + AT1) / RWA × 100
    ///
    /// # Regulatory Minimum
    /// - MAS Notice 637: 8.0% minimum
    pub fn tier1_ratio(&self) -> f64 {
        if self.risk_weighted_assets_sgd == 0 {
            return 0.0;
        }
        let tier1_capital = self.cet1_capital_sgd + self.at1_capital_sgd;
        (tier1_capital as f64 / self.risk_weighted_assets_sgd as f64) * 100.0
    }

    /// Calculate Total Capital Adequacy Ratio (CAR)
    ///
    /// # Formula
    /// Total CAR = (CET1 + AT1 + Tier 2) / RWA × 100
    ///
    /// # Regulatory Minimum
    /// - MAS Notice 637: 10.0% minimum
    pub fn total_capital_ratio(&self) -> f64 {
        if self.risk_weighted_assets_sgd == 0 {
            return 0.0;
        }
        let total_capital = self.cet1_capital_sgd + self.at1_capital_sgd + self.tier2_capital_sgd;
        (total_capital as f64 / self.risk_weighted_assets_sgd as f64) * 100.0
    }

    /// Convert CET1 capital to SGD (from cents)
    pub fn cet1_capital_sgd_amount(&self) -> f64 {
        self.cet1_capital_sgd as f64 / 100.0
    }

    /// Convert AT1 capital to SGD (from cents)
    pub fn at1_capital_sgd_amount(&self) -> f64 {
        self.at1_capital_sgd as f64 / 100.0
    }

    /// Convert Tier 2 capital to SGD (from cents)
    pub fn tier2_capital_sgd_amount(&self) -> f64 {
        self.tier2_capital_sgd as f64 / 100.0
    }

    /// Convert RWA to SGD (from cents)
    pub fn rwa_sgd_amount(&self) -> f64 {
        self.risk_weighted_assets_sgd as f64 / 100.0
    }

    /// Check if capital adequacy meets MAS Notice 637 requirements
    ///
    /// # Regulatory Minimums
    /// - CET1: ≥ 6.5%
    /// - Tier 1: ≥ 8.0%
    /// - Total: ≥ 10.0%
    pub fn meets_regulatory_minimum(&self) -> bool {
        self.cet1_ratio() >= 6.5 && self.tier1_ratio() >= 8.0 && self.total_capital_ratio() >= 10.0
    }
}

/// AML/CFT compliance officer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceOfficer {
    /// Full name of the compliance officer
    pub name: String,
    /// Singapore NRIC or foreign identification
    pub identification: String,
    /// Contact email
    pub email: String,
    /// Contact phone number
    pub phone: String,
    /// Date of appointment
    pub appointed_date: DateTime<Utc>,
    /// Professional qualifications (e.g., ACAMS, ICA)
    pub qualifications: Vec<String>,
}

/// Customer account with risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAccount {
    /// Unique account number
    pub account_number: String,
    /// Customer name
    pub customer_name: String,
    /// Customer identification (NRIC/Passport)
    pub customer_id: String,
    /// Risk category for AML/CFT purposes
    pub risk_category: CustomerRiskCategory,
    /// Date account was opened
    pub account_opened: DateTime<Utc>,
    /// Date of last CDD review
    pub last_cdd_review: DateTime<Utc>,
    /// Whether Enhanced Due Diligence was performed
    pub edd_performed: bool,
    /// Source of funds documented
    pub source_of_funds_verified: bool,
    /// Beneficial owner information collected (for corporate accounts)
    pub beneficial_owner_identified: bool,
    /// Account balance in SGD cents
    pub balance_sgd: u64,
}

impl CustomerAccount {
    /// Check if CDD review is overdue (must be reviewed annually for high risk, every 3 years for others)
    pub fn is_cdd_overdue(&self, current_date: DateTime<Utc>) -> bool {
        let days_since_review = (current_date - self.last_cdd_review).num_days();

        match self.risk_category {
            CustomerRiskCategory::High | CustomerRiskCategory::PoliticallyExposed => {
                days_since_review > 365 // Annual review required
            }
            CustomerRiskCategory::Medium => {
                days_since_review > 730 // Biennial review
            }
            CustomerRiskCategory::Low => {
                days_since_review > 1095 // Triennial review (3 years)
            }
        }
    }

    /// Check if EDD is required but not performed
    pub fn requires_edd(&self) -> bool {
        matches!(
            self.risk_category,
            CustomerRiskCategory::High | CustomerRiskCategory::PoliticallyExposed
        )
    }
}

/// Suspicious transaction report (STR) submitted to STRO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousTransactionReport {
    /// Unique reference number for the STR
    pub reference_number: String,
    /// Account involved
    pub account_number: String,
    /// Customer name
    pub customer_name: String,
    /// Transaction amount in SGD cents
    pub transaction_amount_sgd: u64,
    /// Date of suspicious transaction
    pub transaction_date: DateTime<Utc>,
    /// Date STR was filed with STRO
    pub filing_date: DateTime<Utc>,
    /// Nature of suspicion (structured transactions, unusual patterns, etc.)
    pub suspicion_description: String,
    /// Whether transaction was allowed to proceed
    pub transaction_proceeded: bool,
}

impl SuspiciousTransactionReport {
    /// Check if STR was filed within regulatory timeframe
    ///
    /// # Regulatory Requirement
    /// MAS Notice 626: File STR "as soon as is reasonably practicable" after suspicion arises.
    /// Generally interpreted as within 5-10 business days.
    pub fn filed_timely(&self) -> bool {
        let days_to_file = (self.filing_date - self.transaction_date).num_days();
        days_to_file <= 10
    }
}

/// Large cash transaction report (CTR) - for transactions ≥ SGD 20,000
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashTransactionReport {
    /// Account number
    pub account_number: String,
    /// Customer name
    pub customer_name: String,
    /// Customer identification
    pub customer_id: String,
    /// Transaction amount in SGD cents
    pub amount_sgd: u64,
    /// Transaction type (deposit, withdrawal)
    pub transaction_type: CashTransactionType,
    /// Date of transaction
    pub transaction_date: DateTime<Utc>,
    /// Purpose of transaction
    pub purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CashTransactionType {
    /// Cash deposit
    Deposit,
    /// Cash withdrawal
    Withdrawal,
}

impl CashTransactionReport {
    /// Check if transaction meets CTR reporting threshold
    ///
    /// # Regulatory Threshold
    /// MAS Notice 626: Report cash transactions ≥ SGD 20,000
    pub fn meets_reporting_threshold(&self) -> bool {
        self.amount_sgd >= 2_000_000 // SGD 20,000 in cents
    }

    /// Convert amount to SGD (from cents)
    pub fn amount_in_sgd(&self) -> f64 {
        self.amount_sgd as f64 / 100.0
    }
}

impl Bank {
    /// Create a new bank with basic information
    pub fn new(
        uen: String,
        name: String,
        license_type: BankLicenseType,
        license_date: DateTime<Utc>,
        is_locally_incorporated: bool,
        country_of_incorporation: String,
        capital_adequacy: CapitalAdequacy,
    ) -> Self {
        Bank {
            uen,
            name,
            license_type,
            license_status: LicenseStatus::Active,
            license_date,
            license_expiry: None,
            is_locally_incorporated,
            parent_company: None,
            country_of_incorporation,
            capital_adequacy,
            aml_officer: None,
            total_assets_sgd: 0,
            total_deposits_sgd: 0,
        }
    }

    /// Check if license is currently valid
    pub fn has_valid_license(&self, current_date: DateTime<Utc>) -> bool {
        if self.license_status != LicenseStatus::Active {
            return false;
        }

        if let Some(expiry) = self.license_expiry {
            current_date < expiry
        } else {
            true // No expiry date means perpetual license
        }
    }

    /// Calculate total deposits to total assets ratio (Deposit-Asset Ratio)
    pub fn deposit_asset_ratio(&self) -> f64 {
        if self.total_assets_sgd == 0 {
            return 0.0;
        }
        (self.total_deposits_sgd as f64 / self.total_assets_sgd as f64) * 100.0
    }

    /// Convert total assets to SGD (from cents)
    pub fn total_assets_in_sgd(&self) -> f64 {
        self.total_assets_sgd as f64 / 100.0
    }

    /// Convert total deposits to SGD (from cents)
    pub fn total_deposits_in_sgd(&self) -> f64 {
        self.total_deposits_sgd as f64 / 100.0
    }
}

/// Builder for Bank with validation
pub struct BankBuilder {
    uen: Option<String>,
    name: Option<String>,
    license_type: Option<BankLicenseType>,
    license_date: Option<DateTime<Utc>>,
    is_locally_incorporated: bool,
    country_of_incorporation: Option<String>,
    capital_adequacy: Option<CapitalAdequacy>,
    parent_company: Option<String>,
    aml_officer: Option<ComplianceOfficer>,
    total_assets_sgd: u64,
    total_deposits_sgd: u64,
}

impl BankBuilder {
    pub fn new() -> Self {
        BankBuilder {
            uen: None,
            name: None,
            license_type: None,
            license_date: None,
            is_locally_incorporated: false,
            country_of_incorporation: None,
            capital_adequacy: None,
            parent_company: None,
            aml_officer: None,
            total_assets_sgd: 0,
            total_deposits_sgd: 0,
        }
    }

    pub fn uen(mut self, uen: String) -> Self {
        self.uen = Some(uen);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn license_type(mut self, license_type: BankLicenseType) -> Self {
        self.license_type = Some(license_type);
        self
    }

    pub fn license_date(mut self, date: DateTime<Utc>) -> Self {
        self.license_date = Some(date);
        self
    }

    pub fn locally_incorporated(mut self, is_local: bool) -> Self {
        self.is_locally_incorporated = is_local;
        self
    }

    pub fn country_of_incorporation(mut self, country: String) -> Self {
        self.country_of_incorporation = Some(country);
        self
    }

    pub fn capital_adequacy(mut self, capital: CapitalAdequacy) -> Self {
        self.capital_adequacy = Some(capital);
        self
    }

    pub fn parent_company(mut self, parent: String) -> Self {
        self.parent_company = Some(parent);
        self
    }

    pub fn aml_officer(mut self, officer: ComplianceOfficer) -> Self {
        self.aml_officer = Some(officer);
        self
    }

    pub fn total_assets_sgd(mut self, amount: u64) -> Self {
        self.total_assets_sgd = amount;
        self
    }

    pub fn total_deposits_sgd(mut self, amount: u64) -> Self {
        self.total_deposits_sgd = amount;
        self
    }

    pub fn build(self) -> Result<Bank, &'static str> {
        let uen = self.uen.ok_or("UEN is required")?;
        let name = self.name.ok_or("Bank name is required")?;
        let license_type = self.license_type.ok_or("License type is required")?;
        let license_date = self.license_date.ok_or("License date is required")?;
        let country_of_incorporation = self
            .country_of_incorporation
            .ok_or("Country of incorporation is required")?;
        let capital_adequacy = self
            .capital_adequacy
            .ok_or("Capital adequacy data is required")?;

        Ok(Bank {
            uen,
            name,
            license_type,
            license_status: LicenseStatus::Active,
            license_date,
            license_expiry: None,
            is_locally_incorporated: self.is_locally_incorporated,
            parent_company: self.parent_company,
            country_of_incorporation,
            capital_adequacy,
            aml_officer: self.aml_officer,
            total_assets_sgd: self.total_assets_sgd,
            total_deposits_sgd: self.total_deposits_sgd,
        })
    }
}

impl Default for BankBuilder {
    fn default() -> Self {
        Self::new()
    }
}
