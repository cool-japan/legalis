//! Criminal Penalties under UAE Penal Code

use crate::common::Aed;
use serde::{Deserialize, Serialize};

/// Types of criminal penalties
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Penalty {
    Death,
    LifeImprisonment,
    TemporaryImprisonment { years: u32 },
    Confinement { months: u32 },
    Fine { amount: Aed },
    BloodMoney { amount: Aed },
    Deportation,
    CommunityService { hours: u32 },
}

impl Penalty {
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Death => "Death Penalty",
            Self::LifeImprisonment => "Life Imprisonment",
            Self::TemporaryImprisonment { .. } => "Temporary Imprisonment",
            Self::Confinement { .. } => "Confinement",
            Self::Fine { .. } => "Fine",
            Self::BloodMoney { .. } => "Blood Money (Diya)",
            Self::Deportation => "Deportation",
            Self::CommunityService { .. } => "Community Service",
        }
    }
}
