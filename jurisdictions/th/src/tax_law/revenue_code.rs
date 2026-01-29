//! Thai Revenue Code - ประมวลรัษฎากร
//!
//! Covers:
//! - Corporate Income Tax (CIT) - ภาษีเงินได้นิติบุคคล
//! - Personal Income Tax (PIT) - ภาษีเงินได้บุคคลธรรมดา
//! - Value Added Tax (VAT) - ภาษีมูลค่าเพิ่ม
//! - Specific Business Tax (SBT) - ภาษีธุรกิจเฉพาะ
//! - Withholding Tax (WHT) - ภาษีหัก ณ ที่จ่าย

use serde::{Deserialize, Serialize};

/// Tax types under Revenue Code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxType {
    /// Corporate Income Tax - 20% standard rate
    CorporateIncomeTax,

    /// Personal Income Tax - progressive 0-35%
    PersonalIncomeTax,

    /// Value Added Tax - 7% standard rate
    ValueAddedTax,

    /// Specific Business Tax - 3.3% on specific businesses
    SpecificBusinessTax,

    /// Withholding Tax - various rates
    WithholdingTax,

    /// Stamp duty
    StampDuty,
}

impl TaxType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::CorporateIncomeTax => "ภาษีเงินได้นิติบุคคล",
            Self::PersonalIncomeTax => "ภาษีเงินได้บุคคลธรรมดา",
            Self::ValueAddedTax => "ภาษีมูลค่าเพิ่ม",
            Self::SpecificBusinessTax => "ภาษีธุรกิจเฉพาะ",
            Self::WithholdingTax => "ภาษีหัก ณ ที่จ่าย",
            Self::StampDuty => "อากรแสตมป์",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::CorporateIncomeTax => "Corporate Income Tax",
            Self::PersonalIncomeTax => "Personal Income Tax",
            Self::ValueAddedTax => "Value Added Tax",
            Self::SpecificBusinessTax => "Specific Business Tax",
            Self::WithholdingTax => "Withholding Tax",
            Self::StampDuty => "Stamp Duty",
        }
    }
}

/// Corporate Income Tax rates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CITRate {
    /// Standard rate 20%
    Standard,

    /// SME progressive rate
    SMEProgressive,

    /// BOI promoted company 0%
    BOIExempt,
}

impl CITRate {
    pub fn rate_percent(&self) -> f64 {
        match self {
            Self::Standard => 20.0,
            Self::SMEProgressive => 15.0, // Average for SME
            Self::BOIExempt => 0.0,
        }
    }

    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Standard => "อัตราปกติ 20%",
            Self::SMEProgressive => "อัตราก้าวหน้าสำหรับ SME",
            Self::BOIExempt => "ยกเว้นภาษี (BOI)",
        }
    }
}

/// Personal Income Tax brackets (2024)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PITBracket {
    /// 0 - 150,000 THB: 0%
    Bracket0,
    /// 150,001 - 300,000 THB: 5%
    Bracket5,
    /// 300,001 - 500,000 THB: 10%
    Bracket10,
    /// 500,001 - 750,000 THB: 15%
    Bracket15,
    /// 750,001 - 1,000,000 THB: 20%
    Bracket20,
    /// 1,000,001 - 2,000,000 THB: 25%
    Bracket25,
    /// 2,000,001 - 5,000,000 THB: 30%
    Bracket30,
    /// Over 5,000,000 THB: 35%
    Bracket35,
}

impl PITBracket {
    pub fn rate_percent(&self) -> u32 {
        match self {
            Self::Bracket0 => 0,
            Self::Bracket5 => 5,
            Self::Bracket10 => 10,
            Self::Bracket15 => 15,
            Self::Bracket20 => 20,
            Self::Bracket25 => 25,
            Self::Bracket30 => 30,
            Self::Bracket35 => 35,
        }
    }

    pub fn from_income(annual_income: u64) -> Self {
        match annual_income {
            0..=150_000 => Self::Bracket0,
            150_001..=300_000 => Self::Bracket5,
            300_001..=500_000 => Self::Bracket10,
            500_001..=750_000 => Self::Bracket15,
            750_001..=1_000_000 => Self::Bracket20,
            1_000_001..=2_000_000 => Self::Bracket25,
            2_000_001..=5_000_000 => Self::Bracket30,
            _ => Self::Bracket35,
        }
    }
}

/// VAT registration threshold
pub const VAT_REGISTRATION_THRESHOLD: u64 = 1_800_000; // 1.8M THB annual turnover

/// VAT rates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VATRate {
    /// Standard 7%
    Standard,
    /// Zero-rated (export)
    ZeroRated,
    /// Exempt
    Exempt,
}

impl VATRate {
    pub fn rate_percent(&self) -> f64 {
        match self {
            Self::Standard => 7.0,
            Self::ZeroRated => 0.0,
            Self::Exempt => 0.0,
        }
    }
}

/// Tax filing periods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilingPeriod {
    /// Monthly
    Monthly,
    /// Half-yearly
    HalfYearly,
    /// Annual
    Annual,
}

impl FilingPeriod {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Monthly => "รายเดือน",
            Self::HalfYearly => "ราย 6 เดือน",
            Self::Annual => "รายปี",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Monthly => "Monthly",
            Self::HalfYearly => "Half-Yearly",
            Self::Annual => "Annual",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cit_rates() {
        assert_eq!(CITRate::Standard.rate_percent(), 20.0);
        assert_eq!(CITRate::BOIExempt.rate_percent(), 0.0);
    }

    #[test]
    fn test_pit_brackets() {
        assert_eq!(PITBracket::from_income(100_000), PITBracket::Bracket0);
        assert_eq!(PITBracket::from_income(200_000), PITBracket::Bracket5);
        assert_eq!(PITBracket::from_income(6_000_000), PITBracket::Bracket35);
    }

    #[test]
    fn test_vat_rates() {
        assert_eq!(VATRate::Standard.rate_percent(), 7.0);
        assert_eq!(VATRate::ZeroRated.rate_percent(), 0.0);
    }
}
