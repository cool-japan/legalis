//! Forestry Law Types (ປະເພດກົດໝາຍປ່າໄມ້)
//!
//! Type definitions for Lao forestry law based on:
//! - **Forestry Law 2019** (Law No. 64/NA, dated June 13, 2019)
//! - National Assembly of the Lao PDR
//!
//! # Legal References
//! - Forestry Law 2019 (Law No. 64/NA) - ກົດໝາຍວ່າດ້ວຍປ່າໄມ້ ປີ 2019
//! - Forest Classification Decree - ດຳລັດວ່າດ້ວຍການຈັດປະເພດປ່າໄມ້
//! - Timber Harvesting Regulations - ລະບຽບການຕັດໄມ້
//! - Protected Species List - ບັນຊີຊະນິດພັນທີ່ຖືກປົກປ້ອງ
//!
//! # Bilingual Support
//! All types include both Lao (ລາວ) and English field names where applicable.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Forestry Law 2019 (ກົດໝາຍປ່າໄມ້ ປີ 2019)
// ============================================================================

/// Harvesting season start month (November) - Article 48
/// ເດືອນເລີ່ມຕົ້ນລະດູຕັດໄມ້ (ພະຈິກ)
pub const HARVESTING_SEASON_START_MONTH: u8 = 11;

/// Harvesting season end month (April) - Article 48
/// ເດືອນສິ້ນສຸດລະດູຕັດໄມ້ (ເມສາ)
pub const HARVESTING_SEASON_END_MONTH: u8 = 4;

/// Minimum diameter for teak (cm) - Article 49
/// ເສັ້ນຜ່ານສູນກາງຂັ້ນຕ່ຳສຳລັບໄມ້ສັກ (ຊັງຕີແມັດ)
pub const MIN_DIAMETER_TEAK_CM: u32 = 40;

/// Minimum diameter for rosewood (cm) - Article 49
/// ເສັ້ນຜ່ານສູນກາງຂັ້ນຕ່ຳສຳລັບໄມ້ກໍ່ຫຼວງ (ຊັງຕີແມັດ)
pub const MIN_DIAMETER_ROSEWOOD_CM: u32 = 30;

/// Minimum diameter for other hardwoods (cm) - Article 49
/// ເສັ້ນຜ່ານສູນກາງຂັ້ນຕ່ຳສຳລັບໄມ້ແຂງອື່ນໆ (ຊັງຕີແມັດ)
pub const MIN_DIAMETER_HARDWOOD_CM: u32 = 35;

/// Maximum forest management concession term (years) - Article 62
/// ໄລຍະສູງສຸດສຳປະທານຄຸ້ມຄອງປ່າໄມ້ (ປີ)
pub const MAX_MANAGEMENT_CONCESSION_YEARS: u32 = 40;

/// Maximum forest plantation concession term (years) - Article 63
/// ໄລຍະສູງສຸດສຳປະທານປູກປ່າ (ປີ)
pub const MAX_PLANTATION_CONCESSION_YEARS: u32 = 50;

/// Maximum forest management concession area (hectares) - Article 62
/// ເນື້ອທີ່ສູງສຸດສຳປະທານຄຸ້ມຄອງປ່າໄມ້ (ເຮັກຕາ)
pub const MAX_MANAGEMENT_CONCESSION_HECTARES: f64 = 10_000.0;

/// Maximum forest plantation concession area (hectares) - Article 63
/// ເນື້ອທີ່ສູງສຸດສຳປະທານປູກປ່າ (ເຮັກຕາ)
pub const MAX_PLANTATION_CONCESSION_HECTARES: f64 = 15_000.0;

/// Performance bond percentage for management concession - Article 62
/// ອັດຕາເງິນຄ້ຳປະກັນສຳລັບສຳປະທານຄຸ້ມຄອງ (%)
pub const MANAGEMENT_CONCESSION_BOND_PERCENT: f64 = 5.0;

/// Performance bond percentage for plantation concession - Article 63
/// ອັດຕາເງິນຄ້ຳປະກັນສຳລັບສຳປະທານປູກປ່າ (%)
pub const PLANTATION_CONCESSION_BOND_PERCENT: f64 = 3.0;

/// Village benefit share percentage - Article 94
/// ສ່ວນແບ່ງຜົນປະໂຫຍດຂອງບ້ານ (%)
pub const VILLAGE_BENEFIT_SHARE_PERCENT: f64 = 50.0;

/// District benefit share percentage - Article 94
/// ສ່ວນແບ່ງຜົນປະໂຫຍດຂອງເມືອງ (%)
pub const DISTRICT_BENEFIT_SHARE_PERCENT: f64 = 30.0;

/// National benefit share percentage - Article 94
/// ສ່ວນແບ່ງຜົນປະໂຫຍດຂອງລັດ (%)
pub const NATIONAL_BENEFIT_SHARE_PERCENT: f64 = 20.0;

/// Transport permit validity (days) - Article 122
/// ໄລຍະໃຊ້ງານໃບອະນຸຍາດຂົນສົ່ງ (ມື້)
pub const TRANSPORT_PERMIT_VALIDITY_DAYS: u32 = 30;

/// Reforestation maintenance obligation (years) - Article 110
/// ພັນທະບຳລຸງຮັກສາການປູກປ່າ (ປີ)
pub const REFORESTATION_MAINTENANCE_YEARS: u32 = 5;

/// Illegal logging fine multiplier minimum - Article 107
/// ຕົວຄູນຄ່າປັບໄໝການຕັດໄມ້ຜິດກົດໝາຍຂັ້ນຕ່ຳ
pub const ILLEGAL_LOGGING_FINE_MULTIPLIER_MIN: f64 = 2.0;

/// Illegal logging fine multiplier maximum - Article 107
/// ຕົວຄູນຄ່າປັບໄໝການຕັດໄມ້ຜິດກົດໝາຍຂັ້ນສູງ
pub const ILLEGAL_LOGGING_FINE_MULTIPLIER_MAX: f64 = 10.0;

/// Wildlife trafficking fine multiplier minimum - Article 108
/// ຕົວຄູນຄ່າປັບໄໝການຄ້າສັດປ່າຜິດກົດໝາຍຂັ້ນຕ່ຳ
pub const WILDLIFE_TRAFFICKING_FINE_MULTIPLIER_MIN: f64 = 5.0;

/// Wildlife trafficking fine multiplier maximum - Article 108
/// ຕົວຄູນຄ່າປັບໄໝການຄ້າສັດປ່າຜິດກົດໝາຍຂັ້ນສູງ
pub const WILDLIFE_TRAFFICKING_FINE_MULTIPLIER_MAX: f64 = 20.0;

// ============================================================================
// Forest Classification (ການຈັດປະເພດປ່າໄມ້)
// ============================================================================

/// Forest classification type (ປະເພດການຈັດແບ່ງປ່າໄມ້)
///
/// Articles 10-15: Forest categories under Lao law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ForestClassification {
    /// Protection Forest (ປ່າປ້ອງກັນ) - Article 11
    /// Watershed protection, erosion control, national security
    Protection,

    /// Conservation Forest (ປ່າສະຫງວນ) - Article 12
    /// Biodiversity conservation, wildlife habitat, protected areas
    Conservation,

    /// Production Forest (ປ່າຜະລິດ) - Article 13
    /// Sustainable timber harvesting, commercial utilization
    Production,

    /// Rehabilitation Forest (ປ່າຟື້ນຟູ) - Article 14
    /// Degraded forest restoration, reforestation projects
    Rehabilitation,

    /// Village Forest (ປ່າບ້ານ) - Article 15
    /// Community management, traditional use, local benefits
    Village,
}

impl ForestClassification {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            ForestClassification::Protection => "ປ່າປ້ອງກັນ",
            ForestClassification::Conservation => "ປ່າສະຫງວນ",
            ForestClassification::Production => "ປ່າຜະລິດ",
            ForestClassification::Rehabilitation => "ປ່າຟື້ນຟູ",
            ForestClassification::Village => "ປ່າບ້ານ",
        }
    }

    /// Get the English name
    pub fn english_name(&self) -> &'static str {
        match self {
            ForestClassification::Protection => "Protection Forest",
            ForestClassification::Conservation => "Conservation Forest",
            ForestClassification::Production => "Production Forest",
            ForestClassification::Rehabilitation => "Rehabilitation Forest",
            ForestClassification::Village => "Village Forest",
        }
    }

    /// Check if commercial harvesting is allowed
    pub fn allows_commercial_harvesting(&self) -> bool {
        matches!(self, ForestClassification::Production)
    }

    /// Check if limited harvesting is allowed
    pub fn allows_limited_harvesting(&self) -> bool {
        matches!(
            self,
            ForestClassification::Production | ForestClassification::Village
        )
    }

    /// Get the article number in Forestry Law 2019
    pub fn article_number(&self) -> u32 {
        match self {
            ForestClassification::Protection => 11,
            ForestClassification::Conservation => 12,
            ForestClassification::Production => 13,
            ForestClassification::Rehabilitation => 14,
            ForestClassification::Village => 15,
        }
    }
}

// ============================================================================
// Tree Species (ຊະນິດໄມ້)
// ============================================================================

/// Tree species type (ປະເພດຊະນິດໄມ້)
///
/// Articles 49-50, 76-80: Species classification and protection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TreeSpecies {
    /// Teak (ໄມ້ສັກ) - Tectona grandis
    Teak,

    /// Rosewood (ໄມ້ກໍ່ຫຼວງ/ໄມ້ແດງ) - Dalbergia spp.
    Rosewood,

    /// Agarwood (ໄມ້ກຳລັງ) - Aquilaria spp.
    Agarwood,

    /// Mai Dou (ໄມ້ດູ່) - Pterocarpus macrocarpus
    MaiDou,

    /// Mai Kha (ໄມ້ຄາ) - Shorea spp.
    MaiKha,

    /// Mai Nyeng (ໄມ້ຍາງ) - Dipterocarpus spp.
    MaiNyeng,

    /// Mai Tae (ໄມ້ແຕ້) - Bamboo species
    Bamboo,

    /// Mai Pao (ໄມ້ປໍ) - Various species
    MaiPao,

    /// Pine (ໄມ້ແປກ) - Pinus spp.
    Pine,

    /// Eucalyptus (ໄມ້ຢູຄາລິບຕັດ) - Eucalyptus spp.
    Eucalyptus,

    /// Acacia (ໄມ້ອາເຄເຊຍ) - Acacia spp.
    Acacia,

    /// Rubber (ໄມ້ຢາງພາລາ) - Hevea brasiliensis
    Rubber,

    /// Other hardwood (ໄມ້ແຂງອື່ນໆ)
    OtherHardwood,

    /// Other softwood (ໄມ້ອ່ອນອື່ນໆ)
    OtherSoftwood,
}

impl TreeSpecies {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            TreeSpecies::Teak => "ໄມ້ສັກ",
            TreeSpecies::Rosewood => "ໄມ້ກໍ່ຫຼວງ",
            TreeSpecies::Agarwood => "ໄມ້ກຳລັງ",
            TreeSpecies::MaiDou => "ໄມ້ດູ່",
            TreeSpecies::MaiKha => "ໄມ້ຄາ",
            TreeSpecies::MaiNyeng => "ໄມ້ຍາງ",
            TreeSpecies::Bamboo => "ໄມ້ແຕ້",
            TreeSpecies::MaiPao => "ໄມ້ປໍ",
            TreeSpecies::Pine => "ໄມ້ແປກ",
            TreeSpecies::Eucalyptus => "ໄມ້ຢູຄາລິບຕັດ",
            TreeSpecies::Acacia => "ໄມ້ອາເຄເຊຍ",
            TreeSpecies::Rubber => "ໄມ້ຢາງພາລາ",
            TreeSpecies::OtherHardwood => "ໄມ້ແຂງອື່ນໆ",
            TreeSpecies::OtherSoftwood => "ໄມ້ອ່ອນອື່ນໆ",
        }
    }

    /// Get the scientific name
    pub fn scientific_name(&self) -> &'static str {
        match self {
            TreeSpecies::Teak => "Tectona grandis",
            TreeSpecies::Rosewood => "Dalbergia spp.",
            TreeSpecies::Agarwood => "Aquilaria spp.",
            TreeSpecies::MaiDou => "Pterocarpus macrocarpus",
            TreeSpecies::MaiKha => "Shorea spp.",
            TreeSpecies::MaiNyeng => "Dipterocarpus spp.",
            TreeSpecies::Bamboo => "Bambusa spp.",
            TreeSpecies::MaiPao => "Various spp.",
            TreeSpecies::Pine => "Pinus spp.",
            TreeSpecies::Eucalyptus => "Eucalyptus spp.",
            TreeSpecies::Acacia => "Acacia spp.",
            TreeSpecies::Rubber => "Hevea brasiliensis",
            TreeSpecies::OtherHardwood => "Various hardwood spp.",
            TreeSpecies::OtherSoftwood => "Various softwood spp.",
        }
    }

    /// Get protection category (Articles 77-79)
    pub fn protection_category(&self) -> ProtectionCategory {
        match self {
            TreeSpecies::Rosewood | TreeSpecies::Agarwood => ProtectionCategory::CategoryI,
            TreeSpecies::Teak | TreeSpecies::MaiDou => ProtectionCategory::CategoryII,
            _ => ProtectionCategory::CategoryIII,
        }
    }

    /// Get minimum cutting diameter in cm (Article 49)
    pub fn minimum_diameter_cm(&self) -> u32 {
        match self {
            TreeSpecies::Teak => MIN_DIAMETER_TEAK_CM,
            TreeSpecies::Rosewood => MIN_DIAMETER_ROSEWOOD_CM,
            TreeSpecies::MaiDou | TreeSpecies::MaiKha | TreeSpecies::MaiNyeng => {
                MIN_DIAMETER_HARDWOOD_CM
            }
            TreeSpecies::Agarwood => 25, // Special case
            TreeSpecies::Bamboo => 5,
            _ => MIN_DIAMETER_HARDWOOD_CM,
        }
    }

    /// Check if CITES listed
    pub fn is_cites_listed(&self) -> bool {
        matches!(
            self,
            TreeSpecies::Rosewood | TreeSpecies::Agarwood | TreeSpecies::MaiDou
        )
    }

    /// Get CITES appendix if listed
    pub fn cites_appendix(&self) -> Option<&'static str> {
        match self {
            TreeSpecies::Rosewood => Some("II"),
            TreeSpecies::Agarwood => Some("II"),
            TreeSpecies::MaiDou => Some("II"),
            _ => None,
        }
    }
}

/// Species protection category (ປະເພດການປົກປ້ອງຊະນິດພັນ)
///
/// Articles 77-79: Species classification for protection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProtectionCategory {
    /// Category I: Strictly protected - no harvest (ປະເພດ I: ຫ້າມຕັດເດັດຂາດ)
    CategoryI,

    /// Category II: Managed species - quota required (ປະເພດ II: ຕ້ອງມີໂກຕ້າ)
    CategoryII,

    /// Category III: Common species - standard permit (ປະເພດ III: ໃບອະນຸຍາດທົ່ວໄປ)
    CategoryIII,
}

impl ProtectionCategory {
    /// Get the Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ProtectionCategory::CategoryI => "ປະເພດ I - ຫ້າມຕັດເດັດຂາດ",
            ProtectionCategory::CategoryII => "ປະເພດ II - ຕ້ອງມີໂກຕ້າ",
            ProtectionCategory::CategoryIII => "ປະເພດ III - ໃບອະນຸຍາດທົ່ວໄປ",
        }
    }

    /// Check if harvesting is allowed
    pub fn allows_harvesting(&self) -> bool {
        !matches!(self, ProtectionCategory::CategoryI)
    }

    /// Check if quota is required
    pub fn requires_quota(&self) -> bool {
        matches!(self, ProtectionCategory::CategoryII)
    }
}

// ============================================================================
// Timber Harvesting Permit (ໃບອະນຸຍາດຕັດໄມ້)
// ============================================================================

/// Timber harvesting permit (ໃບອະນຸຍາດຕັດໄມ້)
///
/// Article 32: Timber harvesting permit requirements
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TimberHarvestingPermit {
    /// Permit number (ເລກໃບອະນຸຍາດ)
    pub permit_number: String,

    /// Holder name in English (ຊື່ຜູ້ຖືໃບອະນຸຍາດ)
    pub holder_name: String,

    /// Holder name in Lao (ຊື່ຜູ້ຖືໃບອະນຸຍາດເປັນພາສາລາວ)
    pub holder_name_lao: Option<String>,

    /// Forest classification (ປະເພດປ່າ)
    pub forest_type: ForestClassification,

    /// Province (ແຂວງ)
    pub province: String,

    /// District (ເມືອງ)
    pub district: String,

    /// Village (ບ້ານ)
    pub village: Option<String>,

    /// Tree species (ຊະນິດໄມ້)
    pub species: TreeSpecies,

    /// Volume in cubic meters (ປະລິມານເປັນແມັດກ້ອນ)
    pub volume_cubic_meters: f64,

    /// Number of trees (ຈຳນວນຕົ້ນໄມ້)
    pub tree_count: Option<u32>,

    /// Harvesting month (ເດືອນຕັດໄມ້)
    pub harvesting_month: u8,

    /// Harvesting year (ປີຕັດໄມ້)
    pub harvesting_year: u32,

    /// Minimum diameter in cm (ເສັ້ນຜ່ານສູນກາງຂັ້ນຕ່ຳ)
    pub minimum_diameter_cm: u32,

    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,

    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,

    /// Issuing authority (ອົງການອອກໃບອະນຸຍາດ)
    pub issuing_authority: String,

    /// Annual Allowable Cut allocation (ໂກຕ້າ AAC)
    pub aac_allocation: Option<f64>,

    /// Quota reference number (ເລກອ້າງອິງໂກຕ້າ)
    pub quota_reference: Option<String>,

    /// Permit status (ສະຖານະໃບອະນຸຍາດ)
    pub status: PermitStatus,

    /// Reforestation requirement (ຂໍ້ກຳນົດການປູກປ່າ)
    pub reforestation_required: bool,

    /// Reforestation area in hectares (ເນື້ອທີ່ປູກປ່າ)
    pub reforestation_area_hectares: Option<f64>,
}

impl Default for TimberHarvestingPermit {
    fn default() -> Self {
        Self {
            permit_number: String::new(),
            holder_name: String::new(),
            holder_name_lao: None,
            forest_type: ForestClassification::Production,
            province: String::new(),
            district: String::new(),
            village: None,
            species: TreeSpecies::OtherHardwood,
            volume_cubic_meters: 0.0,
            tree_count: None,
            harvesting_month: HARVESTING_SEASON_START_MONTH,
            harvesting_year: 2026,
            minimum_diameter_cm: MIN_DIAMETER_HARDWOOD_CM,
            issue_date: String::new(),
            expiry_date: String::new(),
            issuing_authority: String::new(),
            aac_allocation: None,
            quota_reference: None,
            status: PermitStatus::Pending,
            reforestation_required: true,
            reforestation_area_hectares: None,
        }
    }
}

/// Builder for TimberHarvestingPermit
#[derive(Debug, Default)]
pub struct TimberHarvestingPermitBuilder {
    permit: TimberHarvestingPermit,
}

impl TimberHarvestingPermitBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set permit number
    pub fn permit_number(mut self, number: impl Into<String>) -> Self {
        self.permit.permit_number = number.into();
        self
    }

    /// Set holder name
    pub fn holder_name(mut self, name: impl Into<String>) -> Self {
        self.permit.holder_name = name.into();
        self
    }

    /// Set holder name in Lao
    pub fn holder_name_lao(mut self, name: impl Into<String>) -> Self {
        self.permit.holder_name_lao = Some(name.into());
        self
    }

    /// Set forest type
    pub fn forest_type(mut self, forest_type: ForestClassification) -> Self {
        self.permit.forest_type = forest_type;
        self
    }

    /// Set province
    pub fn province(mut self, province: impl Into<String>) -> Self {
        self.permit.province = province.into();
        self
    }

    /// Set district
    pub fn district(mut self, district: impl Into<String>) -> Self {
        self.permit.district = district.into();
        self
    }

    /// Set village
    pub fn village(mut self, village: impl Into<String>) -> Self {
        self.permit.village = Some(village.into());
        self
    }

    /// Set species
    pub fn species(mut self, species: TreeSpecies) -> Self {
        self.permit.species = species;
        self.permit.minimum_diameter_cm = species.minimum_diameter_cm();
        self
    }

    /// Set volume in cubic meters
    pub fn volume_cubic_meters(mut self, volume: f64) -> Self {
        self.permit.volume_cubic_meters = volume;
        self
    }

    /// Set tree count
    pub fn tree_count(mut self, count: u32) -> Self {
        self.permit.tree_count = Some(count);
        self
    }

    /// Set harvesting month
    pub fn harvesting_month(mut self, month: u8) -> Self {
        self.permit.harvesting_month = month;
        self
    }

    /// Set harvesting year
    pub fn harvesting_year(mut self, year: u32) -> Self {
        self.permit.harvesting_year = year;
        self
    }

    /// Set minimum diameter
    pub fn minimum_diameter_cm(mut self, diameter: u32) -> Self {
        self.permit.minimum_diameter_cm = diameter;
        self
    }

    /// Set issue date
    pub fn issue_date(mut self, date: impl Into<String>) -> Self {
        self.permit.issue_date = date.into();
        self
    }

    /// Set expiry date
    pub fn expiry_date(mut self, date: impl Into<String>) -> Self {
        self.permit.expiry_date = date.into();
        self
    }

    /// Set issuing authority
    pub fn issuing_authority(mut self, authority: impl Into<String>) -> Self {
        self.permit.issuing_authority = authority.into();
        self
    }

    /// Set AAC allocation
    pub fn aac_allocation(mut self, allocation: f64) -> Self {
        self.permit.aac_allocation = Some(allocation);
        self
    }

    /// Set quota reference
    pub fn quota_reference(mut self, reference: impl Into<String>) -> Self {
        self.permit.quota_reference = Some(reference.into());
        self
    }

    /// Set status
    pub fn status(mut self, status: PermitStatus) -> Self {
        self.permit.status = status;
        self
    }

    /// Set reforestation required
    pub fn reforestation_required(mut self, required: bool) -> Self {
        self.permit.reforestation_required = required;
        self
    }

    /// Set reforestation area
    pub fn reforestation_area_hectares(mut self, area: f64) -> Self {
        self.permit.reforestation_area_hectares = Some(area);
        self
    }

    /// Build the permit
    pub fn build(self) -> TimberHarvestingPermit {
        self.permit
    }
}

/// Permit status (ສະຖານະໃບອະນຸຍາດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PermitStatus {
    /// Pending (ລໍຖ້າ)
    Pending,
    /// Active (ມີຜົນບັງຄັບໃຊ້)
    Active,
    /// Expired (ໝົດອາຍຸ)
    Expired,
    /// Suspended (ໂຈະ)
    Suspended,
    /// Revoked (ຖືກຖອນ)
    Revoked,
    /// Completed (ສຳເລັດ)
    Completed,
}

impl PermitStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            PermitStatus::Pending => "ລໍຖ້າ",
            PermitStatus::Active => "ມີຜົນບັງຄັບໃຊ້",
            PermitStatus::Expired => "ໝົດອາຍຸ",
            PermitStatus::Suspended => "ໂຈະ",
            PermitStatus::Revoked => "ຖືກຖອນ",
            PermitStatus::Completed => "ສຳເລັດ",
        }
    }
}

// ============================================================================
// Forest Concession (ສຳປະທານປ່າໄມ້)
// ============================================================================

/// Concession type (ປະເພດສຳປະທານ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConcessionType {
    /// Forest management concession (ສຳປະທານຄຸ້ມຄອງປ່າໄມ້) - Article 62
    Management,
    /// Forest plantation concession (ສຳປະທານປູກປ່າ) - Article 63
    Plantation,
}

impl ConcessionType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ConcessionType::Management => "ສຳປະທານຄຸ້ມຄອງປ່າໄມ້",
            ConcessionType::Plantation => "ສຳປະທານປູກປ່າ",
        }
    }

    /// Get maximum term in years
    pub fn max_term_years(&self) -> u32 {
        match self {
            ConcessionType::Management => MAX_MANAGEMENT_CONCESSION_YEARS,
            ConcessionType::Plantation => MAX_PLANTATION_CONCESSION_YEARS,
        }
    }

    /// Get maximum area in hectares
    pub fn max_area_hectares(&self) -> f64 {
        match self {
            ConcessionType::Management => MAX_MANAGEMENT_CONCESSION_HECTARES,
            ConcessionType::Plantation => MAX_PLANTATION_CONCESSION_HECTARES,
        }
    }

    /// Get required bond percentage
    pub fn bond_percentage(&self) -> f64 {
        match self {
            ConcessionType::Management => MANAGEMENT_CONCESSION_BOND_PERCENT,
            ConcessionType::Plantation => PLANTATION_CONCESSION_BOND_PERCENT,
        }
    }
}

/// Concession status (ສະຖານະສຳປະທານ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConcessionStatus {
    /// Application pending (ຄຳຮ້ອງລໍຖ້າ)
    ApplicationPending,
    /// Under review (ກຳລັງພິຈາລະນາ)
    UnderReview,
    /// Approved (ອະນຸມັດແລ້ວ)
    Approved,
    /// Active (ດຳເນີນການຢູ່)
    Active,
    /// Suspended (ໂຈະ)
    Suspended,
    /// Terminated (ຢຸດຕິ)
    Terminated,
    /// Expired (ໝົດອາຍຸ)
    Expired,
}

impl ConcessionStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ConcessionStatus::ApplicationPending => "ຄຳຮ້ອງລໍຖ້າ",
            ConcessionStatus::UnderReview => "ກຳລັງພິຈາລະນາ",
            ConcessionStatus::Approved => "ອະນຸມັດແລ້ວ",
            ConcessionStatus::Active => "ດຳເນີນການຢູ່",
            ConcessionStatus::Suspended => "ໂຈະ",
            ConcessionStatus::Terminated => "ຢຸດຕິ",
            ConcessionStatus::Expired => "ໝົດອາຍຸ",
        }
    }
}

/// Forest concession (ສຳປະທານປ່າໄມ້)
///
/// Articles 61-75: Forest concession requirements
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForestConcession {
    /// Concession number (ເລກສຳປະທານ)
    pub concession_number: String,

    /// Holder name (ຊື່ຜູ້ຖືສຳປະທານ)
    pub holder_name: String,

    /// Holder name in Lao (ຊື່ຜູ້ຖືສຳປະທານເປັນພາສາລາວ)
    pub holder_name_lao: Option<String>,

    /// Concession type (ປະເພດສຳປະທານ)
    pub concession_type: ConcessionType,

    /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
    pub area_hectares: f64,

    /// Term in years (ໄລຍະເປັນປີ)
    pub term_years: u32,

    /// Province (ແຂວງ)
    pub province: String,

    /// Districts covered (ເມືອງທີ່ກວມເອົາ)
    pub districts: Vec<String>,

    /// Start date (ວັນທີເລີ່ມຕົ້ນ)
    pub start_date: String,

    /// End date (ວັນທີສິ້ນສຸດ)
    pub end_date: String,

    /// Performance bond amount in LAK (ເງິນຄ້ຳປະກັນ)
    pub performance_bond_lak: u64,

    /// Project value in LAK (ມູນຄ່າໂຄງການ)
    pub project_value_lak: Option<u64>,

    /// Has Environmental Impact Assessment (ມີ EIA)
    pub has_eia: bool,

    /// Has management plan (ມີແຜນຄຸ້ມຄອງ)
    pub has_management_plan: bool,

    /// Status (ສະຖານະ)
    pub status: ConcessionStatus,

    /// Primary species (ຊະນິດໄມ້ຫຼັກ)
    pub primary_species: Vec<TreeSpecies>,

    /// Annual production quota (ໂກຕ້າການຜະລິດປະຈຳປີ)
    pub annual_production_quota_m3: Option<f64>,

    /// Reforestation commitment hectares (ຄຳໝັ້ນສັນຍາປູກປ່າ)
    pub reforestation_commitment_hectares: Option<f64>,

    /// Community benefit agreements (ຂໍ້ຕົກລົງຜົນປະໂຫຍດຊຸມຊົນ)
    pub community_agreements: Vec<String>,
}

impl Default for ForestConcession {
    fn default() -> Self {
        Self {
            concession_number: String::new(),
            holder_name: String::new(),
            holder_name_lao: None,
            concession_type: ConcessionType::Plantation,
            area_hectares: 0.0,
            term_years: 0,
            province: String::new(),
            districts: Vec::new(),
            start_date: String::new(),
            end_date: String::new(),
            performance_bond_lak: 0,
            project_value_lak: None,
            has_eia: false,
            has_management_plan: false,
            status: ConcessionStatus::ApplicationPending,
            primary_species: Vec::new(),
            annual_production_quota_m3: None,
            reforestation_commitment_hectares: None,
            community_agreements: Vec::new(),
        }
    }
}

/// Builder for ForestConcession
#[derive(Debug, Default)]
pub struct ForestConcessionBuilder {
    concession: ForestConcession,
}

impl ForestConcessionBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set concession number
    pub fn concession_number(mut self, number: impl Into<String>) -> Self {
        self.concession.concession_number = number.into();
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

    /// Set concession type
    pub fn concession_type(mut self, concession_type: ConcessionType) -> Self {
        self.concession.concession_type = concession_type;
        self
    }

    /// Set area in hectares
    pub fn area_hectares(mut self, area: f64) -> Self {
        self.concession.area_hectares = area;
        self
    }

    /// Set term in years
    pub fn term_years(mut self, years: u32) -> Self {
        self.concession.term_years = years;
        self
    }

    /// Set province
    pub fn province(mut self, province: impl Into<String>) -> Self {
        self.concession.province = province.into();
        self
    }

    /// Add district
    pub fn add_district(mut self, district: impl Into<String>) -> Self {
        self.concession.districts.push(district.into());
        self
    }

    /// Set start date
    pub fn start_date(mut self, date: impl Into<String>) -> Self {
        self.concession.start_date = date.into();
        self
    }

    /// Set end date
    pub fn end_date(mut self, date: impl Into<String>) -> Self {
        self.concession.end_date = date.into();
        self
    }

    /// Set performance bond
    pub fn performance_bond_lak(mut self, amount: u64) -> Self {
        self.concession.performance_bond_lak = amount;
        self
    }

    /// Set project value
    pub fn project_value_lak(mut self, value: u64) -> Self {
        self.concession.project_value_lak = Some(value);
        self
    }

    /// Set has EIA
    pub fn has_eia(mut self, has: bool) -> Self {
        self.concession.has_eia = has;
        self
    }

    /// Set has management plan
    pub fn has_management_plan(mut self, has: bool) -> Self {
        self.concession.has_management_plan = has;
        self
    }

    /// Set status
    pub fn status(mut self, status: ConcessionStatus) -> Self {
        self.concession.status = status;
        self
    }

    /// Add primary species
    pub fn add_species(mut self, species: TreeSpecies) -> Self {
        self.concession.primary_species.push(species);
        self
    }

    /// Set annual production quota
    pub fn annual_production_quota_m3(mut self, quota: f64) -> Self {
        self.concession.annual_production_quota_m3 = Some(quota);
        self
    }

    /// Set reforestation commitment
    pub fn reforestation_commitment_hectares(mut self, hectares: f64) -> Self {
        self.concession.reforestation_commitment_hectares = Some(hectares);
        self
    }

    /// Build the concession
    pub fn build(self) -> ForestConcession {
        self.concession
    }
}

// ============================================================================
// NTFP Permit (ໃບອະນຸຍາດເກັບກ່ຽວຜະລິດຕະພັນປ່າໄມ້ທີ່ບໍ່ແມ່ນໄມ້)
// ============================================================================

/// Non-Timber Forest Product type (ປະເພດຜະລິດຕະພັນປ່າໄມ້ທີ່ບໍ່ແມ່ນໄມ້)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NtfpType {
    /// Bamboo shoots (ໜໍ່ໄມ້)
    BambooShoots,
    /// Rattan (ຫວາຍ)
    Rattan,
    /// Honey (ເຜິ້ງ)
    Honey,
    /// Resin (ຢາງ)
    Resin,
    /// Cardamom (ໝາກແໜ່ງ)
    Cardamom,
    /// Mushrooms (ເຫັດ)
    Mushrooms,
    /// Medicinal plants (ພືດສະໝຸນໄພ)
    MedicinalPlants,
    /// Tree bark (ເປືອກໄມ້)
    TreeBark,
    /// Forest fruits (ໝາກໄມ້ປ່າ)
    ForestFruits,
    /// Insects (ແມງໄມ້)
    Insects,
    /// Other (ອື່ນໆ)
    Other,
}

impl NtfpType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            NtfpType::BambooShoots => "ໜໍ່ໄມ້",
            NtfpType::Rattan => "ຫວາຍ",
            NtfpType::Honey => "ເຜິ້ງ",
            NtfpType::Resin => "ຢາງ",
            NtfpType::Cardamom => "ໝາກແໜ່ງ",
            NtfpType::Mushrooms => "ເຫັດ",
            NtfpType::MedicinalPlants => "ພືດສະໝຸນໄພ",
            NtfpType::TreeBark => "ເປືອກໄມ້",
            NtfpType::ForestFruits => "ໝາກໄມ້ປ່າ",
            NtfpType::Insects => "ແມງໄມ້",
            NtfpType::Other => "ອື່ນໆ",
        }
    }

    /// Check if requires special permit
    pub fn requires_special_permit(&self) -> bool {
        matches!(self, NtfpType::MedicinalPlants | NtfpType::Resin)
    }
}

/// NTFP Permit (ໃບອະນຸຍາດ NTFP)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NtfpPermit {
    /// Permit number (ເລກໃບອະນຸຍາດ)
    pub permit_number: String,

    /// Holder name (ຊື່ຜູ້ຖືໃບອະນຸຍາດ)
    pub holder_name: String,

    /// Holder name in Lao (ຊື່ຜູ້ຖືໃບອະນຸຍາດເປັນພາສາລາວ)
    pub holder_name_lao: Option<String>,

    /// NTFP type (ປະເພດ NTFP)
    pub ntfp_type: NtfpType,

    /// Collection area province (ແຂວງເຂດເກັບກ່ຽວ)
    pub province: String,

    /// Collection area district (ເມືອງເຂດເກັບກ່ຽວ)
    pub district: String,

    /// Collection area village (ບ້ານເຂດເກັບກ່ຽວ)
    pub village: Option<String>,

    /// Quantity allowed (ປະລິມານທີ່ອະນຸຍາດ)
    pub quantity_allowed: f64,

    /// Quantity unit (ໜ່ວຍ)
    pub quantity_unit: String,

    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,

    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,

    /// Status (ສະຖານະ)
    pub status: PermitStatus,

    /// Is for commercial use (ເພື່ອການຄ້າ)
    pub commercial_use: bool,

    /// Fee paid in LAK (ຄ່າທຳນຽມທີ່ຈ່າຍແລ້ວ)
    pub fee_paid_lak: Option<u64>,
}

impl Default for NtfpPermit {
    fn default() -> Self {
        Self {
            permit_number: String::new(),
            holder_name: String::new(),
            holder_name_lao: None,
            ntfp_type: NtfpType::Other,
            province: String::new(),
            district: String::new(),
            village: None,
            quantity_allowed: 0.0,
            quantity_unit: "kg".to_string(),
            issue_date: String::new(),
            expiry_date: String::new(),
            status: PermitStatus::Pending,
            commercial_use: false,
            fee_paid_lak: None,
        }
    }
}

/// Builder for NtfpPermit
#[derive(Debug, Default)]
pub struct NtfpPermitBuilder {
    permit: NtfpPermit,
}

impl NtfpPermitBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set permit number
    pub fn permit_number(mut self, number: impl Into<String>) -> Self {
        self.permit.permit_number = number.into();
        self
    }

    /// Set holder name
    pub fn holder_name(mut self, name: impl Into<String>) -> Self {
        self.permit.holder_name = name.into();
        self
    }

    /// Set NTFP type
    pub fn ntfp_type(mut self, ntfp_type: NtfpType) -> Self {
        self.permit.ntfp_type = ntfp_type;
        self
    }

    /// Set province
    pub fn province(mut self, province: impl Into<String>) -> Self {
        self.permit.province = province.into();
        self
    }

    /// Set district
    pub fn district(mut self, district: impl Into<String>) -> Self {
        self.permit.district = district.into();
        self
    }

    /// Set quantity allowed
    pub fn quantity_allowed(mut self, quantity: f64, unit: impl Into<String>) -> Self {
        self.permit.quantity_allowed = quantity;
        self.permit.quantity_unit = unit.into();
        self
    }

    /// Set issue date
    pub fn issue_date(mut self, date: impl Into<String>) -> Self {
        self.permit.issue_date = date.into();
        self
    }

    /// Set expiry date
    pub fn expiry_date(mut self, date: impl Into<String>) -> Self {
        self.permit.expiry_date = date.into();
        self
    }

    /// Set commercial use
    pub fn commercial_use(mut self, commercial: bool) -> Self {
        self.permit.commercial_use = commercial;
        self
    }

    /// Set status
    pub fn status(mut self, status: PermitStatus) -> Self {
        self.permit.status = status;
        self
    }

    /// Build the permit
    pub fn build(self) -> NtfpPermit {
        self.permit
    }
}

// ============================================================================
// Village Forest (ປ່າບ້ານ)
// ============================================================================

/// Village forest (ປ່າບ້ານ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VillageForest {
    /// Village name (ຊື່ບ້ານ)
    pub village_name: String,

    /// Village name in Lao (ຊື່ບ້ານເປັນພາສາລາວ)
    pub village_name_lao: String,

    /// District (ເມືອງ)
    pub district: String,

    /// Province (ແຂວງ)
    pub province: String,

    /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
    pub area_hectares: f64,

    /// Registration date (ວັນທີຂຶ້ນທະບຽນ)
    pub registration_date: String,

    /// Has management agreement (ມີຂໍ້ຕົກລົງການຄຸ້ມຄອງ)
    pub has_management_agreement: bool,

    /// Agreement expiry date (ວັນທີໝົດອາຍຸຂໍ້ຕົກລົງ)
    pub agreement_expiry: Option<String>,

    /// Number of households (ຈຳນວນຄົວເຮືອນ)
    pub household_count: u32,

    /// Key species present (ຊະນິດພັນຫຼັກທີ່ມີຢູ່)
    pub key_species: Vec<TreeSpecies>,

    /// Traditional use practices (ການນຳໃຊ້ແບບດັ້ງເດີມ)
    pub traditional_uses: Vec<String>,

    /// Has community enterprise (ມີວິສາຫະກິດຊຸມຊົນ)
    pub has_community_enterprise: bool,
}

impl Default for VillageForest {
    fn default() -> Self {
        Self {
            village_name: String::new(),
            village_name_lao: String::new(),
            district: String::new(),
            province: String::new(),
            area_hectares: 0.0,
            registration_date: String::new(),
            has_management_agreement: false,
            agreement_expiry: None,
            household_count: 0,
            key_species: Vec::new(),
            traditional_uses: Vec::new(),
            has_community_enterprise: false,
        }
    }
}

/// Builder for VillageForest
#[derive(Debug, Default)]
pub struct VillageForestBuilder {
    forest: VillageForest,
}

impl VillageForestBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set village name
    pub fn village_name(mut self, name: impl Into<String>) -> Self {
        self.forest.village_name = name.into();
        self
    }

    /// Set village name in Lao
    pub fn village_name_lao(mut self, name: impl Into<String>) -> Self {
        self.forest.village_name_lao = name.into();
        self
    }

    /// Set district
    pub fn district(mut self, district: impl Into<String>) -> Self {
        self.forest.district = district.into();
        self
    }

    /// Set province
    pub fn province(mut self, province: impl Into<String>) -> Self {
        self.forest.province = province.into();
        self
    }

    /// Set area
    pub fn area_hectares(mut self, area: f64) -> Self {
        self.forest.area_hectares = area;
        self
    }

    /// Set registration date
    pub fn registration_date(mut self, date: impl Into<String>) -> Self {
        self.forest.registration_date = date.into();
        self
    }

    /// Set has management agreement
    pub fn has_management_agreement(mut self, has: bool) -> Self {
        self.forest.has_management_agreement = has;
        self
    }

    /// Set household count
    pub fn household_count(mut self, count: u32) -> Self {
        self.forest.household_count = count;
        self
    }

    /// Add key species
    pub fn add_species(mut self, species: TreeSpecies) -> Self {
        self.forest.key_species.push(species);
        self
    }

    /// Add traditional use
    pub fn add_traditional_use(mut self, use_type: impl Into<String>) -> Self {
        self.forest.traditional_uses.push(use_type.into());
        self
    }

    /// Build the village forest
    pub fn build(self) -> VillageForest {
        self.forest
    }
}

/// Village forest management agreement (ຂໍ້ຕົກລົງການຄຸ້ມຄອງປ່າບ້ານ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VillageForestAgreement {
    /// Agreement number (ເລກຂໍ້ຕົກລົງ)
    pub agreement_number: String,

    /// Village name (ຊື່ບ້ານ)
    pub village_name: String,

    /// District (ເມືອງ)
    pub district: String,

    /// Province (ແຂວງ)
    pub province: String,

    /// Start date (ວັນທີເລີ່ມຕົ້ນ)
    pub start_date: String,

    /// End date (ວັນທີສິ້ນສຸດ)
    pub end_date: String,

    /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
    pub area_hectares: f64,

    /// Permitted activities (ກິດຈະກຳທີ່ໄດ້ຮັບອະນຸຍາດ)
    pub permitted_activities: Vec<String>,

    /// Prohibited activities (ກິດຈະກຳທີ່ຫ້າມ)
    pub prohibited_activities: Vec<String>,

    /// Benefit sharing arrangement (ການແບ່ງປັນຜົນປະໂຫຍດ)
    pub benefit_sharing: Option<BenefitSharingArrangement>,

    /// Has management plan (ມີແຜນການຄຸ້ມຄອງ)
    pub has_management_plan: bool,

    /// Status (ສະຖານະ)
    pub status: PermitStatus,
}

// ============================================================================
// Log Tracking (ການຕິດຕາມໄມ້ທ່ອນ)
// ============================================================================

/// Log entry for tracking (ບັນທຶກໄມ້ທ່ອນສຳລັບການຕິດຕາມ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LogEntry {
    /// Log ID/marking number (ເລກໝາຍໄມ້ທ່ອນ)
    pub log_id: String,

    /// Species (ຊະນິດ)
    pub species: TreeSpecies,

    /// Length in meters (ຄວາມຍາວເປັນແມັດ)
    pub length_meters: f64,

    /// Diameter at breast height in cm (ເສັ້ນຜ່ານສູນກາງ DBH)
    pub diameter_cm: u32,

    /// Volume in cubic meters (ປະລິມານເປັນແມັດກ້ອນ)
    pub volume_cubic_meters: f64,

    /// Harvest permit reference (ອ້າງອິງໃບອະນຸຍາດຕັດ)
    pub harvest_permit_reference: String,

    /// Harvest date (ວັນທີຕັດ)
    pub harvest_date: String,

    /// Harvest location province (ແຂວງທີ່ຕັດ)
    pub harvest_province: String,

    /// Harvest location district (ເມືອງທີ່ຕັດ)
    pub harvest_district: String,

    /// Current location (ສະຖານທີ່ປະຈຸບັນ)
    pub current_location: String,

    /// Chain of custody entries (ບັນທຶກຕ່ອງໂສ້ການຄຸ້ມຄອງ)
    pub chain_of_custody: Vec<ChainOfCustodyEntry>,

    /// Is CITES listed (ຢູ່ໃນບັນຊີ CITES)
    pub is_cites_listed: bool,

    /// Quality grade (ເກຣດຄຸນນະພາບ)
    pub quality_grade: Option<String>,
}

impl Default for LogEntry {
    fn default() -> Self {
        Self {
            log_id: String::new(),
            species: TreeSpecies::OtherHardwood,
            length_meters: 0.0,
            diameter_cm: 0,
            volume_cubic_meters: 0.0,
            harvest_permit_reference: String::new(),
            harvest_date: String::new(),
            harvest_province: String::new(),
            harvest_district: String::new(),
            current_location: String::new(),
            chain_of_custody: Vec::new(),
            is_cites_listed: false,
            quality_grade: None,
        }
    }
}

/// Builder for LogEntry
#[derive(Debug, Default)]
pub struct LogEntryBuilder {
    entry: LogEntry,
}

impl LogEntryBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set log ID
    pub fn log_id(mut self, id: impl Into<String>) -> Self {
        self.entry.log_id = id.into();
        self
    }

    /// Set species
    pub fn species(mut self, species: TreeSpecies) -> Self {
        self.entry.species = species;
        self.entry.is_cites_listed = species.is_cites_listed();
        self
    }

    /// Set dimensions
    pub fn dimensions(mut self, length_m: f64, diameter_cm: u32) -> Self {
        self.entry.length_meters = length_m;
        self.entry.diameter_cm = diameter_cm;
        // Calculate approximate volume (cylinder formula)
        let radius_m = f64::from(diameter_cm) / 200.0;
        self.entry.volume_cubic_meters =
            std::f64::consts::PI * radius_m * radius_m * length_m * 0.7; // 0.7 form factor
        self
    }

    /// Set harvest permit reference
    pub fn harvest_permit_reference(mut self, reference: impl Into<String>) -> Self {
        self.entry.harvest_permit_reference = reference.into();
        self
    }

    /// Set harvest date
    pub fn harvest_date(mut self, date: impl Into<String>) -> Self {
        self.entry.harvest_date = date.into();
        self
    }

    /// Set harvest location
    pub fn harvest_location(
        mut self,
        province: impl Into<String>,
        district: impl Into<String>,
    ) -> Self {
        self.entry.harvest_province = province.into();
        self.entry.harvest_district = district.into();
        self
    }

    /// Set current location
    pub fn current_location(mut self, location: impl Into<String>) -> Self {
        self.entry.current_location = location.into();
        self
    }

    /// Add chain of custody entry
    pub fn add_custody_entry(mut self, entry: ChainOfCustodyEntry) -> Self {
        self.entry.chain_of_custody.push(entry);
        self
    }

    /// Build the log entry
    pub fn build(self) -> LogEntry {
        self.entry
    }
}

/// Chain of custody entry (ບັນທຶກຕ່ອງໂສ້ການຄຸ້ມຄອງ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChainOfCustodyEntry {
    /// Date (ວັນທີ)
    pub date: String,

    /// From location (ຈາກສະຖານທີ່)
    pub from_location: String,

    /// To location (ໄປສະຖານທີ່)
    pub to_location: String,

    /// Transport permit number (ເລກໃບອະນຸຍາດຂົນສົ່ງ)
    pub transport_permit: Option<String>,

    /// Handler name (ຊື່ຜູ້ຮັບຜິດຊອບ)
    pub handler_name: String,

    /// Remarks (ໝາຍເຫດ)
    pub remarks: Option<String>,
}

/// Transport permit (ໃບອະນຸຍາດຂົນສົ່ງໄມ້)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransportPermit {
    /// Permit number (ເລກໃບອະນຸຍາດ)
    pub permit_number: String,

    /// Holder name (ຊື່ຜູ້ຖືໃບອະນຸຍາດ)
    pub holder_name: String,

    /// Origin province (ແຂວງຕົ້ນທາງ)
    pub origin_province: String,

    /// Origin district (ເມືອງຕົ້ນທາງ)
    pub origin_district: String,

    /// Destination province (ແຂວງປາຍທາງ)
    pub destination_province: String,

    /// Destination district (ເມືອງປາຍທາງ)
    pub destination_district: String,

    /// Destination facility (ສະຖານທີ່ປາຍທາງ)
    pub destination_facility: Option<String>,

    /// Species (ຊະນິດ)
    pub species: TreeSpecies,

    /// Volume in cubic meters (ປະລິມານເປັນແມັດກ້ອນ)
    pub volume_cubic_meters: f64,

    /// Number of logs (ຈຳນວນທ່ອນໄມ້)
    pub log_count: u32,

    /// Vehicle registration (ທະບຽນລົດ)
    pub vehicle_registration: String,

    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,

    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,

    /// Specified route (ເສັ້ນທາງທີ່ກຳນົດ)
    pub specified_route: Option<String>,

    /// Status (ສະຖານະ)
    pub status: PermitStatus,

    /// Harvest permit reference (ອ້າງອິງໃບອະນຸຍາດຕັດ)
    pub harvest_permit_reference: String,
}

impl Default for TransportPermit {
    fn default() -> Self {
        Self {
            permit_number: String::new(),
            holder_name: String::new(),
            origin_province: String::new(),
            origin_district: String::new(),
            destination_province: String::new(),
            destination_district: String::new(),
            destination_facility: None,
            species: TreeSpecies::OtherHardwood,
            volume_cubic_meters: 0.0,
            log_count: 0,
            vehicle_registration: String::new(),
            issue_date: String::new(),
            expiry_date: String::new(),
            specified_route: None,
            status: PermitStatus::Pending,
            harvest_permit_reference: String::new(),
        }
    }
}

/// Builder for TransportPermit
#[derive(Debug, Default)]
pub struct TransportPermitBuilder {
    permit: TransportPermit,
}

impl TransportPermitBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set permit number
    pub fn permit_number(mut self, number: impl Into<String>) -> Self {
        self.permit.permit_number = number.into();
        self
    }

    /// Set holder name
    pub fn holder_name(mut self, name: impl Into<String>) -> Self {
        self.permit.holder_name = name.into();
        self
    }

    /// Set origin
    pub fn origin(mut self, province: impl Into<String>, district: impl Into<String>) -> Self {
        self.permit.origin_province = province.into();
        self.permit.origin_district = district.into();
        self
    }

    /// Set destination
    pub fn destination(mut self, province: impl Into<String>, district: impl Into<String>) -> Self {
        self.permit.destination_province = province.into();
        self.permit.destination_district = district.into();
        self
    }

    /// Set species
    pub fn species(mut self, species: TreeSpecies) -> Self {
        self.permit.species = species;
        self
    }

    /// Set volume
    pub fn volume_cubic_meters(mut self, volume: f64) -> Self {
        self.permit.volume_cubic_meters = volume;
        self
    }

    /// Set log count
    pub fn log_count(mut self, count: u32) -> Self {
        self.permit.log_count = count;
        self
    }

    /// Set vehicle registration
    pub fn vehicle_registration(mut self, registration: impl Into<String>) -> Self {
        self.permit.vehicle_registration = registration.into();
        self
    }

    /// Set issue date
    pub fn issue_date(mut self, date: impl Into<String>) -> Self {
        self.permit.issue_date = date.into();
        self
    }

    /// Set expiry date
    pub fn expiry_date(mut self, date: impl Into<String>) -> Self {
        self.permit.expiry_date = date.into();
        self
    }

    /// Set harvest permit reference
    pub fn harvest_permit_reference(mut self, reference: impl Into<String>) -> Self {
        self.permit.harvest_permit_reference = reference.into();
        self
    }

    /// Set status
    pub fn status(mut self, status: PermitStatus) -> Self {
        self.permit.status = status;
        self
    }

    /// Build the permit
    pub fn build(self) -> TransportPermit {
        self.permit
    }
}

// ============================================================================
// Sawmill and Processing (ໂຮງເລື່ອຍ ແລະ ການປຸງແຕ່ງ)
// ============================================================================

/// Sawmill license (ໃບອະນຸຍາດໂຮງເລື່ອຍ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SawmillLicense {
    /// License number (ເລກໃບອະນຸຍາດ)
    pub license_number: String,

    /// Facility name (ຊື່ໂຮງງານ)
    pub facility_name: String,

    /// Facility name in Lao (ຊື່ໂຮງງານເປັນພາສາລາວ)
    pub facility_name_lao: Option<String>,

    /// Owner name (ຊື່ເຈົ້າຂອງ)
    pub owner_name: String,

    /// Province (ແຂວງ)
    pub province: String,

    /// District (ເມືອງ)
    pub district: String,

    /// Annual capacity in cubic meters (ກຳລັງການຜະລິດປະຈຳປີ)
    pub annual_capacity_cubic_meters: f64,

    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,

    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,

    /// Status (ສະຖານະ)
    pub status: PermitStatus,

    /// Has environmental compliance (ມີການປະຕິບັດຕາມສິ່ງແວດລ້ອມ)
    pub environmental_compliance: bool,

    /// Log intake tracking system (ລະບົບຕິດຕາມການຮັບໄມ້ເຂົ້າ)
    pub has_log_tracking: bool,

    /// Permitted species (ຊະນິດທີ່ໄດ້ຮັບອະນຸຍາດ)
    pub permitted_species: Vec<TreeSpecies>,
}

impl Default for SawmillLicense {
    fn default() -> Self {
        Self {
            license_number: String::new(),
            facility_name: String::new(),
            facility_name_lao: None,
            owner_name: String::new(),
            province: String::new(),
            district: String::new(),
            annual_capacity_cubic_meters: 0.0,
            issue_date: String::new(),
            expiry_date: String::new(),
            status: PermitStatus::Pending,
            environmental_compliance: false,
            has_log_tracking: false,
            permitted_species: Vec::new(),
        }
    }
}

/// Builder for SawmillLicense
#[derive(Debug, Default)]
pub struct SawmillLicenseBuilder {
    license: SawmillLicense,
}

impl SawmillLicenseBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set license number
    pub fn license_number(mut self, number: impl Into<String>) -> Self {
        self.license.license_number = number.into();
        self
    }

    /// Set facility name
    pub fn facility_name(mut self, name: impl Into<String>) -> Self {
        self.license.facility_name = name.into();
        self
    }

    /// Set owner name
    pub fn owner_name(mut self, name: impl Into<String>) -> Self {
        self.license.owner_name = name.into();
        self
    }

    /// Set location
    pub fn location(mut self, province: impl Into<String>, district: impl Into<String>) -> Self {
        self.license.province = province.into();
        self.license.district = district.into();
        self
    }

    /// Set annual capacity
    pub fn annual_capacity_cubic_meters(mut self, capacity: f64) -> Self {
        self.license.annual_capacity_cubic_meters = capacity;
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

    /// Set environmental compliance
    pub fn environmental_compliance(mut self, compliant: bool) -> Self {
        self.license.environmental_compliance = compliant;
        self
    }

    /// Set has log tracking
    pub fn has_log_tracking(mut self, has: bool) -> Self {
        self.license.has_log_tracking = has;
        self
    }

    /// Set status
    pub fn status(mut self, status: PermitStatus) -> Self {
        self.license.status = status;
        self
    }

    /// Add permitted species
    pub fn add_permitted_species(mut self, species: TreeSpecies) -> Self {
        self.license.permitted_species.push(species);
        self
    }

    /// Build the license
    pub fn build(self) -> SawmillLicense {
        self.license
    }
}

/// Processing facility license (ໃບອະນຸຍາດໂຮງງານແປຮູບ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProcessingFacilityLicense {
    /// License number (ເລກໃບອະນຸຍາດ)
    pub license_number: String,

    /// Facility name (ຊື່ໂຮງງານ)
    pub facility_name: String,

    /// Province (ແຂວງ)
    pub province: String,

    /// District (ເມືອງ)
    pub district: String,

    /// Processing type (ປະເພດການແປຮູບ)
    pub processing_type: String,

    /// Annual capacity (ກຳລັງການຜະລິດປະຈຳປີ)
    pub annual_capacity: f64,

    /// Capacity unit (ໜ່ວຍ)
    pub capacity_unit: String,

    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,

    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,

    /// Status (ສະຖານະ)
    pub status: PermitStatus,

    /// Raw material tracking (ການຕິດຕາມວັດຖຸດິບ)
    pub has_raw_material_tracking: bool,
}

// ============================================================================
// Export Permit (ໃບອະນຸຍາດສົ່ງອອກ)
// ============================================================================

/// Export product type (ປະເພດຜະລິດຕະພັນສົ່ງອອກ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExportProductType {
    /// Logs (ໄມ້ທ່ອນ)
    Logs,
    /// Sawn timber (ໄມ້ແປຮູບ)
    SawnTimber,
    /// Plywood (ໄມ້ອັດ)
    Plywood,
    /// Furniture (ເຟີນິເຈີ)
    Furniture,
    /// Wood chips (ໄມ້ສັບ)
    WoodChips,
    /// Pulp (ເຍື່ອໄມ້)
    Pulp,
    /// NTFP (ຜະລິດຕະພັນປ່າໄມ້ທີ່ບໍ່ແມ່ນໄມ້)
    Ntfp,
    /// Charcoal (ຖ່ານ)
    Charcoal,
    /// Other (ອື່ນໆ)
    Other,
}

impl ExportProductType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ExportProductType::Logs => "ໄມ້ທ່ອນ",
            ExportProductType::SawnTimber => "ໄມ້ແປຮູບ",
            ExportProductType::Plywood => "ໄມ້ອັດ",
            ExportProductType::Furniture => "ເຟີນິເຈີ",
            ExportProductType::WoodChips => "ໄມ້ສັບ",
            ExportProductType::Pulp => "ເຍື່ອໄມ້",
            ExportProductType::Ntfp => "ຜະລິດຕະພັນປ່າໄມ້ທີ່ບໍ່ແມ່ນໄມ້",
            ExportProductType::Charcoal => "ຖ່ານ",
            ExportProductType::Other => "ອື່ນໆ",
        }
    }

    /// Check if log export is restricted
    pub fn is_restricted(&self) -> bool {
        matches!(self, ExportProductType::Logs | ExportProductType::Charcoal)
    }
}

/// Forest product export permit (ໃບອະນຸຍາດສົ່ງອອກຜະລິດຕະພັນປ່າໄມ້)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForestProductExportPermit {
    /// Permit number (ເລກໃບອະນຸຍາດ)
    pub permit_number: String,

    /// Exporter name (ຊື່ຜູ້ສົ່ງອອກ)
    pub exporter_name: String,

    /// Exporter name in Lao (ຊື່ຜູ້ສົ່ງອອກເປັນພາສາລາວ)
    pub exporter_name_lao: Option<String>,

    /// Product type (ປະເພດຜະລິດຕະພັນ)
    pub product_type: ExportProductType,

    /// Species if applicable (ຊະນິດຖ້າມີ)
    pub species: Option<TreeSpecies>,

    /// Quantity (ປະລິມານ)
    pub quantity: f64,

    /// Quantity unit (ໜ່ວຍ)
    pub quantity_unit: String,

    /// Value in USD (ມູນຄ່າເປັນ USD)
    pub value_usd: Option<f64>,

    /// Destination country (ປະເທດປາຍທາງ)
    pub destination_country: String,

    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,

    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,

    /// Status (ສະຖານະ)
    pub status: PermitStatus,

    /// CITES permit number if required (ເລກໃບອະນຸຍາດ CITES)
    pub cites_permit_number: Option<String>,

    /// Phytosanitary certificate (ໃບຢັ້ງຢືນສຸຂານາໄມພືດ)
    pub phytosanitary_certificate: Option<String>,

    /// Origin certificate (ໃບຢັ້ງຢືນແຫຼ່ງກຳເນີດ)
    pub origin_certificate: Option<String>,

    /// Source permit references (ອ້າງອິງໃບອະນຸຍາດແຫຼ່ງທີ່ມາ)
    pub source_permits: Vec<String>,
}

impl Default for ForestProductExportPermit {
    fn default() -> Self {
        Self {
            permit_number: String::new(),
            exporter_name: String::new(),
            exporter_name_lao: None,
            product_type: ExportProductType::SawnTimber,
            species: None,
            quantity: 0.0,
            quantity_unit: "m³".to_string(),
            value_usd: None,
            destination_country: String::new(),
            issue_date: String::new(),
            expiry_date: String::new(),
            status: PermitStatus::Pending,
            cites_permit_number: None,
            phytosanitary_certificate: None,
            origin_certificate: None,
            source_permits: Vec::new(),
        }
    }
}

/// Builder for ForestProductExportPermit
#[derive(Debug, Default)]
pub struct ForestProductExportPermitBuilder {
    permit: ForestProductExportPermit,
}

impl ForestProductExportPermitBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set permit number
    pub fn permit_number(mut self, number: impl Into<String>) -> Self {
        self.permit.permit_number = number.into();
        self
    }

    /// Set exporter name
    pub fn exporter_name(mut self, name: impl Into<String>) -> Self {
        self.permit.exporter_name = name.into();
        self
    }

    /// Set product type
    pub fn product_type(mut self, product_type: ExportProductType) -> Self {
        self.permit.product_type = product_type;
        self
    }

    /// Set species
    pub fn species(mut self, species: TreeSpecies) -> Self {
        self.permit.species = Some(species);
        self
    }

    /// Set quantity
    pub fn quantity(mut self, quantity: f64, unit: impl Into<String>) -> Self {
        self.permit.quantity = quantity;
        self.permit.quantity_unit = unit.into();
        self
    }

    /// Set destination country
    pub fn destination_country(mut self, country: impl Into<String>) -> Self {
        self.permit.destination_country = country.into();
        self
    }

    /// Set issue date
    pub fn issue_date(mut self, date: impl Into<String>) -> Self {
        self.permit.issue_date = date.into();
        self
    }

    /// Set expiry date
    pub fn expiry_date(mut self, date: impl Into<String>) -> Self {
        self.permit.expiry_date = date.into();
        self
    }

    /// Set CITES permit number
    pub fn cites_permit_number(mut self, number: impl Into<String>) -> Self {
        self.permit.cites_permit_number = Some(number.into());
        self
    }

    /// Set status
    pub fn status(mut self, status: PermitStatus) -> Self {
        self.permit.status = status;
        self
    }

    /// Add source permit
    pub fn add_source_permit(mut self, permit: impl Into<String>) -> Self {
        self.permit.source_permits.push(permit.into());
        self
    }

    /// Build the permit
    pub fn build(self) -> ForestProductExportPermit {
        self.permit
    }
}

// ============================================================================
// Community Forestry (ປ່າໄມ້ຊຸມຊົນ)
// ============================================================================

/// Community forest enterprise (ວິສາຫະກິດປ່າໄມ້ຊຸມຊົນ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommunityForestEnterprise {
    /// Enterprise name (ຊື່ວິສາຫະກິດ)
    pub name: String,

    /// Enterprise name in Lao (ຊື່ວິສາຫະກິດເປັນພາສາລາວ)
    pub name_lao: String,

    /// Registration number (ເລກທະບຽນ)
    pub registration_number: String,

    /// Village name (ຊື່ບ້ານ)
    pub village_name: String,

    /// District (ເມືອງ)
    pub district: String,

    /// Province (ແຂວງ)
    pub province: String,

    /// Member count (ຈຳນວນສະມາຊິກ)
    pub member_count: u32,

    /// Registration date (ວັນທີຂຶ້ນທະບຽນ)
    pub registration_date: String,

    /// Products/services (ຜະລິດຕະພັນ/ບໍລິການ)
    pub products_services: Vec<String>,

    /// Annual revenue in LAK (ລາຍໄດ້ປະຈຳປີ)
    pub annual_revenue_lak: Option<u64>,

    /// Has benefit sharing agreement (ມີຂໍ້ຕົກລົງແບ່ງປັນຜົນປະໂຫຍດ)
    pub has_benefit_sharing: bool,
}

/// Benefit sharing arrangement (ການຈັດສັນການແບ່ງປັນຜົນປະໂຫຍດ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BenefitSharingArrangement {
    /// Village share percentage (ສ່ວນແບ່ງບ້ານ %)
    pub village_share_percent: f64,

    /// District share percentage (ສ່ວນແບ່ງເມືອງ %)
    pub district_share_percent: f64,

    /// National share percentage (ສ່ວນແບ່ງລັດ %)
    pub national_share_percent: f64,

    /// Agreement date (ວັນທີຕົກລົງ)
    pub agreement_date: String,

    /// Agreement validity years (ໄລຍະຂໍ້ຕົກລົງ)
    pub validity_years: u32,
}

impl Default for BenefitSharingArrangement {
    fn default() -> Self {
        Self {
            village_share_percent: VILLAGE_BENEFIT_SHARE_PERCENT,
            district_share_percent: DISTRICT_BENEFIT_SHARE_PERCENT,
            national_share_percent: NATIONAL_BENEFIT_SHARE_PERCENT,
            agreement_date: String::new(),
            validity_years: 5,
        }
    }
}

// ============================================================================
// Violations and Penalties (ການລະເມີດ ແລະ ໂທດ)
// ============================================================================

/// Forestry violation type (ປະເພດການລະເມີດກົດໝາຍປ່າໄມ້)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ViolationType {
    /// Illegal logging (ການຕັດໄມ້ຜິດກົດໝາຍ) - Article 107
    IllegalLogging,
    /// Wildlife trafficking (ການຄ້າສັດປ່າຜິດກົດໝາຍ) - Article 108
    WildlifeTrafficking,
    /// Forest fire (ໄຟໄໝ້ປ່າ) - Article 109
    ForestFire,
    /// Unauthorized land clearing (ການບຸກເບີກທີ່ດິນໂດຍບໍ່ໄດ້ຮັບອະນຸຍາດ)
    UnauthorizedLandClearing,
    /// Permit violation (ການລະເມີດໃບອະນຸຍາດ)
    PermitViolation,
    /// Encroachment on protected area (ການບຸກລຸກເຂດປ່າປ້ອງກັນ)
    ProtectedAreaEncroachment,
    /// Harvesting prohibited species (ການຕັດຊະນິດພັນທີ່ຫ້າມ)
    ProhibitedSpeciesHarvesting,
    /// Transport without permit (ການຂົນສົ່ງໂດຍບໍ່ມີໃບອະນຸຍາດ)
    TransportWithoutPermit,
    /// Failure to reforest (ການບໍ່ປູກປ່າຄືນ)
    FailureToReforest,
    /// CITES violation (ການລະເມີດ CITES)
    CitesViolation,
    /// Other (ອື່ນໆ)
    Other,
}

impl ViolationType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ViolationType::IllegalLogging => "ການຕັດໄມ້ຜິດກົດໝາຍ",
            ViolationType::WildlifeTrafficking => "ການຄ້າສັດປ່າຜິດກົດໝາຍ",
            ViolationType::ForestFire => "ໄຟໄໝ້ປ່າ",
            ViolationType::UnauthorizedLandClearing => "ການບຸກເບີກທີ່ດິນໂດຍບໍ່ໄດ້ຮັບອະນຸຍາດ",
            ViolationType::PermitViolation => "ການລະເມີດໃບອະນຸຍາດ",
            ViolationType::ProtectedAreaEncroachment => "ການບຸກລຸກເຂດປ່າປ້ອງກັນ",
            ViolationType::ProhibitedSpeciesHarvesting => "ການຕັດຊະນິດພັນທີ່ຫ້າມ",
            ViolationType::TransportWithoutPermit => "ການຂົນສົ່ງໂດຍບໍ່ມີໃບອະນຸຍາດ",
            ViolationType::FailureToReforest => "ການບໍ່ປູກປ່າຄືນ",
            ViolationType::CitesViolation => "ການລະເມີດ CITES",
            ViolationType::Other => "ອື່ນໆ",
        }
    }

    /// Get article number
    pub fn article_number(&self) -> Option<u32> {
        match self {
            ViolationType::IllegalLogging => Some(107),
            ViolationType::WildlifeTrafficking => Some(108),
            ViolationType::ForestFire => Some(109),
            ViolationType::FailureToReforest => Some(110),
            _ => None,
        }
    }

    /// Get fine multiplier range
    pub fn fine_multiplier_range(&self) -> (f64, f64) {
        match self {
            ViolationType::IllegalLogging => (
                ILLEGAL_LOGGING_FINE_MULTIPLIER_MIN,
                ILLEGAL_LOGGING_FINE_MULTIPLIER_MAX,
            ),
            ViolationType::WildlifeTrafficking => (
                WILDLIFE_TRAFFICKING_FINE_MULTIPLIER_MIN,
                WILDLIFE_TRAFFICKING_FINE_MULTIPLIER_MAX,
            ),
            ViolationType::ProhibitedSpeciesHarvesting => (
                ILLEGAL_LOGGING_FINE_MULTIPLIER_MAX,
                WILDLIFE_TRAFFICKING_FINE_MULTIPLIER_MAX,
            ),
            _ => (1.0, 5.0),
        }
    }
}

/// Forestry violation record (ບັນທຶກການລະເມີດກົດໝາຍປ່າໄມ້)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForestryViolation {
    /// Case number (ເລກຄະດີ)
    pub case_number: String,

    /// Violation type (ປະເພດການລະເມີດ)
    pub violation_type: ViolationType,

    /// Violator name (ຊື່ຜູ້ລະເມີດ)
    pub violator_name: String,

    /// Violator name in Lao (ຊື່ຜູ້ລະເມີດເປັນພາສາລາວ)
    pub violator_name_lao: Option<String>,

    /// Location province (ແຂວງ)
    pub province: String,

    /// Location district (ເມືອງ)
    pub district: String,

    /// Detection date (ວັນທີກວດພົບ)
    pub detection_date: String,

    /// Species involved (ຊະນິດທີ່ກ່ຽວຂ້ອງ)
    pub species_involved: Option<TreeSpecies>,

    /// Volume involved in m3 (ປະລິມານທີ່ກ່ຽວຂ້ອງ)
    pub volume_cubic_meters: Option<f64>,

    /// Estimated value in LAK (ມູນຄ່າຄາດຄະເນ)
    pub estimated_value_lak: Option<u64>,

    /// Area affected in hectares (ເນື້ອທີ່ທີ່ໄດ້ຮັບຜົນກະທົບ)
    pub area_affected_hectares: Option<f64>,

    /// Penalty assessment (ການປະເມີນໂທດ)
    pub penalty: Option<PenaltyAssessment>,

    /// Status (ສະຖານະ)
    pub status: ViolationStatus,

    /// Confiscated items (ຂອງທີ່ຖືກຍຶດ)
    pub confiscated_items: Vec<String>,

    /// Reforestation required (ຕ້ອງປູກປ່າຄືນ)
    pub reforestation_required: bool,

    /// Reforestation area (ເນື້ອທີ່ປູກປ່າຄືນ)
    pub reforestation_area_hectares: Option<f64>,
}

/// Violation status (ສະຖານະການລະເມີດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ViolationStatus {
    /// Under investigation (ກຳລັງສືບສວນ)
    UnderInvestigation,
    /// Pending prosecution (ລໍຖ້າການດຳເນີນຄະດີ)
    PendingProsecution,
    /// Fine imposed (ປັບໄໝແລ້ວ)
    FineImposed,
    /// Fine paid (ຈ່າຍຄ່າປັບໄໝແລ້ວ)
    FinePaid,
    /// Criminal prosecution (ດຳເນີນຄະດີອາຍາ)
    CriminalProsecution,
    /// Resolved (ແກ້ໄຂແລ້ວ)
    Resolved,
    /// Appeal pending (ລໍຖ້າອຸທອນ)
    AppealPending,
}

impl ViolationStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ViolationStatus::UnderInvestigation => "ກຳລັງສືບສວນ",
            ViolationStatus::PendingProsecution => "ລໍຖ້າການດຳເນີນຄະດີ",
            ViolationStatus::FineImposed => "ປັບໄໝແລ້ວ",
            ViolationStatus::FinePaid => "ຈ່າຍຄ່າປັບໄໝແລ້ວ",
            ViolationStatus::CriminalProsecution => "ດຳເນີນຄະດີອາຍາ",
            ViolationStatus::Resolved => "ແກ້ໄຂແລ້ວ",
            ViolationStatus::AppealPending => "ລໍຖ້າອຸທອນ",
        }
    }
}

/// Penalty assessment (ການປະເມີນໂທດ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PenaltyAssessment {
    /// Fine amount in LAK (ຈຳນວນຄ່າປັບໄໝ)
    pub fine_amount_lak: u64,

    /// Fine multiplier used (ຕົວຄູນຄ່າປັບໄໝທີ່ໃຊ້)
    pub fine_multiplier: f64,

    /// Equipment confiscation (ການຍຶດອຸປະກອນ)
    pub equipment_confiscation: bool,

    /// Confiscated equipment description (ລາຍລະອຽດອຸປະກອນທີ່ຖືກຍຶດ)
    pub confiscated_equipment: Vec<String>,

    /// Imprisonment months (ເດືອນຈຳຄຸກ)
    pub imprisonment_months: Option<u32>,

    /// Reforestation obligation (ພັນທະປູກປ່າຄືນ)
    pub reforestation_obligation_hectares: Option<f64>,

    /// License suspension (ການໂຈະໃບອະນຸຍາດ)
    pub license_suspension: bool,

    /// License revocation (ການຖອນໃບອະນຸຍາດ)
    pub license_revocation: bool,

    /// Assessment date (ວັນທີປະເມີນ)
    pub assessment_date: String,

    /// Payment deadline (ກຳນົດຈ່າຍ)
    pub payment_deadline: Option<String>,
}
