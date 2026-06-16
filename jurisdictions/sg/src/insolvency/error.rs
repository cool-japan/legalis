//! Insolvency, Restructuring and Dissolution Act 2018 (IRDA) - Error Types
//!
//! This module defines error types for Singapore insolvency, restructuring and
//! dissolution matters with bilingual messages in Singapore's primary languages:
//! - English (business and administration)
//! - Chinese/华语 (Chinese community, ~74% of the resident population)
//!
//! The Insolvency, Restructuring and Dissolution Act 2018 (No. 40 of 2018)
//! consolidated the corporate winding-up provisions formerly in the Companies Act
//! (Cap. 50, Part X) and the personal insolvency provisions formerly in the
//! Bankruptcy Act (Cap. 20) into a single omnibus statute. It came into operation
//! on 30 July 2020.
//!
//! Each error carries a statutory reference (where applicable) and a severity
//! grading so that downstream tooling can triage and prioritise compliance issues.

use thiserror::Error;

/// Result type for insolvency, restructuring and dissolution operations.
pub type Result<T> = std::result::Result<T, InsolvencyError>;

/// Insolvency, restructuring and dissolution error types (IRDA 2018).
#[derive(Error, Debug, Clone, PartialEq)]
pub enum InsolvencyError {
    /// The statutory demand has not yet matured into a deemed inability to pay.
    ///
    /// A company is only deemed unable to pay its debts once a statutory demand
    /// for a sum exceeding the prescribed sum remains unsatisfied for 3 weeks
    /// (IRDA s. 125(2)(a)).
    #[error(
        "Statutory demand not yet ripe: {days_elapsed} of {days_required} days elapsed (IRDA s. 125(2)(a))\n\
         法定催款通知尚未成熟: 已过{days_elapsed}天，需满{days_required}天 (2018年破产、重组与解散法令第125(2)(a)条)"
    )]
    StatutoryDemandNotRipe {
        /// Number of days the statutory demand has been outstanding.
        days_elapsed: u32,
        /// Number of days required (21 days = 3 weeks).
        days_required: u32,
    },

    /// The debt does not exceed the prescribed minimum for a statutory demand.
    ///
    /// The prescribed sum for a company statutory demand under the IRDA is
    /// SGD 15,000 (IRDA s. 125(2)(a) read with the prescribed sum).
    #[error(
        "Debt SGD {debt_sgd} does not exceed the prescribed sum of SGD {minimum_sgd} (IRDA s. 125(2)(a))\n\
         债务新币{debt_sgd}元未超过法定最低金额新币{minimum_sgd}元 (2018年破产、重组与解散法令第125(2)(a)条)"
    )]
    DebtBelowPrescribedSum {
        /// The debt amount in whole SGD.
        debt_sgd: u64,
        /// The prescribed minimum in whole SGD.
        minimum_sgd: u64,
    },

    /// No valid ground for compulsory winding up was made out.
    ///
    /// The Court may only order a winding up on one of the grounds enumerated in
    /// IRDA s. 125(1).
    #[error(
        "No valid ground for compulsory winding up under IRDA s. 125(1): {reason}\n\
         没有根据第125(1)条提出有效的强制清盘理由: {reason} (2018年破产、重组与解散法令第125(1)条)"
    )]
    NoWindingUpGround {
        /// Explanation of why no ground is made out.
        reason: String,
    },

    /// A members' voluntary winding up requires a declaration of solvency.
    ///
    /// Directors must make a declaration of solvency before commencing a members'
    /// voluntary winding up (IRDA s. 161).
    #[error(
        "Members' voluntary winding up requires a directors' declaration of solvency (IRDA s. 161)\n\
         成员自愿清盘须有董事偿付能力声明 (2018年破产、重组与解散法令第161条)"
    )]
    MissingDeclarationOfSolvency,

    /// The judicial management application is not properly grounded.
    ///
    /// An application requires both the insolvency limb and at least one statutory
    /// purpose under IRDA s. 89(1).
    #[error(
        "Judicial management application not grounded: {reason} (IRDA s. 89(1))\n\
         司法管理申请理由不足: {reason} (2018年破产、重组与解散法令第89(1)条)"
    )]
    JudicialManagementNotGrounded {
        /// Explanation of the deficiency.
        reason: String,
    },

    /// No statutory purpose of judicial management is reasonably likely.
    ///
    /// At least one of the three purposes in IRDA s. 89(1) must be reasonably
    /// likely to be achieved.
    #[error(
        "No reasonable probability of achieving any statutory purpose of judicial management (IRDA s. 89(1))\n\
         没有合理可能性达成任何司法管理法定目的 (2018年破产、重组与解散法令第89(1)条)"
    )]
    NoJudicialManagementPurpose,

    /// A scheme of arrangement class failed the majority-in-number test.
    ///
    /// A scheme requires a majority in number of the creditors present and voting
    /// in each class (IRDA s. 210(3AB), formerly Companies Act s. 210).
    #[error(
        "Scheme class failed the majority-in-number test: {in_favour} of {total} voted in favour (IRDA s. 210(3AB))\n\
         债务安排方案类别未通过人数多数测试: {total}人中{in_favour}人赞成 (2018年破产、重组与解散法令第210(3AB)条)"
    )]
    SchemeMajorityInNumberFailed {
        /// Number of creditors voting in favour.
        in_favour: u32,
        /// Total number of creditors present and voting.
        total: u32,
    },

    /// A scheme of arrangement class failed the 75%-in-value test.
    ///
    /// A scheme requires 75% in value of the creditors present and voting in each
    /// class (IRDA s. 210(3AB), formerly Companies Act s. 210).
    #[error(
        "Scheme class failed the 75%-in-value test: {percentage:.2}% in value voted in favour (IRDA s. 210(3AB))\n\
         债务安排方案类别未通过75%价值测试: 赞成价值占{percentage:.2}% (2018年破产、重组与解散法令第210(3AB)条)"
    )]
    SchemeValueThresholdFailed {
        /// Percentage in value voting in favour.
        percentage: f64,
    },

    /// A scheme of arrangement has no creditor classes defined.
    #[error(
        "Scheme of arrangement has no creditor classes (IRDA s. 210)\n\
         债务安排方案没有债权人类别 (2018年破产、重组与解散法令第210条)"
    )]
    SchemeHasNoClasses,

    /// The bankruptcy debt is below the statutory threshold.
    ///
    /// A creditor may only present a bankruptcy application where the debt is a
    /// liquidated sum of at least SGD 15,000 (IRDA s. 311(1)(a)).
    #[error(
        "Bankruptcy debt SGD {debt_sgd} is below the threshold of SGD {threshold_sgd} (IRDA s. 311)\n\
         破产债务新币{debt_sgd}元低于门槛新币{threshold_sgd}元 (2018年破产、重组与解散法令第311条)"
    )]
    BankruptcyDebtBelowThreshold {
        /// The debt amount in whole SGD.
        debt_sgd: u64,
        /// The threshold in whole SGD.
        threshold_sgd: u64,
    },

    /// The bankruptcy application does not establish inability to pay.
    ///
    /// A creditor's application must show the debtor is unable to pay the debt
    /// (IRDA s. 311(1)(c)).
    #[error(
        "Bankruptcy application does not establish the debtor's inability to pay (IRDA s. 311)\n\
         破产申请未能证明债务人无力偿还 (2018年破产、重组与解散法令第311条)"
    )]
    DebtorNotShownUnableToPay,

    /// The debtor is ineligible for the Debt Repayment Scheme (DRS).
    ///
    /// The DRS administered by the Official Assignee is available only where the
    /// debtor's aggregate debts do not exceed SGD 150,000 (IRDA s. 289).
    #[error(
        "Debtor ineligible for the Debt Repayment Scheme: debts SGD {debt_sgd} exceed the ceiling of SGD {ceiling_sgd} (IRDA s. 289)\n\
         债务人不符合债务偿还计划资格: 债务新币{debt_sgd}元超过上限新币{ceiling_sgd}元 (2018年破产、重组与解散法令第289条)"
    )]
    DebtRepaymentSchemeIneligible {
        /// The aggregate debt amount in whole SGD.
        debt_sgd: u64,
        /// The DRS ceiling in whole SGD.
        ceiling_sgd: u64,
    },

    /// The proposed moratorium period exceeds the statutory limit.
    ///
    /// The automatic moratorium on an application under IRDA s. 64 lasts 30 days
    /// unless extended by the Court.
    #[error(
        "Moratorium period {days} days exceeds the statutory automatic period of {limit} days without a Court extension (IRDA s. 64)\n\
         暂止期{days}天超过法定自动期限{limit}天，须经法院延长 (2018年破产、重组与解散法令第64条)"
    )]
    MoratoriumPeriodExceeded {
        /// Requested moratorium length in days.
        days: u32,
        /// Statutory automatic limit in days.
        limit: u32,
    },

    /// Generic validation error.
    #[error(
        "Insolvency validation error: {message}\n\
         破产验证错误: {message}"
    )]
    ValidationError {
        /// Human-readable description of the validation failure.
        message: String,
    },
}

impl InsolvencyError {
    /// Returns the statute reference for this error, if any.
    pub fn statute_reference(&self) -> Option<&'static str> {
        match self {
            InsolvencyError::StatutoryDemandNotRipe { .. } => Some("IRDA s. 125(2)(a)"),
            InsolvencyError::DebtBelowPrescribedSum { .. } => Some("IRDA s. 125(2)(a)"),
            InsolvencyError::NoWindingUpGround { .. } => Some("IRDA s. 125(1)"),
            InsolvencyError::MissingDeclarationOfSolvency => Some("IRDA s. 161"),
            InsolvencyError::JudicialManagementNotGrounded { .. } => Some("IRDA s. 89(1)"),
            InsolvencyError::NoJudicialManagementPurpose => Some("IRDA s. 89(1)"),
            InsolvencyError::SchemeMajorityInNumberFailed { .. } => Some("IRDA s. 210(3AB)"),
            InsolvencyError::SchemeValueThresholdFailed { .. } => Some("IRDA s. 210(3AB)"),
            InsolvencyError::SchemeHasNoClasses => Some("IRDA s. 210"),
            InsolvencyError::BankruptcyDebtBelowThreshold { .. } => Some("IRDA s. 311"),
            InsolvencyError::DebtorNotShownUnableToPay => Some("IRDA s. 311"),
            InsolvencyError::DebtRepaymentSchemeIneligible { .. } => Some("IRDA s. 289"),
            InsolvencyError::MoratoriumPeriodExceeded { .. } => Some("IRDA s. 64"),
            InsolvencyError::ValidationError { .. } => None,
        }
    }

    /// Returns the severity level (1-5), where 5 is the most serious.
    pub fn severity(&self) -> u8 {
        match self {
            InsolvencyError::StatutoryDemandNotRipe { .. } => 2,
            InsolvencyError::DebtBelowPrescribedSum { .. } => 2,
            InsolvencyError::NoWindingUpGround { .. } => 4,
            InsolvencyError::MissingDeclarationOfSolvency => 4,
            InsolvencyError::JudicialManagementNotGrounded { .. } => 3,
            InsolvencyError::NoJudicialManagementPurpose => 3,
            InsolvencyError::SchemeMajorityInNumberFailed { .. } => 3,
            InsolvencyError::SchemeValueThresholdFailed { .. } => 3,
            InsolvencyError::SchemeHasNoClasses => 4,
            InsolvencyError::BankruptcyDebtBelowThreshold { .. } => 2,
            InsolvencyError::DebtorNotShownUnableToPay => 4,
            InsolvencyError::DebtRepaymentSchemeIneligible { .. } => 2,
            InsolvencyError::MoratoriumPeriodExceeded { .. } => 3,
            InsolvencyError::ValidationError { .. } => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_statute_reference() {
        let error = InsolvencyError::DebtBelowPrescribedSum {
            debt_sgd: 5_000,
            minimum_sgd: 15_000,
        };
        assert_eq!(error.statute_reference(), Some("IRDA s. 125(2)(a)"));

        let bankruptcy = InsolvencyError::BankruptcyDebtBelowThreshold {
            debt_sgd: 1_000,
            threshold_sgd: 15_000,
        };
        assert_eq!(bankruptcy.statute_reference(), Some("IRDA s. 311"));

        let generic = InsolvencyError::ValidationError {
            message: "x".to_string(),
        };
        assert_eq!(generic.statute_reference(), None);
    }

    #[test]
    fn test_error_severity() {
        let serious = InsolvencyError::NoWindingUpGround {
            reason: "no ground".to_string(),
        };
        assert_eq!(serious.severity(), 4);

        let minor = InsolvencyError::ValidationError {
            message: "x".to_string(),
        };
        assert_eq!(minor.severity(), 1);

        let demand = InsolvencyError::StatutoryDemandNotRipe {
            days_elapsed: 10,
            days_required: 21,
        };
        assert_eq!(demand.severity(), 2);
    }

    #[test]
    fn test_error_display_bilingual() {
        let error = InsolvencyError::NoJudicialManagementPurpose;
        let display = format!("{}", error);
        assert!(display.contains("No reasonable probability"));
        assert!(display.contains("IRDA s. 89(1)"));
        assert!(display.contains("司法管理"));
    }

    #[test]
    fn test_scheme_value_threshold_display() {
        let error = InsolvencyError::SchemeValueThresholdFailed { percentage: 60.0 };
        let display = format!("{}", error);
        assert!(display.contains("60.00%"));
        assert!(display.contains("IRDA s. 210(3AB)"));
    }

    #[test]
    fn test_clone_and_eq() {
        let error = InsolvencyError::MissingDeclarationOfSolvency;
        let cloned = error.clone();
        assert_eq!(error, cloned);
        assert_eq!(error.statute_reference(), Some("IRDA s. 161"));
    }
}
