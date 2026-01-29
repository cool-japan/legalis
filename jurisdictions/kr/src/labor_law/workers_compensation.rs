//! Industrial Accident Compensation Insurance Act (산업재해보상보험법)
//!
//! # 산업재해보상보험법 / Industrial Accident Compensation Insurance Act
//!
//! Enacted: 1963
//!
//! Provides:
//! - Medical benefits (요양급여)
//! - Disability benefits (장해급여)
//! - Survivors' benefits (유족급여)
//! - Funeral expenses (장의비)

use crate::common::KrwAmount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Workers' compensation errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum WorkersCompensationError {
    /// Invalid claim
    #[error("Invalid claim: {0}")]
    InvalidClaim(String),

    /// Calculation error
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

/// Result type for workers' compensation operations
pub type WorkersCompensationResult<T> = Result<T, WorkersCompensationError>;

/// Type of industrial accident
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccidentType {
    /// Occupational injury (업무상 부상)
    OccupationalInjury,
    /// Occupational disease (업무상 질병)
    OccupationalDisease,
    /// Commuting accident (출퇴근 재해)
    CommutingAccident,
    /// Death (사망)
    Death,
}

/// Disability grade (장해등급)
/// Grade 1 = Most severe, Grade 14 = Least severe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisabilityGrade(pub u8);

impl DisabilityGrade {
    /// Create new disability grade (1-14)
    pub fn new(grade: u8) -> Option<Self> {
        if (1..=14).contains(&grade) {
            Some(DisabilityGrade(grade))
        } else {
            None
        }
    }

    /// Get disability rate (percentage)
    pub fn disability_rate(&self) -> f64 {
        match self.0 {
            1 => 100.0,
            2 => 100.0,
            3 => 100.0,
            4 => 92.0,
            5 => 82.0,
            6 => 72.0,
            7 => 62.0,
            8 => 52.0,
            9 => 42.0,
            10 => 32.0,
            11 => 22.0,
            12 => 14.0,
            13 => 8.0,
            14 => 4.0,
            _ => 0.0,
        }
    }
}

/// Industrial accident claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndustrialAccidentClaim {
    /// Claimant/injured worker
    pub claimant: String,
    /// Accident date
    pub accident_date: NaiveDate,
    /// Accident type
    pub accident_type: AccidentType,
    /// Description
    pub description: String,
    /// Average daily wage
    pub average_daily_wage: KrwAmount,
    /// Disability grade (if applicable)
    pub disability_grade: Option<DisabilityGrade>,
}

impl IndustrialAccidentClaim {
    /// Create new claim
    pub fn new(
        claimant: impl Into<String>,
        accident_date: NaiveDate,
        accident_type: AccidentType,
        description: impl Into<String>,
        average_daily_wage: KrwAmount,
    ) -> Self {
        Self {
            claimant: claimant.into(),
            accident_date,
            accident_type,
            description: description.into(),
            average_daily_wage,
            disability_grade: None,
        }
    }

    /// Set disability grade
    pub fn with_disability_grade(mut self, grade: DisabilityGrade) -> Self {
        self.disability_grade = Some(grade);
        self
    }
}

/// Calculate temporary disability benefits (휴업급여)
/// 70% of average daily wage per day of absence
pub fn calculate_temporary_disability_benefit(
    average_daily_wage: &KrwAmount,
    days: u32,
) -> WorkersCompensationResult<KrwAmount> {
    if days == 0 {
        return Err(WorkersCompensationError::CalculationError(
            "Days must be greater than 0".to_string(),
        ));
    }

    let benefit_rate = 0.7;
    let daily_benefit = average_daily_wage.multiply(benefit_rate);
    Ok(daily_benefit.multiply(days as f64))
}

/// Calculate permanent disability benefits (장해급여)
/// Lump sum: Average wage × disability rate × number of days
pub fn calculate_permanent_disability_benefit(
    average_daily_wage: &KrwAmount,
    disability_grade: DisabilityGrade,
) -> WorkersCompensationResult<KrwAmount> {
    // Number of days varies by grade
    let days = match disability_grade.0 {
        1 => 1_474,
        2 => 1_376,
        3 => 1_278,
        4 => 1_155,
        5 => 1_012,
        6 => 869,
        7 => 726,
        8 => 616,
        9 => 506,
        10 => 396,
        11 => 286,
        12 => 198,
        13 => 132,
        14 => 66,
        _ => 0,
    };

    if days == 0 {
        return Err(WorkersCompensationError::CalculationError(
            "Invalid disability grade".to_string(),
        ));
    }

    let benefit = average_daily_wage.multiply(days as f64);
    Ok(benefit)
}

/// Calculate survivors' benefits (유족급여)
/// For death: Average wage × 1,300 days (lump sum)
/// Or monthly pension option
pub fn calculate_survivors_benefit(
    average_daily_wage: &KrwAmount,
) -> WorkersCompensationResult<KrwAmount> {
    let days = 1_300;
    let benefit = average_daily_wage.multiply(days as f64);
    Ok(benefit)
}

/// Calculate funeral expenses (장의비)
/// 120 days of average wage, minimum 10,000,000 KRW
pub fn calculate_funeral_expense(
    average_daily_wage: &KrwAmount,
) -> WorkersCompensationResult<KrwAmount> {
    let days = 120;
    let calculated = average_daily_wage.multiply(days as f64);

    let minimum = KrwAmount::from_man(1_000.0); // 10,000,000 KRW

    if calculated.won < minimum.won {
        Ok(minimum)
    } else {
        Ok(calculated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disability_grade() {
        let grade1 = DisabilityGrade::new(1);
        assert!(grade1.is_some());
        if let Some(g) = grade1 {
            assert_eq!(g.disability_rate(), 100.0);
        }

        let grade14 = DisabilityGrade::new(14);
        assert!(grade14.is_some());
        if let Some(g) = grade14 {
            assert_eq!(g.disability_rate(), 4.0);
        }

        let invalid = DisabilityGrade::new(15);
        assert!(invalid.is_none());
    }

    #[test]
    fn test_industrial_accident_claim() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let claim = IndustrialAccidentClaim::new(
                "김철수",
                date,
                AccidentType::OccupationalInjury,
                "작업 중 낙상",
                KrwAmount::new(100_000.0),
            );

            assert_eq!(claim.claimant, "김철수");
            assert_eq!(claim.accident_type, AccidentType::OccupationalInjury);
        }
    }

    #[test]
    fn test_calculate_temporary_disability_benefit() {
        let daily_wage = KrwAmount::new(100_000.0);
        let result = calculate_temporary_disability_benefit(&daily_wage, 30);
        assert!(result.is_ok());

        if let Ok(benefit) = result {
            // 100,000 * 0.7 * 30 = 2,100,000
            assert!((benefit.won - 2_100_000.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_calculate_permanent_disability_benefit() {
        let daily_wage = KrwAmount::new(100_000.0);
        if let Some(grade) = DisabilityGrade::new(7) {
            let result = calculate_permanent_disability_benefit(&daily_wage, grade);
            assert!(result.is_ok());

            if let Ok(benefit) = result {
                // 100,000 * 726 = 72,600,000
                assert!((benefit.won - 72_600_000.0).abs() < 0.01);
            }
        }
    }

    #[test]
    fn test_calculate_survivors_benefit() {
        let daily_wage = KrwAmount::new(100_000.0);
        let result = calculate_survivors_benefit(&daily_wage);
        assert!(result.is_ok());

        if let Ok(benefit) = result {
            // 100,000 * 1,300 = 130,000,000
            assert!((benefit.won - 130_000_000.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_calculate_funeral_expense() {
        let daily_wage = KrwAmount::new(100_000.0);
        let result = calculate_funeral_expense(&daily_wage);
        assert!(result.is_ok());

        if let Ok(expense) = result {
            // 100,000 * 120 = 12,000,000 (above minimum)
            assert!((expense.won - 12_000_000.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_calculate_funeral_expense_minimum() {
        let daily_wage = KrwAmount::new(50_000.0);
        let result = calculate_funeral_expense(&daily_wage);
        assert!(result.is_ok());

        if let Ok(expense) = result {
            // 50,000 * 120 = 6,000,000 (below minimum, so should be 10,000,000)
            assert!((expense.won - 10_000_000.0).abs() < 0.01);
        }
    }
}
