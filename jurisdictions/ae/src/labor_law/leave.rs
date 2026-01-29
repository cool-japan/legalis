//! Leave entitlements - Articles 29-33

use serde::{Deserialize, Serialize};

/// Leave types and entitlements
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeaveType {
    Annual,
    Sick,
    Maternity,
    Paternity,
    Bereavement { relationship: String },
    Hajj,
    Study,
    NationalService,
}

impl LeaveType {
    pub fn statutory_days(&self) -> u32 {
        match self {
            Self::Annual => 30,
            Self::Sick => 90,
            Self::Maternity => 60,
            Self::Paternity => 5,
            Self::Bereavement { .. } => 3,
            Self::Hajj => 30,
            Self::Study => 10,
            Self::NationalService => 0,
        }
    }
}
