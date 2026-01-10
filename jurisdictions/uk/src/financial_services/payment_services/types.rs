//! Payment Services Types (Payment Services Regulations 2017 - PSD2)

use chrono::NaiveDate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Authorization type for payment services (PSR 2017 Reg 4)
///
/// Five types of payment service providers under PSR 2017.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AuthorizationType {
    /// Authorized Payment Institution (full PSD2 authorization)
    ///
    /// Requires FCA authorization for full range of payment services
    AuthorizedPaymentInstitution {
        /// FCA Firm Reference Number
        frn: String,
    },

    /// Small Payment Institution (PSR 2017 Reg 13)
    ///
    /// Average monthly payment volume < €3 million
    SmallPaymentInstitution {
        /// FCA registration number
        registration_number: String,
    },

    /// Electronic Money Institution (EMR 2011)
    ///
    /// Authorized under Electronic Money Regulations 2011
    ElectronicMoneyInstitution {
        /// FCA Firm Reference Number
        frn: String,
    },

    /// Credit Institution (bank)
    ///
    /// Bank authorized under Capital Requirements Regulation
    CreditInstitution {
        /// FCA Firm Reference Number
        frn: String,
    },

    /// Account Information Service Provider (read-only access)
    ///
    /// Provides account information services only (no payment initiation)
    AccountInformationServiceProvider {
        /// FCA Firm Reference Number
        frn: String,
    },

    /// Payment Initiation Service Provider
    ///
    /// Initiates payments on behalf of users
    PaymentInitiationServiceProvider {
        /// FCA Firm Reference Number
        frn: String,
    },
}

/// Strong Customer Authentication (PSR 2017 Reg 67-68)
///
/// SCA required for:
/// - Accessing payment account online
/// - Initiating electronic payment transaction
/// - Any remote action with risk of fraud or abuse
///
/// Requires **two independent elements** from different categories:
/// 1. Knowledge (something only user knows)
/// 2. Possession (something only user possesses)
/// 3. Inherence (something user is)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StrongCustomerAuthentication {
    /// Date and time of authentication
    pub authentication_date: NaiveDate,

    /// User identifier
    pub user_id: String,

    /// Knowledge factor (something user knows)
    pub knowledge_factor: Option<KnowledgeFactor>,

    /// Possession factor (something user has)
    pub possession_factor: Option<PossessionFactor>,

    /// Inherence factor (something user is)
    pub inherence_factor: Option<InherenceFactor>,

    /// Whether authentication succeeded
    pub authenticated: bool,

    /// Description of authentication method used
    pub authentication_method: String,
}

impl StrongCustomerAuthentication {
    /// Check if SCA compliant (requires 2 independent factors, PSR 2017 Reg 68(1))
    ///
    /// Returns true if:
    /// - At least 2 different factor types are present
    /// - Authentication succeeded
    pub fn is_compliant(&self) -> bool {
        let factors_present = [
            self.knowledge_factor.is_some(),
            self.possession_factor.is_some(),
            self.inherence_factor.is_some(),
        ]
        .iter()
        .filter(|&&f| f)
        .count();

        factors_present >= 2 && self.authenticated
    }

    /// Get factor count for validation
    pub fn factor_count(&self) -> usize {
        [
            self.knowledge_factor.is_some(),
            self.possession_factor.is_some(),
            self.inherence_factor.is_some(),
        ]
        .iter()
        .filter(|&&f| f)
        .count()
    }
}

/// Knowledge factor (something only user knows)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KnowledgeFactor {
    /// Password
    Password,

    /// PIN (Personal Identification Number)
    Pin,

    /// Security question answer
    SecurityQuestion,
}

/// Possession factor (something only user possesses)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PossessionFactor {
    /// Mobile device
    MobileDevice {
        /// Device identifier
        device_id: String,
    },

    /// Hardware token
    HardwareToken {
        /// Token identifier
        token_id: String,
    },

    /// Smart card
    SmartCard {
        /// Card identifier
        card_id: String,
    },
}

/// Inherence factor (something user is)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InherenceFactor {
    /// Fingerprint biometric
    Fingerprint,

    /// Face recognition biometric
    FaceRecognition,

    /// Voice recognition biometric
    VoiceRecognition,

    /// Iris scan biometric
    Iris,
}

/// Open Banking consent (CMA Order 2017, PSD2)
///
/// Consent for Third Party Providers (TPPs) to access payment accounts.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OpenBankingConsent {
    /// Unique consent identifier
    pub consent_id: String,

    /// User identifier
    pub user_id: String,

    /// Type of TPP provider
    pub provider_type: OpenBankingProviderType,

    /// Date consent granted
    pub consent_date: NaiveDate,

    /// Consent expiry date
    ///
    /// - AIS (Account Information Service): Max 90 days
    /// - PIS (Payment Initiation Service): One-off consent
    pub expiry_date: Option<NaiveDate>,

    /// Current consent status
    pub status: ConsentStatus,

    /// Permissions granted to TPP
    pub permissions: Vec<Permission>,

    /// Accounts authorized for access
    pub accounts_authorized: Vec<String>,
}

impl OpenBankingConsent {
    /// Check if consent is valid (authorized and not expired)
    pub fn is_valid(&self, current_date: NaiveDate) -> bool {
        if self.status != ConsentStatus::Authorized {
            return false;
        }

        if let Some(expiry) = self.expiry_date {
            current_date <= expiry
        } else {
            true
        }
    }

    /// Check if consent has expired
    pub fn is_expired(&self, current_date: NaiveDate) -> bool {
        if let Some(expiry) = self.expiry_date {
            current_date > expiry
        } else {
            false
        }
    }
}

/// Open Banking provider type
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OpenBankingProviderType {
    /// Account Information Service Provider (read-only)
    ///
    /// Provides consolidated account information from multiple banks
    AISP,

    /// Payment Initiation Service Provider
    ///
    /// Initiates payments on behalf of user
    PISP,
}

/// Consent status
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConsentStatus {
    /// Awaiting user authorization
    AwaitingAuthorization,

    /// Authorized by user
    Authorized,

    /// Rejected by user
    Rejected,

    /// Revoked by user
    Revoked,

    /// Expired (past expiry date)
    Expired,
}

/// Permission granted to Third Party Provider
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Permission {
    /// Read basic account information
    ReadAccountsBasic,

    /// Read detailed account information
    ReadAccountsDetail,

    /// Read account balances
    ReadBalances,

    /// Read transaction history
    ReadTransactions,

    /// Initiate payment
    InitiatePayment,
}

/// Safeguarding of client funds (PSR 2017 Reg 20-22)
///
/// Payment institutions must safeguard client funds using one of two methods:
/// 1. Segregation in separate account
/// 2. Insurance or guarantee
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClientFundsSafeguarding {
    /// Institution name
    pub institution_name: String,

    /// Safeguarding method used
    pub safeguarding_method: SafeguardingMethod,

    /// Total client funds amount in GBP
    pub client_funds_gbp: f64,

    /// Whether client funds are segregated from firm's funds
    pub client_funds_segregated: bool,

    /// Safeguarding account details (if using segregation method)
    pub safeguarding_account: Option<SafeguardingAccount>,

    /// Whether daily reconciliation is performed
    ///
    /// PSR 2017 Reg 21 requires daily reconciliation
    pub daily_reconciliation_performed: bool,

    /// Date of last reconciliation
    pub last_reconciliation_date: Option<NaiveDate>,
}

impl ClientFundsSafeguarding {
    /// Check if safeguarding is compliant with PSR 2017 Reg 20-22
    pub fn is_compliant(&self) -> bool {
        // If no client funds, safeguarding not required
        if self.client_funds_gbp == 0.0 {
            return true;
        }

        // Client funds must be segregated or insured/guaranteed
        let method_compliant = match self.safeguarding_method {
            SafeguardingMethod::Segregation => {
                self.client_funds_segregated && self.safeguarding_account.is_some()
            }
            SafeguardingMethod::InsuranceOrGuarantee => true,
        };

        // Daily reconciliation required
        method_compliant && self.daily_reconciliation_performed
    }
}

/// Safeguarding method (PSR 2017 Reg 20)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SafeguardingMethod {
    /// PSR 2017 Reg 20(1)(a) - segregation
    ///
    /// Client funds held in separate account, not mixed with firm's funds
    Segregation,

    /// PSR 2017 Reg 20(1)(b) - insurance or guarantee
    ///
    /// Client funds protected by insurance policy or comparable guarantee
    InsuranceOrGuarantee,
}

/// Safeguarding account details
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SafeguardingAccount {
    /// Account name
    pub account_name: String,

    /// Account number
    pub account_number: String,

    /// Sort code
    pub sort_code: String,

    /// Bank name
    pub bank_name: String,
}

/// Payment transaction for PSD2 compliance
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PaymentTransaction {
    /// Transaction identifier
    pub transaction_id: String,

    /// Transaction date
    pub transaction_date: NaiveDate,

    /// Payer information
    pub payer_name: String,

    /// Payer account
    pub payer_account: String,

    /// Payee information
    pub payee_name: String,

    /// Payee account
    pub payee_account: String,

    /// Transaction amount in GBP
    pub amount_gbp: f64,

    /// Currency
    pub currency: String,

    /// Whether SCA was performed
    pub sca_performed: bool,

    /// SCA exemption applied (if any)
    pub sca_exemption: Option<ScaExemption>,
}

/// SCA exemption types (PSR 2017 Reg 68)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScaExemption {
    /// Low value payment (≤€30, max €100/day or 5 transactions)
    LowValue,

    /// Recurring transaction with same amount and payee
    RecurringTransaction,

    /// Trusted beneficiary added with SCA
    TrustedBeneficiary,

    /// Corporate payment via dedicated process
    CorporatePayment,

    /// Merchant-initiated transaction
    MerchantInitiated,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sca_compliant_two_factors() {
        let sca = StrongCustomerAuthentication {
            authentication_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            user_id: "USER001".to_string(),
            knowledge_factor: Some(KnowledgeFactor::Password),
            possession_factor: Some(PossessionFactor::MobileDevice {
                device_id: "DEVICE123".to_string(),
            }),
            inherence_factor: None,
            authenticated: true,
            authentication_method: "Password + SMS OTP".to_string(),
        };

        assert!(sca.is_compliant());
        assert_eq!(sca.factor_count(), 2);
    }

    #[test]
    fn test_sca_non_compliant_one_factor() {
        let sca = StrongCustomerAuthentication {
            authentication_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            user_id: "USER001".to_string(),
            knowledge_factor: Some(KnowledgeFactor::Password),
            possession_factor: None,
            inherence_factor: None,
            authenticated: true,
            authentication_method: "Password only".to_string(),
        };

        assert!(!sca.is_compliant());
        assert_eq!(sca.factor_count(), 1);
    }

    #[test]
    fn test_sca_non_compliant_failed_auth() {
        let sca = StrongCustomerAuthentication {
            authentication_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            user_id: "USER001".to_string(),
            knowledge_factor: Some(KnowledgeFactor::Password),
            possession_factor: Some(PossessionFactor::MobileDevice {
                device_id: "DEVICE123".to_string(),
            }),
            inherence_factor: None,
            authenticated: false, // FAILED
            authentication_method: "Password + SMS OTP".to_string(),
        };

        assert!(!sca.is_compliant());
    }

    #[test]
    fn test_open_banking_consent_valid() {
        let consent = OpenBankingConsent {
            consent_id: "CONSENT001".to_string(),
            user_id: "USER001".to_string(),
            provider_type: OpenBankingProviderType::AISP,
            consent_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            expiry_date: Some(NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()), // 90 days
            status: ConsentStatus::Authorized,
            permissions: vec![Permission::ReadAccountsDetail, Permission::ReadBalances],
            accounts_authorized: vec!["ACC001".to_string()],
        };

        let current = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        assert!(consent.is_valid(current));
        assert!(!consent.is_expired(current));
    }

    #[test]
    fn test_open_banking_consent_expired() {
        let consent = OpenBankingConsent {
            consent_id: "CONSENT002".to_string(),
            user_id: "USER001".to_string(),
            provider_type: OpenBankingProviderType::AISP,
            consent_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            expiry_date: Some(NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()),
            status: ConsentStatus::Authorized,
            permissions: vec![],
            accounts_authorized: vec![],
        };

        let current = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap(); // After expiry
        assert!(!consent.is_valid(current));
        assert!(consent.is_expired(current));
    }

    #[test]
    fn test_safeguarding_compliant() {
        let safeguarding = ClientFundsSafeguarding {
            institution_name: "Test Payment Institution".to_string(),
            safeguarding_method: SafeguardingMethod::Segregation,
            client_funds_gbp: 100_000.0,
            client_funds_segregated: true,
            safeguarding_account: Some(SafeguardingAccount {
                account_name: "Client Funds Account".to_string(),
                account_number: "12345678".to_string(),
                sort_code: "12-34-56".to_string(),
                bank_name: "Test Bank".to_string(),
            }),
            daily_reconciliation_performed: true,
            last_reconciliation_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
        };

        assert!(safeguarding.is_compliant());
    }

    #[test]
    fn test_safeguarding_non_compliant_no_segregation() {
        let safeguarding = ClientFundsSafeguarding {
            institution_name: "Test Payment Institution".to_string(),
            safeguarding_method: SafeguardingMethod::Segregation,
            client_funds_gbp: 100_000.0,
            client_funds_segregated: false, // NOT SEGREGATED
            safeguarding_account: None,
            daily_reconciliation_performed: true,
            last_reconciliation_date: None,
        };

        assert!(!safeguarding.is_compliant());
    }
}
