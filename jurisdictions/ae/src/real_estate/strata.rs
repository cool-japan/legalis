//! Strata Law - Federal Law No. 27/2007

use crate::common::Aed;
use serde::{Deserialize, Serialize};

/// Strata property (jointly owned property)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrataProperty {
    pub building_name: String,
    pub unit_number: String,
    pub unit_area_sqm: f64,
    pub common_area_percentage: f64,
    pub monthly_service_charge: Aed,
    pub has_owners_association: bool,
}

impl StrataProperty {
    pub fn annual_service_charge(&self) -> Aed {
        Aed::from_fils(self.monthly_service_charge.fils() * 12)
    }
}
