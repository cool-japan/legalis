//! Employment Act (Cap. 91) - Error Types
//!
//! This module defines error types for Singapore Employment Act violations.
//!
//! All errors include:
//! - Quadrilingual messages in Singapore's four official languages:
//!   * English
//!   * Chinese (中文/华语)
//!   * Malay (Bahasa Melayu)
//!   * Tamil (தமிழ்)
//! - Statute references (e.g., "EA s. 38")
//! - Contextual information

use thiserror::Error;

/// Result type for Employment Act operations
pub type Result<T> = std::result::Result<T, EmploymentError>;

/// Employment Act error types
#[derive(Error, Debug, Clone, PartialEq)]
pub enum EmploymentError {
    /// Working hours exceed statutory limit (s. 38)
    #[error(
        "Working hours {actual}h/week exceeds limit {limit}h/week (Employment Act s. 38)\n\
         工作时间每周{actual}小时超过法定限制每周{limit}小时 (雇佣法第38条)\n\
         Waktu bekerja {actual}j/minggu melebihi had {limit}j/minggu (Akta Pekerjaan s. 38)"
    )]
    ExcessiveWorkingHours { actual: f64, limit: f64 },

    /// Insufficient rest days (s. 38)
    #[error(
        "Insufficient rest days: {actual}/week, required: {required}/week (Employment Act s. 38)\n\
         休息日不足: 每周{actual}天, 要求: 每周{required}天 (雇佣法第38条)"
    )]
    InsufficientRestDays { actual: u32, required: u32 },

    /// Salary below minimum for category
    #[error(
        "Salary SGD {actual:.2} below threshold for EA coverage\n\
         薪资 SGD {actual:.2} 低于雇佣法适用门槛"
    )]
    BelowMinimumSalary { actual: f64 },

    /// Insufficient termination notice (s. 10/11)
    #[error(
        "Insufficient notice: {actual} days, required: {required} days (Employment Act s. 10/11)\n\
         通知期不足: {actual}天, 要求: {required}天 (雇佣法第10/11条)"
    )]
    InsufficientNotice { actual: u32, required: u32 },

    /// Invalid leave entitlement
    #[error(
        "Invalid leave entitlement: {reason}\n\
         休假权利无效: {reason}"
    )]
    InvalidLeaveEntitlement { reason: String },

    /// CPF contribution calculation error
    #[error(
        "CPF contribution error: {reason}\n\
         公积金缴交错误: {reason}"
    )]
    InvalidCpfCalculation { reason: String },

    /// Overtime payment below statutory rate (s. 38(4))
    #[error(
        "Overtime rate {actual:.2}x below minimum {required:.2}x (Employment Act s. 38(4))\n\
         加班费率{actual:.2}倍低于最低{required:.2}倍 (雇佣法第38(4)条)"
    )]
    OvertimeRateBelowMinimum { actual: f64, required: f64 },

    /// Contract end date before start date
    #[error(
        "Contract end date before start date\n\
         合同结束日期早于开始日期"
    )]
    InvalidContractDates,

    /// Employee not covered by Employment Act
    #[error(
        "Employee earning SGD {salary:.2}/month not covered by Employment Act\n\
         雇员月薪 SGD {salary:.2} 不在雇佣法适用范围"
    )]
    NotCoveredByEA { salary: f64 },

    /// CPF not applicable (non-citizen/PR)
    #[error(
        "CPF contributions not applicable for non-citizens/PRs\n\
         公积金缴交不适用于非公民/永久居民"
    )]
    CpfNotApplicable,

    /// Annual leave calculation error (s. 43)
    #[error(
        "Annual leave calculation error: {years} years service → {calculated} days (expected {expected} days) (Employment Act s. 43)\n\
         年假计算错误: 服务{years}年 → {calculated}天 (预期{expected}天) (雇佣法第43条)"
    )]
    AnnualLeaveCalculationError {
        years: u32,
        calculated: u32,
        expected: u32,
    },

    /// Sick leave entitlement error (s. 89)
    #[error(
        "Sick leave entitlement error: {reason} (Employment Act s. 89)\n\
         病假权利错误: {reason} (雇佣法第89条)"
    )]
    SickLeaveError { reason: String },

    /// Working hours exceed daily maximum
    #[error(
        "Working hours {actual}h/day exceed maximum 12h/day (Employment Act s. 38)\n\
         工作时间每天{actual}小时超过最高12小时 (雇佣法第38条)"
    )]
    ExcessiveDailyHours { actual: f64 },

    /// Generic validation error
    #[error(
        "Validation error: {message}\n\
         验证错误: {message}"
    )]
    ValidationError { message: String },
}

impl EmploymentError {
    /// Returns the Employment Act section reference
    pub fn statute_reference(&self) -> Option<&'static str> {
        match self {
            EmploymentError::ExcessiveWorkingHours { .. } => Some("EA s. 38"),
            EmploymentError::InsufficientRestDays { .. } => Some("EA s. 38"),
            EmploymentError::InsufficientNotice { .. } => Some("EA s. 10/11"),
            EmploymentError::OvertimeRateBelowMinimum { .. } => Some("EA s. 38(4)"),
            EmploymentError::AnnualLeaveCalculationError { .. } => Some("EA s. 43"),
            EmploymentError::SickLeaveError { .. } => Some("EA s. 89"),
            EmploymentError::ExcessiveDailyHours { .. } => Some("EA s. 38"),
            _ => None,
        }
    }

    /// Returns error severity
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            EmploymentError::ExcessiveWorkingHours { .. }
            | EmploymentError::InsufficientRestDays { .. }
            | EmploymentError::OvertimeRateBelowMinimum { .. }
            | EmploymentError::ExcessiveDailyHours { .. } => ErrorSeverity::High,

            EmploymentError::InsufficientNotice { .. }
            | EmploymentError::InvalidLeaveEntitlement { .. }
            | EmploymentError::AnnualLeaveCalculationError { .. } => ErrorSeverity::Medium,

            _ => ErrorSeverity::Low,
        }
    }
}

/// Error severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity (statutory violation)
    High,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_statute_reference() {
        let error = EmploymentError::ExcessiveWorkingHours {
            actual: 50.0,
            limit: 44.0,
        };
        assert_eq!(error.statute_reference(), Some("EA s. 38"));

        let error2 = EmploymentError::InsufficientNotice {
            actual: 3,
            required: 7,
        };
        assert_eq!(error2.statute_reference(), Some("EA s. 10/11"));
    }

    #[test]
    fn test_error_severity() {
        let high = EmploymentError::ExcessiveWorkingHours {
            actual: 50.0,
            limit: 44.0,
        };
        assert_eq!(high.severity(), ErrorSeverity::High);

        let medium = EmploymentError::InsufficientNotice {
            actual: 3,
            required: 7,
        };
        assert_eq!(medium.severity(), ErrorSeverity::Medium);
    }

    #[test]
    fn test_error_display() {
        let error = EmploymentError::ExcessiveWorkingHours {
            actual: 50.0,
            limit: 44.0,
        };
        let display = format!("{}", error);
        assert!(display.contains("50"));
        assert!(display.contains("44"));
        assert!(display.contains("s. 38"));
        assert!(display.contains("工作时间")); // Chinese
    }
}
