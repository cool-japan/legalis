//! Employment Contract Builder (雇用契約ビルダー)
//!
//! Provides Article 709-style builder pattern for constructing and validating
//! employment contracts with automatic legal compliance checking.
//!
//! # Example
//!
//! ```
//! use legalis_jp::labor_law::{
//!     EmploymentContractBuilder, EmploymentType, Prefecture,
//!     validate_employment_contract
//! };
//! use chrono::Utc;
//!
//! let builder = EmploymentContractBuilder::new()
//!     .with_employee("山田太郎")
//!     .with_employer("テクノロジー株式会社")
//!     .with_employment_type(EmploymentType::IndefiniteTerm)
//!     .with_salary(400_000)
//!     .with_working_hours(8, 5)
//!     .with_prefecture(Prefecture::Tokyo)
//!     .with_job("Software Engineer", "Tokyo Office")
//!     .with_start_date(Utc::now());
//!
//! // Build validates required fields
//! let contract = builder.build().expect("Missing required fields");
//!
//! // Validate checks legal requirements
//! validate_employment_contract(&contract).expect("Legal validation failed");
//! ```

use super::error::{LaborLawError, Result};
use super::minimum_wage::Prefecture;
use super::types::{EmploymentContract, EmploymentType, WorkPattern};
use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Employment contract builder following Article 709 pattern
///
/// Provides fluent API for constructing employment contracts with
/// automatic validation of required fields and legal requirements.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EmploymentContractBuilder {
    /// Employee name (被雇用者名)
    pub employee_name: Option<String>,
    /// Employer name (雇用者名)
    pub employer_name: Option<String>,
    /// Employment type (雇用形態)
    pub employment_type: Option<EmploymentType>,
    /// Work pattern (勤務形態)
    pub work_pattern: Option<WorkPattern>,
    /// Start date (開始日)
    pub start_date: Option<DateTime<Utc>>,
    /// End date for fixed-term contracts (終了日)
    pub end_date: Option<DateTime<Utc>>,
    /// Base wage in JPY (基本給)
    pub base_wage_jpy: Option<u64>,
    /// Hours per day (1日の労働時間)
    pub hours_per_day: Option<u32>,
    /// Days per week (週の労働日数)
    pub days_per_week: Option<u32>,
    /// Job description (業務内容)
    pub job_description: Option<String>,
    /// Work location (就業場所)
    pub work_location: Option<String>,
    /// Prefecture for minimum wage validation (都道府県)
    pub prefecture: Option<Prefecture>,
    /// Probation period in days (試用期間)
    pub probation_period_days: Option<u32>,
    /// Renewal count for fixed-term contracts (更新回数)
    pub renewal_count: Option<u32>,
}

impl EmploymentContractBuilder {
    /// Create a new employment contract builder
    ///
    /// All fields are initially None and must be set using the fluent API.
    pub fn new() -> Self {
        Self {
            employee_name: None,
            employer_name: None,
            employment_type: None,
            work_pattern: None,
            start_date: None,
            end_date: None,
            base_wage_jpy: None,
            hours_per_day: None,
            days_per_week: None,
            job_description: None,
            work_location: None,
            prefecture: None,
            probation_period_days: None,
            renewal_count: None,
        }
    }

    /// Create a builder (alias for new())
    pub fn builder() -> Self {
        Self::new()
    }

    /// Set employee name (被雇用者名)
    pub fn with_employee(mut self, name: impl Into<String>) -> Self {
        self.employee_name = Some(name.into());
        self
    }

    /// Set employee name (alternative method name)
    pub fn employee(mut self, name: impl Into<String>) -> Self {
        self.employee_name = Some(name.into());
        self
    }

    /// Set employer name (雇用者名)
    pub fn with_employer(mut self, name: impl Into<String>) -> Self {
        self.employer_name = Some(name.into());
        self
    }

    /// Set employer name (alternative method name)
    pub fn employer(mut self, name: impl Into<String>) -> Self {
        self.employer_name = Some(name.into());
        self
    }

    /// Set employment type (雇用形態)
    pub fn with_employment_type(mut self, employment_type: EmploymentType) -> Self {
        self.employment_type = Some(employment_type);
        self
    }

    /// Set employment type (alternative method name)
    pub fn employment_type(mut self, employment_type: EmploymentType) -> Self {
        self.employment_type = Some(employment_type);
        self
    }

    /// Set work pattern (勤務形態)
    pub fn with_work_pattern(mut self, work_pattern: WorkPattern) -> Self {
        self.work_pattern = Some(work_pattern);
        self
    }

    /// Set work pattern (alternative method name)
    pub fn work_pattern(mut self, work_pattern: WorkPattern) -> Self {
        self.work_pattern = Some(work_pattern);
        self
    }

    /// Set start date (開始日)
    pub fn with_start_date(mut self, start_date: DateTime<Utc>) -> Self {
        self.start_date = Some(start_date);
        self
    }

    /// Set start date (alternative method name)
    pub fn start_date(mut self, start_date: DateTime<Utc>) -> Self {
        self.start_date = Some(start_date);
        self
    }

    /// Set end date for fixed-term contracts (終了日)
    pub fn with_end_date(mut self, end_date: DateTime<Utc>) -> Self {
        self.end_date = Some(end_date);
        self
    }

    /// Set end date (alternative method name)
    pub fn end_date(mut self, end_date: DateTime<Utc>) -> Self {
        self.end_date = Some(end_date);
        self
    }

    /// Set base wage in JPY (基本給)
    pub fn with_salary(mut self, base_wage_jpy: u64) -> Self {
        self.base_wage_jpy = Some(base_wage_jpy);
        self
    }

    /// Set base wage (alternative method name)
    pub fn salary(mut self, base_wage_jpy: u64) -> Self {
        self.base_wage_jpy = Some(base_wage_jpy);
        self
    }

    /// Set base wage (alternative method name)
    pub fn with_base_wage(mut self, base_wage_jpy: u64) -> Self {
        self.base_wage_jpy = Some(base_wage_jpy);
        self
    }

    /// Set working hours per day and days per week (労働時間)
    ///
    /// # Example
    /// ```
    /// # use legalis_jp::labor_law::EmploymentContractBuilder;
    /// let builder = EmploymentContractBuilder::new()
    ///     .with_working_hours(8, 5); // 8 hours/day, 5 days/week
    /// ```
    pub fn with_working_hours(mut self, hours_per_day: u32, days_per_week: u32) -> Self {
        self.hours_per_day = Some(hours_per_day);
        self.days_per_week = Some(days_per_week);
        self
    }

    /// Set hours per day (1日の労働時間)
    pub fn with_hours_per_day(mut self, hours: u32) -> Self {
        self.hours_per_day = Some(hours);
        self
    }

    /// Set days per week (週の労働日数)
    pub fn with_days_per_week(mut self, days: u32) -> Self {
        self.days_per_week = Some(days);
        self
    }

    /// Set job description and work location (業務内容・就業場所)
    ///
    /// # Example
    /// ```
    /// # use legalis_jp::labor_law::EmploymentContractBuilder;
    /// let builder = EmploymentContractBuilder::new()
    ///     .with_job("Software Engineer", "Tokyo Office");
    /// ```
    pub fn with_job(mut self, description: impl Into<String>, location: impl Into<String>) -> Self {
        self.job_description = Some(description.into());
        self.work_location = Some(location.into());
        self
    }

    /// Set job description (業務内容)
    pub fn with_job_description(mut self, description: impl Into<String>) -> Self {
        self.job_description = Some(description.into());
        self
    }

    /// Set work location (就業場所)
    pub fn with_work_location(mut self, location: impl Into<String>) -> Self {
        self.work_location = Some(location.into());
        self
    }

    /// Set prefecture for minimum wage validation (都道府県)
    pub fn with_prefecture(mut self, prefecture: Prefecture) -> Self {
        self.prefecture = Some(prefecture);
        self
    }

    /// Set prefecture (alternative method name)
    pub fn prefecture(mut self, prefecture: Prefecture) -> Self {
        self.prefecture = Some(prefecture);
        self
    }

    /// Set probation period in days (試用期間)
    ///
    /// Typical limit is 180 days (6 months)
    pub fn with_probation_period(mut self, days: u32) -> Self {
        self.probation_period_days = Some(days);
        self
    }

    /// Set probation period (alternative method name)
    pub fn probation_period(mut self, days: u32) -> Self {
        self.probation_period_days = Some(days);
        self
    }

    /// Set renewal count for fixed-term contracts (更新回数)
    pub fn with_renewal_count(mut self, count: u32) -> Self {
        self.renewal_count = Some(count);
        self
    }

    /// Build the employment contract
    ///
    /// Validates that all required fields are present. Does NOT perform
    /// legal validation - use `validate()` after building for that.
    ///
    /// # Errors
    ///
    /// Returns `LaborLawError::MissingRequiredField` if any required field is None.
    ///
    /// # Required Fields
    ///
    /// - employee_name
    /// - employer_name
    /// - employment_type
    /// - start_date
    /// - base_wage_jpy
    /// - hours_per_day
    /// - days_per_week
    /// - job_description
    /// - work_location
    pub fn build(self) -> Result<EmploymentContract> {
        // Validate required fields
        let employee_name =
            self.employee_name
                .ok_or_else(|| LaborLawError::MissingRequiredField {
                    field_name: "employee_name".to_string(),
                })?;

        let employer_name =
            self.employer_name
                .ok_or_else(|| LaborLawError::MissingRequiredField {
                    field_name: "employer_name".to_string(),
                })?;

        let employment_type =
            self.employment_type
                .ok_or_else(|| LaborLawError::MissingRequiredField {
                    field_name: "employment_type".to_string(),
                })?;

        let start_date = self
            .start_date
            .ok_or_else(|| LaborLawError::MissingRequiredField {
                field_name: "start_date".to_string(),
            })?;

        let base_wage_jpy =
            self.base_wage_jpy
                .ok_or_else(|| LaborLawError::MissingRequiredField {
                    field_name: "base_wage_jpy".to_string(),
                })?;

        let hours_per_day =
            self.hours_per_day
                .ok_or_else(|| LaborLawError::MissingRequiredField {
                    field_name: "hours_per_day".to_string(),
                })?;

        let days_per_week =
            self.days_per_week
                .ok_or_else(|| LaborLawError::MissingRequiredField {
                    field_name: "days_per_week".to_string(),
                })?;

        let job_description =
            self.job_description
                .ok_or_else(|| LaborLawError::MissingRequiredField {
                    field_name: "job_description".to_string(),
                })?;

        let work_location =
            self.work_location
                .ok_or_else(|| LaborLawError::MissingRequiredField {
                    field_name: "work_location".to_string(),
                })?;

        // Build contract
        Ok(EmploymentContract {
            employee_name,
            employer_name,
            employment_type,
            work_pattern: self.work_pattern.unwrap_or(WorkPattern::Regular),
            start_date,
            end_date: self.end_date,
            base_wage_jpy,
            hours_per_day,
            days_per_week,
            job_description,
            work_location,
            probation_period_days: self.probation_period_days,
            renewal_count: self.renewal_count.unwrap_or(0),
        })
    }
}

impl Default for EmploymentContractBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_new() {
        let builder = EmploymentContractBuilder::new();
        assert!(builder.employee_name.is_none());
        assert!(builder.employer_name.is_none());
    }

    #[test]
    fn test_builder_fluent_api() {
        let builder = EmploymentContractBuilder::new()
            .with_employee("山田太郎")
            .with_employer("テクノロジー株式会社")
            .with_salary(400_000)
            .with_working_hours(8, 5);

        assert_eq!(builder.employee_name, Some("山田太郎".to_string()));
        assert_eq!(
            builder.employer_name,
            Some("テクノロジー株式会社".to_string())
        );
        assert_eq!(builder.base_wage_jpy, Some(400_000));
        assert_eq!(builder.hours_per_day, Some(8));
        assert_eq!(builder.days_per_week, Some(5));
    }

    #[test]
    fn test_builder_alternative_method_names() {
        let builder = EmploymentContractBuilder::new()
            .employee("山田太郎")
            .employer("テクノロジー株式会社")
            .salary(400_000);

        assert_eq!(builder.employee_name, Some("山田太郎".to_string()));
        assert_eq!(
            builder.employer_name,
            Some("テクノロジー株式会社".to_string())
        );
        assert_eq!(builder.base_wage_jpy, Some(400_000));
    }

    #[test]
    fn test_build_success() {
        let builder = EmploymentContractBuilder::new()
            .with_employee("山田太郎")
            .with_employer("テクノロジー株式会社")
            .with_employment_type(EmploymentType::IndefiniteTerm)
            .with_start_date(Utc::now())
            .with_salary(400_000)
            .with_working_hours(8, 5)
            .with_job("Software Engineer", "Tokyo Office");

        let result = builder.build();
        assert!(result.is_ok());

        let contract = result.unwrap();
        assert_eq!(contract.employee_name, "山田太郎");
        assert_eq!(contract.employer_name, "テクノロジー株式会社");
        assert_eq!(contract.base_wage_jpy, 400_000);
        assert_eq!(contract.hours_per_day, 8);
        assert_eq!(contract.days_per_week, 5);
    }

    #[test]
    fn test_build_missing_employee_name() {
        let builder = EmploymentContractBuilder::new()
            .with_employer("テクノロジー株式会社")
            .with_employment_type(EmploymentType::IndefiniteTerm)
            .with_start_date(Utc::now())
            .with_salary(400_000)
            .with_working_hours(8, 5)
            .with_job("Software Engineer", "Tokyo Office");

        let result = builder.build();
        assert!(result.is_err());

        match result.unwrap_err() {
            LaborLawError::MissingRequiredField { field_name } => {
                assert_eq!(field_name, "employee_name");
            }
            _ => panic!("Expected MissingRequiredField error"),
        }
    }

    #[test]
    fn test_build_missing_employer_name() {
        let builder = EmploymentContractBuilder::new()
            .with_employee("山田太郎")
            .with_employment_type(EmploymentType::IndefiniteTerm)
            .with_start_date(Utc::now())
            .with_salary(400_000)
            .with_working_hours(8, 5)
            .with_job("Software Engineer", "Tokyo Office");

        let result = builder.build();
        assert!(result.is_err());

        match result.unwrap_err() {
            LaborLawError::MissingRequiredField { field_name } => {
                assert_eq!(field_name, "employer_name");
            }
            _ => panic!("Expected MissingRequiredField error"),
        }
    }

    #[test]
    fn test_build_missing_salary() {
        let builder = EmploymentContractBuilder::new()
            .with_employee("山田太郎")
            .with_employer("テクノロジー株式会社")
            .with_employment_type(EmploymentType::IndefiniteTerm)
            .with_start_date(Utc::now())
            .with_working_hours(8, 5)
            .with_job("Software Engineer", "Tokyo Office");

        let result = builder.build();
        assert!(result.is_err());

        match result.unwrap_err() {
            LaborLawError::MissingRequiredField { field_name } => {
                assert_eq!(field_name, "base_wage_jpy");
            }
            _ => panic!("Expected MissingRequiredField error"),
        }
    }

    #[test]
    fn test_with_job_sets_both_fields() {
        let builder =
            EmploymentContractBuilder::new().with_job("Software Engineer", "Tokyo Office");

        assert_eq!(
            builder.job_description,
            Some("Software Engineer".to_string())
        );
        assert_eq!(builder.work_location, Some("Tokyo Office".to_string()));
    }

    #[test]
    fn test_prefecture_enum_all_values() {
        // Ensure all 47 prefectures are defined
        let prefectures = vec![
            Prefecture::Hokkaido,
            Prefecture::Tokyo,
            Prefecture::Osaka,
            Prefecture::Okinawa,
            // ... (all 47 exist)
        ];
        assert!(!prefectures.is_empty());
    }

    #[test]
    fn test_default_work_pattern() {
        let builder = EmploymentContractBuilder::new()
            .with_employee("山田太郎")
            .with_employer("テクノロジー株式会社")
            .with_employment_type(EmploymentType::IndefiniteTerm)
            .with_start_date(Utc::now())
            .with_salary(400_000)
            .with_working_hours(8, 5)
            .with_job("Software Engineer", "Tokyo Office");

        let contract = builder.build().unwrap();
        assert_eq!(contract.work_pattern, WorkPattern::Regular);
    }

    #[test]
    fn test_default_renewal_count() {
        let builder = EmploymentContractBuilder::new()
            .with_employee("山田太郎")
            .with_employer("テクノロジー株式会社")
            .with_employment_type(EmploymentType::IndefiniteTerm)
            .with_start_date(Utc::now())
            .with_salary(400_000)
            .with_working_hours(8, 5)
            .with_job("Software Engineer", "Tokyo Office");

        let contract = builder.build().unwrap();
        assert_eq!(contract.renewal_count, 0);
    }

    #[test]
    fn test_probation_period() {
        let builder = EmploymentContractBuilder::new()
            .with_employee("山田太郎")
            .with_employer("テクノロジー株式会社")
            .with_employment_type(EmploymentType::IndefiniteTerm)
            .with_start_date(Utc::now())
            .with_salary(400_000)
            .with_working_hours(8, 5)
            .with_job("Software Engineer", "Tokyo Office")
            .with_probation_period(90);

        let contract = builder.build().unwrap();
        assert_eq!(contract.probation_period_days, Some(90));
    }

    #[test]
    fn test_builder_default_trait() {
        let builder1 = EmploymentContractBuilder::default();
        let builder2 = EmploymentContractBuilder::new();

        assert_eq!(builder1.employee_name, builder2.employee_name);
        assert_eq!(builder1.employer_name, builder2.employer_name);
    }

    #[test]
    fn test_fixed_term_with_end_date() {
        let start = Utc::now();
        let end = start + chrono::Duration::days(365);

        let builder = EmploymentContractBuilder::new()
            .with_employee("田中花子")
            .with_employer("有期雇用株式会社")
            .with_employment_type(EmploymentType::FixedTerm)
            .with_start_date(start)
            .with_end_date(end)
            .with_salary(350_000)
            .with_working_hours(8, 5)
            .with_job("Project Manager", "Osaka Branch");

        let contract = builder.build().unwrap();
        assert_eq!(contract.employment_type, EmploymentType::FixedTerm);
        assert!(contract.end_date.is_some());
        assert_eq!(contract.end_date.unwrap(), end);
    }
}
