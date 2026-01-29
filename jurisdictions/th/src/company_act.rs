//! Public Limited Companies Act - พ.ร.บ. บริษัทมหาชนจำกัด พ.ศ. 2535

use crate::calendar::BuddhistYear;
use crate::citation::ThaiAct;
use serde::{Deserialize, Serialize};

pub fn company_act() -> ThaiAct {
    ThaiAct::new(
        "บริษัทมหาชนจำกัด",
        "Public Limited Companies Act",
        BuddhistYear::from_be(2535),
    )
}

/// Company types under Thai law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanyType {
    /// Private limited company (บริษัทจำกัด)
    PrivateLimited,
    /// Public limited company (บริษัทมหาชนจำกัด)
    PublicLimited,
    /// Ordinary partnership (ห้างหุ้นส่วนสามัญ)
    OrdinaryPartnership,
    /// Limited partnership (ห้างหุ้นส่วนจำกัด)
    LimitedPartnership,
    /// Foreign branch (สาขาต่างประเทศ)
    ForeignBranch,
}

impl CompanyType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::PrivateLimited => "บริษัทจำกัด",
            Self::PublicLimited => "บริษัทมหาชนจำกัด",
            Self::OrdinaryPartnership => "ห้างหุ้นส่วนสามัญ",
            Self::LimitedPartnership => "ห้างหุ้นส่วนจำกัด",
            Self::ForeignBranch => "สาขาบริษัทต่างประเทศ",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::PrivateLimited => "Private Limited Company",
            Self::PublicLimited => "Public Limited Company",
            Self::OrdinaryPartnership => "Ordinary Partnership",
            Self::LimitedPartnership => "Limited Partnership",
            Self::ForeignBranch => "Foreign Branch",
        }
    }

    pub fn min_shareholders(&self) -> u32 {
        match self {
            Self::PrivateLimited => 1,
            Self::PublicLimited => 15,
            Self::OrdinaryPartnership => 2,
            Self::LimitedPartnership => 2,
            Self::ForeignBranch => 0,
        }
    }

    pub fn min_capital(&self) -> u64 {
        match self {
            Self::PrivateLimited => 0,
            Self::PublicLimited => 5_000_000,
            Self::OrdinaryPartnership => 0,
            Self::LimitedPartnership => 0,
            Self::ForeignBranch => 0,
        }
    }

    pub fn requires_audit(&self) -> bool {
        matches!(self, Self::PublicLimited | Self::PrivateLimited)
    }
}

/// Director types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectorType {
    /// Executive director (กรรมการผู้จัดการ)
    Executive,
    /// Non-executive director (กรรมการที่ไม่เป็นผู้บริหาร)
    NonExecutive,
    /// Independent director (กรรมการอิสระ)
    Independent,
}

impl DirectorType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Executive => "กรรมการผู้จัดการ",
            Self::NonExecutive => "กรรมการที่ไม่เป็นผู้บริหาร",
            Self::Independent => "กรรมการอิสระ",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Executive => "Executive Director",
            Self::NonExecutive => "Non-Executive Director",
            Self::Independent => "Independent Director",
        }
    }
}

/// Shareholder rights
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareholderRight {
    /// Voting rights (สิทธิออกเสียง)
    Voting,
    /// Dividend rights (สิทธิรับเงินปันผล)
    Dividend,
    /// Information rights (สิทธิเข้าถึงข้อมูล)
    Information,
    /// Preemptive rights (สิทธิจองซื้อหุ้นเพิ่มทุน)
    Preemptive,
}

impl ShareholderRight {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Voting => "สิทธิออกเสียง",
            Self::Dividend => "สิทธิรับเงินปันผล",
            Self::Information => "สิทธิเข้าถึงข้อมูล",
            Self::Preemptive => "สิทธิจองซื้อหุ้นเพิ่มทุน",
        }
    }
}

/// Meeting types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeetingType {
    /// Annual General Meeting (AGM)
    AnnualGeneral,
    /// Extraordinary General Meeting (EGM)
    ExtraordinaryGeneral,
    /// Board meeting
    Board,
}

impl MeetingType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::AnnualGeneral => "ประชุมสามัญผู้ถือหุ้น",
            Self::ExtraordinaryGeneral => "ประชุมวิสามัญผู้ถือหุ้น",
            Self::Board => "ประชุมคณะกรรมการ",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::AnnualGeneral => "Annual General Meeting",
            Self::ExtraordinaryGeneral => "Extraordinary General Meeting",
            Self::Board => "Board Meeting",
        }
    }

    pub fn notice_days_required(&self) -> u32 {
        match self {
            Self::AnnualGeneral => 7,
            Self::ExtraordinaryGeneral => 7,
            Self::Board => 3,
        }
    }
}

/// Capital requirements
pub const MIN_PUBLIC_COMPANY_CAPITAL: u64 = 5_000_000; // 5M THB
pub const MIN_PAID_UP_CAPITAL_PERCENT: u32 = 25; // 25% of registered capital

/// Quorum requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuorumRequirement {
    /// Shareholders meeting: 25 shareholders or 1/2 shares
    ShareholdersMeeting,
    /// Board meeting: 1/2 of directors
    BoardMeeting,
}

impl QuorumRequirement {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::ShareholdersMeeting => "ประชุมผู้ถือหุ้น: 25 คน หรือ 1/2 ของหุ้น",
            Self::BoardMeeting => "ประชุมคณะกรรมการ: 1/2 ของกรรมการ",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_types() {
        assert_eq!(CompanyType::PrivateLimited.min_shareholders(), 1);
        assert_eq!(CompanyType::PublicLimited.min_shareholders(), 15);
        assert_eq!(CompanyType::PublicLimited.min_capital(), 5_000_000);
    }

    #[test]
    fn test_meeting_notice() {
        assert_eq!(MeetingType::AnnualGeneral.notice_days_required(), 7);
        assert_eq!(MeetingType::Board.notice_days_required(), 3);
    }

    #[test]
    fn test_director_types() {
        let director = DirectorType::Executive;
        assert_eq!(director.name_en(), "Executive Director");
    }

    #[test]
    fn test_constants() {
        assert_eq!(MIN_PUBLIC_COMPANY_CAPITAL, 5_000_000);
        assert_eq!(MIN_PAID_UP_CAPITAL_PERCENT, 25);
    }
}
