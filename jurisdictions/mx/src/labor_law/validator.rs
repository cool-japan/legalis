//! Labor law validation

use super::types::EmploymentContract;
use super::working_hours;
use thiserror::Error;

/// Labor law validation error
#[derive(Debug, Error)]
pub enum LaborValidationError {
    #[error("Working hours violation: {0}")]
    WorkingHoursViolation(String),
    #[error("Invalid contract: {0}")]
    InvalidContract(String),
    #[error("Minimum wage violation")]
    MinimumWageViolation,
}

/// Validate employment contract
pub fn validate_employment_contract(
    contract: &EmploymentContract,
) -> Result<(), LaborValidationError> {
    // Validate working hours
    if let Err(e) = working_hours::validate_schedule(&contract.jornada) {
        return Err(LaborValidationError::WorkingHoursViolation(e.to_string()));
    }

    // Validate employee name
    if contract.trabajador.is_empty() {
        return Err(LaborValidationError::InvalidContract(
            "employee name required".to_string(),
        ));
    }

    // Validate employer name
    if contract.patron.is_empty() {
        return Err(LaborValidationError::InvalidContract(
            "employer name required".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::MexicanCurrency;
    use crate::labor_law::types::{EmploymentType, WorkSchedule};
    use chrono::Utc;

    #[test]
    fn test_validate_contract() {
        let contract = EmploymentContract::new(
            "Juan Pérez".to_string(),
            "Empresa SA".to_string(),
            EmploymentType::Indefinite,
            MexicanCurrency::from_pesos(300),
            WorkSchedule::standard_day(),
            Utc::now(),
        );

        assert!(validate_employment_contract(&contract).is_ok());
    }
}
