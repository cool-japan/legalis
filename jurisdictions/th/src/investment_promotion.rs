//! Board of Investment (BOI) - คณะกรรมการส่งเสริมการลงทุน พ.ร.บ. ส่งเสริมการลงทุน พ.ศ. 2520

use crate::calendar::BuddhistYear;
use crate::citation::ThaiAct;
use serde::{Deserialize, Serialize};

pub fn boi_act() -> ThaiAct {
    ThaiAct::new(
        "ส่งเสริมการลงทุน",
        "Investment Promotion Act",
        BuddhistYear::from_be(2520),
    )
}

/// BOI promotion categories (กลุ่มกิจการส่งเสริม)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PromotionCategory {
    /// A1: Activities crucial for Thailand's development
    A1HighPriority,
    /// A2: Technology and innovation driven
    A2Technology,
    /// A3: Competitive enhancement
    A3Competitive,
    /// A4: Basic infrastructure and services
    A4Infrastructure,
    /// B1: Supporting industries
    B1Supporting,
    /// B2: General industries
    B2General,
}

impl PromotionCategory {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::A1HighPriority => "A1: กิจการที่มีความสำคัญสูง",
            Self::A2Technology => "A2: เทคโนโลยีและนวัตกรรม",
            Self::A3Competitive => "A3: เพิ่มขีดความสามารถในการแข่งขัน",
            Self::A4Infrastructure => "A4: โครงสร้างพื้นฐาน",
            Self::B1Supporting => "B1: อุตสาหกรรมสนับสนุน",
            Self::B2General => "B2: อุตสาหกรรมทั่วไป",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::A1HighPriority => "A1: High Priority",
            Self::A2Technology => "A2: Technology & Innovation",
            Self::A3Competitive => "A3: Competitive Enhancement",
            Self::A4Infrastructure => "A4: Infrastructure",
            Self::B1Supporting => "B1: Supporting Industries",
            Self::B2General => "B2: General Industries",
        }
    }

    pub fn cit_exemption_years(&self) -> u32 {
        match self {
            Self::A1HighPriority => 8,
            Self::A2Technology => 8,
            Self::A3Competitive => 5,
            Self::A4Infrastructure => 3,
            Self::B1Supporting | Self::B2General => 0,
        }
    }

    pub fn additional_cit_reduction_years(&self) -> u32 {
        match self {
            Self::A1HighPriority | Self::A2Technology => 5,
            _ => 0,
        }
    }
}

/// Investment zones (เขตส่งเสริมการลงทุน)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvestmentZone {
    /// Zone 1: Bangkok and surroundings
    Zone1Bangkok,
    /// Zone 2: Nearby provinces
    Zone2Nearby,
    /// Zone 3: Remote provinces
    Zone3Remote,
    /// Special zones (EEC, SEZ)
    SpecialZone,
}

impl InvestmentZone {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Zone1Bangkok => "เขต 1: กรุงเทพและปริมณฑล",
            Self::Zone2Nearby => "เขต 2: จังหวัดใกล้เคียง",
            Self::Zone3Remote => "เขต 3: จังหวัดห่างไกล",
            Self::SpecialZone => "เขตพิเศษ (EEC, SEZ)",
        }
    }

    pub fn additional_benefits(&self) -> bool {
        matches!(self, Self::Zone3Remote | Self::SpecialZone)
    }
}

/// BOI incentives (สิทธิประโยชน์)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BOIIncentive {
    /// CIT exemption
    CITExemption,
    /// Import duty exemption on machinery
    MachineryDutyExemption,
    /// Import duty exemption on raw materials
    RawMaterialDutyExemption,
    /// Foreign ownership permission
    ForeignOwnershipPermission,
    /// Land ownership permission
    LandOwnershipPermission,
    /// Work permit facilitation
    WorkPermitFacilitation,
}

impl BOIIncentive {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::CITExemption => "ยกเว้นภาษีเงินได้นิติบุคคล",
            Self::MachineryDutyExemption => "ยกเว้นอากรขาเข้าเครื่องจักร",
            Self::RawMaterialDutyExemption => "ยกเว้นอากรขาเข้าวัตถุดิบ",
            Self::ForeignOwnershipPermission => "อนุญาตให้ชาวต่างชาติถือหุ้นได้",
            Self::LandOwnershipPermission => "อนุญาตให้ถือกรรมสิทธิ์ที่ดิน",
            Self::WorkPermitFacilitation => "อำนวยความสะดวกด้านใบอนุญาตทำงาน",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::CITExemption => "CIT Exemption",
            Self::MachineryDutyExemption => "Machinery Import Duty Exemption",
            Self::RawMaterialDutyExemption => "Raw Material Import Duty Exemption",
            Self::ForeignOwnershipPermission => "Foreign Ownership Permission",
            Self::LandOwnershipPermission => "Land Ownership Permission",
            Self::WorkPermitFacilitation => "Work Permit Facilitation",
        }
    }
}

/// EEC (Eastern Economic Corridor) zones
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EECZone {
    /// Chachoengsao
    Chachoengsao,
    /// Chonburi
    Chonburi,
    /// Rayong
    Rayong,
}

impl EECZone {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Chachoengsao => "ฉะเชิงเทรา",
            Self::Chonburi => "ชลบุรี",
            Self::Rayong => "ระยอง",
        }
    }

    pub fn enhanced_incentives(&self) -> bool {
        true // All EEC zones get enhanced incentives
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_promotion_categories() {
        assert_eq!(PromotionCategory::A1HighPriority.cit_exemption_years(), 8);
        assert_eq!(
            PromotionCategory::A1HighPriority.additional_cit_reduction_years(),
            5
        );
    }

    #[test]
    fn test_investment_zones() {
        assert!(InvestmentZone::Zone3Remote.additional_benefits());
        assert!(!InvestmentZone::Zone1Bangkok.additional_benefits());
    }

    #[test]
    fn test_eec_zones() {
        assert!(EECZone::Chonburi.enhanced_incentives());
    }
}
