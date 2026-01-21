//! Health Law Module (ກົດໝາຍສາທາລະນະສຸກ)
//!
//! This module provides comprehensive support for Lao health law based on:
//! - **Healthcare Law 2014** (Law No. 58/NA, effective July 2014)
//! - **Drug and Medical Products Law** - Pharmaceutical regulations
//! - **Public Health Law** - Epidemic and disease control
//! - **Traditional Medicine Law** - Herbal and traditional practices
//!
//! # Legal Framework
//!
//! The Healthcare Law 2014 is the primary legislation governing healthcare
//! in the Lao People's Democratic Republic. It establishes standards for:
//!
//! ## Key Provisions
//!
//! ### Healthcare Facilities (ສະຖານທີ່ສາທາລະນະສຸກ)
//! - **Article 12-16**: Licensing and accreditation requirements
//! - **Article 14**: All facilities must be licensed before operation
//! - **Article 15**: License validity and renewal procedures
//! - **Article 16**: Accreditation standards and requirements
//!
//! ### Medical Professionals (ບຸກຄະລາກອນການແພດ)
//! - **Article 20-27**: Licensing requirements for healthcare workers
//! - **Article 23**: License validity (5 years)
//! - **Article 24**: Scope of practice restrictions
//! - **Article 25-27**: License suspension and revocation
//!
//! ### Patient Rights (ສິດຂອງຄົນເຈັບ)
//! - **Article 30**: Non-discrimination in healthcare
//! - **Article 31**: Right to emergency care
//! - **Article 32**: Right to information and second opinion
//! - **Article 33**: Informed consent requirements
//! - **Article 34**: Privacy and confidentiality
//! - **Article 35**: Access to medical records
//!
//! ### Drug Regulation (ການຄວບຄຸມຢາ)
//! - **Drug Law Article 15-20**: Drug registration requirements
//! - **Drug Law Article 22**: Controlled substance regulations
//! - **Drug Law Article 25**: Counterfeit drug prohibition
//!
//! # Healthcare System Structure
//!
//! The Lao PDR healthcare system is organized into four levels:
//!
//! 1. **Central Level** (ລະດັບສູນກາງ)
//!    - Ministry of Health (ກະຊວງສາທາລະນະສຸກ)
//!    - Central Hospitals (ໂຮງໝໍສູນກາງ)
//!    - National reference laboratories
//!
//! 2. **Provincial Level** (ລະດັບແຂວງ)
//!    - Provincial Health Departments
//!    - Provincial Hospitals (ໂຮງໝໍແຂວງ)
//!    - Minimum 100 beds
//!
//! 3. **District Level** (ລະດັບເມືອງ)
//!    - District Health Offices
//!    - District Hospitals (ໂຮງໝໍເມືອງ)
//!    - Minimum 20 beds
//!
//! 4. **Village Level** (ລະດັບບ້ານ)
//!    - Health Centers (ສຸກສາລາ)
//!    - Village Health Volunteers
//!
//! # Health Insurance Schemes
//!
//! Lao PDR has several health insurance schemes:
//!
//! - **Social Security Organization (SSO)**: For formal sector employees
//! - **Community-Based Health Insurance (CBHI)**: For informal sector
//! - **Health Equity Fund (HEF)**: For the poor and vulnerable
//! - **State Employee Scheme**: For government employees
//!
//! # Features
//!
//! - **Bilingual Support**: All types and errors support both Lao (ລາວ) and English
//! - **Type-safe Validation**: Compile-time guarantees for health law compliance
//! - **Comprehensive Coverage**: All major aspects of Healthcare Law 2014
//! - **Patient Protection**: Strong emphasis on patient rights and safety
//!
//! # Examples
//!
//! ## Validating a Healthcare Facility License
//!
//! ```rust
//! use legalis_la::health_law::*;
//! use chrono::{Utc, Duration};
//!
//! let facility = HealthcareFacility {
//!     name_lao: "ໂຮງໝໍສູນກາງ".to_string(),
//!     name_en: "Central Hospital".to_string(),
//!     facility_type: HealthcareFacilityType::CentralHospital,
//!     license_number: "HC-2024-001".to_string(),
//!     province: "Vientiane".to_string(),
//!     district: "Chanthabouly".to_string(),
//!     village: None,
//!     bed_capacity: Some(300),
//!     services_offered: vec![
//!         HealthcareService::EmergencyServices,
//!         HealthcareService::InpatientCare,
//!         HealthcareService::OutpatientCare,
//!         HealthcareService::Laboratory,
//!     ],
//!     accreditation_status: AccreditationStatus::FullyAccredited {
//!         accreditation_date: Utc::now(),
//!         expiry_date: Utc::now() + Duration::days(365),
//!         accreditation_body: "Ministry of Health".to_string(),
//!     },
//!     license_issue_date: Utc::now(),
//!     license_expiry_date: Utc::now() + Duration::days(365),
//!     operating_hours: Some("24/7".to_string()),
//!     emergency_24h: true,
//!     contact_phone: Some("021-123456".to_string()),
//! };
//!
//! match validate_facility_license(&facility) {
//!     Ok(()) => println!("Facility license is valid"),
//!     Err(e) => {
//!         println!("English: {}", e.english_message());
//!         println!("Lao: {}", e.lao_message());
//!     }
//! }
//! ```
//!
//! ## Validating a Medical Professional License
//!
//! ```rust
//! use legalis_la::health_law::*;
//! use chrono::{Utc, Duration};
//!
//! let doctor = MedicalProfessional {
//!     name: "Dr. Somchai Vongphachanh".to_string(),
//!     name_lao: Some("ທ່ານໝໍ ສົມໃຈ ວົງພະຈັນ".to_string()),
//!     profession_type: MedicalProfessionType::Doctor,
//!     license_number: "MD-2024-001".to_string(),
//!     license_issue_date: Utc::now(),
//!     license_expiry_date: Utc::now() + Duration::days(365 * 5),
//!     specialization: Some("Internal Medicine".to_string()),
//!     practicing_facility: "Central Hospital".to_string(),
//!     license_status: LicenseStatus::Active,
//!     education_qualification: Some("MD from University of Health Sciences".to_string()),
//!     years_of_experience: Some(15),
//! };
//!
//! assert!(validate_medical_license(&doctor).is_ok());
//! ```
//!
//! ## Validating Drug Registration
//!
//! ```rust
//! use legalis_la::health_law::*;
//! use chrono::{Utc, Duration};
//!
//! let drug = DrugRegistration {
//!     drug_name_generic: "Paracetamol".to_string(),
//!     drug_name_brand: "Tylenol".to_string(),
//!     registration_number: "DRG-2024-001".to_string(),
//!     manufacturer: "Pharma Co.".to_string(),
//!     country_of_origin: "Thailand".to_string(),
//!     drug_category: DrugCategory::OverTheCounter,
//!     registration_date: Utc::now(),
//!     expiry_date: Utc::now() + Duration::days(365 * 5),
//!     status: RegistrationStatus::Registered,
//!     therapeutic_class: Some("Analgesic".to_string()),
//!     dosage_form: Some("Tablet".to_string()),
//!     strength: Some("500mg".to_string()),
//! };
//!
//! assert!(validate_drug_registration(&drug).is_ok());
//! ```
//!
//! ## Validating Informed Consent
//!
//! ```rust
//! use legalis_la::health_law::*;
//! use chrono::Utc;
//!
//! let consent = InformedConsent {
//!     patient_name: "Bounmy Souliyavong".to_string(),
//!     patient_age: 35,
//!     procedure_description: "Appendectomy".to_string(),
//!     risks_explained: vec![
//!         "Infection".to_string(),
//!         "Bleeding".to_string(),
//!         "Anesthesia complications".to_string(),
//!     ],
//!     benefits_explained: vec![
//!         "Removal of inflamed appendix".to_string(),
//!         "Prevention of rupture".to_string(),
//!     ],
//!     alternatives_explained: vec![
//!         "Conservative management with antibiotics".to_string(),
//!     ],
//!     consent_status: InformedConsentStatus::ConsentGiven {
//!         consent_date: Utc::now(),
//!         witness_name: Some("Nurse Khamla".to_string()),
//!     },
//!     guardian_consent: None,
//!     healthcare_provider: "Dr. Somchai".to_string(),
//! };
//!
//! assert!(validate_informed_consent(&consent).is_ok());
//! ```
//!
//! ## Validating Health Insurance Coverage
//!
//! ```rust
//! use legalis_la::health_law::*;
//! use chrono::Utc;
//!
//! let insurance = HealthInsurance {
//!     scheme_type: HealthInsuranceScheme::SocialSecurityOrganization,
//!     coverage_percentage: 0.80,
//!     beneficiary_category: BeneficiaryCategory::FormalSectorEmployee,
//!     member_id: Some("SSO-2024-123456".to_string()),
//!     enrollment_date: Some(Utc::now()),
//!     expiry_date: None,
//!     annual_premium_lak: Some(600_000),
//!     employer_contribution: Some(360_000),
//!     employee_contribution: Some(240_000),
//! };
//!
//! assert!(validate_health_insurance_coverage(&insurance).is_ok());
//! ```
//!
//! # Bilingual Error Messages
//!
//! All errors include both English and Lao messages:
//!
//! ```rust
//! use legalis_la::health_law::*;
//!
//! let error = HealthLawError::FacilityUnlicensed {
//!     facility_name: "Test Clinic".to_string(),
//! };
//!
//! println!("English: {}", error.english_message());
//! // "Healthcare facility is unlicensed: Test Clinic (Article 14)"
//!
//! println!("Lao: {}", error.lao_message());
//! // "ສະຖານທີ່ສາທາລະນະສຸກບໍ່ມີໃບອະນຸຍາດ: Test Clinic (ມາດຕາ 14)"
//! ```
//!
//! # Compliance Notes
//!
//! When implementing health law compliance in Laos:
//!
//! 1. **Facility Licensing**: All healthcare facilities must be licensed before operation
//! 2. **Professional Licensing**: All medical professionals must hold valid licenses
//! 3. **Drug Registration**: All drugs must be registered before distribution
//! 4. **Informed Consent**: Required for all non-emergency procedures
//! 5. **Patient Privacy**: Medical records must be kept confidential
//! 6. **Emergency Care**: Cannot be denied regardless of payment ability
//! 7. **Accreditation**: Facilities must maintain proper accreditation status
//! 8. **Controlled Substances**: Special handling requirements for scheduled drugs

pub mod error;
pub mod types;
pub mod validator;

// Re-export error types
pub use error::{HealthLawError, Result};

// Re-export constants
pub use types::{
    CONTROLLED_SUBSTANCE_SCHEDULES, DRUG_REGISTRATION_VALIDITY_YEARS,
    EMERGENCY_RESPONSE_TIME_MINUTES, HEALTH_INSURANCE_COVERAGE_MINIMUM,
    INFORMED_CONSENT_MINIMUM_AGE, MAXIMUM_QUARANTINE_DAYS, MEDICAL_LICENSE_VALIDITY_YEARS,
    MINIMUM_HOSPITAL_BED_DISTRICT, MINIMUM_HOSPITAL_BED_PROVINCIAL,
};

// Re-export healthcare facility types
pub use types::{
    AccreditationStatus, HealthcareFacility, HealthcareFacilityType, HealthcareService,
};

// Re-export medical professional types
pub use types::{LicenseStatus, MedicalProfessionType, MedicalProfessional};

// Re-export drug registration types
pub use types::{DrugCategory, DrugRegistration, RegistrationStatus};

// Re-export patient rights types
pub use types::{PatientRightType, PatientRights};

// Re-export public health types
pub use types::{AuthorityLevel, PublicHealthMeasure, PublicHealthMeasureType};

// Re-export health insurance types
pub use types::{BeneficiaryCategory, HealthInsurance, HealthInsuranceScheme};

// Re-export informed consent types
pub use types::{InformedConsent, InformedConsentStatus};

// Re-export validators
pub use validator::{
    validate_drug_registration, validate_emergency_care_obligation,
    validate_emergency_response_time, validate_facility_comprehensive, validate_facility_license,
    validate_facility_services, validate_health_insurance_coverage, validate_informed_consent,
    validate_medical_license, validate_patient_privacy, validate_practice_scope,
    validate_prescription_requirements, validate_professional_comprehensive,
    validate_public_health_measure, validate_quarantine_compliance, validate_scheme_eligibility,
};
