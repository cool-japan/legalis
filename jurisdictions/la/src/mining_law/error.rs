//! Mining Law Error Types (ປະເພດຄວາມຜິດພາດກົດໝາຍບໍ່ແຮ່)
//!
//! Comprehensive error types for Lao mining law validation and compliance.
//! All errors include bilingual messages (Lao/English) where applicable.
//!
//! # Legal Reference
//! - Mining Law 2017 (Law No. 31/NA) - ກົດໝາຍວ່າດ້ວຍບໍ່ແຮ່ ປີ 2017

use thiserror::Error;

/// Result type for mining law operations
pub type Result<T> = std::result::Result<T, MiningLawError>;

/// Mining law errors (ຄວາມຜິດພາດກົດໝາຍບໍ່ແຮ່)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum MiningLawError {
    // ========================================================================
    // License Errors (ຄວາມຜິດພາດໃບອະນຸຍາດ)
    // ========================================================================
    /// Missing mining license (Article 24)
    /// ຂາດໃບອະນຸຍາດບໍ່ແຮ່ (ມາດຕາ 24)
    #[error(
        "Mining license is required for this activity (Article 24)\nຕ້ອງມີໃບອະນຸຍາດບໍ່ແຮ່ສຳລັບກິດຈະກຳນີ້ (ມາດຕາ 24)"
    )]
    MissingLicense,

    /// Invalid license type for activity (Article 25)
    /// ປະເພດໃບອະນຸຍາດບໍ່ຖືກຕ້ອງສຳລັບກິດຈະກຳ (ມາດຕາ 25)
    #[error(
        "License type '{license_type}' is not valid for activity '{activity}' (Article 25)\nປະເພດໃບອະນຸຍາດ '{license_type}' ບໍ່ຖືກຕ້ອງສຳລັບກິດຈະກຳ '{activity}' (ມາດຕາ 25)"
    )]
    InvalidLicenseType {
        license_type: String,
        activity: String,
    },

    /// License expired (Article 28)
    /// ໃບອະນຸຍາດໝົດອາຍຸແລ້ວ (ມາດຕາ 28)
    #[error(
        "Mining license {license_number} expired on {expiry_date} (Article 28)\nໃບອະນຸຍາດບໍ່ແຮ່ {license_number} ໝົດອາຍຸວັນທີ {expiry_date} (ມາດຕາ 28)"
    )]
    LicenseExpired {
        license_number: String,
        expiry_date: String,
    },

    /// License suspended or revoked (Article 56)
    /// ໃບອະນຸຍາດຖືກໂຈະ ຫຼື ຖືກຖອນ (ມາດຕາ 56)
    #[error(
        "Mining license {license_number} has been {status} (Article 56)\nໃບອະນຸຍາດບໍ່ແຮ່ {license_number} ຖືກ{status} (ມາດຕາ 56)"
    )]
    LicenseSuspendedOrRevoked {
        license_number: String,
        status: String,
    },

    /// License renewal limit exceeded (Article 29)
    /// ການຕໍ່ອາຍຸໃບອະນຸຍາດເກີນກຳນົດ (ມາດຕາ 29)
    #[error(
        "Exploration license renewal limit exceeded: {current_renewals} renewals (max: {max_renewals}) (Article 29)\nການຕໍ່ອາຍຸໃບອະນຸຍາດສຳຫຼວດເກີນກຳນົດ: {current_renewals} ຄັ້ງ (ສູງສຸດ: {max_renewals}) (ມາດຕາ 29)"
    )]
    LicenseRenewalLimitExceeded {
        current_renewals: u32,
        max_renewals: u32,
    },

    // ========================================================================
    // Concession Errors (ຄວາມຜິດພາດສຳປະທານ)
    // ========================================================================
    /// Concession area exceeds maximum (Article 30)
    /// ພື້ນທີ່ສຳປະທານເກີນກຳນົດສູງສຸດ (ມາດຕາ 30)
    #[error(
        "Concession area {actual_hectares} hectares exceeds maximum of {max_hectares} hectares for {mineral_type} (Article 30)\nພື້ນທີ່ສຳປະທານ {actual_hectares} ເຮັກຕາເກີນກຳນົດສູງສຸດ {max_hectares} ເຮັກຕາສຳລັບ {mineral_type} (ມາດຕາ 30)"
    )]
    ConcessionAreaExceedsLimit {
        actual_hectares: f64,
        max_hectares: f64,
        mineral_type: String,
    },

    /// Concession duration exceeds maximum (Article 31)
    /// ໄລຍະເວລາສຳປະທານເກີນກຳນົດສູງສຸດ (ມາດຕາ 31)
    #[error(
        "Concession duration {actual_years} years exceeds maximum of {max_years} years (Article 31)\nໄລຍະເວລາສຳປະທານ {actual_years} ປີເກີນກຳນົດສູງສຸດ {max_years} ປີ (ມາດຕາ 31)"
    )]
    ConcessionDurationExceedsLimit { actual_years: u32, max_years: u32 },

    /// Overlapping concession (Article 33)
    /// ສຳປະທານຊ້ອນກັນ (ມາດຕາ 33)
    #[error(
        "Concession area overlaps with existing concession {existing_id} (Article 33)\nພື້ນທີ່ສຳປະທານຊ້ອນກັບສຳປະທານທີ່ມີຢູ່ {existing_id} (ມາດຕາ 33)"
    )]
    OverlappingConcession { existing_id: String },

    /// Invalid concession status (Article 35)
    /// ສະຖານະສຳປະທານບໍ່ຖືກຕ້ອງ (ມາດຕາ 35)
    #[error(
        "Concession status '{status}' does not allow this operation (Article 35)\nສະຖານະສຳປະທານ '{status}' ບໍ່ອະນຸຍາດໃຫ້ດຳເນີນການນີ້ (ມາດຕາ 35)"
    )]
    InvalidConcessionStatus { status: String },

    // ========================================================================
    // Mineral Classification Errors (ຄວາມຜິດພາດການຈັດປະເພດແຮ່)
    // ========================================================================
    /// Strategic mineral requires special approval (Article 12)
    /// ແຮ່ຍຸດທະສາດຕ້ອງໄດ້ຮັບການອະນຸມັດພິເສດ (ມາດຕາ 12)
    #[error(
        "Strategic mineral '{mineral}' requires Government approval (Article 12)\nແຮ່ຍຸດທະສາດ '{mineral}' ຕ້ອງໄດ້ຮັບການອະນຸມັດຈາກລັດຖະບານ (ມາດຕາ 12)"
    )]
    StrategicMineralRequiresApproval { mineral: String },

    /// Invalid mineral classification (Article 11)
    /// ການຈັດປະເພດແຮ່ບໍ່ຖືກຕ້ອງ (ມາດຕາ 11)
    #[error(
        "Invalid mineral classification for '{mineral}' (Article 11)\nການຈັດປະເພດແຮ່ບໍ່ຖືກຕ້ອງສຳລັບ '{mineral}' (ມາດຕາ 11)"
    )]
    InvalidMineralClassification { mineral: String },

    /// Rare earth extraction restriction (Article 13)
    /// ຂໍ້ຈຳກັດການສະກັດທາດຫາຍາກ (ມາດຕາ 13)
    #[error(
        "Rare earth extraction requires special environmental assessment (Article 13)\nການສະກັດທາດຫາຍາກຕ້ອງມີການປະເມີນສິ່ງແວດລ້ອມພິເສດ (ມາດຕາ 13)"
    )]
    RareEarthRestriction,

    // ========================================================================
    // Royalty Errors (ຄວາມຜິດພາດຄ່າພາກຫຼວງ)
    // ========================================================================
    /// Royalty rate mismatch (Article 45)
    /// ອັດຕາຄ່າພາກຫຼວງບໍ່ກົງກັນ (ມາດຕາ 45)
    #[error(
        "Royalty rate {actual_rate}% does not match required rate {required_rate}% for {mineral} (Article 45)\nອັດຕາຄ່າພາກຫຼວງ {actual_rate}% ບໍ່ກົງກັບອັດຕາທີ່ກຳນົດ {required_rate}% ສຳລັບ {mineral} (ມາດຕາ 45)"
    )]
    RoyaltyRateMismatch {
        mineral: String,
        actual_rate: f64,
        required_rate: f64,
    },

    /// Royalty payment overdue (Article 47)
    /// ຄ່າພາກຫຼວງຄ້າງຊຳລະ (ມາດຕາ 47)
    #[error(
        "Royalty payment overdue by {days_overdue} days for {amount_lak} LAK (Article 47)\nຄ່າພາກຫຼວງຄ້າງຊຳລະ {days_overdue} ມື້ ຈຳນວນ {amount_lak} ກີບ (ມາດຕາ 47)"
    )]
    RoyaltyPaymentOverdue { days_overdue: u32, amount_lak: u64 },

    /// Invalid production volume for royalty (Article 46)
    /// ປະລິມານການຜະລິດບໍ່ຖືກຕ້ອງສຳລັບຄ່າພາກຫຼວງ (ມາດຕາ 46)
    #[error(
        "Production volume {volume} is invalid for royalty calculation (Article 46)\nປະລິມານການຜະລິດ {volume} ບໍ່ຖືກຕ້ອງສຳລັບການຄິດໄລ່ຄ່າພາກຫຼວງ (ມາດຕາ 46)"
    )]
    InvalidProductionVolume { volume: String },

    // ========================================================================
    // Environmental Errors (ຄວາມຜິດພາດສິ່ງແວດລ້ອມ)
    // ========================================================================
    /// Missing EIA for mining (Article 50)
    /// ຂາດການປະເມີນຜົນກະທົບສິ່ງແວດລ້ອມສຳລັບການຂຸດຄົ້ນ (ມາດຕາ 50)
    #[error(
        "Environmental Impact Assessment required for mining operations (Article 50)\nຕ້ອງມີການປະເມີນຜົນກະທົບສິ່ງແວດລ້ອມສຳລັບການຂຸດຄົ້ນບໍ່ແຮ່ (ມາດຕາ 50)"
    )]
    MissingMiningEIA,

    /// Rehabilitation bond insufficient (Article 52)
    /// ເງິນຄ້ຳປະກັນການຟື້ນຟູບໍ່ພຽງພໍ (ມາດຕາ 52)
    #[error(
        "Rehabilitation bond {actual_lak} LAK is less than required {required_lak} LAK (Article 52)\nເງິນຄ້ຳປະກັນການຟື້ນຟູ {actual_lak} ກີບ ນ້ອຍກວ່າທີ່ກຳນົດ {required_lak} ກີບ (ມາດຕາ 52)"
    )]
    InsufficientRehabilitationBond { actual_lak: u64, required_lak: u64 },

    /// Missing closure plan (Article 53)
    /// ຂາດແຜນປິດບໍ່ແຮ່ (ມາດຕາ 53)
    #[error("Mine closure plan is required (Article 53)\nຕ້ອງມີແຜນປິດບໍ່ແຮ່ (ມາດຕາ 53)")]
    MissingClosurePlan,

    /// Too close to protected area (Article 51)
    /// ໃກ້ເຂດປ່າປ້ອງກັນເກີນໄປ (ມາດຕາ 51)
    #[error(
        "Mining site {distance_meters}m from protected area violates minimum distance of {required_meters}m (Article 51)\nພື້ນທີ່ຂຸດຄົ້ນ {distance_meters} ແມັດຈາກເຂດປ່າປ້ອງກັນລະເມີດໄລຍະຫ່າງຂັ້ນຕ່ຳ {required_meters} ແມັດ (ມາດຕາ 51)"
    )]
    TooCloseToProtectedArea {
        distance_meters: u32,
        required_meters: u32,
    },

    /// Environmental violation during mining (Article 54)
    /// ການລະເມີດສິ່ງແວດລ້ອມໃນການຂຸດຄົ້ນ (ມາດຕາ 54)
    #[error(
        "Environmental violation: {description} (Article 54)\nການລະເມີດສິ່ງແວດລ້ອມ: {description} (ມາດຕາ 54)"
    )]
    EnvironmentalViolation { description: String },

    // ========================================================================
    // Foreign Investment Errors (ຄວາມຜິດພາດການລົງທຶນຕ່າງປະເທດ)
    // ========================================================================
    /// Joint venture required for strategic minerals (Article 18)
    /// ຕ້ອງມີການຮ່ວມທຶນສຳລັບແຮ່ຍຸດທະສາດ (ມາດຕາ 18)
    #[error(
        "Joint venture with Lao entity required for strategic mineral '{mineral}' (Article 18)\nຕ້ອງມີການຮ່ວມທຶນກັບນິຕິບຸກຄົນລາວສຳລັບແຮ່ຍຸດທະສາດ '{mineral}' (ມາດຕາ 18)"
    )]
    JointVentureRequired { mineral: String },

    /// Foreign ownership exceeds limit (Article 19)
    /// ການຖືຫຸ້ນຂອງຕ່າງປະເທດເກີນກຳນົດ (ມາດຕາ 19)
    #[error(
        "Foreign ownership {actual_percent}% exceeds limit of {max_percent}% for {mineral_type} (Article 19)\nການຖືຫຸ້ນຂອງຕ່າງປະເທດ {actual_percent}% ເກີນກຳນົດ {max_percent}% ສຳລັບ {mineral_type} (ມາດຕາ 19)"
    )]
    ForeignOwnershipExceedsLimit {
        actual_percent: f64,
        max_percent: f64,
        mineral_type: String,
    },

    /// Local content requirement not met (Article 20)
    /// ບໍ່ປະຕິບັດຕາມຂໍ້ກຳນົດເນື້ອໃນທ້ອງຖິ່ນ (ມາດຕາ 20)
    #[error(
        "Local content requirement {actual_percent}% below minimum {required_percent}% (Article 20)\nເນື້ອໃນທ້ອງຖິ່ນ {actual_percent}% ຕ່ຳກວ່າຂັ້ນຕ່ຳ {required_percent}% (ມາດຕາ 20)"
    )]
    LocalContentRequirementNotMet {
        actual_percent: f64,
        required_percent: f64,
    },

    /// Technology transfer obligation not met (Article 21)
    /// ບໍ່ປະຕິບັດພັນທະການຖ່າຍທອດເຕັກໂນໂລຊີ (ມາດຕາ 21)
    #[error(
        "Technology transfer obligation not met: {missing_item} (Article 21)\nບໍ່ປະຕິບັດພັນທະການຖ່າຍທອດເຕັກໂນໂລຊີ: {missing_item} (ມາດຕາ 21)"
    )]
    TechnologyTransferNotMet { missing_item: String },

    // ========================================================================
    // Community Rights Errors (ຄວາມຜິດພາດສິດຂອງຊຸມຊົນ)
    // ========================================================================
    /// Missing prior consultation (Article 37)
    /// ຂາດການປຶກສາຫາລືລ່ວງໜ້າ (ມາດຕາ 37)
    #[error(
        "Prior consultation with affected communities required (Article 37)\nຕ້ອງປຶກສາຫາລືກັບຊຸມຊົນທີ່ໄດ້ຮັບຜົນກະທົບລ່ວງໜ້າ (ມາດຕາ 37)"
    )]
    MissingPriorConsultation,

    /// Inadequate community compensation (Article 38)
    /// ການຊົດເຊີຍຊຸມຊົນບໍ່ພຽງພໍ (ມາດຕາ 38)
    #[error(
        "Community compensation {actual_lak} LAK is less than required {required_lak} LAK (Article 38)\nການຊົດເຊີຍຊຸມຊົນ {actual_lak} ກີບ ນ້ອຍກວ່າທີ່ກຳນົດ {required_lak} ກີບ (ມາດຕາ 38)"
    )]
    InadequateCommunityCompensation { actual_lak: u64, required_lak: u64 },

    /// Local employment quota not met (Article 39)
    /// ໂຄຕ້າການຈ້າງງານທ້ອງຖິ່ນບໍ່ບັນລຸ (ມາດຕາ 39)
    #[error(
        "Local employment {actual_percent}% below required quota {required_percent}% (Article 39)\nການຈ້າງງານທ້ອງຖິ່ນ {actual_percent}% ຕ່ຳກວ່າໂຄຕ້າ {required_percent}% (ມາດຕາ 39)"
    )]
    LocalEmploymentQuotaNotMet {
        actual_percent: f64,
        required_percent: f64,
    },

    /// Revenue sharing not compliant (Article 40)
    /// ການແບ່ງລາຍຮັບບໍ່ຖືກຕ້ອງ (ມາດຕາ 40)
    #[error(
        "Revenue sharing with local community {actual_percent}% below requirement {required_percent}% (Article 40)\nການແບ່ງລາຍຮັບກັບຊຸມຊົນທ້ອງຖິ່ນ {actual_percent}% ຕ່ຳກວ່າກຳນົດ {required_percent}% (ມາດຕາ 40)"
    )]
    RevenueSharingNotCompliant {
        actual_percent: f64,
        required_percent: f64,
    },

    // ========================================================================
    // Small-Scale Mining Errors (ຄວາມຜິດພາດບໍ່ແຮ່ຂະໜາດນ້ອຍ)
    // ========================================================================
    /// Small-scale mining exceeds limits (Article 42)
    /// ບໍ່ແຮ່ຂະໜາດນ້ອຍເກີນຂອບເຂດ (ມາດຕາ 42)
    #[error(
        "Small-scale mining operation exceeds permitted limits: {violation} (Article 42)\nການຂຸດຄົ້ນບໍ່ແຮ່ຂະໜາດນ້ອຍເກີນຂອບເຂດທີ່ອະນຸຍາດ: {violation} (ມາດຕາ 42)"
    )]
    SmallScaleMiningExceedsLimits { violation: String },

    /// Artisanal mining not registered (Article 43)
    /// ບໍ່ແຮ່ຫັດຖະກຳບໍ່ໄດ້ຈົດທະບຽນ (ມາດຕາ 43)
    #[error(
        "Artisanal mining operation not registered (Article 43)\nການຂຸດຄົ້ນບໍ່ແຮ່ຫັດຖະກຳບໍ່ໄດ້ຈົດທະບຽນ (ມາດຕາ 43)"
    )]
    ArtisanalMiningNotRegistered,

    // ========================================================================
    // Processing Errors (ຄວາມຜິດພາດການປຸງແຕ່ງ)
    // ========================================================================
    /// Missing processing license (Article 34)
    /// ຂາດໃບອະນຸຍາດປຸງແຕ່ງ (ມາດຕາ 34)
    #[error(
        "Processing license required for mineral processing operations (Article 34)\nຕ້ອງມີໃບອະນຸຍາດປຸງແຕ່ງສຳລັບການປຸງແຕ່ງແຮ່ (ມາດຕາ 34)"
    )]
    MissingProcessingLicense,

    /// Export of raw ore restricted (Article 36)
    /// ການສົ່ງອອກແຮ່ດິບຖືກຈຳກັດ (ມາດຕາ 36)
    #[error(
        "Export of raw {mineral} ore is restricted without processing (Article 36)\nການສົ່ງອອກແຮ່ {mineral} ດິບຖືກຈຳກັດໂດຍບໍ່ມີການປຸງແຕ່ງ (ມາດຕາ 36)"
    )]
    RawOreExportRestricted { mineral: String },

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

    /// General non-compliance (Article 55)
    /// ການບໍ່ປະຕິບັດຕາມທົ່ວໄປ (ມາດຕາ 55)
    #[error(
        "Mining law non-compliance: {violation} (Article {article})\nການບໍ່ປະຕິບັດຕາມກົດໝາຍບໍ່ແຮ່: {violation} (ມາດຕາ {article})"
    )]
    NonCompliance { violation: String, article: u32 },

    /// Illegal mining activity (Article 57)
    /// ກິດຈະກຳຂຸດຄົ້ນບໍ່ແຮ່ຜິດກົດໝາຍ (ມາດຕາ 57)
    #[error(
        "Illegal mining activity detected: {description} (Article 57)\nກວດພົບກິດຈະກຳຂຸດຄົ້ນບໍ່ແຮ່ຜິດກົດໝາຍ: {description} (ມາດຕາ 57)"
    )]
    IllegalMiningActivity { description: String },
}

impl MiningLawError {
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
            MiningLawError::IllegalMiningActivity { .. }
                | MiningLawError::EnvironmentalViolation { .. }
                | MiningLawError::TooCloseToProtectedArea { .. }
                | MiningLawError::MissingMiningEIA
                | MiningLawError::RareEarthRestriction
        )
    }

    /// Check if this is an environmental-related error
    /// ກວດສອບວ່າເປັນຄວາມຜິດພາດກ່ຽວກັບສິ່ງແວດລ້ອມ
    pub fn is_environmental(&self) -> bool {
        matches!(
            self,
            MiningLawError::MissingMiningEIA
                | MiningLawError::InsufficientRehabilitationBond { .. }
                | MiningLawError::MissingClosurePlan
                | MiningLawError::TooCloseToProtectedArea { .. }
                | MiningLawError::EnvironmentalViolation { .. }
                | MiningLawError::RareEarthRestriction
        )
    }

    /// Check if this is a license-related error
    /// ກວດສອບວ່າເປັນຄວາມຜິດພາດກ່ຽວກັບໃບອະນຸຍາດ
    pub fn is_license_related(&self) -> bool {
        matches!(
            self,
            MiningLawError::MissingLicense
                | MiningLawError::InvalidLicenseType { .. }
                | MiningLawError::LicenseExpired { .. }
                | MiningLawError::LicenseSuspendedOrRevoked { .. }
                | MiningLawError::LicenseRenewalLimitExceeded { .. }
                | MiningLawError::MissingProcessingLicense
        )
    }

    /// Check if this is a community rights error
    /// ກວດສອບວ່າເປັນຄວາມຜິດພາດກ່ຽວກັບສິດຊຸມຊົນ
    pub fn is_community_rights_related(&self) -> bool {
        matches!(
            self,
            MiningLawError::MissingPriorConsultation
                | MiningLawError::InadequateCommunityCompensation { .. }
                | MiningLawError::LocalEmploymentQuotaNotMet { .. }
                | MiningLawError::RevenueSharingNotCompliant { .. }
        )
    }

    /// Get the article number referenced in this error, if any
    /// ຮັບເລກມາດຕາທີ່ອ້າງອິງໃນຄວາມຜິດພາດນີ້
    pub fn article_number(&self) -> Option<u32> {
        match self {
            MiningLawError::InvalidMineralClassification { .. } => Some(11),
            MiningLawError::StrategicMineralRequiresApproval { .. } => Some(12),
            MiningLawError::RareEarthRestriction => Some(13),
            MiningLawError::JointVentureRequired { .. } => Some(18),
            MiningLawError::ForeignOwnershipExceedsLimit { .. } => Some(19),
            MiningLawError::LocalContentRequirementNotMet { .. } => Some(20),
            MiningLawError::TechnologyTransferNotMet { .. } => Some(21),
            MiningLawError::MissingLicense => Some(24),
            MiningLawError::InvalidLicenseType { .. } => Some(25),
            MiningLawError::LicenseExpired { .. } => Some(28),
            MiningLawError::LicenseRenewalLimitExceeded { .. } => Some(29),
            MiningLawError::ConcessionAreaExceedsLimit { .. } => Some(30),
            MiningLawError::ConcessionDurationExceedsLimit { .. } => Some(31),
            MiningLawError::OverlappingConcession { .. } => Some(33),
            MiningLawError::MissingProcessingLicense => Some(34),
            MiningLawError::InvalidConcessionStatus { .. } => Some(35),
            MiningLawError::RawOreExportRestricted { .. } => Some(36),
            MiningLawError::MissingPriorConsultation => Some(37),
            MiningLawError::InadequateCommunityCompensation { .. } => Some(38),
            MiningLawError::LocalEmploymentQuotaNotMet { .. } => Some(39),
            MiningLawError::RevenueSharingNotCompliant { .. } => Some(40),
            MiningLawError::SmallScaleMiningExceedsLimits { .. } => Some(42),
            MiningLawError::ArtisanalMiningNotRegistered => Some(43),
            MiningLawError::RoyaltyRateMismatch { .. } => Some(45),
            MiningLawError::InvalidProductionVolume { .. } => Some(46),
            MiningLawError::RoyaltyPaymentOverdue { .. } => Some(47),
            MiningLawError::MissingMiningEIA => Some(50),
            MiningLawError::TooCloseToProtectedArea { .. } => Some(51),
            MiningLawError::InsufficientRehabilitationBond { .. } => Some(52),
            MiningLawError::MissingClosurePlan => Some(53),
            MiningLawError::EnvironmentalViolation { .. } => Some(54),
            MiningLawError::NonCompliance { article, .. } => Some(*article),
            MiningLawError::LicenseSuspendedOrRevoked { .. } => Some(56),
            MiningLawError::IllegalMiningActivity { .. } => Some(57),
            _ => None,
        }
    }

    /// Get penalty severity level (1-5, with 5 being most severe)
    /// ຮັບລະດັບຄວາມຮຸນແຮງຂອງໂທດ (1-5, 5 ຮຸນແຮງທີ່ສຸດ)
    pub fn penalty_severity(&self) -> u8 {
        match self {
            // Critical violations - highest penalty
            MiningLawError::IllegalMiningActivity { .. }
            | MiningLawError::EnvironmentalViolation { .. }
            | MiningLawError::RareEarthRestriction => 5,

            // Serious violations
            MiningLawError::TooCloseToProtectedArea { .. }
            | MiningLawError::MissingMiningEIA
            | MiningLawError::MissingClosurePlan
            | MiningLawError::StrategicMineralRequiresApproval { .. } => 4,

            // Significant violations
            MiningLawError::MissingLicense
            | MiningLawError::LicenseSuspendedOrRevoked { .. }
            | MiningLawError::JointVentureRequired { .. }
            | MiningLawError::ForeignOwnershipExceedsLimit { .. }
            | MiningLawError::InsufficientRehabilitationBond { .. } => 3,

            // Moderate violations
            MiningLawError::LicenseExpired { .. }
            | MiningLawError::ConcessionAreaExceedsLimit { .. }
            | MiningLawError::ConcessionDurationExceedsLimit { .. }
            | MiningLawError::RoyaltyPaymentOverdue { .. }
            | MiningLawError::MissingPriorConsultation
            | MiningLawError::LocalEmploymentQuotaNotMet { .. } => 2,

            // Minor violations or administrative issues
            _ => 1,
        }
    }

    /// Check if this error can be corrected/remedied
    /// ກວດສອບວ່າຄວາມຜິດພາດນີ້ສາມາດແກ້ໄຂໄດ້
    pub fn is_correctable(&self) -> bool {
        !matches!(
            self,
            MiningLawError::IllegalMiningActivity { .. }
                | MiningLawError::EnvironmentalViolation { description: _ }
                | MiningLawError::RareEarthRestriction
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilingual_error_messages() {
        let error = MiningLawError::RoyaltyRateMismatch {
            mineral: "Gold".to_string(),
            actual_rate: 3.0,
            required_rate: 5.0,
        };

        let english = error.english_message();
        let lao = error.lao_message();

        assert!(english.contains("Royalty rate 3%"));
        assert!(lao.contains("ອັດຕາຄ່າພາກຫຼວງ 3%"));
    }

    #[test]
    fn test_critical_violations() {
        let illegal = MiningLawError::IllegalMiningActivity {
            description: "Unauthorized extraction".to_string(),
        };
        assert!(illegal.is_critical());

        let license = MiningLawError::LicenseExpired {
            license_number: "ML-001".to_string(),
            expiry_date: "2025-01-01".to_string(),
        };
        assert!(!license.is_critical());
    }

    #[test]
    fn test_environmental_errors() {
        let eia = MiningLawError::MissingMiningEIA;
        assert!(eia.is_environmental());

        let royalty = MiningLawError::RoyaltyPaymentOverdue {
            days_overdue: 30,
            amount_lak: 1_000_000,
        };
        assert!(!royalty.is_environmental());
    }

    #[test]
    fn test_article_numbers() {
        let eia = MiningLawError::MissingMiningEIA;
        assert_eq!(eia.article_number(), Some(50));

        let royalty = MiningLawError::RoyaltyRateMismatch {
            mineral: "Gold".to_string(),
            actual_rate: 3.0,
            required_rate: 5.0,
        };
        assert_eq!(royalty.article_number(), Some(45));
    }

    #[test]
    fn test_penalty_severity() {
        let illegal = MiningLawError::IllegalMiningActivity {
            description: "test".to_string(),
        };
        assert_eq!(illegal.penalty_severity(), 5);

        let validation = MiningLawError::ValidationError {
            message: "test".to_string(),
        };
        assert_eq!(validation.penalty_severity(), 1);
    }

    #[test]
    fn test_correctable() {
        let license = MiningLawError::LicenseExpired {
            license_number: "ML-001".to_string(),
            expiry_date: "2025-01-01".to_string(),
        };
        assert!(license.is_correctable());

        let illegal = MiningLawError::IllegalMiningActivity {
            description: "test".to_string(),
        };
        assert!(!illegal.is_correctable());
    }

    #[test]
    fn test_license_related_errors() {
        let missing = MiningLawError::MissingLicense;
        assert!(missing.is_license_related());

        let expired = MiningLawError::LicenseExpired {
            license_number: "ML-001".to_string(),
            expiry_date: "2025-01-01".to_string(),
        };
        assert!(expired.is_license_related());

        let env = MiningLawError::MissingMiningEIA;
        assert!(!env.is_license_related());
    }

    #[test]
    fn test_community_rights_errors() {
        let consultation = MiningLawError::MissingPriorConsultation;
        assert!(consultation.is_community_rights_related());

        let compensation = MiningLawError::InadequateCommunityCompensation {
            actual_lak: 1_000_000,
            required_lak: 5_000_000,
        };
        assert!(compensation.is_community_rights_related());

        let license = MiningLawError::MissingLicense;
        assert!(!license.is_community_rights_related());
    }
}
