//! Employment contract types

use serde::{Deserialize, Serialize};

/// Employment contract types - Article 8
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    LimitedTerm { duration_months: u32 },
    PartTime { hours_per_week: u32 },
    Temporary { duration_days: u32 },
    Flexible,
    Remote,
    JobSharing,
}

impl ContractType {
    pub fn is_valid_duration(&self) -> bool {
        match self {
            Self::LimitedTerm { duration_months } => *duration_months > 0 && *duration_months <= 36,
            Self::PartTime { hours_per_week } => *hours_per_week > 0 && *hours_per_week < 48,
            Self::Temporary { duration_days } => *duration_days > 0 && *duration_days <= 180,
            _ => true,
        }
    }
}
