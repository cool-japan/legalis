//! Error types for UK Employment Law
//!
//! All errors reference relevant UK legislation:
//! - ERA 1996 (Employment Rights Act 1996)
//! - WTR 1998 (Working Time Regulations 1998)
//! - NMWA 1998 (National Minimum Wage Act 1998)

#![allow(missing_docs)]

use thiserror::Error;

/// Result type for UK employment law operations
pub type Result<T> = std::result::Result<T, EmploymentError>;

/// Errors for UK employment law compliance
#[derive(Error, Debug, Clone, PartialEq)]
pub enum EmploymentError {
    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Written particulars not provided (ERA 1996 s.1)
    #[error(
        "Written statement of employment particulars not provided.\n\
         ERA 1996 s.1 requires employer to provide written particulars within 2 months of start date.\n\
         Must include: employee/employer names, start date, pay, hours, holidays, notice periods, job title."
    )]
    WrittenParticularsNotProvided,

    /// Notice period below statutory minimum (ERA 1996 s.86)
    #[error(
        "Notice period below statutory minimum: {actual} weeks given, {required} weeks required.\n\
         ERA 1996 s.86 statutory minimum notice:\n\
         - Less than 1 month service: No notice required\n\
         - 1 month to 2 years: 1 week\n\
         - 2+ years: 1 week per year of service (maximum 12 weeks)\n\
         Years of service: {years_service}"
    )]
    NoticePeriodBelowStatutory {
        /// Actual notice period given in weeks
        actual: u8,
        /// Required notice period in weeks
        required: u8,
        /// Years of continuous service
        years_service: u8,
    },

    /// Insufficient service for unfair dismissal protection (ERA 1996 s.98)
    #[error(
        "Employee has {years_service} years continuous service.\n\
         ERA 1996 s.98 unfair dismissal protection requires 2 years continuous employment.\n\
         Exception: Automatically unfair reasons (pregnancy, whistleblowing, etc.) have no qualifying period."
    )]
    InsufficientServiceForUnfairDismissal { years_service: u8 },

    /// Unfair dismissal - no fair reason (ERA 1996 s.98)
    #[error(
        "Dismissal potentially unfair under ERA 1996 s.98.\n\
         Fair reasons for dismissal:\n\
         - Capability or qualifications (s.98(2)(a))\n\
         - Conduct (s.98(2)(b))\n\
         - Redundancy (s.98(2)(c))\n\
         - Statutory restriction (s.98(2)(d))\n\
         - Some Other Substantial Reason - SOSR (s.98(1)(b))\n\
         Reason given: {reason}\n\
         Even with fair reason, dismissal must be reasonable in all circumstances (s.98(4))."
    )]
    UnfairDismissal { reason: String },

    /// Working hours exceed 48-hour limit (WTR 1998 Reg 4)
    #[error(
        "Working hours exceed 48-hour weekly limit: {hours} hours per week.\n\
         WTR 1998 Regulation 4: Maximum 48 hours per week (averaged over 17 weeks).\n\
         Employee can opt out in writing, but opt-out not signed.\n\
         Employer in breach of Working Time Regulations."
    )]
    WorkingHoursExceed48HourLimit { hours: u8 },

    /// Insufficient rest break (WTR 1998 Reg 12)
    #[error(
        "Insufficient rest break provided.\n\
         WTR 1998 Regulation 12: Employee working 6+ hours entitled to 20-minute rest break.\n\
         Daily hours: {daily_hours}\n\
         Rest break provided: {rest_break_minutes} minutes (minimum 20 required)"
    )]
    InsufficientRestBreak {
        daily_hours: u8,
        rest_break_minutes: u8,
    },

    /// Annual leave below statutory minimum (WTR 1998 Reg 13)
    #[error(
        "Annual leave below statutory minimum: {actual} days.\n\
         WTR 1998 Regulation 13: Statutory minimum 5.6 weeks (28 days for 5-day week).\n\
         Days per week: {days_per_week}\n\
         Required minimum: {required:.1} days"
    )]
    AnnualLeaveBelowMinimum {
        actual: u8,
        days_per_week: u8,
        required: f64,
    },

    /// Hourly rate below National Minimum Wage (NMWA 1998)
    #[error(
        "Hourly rate below National Minimum Wage: £{actual:.2} per hour.\n\
         NMWA 1998: Minimum wage rates (April 2024):\n\
         - 21+: £11.44/hour (National Living Wage)\n\
         - 18-20: £8.60/hour\n\
         - Under 18: £6.40/hour\n\
         - Apprentice (first year): £6.40/hour\n\
         Employee age: {age}\n\
         Applicable minimum: £{required:.2}/hour\n\
         Shortfall: £{shortfall:.2}/hour"
    )]
    BelowMinimumWage {
        actual: f64,
        required: f64,
        age: u8,
        shortfall: f64,
    },

    /// Illegal exclusivity clause in zero-hours contract
    #[error(
        "Zero-hours contract contains illegal exclusivity clause.\n\
         Exclusivity clauses in zero-hours contracts banned since 26 May 2015.\n\
         Small Business, Enterprise and Employment Act 2015 s.153.\n\
         Worker has right to work for other employers without penalty."
    )]
    IllegalExclusivityClause,

    /// Probation period too long
    #[error(
        "Probation period exceeds reasonable maximum: {months} months.\n\
         No statutory maximum, but typical is 3-6 months.\n\
         Longer periods may be unreasonable and challengeable.\n\
         Consider proportionality to role complexity."
    )]
    ProbationPeriodTooLong { months: u8 },

    /// Fixed-term contract exceeds 4 years
    #[error(
        "Fixed-term contract duration: {duration_years:.1} years.\n\
         Fixed-Term Employees Regulations 2002: Employee on successive fixed-term contracts for 4+ years \
         becomes permanent unless objectively justified.\n\
         Consider converting to permanent contract."
    )]
    FixedTermExceedsFourYears { duration_years: f64 },

    /// Less favourable treatment of fixed-term/part-time worker
    #[error(
        "Less favourable treatment of {worker_type} worker not objectively justified.\n\
         {regulation}: Part-time/fixed-term workers entitled to same terms (pro-rata) as comparable \
         permanent/full-time workers unless objectively justified.\n\
         Treatment: {treatment}"
    )]
    LessFavourableTreatment {
        worker_type: String,
        regulation: String,
        treatment: String,
    },

    /// Pension auto-enrolment not provided
    #[error(
        "Pension auto-enrolment not provided.\n\
         Pensions Act 2008: Employers must auto-enrol eligible workers into pension scheme.\n\
         Eligible: Age 22+, under State Pension age, earning £10,000+ per year.\n\
         Minimum employer contribution: 3%\n\
         Total minimum contribution: 8% (employer 3% + employee 5%)"
    )]
    PensionAutoEnrolmentNotProvided,

    /// Redundancy selection unfair
    #[error(
        "Redundancy selection process potentially unfair.\n\
         ERA 1996 s.98: Redundancy dismissal must be fair.\n\
         Fair selection criteria: Objective, measurable, consistently applied.\n\
         Unfair criteria: 'Last in, first out' alone, trade union membership, pregnancy.\n\
         Issue: {issue}"
    )]
    RedundancySelectionUnfair { issue: String },

    /// Redundancy consultation not carried out
    #[error(
        "Redundancy consultation not carried out.\n\
         Trade Union and Labour Relations (Consolidation) Act 1992:\n\
         - 20-99 redundancies within 90 days: 30 days consultation\n\
         - 100+ redundancies within 90 days: 45 days consultation\n\
         Individual consultation always required to explore alternatives.\n\
         Number of redundancies: {number_of_redundancies}"
    )]
    RedundancyConsultationNotCarriedOut { number_of_redundancies: u32 },

    /// Automatically unfair dismissal
    #[error(
        "Automatically unfair dismissal under ERA 1996.\n\
         Reason: {reason}\n\
         Automatically unfair reasons (no qualifying period required):\n\
         - Pregnancy or maternity\n\
         - Trade union membership/activities\n\
         - Whistleblowing (protected disclosure)\n\
         - Asserting statutory right\n\
         - Health and safety complaint\n\
         - Requesting flexible working\n\
         - Discrimination (protected characteristics)\n\
         Employee can claim unfair dismissal regardless of service length."
    )]
    AutomaticallyUnfairDismissal { reason: String },

    /// Invalid value for field
    #[error("Invalid value for field {field}: {reason}")]
    InvalidValue { field: String, reason: String },

    /// Multiple violations
    #[error("Multiple employment law violations: {0:?}")]
    Multiple(Vec<String>),
}

impl EmploymentError {
    /// Create a missing field error
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField(field.into())
    }

    /// Create an invalid value error
    pub fn invalid_value(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidValue {
            field: field.into(),
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = EmploymentError::NoticePeriodBelowStatutory {
            actual: 1,
            required: 5,
            years_service: 5,
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("ERA 1996 s.86"));
        assert!(error_msg.contains("5 weeks required"));
    }

    #[test]
    fn test_below_minimum_wage_error() {
        let error = EmploymentError::BelowMinimumWage {
            actual: 10.00,
            required: 11.44,
            age: 25,
            shortfall: 1.44,
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("£10.00"));
        assert!(error_msg.contains("£11.44"));
        assert!(error_msg.contains("National Living Wage"));
    }
}
