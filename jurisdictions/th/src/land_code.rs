//! Thai Land Code - ประมวลกฎหมายที่ดิน พ.ศ. 2497
//!
//! Covers land ownership, registration, and restrictions

use crate::calendar::BuddhistYear;
use crate::citation::ThaiAct;
use serde::{Deserialize, Serialize};

pub fn land_code() -> ThaiAct {
    ThaiAct::new("ประมวลกฎหมายที่ดิน", "Land Code", BuddhistYear::from_be(2497))
}

/// Land document types (โฉนดที่ดิน)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandDocument {
    /// Chanote (Nor.Sor. 4) - full title deed
    Chanote,
    /// Nor.Sor. 3 Gor - exploitable land title
    NorSor3Gor,
    /// Nor.Sor. 3 - certificate of use
    NorSor3,
    /// Sor.Kor. 1 - possession claim
    SorKor1,
}

impl LandDocument {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Chanote => "โฉนดที่ดิน (น.ส. 4)",
            Self::NorSor3Gor => "น.ส. 3 ก.",
            Self::NorSor3 => "น.ส. 3",
            Self::SorKor1 => "ส.ค. 1",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Chanote => "Chanote (Title Deed)",
            Self::NorSor3Gor => "Nor Sor 3 Gor",
            Self::NorSor3 => "Nor Sor 3",
            Self::SorKor1 => "Sor Kor 1",
        }
    }

    pub fn is_transferable(&self) -> bool {
        matches!(self, Self::Chanote | Self::NorSor3Gor)
    }

    pub fn can_be_mortgaged(&self) -> bool {
        matches!(self, Self::Chanote | Self::NorSor3Gor)
    }
}

/// Foreign ownership restrictions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForeignOwnershipRule {
    /// Prohibited - cannot own land
    Prohibited,
    /// Condominium - max 49% foreign quota
    Condominium,
    /// BOI promoted - can own land for factory
    BOIPromoted,
    /// Treaty rights - US Treaty of Amity
    TreatyRights,
}

impl ForeignOwnershipRule {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Prohibited => "ห้ามชาวต่างชาติถือกรรมสิทธิ์",
            Self::Condominium => "คอนโดมิเนียม (ไม่เกิน 49%)",
            Self::BOIPromoted => "โครงการ BOI",
            Self::TreatyRights => "สิทธิตามสนธิสัญญา",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Prohibited => "Prohibited",
            Self::Condominium => "Condominium (Max 49%)",
            Self::BOIPromoted => "BOI Promoted",
            Self::TreatyRights => "Treaty Rights",
        }
    }

    pub fn can_own_land(&self) -> bool {
        matches!(self, Self::BOIPromoted | Self::TreatyRights)
    }

    pub fn can_own_condo(&self) -> bool {
        !matches!(self, Self::Prohibited)
    }
}

/// Land rights types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandRight {
    /// Full ownership (กรรมสิทธิ์)
    Ownership,
    /// Usufruct (สิทธิเก็บกิน) - max 30 years
    Usufruct,
    /// Superficies (สิทธิเหนือพื้นดิน) - max 30 years
    Superficies,
    /// Lease (สัญญาเช่า) - max 30 years + 2x renewal
    Lease,
    /// Habitation (สิทธิอยู่อาศัย)
    Habitation,
}

impl LandRight {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Ownership => "กรรมสิทธิ์",
            Self::Usufruct => "สิทธิเก็บกิน",
            Self::Superficies => "สิทธิเหนือพื้นดิน",
            Self::Lease => "สัญญาเช่า",
            Self::Habitation => "สิทธิอยู่อาศัย",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Ownership => "Ownership",
            Self::Usufruct => "Usufruct",
            Self::Superficies => "Superficies",
            Self::Lease => "Lease",
            Self::Habitation => "Habitation",
        }
    }

    pub fn max_duration_years(&self) -> Option<u32> {
        match self {
            Self::Ownership => None,
            Self::Usufruct => Some(30),
            Self::Superficies => Some(30),
            Self::Lease => Some(30),
            Self::Habitation => None, // For life
        }
    }

    pub fn is_renewable(&self) -> bool {
        matches!(self, Self::Lease)
    }

    pub fn requires_registration(&self) -> bool {
        !matches!(self, Self::Habitation)
    }
}

/// Land use zones
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandUseZone {
    /// Agricultural (เกษตรกรรม)
    Agricultural,
    /// Residential (ที่อยู่อาศัย)
    Residential,
    /// Commercial (พาณิชยกรรม)
    Commercial,
    /// Industrial (อุตสาหกรรม)
    Industrial,
    /// Conservation (อนุรักษ์)
    Conservation,
}

impl LandUseZone {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Agricultural => "เกษตรกรรม",
            Self::Residential => "ที่อยู่อาศัย",
            Self::Commercial => "พาณิชยกรรม",
            Self::Industrial => "อุตสาหกรรม",
            Self::Conservation => "อนุรักษ์",
        }
    }
}

/// Condominium foreign quota
pub const CONDO_FOREIGN_QUOTA_PERCENT: u32 = 49;

/// Maximum land lease term
pub const MAX_LEASE_TERM_YEARS: u32 = 30;

/// Maximum lease renewals
pub const MAX_LEASE_RENEWALS: u32 = 2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_land_documents() {
        assert!(LandDocument::Chanote.is_transferable());
        assert!(LandDocument::Chanote.can_be_mortgaged());
        assert!(!LandDocument::SorKor1.is_transferable());
    }

    #[test]
    fn test_foreign_ownership() {
        assert!(!ForeignOwnershipRule::Prohibited.can_own_land());
        assert!(ForeignOwnershipRule::BOIPromoted.can_own_land());
        assert!(ForeignOwnershipRule::Condominium.can_own_condo());
    }

    #[test]
    fn test_land_rights() {
        assert_eq!(LandRight::Ownership.max_duration_years(), None);
        assert_eq!(LandRight::Usufruct.max_duration_years(), Some(30));
        assert!(LandRight::Lease.is_renewable());
        assert!(!LandRight::Ownership.is_renewable());
    }

    #[test]
    fn test_constants() {
        assert_eq!(CONDO_FOREIGN_QUOTA_PERCENT, 49);
        assert_eq!(MAX_LEASE_TERM_YEARS, 30);
    }
}
