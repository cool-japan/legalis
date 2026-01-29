//! Arbitration Act - พ.ร.บ. อนุญาโตตุลาการ พ.ศ. 2545

use serde::{Deserialize, Serialize};

/// Arbitration types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArbitrationType {
    /// Domestic arbitration (อนุญาโตตุลาการภายใน)
    Domestic,
    /// International arbitration (อนุญาโตตุลาการระหว่างประเทศ)
    International,
}

impl ArbitrationType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Domestic => "อนุญาโตตุลาการภายใน",
            Self::International => "อนุญาโตตุลาการระหว่างประเทศ",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Domestic => "Domestic Arbitration",
            Self::International => "International Arbitration",
        }
    }
}

/// Arbitration institutions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArbitrationInstitution {
    /// THAC (Thailand Arbitration Center)
    THAC,
    /// SIAC (Singapore International Arbitration Centre)
    SIAC,
    /// ICC (International Chamber of Commerce)
    ICC,
    /// Ad-hoc arbitration
    AdHoc,
}

impl ArbitrationInstitution {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::THAC => "ศูนย์อนุญาโตตุลาการ (THAC)",
            Self::SIAC => "SIAC",
            Self::ICC => "ICC",
            Self::AdHoc => "อนุญาโตตุลาการเฉพาะกิจ",
        }
    }
}

/// Grounds for setting aside award
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetAsideGround {
    /// No valid arbitration agreement
    NoArbitrationAgreement,
    /// Procedural irregularity
    ProceduralIrregularity,
    /// Excess of authority
    ExcessOfAuthority,
    /// Public policy violation
    PublicPolicyViolation,
}

impl SetAsideGround {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::NoArbitrationAgreement => "ไม่มีข้อตกลงอนุญาโตตุลาการ",
            Self::ProceduralIrregularity => "ขัดต่อหลักการพิจารณา",
            Self::ExcessOfAuthority => "เกินอำนาจ",
            Self::PublicPolicyViolation => "ขัดต่อความสงบเรียบร้อย",
        }
    }
}

/// New York Convention status
pub const NEW_YORK_CONVENTION_RATIFIED: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arbitration_types() {
        assert_eq!(
            ArbitrationType::International.name_en(),
            "International Arbitration"
        );
    }

    #[test]
    fn test_conventions() {
        const _: () = assert!(NEW_YORK_CONVENTION_RATIFIED);
    }
}
