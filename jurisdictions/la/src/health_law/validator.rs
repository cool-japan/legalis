//! Health Law Validators (ຕົວກວດສອບກົດໝາຍສາທາລະນະສຸກ)
//!
//! Validation functions for Lao health law compliance based on:
//! - **Healthcare Law 2014** (Law No. 58/NA)
//! - **Drug and Medical Products Law**
//! - **Public Health Law**

use super::error::{HealthLawError, Result};
use super::types::*;
use chrono::Utc;

// ============================================================================
// Facility License Validation (ການກວດສອບໃບອະນຸຍາດສະຖານທີ່)
// ============================================================================

/// Validate facility license (ກວດສອບໃບອະນຸຍາດສະຖານທີ່)
///
/// Healthcare Law 2014, Article 14-16: All healthcare facilities must be licensed
/// and accredited according to their facility type.
///
/// # Arguments
/// * `facility` - Healthcare facility to validate
///
/// # Returns
/// * `Ok(())` if facility license is valid
/// * `Err(HealthLawError)` if license is invalid, expired, or missing
///
/// # Example
/// ```
/// use legalis_la::health_law::*;
/// use chrono::{Utc, Duration};
///
/// let facility = HealthcareFacility {
///     name_lao: "ໂຮງໝໍສູນກາງ".to_string(),
///     name_en: "Central Hospital".to_string(),
///     facility_type: HealthcareFacilityType::CentralHospital,
///     license_number: "HC-2024-001".to_string(),
///     province: "Vientiane".to_string(),
///     district: "Chanthabouly".to_string(),
///     village: None,
///     bed_capacity: Some(300),
///     services_offered: vec![HealthcareService::EmergencyServices],
///     accreditation_status: AccreditationStatus::FullyAccredited {
///         accreditation_date: Utc::now(),
///         expiry_date: Utc::now() + Duration::days(365),
///         accreditation_body: "Ministry of Health".to_string(),
///     },
///     license_issue_date: Utc::now(),
///     license_expiry_date: Utc::now() + Duration::days(365),
///     operating_hours: Some("24/7".to_string()),
///     emergency_24h: true,
///     contact_phone: Some("021-123456".to_string()),
/// };
///
/// assert!(validate_facility_license(&facility).is_ok());
/// ```
pub fn validate_facility_license(facility: &HealthcareFacility) -> Result<()> {
    // Check license number is not empty
    if facility.license_number.trim().is_empty() {
        return Err(HealthLawError::FacilityUnlicensed {
            facility_name: facility.name_en.clone(),
        });
    }

    // Check if license is expired
    let now = Utc::now();
    if now >= facility.license_expiry_date {
        return Err(HealthLawError::FacilityLicenseExpired {
            facility_name: facility.name_en.clone(),
            expiry_date: facility.license_expiry_date.format("%Y-%m-%d").to_string(),
        });
    }

    // Check bed capacity for hospitals
    if let Some(required_beds) = facility.facility_type.minimum_bed_capacity() {
        let actual_beds = facility.bed_capacity.unwrap_or(0);
        if actual_beds < required_beds {
            return Err(HealthLawError::InadequateBedCapacity {
                actual: actual_beds,
                required: required_beds,
                facility_type: facility.facility_type.description_en().to_string(),
            });
        }
    }

    // Check accreditation status
    match &facility.accreditation_status {
        AccreditationStatus::NotAccredited => {
            return Err(HealthLawError::FacilityNotAccredited {
                facility_name: facility.name_en.clone(),
            });
        }
        AccreditationStatus::Suspended { reason, .. } => {
            return Err(HealthLawError::FacilityAccreditationSuspended {
                facility_name: facility.name_en.clone(),
                reason: reason.clone(),
            });
        }
        AccreditationStatus::Revoked { reason, .. } => {
            return Err(HealthLawError::FacilityAccreditationSuspended {
                facility_name: facility.name_en.clone(),
                reason: reason.clone(),
            });
        }
        AccreditationStatus::FullyAccredited { expiry_date, .. } => {
            if now >= *expiry_date {
                return Err(HealthLawError::FacilityNotAccredited {
                    facility_name: facility.name_en.clone(),
                });
            }
        }
        _ => {}
    }

    Ok(())
}

/// Validate facility has required services for its type
///
/// # Arguments
/// * `facility` - Healthcare facility to validate
///
/// # Returns
/// * `Ok(())` if facility has all required services
/// * `Err(HealthLawError)` if required services are missing
pub fn validate_facility_services(facility: &HealthcareFacility) -> Result<()> {
    let required_services = match facility.facility_type {
        HealthcareFacilityType::CentralHospital | HealthcareFacilityType::ProvincialHospital => {
            vec![
                HealthcareService::EmergencyServices,
                HealthcareService::InpatientCare,
                HealthcareService::OutpatientCare,
                HealthcareService::Laboratory,
            ]
        }
        HealthcareFacilityType::DistrictHospital => {
            vec![
                HealthcareService::EmergencyServices,
                HealthcareService::OutpatientCare,
            ]
        }
        HealthcareFacilityType::Pharmacy => vec![HealthcareService::Pharmacy],
        HealthcareFacilityType::Laboratory => vec![HealthcareService::Laboratory],
        _ => vec![],
    };

    for service in required_services {
        if !facility.services_offered.contains(&service) {
            return Err(HealthLawError::MissingRequiredService {
                service: service.description_en().to_string(),
                facility_type: facility.facility_type.description_en().to_string(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// Medical License Validation (ການກວດສອບໃບອະນຸຍາດແພດ)
// ============================================================================

/// Validate medical professional license (ກວດສອບໃບອະນຸຍາດບຸກຄະລາກອນການແພດ)
///
/// Healthcare Law 2014, Article 20-27: All medical professionals must hold
/// valid licenses to practice.
///
/// # Arguments
/// * `professional` - Medical professional to validate
///
/// # Returns
/// * `Ok(())` if license is valid
/// * `Err(HealthLawError)` if license is invalid, expired, suspended, or revoked
///
/// # Example
/// ```
/// use legalis_la::health_law::*;
/// use chrono::{Utc, Duration};
///
/// let doctor = MedicalProfessional {
///     name: "Dr. Somchai".to_string(),
///     name_lao: Some("ທ່ານໝໍ ສົມໃຈ".to_string()),
///     profession_type: MedicalProfessionType::Doctor,
///     license_number: "MD-2024-001".to_string(),
///     license_issue_date: Utc::now(),
///     license_expiry_date: Utc::now() + Duration::days(365 * 5),
///     specialization: Some("Internal Medicine".to_string()),
///     practicing_facility: "Central Hospital".to_string(),
///     license_status: LicenseStatus::Active,
///     education_qualification: Some("MD from University of Health Sciences".to_string()),
///     years_of_experience: Some(10),
/// };
///
/// assert!(validate_medical_license(&doctor).is_ok());
/// ```
pub fn validate_medical_license(professional: &MedicalProfessional) -> Result<()> {
    // Check license number is not empty
    if professional.license_number.trim().is_empty() {
        return Err(HealthLawError::UnlicensedPractitioner {
            professional_name: professional.name.clone(),
        });
    }

    // Check license status
    match &professional.license_status {
        LicenseStatus::Active => {
            // Check if license is expired by date
            let now = Utc::now();
            if now >= professional.license_expiry_date {
                return Err(HealthLawError::MedicalLicenseExpired {
                    professional_name: professional.name.clone(),
                    expiry_date: professional
                        .license_expiry_date
                        .format("%Y-%m-%d")
                        .to_string(),
                });
            }
        }
        LicenseStatus::Expired { expired_on } => {
            return Err(HealthLawError::MedicalLicenseExpired {
                professional_name: professional.name.clone(),
                expiry_date: expired_on.format("%Y-%m-%d").to_string(),
            });
        }
        LicenseStatus::Suspended { reason, .. } => {
            return Err(HealthLawError::MedicalLicenseSuspended {
                professional_name: professional.name.clone(),
                reason: reason.clone(),
            });
        }
        LicenseStatus::Revoked { reason, .. } => {
            return Err(HealthLawError::MedicalLicenseRevoked {
                professional_name: professional.name.clone(),
                reason: reason.clone(),
            });
        }
        LicenseStatus::PendingRenewal { .. } => {
            // Pending renewal is acceptable if the license hasn't expired yet
            let now = Utc::now();
            if now >= professional.license_expiry_date {
                return Err(HealthLawError::MedicalLicenseExpired {
                    professional_name: professional.name.clone(),
                    expiry_date: professional
                        .license_expiry_date
                        .format("%Y-%m-%d")
                        .to_string(),
                });
            }
        }
    }

    Ok(())
}

/// Validate medical professional's scope of practice
///
/// # Arguments
/// * `professional` - Medical professional
/// * `procedure` - Procedure being performed
///
/// # Returns
/// * `Ok(())` if procedure is within scope
/// * `Err(HealthLawError)` if procedure exceeds scope
pub fn validate_practice_scope(professional: &MedicalProfessional, procedure: &str) -> Result<()> {
    // Define procedures that require specific professions
    let surgical_procedures = ["surgery", "operation", "surgical"];
    let dental_procedures = ["dental", "tooth", "oral surgery"];
    let prescription_procedures = ["prescribe", "prescription"];

    let procedure_lower = procedure.to_lowercase();

    // Check surgical procedures
    if surgical_procedures
        .iter()
        .any(|p| procedure_lower.contains(p))
        && !matches!(professional.profession_type, MedicalProfessionType::Doctor)
    {
        return Err(HealthLawError::PracticeScopeExceeded {
            professional_name: professional.name.clone(),
            profession_type: professional.profession_type.description_en().to_string(),
            procedure: procedure.to_string(),
        });
    }

    // Check dental procedures
    if dental_procedures
        .iter()
        .any(|p| procedure_lower.contains(p))
        && !matches!(
            professional.profession_type,
            MedicalProfessionType::Dentist | MedicalProfessionType::Doctor
        )
    {
        return Err(HealthLawError::PracticeScopeExceeded {
            professional_name: professional.name.clone(),
            profession_type: professional.profession_type.description_en().to_string(),
            procedure: procedure.to_string(),
        });
    }

    // Check prescription authority
    if prescription_procedures
        .iter()
        .any(|p| procedure_lower.contains(p))
        && !matches!(
            professional.profession_type,
            MedicalProfessionType::Doctor
                | MedicalProfessionType::Dentist
                | MedicalProfessionType::Pharmacist
        )
    {
        return Err(HealthLawError::PracticeScopeExceeded {
            professional_name: professional.name.clone(),
            profession_type: professional.profession_type.description_en().to_string(),
            procedure: procedure.to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Drug Registration Validation (ການກວດສອບການຂຶ້ນທະບຽນຢາ)
// ============================================================================

/// Validate drug registration status (ກວດສອບສະຖານະການຂຶ້ນທະບຽນຢາ)
///
/// Drug and Medical Products Law, Article 15-20: All drugs must be registered
/// before distribution in Lao PDR.
///
/// # Arguments
/// * `drug` - Drug registration to validate
///
/// # Returns
/// * `Ok(())` if drug registration is valid
/// * `Err(HealthLawError)` if registration is invalid, expired, or missing
///
/// # Example
/// ```
/// use legalis_la::health_law::*;
/// use chrono::{Utc, Duration};
///
/// let drug = DrugRegistration {
///     drug_name_generic: "Paracetamol".to_string(),
///     drug_name_brand: "Tylenol".to_string(),
///     registration_number: "DRG-2024-001".to_string(),
///     manufacturer: "Pharma Co.".to_string(),
///     country_of_origin: "Thailand".to_string(),
///     drug_category: DrugCategory::OverTheCounter,
///     registration_date: Utc::now(),
///     expiry_date: Utc::now() + Duration::days(365 * 5),
///     status: RegistrationStatus::Registered,
///     therapeutic_class: Some("Analgesic".to_string()),
///     dosage_form: Some("Tablet".to_string()),
///     strength: Some("500mg".to_string()),
/// };
///
/// assert!(validate_drug_registration(&drug).is_ok());
/// ```
pub fn validate_drug_registration(drug: &DrugRegistration) -> Result<()> {
    // Check registration status
    match &drug.status {
        RegistrationStatus::Registered => {
            // Check if registration is expired by date
            let now = Utc::now();
            if now >= drug.expiry_date {
                return Err(HealthLawError::DrugRegistrationExpired {
                    drug_name: drug.drug_name_brand.clone(),
                    expiry_date: drug.expiry_date.format("%Y-%m-%d").to_string(),
                });
            }
        }
        RegistrationStatus::Pending => {
            return Err(HealthLawError::UnregisteredDrug {
                drug_name: drug.drug_name_brand.clone(),
            });
        }
        RegistrationStatus::Expired { expired_on } => {
            return Err(HealthLawError::DrugRegistrationExpired {
                drug_name: drug.drug_name_brand.clone(),
                expiry_date: expired_on.format("%Y-%m-%d").to_string(),
            });
        }
        RegistrationStatus::Suspended { reason, .. } => {
            return Err(HealthLawError::DrugRegistrationSuspended {
                drug_name: drug.drug_name_brand.clone(),
                reason: reason.clone(),
            });
        }
        RegistrationStatus::Revoked { reason, .. } => {
            return Err(HealthLawError::DrugRegistrationSuspended {
                drug_name: drug.drug_name_brand.clone(),
                reason: reason.clone(),
            });
        }
        RegistrationStatus::NotRegistered => {
            return Err(HealthLawError::UnregisteredDrug {
                drug_name: drug.drug_name_brand.clone(),
            });
        }
    }

    // Validate controlled substance schedule if applicable
    if let DrugCategory::ControlledSubstance { schedule } = drug.drug_category
        && schedule > CONTROLLED_SUBSTANCE_SCHEDULES
    {
        return Err(HealthLawError::ControlledSubstanceViolation {
            drug_name: drug.drug_name_brand.clone(),
            schedule,
            violation: format!(
                "Invalid schedule number {} (maximum is {})",
                schedule, CONTROLLED_SUBSTANCE_SCHEDULES
            ),
        });
    }

    Ok(())
}

/// Validate prescription requirements for a drug
///
/// # Arguments
/// * `drug` - Drug being prescribed
/// * `prescriber` - Medical professional prescribing
///
/// # Returns
/// * `Ok(())` if prescription is valid
/// * `Err(HealthLawError)` if prescription requirements are not met
pub fn validate_prescription_requirements(
    drug: &DrugRegistration,
    prescriber: &MedicalProfessional,
) -> Result<()> {
    // First validate the drug registration
    validate_drug_registration(drug)?;

    // First validate the prescriber's license
    validate_medical_license(prescriber)?;

    // Check if drug requires prescription
    if drug.drug_category.requires_prescription() {
        // Only doctors, dentists, and pharmacists can prescribe
        if !matches!(
            prescriber.profession_type,
            MedicalProfessionType::Doctor
                | MedicalProfessionType::Dentist
                | MedicalProfessionType::Pharmacist
        ) {
            return Err(HealthLawError::InvalidPrescription {
                drug_name: drug.drug_name_brand.clone(),
                reason: format!(
                    "{} cannot prescribe medications",
                    prescriber.profession_type.description_en()
                ),
            });
        }

        // Controlled substances have additional restrictions
        if let DrugCategory::ControlledSubstance { schedule } = drug.drug_category {
            // Only doctors can prescribe Schedule 1 and 2
            if schedule <= 2 && !matches!(prescriber.profession_type, MedicalProfessionType::Doctor)
            {
                return Err(HealthLawError::ControlledSubstanceViolation {
                    drug_name: drug.drug_name_brand.clone(),
                    schedule,
                    violation: format!(
                        "Schedule {} substances require doctor prescription",
                        schedule
                    ),
                });
            }
        }
    }

    Ok(())
}

// ============================================================================
// Informed Consent Validation (ການກວດສອບການຍິນຍອມຮູ້ເຫັນ)
// ============================================================================

/// Validate informed consent (ກວດສອບການຍິນຍອມຮູ້ເຫັນ)
///
/// Healthcare Law 2014, Article 33: Patients have the right to informed consent
/// before any medical procedure.
///
/// # Arguments
/// * `consent` - Informed consent record to validate
///
/// # Returns
/// * `Ok(())` if informed consent is valid
/// * `Err(HealthLawError)` if consent is missing or invalid
///
/// # Example
/// ```
/// use legalis_la::health_law::*;
/// use chrono::Utc;
///
/// let consent = InformedConsent {
///     patient_name: "John Doe".to_string(),
///     patient_age: 25,
///     procedure_description: "Blood test".to_string(),
///     risks_explained: vec!["Minor bruising".to_string()],
///     benefits_explained: vec!["Accurate diagnosis".to_string()],
///     alternatives_explained: vec!["Urine test".to_string()],
///     consent_status: InformedConsentStatus::ConsentGiven {
///         consent_date: Utc::now(),
///         witness_name: Some("Nurse".to_string()),
///     },
///     guardian_consent: None,
///     healthcare_provider: "Dr. Test".to_string(),
/// };
///
/// assert!(validate_informed_consent(&consent).is_ok());
/// ```
pub fn validate_informed_consent(consent: &InformedConsent) -> Result<()> {
    // Check consent status
    match &consent.consent_status {
        InformedConsentStatus::ConsentGiven { .. } => {
            // Consent is given, continue validation
        }
        InformedConsentStatus::NotRequired { .. } => {
            // Consent not required (e.g., emergency), valid
            return Ok(());
        }
        InformedConsentStatus::ConsentRefused { .. } => {
            return Err(HealthLawError::InformedConsentViolation {
                patient_name: consent.patient_name.clone(),
                violation_type: "Consent was refused by patient".to_string(),
            });
        }
        InformedConsentStatus::ConsentWithdrawn { .. } => {
            return Err(HealthLawError::InformedConsentViolation {
                patient_name: consent.patient_name.clone(),
                violation_type: "Consent was withdrawn by patient".to_string(),
            });
        }
        InformedConsentStatus::Pending => {
            return Err(HealthLawError::MissingInformedConsent {
                procedure: consent.procedure_description.clone(),
            });
        }
    }

    // Check if minor requires guardian consent
    if consent.patient_age < INFORMED_CONSENT_MINIMUM_AGE && consent.guardian_consent.is_none() {
        return Err(HealthLawError::GuardianConsentRequired {
            patient_name: consent.patient_name.clone(),
            patient_age: consent.patient_age,
        });
    }

    // Verify risks were explained
    if consent.risks_explained.is_empty() {
        return Err(HealthLawError::InformedConsentViolation {
            patient_name: consent.patient_name.clone(),
            violation_type: "Risks were not explained to patient".to_string(),
        });
    }

    // Verify benefits were explained
    if consent.benefits_explained.is_empty() {
        return Err(HealthLawError::InformedConsentViolation {
            patient_name: consent.patient_name.clone(),
            violation_type: "Benefits were not explained to patient".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Patient Privacy Validation (ການກວດສອບຄວາມເປັນສ່ວນຕົວຂອງຄົນເຈັບ)
// ============================================================================

/// Validate patient privacy compliance (ກວດສອບການປະຕິບັດຕາມຄວາມເປັນສ່ວນຕົວ)
///
/// Healthcare Law 2014, Article 34-35: Patient information must be kept confidential.
///
/// # Arguments
/// * `patient_name` - Name of the patient
/// * `accessor_name` - Name of the person accessing records
/// * `accessor_role` - Role of the accessor
/// * `has_authorization` - Whether accessor has authorization
///
/// # Returns
/// * `Ok(())` if access is authorized
/// * `Err(HealthLawError)` if access is unauthorized
pub fn validate_patient_privacy(
    patient_name: &str,
    accessor_name: &str,
    accessor_role: &str,
    has_authorization: bool,
) -> Result<()> {
    // Check if accessor has authorization
    if !has_authorization {
        // Check if accessor role is allowed by default
        let allowed_roles = [
            "treating physician",
            "attending nurse",
            "authorized healthcare provider",
            "patient",
            "legal guardian",
        ];

        if !allowed_roles
            .iter()
            .any(|r| accessor_role.to_lowercase().contains(r))
        {
            return Err(HealthLawError::UnauthorizedRecordAccess {
                patient_name: patient_name.to_string(),
                accessor: accessor_name.to_string(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// Public Health Measure Validation (ການກວດສອບມາດຕະການສາທາລະນະສຸກ)
// ============================================================================

/// Validate public health measure (ກວດສອບມາດຕະການສາທາລະນະສຸກ)
///
/// Public Health Law: Public health measures must be issued by appropriate authority.
///
/// # Arguments
/// * `measure` - Public health measure to validate
///
/// # Returns
/// * `Ok(())` if measure is valid and authorized
/// * `Err(HealthLawError)` if measure is unauthorized
///
/// # Example
/// ```
/// use legalis_la::health_law::*;
/// use chrono::Utc;
///
/// let measure = PublicHealthMeasure {
///     measure_type: PublicHealthMeasureType::MaskMandate,
///     authority_level: AuthorityLevel::National,
///     legal_basis: "COVID-19 Prevention Decree".to_string(),
///     enforcement_date: Utc::now(),
///     end_date: None,
///     description_lao: None,
///     description_en: None,
///     penalty_for_violation: Some("Fine up to 1,000,000 LAK".to_string()),
/// };
///
/// assert!(validate_public_health_measure(&measure).is_ok());
/// ```
pub fn validate_public_health_measure(measure: &PublicHealthMeasure) -> Result<()> {
    // Validate legal basis exists
    if measure.legal_basis.trim().is_empty() {
        return Err(HealthLawError::UnauthorizedPublicHealthMeasure {
            authority_level: measure.authority_level.description_en().to_string(),
            measure_type: measure.measure_type.description_en().to_string(),
        });
    }

    // Validate authority level for certain measures
    match &measure.measure_type {
        PublicHealthMeasureType::TravelRestriction | PublicHealthMeasureType::BorderControl => {
            // These require national authority
            if !matches!(measure.authority_level, AuthorityLevel::National) {
                return Err(HealthLawError::UnauthorizedPublicHealthMeasure {
                    authority_level: measure.authority_level.description_en().to_string(),
                    measure_type: measure.measure_type.description_en().to_string(),
                });
            }
        }
        PublicHealthMeasureType::Quarantine { duration_days } => {
            // Quarantine exceeding maximum days requires national authority
            if *duration_days > MAXIMUM_QUARANTINE_DAYS
                && !matches!(measure.authority_level, AuthorityLevel::National)
            {
                return Err(HealthLawError::UnauthorizedPublicHealthMeasure {
                    authority_level: measure.authority_level.description_en().to_string(),
                    measure_type: format!(
                        "Quarantine ({} days exceeds {} day limit)",
                        duration_days, MAXIMUM_QUARANTINE_DAYS
                    ),
                });
            }
        }
        _ => {}
    }

    Ok(())
}

/// Validate quarantine compliance
///
/// # Arguments
/// * `person_name` - Name of person under quarantine
/// * `quarantine_start` - Start date of quarantine
/// * `quarantine_end` - Expected end date of quarantine
/// * `left_quarantine` - Whether person left quarantine early
/// * `violation_type` - Type of violation if any
///
/// # Returns
/// * `Ok(())` if compliant
/// * `Err(HealthLawError)` if violation occurred
pub fn validate_quarantine_compliance(
    person_name: &str,
    left_quarantine_early: bool,
    violation_description: Option<&str>,
) -> Result<()> {
    if left_quarantine_early {
        return Err(HealthLawError::QuarantineViolation {
            violator_name: person_name.to_string(),
            violation_type: violation_description
                .unwrap_or("Left quarantine before completion")
                .to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Health Insurance Validation (ການກວດສອບປະກັນສຸຂະພາບ)
// ============================================================================

/// Validate health insurance coverage (ກວດສອບການຄຸ້ມຄອງປະກັນສຸຂະພາບ)
///
/// # Arguments
/// * `insurance` - Health insurance record to validate
///
/// # Returns
/// * `Ok(())` if coverage is adequate
/// * `Err(HealthLawError)` if coverage is insufficient or expired
///
/// # Example
/// ```
/// use legalis_la::health_law::*;
/// use chrono::Utc;
///
/// let insurance = HealthInsurance {
///     scheme_type: HealthInsuranceScheme::SocialSecurityOrganization,
///     coverage_percentage: 0.80,
///     beneficiary_category: BeneficiaryCategory::FormalSectorEmployee,
///     member_id: Some("SSO-2024-001".to_string()),
///     enrollment_date: Some(Utc::now()),
///     expiry_date: None,
///     annual_premium_lak: Some(500_000),
///     employer_contribution: Some(300_000),
///     employee_contribution: Some(200_000),
/// };
///
/// assert!(validate_health_insurance_coverage(&insurance).is_ok());
/// ```
pub fn validate_health_insurance_coverage(insurance: &HealthInsurance) -> Result<()> {
    // Check if insurance is active
    if !insurance.is_active() {
        let expiry = insurance
            .expiry_date
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        return Err(HealthLawError::InsuranceExpired {
            member_name: insurance
                .member_id
                .clone()
                .unwrap_or_else(|| "Unknown".to_string()),
            expiry_date: expiry,
        });
    }

    // Check coverage percentage
    if insurance.coverage_percentage < HEALTH_INSURANCE_COVERAGE_MINIMUM {
        return Err(HealthLawError::InsufficientCoverage {
            actual_percentage: insurance.coverage_percentage * 100.0,
            required_percentage: HEALTH_INSURANCE_COVERAGE_MINIMUM * 100.0,
        });
    }

    Ok(())
}

/// Validate beneficiary eligibility for a specific scheme
///
/// # Arguments
/// * `scheme` - Health insurance scheme
/// * `beneficiary` - Beneficiary category
///
/// # Returns
/// * `Ok(())` if beneficiary is eligible
/// * `Err(HealthLawError)` if beneficiary is not eligible
pub fn validate_scheme_eligibility(
    scheme: HealthInsuranceScheme,
    beneficiary: BeneficiaryCategory,
) -> Result<()> {
    let is_eligible = match scheme {
        HealthInsuranceScheme::SocialSecurityOrganization => {
            matches!(beneficiary, BeneficiaryCategory::FormalSectorEmployee)
        }
        HealthInsuranceScheme::CommunityBasedHealthInsurance => {
            matches!(beneficiary, BeneficiaryCategory::InformalSectorWorker)
        }
        HealthInsuranceScheme::HealthEquityFund => {
            matches!(beneficiary, BeneficiaryCategory::PoorAndVulnerable)
        }
        HealthInsuranceScheme::StateEmployeeScheme => {
            matches!(beneficiary, BeneficiaryCategory::GovernmentEmployee)
        }
        HealthInsuranceScheme::MilitaryHealthScheme => {
            matches!(beneficiary, BeneficiaryCategory::MilitaryPersonnel)
        }
        HealthInsuranceScheme::PrivateHealthInsurance => true, // Open to all
    };

    if !is_eligible {
        return Err(HealthLawError::IneligibleForScheme {
            scheme_name: scheme.description_en().to_string(),
            reason: format!(
                "{} is not eligible for {}",
                beneficiary.description_en(),
                scheme.description_en()
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Emergency Care Validation (ການກວດສອບການປິ່ນປົວສຸກເສີນ)
// ============================================================================

/// Validate emergency care obligation (ກວດສອບພັນທະການປິ່ນປົວສຸກເສີນ)
///
/// Healthcare Law 2014, Article 31: Healthcare facilities must provide emergency
/// care regardless of patient's ability to pay.
///
/// # Arguments
/// * `facility` - Healthcare facility
/// * `patient_name` - Name of patient requiring emergency care
/// * `care_provided` - Whether emergency care was provided
/// * `reason_denied` - Reason if care was denied
///
/// # Returns
/// * `Ok(())` if emergency care obligation is met
/// * `Err(HealthLawError)` if emergency care was improperly denied
pub fn validate_emergency_care_obligation(
    facility: &HealthcareFacility,
    patient_name: &str,
    care_provided: bool,
    reason_denied: Option<&str>,
) -> Result<()> {
    // Check if facility offers emergency services
    if !facility
        .services_offered
        .contains(&HealthcareService::EmergencyServices)
    {
        return Err(HealthLawError::NoEmergencyServicesAvailable {
            facility_name: facility.name_en.clone(),
        });
    }

    // Check if care was provided
    if !care_provided {
        let reason = reason_denied.unwrap_or("No reason provided");
        return Err(HealthLawError::EmergencyCareDenied {
            patient_name: patient_name.to_string(),
            reason: reason.to_string(),
        });
    }

    Ok(())
}

/// Validate emergency response time
///
/// # Arguments
/// * `response_time_minutes` - Actual response time in minutes
///
/// # Returns
/// * `Ok(())` if response time is acceptable
/// * `Err(HealthLawError)` if response time exceeds requirement
pub fn validate_emergency_response_time(response_time_minutes: u32) -> Result<()> {
    if response_time_minutes > EMERGENCY_RESPONSE_TIME_MINUTES {
        return Err(HealthLawError::EmergencyResponseDelay {
            actual_minutes: response_time_minutes,
            required_minutes: EMERGENCY_RESPONSE_TIME_MINUTES,
        });
    }

    Ok(())
}

// ============================================================================
// Comprehensive Validation (ການກວດສອບແບບຄົບຖ້ວນ)
// ============================================================================

/// Perform comprehensive validation of a healthcare facility
///
/// # Arguments
/// * `facility` - Healthcare facility to validate
///
/// # Returns
/// * `Ok(Vec<String>)` - List of warnings (non-critical issues)
/// * `Err(HealthLawError)` - Critical violation found
pub fn validate_facility_comprehensive(facility: &HealthcareFacility) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Critical validations
    validate_facility_license(facility)?;
    validate_facility_services(facility)?;

    // Non-critical checks
    if facility.contact_phone.is_none() {
        warnings.push("Facility has no contact phone listed".to_string());
    }

    if facility.operating_hours.is_none() {
        warnings.push("Facility has no operating hours listed".to_string());
    }

    if !facility.emergency_24h
        && facility
            .services_offered
            .contains(&HealthcareService::EmergencyServices)
    {
        warnings.push("Facility offers emergency services but not 24/7".to_string());
    }

    // Check if license expires soon (within 90 days)
    let days_until_expiry = (facility.license_expiry_date - Utc::now()).num_days();
    if days_until_expiry > 0 && days_until_expiry <= 90 {
        warnings.push(format!(
            "License expires in {} days - renewal recommended",
            days_until_expiry
        ));
    }

    Ok(warnings)
}

/// Perform comprehensive validation of a medical professional
///
/// # Arguments
/// * `professional` - Medical professional to validate
///
/// # Returns
/// * `Ok(Vec<String>)` - List of warnings (non-critical issues)
/// * `Err(HealthLawError)` - Critical violation found
pub fn validate_professional_comprehensive(
    professional: &MedicalProfessional,
) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Critical validations
    validate_medical_license(professional)?;

    // Non-critical checks
    if professional.specialization.is_none() {
        warnings.push("No specialization recorded".to_string());
    }

    if professional.education_qualification.is_none() {
        warnings.push("No education qualification recorded".to_string());
    }

    // Check if license expires soon
    if professional.needs_renewal_soon() {
        warnings.push(format!(
            "License expires in {} days - renewal recommended",
            professional.days_until_expiry()
        ));
    }

    Ok(warnings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn create_valid_facility() -> HealthcareFacility {
        HealthcareFacility {
            name_lao: "ໂຮງໝໍທົດສອບ".to_string(),
            name_en: "Test Hospital".to_string(),
            facility_type: HealthcareFacilityType::DistrictHospital,
            license_number: "HC-2024-001".to_string(),
            province: "Vientiane".to_string(),
            district: "Chanthabouly".to_string(),
            village: None,
            bed_capacity: Some(25),
            services_offered: vec![
                HealthcareService::EmergencyServices,
                HealthcareService::OutpatientCare,
            ],
            accreditation_status: AccreditationStatus::FullyAccredited {
                accreditation_date: Utc::now(),
                expiry_date: Utc::now() + Duration::days(365),
                accreditation_body: "Ministry of Health".to_string(),
            },
            license_issue_date: Utc::now(),
            license_expiry_date: Utc::now() + Duration::days(365),
            operating_hours: Some("08:00-17:00".to_string()),
            emergency_24h: true,
            contact_phone: Some("021-123456".to_string()),
        }
    }

    fn create_valid_professional() -> MedicalProfessional {
        MedicalProfessional {
            name: "Dr. Test".to_string(),
            name_lao: Some("ທ່ານໝໍ ທົດສອບ".to_string()),
            profession_type: MedicalProfessionType::Doctor,
            license_number: "MD-2024-001".to_string(),
            license_issue_date: Utc::now(),
            license_expiry_date: Utc::now() + Duration::days(365 * 5),
            specialization: Some("Internal Medicine".to_string()),
            practicing_facility: "Test Hospital".to_string(),
            license_status: LicenseStatus::Active,
            education_qualification: Some("MD".to_string()),
            years_of_experience: Some(10),
        }
    }

    #[test]
    fn test_valid_facility_license() {
        let facility = create_valid_facility();
        assert!(validate_facility_license(&facility).is_ok());
    }

    #[test]
    fn test_expired_facility_license() {
        let mut facility = create_valid_facility();
        facility.license_expiry_date = Utc::now() - Duration::days(1);

        let result = validate_facility_license(&facility);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HealthLawError::FacilityLicenseExpired { .. }
        ));
    }

    #[test]
    fn test_inadequate_bed_capacity() {
        let mut facility = create_valid_facility();
        facility.bed_capacity = Some(10); // Below minimum of 20

        let result = validate_facility_license(&facility);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HealthLawError::InadequateBedCapacity { .. }
        ));
    }

    #[test]
    fn test_valid_medical_license() {
        let professional = create_valid_professional();
        assert!(validate_medical_license(&professional).is_ok());
    }

    #[test]
    fn test_expired_medical_license() {
        let mut professional = create_valid_professional();
        professional.license_expiry_date = Utc::now() - Duration::days(1);

        let result = validate_medical_license(&professional);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HealthLawError::MedicalLicenseExpired { .. }
        ));
    }

    #[test]
    fn test_practice_scope_exceeded() {
        let mut professional = create_valid_professional();
        professional.profession_type = MedicalProfessionType::Nurse;

        let result = validate_practice_scope(&professional, "Surgery");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HealthLawError::PracticeScopeExceeded { .. }
        ));
    }

    #[test]
    fn test_valid_drug_registration() {
        let drug = DrugRegistration {
            drug_name_generic: "Paracetamol".to_string(),
            drug_name_brand: "Tylenol".to_string(),
            registration_number: "DRG-2024-001".to_string(),
            manufacturer: "Pharma Co.".to_string(),
            country_of_origin: "Thailand".to_string(),
            drug_category: DrugCategory::OverTheCounter,
            registration_date: Utc::now(),
            expiry_date: Utc::now() + Duration::days(365 * 5),
            status: RegistrationStatus::Registered,
            therapeutic_class: None,
            dosage_form: None,
            strength: None,
        };

        assert!(validate_drug_registration(&drug).is_ok());
    }

    #[test]
    fn test_unregistered_drug() {
        let drug = DrugRegistration {
            drug_name_generic: "Unknown".to_string(),
            drug_name_brand: "Unknown".to_string(),
            registration_number: "".to_string(),
            manufacturer: "".to_string(),
            country_of_origin: "".to_string(),
            drug_category: DrugCategory::PrescriptionOnly,
            registration_date: Utc::now(),
            expiry_date: Utc::now(),
            status: RegistrationStatus::NotRegistered,
            therapeutic_class: None,
            dosage_form: None,
            strength: None,
        };

        let result = validate_drug_registration(&drug);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HealthLawError::UnregisteredDrug { .. }
        ));
    }

    #[test]
    fn test_informed_consent_valid() {
        let consent = InformedConsent {
            patient_name: "Test Patient".to_string(),
            patient_age: 25,
            procedure_description: "Blood test".to_string(),
            risks_explained: vec!["Minor bruising".to_string()],
            benefits_explained: vec!["Accurate diagnosis".to_string()],
            alternatives_explained: vec![],
            consent_status: InformedConsentStatus::ConsentGiven {
                consent_date: Utc::now(),
                witness_name: None,
            },
            guardian_consent: None,
            healthcare_provider: "Dr. Test".to_string(),
        };

        assert!(validate_informed_consent(&consent).is_ok());
    }

    #[test]
    fn test_guardian_consent_required() {
        let consent = InformedConsent {
            patient_name: "Minor Patient".to_string(),
            patient_age: 15,
            procedure_description: "Surgery".to_string(),
            risks_explained: vec!["Risk 1".to_string()],
            benefits_explained: vec!["Benefit 1".to_string()],
            alternatives_explained: vec![],
            consent_status: InformedConsentStatus::ConsentGiven {
                consent_date: Utc::now(),
                witness_name: None,
            },
            guardian_consent: None,
            healthcare_provider: "Dr. Test".to_string(),
        };

        let result = validate_informed_consent(&consent);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HealthLawError::GuardianConsentRequired { .. }
        ));
    }

    #[test]
    fn test_valid_health_insurance() {
        let insurance = HealthInsurance {
            scheme_type: HealthInsuranceScheme::SocialSecurityOrganization,
            coverage_percentage: 0.80,
            beneficiary_category: BeneficiaryCategory::FormalSectorEmployee,
            member_id: Some("SSO-001".to_string()),
            enrollment_date: Some(Utc::now()),
            expiry_date: None,
            annual_premium_lak: Some(500_000),
            employer_contribution: Some(300_000),
            employee_contribution: Some(200_000),
        };

        assert!(validate_health_insurance_coverage(&insurance).is_ok());
    }

    #[test]
    fn test_insufficient_coverage() {
        let insurance = HealthInsurance {
            scheme_type: HealthInsuranceScheme::PrivateHealthInsurance,
            coverage_percentage: 0.50, // Below minimum
            beneficiary_category: BeneficiaryCategory::FormalSectorEmployee,
            member_id: None,
            enrollment_date: None,
            expiry_date: None,
            annual_premium_lak: None,
            employer_contribution: None,
            employee_contribution: None,
        };

        let result = validate_health_insurance_coverage(&insurance);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HealthLawError::InsufficientCoverage { .. }
        ));
    }

    #[test]
    fn test_emergency_response_time() {
        assert!(validate_emergency_response_time(25).is_ok());
        assert!(validate_emergency_response_time(35).is_err());
    }

    #[test]
    fn test_comprehensive_facility_validation() {
        let facility = create_valid_facility();
        let result = validate_facility_comprehensive(&facility);
        assert!(result.is_ok());
    }
}
