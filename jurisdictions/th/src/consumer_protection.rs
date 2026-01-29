//! Consumer Protection Act - พ.ร.บ. คุ้มครองผู้บริโภค พ.ศ. 2522

use serde::{Deserialize, Serialize};

/// Consumer rights
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsumerRight {
    /// Right to safety (สิทธิความปลอดภัย)
    Safety,
    /// Right to information (สิทธิได้รับข้อมูล)
    Information,
    /// Right to choose (สิทธิเลือก)
    Choice,
    /// Right to be heard (สิทธิร้องเรียน)
    BeHeard,
    /// Right to redress (สิทธิได้รับการเยียวยา)
    Redress,
}

impl ConsumerRight {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Safety => "สิทธิความปลอดภัย",
            Self::Information => "สิทธิได้รับข้อมูล",
            Self::Choice => "สิทธิเลือก",
            Self::BeHeard => "สิทธิร้องเรียน",
            Self::Redress => "สิทธิได้รับการเยียวยา",
        }
    }
}

/// Unfair contract terms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnfairTerm {
    /// Exemption of liability
    LiabilityExemption,
    /// Unilateral amendment
    UnilateralAmendment,
    /// Unfair penalty
    UnfairPenalty,
    /// Excessive collateral
    ExcessiveCollateral,
}

impl UnfairTerm {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::LiabilityExemption => "การจำกัดความรับผิด",
            Self::UnilateralAmendment => "การแก้ไขสัญญาฝ่ายเดียว",
            Self::UnfairPenalty => "เบี้ยปรับที่ไม่เป็นธรรม",
            Self::ExcessiveCollateral => "หลักประกันเกินควร",
        }
    }
}

/// Product safety standards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductStandard {
    /// Food safety
    FoodSafety,
    /// Electrical safety
    ElectricalSafety,
    /// Toy safety
    ToySafety,
    /// Cosmetic safety
    CosmeticSafety,
}

impl ProductStandard {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::FoodSafety => "มาตรฐานอาหาร",
            Self::ElectricalSafety => "มาตรฐานไฟฟ้า",
            Self::ToySafety => "มาตรฐานของเล่น",
            Self::CosmeticSafety => "มาตรฐานเครื่องสำอาง",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumer_rights() {
        let right = ConsumerRight::Safety;
        assert_eq!(right.name_th(), "สิทธิความปลอดภัย");
    }
}
