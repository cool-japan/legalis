//! EvaluationContext implementations for French law types.
//!
//! This module provides implementations of the `EvaluationContext` trait
//! for existing French law types, enabling them to be analyzed by the
//! legal reasoning engine.

use chrono::NaiveDate;
use legalis_core::{DurationUnit, EvaluationContext, RegionType, RelationshipType};

use crate::company::ArticlesOfIncorporation;
use crate::contract::Contract;
use crate::labor::EmploymentContract;

/// Implementation of EvaluationContext for Contract.
///
/// Maps contract fields to context attributes needed for statute evaluation.
impl EvaluationContext for Contract {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "consent_given" => Some(self.consent_given.to_string()),
            "good_faith" => Some(self.good_faith.to_string()),
            "not_under_guardianship" => Some("true".to_string()), // Assume true unless specified
            "content_lawful" => Some(self.validity_defects.is_empty().to_string()),
            "content_certain" => Some(self.contract_type.is_some().to_string()),
            "non_performance" => Some(
                matches!(
                    self.breach,
                    Some(crate::contract::BreachType::NonPerformance)
                )
                .to_string(),
            ),
            "delayed_performance" => Some(
                matches!(
                    self.breach,
                    Some(crate::contract::BreachType::DelayedPerformance)
                )
                .to_string(),
            ),
            "defective_performance" => Some(
                matches!(
                    self.breach,
                    Some(crate::contract::BreachType::DefectivePerformance)
                )
                .to_string(),
            ),
            "force_majeure" => Some("false".to_string()), // Assume no force majeure unless specified
            "harm_suffered" => Some(self.actual_loss.is_some().to_string()),
            "contract_value" => self.contract_value.map(|v| v.to_string()),
            "actual_loss" => self.actual_loss.map(|v| v.to_string()),
            "penalty_clause" => self.penalty_clause.map(|v| v.to_string()),
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        // Contracts don't typically store party ages
        // Assume adult age (18+) unless specified otherwise
        Some(25) // Default to adult age
    }

    fn get_income(&self) -> Option<u64> {
        None
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().naive_utc().date())
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
        // Default to French jurisdiction
        true
    }

    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, unit: DurationUnit) -> Option<u32> {
        match unit {
            DurationUnit::Days => {
                if let (Some(formation), Some(performance)) =
                    (self.formation_date, self.performance_date)
                {
                    let duration = performance.signed_duration_since(formation);
                    Some(duration.num_days() as u32)
                } else {
                    None
                }
            }
            DurationUnit::Weeks => {
                if let (Some(formation), Some(performance)) =
                    (self.formation_date, self.performance_date)
                {
                    let duration = performance.signed_duration_since(formation);
                    Some((duration.num_days() / 7) as u32)
                } else {
                    None
                }
            }
            DurationUnit::Months => {
                if let (Some(formation), Some(performance)) =
                    (self.formation_date, self.performance_date)
                {
                    let duration = performance.signed_duration_since(formation);
                    Some((duration.num_days() / 30) as u32)
                } else {
                    None
                }
            }
            DurationUnit::Years => {
                if let (Some(formation), Some(performance)) =
                    (self.formation_date, self.performance_date)
                {
                    let duration = performance.signed_duration_since(formation);
                    Some((duration.num_days() / 365) as u32)
                } else {
                    None
                }
            }
        }
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        // Simple formula evaluation for damages calculation
        match formula {
            "actual_loss + lost_profit" => {
                let actual = self.actual_loss.unwrap_or(0) as f64;
                // Estimate lost profit as 20% of contract value if not specified
                let lost_profit = self.contract_value.map(|v| v as f64 * 0.2).unwrap_or(0.0);
                Some(actual + lost_profit)
            }
            "penalty_clause" => self.penalty_clause.map(|v| v as f64),
            _ => None,
        }
    }
}

/// Implementation of EvaluationContext for EmploymentContract.
///
/// Maps employment contract fields to context attributes for labor law evaluation.
impl EvaluationContext for EmploymentContract {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "contract_type" => Some(
                match self.contract_type {
                    crate::labor::EmploymentContractType::CDI => "CDI",
                    crate::labor::EmploymentContractType::CDD { .. } => "CDD",
                    crate::labor::EmploymentContractType::Interim { .. } => "Interim",
                    crate::labor::EmploymentContractType::Apprenticeship { .. } => "Apprenticeship",
                }
                .to_string(),
            ),
            "written" => Some(self.written.to_string()),
            "weekly_hours" => Some(self.working_hours.weekly_hours.to_string()),
            "daily_hours" => self.working_hours.daily_hours.map(|h| h.to_string()),
            "hourly_rate" => Some(self.hourly_rate.to_string()),
            "trial_period_months" => self.trial_period_months.map(|d| d.to_string()),
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        // Employment contracts don't typically store employee age
        Some(30) // Default to working age
    }

    fn get_income(&self) -> Option<u64> {
        // Calculate annual income from hourly rate and hours
        let weekly_income = (self.working_hours.weekly_hours * self.hourly_rate) as u64;
        Some(weekly_income * 52) // Annual
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(self.start_date)
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
        true // French jurisdiction
    }

    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, unit: DurationUnit) -> Option<u32> {
        match unit {
            DurationUnit::Months => {
                if let crate::labor::EmploymentContractType::CDD {
                    duration_months, ..
                } = self.contract_type
                {
                    Some(duration_months.into())
                } else {
                    self.trial_period_months.map(|m| m.into())
                }
            }
            DurationUnit::Weeks => None,
            DurationUnit::Days | DurationUnit::Years => None,
        }
    }

    fn get_percentage(&self, context: &str) -> Option<u32> {
        match context {
            "overtime_premium" => {
                if self.working_hours.weekly_hours > 35.0 {
                    Some(25) // First 8 hours: 25%
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "monthly_salary" => {
                Some((self.working_hours.weekly_hours * self.hourly_rate * 52.0 / 12.0) as f64)
            }
            "overtime_hours" => Some((self.working_hours.weekly_hours - 35.0).max(0.0) as f64),
            _ => None,
        }
    }
}

/// Implementation of EvaluationContext for ArticlesOfIncorporation.
///
/// Maps company formation fields to context attributes for company law evaluation.
impl EvaluationContext for ArticlesOfIncorporation {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "company_name" => Some(self.company_name.clone()),
            "company_type" => Some(
                match self.company_type {
                    crate::company::CompanyType::SA => "SA",
                    crate::company::CompanyType::SARL => "SARL",
                    crate::company::CompanyType::SAS => "SAS",
                }
                .to_string(),
            ),
            "capital_eur" => Some(self.capital.amount_eur.to_string()),
            "business_purpose" => self.business_purpose.first().cloned(),
            "head_office" => Some(self.head_office.clone()),
            // Note: board field doesn't exist in ArticlesOfIncorporation
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        // Companies don't have age in this context
        None
    }

    fn get_income(&self) -> Option<u64> {
        None
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        self.incorporation_date
    }

    fn check_geographic(&self, region_type: RegionType, region_id: &str) -> bool {
        match region_type {
            RegionType::Country => region_id == "FR",
            _ => false,
        }
    }

    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, unit: DurationUnit) -> Option<u32> {
        // Duration since incorporation
        if let Some(incorporation) = self.incorporation_date {
            let now = chrono::Utc::now().naive_utc().date();
            let duration = now.signed_duration_since(incorporation);

            match unit {
                DurationUnit::Days => Some(duration.num_days() as u32),
                DurationUnit::Weeks => Some((duration.num_days() / 7) as u32),
                DurationUnit::Months => Some((duration.num_days() / 30) as u32),
                DurationUnit::Years => Some((duration.num_days() / 365) as u32),
            }
        } else {
            None
        }
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "capital_per_share" => {
                let total = self.capital.amount_eur as f64;
                let shares = self.shareholders.iter().map(|s| s.shares).sum::<u64>() as f64;
                if shares > 0.0 {
                    Some(total / shares)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::company::{Capital, CompanyType, Shareholder};
    use crate::contract::BreachType;
    use crate::labor::EmploymentContractType;

    #[test]
    fn test_contract_context_consent() {
        let contract = Contract::new().with_consent(true);
        assert_eq!(
            contract.get_attribute("consent_given"),
            Some("true".to_string())
        );
    }

    #[test]
    fn test_contract_context_breach() {
        let contract = Contract::new().with_breach(BreachType::NonPerformance);
        assert_eq!(
            contract.get_attribute("non_performance"),
            Some("true".to_string())
        );
        assert_eq!(
            contract.get_attribute("delayed_performance"),
            Some("false".to_string())
        );
    }

    #[test]
    fn test_contract_context_age() {
        let contract = Contract::new();
        assert!(contract.get_age().unwrap() >= 18);
    }

    #[test]
    fn test_employment_context_contract_type() {
        let contract = EmploymentContract::new(
            EmploymentContractType::CDI,
            "Employee".to_string(),
            "Employer".to_string(),
        );
        assert_eq!(
            contract.get_attribute("contract_type"),
            Some("CDI".to_string())
        );
    }

    #[test]
    fn test_employment_context_working_hours() {
        let contract = EmploymentContract::new(
            EmploymentContractType::CDI,
            "Employee".to_string(),
            "Employer".to_string(),
        );

        assert_eq!(
            contract.get_attribute("weekly_hours"),
            Some("35".to_string())
        );
        assert_eq!(contract.get_attribute("daily_hours"), None); // Not set by default
    }

    #[test]
    fn test_employment_context_duration() {
        use chrono::Utc;
        let end_date = Utc::now().naive_utc().date() + chrono::Duration::days(365);
        let contract = EmploymentContract::new(
            EmploymentContractType::CDD {
                duration_months: 12,
                reason: crate::labor::CDDReason::ReplacementAbsentEmployee,
                end_date,
            },
            "Employee".to_string(),
            "Employer".to_string(),
        );

        assert_eq!(contract.get_duration(DurationUnit::Months), Some(12));
    }

    #[test]
    fn test_employment_context_formula() {
        let contract = EmploymentContract::new(
            EmploymentContractType::CDI,
            "Employee".to_string(),
            "Employer".to_string(),
        );

        let monthly = contract.evaluate_formula("monthly_salary");
        assert!(monthly.is_some());
        assert!(monthly.unwrap() > 0.0);
    }

    #[test]
    fn test_articles_context_company_name() {
        let capital = Capital::new(37_000);
        let articles =
            ArticlesOfIncorporation::new("Tech Solutions SA".to_string(), CompanyType::SA, capital);

        assert_eq!(
            articles.get_attribute("company_name"),
            Some("Tech Solutions SA".to_string())
        );
        assert_eq!(
            articles.get_attribute("company_type"),
            Some("SA".to_string())
        );
    }

    #[test]
    fn test_articles_context_capital() {
        let capital = Capital::new(50_000);
        let articles =
            ArticlesOfIncorporation::new("Test Company".to_string(), CompanyType::SA, capital);

        assert_eq!(
            articles.get_attribute("capital_eur"),
            Some("50000".to_string())
        );
    }

    #[test]
    fn test_articles_context_formula() {
        let capital = Capital::new(100_000);
        let shareholder = Shareholder::new("Investor A".to_string(), 1000, 100_000);

        let articles =
            ArticlesOfIncorporation::new("Test Company".to_string(), CompanyType::SA, capital)
                .with_shareholder(shareholder);

        let per_share = articles.evaluate_formula("capital_per_share");
        assert_eq!(per_share, Some(100.0)); // 100,000 / 1,000 = 100
    }

    #[test]
    fn test_articles_context_geographic() {
        let capital = Capital::new(37_000);
        let articles =
            ArticlesOfIncorporation::new("Test Company".to_string(), CompanyType::SA, capital);

        assert!(articles.check_geographic(RegionType::Country, "FR"));
        assert!(!articles.check_geographic(RegionType::Country, "US"));
    }
}
