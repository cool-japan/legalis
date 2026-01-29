//! Securities and Exchange Act - พ.ร.บ. หลักทรัพย์และตลาดหลักทรัพย์ พ.ศ. 2535

use serde::{Deserialize, Serialize};

/// Securities types (financial instruments)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecuritiesType {
    /// Shares (หุ้น)
    Shares,
    /// Debentures (หุ้นกู้)
    Debentures,
    /// Bonds (พันธบัตร)
    Bonds,
    /// Derivatives (สัญญาซื้อขายล่วงหน้า)
    Derivatives,
    /// Investment units (หน่วยลงทุน)
    InvestmentUnits,
}

impl SecuritiesType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Shares => "หุ้น",
            Self::Debentures => "หุ้นกู้",
            Self::Bonds => "พันธบัตร",
            Self::Derivatives => "สัญญาซื้อขายล่วงหน้า",
            Self::InvestmentUnits => "หน่วยลงทุน",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Shares => "Shares",
            Self::Debentures => "Debentures",
            Self::Bonds => "Bonds",
            Self::Derivatives => "Derivatives",
            Self::InvestmentUnits => "Investment Units",
        }
    }
}

/// Market types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketType {
    /// SET (Stock Exchange of Thailand)
    SET,
    /// mai (Market for Alternative Investment)
    MAI,
    /// TFEX (Thailand Futures Exchange)
    TFEX,
}

impl MarketType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::SET => "ตลาดหลักทรัพย์แห่งประเทศไทย (SET)",
            Self::MAI => "ตลาดหลักทรัพย์ mai",
            Self::TFEX => "ตลาดสัญญาซื้อขายล่วงหน้า (TFEX)",
        }
    }
}

/// Disclosure requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisclosureType {
    /// Initial public offering (IPO)
    IPO,
    /// Annual report (56-1)
    AnnualReport,
    /// Quarterly report (Q1-Q3)
    QuarterlyReport,
    /// Material information
    MaterialInformation,
    /// Insider trading report
    InsiderTrading,
}

impl DisclosureType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::IPO => "การเสนอขายหุ้นต่อประชาชน",
            Self::AnnualReport => "แบบ 56-1 One Report",
            Self::QuarterlyReport => "งบการเงินรายไตรมาส",
            Self::MaterialInformation => "สารสนเทศ",
            Self::InsiderTrading => "รายงานการถือหลักทรัพย์",
        }
    }

    pub fn filing_deadline_days(&self) -> Option<u32> {
        match self {
            Self::IPO => None,
            Self::AnnualReport => Some(90), // Within 90 days after fiscal year end
            Self::QuarterlyReport => Some(45), // Within 45 days after quarter end
            Self::MaterialInformation => Some(1), // Immediately
            Self::InsiderTrading => Some(3), // Within 3 days
        }
    }
}

/// Prohibited acts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProhibitedAct {
    /// Insider trading (การใช้ข้อมูลภายใน)
    InsiderTrading,
    /// Market manipulation (การจัดการราคา)
    MarketManipulation,
    /// False statements (การให้ข้อมูลเท็จ)
    FalseStatements,
    /// Front running (การซื้อขายก่อนลูกค้า)
    FrontRunning,
}

impl ProhibitedAct {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::InsiderTrading => "การใช้ข้อมูลภายใน",
            Self::MarketManipulation => "การจัดการราคา",
            Self::FalseStatements => "การให้ข้อมูลเท็จ",
            Self::FrontRunning => "การซื้อขายก่อนลูกค้า",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_securities_types() {
        assert_eq!(SecuritiesType::Shares.name_en(), "Shares");
    }

    #[test]
    fn test_disclosure_deadlines() {
        assert_eq!(
            DisclosureType::AnnualReport.filing_deadline_days(),
            Some(90)
        );
        assert_eq!(
            DisclosureType::QuarterlyReport.filing_deadline_days(),
            Some(45)
        );
    }
}
