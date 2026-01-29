//! Book IV: Commercial Provisions - Civil and Commercial Code B.E. 2535
//!
//! Book IV covers commercial law including:
//! - General commercial provisions (บททั่วไปเกี่ยวกับพาณิชย์)
//! - Commercial registration (ทะเบียนพาณิชย์)
//! - Business names (ชื่อพาณิชย์)
//! - Agency (ตัวแทน)
//! - Commercial sale (การซื้อขายทางพาณิชย์)

use serde::{Deserialize, Serialize};

/// Commercial acts (พาณิชยกรรม) - Section 3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommercialAct {
    /// Buying for resale (ซื้อเพื่อขาย)
    BuyingForResale,

    /// Commission business (รับจ้างซื้อขาย)
    Commission,

    /// Agency (ตัวแทน)
    Agency,

    /// Manufacturing (อุตสาหกรรม)
    Manufacturing,

    /// Transport (ขนส่ง)
    Transport,

    /// Banking (ธนาคาร)
    Banking,

    /// Insurance (ประกันภัย)
    Insurance,

    /// Brokerage (นายหน้า)
    Brokerage,
}

impl CommercialAct {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::BuyingForResale => "ซื้อเพื่อขาย",
            Self::Commission => "รับจ้างซื้อขาย",
            Self::Agency => "ตัวแทน",
            Self::Manufacturing => "อุตสาหกรรม",
            Self::Transport => "ขนส่ง",
            Self::Banking => "ธนาคาร",
            Self::Insurance => "ประกันภัย",
            Self::Brokerage => "นายหน้า",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::BuyingForResale => "Buying for Resale",
            Self::Commission => "Commission Business",
            Self::Agency => "Agency",
            Self::Manufacturing => "Manufacturing",
            Self::Transport => "Transport",
            Self::Banking => "Banking",
            Self::Insurance => "Insurance",
            Self::Brokerage => "Brokerage",
        }
    }
}

/// Commercial registration types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommercialRegistrationType {
    /// Sole proprietorship (ร้านค้าเดี่ยว)
    SoleProprietorship,

    /// Ordinary partnership (ห้างหุ้นส่วนสามัญ)
    OrdinaryPartnership,

    /// Limited partnership (ห้างหุ้นส่วนจำกัด)
    LimitedPartnership,

    /// Branch office (สาขา)
    BranchOffice,
}

impl CommercialRegistrationType {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::SoleProprietorship => "ร้านค้าเดี่ยว",
            Self::OrdinaryPartnership => "ห้างหุ้นส่วนสามัญ",
            Self::LimitedPartnership => "ห้างหุ้นส่วนจำกัด",
            Self::BranchOffice => "สาขา",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::SoleProprietorship => "Sole Proprietorship",
            Self::OrdinaryPartnership => "Ordinary Partnership",
            Self::LimitedPartnership => "Limited Partnership",
            Self::BranchOffice => "Branch Office",
        }
    }
}

/// Agent types (ประเภทตัวแทน)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentType {
    /// General agent (ตัวแทนทั่วไป) - Section 800
    General,

    /// Special agent (ตัวแทนพิเศษ) - Section 800
    Special,

    /// Commission agent (ตัวแทนรับจ้างซื้อขาย) - Section 817
    Commission,

    /// Del credere agent (ตัวแทนค้ำประกัน) - Section 826
    DelCredere,
}

impl AgentType {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::General => "ตัวแทนทั่วไป",
            Self::Special => "ตัวแทนพิเศษ",
            Self::Commission => "ตัวแทนรับจ้างซื้อขาย",
            Self::DelCredere => "ตัวแทนค้ำประกัน",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::General => "General Agent",
            Self::Special => "Special Agent",
            Self::Commission => "Commission Agent",
            Self::DelCredere => "Del Credere Agent",
        }
    }

    /// Get CCC section
    pub fn section(&self) -> u32 {
        match self {
            Self::General | Self::Special => 800,
            Self::Commission => 817,
            Self::DelCredere => 826,
        }
    }
}

/// Commercial sale characteristics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommercialSaleFeature {
    /// Prompt inspection required (ต้องตรวจสอบโดยเร็ว)
    PromptInspection,

    /// Interest on price (ดอกเบี้ยของราคา)
    InterestOnPrice,

    /// Resale right (สิทธิขายทอด)
    ResaleRight,

    /// Retention right (สิทธิหน่วง)
    RetentionRight,
}

impl CommercialSaleFeature {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::PromptInspection => "ต้องตรวจสอบโดยเร็ว",
            Self::InterestOnPrice => "ดอกเบี้ยของราคา",
            Self::ResaleRight => "สิทธิขายทอด",
            Self::RetentionRight => "สิทธิหน่วง",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::PromptInspection => "Prompt Inspection Requirement",
            Self::InterestOnPrice => "Interest on Price",
            Self::ResaleRight => "Right of Resale",
            Self::RetentionRight => "Right of Retention",
        }
    }
}

/// Commercial business entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommercialBusiness {
    /// Business name
    pub name: String,

    /// Registration type
    pub registration_type: CommercialRegistrationType,

    /// Owner/partners
    pub owners: Vec<String>,

    /// Registered address
    pub address: String,

    /// Registration number
    pub registration_number: Option<String>,

    /// Commercial acts performed
    pub commercial_acts: Vec<CommercialAct>,
}

impl CommercialBusiness {
    /// Create a new commercial business
    pub fn new(
        name: String,
        registration_type: CommercialRegistrationType,
        owners: Vec<String>,
        address: String,
    ) -> Self {
        Self {
            name,
            registration_type,
            owners,
            address,
            registration_number: None,
            commercial_acts: Vec::new(),
        }
    }

    /// Check if registered
    pub fn is_registered(&self) -> bool {
        self.registration_number.is_some()
    }

    /// Add commercial act
    pub fn add_commercial_act(&mut self, act: CommercialAct) {
        if !self.commercial_acts.contains(&act) {
            self.commercial_acts.push(act);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commercial_acts() {
        let act = CommercialAct::BuyingForResale;
        assert_eq!(act.name_en(), "Buying for Resale");
    }

    #[test]
    fn test_agent_types() {
        assert_eq!(AgentType::General.section(), 800);
        assert_eq!(AgentType::Commission.section(), 817);
    }

    #[test]
    fn test_commercial_business() {
        let mut business = CommercialBusiness::new(
            "Test Shop".to_string(),
            CommercialRegistrationType::SoleProprietorship,
            vec!["Owner A".to_string()],
            "Bangkok".to_string(),
        );

        assert!(!business.is_registered());
        business.registration_number = Some("REG123".to_string());
        assert!(business.is_registered());

        business.add_commercial_act(CommercialAct::BuyingForResale);
        assert_eq!(business.commercial_acts.len(), 1);
    }
}
