//! Anti-Corruption Law Types (ປະເພດກົດໝາຍຕ້ານການສໍ້ລາດບັງຫຼວງ)
//!
//! Type definitions for Lao anti-corruption law based on:
//! - **Anti-Corruption Law 2012** (Law No. 03/NA, amended 2019)
//! - National Assembly of the Lao PDR
//!
//! # Legal References
//!
//! - Anti-Corruption Law 2012 (Law No. 03/NA) - ກົດໝາຍວ່າດ້ວຍການຕ້ານການສໍ້ລາດບັງຫຼວງ ປີ 2012
//! - Amendment 2019 - ການປັບປຸງ ປີ 2019
//! - UNCAC (UN Convention Against Corruption) - ສົນທິສັນຍາສະຫະປະຊາຊາດຕ້ານການສໍ້ລາດບັງຫຼວງ
//!
//! # Bilingual Support
//! All types include both Lao (ລາວ) and English field names where applicable.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Anti-Corruption Law 2012 (ກົດໝາຍຕ້ານການສໍ້ລາດບັງຫຼວງ ປີ 2012)
// ============================================================================

/// Minor corruption threshold (Article 65) - LAK
/// ເກນການສໍ້ລາດບັງຫຼວງຂະໜາດນ້ອຍ
pub const MINOR_CORRUPTION_THRESHOLD_LAK: u64 = 5_000_000;

/// Medium corruption threshold (Article 66) - LAK
/// ເກນການສໍ້ລາດບັງຫຼວງຂະໜາດກາງ
pub const MEDIUM_CORRUPTION_THRESHOLD_LAK: u64 = 50_000_000;

/// Serious corruption threshold (Article 67) - LAK
/// ເກນການສໍ້ລາດບັງຫຼວງຂະໜາດຮ້າຍແຮງ
pub const SERIOUS_CORRUPTION_THRESHOLD_LAK: u64 = 500_000_000;

/// Very serious corruption threshold (Article 68) - LAK
/// ເກນການສໍ້ລາດບັງຫຼວງຂະໜາດຮ້າຍແຮງຫຼາຍ
pub const VERY_SERIOUS_CORRUPTION_THRESHOLD_LAK: u64 = 500_000_001;

/// Gift limit for official functions (Article 108) - LAK
/// ກຳນົດຂອງຂວັນສຳລັບວຽກທາງການ
pub const GIFT_LIMIT_OFFICIAL_FUNCTION_LAK: u64 = 500_000;

/// Annual declaration deadline month (Article 52) - March
/// ເດືອນກຳນົດຍື່ນປະກາດປະຈຳປີ - ເດືອນ 3
pub const ANNUAL_DECLARATION_DEADLINE_MONTH: u8 = 3;

/// Cooling-off period in years (Article 112)
/// ໄລຍະຫ່າງເປັນປີ
pub const COOLING_OFF_PERIOD_YEARS: u8 = 2;

/// Preliminary investigation deadline in days (Article 12)
/// ກຳນົດເວລາສືບສວນເບື້ອງຕົ້ນເປັນວັນ
pub const INVESTIGATION_PRELIMINARY_DAYS: u32 = 90;

/// Full investigation deadline in days (Article 13)
/// ກຳນົດເວລາສືບສວນເຕັມຮູບແບບເປັນວັນ
pub const INVESTIGATION_FULL_DAYS: u32 = 180;

/// Prosecution referral deadline in days (Article 15)
/// ກຳນົດເວລາສົ່ງຟ້ອງເປັນວັນ
pub const PROSECUTION_REFERRAL_DAYS: u32 = 30;

/// Minimum whistleblower reward percentage (Article 90)
/// ອັດຕາລາງວັນຜູ້ແຈ້ງຂ່າວຕ່ຳສຸດ (%)
pub const WHISTLEBLOWER_REWARD_MIN_PERCENT: u8 = 5;

/// Maximum whistleblower reward percentage (Article 90)
/// ອັດຕາລາງວັນຜູ້ແຈ້ງຂ່າວສູງສຸດ (%)
pub const WHISTLEBLOWER_REWARD_MAX_PERCENT: u8 = 15;

// ============================================================================
// State Inspection Authority (SIA) - ອົງການກວດກາແຫ່ງລັດ (ອກລ)
// ============================================================================

/// SIA office level - ລະດັບຫ້ອງການ ອກລ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SIAOfficeLevel {
    /// Central office (ຫ້ອງການສູນກາງ)
    Central,
    /// Provincial office (ຫ້ອງການແຂວງ)
    Provincial,
    /// District office (ຫ້ອງການເມືອງ)
    District,
}

impl SIAOfficeLevel {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            SIAOfficeLevel::Central => "ຫ້ອງການສູນກາງ ອກລ",
            SIAOfficeLevel::Provincial => "ຫ້ອງການ ອກລ ແຂວງ",
            SIAOfficeLevel::District => "ຫ້ອງການ ອກລ ເມືອງ",
        }
    }

    /// Get the English name
    pub fn english_name(&self) -> &'static str {
        match self {
            SIAOfficeLevel::Central => "Central SIA Office",
            SIAOfficeLevel::Provincial => "Provincial SIA Office",
            SIAOfficeLevel::District => "District SIA Office",
        }
    }

    /// Get hierarchy level (0 = highest)
    pub fn hierarchy_level(&self) -> u8 {
        match self {
            SIAOfficeLevel::Central => 0,
            SIAOfficeLevel::Provincial => 1,
            SIAOfficeLevel::District => 2,
        }
    }
}

/// SIA office - ຫ້ອງການ ອກລ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SIAOffice {
    /// Office level (ລະດັບຫ້ອງການ)
    pub level: SIAOfficeLevel,
    /// Office name in Lao (ຊື່ຫ້ອງການເປັນພາສາລາວ)
    pub name_lao: String,
    /// Office name in English (ຊື່ຫ້ອງການເປັນພາສາອັງກິດ)
    pub name_en: String,
    /// Province if applicable (ແຂວງຖ້າມີ)
    pub province: Option<String>,
    /// District if applicable (ເມືອງຖ້າມີ)
    pub district: Option<String>,
}

/// SIA powers - ອຳນາດ ອກລ (Article 10)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SIAPower {
    /// Inspection power (ອຳນາດກວດກາ)
    Inspection,
    /// Investigation power (ອຳນາດສືບສວນ)
    Investigation,
    /// Document request power (ອຳນາດຮຽກຮ້ອງເອກະສານ)
    DocumentRequest,
    /// Witness summons power (ອຳນາດເອີ້ນພະຍານ)
    WitnessSummons,
    /// Asset freeze power (ອຳນາດອາຍັດຊັບສິນ)
    AssetFreeze,
    /// Arrest referral power (ອຳນາດສົ່ງຈັບກຸມ)
    ArrestReferral,
    /// Prosecution referral power (ອຳນາດສົ່ງຟ້ອງ)
    ProsecutionReferral,
}

impl SIAPower {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            SIAPower::Inspection => "ອຳນາດກວດກາ",
            SIAPower::Investigation => "ອຳນາດສືບສວນ",
            SIAPower::DocumentRequest => "ອຳນາດຮຽກຮ້ອງເອກະສານ",
            SIAPower::WitnessSummons => "ອຳນາດເອີ້ນພະຍານ",
            SIAPower::AssetFreeze => "ອຳນາດອາຍັດຊັບສິນ",
            SIAPower::ArrestReferral => "ອຳນາດສົ່ງຈັບກຸມ",
            SIAPower::ProsecutionReferral => "ອຳນາດສົ່ງຟ້ອງ",
        }
    }
}

// ============================================================================
// Public Officials - ພະນັກງານລັດ
// ============================================================================

/// Position grade for public officials - ລະດັບຕຳແໜ່ງ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PositionGrade {
    /// Grade 1 - Minister/Vice Minister level (ລະດັບລັດຖະມົນຕີ/ຮອງລັດຖະມົນຕີ)
    Grade1,
    /// Grade 2 - Director General level (ລະດັບຫົວໜ້າກົມ)
    Grade2,
    /// Grade 3 - Deputy Director General level (ລະດັບຮອງຫົວໜ້າກົມ)
    Grade3,
    /// Grade 4 - Division Chief level (ລະດັບຫົວໜ້າພະແນກ)
    Grade4,
    /// Grade 5 - Deputy Division Chief level (ລະດັບຮອງຫົວໜ້າພະແນກ)
    Grade5,
    /// Grade 6 - Senior Officer level (ລະດັບພະນັກງານອາວຸໂສ)
    Grade6,
    /// Grade 7 - Junior Officer level (ລະດັບພະນັກງານຊັ້ນຕ່ຳ)
    Grade7,
    /// Below Grade 7 (ຕ່ຳກວ່າລະດັບ 7)
    BelowGrade7,
}

impl PositionGrade {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            PositionGrade::Grade1 => "ລະດັບ 1 - ລັດຖະມົນຕີ/ຮອງລັດຖະມົນຕີ",
            PositionGrade::Grade2 => "ລະດັບ 2 - ຫົວໜ້າກົມ",
            PositionGrade::Grade3 => "ລະດັບ 3 - ຮອງຫົວໜ້າກົມ",
            PositionGrade::Grade4 => "ລະດັບ 4 - ຫົວໜ້າພະແນກ",
            PositionGrade::Grade5 => "ລະດັບ 5 - ຮອງຫົວໜ້າພະແນກ",
            PositionGrade::Grade6 => "ລະດັບ 6 - ພະນັກງານອາວຸໂສ",
            PositionGrade::Grade7 => "ລະດັບ 7 - ພະນັກງານ",
            PositionGrade::BelowGrade7 => "ຕ່ຳກວ່າລະດັບ 7",
        }
    }

    /// Check if asset declaration is required (Article 50)
    /// ກວດວ່າຕ້ອງປະກາດຊັບສິນຫຼືບໍ່
    pub fn requires_asset_declaration(&self) -> bool {
        matches!(
            self,
            PositionGrade::Grade1
                | PositionGrade::Grade2
                | PositionGrade::Grade3
                | PositionGrade::Grade4
                | PositionGrade::Grade5
        )
    }

    /// Get the numeric grade value
    pub fn grade_number(&self) -> u8 {
        match self {
            PositionGrade::Grade1 => 1,
            PositionGrade::Grade2 => 2,
            PositionGrade::Grade3 => 3,
            PositionGrade::Grade4 => 4,
            PositionGrade::Grade5 => 5,
            PositionGrade::Grade6 => 6,
            PositionGrade::Grade7 => 7,
            PositionGrade::BelowGrade7 => 8,
        }
    }
}

/// Official category - ປະເພດພະນັກງານ (Article 5)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OfficialCategory {
    /// Government officials (ພະນັກງານລັດຖະບານ)
    Government,
    /// Elected officials (ພະນັກງານຖືກເລືອກຕັ້ງ)
    Elected,
    /// Military personnel (ພະນັກງານທະຫານ)
    Military,
    /// Police personnel (ພະນັກງານຕຳຫຼວດ)
    Police,
    /// State enterprise employees (ພະນັກງານວິສາຫະກິດລັດ)
    StateEnterprise,
    /// Judges (ຜູ້ພິພາກສາ)
    Judge,
    /// Prosecutors (ໄອຍະການ)
    Prosecutor,
}

impl OfficialCategory {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            OfficialCategory::Government => "ພະນັກງານລັດຖະບານ",
            OfficialCategory::Elected => "ພະນັກງານຖືກເລືອກຕັ້ງ",
            OfficialCategory::Military => "ພະນັກງານທະຫານ",
            OfficialCategory::Police => "ພະນັກງານຕຳຫຼວດ",
            OfficialCategory::StateEnterprise => "ພະນັກງານວິສາຫະກິດລັດ",
            OfficialCategory::Judge => "ຜູ້ພິພາກສາ",
            OfficialCategory::Prosecutor => "ໄອຍະການ",
        }
    }

    /// Check if this category is covered by the Anti-Corruption Law
    pub fn is_covered(&self) -> bool {
        true // All categories are covered by the law
    }
}

/// Official type - ປະເພດພະນັກງານ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OfficialType {
    /// Government official (ພະນັກງານລັດຖະບານ)
    GovernmentOfficial {
        position_grade: PositionGrade,
        ministry: Option<String>,
    },
    /// Elected official (ພະນັກງານຖືກເລືອກຕັ້ງ)
    ElectedOfficial { position: String, level: String },
    /// Military personnel (ທະຫານ)
    MilitaryPersonnel { rank: String, unit: Option<String> },
    /// Police personnel (ຕຳຫຼວດ)
    PolicePersonnel { rank: String, unit: Option<String> },
    /// State enterprise employee (ພະນັກງານວິສາຫະກິດລັດ)
    StateEnterpriseEmployee {
        position: String,
        enterprise_name: String,
    },
    /// Judge (ຜູ້ພິພາກສາ)
    Judge {
        court_level: String,
        position: String,
    },
    /// Prosecutor (ໄອຍະການ)
    Prosecutor {
        office_level: String,
        position: String,
    },
}

impl OfficialType {
    /// Get the category for this official type
    pub fn category(&self) -> OfficialCategory {
        match self {
            OfficialType::GovernmentOfficial { .. } => OfficialCategory::Government,
            OfficialType::ElectedOfficial { .. } => OfficialCategory::Elected,
            OfficialType::MilitaryPersonnel { .. } => OfficialCategory::Military,
            OfficialType::PolicePersonnel { .. } => OfficialCategory::Police,
            OfficialType::StateEnterpriseEmployee { .. } => OfficialCategory::StateEnterprise,
            OfficialType::Judge { .. } => OfficialCategory::Judge,
            OfficialType::Prosecutor { .. } => OfficialCategory::Prosecutor,
        }
    }

    /// Get the Lao description (ຮັບລາຍລະອຽດເປັນພາສາລາວ)
    pub fn lao_description(&self) -> String {
        match self {
            OfficialType::GovernmentOfficial {
                position_grade,
                ministry,
            } => {
                let ministry_str = ministry
                    .as_ref()
                    .map_or(String::new(), |m| format!(" - {}", m));
                format!(
                    "ພະນັກງານລັດຖະບານ {}{}",
                    position_grade.lao_name(),
                    ministry_str
                )
            }
            OfficialType::ElectedOfficial { position, level } => {
                format!("ພະນັກງານຖືກເລືອກຕັ້ງ: {} ລະດັບ {}", position, level)
            }
            OfficialType::MilitaryPersonnel { rank, unit } => {
                let unit_str = unit.as_ref().map_or(String::new(), |u| format!(" - {}", u));
                format!("ທະຫານ ຍົດ {}{}", rank, unit_str)
            }
            OfficialType::PolicePersonnel { rank, unit } => {
                let unit_str = unit.as_ref().map_or(String::new(), |u| format!(" - {}", u));
                format!("ຕຳຫຼວດ ຍົດ {}{}", rank, unit_str)
            }
            OfficialType::StateEnterpriseEmployee {
                position,
                enterprise_name,
            } => {
                format!("ພະນັກງານວິສາຫະກິດລັດ: {} - {}", position, enterprise_name)
            }
            OfficialType::Judge {
                court_level,
                position,
            } => {
                format!("ຜູ້ພິພາກສາ {} ສານ {}", position, court_level)
            }
            OfficialType::Prosecutor {
                office_level,
                position,
            } => {
                format!("ໄອຍະການ {} ຫ້ອງການ {}", position, office_level)
            }
        }
    }
}

// ============================================================================
// Corruption Offenses - ການກະທຳຜິດສໍ້ລາດບັງຫຼວງ
// ============================================================================

/// Bribery direction - ທິດທາງສິນບົນ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BriberyDirection {
    /// Giving bribe (ໃຫ້ສິນບົນ)
    Giving,
    /// Receiving bribe (ຮັບສິນບົນ)
    Receiving,
}

impl BriberyDirection {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            BriberyDirection::Giving => "ໃຫ້ສິນບົນ",
            BriberyDirection::Receiving => "ຮັບສິນບົນ",
        }
    }
}

/// Fund source for embezzlement - ແຫຼ່ງເງິນທຶນການສໍ້ໂກງ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FundSource {
    /// State budget (ງົບປະມານແຫ່ງລັດ)
    StateBudget,
    /// Foreign aid/loan (ເງິນກູ້/ເງິນຊ່ວຍເຫຼືອຕ່າງປະເທດ)
    ForeignAid,
    /// State enterprise funds (ເງິນວິສາຫະກິດລັດ)
    StateEnterprise,
    /// Social fund (ກອງທຶນສັງຄົມ)
    SocialFund,
    /// Public donation (ເງິນບໍລິຈາກສາທາລະນະ)
    PublicDonation,
}

impl FundSource {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            FundSource::StateBudget => "ງົບປະມານແຫ່ງລັດ",
            FundSource::ForeignAid => "ເງິນກູ້/ເງິນຊ່ວຍເຫຼືອຕ່າງປະເທດ",
            FundSource::StateEnterprise => "ເງິນວິສາຫະກິດລັດ",
            FundSource::SocialFund => "ກອງທຶນສັງຄົມ",
            FundSource::PublicDonation => "ເງິນບໍລິຈາກສາທາລະນະ",
        }
    }
}

/// Corruption offense type - ປະເພດການກະທຳຜິດສໍ້ລາດບັງຫຼວງ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CorruptionOffenseType {
    /// Bribery (ການໃຫ້/ຮັບສິນບົນ) - Articles 25-27
    Bribery {
        direction: BriberyDirection,
        amount_lak: u64,
    },
    /// Embezzlement (ການສໍ້ໂກງ) - Articles 28-31
    Embezzlement {
        amount_lak: u64,
        fund_source: FundSource,
    },
    /// Abuse of position (ການໃຊ້ຕຳແໜ່ງໃນທາງທີ່ຜິດ) - Articles 32-34
    AbuseOfPosition {
        description_lao: String,
        description_en: String,
        benefit_value_lak: Option<u64>,
    },
    /// Nepotism (ການລຳອຽງເພາະຍາດພີ່ນ້ອງ) - Articles 35-37
    Nepotism {
        relationship: String,
        benefit_description: String,
    },
    /// Conflict of interest (ຜົນປະໂຫຍດຂັດກັນ) - Articles 38-41
    ConflictOfInterest {
        conflict_description_lao: String,
        conflict_description_en: String,
        personal_benefit_lak: Option<u64>,
    },
    /// Illicit enrichment (ການຮັ່ງມີທີ່ບໍ່ຊອບດ້ວຍກົດໝາຍ) - Articles 42-45
    IllicitEnrichment {
        unexplained_assets_lak: u64,
        declared_income_lak: u64,
    },
}

impl CorruptionOffenseType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            CorruptionOffenseType::Bribery { .. } => "ການໃຫ້/ຮັບສິນບົນ",
            CorruptionOffenseType::Embezzlement { .. } => "ການສໍ້ໂກງ",
            CorruptionOffenseType::AbuseOfPosition { .. } => "ການໃຊ້ຕຳແໜ່ງໃນທາງທີ່ຜິດ",
            CorruptionOffenseType::Nepotism { .. } => "ການລຳອຽງເພາະຍາດພີ່ນ້ອງ",
            CorruptionOffenseType::ConflictOfInterest { .. } => "ຜົນປະໂຫຍດຂັດກັນ",
            CorruptionOffenseType::IllicitEnrichment { .. } => "ການຮັ່ງມີທີ່ບໍ່ຊອບດ້ວຍກົດໝາຍ",
        }
    }

    /// Get the relevant article numbers
    pub fn article_numbers(&self) -> Vec<u32> {
        match self {
            CorruptionOffenseType::Bribery { .. } => vec![25, 26, 27],
            CorruptionOffenseType::Embezzlement { .. } => vec![28, 29, 30, 31],
            CorruptionOffenseType::AbuseOfPosition { .. } => vec![32, 33, 34],
            CorruptionOffenseType::Nepotism { .. } => vec![35, 36, 37],
            CorruptionOffenseType::ConflictOfInterest { .. } => vec![38, 39, 40, 41],
            CorruptionOffenseType::IllicitEnrichment { .. } => vec![42, 43, 44, 45],
        }
    }

    /// Get the amount involved (if applicable)
    pub fn amount_involved(&self) -> Option<u64> {
        match self {
            CorruptionOffenseType::Bribery { amount_lak, .. } => Some(*amount_lak),
            CorruptionOffenseType::Embezzlement { amount_lak, .. } => Some(*amount_lak),
            CorruptionOffenseType::AbuseOfPosition {
                benefit_value_lak, ..
            } => *benefit_value_lak,
            CorruptionOffenseType::ConflictOfInterest {
                personal_benefit_lak,
                ..
            } => *personal_benefit_lak,
            CorruptionOffenseType::IllicitEnrichment {
                unexplained_assets_lak,
                ..
            } => Some(*unexplained_assets_lak),
            CorruptionOffenseType::Nepotism { .. } => None,
        }
    }
}

/// Corruption severity level - ລະດັບຄວາມຮ້າຍແຮງການສໍ້ລາດບັງຫຼວງ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CorruptionSeverity {
    /// Minor (< 5 million LAK) - ຂະໜາດນ້ອຍ
    Minor,
    /// Medium (5-50 million LAK) - ຂະໜາດກາງ
    Medium,
    /// Serious (50-500 million LAK) - ຂະໜາດຮ້າຍແຮງ
    Serious,
    /// Very serious (> 500 million LAK) - ຂະໜາດຮ້າຍແຮງຫຼາຍ
    VerySerious,
}

impl CorruptionSeverity {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            CorruptionSeverity::Minor => "ຂະໜາດນ້ອຍ",
            CorruptionSeverity::Medium => "ຂະໜາດກາງ",
            CorruptionSeverity::Serious => "ຂະໜາດຮ້າຍແຮງ",
            CorruptionSeverity::VerySerious => "ຂະໜາດຮ້າຍແຮງຫຼາຍ",
        }
    }

    /// Determine severity from amount
    pub fn from_amount(amount_lak: u64) -> Self {
        if amount_lak < MINOR_CORRUPTION_THRESHOLD_LAK {
            CorruptionSeverity::Minor
        } else if amount_lak < MEDIUM_CORRUPTION_THRESHOLD_LAK {
            CorruptionSeverity::Medium
        } else if amount_lak < SERIOUS_CORRUPTION_THRESHOLD_LAK {
            CorruptionSeverity::Serious
        } else {
            CorruptionSeverity::VerySerious
        }
    }
}

/// Investigation status - ສະຖານະການສືບສວນ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InvestigationStatus {
    /// Preliminary investigation (ການສືບສວນເບື້ອງຕົ້ນ)
    Preliminary,
    /// Under investigation (ກຳລັງສືບສວນ)
    UnderInvestigation,
    /// Investigation completed (ສືບສວນສຳເລັດ)
    Completed,
    /// Referred to prosecutor (ສົ່ງຟ້ອງ)
    ReferredToProsecutor,
    /// Case closed (ປິດຄະດີ)
    Closed,
}

impl InvestigationStatus {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            InvestigationStatus::Preliminary => "ການສືບສວນເບື້ອງຕົ້ນ",
            InvestigationStatus::UnderInvestigation => "ກຳລັງສືບສວນ",
            InvestigationStatus::Completed => "ສືບສວນສຳເລັດ",
            InvestigationStatus::ReferredToProsecutor => "ສົ່ງຟ້ອງ",
            InvestigationStatus::Closed => "ປິດຄະດີ",
        }
    }
}

/// Corruption offense - ການກະທຳຜິດສໍ້ລາດບັງຫຼວງ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CorruptionOffense {
    /// Offense type (ປະເພດການກະທຳຜິດ)
    pub offense_type: CorruptionOffenseType,
    /// Perpetrator (ຜູ້ກະທຳຜິດ)
    pub perpetrator: OfficialType,
    /// Date of offense (ວັນທີກະທຳຜິດ)
    pub date_of_offense: String,
    /// Location province (ແຂວງ)
    pub location_province: String,
    /// Description in Lao (ລາຍລະອຽດເປັນພາສາລາວ)
    pub description_lao: String,
    /// Description in English (ລາຍລະອຽດເປັນພາສາອັງກິດ)
    pub description_en: String,
    /// Evidence collected (ເກັບກຳຫຼັກຖານແລ້ວ)
    pub evidence_collected: bool,
    /// Investigation status (ສະຖານະການສືບສວນ)
    pub investigation_status: InvestigationStatus,
}

// ============================================================================
// Asset Declaration - ການປະກາດຊັບສິນ
// ============================================================================

/// Property type - ປະເພດອະສັງຫາລິມະຊັບ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PropertyType {
    /// House (ເຮືອນ)
    House,
    /// Land (ທີ່ດິນ)
    Land,
    /// Apartment/Condominium (ຫ້ອງແຖວ/ຄອນໂດ)
    Apartment,
    /// Commercial building (ອາຄານການຄ້າ)
    Commercial,
    /// Agricultural land (ທີ່ດິນກະສິກຳ)
    Agricultural,
}

impl PropertyType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            PropertyType::House => "ເຮືອນ",
            PropertyType::Land => "ທີ່ດິນ",
            PropertyType::Apartment => "ຫ້ອງແຖວ/ຄອນໂດ",
            PropertyType::Commercial => "ອາຄານການຄ້າ",
            PropertyType::Agricultural => "ທີ່ດິນກະສິກຳ",
        }
    }
}

/// Vehicle type - ປະເພດພາຫະນະ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VehicleType {
    /// Car (ລົດໃຫຍ່)
    Car,
    /// Motorcycle (ລົດຈັກ)
    Motorcycle,
    /// Truck (ລົດບັນທຸກ)
    Truck,
    /// Boat (ເຮືອ)
    Boat,
}

impl VehicleType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            VehicleType::Car => "ລົດໃຫຍ່",
            VehicleType::Motorcycle => "ລົດຈັກ",
            VehicleType::Truck => "ລົດບັນທຸກ",
            VehicleType::Boat => "ເຮືອ",
        }
    }
}

/// Acquisition method - ວິທີການໄດ້ມາ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AcquisitionMethod {
    /// Purchase (ຊື້)
    Purchase,
    /// Inheritance (ມໍລະດົກ)
    Inheritance,
    /// Gift (ຂອງຂວັນ)
    Gift,
    /// Salary/Income (ເງິນເດືອນ/ລາຍຮັບ)
    SalaryIncome,
    /// Loan (ກູ້ຢືມ)
    Loan,
    /// Other (ອື່ນໆ)
    Other,
}

impl AcquisitionMethod {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            AcquisitionMethod::Purchase => "ຊື້",
            AcquisitionMethod::Inheritance => "ມໍລະດົກ",
            AcquisitionMethod::Gift => "ຂອງຂວັນ",
            AcquisitionMethod::SalaryIncome => "ເງິນເດືອນ/ລາຍຮັບ",
            AcquisitionMethod::Loan => "ກູ້ຢືມ",
            AcquisitionMethod::Other => "ອື່ນໆ",
        }
    }
}

/// Real estate asset - ຊັບສິນອະສັງຫາລິມະຊັບ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RealEstate {
    /// Property type (ປະເພດອະສັງຫາລິມະຊັບ)
    pub property_type: PropertyType,
    /// Location in Lao (ສະຖານທີ່ເປັນພາສາລາວ)
    pub location_lao: String,
    /// Location in English (ສະຖານທີ່ເປັນພາສາອັງກິດ)
    pub location_en: String,
    /// Estimated value in LAK (ມູນຄ່າຄາດຄະເນເປັນກີບ)
    pub estimated_value_lak: u64,
    /// Acquisition date (ວັນທີໄດ້ມາ)
    pub acquisition_date: Option<String>,
    /// Acquisition method (ວິທີການໄດ້ມາ)
    pub acquisition_method: AcquisitionMethod,
}

/// Vehicle asset - ຊັບສິນພາຫະນະ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vehicle {
    /// Vehicle type (ປະເພດພາຫະນະ)
    pub vehicle_type: VehicleType,
    /// Make and model (ຍີ່ຫໍ້ ແລະ ລຸ້ນ)
    pub make_model: String,
    /// Year (ປີ)
    pub year: u16,
    /// Estimated value in LAK (ມູນຄ່າຄາດຄະເນເປັນກີບ)
    pub estimated_value_lak: u64,
    /// Acquisition method (ວິທີການໄດ້ມາ)
    pub acquisition_method: AcquisitionMethod,
}

/// Income source type - ປະເພດແຫຼ່ງລາຍຮັບ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IncomeSourceType {
    /// Salary (ເງິນເດືອນ)
    Salary,
    /// Allowances (ເງິນອຸດໜູນ)
    Allowances,
    /// Business income (ລາຍຮັບທຸລະກິດ)
    Business,
    /// Rental income (ລາຍຮັບຈາກການເຊົ່າ)
    Rental,
    /// Investment income (ລາຍຮັບຈາກການລົງທຶນ)
    Investment,
    /// Spouse income (ລາຍຮັບຂອງຄູ່ສົມລົດ)
    SpouseIncome,
    /// Other (ອື່ນໆ)
    Other,
}

impl IncomeSourceType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            IncomeSourceType::Salary => "ເງິນເດືອນ",
            IncomeSourceType::Allowances => "ເງິນອຸດໜູນ",
            IncomeSourceType::Business => "ລາຍຮັບທຸລະກິດ",
            IncomeSourceType::Rental => "ລາຍຮັບຈາກການເຊົ່າ",
            IncomeSourceType::Investment => "ລາຍຮັບຈາກການລົງທຶນ",
            IncomeSourceType::SpouseIncome => "ລາຍຮັບຂອງຄູ່ສົມລົດ",
            IncomeSourceType::Other => "ອື່ນໆ",
        }
    }
}

/// Income source - ແຫຼ່ງລາຍຮັບ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IncomeSource {
    /// Source type (ປະເພດແຫຼ່ງ)
    pub source_type: IncomeSourceType,
    /// Description in Lao (ລາຍລະອຽດເປັນພາສາລາວ)
    pub description_lao: String,
    /// Description in English (ລາຍລະອຽດເປັນພາສາອັງກິດ)
    pub description_en: String,
    /// Annual amount in LAK (ຈຳນວນເງິນປະຈຳປີເປັນກີບ)
    pub annual_amount_lak: u64,
}

/// Asset declaration status - ສະຖານະການປະກາດຊັບສິນ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AssetDeclarationStatus {
    /// Pending submission (ລໍຖ້າການຍື່ນ)
    Pending,
    /// Submitted (ຍື່ນແລ້ວ)
    Submitted,
    /// Under verification (ກຳລັງກວດສອບ)
    UnderVerification,
    /// Verified (ກວດສອບແລ້ວ)
    Verified,
    /// Discrepancy found (ພົບຄວາມແຕກຕ່າງ)
    DiscrepancyFound,
    /// Rejected (ປະຕິເສດ)
    Rejected,
}

impl AssetDeclarationStatus {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            AssetDeclarationStatus::Pending => "ລໍຖ້າການຍື່ນ",
            AssetDeclarationStatus::Submitted => "ຍື່ນແລ້ວ",
            AssetDeclarationStatus::UnderVerification => "ກຳລັງກວດສອບ",
            AssetDeclarationStatus::Verified => "ກວດສອບແລ້ວ",
            AssetDeclarationStatus::DiscrepancyFound => "ພົບຄວາມແຕກຕ່າງ",
            AssetDeclarationStatus::Rejected => "ປະຕິເສດ",
        }
    }
}

/// Asset declaration - ການປະກາດຊັບສິນ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AssetDeclaration {
    /// Official ID (ລະຫັດພະນັກງານ)
    pub official_id: String,
    /// Official name in Lao (ຊື່ພະນັກງານເປັນພາສາລາວ)
    pub official_name_lao: String,
    /// Official name in English (ຊື່ພະນັກງານເປັນພາສາອັງກິດ)
    pub official_name_en: String,
    /// Position grade (ລະດັບຕຳແໜ່ງ)
    pub position_grade: PositionGrade,
    /// Ministry/Organization (ກະຊວງ/ອົງການ)
    pub ministry: String,
    /// Declaration year (ປີປະກາດ)
    pub declaration_year: u16,
    /// Real estate assets (ອະສັງຫາລິມະຊັບ)
    pub real_estate: Vec<RealEstate>,
    /// Vehicle assets (ພາຫະນະ)
    pub vehicles: Vec<Vehicle>,
    /// Bank accounts balance in LAK (ຍອດເງິນໃນບັນຊີທະນາຄານ)
    pub bank_balance_lak: u64,
    /// Other assets value in LAK (ມູນຄ່າຊັບສິນອື່ນໆ)
    pub other_assets_lak: u64,
    /// Income sources (ແຫຼ່ງລາຍຮັບ)
    pub income_sources: Vec<IncomeSource>,
    /// Total assets in LAK (ຊັບສິນທັງໝົດເປັນກີບ)
    pub total_assets_lak: u64,
    /// Total liabilities in LAK (ໜີ້ສິນທັງໝົດເປັນກີບ)
    pub total_liabilities_lak: u64,
    /// Net worth in LAK (ມູນຄ່າສຸດທິເປັນກີບ)
    pub net_worth_lak: i64,
    /// Submission date (ວັນທີຍື່ນ)
    pub submission_date: Option<String>,
    /// Status (ສະຖານະ)
    pub status: AssetDeclarationStatus,
    /// Verification result (ຜົນການກວດສອບ)
    pub verification_result: Option<VerificationResult>,
}

impl Default for AssetDeclaration {
    fn default() -> Self {
        Self {
            official_id: String::new(),
            official_name_lao: String::new(),
            official_name_en: String::new(),
            position_grade: PositionGrade::Grade5,
            ministry: String::new(),
            declaration_year: 2025,
            real_estate: Vec::new(),
            vehicles: Vec::new(),
            bank_balance_lak: 0,
            other_assets_lak: 0,
            income_sources: Vec::new(),
            total_assets_lak: 0,
            total_liabilities_lak: 0,
            net_worth_lak: 0,
            submission_date: None,
            status: AssetDeclarationStatus::Pending,
            verification_result: None,
        }
    }
}

/// Builder for AssetDeclaration
#[derive(Debug, Default)]
pub struct AssetDeclarationBuilder {
    declaration: AssetDeclaration,
}

impl AssetDeclarationBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set official ID
    pub fn official_id(mut self, id: impl Into<String>) -> Self {
        self.declaration.official_id = id.into();
        self
    }

    /// Set official name in Lao
    pub fn official_name_lao(mut self, name: impl Into<String>) -> Self {
        self.declaration.official_name_lao = name.into();
        self
    }

    /// Set official name in English
    pub fn official_name_en(mut self, name: impl Into<String>) -> Self {
        self.declaration.official_name_en = name.into();
        self
    }

    /// Set position grade
    pub fn position_grade(mut self, grade: PositionGrade) -> Self {
        self.declaration.position_grade = grade;
        self
    }

    /// Set ministry
    pub fn ministry(mut self, ministry: impl Into<String>) -> Self {
        self.declaration.ministry = ministry.into();
        self
    }

    /// Set declaration year
    pub fn declaration_year(mut self, year: u16) -> Self {
        self.declaration.declaration_year = year;
        self
    }

    /// Add real estate
    pub fn add_real_estate(mut self, property: RealEstate) -> Self {
        self.declaration.real_estate.push(property);
        self
    }

    /// Add vehicle
    pub fn add_vehicle(mut self, vehicle: Vehicle) -> Self {
        self.declaration.vehicles.push(vehicle);
        self
    }

    /// Set bank balance
    pub fn bank_balance_lak(mut self, balance: u64) -> Self {
        self.declaration.bank_balance_lak = balance;
        self
    }

    /// Add income source
    pub fn add_income_source(mut self, source: IncomeSource) -> Self {
        self.declaration.income_sources.push(source);
        self
    }

    /// Set total assets
    pub fn total_assets_lak(mut self, amount: u64) -> Self {
        self.declaration.total_assets_lak = amount;
        self
    }

    /// Set total liabilities
    pub fn total_liabilities_lak(mut self, amount: u64) -> Self {
        self.declaration.total_liabilities_lak = amount;
        self
    }

    /// Set submission date
    pub fn submission_date(mut self, date: impl Into<String>) -> Self {
        self.declaration.submission_date = Some(date.into());
        self.declaration.status = AssetDeclarationStatus::Submitted;
        self
    }

    /// Build the declaration
    pub fn build(mut self) -> AssetDeclaration {
        // Calculate net worth
        self.declaration.net_worth_lak = self.declaration.total_assets_lak as i64
            - self.declaration.total_liabilities_lak as i64;
        self.declaration
    }
}

/// Verification result - ຜົນການກວດສອບ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VerificationResult {
    /// Verification date (ວັນທີກວດສອບ)
    pub verification_date: String,
    /// Verifier name (ຊື່ຜູ້ກວດສອບ)
    pub verifier_name: String,
    /// Status (ສະຖານະ)
    pub status: VerificationStatus,
    /// Discrepancy amount if any (ຈຳນວນເງິນທີ່ແຕກຕ່າງ)
    pub discrepancy_amount_lak: Option<u64>,
    /// Notes (ບັນທຶກ)
    pub notes: Option<String>,
}

/// Verification status - ສະຖານະການກວດສອບ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VerificationStatus {
    /// Verified - accurate (ກວດສອບແລ້ວ - ຖືກຕ້ອງ)
    Accurate,
    /// Minor discrepancy (ຄວາມແຕກຕ່າງນ້ອຍ)
    MinorDiscrepancy,
    /// Major discrepancy (ຄວາມແຕກຕ່າງໃຫຍ່)
    MajorDiscrepancy,
    /// Under investigation (ກຳລັງສືບສວນ)
    UnderInvestigation,
}

impl VerificationStatus {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            VerificationStatus::Accurate => "ຖືກຕ້ອງ",
            VerificationStatus::MinorDiscrepancy => "ຄວາມແຕກຕ່າງນ້ອຍ",
            VerificationStatus::MajorDiscrepancy => "ຄວາມແຕກຕ່າງໃຫຍ່",
            VerificationStatus::UnderInvestigation => "ກຳລັງສືບສວນ",
        }
    }
}

// ============================================================================
// Penalties - ການລົງໂທດ
// ============================================================================

/// Penalty type - ປະເພດການລົງໂທດ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PenaltyType {
    /// Imprisonment (ຈຳຄຸກ)
    Imprisonment { min_months: u32, max_months: u32 },
    /// Fine (ປັບໃໝ)
    Fine { min_lak: u64, max_lak: u64 },
    /// Asset forfeiture (ຍຶດຊັບສິນ)
    AssetForfeiture { amount_lak: u64 },
    /// Dismissal from office (ປົດຕຳແໜ່ງ)
    Dismissal,
    /// Prohibition from holding public office (ຫ້າມດຳລົງຕຳແໜ່ງສາທາລະນະ)
    ProhibitionFromOffice { years: u32 },
    /// Disciplinary measure (ມາດຕະການວິໄນ)
    DisciplinaryMeasure { measure_type: String },
}

impl PenaltyType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            PenaltyType::Imprisonment { .. } => "ຈຳຄຸກ",
            PenaltyType::Fine { .. } => "ປັບໃໝ",
            PenaltyType::AssetForfeiture { .. } => "ຍຶດຊັບສິນ",
            PenaltyType::Dismissal => "ປົດຕຳແໜ່ງ",
            PenaltyType::ProhibitionFromOffice { .. } => "ຫ້າມດຳລົງຕຳແໜ່ງສາທາລະນະ",
            PenaltyType::DisciplinaryMeasure { .. } => "ມາດຕະການວິໄນ",
        }
    }
}

/// Penalty range - ຂອບເຂດການລົງໂທດ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PenaltyRange {
    /// Severity (ລະດັບຄວາມຮ້າຍແຮງ)
    pub severity: CorruptionSeverity,
    /// Minimum imprisonment months (ຈຳຄຸກຕ່ຳສຸດ - ເດືອນ)
    pub min_imprisonment_months: u32,
    /// Maximum imprisonment months (ຈຳຄຸກສູງສຸດ - ເດືອນ)
    pub max_imprisonment_months: u32,
    /// Life imprisonment possible (ອາດຈຳຄຸກຕະຫຼອດຊີວິດ)
    pub life_imprisonment_possible: bool,
    /// Asset forfeiture applicable (ນຳໃຊ້ການຍຶດຊັບສິນ)
    pub asset_forfeiture_applicable: bool,
    /// Dismissal mandatory (ບັງຄັບປົດຕຳແໜ່ງ)
    pub dismissal_mandatory: bool,
}

impl PenaltyRange {
    /// Create penalty range for minor corruption
    pub fn minor() -> Self {
        Self {
            severity: CorruptionSeverity::Minor,
            min_imprisonment_months: 3,
            max_imprisonment_months: 12,
            life_imprisonment_possible: false,
            asset_forfeiture_applicable: false,
            dismissal_mandatory: false,
        }
    }

    /// Create penalty range for medium corruption
    pub fn medium() -> Self {
        Self {
            severity: CorruptionSeverity::Medium,
            min_imprisonment_months: 12,
            max_imprisonment_months: 60,
            life_imprisonment_possible: false,
            asset_forfeiture_applicable: true,
            dismissal_mandatory: true,
        }
    }

    /// Create penalty range for serious corruption
    pub fn serious() -> Self {
        Self {
            severity: CorruptionSeverity::Serious,
            min_imprisonment_months: 60,
            max_imprisonment_months: 120,
            life_imprisonment_possible: false,
            asset_forfeiture_applicable: true,
            dismissal_mandatory: true,
        }
    }

    /// Create penalty range for very serious corruption
    pub fn very_serious() -> Self {
        Self {
            severity: CorruptionSeverity::VerySerious,
            min_imprisonment_months: 120,
            max_imprisonment_months: 240,
            life_imprisonment_possible: true,
            asset_forfeiture_applicable: true,
            dismissal_mandatory: true,
        }
    }
}

// ============================================================================
// Whistleblower Protection - ການປົກປ້ອງຜູ້ແຈ້ງຂ່າວ
// ============================================================================

/// Whistleblower report status - ສະຖານະການແຈ້ງຂ່າວ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WhistleblowerReportStatus {
    /// Received (ໄດ້ຮັບແລ້ວ)
    Received,
    /// Under review (ກຳລັງພິຈາລະນາ)
    UnderReview,
    /// Accepted (ຍອมຮັບ)
    Accepted,
    /// Rejected (ປະຕິເສດ)
    Rejected,
    /// Under investigation (ກຳລັງສືບສວນ)
    UnderInvestigation,
    /// Resolved (ແກ້ໄຂແລ້ວ)
    Resolved,
}

impl WhistleblowerReportStatus {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            WhistleblowerReportStatus::Received => "ໄດ້ຮັບແລ້ວ",
            WhistleblowerReportStatus::UnderReview => "ກຳລັງພິຈາລະນາ",
            WhistleblowerReportStatus::Accepted => "ຍອมຮັບ",
            WhistleblowerReportStatus::Rejected => "ປະຕິເສດ",
            WhistleblowerReportStatus::UnderInvestigation => "ກຳລັງສືບສວນ",
            WhistleblowerReportStatus::Resolved => "ແກ້ໄຂແລ້ວ",
        }
    }
}

/// Whistleblower protection type - ປະເພດການປົກປ້ອງຜູ້ແຈ້ງຂ່າວ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WhistleblowerProtectionType {
    /// Identity protection (ປົກປ້ອງຕົວຕົນ)
    IdentityProtection,
    /// Employment protection (ປົກປ້ອງການຈ້າງງານ)
    EmploymentProtection,
    /// Physical protection (ປົກປ້ອງຮ່າງກາຍ)
    PhysicalProtection,
    /// Legal representation (ຕົວແທນທາງກົດໝາຍ)
    LegalRepresentation,
    /// Relocation assistance (ຊ່ວຍເຫຼືອການຍ້າຍຖິ່ນ)
    RelocationAssistance,
}

impl WhistleblowerProtectionType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            WhistleblowerProtectionType::IdentityProtection => "ປົກປ້ອງຕົວຕົນ",
            WhistleblowerProtectionType::EmploymentProtection => "ປົກປ້ອງການຈ້າງງານ",
            WhistleblowerProtectionType::PhysicalProtection => "ປົກປ້ອງຮ່າງກາຍ",
            WhistleblowerProtectionType::LegalRepresentation => "ຕົວແທນທາງກົດໝາຍ",
            WhistleblowerProtectionType::RelocationAssistance => "ຊ່ວຍເຫຼືອການຍ້າຍຖິ່ນ",
        }
    }
}

/// Whistleblower protection - ການປົກປ້ອງຜູ້ແຈ້ງຂ່າວ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WhistleblowerProtection {
    /// Protection types granted (ປະເພດການປົກປ້ອງທີ່ໃຫ້)
    pub protection_types: Vec<WhistleblowerProtectionType>,
    /// Start date (ວັນທີເລີ່ມຕົ້ນ)
    pub start_date: String,
    /// End date if applicable (ວັນທີສິ້ນສຸດ)
    pub end_date: Option<String>,
    /// Active (ມີຜົນບັງຄັບໃຊ້)
    pub active: bool,
}

/// Whistleblower report - ການແຈ້ງຂ່າວ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WhistleblowerReport {
    /// Report ID (ລະຫັດການແຈ້ງຂ່າວ)
    pub report_id: Option<String>,
    /// Anonymous (ບໍ່ລະບຸຊື່)
    pub anonymous: bool,
    /// Reporter name if not anonymous (ຊື່ຜູ້ແຈ້ງຂ່າວ)
    pub reporter_name: Option<String>,
    /// Reporter contact if not anonymous (ຂໍ້ມູນຕິດຕໍ່ຜູ້ແຈ້ງຂ່າວ)
    pub reporter_contact: Option<String>,
    /// Allegation type (ປະເພດຂໍ້ກ່າວຫາ)
    pub allegation_type: CorruptionOffenseType,
    /// Description in Lao (ລາຍລະອຽດເປັນພາສາລາວ)
    pub description_lao: String,
    /// Description in English (ລາຍລະອຽດເປັນພາສາອັງກິດ)
    pub description_en: String,
    /// Accused official description (ລາຍລະອຽດພະນັກງານຖືກກ່າວຫາ)
    pub accused_official_description: String,
    /// Evidence description (ລາຍລະອຽດຫຼັກຖານ)
    pub evidence_description: Option<String>,
    /// Submission date (ວັນທີຍື່ນ)
    pub submission_date: String,
    /// Status (ສະຖານະ)
    pub status: WhistleblowerReportStatus,
    /// Protection granted (ການປົກປ້ອງທີ່ໃຫ້)
    pub protection: Option<WhistleblowerProtection>,
    /// Reward amount if applicable (ຈຳນວນລາງວັນ)
    pub reward_amount_lak: Option<u64>,
}

impl Default for WhistleblowerReport {
    fn default() -> Self {
        Self {
            report_id: None,
            anonymous: true,
            reporter_name: None,
            reporter_contact: None,
            allegation_type: CorruptionOffenseType::Bribery {
                direction: BriberyDirection::Receiving,
                amount_lak: 0,
            },
            description_lao: String::new(),
            description_en: String::new(),
            accused_official_description: String::new(),
            evidence_description: None,
            submission_date: String::new(),
            status: WhistleblowerReportStatus::Received,
            protection: None,
            reward_amount_lak: None,
        }
    }
}

/// Builder for WhistleblowerReport
#[derive(Debug, Default)]
pub struct WhistleblowerReportBuilder {
    report: WhistleblowerReport,
}

impl WhistleblowerReportBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set anonymous status
    pub fn anonymous(mut self, anonymous: bool) -> Self {
        self.report.anonymous = anonymous;
        self
    }

    /// Set reporter name
    pub fn reporter_name(mut self, name: impl Into<String>) -> Self {
        self.report.reporter_name = Some(name.into());
        self.report.anonymous = false;
        self
    }

    /// Set allegation type
    pub fn allegation_type(mut self, allegation: CorruptionOffenseType) -> Self {
        self.report.allegation_type = allegation;
        self
    }

    /// Set description in Lao
    pub fn description_lao(mut self, desc: impl Into<String>) -> Self {
        self.report.description_lao = desc.into();
        self
    }

    /// Set description in English
    pub fn description_en(mut self, desc: impl Into<String>) -> Self {
        self.report.description_en = desc.into();
        self
    }

    /// Set accused official description
    pub fn accused_official_description(mut self, desc: impl Into<String>) -> Self {
        self.report.accused_official_description = desc.into();
        self
    }

    /// Set evidence description
    pub fn evidence_description(mut self, desc: impl Into<String>) -> Self {
        self.report.evidence_description = Some(desc.into());
        self
    }

    /// Set submission date
    pub fn submission_date(mut self, date: impl Into<String>) -> Self {
        self.report.submission_date = date.into();
        self
    }

    /// Build the report
    pub fn build(self) -> WhistleblowerReport {
        self.report
    }
}

// ============================================================================
// Prevention Measures - ມາດຕະການປ້ອງກັນ
// ============================================================================

/// Prevention measure type - ປະເພດມາດຕະການປ້ອງກັນ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PreventionMeasureType {
    /// Code of conduct (ລະບຽບພຶດຕິກຳ)
    CodeOfConduct,
    /// Procurement transparency (ຄວາມໂປ່ງໃສໃນການຈັດຊື້)
    ProcurementTransparency,
    /// Gift restrictions (ຂໍ້ຈຳກັດຂອງຂວັນ)
    GiftRestrictions,
    /// Cooling-off period (ໄລຍະຫ່າງ)
    CoolingOffPeriod,
    /// Conflict of interest disclosure (ການເປີດເຜີຍຜົນປະໂຫຍດຂັດກັນ)
    ConflictOfInterestDisclosure,
    /// Training and awareness (ການຝຶກອົບຮົມ ແລະ ການສ້າງຄວາມຮູ້)
    TrainingAwareness,
}

impl PreventionMeasureType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            PreventionMeasureType::CodeOfConduct => "ລະບຽບພຶດຕິກຳ",
            PreventionMeasureType::ProcurementTransparency => "ຄວາມໂປ່ງໃສໃນການຈັດຊື້",
            PreventionMeasureType::GiftRestrictions => "ຂໍ້ຈຳກັດຂອງຂວັນ",
            PreventionMeasureType::CoolingOffPeriod => "ໄລຍະຫ່າງ",
            PreventionMeasureType::ConflictOfInterestDisclosure => "ການເປີດເຜີຍຜົນປະໂຫຍດຂັດກັນ",
            PreventionMeasureType::TrainingAwareness => "ການຝຶກອົບຮົມ ແລະ ການສ້າງຄວາມຮູ້",
        }
    }
}

/// Prevention measure - ມາດຕະການປ້ອງກັນ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PreventionMeasure {
    /// Measure type (ປະເພດມາດຕະການ)
    pub measure_type: PreventionMeasureType,
    /// Description in Lao (ລາຍລະອຽດເປັນພາສາລາວ)
    pub description_lao: String,
    /// Description in English (ລາຍລະອຽດເປັນພາສາອັງກິດ)
    pub description_en: String,
    /// Applicable to (ນຳໃຊ້ກັບ)
    pub applicable_to: Vec<OfficialCategory>,
    /// Implementation date (ວັນທີນຳໃຊ້)
    pub implementation_date: Option<String>,
}

/// Gift type - ປະເພດຂອງຂວັນ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GiftType {
    /// Cash (ເງິນສົດ)
    Cash,
    /// Item (ສິ່ງຂອງ)
    Item,
    /// Service (ບໍລິການ)
    Service,
    /// Entertainment (ການບັນເທີງ)
    Entertainment,
    /// Travel (ການເດີນທາງ)
    Travel,
}

impl GiftType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            GiftType::Cash => "ເງິນສົດ",
            GiftType::Item => "ສິ່ງຂອງ",
            GiftType::Service => "ບໍລິການ",
            GiftType::Entertainment => "ການບັນເທີງ",
            GiftType::Travel => "ການເດີນທາງ",
        }
    }
}

/// Gift record - ບັນທຶກຂອງຂວັນ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Gift {
    /// Gift type (ປະເພດຂອງຂວັນ)
    pub gift_type: GiftType,
    /// Value in LAK (ມູນຄ່າເປັນກີບ)
    pub value_lak: u64,
    /// Giver description (ລາຍລະອຽດຜູ້ໃຫ້)
    pub giver_description: String,
    /// Occasion (ໂອກາດ)
    pub occasion: String,
    /// Date received (ວັນທີໄດ້ຮັບ)
    pub date_received: String,
    /// Reported (ລາຍງານແລ້ວ)
    pub reported: bool,
}

/// Code of conduct violation type - ປະເພດການລະເມີດລະບຽບພຶດຕິກຳ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CodeOfConductViolationType {
    /// Unauthorized outside employment (ການເຮັດວຽກນອກໂດຍບໍ່ໄດ້ຮັບອະນຸຍາດ)
    UnauthorizedOutsideEmployment,
    /// Improper use of resources (ການໃຊ້ຊັບພະຍາກອນບໍ່ຖືກຕ້ອງ)
    ImproperUseOfResources,
    /// Disclosure of confidential information (ການເປີດເຜີຍຂໍ້ມູນລັບ)
    DisclosureOfConfidentialInfo,
    /// Political activity violation (ການລະເມີດກິດຈະກຳທາງການເມືອງ)
    PoliticalActivityViolation,
    /// Failure to report (ການບໍ່ລາຍງານ)
    FailureToReport,
}

impl CodeOfConductViolationType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            CodeOfConductViolationType::UnauthorizedOutsideEmployment => {
                "ການເຮັດວຽກນອກໂດຍບໍ່ໄດ້ຮັບອະນຸຍາດ"
            }
            CodeOfConductViolationType::ImproperUseOfResources => "ການໃຊ້ຊັບພະຍາກອນບໍ່ຖືກຕ້ອງ",
            CodeOfConductViolationType::DisclosureOfConfidentialInfo => "ການເປີດເຜີຍຂໍ້ມູນລັບ",
            CodeOfConductViolationType::PoliticalActivityViolation => "ການລະເມີດກິດຈະກຳທາງການເມືອງ",
            CodeOfConductViolationType::FailureToReport => "ການບໍ່ລາຍງານ",
        }
    }
}

/// Code of conduct violation - ການລະເມີດລະບຽບພຶດຕິກຳ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CodeOfConductViolation {
    /// Violation type (ປະເພດການລະເມີດ)
    pub violation_type: CodeOfConductViolationType,
    /// Description in Lao (ລາຍລະອຽດເປັນພາສາລາວ)
    pub description_lao: String,
    /// Description in English (ລາຍລະອຽດເປັນພາສາອັງກິດ)
    pub description_en: String,
    /// Date of violation (ວັນທີລະເມີດ)
    pub date_of_violation: String,
    /// Disciplinary action taken (ມາດຕະການວິໄນທີ່ດຳເນີນ)
    pub disciplinary_action: Option<String>,
}

// ============================================================================
// International Cooperation - ການຮ່ວມມືສາກົນ
// ============================================================================

/// International cooperation type - ປະເພດການຮ່ວມມືສາກົນ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InternationalCooperationType {
    /// Mutual legal assistance (ການຊ່ວຍເຫຼືອທາງກົດໝາຍ)
    MutualLegalAssistance,
    /// Asset recovery (ການຂໍຊັບຄືນ)
    AssetRecovery,
    /// Extradition (ການສົ່ງຜູ້ຮ້າຍຂ້າມແດນ)
    Extradition,
    /// Information exchange (ການແລກປ່ຽນຂໍ້ມູນ)
    InformationExchange,
    /// Joint investigation (ການສືບສວນຮ່ວມ)
    JointInvestigation,
}

impl InternationalCooperationType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            InternationalCooperationType::MutualLegalAssistance => "ການຊ່ວຍເຫຼືອທາງກົດໝາຍ",
            InternationalCooperationType::AssetRecovery => "ການຂໍຊັບຄືນ",
            InternationalCooperationType::Extradition => "ການສົ່ງຜູ້ຮ້າຍຂ້າມແດນ",
            InternationalCooperationType::InformationExchange => "ການແລກປ່ຽນຂໍ້ມູນ",
            InternationalCooperationType::JointInvestigation => "ການສືບສວນຮ່ວມ",
        }
    }
}

/// International cooperation record - ບັນທຶກການຮ່ວມມືສາກົນ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InternationalCooperation {
    /// Cooperation type (ປະເພດການຮ່ວມມື)
    pub cooperation_type: InternationalCooperationType,
    /// Foreign country (ປະເທດຕ່າງປະເທດ)
    pub foreign_country: String,
    /// Description in Lao (ລາຍລະອຽດເປັນພາສາລາວ)
    pub description_lao: String,
    /// Description in English (ລາຍລະອຽດເປັນພາສາອັງກິດ)
    pub description_en: String,
    /// Request date (ວັນທີຮ້ອງຂໍ)
    pub request_date: String,
    /// Status (ສະຖານະ)
    pub status: String,
    /// Amount involved if applicable (ຈຳນວນເງິນທີ່ກ່ຽວຂ້ອງ)
    pub amount_involved_lak: Option<u64>,
}

// ============================================================================
// Investigation - ການສືບສວນ
// ============================================================================

/// Investigation type - ປະເພດການສືບສວນ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InvestigationType {
    /// Preliminary (ເບື້ອງຕົ້ນ)
    Preliminary,
    /// Full (ເຕັມຮູບແບບ)
    Full,
}

impl InvestigationType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            InvestigationType::Preliminary => "ການສືບສວນເບື້ອງຕົ້ນ",
            InvestigationType::Full => "ການສືບສວນເຕັມຮູບແບບ",
        }
    }

    /// Get the maximum duration in days
    pub fn max_duration_days(&self) -> u32 {
        match self {
            InvestigationType::Preliminary => INVESTIGATION_PRELIMINARY_DAYS,
            InvestigationType::Full => INVESTIGATION_FULL_DAYS,
        }
    }
}

/// Investigation record - ບັນທຶກການສືບສວນ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Investigation {
    /// Investigation ID (ລະຫັດການສືບສວນ)
    pub investigation_id: String,
    /// Investigation type (ປະເພດການສືບສວນ)
    pub investigation_type: InvestigationType,
    /// Start date (ວັນທີເລີ່ມຕົ້ນ)
    pub start_date: String,
    /// End date (ວັນທີສິ້ນສຸດ)
    pub end_date: Option<String>,
    /// Status (ສະຖານະ)
    pub status: InvestigationStatus,
    /// Lead investigator (ຜູ້ສືບສວນຫຼັກ)
    pub lead_investigator: String,
    /// SIA office (ຫ້ອງການ ອກລ)
    pub sia_office: SIAOffice,
    /// Alleged offense (ຂໍ້ກ່າວຫາ)
    pub alleged_offense: CorruptionOffenseType,
    /// Evidence collected (ຫຼັກຖານທີ່ເກັບກຳໄດ້)
    pub evidence_summary: Option<String>,
    /// Referral to prosecution (ສົ່ງຟ້ອງ)
    pub prosecution_referral: Option<ProsecutionReferral>,
}

/// Prosecution referral - ການສົ່ງຟ້ອງ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProsecutionReferral {
    /// Referral date (ວັນທີສົ່ງຟ້ອງ)
    pub referral_date: String,
    /// Prosecutor office (ຫ້ອງການໄອຍະການ)
    pub prosecutor_office: String,
    /// Status (ສະຖານະ)
    pub status: ProsecutionStatus,
    /// Charges (ຂໍ້ກ່າວຫາ)
    pub charges: Vec<String>,
}

/// Prosecution status - ສະຖານະການດຳເນີນຄະດີ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProsecutionStatus {
    /// Pending (ລໍຖ້າ)
    Pending,
    /// Under review (ກຳລັງພິຈາລະນາ)
    UnderReview,
    /// Indicted (ຟ້ອງແລ້ວ)
    Indicted,
    /// Trial (ພິຈາລະນາຄະດີ)
    Trial,
    /// Convicted (ຕັດສິນລົງໂທດ)
    Convicted,
    /// Acquitted (ຕັດສິນພົ້ນໂທດ)
    Acquitted,
    /// Dismissed (ຍົກຟ້ອງ)
    Dismissed,
}

impl ProsecutionStatus {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            ProsecutionStatus::Pending => "ລໍຖ້າ",
            ProsecutionStatus::UnderReview => "ກຳລັງພິຈາລະນາ",
            ProsecutionStatus::Indicted => "ຟ້ອງແລ້ວ",
            ProsecutionStatus::Trial => "ພິຈາລະນາຄະດີ",
            ProsecutionStatus::Convicted => "ຕັດສິນລົງໂທດ",
            ProsecutionStatus::Acquitted => "ຕັດສິນພົ້ນໂທດ",
            ProsecutionStatus::Dismissed => "ຍົກຟ້ອງ",
        }
    }
}
