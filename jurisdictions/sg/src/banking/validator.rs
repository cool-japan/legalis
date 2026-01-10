//! Banking Act Validation Logic
//!
//! Comprehensive validation for:
//! 1. Banking license validity (Banking Act s. 4-28)
//! 2. Capital adequacy (MAS Notice 637 - Basel III)
//! 3. AML/CFT compliance (MAS Notice 626)
//! 4. Operational soundness

use super::error::{BankingError, Result};
use super::types::*;
use chrono::Utc;

/// Validation report for a bank
#[derive(Debug, Clone)]
pub struct BankValidationReport {
    /// Whether the bank is compliant with all regulations
    pub is_compliant: bool,
    /// Critical errors that require immediate attention
    pub errors: Vec<BankingError>,
    /// Warnings that should be addressed
    pub warnings: Vec<String>,
    /// Capital adequacy status
    pub capital_status: CapitalAdequacyStatus,
    /// AML/CFT compliance status
    pub aml_compliance: AmlComplianceStatus,
}

/// Capital adequacy status breakdown
#[derive(Debug, Clone)]
pub struct CapitalAdequacyStatus {
    /// CET1 ratio (minimum 6.5%)
    pub cet1_ratio: f64,
    /// Tier 1 ratio (minimum 8.0%)
    pub tier1_ratio: f64,
    /// Total capital ratio (minimum 10.0%)
    pub total_capital_ratio: f64,
    /// Whether all ratios meet regulatory minimums
    pub meets_minimum: bool,
    /// Buffer above minimum (in percentage points)
    pub cet1_buffer: f64,
    pub tier1_buffer: f64,
    pub total_buffer: f64,
}

/// AML/CFT compliance status
#[derive(Debug, Clone)]
pub struct AmlComplianceStatus {
    /// Whether AML officer is appointed
    pub has_aml_officer: bool,
    /// Number of high-risk accounts requiring EDD
    pub high_risk_accounts: usize,
    /// Number of overdue CDD reviews
    pub overdue_cdd_reviews: usize,
}

/// Comprehensive validation of a bank
///
/// # Checks Performed
/// 1. License validity (Banking Act s. 4)
/// 2. License status (active, not suspended/revoked)
/// 3. Capital adequacy ratios (MAS Notice 637)
/// 4. AML/CFT compliance officer (MAS Notice 626)
/// 5. Operational soundness (asset-deposit ratios)
///
/// # Returns
/// - `Ok(BankValidationReport)` with detailed compliance status
/// - `Err(BankingError)` for critical structural issues
pub fn validate_bank(bank: &Bank) -> Result<BankValidationReport> {
    let mut report = BankValidationReport {
        is_compliant: true,
        errors: Vec::new(),
        warnings: Vec::new(),
        capital_status: CapitalAdequacyStatus {
            cet1_ratio: bank.capital_adequacy.cet1_ratio(),
            tier1_ratio: bank.capital_adequacy.tier1_ratio(),
            total_capital_ratio: bank.capital_adequacy.total_capital_ratio(),
            meets_minimum: false,
            cet1_buffer: 0.0,
            tier1_buffer: 0.0,
            total_buffer: 0.0,
        },
        aml_compliance: AmlComplianceStatus {
            has_aml_officer: bank.aml_officer.is_some(),
            high_risk_accounts: 0,
            overdue_cdd_reviews: 0,
        },
    };

    // 1. Validate UEN format
    if !is_valid_uen(&bank.uen) {
        report.errors.push(BankingError::InvalidUen {
            uen: bank.uen.clone(),
        });
        report.is_compliant = false;
    }

    // 2. Validate bank name
    if bank.name.len() < 3 {
        report.errors.push(BankingError::InvalidBankName);
        report.is_compliant = false;
    }

    // 3. Check license validity
    match validate_license_status(bank) {
        Ok(_) => {}
        Err(e) => {
            report.errors.push(e);
            report.is_compliant = false;
        }
    }

    // 4. Validate capital adequacy
    match validate_capital_adequacy(&bank.capital_adequacy) {
        Ok(status) => {
            report.capital_status = status;
        }
        Err(e) => {
            report.errors.push(e);
            report.is_compliant = false;
        }
    }

    // 5. Check AML officer requirement
    if bank.aml_officer.is_none() {
        report.errors.push(BankingError::NoAmlOfficer);
        report.is_compliant = false;
    }

    // 6. Validate asset-deposit relationship
    if bank.total_assets_sgd > 0 && bank.total_deposits_sgd > bank.total_assets_sgd {
        report.errors.push(BankingError::AssetsLessThanDeposits {
            assets: bank.total_assets_in_sgd(),
            deposits: bank.total_deposits_in_sgd(),
        });
        report.is_compliant = false;
    }

    // 7. Check deposit-to-asset ratio (warning if > 90%)
    let deposit_ratio = bank.deposit_asset_ratio();
    if deposit_ratio > 90.0 {
        report.warnings.push(format!(
            "High deposit-to-asset ratio {:.2}% may indicate liquidity risk (prudent limit: 90%)",
            deposit_ratio
        ));
    }

    // 8. Capital buffer warnings
    if report.capital_status.cet1_buffer < 1.0 && report.capital_status.meets_minimum {
        report.warnings.push(format!(
            "CET1 buffer only {:.2}% above minimum - consider raising capital",
            report.capital_status.cet1_buffer
        ));
    }

    Ok(report)
}

/// Validate banking license status and type-specific requirements
fn validate_license_status(bank: &Bank) -> Result<()> {
    let now = Utc::now();

    // Check license status
    match bank.license_status {
        LicenseStatus::Active => {
            // Check expiry
            if let Some(expiry) = bank.license_expiry {
                if now >= expiry {
                    return Err(BankingError::LicenseExpired {
                        expiry_date: expiry.format("%Y-%m-%d").to_string(),
                    });
                }
            }
        }
        LicenseStatus::Suspended => {
            return Err(BankingError::LicenseSuspended);
        }
        LicenseStatus::Revoked => {
            return Err(BankingError::LicenseRevoked);
        }
        LicenseStatus::UnderReview => {
            // Not an error, but operations may be restricted
        }
    }

    Ok(())
}

/// Validate Basel III capital adequacy ratios (MAS Notice 637)
///
/// # Regulatory Minimums
/// - CET1: ≥ 6.5% (includes 2.5% capital conservation buffer)
/// - Tier 1: ≥ 8.0%
/// - Total: ≥ 10.0%
fn validate_capital_adequacy(capital: &CapitalAdequacy) -> Result<CapitalAdequacyStatus> {
    // Check for zero RWA
    if capital.risk_weighted_assets_sgd == 0 {
        return Err(BankingError::ZeroRiskWeightedAssets);
    }

    let cet1_ratio = capital.cet1_ratio();
    let tier1_ratio = capital.tier1_ratio();
    let total_ratio = capital.total_capital_ratio();

    let mut status = CapitalAdequacyStatus {
        cet1_ratio,
        tier1_ratio,
        total_capital_ratio: total_ratio,
        meets_minimum: false,
        cet1_buffer: cet1_ratio - 6.5,
        tier1_buffer: tier1_ratio - 8.0,
        total_buffer: total_ratio - 10.0,
    };

    // Check CET1
    if cet1_ratio < 6.5 {
        return Err(BankingError::InsufficientCet1 { ratio: cet1_ratio });
    }

    // Check Tier 1
    if tier1_ratio < 8.0 {
        return Err(BankingError::InsufficientTier1 { ratio: tier1_ratio });
    }

    // Check Total CAR
    if total_ratio < 10.0 {
        return Err(BankingError::InsufficientTotalCapital { ratio: total_ratio });
    }

    status.meets_minimum = true;
    Ok(status)
}

/// Validate customer account AML/CFT compliance
///
/// # Checks
/// 1. CDD performed
/// 2. EDD performed for high-risk customers
/// 3. Source of funds verified
/// 4. Beneficial owner identified (for corporate accounts)
/// 5. CDD review not overdue
pub fn validate_customer_account(account: &CustomerAccount) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Check if EDD is required but not performed
    if account.requires_edd() && !account.edd_performed {
        return Err(BankingError::EddRequired {
            account_number: account.account_number.clone(),
        });
    }

    // Check if source of funds is verified
    if !account.source_of_funds_verified {
        return Err(BankingError::SourceOfFundsNotVerified {
            account_number: account.account_number.clone(),
        });
    }

    // Check if beneficial owner is identified (for corporate accounts)
    if !account.beneficial_owner_identified && account.balance_sgd > 10_000_000 {
        // Assume accounts > SGD 100k are likely corporate
        return Err(BankingError::BeneficialOwnerNotIdentified {
            account_number: account.account_number.clone(),
        });
    }

    // Check if CDD review is overdue
    if account.is_cdd_overdue(Utc::now()) {
        let days_overdue = (Utc::now() - account.last_cdd_review).num_days();
        return Err(BankingError::CddReviewOverdue {
            account_number: account.account_number.clone(),
            days_ago: days_overdue,
        });
    }

    // Warning if CDD review is approaching due date
    let days_since_review = (Utc::now() - account.last_cdd_review).num_days();
    let review_threshold = match account.risk_category {
        CustomerRiskCategory::High | CustomerRiskCategory::PoliticallyExposed => 365,
        CustomerRiskCategory::Medium => 730,
        CustomerRiskCategory::Low => 1095,
    };

    if days_since_review > (review_threshold - 60) {
        warnings.push(format!(
            "CDD review due within 60 days for account {}",
            account.account_number
        ));
    }

    Ok(warnings)
}

/// Validate suspicious transaction report filing
pub fn validate_str_filing(str: &SuspiciousTransactionReport) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    if !str.filed_timely() {
        let days_late = (str.filing_date - str.transaction_date).num_days();
        return Err(BankingError::StrFiledLate { days_late });
    }

    // Warning if filed close to deadline
    let days_to_file = (str.filing_date - str.transaction_date).num_days();
    if (7..=10).contains(&days_to_file) {
        warnings.push(format!(
            "STR filed {} days after transaction - close to recommended 5-day timeframe",
            days_to_file
        ));
    }

    Ok(warnings)
}

/// Validate cash transaction report requirements
pub fn validate_cash_transaction(ctr: &CashTransactionReport) -> Result<()> {
    if !ctr.meets_reporting_threshold() {
        return Err(BankingError::CashTransactionNotReported {
            amount: ctr.amount_in_sgd(),
        });
    }
    Ok(())
}

/// Validate wholesale bank deposit amount
///
/// # Requirement
/// Banking Act s. 4: Wholesale banks cannot accept deposits < SGD 250,000
pub fn validate_wholesale_deposit(
    license_type: &BankLicenseType,
    deposit_amount_sgd: f64,
) -> Result<()> {
    if matches!(license_type, BankLicenseType::WholesaleBank) && deposit_amount_sgd < 250_000.0 {
        return Err(BankingError::WholesaleBankMinimumDeposit {
            amount: deposit_amount_sgd,
        });
    }
    Ok(())
}

/// Validate merchant bank restrictions
///
/// # Requirement
/// Banking Act s. 28: Merchant banks cannot accept retail deposits
pub fn validate_merchant_bank_activities(
    license_type: &BankLicenseType,
    accepts_retail_deposits: bool,
) -> Result<()> {
    if matches!(license_type, BankLicenseType::MerchantBank) && accepts_retail_deposits {
        return Err(BankingError::MerchantBankRetailDeposit);
    }
    Ok(())
}

/// Assess overall AML/CFT compliance for multiple accounts
pub fn assess_aml_compliance(accounts: &[CustomerAccount]) -> AmlComplianceStatus {
    let high_risk_accounts = accounts
        .iter()
        .filter(|a| {
            matches!(
                a.risk_category,
                CustomerRiskCategory::High | CustomerRiskCategory::PoliticallyExposed
            )
        })
        .count();

    let overdue_cdd_reviews = accounts
        .iter()
        .filter(|a| a.is_cdd_overdue(Utc::now()))
        .count();

    AmlComplianceStatus {
        has_aml_officer: true, // This should be checked against Bank struct
        high_risk_accounts,
        overdue_cdd_reviews,
    }
}

/// Check if UEN format is valid (9-10 digits assigned by ACRA)
fn is_valid_uen(uen: &str) -> bool {
    // UEN format: 9-10 characters, mix of letters and numbers
    // Examples: 197700001E, S99XX1234A, T08LL1234A
    if uen.len() < 9 || uen.len() > 10 {
        return false;
    }

    // Must contain at least one digit
    uen.chars().any(|c| c.is_ascii_digit())
}

/// Calculate capital required for a given risk-weighted asset amount
///
/// # Parameters
/// - `rwa_sgd`: Risk-weighted assets in SGD cents
/// - `target_ratio`: Desired capital ratio (e.g., 12.0 for 12%)
///
/// # Returns
/// Required capital in SGD cents
pub fn calculate_required_capital(rwa_sgd: u64, target_ratio: f64) -> u64 {
    ((rwa_sgd as f64) * (target_ratio / 100.0)) as u64
}

/// Calculate additional capital needed to meet target ratio
///
/// # Returns
/// - Positive number: additional capital needed
/// - Zero: already meeting target
pub fn calculate_capital_shortfall(
    current_capital_sgd: u64,
    rwa_sgd: u64,
    target_ratio: f64,
) -> u64 {
    let required = calculate_required_capital(rwa_sgd, target_ratio);
    required.saturating_sub(current_capital_sgd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_valid_uen() {
        assert!(is_valid_uen("197700001E"));
        assert!(is_valid_uen("S99XX1234A"));
        assert!(is_valid_uen("T08LL1234A"));
        assert!(!is_valid_uen("123")); // Too short
        assert!(!is_valid_uen("ABCDEFGHIJK")); // Too long, no digits
    }

    #[test]
    fn test_capital_adequacy_calculations() {
        let capital = CapitalAdequacy {
            cet1_capital_sgd: 1_000_000_000_00,          // SGD 10M
            at1_capital_sgd: 200_000_000_00,             // SGD 2M
            tier2_capital_sgd: 300_000_000_00,           // SGD 3M
            risk_weighted_assets_sgd: 10_000_000_000_00, // SGD 100M
            calculation_date: Utc::now(),
        };

        // CET1: 10M / 100M = 10%
        assert_eq!(capital.cet1_ratio(), 10.0);

        // Tier 1: (10M + 2M) / 100M = 12%
        assert_eq!(capital.tier1_ratio(), 12.0);

        // Total: (10M + 2M + 3M) / 100M = 15%
        assert_eq!(capital.total_capital_ratio(), 15.0);

        assert!(capital.meets_regulatory_minimum());
    }

    #[test]
    fn test_insufficient_capital() {
        let capital = CapitalAdequacy {
            cet1_capital_sgd: 500_000_000_00, // SGD 5M - insufficient (5%)
            at1_capital_sgd: 100_000_000_00,
            tier2_capital_sgd: 200_000_000_00,
            risk_weighted_assets_sgd: 10_000_000_000_00, // SGD 100M
            calculation_date: Utc::now(),
        };

        assert!(!capital.meets_regulatory_minimum());
        assert!(capital.cet1_ratio() < 6.5);
    }

    #[test]
    fn test_cdd_overdue_high_risk() {
        let account = CustomerAccount {
            account_number: "ACC001".to_string(),
            customer_name: "Test Customer".to_string(),
            customer_id: "S1234567A".to_string(),
            risk_category: CustomerRiskCategory::High,
            account_opened: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
            last_cdd_review: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
            edd_performed: true,
            source_of_funds_verified: true,
            beneficial_owner_identified: true,
            balance_sgd: 50_000_00,
        };

        // High risk: review required annually (365 days)
        let check_date = Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap(); // 366 days later
        assert!(account.is_cdd_overdue(check_date));
    }

    #[test]
    fn test_cash_transaction_threshold() {
        let ctr = CashTransactionReport {
            account_number: "ACC001".to_string(),
            customer_name: "Test".to_string(),
            customer_id: "S1234567A".to_string(),
            amount_sgd: 2_500_000, // SGD 25,000
            transaction_type: CashTransactionType::Deposit,
            transaction_date: Utc::now(),
            purpose: "Business proceeds".to_string(),
        };

        assert!(ctr.meets_reporting_threshold()); // >= 20,000
    }
}
