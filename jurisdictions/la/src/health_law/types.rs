//! Health Law Types (ປະເພດກົດໝາຍສາທາລະນະສຸກ)
//!
//! Comprehensive type definitions for Lao PDR health law system.
//!
//! ## Legal Basis
//!
//! - **Healthcare Law 2014** (Law No. 58/NA, effective July 2014)
//! - **Drug and Medical Products Law** - Pharmaceutical regulations
//! - **Public Health Law** - Epidemic and disease control
//! - **Traditional Medicine Law** - Herbal and traditional practices
//!
//! ## Healthcare System Structure in Lao PDR
//!
//! The Lao PDR healthcare system is organized into four levels:
//! - **Central Level**: Ministry of Health, Central Hospitals
//! - **Provincial Level**: Provincial Health Departments, Provincial Hospitals
//! - **District Level**: District Health Offices, District Hospitals
//! - **Village Level**: Health Centers, Village Health Volunteers
//!
//! ## Health Insurance Schemes
//!
//! - **Social Security Organization (SSO)**: For formal sector employees
//! - **Community-Based Health Insurance (CBHI)**: For informal sector
//! - **Health Equity Fund (HEF)**: For the poor and vulnerable
//! - **State Employee Scheme**: For government employees

use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants (ຄ່າຄົງທີ່)
// ============================================================================

/// Minimum hospital beds for district hospital (Healthcare Law 2014)
/// ຈຳນວນຕຽງຂັ້ນຕ່ຳສຳລັບໂຮງໝໍເມືອງ
pub const MINIMUM_HOSPITAL_BED_DISTRICT: u32 = 20;

/// Minimum hospital beds for provincial hospital
/// ຈຳນວນຕຽງຂັ້ນຕ່ຳສຳລັບໂຮງໝໍແຂວງ
pub const MINIMUM_HOSPITAL_BED_PROVINCIAL: u32 = 100;

/// Medical license validity period in years (Healthcare Law 2014, Article 25)
/// ໄລຍະເວລາໃບອະນຸຍາດປະກອບວິຊາຊີບແພດ (ປີ)
pub const MEDICAL_LICENSE_VALIDITY_YEARS: u8 = 5;

/// Drug registration validity period in years (Drug Law, Article 18)
/// ໄລຍະເວລາການຂຶ້ນທະບຽນຢາ (ປີ)
pub const DRUG_REGISTRATION_VALIDITY_YEARS: u8 = 5;

/// Minimum age for informed consent (Healthcare Law 2014, Article 33)
/// ອາຍຸຂັ້ນຕ່ຳສຳລັບການຍິນຍອມຮູ້ເຫັນ
pub const INFORMED_CONSENT_MINIMUM_AGE: u8 = 18;

/// Maximum quarantine period for epidemic control (days)
/// ໄລຍະກັກກັນສູງສຸດສຳລັບການຄວບຄຸມພະຍາດລະບາດ (ມື້)
pub const MAXIMUM_QUARANTINE_DAYS: u8 = 30;

/// Emergency response time requirement (minutes)
/// ເວລາຕອບສະໜອງສຸກເສີນທີ່ກຳນົດ (ນາທີ)
pub const EMERGENCY_RESPONSE_TIME_MINUTES: u32 = 30;

/// Controlled substance schedules count
/// ຈຳນວນຕາຕະລາງຢາຄວບຄຸມ
pub const CONTROLLED_SUBSTANCE_SCHEDULES: u8 = 4;

/// Health Insurance coverage percentage minimum
/// ອັດຕາການຄຸ້ມຄອງປະກັນສຸຂະພາບຂັ້ນຕ່ຳ
pub const HEALTH_INSURANCE_COVERAGE_MINIMUM: f64 = 0.70;

// ============================================================================
// Healthcare Facility Types (ປະເພດສະຖານທີ່ສາທາລະນະສຸກ)
// ============================================================================

/// Healthcare facility type (ປະເພດສະຖານທີ່ສາທາລະນະສຸກ)
///
/// Healthcare Law 2014, Article 12: Healthcare facilities are classified as follows
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HealthcareFacilityType {
    /// Central hospital (ໂຮງໝໍສູນກາງ)
    /// Tertiary care facilities with specialized services
    CentralHospital,

    /// Provincial hospital (ໂຮງໝໍແຂວງ)
    /// Secondary care facilities with general specialist services
    ProvincialHospital,

    /// District hospital (ໂຮງໝໍເມືອງ)
    /// Primary/secondary care with basic services
    DistrictHospital,

    /// Health center (ສຸກສາລາ)
    /// Primary care facilities at village/community level
    HealthCenter,

    /// Private clinic (ຄລີນິກເອກະຊົນ)
    /// Privately operated outpatient facility
    Clinic,

    /// Pharmacy (ຮ້ານຂາຍຢາ)
    /// Drug dispensing establishment
    Pharmacy,

    /// Traditional medicine clinic (ຄລີນິກຢາພື້ນເມືອງ)
    /// Traditional Lao medicine practice
    TraditionalMedicineClinic,

    /// Dental clinic (ຄລີນິກແຂ້ວ)
    /// Specialized dental services
    DentalClinic,

    /// Laboratory (ຫ້ອງວິເຄາະ)
    /// Diagnostic laboratory services
    Laboratory,

    /// Maternity center (ສູນແມ່ແລະເດັກ)
    /// Maternal and child health services
    MaternityCenter,

    /// Rehabilitation center (ສູນຟື້ນຟູ)
    /// Physical therapy and rehabilitation
    RehabilitationCenter,
}

impl HealthcareFacilityType {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::CentralHospital => "ໂຮງໝໍສູນກາງ",
            Self::ProvincialHospital => "ໂຮງໝໍແຂວງ",
            Self::DistrictHospital => "ໂຮງໝໍເມືອງ",
            Self::HealthCenter => "ສຸກສາລາ",
            Self::Clinic => "ຄລີນິກເອກະຊົນ",
            Self::Pharmacy => "ຮ້ານຂາຍຢາ",
            Self::TraditionalMedicineClinic => "ຄລີນິກຢາພື້ນເມືອງ",
            Self::DentalClinic => "ຄລີນິກແຂ້ວ",
            Self::Laboratory => "ຫ້ອງວິເຄາະ",
            Self::MaternityCenter => "ສູນແມ່ແລະເດັກ",
            Self::RehabilitationCenter => "ສູນຟື້ນຟູ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::CentralHospital => "Central Hospital",
            Self::ProvincialHospital => "Provincial Hospital",
            Self::DistrictHospital => "District Hospital",
            Self::HealthCenter => "Health Center",
            Self::Clinic => "Private Clinic",
            Self::Pharmacy => "Pharmacy",
            Self::TraditionalMedicineClinic => "Traditional Medicine Clinic",
            Self::DentalClinic => "Dental Clinic",
            Self::Laboratory => "Laboratory",
            Self::MaternityCenter => "Maternity Center",
            Self::RehabilitationCenter => "Rehabilitation Center",
        }
    }

    /// Get minimum bed capacity requirement (if applicable)
    pub fn minimum_bed_capacity(&self) -> Option<u32> {
        match self {
            Self::CentralHospital => Some(200),
            Self::ProvincialHospital => Some(MINIMUM_HOSPITAL_BED_PROVINCIAL),
            Self::DistrictHospital => Some(MINIMUM_HOSPITAL_BED_DISTRICT),
            Self::MaternityCenter => Some(10),
            _ => None,
        }
    }
}

/// Healthcare services offered (ການບໍລິການສາທາລະນະສຸກ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HealthcareService {
    /// Emergency services (ການບໍລິການສຸກເສີນ)
    EmergencyServices,

    /// Outpatient care (ການປິ່ນປົວຜູ້ປ່ວຍນອກ)
    OutpatientCare,

    /// Inpatient care (ການປິ່ນປົວຜູ້ປ່ວຍໃນ)
    InpatientCare,

    /// Surgery (ການຜ່າຕັດ)
    Surgery,

    /// Maternity services (ການບໍລິການແມ່ແລະເດັກ)
    Maternity,

    /// Pediatrics (ການປິ່ນປົວເດັກ)
    Pediatrics,

    /// Traditional medicine (ຢາພື້ນເມືອງ)
    TraditionalMedicine,

    /// Pharmacy services (ການບໍລິການຢາ)
    Pharmacy,

    /// Laboratory services (ການບໍລິການວິເຄາະ)
    Laboratory,

    /// Imaging services (ການບໍລິການພາບຖ່າຍ)
    Imaging,

    /// Vaccination (ການສັກວັກຊີນ)
    Vaccination,

    /// Mental health (ສຸຂະພາບຈິດ)
    MentalHealth,

    /// Dental services (ການບໍລິການແຂ້ວ)
    DentalServices,

    /// Rehabilitation (ການຟື້ນຟູ)
    Rehabilitation,

    /// Dialysis (ການຟອກໄຕ)
    Dialysis,

    /// Intensive care (ການປິ່ນປົວສະເພາະ)
    IntensiveCare,
}

impl HealthcareService {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::EmergencyServices => "ການບໍລິການສຸກເສີນ",
            Self::OutpatientCare => "ການປິ່ນປົວຜູ້ປ່ວຍນອກ",
            Self::InpatientCare => "ການປິ່ນປົວຜູ້ປ່ວຍໃນ",
            Self::Surgery => "ການຜ່າຕັດ",
            Self::Maternity => "ການບໍລິການແມ່ແລະເດັກ",
            Self::Pediatrics => "ການປິ່ນປົວເດັກ",
            Self::TraditionalMedicine => "ຢາພື້ນເມືອງ",
            Self::Pharmacy => "ການບໍລິການຢາ",
            Self::Laboratory => "ການບໍລິການວິເຄາະ",
            Self::Imaging => "ການບໍລິການພາບຖ່າຍ",
            Self::Vaccination => "ການສັກວັກຊີນ",
            Self::MentalHealth => "ສຸຂະພາບຈິດ",
            Self::DentalServices => "ການບໍລິການແຂ້ວ",
            Self::Rehabilitation => "ການຟື້ນຟູ",
            Self::Dialysis => "ການຟອກໄຕ",
            Self::IntensiveCare => "ການປິ່ນປົວສະເພາະ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::EmergencyServices => "Emergency Services",
            Self::OutpatientCare => "Outpatient Care",
            Self::InpatientCare => "Inpatient Care",
            Self::Surgery => "Surgery",
            Self::Maternity => "Maternity Services",
            Self::Pediatrics => "Pediatrics",
            Self::TraditionalMedicine => "Traditional Medicine",
            Self::Pharmacy => "Pharmacy Services",
            Self::Laboratory => "Laboratory Services",
            Self::Imaging => "Imaging Services",
            Self::Vaccination => "Vaccination",
            Self::MentalHealth => "Mental Health",
            Self::DentalServices => "Dental Services",
            Self::Rehabilitation => "Rehabilitation",
            Self::Dialysis => "Dialysis",
            Self::IntensiveCare => "Intensive Care",
        }
    }
}

/// Accreditation status (ສະຖານະການຮັບຮອງ)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AccreditationStatus {
    /// Fully accredited (ຮັບຮອງຄົບຖ້ວນ)
    FullyAccredited {
        /// Accreditation date
        accreditation_date: DateTime<Utc>,
        /// Expiry date
        expiry_date: DateTime<Utc>,
        /// Accreditation body
        accreditation_body: String,
    },

    /// Conditionally accredited (ຮັບຮອງແບບມີເງື່ອນໄຂ)
    ConditionallyAccredited {
        /// Conditions to be met
        conditions: Vec<String>,
        /// Review date
        review_date: DateTime<Utc>,
    },

    /// Pending accreditation (ລໍຖ້າການຮັບຮອງ)
    Pending {
        /// Application date
        application_date: DateTime<Utc>,
    },

    /// Not accredited (ບໍ່ໄດ້ຮັບຮອງ)
    NotAccredited,

    /// Accreditation suspended (ການຮັບຮອງຖືກລະງັບ)
    Suspended {
        /// Suspension date
        suspension_date: DateTime<Utc>,
        /// Reason
        reason: String,
    },

    /// Accreditation revoked (ການຮັບຮອງຖືກຍົກເລີກ)
    Revoked {
        /// Revocation date
        revocation_date: DateTime<Utc>,
        /// Reason
        reason: String,
    },
}

/// Healthcare facility (ສະຖານທີ່ສາທາລະນະສຸກ)
///
/// Healthcare Law 2014, Article 12-15
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HealthcareFacility {
    /// Facility name in Lao (ຊື່ສະຖານທີ່ເປັນພາສາລາວ)
    pub name_lao: String,

    /// Facility name in English (ຊື່ສະຖານທີ່ເປັນພາສາອັງກິດ)
    pub name_en: String,

    /// Facility type (ປະເພດສະຖານທີ່)
    pub facility_type: HealthcareFacilityType,

    /// License number (ເລກທີໃບອະນຸຍາດ)
    pub license_number: String,

    /// Province (ແຂວງ)
    pub province: String,

    /// District (ເມືອງ)
    pub district: String,

    /// Village (ບ້ານ)
    pub village: Option<String>,

    /// Bed capacity (ຄວາມຈຸຕຽງ)
    pub bed_capacity: Option<u32>,

    /// Services offered (ການບໍລິການ)
    pub services_offered: Vec<HealthcareService>,

    /// Accreditation status (ສະຖານະການຮັບຮອງ)
    pub accreditation_status: AccreditationStatus,

    /// License issue date (ວັນທີອອກໃບອະນຸຍາດ)
    pub license_issue_date: DateTime<Utc>,

    /// License expiry date (ວັນທີໝົດອາຍຸໃບອະນຸຍາດ)
    pub license_expiry_date: DateTime<Utc>,

    /// Operating hours (ເວລາເປີດບໍລິການ)
    pub operating_hours: Option<String>,

    /// Emergency services available 24/7 (ມີການບໍລິການສຸກເສີນ 24 ຊົ່ວໂມງ)
    pub emergency_24h: bool,

    /// Contact phone (ເບີໂທລະສັບ)
    pub contact_phone: Option<String>,
}

impl HealthcareFacility {
    /// Check if facility license is valid
    pub fn is_license_valid(&self) -> bool {
        Utc::now() < self.license_expiry_date
    }

    /// Check if facility meets minimum bed capacity requirement
    pub fn meets_bed_capacity_requirement(&self) -> bool {
        match self.facility_type.minimum_bed_capacity() {
            Some(min_capacity) => self.bed_capacity.unwrap_or(0) >= min_capacity,
            None => true,
        }
    }

    /// Check if facility is fully operational
    pub fn is_fully_operational(&self) -> bool {
        self.is_license_valid()
            && matches!(
                self.accreditation_status,
                AccreditationStatus::FullyAccredited { .. }
            )
    }
}

// ============================================================================
// Medical Professional Types (ປະເພດບຸກຄະລາກອນການແພດ)
// ============================================================================

/// Medical profession type (ປະເພດວິຊາຊີບການແພດ)
///
/// Healthcare Law 2014, Article 20-25
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MedicalProfessionType {
    /// Medical doctor (ແພດ)
    Doctor,

    /// Nurse (ພະຍາບານ)
    Nurse,

    /// Midwife (ຜະດຸງຄັນ)
    Midwife,

    /// Pharmacist (ແພດການຢາ)
    Pharmacist,

    /// Dentist (ແພດແຂ້ວ)
    Dentist,

    /// Traditional medicine practitioner (ໝໍພື້ນເມືອງ)
    TraditionalMedicinePractitioner,

    /// Laboratory technician (ນັກວິເຄາະ)
    LaboratoryTechnician,

    /// Paramedic (ນັກປະຖົມພະຍາບານ)
    Paramedic,

    /// Radiologist technician (ນັກວິຊາການຮັງສີ)
    RadiologistTechnician,

    /// Physical therapist (ນັກກາຍະພາບບຳບັດ)
    PhysicalTherapist,

    /// Optometrist (ນັກວັດສາຍຕາ)
    Optometrist,

    /// Medical assistant (ຜູ້ຊ່ວຍແພດ)
    MedicalAssistant,
}

impl MedicalProfessionType {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::Doctor => "ແພດ",
            Self::Nurse => "ພະຍາບານ",
            Self::Midwife => "ຜະດຸງຄັນ",
            Self::Pharmacist => "ແພດການຢາ",
            Self::Dentist => "ແພດແຂ້ວ",
            Self::TraditionalMedicinePractitioner => "ໝໍພື້ນເມືອງ",
            Self::LaboratoryTechnician => "ນັກວິເຄາະ",
            Self::Paramedic => "ນັກປະຖົມພະຍາບານ",
            Self::RadiologistTechnician => "ນັກວິຊາການຮັງສີ",
            Self::PhysicalTherapist => "ນັກກາຍະພາບບຳບັດ",
            Self::Optometrist => "ນັກວັດສາຍຕາ",
            Self::MedicalAssistant => "ຜູ້ຊ່ວຍແພດ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::Doctor => "Doctor",
            Self::Nurse => "Nurse",
            Self::Midwife => "Midwife",
            Self::Pharmacist => "Pharmacist",
            Self::Dentist => "Dentist",
            Self::TraditionalMedicinePractitioner => "Traditional Medicine Practitioner",
            Self::LaboratoryTechnician => "Laboratory Technician",
            Self::Paramedic => "Paramedic",
            Self::RadiologistTechnician => "Radiologist Technician",
            Self::PhysicalTherapist => "Physical Therapist",
            Self::Optometrist => "Optometrist",
            Self::MedicalAssistant => "Medical Assistant",
        }
    }

    /// Get minimum education requirement (years)
    pub fn minimum_education_years(&self) -> u8 {
        match self {
            Self::Doctor => 6,
            Self::Dentist => 5,
            Self::Pharmacist => 5,
            Self::Nurse => 3,
            Self::Midwife => 3,
            Self::LaboratoryTechnician => 3,
            Self::RadiologistTechnician => 3,
            Self::PhysicalTherapist => 4,
            Self::TraditionalMedicinePractitioner => 2,
            Self::Paramedic => 2,
            Self::Optometrist => 4,
            Self::MedicalAssistant => 2,
        }
    }
}

/// License status (ສະຖານະໃບອະນຸຍາດ)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LicenseStatus {
    /// Active license (ໃບອະນຸຍາດໃຊ້ໄດ້)
    Active,

    /// Expired license (ໃບອະນຸຍາດໝົດອາຍຸ)
    Expired {
        /// Expiry date
        expired_on: DateTime<Utc>,
    },

    /// Suspended license (ໃບອະນຸຍາດຖືກລະງັບ)
    Suspended {
        /// Suspension date
        suspended_on: DateTime<Utc>,
        /// Reason for suspension
        reason: String,
        /// Expected reinstatement date
        reinstatement_date: Option<DateTime<Utc>>,
    },

    /// Revoked license (ໃບອະນຸຍາດຖືກຍົກເລີກ)
    Revoked {
        /// Revocation date
        revoked_on: DateTime<Utc>,
        /// Reason for revocation
        reason: String,
    },

    /// Pending renewal (ລໍຖ້າຕໍ່ອາຍຸ)
    PendingRenewal {
        /// Renewal application date
        application_date: DateTime<Utc>,
    },
}

/// Medical professional (ບຸກຄະລາກອນການແພດ)
///
/// Healthcare Law 2014, Article 20-25
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MedicalProfessional {
    /// Name (ຊື່)
    pub name: String,

    /// Name in Lao (ຊື່ເປັນພາສາລາວ)
    pub name_lao: Option<String>,

    /// Profession type (ປະເພດວິຊາຊີບ)
    pub profession_type: MedicalProfessionType,

    /// License number (ເລກທີໃບອະນຸຍາດ)
    pub license_number: String,

    /// License issue date (ວັນທີອອກໃບອະນຸຍາດ)
    pub license_issue_date: DateTime<Utc>,

    /// License expiry date (ວັນທີໝົດອາຍຸໃບອະນຸຍາດ)
    pub license_expiry_date: DateTime<Utc>,

    /// Specialization (ຄວາມຊ່ຽວຊານ)
    pub specialization: Option<String>,

    /// Practicing facility (ສະຖານທີ່ປະຕິບັດງານ)
    pub practicing_facility: String,

    /// License status (ສະຖານະໃບອະນຸຍາດ)
    pub license_status: LicenseStatus,

    /// Education qualification (ວຸດທິການສຶກສາ)
    pub education_qualification: Option<String>,

    /// Years of experience (ປະສົບການ)
    pub years_of_experience: Option<u32>,
}

impl MedicalProfessional {
    /// Check if license is currently valid
    pub fn is_license_valid(&self) -> bool {
        matches!(self.license_status, LicenseStatus::Active)
            && Utc::now() < self.license_expiry_date
    }

    /// Get days until license expiry
    pub fn days_until_expiry(&self) -> i64 {
        (self.license_expiry_date - Utc::now()).num_days()
    }

    /// Check if license needs renewal soon (within 90 days)
    pub fn needs_renewal_soon(&self) -> bool {
        let days = self.days_until_expiry();
        days > 0 && days <= 90
    }
}

// ============================================================================
// Drug Registration Types (ປະເພດການຂຶ້ນທະບຽນຢາ)
// ============================================================================

/// Drug category (ປະເພດຢາ)
///
/// Drug and Medical Products Law
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DrugCategory {
    /// Prescription only (ຢາຕາມໃບສັ່ງແພດ)
    PrescriptionOnly,

    /// Pharmacist recommended (ຢາແນະນຳໂດຍເພັດການຢາ)
    PharmacistRecommended,

    /// Over the counter (ຢາຂາຍທົ່ວໄປ)
    OverTheCounter,

    /// Controlled substance (ຢາຄວບຄຸມ)
    ControlledSubstance {
        /// Schedule number (1-4)
        schedule: u8,
    },

    /// Traditional medicine (ຢາພື້ນເມືອງ)
    TraditionalMedicine,

    /// Vaccine (ວັກຊີນ)
    Vaccine,

    /// Biological product (ຜະລິດຕະພັນຊີວະພາບ)
    BiologicalProduct,

    /// Medical device (ອຸປະກອນການແພດ)
    MedicalDevice,
}

impl DrugCategory {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::PrescriptionOnly => "ຢາຕາມໃບສັ່ງແພດ",
            Self::PharmacistRecommended => "ຢາແນະນຳໂດຍເພັດການຢາ",
            Self::OverTheCounter => "ຢາຂາຍທົ່ວໄປ",
            Self::ControlledSubstance { .. } => "ຢາຄວບຄຸມ",
            Self::TraditionalMedicine => "ຢາພື້ນເມືອງ",
            Self::Vaccine => "ວັກຊີນ",
            Self::BiologicalProduct => "ຜະລິດຕະພັນຊີວະພາບ",
            Self::MedicalDevice => "ອຸປະກອນການແພດ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::PrescriptionOnly => "Prescription Only",
            Self::PharmacistRecommended => "Pharmacist Recommended",
            Self::OverTheCounter => "Over The Counter",
            Self::ControlledSubstance { .. } => "Controlled Substance",
            Self::TraditionalMedicine => "Traditional Medicine",
            Self::Vaccine => "Vaccine",
            Self::BiologicalProduct => "Biological Product",
            Self::MedicalDevice => "Medical Device",
        }
    }

    /// Check if requires prescription
    pub fn requires_prescription(&self) -> bool {
        matches!(
            self,
            Self::PrescriptionOnly | Self::ControlledSubstance { .. } | Self::Vaccine
        )
    }
}

/// Registration status (ສະຖານະການຂຶ້ນທະບຽນ)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RegistrationStatus {
    /// Registered (ຂຶ້ນທະບຽນແລ້ວ)
    Registered,

    /// Pending registration (ລໍຖ້າການຂຶ້ນທະບຽນ)
    Pending,

    /// Registration expired (ການຂຶ້ນທະບຽນໝົດອາຍຸ)
    Expired {
        /// Expiry date
        expired_on: DateTime<Utc>,
    },

    /// Registration suspended (ການຂຶ້ນທະບຽນຖືກລະງັບ)
    Suspended {
        /// Suspension date
        suspended_on: DateTime<Utc>,
        /// Reason
        reason: String,
    },

    /// Registration revoked (ການຂຶ້ນທະບຽນຖືກຍົກເລີກ)
    Revoked {
        /// Revocation date
        revoked_on: DateTime<Utc>,
        /// Reason
        reason: String,
    },

    /// Not registered (ບໍ່ໄດ້ຂຶ້ນທະບຽນ)
    NotRegistered,
}

/// Drug registration (ການຂຶ້ນທະບຽນຢາ)
///
/// Drug and Medical Products Law, Article 15-20
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DrugRegistration {
    /// Generic name (ຊື່ສາມັນ)
    pub drug_name_generic: String,

    /// Brand name (ຊື່ການຄ້າ)
    pub drug_name_brand: String,

    /// Registration number (ເລກທີການຂຶ້ນທະບຽນ)
    pub registration_number: String,

    /// Manufacturer (ຜູ້ຜະລິດ)
    pub manufacturer: String,

    /// Country of origin (ປະເທດຕົ້ນກຳເນີດ)
    pub country_of_origin: String,

    /// Drug category (ປະເພດຢາ)
    pub drug_category: DrugCategory,

    /// Registration date (ວັນທີຂຶ້ນທະບຽນ)
    pub registration_date: DateTime<Utc>,

    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: DateTime<Utc>,

    /// Registration status (ສະຖານະການຂຶ້ນທະບຽນ)
    pub status: RegistrationStatus,

    /// Therapeutic class (ປະເພດການປິ່ນປົວ)
    pub therapeutic_class: Option<String>,

    /// Dosage form (ຮູບແບບຢາ)
    pub dosage_form: Option<String>,

    /// Strength (ຄວາມເຂັ້ມຂົ້ນ)
    pub strength: Option<String>,
}

impl DrugRegistration {
    /// Check if registration is valid
    pub fn is_registration_valid(&self) -> bool {
        matches!(self.status, RegistrationStatus::Registered) && Utc::now() < self.expiry_date
    }

    /// Get days until expiry
    pub fn days_until_expiry(&self) -> i64 {
        (self.expiry_date - Utc::now()).num_days()
    }
}

// ============================================================================
// Patient Rights Types (ສິດຂອງຄົນເຈັບ)
// ============================================================================

/// Patient right type (ປະເພດສິດຂອງຄົນເຈັບ)
///
/// Healthcare Law 2014, Article 30-35
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PatientRightType {
    /// Informed consent (ການຍິນຍອມຮູ້ເຫັນ)
    InformedConsent,

    /// Privacy and confidentiality (ຄວາມເປັນສ່ວນຕົວແລະຄວາມລັບ)
    PrivacyAndConfidentiality,

    /// Access to medical records (ການເຂົ້າເຖິງເວດບັນທຶກ)
    AccessToMedicalRecords,

    /// Right to refuse treatment (ສິດປະຕິເສດການປິ່ນປົວ)
    RightToRefuseTreatment,

    /// Emergency care (ການປິ່ນປົວສຸກເສີນ)
    EmergencyCare,

    /// Second opinion (ການຂໍຄວາມເຫັນທີສອງ)
    SecondOpinion,

    /// Complaint (ການຮ້ອງທຸກ)
    Complaint,

    /// Non-discrimination (ບໍ່ຈຳແນກ)
    NonDiscrimination,

    /// Information about treatment (ຂໍ້ມູນກ່ຽວກັບການປິ່ນປົວ)
    InformationAboutTreatment,

    /// Dignity and respect (ກຽດສັກສີແລະຄວາມເຄົາລົບ)
    DignityAndRespect,
}

impl PatientRightType {
    /// Get the article reference in Healthcare Law 2014
    pub fn article_reference(&self) -> u16 {
        match self {
            Self::InformedConsent => 33,
            Self::PrivacyAndConfidentiality => 34,
            Self::AccessToMedicalRecords => 35,
            Self::RightToRefuseTreatment => 33,
            Self::EmergencyCare => 31,
            Self::SecondOpinion => 32,
            Self::Complaint => 36,
            Self::NonDiscrimination => 30,
            Self::InformationAboutTreatment => 32,
            Self::DignityAndRespect => 30,
        }
    }

    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::InformedConsent => "ການຍິນຍອມຮູ້ເຫັນ",
            Self::PrivacyAndConfidentiality => "ຄວາມເປັນສ່ວນຕົວແລະຄວາມລັບ",
            Self::AccessToMedicalRecords => "ການເຂົ້າເຖິງເວດບັນທຶກ",
            Self::RightToRefuseTreatment => "ສິດປະຕິເສດການປິ່ນປົວ",
            Self::EmergencyCare => "ການປິ່ນປົວສຸກເສີນ",
            Self::SecondOpinion => "ການຂໍຄວາມເຫັນທີສອງ",
            Self::Complaint => "ການຮ້ອງທຸກ",
            Self::NonDiscrimination => "ບໍ່ຈຳແນກ",
            Self::InformationAboutTreatment => "ຂໍ້ມູນກ່ຽວກັບການປິ່ນປົວ",
            Self::DignityAndRespect => "ກຽດສັກສີແລະຄວາມເຄົາລົບ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::InformedConsent => "Informed Consent",
            Self::PrivacyAndConfidentiality => "Privacy and Confidentiality",
            Self::AccessToMedicalRecords => "Access to Medical Records",
            Self::RightToRefuseTreatment => "Right to Refuse Treatment",
            Self::EmergencyCare => "Emergency Care",
            Self::SecondOpinion => "Second Opinion",
            Self::Complaint => "Complaint",
            Self::NonDiscrimination => "Non-discrimination",
            Self::InformationAboutTreatment => "Information About Treatment",
            Self::DignityAndRespect => "Dignity and Respect",
        }
    }
}

/// Patient rights record (ບັນທຶກສິດຂອງຄົນເຈັບ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PatientRights {
    /// Right type (ປະເພດສິດ)
    pub right_type: PatientRightType,

    /// Article reference in Healthcare Law 2014 (ອ້າງອິງມາດຕາ)
    pub article_reference: u16,

    /// Description in Lao (ຄຳອະທິບາຍເປັນພາສາລາວ)
    pub description_lao: String,

    /// Description in English (ຄຳອະທິບາຍເປັນພາສາອັງກິດ)
    pub description_en: String,
}

// ============================================================================
// Public Health Types (ສາທາລະນະສຸກ)
// ============================================================================

/// Public health measure type (ປະເພດມາດຕະການສາທາລະນະສຸກ)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PublicHealthMeasureType {
    /// Vaccination (ການສັກວັກຊີນ)
    Vaccination {
        /// Disease name
        disease: String,
    },

    /// Quarantine (ການກັກກັນ)
    Quarantine {
        /// Duration in days
        duration_days: u8,
    },

    /// Isolation (ການແຍກຕົວ)
    Isolation,

    /// Contact tracing (ການຕິດຕາມຜູ້ສຳຜັດ)
    ContactTracing,

    /// Travel restriction (ການຈຳກັດການເດີນທາງ)
    TravelRestriction,

    /// Public gathering limit (ການຈຳກັດການຊຸມນຸມ)
    PublicGathering {
        /// Maximum persons allowed
        max_persons: u32,
    },

    /// Mask mandate (ການບັງຄັບໃສ່ໜ້າກາກ)
    MaskMandate,

    /// Disinfection (ການຂ້າເຊື້ອ)
    Disinfection,

    /// Health screening (ການກວດສຸຂະພາບ)
    HealthScreening,

    /// Border control (ການຄວບຄຸມຊາຍແດນ)
    BorderControl,
}

impl PublicHealthMeasureType {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::Vaccination { .. } => "ການສັກວັກຊີນ",
            Self::Quarantine { .. } => "ການກັກກັນ",
            Self::Isolation => "ການແຍກຕົວ",
            Self::ContactTracing => "ການຕິດຕາມຜູ້ສຳຜັດ",
            Self::TravelRestriction => "ການຈຳກັດການເດີນທາງ",
            Self::PublicGathering { .. } => "ການຈຳກັດການຊຸມນຸມ",
            Self::MaskMandate => "ການບັງຄັບໃສ່ໜ້າກາກ",
            Self::Disinfection => "ການຂ້າເຊື້ອ",
            Self::HealthScreening => "ການກວດສຸຂະພາບ",
            Self::BorderControl => "ການຄວບຄຸມຊາຍແດນ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::Vaccination { .. } => "Vaccination",
            Self::Quarantine { .. } => "Quarantine",
            Self::Isolation => "Isolation",
            Self::ContactTracing => "Contact Tracing",
            Self::TravelRestriction => "Travel Restriction",
            Self::PublicGathering { .. } => "Public Gathering Limit",
            Self::MaskMandate => "Mask Mandate",
            Self::Disinfection => "Disinfection",
            Self::HealthScreening => "Health Screening",
            Self::BorderControl => "Border Control",
        }
    }
}

/// Authority level (ລະດັບອຳນາດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AuthorityLevel {
    /// National level (ລະດັບຊາດ)
    National,

    /// Provincial level (ລະດັບແຂວງ)
    Provincial,

    /// District level (ລະດັບເມືອງ)
    District,

    /// Village level (ລະດັບບ້ານ)
    Village,
}

impl AuthorityLevel {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::National => "ລະດັບຊາດ",
            Self::Provincial => "ລະດັບແຂວງ",
            Self::District => "ລະດັບເມືອງ",
            Self::Village => "ລະດັບບ້ານ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::National => "National",
            Self::Provincial => "Provincial",
            Self::District => "District",
            Self::Village => "Village",
        }
    }
}

/// Public health measure (ມາດຕະການສາທາລະນະສຸກ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PublicHealthMeasure {
    /// Measure type (ປະເພດມາດຕະການ)
    pub measure_type: PublicHealthMeasureType,

    /// Authority level (ລະດັບອຳນາດ)
    pub authority_level: AuthorityLevel,

    /// Legal basis (ພື້ນຖານທາງກົດໝາຍ)
    pub legal_basis: String,

    /// Enforcement date (ວັນທີບັງຄັບໃຊ້)
    pub enforcement_date: DateTime<Utc>,

    /// End date (if known) (ວັນທີສິ້ນສຸດ)
    pub end_date: Option<DateTime<Utc>>,

    /// Description in Lao (ຄຳອະທິບາຍເປັນພາສາລາວ)
    pub description_lao: Option<String>,

    /// Description in English (ຄຳອະທິບາຍເປັນພາສາອັງກິດ)
    pub description_en: Option<String>,

    /// Penalty for violation (ໂທດສຳລັບການລະເມີດ)
    pub penalty_for_violation: Option<String>,
}

impl PublicHealthMeasure {
    /// Check if measure is currently active
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        now >= self.enforcement_date && self.end_date.is_none_or(|end| now < end)
    }
}

// ============================================================================
// Health Insurance Types (ປະກັນສຸຂະພາບ)
// ============================================================================

/// Health insurance scheme (ແຜນປະກັນສຸຂະພາບ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HealthInsuranceScheme {
    /// Social Security Organization (ອົງການປະກັນສັງຄົມ)
    SocialSecurityOrganization,

    /// Community-Based Health Insurance (ປະກັນສຸຂະພາບຊຸມຊົນ)
    CommunityBasedHealthInsurance,

    /// Health Equity Fund (ກອງທຶນຊ່ວຍເຫຼືອສຸຂະພາບ)
    HealthEquityFund,

    /// Private Health Insurance (ປະກັນສຸຂະພາບເອກະຊົນ)
    PrivateHealthInsurance,

    /// State Employee Scheme (ແຜນສຳລັບພະນັກງານລັດ)
    StateEmployeeScheme,

    /// Military Health Scheme (ແຜນສຸຂະພາບທະຫານ)
    MilitaryHealthScheme,
}

impl HealthInsuranceScheme {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::SocialSecurityOrganization => "ອົງການປະກັນສັງຄົມ",
            Self::CommunityBasedHealthInsurance => "ປະກັນສຸຂະພາບຊຸມຊົນ",
            Self::HealthEquityFund => "ກອງທຶນຊ່ວຍເຫຼືອສຸຂະພາບ",
            Self::PrivateHealthInsurance => "ປະກັນສຸຂະພາບເອກະຊົນ",
            Self::StateEmployeeScheme => "ແຜນສຳລັບພະນັກງານລັດ",
            Self::MilitaryHealthScheme => "ແຜນສຸຂະພາບທະຫານ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::SocialSecurityOrganization => "Social Security Organization (SSO)",
            Self::CommunityBasedHealthInsurance => "Community-Based Health Insurance (CBHI)",
            Self::HealthEquityFund => "Health Equity Fund (HEF)",
            Self::PrivateHealthInsurance => "Private Health Insurance",
            Self::StateEmployeeScheme => "State Employee Scheme",
            Self::MilitaryHealthScheme => "Military Health Scheme",
        }
    }

    /// Get typical coverage percentage
    pub fn typical_coverage_percentage(&self) -> f64 {
        match self {
            Self::SocialSecurityOrganization => 0.80,
            Self::CommunityBasedHealthInsurance => 0.75,
            Self::HealthEquityFund => 1.00, // Full coverage for the poor
            Self::PrivateHealthInsurance => 0.90,
            Self::StateEmployeeScheme => 0.85,
            Self::MilitaryHealthScheme => 1.00,
        }
    }
}

/// Beneficiary category (ປະເພດຜູ້ຮັບຜົນປະໂຫຍດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BeneficiaryCategory {
    /// Formal sector employee (ພະນັກງານພາກສ່ວນທາງການ)
    FormalSectorEmployee,

    /// Informal sector worker (ແຮງງານພາກສ່ວນບໍ່ທາງການ)
    InformalSectorWorker,

    /// Government employee (ພະນັກງານລັດ)
    GovernmentEmployee,

    /// Poor and vulnerable (ຄົນທຸກຍາກ)
    PoorAndVulnerable,

    /// Child under 5 (ເດັກອາຍຸຕ່ຳກວ່າ 5 ປີ)
    ChildUnder5,

    /// Pregnant woman (ແມ່ຍິງຖືພາ)
    PregnantWoman,

    /// Elderly (ຜູ້ສູງອາຍຸ)
    Elderly,

    /// Person with disability (ຄົນພິການ)
    PersonWithDisability,

    /// Military personnel (ທະຫານ)
    MilitaryPersonnel,
}

impl BeneficiaryCategory {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::FormalSectorEmployee => "ພະນັກງານພາກສ່ວນທາງການ",
            Self::InformalSectorWorker => "ແຮງງານພາກສ່ວນບໍ່ທາງການ",
            Self::GovernmentEmployee => "ພະນັກງານລັດ",
            Self::PoorAndVulnerable => "ຄົນທຸກຍາກ",
            Self::ChildUnder5 => "ເດັກອາຍຸຕ່ຳກວ່າ 5 ປີ",
            Self::PregnantWoman => "ແມ່ຍິງຖືພາ",
            Self::Elderly => "ຜູ້ສູງອາຍຸ",
            Self::PersonWithDisability => "ຄົນພິການ",
            Self::MilitaryPersonnel => "ທະຫານ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::FormalSectorEmployee => "Formal Sector Employee",
            Self::InformalSectorWorker => "Informal Sector Worker",
            Self::GovernmentEmployee => "Government Employee",
            Self::PoorAndVulnerable => "Poor and Vulnerable",
            Self::ChildUnder5 => "Child Under 5",
            Self::PregnantWoman => "Pregnant Woman",
            Self::Elderly => "Elderly",
            Self::PersonWithDisability => "Person with Disability",
            Self::MilitaryPersonnel => "Military Personnel",
        }
    }
}

/// Health insurance record (ບັນທຶກປະກັນສຸຂະພາບ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HealthInsurance {
    /// Scheme type (ປະເພດແຜນ)
    pub scheme_type: HealthInsuranceScheme,

    /// Coverage percentage (ອັດຕາການຄຸ້ມຄອງ)
    pub coverage_percentage: f64,

    /// Beneficiary category (ປະເພດຜູ້ຮັບຜົນປະໂຫຍດ)
    pub beneficiary_category: BeneficiaryCategory,

    /// Member ID (ເລກສະມາຊິກ)
    pub member_id: Option<String>,

    /// Enrollment date (ວັນທີຂຶ້ນທະບຽນ)
    pub enrollment_date: Option<DateTime<Utc>>,

    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: Option<DateTime<Utc>>,

    /// Annual premium in LAK (ຄ່າປະກັນປະຈຳປີ)
    pub annual_premium_lak: Option<u64>,

    /// Employer contribution (ການປະກອບສ່ວນຂອງນາຍຈ້າງ)
    pub employer_contribution: Option<u64>,

    /// Employee contribution (ການປະກອບສ່ວນຂອງລູກຈ້າງ)
    pub employee_contribution: Option<u64>,
}

impl HealthInsurance {
    /// Check if insurance is active
    pub fn is_active(&self) -> bool {
        if let Some(expiry) = self.expiry_date {
            Utc::now() < expiry
        } else {
            true // No expiry date means perpetual coverage
        }
    }

    /// Calculate total annual contribution
    pub fn total_annual_contribution(&self) -> u64 {
        self.employer_contribution.unwrap_or(0) + self.employee_contribution.unwrap_or(0)
    }
}

// ============================================================================
// Informed Consent Types (ການຍິນຍອມຮູ້ເຫັນ)
// ============================================================================

/// Informed consent status (ສະຖານະການຍິນຍອມຮູ້ເຫັນ)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InformedConsentStatus {
    /// Consent given (ຍິນຍອມແລ້ວ)
    ConsentGiven {
        /// Date of consent
        consent_date: DateTime<Utc>,
        /// Witness name
        witness_name: Option<String>,
    },

    /// Consent refused (ປະຕິເສດການຍິນຍອມ)
    ConsentRefused {
        /// Date of refusal
        refusal_date: DateTime<Utc>,
        /// Reason for refusal
        reason: Option<String>,
    },

    /// Consent withdrawn (ຖອນການຍິນຍອມ)
    ConsentWithdrawn {
        /// Date of withdrawal
        withdrawal_date: DateTime<Utc>,
        /// Reason for withdrawal
        reason: Option<String>,
    },

    /// Consent not required (emergency) (ບໍ່ຕ້ອງການການຍິນຍອມ - ກໍລະນີສຸກເສີນ)
    NotRequired {
        /// Reason not required
        reason: String,
    },

    /// Consent pending (ລໍຖ້າການຍິນຍອມ)
    Pending,
}

/// Informed consent record (ບັນທຶກການຍິນຍອມຮູ້ເຫັນ)
///
/// Healthcare Law 2014, Article 33
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InformedConsent {
    /// Patient name (ຊື່ຄົນເຈັບ)
    pub patient_name: String,

    /// Patient age (ອາຍຸຄົນເຈັບ)
    pub patient_age: u8,

    /// Procedure/treatment description (ຄຳອະທິບາຍການປິ່ນປົວ)
    pub procedure_description: String,

    /// Risks explained (ຄວາມສ່ຽງທີ່ອະທິບາຍແລ້ວ)
    pub risks_explained: Vec<String>,

    /// Benefits explained (ຜົນປະໂຫຍດທີ່ອະທິບາຍແລ້ວ)
    pub benefits_explained: Vec<String>,

    /// Alternatives explained (ທາງເລືອກອື່ນທີ່ອະທິບາຍແລ້ວ)
    pub alternatives_explained: Vec<String>,

    /// Consent status (ສະຖານະການຍິນຍອມ)
    pub consent_status: InformedConsentStatus,

    /// Guardian consent (if minor) (ການຍິນຍອມຂອງຜູ້ປົກຄອງ)
    pub guardian_consent: Option<String>,

    /// Healthcare provider name (ຊື່ຜູ້ໃຫ້ບໍລິການສຸຂະພາບ)
    pub healthcare_provider: String,
}

impl InformedConsent {
    /// Check if consent is valid
    pub fn is_consent_valid(&self) -> bool {
        matches!(
            &self.consent_status,
            InformedConsentStatus::ConsentGiven { .. } | InformedConsentStatus::NotRequired { .. }
        )
    }

    /// Check if guardian consent is required
    pub fn requires_guardian_consent(&self) -> bool {
        self.patient_age < INFORMED_CONSENT_MINIMUM_AGE
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthcare_facility_type_descriptions() {
        let facility_type = HealthcareFacilityType::CentralHospital;
        assert_eq!(facility_type.description_lao(), "ໂຮງໝໍສູນກາງ");
        assert_eq!(facility_type.description_en(), "Central Hospital");
    }

    #[test]
    fn test_minimum_bed_capacity() {
        assert_eq!(
            HealthcareFacilityType::DistrictHospital.minimum_bed_capacity(),
            Some(MINIMUM_HOSPITAL_BED_DISTRICT)
        );
        assert_eq!(HealthcareFacilityType::Clinic.minimum_bed_capacity(), None);
    }

    #[test]
    fn test_medical_profession_education_requirements() {
        assert_eq!(MedicalProfessionType::Doctor.minimum_education_years(), 6);
        assert_eq!(MedicalProfessionType::Nurse.minimum_education_years(), 3);
    }

    #[test]
    fn test_drug_category_prescription_requirement() {
        assert!(DrugCategory::PrescriptionOnly.requires_prescription());
        assert!(!DrugCategory::OverTheCounter.requires_prescription());
        assert!(DrugCategory::ControlledSubstance { schedule: 1 }.requires_prescription());
    }

    #[test]
    fn test_patient_right_article_reference() {
        assert_eq!(PatientRightType::InformedConsent.article_reference(), 33);
        assert_eq!(PatientRightType::EmergencyCare.article_reference(), 31);
    }

    #[test]
    fn test_health_insurance_scheme_coverage() {
        assert_eq!(
            HealthInsuranceScheme::HealthEquityFund.typical_coverage_percentage(),
            1.00
        );
        assert!(
            HealthInsuranceScheme::SocialSecurityOrganization.typical_coverage_percentage() > 0.70
        );
    }

    #[test]
    fn test_authority_level_descriptions() {
        let level = AuthorityLevel::National;
        assert_eq!(level.description_lao(), "ລະດັບຊາດ");
        assert_eq!(level.description_en(), "National");
    }

    #[test]
    fn test_informed_consent_guardian_requirement() {
        let consent = InformedConsent {
            patient_name: "Test Patient".to_string(),
            patient_age: 15,
            procedure_description: "Test procedure".to_string(),
            risks_explained: vec![],
            benefits_explained: vec![],
            alternatives_explained: vec![],
            consent_status: InformedConsentStatus::Pending,
            guardian_consent: None,
            healthcare_provider: "Dr. Test".to_string(),
        };

        assert!(consent.requires_guardian_consent());
    }

    #[test]
    fn test_constants() {
        assert_eq!(MINIMUM_HOSPITAL_BED_DISTRICT, 20);
        assert_eq!(MINIMUM_HOSPITAL_BED_PROVINCIAL, 100);
        assert_eq!(MEDICAL_LICENSE_VALIDITY_YEARS, 5);
        assert_eq!(DRUG_REGISTRATION_VALIDITY_YEARS, 5);
        assert_eq!(INFORMED_CONSENT_MINIMUM_AGE, 18);
    }
}
