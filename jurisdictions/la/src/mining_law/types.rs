//! Mining Law Types (ປະເພດກົດໝາຍບໍ່ແຮ່)
//!
//! Type definitions for Lao mining law based on:
//! - **Mining Law 2017** (Law No. 31/NA, dated June 28, 2017)
//! - National Assembly of the Lao PDR
//!
//! # Legal References
//! - Mining Law 2017 (Law No. 31/NA) - ກົດໝາຍວ່າດ້ວຍບໍ່ແຮ່ ປີ 2017
//! - Mining Regulations (PM Decree No. 248/PM) - ດຳລັດວ່າດ້ວຍບໍ່ແຮ່
//!
//! # Bilingual Support
//! All types include both Lao (ລາວ) and English field names where applicable.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Mining Law 2017 (ກົດໝາຍບໍ່ແຮ່ ປີ 2017)
// ============================================================================

// === Royalty Rates (Article 45) ===

/// Royalty rate for gold mining - 5% (ມາດຕາ 45)
pub const ROYALTY_RATE_GOLD: f64 = 5.0;

/// Royalty rate for copper mining - 3% (ມາດຕາ 45)
pub const ROYALTY_RATE_COPPER: f64 = 3.0;

/// Royalty rate for potash mining - 2% (ມາດຕາ 45)
pub const ROYALTY_RATE_POTASH: f64 = 2.0;

/// Royalty rate for gemstones - 10% (ມາດຕາ 45)
pub const ROYALTY_RATE_GEMSTONES: f64 = 10.0;

/// Minimum royalty rate for common minerals - 1% (ມາດຕາ 45)
pub const ROYALTY_RATE_COMMON_MIN: f64 = 1.0;

/// Maximum royalty rate for common minerals - 3% (ມາດຕາ 45)
pub const ROYALTY_RATE_COMMON_MAX: f64 = 3.0;

/// Royalty rate for rare earth elements - 8% (ມາດຕາ 45)
pub const ROYALTY_RATE_RARE_EARTH: f64 = 8.0;

/// Royalty rate for bauxite - 4% (ມາດຕາ 45)
pub const ROYALTY_RATE_BAUXITE: f64 = 4.0;

// === Concession Limits (Articles 30-31) ===

/// Maximum exploration license duration in years (ມາດຕາ 30)
pub const EXPLORATION_LICENSE_MAX_YEARS: u32 = 2;

/// Maximum exploration license renewals (ມາດຕາ 30)
pub const EXPLORATION_LICENSE_MAX_RENEWALS: u32 = 1;

/// Maximum mining concession for strategic minerals in years (ມາດຕາ 31)
pub const MINING_CONCESSION_STRATEGIC_MAX_YEARS: u32 = 30;

/// Minimum mining concession for strategic minerals in years (ມາດຕາ 31)
pub const MINING_CONCESSION_STRATEGIC_MIN_YEARS: u32 = 20;

/// Maximum small-scale mining license in years (ມາດຕາ 42)
pub const SMALL_SCALE_MINING_MAX_YEARS: u32 = 10;

/// Maximum processing license duration in years (ມາດຕາ 34)
pub const PROCESSING_LICENSE_MAX_YEARS: u32 = 20;

// === Area Limits (Article 30) ===

/// Maximum exploration area for strategic minerals in hectares (ມາດຕາ 30)
pub const EXPLORATION_AREA_STRATEGIC_MAX_HECTARES: f64 = 10000.0;

/// Maximum exploration area for common minerals in hectares (ມາດຕາ 30)
pub const EXPLORATION_AREA_COMMON_MAX_HECTARES: f64 = 5000.0;

/// Maximum small-scale mining area in hectares (ມາດຕາ 42)
pub const SMALL_SCALE_MINING_MAX_HECTARES: f64 = 100.0;

/// Maximum artisanal mining area in hectares (ມາດຕາ 43)
pub const ARTISANAL_MINING_MAX_HECTARES: f64 = 5.0;

// === Environmental Requirements (Articles 50-53) ===

/// Minimum distance from protected areas in meters (ມາດຕາ 51)
pub const MIN_DISTANCE_FROM_PROTECTED_AREA_METERS: u32 = 1000;

/// Minimum rehabilitation bond as percentage of project cost (ມາດຕາ 52)
pub const REHABILITATION_BOND_MIN_PERCENT: f64 = 5.0;

// === Foreign Investment Limits (Articles 18-21) ===

/// Maximum foreign ownership for strategic minerals - percent (ມາດຕາ 19)
pub const FOREIGN_OWNERSHIP_STRATEGIC_MAX_PERCENT: f64 = 75.0;

/// Maximum foreign ownership for common minerals - percent (ມາດຕາ 19)
pub const FOREIGN_OWNERSHIP_COMMON_MAX_PERCENT: f64 = 100.0;

/// Minimum local content requirement - percent (ມາດຕາ 20)
pub const LOCAL_CONTENT_MIN_PERCENT: f64 = 30.0;

// === Community Requirements (Articles 37-40) ===

/// Minimum local employment quota - percent (ມາດຕາ 39)
pub const LOCAL_EMPLOYMENT_MIN_PERCENT: f64 = 70.0;

/// Minimum revenue sharing with local community - percent (ມາດຕາ 40)
pub const COMMUNITY_REVENUE_SHARE_MIN_PERCENT: f64 = 1.0;

// ============================================================================
// Mineral Classification (ການຈັດປະເພດແຮ່)
// ============================================================================

/// Mineral classification type (ປະເພດການຈັດປະເພດແຮ່)
///
/// Article 11-13: Classification of minerals for regulatory purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MineralClassification {
    /// Strategic minerals requiring special government approval
    /// ແຮ່ຍຸດທະສາດທີ່ຕ້ອງມີການອະນຸມັດພິເສດຈາກລັດຖະບານ
    Strategic,

    /// Common minerals for general use
    /// ແຮ່ທົ່ວໄປສຳລັບການນຳໃຊ້ທົ່ວໄປ
    Common,

    /// Gemstones with special value considerations
    /// ແກ້ວປະເສີດທີ່ມີການພິຈາລະນາມູນຄ່າພິເສດ
    Gemstone,

    /// Rare earth elements with special environmental requirements
    /// ທາດຫາຍາກທີ່ມີຂໍ້ກຳນົດສິ່ງແວດລ້ອມພິເສດ
    RareEarth,
}

impl MineralClassification {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            MineralClassification::Strategic => "ແຮ່ຍຸດທະສາດ",
            MineralClassification::Common => "ແຮ່ທົ່ວໄປ",
            MineralClassification::Gemstone => "ແກ້ວປະເສີດ",
            MineralClassification::RareEarth => "ທາດຫາຍາກ",
        }
    }

    /// Check if this classification requires government approval (Article 12)
    pub fn requires_government_approval(&self) -> bool {
        matches!(
            self,
            MineralClassification::Strategic | MineralClassification::RareEarth
        )
    }

    /// Check if this classification requires joint venture (Article 18)
    pub fn requires_joint_venture(&self) -> bool {
        matches!(self, MineralClassification::Strategic)
    }

    /// Get the maximum foreign ownership percentage (Article 19)
    pub fn max_foreign_ownership_percent(&self) -> f64 {
        match self {
            MineralClassification::Strategic | MineralClassification::RareEarth => {
                FOREIGN_OWNERSHIP_STRATEGIC_MAX_PERCENT
            }
            MineralClassification::Common | MineralClassification::Gemstone => {
                FOREIGN_OWNERSHIP_COMMON_MAX_PERCENT
            }
        }
    }
}

/// Specific mineral type (ປະເພດແຮ່ສະເພາະ)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MineralType {
    // Strategic minerals (ແຮ່ຍຸດທະສາດ)
    /// Gold (ຄຳ)
    Gold,
    /// Copper (ທອງແດງ)
    Copper,
    /// Potash (ໂປແຕສ)
    Potash,
    /// Bauxite (ບອກໄຊ)
    Bauxite,
    /// Iron ore (ແຮ່ເຫຼັກ)
    IronOre,
    /// Tin (ກົ່ວ)
    Tin,
    /// Lead (ຕະກົ່ວ)
    Lead,
    /// Zinc (ສັງກະສີ)
    Zinc,

    // Common minerals (ແຮ່ທົ່ວໄປ)
    /// Stone (ຫີນ)
    Stone,
    /// Sand (ຊາຍ)
    Sand,
    /// Gravel (ກ້ອນຫີນ)
    Gravel,
    /// Clay (ດິນເໜຍວ)
    Clay,
    /// Limestone (ຫີນປູນ)
    Limestone,
    /// Gypsum (ຍິບສັມ)
    Gypsum,

    // Gemstones (ແກ້ວປະເສີດ)
    /// Sapphires (ນິນ)
    Sapphire,
    /// Rubies (ທັບທິມ)
    Ruby,
    /// Other gemstones (ແກ້ວປະເສີດອື່ນໆ)
    OtherGemstone(String),

    // Rare earth elements (ທາດຫາຍາກ)
    /// Rare earth elements (ທາດຫາຍາກ)
    RareEarthElement(String),

    /// Other mineral type (ແຮ່ປະເພດອື່ນ)
    Other(String),
}

impl MineralType {
    /// Get the classification for this mineral type
    pub fn classification(&self) -> MineralClassification {
        match self {
            MineralType::Gold
            | MineralType::Copper
            | MineralType::Potash
            | MineralType::Bauxite
            | MineralType::IronOre
            | MineralType::Tin
            | MineralType::Lead
            | MineralType::Zinc => MineralClassification::Strategic,
            MineralType::Stone
            | MineralType::Sand
            | MineralType::Gravel
            | MineralType::Clay
            | MineralType::Limestone
            | MineralType::Gypsum
            | MineralType::Other(_) => MineralClassification::Common,
            MineralType::Sapphire | MineralType::Ruby | MineralType::OtherGemstone(_) => {
                MineralClassification::Gemstone
            }
            MineralType::RareEarthElement(_) => MineralClassification::RareEarth,
        }
    }

    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            MineralType::Gold => "ຄຳ",
            MineralType::Copper => "ທອງແດງ",
            MineralType::Potash => "ໂປແຕສ",
            MineralType::Bauxite => "ບອກໄຊ",
            MineralType::IronOre => "ແຮ່ເຫຼັກ",
            MineralType::Tin => "ກົ່ວ",
            MineralType::Lead => "ຕະກົ່ວ",
            MineralType::Zinc => "ສັງກະສີ",
            MineralType::Stone => "ຫີນ",
            MineralType::Sand => "ຊາຍ",
            MineralType::Gravel => "ກ້ອນຫີນ",
            MineralType::Clay => "ດິນເໜຍວ",
            MineralType::Limestone => "ຫີນປູນ",
            MineralType::Gypsum => "ຍິບສັມ",
            MineralType::Sapphire => "ນິນ",
            MineralType::Ruby => "ທັບທິມ",
            MineralType::OtherGemstone(_) => "ແກ້ວປະເສີດອື່ນໆ",
            MineralType::RareEarthElement(_) => "ທາດຫາຍາກ",
            MineralType::Other(_) => "ແຮ່ອື່ນໆ",
        }
    }

    /// Get the royalty rate for this mineral type (Article 45)
    pub fn royalty_rate(&self) -> f64 {
        match self {
            MineralType::Gold => ROYALTY_RATE_GOLD,
            MineralType::Copper => ROYALTY_RATE_COPPER,
            MineralType::Potash => ROYALTY_RATE_POTASH,
            MineralType::Bauxite => ROYALTY_RATE_BAUXITE,
            MineralType::IronOre => 4.0,
            MineralType::Tin | MineralType::Lead | MineralType::Zinc => 3.5,
            MineralType::Stone
            | MineralType::Sand
            | MineralType::Gravel
            | MineralType::Clay
            | MineralType::Limestone
            | MineralType::Gypsum
            | MineralType::Other(_) => ROYALTY_RATE_COMMON_MIN,
            MineralType::Sapphire | MineralType::Ruby | MineralType::OtherGemstone(_) => {
                ROYALTY_RATE_GEMSTONES
            }
            MineralType::RareEarthElement(_) => ROYALTY_RATE_RARE_EARTH,
        }
    }
}

// ============================================================================
// Mining License Types (ປະເພດໃບອະນຸຍາດບໍ່ແຮ່)
// ============================================================================

/// Mining license type (ປະເພດໃບອະນຸຍາດບໍ່ແຮ່)
///
/// Articles 24-27: Types of mining licenses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MiningLicenseType {
    /// Exploration license (ໃບອະນຸຍາດສຳຫຼວດ)
    /// Article 25: For geological survey and mineral exploration
    Exploration,

    /// Mining license (ໃບອະນຸຍາດຂຸດຄົ້ນ)
    /// Article 26: For extraction of minerals
    Mining,

    /// Processing license (ໃບອະນຸຍາດປຸງແຕ່ງ)
    /// Article 27: For mineral processing and refining
    Processing,

    /// Small-scale mining license (ໃບອະນຸຍາດຂຸດຄົ້ນຂະໜາດນ້ອຍ)
    /// Article 42: For small-scale operations
    SmallScale,
}

impl MiningLicenseType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            MiningLicenseType::Exploration => "ໃບອະນຸຍາດສຳຫຼວດ",
            MiningLicenseType::Mining => "ໃບອະນຸຍາດຂຸດຄົ້ນ",
            MiningLicenseType::Processing => "ໃບອະນຸຍາດປຸງແຕ່ງ",
            MiningLicenseType::SmallScale => "ໃບອະນຸຍາດຂຸດຄົ້ນຂະໜາດນ້ອຍ",
        }
    }

    /// Get the maximum duration in years
    pub fn max_duration_years(&self) -> u32 {
        match self {
            MiningLicenseType::Exploration => EXPLORATION_LICENSE_MAX_YEARS,
            MiningLicenseType::Mining => MINING_CONCESSION_STRATEGIC_MAX_YEARS,
            MiningLicenseType::Processing => PROCESSING_LICENSE_MAX_YEARS,
            MiningLicenseType::SmallScale => SMALL_SCALE_MINING_MAX_YEARS,
        }
    }

    /// Check if license requires EIA
    pub fn requires_eia(&self) -> bool {
        matches!(
            self,
            MiningLicenseType::Mining | MiningLicenseType::Processing
        )
    }
}

/// License status (ສະຖານະໃບອະນຸຍາດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LicenseStatus {
    /// Pending approval (ລໍຖ້າການອະນຸມັດ)
    Pending,
    /// Active (ມີຜົນບັງຄັບໃຊ້)
    Active,
    /// Suspended (ໂຈະ)
    Suspended,
    /// Revoked (ຖືກຖອນ)
    Revoked,
    /// Expired (ໝົດອາຍຸ)
    Expired,
    /// Renewed (ຕໍ່ອາຍຸແລ້ວ)
    Renewed,
    /// Under review (ກຳລັງພິຈາລະນາ)
    UnderReview,
}

impl LicenseStatus {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            LicenseStatus::Pending => "ລໍຖ້າການອະນຸມັດ",
            LicenseStatus::Active => "ມີຜົນບັງຄັບໃຊ້",
            LicenseStatus::Suspended => "ໂຈະ",
            LicenseStatus::Revoked => "ຖືກຖອນ",
            LicenseStatus::Expired => "ໝົດອາຍຸ",
            LicenseStatus::Renewed => "ຕໍ່ອາຍຸແລ້ວ",
            LicenseStatus::UnderReview => "ກຳລັງພິຈາລະນາ",
        }
    }

    /// Check if license is valid for operations
    pub fn is_valid(&self) -> bool {
        matches!(self, LicenseStatus::Active | LicenseStatus::Renewed)
    }
}

/// Mining license (ໃບອະນຸຍາດບໍ່ແຮ່)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MiningLicense {
    /// License number (ເລກໃບອະນຸຍາດ)
    pub license_number: String,

    /// License type (ປະເພດໃບອະນຸຍາດ)
    pub license_type: MiningLicenseType,

    /// Holder name (ຊື່ຜູ້ຖື)
    pub holder_name: String,

    /// Holder name in Lao (ຊື່ຜູ້ຖືເປັນພາສາລາວ)
    pub holder_name_lao: Option<String>,

    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,

    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,

    /// Status (ສະຖານະ)
    pub status: LicenseStatus,

    /// Renewals count (ຈຳນວນການຕໍ່ອາຍຸ)
    pub renewals: u32,

    /// Mineral types covered (ປະເພດແຮ່ທີ່ກວມເອົາ)
    pub mineral_types: Vec<MineralType>,

    /// Province (ແຂວງ)
    pub province: String,

    /// District (ເມືອງ)
    pub district: Option<String>,

    /// Issuing authority (ອົງການອອກໃບອະນຸຍາດ)
    pub issuing_authority: String,

    /// Conditions (ເງື່ອນໄຂ)
    pub conditions: Vec<LicenseCondition>,
}

impl Default for MiningLicense {
    fn default() -> Self {
        Self {
            license_number: String::new(),
            license_type: MiningLicenseType::Exploration,
            holder_name: String::new(),
            holder_name_lao: None,
            issue_date: String::new(),
            expiry_date: String::new(),
            status: LicenseStatus::Pending,
            renewals: 0,
            mineral_types: Vec::new(),
            province: String::new(),
            district: None,
            issuing_authority: String::new(),
            conditions: Vec::new(),
        }
    }
}

/// License condition (ເງື່ອນໄຂໃບອະນຸຍາດ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LicenseCondition {
    /// Description (ລາຍລະອຽດ)
    pub description: String,
    /// Description in Lao (ລາຍລະອຽດເປັນພາສາລາວ)
    pub description_lao: Option<String>,
    /// Compliance deadline (ກຳນົດການປະຕິບັດ)
    pub compliance_deadline: Option<String>,
    /// Compliant status (ສະຖານະການປະຕິບັດ)
    pub compliant: bool,
}

/// Builder for MiningLicense
#[derive(Debug, Default)]
pub struct MiningLicenseBuilder {
    license: MiningLicense,
}

impl MiningLicenseBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set license number
    pub fn license_number(mut self, number: impl Into<String>) -> Self {
        self.license.license_number = number.into();
        self
    }

    /// Set license type
    pub fn license_type(mut self, license_type: MiningLicenseType) -> Self {
        self.license.license_type = license_type;
        self
    }

    /// Set holder name
    pub fn holder_name(mut self, name: impl Into<String>) -> Self {
        self.license.holder_name = name.into();
        self
    }

    /// Set holder name in Lao
    pub fn holder_name_lao(mut self, name: impl Into<String>) -> Self {
        self.license.holder_name_lao = Some(name.into());
        self
    }

    /// Set issue date
    pub fn issue_date(mut self, date: impl Into<String>) -> Self {
        self.license.issue_date = date.into();
        self
    }

    /// Set expiry date
    pub fn expiry_date(mut self, date: impl Into<String>) -> Self {
        self.license.expiry_date = date.into();
        self
    }

    /// Set status
    pub fn status(mut self, status: LicenseStatus) -> Self {
        self.license.status = status;
        self
    }

    /// Set renewals count
    pub fn renewals(mut self, renewals: u32) -> Self {
        self.license.renewals = renewals;
        self
    }

    /// Add mineral type
    pub fn add_mineral_type(mut self, mineral: MineralType) -> Self {
        self.license.mineral_types.push(mineral);
        self
    }

    /// Set province
    pub fn province(mut self, province: impl Into<String>) -> Self {
        self.license.province = province.into();
        self
    }

    /// Set district
    pub fn district(mut self, district: impl Into<String>) -> Self {
        self.license.district = Some(district.into());
        self
    }

    /// Set issuing authority
    pub fn issuing_authority(mut self, authority: impl Into<String>) -> Self {
        self.license.issuing_authority = authority.into();
        self
    }

    /// Add condition
    pub fn add_condition(mut self, condition: LicenseCondition) -> Self {
        self.license.conditions.push(condition);
        self
    }

    /// Build the license
    pub fn build(self) -> MiningLicense {
        self.license
    }
}

// ============================================================================
// Concession Framework (ກອບສຳປະທານ)
// ============================================================================

/// Concession type (ປະເພດສຳປະທານ)
///
/// Articles 30-31: Concession framework
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConcessionType {
    /// Exploration concession (ສຳປະທານສຳຫຼວດ)
    Exploration,
    /// Mining concession (ສຳປະທານຂຸດຄົ້ນ)
    Mining,
    /// Small-scale mining (ຂຸດຄົ້ນຂະໜາດນ້ອຍ)
    SmallScale,
    /// Artisanal mining (ຂຸດຄົ້ນຫັດຖະກຳ)
    Artisanal,
}

impl ConcessionType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            ConcessionType::Exploration => "ສຳປະທານສຳຫຼວດ",
            ConcessionType::Mining => "ສຳປະທານຂຸດຄົ້ນ",
            ConcessionType::SmallScale => "ຂຸດຄົ້ນຂະໜາດນ້ອຍ",
            ConcessionType::Artisanal => "ຂຸດຄົ້ນຫັດຖະກຳ",
        }
    }

    /// Get maximum duration for this concession type (years)
    pub fn max_duration_years(&self, mineral_classification: MineralClassification) -> u32 {
        match self {
            ConcessionType::Exploration => EXPLORATION_LICENSE_MAX_YEARS,
            ConcessionType::Mining => match mineral_classification {
                MineralClassification::Strategic | MineralClassification::RareEarth => {
                    MINING_CONCESSION_STRATEGIC_MAX_YEARS
                }
                _ => 25,
            },
            ConcessionType::SmallScale => SMALL_SCALE_MINING_MAX_YEARS,
            ConcessionType::Artisanal => 5,
        }
    }

    /// Get maximum area for this concession type (hectares)
    pub fn max_area_hectares(&self, mineral_classification: MineralClassification) -> f64 {
        match self {
            ConcessionType::Exploration => match mineral_classification {
                MineralClassification::Strategic | MineralClassification::RareEarth => {
                    EXPLORATION_AREA_STRATEGIC_MAX_HECTARES
                }
                _ => EXPLORATION_AREA_COMMON_MAX_HECTARES,
            },
            ConcessionType::Mining => 50000.0, // No strict limit, depends on approval
            ConcessionType::SmallScale => SMALL_SCALE_MINING_MAX_HECTARES,
            ConcessionType::Artisanal => ARTISANAL_MINING_MAX_HECTARES,
        }
    }
}

/// Concession status (ສະຖານະສຳປະທານ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConcessionStatus {
    /// Pending approval (ລໍຖ້າການອະນຸມັດ)
    Pending,
    /// Approved and active (ອະນຸມັດແລ້ວ ແລະ ມີຜົນບັງຄັບໃຊ້)
    Active,
    /// Temporarily suspended (ໂຈະຊົ່ວຄາວ)
    Suspended,
    /// Permanently revoked (ຖືກຖອນ)
    Revoked,
    /// Expired (ໝົດອາຍຸ)
    Expired,
    /// Under development (ກຳລັງພັດທະນາ)
    UnderDevelopment,
    /// In production (ກຳລັງຜະລິດ)
    InProduction,
    /// Closure phase (ໄລຍະປິດ)
    Closure,
    /// Rehabilitated (ຟື້ນຟູແລ້ວ)
    Rehabilitated,
}

impl ConcessionStatus {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            ConcessionStatus::Pending => "ລໍຖ້າການອະນຸມັດ",
            ConcessionStatus::Active => "ມີຜົນບັງຄັບໃຊ້",
            ConcessionStatus::Suspended => "ໂຈະຊົ່ວຄາວ",
            ConcessionStatus::Revoked => "ຖືກຖອນ",
            ConcessionStatus::Expired => "ໝົດອາຍຸ",
            ConcessionStatus::UnderDevelopment => "ກຳລັງພັດທະນາ",
            ConcessionStatus::InProduction => "ກຳລັງຜະລິດ",
            ConcessionStatus::Closure => "ໄລຍະປິດ",
            ConcessionStatus::Rehabilitated => "ຟື້ນຟູແລ້ວ",
        }
    }

    /// Check if concession allows mining operations
    pub fn allows_mining(&self) -> bool {
        matches!(
            self,
            ConcessionStatus::Active
                | ConcessionStatus::UnderDevelopment
                | ConcessionStatus::InProduction
        )
    }
}

/// Mining concession (ສຳປະທານບໍ່ແຮ່)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MiningConcession {
    /// Concession ID (ລະຫັດສຳປະທານ)
    pub concession_id: String,

    /// Concession type (ປະເພດສຳປະທານ)
    pub concession_type: ConcessionType,

    /// Holder name (ຊື່ຜູ້ຖື)
    pub holder_name: String,

    /// Holder name in Lao (ຊື່ຜູ້ຖືເປັນພາສາລາວ)
    pub holder_name_lao: Option<String>,

    /// Primary mineral type (ປະເພດແຮ່ຫຼັກ)
    pub primary_mineral: MineralType,

    /// Secondary mineral types (ປະເພດແຮ່ຮອງ)
    pub secondary_minerals: Vec<MineralType>,

    /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
    pub area_hectares: f64,

    /// Province (ແຂວງ)
    pub province: String,

    /// District (ເມືອງ)
    pub district: Option<String>,

    /// Villages affected (ບ້ານທີ່ໄດ້ຮັບຜົນກະທົບ)
    pub villages: Vec<String>,

    /// Start date (ວັນທີເລີ່ມຕົ້ນ)
    pub start_date: String,

    /// End date (ວັນທີສິ້ນສຸດ)
    pub end_date: String,

    /// Duration in years (ໄລຍະເວລາເປັນປີ)
    pub duration_years: u32,

    /// Status (ສະຖານະ)
    pub status: ConcessionStatus,

    /// EIA approved (EIA ອະນຸມັດແລ້ວ)
    pub eia_approved: bool,

    /// EIA certificate number (ເລກໃບຢັ້ງຢືນ EIA)
    pub eia_certificate_number: Option<String>,

    /// Rehabilitation bond amount in LAK (ເງິນຄ້ຳປະກັນການຟື້ນຟູ)
    pub rehabilitation_bond_lak: u64,

    /// Closure plan submitted (ແຜນປິດສົ່ງແລ້ວ)
    pub closure_plan_submitted: bool,

    /// Foreign ownership percentage (ເປີເຊັນການຖືຫຸ້ນຕ່າງປະເທດ)
    pub foreign_ownership_percent: f64,

    /// Local content percentage (ເປີເຊັນເນື້ອໃນທ້ອງຖິ່ນ)
    pub local_content_percent: f64,

    /// Distance from nearest protected area in meters
    /// (ໄລຍະຫ່າງຈາກເຂດປ່າປ້ອງກັນທີ່ໃກ້ທີ່ສຸດ)
    pub distance_from_protected_area_meters: Option<u32>,
}

impl Default for MiningConcession {
    fn default() -> Self {
        Self {
            concession_id: String::new(),
            concession_type: ConcessionType::Exploration,
            holder_name: String::new(),
            holder_name_lao: None,
            primary_mineral: MineralType::Other(String::new()),
            secondary_minerals: Vec::new(),
            area_hectares: 0.0,
            province: String::new(),
            district: None,
            villages: Vec::new(),
            start_date: String::new(),
            end_date: String::new(),
            duration_years: 0,
            status: ConcessionStatus::Pending,
            eia_approved: false,
            eia_certificate_number: None,
            rehabilitation_bond_lak: 0,
            closure_plan_submitted: false,
            foreign_ownership_percent: 0.0,
            local_content_percent: 0.0,
            distance_from_protected_area_meters: None,
        }
    }
}

/// Builder for MiningConcession
#[derive(Debug, Default)]
pub struct MiningConcessionBuilder {
    concession: MiningConcession,
}

impl MiningConcessionBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set concession ID
    pub fn concession_id(mut self, id: impl Into<String>) -> Self {
        self.concession.concession_id = id.into();
        self
    }

    /// Set concession type
    pub fn concession_type(mut self, concession_type: ConcessionType) -> Self {
        self.concession.concession_type = concession_type;
        self
    }

    /// Set holder name
    pub fn holder_name(mut self, name: impl Into<String>) -> Self {
        self.concession.holder_name = name.into();
        self
    }

    /// Set holder name in Lao
    pub fn holder_name_lao(mut self, name: impl Into<String>) -> Self {
        self.concession.holder_name_lao = Some(name.into());
        self
    }

    /// Set primary mineral
    pub fn primary_mineral(mut self, mineral: MineralType) -> Self {
        self.concession.primary_mineral = mineral;
        self
    }

    /// Add secondary mineral
    pub fn add_secondary_mineral(mut self, mineral: MineralType) -> Self {
        self.concession.secondary_minerals.push(mineral);
        self
    }

    /// Set area in hectares
    pub fn area_hectares(mut self, area: f64) -> Self {
        self.concession.area_hectares = area;
        self
    }

    /// Set province
    pub fn province(mut self, province: impl Into<String>) -> Self {
        self.concession.province = province.into();
        self
    }

    /// Set district
    pub fn district(mut self, district: impl Into<String>) -> Self {
        self.concession.district = Some(district.into());
        self
    }

    /// Add village
    pub fn add_village(mut self, village: impl Into<String>) -> Self {
        self.concession.villages.push(village.into());
        self
    }

    /// Set dates
    pub fn dates(
        mut self,
        start: impl Into<String>,
        end: impl Into<String>,
        duration_years: u32,
    ) -> Self {
        self.concession.start_date = start.into();
        self.concession.end_date = end.into();
        self.concession.duration_years = duration_years;
        self
    }

    /// Set status
    pub fn status(mut self, status: ConcessionStatus) -> Self {
        self.concession.status = status;
        self
    }

    /// Set EIA approval
    pub fn eia_approved(mut self, approved: bool, certificate_number: Option<String>) -> Self {
        self.concession.eia_approved = approved;
        self.concession.eia_certificate_number = certificate_number;
        self
    }

    /// Set rehabilitation bond
    pub fn rehabilitation_bond(mut self, amount_lak: u64) -> Self {
        self.concession.rehabilitation_bond_lak = amount_lak;
        self
    }

    /// Set closure plan submitted
    pub fn closure_plan_submitted(mut self, submitted: bool) -> Self {
        self.concession.closure_plan_submitted = submitted;
        self
    }

    /// Set foreign ownership percentage
    pub fn foreign_ownership_percent(mut self, percent: f64) -> Self {
        self.concession.foreign_ownership_percent = percent;
        self
    }

    /// Set local content percentage
    pub fn local_content_percent(mut self, percent: f64) -> Self {
        self.concession.local_content_percent = percent;
        self
    }

    /// Set distance from protected area
    pub fn distance_from_protected_area(mut self, distance_meters: u32) -> Self {
        self.concession.distance_from_protected_area_meters = Some(distance_meters);
        self
    }

    /// Build the concession
    pub fn build(self) -> MiningConcession {
        self.concession
    }
}

// ============================================================================
// Royalty and Payment (ຄ່າພາກຫຼວງ ແລະ ການຊຳລະ)
// ============================================================================

/// Royalty payment (ການຊຳລະຄ່າພາກຫຼວງ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RoyaltyPayment {
    /// Payment ID (ລະຫັດການຊຳລະ)
    pub payment_id: String,

    /// Concession ID (ລະຫັດສຳປະທານ)
    pub concession_id: String,

    /// Mineral type (ປະເພດແຮ່)
    pub mineral_type: MineralType,

    /// Production volume (ປະລິມານການຜະລິດ)
    pub production_volume: f64,

    /// Production unit (ໜ່ວຍການຜະລິດ)
    pub production_unit: String,

    /// Market value in LAK (ມູນຄ່າຕະຫຼາດ)
    pub market_value_lak: u64,

    /// Royalty rate (ອັດຕາຄ່າພາກຫຼວງ)
    pub royalty_rate_percent: f64,

    /// Royalty amount in LAK (ຈຳນວນຄ່າພາກຫຼວງ)
    pub royalty_amount_lak: u64,

    /// Payment period start (ວັນເລີ່ມໄລຍະເວລາ)
    pub period_start: String,

    /// Payment period end (ວັນສິ້ນສຸດໄລຍະເວລາ)
    pub period_end: String,

    /// Due date (ວັນກຳນົດຊຳລະ)
    pub due_date: String,

    /// Payment date (ວັນທີຊຳລະ)
    pub payment_date: Option<String>,

    /// Payment status (ສະຖານະການຊຳລະ)
    pub status: PaymentStatus,
}

/// Payment status (ສະຖານະການຊຳລະ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PaymentStatus {
    /// Pending (ລໍຖ້າ)
    Pending,
    /// Paid (ຊຳລະແລ້ວ)
    Paid,
    /// Overdue (ຄ້າງຊຳລະ)
    Overdue,
    /// Partial (ຊຳລະບາງສ່ວນ)
    Partial,
    /// Disputed (ມີຂໍ້ຂັດແຍ່ງ)
    Disputed,
}

impl PaymentStatus {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            PaymentStatus::Pending => "ລໍຖ້າ",
            PaymentStatus::Paid => "ຊຳລະແລ້ວ",
            PaymentStatus::Overdue => "ຄ້າງຊຳລະ",
            PaymentStatus::Partial => "ຊຳລະບາງສ່ວນ",
            PaymentStatus::Disputed => "ມີຂໍ້ຂັດແຍ່ງ",
        }
    }
}

// ============================================================================
// Community Rights (ສິດຂອງຊຸມຊົນ)
// ============================================================================

/// Community consultation record (ບັນທຶກການປຶກສາຫາລືກັບຊຸມຊົນ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommunityConsultation {
    /// Consultation ID (ລະຫັດການປຶກສາຫາລື)
    pub consultation_id: String,

    /// Concession ID (ລະຫັດສຳປະທານ)
    pub concession_id: String,

    /// Village name (ຊື່ບ້ານ)
    pub village_name: String,

    /// Village name in Lao (ຊື່ບ້ານເປັນພາສາລາວ)
    pub village_name_lao: String,

    /// District (ເມືອງ)
    pub district: String,

    /// Consultation date (ວັນທີປຶກສາຫາລື)
    pub consultation_date: String,

    /// Participants count (ຈຳນວນຜູ້ເຂົ້າຮ່ວມ)
    pub participants_count: u32,

    /// Households represented (ຄົວເຮືອນທີ່ເປັນຕົວແທນ)
    pub households_represented: u32,

    /// Issues raised (ບັນຫາທີ່ຍົກຂຶ້ນ)
    pub issues_raised: Vec<String>,

    /// Community consent obtained (ໄດ້ຮັບຄວາມເຫັນດີຈາກຊຸມຊົນ)
    pub consent_obtained: bool,

    /// Consent conditions (ເງື່ອນໄຂຄວາມເຫັນດີ)
    pub consent_conditions: Vec<String>,

    /// Documentation available (ມີເອກະສານ)
    pub documentation_available: bool,
}

/// Community compensation (ການຊົດເຊີຍຊຸມຊົນ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommunityCompensation {
    /// Compensation ID (ລະຫັດການຊົດເຊີຍ)
    pub compensation_id: String,

    /// Concession ID (ລະຫັດສຳປະທານ)
    pub concession_id: String,

    /// Beneficiary village (ບ້ານທີ່ໄດ້ຮັບຜົນປະໂຫຍດ)
    pub beneficiary_village: String,

    /// Compensation type (ປະເພດການຊົດເຊີຍ)
    pub compensation_type: CompensationType,

    /// Amount in LAK (ຈຳນວນເງິນ)
    pub amount_lak: u64,

    /// Description (ລາຍລະອຽດ)
    pub description: String,

    /// Payment date (ວັນທີຊຳລະ)
    pub payment_date: Option<String>,

    /// Status (ສະຖານະ)
    pub status: PaymentStatus,
}

/// Compensation type (ປະເພດການຊົດເຊີຍ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CompensationType {
    /// Land compensation (ຊົດເຊີຍທີ່ດິນ)
    LandCompensation,
    /// Crop compensation (ຊົດເຊີຍພືດ)
    CropCompensation,
    /// Resettlement support (ສະໜັບສະໜູນການຍົກຍ້າຍ)
    ResettlementSupport,
    /// Livelihood restoration (ການຟື້ນຟູການດຳລົງຊີວິດ)
    LivelihoodRestoration,
    /// Infrastructure support (ສະໜັບສະໜູນໂຄງສ້າງພື້ນຖານ)
    InfrastructureSupport,
    /// Revenue sharing (ການແບ່ງລາຍຮັບ)
    RevenueSharing,
    /// Other (ອື່ນໆ)
    Other,
}

impl CompensationType {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            CompensationType::LandCompensation => "ຊົດເຊີຍທີ່ດິນ",
            CompensationType::CropCompensation => "ຊົດເຊີຍພືດ",
            CompensationType::ResettlementSupport => "ສະໜັບສະໜູນການຍົກຍ້າຍ",
            CompensationType::LivelihoodRestoration => "ການຟື້ນຟູການດຳລົງຊີວິດ",
            CompensationType::InfrastructureSupport => "ສະໜັບສະໜູນໂຄງສ້າງພື້ນຖານ",
            CompensationType::RevenueSharing => "ການແບ່ງລາຍຮັບ",
            CompensationType::Other => "ອື່ນໆ",
        }
    }
}

/// Local employment record (ບັນທຶກການຈ້າງງານທ້ອງຖິ່ນ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LocalEmployment {
    /// Record ID (ລະຫັດບັນທຶກ)
    pub record_id: String,

    /// Concession ID (ລະຫັດສຳປະທານ)
    pub concession_id: String,

    /// Reporting period (ໄລຍະເວລາລາຍງານ)
    pub reporting_period: String,

    /// Total employees (ພະນັກງານທັງໝົດ)
    pub total_employees: u32,

    /// Local employees (ພະນັກງານທ້ອງຖິ່ນ)
    pub local_employees: u32,

    /// Local percentage (ເປີເຊັນທ້ອງຖິ່ນ)
    pub local_percentage: f64,

    /// Lao national employees (ພະນັກງານຊາວລາວ)
    pub lao_national_employees: u32,

    /// Foreign employees (ພະນັກງານຕ່າງປະເທດ)
    pub foreign_employees: u32,

    /// Skill positions filled by locals (ຕຳແໜ່ງຊ່ຽວຊານທີ່ຈ້າງຄົນທ້ອງຖິ່ນ)
    pub skilled_positions_local: u32,

    /// Total skilled positions (ຕຳແໜ່ງຊ່ຽວຊານທັງໝົດ)
    pub skilled_positions_total: u32,
}

// ============================================================================
// Foreign Investment (ການລົງທຶນຕ່າງປະເທດ)
// ============================================================================

/// Foreign investment structure (ໂຄງສ້າງການລົງທຶນຕ່າງປະເທດ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForeignInvestment {
    /// Investment ID (ລະຫັດການລົງທຶນ)
    pub investment_id: String,

    /// Concession ID (ລະຫັດສຳປະທານ)
    pub concession_id: String,

    /// Foreign investor name (ຊື່ນັກລົງທຶນຕ່າງປະເທດ)
    pub foreign_investor_name: String,

    /// Foreign investor country (ປະເທດນັກລົງທຶນຕ່າງປະເທດ)
    pub foreign_investor_country: String,

    /// Lao partner name (ຊື່ຄູ່ຮ່ວມລາວ)
    pub lao_partner_name: Option<String>,

    /// Foreign ownership percentage (ເປີເຊັນການຖືຫຸ້ນຕ່າງປະເທດ)
    pub foreign_ownership_percent: f64,

    /// Lao ownership percentage (ເປີເຊັນການຖືຫຸ້ນລາວ)
    pub lao_ownership_percent: f64,

    /// Total investment in USD (ການລົງທຶນທັງໝົດເປັນ USD)
    pub total_investment_usd: u64,

    /// Joint venture agreement date (ວັນທີສັນຍາຮ່ວມທຶນ)
    pub joint_venture_date: Option<String>,

    /// Technology transfer commitments (ຄຳໝັ້ນສັນຍາການຖ່າຍທອດເຕັກໂນໂລຊີ)
    pub technology_transfer_commitments: Vec<TechnologyTransfer>,

    /// Local content commitments (ຄຳໝັ້ນສັນຍາເນື້ອໃນທ້ອງຖິ່ນ)
    pub local_content_percent_commitment: f64,
}

/// Technology transfer commitment (ຄຳໝັ້ນສັນຍາການຖ່າຍທອດເຕັກໂນໂລຊີ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TechnologyTransfer {
    /// Description (ລາຍລະອຽດ)
    pub description: String,

    /// Target date (ວັນເປົ້າໝາຍ)
    pub target_date: String,

    /// Completed (ສຳເລັດແລ້ວ)
    pub completed: bool,

    /// Beneficiaries count (ຈຳນວນຜູ້ໄດ້ຮັບຜົນປະໂຫຍດ)
    pub beneficiaries_count: Option<u32>,
}

// ============================================================================
// Environmental Compliance (ການປະຕິບັດຕາມສິ່ງແວດລ້ອມ)
// ============================================================================

/// Mining environmental compliance (ການປະຕິບັດຕາມສິ່ງແວດລ້ອມບໍ່ແຮ່)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MiningEnvironmentalCompliance {
    /// Concession ID (ລະຫັດສຳປະທານ)
    pub concession_id: String,

    /// EIA approved (EIA ອະນຸມັດແລ້ວ)
    pub eia_approved: bool,

    /// EIA certificate number (ເລກໃບຢັ້ງຢືນ EIA)
    pub eia_certificate_number: Option<String>,

    /// EIA expiry date (ວັນໝົດອາຍຸ EIA)
    pub eia_expiry_date: Option<String>,

    /// Rehabilitation bond amount LAK (ເງິນຄ້ຳປະກັນການຟື້ນຟູ)
    pub rehabilitation_bond_lak: u64,

    /// Required rehabilitation bond LAK (ເງິນຄ້ຳປະກັນທີ່ຕ້ອງການ)
    pub required_rehabilitation_bond_lak: u64,

    /// Closure plan submitted (ແຜນປິດສົ່ງແລ້ວ)
    pub closure_plan_submitted: bool,

    /// Closure plan approved (ແຜນປິດອະນຸມັດແລ້ວ)
    pub closure_plan_approved: bool,

    /// Environmental monitoring reports (ບົດລາຍງານການຕິດຕາມສິ່ງແວດລ້ອມ)
    pub monitoring_reports_submitted: u32,

    /// Last monitoring report date (ວັນທີບົດລາຍງານຫຼ້າສຸດ)
    pub last_monitoring_report_date: Option<String>,

    /// Environmental violations (ການລະເມີດສິ່ງແວດລ້ອມ)
    pub environmental_violations: Vec<EnvironmentalViolation>,
}

/// Environmental violation record (ບັນທຶກການລະເມີດສິ່ງແວດລ້ອມ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnvironmentalViolation {
    /// Violation ID (ລະຫັດການລະເມີດ)
    pub violation_id: String,

    /// Violation date (ວັນທີລະເມີດ)
    pub violation_date: String,

    /// Description (ລາຍລະອຽດ)
    pub description: String,

    /// Severity (ຄວາມຮຸນແຮງ)
    pub severity: ViolationSeverity,

    /// Fine amount LAK (ຈຳນວນຄ່າປັບ)
    pub fine_amount_lak: Option<u64>,

    /// Corrective action required (ຕ້ອງການການແກ້ໄຂ)
    pub corrective_action_required: String,

    /// Corrective action completed (ການແກ້ໄຂສຳເລັດແລ້ວ)
    pub corrective_action_completed: bool,

    /// Completion date (ວັນທີສຳເລັດ)
    pub completion_date: Option<String>,
}

/// Violation severity (ຄວາມຮຸນແຮງການລະເມີດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ViolationSeverity {
    /// Minor (ເລັກນ້ອຍ)
    Minor,
    /// Moderate (ປານກາງ)
    Moderate,
    /// Major (ໃຫຍ່)
    Major,
    /// Critical (ຮ້າຍແຮງ)
    Critical,
}

impl ViolationSeverity {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            ViolationSeverity::Minor => "ເລັກນ້ອຍ",
            ViolationSeverity::Moderate => "ປານກາງ",
            ViolationSeverity::Major => "ໃຫຍ່",
            ViolationSeverity::Critical => "ຮ້າຍແຮງ",
        }
    }
}
