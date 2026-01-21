//! Water Law Error Types (ປະເພດຄວາມຜິດພາດກົດໝາຍຊັບພະຍາກອນນໍ້າ)
//!
//! Comprehensive error types for Lao water and water resources law validation.
//! All errors include bilingual messages (Lao/English) per Article 10 requirements.
//!
//! # Legal Reference
//! - Water and Water Resources Law 2017 (Law No. 23/NA) - ກົດໝາຍວ່າດ້ວຍນໍ້າ ແລະ ຊັບພະຍາກອນນໍ້າ ປີ 2017

use thiserror::Error;

/// Result type for water law operations
pub type Result<T> = std::result::Result<T, WaterLawError>;

/// Water law errors (ຄວາມຜິດພາດກົດໝາຍຊັບພະຍາກອນນໍ້າ)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum WaterLawError {
    // ========================================================================
    // Water Use Rights Errors (ຄວາມຜິດພາດສິດນຳໃຊ້ນໍ້າ)
    // ========================================================================
    /// Missing water use permit (Article 35)
    /// ຂາດໃບອະນຸຍາດນຳໃຊ້ນໍ້າ (ມາດຕາ 35)
    #[error(
        "Water use permit is required for {use_type} (Article 35)\nຕ້ອງມີໃບອະນຸຍາດນຳໃຊ້ນໍ້າສຳລັບ {use_type} (ມາດຕາ 35)"
    )]
    MissingWaterUsePermit { use_type: String },

    /// Water extraction exceeds permit limit (Article 36)
    /// ການສູບນໍ້າເກີນໃບອະນຸຍາດ (ມາດຕາ 36)
    #[error(
        "Water extraction {actual} m3/day exceeds permit limit of {limit} m3/day (Article 36)\nການສູບນໍ້າ {actual} ແມັດກ້ອນ/ມື້ ເກີນຂີດຈຳກັດ {limit} ແມັດກ້ອນ/ມື້ (ມາດຕາ 36)"
    )]
    ExtractionExceedsLimit { actual: f64, limit: f64 },

    /// Unauthorized water use (Article 37)
    /// ການນຳໃຊ້ນໍ້າໂດຍບໍ່ໄດ້ຮັບອະນຸຍາດ (ມາດຕາ 37)
    #[error(
        "Unauthorized water use: {description} (Article 37)\nການນຳໃຊ້ນໍ້າໂດຍບໍ່ໄດ້ຮັບອະນຸຍາດ: {description} (ມາດຕາ 37)"
    )]
    UnauthorizedWaterUse { description: String },

    /// Domestic water priority violation (Article 38)
    /// ການລະເມີດສິດທິບຸລິມະສິດນໍ້າຄົວເຮືອນ (ມາດຕາ 38)
    #[error(
        "Domestic water use has priority over {other_use} during water shortage (Article 38)\nການນຳໃຊ້ນໍ້າຄົວເຮືອນມີບຸລິມະສິດເໜືອ {other_use} ໃນຍາມຂາດແຄນນໍ້າ (ມາດຕາ 38)"
    )]
    DomesticPriorityViolation { other_use: String },

    // ========================================================================
    // Hydropower Errors (ຄວາມຜິດພາດໄຟຟ້ານໍ້າຕົກ)
    // ========================================================================
    /// Missing hydropower concession (Article 45)
    /// ຂາດສຳປະທານໄຟຟ້ານໍ້າຕົກ (ມາດຕາ 45)
    #[error(
        "Hydropower concession is required for {capacity_mw} MW project (Article 45)\nຕ້ອງມີສຳປະທານໄຟຟ້ານໍ້າຕົກສຳລັບໂຄງການ {capacity_mw} ເມກາວັດ (ມາດຕາ 45)"
    )]
    MissingHydropowerConcession { capacity_mw: f64 },

    /// Hydropower concession expired (Article 47)
    /// ສຳປະທານໄຟຟ້ານໍ້າຕົກໝົດອາຍຸ (ມາດຕາ 47)
    #[error(
        "Hydropower concession expired on {expiry_date} (Article 47)\nສຳປະທານໄຟຟ້ານໍ້າຕົກໝົດອາຍຸວັນທີ {expiry_date} (ມາດຕາ 47)"
    )]
    ConcessionExpired { expiry_date: String },

    /// Minimum environmental flow violation (Article 48)
    /// ການລະເມີດການໄຫຼຂັ້ນຕໍ່າຂອງສິ່ງແວດລ້ອມ (ມາດຕາ 48)
    #[error(
        "Environmental flow {actual} m3/s below minimum required {minimum} m3/s (Article 48)\nການໄຫຼຂອງສິ່ງແວດລ້ອມ {actual} ແມັດກ້ອນ/ວິນາທີ ຕໍ່າກວ່າຂັ້ນຕໍ່າ {minimum} ແມັດກ້ອນ/ວິນາທີ (ມາດຕາ 48)"
    )]
    MinimumFlowViolation { actual: f64, minimum: f64 },

    /// Missing resettlement plan (Article 50)
    /// ຂາດແຜນຍົກຍ້າຍຈັດສັນ (ມາດຕາ 50)
    #[error(
        "Resettlement plan required for {affected_households} affected households (Article 50)\nຕ້ອງມີແຜນຍົກຍ້າຍຈັດສັນສຳລັບ {affected_households} ຄົວເຮືອນທີ່ໄດ້ຮັບຜົນກະທົບ (ມາດຕາ 50)"
    )]
    MissingResettlementPlan { affected_households: u32 },

    /// Invalid hydropower category (Article 46)
    /// ປະເພດໄຟຟ້ານໍ້າຕົກບໍ່ຖືກຕ້ອງ (ມາດຕາ 46)
    #[error(
        "Invalid hydropower category for {capacity_mw} MW: {reason} (Article 46)\nປະເພດໄຟຟ້ານໍ້າຕົກບໍ່ຖືກຕ້ອງສຳລັບ {capacity_mw} ເມກາວັດ: {reason} (ມາດຕາ 46)"
    )]
    InvalidHydropowerCategory { capacity_mw: f64, reason: String },

    // ========================================================================
    // Water Quality Errors (ຄວາມຜິດພາດຄຸນນະພາບນໍ້າ)
    // ========================================================================
    /// Drinking water quality violation (Article 55)
    /// ການລະເມີດຄຸນນະພາບນໍ້າດື່ມ (ມາດຕາ 55)
    #[error(
        "Drinking water quality violation: {parameter} at {actual} {unit} exceeds limit {limit} {unit} (Article 55)\nການລະເມີດຄຸນນະພາບນໍ້າດື່ມ: {parameter} ທີ່ {actual} {unit} ເກີນມາດຕະຖານ {limit} {unit} (ມາດຕາ 55)"
    )]
    DrinkingWaterViolation {
        parameter: String,
        actual: f64,
        limit: f64,
        unit: String,
    },

    /// Agricultural water quality violation (Article 56)
    /// ການລະເມີດຄຸນນະພາບນໍ້າກະສິກຳ (ມາດຕາ 56)
    #[error(
        "Agricultural water quality violation: {parameter} unsuitable for irrigation (Article 56)\nການລະເມີດຄຸນນະພາບນໍ້າກະສິກຳ: {parameter} ບໍ່ເໝາະສົມສຳລັບການຊົນລະປະທານ (ມາດຕາ 56)"
    )]
    AgriculturalWaterViolation { parameter: String },

    /// Industrial discharge violation (Article 57)
    /// ການລະເມີດການປ່ອຍນໍ້າເສຍອຸດສາຫະກຳ (ມາດຕາ 57)
    #[error(
        "Industrial discharge violation: {pollutant} at {actual} {unit} exceeds limit {limit} {unit} (Article 57)\nການລະເມີດການປ່ອຍນໍ້າເສຍອຸດສາຫະກຳ: {pollutant} ທີ່ {actual} {unit} ເກີນມາດຕະຖານ {limit} {unit} (ມາດຕາ 57)"
    )]
    IndustrialDischargeViolation {
        pollutant: String,
        actual: f64,
        limit: f64,
        unit: String,
    },

    /// Missing wastewater treatment (Article 58)
    /// ຂາດການບຳບັດນໍ້າເສຍ (ມາດຕາ 58)
    #[error(
        "Wastewater treatment required before discharge (Article 58)\nຕ້ອງບຳບັດນໍ້າເສຍກ່ອນປ່ອຍ (ມາດຕາ 58)"
    )]
    MissingWastewaterTreatment,

    // ========================================================================
    // Mekong River Commission Errors (ຄວາມຜິດພາດຄະນະກຳມາທິການແມ່ນໍ້າຂອງ)
    // ========================================================================
    /// Missing prior consultation (Article 60)
    /// ຂາດການປຶກສາຫາລືລ່ວງໜ້າ (ມາດຕາ 60)
    #[error(
        "Prior consultation with MRC required for {project_type} on Mekong mainstream (Article 60)\nຕ້ອງມີການປຶກສາຫາລືລ່ວງໜ້າກັບ MRC ສຳລັບ {project_type} ຢູ່ແມ່ນໍ້າຂອງສາຍຫຼັກ (ມາດຕາ 60)"
    )]
    MissingPriorConsultation { project_type: String },

    /// Missing notification to MRC (Article 61)
    /// ຂາດການແຈ້ງເຕືອນ MRC (ມາດຕາ 61)
    #[error(
        "Notification to MRC required for tributary project with transboundary impact (Article 61)\nຕ້ອງແຈ້ງເຕືອນ MRC ສຳລັບໂຄງການສາຂາທີ່ມີຜົນກະທົບຂ້າມຊາຍແດນ (ມາດຕາ 61)"
    )]
    MissingMRCNotification,

    /// Missing transboundary impact assessment (Article 62)
    /// ຂາດການປະເມີນຜົນກະທົບຂ້າມຊາຍແດນ (ມາດຕາ 62)
    #[error(
        "Transboundary impact assessment required for {project_name} (Article 62)\nຕ້ອງມີການປະເມີນຜົນກະທົບຂ້າມຊາຍແດນສຳລັບ {project_name} (ມາດຕາ 62)"
    )]
    MissingTransboundaryAssessment { project_name: String },

    /// Data sharing obligation violation (Article 63)
    /// ການລະເມີດຂໍ້ຜູກມັດການແບ່ງປັນຂໍ້ມູນ (ມາດຕາ 63)
    #[error(
        "Data sharing with MRC is required: {data_type} (Article 63)\nຕ້ອງແບ່ງປັນຂໍ້ມູນກັບ MRC: {data_type} (ມາດຕາ 63)"
    )]
    DataSharingViolation { data_type: String },

    // ========================================================================
    // Irrigation District Errors (ຄວາມຜິດພາດເຂດຊົນລະປະທານ)
    // ========================================================================
    /// Water User Association not registered (Article 70)
    /// ສະມາຄົມຜູ້ນຳໃຊ້ນໍ້າບໍ່ໄດ້ລົງທະບຽນ (ມາດຕາ 70)
    #[error(
        "Water User Association must be registered (Article 70)\nສະມາຄົມຜູ້ນຳໃຊ້ນໍ້າຕ້ອງລົງທະບຽນ (ມາດຕາ 70)"
    )]
    WUANotRegistered,

    /// Irrigation service fee not paid (Article 72)
    /// ຄ່າບໍລິການຊົນລະປະທານບໍ່ໄດ້ຈ່າຍ (ມາດຕາ 72)
    #[error(
        "Irrigation service fee of {amount} LAK is overdue by {days} days (Article 72)\nຄ່າບໍລິການຊົນລະປະທານ {amount} ກີບ ຄ້າງຊຳລະ {days} ມື້ (ມາດຕາ 72)"
    )]
    IrrigationFeeOverdue { amount: u64, days: u32 },

    /// Water delivery schedule violation (Article 73)
    /// ການລະເມີດຕາຕະລາງສົ່ງນໍ້າ (ມາດຕາ 73)
    #[error(
        "Water delivery schedule violation: {description} (Article 73)\nການລະເມີດຕາຕະລາງສົ່ງນໍ້າ: {description} (ມາດຕາ 73)"
    )]
    WaterDeliveryScheduleViolation { description: String },

    // ========================================================================
    // Groundwater Errors (ຄວາມຜິດພາດນໍ້າໃຕ້ດິນ)
    // ========================================================================
    /// Missing well drilling permit (Article 75)
    /// ຂາດໃບອະນຸຍາດຂຸດເຈາະບໍ່ (ມາດຕາ 75)
    #[error(
        "Well drilling permit required for depth > {threshold} meters (Article 75)\nຕ້ອງມີໃບອະນຸຍາດຂຸດເຈາະບໍ່ສຳລັບຄວາມເລິກ > {threshold} ແມັດ (ມາດຕາ 75)"
    )]
    MissingWellDrillingPermit { threshold: u32 },

    /// Groundwater extraction exceeds limit (Article 76)
    /// ການສູບນໍ້າໃຕ້ດິນເກີນຂີດຈຳກັດ (ມາດຕາ 76)
    #[error(
        "Groundwater extraction {actual} m3/day exceeds sustainable yield {limit} m3/day (Article 76)\nການສູບນໍ້າໃຕ້ດິນ {actual} ແມັດກ້ອນ/ມື້ ເກີນຂີດຈຳກັດແບບຍືນຍົງ {limit} ແມັດກ້ອນ/ມື້ (ມາດຕາ 76)"
    )]
    GroundwaterExtractionExceeds { actual: f64, limit: f64 },

    /// Aquifer protection zone violation (Article 77)
    /// ການລະເມີດເຂດປ້ອງກັນຊັ້ນນໍ້າໃຕ້ດິນ (ມາດຕາ 77)
    #[error(
        "Activity prohibited in aquifer protection zone: {activity} (Article 77)\nກິດຈະກຳຖືກຫ້າມໃນເຂດປ້ອງກັນຊັ້ນນໍ້າໃຕ້ດິນ: {activity} (ມາດຕາ 77)"
    )]
    AquiferProtectionZoneViolation { activity: String },

    /// Missing groundwater monitoring (Article 78)
    /// ຂາດການຕິດຕາມກວດການໍ້າໃຕ້ດິນ (ມາດຕາ 78)
    #[error(
        "Groundwater monitoring required: last report {days_since} days ago (Article 78)\nຕ້ອງມີການຕິດຕາມກວດການໍ້າໃຕ້ດິນ: ລາຍງານລ່າສຸດ {days_since} ມື້ກ່ອນ (ມາດຕາ 78)"
    )]
    MissingGroundwaterMonitoring { days_since: u32 },

    // ========================================================================
    // Pollution Prevention Errors (ຄວາມຜິດພາດການປ້ອງກັນມົນລະພິດ)
    // ========================================================================
    /// Polluter pays principle violation (Article 80)
    /// ການລະເມີດຫຼັກການຜູ້ກໍ່ມົນລະພິດຕ້ອງຮັບຜິດຊອບ (ມາດຕາ 80)
    #[error(
        "Polluter pays: {polluter} must pay {cost} LAK for remediation (Article 80)\nຜູ້ກໍ່ມົນລະພິດຕ້ອງຈ່າຍ: {polluter} ຕ້ອງຈ່າຍ {cost} ກີບ ສຳລັບການແກ້ໄຂ (ມາດຕາ 80)"
    )]
    PolluterPaysViolation { polluter: String, cost: u64 },

    /// Agricultural runoff violation (Article 82)
    /// ການລະເມີດນໍ້າໄຫຼລົ້ນຈາກກະສິກຳ (ມາດຕາ 82)
    #[error(
        "Agricultural runoff control required: {contaminant} detected (Article 82)\nຕ້ອງຄວບຄຸມນໍ້າໄຫຼລົ້ນຈາກກະສິກຳ: ກວດພົບ {contaminant} (ມາດຕາ 82)"
    )]
    AgriculturalRunoffViolation { contaminant: String },

    // ========================================================================
    // Water Allocation Errors (ຄວາມຜິດພາດການຈັດສັນນໍ້າ)
    // ========================================================================
    /// Water allocation priority violation (Article 40)
    /// ການລະເມີດລຳດັບບຸລິມະສິດການຈັດສັນນໍ້າ (ມາດຕາ 40)
    #[error(
        "Water allocation priority violated: {higher_priority} must be served before {lower_priority} (Article 40)\nລະເມີດລຳດັບບຸລິມະສິດການຈັດສັນນໍ້າ: ຕ້ອງຈັດໃຫ້ {higher_priority} ກ່ອນ {lower_priority} (ມາດຕາ 40)"
    )]
    AllocationPriorityViolation {
        higher_priority: String,
        lower_priority: String,
    },

    /// Seasonal allocation violation (Article 41)
    /// ການລະເມີດການຈັດສັນຕາມລະດູການ (ມາດຕາ 41)
    #[error(
        "Seasonal water allocation exceeded for {season}: {actual}% of quota used (Article 41)\nການຈັດສັນນໍ້າຕາມລະດູການເກີນສຳລັບ {season}: ໃຊ້ {actual}% ຂອງໂຄຕ້າ (ມາດຕາ 41)"
    )]
    SeasonalAllocationViolation { season: String, actual: f64 },

    /// Drought protocol violation (Article 42)
    /// ການລະເມີດລະບຽບການແຫ້ງແລ້ງ (ມາດຕາ 42)
    #[error(
        "Drought management protocol violated: {violation} (Article 42)\nລະເມີດລະບຽບການຄຸ້ມຄອງຄວາມແຫ້ງແລ້ງ: {violation} (ມາດຕາ 42)"
    )]
    DroughtProtocolViolation { violation: String },

    // ========================================================================
    // Water Source Classification Errors (ຄວາມຜິດພາດການຈັດປະເພດແຫຼ່ງນໍ້າ)
    // ========================================================================
    /// Invalid water source classification (Article 15)
    /// ການຈັດປະເພດແຫຼ່ງນໍ້າບໍ່ຖືກຕ້ອງ (ມາດຕາ 15)
    #[error(
        "Invalid water source classification: {reason} (Article 15)\nການຈັດປະເພດແຫຼ່ງນໍ້າບໍ່ຖືກຕ້ອງ: {reason} (ມາດຕາ 15)"
    )]
    InvalidWaterSourceClassification { reason: String },

    /// Wetland protection violation (Article 20)
    /// ການລະເມີດການປ້ອງກັນທີ່ດິນບຶງ (ມາດຕາ 20)
    #[error(
        "Wetland protection violated: {activity} prohibited in {wetland_name} (Article 20)\nລະເມີດການປ້ອງກັນທີ່ດິນບຶງ: {activity} ຖືກຫ້າມໃນ {wetland_name} (ມາດຕາ 20)"
    )]
    WetlandProtectionViolation {
        activity: String,
        wetland_name: String,
    },

    // ========================================================================
    // Permit Errors (ຄວາມຜິດພາດໃບອະນຸຍາດ)
    // ========================================================================
    /// Water permit expired (Article 35)
    /// ໃບອະນຸຍາດນໍ້າໝົດອາຍຸ (ມາດຕາ 35)
    #[error(
        "Water permit {permit_number} expired on {expiry_date} (Article 35)\nໃບອະນຸຍາດນໍ້າ {permit_number} ໝົດອາຍຸວັນທີ {expiry_date} (ມາດຕາ 35)"
    )]
    PermitExpired {
        permit_number: String,
        expiry_date: String,
    },

    /// Permit condition violation (Article 36)
    /// ການລະເມີດເງື່ອນໄຂໃບອະນຸຍາດ (ມາດຕາ 36)
    #[error(
        "Water permit condition violated: {condition} (Article 36)\nເງື່ອນໄຂໃບອະນຸຍາດນໍ້າຖືກລະເມີດ: {condition} (ມາດຕາ 36)"
    )]
    PermitConditionViolation { condition: String },

    /// Permit suspended or revoked (Article 37)
    /// ໃບອະນຸຍາດຖືກໂຈະ ຫຼື ຖືກຖອນ (ມາດຕາ 37)
    #[error(
        "Water permit {permit_number} has been {status} (Article 37)\nໃບອະນຸຍາດນໍ້າ {permit_number} ຖືກ{status} (ມາດຕາ 37)"
    )]
    PermitSuspendedOrRevoked {
        permit_number: String,
        status: String,
    },

    // ========================================================================
    // General Errors (ຄວາມຜິດພາດທົ່ວໄປ)
    // ========================================================================
    /// Validation error
    /// ຄວາມຜິດພາດການກວດສອບ
    #[error("Validation error: {message}\nຄວາມຜິດພາດການກວດສອບ: {message}")]
    ValidationError { message: String },

    /// Missing required field
    /// ຂາດຊ່ອງຂໍ້ມູນທີ່ຕ້ອງການ
    #[error("Missing required field: {field_name}\nຂາດຊ່ອງຂໍ້ມູນທີ່ຕ້ອງການ: {field_name}")]
    MissingRequiredField { field_name: String },

    /// Invalid date format
    /// ຮູບແບບວັນທີບໍ່ຖືກຕ້ອງ
    #[error("Invalid date format for {field}: {value}\nຮູບແບບວັນທີບໍ່ຖືກຕ້ອງສຳລັບ {field}: {value}")]
    InvalidDateFormat { field: String, value: String },

    /// Water emergency
    /// ເຫດສຸກເສີນທາງນໍ້າ
    #[error("Water emergency: {description}\nເຫດສຸກເສີນທາງນໍ້າ: {description}")]
    WaterEmergency { description: String },
}

impl WaterLawError {
    /// Get the error message in Lao language
    /// ຮັບຂໍ້ຄວາມຄວາມຜິດພາດເປັນພາສາລາວ
    pub fn lao_message(&self) -> String {
        let full_msg = format!("{}", self);
        // Extract the Lao part after the newline
        if let Some((_english, lao)) = full_msg.split_once('\n') {
            lao.to_string()
        } else {
            full_msg
        }
    }

    /// Get the error message in English language
    /// ຮັບຂໍ້ຄວາມຄວາມຜິດພາດເປັນພາສາອັງກິດ
    pub fn english_message(&self) -> String {
        let full_msg = format!("{}", self);
        // Extract the English part before the newline
        if let Some((english, _lao)) = full_msg.split_once('\n') {
            english.to_string()
        } else {
            full_msg
        }
    }

    /// Check if this is a critical violation requiring immediate action
    /// ກວດສອບວ່າເປັນການລະເມີດຮ້າຍແຮງທີ່ຕ້ອງແກ້ໄຂທັນທີ
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            WaterLawError::DrinkingWaterViolation { .. }
                | WaterLawError::WaterEmergency { .. }
                | WaterLawError::DomesticPriorityViolation { .. }
                | WaterLawError::MinimumFlowViolation { .. }
                | WaterLawError::AquiferProtectionZoneViolation { .. }
        )
    }

    /// Check if this is a permit-related error
    /// ກວດສອບວ່າເປັນຄວາມຜິດພາດກ່ຽວກັບໃບອະນຸຍາດ
    pub fn is_permit_related(&self) -> bool {
        matches!(
            self,
            WaterLawError::MissingWaterUsePermit { .. }
                | WaterLawError::MissingHydropowerConcession { .. }
                | WaterLawError::ConcessionExpired { .. }
                | WaterLawError::MissingWellDrillingPermit { .. }
                | WaterLawError::PermitExpired { .. }
                | WaterLawError::PermitConditionViolation { .. }
                | WaterLawError::PermitSuspendedOrRevoked { .. }
        )
    }

    /// Check if this is a MRC-related error
    /// ກວດສອບວ່າເປັນຄວາມຜິດພາດກ່ຽວກັບ MRC
    pub fn is_mrc_related(&self) -> bool {
        matches!(
            self,
            WaterLawError::MissingPriorConsultation { .. }
                | WaterLawError::MissingMRCNotification
                | WaterLawError::MissingTransboundaryAssessment { .. }
                | WaterLawError::DataSharingViolation { .. }
        )
    }

    /// Check if this is a water quality error
    /// ກວດສອບວ່າເປັນຄວາມຜິດພາດຄຸນນະພາບນໍ້າ
    pub fn is_water_quality_related(&self) -> bool {
        matches!(
            self,
            WaterLawError::DrinkingWaterViolation { .. }
                | WaterLawError::AgriculturalWaterViolation { .. }
                | WaterLawError::IndustrialDischargeViolation { .. }
                | WaterLawError::MissingWastewaterTreatment
        )
    }

    /// Get the article number referenced in this error, if any
    /// ຮັບເລກມາດຕາທີ່ອ້າງອິງໃນຄວາມຜິດພາດນີ້
    pub fn article_number(&self) -> Option<u32> {
        match self {
            WaterLawError::InvalidWaterSourceClassification { .. } => Some(15),
            WaterLawError::WetlandProtectionViolation { .. } => Some(20),
            WaterLawError::MissingWaterUsePermit { .. } => Some(35),
            WaterLawError::ExtractionExceedsLimit { .. } => Some(36),
            WaterLawError::UnauthorizedWaterUse { .. } => Some(37),
            WaterLawError::DomesticPriorityViolation { .. } => Some(38),
            WaterLawError::AllocationPriorityViolation { .. } => Some(40),
            WaterLawError::SeasonalAllocationViolation { .. } => Some(41),
            WaterLawError::DroughtProtocolViolation { .. } => Some(42),
            WaterLawError::MissingHydropowerConcession { .. } => Some(45),
            WaterLawError::InvalidHydropowerCategory { .. } => Some(46),
            WaterLawError::ConcessionExpired { .. } => Some(47),
            WaterLawError::MinimumFlowViolation { .. } => Some(48),
            WaterLawError::MissingResettlementPlan { .. } => Some(50),
            WaterLawError::DrinkingWaterViolation { .. } => Some(55),
            WaterLawError::AgriculturalWaterViolation { .. } => Some(56),
            WaterLawError::IndustrialDischargeViolation { .. } => Some(57),
            WaterLawError::MissingWastewaterTreatment => Some(58),
            WaterLawError::MissingPriorConsultation { .. } => Some(60),
            WaterLawError::MissingMRCNotification => Some(61),
            WaterLawError::MissingTransboundaryAssessment { .. } => Some(62),
            WaterLawError::DataSharingViolation { .. } => Some(63),
            WaterLawError::WUANotRegistered => Some(70),
            WaterLawError::IrrigationFeeOverdue { .. } => Some(72),
            WaterLawError::WaterDeliveryScheduleViolation { .. } => Some(73),
            WaterLawError::MissingWellDrillingPermit { .. } => Some(75),
            WaterLawError::GroundwaterExtractionExceeds { .. } => Some(76),
            WaterLawError::AquiferProtectionZoneViolation { .. } => Some(77),
            WaterLawError::MissingGroundwaterMonitoring { .. } => Some(78),
            WaterLawError::PolluterPaysViolation { .. } => Some(80),
            WaterLawError::AgriculturalRunoffViolation { .. } => Some(82),
            WaterLawError::PermitExpired { .. } => Some(35),
            WaterLawError::PermitConditionViolation { .. } => Some(36),
            WaterLawError::PermitSuspendedOrRevoked { .. } => Some(37),
            _ => None,
        }
    }

    /// Get penalty severity level (1-5, with 5 being most severe)
    /// ຮັບລະດັບຄວາມຮຸນແຮງຂອງໂທດ (1-5, 5 ຮຸນແຮງທີ່ສຸດ)
    pub fn penalty_severity(&self) -> u8 {
        match self {
            // Critical violations - highest penalty
            WaterLawError::DrinkingWaterViolation { .. }
            | WaterLawError::WaterEmergency { .. }
            | WaterLawError::AquiferProtectionZoneViolation { .. } => 5,

            // Serious violations
            WaterLawError::MinimumFlowViolation { .. }
            | WaterLawError::DomesticPriorityViolation { .. }
            | WaterLawError::MissingPriorConsultation { .. }
            | WaterLawError::IndustrialDischargeViolation { .. } => 4,

            // Significant violations
            WaterLawError::MissingHydropowerConcession { .. }
            | WaterLawError::ExtractionExceedsLimit { .. }
            | WaterLawError::GroundwaterExtractionExceeds { .. }
            | WaterLawError::PolluterPaysViolation { .. } => 3,

            // Moderate violations
            WaterLawError::PermitExpired { .. }
            | WaterLawError::ConcessionExpired { .. }
            | WaterLawError::MissingWaterUsePermit { .. }
            | WaterLawError::WUANotRegistered => 2,

            // Minor violations or administrative issues
            _ => 1,
        }
    }

    /// Check if this error can be corrected/remedied
    /// ກວດສອບວ່າຄວາມຜິດພາດນີ້ສາມາດແກ້ໄຂໄດ້
    pub fn is_correctable(&self) -> bool {
        !matches!(
            self,
            WaterLawError::WaterEmergency { .. }
                | WaterLawError::AquiferProtectionZoneViolation { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilingual_error_messages() {
        let error = WaterLawError::DrinkingWaterViolation {
            parameter: "E. coli".to_string(),
            actual: 100.0,
            limit: 0.0,
            unit: "CFU/100mL".to_string(),
        };

        let english = error.english_message();
        let lao = error.lao_message();

        assert!(english.contains("Drinking water quality violation"));
        assert!(lao.contains("ການລະເມີດຄຸນນະພາບນໍ້າດື່ມ"));
    }

    #[test]
    fn test_critical_violations() {
        let drinking = WaterLawError::DrinkingWaterViolation {
            parameter: "Arsenic".to_string(),
            actual: 0.05,
            limit: 0.01,
            unit: "mg/L".to_string(),
        };
        assert!(drinking.is_critical());

        let permit = WaterLawError::PermitExpired {
            permit_number: "WP-001".to_string(),
            expiry_date: "2025-01-01".to_string(),
        };
        assert!(!permit.is_critical());
    }

    #[test]
    fn test_permit_related() {
        let missing_permit = WaterLawError::MissingWaterUsePermit {
            use_type: "industrial".to_string(),
        };
        assert!(missing_permit.is_permit_related());

        let water_quality = WaterLawError::DrinkingWaterViolation {
            parameter: "pH".to_string(),
            actual: 4.0,
            limit: 6.5,
            unit: "".to_string(),
        };
        assert!(!water_quality.is_permit_related());
    }

    #[test]
    fn test_mrc_related() {
        let prior_consultation = WaterLawError::MissingPriorConsultation {
            project_type: "hydropower dam".to_string(),
        };
        assert!(prior_consultation.is_mrc_related());

        let irrigation = WaterLawError::IrrigationFeeOverdue {
            amount: 500_000,
            days: 30,
        };
        assert!(!irrigation.is_mrc_related());
    }

    #[test]
    fn test_article_numbers() {
        let mrc = WaterLawError::MissingPriorConsultation {
            project_type: "dam".to_string(),
        };
        assert_eq!(mrc.article_number(), Some(60));

        let groundwater = WaterLawError::GroundwaterExtractionExceeds {
            actual: 1000.0,
            limit: 500.0,
        };
        assert_eq!(groundwater.article_number(), Some(76));
    }

    #[test]
    fn test_penalty_severity() {
        let emergency = WaterLawError::WaterEmergency {
            description: "Contamination event".to_string(),
        };
        assert_eq!(emergency.penalty_severity(), 5);

        let validation = WaterLawError::ValidationError {
            message: "test".to_string(),
        };
        assert_eq!(validation.penalty_severity(), 1);
    }

    #[test]
    fn test_correctable() {
        let permit = WaterLawError::PermitExpired {
            permit_number: "WP-001".to_string(),
            expiry_date: "2025-01-01".to_string(),
        };
        assert!(permit.is_correctable());

        let emergency = WaterLawError::WaterEmergency {
            description: "Major spill".to_string(),
        };
        assert!(!emergency.is_correctable());
    }
}
