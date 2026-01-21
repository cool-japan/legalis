//! GST Error Types
//!
//! Error types for GST compliance under CGST Act 2017

use crate::citation::{Citation, cite};
use thiserror::Error;

/// GST errors
#[derive(Debug, Clone, Error)]
pub enum GstError {
    // Registration errors
    /// Not registered under GST
    #[error("CGST Act Section 22: Person liable to be registered has not obtained registration")]
    NotRegistered,

    /// Threshold not exceeded
    #[error("CGST Act Section 22: Aggregate turnover does not exceed threshold limit")]
    ThresholdNotExceeded,

    /// Invalid GSTIN format
    #[error("Invalid GSTIN format: {gstin}")]
    InvalidGstin { gstin: String },

    /// GSTIN verification failed
    #[error("GSTIN verification failed for {gstin}: {reason}")]
    GstinVerificationFailed { gstin: String, reason: String },

    /// Registration cancelled
    #[error("CGST Act Section 29: GST registration has been cancelled")]
    RegistrationCancelled,

    /// Registration suspended
    #[error("CGST Act Section 29: GST registration is suspended")]
    RegistrationSuspended,

    /// Composition ineligible
    #[error("CGST Act Section 10: Not eligible for composition scheme - {reason}")]
    CompositionIneligible { reason: String },

    // Invoice errors
    /// Invalid invoice
    #[error("CGST Act Section 31: Invoice does not comply with requirements")]
    InvalidInvoice { reason: String },

    /// Invoice not issued
    #[error("CGST Act Section 31: Tax invoice not issued at time of supply")]
    InvoiceNotIssued,

    /// Duplicate invoice number
    #[error("CGST Act Section 31: Duplicate invoice number {number}")]
    DuplicateInvoice { number: String },

    /// Invalid HSN/SAC code
    #[error("Invalid HSN/SAC code: {code}")]
    InvalidHsnSac { code: String },

    /// Credit/debit note not issued timely
    #[error("CGST Act Section 34: Credit/debit note not issued within time limit")]
    CreditDebitNoteNotTimely,

    // E-way bill errors
    /// E-way bill not generated
    #[error("Rule 138: E-way bill not generated for movement of goods")]
    EwayBillNotGenerated,

    /// E-way bill expired
    #[error("Rule 138: E-way bill has expired")]
    EwayBillExpired,

    /// E-way bill vehicle mismatch
    #[error("Rule 138: Vehicle number in e-way bill does not match")]
    EwayBillVehicleMismatch,

    /// E-way bill not required
    #[error("Rule 138: E-way bill not required for this transaction")]
    EwayBillNotRequired,

    // Return filing errors
    /// Return not filed
    #[error("CGST Act Section 39: Return {return_type} not filed for {period}")]
    ReturnNotFiled { return_type: String, period: String },

    /// Return filed late
    #[error("CGST Act Section 39: Return {return_type} filed late - late fee applicable")]
    ReturnFiledLate { return_type: String, late_fee: f64 },

    /// GSTR-1 and GSTR-3B mismatch
    #[error("GSTR-1 and GSTR-3B liability mismatch for period {period}")]
    ReturnMismatch { period: String, difference: f64 },

    /// Annual return not filed
    #[error("CGST Act Section 44: Annual return GSTR-9 not filed for FY {fy}")]
    AnnualReturnNotFiled { fy: String },

    /// Reconciliation statement not filed
    #[error("CGST Act Section 44(2): GSTR-9C reconciliation statement not filed")]
    ReconciliationNotFiled { fy: String },

    // ITC errors
    /// ITC not available
    #[error("CGST Act Section 16: Input tax credit not available - {reason}")]
    ItcNotAvailable { reason: String },

    /// ITC blocked
    #[error("CGST Act Section 17(5): Input tax credit is blocked for {item}")]
    ItcBlocked { item: String },

    /// ITC reversed
    #[error("CGST Act Section 17(2): Input tax credit reversed for non-business use")]
    ItcReversed { amount: f64 },

    /// ITC time barred
    #[error(
        "CGST Act Section 16(4): ITC claim time barred - not claimed before due date of September return"
    )]
    ItcTimeBarred { invoice_date: String },

    /// ITC mismatch with GSTR-2A/2B
    #[error("ITC claimed exceeds ITC available in GSTR-2B by {excess}")]
    ItcMismatch { excess: f64 },

    /// ITC eligibility conditions not met
    #[error("CGST Act Section 16(2): ITC conditions not satisfied - {condition}")]
    ItcConditionsNotMet { condition: String },

    // Payment errors
    /// Tax not paid
    #[error("CGST Act Section 49: Tax liability not discharged for period {period}")]
    TaxNotPaid { period: String, amount: f64 },

    /// Interest payable
    #[error("CGST Act Section 50: Interest payable on delayed payment at {rate}% p.a.")]
    InterestPayable { rate: f64, amount: f64 },

    /// Insufficient balance in electronic ledger
    #[error("Insufficient balance in electronic {ledger} ledger")]
    InsufficientBalance { ledger: String },

    /// ITC utilization violation
    #[error("CGST Act Section 49: ITC utilization sequence violated")]
    ItcUtilizationViolation,

    // Supply classification errors
    /// Supply place determination error
    #[error("IGST Act Section 10/12: Unable to determine place of supply")]
    PlaceOfSupplyError { reason: String },

    /// Time of supply error
    #[error("CGST Act Section 12/13: Time of supply incorrectly determined")]
    TimeOfSupplyError,

    /// Value of supply error
    #[error("CGST Act Section 15: Value of supply incorrectly determined")]
    ValueOfSupplyError { reason: String },

    // Reverse charge errors
    /// Reverse charge not applied
    #[error("CGST Act Section 9(3): Reverse charge mechanism not applied")]
    ReverseChargeNotApplied { service: String },

    /// Reverse charge incorrectly applied
    #[error("CGST Act Section 9(3): Reverse charge incorrectly applied")]
    ReverseChargeIncorrect,

    // Refund errors
    /// Refund claim rejected
    #[error("CGST Act Section 54: Refund claim rejected - {reason}")]
    RefundRejected { reason: String },

    /// Refund time barred
    #[error("CGST Act Section 54: Refund claim time barred - must be filed within 2 years")]
    RefundTimeBarred,

    /// LUT not furnished
    #[error("Rule 96A: Letter of Undertaking not furnished for zero-rated supply")]
    LutNotFurnished,

    // Anti-profiteering
    /// Profiteering detected
    #[error("CGST Act Section 171: Profiteering detected - benefit not passed to consumer")]
    ProfiteeringDetected { amount: f64 },

    // Penalty errors
    /// General penalty
    #[error("CGST Act Section {section}: Penalty of Rs. {amount}")]
    Penalty { section: u32, amount: f64 },

    /// Seizure of goods
    #[error("CGST Act Section 129: Goods liable for seizure - {reason}")]
    SeizureOfGoods { reason: String },

    /// Confiscation of goods
    #[error("CGST Act Section 130: Goods liable for confiscation")]
    ConfiscationOfGoods,

    // Assessment errors
    /// Self-assessment error
    #[error("CGST Act Section 59: Self-assessment under-reported tax liability")]
    SelfAssessmentError { shortfall: f64 },

    /// Scrutiny notice
    #[error("CGST Act Section 61: Scrutiny notice issued for discrepancies")]
    ScrutinyNotice { reference: String },

    /// Demand notice
    #[error("CGST Act Section 73/74: Demand notice issued for {amount}")]
    DemandNotice { amount: f64, section: u32 },

    // Audit errors
    /// Audit finding
    #[error("CGST Act Section 65: GST audit finding - {finding}")]
    AuditFinding { finding: String },

    /// Records not maintained
    #[error("CGST Act Section 35: Required records and accounts not maintained")]
    RecordsNotMaintained,

    // Technical errors
    /// GST portal error
    #[error("GST portal technical error: {message}")]
    PortalError { message: String },

    /// Validation error
    #[error("GST validation error: {message}")]
    ValidationError { message: String },
}

impl GstError {
    /// Get relevant citation
    pub fn citation(&self) -> Option<Citation> {
        match self {
            Self::NotRegistered | Self::ThresholdNotExceeded => Some(cite::cgst(22)),
            Self::RegistrationCancelled | Self::RegistrationSuspended => Some(cite::cgst(29)),
            Self::CompositionIneligible { .. } => Some(cite::cgst(10)),
            Self::InvalidInvoice { .. }
            | Self::InvoiceNotIssued
            | Self::DuplicateInvoice { .. } => Some(cite::cgst(31)),
            Self::CreditDebitNoteNotTimely => Some(cite::cgst(34)),
            Self::ReturnNotFiled { .. } | Self::ReturnFiledLate { .. } => Some(cite::cgst(39)),
            Self::AnnualReturnNotFiled { .. } | Self::ReconciliationNotFiled { .. } => {
                Some(cite::cgst(44))
            }
            Self::ItcNotAvailable { .. }
            | Self::ItcTimeBarred { .. }
            | Self::ItcConditionsNotMet { .. } => Some(cite::cgst(16)),
            Self::ItcBlocked { .. } => Some(cite::cgst(17)),
            Self::ItcReversed { .. } => Some(cite::cgst(17)),
            Self::TaxNotPaid { .. } | Self::ItcUtilizationViolation => Some(cite::cgst(49)),
            Self::InterestPayable { .. } => Some(cite::cgst(50)),
            Self::TimeOfSupplyError => Some(cite::cgst(12)),
            Self::ValueOfSupplyError { .. } => Some(cite::cgst(15)),
            Self::ReverseChargeNotApplied { .. } | Self::ReverseChargeIncorrect => {
                Some(cite::cgst(9))
            }
            Self::RefundRejected { .. } | Self::RefundTimeBarred => Some(cite::cgst(54)),
            Self::ProfiteeringDetected { .. } => Some(cite::cgst(171)),
            Self::SeizureOfGoods { .. } => Some(cite::cgst(129)),
            Self::ConfiscationOfGoods => Some(cite::cgst(130)),
            Self::SelfAssessmentError { .. } => Some(cite::cgst(59)),
            Self::ScrutinyNotice { .. } => Some(cite::cgst(61)),
            Self::DemandNotice { section, .. } => Some(cite::cgst(*section)),
            Self::AuditFinding { .. } => Some(cite::cgst(65)),
            Self::RecordsNotMaintained => Some(cite::cgst(35)),
            Self::Penalty { section, .. } => Some(cite::cgst(*section)),
            _ => None,
        }
    }

    /// Get penalty/interest information
    pub fn penalty_info(&self) -> Option<PenaltyInfo> {
        match self {
            Self::ReturnFiledLate { late_fee, .. } => Some(PenaltyInfo {
                penalty_type: PenaltyType::LateFee,
                amount: *late_fee,
                section: "Section 47",
                description: "Late fee for delayed filing of returns",
            }),
            Self::InterestPayable { amount, rate } => Some(PenaltyInfo {
                penalty_type: PenaltyType::Interest,
                amount: *amount,
                section: "Section 50",
                description: if *rate > 18.0 {
                    "Interest at 24% p.a. for wrong ITC claim"
                } else {
                    "Interest at 18% p.a. for delayed payment"
                },
            }),
            Self::TaxNotPaid { amount, .. } => Some(PenaltyInfo {
                penalty_type: PenaltyType::TaxDemand,
                amount: *amount,
                section: "Section 73/74",
                description: "Tax demand for unpaid liability",
            }),
            Self::ProfiteeringDetected { amount } => Some(PenaltyInfo {
                penalty_type: PenaltyType::AntiProfiteering,
                amount: *amount,
                section: "Section 171",
                description: "Amount not passed on to consumer",
            }),
            Self::Penalty { amount, section } => Some(PenaltyInfo {
                penalty_type: PenaltyType::GeneralPenalty,
                amount: *amount,
                section: match section {
                    122 => "Section 122 - Offences and penalties",
                    123 => "Section 123 - Penalty for failure to furnish information",
                    125 => "Section 125 - General penalty",
                    _ => "Various sections",
                },
                description: "Penalty for GST violation",
            }),
            Self::SelfAssessmentError { shortfall } => Some(PenaltyInfo {
                penalty_type: PenaltyType::TaxDemand,
                amount: *shortfall,
                section: "Section 73/74",
                description: "Tax shortfall from self-assessment",
            }),
            _ => None,
        }
    }

    /// Check if this is a serious offence (criminal liability)
    pub fn is_criminal_offence(&self) -> bool {
        matches!(
            self,
            Self::SeizureOfGoods { .. }
                | Self::ConfiscationOfGoods
                | Self::DemandNotice { section: 74, .. }
        )
    }

    /// Get remedial action
    pub fn remedial_action(&self) -> &'static str {
        match self {
            Self::NotRegistered => "Apply for GST registration using Form GST REG-01",
            Self::ReturnNotFiled { .. } => "File pending returns with late fee",
            Self::ItcNotAvailable { .. } | Self::ItcBlocked { .. } => {
                "Review ITC claim and reverse if ineligible"
            }
            Self::TaxNotPaid { .. } => "Pay tax with interest through electronic cash ledger",
            Self::EwayBillNotGenerated => "Generate e-way bill before movement of goods",
            Self::RefundTimeBarred => "No remedy - claim is time barred",
            Self::ProfiteeringDetected { .. } => {
                "Pass on benefit to consumers and file revised pricing"
            }
            Self::DemandNotice { .. } => "File reply within 30 days or pay the demand",
            Self::ScrutinyNotice { .. } => "Submit explanation within prescribed time",
            _ => "Consult GST practitioner for specific guidance",
        }
    }
}

/// Penalty type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PenaltyType {
    /// Late fee for delayed returns
    LateFee,
    /// Interest on delayed payment
    Interest,
    /// Tax demand
    TaxDemand,
    /// Anti-profiteering penalty
    AntiProfiteering,
    /// General penalty
    GeneralPenalty,
}

/// Penalty information
#[derive(Debug, Clone)]
pub struct PenaltyInfo {
    /// Type of penalty
    pub penalty_type: PenaltyType,
    /// Amount
    pub amount: f64,
    /// Section reference
    pub section: &'static str,
    /// Description
    pub description: &'static str,
}

/// GST compliance report
#[derive(Debug, Clone, Default)]
pub struct GstComplianceReport {
    /// Overall compliance status
    pub compliant: bool,
    /// GSTIN verified
    pub gstin_valid: bool,
    /// Returns filed status
    pub returns_status: Vec<ReturnComplianceStatus>,
    /// ITC compliance
    pub itc_compliant: bool,
    /// Violations found
    pub violations: Vec<GstError>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Total penalty exposure
    pub penalty_exposure: f64,
    /// Interest liability
    pub interest_liability: f64,
}

/// Return compliance status
#[derive(Debug, Clone)]
pub struct ReturnComplianceStatus {
    /// Return type
    pub return_type: String,
    /// Period
    pub period: String,
    /// Filed on time
    pub filed_on_time: bool,
    /// Late fee if applicable
    pub late_fee: f64,
}

/// Result type for GST operations
pub type GstResult<T> = Result<T, GstError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_citation() {
        let error = GstError::NotRegistered;
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.number, 22);
    }

    #[test]
    fn test_late_fee_penalty() {
        let error = GstError::ReturnFiledLate {
            return_type: "GSTR-3B".to_string(),
            late_fee: 2000.0,
        };
        let penalty = error.penalty_info().expect("Should have penalty");
        assert_eq!(penalty.amount, 2000.0);
    }

    #[test]
    fn test_criminal_offence() {
        let seizure = GstError::SeizureOfGoods {
            reason: "No e-way bill".to_string(),
        };
        assert!(seizure.is_criminal_offence());

        let late_return = GstError::ReturnFiledLate {
            return_type: "GSTR-1".to_string(),
            late_fee: 100.0,
        };
        assert!(!late_return.is_criminal_offence());
    }

    #[test]
    fn test_itc_error_citation() {
        let error = GstError::ItcBlocked {
            item: "Motor vehicle".to_string(),
        };
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.number, 17);
    }

    #[test]
    fn test_remedial_action() {
        let error = GstError::EwayBillNotGenerated;
        assert!(error.remedial_action().contains("e-way bill"));
    }
}
