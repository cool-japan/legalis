//! Health Law Error Types (ປະເພດຄວາມຜິດພາດກົດໝາຍສາທາລະນະສຸກ)
//!
//! Comprehensive error types for Lao health law validation and compliance.
//! All errors include bilingual messages (Lao/English).
//!
//! ## Legal Basis
//!
//! - **Healthcare Law 2014** (Law No. 58/NA)
//! - **Drug and Medical Products Law**
//! - **Public Health Law**

use thiserror::Error;

/// Result type for health law operations
pub type Result<T> = std::result::Result<T, HealthLawError>;

/// Health law errors (ຄວາມຜິດພາດກົດໝາຍສາທາລະນະສຸກ)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum HealthLawError {
    // ========================================================================
    // Facility License Errors (ຄວາມຜິດພາດໃບອະນຸຍາດສະຖານທີ່)
    // ========================================================================
    /// Facility is unlicensed (Article 14)
    /// ສະຖານທີ່ບໍ່ມີໃບອະນຸຍາດ (ມາດຕາ 14)
    #[error(
        "Healthcare facility is unlicensed: {facility_name} (Article 14)\nສະຖານທີ່ສາທາລະນະສຸກບໍ່ມີໃບອະນຸຍາດ: {facility_name} (ມາດຕາ 14)"
    )]
    FacilityUnlicensed { facility_name: String },

    /// Facility license expired (Article 15)
    /// ໃບອະນຸຍາດສະຖານທີ່ໝົດອາຍຸ (ມາດຕາ 15)
    #[error(
        "Healthcare facility license expired on {expiry_date}: {facility_name} (Article 15)\nໃບອະນຸຍາດສະຖານທີ່ໝົດອາຍຸວັນທີ {expiry_date}: {facility_name} (ມາດຕາ 15)"
    )]
    FacilityLicenseExpired {
        facility_name: String,
        expiry_date: String,
    },

    /// Inadequate bed capacity (Article 12)
    /// ຄວາມຈຸຕຽງບໍ່ພຽງພໍ (ມາດຕາ 12)
    #[error(
        "Inadequate bed capacity: {actual} beds, required minimum {required} for {facility_type} (Article 12)\nຄວາມຈຸຕຽງບໍ່ພຽງພໍ: {actual} ຕຽງ, ຕ້ອງການຂັ້ນຕ່ຳ {required} ສຳລັບ {facility_type} (ມາດຕາ 12)"
    )]
    InadequateBedCapacity {
        actual: u32,
        required: u32,
        facility_type: String,
    },

    /// Facility not accredited (Article 16)
    /// ສະຖານທີ່ບໍ່ໄດ້ຮັບຮອງ (ມາດຕາ 16)
    #[error(
        "Healthcare facility not accredited: {facility_name} (Article 16)\nສະຖານທີ່ສາທາລະນະສຸກບໍ່ໄດ້ຮັບຮອງ: {facility_name} (ມາດຕາ 16)"
    )]
    FacilityNotAccredited { facility_name: String },

    /// Facility accreditation suspended (Article 16)
    /// ການຮັບຮອງສະຖານທີ່ຖືກລະງັບ (ມາດຕາ 16)
    #[error(
        "Facility accreditation suspended: {facility_name}, reason: {reason} (Article 16)\nການຮັບຮອງສະຖານທີ່ຖືກລະງັບ: {facility_name}, ເຫດຜົນ: {reason} (ມາດຕາ 16)"
    )]
    FacilityAccreditationSuspended {
        facility_name: String,
        reason: String,
    },

    /// Missing required service (Article 13)
    /// ຂາດການບໍລິການທີ່ຕ້ອງການ (ມາດຕາ 13)
    #[error(
        "Missing required service '{service}' for facility type '{facility_type}' (Article 13)\nຂາດການບໍລິການທີ່ຕ້ອງການ '{service}' ສຳລັບປະເພດສະຖານທີ່ '{facility_type}' (ມາດຕາ 13)"
    )]
    MissingRequiredService {
        service: String,
        facility_type: String,
    },

    // ========================================================================
    // Medical License Errors (ຄວາມຜິດພາດໃບອະນຸຍາດແພດ)
    // ========================================================================
    /// Invalid medical license (Article 23)
    /// ໃບອະນຸຍາດແພດບໍ່ຖືກຕ້ອງ (ມາດຕາ 23)
    #[error(
        "Invalid medical license: {license_number} for {professional_name} (Article 23)\nໃບອະນຸຍາດແພດບໍ່ຖືກຕ້ອງ: {license_number} ສຳລັບ {professional_name} (ມາດຕາ 23)"
    )]
    InvalidMedicalLicense {
        license_number: String,
        professional_name: String,
    },

    /// Medical license expired (Article 25)
    /// ໃບອະນຸຍາດແພດໝົດອາຍຸ (ມາດຕາ 25)
    #[error(
        "Medical license expired on {expiry_date}: {professional_name} (Article 25)\nໃບອະນຸຍາດແພດໝົດອາຍຸວັນທີ {expiry_date}: {professional_name} (ມາດຕາ 25)"
    )]
    MedicalLicenseExpired {
        professional_name: String,
        expiry_date: String,
    },

    /// Medical license suspended (Article 26)
    /// ໃບອະນຸຍາດແພດຖືກລະງັບ (ມາດຕາ 26)
    #[error(
        "Medical license suspended: {professional_name}, reason: {reason} (Article 26)\nໃບອະນຸຍາດແພດຖືກລະງັບ: {professional_name}, ເຫດຜົນ: {reason} (ມາດຕາ 26)"
    )]
    MedicalLicenseSuspended {
        professional_name: String,
        reason: String,
    },

    /// Medical license revoked (Article 27)
    /// ໃບອະນຸຍາດແພດຖືກຍົກເລີກ (ມາດຕາ 27)
    #[error(
        "Medical license revoked: {professional_name}, reason: {reason} (Article 27)\nໃບອະນຸຍາດແພດຖືກຍົກເລີກ: {professional_name}, ເຫດຜົນ: {reason} (ມາດຕາ 27)"
    )]
    MedicalLicenseRevoked {
        professional_name: String,
        reason: String,
    },

    /// Practice scope exceeded (Article 24)
    /// ເກີນຂອບເຂດການປະຕິບັດ (ມາດຕາ 24)
    #[error(
        "Practice scope exceeded: {professional_name} ({profession_type}) performing {procedure} (Article 24)\nເກີນຂອບເຂດການປະຕິບັດ: {professional_name} ({profession_type}) ກຳລັງປະຕິບັດ {procedure} (ມາດຕາ 24)"
    )]
    PracticeScopeExceeded {
        professional_name: String,
        profession_type: String,
        procedure: String,
    },

    /// Unlicensed practitioner (Article 20)
    /// ຜູ້ປະຕິບັດບໍ່ມີໃບອະນຸຍາດ (ມາດຕາ 20)
    #[error(
        "Unlicensed medical practitioner: {professional_name} (Article 20)\nຜູ້ປະຕິບັດການແພດບໍ່ມີໃບອະນຸຍາດ: {professional_name} (ມາດຕາ 20)"
    )]
    UnlicensedPractitioner { professional_name: String },

    // ========================================================================
    // Drug Registration Errors (ຄວາມຜິດພາດການຂຶ້ນທະບຽນຢາ)
    // ========================================================================
    /// Unregistered drug (Drug Law Article 15)
    /// ຢາບໍ່ໄດ້ຂຶ້ນທະບຽນ (ກົດໝາຍຢາ ມາດຕາ 15)
    #[error(
        "Unregistered drug: {drug_name} (Drug Law Article 15)\nຢາບໍ່ໄດ້ຂຶ້ນທະບຽນ: {drug_name} (ກົດໝາຍຢາ ມາດຕາ 15)"
    )]
    UnregisteredDrug { drug_name: String },

    /// Drug registration expired (Drug Law Article 18)
    /// ການຂຶ້ນທະບຽນຢາໝົດອາຍຸ (ກົດໝາຍຢາ ມາດຕາ 18)
    #[error(
        "Drug registration expired on {expiry_date}: {drug_name} (Drug Law Article 18)\nການຂຶ້ນທະບຽນຢາໝົດອາຍຸວັນທີ {expiry_date}: {drug_name} (ກົດໝາຍຢາ ມາດຕາ 18)"
    )]
    DrugRegistrationExpired {
        drug_name: String,
        expiry_date: String,
    },

    /// Drug registration suspended (Drug Law Article 19)
    /// ການຂຶ້ນທະບຽນຢາຖືກລະງັບ (ກົດໝາຍຢາ ມາດຕາ 19)
    #[error(
        "Drug registration suspended: {drug_name}, reason: {reason} (Drug Law Article 19)\nການຂຶ້ນທະບຽນຢາຖືກລະງັບ: {drug_name}, ເຫດຜົນ: {reason} (ກົດໝາຍຢາ ມາດຕາ 19)"
    )]
    DrugRegistrationSuspended { drug_name: String, reason: String },

    /// Counterfeit drug (Drug Law Article 25)
    /// ຢາປອມ (ກົດໝາຍຢາ ມາດຕາ 25)
    #[error(
        "Counterfeit drug detected: {drug_name} (Drug Law Article 25)\nກວດພົບຢາປອມ: {drug_name} (ກົດໝາຍຢາ ມາດຕາ 25)"
    )]
    CounterfeitDrug { drug_name: String },

    /// Controlled substance violation (Drug Law Article 22)
    /// ການລະເມີດຢາຄວບຄຸມ (ກົດໝາຍຢາ ມາດຕາ 22)
    #[error(
        "Controlled substance violation: {drug_name} (Schedule {schedule}) - {violation} (Drug Law Article 22)\nການລະເມີດຢາຄວບຄຸມ: {drug_name} (ຕາຕະລາງ {schedule}) - {violation} (ກົດໝາຍຢາ ມາດຕາ 22)"
    )]
    ControlledSubstanceViolation {
        drug_name: String,
        schedule: u8,
        violation: String,
    },

    /// Invalid prescription (Drug Law Article 20)
    /// ໃບສັ່ງຢາບໍ່ຖືກຕ້ອງ (ກົດໝາຍຢາ ມາດຕາ 20)
    #[error(
        "Invalid prescription for {drug_name}: {reason} (Drug Law Article 20)\nໃບສັ່ງຢາບໍ່ຖືກຕ້ອງສຳລັບ {drug_name}: {reason} (ກົດໝາຍຢາ ມາດຕາ 20)"
    )]
    InvalidPrescription { drug_name: String, reason: String },

    // ========================================================================
    // Patient Rights Errors (ຄວາມຜິດພາດສິດຄົນເຈັບ)
    // ========================================================================
    /// Informed consent violation (Article 33)
    /// ການລະເມີດການຍິນຍອມຮູ້ເຫັນ (ມາດຕາ 33)
    #[error(
        "Informed consent violation: {violation_type} for patient {patient_name} (Article 33)\nການລະເມີດການຍິນຍອມຮູ້ເຫັນ: {violation_type} ສຳລັບຄົນເຈັບ {patient_name} (ມາດຕາ 33)"
    )]
    InformedConsentViolation {
        patient_name: String,
        violation_type: String,
    },

    /// Missing informed consent (Article 33)
    /// ຂາດການຍິນຍອມຮູ້ເຫັນ (ມາດຕາ 33)
    #[error(
        "Missing informed consent for procedure '{procedure}' (Article 33)\nຂາດການຍິນຍອມຮູ້ເຫັນສຳລັບການປິ່ນປົວ '{procedure}' (ມາດຕາ 33)"
    )]
    MissingInformedConsent { procedure: String },

    /// Guardian consent required (Article 33)
    /// ຕ້ອງການການຍິນຍອມຂອງຜູ້ປົກຄອງ (ມາດຕາ 33)
    #[error(
        "Guardian consent required for minor patient (age {patient_age}): {patient_name} (Article 33)\nຕ້ອງການການຍິນຍອມຂອງຜູ້ປົກຄອງສຳລັບຄົນເຈັບຍັງບໍ່ບັນລຸນິຕິພາວະ (ອາຍຸ {patient_age}): {patient_name} (ມາດຕາ 33)"
    )]
    GuardianConsentRequired {
        patient_name: String,
        patient_age: u8,
    },

    /// Privacy breach (Article 34)
    /// ການລະເມີດຄວາມເປັນສ່ວນຕົວ (ມາດຕາ 34)
    #[error(
        "Patient privacy breach: {breach_type} for patient {patient_name} (Article 34)\nການລະເມີດຄວາມເປັນສ່ວນຕົວຄົນເຈັບ: {breach_type} ສຳລັບຄົນເຈັບ {patient_name} (ມາດຕາ 34)"
    )]
    PrivacyBreach {
        patient_name: String,
        breach_type: String,
    },

    /// Unauthorized medical record access (Article 35)
    /// ການເຂົ້າເຖິງເວດບັນທຶກໂດຍບໍ່ໄດ້ຮັບອະນຸຍາດ (ມາດຕາ 35)
    #[error(
        "Unauthorized access to medical records of {patient_name} by {accessor} (Article 35)\nການເຂົ້າເຖິງເວດບັນທຶກໂດຍບໍ່ໄດ້ຮັບອະນຸຍາດຂອງ {patient_name} ໂດຍ {accessor} (ມາດຕາ 35)"
    )]
    UnauthorizedRecordAccess {
        patient_name: String,
        accessor: String,
    },

    /// Discrimination (Article 30)
    /// ການຈຳແນກ (ມາດຕາ 30)
    #[error(
        "Healthcare discrimination: {discrimination_type} against patient {patient_name} (Article 30)\nການຈຳແນກທາງສາທາລະນະສຸກ: {discrimination_type} ຕໍ່ຄົນເຈັບ {patient_name} (ມາດຕາ 30)"
    )]
    Discrimination {
        patient_name: String,
        discrimination_type: String,
    },

    // ========================================================================
    // Public Health Errors (ຄວາມຜິດພາດສາທາລະນະສຸກ)
    // ========================================================================
    /// Quarantine violation (Public Health Law)
    /// ການລະເມີດການກັກກັນ
    #[error(
        "Quarantine violation: {violation_type} by {violator_name} (Public Health Law)\nການລະເມີດການກັກກັນ: {violation_type} ໂດຍ {violator_name} (ກົດໝາຍສາທາລະນະສຸກ)"
    )]
    QuarantineViolation {
        violator_name: String,
        violation_type: String,
    },

    /// Vaccination refusal (mandatory vaccination)
    /// ການປະຕິເສດການສັກວັກຊີນ (ການສັກວັກຊີນບັງຄັບ)
    #[error(
        "Mandatory vaccination refusal: {vaccine_name} refused by {person_name}\nການປະຕິເສດການສັກວັກຊີນບັງຄັບ: {vaccine_name} ປະຕິເສດໂດຍ {person_name}"
    )]
    VaccinationRefusal {
        person_name: String,
        vaccine_name: String,
    },

    /// Public gathering limit exceeded
    /// ເກີນຂອບເຂດການຊຸມນຸມ
    #[error(
        "Public gathering limit exceeded: {actual} persons (maximum allowed: {max_allowed})\nເກີນຂອບເຂດການຊຸມນຸມ: {actual} ຄົນ (ສູງສຸດອະນຸຍາດ: {max_allowed})"
    )]
    PublicGatheringLimitExceeded { actual: u32, max_allowed: u32 },

    /// Unauthorized public health measure
    /// ມາດຕະການສາທາລະນະສຸກບໍ່ໄດ້ຮັບອະນຸຍາດ
    #[error(
        "Unauthorized public health measure at {authority_level} level: {measure_type}\nມາດຕະການສາທາລະນະສຸກບໍ່ໄດ້ຮັບອະນຸຍາດໃນລະດັບ {authority_level}: {measure_type}"
    )]
    UnauthorizedPublicHealthMeasure {
        authority_level: String,
        measure_type: String,
    },

    /// Disease not reported (notifiable disease)
    /// ພະຍາດບໍ່ໄດ້ລາຍງານ (ພະຍາດຕ້ອງລາຍງານ)
    #[error(
        "Notifiable disease not reported: {disease_name} at {location}\nພະຍາດຕ້ອງລາຍງານບໍ່ໄດ້ລາຍງານ: {disease_name} ທີ່ {location}"
    )]
    DiseaseNotReported {
        disease_name: String,
        location: String,
    },

    // ========================================================================
    // Health Insurance Errors (ຄວາມຜິດພາດປະກັນສຸຂະພາບ)
    // ========================================================================
    /// Health insurance coverage insufficient
    /// ການຄຸ້ມຄອງປະກັນສຸຂະພາບບໍ່ພຽງພໍ
    #[error(
        "Health insurance coverage insufficient: {actual_percentage:.1}% (minimum required: {required_percentage:.1}%)\nການຄຸ້ມຄອງປະກັນສຸຂະພາບບໍ່ພຽງພໍ: {actual_percentage:.1}% (ຕ້ອງການຂັ້ນຕ່ຳ: {required_percentage:.1}%)"
    )]
    InsufficientCoverage {
        actual_percentage: f64,
        required_percentage: f64,
    },

    /// Health insurance expired
    /// ປະກັນສຸຂະພາບໝົດອາຍຸ
    #[error(
        "Health insurance expired on {expiry_date} for {member_name}\nປະກັນສຸຂະພາບໝົດອາຍຸວັນທີ {expiry_date} ສຳລັບ {member_name}"
    )]
    InsuranceExpired {
        member_name: String,
        expiry_date: String,
    },

    /// Ineligible for scheme
    /// ບໍ່ມີສິດໃນແຜນ
    #[error("Ineligible for {scheme_name}: {reason}\nບໍ່ມີສິດໃນແຜນ {scheme_name}: {reason}")]
    IneligibleForScheme { scheme_name: String, reason: String },

    /// Unpaid premium
    /// ຄ່າປະກັນບໍ່ໄດ້ຈ່າຍ
    #[error(
        "Unpaid health insurance premium: {amount_lak} LAK for {member_name}\nຄ່າປະກັນສຸຂະພາບບໍ່ໄດ້ຈ່າຍ: {amount_lak} ກີບ ສຳລັບ {member_name}"
    )]
    UnpaidPremium {
        member_name: String,
        amount_lak: u64,
    },

    // ========================================================================
    // Emergency Care Errors (ຄວາມຜິດພາດການປິ່ນປົວສຸກເສີນ)
    // ========================================================================
    /// Emergency care denied (Article 31)
    /// ການປະຕິເສດການປິ່ນປົວສຸກເສີນ (ມາດຕາ 31)
    #[error(
        "Emergency care denied to patient {patient_name}: {reason} (Article 31)\nການປະຕິເສດການປິ່ນປົວສຸກເສີນຕໍ່ຄົນເຈັບ {patient_name}: {reason} (ມາດຕາ 31)"
    )]
    EmergencyCareDenied {
        patient_name: String,
        reason: String,
    },

    /// Emergency response delay
    /// ການຕອບສະໜອງສຸກເສີນຊັກຊ້າ
    #[error(
        "Emergency response delay: {actual_minutes} minutes (required: {required_minutes} minutes)\nການຕອບສະໜອງສຸກເສີນຊັກຊ້າ: {actual_minutes} ນາທີ (ຕ້ອງການ: {required_minutes} ນາທີ)"
    )]
    EmergencyResponseDelay {
        actual_minutes: u32,
        required_minutes: u32,
    },

    /// No emergency services available (Article 13)
    /// ບໍ່ມີການບໍລິການສຸກເສີນ (ມາດຕາ 13)
    #[error(
        "No emergency services available at {facility_name} (Article 13)\nບໍ່ມີການບໍລິການສຸກເສີນທີ່ {facility_name} (ມາດຕາ 13)"
    )]
    NoEmergencyServicesAvailable { facility_name: String },

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

    /// Invalid date
    /// ວັນທີບໍ່ຖືກຕ້ອງ
    #[error("Invalid date: {date_description}\nວັນທີບໍ່ຖືກຕ້ອງ: {date_description}")]
    InvalidDate { date_description: String },

    /// General health law violation
    /// ການລະເມີດກົດໝາຍສາທາລະນະສຸກທົ່ວໄປ
    #[error(
        "Health law violation: {violation} (Article {article})\nການລະເມີດກົດໝາຍສາທາລະນະສຸກ: {violation} (ມາດຕາ {article})"
    )]
    HealthLawViolation { violation: String, article: u32 },
}

impl HealthLawError {
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
            HealthLawError::CounterfeitDrug { .. }
                | HealthLawError::ControlledSubstanceViolation { .. }
                | HealthLawError::EmergencyCareDenied { .. }
                | HealthLawError::UnlicensedPractitioner { .. }
                | HealthLawError::PrivacyBreach { .. }
                | HealthLawError::QuarantineViolation { .. }
                | HealthLawError::DiseaseNotReported { .. }
        )
    }

    /// Check if this is a patient safety issue
    /// ກວດສອບວ່າເປັນບັນຫາຄວາມປອດໄພຂອງຄົນເຈັບ
    pub fn is_patient_safety_issue(&self) -> bool {
        matches!(
            self,
            HealthLawError::UnlicensedPractitioner { .. }
                | HealthLawError::MedicalLicenseExpired { .. }
                | HealthLawError::MedicalLicenseSuspended { .. }
                | HealthLawError::MedicalLicenseRevoked { .. }
                | HealthLawError::PracticeScopeExceeded { .. }
                | HealthLawError::CounterfeitDrug { .. }
                | HealthLawError::EmergencyCareDenied { .. }
                | HealthLawError::NoEmergencyServicesAvailable { .. }
        )
    }

    /// Get the article number referenced in this error, if any
    /// ຮັບເລກມາດຕາທີ່ອ້າງອິງໃນຄວາມຜິດພາດນີ້
    pub fn article_number(&self) -> Option<u32> {
        match self {
            HealthLawError::FacilityUnlicensed { .. } => Some(14),
            HealthLawError::FacilityLicenseExpired { .. } => Some(15),
            HealthLawError::InadequateBedCapacity { .. } => Some(12),
            HealthLawError::FacilityNotAccredited { .. } => Some(16),
            HealthLawError::FacilityAccreditationSuspended { .. } => Some(16),
            HealthLawError::MissingRequiredService { .. } => Some(13),
            HealthLawError::InvalidMedicalLicense { .. } => Some(23),
            HealthLawError::MedicalLicenseExpired { .. } => Some(25),
            HealthLawError::MedicalLicenseSuspended { .. } => Some(26),
            HealthLawError::MedicalLicenseRevoked { .. } => Some(27),
            HealthLawError::PracticeScopeExceeded { .. } => Some(24),
            HealthLawError::UnlicensedPractitioner { .. } => Some(20),
            HealthLawError::InformedConsentViolation { .. } => Some(33),
            HealthLawError::MissingInformedConsent { .. } => Some(33),
            HealthLawError::GuardianConsentRequired { .. } => Some(33),
            HealthLawError::PrivacyBreach { .. } => Some(34),
            HealthLawError::UnauthorizedRecordAccess { .. } => Some(35),
            HealthLawError::Discrimination { .. } => Some(30),
            HealthLawError::EmergencyCareDenied { .. } => Some(31),
            HealthLawError::NoEmergencyServicesAvailable { .. } => Some(13),
            HealthLawError::HealthLawViolation { article, .. } => Some(*article),
            _ => None,
        }
    }

    /// Get the legal basis (law name) for this error
    /// ຮັບພື້ນຖານທາງກົດໝາຍສຳລັບຄວາມຜິດພາດນີ້
    pub fn legal_basis(&self) -> &'static str {
        match self {
            HealthLawError::UnregisteredDrug { .. }
            | HealthLawError::DrugRegistrationExpired { .. }
            | HealthLawError::DrugRegistrationSuspended { .. }
            | HealthLawError::CounterfeitDrug { .. }
            | HealthLawError::ControlledSubstanceViolation { .. }
            | HealthLawError::InvalidPrescription { .. } => "Drug and Medical Products Law",
            HealthLawError::QuarantineViolation { .. }
            | HealthLawError::VaccinationRefusal { .. }
            | HealthLawError::PublicGatheringLimitExceeded { .. }
            | HealthLawError::UnauthorizedPublicHealthMeasure { .. }
            | HealthLawError::DiseaseNotReported { .. } => "Public Health Law",
            _ => "Healthcare Law 2014 (Law No. 58/NA)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilingual_error_messages() {
        let error = HealthLawError::FacilityUnlicensed {
            facility_name: "Test Clinic".to_string(),
        };

        let english = error.english_message();
        let lao = error.lao_message();

        assert!(english.contains("Healthcare facility is unlicensed"));
        assert!(lao.contains("ສະຖານທີ່ສາທາລະນະສຸກບໍ່ມີໃບອະນຸຍາດ"));
    }

    #[test]
    fn test_critical_violations() {
        let counterfeit = HealthLawError::CounterfeitDrug {
            drug_name: "Test Drug".to_string(),
        };
        assert!(counterfeit.is_critical());

        let license_expired = HealthLawError::FacilityLicenseExpired {
            facility_name: "Test Clinic".to_string(),
            expiry_date: "2023-01-01".to_string(),
        };
        assert!(!license_expired.is_critical());
    }

    #[test]
    fn test_patient_safety_issues() {
        let unlicensed = HealthLawError::UnlicensedPractitioner {
            professional_name: "Dr. Test".to_string(),
        };
        assert!(unlicensed.is_patient_safety_issue());

        let privacy = HealthLawError::PrivacyBreach {
            patient_name: "Test Patient".to_string(),
            breach_type: "Unauthorized disclosure".to_string(),
        };
        assert!(!privacy.is_patient_safety_issue());
    }

    #[test]
    fn test_article_numbers() {
        let error = HealthLawError::InformedConsentViolation {
            patient_name: "Test".to_string(),
            violation_type: "No consent obtained".to_string(),
        };
        assert_eq!(error.article_number(), Some(33));

        let error = HealthLawError::FacilityUnlicensed {
            facility_name: "Test".to_string(),
        };
        assert_eq!(error.article_number(), Some(14));
    }

    #[test]
    fn test_legal_basis() {
        let drug_error = HealthLawError::UnregisteredDrug {
            drug_name: "Test Drug".to_string(),
        };
        assert_eq!(drug_error.legal_basis(), "Drug and Medical Products Law");

        let facility_error = HealthLawError::FacilityUnlicensed {
            facility_name: "Test".to_string(),
        };
        assert_eq!(
            facility_error.legal_basis(),
            "Healthcare Law 2014 (Law No. 58/NA)"
        );
    }
}
