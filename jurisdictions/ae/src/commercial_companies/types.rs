//! Company types under UAE Commercial Companies Law

use serde::{Deserialize, Serialize};

/// Company types - Article 8
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompanyType {
    Llc,
    Pjsc,
    PrJsc,
    GeneralPartnership,
    LimitedPartnership,
    PartnershipLimitedByShares,
    SoleProprietorship,
    ForeignBranch,
    FreeZoneCompany { zone: String },
}

impl CompanyType {
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Llc => "Limited Liability Company",
            Self::Pjsc => "Public Joint Stock Company",
            Self::PrJsc => "Private Joint Stock Company",
            Self::GeneralPartnership => "General Partnership",
            Self::LimitedPartnership => "Limited Partnership",
            Self::PartnershipLimitedByShares => "Partnership Limited by Shares",
            Self::SoleProprietorship => "Sole Proprietorship",
            Self::ForeignBranch => "Foreign Branch",
            Self::FreeZoneCompany { .. } => "Free Zone Company",
        }
    }

    pub fn has_limited_liability(&self) -> bool {
        matches!(
            self,
            Self::Llc | Self::Pjsc | Self::PrJsc | Self::LimitedPartnership | Self::FreeZoneCompany { .. }
        )
    }
}
