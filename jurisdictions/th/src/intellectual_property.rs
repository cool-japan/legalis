//! Thai Intellectual Property Law
//!
//! Covers:
//! - Patent Act - พ.ร.บ. สิทธิบัตร พ.ศ. 2522
//! - Trademark Act - พ.ร.บ. เครื่องหมายการค้า พ.ศ. 2534
//! - Copyright Act - พ.ร.บ. ลิขสิทธิ์ พ.ศ. 2537

use serde::{Deserialize, Serialize};

/// IP types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IPType {
    /// Patent (สิทธิบัตร)
    Patent,
    /// Petty patent (อนุสิทธิบัตร)
    PettyPatent,
    /// Trademark (เครื่องหมายการค้า)
    Trademark,
    /// Copyright (ลิขสิทธิ์)
    Copyright,
    /// Trade secret (ความลับทางการค้า)
    TradeSecret,
    /// Geographical indication (สิ่งบ่งชี้ทางภูมิศาสตร์)
    GeographicalIndication,
}

impl IPType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Patent => "สิทธิบัตร",
            Self::PettyPatent => "อนุสิทธิบัตร",
            Self::Trademark => "เครื่องหมายการค้า",
            Self::Copyright => "ลิขสิทธิ์",
            Self::TradeSecret => "ความลับทางการค้า",
            Self::GeographicalIndication => "สิ่งบ่งชี้ทางภูมิศาสตร์",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Patent => "Patent",
            Self::PettyPatent => "Petty Patent",
            Self::Trademark => "Trademark",
            Self::Copyright => "Copyright",
            Self::TradeSecret => "Trade Secret",
            Self::GeographicalIndication => "Geographical Indication",
        }
    }

    pub fn protection_period_years(&self) -> u32 {
        match self {
            Self::Patent => 20,
            Self::PettyPatent => 10,
            Self::Trademark => 10, // renewable
            Self::Copyright => 50,
            Self::TradeSecret => 0,             // unlimited if maintained
            Self::GeographicalIndication => 10, // renewable
        }
    }

    pub fn is_renewable(&self) -> bool {
        matches!(self, Self::Trademark | Self::GeographicalIndication)
    }
}

/// Patent types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatentType {
    /// Invention patent
    Invention,
    /// Design patent
    Design,
    /// Petty patent (utility model)
    PettyPatent,
}

impl PatentType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Invention => "สิทธิบัตรการประดิษฐ์",
            Self::Design => "สิทธิบัตรการออกแบบ",
            Self::PettyPatent => "อนุสิทธิบัตร",
        }
    }

    pub fn protection_years(&self) -> u32 {
        match self {
            Self::Invention => 20,
            Self::Design => 10,
            Self::PettyPatent => 10,
        }
    }
}

/// Trademark classes (Nice Classification)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrademarkClass {
    /// Class 1-34: Goods
    Goods,
    /// Class 35-45: Services
    Services,
}

impl TrademarkClass {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Goods => "สินค้า",
            Self::Services => "บริการ",
        }
    }
}

/// Copyright works
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CopyrightWork {
    /// Literary works
    Literary,
    /// Dramatic works
    Dramatic,
    /// Artistic works
    Artistic,
    /// Musical works
    Musical,
    /// Audiovisual works
    Audiovisual,
    /// Cinematographic works
    Cinematographic,
    /// Sound recordings
    SoundRecording,
    /// Broadcasts
    Broadcast,
    /// Computer programs
    ComputerProgram,
}

impl CopyrightWork {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Literary => "งานวรรณกรรม",
            Self::Dramatic => "งานนาฏกรรม",
            Self::Artistic => "งานศิลปกรรม",
            Self::Musical => "งานดนตรีกรรม",
            Self::Audiovisual => "งานโสตทัศน์",
            Self::Cinematographic => "งานภาพยนตร์",
            Self::SoundRecording => "งานบันทึกเสียง",
            Self::Broadcast => "งานแพร่เสียงแพร่ภาพ",
            Self::ComputerProgram => "โปรแกรมคอมพิวเตอร์",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Literary => "Literary Work",
            Self::Dramatic => "Dramatic Work",
            Self::Artistic => "Artistic Work",
            Self::Musical => "Musical Work",
            Self::Audiovisual => "Audiovisual Work",
            Self::Cinematographic => "Cinematographic Work",
            Self::SoundRecording => "Sound Recording",
            Self::Broadcast => "Broadcast",
            Self::ComputerProgram => "Computer Program",
        }
    }

    pub fn protection_period_years(&self) -> u32 {
        match self {
            Self::Literary
            | Self::Dramatic
            | Self::Artistic
            | Self::Musical
            | Self::ComputerProgram => 50, // life + 50 years
            Self::Audiovisual | Self::Cinematographic => 50,
            Self::SoundRecording => 50,
            Self::Broadcast => 25,
        }
    }
}

/// IP infringement types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfringementType {
    /// Direct infringement
    Direct,
    /// Contributory infringement
    Contributory,
    /// Inducement
    Inducement,
    /// Passing off
    PassingOff,
}

impl InfringementType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Direct => "การละเมิดโดยตรง",
            Self::Contributory => "การละเมิดโดยอ้อม",
            Self::Inducement => "การชักจูง",
            Self::PassingOff => "การปลอมแปลง",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Direct => "Direct Infringement",
            Self::Contributory => "Contributory Infringement",
            Self::Inducement => "Inducement",
            Self::PassingOff => "Passing Off",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_types() {
        assert_eq!(IPType::Patent.protection_period_years(), 20);
        assert!(IPType::Trademark.is_renewable());
        assert!(!IPType::Patent.is_renewable());
    }

    #[test]
    fn test_patent_types() {
        assert_eq!(PatentType::Invention.protection_years(), 20);
        assert_eq!(PatentType::Design.protection_years(), 10);
    }

    #[test]
    fn test_copyright_works() {
        assert_eq!(CopyrightWork::Literary.protection_period_years(), 50);
        assert_eq!(CopyrightWork::Broadcast.protection_period_years(), 25);
    }
}
