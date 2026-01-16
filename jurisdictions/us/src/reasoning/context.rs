//! EvaluationContext implementations for US legal entities.
//!
//! Bridges US law types to the legalis-core reasoning framework.

use chrono::NaiveDate;
use legalis_core::{DurationUnit, EvaluationContext, RegionType, RelationshipType};

use crate::tax::income_tax::IncomeTaxStructure;
use crate::uniform_acts::ucc::UCCAdoption;

/// Wrapper providing EvaluationContext for US legal entities
pub struct UsEvaluationContext<'a, T> {
    entity: &'a T,
}

impl<'a, T> UsEvaluationContext<'a, T> {
    /// Create a new evaluation context
    #[must_use]
    pub const fn new(entity: &'a T) -> Self {
        Self { entity }
    }
}

// ============================================================================
// EvaluationContext for IncomeTaxStructure (State Tax)
// ============================================================================

impl EvaluationContext for UsEvaluationContext<'_, IncomeTaxStructure> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "state_id" => Some(format!("{:?}", self.entity.state_id)),
            "tax_type" => Some(format!("{:?}", self.entity.tax_type)),
            "has_local_tax" => Some(self.entity.has_local_income_tax.to_string()),
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

    fn check_geographic(&self, region_type: RegionType, region_id: &str) -> bool {
        match region_type {
            RegionType::Country => region_id == "US",
            RegionType::State => format!("{:?}", self.entity.state_id).contains(region_id),
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

    fn get_duration(&self, _unit: DurationUnit) -> Option<u32> {
        None
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "flat_rate" => {
                if let crate::tax::income_tax::IncomeTaxType::Flat { rate } = self.entity.tax_type {
                    Some(rate)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

// ============================================================================
// EvaluationContext for UCCAdoption
// ============================================================================

impl EvaluationContext for UsEvaluationContext<'_, UCCAdoption> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "state" => Some(self.entity.state.clone()),
            "article" => Some(format!("{:?}", self.entity.article)),
            "version" => Some(format!("{:?}", self.entity.version)),
            "adopted" => Some(self.entity.adopted.to_string()),
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

    fn check_geographic(&self, region_type: RegionType, region_id: &str) -> bool {
        match region_type {
            RegionType::Country => region_id == "US",
            RegionType::State => format!("{:?}", self.entity.state).contains(region_id),
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

    fn get_duration(&self, _unit: DurationUnit) -> Option<u32> {
        None
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, _formula: &str) -> Option<f64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tax::income_tax::income_tax_structure;

    #[test]
    fn test_tax_context() {
        let tax = income_tax_structure("CA");
        let ctx = UsEvaluationContext::new(&tax);
        assert!(ctx.get_attribute("tax_type").is_some());
        assert!(ctx.check_geographic(RegionType::Country, "US"));
    }
}
