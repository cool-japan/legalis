//! EvaluationContext implementations for UK legal entities.
//!
//! Bridges UK employment law types to the legalis-core reasoning framework.

use chrono::NaiveDate;
use legalis_core::{DurationUnit, EvaluationContext, RegionType, RelationshipType};

use crate::employment::types::{EmploymentContract, MinimumWageAssessment, WorkingHours};

/// Wrapper providing EvaluationContext for UK employment contracts
pub struct UkEvaluationContext<'a, T> {
    entity: &'a T,
}

impl<'a, T> UkEvaluationContext<'a, T> {
    /// Create a new evaluation context for an entity
    #[must_use]
    pub const fn new(entity: &'a T) -> Self {
        Self { entity }
    }
}

// ============================================================================
// EvaluationContext for EmploymentContract
// ============================================================================

impl EvaluationContext for UkEvaluationContext<'_, EmploymentContract> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "contract_type" => Some(format!("{:?}", self.entity.contract_type)),
            "employee_name" => Some(self.entity.employee.name.clone()),
            "employer_name" => Some(self.entity.employer.name.clone()),
            "duties" => Some(self.entity.duties.clone()),
            "written_particulars" => Some(self.entity.written_particulars_provided.to_string()),
            "has_pension" => Some(self.entity.pension_scheme.is_some().to_string()),
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        let today = chrono::Utc::now().date_naive();
        Some(self.entity.employee.age_at(today) as u32)
    }

    fn get_income(&self) -> Option<u64> {
        // Return annual salary in pence
        Some((self.entity.salary.gross_annual_gbp * 100.0) as u64)
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn check_geographic(&self, region_type: RegionType, _region_id: &str) -> bool {
        // UK is always true for UK jurisdiction
        matches!(region_type, RegionType::Country)
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
        let today = chrono::Utc::now().date_naive();
        let days = (today - self.entity.start_date).num_days().max(0) as u32;
        match unit {
            DurationUnit::Days => Some(days),
            DurationUnit::Weeks => Some(days / 7),
            DurationUnit::Months => Some(days / 30),
            DurationUnit::Years => Some(days / 365),
        }
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "hours_per_week" => Some(self.entity.working_hours.hours_per_week as f64),
            "days_per_week" => Some(self.entity.working_hours.days_per_week as f64),
            "gross_monthly" => Some(self.entity.salary.gross_monthly()),
            "gross_annual" => Some(self.entity.salary.gross_annual_gbp),
            "hourly_rate" => Some(
                self.entity
                    .salary
                    .gross_hourly(self.entity.working_hours.hours_per_week),
            ),
            _ => None,
        }
    }
}

// ============================================================================
// EvaluationContext for WorkingHours
// ============================================================================

impl EvaluationContext for UkEvaluationContext<'_, WorkingHours> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "hours_per_week" => Some(self.entity.hours_per_week.to_string()),
            "days_per_week" => Some(self.entity.days_per_week.to_string()),
            "opted_out_48h" => Some(self.entity.opted_out_of_48h_limit.to_string()),
            "has_night_work" => Some(self.entity.night_work_hours.is_some().to_string()),
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        None
    }

    fn get_income(&self) -> Option<u64> {
        None
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
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

    fn get_duration(&self, _unit: DurationUnit) -> Option<u32> {
        None
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "hours_per_week" => Some(self.entity.hours_per_week as f64),
            "hours_per_day" => {
                if self.entity.days_per_week > 0 {
                    Some(self.entity.hours_per_week as f64 / self.entity.days_per_week as f64)
                } else {
                    None
                }
            }
            "night_hours" => self.entity.night_work_hours.map(|h| h as f64),
            _ => None,
        }
    }
}

// ============================================================================
// EvaluationContext for MinimumWageAssessment
// ============================================================================

impl EvaluationContext for UkEvaluationContext<'_, MinimumWageAssessment> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "age" => Some(self.entity.age.to_string()),
            "is_apprentice" => Some(self.entity.apprentice.to_string()),
            "is_compliant" => Some(self.entity.is_compliant().to_string()),
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        Some(self.entity.age as u32)
    }

    fn get_income(&self) -> Option<u64> {
        // Hourly rate in pence
        Some((self.entity.hourly_rate_gbp * 100.0) as u64)
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
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

    fn get_duration(&self, _unit: DurationUnit) -> Option<u32> {
        None
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "hourly_rate" => Some(self.entity.hourly_rate_gbp),
            "minimum_wage" => Some(self.entity.applicable_minimum_wage()),
            "shortfall" => {
                let min = self.entity.applicable_minimum_wage();
                if self.entity.hourly_rate_gbp < min {
                    Some(min - self.entity.hourly_rate_gbp)
                } else {
                    Some(0.0)
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employment_contract_context() {
        let contract = EmploymentContract::default();
        let ctx = UkEvaluationContext::new(&contract);
        assert!(ctx.get_attribute("contract_type").is_some());
    }
}
