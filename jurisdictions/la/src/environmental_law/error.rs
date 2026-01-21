//! Environmental Law Error Types (ປະເພດຄວາມຜິດພາດກົດໝາຍສິ່ງແວດລ້ອມ)
//!
//! Comprehensive error types for Lao environmental law validation and compliance.
//! All errors include bilingual messages (Lao/English) where applicable.
//!
//! # Legal Reference
//! - Environmental Protection Law 2012 (Law No. 29/NA) - ກົດໝາຍວ່າດ້ວຍການປົກປັກຮັກສາສິ່ງແວດລ້ອມ ປີ 2012

use thiserror::Error;

/// Result type for environmental law operations
pub type Result<T> = std::result::Result<T, EnvironmentalLawError>;

/// Environmental law errors (ຄວາມຜິດພາດກົດໝາຍສິ່ງແວດລ້ອມ)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum EnvironmentalLawError {
    // ========================================================================
    // EIA Errors (ຄວາມຜິດພາດ EIA)
    // ========================================================================
    /// Missing EIA assessment (Article 18)
    /// ຂາດການປະເມີນຜົນກະທົບສິ່ງແວດລ້ອມ (ມາດຕາ 18)
    #[error(
        "Environmental Impact Assessment is required for this project type (Article 18)\nຕ້ອງມີການປະເມີນຜົນກະທົບສິ່ງແວດລ້ອມ (EIA) ສຳລັບໂຄງການປະເພດນີ້ (ມາດຕາ 18)"
    )]
    MissingEIA,

    /// Invalid project type for EIA
    /// ປະເພດໂຄງການບໍ່ຖືກຕ້ອງສຳລັບ EIA
    #[error(
        "Invalid project type for EIA: {project_type}\nປະເພດໂຄງການບໍ່ຖືກຕ້ອງສຳລັບ EIA: {project_type}"
    )]
    InvalidProjectType { project_type: String },

    /// Incomplete EIA assessment (Article 20)
    /// ການປະເມີນ EIA ບໍ່ຄົບຖ້ວນ (ມາດຕາ 20)
    #[error(
        "EIA assessment is incomplete: missing {missing_component} (Article 20)\nການປະເມີນ EIA ບໍ່ຄົບຖ້ວນ: ຂາດ {missing_component} (ມາດຕາ 20)"
    )]
    IncompleteEIA { missing_component: String },

    /// Missing public consultation (Article 21)
    /// ຂາດການປຶກສາຫາລືກັບປະຊາຊົນ (ມາດຕາ 21)
    #[error(
        "Public consultation is required for Category A projects (Article 21)\nຕ້ອງມີການປຶກສາຫາລືກັບປະຊາຊົນສຳລັບໂຄງການປະເພດ ກ (ມາດຕາ 21)"
    )]
    MissingPublicConsultation,

    /// EIA expired (Article 22)
    /// EIA ໝົດອາຍຸແລ້ວ (ມາດຕາ 22)
    #[error(
        "EIA certificate has expired: valid until {expiry_date}, project start {project_date} (Article 22)\nໃບຢັ້ງຢືນ EIA ໝົດອາຍຸແລ້ວ: ໃຊ້ໄດ້ຮອດ {expiry_date}, ເລີ່ມໂຄງການ {project_date} (ມາດຕາ 22)"
    )]
    EIAExpired {
        expiry_date: String,
        project_date: String,
    },

    /// Missing mitigation measures for significant impact
    /// ຂາດມາດຕະການບັນເທົາສຳລັບຜົນກະທົບທີ່ສຳຄັນ
    #[error(
        "Mitigation measures required for {impact_type} with severity: {severity}\nຕ້ອງມີມາດຕະການບັນເທົາສຳລັບ {impact_type} ລະດັບ: {severity}"
    )]
    MissingMitigationMeasures {
        impact_type: String,
        severity: String,
    },

    /// Missing environmental management plan
    /// ຂາດແຜນຄຸ້ມຄອງສິ່ງແວດລ້ອມ
    #[error(
        "Environmental Management Plan (EMP) is required for Category A projects\nຕ້ອງມີແຜນຄຸ້ມຄອງສິ່ງແວດລ້ອມ (EMP) ສຳລັບໂຄງການປະເພດ ກ"
    )]
    MissingManagementPlan,

    // ========================================================================
    // Pollution Errors (ຄວາມຜິດພາດມົນລະພິດ)
    // ========================================================================
    /// Air quality exceeds limits (Article 30)
    /// ຄຸນນະພາບອາກາດເກີນມາດຕະຖານ (ມາດຕາ 30)
    #[error(
        "Air quality violation: {pollutant} level {actual} {unit} exceeds limit of {limit} {unit} (Article 30)\nການລະເມີດຄຸນນະພາບອາກາດ: {pollutant} ລະດັບ {actual} {unit} ເກີນມາດຕະຖານ {limit} {unit} (ມາດຕາ 30)"
    )]
    AirQualityExceedsLimit {
        pollutant: String,
        actual: f64,
        limit: f64,
        unit: String,
    },

    /// Water quality exceeds limits (Article 31)
    /// ຄຸນນະພາບນ້ຳເກີນມາດຕະຖານ (ມາດຕາ 31)
    #[error(
        "Water quality violation: {parameter} level {actual} {unit} exceeds limit of {limit} {unit} (Article 31)\nການລະເມີດຄຸນນະພາບນ້ຳ: {parameter} ລະດັບ {actual} {unit} ເກີນມາດຕະຖານ {limit} {unit} (ມາດຕາ 31)"
    )]
    WaterQualityExceedsLimit {
        parameter: String,
        actual: f64,
        limit: f64,
        unit: String,
    },

    /// Noise level exceeds limits (Article 32)
    /// ລະດັບສຽງເກີນມາດຕະຖານ (ມາດຕາ 32)
    #[error(
        "Noise violation in {zone_type} zone: {actual} dB exceeds {period} limit of {limit} dB (Article 32)\nການລະເມີດລະດັບສຽງໃນເຂດ {zone_type}: {actual} dB ເກີນມາດຕະຖານ{period} {limit} dB (ມາດຕາ 32)"
    )]
    NoiseLevelExceedsLimit {
        zone_type: String,
        actual: u8,
        limit: u8,
        period: String,
    },

    /// pH out of range (Article 31)
    /// pH ຢູ່ນອກຂອບເຂດ (ມາດຕາ 31)
    #[error(
        "pH level {actual} is outside acceptable range ({min} - {max}) (Article 31)\nລະດັບ pH {actual} ຢູ່ນອກຂອບເຂດທີ່ຍອມຮັບໄດ້ ({min} - {max}) (ມາດຕາ 31)"
    )]
    PHOutOfRange { actual: f64, min: f64, max: f64 },

    /// Temperature exceeds limit (Article 31)
    /// ອຸນຫະພູມເກີນມາດຕະຖານ (ມາດຕາ 31)
    #[error(
        "Water discharge temperature {actual}°C exceeds limit of {limit}°C (Article 31)\nອຸນຫະພູມນ້ຳເສຍ {actual}°C ເກີນມາດຕະຖານ {limit}°C (ມາດຕາ 31)"
    )]
    TemperatureExceedsLimit { actual: f64, limit: f64 },

    /// Missing pollution monitoring (Article 33)
    /// ຂາດການຕິດຕາມກວດກາມົນລະພິດ (ມາດຕາ 33)
    #[error(
        "Pollution monitoring is required: last inspection was {days_since} days ago (Article 33)\nຕ້ອງມີການຕິດຕາມກວດກາມົນລະພິດ: ການກວດກາຄັ້ງສຸດທ້າຍແມ່ນ {days_since} ມື້ກ່ອນ (ມາດຕາ 33)"
    )]
    MissingPollutionMonitoring { days_since: u32 },

    /// Non-compliant pollution source
    /// ແຫຼ່ງມົນລະພິດບໍ່ປະຕິບັດຕາມ
    #[error(
        "Pollution source '{source_name}' is non-compliant\nແຫຼ່ງມົນລະພິດ '{source_name}' ບໍ່ປະຕິບັດຕາມ"
    )]
    NonCompliantPollutionSource { source_name: String },

    // ========================================================================
    // Protected Area Errors (ຄວາມຜິດພາດເຂດປ່າປ້ອງກັນ)
    // ========================================================================
    /// Unauthorized activity in protected area (Article 40)
    /// ກິດຈະກຳທີ່ບໍ່ໄດ້ຮັບອະນຸຍາດໃນເຂດປ່າປ້ອງກັນ (ມາດຕາ 40)
    #[error(
        "Activity '{activity}' is not permitted in {area_type} (Article 40)\nກິດຈະກຳ '{activity}' ບໍ່ໄດ້ຮັບອະນຸຍາດໃນ {area_type} (ມາດຕາ 40)"
    )]
    UnauthorizedProtectedAreaActivity { activity: String, area_type: String },

    /// Protected area boundary violation (Article 41)
    /// ການລະເມີດເຂດແດນປ່າປ້ອງກັນ (ມາດຕາ 41)
    #[error(
        "Activity within {distance}m of protected area boundary violates buffer zone requirement of {required}m (Article 41)\nກິດຈະກຳພາຍໃນ {distance} ແມັດຈາກເຂດແດນປ່າປ້ອງກັນລະເມີດຂໍ້ກຳນົດເຂດກັນຊົນ {required} ແມັດ (ມາດຕາ 41)"
    )]
    ProtectedAreaBoundaryViolation { distance: u32, required: u32 },

    /// Activity affects endangered species
    /// ກິດຈະກຳມີຜົນກະທົບຕໍ່ຊະນິດພັນທີ່ໃກ້ສູນພັນ
    #[error(
        "Activity threatens endangered species in protected area\nກິດຈະກຳມີໄພຄຸກຄາມຕໍ່ຊະນິດພັນທີ່ໃກ້ສູນພັນໃນເຂດປ່າປ້ອງກັນ"
    )]
    EndangeredSpeciesThreat,

    /// Deforestation in protected area
    /// ການຕັດໄມ້ທຳລາຍປ່າໃນເຂດປ່າປ້ອງກັນ
    #[error(
        "Deforestation of {area} hectares in protected area is prohibited\nຫ້າມການຕັດໄມ້ທຳລາຍປ່າ {area} ເຮັກຕາໃນເຂດປ່າປ້ອງກັນ"
    )]
    DeforestationInProtectedArea { area: f64 },

    // ========================================================================
    // Permit Errors (ຄວາມຜິດພາດໃບອະນຸຍາດ)
    // ========================================================================
    /// Environmental permit expired (Article 25)
    /// ໃບອະນຸຍາດສິ່ງແວດລ້ອມໝົດອາຍຸ (ມາດຕາ 25)
    #[error(
        "Environmental permit {permit_number} expired on {expiry_date} (Article 25)\nໃບອະນຸຍາດສິ່ງແວດລ້ອມ {permit_number} ໝົດອາຍຸວັນທີ {expiry_date} (ມາດຕາ 25)"
    )]
    PermitExpired {
        permit_number: String,
        expiry_date: String,
    },

    /// Permit conditions violated (Article 26)
    /// ການລະເມີດເງື່ອນໄຂໃບອະນຸຍາດ (ມາດຕາ 26)
    #[error(
        "Permit condition violated: {condition} (Article 26)\nເງື່ອນໄຂໃບອະນຸຍາດຖືກລະເມີດ: {condition} (ມາດຕາ 26)"
    )]
    PermitConditionViolated { condition: String },

    /// Missing required permit
    /// ຂາດໃບອະນຸຍາດທີ່ຕ້ອງການ
    #[error(
        "Required environmental permit '{permit_type}' is missing\nຂາດໃບອະນຸຍາດສິ່ງແວດລ້ອມທີ່ຕ້ອງການ '{permit_type}'"
    )]
    MissingRequiredPermit { permit_type: String },

    /// Permit suspended or revoked
    /// ໃບອະນຸຍາດຖືກໂຈະ ຫຼື ຖືກຖອນ
    #[error("Permit {permit_number} has been {status}\nໃບອະນຸຍາດ {permit_number} ຖືກ{status}")]
    PermitSuspendedOrRevoked {
        permit_number: String,
        status: String,
    },

    /// Invalid permit for activity
    /// ໃບອະນຸຍາດບໍ່ຖືກຕ້ອງສຳລັບກິດຈະກຳ
    #[error(
        "Permit type '{permit_type}' is not valid for activity '{activity}'\nປະເພດໃບອະນຸຍາດ '{permit_type}' ບໍ່ຖືກຕ້ອງສຳລັບກິດຈະກຳ '{activity}'"
    )]
    InvalidPermitForActivity {
        permit_type: String,
        activity: String,
    },

    // ========================================================================
    // Compliance Errors (ຄວາມຜິດພາດການປະຕິບັດຕາມ)
    // ========================================================================
    /// General non-compliance (Article 46)
    /// ການບໍ່ປະຕິບັດຕາມທົ່ວໄປ (ມາດຕາ 46)
    #[error(
        "Environmental non-compliance: {violation} (Article {article})\nການບໍ່ປະຕິບັດຕາມກົດໝາຍສິ່ງແວດລ້ອມ: {violation} (ມາດຕາ {article})"
    )]
    NonCompliance { violation: String, article: u32 },

    /// Failed environmental inspection (Article 47)
    /// ການກວດກາສິ່ງແວດລ້ອມບໍ່ຜ່ານ (ມາດຕາ 47)
    #[error(
        "Failed environmental inspection: {reasons} (Article 47)\nການກວດກາສິ່ງແວດລ້ອມບໍ່ຜ່ານ: {reasons} (ມາດຕາ 47)"
    )]
    FailedInspection { reasons: String },

    /// Repeated violation
    /// ການລະເມີດຊ້ຳ
    #[error(
        "Repeated environmental violation: {violation} (violation count: {count})\nການລະເມີດສິ່ງແວດລ້ອມຊ້ຳ: {violation} (ຈຳນວນການລະເມີດ: {count})"
    )]
    RepeatedViolation { violation: String, count: u32 },

    // ========================================================================
    // Waste Disposal Errors (ຄວາມຜິດພາດການກຳຈັດຂີ້ເຫຍື້ອ)
    // ========================================================================
    /// Improper waste disposal (Article 34)
    /// ການກຳຈັດຂີ້ເຫຍື້ອບໍ່ຖືກຕ້ອງ (ມາດຕາ 34)
    #[error(
        "Improper disposal of {waste_type}: method '{method}' is not appropriate (Article 34)\nການກຳຈັດ {waste_type} ບໍ່ຖືກຕ້ອງ: ວິທີ '{method}' ບໍ່ເໝາະສົມ (ມາດຕາ 34)"
    )]
    ImproperWasteDisposal { waste_type: String, method: String },

    /// Hazardous waste handling violation (Article 35)
    /// ການລະເມີດການຈັດການຂີ້ເຫຍື້ອອັນຕະລາຍ (ມາດຕາ 35)
    #[error(
        "Hazardous waste violation: {description} (Article 35)\nການລະເມີດການຈັດການຂີ້ເຫຍື້ອອັນຕະລາຍ: {description} (ມາດຕາ 35)"
    )]
    HazardousWasteViolation { description: String },

    /// Missing waste transport permit
    /// ຂາດໃບອະນຸຍາດຂົນສົ່ງຂີ້ເຫຍື້ອ
    #[error(
        "Hazardous waste transport requires permit (missing for {waste_type})\nການຂົນສົ່ງຂີ້ເຫຍື້ອອັນຕະລາຍຕ້ອງມີໃບອະນຸຍາດ (ຂາດສຳລັບ {waste_type})"
    )]
    MissingWasteTransportPermit { waste_type: String },

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

    /// Environmental emergency
    /// ເຫດສຸກເສີນສິ່ງແວດລ້ອມ
    #[error("Environmental emergency: {description}\nເຫດສຸກເສີນສິ່ງແວດລ້ອມ: {description}")]
    EnvironmentalEmergency { description: String },
}

impl EnvironmentalLawError {
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
            EnvironmentalLawError::EndangeredSpeciesThreat
                | EnvironmentalLawError::DeforestationInProtectedArea { .. }
                | EnvironmentalLawError::HazardousWasteViolation { .. }
                | EnvironmentalLawError::EnvironmentalEmergency { .. }
                | EnvironmentalLawError::UnauthorizedProtectedAreaActivity { .. }
        )
    }

    /// Check if this is a pollution-related error
    /// ກວດສອບວ່າເປັນຄວາມຜິດພາດກ່ຽວກັບມົນລະພິດ
    pub fn is_pollution_related(&self) -> bool {
        matches!(
            self,
            EnvironmentalLawError::AirQualityExceedsLimit { .. }
                | EnvironmentalLawError::WaterQualityExceedsLimit { .. }
                | EnvironmentalLawError::NoiseLevelExceedsLimit { .. }
                | EnvironmentalLawError::PHOutOfRange { .. }
                | EnvironmentalLawError::TemperatureExceedsLimit { .. }
                | EnvironmentalLawError::MissingPollutionMonitoring { .. }
                | EnvironmentalLawError::NonCompliantPollutionSource { .. }
        )
    }

    /// Check if this is an EIA-related error
    /// ກວດສອບວ່າເປັນຄວາມຜິດພາດກ່ຽວກັບ EIA
    pub fn is_eia_related(&self) -> bool {
        matches!(
            self,
            EnvironmentalLawError::MissingEIA
                | EnvironmentalLawError::InvalidProjectType { .. }
                | EnvironmentalLawError::IncompleteEIA { .. }
                | EnvironmentalLawError::MissingPublicConsultation
                | EnvironmentalLawError::EIAExpired { .. }
                | EnvironmentalLawError::MissingMitigationMeasures { .. }
                | EnvironmentalLawError::MissingManagementPlan
        )
    }

    /// Get the article number referenced in this error, if any
    /// ຮັບເລກມາດຕາທີ່ອ້າງອິງໃນຄວາມຜິດພາດນີ້
    pub fn article_number(&self) -> Option<u32> {
        match self {
            EnvironmentalLawError::MissingEIA => Some(18),
            EnvironmentalLawError::IncompleteEIA { .. } => Some(20),
            EnvironmentalLawError::MissingPublicConsultation => Some(21),
            EnvironmentalLawError::EIAExpired { .. } => Some(22),
            EnvironmentalLawError::PermitExpired { .. } => Some(25),
            EnvironmentalLawError::PermitConditionViolated { .. } => Some(26),
            EnvironmentalLawError::AirQualityExceedsLimit { .. } => Some(30),
            EnvironmentalLawError::WaterQualityExceedsLimit { .. } => Some(31),
            EnvironmentalLawError::PHOutOfRange { .. } => Some(31),
            EnvironmentalLawError::TemperatureExceedsLimit { .. } => Some(31),
            EnvironmentalLawError::NoiseLevelExceedsLimit { .. } => Some(32),
            EnvironmentalLawError::MissingPollutionMonitoring { .. } => Some(33),
            EnvironmentalLawError::ImproperWasteDisposal { .. } => Some(34),
            EnvironmentalLawError::HazardousWasteViolation { .. } => Some(35),
            EnvironmentalLawError::UnauthorizedProtectedAreaActivity { .. } => Some(40),
            EnvironmentalLawError::ProtectedAreaBoundaryViolation { .. } => Some(41),
            EnvironmentalLawError::NonCompliance { article, .. } => Some(*article),
            EnvironmentalLawError::FailedInspection { .. } => Some(47),
            _ => None,
        }
    }

    /// Get penalty severity level (1-5, with 5 being most severe)
    /// ຮັບລະດັບຄວາມຮຸນແຮງຂອງໂທດ (1-5, 5 ຮຸນແຮງທີ່ສຸດ)
    pub fn penalty_severity(&self) -> u8 {
        match self {
            // Critical violations - highest penalty
            EnvironmentalLawError::EndangeredSpeciesThreat
            | EnvironmentalLawError::DeforestationInProtectedArea { .. }
            | EnvironmentalLawError::HazardousWasteViolation { .. }
            | EnvironmentalLawError::EnvironmentalEmergency { .. } => 5,

            // Serious violations
            EnvironmentalLawError::UnauthorizedProtectedAreaActivity { .. }
            | EnvironmentalLawError::ProtectedAreaBoundaryViolation { .. }
            | EnvironmentalLawError::RepeatedViolation { .. } => 4,

            // Significant violations
            EnvironmentalLawError::MissingEIA
            | EnvironmentalLawError::AirQualityExceedsLimit { .. }
            | EnvironmentalLawError::WaterQualityExceedsLimit { .. }
            | EnvironmentalLawError::ImproperWasteDisposal { .. } => 3,

            // Moderate violations
            EnvironmentalLawError::PermitExpired { .. }
            | EnvironmentalLawError::PermitConditionViolated { .. }
            | EnvironmentalLawError::EIAExpired { .. }
            | EnvironmentalLawError::NoiseLevelExceedsLimit { .. } => 2,

            // Minor violations or administrative issues
            _ => 1,
        }
    }

    /// Check if this error can be corrected/remedied
    /// ກວດສອບວ່າຄວາມຜິດພາດນີ້ສາມາດແກ້ໄຂໄດ້
    pub fn is_correctable(&self) -> bool {
        !matches!(
            self,
            EnvironmentalLawError::DeforestationInProtectedArea { .. }
                | EnvironmentalLawError::EndangeredSpeciesThreat
                | EnvironmentalLawError::EnvironmentalEmergency { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilingual_error_messages() {
        let error = EnvironmentalLawError::AirQualityExceedsLimit {
            pollutant: "PM2.5".to_string(),
            actual: 35.0,
            limit: 25.0,
            unit: "μg/m³".to_string(),
        };

        let english = error.english_message();
        let lao = error.lao_message();

        assert!(english.contains("Air quality violation"));
        assert!(lao.contains("ການລະເມີດຄຸນນະພາບອາກາດ"));
    }

    #[test]
    fn test_critical_violations() {
        let endangered = EnvironmentalLawError::EndangeredSpeciesThreat;
        assert!(endangered.is_critical());

        let permit = EnvironmentalLawError::PermitExpired {
            permit_number: "EP-001".to_string(),
            expiry_date: "2025-01-01".to_string(),
        };
        assert!(!permit.is_critical());
    }

    #[test]
    fn test_pollution_related() {
        let air = EnvironmentalLawError::AirQualityExceedsLimit {
            pollutant: "PM10".to_string(),
            actual: 60.0,
            limit: 50.0,
            unit: "μg/m³".to_string(),
        };
        assert!(air.is_pollution_related());

        let eia = EnvironmentalLawError::MissingEIA;
        assert!(!eia.is_pollution_related());
    }

    #[test]
    fn test_article_numbers() {
        let eia = EnvironmentalLawError::MissingEIA;
        assert_eq!(eia.article_number(), Some(18));

        let water = EnvironmentalLawError::WaterQualityExceedsLimit {
            parameter: "BOD".to_string(),
            actual: 30.0,
            limit: 20.0,
            unit: "mg/L".to_string(),
        };
        assert_eq!(water.article_number(), Some(31));
    }

    #[test]
    fn test_penalty_severity() {
        let endangered = EnvironmentalLawError::EndangeredSpeciesThreat;
        assert_eq!(endangered.penalty_severity(), 5);

        let validation = EnvironmentalLawError::ValidationError {
            message: "test".to_string(),
        };
        assert_eq!(validation.penalty_severity(), 1);
    }

    #[test]
    fn test_correctable() {
        let permit = EnvironmentalLawError::PermitExpired {
            permit_number: "EP-001".to_string(),
            expiry_date: "2025-01-01".to_string(),
        };
        assert!(permit.is_correctable());

        let deforestation = EnvironmentalLawError::DeforestationInProtectedArea { area: 100.0 };
        assert!(!deforestation.is_correctable());
    }
}
