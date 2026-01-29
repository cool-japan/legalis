//! Tort Liability under UAE Civil Code

use serde::{Deserialize, Serialize};

/// Tort liability types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TortLiability {
    DirectHarm,
    IndirectHarm,
    JointLiability,
    StrictLiability,
}

impl TortLiability {
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::DirectHarm => "Direct Harm",
            Self::IndirectHarm => "Indirect Harm",
            Self::JointLiability => "Joint and Several Liability",
            Self::StrictLiability => "Strict Liability",
        }
    }
}
