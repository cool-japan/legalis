//! Data processing principles

use serde::{Deserialize, Serialize};

/// Legal basis for processing
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalBasis {
    Consent,
    ContractPerformance,
    LegalObligation,
    VitalInterests,
    PublicTask,
    LegitimateInterests,
}

impl LegalBasis {
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Consent => "Consent",
            Self::ContractPerformance => "Contract Performance",
            Self::LegalObligation => "Legal Obligation",
            Self::VitalInterests => "Vital Interests",
            Self::PublicTask => "Public Task",
            Self::LegitimateInterests => "Legitimate Interests",
        }
    }
}
