//! Labor Code of the Russian Federation (Трудовой кодекс РФ).
//!
//! Federal Law No. 197-FZ of December 30, 2001
//!
//! This module provides:
//! - Employment contract regulations
//! - Working time and rest periods (40-hour work week)
//! - Labor rights and obligations
//! - Termination procedures

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors related to Labor Code operations
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum LaborCodeError {
    /// Invalid employment contract
    #[error("Invalid employment contract: {0}")]
    InvalidContract(String),

    /// Invalid working hours
    #[error("Invalid working hours: {0}")]
    InvalidWorkingHours(String),

    /// Invalid termination
    #[error("Invalid termination: {0}")]
    InvalidTermination(String),

    /// Validation failed
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

/// Types of employment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentType {
    /// Indefinite term (бессрочный)
    Indefinite,
    /// Fixed term (срочный) - up to 5 years
    FixedTerm { end_date: chrono::NaiveDate },
    /// Temporary (временный) - up to 2 months
    Temporary { end_date: chrono::NaiveDate },
    /// Seasonal (сезонный)
    Seasonal { end_date: chrono::NaiveDate },
}

/// Working time regime (Article 100)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingTimeRegime {
    /// Normal working hours per week (default 40)
    pub hours_per_week: u32,
    /// Working days per week
    pub days_per_week: u32,
    /// Start time
    pub start_time: chrono::NaiveTime,
    /// End time
    pub end_time: chrono::NaiveTime,
    /// Lunch break duration in minutes
    pub lunch_break_minutes: u32,
}

impl WorkingTimeRegime {
    /// Creates standard 40-hour work week regime
    pub fn standard() -> Self {
        Self {
            hours_per_week: 40,
            days_per_week: 5,
            start_time: chrono::NaiveTime::from_hms_opt(9, 0, 0).expect("Valid time"),
            end_time: chrono::NaiveTime::from_hms_opt(18, 0, 0).expect("Valid time"),
            lunch_break_minutes: 60,
        }
    }

    /// Creates reduced working time (Article 92)
    pub fn reduced(hours_per_week: u32) -> Self {
        Self {
            hours_per_week,
            days_per_week: 5,
            start_time: chrono::NaiveTime::from_hms_opt(9, 0, 0).expect("Valid time"),
            end_time: chrono::NaiveTime::from_hms_opt(18, 0, 0).expect("Valid time"),
            lunch_break_minutes: 60,
        }
    }

    /// Validates working time regime
    pub fn validate(&self) -> Result<(), LaborCodeError> {
        if self.hours_per_week > 40 {
            return Err(LaborCodeError::InvalidWorkingHours(
                "Normal working time cannot exceed 40 hours per week".to_string(),
            ));
        }

        if self.days_per_week > 6 {
            return Err(LaborCodeError::InvalidWorkingHours(
                "Working days per week cannot exceed 6".to_string(),
            ));
        }

        if self.lunch_break_minutes < 30 || self.lunch_break_minutes > 120 {
            return Err(LaborCodeError::InvalidWorkingHours(
                "Lunch break must be between 30 and 120 minutes".to_string(),
            ));
        }

        Ok(())
    }
}

/// Employment contract (Article 56)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentContract {
    /// Employer name
    pub employer: String,
    /// Employee name
    pub employee: String,
    /// Contract date
    pub contract_date: chrono::NaiveDate,
    /// Start date of work
    pub start_date: chrono::NaiveDate,
    /// Employment type
    pub employment_type: EmploymentType,
    /// Position
    pub position: String,
    /// Salary (monthly)
    pub monthly_salary: crate::common::Currency,
    /// Working time regime
    pub working_time: WorkingTimeRegime,
    /// Workplace location
    pub workplace: String,
    /// Is in written form
    pub written_form: bool,
}

impl EmploymentContract {
    /// Creates a new employment contract
    pub fn new(
        employer: impl Into<String>,
        employee: impl Into<String>,
        contract_date: chrono::NaiveDate,
        start_date: chrono::NaiveDate,
        position: impl Into<String>,
        monthly_salary: crate::common::Currency,
    ) -> Self {
        Self {
            employer: employer.into(),
            employee: employee.into(),
            contract_date,
            start_date,
            employment_type: EmploymentType::Indefinite,
            position: position.into(),
            monthly_salary,
            working_time: WorkingTimeRegime::standard(),
            workplace: String::new(),
            written_form: false,
        }
    }

    /// Sets employment type
    pub fn with_employment_type(mut self, employment_type: EmploymentType) -> Self {
        self.employment_type = employment_type;
        self
    }

    /// Sets working time regime
    pub fn with_working_time(mut self, working_time: WorkingTimeRegime) -> Self {
        self.working_time = working_time;
        self
    }

    /// Sets workplace location
    pub fn with_workplace(mut self, workplace: impl Into<String>) -> Self {
        self.workplace = workplace.into();
        self
    }

    /// Sets written form
    pub fn with_written_form(mut self, written: bool) -> Self {
        self.written_form = written;
        self
    }

    /// Validates the employment contract
    pub fn validate(&self) -> Result<(), LaborCodeError> {
        // Contract must be in writing (Article 67)
        if !self.written_form {
            return Err(LaborCodeError::InvalidContract(
                "Employment contract must be in written form".to_string(),
            ));
        }

        // Start date should not be before contract date
        if self.start_date < self.contract_date {
            return Err(LaborCodeError::InvalidContract(
                "Start date cannot be before contract date".to_string(),
            ));
        }

        // Validate working time
        self.working_time.validate()?;

        // Salary must be positive
        if !self.monthly_salary.is_positive() {
            return Err(LaborCodeError::InvalidContract(
                "Monthly salary must be positive".to_string(),
            ));
        }

        Ok(())
    }
}

/// Labor rights (Article 21)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborRights {
    /// Right to appropriate working conditions
    pub safe_working_conditions: bool,
    /// Right to timely payment of salary
    pub timely_salary: bool,
    /// Right to rest
    pub rest_periods: bool,
    /// Right to professional training
    pub professional_training: bool,
    /// Right to collective bargaining
    pub collective_bargaining: bool,
}

impl LaborRights {
    /// Creates full labor rights
    pub fn full_rights() -> Self {
        Self {
            safe_working_conditions: true,
            timely_salary: true,
            rest_periods: true,
            professional_training: true,
            collective_bargaining: true,
        }
    }
}

/// Grounds for termination (Article 77, 81)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminationGround {
    /// Mutual agreement (соглашение сторон)
    MutualAgreement,
    /// Expiration of contract term (истечение срока договора)
    ContractExpiration,
    /// Employee's initiative (инициатива работника)
    EmployeeInitiative,
    /// Employer's initiative (инициатива работодателя)
    EmployerInitiative { reason: EmployerTerminationReason },
    /// Transfer to another employer
    Transfer,
    /// Employee's refusal to continue work
    EmployeeRefusal,
    /// Circumstances beyond control (форс-мажор)
    ForceMajeure,
}

/// Reasons for employer-initiated termination (Article 81)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmployerTerminationReason {
    /// Liquidation of organization
    Liquidation,
    /// Staff reduction (сокращение штата)
    StaffReduction,
    /// Inadequate qualifications
    InadequateQualifications,
    /// Repeated failure to perform duties
    RepeatedFailure,
    /// Gross violation of labor duties
    GrossViolation,
    /// Loss of trust
    LossOfTrust,
    /// Immoral act (for educators)
    ImmoralAct,
}

/// Termination notice representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminationNotice {
    /// Party initiating termination
    pub initiated_by: String,
    /// Termination ground
    pub ground: TerminationGround,
    /// Notice date
    pub notice_date: chrono::NaiveDate,
    /// Termination date
    pub termination_date: chrono::NaiveDate,
    /// Notice period in days
    pub notice_period_days: u32,
}

impl TerminationNotice {
    /// Creates a new termination notice
    pub fn new(
        initiated_by: impl Into<String>,
        ground: TerminationGround,
        notice_date: chrono::NaiveDate,
        termination_date: chrono::NaiveDate,
    ) -> Self {
        let notice_period_days = (termination_date - notice_date).num_days() as u32;

        Self {
            initiated_by: initiated_by.into(),
            ground,
            notice_date,
            termination_date,
            notice_period_days,
        }
    }

    /// Validates the termination notice
    pub fn validate(&self) -> Result<(), LaborCodeError> {
        // Employee must give at least 14 days notice (Article 80)
        if matches!(self.ground, TerminationGround::EmployeeInitiative)
            && self.notice_period_days < 14
        {
            return Err(LaborCodeError::InvalidTermination(
                "Employee must give at least 14 days notice".to_string(),
            ));
        }

        // Employer must give notice for staff reduction (Article 180)
        if matches!(
            self.ground,
            TerminationGround::EmployerInitiative {
                reason: EmployerTerminationReason::StaffReduction
            }
        ) && self.notice_period_days < 60
        {
            return Err(LaborCodeError::InvalidTermination(
                "Employer must give at least 60 days notice for staff reduction".to_string(),
            ));
        }

        Ok(())
    }
}

/// Quick validation for employment contract
pub fn quick_validate_employment_contract(
    contract: &EmploymentContract,
) -> Result<(), LaborCodeError> {
    contract.validate()
}

/// Article 133: Minimum wage check
pub fn check_minimum_wage(salary: &crate::common::Currency) -> Result<(), LaborCodeError> {
    // As of 2024, federal minimum wage is approximately 19,242 RUB
    let min_wage = crate::common::Currency::from_rubles(19242);

    if salary.kopecks < min_wage.kopecks {
        return Err(LaborCodeError::InvalidContract(format!(
            "Salary {} is below minimum wage {}",
            salary, min_wage
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_time_regime() {
        let standard = WorkingTimeRegime::standard();
        assert_eq!(standard.hours_per_week, 40);
        assert!(standard.validate().is_ok());

        let excessive = WorkingTimeRegime {
            hours_per_week: 50,
            days_per_week: 5,
            start_time: chrono::NaiveTime::from_hms_opt(9, 0, 0).expect("Valid time"),
            end_time: chrono::NaiveTime::from_hms_opt(18, 0, 0).expect("Valid time"),
            lunch_break_minutes: 60,
        };
        assert!(excessive.validate().is_err());
    }

    #[test]
    fn test_employment_contract() {
        let contract = EmploymentContract::new(
            "ООО Компания",
            "Иванов Иван",
            chrono::NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date"),
            chrono::NaiveDate::from_ymd_opt(2024, 1, 15).expect("Valid date"),
            "Software Engineer",
            crate::common::Currency::from_rubles(100000),
        )
        .with_workplace("Moscow")
        .with_written_form(true);

        assert!(contract.validate().is_ok());
    }

    #[test]
    fn test_termination_notice() {
        let notice = TerminationNotice::new(
            "Employee",
            TerminationGround::EmployeeInitiative,
            chrono::NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date"),
            chrono::NaiveDate::from_ymd_opt(2024, 1, 16).expect("Valid date"),
        );

        assert!(notice.validate().is_ok());

        let short_notice = TerminationNotice::new(
            "Employee",
            TerminationGround::EmployeeInitiative,
            chrono::NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date"),
            chrono::NaiveDate::from_ymd_opt(2024, 1, 8).expect("Valid date"),
        );

        assert!(short_notice.validate().is_err());
    }

    #[test]
    fn test_minimum_wage() {
        let adequate = crate::common::Currency::from_rubles(25000);
        assert!(check_minimum_wage(&adequate).is_ok());

        let inadequate = crate::common::Currency::from_rubles(10000);
        assert!(check_minimum_wage(&inadequate).is_err());
    }
}
