//! Forestry Law Error Types (ປະເພດຄວາມຜິດພາດກົດໝາຍປ່າໄມ້)
//!
//! Comprehensive error types for Lao forestry law validation and compliance.
//! All errors include bilingual messages (Lao/English) where applicable.
//!
//! # Legal Reference
//! - Forestry Law 2019 (Law No. 64/NA) - ກົດໝາຍວ່າດ້ວຍປ່າໄມ້ ປີ 2019

use thiserror::Error;

/// Result type for forestry law operations
pub type Result<T> = std::result::Result<T, ForestryLawError>;

/// Forestry law errors (ຄວາມຜິດພາດກົດໝາຍປ່າໄມ້)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ForestryLawError {
    // ========================================================================
    // Harvesting Errors (ຄວາມຜິດພາດການຕັດໄມ້)
    // ========================================================================
    /// Harvesting outside permitted season (Article 48)
    /// ການຕັດໄມ້ນອກລະດູທີ່ອະນຸຍາດ (ມາດຕາ 48)
    #[error(
        "Harvesting is only permitted during dry season (November-April). Month {month} is not allowed (Article 48)\nການຕັດໄມ້ອະນຸຍາດສະເພາະລະດູແລ້ງ (ພະຈິກ-ເມສາ). ເດືອນ {month} ບໍ່ອະນຸຍາດ (ມາດຕາ 48)"
    )]
    HarvestingOutsideSeason { month: u8 },

    /// Diameter below minimum (Article 49)
    /// ເສັ້ນຜ່ານສູນກາງຕ່ຳກວ່າຂັ້ນຕ່ຳ (ມາດຕາ 49)
    #[error(
        "Tree diameter {actual_cm} cm is below minimum {required_cm} cm for {species} (Article 49)\nເສັ້ນຜ່ານສູນກາງ {actual_cm} ຊມ ຕ່ຳກວ່າຂັ້ນຕ່ຳ {required_cm} ຊມ ສຳລັບ {species} (ມາດຕາ 49)"
    )]
    DiameterBelowMinimum {
        actual_cm: u32,
        required_cm: u32,
        species: String,
    },

    /// Harvesting prohibited species (Article 50, 77)
    /// ການຕັດຊະນິດພັນທີ່ຫ້າມ (ມາດຕາ 50, 77)
    #[error(
        "Harvesting of {species} is prohibited - Category I protected species (Articles 50, 77)\nການຕັດ {species} ຖືກຫ້າມ - ຊະນິດພັນປ່າປ້ອງກັນປະເພດ I (ມາດຕາ 50, 77)"
    )]
    ProhibitedSpeciesHarvesting { species: String },

    /// Illegal harvesting
    /// ການຕັດໄມ້ຜິດກົດໝາຍ
    #[error(
        "Illegal harvesting of {species}: {reason} (Article 107)\nການຕັດ {species} ຜິດກົດໝາຍ: {reason} (ມາດຕາ 107)"
    )]
    IllegalHarvesting { species: String, reason: String },

    /// Harvesting in wrong forest type (Articles 11-15)
    /// ການຕັດໄມ້ໃນປະເພດປ່າທີ່ບໍ່ຖືກຕ້ອງ (ມາດຕາ 11-15)
    #[error(
        "Commercial harvesting not permitted in {forest_type} (Article {article})\nການຕັດໄມ້ເພື່ອການຄ້າບໍ່ໄດ້ຮັບອະນຸຍາດໃນ {forest_type} (ມາດຕາ {article})"
    )]
    HarvestingInWrongForestType { forest_type: String, article: u32 },

    /// Missing quota allocation
    /// ຂາດການຈັດສັນໂກຕ້າ
    #[error(
        "Quota allocation required for {species} - Category II managed species (Article 78)\nຕ້ອງມີການຈັດສັນໂກຕ້າສຳລັບ {species} - ຊະນິດພັນຄຸ້ມຄອງປະເພດ II (ມາດຕາ 78)"
    )]
    MissingQuotaAllocation { species: String },

    // ========================================================================
    // Concession Errors (ຄວາມຜິດພາດສຳປະທານ)
    // ========================================================================
    /// Concession area exceeds maximum (Article 62-63)
    /// ເນື້ອທີ່ສຳປະທານເກີນຂີດຈຳກັດສູງສຸດ (ມາດຕາ 62-63)
    #[error(
        "Concession area {actual_ha} hectares exceeds maximum {max_ha} hectares for {concession_type} (Article {article})\nເນື້ອທີ່ສຳປະທານ {actual_ha} ເຮັກຕາ ເກີນສູງສຸດ {max_ha} ເຮັກຕາ ສຳລັບ {concession_type} (ມາດຕາ {article})"
    )]
    ConcessionAreaExceedsMaximum {
        actual_ha: f64,
        max_ha: f64,
        concession_type: String,
        article: u32,
    },

    /// Concession term exceeds maximum (Article 62-63)
    /// ໄລຍະສຳປະທານເກີນຂີດຈຳກັດສູງສຸດ (ມາດຕາ 62-63)
    #[error(
        "Concession term {actual_years} years exceeds maximum {max_years} years for {concession_type} (Article {article})\nໄລຍະສຳປະທານ {actual_years} ປີ ເກີນສູງສຸດ {max_years} ປີ ສຳລັບ {concession_type} (ມາດຕາ {article})"
    )]
    ConcessionTermExceedsMaximum {
        actual_years: u32,
        max_years: u32,
        concession_type: String,
        article: u32,
    },

    /// Insufficient performance bond (Article 62-63)
    /// ເງິນຄ້ຳປະກັນບໍ່ພຽງພໍ (ມາດຕາ 62-63)
    #[error(
        "Performance bond {actual_lak} LAK is below required {required_lak} LAK ({percent}% of project value) (Article {article})\nເງິນຄ້ຳປະກັນ {actual_lak} ກີບ ຕ່ຳກວ່າທີ່ຕ້ອງການ {required_lak} ກີບ ({percent}% ຂອງມູນຄ່າໂຄງການ) (ມາດຕາ {article})"
    )]
    InsufficientPerformanceBond {
        actual_lak: u64,
        required_lak: u64,
        percent: f64,
        article: u32,
    },

    /// Missing Environmental Impact Assessment
    /// ຂາດການປະເມີນຜົນກະທົບສິ່ງແວດລ້ອມ
    #[error(
        "Environmental Impact Assessment (EIA) is required for forest concession\nຕ້ອງມີການປະເມີນຜົນກະທົບສິ່ງແວດລ້ອມ (EIA) ສຳລັບສຳປະທານປ່າໄມ້"
    )]
    MissingEIA,

    /// Missing management plan
    /// ຂາດແຜນການຄຸ້ມຄອງ
    #[error(
        "Sustainable forest management plan is required for concession (Article 62)\nຕ້ອງມີແຜນການຄຸ້ມຄອງປ່າໄມ້ແບບຍືນຍົງສຳລັບສຳປະທານ (ມາດຕາ 62)"
    )]
    MissingManagementPlan,

    // ========================================================================
    // Permit Errors (ຄວາມຜິດພາດໃບອະນຸຍາດ)
    // ========================================================================
    /// Permit expired (Article 121-130)
    /// ໃບອະນຸຍາດໝົດອາຍຸ (ມາດຕາ 121-130)
    #[error(
        "Permit {permit_number} expired on {expiry_date}\nໃບອະນຸຍາດ {permit_number} ໝົດອາຍຸວັນທີ {expiry_date}"
    )]
    PermitExpired {
        permit_number: String,
        expiry_date: String,
    },

    /// Missing required permit
    /// ຂາດໃບອະນຸຍາດທີ່ຕ້ອງການ
    #[error("Required permit '{permit_type}' is missing\nຂາດໃບອະນຸຍາດທີ່ຕ້ອງການ '{permit_type}'")]
    MissingRequiredPermit { permit_type: String },

    /// Permit suspended or revoked
    /// ໃບອະນຸຍາດຖືກໂຈະ ຫຼື ຖືກຖອນ
    #[error("Permit {permit_number} has been {status}\nໃບອະນຸຍາດ {permit_number} ຖືກ {status}")]
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

    /// Transport permit violation (Article 122)
    /// ການລະເມີດໃບອະນຸຍາດຂົນສົ່ງ (ມາດຕາ 122)
    #[error(
        "Transport permit violation: {reason} (Article 122)\nການລະເມີດໃບອະນຸຍາດຂົນສົ່ງ: {reason} (ມາດຕາ 122)"
    )]
    TransportPermitViolation { reason: String },

    // ========================================================================
    // CITES Errors (ຄວາມຜິດພາດ CITES)
    // ========================================================================
    /// CITES permit required (Article 80)
    /// ຕ້ອງມີໃບອະນຸຍາດ CITES (ມາດຕາ 80)
    #[error(
        "CITES permit required for export of {species} (Appendix {appendix}) (Article 80)\nຕ້ອງມີໃບອະນຸຍາດ CITES ສຳລັບການສົ່ງອອກ {species} (ບັນຊີ {appendix}) (ມາດຕາ 80)"
    )]
    CitesPermitRequired { species: String, appendix: String },

    /// CITES violation
    /// ການລະເມີດ CITES
    #[error("CITES violation: {description}\nການລະເມີດ CITES: {description}")]
    CitesViolation { description: String },

    // ========================================================================
    // Chain of Custody Errors (ຄວາມຜິດພາດຕ່ອງໂສ້ການຄຸ້ມຄອງ)
    // ========================================================================
    /// Missing log marking (Article 51)
    /// ຂາດການໝາຍໄມ້ທ່ອນ (ມາດຕາ 51)
    #[error(
        "Log marking/ID is required for chain of custody (Article 51)\nຕ້ອງມີການໝາຍ/ລະຫັດໄມ້ທ່ອນ ສຳລັບຕ່ອງໂສ້ການຄຸ້ມຄອງ (ມາດຕາ 51)"
    )]
    MissingLogMarking,

    /// Broken chain of custody (Article 51)
    /// ຕ່ອງໂສ້ການຄຸ້ມຄອງຂາດຕອນ (ມາດຕາ 51)
    #[error(
        "Chain of custody is incomplete: {gap_description} (Article 51)\nຕ່ອງໂສ້ການຄຸ້ມຄອງບໍ່ຄົບຖ້ວນ: {gap_description} (ມາດຕາ 51)"
    )]
    BrokenChainOfCustody { gap_description: String },

    /// Invalid harvest permit reference
    /// ການອ້າງອິງໃບອະນຸຍາດຕັດບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid harvest permit reference for log {log_id}\nການອ້າງອິງໃບອະນຸຍາດຕັດບໍ່ຖືກຕ້ອງສຳລັບໄມ້ທ່ອນ {log_id}"
    )]
    InvalidHarvestPermitReference { log_id: String },

    // ========================================================================
    // Village Forest Errors (ຄວາມຜິດພາດປ່າບ້ານ)
    // ========================================================================
    /// Missing village forest agreement (Article 92)
    /// ຂາດຂໍ້ຕົກລົງປ່າບ້ານ (ມາດຕາ 92)
    #[error(
        "Village forest management agreement is required (Article 92)\nຕ້ອງມີຂໍ້ຕົກລົງການຄຸ້ມຄອງປ່າບ້ານ (ມາດຕາ 92)"
    )]
    MissingVillageForestAgreement,

    /// Invalid benefit sharing (Article 94)
    /// ການແບ່ງປັນຜົນປະໂຫຍດບໍ່ຖືກຕ້ອງ (ມາດຕາ 94)
    #[error(
        "Benefit sharing percentages must total 100%: village {village}%, district {district}%, national {national}% (Article 94)\nອັດຕາສ່ວນແບ່ງຜົນປະໂຫຍດຕ້ອງລວມ 100%: ບ້ານ {village}%, ເມືອງ {district}%, ລັດ {national}% (ມາດຕາ 94)"
    )]
    InvalidBenefitSharing {
        village: f64,
        district: f64,
        national: f64,
    },

    /// Traditional use violation (Article 95)
    /// ການລະເມີດສິດນຳໃຊ້ແບບດັ້ງເດີມ (ມາດຕາ 95)
    #[error(
        "Traditional use rights must be protected: {description} (Article 95)\nຕ້ອງປົກປ້ອງສິດນຳໃຊ້ແບບດັ້ງເດີມ: {description} (ມາດຕາ 95)"
    )]
    TraditionalUseViolation { description: String },

    // ========================================================================
    // Reforestation Errors (ຄວາມຜິດພາດການປູກປ່າຄືນ)
    // ========================================================================
    /// Reforestation obligation not met (Article 110)
    /// ບໍ່ປະຕິບັດຕາມພັນທະປູກປ່າຄືນ (ມາດຕາ 110)
    #[error(
        "Reforestation obligation not met: required {required_ha} hectares, completed {completed_ha} hectares (Article 110)\nບໍ່ປະຕິບັດຕາມພັນທະປູກປ່າຄືນ: ຕ້ອງການ {required_ha} ເຮັກຕາ, ສຳເລັດ {completed_ha} ເຮັກຕາ (ມາດຕາ 110)"
    )]
    ReforestationObligationNotMet { required_ha: f64, completed_ha: f64 },

    /// Reforestation maintenance period not completed
    /// ບໍ່ສຳເລັດໄລຍະບຳລຸງຮັກສາການປູກປ່າ
    #[error(
        "Reforestation maintenance period of {required_years} years not completed ({completed_years} years elapsed)\nບໍ່ສຳເລັດໄລຍະບຳລຸງຮັກສາການປູກປ່າ {required_years} ປີ (ຜ່ານໄປ {completed_years} ປີ)"
    )]
    ReforestationMaintenanceIncomplete {
        required_years: u32,
        completed_years: u32,
    },

    // ========================================================================
    // Sawmill/Processing Errors (ຄວາມຜິດພາດໂຮງເລື່ອຍ/ການປຸງແຕ່ງ)
    // ========================================================================
    /// Missing sawmill license (Article 123)
    /// ຂາດໃບອະນຸຍາດໂຮງເລື່ອຍ (ມາດຕາ 123)
    #[error(
        "Sawmill license is required for operation (Article 123)\nຕ້ອງມີໃບອະນຸຍາດໂຮງເລື່ອຍສຳລັບການດຳເນີນງານ (ມາດຕາ 123)"
    )]
    MissingSawmillLicense,

    /// Missing log tracking system (Article 51, 123)
    /// ຂາດລະບົບຕິດຕາມໄມ້ທ່ອນ (ມາດຕາ 51, 123)
    #[error(
        "Log intake tracking system is required for sawmill operation (Articles 51, 123)\nຕ້ອງມີລະບົບຕິດຕາມການຮັບໄມ້ເຂົ້າສຳລັບການດຳເນີນງານໂຮງເລື່ອຍ (ມາດຕາ 51, 123)"
    )]
    MissingLogTrackingSystem,

    /// Environmental compliance violation
    /// ການລະເມີດການປະຕິບັດຕາມສິ່ງແວດລ້ອມ
    #[error(
        "Sawmill/processing facility environmental compliance violation: {description}\nການລະເມີດການປະຕິບັດຕາມສິ່ງແວດລ້ອມຂອງໂຮງເລື່ອຍ/ໂຮງງານແປຮູບ: {description}"
    )]
    EnvironmentalComplianceViolation { description: String },

    // ========================================================================
    // Export Errors (ຄວາມຜິດພາດການສົ່ງອອກ)
    // ========================================================================
    /// Log export restriction (Article 124)
    /// ຂໍ້ຈຳກັດການສົ່ງອອກໄມ້ທ່ອນ (ມາດຕາ 124)
    #[error(
        "Export of unprocessed logs is restricted (Article 124)\nການສົ່ງອອກໄມ້ທ່ອນທີ່ບໍ່ໄດ້ແປຮູບຖືກຈຳກັດ (ມາດຕາ 124)"
    )]
    LogExportRestricted,

    /// Missing export documentation
    /// ຂາດເອກະສານສົ່ງອອກ
    #[error(
        "Missing required export documentation: {document}\nຂາດເອກະສານສົ່ງອອກທີ່ຕ້ອງການ: {document}"
    )]
    MissingExportDocumentation { document: String },

    /// Invalid source permits for export
    /// ໃບອະນຸຍາດແຫຼ່ງທີ່ມາບໍ່ຖືກຕ້ອງສຳລັບການສົ່ງອອກ
    #[error(
        "Source permits are required for forest product export\nຕ້ອງມີໃບອະນຸຍາດແຫຼ່ງທີ່ມາສຳລັບການສົ່ງອອກຜະລິດຕະພັນປ່າໄມ້"
    )]
    MissingSourcePermits,

    // ========================================================================
    // Protected Area Errors (ຄວາມຜິດພາດເຂດປ່າປ້ອງກັນ)
    // ========================================================================
    /// Encroachment on protected area (Article 11-12)
    /// ການບຸກລຸກເຂດປ່າປ້ອງກັນ (ມາດຕາ 11-12)
    #[error(
        "Encroachment on {forest_type}: {activity} is prohibited (Article {article})\nການບຸກລຸກ {forest_type}: {activity} ຖືກຫ້າມ (ມາດຕາ {article})"
    )]
    ProtectedAreaEncroachment {
        forest_type: String,
        activity: String,
        article: u32,
    },

    // ========================================================================
    // Penalty Errors (ຄວາມຜິດພາດໂທດ)
    // ========================================================================
    /// Penalty assessment required
    /// ຕ້ອງມີການປະເມີນໂທດ
    #[error(
        "Penalty assessment is required for violation type: {violation_type}\nຕ້ອງມີການປະເມີນໂທດສຳລັບປະເພດການລະເມີດ: {violation_type}"
    )]
    PenaltyAssessmentRequired { violation_type: String },

    /// Repeat offender (Article 107)
    /// ຜູ້ກະທຳຜິດຊ້ຳ (ມາດຕາ 107)
    #[error(
        "Repeat offender - enhanced penalties apply: {description} (Article 107)\nຜູ້ກະທຳຜິດຊ້ຳ - ໃຊ້ໂທດທີ່ເຂັ້ມງວດຂຶ້ນ: {description} (ມາດຕາ 107)"
    )]
    RepeatOffender { description: String },

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

    /// Negative value not allowed
    /// ຄ່າລົບບໍ່ອະນຸຍາດ
    #[error("Negative value not allowed for {field}: {value}\nຄ່າລົບບໍ່ອະນຸຍາດສຳລັບ {field}: {value}")]
    NegativeValueNotAllowed { field: String, value: f64 },
}

impl ForestryLawError {
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
            ForestryLawError::ProhibitedSpeciesHarvesting { .. }
                | ForestryLawError::IllegalHarvesting { .. }
                | ForestryLawError::CitesViolation { .. }
                | ForestryLawError::ProtectedAreaEncroachment { .. }
                | ForestryLawError::RepeatOffender { .. }
        )
    }

    /// Check if this is a permit-related error
    /// ກວດສອບວ່າເປັນຄວາມຜິດພາດກ່ຽວກັບໃບອະນຸຍາດ
    pub fn is_permit_related(&self) -> bool {
        matches!(
            self,
            ForestryLawError::PermitExpired { .. }
                | ForestryLawError::MissingRequiredPermit { .. }
                | ForestryLawError::PermitSuspendedOrRevoked { .. }
                | ForestryLawError::InvalidPermitForActivity { .. }
                | ForestryLawError::TransportPermitViolation { .. }
                | ForestryLawError::MissingSawmillLicense
                | ForestryLawError::CitesPermitRequired { .. }
        )
    }

    /// Check if this is a harvesting-related error
    /// ກວດສອບວ່າເປັນຄວາມຜິດພາດກ່ຽວກັບການຕັດໄມ້
    pub fn is_harvesting_related(&self) -> bool {
        matches!(
            self,
            ForestryLawError::HarvestingOutsideSeason { .. }
                | ForestryLawError::DiameterBelowMinimum { .. }
                | ForestryLawError::ProhibitedSpeciesHarvesting { .. }
                | ForestryLawError::IllegalHarvesting { .. }
                | ForestryLawError::HarvestingInWrongForestType { .. }
                | ForestryLawError::MissingQuotaAllocation { .. }
        )
    }

    /// Get the article number referenced in this error, if any
    /// ຮັບເລກມາດຕາທີ່ອ້າງອິງໃນຄວາມຜິດພາດນີ້
    pub fn article_number(&self) -> Option<u32> {
        match self {
            ForestryLawError::HarvestingOutsideSeason { .. } => Some(48),
            ForestryLawError::DiameterBelowMinimum { .. } => Some(49),
            ForestryLawError::ProhibitedSpeciesHarvesting { .. } => Some(77),
            ForestryLawError::IllegalHarvesting { .. } => Some(107),
            ForestryLawError::HarvestingInWrongForestType { article, .. } => Some(*article),
            ForestryLawError::MissingQuotaAllocation { .. } => Some(78),
            ForestryLawError::ConcessionAreaExceedsMaximum { article, .. } => Some(*article),
            ForestryLawError::ConcessionTermExceedsMaximum { article, .. } => Some(*article),
            ForestryLawError::InsufficientPerformanceBond { article, .. } => Some(*article),
            ForestryLawError::MissingManagementPlan => Some(62),
            ForestryLawError::TransportPermitViolation { .. } => Some(122),
            ForestryLawError::CitesPermitRequired { .. } => Some(80),
            ForestryLawError::MissingLogMarking => Some(51),
            ForestryLawError::BrokenChainOfCustody { .. } => Some(51),
            ForestryLawError::MissingVillageForestAgreement => Some(92),
            ForestryLawError::InvalidBenefitSharing { .. } => Some(94),
            ForestryLawError::TraditionalUseViolation { .. } => Some(95),
            ForestryLawError::ReforestationObligationNotMet { .. } => Some(110),
            ForestryLawError::MissingSawmillLicense => Some(123),
            ForestryLawError::LogExportRestricted => Some(124),
            ForestryLawError::ProtectedAreaEncroachment { article, .. } => Some(*article),
            ForestryLawError::RepeatOffender { .. } => Some(107),
            _ => None,
        }
    }

    /// Get penalty severity level (1-5, with 5 being most severe)
    /// ຮັບລະດັບຄວາມຮຸນແຮງຂອງໂທດ (1-5, 5 ຮຸນແຮງທີ່ສຸດ)
    pub fn penalty_severity(&self) -> u8 {
        match self {
            // Critical violations - highest penalty
            ForestryLawError::ProhibitedSpeciesHarvesting { .. }
            | ForestryLawError::IllegalHarvesting { .. }
            | ForestryLawError::CitesViolation { .. }
            | ForestryLawError::RepeatOffender { .. } => 5,

            // Serious violations
            ForestryLawError::ProtectedAreaEncroachment { .. }
            | ForestryLawError::CitesPermitRequired { .. }
            | ForestryLawError::BrokenChainOfCustody { .. } => 4,

            // Significant violations
            ForestryLawError::HarvestingOutsideSeason { .. }
            | ForestryLawError::DiameterBelowMinimum { .. }
            | ForestryLawError::HarvestingInWrongForestType { .. }
            | ForestryLawError::LogExportRestricted => 3,

            // Moderate violations
            ForestryLawError::PermitExpired { .. }
            | ForestryLawError::TransportPermitViolation { .. }
            | ForestryLawError::ReforestationObligationNotMet { .. }
            | ForestryLawError::MissingSawmillLicense => 2,

            // Minor violations or administrative issues
            _ => 1,
        }
    }

    /// Check if this error can be corrected/remedied
    /// ກວດສອບວ່າຄວາມຜິດພາດນີ້ສາມາດແກ້ໄຂໄດ້
    pub fn is_correctable(&self) -> bool {
        !matches!(
            self,
            ForestryLawError::ProhibitedSpeciesHarvesting { .. }
                | ForestryLawError::IllegalHarvesting { .. }
                | ForestryLawError::CitesViolation { .. }
                | ForestryLawError::ProtectedAreaEncroachment { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilingual_error_messages() {
        let error = ForestryLawError::HarvestingOutsideSeason { month: 7 };

        let english = error.english_message();
        let lao = error.lao_message();

        assert!(english.contains("dry season"));
        assert!(lao.contains("ລະດູແລ້ງ"));
    }

    #[test]
    fn test_critical_violations() {
        let prohibited = ForestryLawError::ProhibitedSpeciesHarvesting {
            species: "Rosewood".to_string(),
        };
        assert!(prohibited.is_critical());

        let permit = ForestryLawError::PermitExpired {
            permit_number: "THP-001".to_string(),
            expiry_date: "2025-01-01".to_string(),
        };
        assert!(!permit.is_critical());
    }

    #[test]
    fn test_harvesting_related() {
        let season = ForestryLawError::HarvestingOutsideSeason { month: 7 };
        assert!(season.is_harvesting_related());

        let cites = ForestryLawError::CitesViolation {
            description: "test".to_string(),
        };
        assert!(!cites.is_harvesting_related());
    }

    #[test]
    fn test_article_numbers() {
        let season = ForestryLawError::HarvestingOutsideSeason { month: 7 };
        assert_eq!(season.article_number(), Some(48));

        let diameter = ForestryLawError::DiameterBelowMinimum {
            actual_cm: 30,
            required_cm: 40,
            species: "Teak".to_string(),
        };
        assert_eq!(diameter.article_number(), Some(49));
    }

    #[test]
    fn test_penalty_severity() {
        let prohibited = ForestryLawError::ProhibitedSpeciesHarvesting {
            species: "Rosewood".to_string(),
        };
        assert_eq!(prohibited.penalty_severity(), 5);

        let validation = ForestryLawError::ValidationError {
            message: "test".to_string(),
        };
        assert_eq!(validation.penalty_severity(), 1);
    }

    #[test]
    fn test_correctable() {
        let permit = ForestryLawError::PermitExpired {
            permit_number: "THP-001".to_string(),
            expiry_date: "2025-01-01".to_string(),
        };
        assert!(permit.is_correctable());

        let illegal = ForestryLawError::IllegalHarvesting {
            species: "Teak".to_string(),
            reason: "No permit".to_string(),
        };
        assert!(!illegal.is_correctable());
    }
}
