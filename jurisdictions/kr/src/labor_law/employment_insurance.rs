//! Employment Insurance Act (고용보험법)
//!
//! # 고용보험법 / Employment Insurance Act
//!
//! Enacted: 1993
//!
//! Provides:
//! - Unemployment benefits (실업급여)
//! - Job search assistance
//! - Employment promotion programs

use crate::common::KrwAmount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Employment insurance errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum EmploymentInsuranceError {
    /// Ineligible for benefits
    #[error("Ineligible for benefits: {0}")]
    Ineligible(String),

    /// Calculation error
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

/// Result type for employment insurance operations
pub type EmploymentInsuranceResult<T> = Result<T, EmploymentInsuranceError>;

/// Reason for job loss
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobLossReason {
    /// Involuntary dismissal (비자발적 퇴사)
    InvoluntaryDismissal,
    /// Company closure (회사 폐업)
    CompanyClosure,
    /// Contract expiration (계약 만료)
    ContractExpiration,
    /// Voluntary resignation (자발적 퇴사)
    VoluntaryResignation,
}

/// Unemployment claim (실업급여 신청)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnemploymentClaim {
    /// Claimant name
    pub claimant: String,
    /// Last employment start date
    pub employment_start_date: NaiveDate,
    /// Last employment end date
    pub employment_end_date: NaiveDate,
    /// Reason for job loss
    pub reason: JobLossReason,
    /// Average wage (last 3 months)
    pub average_wage: KrwAmount,
    /// Total insured period (months)
    pub insured_months: u32,
}

impl UnemploymentClaim {
    /// Create new unemployment claim
    pub fn new(
        claimant: impl Into<String>,
        employment_start_date: NaiveDate,
        employment_end_date: NaiveDate,
        reason: JobLossReason,
        average_wage: KrwAmount,
        insured_months: u32,
    ) -> Self {
        Self {
            claimant: claimant.into(),
            employment_start_date,
            employment_end_date,
            reason,
            average_wage,
            insured_months,
        }
    }
}

/// Check eligibility for unemployment benefits
/// Requirements:
/// - Must have been insured for at least 180 days (6 months) in last 18 months
/// - Must be involuntarily unemployed (not voluntary resignation)
pub fn check_eligibility(claim: &UnemploymentClaim) -> EmploymentInsuranceResult<bool> {
    // Check insured period
    if claim.insured_months < 6 {
        return Err(EmploymentInsuranceError::Ineligible(
            "Must be insured for at least 6 months".to_string(),
        ));
    }

    // Check reason for job loss
    if claim.reason == JobLossReason::VoluntaryResignation {
        return Err(EmploymentInsuranceError::Ineligible(
            "Voluntary resignation is not eligible for unemployment benefits".to_string(),
        ));
    }

    Ok(true)
}

/// Calculate unemployment benefit amount
/// Benefit is 60% of average wage, with minimum and maximum limits
pub fn calculate_benefit_amount(
    average_daily_wage: &KrwAmount,
) -> EmploymentInsuranceResult<KrwAmount> {
    // 60% of average daily wage
    let benefit_rate = 0.6;
    let benefit = average_daily_wage.multiply(benefit_rate);

    // Minimum benefit: 60% of minimum wage daily amount (approx. 60,000 KRW/day)
    let minimum_benefit = KrwAmount::new(60_000.0);

    // Maximum benefit (approx. 66,000 KRW/day)
    let maximum_benefit = KrwAmount::new(66_000.0);

    let final_benefit = if benefit.won < minimum_benefit.won {
        minimum_benefit
    } else if benefit.won > maximum_benefit.won {
        maximum_benefit
    } else {
        benefit
    };

    Ok(final_benefit)
}

/// Calculate benefit period (in days)
/// Based on age and insured period
pub fn calculate_benefit_period(age: u32, insured_months: u32) -> u32 {
    if age < 50 {
        if insured_months < 12 {
            120 // 4 months
        } else if insured_months < 36 {
            150 // 5 months
        } else if insured_months < 60 {
            180 // 6 months
        } else if insured_months < 120 {
            210 // 7 months
        } else {
            240 // 8 months
        }
    } else {
        // Age 50+
        if insured_months < 12 {
            120
        } else if insured_months < 36 {
            180
        } else if insured_months < 60 {
            210
        } else if insured_months < 120 {
            240
        } else {
            270 // 9 months
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unemployment_claim_creation() {
        if let (Some(start), Some(end)) = (
            NaiveDate::from_ymd_opt(2023, 1, 1),
            NaiveDate::from_ymd_opt(2024, 1, 1),
        ) {
            let claim = UnemploymentClaim::new(
                "김철수",
                start,
                end,
                JobLossReason::InvoluntaryDismissal,
                KrwAmount::from_man(300.0),
                12,
            );

            assert_eq!(claim.claimant, "김철수");
            assert_eq!(claim.insured_months, 12);
        }
    }

    #[test]
    fn test_check_eligibility() {
        if let (Some(start), Some(end)) = (
            NaiveDate::from_ymd_opt(2023, 1, 1),
            NaiveDate::from_ymd_opt(2024, 1, 1),
        ) {
            let claim = UnemploymentClaim::new(
                "김철수",
                start,
                end,
                JobLossReason::InvoluntaryDismissal,
                KrwAmount::from_man(300.0),
                12,
            );

            let result = check_eligibility(&claim);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_check_eligibility_insufficient_period() {
        if let (Some(start), Some(end)) = (
            NaiveDate::from_ymd_opt(2023, 10, 1),
            NaiveDate::from_ymd_opt(2024, 1, 1),
        ) {
            let claim = UnemploymentClaim::new(
                "김철수",
                start,
                end,
                JobLossReason::InvoluntaryDismissal,
                KrwAmount::from_man(300.0),
                3,
            );

            let result = check_eligibility(&claim);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_calculate_benefit_amount() {
        let daily_wage = KrwAmount::new(120_000.0);
        let result = calculate_benefit_amount(&daily_wage);
        assert!(result.is_ok());

        if let Ok(benefit) = result {
            // Should be capped at maximum (66,000)
            // 60% of 120,000 = 72,000, capped to 66,000
            assert!((benefit.won - 66_000.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_calculate_benefit_period() {
        let period1 = calculate_benefit_period(30, 12);
        assert_eq!(period1, 150);

        let period2 = calculate_benefit_period(55, 59);
        assert_eq!(period2, 210);
    }
}
