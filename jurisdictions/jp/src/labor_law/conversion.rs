//! Article 18 Indefinite-Term Conversion (無期転換ルール)
//!
//! Implementation of Labor Contract Act Article 18, which grants employees
//! the right to convert from fixed-term to indefinite-term employment after
//! 5 years of continuous service.
//!
//! # Legal Basis
//!
//! Labor Contract Act Article 18 (労働契約法第18条)
//! Fixed-term employees who have been employed for more than 5 years
//! have the right to request conversion to indefinite-term employment.
//!
//! # Example
//!
//! ```
//! use legalis_jp::labor_law::{EmploymentContract, EmploymentType, IndefiniteConversionBuilder};
//! use chrono::{Utc, Duration};
//!
//! let mut original = EmploymentContract {
//!     employee_name: "山田太郎".to_string(),
//!     employer_name: "テクノロジー株式会社".to_string(),
//!     employment_type: EmploymentType::FixedTerm,
//!     work_pattern: legalis_jp::labor_law::WorkPattern::Regular,
//!     start_date: Utc::now() - Duration::days(1826), // 5+ years ago
//!     end_date: Some(Utc::now() + Duration::days(30)),
//!     base_wage_jpy: 350_000,
//!     hours_per_day: 8,
//!     days_per_week: 5,
//!     job_description: "Software Engineer".to_string(),
//!     work_location: "Tokyo Office".to_string(),
//!     probation_period_days: None,
//!     renewal_count: 5,
//! };
//!
//! // Employee requests conversion
//! let converter = IndefiniteConversionBuilder::from_fixed_term(&original)
//!     .expect("Employee is eligible");
//!
//! let indefinite_contract = converter.build().expect("Conversion succeeded");
//! assert_eq!(indefinite_contract.employment_type, EmploymentType::IndefiniteTerm);
//! ```

use super::error::{LaborLawError, Result};
use super::types::{EmploymentContract, EmploymentType};
use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Minimum years of service required for indefinite conversion (最低勤続年数)
pub const CONVERSION_REQUIRED_YEARS: u32 = 5;

/// Indefinite-term conversion builder (無期転換ビルダー)
///
/// Converts a fixed-term employment contract to indefinite-term
/// under Labor Contract Act Article 18.
///
/// # Legal Requirements
///
/// 1. Fixed-term contract with 5+ years of service
/// 2. Employee must request conversion
/// 3. Conversion must occur before current term expires
/// 4. Terms cannot be worse than original (不利益変更禁止)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IndefiniteConversionBuilder {
    /// Original fixed-term contract
    original_contract: EmploymentContract,

    /// Conversion effective date (転換日)
    conversion_date: DateTime<Utc>,

    /// Modified base wage (変更後基本給)
    new_base_wage: Option<u64>,

    /// Modified work pattern (変更後勤務形態)
    new_work_pattern: Option<super::types::WorkPattern>,

    /// Modified job description (変更後業務内容)
    new_job_description: Option<String>,

    /// Modified work location (変更後勤務地)
    new_work_location: Option<String>,
}

impl IndefiniteConversionBuilder {
    /// Create conversion builder from fixed-term contract
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Contract is not fixed-term
    /// - Employee has not worked for 5+ years
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_jp::labor_law::{EmploymentContract, EmploymentType, IndefiniteConversionBuilder};
    /// use chrono::{Utc, Duration};
    ///
    /// let contract = EmploymentContract {
    ///     employee_name: "山田太郎".to_string(),
    ///     employer_name: "株式会社ABC".to_string(),
    ///     employment_type: EmploymentType::FixedTerm,
    ///     work_pattern: legalis_jp::labor_law::WorkPattern::Regular,
    ///     start_date: Utc::now() - Duration::days(1826),
    ///     end_date: Some(Utc::now() + Duration::days(30)),
    ///     base_wage_jpy: 350_000,
    ///     hours_per_day: 8,
    ///     days_per_week: 5,
    ///     job_description: "Engineer".to_string(),
    ///     work_location: "Tokyo".to_string(),
    ///     probation_period_days: None,
    ///     renewal_count: 5,
    /// };
    ///
    /// let converter = IndefiniteConversionBuilder::from_fixed_term(&contract);
    /// assert!(converter.is_ok());
    /// ```
    pub fn from_fixed_term(contract: &EmploymentContract) -> Result<Self> {
        // Check that contract is fixed-term
        if contract.employment_type != EmploymentType::FixedTerm {
            return Err(LaborLawError::InvalidContractType {
                expected: "FixedTerm".to_string(),
                actual: format!("{:?}", contract.employment_type),
            });
        }

        // Check eligibility (5+ years)
        if !contract.is_eligible_for_indefinite_conversion() {
            let years = (Utc::now() - contract.start_date).num_days() as f64 / 365.0;
            return Err(LaborLawError::NotEligibleForConversion {
                years_worked: years,
                required_years: CONVERSION_REQUIRED_YEARS,
            });
        }

        Ok(Self {
            original_contract: contract.clone(),
            conversion_date: Utc::now(),
            new_base_wage: None,
            new_work_pattern: None,
            new_job_description: None,
            new_work_location: None,
        })
    }

    /// Set conversion effective date (転換日設定)
    pub fn with_conversion_date(mut self, date: DateTime<Utc>) -> Self {
        self.conversion_date = date;
        self
    }

    /// Preserve all original terms (原契約条件維持)
    ///
    /// This is the default - all terms remain the same except employment type.
    pub fn preserve_terms(self) -> Self {
        self
    }

    /// Modify base wage (基本給変更)
    ///
    /// Note: New wage must not be lower than original (不利益変更禁止)
    pub fn with_new_salary(mut self, wage: u64) -> Self {
        self.new_base_wage = Some(wage);
        self
    }

    /// Modify work pattern (勤務形態変更)
    pub fn with_new_work_pattern(mut self, pattern: super::types::WorkPattern) -> Self {
        self.new_work_pattern = Some(pattern);
        self
    }

    /// Modify job description (業務内容変更)
    pub fn with_new_job_description(mut self, description: impl Into<String>) -> Self {
        self.new_job_description = Some(description.into());
        self
    }

    /// Modify work location (勤務地変更)
    pub fn with_new_work_location(mut self, location: impl Into<String>) -> Self {
        self.new_work_location = Some(location.into());
        self
    }

    /// Build the indefinite-term contract (無期契約生成)
    ///
    /// # Errors
    ///
    /// Returns error if modifications make terms worse than original.
    ///
    /// # Example
    ///
    /// ```
    /// # use legalis_jp::labor_law::{EmploymentContract, EmploymentType, IndefiniteConversionBuilder};
    /// # use chrono::{Utc, Duration};
    /// # let contract = EmploymentContract {
    /// #     employee_name: "山田太郎".to_string(),
    /// #     employer_name: "株式会社ABC".to_string(),
    /// #     employment_type: EmploymentType::FixedTerm,
    /// #     work_pattern: legalis_jp::labor_law::WorkPattern::Regular,
    /// #     start_date: Utc::now() - Duration::days(1826),
    /// #     end_date: Some(Utc::now() + Duration::days(30)),
    /// #     base_wage_jpy: 350_000,
    /// #     hours_per_day: 8,
    /// #     days_per_week: 5,
    /// #     job_description: "Engineer".to_string(),
    /// #     work_location: "Tokyo".to_string(),
    /// #     probation_period_days: None,
    /// #     renewal_count: 5,
    /// # };
    /// let converter = IndefiniteConversionBuilder::from_fixed_term(&contract).unwrap();
    /// let indefinite = converter.build().unwrap();
    ///
    /// assert_eq!(indefinite.employment_type, EmploymentType::IndefiniteTerm);
    /// assert_eq!(indefinite.end_date, None);
    /// ```
    pub fn build(self) -> Result<EmploymentContract> {
        // Check for adverse changes (不利益変更チェック)
        if let Some(new_wage) = self.new_base_wage
            && new_wage < self.original_contract.base_wage_jpy
        {
            return Err(LaborLawError::AdverseChange {
                field: "base_wage_jpy".to_string(),
                reason: format!(
                    "New wage ¥{} is lower than original ¥{}",
                    new_wage, self.original_contract.base_wage_jpy
                ),
            });
        }

        // Create new indefinite-term contract
        Ok(EmploymentContract {
            employee_name: self.original_contract.employee_name.clone(),
            employer_name: self.original_contract.employer_name.clone(),
            employment_type: EmploymentType::IndefiniteTerm,
            work_pattern: self
                .new_work_pattern
                .unwrap_or(self.original_contract.work_pattern),
            start_date: self.conversion_date,
            end_date: None, // Indefinite-term has no end date
            base_wage_jpy: self
                .new_base_wage
                .unwrap_or(self.original_contract.base_wage_jpy),
            hours_per_day: self.original_contract.hours_per_day,
            days_per_week: self.original_contract.days_per_week,
            job_description: self
                .new_job_description
                .unwrap_or(self.original_contract.job_description.clone()),
            work_location: self
                .new_work_location
                .unwrap_or(self.original_contract.work_location.clone()),
            probation_period_days: None, // No probation for conversion
            renewal_count: 0,            // Reset renewal count
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn create_eligible_contract() -> EmploymentContract {
        EmploymentContract {
            employee_name: "山田太郎".to_string(),
            employer_name: "テクノロジー株式会社".to_string(),
            employment_type: EmploymentType::FixedTerm,
            work_pattern: super::super::types::WorkPattern::Regular,
            start_date: Utc::now() - Duration::days(1826), // 5+ years
            end_date: Some(Utc::now() + Duration::days(30)),
            base_wage_jpy: 350_000,
            hours_per_day: 8,
            days_per_week: 5,
            job_description: "Software Engineer".to_string(),
            work_location: "Tokyo Office".to_string(),
            probation_period_days: None,
            renewal_count: 5,
        }
    }

    #[test]
    fn test_from_fixed_term_success() {
        let contract = create_eligible_contract();
        let result = IndefiniteConversionBuilder::from_fixed_term(&contract);
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_fixed_term_not_fixed_term() {
        let mut contract = create_eligible_contract();
        contract.employment_type = EmploymentType::IndefiniteTerm;

        let result = IndefiniteConversionBuilder::from_fixed_term(&contract);
        assert!(result.is_err());

        match result.unwrap_err() {
            LaborLawError::InvalidContractType { .. } => {}
            _ => panic!("Expected InvalidContractType error"),
        }
    }

    #[test]
    fn test_from_fixed_term_not_eligible() {
        let mut contract = create_eligible_contract();
        contract.start_date = Utc::now() - Duration::days(1000); // < 5 years

        let result = IndefiniteConversionBuilder::from_fixed_term(&contract);
        assert!(result.is_err());

        match result.unwrap_err() {
            LaborLawError::NotEligibleForConversion { .. } => {}
            _ => panic!("Expected NotEligibleForConversion error"),
        }
    }

    #[test]
    fn test_build_preserve_terms() {
        let contract = create_eligible_contract();
        let converter = IndefiniteConversionBuilder::from_fixed_term(&contract).unwrap();

        let indefinite = converter.preserve_terms().build().unwrap();

        assert_eq!(indefinite.employment_type, EmploymentType::IndefiniteTerm);
        assert_eq!(indefinite.end_date, None);
        assert_eq!(indefinite.base_wage_jpy, contract.base_wage_jpy);
        assert_eq!(indefinite.employee_name, contract.employee_name);
        assert_eq!(indefinite.probation_period_days, None);
        assert_eq!(indefinite.renewal_count, 0);
    }

    #[test]
    fn test_build_with_wage_increase() {
        let contract = create_eligible_contract();
        let converter = IndefiniteConversionBuilder::from_fixed_term(&contract).unwrap();

        let indefinite = converter.with_new_salary(400_000).build().unwrap();

        assert_eq!(indefinite.base_wage_jpy, 400_000);
    }

    #[test]
    fn test_build_with_adverse_wage_change() {
        let contract = create_eligible_contract();
        let converter = IndefiniteConversionBuilder::from_fixed_term(&contract).unwrap();

        let result = converter.with_new_salary(300_000).build(); // Lower than original

        assert!(result.is_err());
        match result.unwrap_err() {
            LaborLawError::AdverseChange { field, .. } => {
                assert_eq!(field, "base_wage_jpy");
            }
            _ => panic!("Expected AdverseChange error"),
        }
    }

    #[test]
    fn test_build_with_modified_job() {
        let contract = create_eligible_contract();
        let converter = IndefiniteConversionBuilder::from_fixed_term(&contract).unwrap();

        let indefinite = converter
            .with_new_job_description("Senior Software Engineer")
            .with_new_work_location("Osaka Branch")
            .build()
            .unwrap();

        assert_eq!(indefinite.job_description, "Senior Software Engineer");
        assert_eq!(indefinite.work_location, "Osaka Branch");
    }

    #[test]
    fn test_conversion_date() {
        let contract = create_eligible_contract();
        let conversion_date = Utc::now() + Duration::days(30);

        let converter = IndefiniteConversionBuilder::from_fixed_term(&contract).unwrap();
        let indefinite = converter
            .with_conversion_date(conversion_date)
            .build()
            .unwrap();

        // Allow for small time differences
        let diff = (indefinite.start_date - conversion_date)
            .num_seconds()
            .abs();
        assert!(diff < 2);
    }
}
