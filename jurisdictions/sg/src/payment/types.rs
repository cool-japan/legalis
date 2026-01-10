//! Payment Services Act 2019 - Type Definitions
//!
//! This module provides type-safe representations of payment service providers and
//! digital payment tokens regulated under Singapore's Payment Services Act 2019.
//!
//! # Key Concepts
//!
//! ## Payment Services (Section 3)
//!
//! Seven types of payment services regulated under the Act:
//!
//! 1. **Account Issuance Service**: Issuing payment accounts (e.g., e-wallets)
//! 2. **Domestic Money Transfer**: Transferring money within Singapore
//! 3. **Cross-Border Money Transfer**: International remittances
//! 4. **Merchant Acquisition**: Accepting card/digital payments for merchants
//! 5. **E-Money Issuance**: Stored value facilities (prepaid cards, e-wallets)
//! 6. **Digital Payment Token (DPT) Service**: Crypto exchange, wallet services
//! 7. **Money-Changing**: Foreign currency exchange
//!
//! ## License Tiers
//!
//! - **Money-Changing License**: For money-changing services only
//! - **Standard Payment Institution (SPI)**: Single service, monthly volume ≤ SGD 3M
//! - **Major Payment Institution (MPI)**: Multiple services OR volume > SGD 3M
//!
//! ## Digital Payment Tokens (DPT)
//!
//! Cryptocurrency services regulated since January 2020:
//! - Buying/selling DPTs (exchange services)
//! - Facilitating DPT exchange
//! - DPT wallet custody services
//!
//! # References
//! - Payment Services Act 2019: <https://sso.agc.gov.sg/Act/PSA2019>
//! - MAS Licensing: <https://www.mas.gov.sg/regulation/payments>

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Type of payment service provided (PSA s. 3)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentServiceType {
    /// Account issuance service - issuing payment accounts
    AccountIssuance,
    /// Domestic money transfer service - within Singapore
    DomesticMoneyTransfer,
    /// Cross-border money transfer service - international remittances
    CrossBorderMoneyTransfer,
    /// Merchant acquisition service - accepting payments for merchants
    MerchantAcquisition,
    /// E-money issuance service - stored value facilities
    EMoneyIssuance,
    /// Digital payment token service - cryptocurrency exchange/wallet
    DigitalPaymentToken,
    /// Money-changing service - foreign currency exchange
    MoneyChanging,
}

/// License type under Payment Services Act
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentLicenseType {
    /// Money-changing license only (PSA s. 6)
    MoneyChangingLicense,
    /// Standard Payment Institution - single service, volume ≤ SGD 3M/month (PSA s. 5)
    StandardPaymentInstitution,
    /// Major Payment Institution - multiple services OR volume > SGD 3M/month (PSA s. 5)
    MajorPaymentInstitution,
}

/// License status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseStatus {
    /// License is active
    Active,
    /// License suspended by MAS
    Suspended,
    /// License revoked
    Revoked,
    /// Application pending
    Pending,
}

/// Digital Payment Token (DPT) service type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DptServiceType {
    /// Dealing in DPTs (buying/selling)
    Dealing,
    /// Facilitating exchange of DPTs
    Exchange,
    /// DPT wallet custody/administration
    Custody,
}

/// A payment service provider licensed under the PSA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentServiceProvider {
    /// Unique Entity Number (UEN)
    pub uen: String,
    /// Registered business name
    pub name: String,
    /// Type of license held
    pub license_type: PaymentLicenseType,
    /// License status
    pub license_status: LicenseStatus,
    /// License grant date
    pub license_date: DateTime<Utc>,
    /// License expiry (if applicable)
    pub license_expiry: Option<DateTime<Utc>>,
    /// Payment services provided
    pub services: Vec<PaymentServiceType>,
    /// Average monthly transaction volume in SGD cents
    pub monthly_volume_sgd: u64,
    /// Total float/e-money outstanding in SGD cents
    pub float_outstanding_sgd: u64,
    /// Whether safeguarding arrangements are in place
    pub safeguarding_enabled: bool,
    /// DPT services (if applicable)
    pub dpt_services: Vec<DptServiceType>,
    /// Whether AML/CFT compliance officer appointed
    pub has_aml_officer: bool,
}

impl PaymentServiceProvider {
    /// Create a new payment service provider
    pub fn new(
        uen: String,
        name: String,
        license_type: PaymentLicenseType,
        license_date: DateTime<Utc>,
        services: Vec<PaymentServiceType>,
    ) -> Self {
        PaymentServiceProvider {
            uen,
            name,
            license_type,
            license_status: LicenseStatus::Active,
            license_date,
            license_expiry: None,
            services,
            monthly_volume_sgd: 0,
            float_outstanding_sgd: 0,
            safeguarding_enabled: false,
            dpt_services: Vec::new(),
            has_aml_officer: false,
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
            true
        }
    }

    /// Convert monthly volume to SGD
    pub fn monthly_volume_in_sgd(&self) -> f64 {
        self.monthly_volume_sgd as f64 / 100.0
    }

    /// Convert float outstanding to SGD
    pub fn float_in_sgd(&self) -> f64 {
        self.float_outstanding_sgd as f64 / 100.0
    }

    /// Check if provider requires MPI license based on volume
    ///
    /// # Rule
    /// PSA s. 5: MPI required if monthly volume > SGD 3,000,000
    pub fn requires_mpi_license(&self) -> bool {
        self.monthly_volume_sgd > 300_000_000 // SGD 3M in cents
    }

    /// Check if safeguarding is required
    ///
    /// # Rule
    /// PSA s. 23: Safeguarding required for:
    /// - E-money issuance
    /// - Account issuance
    /// - Domestic money transfer
    pub fn requires_safeguarding(&self) -> bool {
        self.services.iter().any(|s| {
            matches!(
                s,
                PaymentServiceType::EMoneyIssuance
                    | PaymentServiceType::AccountIssuance
                    | PaymentServiceType::DomesticMoneyTransfer
            )
        })
    }

    /// Check if providing DPT services
    pub fn provides_dpt_services(&self) -> bool {
        self.services
            .contains(&PaymentServiceType::DigitalPaymentToken)
    }
}

/// Safeguarding arrangement for customer funds (PSA s. 23)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeguardingArrangement {
    /// Type of safeguarding
    pub arrangement_type: SafeguardingType,
    /// Bank or trustee name
    pub institution_name: String,
    /// Account number or trust reference
    pub reference: String,
    /// Amount safeguarded in SGD cents
    pub amount_safeguarded_sgd: u64,
    /// Date arrangement established
    pub established_date: DateTime<Utc>,
    /// Last verification date
    pub last_verified: DateTime<Utc>,
}

/// Type of safeguarding arrangement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafeguardingType {
    /// Funds held in trust with licensed bank
    TrustAccount,
    /// Funds segregated in statutory deposit with MAS
    StatutoryDeposit,
    /// Insurance or guarantee arrangement
    Insurance,
}

impl SafeguardingArrangement {
    /// Convert safeguarded amount to SGD
    pub fn amount_in_sgd(&self) -> f64 {
        self.amount_safeguarded_sgd as f64 / 100.0
    }

    /// Check if verification is overdue (must verify at least annually)
    pub fn verification_overdue(&self, current_date: DateTime<Utc>) -> bool {
        (current_date - self.last_verified).num_days() > 365
    }
}

/// Digital Payment Token (cryptocurrency)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalPaymentToken {
    /// Token symbol (e.g., BTC, ETH)
    pub symbol: String,
    /// Full token name
    pub name: String,
    /// Whether token is supported by the PSP
    pub is_supported: bool,
    /// Daily trading volume in SGD cents
    pub daily_volume_sgd: u64,
    /// Current price in SGD cents
    pub price_sgd: u64,
}

impl DigitalPaymentToken {
    /// Convert daily volume to SGD
    pub fn daily_volume_in_sgd(&self) -> f64 {
        self.daily_volume_sgd as f64 / 100.0
    }

    /// Convert price to SGD
    pub fn price_in_sgd(&self) -> f64 {
        self.price_sgd as f64 / 100.0
    }
}

/// Transaction record for reporting purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTransaction {
    /// Unique transaction ID
    pub transaction_id: String,
    /// Payment service type used
    pub service_type: PaymentServiceType,
    /// Sender account/identifier
    pub sender: String,
    /// Recipient account/identifier
    pub recipient: String,
    /// Transaction amount in SGD cents
    pub amount_sgd: u64,
    /// Transaction currency (if not SGD)
    pub currency: Option<String>,
    /// Transaction timestamp
    pub timestamp: DateTime<Utc>,
    /// Whether transaction is cross-border
    pub is_cross_border: bool,
    /// Originating country (for cross-border)
    pub originating_country: Option<String>,
    /// Beneficiary country (for cross-border)
    pub beneficiary_country: Option<String>,
}

impl PaymentTransaction {
    /// Convert amount to SGD
    pub fn amount_in_sgd(&self) -> f64 {
        self.amount_sgd as f64 / 100.0
    }

    /// Check if transaction exceeds reporting threshold
    ///
    /// # Rule
    /// PSA regulations: Report transactions ≥ SGD 5,000 for certain purposes
    pub fn exceeds_reporting_threshold(&self) -> bool {
        self.amount_sgd >= 500_000 // SGD 5,000 in cents
    }
}

/// Customer account with payment service provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPaymentAccount {
    /// Account number/identifier
    pub account_id: String,
    /// Customer name
    pub customer_name: String,
    /// Customer identification (NRIC/Passport)
    pub customer_id: String,
    /// Account type
    pub account_type: PaymentAccountType,
    /// Account balance in SGD cents
    pub balance_sgd: u64,
    /// Date account opened
    pub opened_date: DateTime<Utc>,
    /// Whether KYC completed
    pub kyc_completed: bool,
    /// Risk category for AML purposes
    pub risk_category: RiskCategory,
    /// Whether account is verified
    pub is_verified: bool,
}

/// Type of payment account
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentAccountType {
    /// E-wallet for stored value
    EWallet,
    /// Remittance account
    Remittance,
    /// Cryptocurrency wallet
    CryptoWallet,
    /// Merchant account
    Merchant,
}

/// Risk category for AML/CFT
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskCategory {
    /// Low risk customer
    Low,
    /// Medium risk customer
    Medium,
    /// High risk customer - EDD required
    High,
}

impl CustomerPaymentAccount {
    /// Convert balance to SGD
    pub fn balance_in_sgd(&self) -> f64 {
        self.balance_sgd as f64 / 100.0
    }

    /// Check if account requires enhanced verification
    ///
    /// # Rule
    /// PSA Notice PSN02: Enhanced verification for balances > SGD 5,000
    pub fn requires_enhanced_verification(&self) -> bool {
        self.balance_sgd > 500_000 || self.risk_category == RiskCategory::High
    }
}

/// Builder for PaymentServiceProvider
pub struct PaymentServiceProviderBuilder {
    uen: Option<String>,
    name: Option<String>,
    license_type: Option<PaymentLicenseType>,
    license_date: Option<DateTime<Utc>>,
    services: Vec<PaymentServiceType>,
    monthly_volume_sgd: u64,
    float_outstanding_sgd: u64,
    safeguarding_enabled: bool,
    dpt_services: Vec<DptServiceType>,
    has_aml_officer: bool,
}

impl PaymentServiceProviderBuilder {
    pub fn new() -> Self {
        PaymentServiceProviderBuilder {
            uen: None,
            name: None,
            license_type: None,
            license_date: None,
            services: Vec::new(),
            monthly_volume_sgd: 0,
            float_outstanding_sgd: 0,
            safeguarding_enabled: false,
            dpt_services: Vec::new(),
            has_aml_officer: false,
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

    pub fn license_type(mut self, license_type: PaymentLicenseType) -> Self {
        self.license_type = Some(license_type);
        self
    }

    pub fn license_date(mut self, date: DateTime<Utc>) -> Self {
        self.license_date = Some(date);
        self
    }

    pub fn add_service(mut self, service: PaymentServiceType) -> Self {
        self.services.push(service);
        self
    }

    pub fn services(mut self, services: Vec<PaymentServiceType>) -> Self {
        self.services = services;
        self
    }

    pub fn monthly_volume_sgd(mut self, amount: u64) -> Self {
        self.monthly_volume_sgd = amount;
        self
    }

    pub fn float_outstanding_sgd(mut self, amount: u64) -> Self {
        self.float_outstanding_sgd = amount;
        self
    }

    pub fn safeguarding_enabled(mut self, enabled: bool) -> Self {
        self.safeguarding_enabled = enabled;
        self
    }

    pub fn add_dpt_service(mut self, service: DptServiceType) -> Self {
        self.dpt_services.push(service);
        self
    }

    pub fn has_aml_officer(mut self, has: bool) -> Self {
        self.has_aml_officer = has;
        self
    }

    pub fn build(self) -> Result<PaymentServiceProvider, &'static str> {
        let uen = self.uen.ok_or("UEN is required")?;
        let name = self.name.ok_or("Name is required")?;
        let license_type = self.license_type.ok_or("License type is required")?;
        let license_date = self.license_date.ok_or("License date is required")?;

        if self.services.is_empty() {
            return Err("At least one payment service is required");
        }

        Ok(PaymentServiceProvider {
            uen,
            name,
            license_type,
            license_status: LicenseStatus::Active,
            license_date,
            license_expiry: None,
            services: self.services,
            monthly_volume_sgd: self.monthly_volume_sgd,
            float_outstanding_sgd: self.float_outstanding_sgd,
            safeguarding_enabled: self.safeguarding_enabled,
            dpt_services: self.dpt_services,
            has_aml_officer: self.has_aml_officer,
        })
    }
}

impl Default for PaymentServiceProviderBuilder {
    fn default() -> Self {
        Self::new()
    }
}
