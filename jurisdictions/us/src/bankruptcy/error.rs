//! Bankruptcy Law Errors (Bankruptcy Code Title 11)

#![allow(missing_docs)]

use thiserror::Error;

/// Errors related to US bankruptcy law
#[derive(Debug, Clone, Error, PartialEq)]
pub enum BankruptcyError {
    // ============================================================================
    // Eligibility Errors
    // ============================================================================
    /// Debtor not eligible for chapter
    #[error("Debtor not eligible for {chapter:?}. Reason: {reason}. See 11 USC § {section}.")]
    NotEligibleForChapter {
        chapter: String,
        reason: String,
        section: String,
    },

    /// Debt limits exceeded for Chapter 13
    #[error(
        "Debt limits exceeded for Chapter 13. Secured debt: ${secured}, limit: $2,750,000. Unsecured debt: ${unsecured}, limit: $2,750,000. See 11 USC § 109(e). Consider Chapter 11 instead."
    )]
    Chapter13DebtLimitExceeded { secured: f64, unsecured: f64 },

    /// Prior discharge bar (Chapter 7)
    #[error(
        "Chapter 7 discharge barred by prior discharge on {prior_discharge_date}. Must wait 8 years between Chapter 7 discharges. See 11 USC § 727(a)(8)."
    )]
    PriorDischargeBarChapter7 { prior_discharge_date: String },

    /// Prior discharge bar (Chapter 13)
    #[error(
        "Chapter 13 discharge barred by prior {prior_chapter:?} discharge on {prior_discharge_date}. Waiting periods: 4 years from Chapter 7/11, 2 years from Chapter 13. See 11 USC § 1328(f)."
    )]
    PriorDischargeBarChapter13 {
        prior_chapter: String,
        prior_discharge_date: String,
    },

    /// Serial filer (bankruptcy abuse)
    #[error(
        "Serial bankruptcy filer detected. {number_of_filings} filings in past {years} years. Automatic stay may be limited or denied. See 11 USC § 362(c)(3)-(4)."
    )]
    SerialFiler {
        number_of_filings: usize,
        years: u32,
    },

    // ============================================================================
    // Means Test Errors (Chapter 7)
    // ============================================================================
    /// Fails means test
    #[error(
        "Debtor fails means test for Chapter 7. Current monthly income: ${income}, State median income: ${median}. Presumption of abuse under 11 USC § 707(b)(2). Must file Chapter 13 or rebut presumption."
    )]
    FailsMeansTest { income: f64, median: f64 },

    /// Substantial abuse (totality of circumstances)
    #[error(
        "Chapter 7 filing constitutes substantial abuse under totality of circumstances. Reason: {reason}. See 11 USC § 707(b)(3). Court may dismiss case or convert to Chapter 13."
    )]
    SubstantialAbuse { reason: String },

    // ============================================================================
    // Automatic Stay Errors (Section 362)
    // ============================================================================
    /// Automatic stay violation
    #[error(
        "Violation of automatic stay by {creditor}. Action: {action}. Stay effective from {stay_date}. Willful violation may result in damages and contempt. See 11 USC § 362(k)."
    )]
    AutomaticStayViolation {
        creditor: String,
        action: String,
        stay_date: String,
    },

    /// Automatic stay terminated
    #[error(
        "Automatic stay terminated on {termination_date}. Reason: {reason}. Creditors may resume collection activities. See 11 USC § 362(c), (d)."
    )]
    StayTerminated {
        termination_date: String,
        reason: String,
    },

    /// Stay does not apply (exception)
    #[error(
        "Automatic stay does not apply to {action}. Exception: {exception}. See 11 USC § 362(b). Creditor may proceed without relief from stay."
    )]
    StayException { action: String, exception: String },

    // ============================================================================
    // Property and Exemption Errors
    // ============================================================================
    /// Exemption not allowed
    #[error(
        "Exemption of {amount} for {property} not allowed. Reason: {reason}. Applicable exemption limit: ${limit}. See applicable state or federal exemption statute."
    )]
    ExemptionNotAllowed {
        property: String,
        amount: f64,
        reason: String,
        limit: f64,
    },

    /// Exemption exceeds statutory limit
    #[error(
        "Exemption amount ${claimed} exceeds statutory limit of ${limit} for {exemption_type}. Objection to exemption filed."
    )]
    ExemptionExceedsLimit {
        exemption_type: String,
        claimed: f64,
        limit: f64,
    },

    /// Property not part of estate
    #[error(
        "Property '{property}' not part of bankruptcy estate. Reason: {reason}. See 11 USC § 541 (property of estate) and § 522 (exemptions)."
    )]
    NotPropertyOfEstate { property: String, reason: String },

    /// Fraudulent transfer
    #[error(
        "Fraudulent transfer of {property} on {transfer_date}. Transferee: {transferee}. Transfer may be avoided under 11 USC § 548 (2 years) or applicable state law (up to 10 years in some states)."
    )]
    FraudulentTransfer {
        property: String,
        transfer_date: String,
        transferee: String,
    },

    /// Preferential transfer
    #[error(
        "Preferential transfer to {creditor} on {transfer_date}. Amount: ${amount}. Transfer within 90 days of bankruptcy (1 year for insiders) may be avoided. See 11 USC § 547."
    )]
    PreferentialTransfer {
        creditor: String,
        transfer_date: String,
        amount: f64,
    },

    // ============================================================================
    // Discharge Errors
    // ============================================================================
    /// Discharge denied
    #[error(
        "Discharge denied under 11 USC § 727(a). Grounds: {grounds}. Debtor receives no discharge; debts remain collectible."
    )]
    DischargeDenied { grounds: String },

    /// Debt not dischargeable
    #[error(
        "Debt to {creditor} (${amount}) is nondischargeable under 11 USC § 523(a)({subsection}). Reason: {reason}. Debtor remains liable after bankruptcy."
    )]
    DebtNotDischargeable {
        creditor: String,
        amount: f64,
        subsection: String,
        reason: String,
    },

    /// Student loan not dischargeable (no undue hardship)
    #[error(
        "Student loan debt of ${amount} not dischargeable. Undue hardship not established under Brunner test: (1) cannot maintain minimal standard of living, (2) circumstances likely to persist, (3) good faith efforts to repay. See 11 USC § 523(a)(8)."
    )]
    StudentLoanNotDischargeable { amount: f64 },

    /// Discharge revoked
    #[error(
        "Discharge granted on {discharge_date} revoked. Grounds: {grounds}. See 11 USC § 727(d). Debts not discharged; creditors may resume collection."
    )]
    DischargeRevoked {
        discharge_date: String,
        grounds: String,
    },

    // ============================================================================
    // Chapter 13 Plan Errors
    // ============================================================================
    /// Chapter 13 plan not feasible
    #[error(
        "Chapter 13 plan not feasible. Monthly plan payment: ${payment}, Debtor's disposable income: ${disposable}. Plan must be feasible and propose to pay creditors. See 11 USC § 1325(a)(6)."
    )]
    PlanNotFeasible { payment: f64, disposable: f64 },

    /// Chapter 13 plan does not meet best interests test
    #[error(
        "Chapter 13 plan fails best interests test. Unsecured creditors would receive ${chapter13} under plan, but ${chapter7} in Chapter 7 liquidation. Plan must pay at least liquidation value. See 11 USC § 1325(a)(4)."
    )]
    BestInterestsTestFailed { chapter13: f64, chapter7: f64 },

    /// Chapter 13 plan duration exceeds limit
    #[error(
        "Chapter 13 plan duration of {months} months exceeds limit. Below-median debtors: 36-month limit. Above-median debtors: 60-month limit. See 11 USC § 1322(d)."
    )]
    PlanDurationExceedsLimit { months: u32 },

    /// Failure to make plan payments
    #[error(
        "Debtor failed to make Chapter 13 plan payments. {missed_payments} payments missed. Trustee may move to dismiss case. See 11 USC § 1307(c)(4)."
    )]
    FailureToMakePayments { missed_payments: u32 },

    // ============================================================================
    // Chapter 11 Plan Errors
    // ============================================================================
    /// Chapter 11 plan not confirmed
    #[error(
        "Chapter 11 plan not confirmed. Reason: {reason}. Plan must satisfy requirements of 11 USC § 1129, including feasibility and best interests."
    )]
    PlanNotConfirmed { reason: String },

    /// Cramdown requirements not met
    #[error(
        "Cramdown requirements not satisfied for dissenting class '{class}'. Plan must be fair and equitable (11 USC § 1129(b)(2)) and not discriminate unfairly. Absolute priority rule may apply."
    )]
    CramdownRequirementsNotMet { class: String },

    /// Chapter 11 plan modification not permitted
    #[error(
        "Post-confirmation plan modification not permitted. Reason: {reason}. See 11 USC § 1127 for modification requirements."
    )]
    PlanModificationNotPermitted { reason: String },

    // ============================================================================
    // Procedural Errors
    // ============================================================================
    /// Schedules not filed
    #[error(
        "Required schedules (A/B, C, D, E/F, G, H, I, J) not filed within 14 days of petition. See Bankruptcy Rule 1007(c). Case may be dismissed for failure to file schedules."
    )]
    SchedulesNotFiled,

    /// Statement of financial affairs not filed
    #[error(
        "Statement of Financial Affairs (SOFA) not filed. Required within 14 days of petition. See Bankruptcy Rule 1007(c). Case subject to dismissal."
    )]
    SofaNotFiled,

    /// Credit counseling not completed
    #[error(
        "Pre-petition credit counseling not completed. Individual debtors must complete approved credit counseling within 180 days before filing. See 11 USC § 109(h). Case may be dismissed."
    )]
    CreditCounselingNotCompleted,

    /// Debtor education course not completed (Chapter 7/13)
    #[error(
        "Debtor education course not completed. Individual Chapter 7/13 debtors must complete financial management course before discharge. See 11 USC § 727(a)(11), § 1328(g)."
    )]
    DebtorEducationNotCompleted,

    /// Meeting of creditors not attended (341 meeting)
    #[error(
        "Debtor failed to appear at meeting of creditors (341 meeting) on {meeting_date}. Appearance required by 11 USC § 343. Case may be dismissed for failure to appear."
    )]
    MeetingOfCreditorsNotAttended { meeting_date: String },

    /// Failure to cooperate with trustee
    #[error(
        "Debtor failed to cooperate with trustee. {details}. Debtor must cooperate and provide information. See 11 USC § 521. May result in dismissal or denial of discharge."
    )]
    FailureToCooperate { details: String },

    // ============================================================================
    // Claim Errors
    // ============================================================================
    /// Proof of claim not filed
    #[error(
        "Proof of claim not filed by {creditor} within bar date {bar_date}. Late-filed claims may be disallowed. See Bankruptcy Rule 3002, 3003."
    )]
    ProofOfClaimNotFiled { creditor: String, bar_date: String },

    /// Claim objection sustained
    #[error(
        "Objection to claim of {creditor} sustained. Claimed: ${claimed}, Allowed: ${allowed}. Reason: {reason}."
    )]
    ClaimObjectionSustained {
        creditor: String,
        claimed: f64,
        allowed: f64,
        reason: String,
    },

    /// Secured claim exceeds collateral value
    #[error(
        "Secured claim of ${claim} exceeds collateral value of ${collateral}. Claim bifurcated: ${collateral} secured, ${unsecured} unsecured. See 11 USC § 506(a)."
    )]
    SecuredClaimBifurcated {
        claim: f64,
        collateral: f64,
        unsecured: f64,
    },

    // ============================================================================
    // Criminal and Fraud Errors
    // ============================================================================
    /// Bankruptcy fraud
    #[error(
        "Bankruptcy fraud detected: {fraud_type}. {details}. Bankruptcy fraud is a federal crime under 18 USC § 152. Penalties: up to 5 years imprisonment and $250,000 fine."
    )]
    BankruptcyFraud { fraud_type: String, details: String },

    /// Concealment of assets
    #[error(
        "Concealment of assets: {assets}. Value: ${value}. Concealing assets from trustee is bankruptcy fraud. Criminal prosecution possible. Discharge may be denied."
    )]
    ConcealmentOfAssets { assets: String, value: f64 },

    /// False oath
    #[error(
        "False oath or account in bankruptcy schedules. {details}. Knowingly making false statement under penalty of perjury is criminal offense. Discharge may be denied under 11 USC § 727(a)(4)."
    )]
    FalseOath { details: String },

    // ============================================================================
    // Conversion and Dismissal Errors
    // ============================================================================
    /// Case dismissed
    #[error(
        "Bankruptcy case dismissed. Reason: {reason}. Automatic stay terminated. Creditors may resume collection. Debtor may refile subject to limitations."
    )]
    CaseDismissed { reason: String },

    /// Conversion not allowed
    #[error(
        "Conversion from {from_chapter:?} to {to_chapter:?} not allowed. Reason: {reason}. See 11 USC § 706, § 1112, § 1307, § 1208."
    )]
    ConversionNotAllowed {
        from_chapter: String,
        to_chapter: String,
        reason: String,
    },

    // ============================================================================
    // General Errors
    // ============================================================================
    /// Validation error
    #[error("Bankruptcy validation error: {message}")]
    ValidationError { message: String },

    /// Multiple errors
    #[error("Multiple bankruptcy errors: {errors:?}")]
    MultipleErrors { errors: Vec<String> },

    /// Statute of limitations expired
    #[error(
        "Statute of limitations expired for {action}. Filing deadline was {deadline}. Action time-barred."
    )]
    StatuteOfLimitationsExpired { action: String, deadline: String },
}

/// Result type for bankruptcy operations
pub type Result<T> = std::result::Result<T, BankruptcyError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_variants_exist() {
        // Just ensure the error variants compile
        let _error = BankruptcyError::ValidationError {
            message: "test".to_string(),
        };
    }
}
