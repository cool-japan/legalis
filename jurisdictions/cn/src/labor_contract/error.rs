//! Labor Contract Law Error Types
//!
//! # 劳动合同法错误类型

#![allow(missing_docs)]

use crate::citation::{Citation, cite};
use crate::i18n::BilingualText;
use thiserror::Error;

/// Labor Contract Law compliance errors
#[derive(Debug, Clone, Error)]
pub enum LaborContractError {
    /// Written contract not signed within 1 month
    #[error(
        "Labor Contract Law Article 10: Written contract not signed within one month of employment"
    )]
    NoWrittenContract,

    /// Probation period exceeds limit
    #[error(
        "Labor Contract Law Article 19: Probation period ({actual_days} days) exceeds limit ({max_days} days)"
    )]
    ProbationExceedsLimit { actual_days: u32, max_days: u32 },

    /// Probation not allowed for contract type
    #[error("Labor Contract Law Article 19: Probation period not allowed for this contract type")]
    ProbationNotAllowed,

    /// Probation salary below threshold
    #[error(
        "Labor Contract Law Article 20: Probation salary must be at least 80% of agreed salary or local minimum wage"
    )]
    ProbationSalaryBelowMinimum,

    /// Duplicate probation period
    #[error(
        "Labor Contract Law Article 19: Same employer can only set one probation period with same employee"
    )]
    DuplicateProbation,

    /// Social insurance not enrolled
    #[error(
        "Labor Contract Law Article 17: Social insurance must be provided (missing: {missing})"
    )]
    SocialInsuranceIncomplete { missing: String },

    /// Invalid termination of protected employee
    #[error(
        "Labor Contract Law Article 42: Cannot terminate contract for employee with protected status: {category}"
    )]
    ProtectedEmployeeTermination { category: String },

    /// Termination without proper notice
    #[error("Labor Contract Law Article 40: 30 days notice or one month salary in lieu required")]
    TerminationWithoutNotice,

    /// Economic layoff procedural violation
    #[error(
        "Labor Contract Law Article 41: Economic layoff requires 30 days notice to union and labor bureau report"
    )]
    LayoffProcedureViolation { missing_step: String },

    /// Severance not paid
    #[error("Labor Contract Law Article 46-47: Severance payment required but not paid")]
    SeveranceNotPaid,

    /// Severance calculation error
    #[error(
        "Labor Contract Law Article 47: Severance calculation incorrect (expected {expected}, actual {actual})"
    )]
    SeveranceCalculationError { expected: String, actual: String },

    /// Open-ended contract not offered
    #[error(
        "Labor Contract Law Article 14: Must offer open-ended contract after {renewals} renewals or {years} years"
    )]
    OpenEndedContractNotOffered { renewals: u32, years: f64 },

    /// Non-compete period exceeds limit
    #[error(
        "Labor Contract Law Article 24: Non-compete period ({months} months) exceeds 24-month limit"
    )]
    NonCompetePeriodExceeds { months: u32 },

    /// Non-compete compensation not paid
    #[error(
        "Labor Contract Law Article 23: Non-compete compensation must be paid during non-compete period"
    )]
    NonCompeteCompensationNotPaid,

    /// Training cost repayment excessive
    #[error(
        "Labor Contract Law Article 22: Training cost repayment cannot exceed remaining service period pro-rata"
    )]
    TrainingCostExcessive,

    /// Overtime pay not calculated correctly
    #[error(
        "Labor Contract Law Article 44: Overtime pay incorrect (type: {overtime_type}, required rate: {required_rate}x)"
    )]
    OvertimePayIncorrect {
        overtime_type: String,
        required_rate: f64,
    },

    /// Rest day violation
    #[error("Labor Contract Law: Worker entitled to at least one rest day per week")]
    RestDayViolation,

    /// Annual leave not granted
    #[error("Annual Leave Regulations Article 3: Employee entitled to {days} days annual leave")]
    AnnualLeaveNotGranted { days: u32 },

    /// Labor dispatch ratio exceeded
    #[error(
        "Labor Dispatch Regulations Article 4: Dispatched workers cannot exceed 10% of workforce"
    )]
    DispatchRatioExceeded { actual_pct: f64 },

    /// Dispatch position type violation
    #[error(
        "Labor Contract Law Article 66: Labor dispatch limited to temporary, auxiliary, or substitute positions"
    )]
    InvalidDispatchPosition,

    /// Wage below minimum
    #[error("Labor Contract Law Article 48: Wage must not be below local minimum wage")]
    WageBelowMinimum { actual: f64, minimum: f64 },

    /// Wage payment delayed
    #[error("Labor Contract Law: Wages must be paid in full and on time")]
    WagePaymentDelayed,

    /// Illegal penalty deduction
    #[error("Labor Contract Law: Employer cannot impose fines or arbitrary deductions from wages")]
    IllegalWageDeduction,

    /// Contract missing essential terms
    #[error("Labor Contract Law Article 17: Contract missing essential terms: {missing_terms}")]
    MissingEssentialTerms { missing_terms: String },

    /// General validation error
    #[error("Labor Contract Law validation error: {message}")]
    ValidationError { message: String },
}

impl LaborContractError {
    /// Get relevant citation
    pub fn citation(&self) -> Option<Citation> {
        match self {
            Self::NoWrittenContract => Some(cite::labor_contract(10)),
            Self::ProbationExceedsLimit { .. } => Some(cite::labor_contract(19)),
            Self::ProbationNotAllowed => Some(cite::labor_contract(19)),
            Self::ProbationSalaryBelowMinimum => Some(cite::labor_contract(20)),
            Self::DuplicateProbation => Some(cite::labor_contract(19)),
            Self::SocialInsuranceIncomplete { .. } => Some(cite::labor_contract(17)),
            Self::ProtectedEmployeeTermination { .. } => Some(cite::labor_contract(42)),
            Self::TerminationWithoutNotice => Some(cite::labor_contract(40)),
            Self::LayoffProcedureViolation { .. } => Some(cite::labor_contract(41)),
            Self::SeveranceNotPaid => Some(cite::labor_contract(46)),
            Self::SeveranceCalculationError { .. } => Some(cite::labor_contract(47)),
            Self::OpenEndedContractNotOffered { .. } => Some(cite::labor_contract(14)),
            Self::NonCompetePeriodExceeds { .. } => Some(cite::labor_contract(24)),
            Self::NonCompeteCompensationNotPaid => Some(cite::labor_contract(23)),
            Self::TrainingCostExcessive => Some(cite::labor_contract(22)),
            Self::OvertimePayIncorrect { .. } => Some(cite::labor_contract(44)),
            Self::RestDayViolation => Some(cite::labor_contract(38)),
            Self::AnnualLeaveNotGranted { .. } => None, // Annual Leave Regulations
            Self::DispatchRatioExceeded { .. } => None, // Labor Dispatch Regulations
            Self::InvalidDispatchPosition => Some(cite::labor_contract(66)),
            Self::WageBelowMinimum { .. } => Some(cite::labor_contract(48)),
            Self::WagePaymentDelayed => Some(cite::labor_contract(30)),
            Self::IllegalWageDeduction => Some(cite::labor_contract(9)),
            Self::MissingEssentialTerms { .. } => Some(cite::labor_contract(17)),
            Self::ValidationError { .. } => None,
        }
    }

    /// Get bilingual error message
    pub fn bilingual_message(&self) -> BilingualText {
        match self {
            Self::NoWrittenContract => BilingualText::new(
                "用人单位应当自用工之日起一个月内订立书面劳动合同",
                "Written labor contract must be signed within one month of employment",
            ),
            Self::ProbationExceedsLimit {
                actual_days,
                max_days,
            } => BilingualText::new(
                format!("试用期{}天超过法定上限{}天", actual_days, max_days),
                format!(
                    "Probation period {} days exceeds limit of {} days",
                    actual_days, max_days
                ),
            ),
            Self::SocialInsuranceIncomplete { missing } => BilingualText::new(
                format!("社会保险缴纳不完整，缺少：{}", missing),
                format!("Social insurance incomplete, missing: {}", missing),
            ),
            Self::ProtectedEmployeeTermination { category } => BilingualText::new(
                format!("不得解除{}的劳动合同", category),
                format!(
                    "Cannot terminate contract of employee with {} status",
                    category
                ),
            ),
            Self::SeveranceNotPaid => BilingualText::new(
                "用人单位应当依法支付经济补偿",
                "Employer must pay severance as required by law",
            ),
            Self::NonCompetePeriodExceeds { months } => BilingualText::new(
                format!("竞业限制期限{}个月超过二年上限", months),
                format!(
                    "Non-compete period {} months exceeds 24-month limit",
                    months
                ),
            ),
            Self::WageBelowMinimum { actual, minimum } => BilingualText::new(
                format!("工资{:.2}元低于最低工资标准{:.2}元", actual, minimum),
                format!("Wage {:.2} is below minimum wage {:.2}", actual, minimum),
            ),
            _ => BilingualText::new("劳动合同法合规错误".to_string(), self.to_string()),
        }
    }

    /// Get penalty information
    pub fn penalty_info(&self) -> LaborPenalty {
        match self {
            Self::NoWrittenContract => LaborPenalty::DoubleWages,
            Self::SocialInsuranceIncomplete { .. } => LaborPenalty::Administrative,
            Self::ProtectedEmployeeTermination { .. } => LaborPenalty::IllegalTermination,
            Self::SeveranceNotPaid | Self::SeveranceCalculationError { .. } => {
                LaborPenalty::AdditionalCompensation
            }
            Self::WageBelowMinimum { .. } | Self::WagePaymentDelayed => LaborPenalty::WageArrears,
            _ => LaborPenalty::Administrative,
        }
    }
}

/// Labor law penalty types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaborPenalty {
    /// 双倍工资 / Double wages (Article 82)
    DoubleWages,
    /// 违法解除赔偿金 / Illegal termination compensation (2x severance)
    IllegalTermination,
    /// 补足差额及额外补偿 / Make up difference plus additional compensation
    AdditionalCompensation,
    /// 补发工资及赔偿金 / Back pay plus damages
    WageArrears,
    /// 行政处罚 / Administrative penalty
    Administrative,
}

impl LaborPenalty {
    pub fn description(&self) -> BilingualText {
        match self {
            Self::DoubleWages => BilingualText::new(
                "自第二个月起支付双倍工资",
                "Pay double wages from second month",
            ),
            Self::IllegalTermination => BilingualText::new(
                "支付二倍经济补偿的赔偿金",
                "Pay compensation equal to 2x severance",
            ),
            Self::AdditionalCompensation => BilingualText::new(
                "补足差额并支付额外补偿",
                "Make up difference plus additional compensation",
            ),
            Self::WageArrears => BilingualText::new(
                "补发工资并支付相应赔偿金",
                "Pay back wages plus applicable damages",
            ),
            Self::Administrative => BilingualText::new(
                "行政处罚及责令改正",
                "Administrative penalty and order to rectify",
            ),
        }
    }
}

/// Result type for labor contract operations
pub type LaborContractResult<T> = Result<T, LaborContractError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_citation() {
        let error = LaborContractError::ProbationExceedsLimit {
            actual_days: 90,
            max_days: 60,
        };
        let citation = error.citation().expect("Should have citation");
        assert_eq!(citation.article, 19);
    }

    #[test]
    fn test_penalty_type() {
        let error = LaborContractError::NoWrittenContract;
        assert_eq!(error.penalty_info(), LaborPenalty::DoubleWages);

        let error = LaborContractError::ProtectedEmployeeTermination {
            category: "孕期女职工".to_string(),
        };
        assert_eq!(error.penalty_info(), LaborPenalty::IllegalTermination);
    }

    #[test]
    fn test_bilingual_message() {
        let error = LaborContractError::WageBelowMinimum {
            actual: 1800.0,
            minimum: 2200.0,
        };
        let msg = error.bilingual_message();
        assert!(msg.zh.contains("1800.00"));
        assert!(msg.en.contains("1800.00"));
    }
}
