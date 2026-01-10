//! Withdrawal rights implementation (Articles 9-16)

use super::error::ConsumerRightsError;
use super::types::{DistanceContract, OffPremisesContract, WithdrawalException};
use chrono::{DateTime, Duration, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Withdrawal period calculation result
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WithdrawalPeriod {
    /// Start date of withdrawal period
    pub start_date: DateTime<Utc>,

    /// Deadline for withdrawal (14 days from start, or extended)
    pub deadline: DateTime<Utc>,

    /// Number of days in the period (14 or extended to 12 months)
    pub period_days: i64,

    /// Days remaining (negative if expired)
    pub days_remaining: i64,

    /// Whether period was extended due to missing information
    pub extended: bool,

    /// Reason for extension if applicable
    pub extension_reason: Option<String>,
}

impl WithdrawalPeriod {
    /// Check if withdrawal period is still active
    pub fn is_active(&self) -> bool {
        self.days_remaining >= 0
    }

    /// Check if period was extended
    pub fn is_extended(&self) -> bool {
        self.extended
    }
}

/// Withdrawal right validator
#[derive(Debug, Clone)]
pub struct WithdrawalRight {
    contract_date: Option<DateTime<Utc>>,
    delivery_date: Option<DateTime<Utc>>,
    information_complete: bool,
    #[allow(dead_code)]
    withdrawal_form_provided: bool,
    exception: Option<WithdrawalException>,
}

impl WithdrawalRight {
    pub fn new() -> Self {
        Self {
            contract_date: None,
            delivery_date: None,
            information_complete: false,
            withdrawal_form_provided: false,
            exception: None,
        }
    }

    /// Create from distance contract
    pub fn from_distance_contract(contract: &DistanceContract) -> Self {
        let information_complete = Self::check_information_complete(
            &contract.information_provided,
            contract.withdrawal_form_provided,
        );

        Self {
            contract_date: contract.contract_date,
            delivery_date: contract.delivery_date,
            information_complete,
            withdrawal_form_provided: contract.withdrawal_form_provided,
            exception: None,
        }
    }

    /// Create from off-premises contract
    pub fn from_off_premises_contract(contract: &OffPremisesContract) -> Self {
        let information_complete = Self::check_information_complete(
            &contract.information_provided,
            contract.withdrawal_form_provided,
        );

        Self {
            contract_date: contract.contract_date,
            delivery_date: None, // Off-premises contracts don't have delivery dates
            information_complete,
            withdrawal_form_provided: contract.withdrawal_form_provided,
            exception: None,
        }
    }

    pub fn with_exception(mut self, exception: WithdrawalException) -> Self {
        self.exception = Some(exception);
        self
    }

    /// Check if required information was provided (Article 6)
    fn check_information_complete(
        _provided: &[super::types::InformationRequirement],
        withdrawal_form: bool,
    ) -> bool {
        // Simplified check - in real implementation would verify all required fields
        // For now, just check if withdrawal form was provided
        withdrawal_form
    }

    /// Calculate withdrawal period (Articles 9-10)
    pub fn calculate_period(&self) -> Result<WithdrawalPeriod, ConsumerRightsError> {
        // Check for exceptions first
        if let Some(ref exception) = self.exception {
            return Err(ConsumerRightsError::withdrawal_exception(format!(
                "Article 17 exception applies: {:?}",
                exception
            )));
        }

        // Determine start date (Article 9)
        // For goods: from receipt of goods (delivery_date)
        // For services: from contract conclusion (contract_date)
        let start_date = self
            .delivery_date
            .or(self.contract_date)
            .ok_or_else(|| ConsumerRightsError::missing_field("contract_date or delivery_date"))?;

        // Standard withdrawal period: 14 days (Article 9(1))
        let standard_deadline = start_date + Duration::days(14);

        // Extended period if information not provided: 12 months (Article 10)
        let (deadline, period_days, extended, extension_reason) = if !self.information_complete {
            let extended_deadline = start_date + Duration::days(365); // 12 months
            (
                extended_deadline,
                365,
                true,
                Some(
                    "Information requirements not met - period extended to 12 months (Article 10)"
                        .to_string(),
                ),
            )
        } else {
            (standard_deadline, 14, false, None)
        };

        // Calculate days remaining
        let now = Utc::now();
        let days_remaining = (deadline - now).num_days();

        Ok(WithdrawalPeriod {
            start_date,
            deadline,
            period_days,
            days_remaining,
            extended,
            extension_reason,
        })
    }

    /// Validate withdrawal notice
    pub fn validate_withdrawal(
        &self,
        withdrawal_date: DateTime<Utc>,
    ) -> Result<(), ConsumerRightsError> {
        let period = self.calculate_period()?;

        if withdrawal_date > period.deadline {
            return Err(ConsumerRightsError::WithdrawalPeriodExpired {
                deadline: period.deadline.to_rfc3339(),
            });
        }

        Ok(())
    }
}

impl Default for WithdrawalRight {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consumer_rights::types::InformationRequirement;

    #[test]
    fn test_standard_withdrawal_period() {
        let start = Utc::now();

        let right = WithdrawalRight {
            contract_date: Some(start),
            delivery_date: Some(start),
            information_complete: true,
            withdrawal_form_provided: true,
            exception: None,
        };

        let period = right.calculate_period().unwrap();
        assert_eq!(period.period_days, 14);
        assert!(!period.extended);
        assert!(period.is_active());
    }

    #[test]
    fn test_extended_withdrawal_period_missing_info() {
        let start = Utc::now();

        let right = WithdrawalRight {
            contract_date: Some(start),
            delivery_date: Some(start),
            information_complete: false, // Missing required information
            withdrawal_form_provided: false,
            exception: None,
        };

        let period = right.calculate_period().unwrap();
        assert_eq!(period.period_days, 365); // Extended to 12 months
        assert!(period.extended);
        assert!(period.extension_reason.is_some());
    }

    #[test]
    fn test_withdrawal_with_exception() {
        let right = WithdrawalRight {
            contract_date: Some(Utc::now()),
            delivery_date: None,
            information_complete: true,
            withdrawal_form_provided: true,
            exception: Some(WithdrawalException::PerishableGoods),
        };

        let result = right.calculate_period();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConsumerRightsError::WithdrawalExceptionApplies { .. }
        ));
    }

    #[test]
    fn test_expired_withdrawal_period() {
        let start = Utc::now() - Duration::days(20); // 20 days ago

        let right = WithdrawalRight {
            contract_date: Some(start),
            delivery_date: Some(start),
            information_complete: true,
            withdrawal_form_provided: true,
            exception: None,
        };

        let period = right.calculate_period().unwrap();
        assert!(!period.is_active());
        assert!(period.days_remaining < 0);

        let withdrawal_attempt = Utc::now();
        let result = right.validate_withdrawal(withdrawal_attempt);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConsumerRightsError::WithdrawalPeriodExpired { .. }
        ));
    }

    #[test]
    fn test_from_distance_contract() {
        let contract = DistanceContract::new()
            .with_trader("Online Shop")
            .with_consumer("John Doe")
            .with_contract_date(Utc::now())
            .with_delivery_date(Utc::now())
            .with_information(InformationRequirement::RightOfWithdrawal)
            .with_withdrawal_form(true);

        let right = WithdrawalRight::from_distance_contract(&contract);
        assert!(right.information_complete);
        assert!(right.withdrawal_form_provided);
    }
}
