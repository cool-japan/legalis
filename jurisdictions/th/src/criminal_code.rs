//! Thai Criminal Code - ประมวลกฎหมายอาญา พ.ศ. 2499
//!
//! The Criminal Code B.E. 2499 (1956 CE) defines criminal offenses and penalties in Thailand.
//!
//! ## Key Features
//!
//! - Minimum age of criminal responsibility: 10 years (Section 73)
//! - Maximum imprisonment: Life (Section 25)
//! - Death penalty for serious crimes (Sections 91-100)
//! - Statute of limitations: 1-20 years depending on offense

use crate::calendar::BuddhistYear;
use crate::citation::ThaiAct;
use serde::{Deserialize, Serialize};

/// Criminal Code reference
pub fn criminal_code() -> ThaiAct {
    ThaiAct::new(
        "ประมวลกฎหมายอาญา",
        "Criminal Code",
        BuddhistYear::from_be(2499),
    )
}

/// Categories of criminal offenses (ประเภทความผิดอาญา)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OffenseCategory {
    /// Offenses against the King, Queen, Heir (มาตรา 107-112)
    LeseMajeste,

    /// Offenses against national security (มาตรา 113-136)
    NationalSecurity,

    /// Offenses against public peace (มาตรา 209-217)
    PublicPeace,

    /// Offenses against property (มาตรา 334-365)
    Property,

    /// Offenses against life and body (มาตรา 288-310)
    LifeAndBody,

    /// Offenses against liberty and reputation (มาตรา 311-333)
    LibertyAndReputation,

    /// Sexual offenses (มาตรา 276-287)
    Sexual,

    /// Offenses against public administration (มาตรา 137-167)
    PublicAdministration,

    /// Corruption offenses (มาตรา 147-158)
    Corruption,

    /// Drug offenses (พ.ร.บ. ยาเสพติด separate act)
    Drugs,
}

impl OffenseCategory {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::LeseMajeste => "ความผิดต่อพระมหากษัตริย์",
            Self::NationalSecurity => "ความผิดต่อความมั่นคงของรัฐ",
            Self::PublicPeace => "ความผิดต่อสันติราษฎร์",
            Self::Property => "ความผิดเกี่ยวกับทรัพย์",
            Self::LifeAndBody => "ความผิดต่อชีวิตและร่างกาย",
            Self::LibertyAndReputation => "ความผิดต่อเสรีภาพและชื่อเสียง",
            Self::Sexual => "ความผิดเกี่ยวกับเพศ",
            Self::PublicAdministration => "ความผิดต่อการบริหารราชการแผ่นดิน",
            Self::Corruption => "ความผิดเกี่ยวกับการทุจริต",
            Self::Drugs => "ความผิดเกี่ยวกับยาเสพติด",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::LeseMajeste => "Lese-Majeste",
            Self::NationalSecurity => "Offenses Against National Security",
            Self::PublicPeace => "Offenses Against Public Peace",
            Self::Property => "Offenses Against Property",
            Self::LifeAndBody => "Offenses Against Life and Body",
            Self::LibertyAndReputation => "Offenses Against Liberty and Reputation",
            Self::Sexual => "Sexual Offenses",
            Self::PublicAdministration => "Offenses Against Public Administration",
            Self::Corruption => "Corruption Offenses",
            Self::Drugs => "Drug Offenses",
        }
    }
}

/// Types of punishment (โทษ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Punishment {
    /// Death (ประหารชีวิต) - Section 17
    Death,

    /// Life imprisonment (จำคุกตลอดชีวิต) - Section 25
    LifeImprisonment,

    /// Imprisonment (จำคุก) - 1 day to 50 years - Section 21
    Imprisonment,

    /// Confinement (กักขัง) - 1 day to 60 days - Section 18
    Confinement,

    /// Fine (ปรับ) - Section 19
    Fine,

    /// Forfeiture (ริบทรัพย์สิน) - Section 32
    Forfeiture,
}

impl Punishment {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Death => "ประหารชีวิต",
            Self::LifeImprisonment => "จำคุกตลอดชีวิต",
            Self::Imprisonment => "จำคุก",
            Self::Confinement => "กักขัง",
            Self::Fine => "ปรับ",
            Self::Forfeiture => "ริบทรัพย์สิน",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Death => "Death Penalty",
            Self::LifeImprisonment => "Life Imprisonment",
            Self::Imprisonment => "Imprisonment",
            Self::Confinement => "Confinement",
            Self::Fine => "Fine",
            Self::Forfeiture => "Forfeiture",
        }
    }

    /// Get CCC section
    pub fn section(&self) -> u32 {
        match self {
            Self::Death => 17,
            Self::LifeImprisonment => 25,
            Self::Imprisonment => 21,
            Self::Confinement => 18,
            Self::Fine => 19,
            Self::Forfeiture => 32,
        }
    }
}

/// Mitigating circumstances (เหตุลดหย่อนโทษ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MitigatingCircumstance {
    /// Voluntary confession (รับสารภาพ) - Section 78
    Confession,

    /// Provocation (ถูกยั่วยุ) - Section 75
    Provocation,

    /// Mental defect (มีจิตฟั่นเฟือน) - Section 67
    MentalDefect,

    /// Minor participation (มีส่วนร่วมเล็กน้อย)
    MinorParticipation,

    /// First offense (ครั้งแรก)
    FirstOffense,
}

impl MitigatingCircumstance {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Confession => "รับสารภาพ",
            Self::Provocation => "ถูกยั่วยุ",
            Self::MentalDefect => "มีจิตฟั่นเฟือน",
            Self::MinorParticipation => "มีส่วนร่วมเล็กน้อย",
            Self::FirstOffense => "กระทำผิดครั้งแรก",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Confession => "Voluntary Confession",
            Self::Provocation => "Provocation",
            Self::MentalDefect => "Mental Defect",
            Self::MinorParticipation => "Minor Participation",
            Self::FirstOffense => "First Offense",
        }
    }
}

/// Aggravating circumstances (เหตุเพิ่มโทษ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggravatingCircumstance {
    /// Repeat offender (กระทำซ้ำ) - Section 90
    RepeatOffender,

    /// Use of weapon (ใช้อาวุธ)
    UseOfWeapon,

    /// Against vulnerable victim (ต่อผู้เสียเปรียบ)
    VulnerableVictim,

    /// Public servant (เป็นเจ้าหน้าที่)
    PublicServant,

    /// Organized crime (เป็นองค์กร)
    OrganizedCrime,
}

impl AggravatingCircumstance {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::RepeatOffender => "กระทำความผิดซ้ำ",
            Self::UseOfWeapon => "ใช้อาวุธ",
            Self::VulnerableVictim => "กระทำต่อผู้เสียเปรียบ",
            Self::PublicServant => "เป็นเจ้าหน้าที่ของรัฐ",
            Self::OrganizedCrime => "เป็นองค์กรอาชญากรรม",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::RepeatOffender => "Repeat Offender",
            Self::UseOfWeapon => "Use of Weapon",
            Self::VulnerableVictim => "Against Vulnerable Victim",
            Self::PublicServant => "Committed by Public Servant",
            Self::OrganizedCrime => "Organized Crime",
        }
    }
}

/// Criminal responsibility (ความรับผิดทางอาญา)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CriminalResponsibility {
    /// Full responsibility (รับผิดเต็มที่)
    Full,

    /// Diminished responsibility (ลดหย่อนความรับผิด)
    Diminished,

    /// No responsibility (ไม่มีความรับผิด)
    None,
}

impl CriminalResponsibility {
    /// Determine responsibility based on age
    pub fn from_age(age: u32) -> Self {
        if age >= 18 {
            Self::Full
        } else if age >= 10 {
            Self::Diminished
        } else {
            Self::None
        }
    }

    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Full => "มีความรับผิดทางอาญาเต็มที่",
            Self::Diminished => "มีความรับผิดทางอาญาแต่ลดหย่อน",
            Self::None => "ไม่มีความรับผิดทางอาญา",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Full => "Full Criminal Responsibility",
            Self::Diminished => "Diminished Criminal Responsibility",
            Self::None => "No Criminal Responsibility",
        }
    }
}

/// Statute of limitations (อายุความ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatuteOfLimitations {
    /// 20 years for death penalty offenses
    TwentyYears,

    /// 15 years for life imprisonment offenses
    FifteenYears,

    /// 10 years for max imprisonment offenses
    TenYears,

    /// 5 years for other offenses
    FiveYears,

    /// 1 year for petty offenses
    OneYear,
}

impl StatuteOfLimitations {
    /// Get period in years
    pub fn years(&self) -> u32 {
        match self {
            Self::TwentyYears => 20,
            Self::FifteenYears => 15,
            Self::TenYears => 10,
            Self::FiveYears => 5,
            Self::OneYear => 1,
        }
    }

    /// Determine limitation period based on maximum punishment
    pub fn from_punishment(punishment: Punishment) -> Self {
        match punishment {
            Punishment::Death => Self::TwentyYears,
            Punishment::LifeImprisonment => Self::FifteenYears,
            Punishment::Imprisonment => Self::TenYears,
            Punishment::Confinement => Self::FiveYears,
            Punishment::Fine => Self::OneYear,
            Punishment::Forfeiture => Self::FiveYears,
        }
    }

    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::TwentyYears => "อายุความ 20 ปี",
            Self::FifteenYears => "อายุความ 15 ปี",
            Self::TenYears => "อายุความ 10 ปี",
            Self::FiveYears => "อายุความ 5 ปี",
            Self::OneYear => "อายุความ 1 ปี",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::TwentyYears => "20-Year Statute of Limitations",
            Self::FifteenYears => "15-Year Statute of Limitations",
            Self::TenYears => "10-Year Statute of Limitations",
            Self::FiveYears => "5-Year Statute of Limitations",
            Self::OneYear => "1-Year Statute of Limitations",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offense_categories() {
        let category = OffenseCategory::Property;
        assert_eq!(category.name_en(), "Offenses Against Property");
    }

    #[test]
    fn test_punishments() {
        assert_eq!(Punishment::Death.section(), 17);
        assert_eq!(Punishment::Imprisonment.section(), 21);
    }

    #[test]
    fn test_criminal_responsibility_age() {
        assert_eq!(
            CriminalResponsibility::from_age(25),
            CriminalResponsibility::Full
        );
        assert_eq!(
            CriminalResponsibility::from_age(15),
            CriminalResponsibility::Diminished
        );
        assert_eq!(
            CriminalResponsibility::from_age(8),
            CriminalResponsibility::None
        );
    }

    #[test]
    fn test_statute_of_limitations() {
        assert_eq!(
            StatuteOfLimitations::from_punishment(Punishment::Death),
            StatuteOfLimitations::TwentyYears
        );
        assert_eq!(
            StatuteOfLimitations::from_punishment(Punishment::Imprisonment),
            StatuteOfLimitations::TenYears
        );
    }

    #[test]
    fn test_limitation_years() {
        assert_eq!(StatuteOfLimitations::TwentyYears.years(), 20);
        assert_eq!(StatuteOfLimitations::OneYear.years(), 1);
    }
}
