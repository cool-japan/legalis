//! Evaluation context implementations for Singapore law.
//!
//! This module provides `EvaluationContext` implementations for Singapore
//! domain types, enabling them to be evaluated against `legalis-core` Statutes.

use chrono::NaiveDate;
use legalis_core::{DurationUnit, EvaluationContext, RegionType, RelationshipType};

use crate::employment::types::{ContractType, EmploymentContract};

/// Singapore-specific evaluation context wrapper
pub struct SingaporeEvaluationContext<'a, T> {
    entity: &'a T,
}

impl<'a, T> SingaporeEvaluationContext<'a, T> {
    /// Create a new Singapore evaluation context
    pub fn new(entity: &'a T) -> Self {
        Self { entity }
    }

    /// Get the wrapped entity
    pub fn entity(&self) -> &T {
        self.entity
    }
}

// Implementation for EmploymentContract
impl EvaluationContext for SingaporeEvaluationContext<'_, EmploymentContract> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "employee_name" => Some(self.entity.employee_name.clone()),
            "employer_name" => Some(self.entity.employer_name.clone()),
            "contract_type" => Some(format!("{:?}", self.entity.contract_type)),
            "cpf_applicable" => Some(self.entity.cpf_applicable.to_string()),
            "covered_by_ea" => Some(self.entity.covered_by_ea.to_string()),
            "is_shift_work" => Some(self.entity.working_hours.is_shift_work.to_string()),
            "overtime_eligible" => Some(self.entity.working_hours.overtime_eligible.to_string()),
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        // Age not directly stored in EmploymentContract
        None
    }

    fn get_income(&self) -> Option<u64> {
        // Return monthly salary in cents
        Some(self.entity.basic_salary_cents)
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn get_current_timestamp(&self) -> Option<i64> {
        Some(chrono::Utc::now().timestamp())
    }

    fn check_geographic(&self, region_type: RegionType, region_id: &str) -> bool {
        // Singapore jurisdiction check
        match region_type {
            RegionType::Country => region_id.to_uppercase() == "SG" || region_id == "Singapore",
            _ => false,
        }
    }

    fn check_relationship(
        &self,
        relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        matches!(relationship_type, RelationshipType::Employment)
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, unit: DurationUnit) -> Option<u32> {
        let years = self.entity.years_of_service();
        match unit {
            DurationUnit::Years => Some(years),
            DurationUnit::Months => Some(years * 12),
            DurationUnit::Weeks => Some(years * 52),
            DurationUnit::Days => Some(years * 365),
        }
    }

    fn get_percentage(&self, context: &str) -> Option<u32> {
        match context {
            "cpf_employer_rate" => {
                // Default to age ≤55 rate (17%)
                Some(17)
            }
            "cpf_employee_rate" => {
                // Default to age ≤55 rate (20%)
                Some(20)
            }
            _ => None,
        }
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "weekly_hours" => Some(self.entity.working_hours.hours_per_week),
            "daily_hours" => Some(self.entity.working_hours.hours_per_day),
            "overtime_hours" => Some(self.entity.working_hours.overtime_hours()),
            "max_weekly_hours" => Some(self.entity.working_hours.max_weekly_hours()),
            "monthly_salary_sgd" => Some(self.entity.basic_salary_sgd()),
            "total_salary_sgd" => Some(self.entity.total_monthly_salary_sgd()),
            "annual_leave_days" => Some(self.entity.leave_entitlement.annual_leave_days as f64),
            _ => None,
        }
    }
}

/// Employment Act coverage thresholds (in cents)
pub const EA_WORKMEN_THRESHOLD_CENTS: u64 = 450_000; // SGD 4,500
pub const EA_NON_WORKMEN_THRESHOLD_CENTS: u64 = 260_000; // SGD 2,600

/// Check if an employment contract is covered by the Employment Act
pub fn is_covered_by_employment_act(contract: &EmploymentContract) -> bool {
    contract.covered_by_ea
}

/// Determine EA coverage based on salary and worker type
pub fn determine_ea_coverage(salary_cents: u64, is_workman: bool) -> bool {
    if is_workman {
        salary_cents <= EA_WORKMEN_THRESHOLD_CENTS
    } else {
        salary_cents <= EA_NON_WORKMEN_THRESHOLD_CENTS
    }
}

/// Contract type helper for EvaluationContext
impl EmploymentContract {
    /// Check if this is an indefinite contract
    pub fn is_indefinite(&self) -> bool {
        matches!(self.contract_type, ContractType::Indefinite)
    }

    /// Check if this is a fixed-term contract
    pub fn is_fixed_term(&self) -> bool {
        matches!(self.contract_type, ContractType::FixedTerm)
    }

    /// Check if this is a part-time contract
    pub fn is_part_time(&self) -> bool {
        matches!(self.contract_type, ContractType::PartTime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::employment::types::{LeaveEntitlement, WorkingHours};
    use chrono::Utc;

    fn sample_contract() -> EmploymentContract {
        EmploymentContract {
            employee_name: "John Tan".to_string(),
            employer_name: "Tech Innovations Pte Ltd".to_string(),
            contract_type: ContractType::Indefinite,
            start_date: Utc::now(),
            end_date: None,
            basic_salary_cents: 500_000, // SGD 5,000
            allowances: vec![],
            working_hours: WorkingHours::standard(),
            leave_entitlement: LeaveEntitlement::new(0),
            cpf_applicable: true,
            covered_by_ea: false, // Above threshold
        }
    }

    #[test]
    fn test_evaluation_context_income() {
        let contract = sample_contract();
        let ctx = SingaporeEvaluationContext::new(&contract);

        assert_eq!(ctx.get_income(), Some(500_000));
    }

    #[test]
    fn test_evaluation_context_attributes() {
        let contract = sample_contract();
        let ctx = SingaporeEvaluationContext::new(&contract);

        assert_eq!(
            ctx.get_attribute("employee_name"),
            Some("John Tan".to_string())
        );
        assert_eq!(
            ctx.get_attribute("cpf_applicable"),
            Some("true".to_string())
        );
    }

    #[test]
    fn test_evaluation_context_formula() {
        let contract = sample_contract();
        let ctx = SingaporeEvaluationContext::new(&contract);

        assert_eq!(ctx.evaluate_formula("weekly_hours"), Some(44.0));
        assert_eq!(ctx.evaluate_formula("monthly_salary_sgd"), Some(5000.0));
    }

    #[test]
    fn test_ea_coverage_thresholds() {
        // Workman at threshold
        assert!(determine_ea_coverage(450_000, true));
        assert!(!determine_ea_coverage(450_001, true));

        // Non-workman at threshold
        assert!(determine_ea_coverage(260_000, false));
        assert!(!determine_ea_coverage(260_001, false));
    }

    #[test]
    fn test_geographic_check() {
        let contract = sample_contract();
        let ctx = SingaporeEvaluationContext::new(&contract);

        assert!(ctx.check_geographic(RegionType::Country, "SG"));
        assert!(ctx.check_geographic(RegionType::Country, "Singapore"));
        assert!(!ctx.check_geographic(RegionType::Country, "MY"));
    }
}
