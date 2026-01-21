//! Banking Law Validators (ການກວດສອບກົດໝາຍທະນາຄານ)
//!
//! Validation functions for Lao banking and financial services law compliance.
//!
//! ## Legal Basis
//!
//! - **Commercial Bank Law 2006** (Law No. 03/NA, amended 2018)
//! - **Bank of Lao PDR Law 2018** (Law No. 50/NA)
//! - **AML/CFT Law 2014** (Law No. 50/NA)

use super::error::{BankingLawError, Result};
use super::types::*;
use chrono::Utc;

// ============================================================================
// License Validators (ການກວດສອບໃບອະນຸຍາດ)
// ============================================================================

/// Validate minimum capital requirement for bank type
/// ກວດສອບທຶນຂັ້ນຕ່ຳສຳລັບປະເພດທະນາຄານ
pub fn validate_minimum_capital(bank_type: &BankType, capital_lak: u64) -> Result<()> {
    let required = bank_type.minimum_capital_lak();
    if capital_lak < required {
        return Err(BankingLawError::InsufficientCapital {
            capital_lak,
            required_lak: required,
            bank_type: format!("{:?}", bank_type),
        });
    }
    Ok(())
}

/// Validate minimum capital for microfinance institution
/// ກວດສອບທຶນຂັ້ນຕ່ຳສຳລັບ MFI
pub fn validate_mfi_capital(mfi_type: &MicrofinanceType, capital_lak: u64) -> Result<()> {
    let required = mfi_type.minimum_capital_lak();
    if capital_lak < required {
        return Err(BankingLawError::InsufficientCapital {
            capital_lak,
            required_lak: required,
            bank_type: format!("{:?}", mfi_type),
        });
    }
    Ok(())
}

/// Validate banking license status
/// ກວດສອບສະຖານະໃບອະນຸຍາດທະນາຄານ
pub fn validate_banking_license(license: &BankingLicense) -> Result<()> {
    match &license.status {
        LicenseStatus::Active => {
            if Utc::now() >= license.expires_at {
                return Err(BankingLawError::LicenseExpired {
                    expiry_date: license.expires_at.format("%Y-%m-%d").to_string(),
                    bank_name: license.bank_name_eng.clone(),
                });
            }
            Ok(())
        }
        LicenseStatus::Suspended { reason, .. } => Err(BankingLawError::LicenseSuspended {
            reason: reason.clone(),
        }),
        LicenseStatus::Revoked { reason, .. } => Err(BankingLawError::LicenseRevoked {
            reason: reason.clone(),
            decision_number: "N/A".to_string(),
        }),
        LicenseStatus::Expired => Err(BankingLawError::LicenseExpired {
            expiry_date: license.expires_at.format("%Y-%m-%d").to_string(),
            bank_name: license.bank_name_eng.clone(),
        }),
        LicenseStatus::UnderReview => Ok(()), // Still valid during review
    }
}

/// Validate license for specific activity
/// ກວດສອບໃບອະນຸຍາດສຳລັບກິດຈະກຳສະເພາະ
pub fn validate_license_for_activity(
    license: &BankingLicense,
    activity: &BankingActivity,
) -> Result<()> {
    validate_banking_license(license)?;

    if !license.licensed_activities.contains(activity) {
        return Err(BankingLawError::InvalidLicenseType {
            license_type: license.license_number.clone(),
            activity: format!("{:?}", activity),
        });
    }
    Ok(())
}

/// Validate fit and proper assessment
/// ກວດສອບການປະເມີນຄວາມເໝາະສົມ
pub fn validate_fit_and_proper(assessment: &FitAndProperAssessment) -> Result<()> {
    if !assessment.all_criteria_met() {
        let mut reasons = Vec::new();
        if !assessment.education_met {
            reasons.push("Education requirements not met");
        }
        if !assessment.experience_met {
            reasons.push("Experience requirements not met");
        }
        if !assessment.no_criminal_record {
            reasons.push("Criminal record found");
        }
        if !assessment.no_bankruptcy {
            reasons.push("Bankruptcy history");
        }
        if !assessment.not_disqualified {
            reasons.push("Disqualified from other positions");
        }

        return Err(BankingLawError::FitAndProperFailure {
            name: assessment.director_name.clone(),
            reason: reasons.join("; "),
        });
    }
    Ok(())
}

// ============================================================================
// Capital Adequacy Validators (ການກວດສອບຄວາມພຽງພໍຂອງທຶນ)
// ============================================================================

/// Validate capital adequacy ratio
/// ກວດສອບອັດຕາສ່ວນຄວາມພຽງພໍຂອງທຶນ
pub fn validate_car(report: &CapitalAdequacyReport) -> Result<()> {
    if report.car_percent < MIN_CAPITAL_ADEQUACY_RATIO_PERCENT {
        return Err(BankingLawError::InsufficientCAR {
            car_percent: report.car_percent,
            min_percent: MIN_CAPITAL_ADEQUACY_RATIO_PERCENT,
        });
    }
    Ok(())
}

/// Validate Tier 1 capital ratio
/// ກວດສອບອັດຕາສ່ວນທຶນຂັ້ນ 1
pub fn validate_tier1_ratio(report: &CapitalAdequacyReport) -> Result<()> {
    if report.tier1_ratio_percent < MIN_TIER1_CAPITAL_RATIO_PERCENT {
        return Err(BankingLawError::InsufficientTier1Capital {
            tier1_percent: report.tier1_ratio_percent,
            min_percent: MIN_TIER1_CAPITAL_RATIO_PERCENT,
        });
    }
    Ok(())
}

/// Validate leverage ratio
/// ກວດສອບອັດຕາສ່ວນໜີ້ສິນ
pub fn validate_leverage_ratio(report: &CapitalAdequacyReport) -> Result<()> {
    if report.leverage_ratio_percent < MIN_LEVERAGE_RATIO_PERCENT {
        return Err(BankingLawError::InsufficientLeverageRatio {
            leverage_percent: report.leverage_ratio_percent,
            min_percent: MIN_LEVERAGE_RATIO_PERCENT,
        });
    }
    Ok(())
}

/// Validate all capital adequacy requirements
/// ກວດສອບເງື່ອນໄຂຄວາມພຽງພໍຂອງທຶນທັງໝົດ
pub fn validate_capital_adequacy(report: &CapitalAdequacyReport) -> Result<()> {
    validate_car(report)?;
    validate_tier1_ratio(report)?;
    validate_leverage_ratio(report)?;
    Ok(())
}

/// Validate risk weight for asset class
/// ກວດສອບນ້ຳໜັກຄວາມສ່ຽງສຳລັບປະເພດຊັບສິນ
pub fn validate_risk_weight(weight_percent: f64, asset_class: &str) -> Result<()> {
    let valid_weights = [0.0, 20.0, 50.0, 100.0, 150.0];
    if !valid_weights.contains(&weight_percent) {
        return Err(BankingLawError::InvalidRiskWeight {
            weight: weight_percent,
            asset_class: asset_class.to_string(),
        });
    }
    Ok(())
}

// ============================================================================
// Prudential Validators (ການກວດສອບຄວາມສະຫຼາດສຸຂຸມ)
// ============================================================================

/// Validate single borrower exposure
/// ກວດສອບການເປີດຮັບຄວາມສ່ຽງຜູ້ກູ້ຢືມລາຍດຽວ
pub fn validate_single_borrower_limit(
    exposure: &BorrowerExposure,
    total_capital: u64,
) -> Result<()> {
    if exposure.is_related_party {
        return validate_related_party_limit(exposure, total_capital);
    }

    let exposure_percent = exposure.exposure_percent(total_capital);
    if exposure_percent > SINGLE_BORROWER_LIMIT_PERCENT {
        return Err(BankingLawError::SingleBorrowerLimitExceeded {
            exposure_percent,
            max_percent: SINGLE_BORROWER_LIMIT_PERCENT,
            borrower: exposure.borrower_name.clone(),
        });
    }
    Ok(())
}

/// Validate related party lending limit
/// ກວດສອບຂີດຈຳກັດການໃຫ້ກູ້ຢືມບຸກຄົນກ່ຽວຂ້ອງ
pub fn validate_related_party_limit(exposure: &BorrowerExposure, total_capital: u64) -> Result<()> {
    let exposure_percent = exposure.exposure_percent(total_capital);
    if exposure_percent > RELATED_PARTY_LIMIT_PERCENT {
        return Err(BankingLawError::RelatedPartyLimitExceeded {
            exposure_percent,
            max_percent: RELATED_PARTY_LIMIT_PERCENT,
            party: exposure.borrower_name.clone(),
        });
    }
    Ok(())
}

/// Validate all large exposures in report
/// ກວດສອບການເປີດຮັບຄວາມສ່ຽງທັງໝົດ
pub fn validate_large_exposures(report: &LargeExposureReport) -> Result<()> {
    for exposure in &report.exposures {
        validate_single_borrower_limit(exposure, report.total_capital_lak)?;
    }
    Ok(())
}

/// Validate liquidity coverage ratio
/// ກວດສອບອັດຕາສ່ວນຄຸ້ມຄອງສະພາບຄ່ອງ
pub fn validate_lcr(report: &LiquidityReport) -> Result<()> {
    if report.lcr_percent < MIN_LCR_PERCENT {
        return Err(BankingLawError::InsufficientLCR {
            lcr_percent: report.lcr_percent,
            min_percent: MIN_LCR_PERCENT,
        });
    }
    Ok(())
}

/// Validate net stable funding ratio
/// ກວດສອບອັດຕາສ່ວນແຫຼ່ງທຶນໝັ້ນຄົງສຸດທິ
pub fn validate_nsfr(report: &LiquidityReport) -> Result<()> {
    if report.nsfr_percent < MIN_NSFR_PERCENT {
        return Err(BankingLawError::InsufficientNSFR {
            nsfr_percent: report.nsfr_percent,
            min_percent: MIN_NSFR_PERCENT,
        });
    }
    Ok(())
}

/// Validate all liquidity requirements
/// ກວດສອບເງື່ອນໄຂສະພາບຄ່ອງທັງໝົດ
pub fn validate_liquidity(report: &LiquidityReport) -> Result<()> {
    validate_lcr(report)?;
    validate_nsfr(report)?;
    Ok(())
}

// ============================================================================
// Deposit Protection Validators (ການກວດສອບການປົກປ້ອງເງິນຝາກ)
// ============================================================================

/// Validate deposit type is insured
/// ກວດສອບວ່າປະເພດເງິນຝາກໄດ້ຮັບການປະກັນ
pub fn validate_deposit_insured(deposit_type: &DepositType) -> Result<()> {
    if !deposit_type.is_insured() {
        return Err(BankingLawError::DepositNotInsured {
            deposit_type: format!("{:?}", deposit_type),
        });
    }
    Ok(())
}

/// Validate deposit insurance claim
/// ກວດສອບການຮຽກຮ້ອງປະກັນເງິນຝາກ
pub fn validate_deposit_claim(claim: &DepositInsuranceClaim) -> Result<()> {
    // Check if any deposits are insurable
    let has_insured_deposit = claim.deposit_types.iter().any(|dt| dt.is_insured());
    if !has_insured_deposit {
        return Err(BankingLawError::InvalidDepositClaim {
            reason: "No insured deposit types in claim".to_string(),
        });
    }

    // Check if total exceeds coverage limit
    if claim.total_deposit_lak > DEPOSIT_INSURANCE_LIMIT_LAK {
        return Err(BankingLawError::DepositCoverageLimitExceeded {
            amount_lak: claim.total_deposit_lak,
            limit_lak: DEPOSIT_INSURANCE_LIMIT_LAK,
        });
    }

    Ok(())
}

/// Calculate insured amount for deposit
/// ຄຳນວນຈຳນວນທີ່ປະກັນ
pub fn calculate_insured_amount(deposit_amount_lak: u64, deposit_type: &DepositType) -> u64 {
    if !deposit_type.is_insured() {
        return 0;
    }
    deposit_amount_lak.min(DEPOSIT_INSURANCE_LIMIT_LAK)
}

// ============================================================================
// Foreign Exchange Validators (ການກວດສອບການແລກປ່ຽນເງິນຕາ)
// ============================================================================

/// Validate exchange rate against BOL reference
/// ກວດສອບອັດຕາແລກປ່ຽນກັບອ້າງອິງທະນາຄານກາງ
pub fn validate_exchange_rate(
    rate: f64,
    bol_rate: f64,
    currency_pair: &str,
    max_deviation_percent: f64,
) -> Result<()> {
    let deviation = ((rate - bol_rate) / bol_rate * 100.0).abs();
    if deviation > max_deviation_percent {
        return Err(BankingLawError::InvalidExchangeRate {
            rate,
            currency_pair: currency_pair.to_string(),
            bol_rate,
        });
    }
    Ok(())
}

/// Validate foreign currency account
/// ກວດສອບບັນຊີເງິນຕາຕ່າງປະເທດ
pub fn validate_fx_account(currency: &str, purpose: &str, is_authorized: bool) -> Result<()> {
    if !is_authorized {
        return Err(BankingLawError::ForeignCurrencyAccountViolation {
            reason: format!("Unauthorized account for purpose: {}", purpose),
            currency: currency.to_string(),
        });
    }
    Ok(())
}

/// Validate capital flow limits
/// ກວດສອບຂີດຈຳກັດກະແສເງິນທຶນ
pub fn validate_capital_flow(flow_type: &str, amount_usd: u64, limit_usd: u64) -> Result<()> {
    if amount_usd > limit_usd {
        return Err(BankingLawError::CapitalFlowViolation {
            violation_type: flow_type.to_string(),
            limit_usd,
        });
    }
    Ok(())
}

// ============================================================================
// AML/CFT Validators (ການກວດສອບການຕ້ານການຟອກເງິນ)
// ============================================================================

/// Validate customer due diligence completion
/// ກວດສອບການກວດສອບລູກຄ້າ
pub fn validate_cdd(cdd: &CustomerDueDiligence) -> Result<()> {
    if !cdd.is_complete() {
        let mut missing = Vec::new();
        if !cdd.identity_verified {
            missing.push("identity verification");
        }
        if !cdd.address_verified {
            missing.push("address verification");
        }
        if !cdd.source_of_funds_documented {
            missing.push("source of funds documentation");
        }

        return Err(BankingLawError::CDDFailure {
            customer: cdd.customer_name.clone(),
            reason: format!("Missing: {}", missing.join(", ")),
        });
    }
    Ok(())
}

/// Validate CDD review is current
/// ກວດສອບວ່າການກວດສອບ CDD ເປັນປະຈຸບັນ
pub fn validate_cdd_review(cdd: &CustomerDueDiligence) -> Result<()> {
    if cdd.is_review_due() {
        return Err(BankingLawError::CDDFailure {
            customer: cdd.customer_name.clone(),
            reason: "CDD review is overdue".to_string(),
        });
    }
    Ok(())
}

/// Validate PEP identification
/// ກວດສອບການລະບຸ PEP
pub fn validate_pep_identification(
    customer_name: &str,
    pep_status: &PEPStatus,
    enhanced_due_diligence_done: bool,
) -> Result<()> {
    match pep_status {
        PEPStatus::NotPEP => Ok(()),
        _ => {
            if !enhanced_due_diligence_done {
                return Err(BankingLawError::PEPNotIdentified {
                    pep_name: customer_name.to_string(),
                });
            }
            Ok(())
        }
    }
}

/// Validate STR reporting deadline
/// ກວດສອບກຳນົດລາຍງານ STR
pub fn validate_str_reporting(str_report: &SuspiciousTransactionReport) -> Result<()> {
    match &str_report.status {
        STRStatus::Draft => {
            let hours_since_creation = (Utc::now() - str_report.report_date).num_hours();
            if hours_since_creation > STR_REPORTING_DEADLINE_HOURS as i64 {
                return Err(BankingLawError::STRNotReported {
                    transaction_id: str_report.transaction_ids.join(", "),
                    deadline_hours: STR_REPORTING_DEADLINE_HOURS,
                });
            }
        }
        STRStatus::SubmittedToFIU { .. }
        | STRStatus::UnderInvestigation
        | STRStatus::Closed { .. } => {}
    }
    Ok(())
}

/// Validate record keeping period
/// ກວດສອບໄລຍະເວລາເກັບຮັກສາບັນທຶກ
pub fn validate_record_keeping(transaction_id: &str, record_years: u32) -> Result<()> {
    if record_years < AML_RECORD_KEEPING_YEARS {
        return Err(BankingLawError::RecordKeepingViolation {
            transaction_id: transaction_id.to_string(),
            required_years: AML_RECORD_KEEPING_YEARS,
        });
    }
    Ok(())
}

/// Validate sanctions screening
/// ກວດສອບການກັ່ນຕອງລາຍຊື່ຄວ່ຳບາດ
pub fn validate_sanctions_screening(
    entity_name: &str,
    is_match: bool,
    sanctions_list: &str,
) -> Result<()> {
    if is_match {
        return Err(BankingLawError::SanctionsScreeningFailure {
            entity: entity_name.to_string(),
            sanctions_list: sanctions_list.to_string(),
        });
    }
    Ok(())
}

// ============================================================================
// Interest Rate Validators (ການກວດສອບອັດຕາດອກເບ້ຍ)
// ============================================================================

/// Validate lending rate does not exceed maximum
/// ກວດສອບອັດຕາດອກເບ້ຍກູ້ຢືມບໍ່ເກີນສູງສຸດ
pub fn validate_lending_rate(rate_percent: f64) -> Result<()> {
    if rate_percent > MAX_LENDING_RATE_PERCENT {
        return Err(BankingLawError::LendingRateExceeded {
            rate_percent,
            max_percent: MAX_LENDING_RATE_PERCENT,
        });
    }
    Ok(())
}

/// Validate lending rate for usury
/// ກວດສອບອັດຕາດອກເບ້ຍເກີນຂອບເຂດ
pub fn validate_usury(rate_percent: f64, usury_threshold: f64) -> Result<()> {
    if rate_percent > usury_threshold {
        return Err(BankingLawError::UsuryRateDetected { rate_percent });
    }
    Ok(())
}

/// Validate deposit rate meets floor
/// ກວດສອບອັດຕາເງິນຝາກຜ່ານຂັ້ນຕ່ຳ
pub fn validate_deposit_rate(rate_percent: f64, floor_percent: f64) -> Result<()> {
    if rate_percent < floor_percent {
        return Err(BankingLawError::DepositRateBelowFloor {
            rate_percent,
            floor_percent,
        });
    }
    Ok(())
}

// ============================================================================
// Payment System Validators (ການກວດສອບລະບົບການຊຳລະເງິນ)
// ============================================================================

/// Validate RTGS transaction
/// ກວດສອບທຸລະກຳ RTGS
pub fn validate_rtgs_transaction(txn: &RTGSTransaction) -> Result<()> {
    match &txn.status {
        RTGSStatus::Rejected { reason } => Err(BankingLawError::RTGSTransactionFailure {
            reason: reason.clone(),
            reference: txn.reference.clone(),
        }),
        _ => Ok(()),
    }
}

/// Validate payment service provider license
/// ກວດສອບໃບອະນຸຍາດຜູ້ໃຫ້ບໍລິການຊຳລະເງິນ
pub fn validate_payment_provider(license: &PaymentServiceLicense) -> Result<()> {
    match &license.status {
        LicenseStatus::Active => {
            if Utc::now() >= license.expires_at {
                return Err(BankingLawError::UnauthorizedPaymentProvider {
                    provider: license.provider_name.clone(),
                });
            }
            Ok(())
        }
        _ => Err(BankingLawError::UnauthorizedPaymentProvider {
            provider: license.provider_name.clone(),
        }),
    }
}

/// Validate mobile banking compliance
/// ກວດສອບການປະຕິບັດຕາມທະນາຄານມືຖື
pub fn validate_mobile_banking_compliance(
    has_security_measures: bool,
    has_kyc_integration: bool,
    has_transaction_limits: bool,
) -> Result<()> {
    let mut missing = Vec::new();
    if !has_security_measures {
        missing.push("security measures");
    }
    if !has_kyc_integration {
        missing.push("KYC integration");
    }
    if !has_transaction_limits {
        missing.push("transaction limits");
    }

    if !missing.is_empty() {
        return Err(BankingLawError::MobileBankingComplianceFailure {
            requirement: format!("Missing: {}", missing.join(", ")),
        });
    }
    Ok(())
}

// ============================================================================
// BOL Supervision Validators (ການກວດສອບການກຳກັບທະນາຄານກາງ)
// ============================================================================

/// Validate reserve requirement
/// ກວດສອບເງື່ອນໄຂສະຫງວນ
pub fn validate_reserve_requirement(reserve_percent: f64) -> Result<()> {
    if reserve_percent < RESERVE_REQUIREMENT_PERCENT {
        return Err(BankingLawError::ReserveRequirementNotMet {
            reserve_percent,
            required_percent: RESERVE_REQUIREMENT_PERCENT,
        });
    }
    Ok(())
}

/// Validate BOL reporting compliance
/// ກວດສອບການປະຕິບັດຕາມການລາຍງານທະນາຄານກາງ
pub fn validate_bol_reporting(
    report_type: &BOLReportType,
    due_date: &str,
    is_submitted: bool,
) -> Result<()> {
    if !is_submitted {
        return Err(BankingLawError::BOLReportingDeadlineMissed {
            report_type: format!("{:?}", report_type),
            due_date: due_date.to_string(),
        });
    }
    Ok(())
}

// ============================================================================
// Comprehensive Validators (ການກວດສອບແບບລວມ)
// ============================================================================

/// Validate overall banking compliance
/// ກວດສອບການປະຕິບັດຕາມທະນາຄານໂດຍລວມ
pub fn validate_banking_compliance(
    license: &BankingLicense,
    car_report: &CapitalAdequacyReport,
    liquidity_report: &LiquidityReport,
) -> Result<()> {
    validate_banking_license(license)?;
    validate_capital_adequacy(car_report)?;
    validate_liquidity(liquidity_report)?;
    Ok(())
}

/// Validate AML compliance
/// ກວດສອບການປະຕິບັດຕາມ AML
pub fn validate_aml_compliance(
    cdd: &CustomerDueDiligence,
    pep_checked: bool,
    sanctions_checked: bool,
) -> Result<()> {
    validate_cdd(cdd)?;
    validate_cdd_review(cdd)?;

    if !pep_checked {
        return Err(BankingLawError::PEPNotIdentified {
            pep_name: cdd.customer_name.clone(),
        });
    }

    if !sanctions_checked {
        return Err(BankingLawError::SanctionsScreeningFailure {
            entity: cdd.customer_name.clone(),
            sanctions_list: "Not screened".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_minimum_capital_success() {
        let bank_type = BankType::StateOwned;
        let result = validate_minimum_capital(&bank_type, 350_000_000_000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_minimum_capital_failure() {
        let bank_type = BankType::StateOwned;
        let result = validate_minimum_capital(&bank_type, 100_000_000_000);
        assert!(result.is_err());
        if let Err(BankingLawError::InsufficientCapital { .. }) = result {
            // Expected
        } else {
            panic!("Expected InsufficientCapital error");
        }
    }

    #[test]
    fn test_validate_foreign_branch_capital() {
        let bank_type = BankType::ForeignBranch {
            parent_country: "Thailand".to_string(),
            parent_bank_name: "Thai Bank".to_string(),
        };
        let result = validate_minimum_capital(&bank_type, 60_000_000_000);
        assert!(result.is_ok());

        let result2 = validate_minimum_capital(&bank_type, 40_000_000_000);
        assert!(result2.is_err());
    }

    #[test]
    fn test_validate_mfi_capital() {
        let mfi_type = MicrofinanceType::DepositTaking;
        let result = validate_mfi_capital(&mfi_type, 15_000_000_000);
        assert!(result.is_ok());

        let result2 = validate_mfi_capital(&mfi_type, 5_000_000_000);
        assert!(result2.is_err());
    }

    #[test]
    fn test_validate_car_success() {
        let report = CapitalAdequacyReport {
            bank_name: "Test Bank".to_string(),
            report_date: Utc::now(),
            tier1_capital: Tier1Capital {
                common_equity_lak: 100_000_000_000,
                retained_earnings_lak: 20_000_000_000,
                other_reserves_lak: 10_000_000_000,
                regulatory_deductions_lak: 5_000_000_000,
                total_cet1_lak: 125_000_000_000,
                additional_tier1_lak: 10_000_000_000,
                total_tier1_lak: 135_000_000_000,
            },
            tier2_capital: Tier2Capital {
                subordinated_debt_lak: 20_000_000_000,
                general_provisions_lak: 5_000_000_000,
                revaluation_reserves_lak: 5_000_000_000,
                total_tier2_lak: 30_000_000_000,
            },
            risk_weighted_assets: RiskWeightedAssets {
                credit_risk_lak: 1_000_000_000_000,
                market_risk_lak: 100_000_000_000,
                operational_risk_lak: 150_000_000_000,
                total_rwa_lak: 1_250_000_000_000,
            },
            total_capital_lak: 165_000_000_000,
            car_percent: 13.2,
            tier1_ratio_percent: 10.8,
            cet1_ratio_percent: 10.0,
            leverage_ratio_percent: 5.0,
        };

        assert!(validate_car(&report).is_ok());
        assert!(validate_tier1_ratio(&report).is_ok());
        assert!(validate_leverage_ratio(&report).is_ok());
        assert!(validate_capital_adequacy(&report).is_ok());
    }

    #[test]
    fn test_validate_car_failure() {
        let report = CapitalAdequacyReport {
            bank_name: "Test Bank".to_string(),
            report_date: Utc::now(),
            tier1_capital: Tier1Capital {
                common_equity_lak: 50_000_000_000,
                retained_earnings_lak: 10_000_000_000,
                other_reserves_lak: 5_000_000_000,
                regulatory_deductions_lak: 5_000_000_000,
                total_cet1_lak: 60_000_000_000,
                additional_tier1_lak: 5_000_000_000,
                total_tier1_lak: 65_000_000_000,
            },
            tier2_capital: Tier2Capital {
                subordinated_debt_lak: 10_000_000_000,
                general_provisions_lak: 2_000_000_000,
                revaluation_reserves_lak: 3_000_000_000,
                total_tier2_lak: 15_000_000_000,
            },
            risk_weighted_assets: RiskWeightedAssets {
                credit_risk_lak: 1_000_000_000_000,
                market_risk_lak: 100_000_000_000,
                operational_risk_lak: 150_000_000_000,
                total_rwa_lak: 1_250_000_000_000,
            },
            total_capital_lak: 80_000_000_000,
            car_percent: 6.4, // Below 8%
            tier1_ratio_percent: 5.2,
            cet1_ratio_percent: 4.8,
            leverage_ratio_percent: 2.5,
        };

        assert!(validate_car(&report).is_err());
    }

    #[test]
    fn test_validate_single_borrower_limit() {
        let exposure = BorrowerExposure {
            borrower_name: "ABC Company".to_string(),
            borrower_id: "ABC-001".to_string(),
            is_related_party: false,
            exposure_amount_lak: 50_000_000_000,
            funded_exposure_lak: 40_000_000_000,
            unfunded_exposure_lak: 10_000_000_000,
        };

        let result = validate_single_borrower_limit(&exposure, 300_000_000_000);
        assert!(result.is_ok());

        // Exceeds 25%
        let result2 = validate_single_borrower_limit(&exposure, 150_000_000_000);
        assert!(result2.is_err());
    }

    #[test]
    fn test_validate_related_party_limit() {
        let exposure = BorrowerExposure {
            borrower_name: "Director's Company".to_string(),
            borrower_id: "DIR-001".to_string(),
            is_related_party: true,
            exposure_amount_lak: 30_000_000_000,
            funded_exposure_lak: 30_000_000_000,
            unfunded_exposure_lak: 0,
        };

        let result = validate_related_party_limit(&exposure, 300_000_000_000);
        assert!(result.is_ok());

        // Exceeds 15%
        let result2 = validate_related_party_limit(&exposure, 150_000_000_000);
        assert!(result2.is_err());
    }

    #[test]
    fn test_validate_liquidity() {
        let report = LiquidityReport {
            bank_name: "Test Bank".to_string(),
            report_date: Utc::now(),
            hqla_lak: 200_000_000_000,
            net_cash_outflows_30d_lak: 150_000_000_000,
            lcr_percent: 133.3,
            available_stable_funding_lak: 500_000_000_000,
            required_stable_funding_lak: 400_000_000_000,
            nsfr_percent: 125.0,
        };

        assert!(validate_lcr(&report).is_ok());
        assert!(validate_nsfr(&report).is_ok());
        assert!(validate_liquidity(&report).is_ok());
    }

    #[test]
    fn test_validate_deposit_insured() {
        assert!(validate_deposit_insured(&DepositType::Savings).is_ok());
        assert!(validate_deposit_insured(&DepositType::Current).is_ok());
        assert!(validate_deposit_insured(&DepositType::Fixed { term_months: 12 }).is_ok());
        assert!(
            validate_deposit_insured(&DepositType::ForeignCurrency {
                currency: "USD".to_string()
            })
            .is_err()
        );
    }

    #[test]
    fn test_calculate_insured_amount() {
        let amount = calculate_insured_amount(100_000_000, &DepositType::Savings);
        assert_eq!(amount, DEPOSIT_INSURANCE_LIMIT_LAK);

        let amount2 = calculate_insured_amount(30_000_000, &DepositType::Savings);
        assert_eq!(amount2, 30_000_000);

        let amount3 = calculate_insured_amount(
            100_000_000,
            &DepositType::ForeignCurrency {
                currency: "USD".to_string(),
            },
        );
        assert_eq!(amount3, 0);
    }

    #[test]
    fn test_validate_exchange_rate() {
        let result = validate_exchange_rate(20500.0, 20000.0, "USD/LAK", 5.0);
        assert!(result.is_ok());

        let result2 = validate_exchange_rate(22000.0, 20000.0, "USD/LAK", 5.0);
        assert!(result2.is_err());
    }

    #[test]
    fn test_validate_cdd() {
        let complete_cdd = CustomerDueDiligence {
            customer_id: "C001".to_string(),
            customer_name: "Test Customer".to_string(),
            customer_type: CustomerType::Individual,
            cdd_level: CDDLevel::Standard,
            identity_verified: true,
            address_verified: true,
            source_of_funds_documented: true,
            pep_status: PEPStatus::NotPEP,
            risk_rating: RiskRating::Low,
            last_review_date: Utc::now(),
            next_review_date: Utc::now() + chrono::Duration::days(365),
        };

        assert!(validate_cdd(&complete_cdd).is_ok());

        let incomplete_cdd = CustomerDueDiligence {
            customer_id: "C002".to_string(),
            customer_name: "Incomplete Customer".to_string(),
            customer_type: CustomerType::Individual,
            cdd_level: CDDLevel::Standard,
            identity_verified: true,
            address_verified: false,
            source_of_funds_documented: false,
            pep_status: PEPStatus::NotPEP,
            risk_rating: RiskRating::Medium,
            last_review_date: Utc::now(),
            next_review_date: Utc::now() + chrono::Duration::days(365),
        };

        assert!(validate_cdd(&incomplete_cdd).is_err());
    }

    #[test]
    fn test_validate_lending_rate() {
        assert!(validate_lending_rate(15.0).is_ok());
        assert!(validate_lending_rate(18.0).is_ok());
        assert!(validate_lending_rate(20.0).is_err());
    }

    #[test]
    fn test_validate_usury() {
        assert!(validate_usury(20.0, 30.0).is_ok());
        assert!(validate_usury(35.0, 30.0).is_err());
    }

    #[test]
    fn test_validate_deposit_rate() {
        assert!(validate_deposit_rate(5.0, 3.0).is_ok());
        assert!(validate_deposit_rate(2.0, 3.0).is_err());
    }

    #[test]
    fn test_validate_reserve_requirement() {
        assert!(validate_reserve_requirement(6.0).is_ok());
        assert!(validate_reserve_requirement(5.0).is_ok());
        assert!(validate_reserve_requirement(4.0).is_err());
    }

    #[test]
    fn test_validate_risk_weight() {
        assert!(validate_risk_weight(0.0, "Cash").is_ok());
        assert!(validate_risk_weight(20.0, "Interbank").is_ok());
        assert!(validate_risk_weight(100.0, "Corporate").is_ok());
        assert!(validate_risk_weight(75.0, "Invalid").is_err());
    }

    #[test]
    fn test_validate_capital_flow() {
        assert!(validate_capital_flow("Investment", 50000, 100000).is_ok());
        assert!(validate_capital_flow("Investment", 150000, 100000).is_err());
    }

    #[test]
    fn test_validate_record_keeping() {
        assert!(validate_record_keeping("TXN-001", 5).is_ok());
        assert!(validate_record_keeping("TXN-002", 6).is_ok());
        assert!(validate_record_keeping("TXN-003", 3).is_err());
    }

    #[test]
    fn test_validate_sanctions_screening() {
        assert!(validate_sanctions_screening("Clean Entity", false, "UN List").is_ok());
        assert!(validate_sanctions_screening("Bad Entity", true, "UN List").is_err());
    }

    #[test]
    fn test_validate_mobile_banking_compliance() {
        assert!(validate_mobile_banking_compliance(true, true, true).is_ok());
        assert!(validate_mobile_banking_compliance(false, true, true).is_err());
        assert!(validate_mobile_banking_compliance(true, false, false).is_err());
    }

    #[test]
    fn test_validate_fit_and_proper() {
        let passed = FitAndProperAssessment {
            director_name: "Good Director".to_string(),
            position: "CEO".to_string(),
            assessment_date: Utc::now(),
            education_met: true,
            experience_met: true,
            no_criminal_record: true,
            no_bankruptcy: true,
            not_disqualified: true,
            passed: true,
            conditions: None,
        };
        assert!(validate_fit_and_proper(&passed).is_ok());

        let failed = FitAndProperAssessment {
            director_name: "Bad Director".to_string(),
            position: "CFO".to_string(),
            assessment_date: Utc::now(),
            education_met: true,
            experience_met: false,
            no_criminal_record: false,
            no_bankruptcy: true,
            not_disqualified: true,
            passed: false,
            conditions: None,
        };
        assert!(validate_fit_and_proper(&failed).is_err());
    }
}
