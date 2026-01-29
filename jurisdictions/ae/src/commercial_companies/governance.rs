//! Corporate governance requirements

use serde::{Deserialize, Serialize};

/// Company governance requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceRequirements {
    pub min_board_members: u32,
    pub max_board_members: u32,
    pub requires_independent_directors: bool,
    pub independent_director_percent: Option<u32>,
    pub requires_audit_committee: bool,
    pub requires_agm: bool,
    pub reporting_period_months: u32,
}

impl GovernanceRequirements {
    pub fn standard() -> Self {
        Self {
            min_board_members: 3,
            max_board_members: 11,
            requires_independent_directors: false,
            independent_director_percent: None,
            requires_audit_committee: false,
            requires_agm: true,
            reporting_period_months: 12,
        }
    }
}
