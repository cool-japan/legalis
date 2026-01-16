//! EvaluationContext implementations for EU legal entities.
//!
//! Bridges EU law types to the legalis-core reasoning framework.

use chrono::NaiveDate;
use legalis_core::{DurationUnit, EvaluationContext, RegionType, RelationshipType};

use crate::competition::types::{RelevantMarket, Undertaking};
use crate::gdpr::types::DataController;

/// Wrapper providing EvaluationContext for EU legal entities
pub struct EuEvaluationContext<'a, T> {
    entity: &'a T,
}

impl<'a, T> EuEvaluationContext<'a, T> {
    /// Create a new evaluation context
    #[must_use]
    pub const fn new(entity: &'a T) -> Self {
        Self { entity }
    }
}

// ============================================================================
// EvaluationContext for DataController (GDPR)
// ============================================================================

impl EvaluationContext for EuEvaluationContext<'_, DataController> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "id" => Some(self.entity.id.clone()),
            "name" => Some(self.entity.name.clone()),
            "established_in_eu" => Some(self.entity.established_in_eu.to_string()),
            "dpo_appointed" => Some(self.entity.dpo_appointed.to_string()),
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
            RegionType::Country => {
                // Check if established in EU
                if region_id == "EU" {
                    self.entity.established_in_eu
                } else {
                    false
                }
            }
            RegionType::Custom => region_id == "EU" || region_id == "EEA",
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
            "dpo_requirement" => {
                // Controllers must appoint DPO under certain conditions
                Some(if self.entity.dpo_appointed { 1.0 } else { 0.0 })
            }
            _ => None,
        }
    }
}

// ============================================================================
// EvaluationContext for Undertaking (Competition Law)
// ============================================================================

impl EvaluationContext for EuEvaluationContext<'_, Undertaking> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "name" => Some(self.entity.name.clone()),
            "market_share" => self.entity.market_share.map(|s| format!("{:.2}", s)),
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
            RegionType::Custom => region_id == "EU" || region_id == "EEA",
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

    fn get_percentage(&self, context: &str) -> Option<u32> {
        match context {
            "market_share" => self.entity.market_share.map(|s| (s * 100.0).round() as u32),
            _ => None,
        }
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "market_share" => self.entity.market_share,
            "dominance_threshold" => {
                // 40% market share typically indicates dominance
                self.entity
                    .market_share
                    .map(|s| if s > 0.40 { 1.0 } else { 0.0 })
            }
            _ => None,
        }
    }
}

// ============================================================================
// EvaluationContext for RelevantMarket (Competition Law)
// ============================================================================

impl EvaluationContext for EuEvaluationContext<'_, RelevantMarket> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "product_market" => Some(self.entity.product_market.clone()),
            "market_share" => Some(format!("{:.2}", self.entity.market_share)),
            "geographic_market" => Some(format!("{:?}", self.entity.geographic_market)),
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
            RegionType::Custom => region_id == "EU" || region_id == "EEA",
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

    fn get_percentage(&self, context: &str) -> Option<u32> {
        match context {
            "market_share" => Some((self.entity.market_share * 100.0).round() as u32),
            _ => None,
        }
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "market_share" => Some(self.entity.market_share),
            "indicates_dominance" => Some(if self.entity.indicates_dominance() {
                1.0
            } else {
                0.0
            }),
            "is_very_dominant" => Some(if self.entity.is_very_dominant() {
                1.0
            } else {
                0.0
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_controller_context() {
        let controller = DataController {
            id: "DC001".to_string(),
            name: "Acme Corp".to_string(),
            established_in_eu: true,
            dpo_appointed: true,
        };
        let ctx = EuEvaluationContext::new(&controller);

        assert_eq!(ctx.get_attribute("name"), Some("Acme Corp".to_string()));
        assert!(ctx.check_geographic(RegionType::Country, "EU"));
        assert!(ctx.check_geographic(RegionType::Custom, "EEA"));
    }

    #[test]
    fn test_undertaking_context() {
        let undertaking = Undertaking::new("BigTech Inc").with_market_share(0.55);
        let ctx = EuEvaluationContext::new(&undertaking);

        assert_eq!(ctx.get_percentage("market_share"), Some(55));
        assert_eq!(ctx.evaluate_formula("dominance_threshold"), Some(1.0));
    }
}
