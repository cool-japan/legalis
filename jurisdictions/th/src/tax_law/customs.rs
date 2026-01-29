//! Thai Customs Act - พระราชบัญญัติศุลกากร
//!
//! Covers import/export duties, customs procedures, and trade facilitation

use serde::{Deserialize, Serialize};

/// Customs duty types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomsDutyType {
    /// Import duty (ศุลกากรนำเข้า)
    Import,

    /// Export duty (ศุลกากรส่งออก)
    Export,

    /// Excise tax (ภาษีสรรพสามิต)
    Excise,
}

impl CustomsDutyType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Import => "ศุลกากรนำเข้า",
            Self::Export => "ศุลกากรส่งออก",
            Self::Excise => "ภาษีสรรพสามิต",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Import => "Import Duty",
            Self::Export => "Export Duty",
            Self::Excise => "Excise Tax",
        }
    }
}

/// Customs valuation methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValuationMethod {
    /// Transaction value (CIF)
    TransactionValue,

    /// Identical goods
    IdenticalGoods,

    /// Similar goods
    SimilarGoods,

    /// Deductive value
    DeductiveValue,

    /// Computed value
    ComputedValue,

    /// Fallback method
    FallbackMethod,
}

impl ValuationMethod {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::TransactionValue => "มูลค่าตามรายการ (CIF)",
            Self::IdenticalGoods => "มูลค่าของสินค้าเหมือน",
            Self::SimilarGoods => "มูลค่าของสินค้าที่คล้ายกัน",
            Self::DeductiveValue => "วิธีหักลบ",
            Self::ComputedValue => "วิธีคำนวณ",
            Self::FallbackMethod => "วิธีสำรอง",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::TransactionValue => "Transaction Value (CIF)",
            Self::IdenticalGoods => "Identical Goods",
            Self::SimilarGoods => "Similar Goods",
            Self::DeductiveValue => "Deductive Value",
            Self::ComputedValue => "Computed Value",
            Self::FallbackMethod => "Fallback Method",
        }
    }
}

/// Customs procedures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomsProcedure {
    /// Regular import
    RegularImport,

    /// Temporary import
    TemporaryImport,

    /// Re-export
    ReExport,

    /// Bonded warehouse
    BondedWarehouse,

    /// Free zone
    FreeZone,
}

impl CustomsProcedure {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::RegularImport => "นำเข้าปกติ",
            Self::TemporaryImport => "นำเข้าชั่วคราว",
            Self::ReExport => "ส่งออกกลับ",
            Self::BondedWarehouse => "คลังสินค้าทัณฑ์บน",
            Self::FreeZone => "เขตปลอดอากร",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::RegularImport => "Regular Import",
            Self::TemporaryImport => "Temporary Import",
            Self::ReExport => "Re-Export",
            Self::BondedWarehouse => "Bonded Warehouse",
            Self::FreeZone => "Free Zone",
        }
    }
}

/// Preferential tariff schemes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreferentialScheme {
    /// ASEAN Trade in Goods Agreement
    ATIGA,

    /// ASEAN-China FTA
    ACFTA,

    /// ASEAN-Japan FTA
    AJCEP,

    /// Thailand-Australia FTA
    TAFTA,

    /// Most Favoured Nation
    MFN,
}

impl PreferentialScheme {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::ATIGA => "ATIGA (ASEAN)",
            Self::ACFTA => "ACFTA (ASEAN-จีน)",
            Self::AJCEP => "AJCEP (ASEAN-ญี่ปุ่น)",
            Self::TAFTA => "TAFTA (ไทย-ออสเตรเลีย)",
            Self::MFN => "MFN",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::ATIGA => "ATIGA",
            Self::ACFTA => "ASEAN-China FTA",
            Self::AJCEP => "ASEAN-Japan FTA",
            Self::TAFTA => "Thailand-Australia FTA",
            Self::MFN => "Most Favoured Nation",
        }
    }

    pub fn typical_reduction_percent(&self) -> u32 {
        match self {
            Self::ATIGA => 0, // 0% for most items
            Self::ACFTA => 0, // 0% for most items
            Self::AJCEP => 0, // 0% for most items
            Self::TAFTA => 0, // 0% for most items
            Self::MFN => 5,   // Varies, average ~5%
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customs_duty_types() {
        let duty = CustomsDutyType::Import;
        assert_eq!(duty.name_en(), "Import Duty");
    }

    #[test]
    fn test_valuation_methods() {
        let method = ValuationMethod::TransactionValue;
        assert_eq!(method.name_en(), "Transaction Value (CIF)");
    }

    #[test]
    fn test_preferential_schemes() {
        assert_eq!(PreferentialScheme::ATIGA.typical_reduction_percent(), 0);
    }
}
