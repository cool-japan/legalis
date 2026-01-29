//! Thai Immigration Act - พ.ร.บ. คนเข้าเมือง พ.ศ. 2522
//!
//! Covers visas, work permits, and immigration procedures

use crate::calendar::BuddhistYear;
use crate::citation::ThaiAct;
use serde::{Deserialize, Serialize};

pub fn immigration_act() -> ThaiAct {
    ThaiAct::new("คนเข้าเมือง", "Immigration Act", BuddhistYear::from_be(2522))
}

/// Visa types (ประเภทวีซ่า)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisaType {
    /// Tourist visa (TR) - 60 days
    Tourist,
    /// Non-Immigrant B (business) - 90 days
    NonImmigrantB,
    /// Non-Immigrant O (family) - 90 days
    NonImmigrantO,
    /// Non-Immigrant ED (education) - 90 days
    NonImmigrantED,
    /// SMART Visa
    SMART,
    /// LTR (Long-Term Resident)
    LTR,
    /// Retirement visa (O-A) - 1 year
    Retirement,
    /// Thailand Elite
    ThailandElite,
}

impl VisaType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Tourist => "วีซ่าท่องเที่ยว (TR)",
            Self::NonImmigrantB => "วีซ่าธุรกิจ (Non-B)",
            Self::NonImmigrantO => "วีซ่าครอบครัว (Non-O)",
            Self::NonImmigrantED => "วีซ่าการศึกษา (Non-ED)",
            Self::SMART => "SMART Visa",
            Self::LTR => "Long-Term Resident (LTR)",
            Self::Retirement => "วีซ่าเกษียณอายุ (O-A)",
            Self::ThailandElite => "Thailand Elite",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Tourist => "Tourist Visa (TR)",
            Self::NonImmigrantB => "Non-Immigrant B (Business)",
            Self::NonImmigrantO => "Non-Immigrant O (Family)",
            Self::NonImmigrantED => "Non-Immigrant ED (Education)",
            Self::SMART => "SMART Visa",
            Self::LTR => "Long-Term Resident (LTR)",
            Self::Retirement => "Retirement Visa (O-A)",
            Self::ThailandElite => "Thailand Elite",
        }
    }

    pub fn initial_validity_days(&self) -> u32 {
        match self {
            Self::Tourist => 60,
            Self::NonImmigrantB | Self::NonImmigrantO | Self::NonImmigrantED => 90,
            Self::SMART | Self::Retirement => 365,
            Self::LTR => 1825,           // 5 years
            Self::ThailandElite => 1825, // 5-20 years
        }
    }

    pub fn is_extendable(&self) -> bool {
        !matches!(self, Self::Tourist)
    }
}

/// Work permit categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkPermitCategory {
    /// Category 1: Manager/Executive
    Manager,
    /// Category 2: Specialist/Expert
    Specialist,
    /// Category 3: Skilled worker
    SkilledWorker,
    /// Category 4: Manual worker
    ManualWorker,
}

impl WorkPermitCategory {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Manager => "ผู้บริหาร",
            Self::Specialist => "ผู้เชี่ยวชาญ",
            Self::SkilledWorker => "คนงานฝีมือ",
            Self::ManualWorker => "คนงานทั่วไป",
        }
    }

    pub fn minimum_salary_requirement(&self) -> u32 {
        match self {
            Self::Manager => 50_000,       // 50k THB/month
            Self::Specialist => 50_000,    // 50k THB/month
            Self::SkilledWorker => 15_000, // 15k THB/month
            Self::ManualWorker => 9_000,   // Minimum wage
        }
    }
}

/// Extension of stay types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtensionType {
    /// Business/Work
    Business,
    /// Family/Marriage
    Family,
    /// Retirement
    Retirement,
    /// Education
    Education,
    /// Thai spouse
    ThaiSpouse,
    /// Thai child
    ThaiChild,
}

impl ExtensionType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Business => "ธุรกิจ/ทำงาน",
            Self::Family => "ครอบครัว",
            Self::Retirement => "เกษียณอายุ",
            Self::Education => "การศึกษา",
            Self::ThaiSpouse => "มีคู่สมรสเป็นคนไทย",
            Self::ThaiChild => "มีบุตรเป็นคนไทย",
        }
    }

    pub fn extension_period_days(&self) -> u32 {
        match self {
            Self::Business => 365,
            Self::Family => 365,
            Self::Retirement => 365,
            Self::Education => 365,
            Self::ThaiSpouse => 365,
            Self::ThaiChild => 365,
        }
    }

    pub fn financial_requirement(&self) -> Option<u64> {
        match self {
            Self::Retirement => Some(800_000), // 800k THB in bank
            Self::ThaiSpouse => Some(400_000), // 400k THB or 40k/month
            Self::ThaiChild => Some(400_000),  // 400k THB
            Self::Business | Self::Family | Self::Education => None,
        }
    }
}

/// 90-day reporting requirement
pub const NINETY_DAY_REPORTING_REQUIRED: bool = true;

/// Overstay penalties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverstayPenalty {
    /// Under 90 days: 500 THB/day
    ShortTerm,
    /// 90 days to 1 year: deportation + 1 year ban
    MediumTerm,
    /// 1-3 years: deportation + 3 year ban
    LongTerm,
    /// 3-5 years: deportation + 5 year ban
    VeryLongTerm,
    /// Over 5 years: deportation + 10 year ban
    Extreme,
}

impl OverstayPenalty {
    pub fn from_days(overstay_days: u32) -> Self {
        match overstay_days {
            0..=89 => Self::ShortTerm,
            90..=364 => Self::MediumTerm,
            365..=1094 => Self::LongTerm,
            1095..=1824 => Self::VeryLongTerm,
            _ => Self::Extreme,
        }
    }

    pub fn fine_per_day(&self) -> u32 {
        match self {
            Self::ShortTerm => 500,
            _ => 0, // Deportation instead
        }
    }

    pub fn ban_years(&self) -> u32 {
        match self {
            Self::ShortTerm => 0,
            Self::MediumTerm => 1,
            Self::LongTerm => 3,
            Self::VeryLongTerm => 5,
            Self::Extreme => 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visa_types() {
        assert_eq!(VisaType::Tourist.initial_validity_days(), 60);
        assert_eq!(VisaType::LTR.initial_validity_days(), 1825);
        assert!(!VisaType::Tourist.is_extendable());
        assert!(VisaType::NonImmigrantB.is_extendable());
    }

    #[test]
    fn test_work_permit_salary() {
        assert_eq!(
            WorkPermitCategory::Manager.minimum_salary_requirement(),
            50_000
        );
    }

    #[test]
    fn test_overstay_penalties() {
        assert_eq!(OverstayPenalty::from_days(30), OverstayPenalty::ShortTerm);
        assert_eq!(OverstayPenalty::from_days(100), OverstayPenalty::MediumTerm);
        assert_eq!(OverstayPenalty::ShortTerm.fine_per_day(), 500);
        assert_eq!(OverstayPenalty::MediumTerm.ban_years(), 1);
    }

    #[test]
    fn test_extension_types() {
        assert_eq!(
            ExtensionType::Retirement.financial_requirement(),
            Some(800_000)
        );
    }
}
