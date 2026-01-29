//! Book II: Obligations - Civil and Commercial Code B.E. 2535
//!
//! Book II (มาตรา 249-537) covers:
//! - General principles of obligations (หนี้)
//! - Contracts (สัญญา)
//! - Torts (ละเมิด)
//! - Unjust enrichment (การจัดการงานนอกสั่ง)

use crate::calendar::BuddhistYear;
use crate::citation::ThaiAct;
use serde::{Deserialize, Serialize};

/// Sources of obligations (แหล่งที่มาของหนี้)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationSource {
    /// Contract (สัญญา) - Section 349
    Contract,

    /// Tort (ละเมิด) - Section 420
    Tort,

    /// Unjust enrichment (ผลประโยชน์โดยไม่มีกฎหมายกำหนด) - Section 406
    UnjustEnrichment,

    /// Management of affairs without mandate (จัดการงานนอกสั่ง) - Section 388
    QuasiContract,
}

impl ObligationSource {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Contract => "สัญญา",
            Self::Tort => "ละเมิด",
            Self::UnjustEnrichment => "ผลประโยชน์โดยไม่มีกฎหมายกำหนด",
            Self::QuasiContract => "จัดการงานนอกสั่ง",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Contract => "Contract",
            Self::Tort => "Tort",
            Self::UnjustEnrichment => "Unjust Enrichment",
            Self::QuasiContract => "Quasi-Contract",
        }
    }

    /// Get starting section number
    pub fn section(&self) -> u32 {
        match self {
            Self::Contract => 349,
            Self::Tort => 420,
            Self::UnjustEnrichment => 406,
            Self::QuasiContract => 388,
        }
    }
}

/// Contract formation requirements (หลักเกณฑ์การเกิดสัญญา)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractRequirement {
    /// Offer (คำเสนอ) - Section 355
    Offer,

    /// Acceptance (การตอบรับคำเสนอ) - Section 356
    Acceptance,

    /// Consideration (เหตุอันควร) - Section 374
    Consideration,

    /// Legal capacity (ความสามารถในการทำนิติกรรม) - Section 147
    LegalCapacity,

    /// Lawful object (วัตถุอันชอบด้วยกฎหมาย) - Section 150
    LawfulObject,
}

impl ContractRequirement {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Offer => "คำเสนอ",
            Self::Acceptance => "การตอบรับคำเสนอ",
            Self::Consideration => "เหตุอันควร",
            Self::LegalCapacity => "ความสามารถในการทำนิติกรรม",
            Self::LawfulObject => "วัตถุอันชอบด้วยกฎหมาย",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Offer => "Offer",
            Self::Acceptance => "Acceptance",
            Self::Consideration => "Consideration",
            Self::LegalCapacity => "Legal Capacity",
            Self::LawfulObject => "Lawful Object",
        }
    }
}

/// Contract interpretation principles (หลักการตีความสัญญา)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterpretationPrinciple {
    /// Literal meaning (ความหมายตามตัวอักษร) - Section 367
    Literal,

    /// Intention of parties (เจตนาของคู่สัญญา) - Section 368
    Intention,

    /// Good faith (หลักความสุจริต) - Section 5
    GoodFaith,

    /// Custom and usage (จารีตประเพณีและการค้า) - Section 371
    CustomAndUsage,

    /// Contra proferentem (ตีความให้เป็นประโยชน์แก่ลูกหนี้) - Section 369
    ContraProferentem,
}

impl InterpretationPrinciple {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Literal => "ตีความตามตัวอักษร",
            Self::Intention => "ตีความตามเจตนาของคู่สัญญา",
            Self::GoodFaith => "หลักความสุจริต",
            Self::CustomAndUsage => "ตีความตามจารีตประเพณี",
            Self::ContraProferentem => "ตีความให้เป็นประโยชน์แก่ลูกหนี้",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Literal => "Literal Interpretation",
            Self::Intention => "Intention of Parties",
            Self::GoodFaith => "Good Faith Principle",
            Self::CustomAndUsage => "Custom and Usage",
            Self::ContraProferentem => "Contra Proferentem Rule",
        }
    }
}

/// Tort liability types (ประเภทความรับผิดทางละเมิด)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TortLiability {
    /// General tort (ละเมิดทั่วไป) - Section 420
    General,

    /// Vicarious liability (ละเมิดโดยบุคคลในปกครอง) - Section 425
    Vicarious,

    /// Product liability (ละเมิดจากสินค้า) - Section 420
    Product,

    /// Nuisance (การกระทำให้เสื่อมเสีย) - Section 420
    Nuisance,

    /// Defamation (หมิ่นประมาท) - Sections 423-424
    Defamation,
}

impl TortLiability {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::General => "ละเมิดทั่วไป",
            Self::Vicarious => "ละเมิดโดยบุคคลในปกครอง",
            Self::Product => "ละเมิดจากสินค้า",
            Self::Nuisance => "การกระทำให้เสื่อมเสีย",
            Self::Defamation => "หมิ่นประมาท",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::General => "General Tort",
            Self::Vicarious => "Vicarious Liability",
            Self::Product => "Product Liability",
            Self::Nuisance => "Nuisance",
            Self::Defamation => "Defamation",
        }
    }

    /// Get main CCC section
    pub fn section(&self) -> u32 {
        match self {
            Self::General => 420,
            Self::Vicarious => 425,
            Self::Product => 420,
            Self::Nuisance => 420,
            Self::Defamation => 423,
        }
    }
}

/// Remedies for breach of contract (การเยียวยาจากการผิดสัญญา)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractRemedy {
    /// Damages (ค่าเสียหาย) - Section 215
    Damages,

    /// Specific performance (การบังคับจำเพาะ) - Section 212
    SpecificPerformance,

    /// Rescission (การบอกเลิกสัญญา) - Section 386
    Rescission,

    /// Injunction (คำสั่งห้าม) - Court remedies
    Injunction,

    /// Reduction of price (การลดราคา) - Section 386
    PriceReduction,
}

impl ContractRemedy {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Damages => "ค่าเสียหาย",
            Self::SpecificPerformance => "การบังคับให้ปฏิบัติตามสัญญา",
            Self::Rescission => "การบอกเลิกสัญญา",
            Self::Injunction => "คำสั่งห้ามของศาล",
            Self::PriceReduction => "การลดราคา",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Damages => "Damages",
            Self::SpecificPerformance => "Specific Performance",
            Self::Rescission => "Rescission",
            Self::Injunction => "Injunction",
            Self::PriceReduction => "Price Reduction",
        }
    }
}

/// Damages types (ประเภทค่าเสียหาย)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamagesType {
    /// Compensatory damages (ค่าเสียหายตามปกติ) - Section 222
    Compensatory,

    /// Consequential damages (ค่าเสียหายพิเศษ) - Section 222
    Consequential,

    /// Penalty (เบี้ยปรับ) - Section 379
    Penalty,

    /// Nominal damages (ค่าเสียหายเล็กน้อย)
    Nominal,

    /// Liquidated damages (ค่าเสียหายที่ตกลงล่วงหน้า) - Section 379
    Liquidated,
}

impl DamagesType {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Compensatory => "ค่าเสียหายตามปกติ",
            Self::Consequential => "ค่าเสียหายพิเศษ",
            Self::Penalty => "เบี้ยปรับ",
            Self::Nominal => "ค่าเสียหายเล็กน้อย",
            Self::Liquidated => "ค่าเสียหายที่ตกลงล่วงหน้า",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Compensatory => "Compensatory Damages",
            Self::Consequential => "Consequential Damages",
            Self::Penalty => "Penalty",
            Self::Nominal => "Nominal Damages",
            Self::Liquidated => "Liquidated Damages",
        }
    }
}

/// Performance of obligations (การปฏิบัติหนี้)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObligationPerformance {
    /// Is performed in good faith
    pub good_faith: bool,

    /// Performed at the proper time
    pub timely: bool,

    /// Performed at the proper place
    pub proper_place: bool,

    /// Performed by the debtor or authorized person
    pub proper_party: bool,
}

impl ObligationPerformance {
    /// Check if performance is valid under Section 215
    pub fn is_valid(&self) -> bool {
        self.good_faith && self.timely && self.proper_place && self.proper_party
    }

    /// Create standard valid performance
    pub fn valid() -> Self {
        Self {
            good_faith: true,
            timely: true,
            proper_place: true,
            proper_party: true,
        }
    }
}

/// Contract struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    /// Contract name
    pub name: String,

    /// Parties
    pub parties: Vec<String>,

    /// Subject matter
    pub subject_matter: String,

    /// Is in writing
    pub in_writing: bool,

    /// Formation date
    pub formation_date: Option<chrono::NaiveDate>,

    /// Consideration provided
    pub has_consideration: bool,
}

impl Contract {
    /// Create a new contract
    pub fn new(name: String, parties: Vec<String>, subject_matter: String) -> Self {
        Self {
            name,
            parties,
            subject_matter,
            in_writing: false,
            formation_date: None,
            has_consideration: true,
        }
    }

    /// Check if contract is valid
    pub fn is_valid(&self) -> bool {
        !self.parties.is_empty() && self.has_consideration
    }

    /// Get CCC citation
    pub fn ccc_citation(&self) -> ThaiAct {
        ThaiAct::new(
            "ประมวลกฎหมายแพ่งและพาณิชย์",
            "Civil and Commercial Code",
            BuddhistYear::from_be(2535),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obligation_sources() {
        assert_eq!(ObligationSource::Contract.section(), 349);
        assert_eq!(ObligationSource::Tort.section(), 420);
    }

    #[test]
    fn test_tort_liability() {
        assert_eq!(TortLiability::General.section(), 420);
        assert_eq!(TortLiability::Defamation.section(), 423);
    }

    #[test]
    fn test_obligation_performance() {
        let valid = ObligationPerformance::valid();
        assert!(valid.is_valid());

        let invalid = ObligationPerformance {
            good_faith: false,
            timely: true,
            proper_place: true,
            proper_party: true,
        };
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_contract_validity() {
        let contract = Contract::new(
            "Sale Agreement".to_string(),
            vec!["Party A".to_string(), "Party B".to_string()],
            "Sale of goods".to_string(),
        );
        assert!(contract.is_valid());

        let invalid = Contract::new("Invalid".to_string(), vec![], "Nothing".to_string());
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_contract_requirements() {
        let req = ContractRequirement::Offer;
        assert_eq!(req.name_en(), "Offer");
    }

    #[test]
    fn test_damages_types() {
        let damages = DamagesType::Compensatory;
        assert_eq!(damages.name_th(), "ค่าเสียหายตามปกติ");
    }
}
