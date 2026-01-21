//! Water Law Types (ປະເພດກົດໝາຍຊັບພະຍາກອນນໍ້າ)
//!
//! Type definitions for Lao water and water resources law based on:
//! - **Water and Water Resources Law 2017** (Law No. 23/NA, dated June 30, 2017)
//! - Mekong River Commission 1995 Agreement
//!
//! # Legal References
//! - Water and Water Resources Law 2017 (Law No. 23/NA) - ກົດໝາຍວ່າດ້ວຍນໍ້າ ແລະ ຊັບພະຍາກອນນໍ້າ ປີ 2017
//! - Procedures for Notification, Prior Consultation and Agreement (PNPCA)
//!
//! # Bilingual Support
//! All types include both Lao (ລາວ) and English field names where applicable.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Water and Water Resources Law 2017
// ============================================================================

/// Small hydropower threshold (MW) - Article 46
/// ເກນໄຟຟ້ານໍ້າຕົກຂະໜາດນ້ອຍ
pub const SMALL_HYDROPOWER_THRESHOLD_MW: f64 = 15.0;

/// Medium hydropower upper threshold (MW) - Article 46
/// ເກນເທິງໄຟຟ້ານໍ້າຕົກຂະໜາດກາງ
pub const MEDIUM_HYDROPOWER_THRESHOLD_MW: f64 = 100.0;

/// Minimum hydropower concession period (years) - Article 47
/// ໄລຍະສຳປະທານໄຟຟ້ານໍ້າຕົກຂັ້ນຕໍ່າ
pub const HYDROPOWER_CONCESSION_MIN_YEARS: u32 = 25;

/// Maximum hydropower concession period (years) - Article 47
/// ໄລຍະສຳປະທານໄຟຟ້ານໍ້າຕົກສູງສຸດ
pub const HYDROPOWER_CONCESSION_MAX_YEARS: u32 = 30;

/// Well depth threshold requiring permit (meters) - Article 75
/// ເກນຄວາມເລິກບໍ່ທີ່ຕ້ອງມີໃບອະນຸຍາດ
pub const WELL_PERMIT_DEPTH_THRESHOLD_M: u32 = 20;

/// Maximum drinking water turbidity (NTU) - Article 55
/// ຄວາມຂຸ່ນນໍ້າດື່ມສູງສຸດ
pub const DRINKING_WATER_MAX_TURBIDITY_NTU: f64 = 5.0;

/// Maximum drinking water pH - Article 55
/// pH ນໍ້າດື່ມສູງສຸດ
pub const DRINKING_WATER_MAX_PH: f64 = 8.5;

/// Minimum drinking water pH - Article 55
/// pH ນໍ້າດື່ມຕໍ່າສຸດ
pub const DRINKING_WATER_MIN_PH: f64 = 6.5;

/// Maximum drinking water arsenic (mg/L) - Article 55
/// ສານຫນູນໍ້າດື່ມສູງສຸດ
pub const DRINKING_WATER_MAX_ARSENIC_MG_L: f64 = 0.01;

/// Maximum drinking water lead (mg/L) - Article 55
/// ຕະກົ່ວນໍ້າດື່ມສູງສຸດ
pub const DRINKING_WATER_MAX_LEAD_MG_L: f64 = 0.01;

/// Maximum drinking water E. coli (CFU/100mL) - Article 55
/// E. coli ນໍ້າດື່ມສູງສຸດ
pub const DRINKING_WATER_MAX_ECOLI: f64 = 0.0;

/// Maximum BOD for industrial discharge (mg/L) - Article 57
/// BOD ສູງສຸດສຳລັບການປ່ອຍນໍ້າເສຍອຸດສາຫະກຳ
pub const INDUSTRIAL_DISCHARGE_MAX_BOD_MG_L: f64 = 20.0;

/// Maximum COD for industrial discharge (mg/L) - Article 57
/// COD ສູງສຸດສຳລັບການປ່ອຍນໍ້າເສຍອຸດສາຫະກຳ
pub const INDUSTRIAL_DISCHARGE_MAX_COD_MG_L: f64 = 120.0;

/// Maximum TSS for industrial discharge (mg/L) - Article 57
/// TSS ສູງສຸດສຳລັບການປ່ອຍນໍ້າເສຍອຸດສາຫະກຳ
pub const INDUSTRIAL_DISCHARGE_MAX_TSS_MG_L: f64 = 50.0;

/// MRC prior consultation period (months) - Article 60
/// ໄລຍະປຶກສາຫາລືລ່ວງໜ້າ MRC
pub const MRC_PRIOR_CONSULTATION_MONTHS: u32 = 6;

/// Irrigation service fee per hectare per season (LAK) - Article 72
/// ຄ່າບໍລິການຊົນລະປະທານຕໍ່ເຮັກຕາຕໍ່ລະດູການ
pub const IRRIGATION_FEE_PER_HECTARE_LAK: u64 = 150_000;

/// Water permit validity period (years) - Article 35
/// ໄລຍະໃບອະນຸຍາດນໍ້າ
pub const WATER_PERMIT_VALIDITY_YEARS: u32 = 5;

/// Groundwater monitoring interval (days) - Article 78
/// ໄລຍະການຕິດຕາມນໍ້າໃຕ້ດິນ
pub const GROUNDWATER_MONITORING_INTERVAL_DAYS: u32 = 90;

// ============================================================================
// Water Source Classification (ການຈັດປະເພດແຫຼ່ງນໍ້າ)
// ============================================================================

/// Water source type (ປະເພດແຫຼ່ງນໍ້າ)
///
/// Article 15: Classification of water sources in Lao PDR
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WaterSourceType {
    /// Surface water - rivers, streams, lakes, reservoirs (ນໍ້າຜິວໜ້າ)
    SurfaceWater {
        /// Water body type (ປະເພດແຫຼ່ງນໍ້າ)
        body_type: SurfaceWaterBodyType,
        /// Name of the water body (ຊື່ແຫຼ່ງນໍ້າ)
        name: String,
        /// Catchment area in km2 (ເຂດລຸ່ມນໍ້າ)
        catchment_area_km2: Option<f64>,
    },

    /// Groundwater - aquifers and wells (ນໍ້າໃຕ້ດິນ)
    Groundwater {
        /// Aquifer type (ປະເພດຊັ້ນນໍ້າໃຕ້ດິນ)
        aquifer_type: AquiferType,
        /// Depth in meters (ຄວາມເລິກ)
        depth_m: f64,
        /// Sustainable yield m3/day (ຜົນຜະລິດແບບຍືນຍົງ)
        sustainable_yield_m3_day: Option<f64>,
    },

    /// Mekong River system - international treaty waters (ລະບົບແມ່ນໍ້າຂອງ)
    MekongRiverSystem {
        /// Location type (ປະເພດສະຖານທີ່)
        location: MekongLocation,
        /// River section name (ຊື່ສ່ວນແມ່ນໍ້າ)
        section_name: Option<String>,
        /// Distance from border in km (ໄລຍະຫ່າງຈາກຊາຍແດນ)
        distance_from_border_km: Option<f64>,
    },

    /// Wetlands and floodplains (ທີ່ດິນບຶງ ແລະ ທີ່ລຸ່ມ)
    Wetland {
        /// Wetland type (ປະເພດທີ່ດິນບຶງ)
        wetland_type: WetlandType,
        /// Area in hectares (ເນື້ອທີ່)
        area_hectares: f64,
        /// Ecological significance (ຄວາມສຳຄັນທາງນິເວດ)
        ecological_significance: EcologicalSignificance,
    },
}

impl WaterSourceType {
    /// Get the Lao name of the water source type
    pub fn lao_name(&self) -> &'static str {
        match self {
            WaterSourceType::SurfaceWater { .. } => "ນໍ້າຜິວໜ້າ",
            WaterSourceType::Groundwater { .. } => "ນໍ້າໃຕ້ດິນ",
            WaterSourceType::MekongRiverSystem { .. } => "ລະບົບແມ່ນໍ້າຂອງ",
            WaterSourceType::Wetland { .. } => "ທີ່ດິນບຶງ",
        }
    }

    /// Check if MRC procedures apply (Article 60-63)
    pub fn requires_mrc_procedures(&self) -> bool {
        matches!(
            self,
            WaterSourceType::MekongRiverSystem {
                location: MekongLocation::Mainstream,
                ..
            }
        )
    }
}

/// Surface water body type (ປະເພດແຫຼ່ງນໍ້າຜິວໜ້າ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SurfaceWaterBodyType {
    /// Major river (ແມ່ນໍ້າໃຫຍ່)
    MajorRiver,
    /// Tributary (ສາຂາ)
    Tributary,
    /// Stream (ຫ້ວຍ)
    Stream,
    /// Natural lake (ທະເລສາບທຳມະຊາດ)
    NaturalLake,
    /// Reservoir (ອ່າງເກັບນໍ້າ)
    Reservoir,
    /// Pond (ໜອງ)
    Pond,
}

impl SurfaceWaterBodyType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            SurfaceWaterBodyType::MajorRiver => "ແມ່ນໍ້າໃຫຍ່",
            SurfaceWaterBodyType::Tributary => "ສາຂາ",
            SurfaceWaterBodyType::Stream => "ຫ້ວຍ",
            SurfaceWaterBodyType::NaturalLake => "ທະເລສາບທຳມະຊາດ",
            SurfaceWaterBodyType::Reservoir => "ອ່າງເກັບນໍ້າ",
            SurfaceWaterBodyType::Pond => "ໜອງ",
        }
    }
}

/// Aquifer type (ປະເພດຊັ້ນນໍ້າໃຕ້ດິນ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AquiferType {
    /// Unconfined aquifer (ຊັ້ນນໍ້າໃຕ້ດິນແບບເປີດ)
    Unconfined,
    /// Confined aquifer (ຊັ້ນນໍ້າໃຕ້ດິນແບບປິດ)
    Confined,
    /// Semi-confined aquifer (ຊັ້ນນໍ້າໃຕ້ດິນແບບກາງ)
    SemiConfined,
    /// Perched aquifer (ຊັ້ນນໍ້າໃຕ້ດິນແບບລອຍ)
    Perched,
}

impl AquiferType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            AquiferType::Unconfined => "ຊັ້ນນໍ້າໃຕ້ດິນແບບເປີດ",
            AquiferType::Confined => "ຊັ້ນນໍ້າໃຕ້ດິນແບບປິດ",
            AquiferType::SemiConfined => "ຊັ້ນນໍ້າໃຕ້ດິນແບບກາງ",
            AquiferType::Perched => "ຊັ້ນນໍ້າໃຕ້ດິນແບບລອຍ",
        }
    }
}

/// Mekong River location (ສະຖານທີ່ແມ່ນໍ້າຂອງ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MekongLocation {
    /// Mainstream - requires MRC prior consultation (ສາຍຫຼັກ)
    Mainstream,
    /// Major tributary (ສາຂາໃຫຍ່)
    MajorTributary,
    /// Minor tributary (ສາຂານ້ອຍ)
    MinorTributary,
}

impl MekongLocation {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            MekongLocation::Mainstream => "ສາຍຫຼັກ",
            MekongLocation::MajorTributary => "ສາຂາໃຫຍ່",
            MekongLocation::MinorTributary => "ສາຂານ້ອຍ",
        }
    }

    /// Check if prior consultation required
    pub fn requires_prior_consultation(&self) -> bool {
        matches!(self, MekongLocation::Mainstream)
    }
}

/// Wetland type (ປະເພດທີ່ດິນບຶງ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WetlandType {
    /// Permanent freshwater marsh (ບຶງນໍ້າຈືດຖາວອນ)
    PermanentMarsh,
    /// Seasonal floodplain (ທີ່ລຸ່ມຕາມລະດູການ)
    SeasonalFloodplain,
    /// Peat swamp (ບຶງພີດ)
    PeatSwamp,
    /// Riverine wetland (ທີ່ດິນບຶງລຽບຕາມແມ່ນໍ້າ)
    Riverine,
    /// Lacustrine wetland (ທີ່ດິນບຶງລຽບຕາມທະເລສາບ)
    Lacustrine,
}

impl WetlandType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            WetlandType::PermanentMarsh => "ບຶງນໍ້າຈືດຖາວອນ",
            WetlandType::SeasonalFloodplain => "ທີ່ລຸ່ມຕາມລະດູການ",
            WetlandType::PeatSwamp => "ບຶງພີດ",
            WetlandType::Riverine => "ທີ່ດິນບຶງລຽບຕາມແມ່ນໍ້າ",
            WetlandType::Lacustrine => "ທີ່ດິນບຶງລຽບຕາມທະເລສາບ",
        }
    }
}

/// Ecological significance (ຄວາມສຳຄັນທາງນິເວດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EcologicalSignificance {
    /// Local importance (ຄວາມສຳຄັນທ້ອງຖິ່ນ)
    Local,
    /// Regional importance (ຄວາມສຳຄັນພາກພື້ນ)
    Regional,
    /// National importance (ຄວາມສຳຄັນແຫ່ງຊາດ)
    National,
    /// International importance (Ramsar) (ຄວາມສຳຄັນສາກົນ)
    International,
}

impl EcologicalSignificance {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            EcologicalSignificance::Local => "ຄວາມສຳຄັນທ້ອງຖິ່ນ",
            EcologicalSignificance::Regional => "ຄວາມສຳຄັນພາກພື້ນ",
            EcologicalSignificance::National => "ຄວາມສຳຄັນແຫ່ງຊາດ",
            EcologicalSignificance::International => "ຄວາມສຳຄັນສາກົນ (Ramsar)",
        }
    }
}

// ============================================================================
// Water Use Rights (ສິດນຳໃຊ້ນໍ້າ)
// ============================================================================

/// Water use type (ປະເພດການນຳໃຊ້ນໍ້າ)
///
/// Article 38-44: Water use priority hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WaterUseType {
    /// Domestic use - highest priority (ການນຳໃຊ້ຄົວເຮືອນ)
    Domestic = 1,
    /// Agricultural use - second priority (ການນຳໃຊ້ກະສິກຳ)
    Agricultural = 2,
    /// Industrial use - third priority (ການນຳໃຊ້ອຸດສາຫະກຳ)
    Industrial = 3,
    /// Hydropower generation - fourth priority (ການຜະລິດໄຟຟ້ານໍ້າຕົກ)
    Hydropower = 4,
    /// Navigation - fifth priority (ການເດີນເຮືອ)
    Navigation = 5,
    /// Aquaculture (ການລ້ຽງສັດນໍ້າ)
    Aquaculture = 6,
    /// Recreation (ການພັກຜ່ອນ)
    Recreation = 7,
    /// Environmental flow (ການໄຫຼເພື່ອສິ່ງແວດລ້ອມ)
    Environmental = 8,
}

impl WaterUseType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            WaterUseType::Domestic => "ການນຳໃຊ້ຄົວເຮືອນ",
            WaterUseType::Agricultural => "ການນຳໃຊ້ກະສິກຳ",
            WaterUseType::Industrial => "ການນຳໃຊ້ອຸດສາຫະກຳ",
            WaterUseType::Hydropower => "ການຜະລິດໄຟຟ້ານໍ້າຕົກ",
            WaterUseType::Navigation => "ການເດີນເຮືອ",
            WaterUseType::Aquaculture => "ການລ້ຽງສັດນໍ້າ",
            WaterUseType::Recreation => "ການພັກຜ່ອນ",
            WaterUseType::Environmental => "ການໄຫຼເພື່ອສິ່ງແວດລ້ອມ",
        }
    }

    /// Get priority level (1 = highest)
    pub fn priority(&self) -> u8 {
        *self as u8
    }

    /// Check if permit required (Article 35)
    pub fn requires_permit(&self) -> bool {
        !matches!(self, WaterUseType::Domestic | WaterUseType::Navigation)
    }

    /// Check if has higher priority than another use
    pub fn has_priority_over(&self, other: &WaterUseType) -> bool {
        (*self as u8) < (*other as u8)
    }
}

/// Water use right (ສິດນຳໃຊ້ນໍ້າ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WaterUseRight {
    /// Permit number (ເລກໃບອະນຸຍາດ)
    pub permit_number: String,
    /// Holder name (ຊື່ຜູ້ຖື)
    pub holder_name: String,
    /// Holder name in Lao (ຊື່ຜູ້ຖືເປັນພາສາລາວ)
    pub holder_name_lao: Option<String>,
    /// Use type (ປະເພດການນຳໃຊ້)
    pub use_type: WaterUseType,
    /// Water source (ແຫຼ່ງນໍ້າ)
    pub water_source: WaterSourceType,
    /// Extraction limit m3/day (ຂີດຈຳກັດການສູບນໍ້າ)
    pub extraction_limit_m3_day: f64,
    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,
    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,
    /// Issuing authority (ອົງການອອກໃບອະນຸຍາດ)
    pub issuing_authority: String,
    /// Province (ແຂວງ)
    pub province: String,
    /// Permit status (ສະຖານະໃບອະນຸຍາດ)
    pub status: WaterPermitStatus,
    /// Conditions (ເງື່ອນໄຂ)
    pub conditions: Vec<PermitCondition>,
}

/// Water permit status (ສະຖານະໃບອະນຸຍາດນໍ້າ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WaterPermitStatus {
    /// Active (ມີຜົນບັງຄັບໃຊ້)
    Active,
    /// Pending (ລໍຖ້າ)
    Pending,
    /// Suspended (ໂຈະ)
    Suspended,
    /// Revoked (ຖືກຖອນ)
    Revoked,
    /// Expired (ໝົດອາຍຸ)
    Expired,
    /// Renewed (ຕໍ່ອາຍຸແລ້ວ)
    Renewed,
}

impl WaterPermitStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            WaterPermitStatus::Active => "ມີຜົນບັງຄັບໃຊ້",
            WaterPermitStatus::Pending => "ລໍຖ້າ",
            WaterPermitStatus::Suspended => "ໂຈະ",
            WaterPermitStatus::Revoked => "ຖືກຖອນ",
            WaterPermitStatus::Expired => "ໝົດອາຍຸ",
            WaterPermitStatus::Renewed => "ຕໍ່ອາຍຸແລ້ວ",
        }
    }
}

/// Permit condition (ເງື່ອນໄຂໃບອະນຸຍາດ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PermitCondition {
    /// Condition description (ລາຍລະອຽດເງື່ອນໄຂ)
    pub description: String,
    /// Condition description in Lao (ລາຍລະອຽດເປັນພາສາລາວ)
    pub description_lao: Option<String>,
    /// Compliance deadline (ກຳນົດການປະຕິບັດຕາມ)
    pub compliance_deadline: Option<String>,
    /// Compliance status (ສະຖານະການປະຕິບັດຕາມ)
    pub compliant: bool,
}

// ============================================================================
// Water Allocation Framework (ກອບການຈັດສັນນໍ້າ)
// ============================================================================

/// Season type (ປະເພດລະດູການ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Season {
    /// Wet season (May-October) (ລະດູຝົນ)
    Wet,
    /// Dry season (November-April) (ລະດູແລ້ງ)
    Dry,
    /// Transition period (ໄລຍະປ່ຽນຜ່ານ)
    Transition,
}

impl Season {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            Season::Wet => "ລະດູຝົນ",
            Season::Dry => "ລະດູແລ້ງ",
            Season::Transition => "ໄລຍະປ່ຽນຜ່ານ",
        }
    }

    /// Get season from month (1-12)
    pub fn from_month(month: u8) -> Self {
        match month {
            5..=10 => Season::Wet,
            11 | 12 | 1..=4 => Season::Dry,
            _ => Season::Transition,
        }
    }
}

/// Drought severity level (ລະດັບຄວາມຮຸນແຮງຂອງຄວາມແຫ້ງແລ້ງ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DroughtLevel {
    /// Normal conditions (ສະພາບປົກກະຕິ)
    Normal,
    /// Watch - early warning (ເຝົ້າລະວັງ)
    Watch,
    /// Warning - moderate drought (ເຕືອນ)
    Warning,
    /// Emergency - severe drought (ສຸກເສີນ)
    Emergency,
    /// Crisis - extreme drought (ວິກິດ)
    Crisis,
}

impl DroughtLevel {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            DroughtLevel::Normal => "ສະພາບປົກກະຕິ",
            DroughtLevel::Watch => "ເຝົ້າລະວັງ",
            DroughtLevel::Warning => "ເຕືອນ",
            DroughtLevel::Emergency => "ສຸກເສີນ",
            DroughtLevel::Crisis => "ວິກິດ",
        }
    }

    /// Get water use restrictions (percentage reduction)
    pub fn use_restrictions(&self) -> DroughtRestrictions {
        match self {
            DroughtLevel::Normal => DroughtRestrictions {
                domestic_reduction_pct: 0.0,
                agricultural_reduction_pct: 0.0,
                industrial_reduction_pct: 0.0,
                hydropower_reduction_pct: 0.0,
            },
            DroughtLevel::Watch => DroughtRestrictions {
                domestic_reduction_pct: 0.0,
                agricultural_reduction_pct: 10.0,
                industrial_reduction_pct: 10.0,
                hydropower_reduction_pct: 5.0,
            },
            DroughtLevel::Warning => DroughtRestrictions {
                domestic_reduction_pct: 0.0,
                agricultural_reduction_pct: 25.0,
                industrial_reduction_pct: 25.0,
                hydropower_reduction_pct: 15.0,
            },
            DroughtLevel::Emergency => DroughtRestrictions {
                domestic_reduction_pct: 10.0,
                agricultural_reduction_pct: 50.0,
                industrial_reduction_pct: 50.0,
                hydropower_reduction_pct: 30.0,
            },
            DroughtLevel::Crisis => DroughtRestrictions {
                domestic_reduction_pct: 20.0,
                agricultural_reduction_pct: 75.0,
                industrial_reduction_pct: 75.0,
                hydropower_reduction_pct: 50.0,
            },
        }
    }
}

/// Drought restrictions (ການຈຳກັດໃນຍາມແຫ້ງແລ້ງ)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DroughtRestrictions {
    /// Domestic use reduction percentage
    pub domestic_reduction_pct: f64,
    /// Agricultural use reduction percentage
    pub agricultural_reduction_pct: f64,
    /// Industrial use reduction percentage
    pub industrial_reduction_pct: f64,
    /// Hydropower use reduction percentage
    pub hydropower_reduction_pct: f64,
}

/// Water allocation (ການຈັດສັນນໍ້າ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WaterAllocation {
    /// Allocation ID (ລະຫັດການຈັດສັນ)
    pub allocation_id: String,
    /// Water source (ແຫຼ່ງນໍ້າ)
    pub water_source: String,
    /// Season (ລະດູການ)
    pub season: Season,
    /// Year (ປີ)
    pub year: u32,
    /// Total available m3 (ນໍ້າທັງໝົດທີ່ມີ)
    pub total_available_m3: f64,
    /// Domestic allocation m3 (ການຈັດສັນຄົວເຮືອນ)
    pub domestic_allocation_m3: f64,
    /// Agricultural allocation m3 (ການຈັດສັນກະສິກຳ)
    pub agricultural_allocation_m3: f64,
    /// Industrial allocation m3 (ການຈັດສັນອຸດສາຫະກຳ)
    pub industrial_allocation_m3: f64,
    /// Hydropower allocation m3 (ການຈັດສັນໄຟຟ້ານໍ້າຕົກ)
    pub hydropower_allocation_m3: f64,
    /// Environmental flow m3 (ການໄຫຼເພື່ອສິ່ງແວດລ້ອມ)
    pub environmental_flow_m3: f64,
    /// Drought level (ລະດັບຄວາມແຫ້ງແລ້ງ)
    pub drought_level: DroughtLevel,
}

// ============================================================================
// Hydropower Regulations (ລະບຽບການໄຟຟ້ານໍ້າຕົກ)
// ============================================================================

/// Hydropower project category (ປະເພດໂຄງການໄຟຟ້ານໍ້າຕົກ)
///
/// Article 46: Hydropower classification by capacity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HydropowerCategory {
    /// Small hydropower < 15 MW (ໄຟຟ້ານໍ້າຕົກຂະໜາດນ້ອຍ)
    Small,
    /// Medium hydropower 15-100 MW (ໄຟຟ້ານໍ້າຕົກຂະໜາດກາງ)
    Medium,
    /// Large hydropower > 100 MW (ໄຟຟ້ານໍ້າຕົກຂະໜາດໃຫຍ່)
    Large,
}

impl HydropowerCategory {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            HydropowerCategory::Small => "ໄຟຟ້ານໍ້າຕົກຂະໜາດນ້ອຍ",
            HydropowerCategory::Medium => "ໄຟຟ້ານໍ້າຕົກຂະໜາດກາງ",
            HydropowerCategory::Large => "ໄຟຟ້ານໍ້າຕົກຂະໜາດໃຫຍ່",
        }
    }

    /// Get category from capacity (MW)
    pub fn from_capacity(capacity_mw: f64) -> Self {
        if capacity_mw < SMALL_HYDROPOWER_THRESHOLD_MW {
            HydropowerCategory::Small
        } else if capacity_mw <= MEDIUM_HYDROPOWER_THRESHOLD_MW {
            HydropowerCategory::Medium
        } else {
            HydropowerCategory::Large
        }
    }

    /// Get standard concession period (years)
    pub fn standard_concession_years(&self) -> u32 {
        match self {
            HydropowerCategory::Small => 25,
            HydropowerCategory::Medium => 27,
            HydropowerCategory::Large => 30,
        }
    }

    /// Check if requires MRC prior consultation
    pub fn requires_mrc_consultation(&self, on_mainstream: bool) -> bool {
        on_mainstream || matches!(self, HydropowerCategory::Large)
    }
}

/// Hydropower concession (ສຳປະທານໄຟຟ້ານໍ້າຕົກ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HydropowerConcession {
    /// Concession number (ເລກສຳປະທານ)
    pub concession_number: String,
    /// Project name (ຊື່ໂຄງການ)
    pub project_name: String,
    /// Project name in Lao (ຊື່ໂຄງການເປັນພາສາລາວ)
    pub project_name_lao: Option<String>,
    /// Developer name (ຊື່ຜູ້ພັດທະນາ)
    pub developer_name: String,
    /// Installed capacity MW (ກຳລັງຕິດຕັ້ງ)
    pub installed_capacity_mw: f64,
    /// Category (ປະເພດ)
    pub category: HydropowerCategory,
    /// River name (ຊື່ແມ່ນໍ້າ)
    pub river_name: String,
    /// On Mekong mainstream (ຢູ່ແມ່ນໍ້າຂອງສາຍຫຼັກ)
    pub on_mekong_mainstream: bool,
    /// Province (ແຂວງ)
    pub province: String,
    /// Reservoir area hectares (ເນື້ອທີ່ອ່າງເກັບນໍ້າ)
    pub reservoir_area_hectares: Option<f64>,
    /// Concession start date (ວັນທີເລີ່ມສຳປະທານ)
    pub start_date: String,
    /// Concession end date (ວັນທີສິ້ນສຸດສຳປະທານ)
    pub end_date: String,
    /// Concession period years (ໄລຍະສຳປະທານ)
    pub concession_years: u32,
    /// Status (ສະຖານະ)
    pub status: ConcessionStatus,
    /// Power purchase agreement (ສັນຍາຊື້ຂາຍໄຟຟ້າ)
    pub ppa: Option<PowerPurchaseAgreement>,
    /// Minimum environmental flow m3/s (ການໄຫຼຂັ້ນຕໍ່າເພື່ອສິ່ງແວດລ້ອມ)
    pub minimum_environmental_flow_m3s: f64,
    /// Resettlement plan (ແຜນຍົກຍ້າຍຈັດສັນ)
    pub resettlement_plan: Option<ResettlementPlan>,
    /// MRC prior consultation completed (ສຳເລັດການປຶກສາຫາລືລ່ວງໜ້າ MRC)
    pub mrc_consultation_completed: bool,
}

impl HydropowerConcession {
    /// Check if concession is valid
    pub fn is_valid(&self) -> bool {
        matches!(
            self.status,
            ConcessionStatus::Active | ConcessionStatus::Construction
        )
    }
}

/// Concession status (ສະຖານະສຳປະທານ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConcessionStatus {
    /// Planning phase (ໄລຍະວາງແຜນ)
    Planning,
    /// Under construction (ກຳລັງກໍ່ສ້າງ)
    Construction,
    /// Active/Operating (ດຳເນີນງານ)
    Active,
    /// Suspended (ໂຈະ)
    Suspended,
    /// Terminated (ສິ້ນສຸດ)
    Terminated,
    /// Transferred (ໂອນແລ້ວ)
    Transferred,
}

impl ConcessionStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ConcessionStatus::Planning => "ໄລຍະວາງແຜນ",
            ConcessionStatus::Construction => "ກຳລັງກໍ່ສ້າງ",
            ConcessionStatus::Active => "ດຳເນີນງານ",
            ConcessionStatus::Suspended => "ໂຈະ",
            ConcessionStatus::Terminated => "ສິ້ນສຸດ",
            ConcessionStatus::Transferred => "ໂອນແລ້ວ",
        }
    }
}

/// Power purchase agreement (ສັນຍາຊື້ຂາຍໄຟຟ້າ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerPurchaseAgreement {
    /// PPA number (ເລກ PPA)
    pub ppa_number: String,
    /// Off-taker (ຜູ້ຊື້)
    pub off_taker: String,
    /// Export destination (ປະເທດສົ່ງອອກ)
    pub export_destination: Option<String>,
    /// Contracted capacity MW (ກຳລັງສັນຍາ)
    pub contracted_capacity_mw: f64,
    /// Tariff LAK/kWh (ອັດຕາຄ່າໄຟ)
    pub tariff_lak_kwh: f64,
    /// Start date (ວັນທີເລີ່ມ)
    pub start_date: String,
    /// End date (ວັນທີສິ້ນສຸດ)
    pub end_date: String,
    /// Domestic allocation percentage (ເປີເຊັນຈັດສັນພາຍໃນ)
    pub domestic_allocation_pct: f64,
}

/// Resettlement plan (ແຜນຍົກຍ້າຍຈັດສັນ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResettlementPlan {
    /// Affected households (ຄົວເຮືອນທີ່ໄດ້ຮັບຜົນກະທົບ)
    pub affected_households: u32,
    /// Affected persons (ບຸກຄົນທີ່ໄດ້ຮັບຜົນກະທົບ)
    pub affected_persons: u32,
    /// Villages affected (ບ້ານທີ່ໄດ້ຮັບຜົນກະທົບ)
    pub villages_affected: Vec<String>,
    /// Compensation budget LAK (ງົບປະມານຊົດເຊີຍ)
    pub compensation_budget_lak: u64,
    /// New settlement location (ສະຖານທີ່ຕັ້ງຖິ່ນຖານໃໝ່)
    pub new_settlement_location: Option<String>,
    /// Livelihood restoration plan (ແຜນຟື້ນຟູການດຳລົງຊີວິດ)
    pub livelihood_restoration_included: bool,
    /// Approval status (ສະຖານະການອະນຸມັດ)
    pub approval_status: ResettlementApprovalStatus,
}

/// Resettlement approval status (ສະຖານະການອະນຸມັດແຜນຍົກຍ້າຍ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ResettlementApprovalStatus {
    /// Draft (ຮ່າງ)
    Draft,
    /// Under review (ກຳລັງພິຈາລະນາ)
    UnderReview,
    /// Approved (ອະນຸມັດແລ້ວ)
    Approved,
    /// Implementation (ກຳລັງປະຕິບັດ)
    Implementation,
    /// Completed (ສຳເລັດ)
    Completed,
}

impl ResettlementApprovalStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ResettlementApprovalStatus::Draft => "ຮ່າງ",
            ResettlementApprovalStatus::UnderReview => "ກຳລັງພິຈາລະນາ",
            ResettlementApprovalStatus::Approved => "ອະນຸມັດແລ້ວ",
            ResettlementApprovalStatus::Implementation => "ກຳລັງປະຕິບັດ",
            ResettlementApprovalStatus::Completed => "ສຳເລັດ",
        }
    }
}

// ============================================================================
// Water Quality Standards (ມາດຕະຖານຄຸນນະພາບນໍ້າ)
// ============================================================================

/// Water quality class (ລະດັບຄຸນນະພາບນໍ້າ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WaterQualityClass {
    /// Class 1 - Drinking water source (ນໍ້າດື່ມ)
    Class1Drinking,
    /// Class 2 - Agricultural use (ນໍ້າກະສິກຳ)
    Class2Agricultural,
    /// Class 3 - Industrial use (ນໍ້າອຸດສາຫະກຳ)
    Class3Industrial,
    /// Class 4 - Navigation/Recreation (ນໍ້າເດີນເຮືອ/ພັກຜ່ອນ)
    Class4Recreation,
}

impl WaterQualityClass {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            WaterQualityClass::Class1Drinking => "ລະດັບ 1 - ນໍ້າດື່ມ",
            WaterQualityClass::Class2Agricultural => "ລະດັບ 2 - ນໍ້າກະສິກຳ",
            WaterQualityClass::Class3Industrial => "ລະດັບ 3 - ນໍ້າອຸດສາຫະກຳ",
            WaterQualityClass::Class4Recreation => "ລະດັບ 4 - ນໍ້າເດີນເຮືອ/ພັກຜ່ອນ",
        }
    }
}

/// Water quality parameter (ພາລາມິເຕີຄຸນນະພາບນໍ້າ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WaterQualityParameter {
    /// pH level
    Ph,
    /// Turbidity (NTU)
    Turbidity,
    /// Total Dissolved Solids (mg/L)
    Tds,
    /// Biochemical Oxygen Demand (mg/L)
    Bod,
    /// Chemical Oxygen Demand (mg/L)
    Cod,
    /// Total Suspended Solids (mg/L)
    Tss,
    /// Dissolved Oxygen (mg/L)
    DissolvedOxygen,
    /// Arsenic (mg/L)
    Arsenic,
    /// Lead (mg/L)
    Lead,
    /// Mercury (mg/L)
    Mercury,
    /// Cadmium (mg/L)
    Cadmium,
    /// E. coli (CFU/100mL)
    EColi,
    /// Total Coliform (CFU/100mL)
    TotalColiform,
    /// Nitrate (mg/L)
    Nitrate,
    /// Phosphate (mg/L)
    Phosphate,
    /// Temperature (C)
    Temperature,
}

impl WaterQualityParameter {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            WaterQualityParameter::Ph => "ຄ່າ pH",
            WaterQualityParameter::Turbidity => "ຄວາມຂຸ່ນ",
            WaterQualityParameter::Tds => "ຂອງແຂງລະລາຍທັງໝົດ",
            WaterQualityParameter::Bod => "BOD",
            WaterQualityParameter::Cod => "COD",
            WaterQualityParameter::Tss => "TSS",
            WaterQualityParameter::DissolvedOxygen => "ອົກຊີເຈນລະລາຍ",
            WaterQualityParameter::Arsenic => "ສານຫນູ",
            WaterQualityParameter::Lead => "ຕະກົ່ວ",
            WaterQualityParameter::Mercury => "ປອດ",
            WaterQualityParameter::Cadmium => "ແຄດມຽມ",
            WaterQualityParameter::EColi => "E. coli",
            WaterQualityParameter::TotalColiform => "ໂຄລິຟອມທັງໝົດ",
            WaterQualityParameter::Nitrate => "ໄນເຕຣດ",
            WaterQualityParameter::Phosphate => "ຟອສເຟດ",
            WaterQualityParameter::Temperature => "ອຸນຫະພູມ",
        }
    }

    /// Get unit
    pub fn unit(&self) -> &'static str {
        match self {
            WaterQualityParameter::Ph => "",
            WaterQualityParameter::Turbidity => "NTU",
            WaterQualityParameter::Temperature => "C",
            WaterQualityParameter::EColi | WaterQualityParameter::TotalColiform => "CFU/100mL",
            _ => "mg/L",
        }
    }

    /// Get drinking water limit (Article 55)
    pub fn drinking_water_limit(&self) -> Option<f64> {
        match self {
            WaterQualityParameter::Ph => None, // Range check needed
            WaterQualityParameter::Turbidity => Some(DRINKING_WATER_MAX_TURBIDITY_NTU),
            WaterQualityParameter::Tds => Some(500.0),
            WaterQualityParameter::Arsenic => Some(DRINKING_WATER_MAX_ARSENIC_MG_L),
            WaterQualityParameter::Lead => Some(DRINKING_WATER_MAX_LEAD_MG_L),
            WaterQualityParameter::Mercury => Some(0.001),
            WaterQualityParameter::Cadmium => Some(0.003),
            WaterQualityParameter::EColi => Some(DRINKING_WATER_MAX_ECOLI),
            WaterQualityParameter::TotalColiform => Some(0.0),
            WaterQualityParameter::Nitrate => Some(50.0),
            _ => None,
        }
    }
}

/// Water quality measurement (ການວັດແທກຄຸນນະພາບນໍ້າ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WaterQualityMeasurement {
    /// Sample ID (ລະຫັດຕົວຢ່າງ)
    pub sample_id: String,
    /// Location name (ຊື່ສະຖານທີ່)
    pub location_name: String,
    /// Water source (ແຫຼ່ງນໍ້າ)
    pub water_source: String,
    /// Sampling date (ວັນທີເກັບຕົວຢ່າງ)
    pub sampling_date: String,
    /// Parameter (ພາລາມິເຕີ)
    pub parameter: WaterQualityParameter,
    /// Value (ຄ່າ)
    pub value: f64,
    /// Quality class (ລະດັບຄຸນນະພາບ)
    pub quality_class: WaterQualityClass,
    /// Compliant (ປະຕິບັດຕາມ)
    pub compliant: bool,
}

// ============================================================================
// Mekong River Commission Compliance (ການປະຕິບັດຕາມຄະນະກຳມາທິການແມ່ນໍ້າຂອງ)
// ============================================================================

/// MRC procedure type (ປະເພດຂັ້ນຕອນ MRC)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MRCProcedureType {
    /// Prior Consultation (PNPCA) (ການປຶກສາຫາລືລ່ວງໜ້າ)
    PriorConsultation,
    /// Notification (ການແຈ້ງເຕືອນ)
    Notification,
    /// Agreement (ການຕົກລົງ)
    Agreement,
}

impl MRCProcedureType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            MRCProcedureType::PriorConsultation => "ການປຶກສາຫາລືລ່ວງໜ້າ",
            MRCProcedureType::Notification => "ການແຈ້ງເຕືອນ",
            MRCProcedureType::Agreement => "ການຕົກລົງ",
        }
    }
}

/// MRC compliance record (ບັນທຶກການປະຕິບັດຕາມ MRC)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MRCComplianceRecord {
    /// Record ID (ລະຫັດບັນທຶກ)
    pub record_id: String,
    /// Project name (ຊື່ໂຄງການ)
    pub project_name: String,
    /// Procedure type (ປະເພດຂັ້ນຕອນ)
    pub procedure_type: MRCProcedureType,
    /// Submission date (ວັນທີສົ່ງ)
    pub submission_date: String,
    /// Response deadline (ກຳນົດຕອບ)
    pub response_deadline: String,
    /// Status (ສະຖານະ)
    pub status: MRCComplianceStatus,
    /// Transboundary impact assessment (ການປະເມີນຜົນກະທົບຂ້າມຊາຍແດນ)
    pub transboundary_assessment_completed: bool,
    /// Countries notified (ປະເທດທີ່ແຈ້ງ)
    pub countries_notified: Vec<String>,
    /// Data shared (ຂໍ້ມູນທີ່ແບ່ງປັນ)
    pub data_shared: Vec<String>,
}

/// MRC compliance status (ສະຖານະການປະຕິບັດຕາມ MRC)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MRCComplianceStatus {
    /// Pending submission (ລໍຖ້າສົ່ງ)
    PendingSubmission,
    /// Submitted (ສົ່ງແລ້ວ)
    Submitted,
    /// Under review (ກຳລັງພິຈາລະນາ)
    UnderReview,
    /// Additional information requested (ຕ້ອງການຂໍ້ມູນເພີ່ມເຕີມ)
    AdditionalInfoRequested,
    /// Completed (ສຳເລັດ)
    Completed,
    /// Expired (ໝົດອາຍຸ)
    Expired,
}

impl MRCComplianceStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            MRCComplianceStatus::PendingSubmission => "ລໍຖ້າສົ່ງ",
            MRCComplianceStatus::Submitted => "ສົ່ງແລ້ວ",
            MRCComplianceStatus::UnderReview => "ກຳລັງພິຈາລະນາ",
            MRCComplianceStatus::AdditionalInfoRequested => "ຕ້ອງການຂໍ້ມູນເພີ່ມເຕີມ",
            MRCComplianceStatus::Completed => "ສຳເລັດ",
            MRCComplianceStatus::Expired => "ໝົດອາຍຸ",
        }
    }
}

// ============================================================================
// Irrigation Districts (ເຂດຊົນລະປະທານ)
// ============================================================================

/// Water User Association (ສະມາຄົມຜູ້ນຳໃຊ້ນໍ້າ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WaterUserAssociation {
    /// Registration number (ເລກທະບຽນ)
    pub registration_number: String,
    /// Name (ຊື່)
    pub name: String,
    /// Name in Lao (ຊື່ເປັນພາສາລາວ)
    pub name_lao: String,
    /// Province (ແຂວງ)
    pub province: String,
    /// District (ເມືອງ)
    pub district: String,
    /// Villages covered (ບ້ານທີ່ກວມເອົາ)
    pub villages: Vec<String>,
    /// Members count (ຈຳນວນສະມາຊິກ)
    pub members_count: u32,
    /// Command area hectares (ເນື້ອທີ່ຄວບຄຸມ)
    pub command_area_hectares: f64,
    /// Water source (ແຫຼ່ງນໍ້າ)
    pub water_source: String,
    /// Registration date (ວັນທີລົງທະບຽນ)
    pub registration_date: String,
    /// Status (ສະຖານະ)
    pub status: WUAStatus,
    /// Annual fee collection LAK (ການເກັບຄ່າທຳນຽມປະຈຳປີ)
    pub annual_fee_collection_lak: u64,
}

/// WUA status (ສະຖານະ WUA)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WUAStatus {
    /// Active (ເຄື່ອນໄຫວ)
    Active,
    /// Inactive (ບໍ່ເຄື່ອນໄຫວ)
    Inactive,
    /// Under formation (ກຳລັງສ້າງຕັ້ງ)
    UnderFormation,
    /// Dissolved (ຍຸບ)
    Dissolved,
}

impl WUAStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            WUAStatus::Active => "ເຄື່ອນໄຫວ",
            WUAStatus::Inactive => "ບໍ່ເຄື່ອນໄຫວ",
            WUAStatus::UnderFormation => "ກຳລັງສ້າງຕັ້ງ",
            WUAStatus::Dissolved => "ຍຸບ",
        }
    }
}

/// Irrigation service fee record (ບັນທຶກຄ່າບໍລິການຊົນລະປະທານ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IrrigationServiceFee {
    /// Fee ID (ລະຫັດຄ່າທຳນຽມ)
    pub fee_id: String,
    /// WUA registration number (ເລກທະບຽນ WUA)
    pub wua_registration_number: String,
    /// Member name (ຊື່ສະມາຊິກ)
    pub member_name: String,
    /// Area serviced hectares (ເນື້ອທີ່ໃຫ້ບໍລິການ)
    pub area_serviced_hectares: f64,
    /// Season (ລະດູການ)
    pub season: Season,
    /// Year (ປີ)
    pub year: u32,
    /// Amount due LAK (ຈຳນວນທີ່ຕ້ອງຈ່າຍ)
    pub amount_due_lak: u64,
    /// Amount paid LAK (ຈຳນວນທີ່ຈ່າຍແລ້ວ)
    pub amount_paid_lak: u64,
    /// Due date (ກຳນົດຈ່າຍ)
    pub due_date: String,
    /// Payment date (ວັນທີຈ່າຍ)
    pub payment_date: Option<String>,
    /// Status (ສະຖານະ)
    pub status: FeePaymentStatus,
}

/// Fee payment status (ສະຖານະການຈ່າຍຄ່າທຳນຽມ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FeePaymentStatus {
    /// Pending (ລໍຖ້າ)
    Pending,
    /// Paid (ຈ່າຍແລ້ວ)
    Paid,
    /// Overdue (ຄ້າງຊຳລະ)
    Overdue,
    /// Waived (ຍົກເວັ້ນ)
    Waived,
    /// Partial (ບາງສ່ວນ)
    Partial,
}

impl FeePaymentStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            FeePaymentStatus::Pending => "ລໍຖ້າ",
            FeePaymentStatus::Paid => "ຈ່າຍແລ້ວ",
            FeePaymentStatus::Overdue => "ຄ້າງຊຳລະ",
            FeePaymentStatus::Waived => "ຍົກເວັ້ນ",
            FeePaymentStatus::Partial => "ບາງສ່ວນ",
        }
    }
}

// ============================================================================
// Groundwater Management (ການຄຸ້ມຄອງນໍ້າໃຕ້ດິນ)
// ============================================================================

/// Well permit (ໃບອະນຸຍາດບໍ່ນໍ້າ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WellPermit {
    /// Permit number (ເລກໃບອະນຸຍາດ)
    pub permit_number: String,
    /// Owner name (ຊື່ເຈົ້າຂອງ)
    pub owner_name: String,
    /// Owner name in Lao (ຊື່ເຈົ້າຂອງເປັນພາສາລາວ)
    pub owner_name_lao: Option<String>,
    /// Well location (ສະຖານທີ່ບໍ່)
    pub location: String,
    /// Province (ແຂວງ)
    pub province: String,
    /// District (ເມືອງ)
    pub district: String,
    /// Well depth meters (ຄວາມເລິກບໍ່)
    pub depth_meters: f64,
    /// Well diameter cm (ຂະໜາດເສັ້ນຜ່ານສູນກາງ)
    pub diameter_cm: f64,
    /// Aquifer type (ປະເພດຊັ້ນນໍ້າໃຕ້ດິນ)
    pub aquifer_type: AquiferType,
    /// Permitted extraction m3/day (ການສູບທີ່ອະນຸຍາດ)
    pub permitted_extraction_m3_day: f64,
    /// Purpose (ຈຸດປະສົງ)
    pub purpose: WaterUseType,
    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,
    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,
    /// Status (ສະຖານະ)
    pub status: WaterPermitStatus,
    /// In protection zone (ຢູ່ໃນເຂດປ້ອງກັນ)
    pub in_protection_zone: bool,
}

/// Groundwater monitoring record (ບັນທຶກການຕິດຕາມນໍ້າໃຕ້ດິນ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GroundwaterMonitoringRecord {
    /// Record ID (ລະຫັດບັນທຶກ)
    pub record_id: String,
    /// Well permit number (ເລກໃບອະນຸຍາດບໍ່)
    pub well_permit_number: String,
    /// Monitoring date (ວັນທີຕິດຕາມ)
    pub monitoring_date: String,
    /// Water level meters below ground (ລະດັບນໍ້າຕໍ່າກວ່າໜ້າດິນ)
    pub water_level_m: f64,
    /// Extraction m3/day (ການສູບ)
    pub extraction_m3_day: f64,
    /// Water quality measurements (ການວັດແທກຄຸນນະພາບນໍ້າ)
    pub quality_measurements: Vec<WaterQualityMeasurement>,
    /// Submitted to authority (ສົ່ງໃຫ້ເຈົ້າໜ້າທີ່ແລ້ວ)
    pub submitted: bool,
}

/// Aquifer protection zone (ເຂດປ້ອງກັນຊັ້ນນໍ້າໃຕ້ດິນ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AquiferProtectionZone {
    /// Zone ID (ລະຫັດເຂດ)
    pub zone_id: String,
    /// Zone name (ຊື່ເຂດ)
    pub name: String,
    /// Zone name in Lao (ຊື່ເຂດເປັນພາສາລາວ)
    pub name_lao: String,
    /// Province (ແຂວງ)
    pub province: String,
    /// Area hectares (ເນື້ອທີ່)
    pub area_hectares: f64,
    /// Protection level (ລະດັບການປ້ອງກັນ)
    pub protection_level: ProtectionLevel,
    /// Prohibited activities (ກິດຈະກຳທີ່ຫ້າມ)
    pub prohibited_activities: Vec<String>,
    /// Establishment date (ວັນທີສ້າງຕັ້ງ)
    pub establishment_date: String,
}

/// Protection level (ລະດັບການປ້ອງກັນ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProtectionLevel {
    /// Low (ຕໍ່າ)
    Low,
    /// Medium (ກາງ)
    Medium,
    /// High (ສູງ)
    High,
    /// Critical (ວິກິດ)
    Critical,
}

impl ProtectionLevel {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ProtectionLevel::Low => "ຕໍ່າ",
            ProtectionLevel::Medium => "ກາງ",
            ProtectionLevel::High => "ສູງ",
            ProtectionLevel::Critical => "ວິກິດ",
        }
    }
}

// ============================================================================
// Pollution Prevention (ການປ້ອງກັນມົນລະພິດ)
// ============================================================================

/// Polluter record (ບັນທຶກຜູ້ກໍ່ມົນລະພິດ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PolluterRecord {
    /// Record ID (ລະຫັດບັນທຶກ)
    pub record_id: String,
    /// Polluter name (ຊື່ຜູ້ກໍ່ມົນລະພິດ)
    pub polluter_name: String,
    /// Polluter name in Lao (ຊື່ຜູ້ກໍ່ມົນລະພິດເປັນພາສາລາວ)
    pub polluter_name_lao: Option<String>,
    /// Pollution type (ປະເພດມົນລະພິດ)
    pub pollution_type: PollutionType,
    /// Affected water source (ແຫຼ່ງນໍ້າທີ່ໄດ້ຮັບຜົນກະທົບ)
    pub affected_water_source: String,
    /// Incident date (ວັນທີເກີດເຫດ)
    pub incident_date: String,
    /// Remediation required (ຕ້ອງແກ້ໄຂ)
    pub remediation_required: bool,
    /// Remediation cost LAK (ຄ່າໃຊ້ຈ່າຍແກ້ໄຂ)
    pub remediation_cost_lak: u64,
    /// Remediation status (ສະຖານະການແກ້ໄຂ)
    pub remediation_status: RemediationStatus,
    /// Penalty imposed LAK (ໂທດທີ່ກຳນົດ)
    pub penalty_lak: Option<u64>,
}

/// Pollution type (ປະເພດມົນລະພິດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PollutionType {
    /// Industrial discharge (ການປ່ອຍນໍ້າເສຍອຸດສາຫະກຳ)
    IndustrialDischarge,
    /// Agricultural runoff (ນໍ້າໄຫຼລົ້ນຈາກກະສິກຳ)
    AgriculturalRunoff,
    /// Mining effluent (ນໍ້າເສຍຈາກບໍ່ແຮ່)
    MiningEffluent,
    /// Sewage (ນໍ້າເສຍຄົວເຮືອນ)
    Sewage,
    /// Oil spill (ນໍ້າມັນຮົ່ວ)
    OilSpill,
    /// Chemical contamination (ການປົນເປື້ອນສານເຄມີ)
    ChemicalContamination,
}

impl PollutionType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            PollutionType::IndustrialDischarge => "ການປ່ອຍນໍ້າເສຍອຸດສາຫະກຳ",
            PollutionType::AgriculturalRunoff => "ນໍ້າໄຫຼລົ້ນຈາກກະສິກຳ",
            PollutionType::MiningEffluent => "ນໍ້າເສຍຈາກບໍ່ແຮ່",
            PollutionType::Sewage => "ນໍ້າເສຍຄົວເຮືອນ",
            PollutionType::OilSpill => "ນໍ້າມັນຮົ່ວ",
            PollutionType::ChemicalContamination => "ການປົນເປື້ອນສານເຄມີ",
        }
    }
}

/// Remediation status (ສະຖານະການແກ້ໄຂ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RemediationStatus {
    /// Pending (ລໍຖ້າ)
    Pending,
    /// In progress (ກຳລັງດຳເນີນ)
    InProgress,
    /// Completed (ສຳເລັດ)
    Completed,
    /// Verified (ກວດສອບແລ້ວ)
    Verified,
    /// Non-compliant (ບໍ່ປະຕິບັດຕາມ)
    NonCompliant,
}

impl RemediationStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            RemediationStatus::Pending => "ລໍຖ້າ",
            RemediationStatus::InProgress => "ກຳລັງດຳເນີນ",
            RemediationStatus::Completed => "ສຳເລັດ",
            RemediationStatus::Verified => "ກວດສອບແລ້ວ",
            RemediationStatus::NonCompliant => "ບໍ່ປະຕິບັດຕາມ",
        }
    }
}

/// Wastewater treatment facility (ສະຖານທີ່ບຳບັດນໍ້າເສຍ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WastewaterTreatmentFacility {
    /// Facility ID (ລະຫັດສະຖານທີ່)
    pub facility_id: String,
    /// Facility name (ຊື່ສະຖານທີ່)
    pub name: String,
    /// Facility name in Lao (ຊື່ສະຖານທີ່ເປັນພາສາລາວ)
    pub name_lao: Option<String>,
    /// Operator name (ຊື່ຜູ້ດຳເນີນງານ)
    pub operator_name: String,
    /// Province (ແຂວງ)
    pub province: String,
    /// Treatment capacity m3/day (ກຳລັງການບຳບັດ)
    pub capacity_m3_day: f64,
    /// Treatment type (ປະເພດການບຳບັດ)
    pub treatment_type: TreatmentType,
    /// Discharge point (ຈຸດປ່ອຍ)
    pub discharge_point: String,
    /// Permit number (ເລກໃບອະນຸຍາດ)
    pub permit_number: Option<String>,
    /// Operational status (ສະຖານະການດຳເນີນງານ)
    pub operational_status: FacilityStatus,
}

/// Treatment type (ປະເພດການບຳບັດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TreatmentType {
    /// Primary treatment (ການບຳບັດຂັ້ນຕົ້ນ)
    Primary,
    /// Secondary treatment (ການບຳບັດຂັ້ນທີ 2)
    Secondary,
    /// Tertiary treatment (ການບຳບັດຂັ້ນທີ 3)
    Tertiary,
    /// Advanced treatment (ການບຳບັດຂັ້ນສູງ)
    Advanced,
}

impl TreatmentType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            TreatmentType::Primary => "ການບຳບັດຂັ້ນຕົ້ນ",
            TreatmentType::Secondary => "ການບຳບັດຂັ້ນທີ 2",
            TreatmentType::Tertiary => "ການບຳບັດຂັ້ນທີ 3",
            TreatmentType::Advanced => "ການບຳບັດຂັ້ນສູງ",
        }
    }
}

/// Facility status (ສະຖານະສະຖານທີ່)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FacilityStatus {
    /// Operational (ດຳເນີນງານ)
    Operational,
    /// Under construction (ກຳລັງກໍ່ສ້າງ)
    UnderConstruction,
    /// Maintenance (ບຳລຸງຮັກສາ)
    Maintenance,
    /// Closed (ປິດ)
    Closed,
}

impl FacilityStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            FacilityStatus::Operational => "ດຳເນີນງານ",
            FacilityStatus::UnderConstruction => "ກຳລັງກໍ່ສ້າງ",
            FacilityStatus::Maintenance => "ບຳລຸງຮັກສາ",
            FacilityStatus::Closed => "ປິດ",
        }
    }
}

// ============================================================================
// Builders (ຕົວສ້າງ)
// ============================================================================

/// Builder for HydropowerConcession
#[derive(Debug, Default)]
pub struct HydropowerConcessionBuilder {
    concession: HydropowerConcession,
}

impl Default for HydropowerConcession {
    fn default() -> Self {
        Self {
            concession_number: String::new(),
            project_name: String::new(),
            project_name_lao: None,
            developer_name: String::new(),
            installed_capacity_mw: 0.0,
            category: HydropowerCategory::Small,
            river_name: String::new(),
            on_mekong_mainstream: false,
            province: String::new(),
            reservoir_area_hectares: None,
            start_date: String::new(),
            end_date: String::new(),
            concession_years: HYDROPOWER_CONCESSION_MIN_YEARS,
            status: ConcessionStatus::Planning,
            ppa: None,
            minimum_environmental_flow_m3s: 0.0,
            resettlement_plan: None,
            mrc_consultation_completed: false,
        }
    }
}

impl HydropowerConcessionBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set concession number
    pub fn concession_number(mut self, number: impl Into<String>) -> Self {
        self.concession.concession_number = number.into();
        self
    }

    /// Set project name
    pub fn project_name(mut self, name: impl Into<String>) -> Self {
        self.concession.project_name = name.into();
        self
    }

    /// Set project name in Lao
    pub fn project_name_lao(mut self, name: impl Into<String>) -> Self {
        self.concession.project_name_lao = Some(name.into());
        self
    }

    /// Set developer name
    pub fn developer_name(mut self, name: impl Into<String>) -> Self {
        self.concession.developer_name = name.into();
        self
    }

    /// Set installed capacity (automatically sets category)
    pub fn installed_capacity_mw(mut self, capacity: f64) -> Self {
        self.concession.installed_capacity_mw = capacity;
        self.concession.category = HydropowerCategory::from_capacity(capacity);
        self
    }

    /// Set river name
    pub fn river_name(mut self, name: impl Into<String>) -> Self {
        self.concession.river_name = name.into();
        self
    }

    /// Set whether on Mekong mainstream
    pub fn on_mekong_mainstream(mut self, value: bool) -> Self {
        self.concession.on_mekong_mainstream = value;
        self
    }

    /// Set province
    pub fn province(mut self, province: impl Into<String>) -> Self {
        self.concession.province = province.into();
        self
    }

    /// Set reservoir area
    pub fn reservoir_area_hectares(mut self, area: f64) -> Self {
        self.concession.reservoir_area_hectares = Some(area);
        self
    }

    /// Set concession dates
    pub fn concession_dates(
        mut self,
        start: impl Into<String>,
        end: impl Into<String>,
        years: u32,
    ) -> Self {
        self.concession.start_date = start.into();
        self.concession.end_date = end.into();
        self.concession.concession_years = years;
        self
    }

    /// Set status
    pub fn status(mut self, status: ConcessionStatus) -> Self {
        self.concession.status = status;
        self
    }

    /// Set minimum environmental flow
    pub fn minimum_environmental_flow_m3s(mut self, flow: f64) -> Self {
        self.concession.minimum_environmental_flow_m3s = flow;
        self
    }

    /// Set MRC consultation completed
    pub fn mrc_consultation_completed(mut self, completed: bool) -> Self {
        self.concession.mrc_consultation_completed = completed;
        self
    }

    /// Set PPA
    pub fn ppa(mut self, ppa: PowerPurchaseAgreement) -> Self {
        self.concession.ppa = Some(ppa);
        self
    }

    /// Set resettlement plan
    pub fn resettlement_plan(mut self, plan: ResettlementPlan) -> Self {
        self.concession.resettlement_plan = Some(plan);
        self
    }

    /// Build the concession
    pub fn build(self) -> HydropowerConcession {
        self.concession
    }
}

/// Builder for WaterUseRight
#[derive(Debug, Default)]
pub struct WaterUseRightBuilder {
    right: WaterUseRight,
}

impl Default for WaterUseRight {
    fn default() -> Self {
        Self {
            permit_number: String::new(),
            holder_name: String::new(),
            holder_name_lao: None,
            use_type: WaterUseType::Domestic,
            water_source: WaterSourceType::SurfaceWater {
                body_type: SurfaceWaterBodyType::Stream,
                name: String::new(),
                catchment_area_km2: None,
            },
            extraction_limit_m3_day: 0.0,
            issue_date: String::new(),
            expiry_date: String::new(),
            issuing_authority: String::new(),
            province: String::new(),
            status: WaterPermitStatus::Pending,
            conditions: Vec::new(),
        }
    }
}

impl WaterUseRightBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set permit number
    pub fn permit_number(mut self, number: impl Into<String>) -> Self {
        self.right.permit_number = number.into();
        self
    }

    /// Set holder name
    pub fn holder_name(mut self, name: impl Into<String>) -> Self {
        self.right.holder_name = name.into();
        self
    }

    /// Set holder name in Lao
    pub fn holder_name_lao(mut self, name: impl Into<String>) -> Self {
        self.right.holder_name_lao = Some(name.into());
        self
    }

    /// Set use type
    pub fn use_type(mut self, use_type: WaterUseType) -> Self {
        self.right.use_type = use_type;
        self
    }

    /// Set water source
    pub fn water_source(mut self, source: WaterSourceType) -> Self {
        self.right.water_source = source;
        self
    }

    /// Set extraction limit
    pub fn extraction_limit_m3_day(mut self, limit: f64) -> Self {
        self.right.extraction_limit_m3_day = limit;
        self
    }

    /// Set permit dates
    pub fn permit_dates(mut self, issue: impl Into<String>, expiry: impl Into<String>) -> Self {
        self.right.issue_date = issue.into();
        self.right.expiry_date = expiry.into();
        self
    }

    /// Set issuing authority
    pub fn issuing_authority(mut self, authority: impl Into<String>) -> Self {
        self.right.issuing_authority = authority.into();
        self
    }

    /// Set province
    pub fn province(mut self, province: impl Into<String>) -> Self {
        self.right.province = province.into();
        self
    }

    /// Set status
    pub fn status(mut self, status: WaterPermitStatus) -> Self {
        self.right.status = status;
        self
    }

    /// Add condition
    pub fn add_condition(mut self, condition: PermitCondition) -> Self {
        self.right.conditions.push(condition);
        self
    }

    /// Build the water use right
    pub fn build(self) -> WaterUseRight {
        self.right
    }
}
