//! Education Law Validators (ຕົວກວດສອບກົດໝາຍການສຶກສາ)
//!
//! Validation functions for Lao education law compliance based on Education Law 2015.
//!
//! # Legal References
//! - Education Law 2015 (Law No. 62/NA) - ກົດໝາຍວ່າດ້ວຍການສຶກສາ ປີ 2015

use super::error::{EducationLawError, Result};
use super::types::*;

// ============================================================================
// Institution License Validation (ການກວດສອບໃບອະນຸຍາດສະຖາບັນ)
// ============================================================================

/// Validate institution license (ກວດສອບໃບອະນຸຍາດສະຖາບັນ)
///
/// Validates compliance with:
/// - Article 42: Institution licensing requirements
/// - Article 47: Capacity and staffing requirements
///
/// # Arguments
/// * `institution` - Educational institution to validate
/// * `current_date` - Current date for license validity check (YYYY-MM-DD format)
///
/// # Returns
/// * `Ok(())` if institution license is valid
/// * `Err(EducationLawError)` if license is invalid
///
/// # Example
/// ```rust,ignore
/// use legalis_la::education_law::*;
///
/// let institution = EducationalInstitution { /* ... */ };
/// match validate_institution_license(&institution, "2024-01-15") {
///     Ok(()) => println!("Institution license is valid"),
///     Err(e) => println!("License error: {}", e.english_message()),
/// }
/// ```
pub fn validate_institution_license(
    institution: &EducationalInstitution,
    current_date: &str,
) -> Result<()> {
    // Check license number exists
    if institution.license_number.trim().is_empty() {
        return Err(EducationLawError::MissingLicenseNumber);
    }

    // Check license expiry date
    if institution.license_expiry_date.as_str() < current_date {
        return Err(EducationLawError::InstitutionLicenseExpired {
            expiry_date: institution.license_expiry_date.clone(),
        });
    }

    // Check capacity
    if institution.current_enrollment > institution.student_capacity {
        return Err(EducationLawError::CapacityExceeded {
            current: institution.current_enrollment,
            capacity: institution.student_capacity,
        });
    }

    // Check teacher qualifications
    if institution.teacher_count > 0 {
        let qualified_percentage = institution.qualified_teacher_percentage();
        if qualified_percentage < 70.0 {
            return Err(EducationLawError::InsufficientQualifiedTeachers {
                qualified: institution.qualified_teacher_count,
                total: institution.teacher_count,
                percentage: qualified_percentage,
            });
        }
    }

    Ok(())
}

/// Validate institution type matches programs offered (Article 43-54)
/// ກວດສອບປະເພດສະຖາບັນກັບຫຼັກສູດທີ່ສອນ (ມາດຕາ 43-54)
pub fn validate_institution_programs(institution: &EducationalInstitution) -> Result<()> {
    let allowed_levels = institution.institution_type.education_levels();

    for program in &institution.programs_offered {
        let program_level = match &program.level {
            EducationLevel::PrePrimary { .. } => "Pre-Primary",
            EducationLevel::Primary { .. } => "Primary",
            EducationLevel::LowerSecondary { .. } => "Lower Secondary",
            EducationLevel::UpperSecondary { .. } => "Upper Secondary",
            EducationLevel::TechnicalVocational { .. } => "Technical Vocational",
            EducationLevel::HigherEducation { .. } => "Higher Education",
            EducationLevel::NonFormal => "Non-Formal",
            EducationLevel::SpecialEducation { .. } => "Special Education",
        };

        if !allowed_levels.contains(&program_level) {
            return Err(EducationLawError::InstitutionTypeMismatch {
                institution_type: format!("{:?}", institution.institution_type),
                program_level: program_level.to_string(),
            });
        }
    }

    Ok(())
}

/// Validate institution has valid accreditation
/// ກວດສອບສະຖາບັນມີການຮັບຮອງຄຸນນະພາບທີ່ຖືກຕ້ອງ
pub fn validate_institution_accreditation(
    institution: &EducationalInstitution,
    current_date: &str,
) -> Result<()> {
    match &institution.accreditation_status {
        AccreditationStatus::FullyAccredited { expiry_date, .. } => {
            if expiry_date < &current_date.to_string() {
                return Err(EducationLawError::AccreditationExpired {
                    program_name: institution.name_en.clone(),
                    expiry_date: expiry_date.clone(),
                });
            }
        }
        AccreditationStatus::ProvisionallyAccredited {
            end_date,
            conditions,
        } => {
            if end_date < &current_date.to_string() {
                return Err(EducationLawError::ProvisionalConditionsNotMet {
                    conditions: conditions.join(", "),
                });
            }
        }
        AccreditationStatus::NotAccredited | AccreditationStatus::Pending { .. } => {
            return Err(EducationLawError::InstitutionNotAccredited {
                institution_name: institution.name_en.clone(),
            });
        }
        AccreditationStatus::Expired { last_valid_date } => {
            return Err(EducationLawError::AccreditationExpired {
                program_name: institution.name_en.clone(),
                expiry_date: last_valid_date.clone(),
            });
        }
        AccreditationStatus::Revoked { reason, .. } => {
            return Err(EducationLawError::AccreditationRevoked {
                program_name: institution.name_en.clone(),
                reason: reason.clone(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// Program Accreditation Validation (ການກວດສອບການຮັບຮອງຫຼັກສູດ)
// ============================================================================

/// Validate program accreditation (ກວດສອບການຮັບຮອງຫຼັກສູດ)
///
/// Validates compliance with:
/// - Article 55: Program accreditation requirements
///
/// # Arguments
/// * `program` - Education program to validate
/// * `current_date` - Current date for accreditation validity check
///
/// # Returns
/// * `Ok(())` if program is properly accredited
/// * `Err(EducationLawError)` if accreditation is invalid
pub fn validate_program_accreditation(
    program: &EducationProgram,
    current_date: &str,
) -> Result<()> {
    match &program.accreditation_status {
        AccreditationStatus::FullyAccredited { expiry_date, .. } => {
            if expiry_date < &current_date.to_string() {
                return Err(EducationLawError::AccreditationExpired {
                    program_name: program.program_name_en.clone(),
                    expiry_date: expiry_date.clone(),
                });
            }
        }
        AccreditationStatus::ProvisionallyAccredited {
            end_date,
            conditions,
        } => {
            if end_date < &current_date.to_string() {
                return Err(EducationLawError::ProvisionalConditionsNotMet {
                    conditions: conditions.join(", "),
                });
            }
        }
        AccreditationStatus::NotAccredited | AccreditationStatus::Pending { .. } => {
            return Err(EducationLawError::ProgramNotAccredited {
                program_name: program.program_name_en.clone(),
            });
        }
        AccreditationStatus::Expired { last_valid_date } => {
            return Err(EducationLawError::AccreditationExpired {
                program_name: program.program_name_en.clone(),
                expiry_date: last_valid_date.clone(),
            });
        }
        AccreditationStatus::Revoked { reason, .. } => {
            return Err(EducationLawError::AccreditationRevoked {
                program_name: program.program_name_en.clone(),
                reason: reason.clone(),
            });
        }
    }

    // Check if program is active
    if !program.is_active {
        return Err(EducationLawError::ProgramNotActive {
            program_name: program.program_name_en.clone(),
        });
    }

    Ok(())
}

/// Validate program duration matches education level standards
/// ກວດສອບໄລຍະຫຼັກສູດກັບມາດຕະຖານລະດັບການສຶກສາ
pub fn validate_program_duration(program: &EducationProgram) -> Result<()> {
    if let Some(expected_duration) = program.level.standard_duration_years() {
        // Allow some flexibility (e.g., 4-year program can be 3.5-4.5 years)
        let min_duration = expected_duration as f32 - 0.5;
        let max_duration = expected_duration as f32 + 0.5;

        if program.duration_years < min_duration || program.duration_years > max_duration {
            return Err(EducationLawError::InvalidProgramDuration {
                actual_years: program.duration_years,
                expected_years: expected_duration,
                level: format!("{:?}", program.level),
            });
        }
    }

    Ok(())
}

/// Validate program credits meet minimum requirements
/// ກວດສອບໜ່ວຍກິດຫຼັກສູດຕາມເກນຂັ້ນຕ່ຳ
pub fn validate_program_credits(program: &EducationProgram) -> Result<()> {
    if let EducationLevel::HigherEducation { degree_type } = &program.level
        && let Some(min_credits) = degree_type.minimum_credits()
        && let Some(credits) = program.credits
        && credits < min_credits
    {
        return Err(EducationLawError::InsufficientCredits {
            actual: credits,
            required: min_credits,
            degree_type: format!("{:?}", degree_type),
        });
    }

    Ok(())
}

// ============================================================================
// Teacher Qualification Validation (ການກວດສອບຄຸນວຸດທິຄູ)
// ============================================================================

/// Validate teacher qualification (ກວດສອບຄຸນວຸດທິຄູ)
///
/// Validates compliance with:
/// - Article 58: Teacher qualification requirements
/// - Article 60: Teacher licensing requirements
///
/// # Arguments
/// * `teacher` - Teacher to validate
///
/// # Returns
/// * `Ok(())` if teacher is qualified
/// * `Err(EducationLawError)` if qualification is insufficient
pub fn validate_teacher_qualification(teacher: &Teacher) -> Result<()> {
    // Check if teacher meets minimum qualification for their level
    if !teacher.meets_minimum_qualification() {
        let required = match &teacher.teaching_level {
            EducationLevel::PrePrimary { .. } => "Teacher Certificate",
            EducationLevel::Primary { .. } => "Diploma",
            EducationLevel::LowerSecondary { .. } | EducationLevel::UpperSecondary { .. } => {
                "Bachelor in Education"
            }
            EducationLevel::TechnicalVocational { .. } => "Bachelor in Education",
            EducationLevel::HigherEducation { .. } => "Master in Education",
            _ => "Teacher Certificate",
        };

        return Err(EducationLawError::TeacherUnqualified {
            teacher_name: teacher.name.clone(),
            level: format!("{:?}", teacher.teaching_level),
            actual: format!("{:?}", teacher.qualification),
            required: required.to_string(),
        });
    }

    // Check license status
    match &teacher.license_status {
        TeacherLicenseStatus::Licensed { .. } => Ok(()),
        TeacherLicenseStatus::Provisional { reason: _, .. } => {
            // Provisional is allowed but should be noted
            Ok(())
        }
        TeacherLicenseStatus::Expired { last_valid } => {
            Err(EducationLawError::TeacherLicenseExpired {
                teacher_name: teacher.name.clone(),
                expiry_date: last_valid.clone(),
            })
        }
        TeacherLicenseStatus::Revoked { reason, .. } => {
            Err(EducationLawError::TeacherLicenseRevoked {
                teacher_name: teacher.name.clone(),
                reason: reason.clone(),
            })
        }
        TeacherLicenseStatus::None => Err(EducationLawError::TeacherNoLicense {
            teacher_name: teacher.name.clone(),
        }),
    }
}

/// Validate teacher license validity (ກວດສອບໃບອະນຸຍາດຄູ)
pub fn validate_teacher_license(teacher: &Teacher, current_date: &str) -> Result<()> {
    match &teacher.license_status {
        TeacherLicenseStatus::Licensed { expiry_date, .. } => {
            if expiry_date < &current_date.to_string() {
                return Err(EducationLawError::TeacherLicenseExpired {
                    teacher_name: teacher.name.clone(),
                    expiry_date: expiry_date.clone(),
                });
            }
        }
        TeacherLicenseStatus::Provisional { end_date, .. } => {
            if end_date < &current_date.to_string() {
                return Err(EducationLawError::TeacherLicenseExpired {
                    teacher_name: teacher.name.clone(),
                    expiry_date: end_date.clone(),
                });
            }
        }
        TeacherLicenseStatus::Expired { last_valid } => {
            return Err(EducationLawError::TeacherLicenseExpired {
                teacher_name: teacher.name.clone(),
                expiry_date: last_valid.clone(),
            });
        }
        TeacherLicenseStatus::Revoked { reason, .. } => {
            return Err(EducationLawError::TeacherLicenseRevoked {
                teacher_name: teacher.name.clone(),
                reason: reason.clone(),
            });
        }
        TeacherLicenseStatus::None => {
            return Err(EducationLawError::TeacherNoLicense {
                teacher_name: teacher.name.clone(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// Compulsory Education Validation (ການກວດສອບການສຶກສາບັງຄັບ)
// ============================================================================

/// Validate compulsory education enrollment (ກວດສອບການລົງທະບຽນການສຶກສາບັງຄັບ)
///
/// Validates compliance with:
/// - Article 26: Compulsory education requirements (ages 6-14)
///
/// # Arguments
/// * `record` - Compulsory education record to validate
///
/// # Returns
/// * `Ok(())` if child is properly enrolled
/// * `Err(EducationLawError)` if enrollment issues exist
pub fn validate_compulsory_enrollment(record: &CompulsoryEducation) -> Result<()> {
    // Check if child is within compulsory education age
    if record.current_age < COMPULSORY_EDUCATION_START_AGE {
        return Err(EducationLawError::EnrollmentAgeNotReached {
            child_name: record.child_name.clone(),
            current_age: record.current_age,
            required_age: COMPULSORY_EDUCATION_START_AGE,
        });
    }

    // If child is within compulsory age, check enrollment status
    if record.is_compulsory_age() {
        match &record.enrollment_status {
            EnrollmentStatus::Enrolled { .. } => {
                // Check if grade is appropriate for age
                let expected_grade = record.expected_grade();
                if record.current_grade < expected_grade.saturating_sub(1) {
                    // Allow 1 grade behind, but flag if more
                    // This is a warning, not an error
                }
            }
            EnrollmentStatus::NotEnrolled { reason } => {
                // Some reasons are acceptable, others are not
                match reason {
                    NonEnrollmentReason::AgeNotReached => {
                        // This shouldn't happen if is_compulsory_age is true
                        return Err(EducationLawError::ChildNotEnrolled {
                            child_name: record.child_name.clone(),
                            age: record.current_age,
                        });
                    }
                    NonEnrollmentReason::Disability { .. } if record.has_special_needs => {
                        // Special needs children may have alternative arrangements
                        // Still check if appropriate services are being provided
                    }
                    _ => {
                        return Err(EducationLawError::ChildNotEnrolled {
                            child_name: record.child_name.clone(),
                            age: record.current_age,
                        });
                    }
                }
            }
            EnrollmentStatus::Dropped { reason, .. } => {
                return Err(EducationLawError::ChildDropout {
                    child_name: record.child_name.clone(),
                    reason: reason.clone(),
                });
            }
            EnrollmentStatus::Graduated { .. } => {
                // Child has completed compulsory education
            }
            EnrollmentStatus::Transferred { .. } => {
                // Transfer is acceptable
            }
            EnrollmentStatus::Suspended { reason, .. } => {
                return Err(EducationLawError::ChildSuspended {
                    child_name: record.child_name.clone(),
                    reason: reason.clone(),
                });
            }
        }
    } else if record.current_age > COMPULSORY_EDUCATION_END_AGE {
        // Check if child completed compulsory education
        match &record.enrollment_status {
            EnrollmentStatus::Graduated { .. } => {
                // Completed successfully
            }
            EnrollmentStatus::Enrolled { .. } => {
                if record.current_grade < 9 {
                    return Err(EducationLawError::CompulsoryEducationIncomplete {
                        child_name: record.child_name.clone(),
                        age: record.current_age,
                        completed_grade: record.current_grade,
                    });
                }
            }
            _ => {
                return Err(EducationLawError::CompulsoryEducationIncomplete {
                    child_name: record.child_name.clone(),
                    age: record.current_age,
                    completed_grade: record.current_grade,
                });
            }
        }
    }

    Ok(())
}

/// Check if child should be enrolled but is not
/// ກວດສອບວ່າເດັກຄວນຈະລົງທະບຽນແຕ່ບໍ່ໄດ້ລົງ
pub fn check_enrollment_requirement(record: &CompulsoryEducation) -> bool {
    record.should_be_enrolled()
}

// ============================================================================
// Student Rights Validation (ການກວດສອບສິດນັກຮຽນ)
// ============================================================================

/// Validate student rights compliance (ກວດສອບການປະຕິບັດຕາມສິດນັກຮຽນ)
///
/// Validates compliance with:
/// - Article 72-79: Student rights protections
///
/// # Arguments
/// * `right` - Student right to validate
/// * `is_compliant` - Whether the institution is compliant
/// * `violation_description` - Description of any violation
///
/// # Returns
/// * `Ok(())` if rights are respected
/// * `Err(EducationLawError)` if rights are violated
pub fn validate_student_rights(
    right: &StudentRight,
    is_compliant: bool,
    violation_description: Option<&str>,
) -> Result<()> {
    if !is_compliant {
        let description = violation_description
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Rights violation detected".to_string());

        match right.right_type {
            StudentRightType::NonDiscrimination => {
                return Err(EducationLawError::Discrimination { description });
            }
            StudentRightType::SafeLearningEnvironment => {
                return Err(EducationLawError::UnsafeEnvironment {
                    hazard: description,
                });
            }
            StudentRightType::PrivacyOfRecords => {
                return Err(EducationLawError::PrivacyViolation { description });
            }
            StudentRightType::SpecialNeedsAccommodation => {
                return Err(EducationLawError::SpecialNeedsAccommodationDenied { description });
            }
            StudentRightType::MotherTongueEducation => {
                return Err(EducationLawError::MotherTongueEducationDenied {
                    ethnic_group: description,
                });
            }
            _ => {
                return Err(EducationLawError::StudentRightsViolation {
                    right_type: format!("{:?}", right.right_type),
                    description,
                    article: right.article_reference,
                });
            }
        }
    }

    Ok(())
}

/// Validate no discrimination (Article 72)
/// ກວດສອບບໍ່ມີການຈຳແນກ (ມາດຕາ 72)
pub fn validate_non_discrimination(
    has_discrimination: bool,
    discrimination_type: Option<&str>,
) -> Result<()> {
    if has_discrimination {
        return Err(EducationLawError::Discrimination {
            description: discrimination_type
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Unspecified discrimination".to_string()),
        });
    }
    Ok(())
}

/// Validate safe learning environment (Article 75)
/// ກວດສອບສະພາບແວດລ້ອມການຮຽນທີ່ປອດໄພ (ມາດຕາ 75)
pub fn validate_safe_environment(is_safe: bool, hazard_description: Option<&str>) -> Result<()> {
    if !is_safe {
        return Err(EducationLawError::UnsafeEnvironment {
            hazard: hazard_description
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Unsafe condition detected".to_string()),
        });
    }
    Ok(())
}

// ============================================================================
// Student Age Validation (ການກວດສອບອາຍຸນັກຮຽນ)
// ============================================================================

/// Validate student age for education level (ກວດສອບອາຍຸນັກຮຽນສຳລັບລະດັບການສຶກສາ)
///
/// # Arguments
/// * `age` - Student age
/// * `level` - Education level
///
/// # Returns
/// * `Ok(())` if age is appropriate
/// * `Err(EducationLawError)` if age is inappropriate
pub fn validate_student_age_for_level(age: u8, level: &EducationLevel) -> Result<()> {
    if let Some((min_age, max_age)) = level.typical_age_range() {
        // Allow some flexibility (2 years above or below)
        let flexible_min = min_age.saturating_sub(2);
        let flexible_max = max_age + 2;

        if age < flexible_min {
            return Err(EducationLawError::BelowMinimumAge {
                age,
                min_age: flexible_min,
                level: format!("{:?}", level),
            });
        }

        if age > flexible_max {
            return Err(EducationLawError::AboveMaximumAge {
                age,
                max_age: flexible_max,
                level: format!("{:?}", level),
            });
        }

        // Check if age is significantly outside typical range
        if age < min_age || age > max_age {
            return Err(EducationLawError::InappropriateAgeForLevel {
                age,
                level: format!("{:?}", level),
                min_age,
                max_age,
            });
        }
    }

    Ok(())
}

/// Validate pre-primary age (Article 25)
/// ກວດສອບອາຍຸກ່ອນປະຖົມ (ມາດຕາ 25)
pub fn validate_pre_primary_age(age: u8) -> Result<()> {
    if age < PRE_PRIMARY_MIN_AGE {
        return Err(EducationLawError::BelowMinimumAge {
            age,
            min_age: PRE_PRIMARY_MIN_AGE,
            level: "Pre-Primary".to_string(),
        });
    }

    if age > PRE_PRIMARY_MAX_AGE {
        return Err(EducationLawError::AboveMaximumAge {
            age,
            max_age: PRE_PRIMARY_MAX_AGE,
            level: "Pre-Primary".to_string(),
        });
    }

    Ok(())
}

/// Validate primary school entry age (Article 27)
/// ກວດສອບອາຍຸເຂົ້າປະຖົມ (ມາດຕາ 27)
pub fn validate_primary_entry_age(age: u8) -> Result<()> {
    if age < COMPULSORY_EDUCATION_START_AGE {
        return Err(EducationLawError::BelowMinimumAge {
            age,
            min_age: COMPULSORY_EDUCATION_START_AGE,
            level: "Primary".to_string(),
        });
    }

    // Allow late entry up to age 8
    if age > 8 {
        return Err(EducationLawError::AboveMaximumAge {
            age,
            max_age: 8,
            level: "Primary (entry)".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Scholarship Eligibility Validation (ການກວດສອບສິດຮັບທຶນການສຶກສາ)
// ============================================================================

/// Validate scholarship eligibility (ກວດສອບສິດຮັບທຶນການສຶກສາ)
///
/// # Arguments
/// * `scholarship` - Scholarship to validate against
/// * `applicant_gpa` - Applicant's GPA
/// * `family_income` - Family income in LAK (optional)
/// * `age` - Applicant's age (optional)
/// * `is_ethnic_minority` - Whether applicant is ethnic minority (optional)
///
/// # Returns
/// * `Ok(())` if applicant is eligible
/// * `Err(EducationLawError)` if applicant is not eligible
pub fn validate_scholarship_eligibility(
    scholarship: &Scholarship,
    applicant_gpa: f64,
    family_income: Option<u64>,
    age: Option<u8>,
    is_ethnic_minority: Option<bool>,
) -> Result<()> {
    for criterion in &scholarship.eligibility_criteria {
        match criterion {
            EligibilityCriterion::MinimumGPA(min_gpa) => {
                if applicant_gpa < *min_gpa {
                    return Err(EducationLawError::GPABelowMinimum {
                        actual: applicant_gpa,
                        required: *min_gpa,
                    });
                }
            }
            EligibilityCriterion::EconomicNeed { max_family_income } => {
                if let (Some(max), Some(actual)) = (max_family_income, family_income)
                    && actual > *max
                {
                    return Err(EducationLawError::IncomeAboveMaximum {
                        actual_lak: actual,
                        max_lak: *max,
                    });
                }
            }
            EligibilityCriterion::AgeLimit { min, max } => {
                if let Some(applicant_age) = age {
                    if let Some(min_age) = min
                        && applicant_age < *min_age
                    {
                        return Err(EducationLawError::ScholarshipEligibilityNotMet {
                            scholarship_name: scholarship.scholarship_name.clone(),
                            reason: format!("Age {} is below minimum {}", applicant_age, min_age),
                        });
                    }
                    if let Some(max_age) = max
                        && applicant_age > *max_age
                    {
                        return Err(EducationLawError::ScholarshipEligibilityNotMet {
                            scholarship_name: scholarship.scholarship_name.clone(),
                            reason: format!("Age {} is above maximum {}", applicant_age, max_age),
                        });
                    }
                }
            }
            EligibilityCriterion::EthnicMinority { ethnic_group: _ } => {
                if let Some(is_minority) = is_ethnic_minority
                    && !is_minority
                {
                    return Err(EducationLawError::ScholarshipEligibilityNotMet {
                        scholarship_name: scholarship.scholarship_name.clone(),
                        reason: "Scholarship requires ethnic minority status".to_string(),
                    });
                }
            }
            _ => {
                // Other criteria need specific validation
            }
        }
    }

    Ok(())
}

// ============================================================================
// Curriculum Compliance Validation (ການກວດສອບການປະຕິບັດຕາມຫຼັກສູດ)
// ============================================================================

/// Validate curriculum compliance with national standards (Article 16)
/// ກວດສອບການປະຕິບັດຕາມຫຼັກສູດແຫ່ງຊາດ (ມາດຕາ 16)
pub fn validate_curriculum_compliance(
    curriculum: &NationalCurriculum,
    required_subjects: &[&str],
    min_hours_per_year: u32,
) -> Result<()> {
    // Check total hours
    if curriculum.total_hours_per_year < min_hours_per_year {
        return Err(EducationLawError::InsufficientInstructionalHours {
            actual: curriculum.total_hours_per_year,
            required: min_hours_per_year,
            subject: "Total".to_string(),
        });
    }

    // Check required subjects
    for required in required_subjects {
        let found = curriculum.core_subjects.iter().any(|s| {
            s.name_en.to_lowercase().contains(&required.to_lowercase())
                || s.name_lao.contains(required)
        });

        if !found {
            return Err(EducationLawError::MissingRequiredSubject {
                subject_name: (*required).to_string(),
            });
        }
    }

    Ok(())
}

/// Validate subject hours meet minimum requirements
/// ກວດສອບຊົ່ວໂມງວິຊາຕາມເກນຂັ້ນຕ່ຳ
pub fn validate_subject_hours(subject: &Subject, min_hours_per_week: u8) -> Result<()> {
    if subject.hours_per_week < min_hours_per_week {
        return Err(EducationLawError::InsufficientInstructionalHours {
            actual: subject.hours_per_week as u32,
            required: min_hours_per_week as u32,
            subject: subject.name_en.clone(),
        });
    }

    Ok(())
}

// ============================================================================
// Comprehensive Validation (ການກວດສອບແບບຄົບຖ້ວນ)
// ============================================================================

/// Perform comprehensive validation of an educational institution
/// ກວດສອບສະຖາບັນການສຶກສາແບບຄົບຖ້ວນ
///
/// # Arguments
/// * `institution` - Educational institution to validate
/// * `current_date` - Current date for validity checks
///
/// # Returns
/// * `Ok(Vec<String>)` - List of warnings (non-critical issues)
/// * `Err(EducationLawError)` - Critical violation found
pub fn validate_institution_comprehensive(
    institution: &EducationalInstitution,
    current_date: &str,
) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Critical validations
    validate_institution_license(institution, current_date)?;
    validate_institution_programs(institution)?;

    // Check accreditation (may be pending for new institutions)
    match validate_institution_accreditation(institution, current_date) {
        Ok(()) => {}
        Err(EducationLawError::InstitutionNotAccredited { .. }) => {
            warnings.push("Institution is not yet accredited (ສະຖາບັນຍັງບໍ່ໄດ້ຮັບການຮັບຮອງ)".to_string());
        }
        Err(e) => return Err(e),
    }

    // Non-critical checks
    if institution.name_lao.is_empty() {
        warnings.push("Missing institution name in Lao (ຂາດຊື່ສະຖາບັນເປັນພາສາລາວ)".to_string());
    }

    if institution.contact_phone.is_none() && institution.contact_email.is_none() {
        warnings.push("No contact information provided (ບໍ່ມີຂໍ້ມູນຕິດຕໍ່)".to_string());
    }

    if let Some(ratio) = institution.teacher_student_ratio()
        && ratio > 40.0
    {
        warnings.push(format!(
            "High teacher-student ratio: {:.1}:1 (ອັດຕາສ່ວນຄູຕໍ່ນັກຮຽນສູງ: {:.1}:1)",
            ratio, ratio
        ));
    }

    if institution.programs_offered.is_empty() {
        warnings.push("No programs currently offered (ບໍ່ມີຫຼັກສູດທີ່ເປີດສອນ)".to_string());
    }

    Ok(warnings)
}

/// Validate all programs in an institution
/// ກວດສອບທຸກຫຼັກສູດໃນສະຖາບັນ
pub fn validate_all_programs(
    institution: &EducationalInstitution,
    current_date: &str,
) -> Vec<Result<()>> {
    institution
        .programs_offered
        .iter()
        .map(|program| validate_program_accreditation(program, current_date))
        .collect()
}

/// Validate class size (Article 47)
/// ກວດສອບຂະໜາດຫ້ອງຮຽນ (ມາດຕາ 47)
pub fn validate_class_size(class_size: u32, level: &EducationLevel) -> Result<()> {
    let max_size = match level {
        EducationLevel::PrePrimary { .. } => 30,
        EducationLevel::Primary { .. } => MAX_CLASS_SIZE_PRIMARY,
        EducationLevel::LowerSecondary { .. } | EducationLevel::UpperSecondary { .. } => {
            MAX_CLASS_SIZE_SECONDARY
        }
        _ => 50,
    };

    if class_size > max_size {
        return Err(EducationLawError::ClassSizeExceeded {
            actual: class_size,
            max: max_size,
        });
    }

    Ok(())
}

/// Validate teacher-student ratio
/// ກວດສອບອັດຕາສ່ວນຄູຕໍ່ນັກຮຽນ
pub fn validate_teacher_student_ratio(
    teacher_count: u32,
    student_count: u32,
    level: &EducationLevel,
) -> Result<()> {
    if teacher_count == 0 {
        if student_count > 0 {
            return Err(EducationLawError::ValidationError {
                message: "No teachers assigned to students".to_string(),
            });
        }
        return Ok(());
    }

    let ratio = student_count as f64 / teacher_count as f64;
    let max_ratio = match level {
        EducationLevel::PrePrimary { .. } => 20.0,
        EducationLevel::Primary { .. } => 35.0,
        EducationLevel::LowerSecondary { .. } | EducationLevel::UpperSecondary { .. } => 40.0,
        _ => 50.0,
    };

    if ratio > max_ratio {
        return Err(EducationLawError::TeacherStudentRatioExceeded {
            actual: ratio,
            max: max_ratio,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_institution() -> EducationalInstitution {
        EducationalInstitutionBuilder::new()
            .name_lao("ໂຮງຮຽນທົດສອບ")
            .name_en("Test School")
            .institution_type(InstitutionType::PrimarySchool)
            .ownership_type(OwnershipType::Public {
                ministry: "Ministry of Education".to_string(),
            })
            .license_number("LIC-2024-001")
            .license_dates("2024-01-01", "2029-01-01")
            .location("Vientiane", "Chanthabuly", "123 Test Road")
            .accreditation_status(AccreditationStatus::FullyAccredited {
                accreditation_date: "2024-01-01".to_string(),
                expiry_date: "2029-01-01".to_string(),
                accrediting_body: "MOES".to_string(),
            })
            .capacity(500, 350)
            .teachers(20, 18)
            .established_year(2000)
            .build()
            .expect("Should build successfully")
    }

    fn create_valid_teacher() -> Teacher {
        Teacher {
            name: "ຄູທົດສອບ".to_string(),
            name_lao: Some("ຄູທົດສອບ".to_string()),
            teacher_id: "TCH-001".to_string(),
            qualification: TeacherQualification::BachelorInEducation,
            teaching_level: EducationLevel::Primary { grades: 5 },
            subject_specialization: Some("Mathematics".to_string()),
            license_status: TeacherLicenseStatus::Licensed {
                expiry_date: "2029-01-01".to_string(),
                license_number: "LIC-TCH-001".to_string(),
            },
            years_of_experience: 5,
            employment_status: TeacherEmploymentStatus::FullTime,
            institution_id: Some("SCH-001".to_string()),
            professional_development_hours: 100,
            teaching_subjects: vec!["Mathematics".to_string(), "Science".to_string()],
        }
    }

    fn create_valid_compulsory_education() -> CompulsoryEducation {
        CompulsoryEducation {
            child_name: "ເດັກທົດສອບ".to_string(),
            child_name_lao: Some("ເດັກທົດສອບ".to_string()),
            date_of_birth: "2017-01-01".to_string(),
            current_age: 7,
            enrollment_status: EnrollmentStatus::Enrolled {
                enrollment_date: "2023-09-01".to_string(),
                student_id: Some("STU-001".to_string()),
            },
            current_grade: 2,
            school_name: "Test Primary School".to_string(),
            school_id: Some("SCH-001".to_string()),
            guardian_name: "ພໍ່ແມ່ທົດສອບ".to_string(),
            guardian_relationship: "Parent".to_string(),
            guardian_contact: Some("020-12345678".to_string()),
            province: "Vientiane".to_string(),
            district: "Chanthabuly".to_string(),
            village: "Test Village".to_string(),
            is_ethnic_minority: false,
            ethnic_group: None,
            has_special_needs: false,
            special_needs_type: None,
        }
    }

    #[test]
    fn test_valid_institution_license() {
        let institution = create_valid_institution();
        assert!(validate_institution_license(&institution, "2024-06-15").is_ok());
    }

    #[test]
    fn test_expired_institution_license() {
        let mut institution = create_valid_institution();
        institution.license_expiry_date = "2023-01-01".to_string();

        let result = validate_institution_license(&institution, "2024-06-15");
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(EducationLawError::InstitutionLicenseExpired { .. })
        ));
    }

    #[test]
    fn test_capacity_exceeded() {
        let mut institution = create_valid_institution();
        institution.current_enrollment = 600;
        institution.student_capacity = 500;

        let result = validate_institution_license(&institution, "2024-06-15");
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(EducationLawError::CapacityExceeded { .. })
        ));
    }

    #[test]
    fn test_valid_teacher_qualification() {
        let teacher = create_valid_teacher();
        assert!(validate_teacher_qualification(&teacher).is_ok());
    }

    #[test]
    fn test_unqualified_teacher() {
        let mut teacher = create_valid_teacher();
        teacher.qualification = TeacherQualification::TeacherCertificate;
        teacher.teaching_level = EducationLevel::UpperSecondary { grades: 3 };

        let result = validate_teacher_qualification(&teacher);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(EducationLawError::TeacherUnqualified { .. })
        ));
    }

    #[test]
    fn test_valid_compulsory_enrollment() {
        let record = create_valid_compulsory_education();
        assert!(validate_compulsory_enrollment(&record).is_ok());
    }

    #[test]
    fn test_child_not_enrolled() {
        let mut record = create_valid_compulsory_education();
        record.enrollment_status = EnrollmentStatus::NotEnrolled {
            reason: NonEnrollmentReason::ParentalRefusal,
        };

        let result = validate_compulsory_enrollment(&record);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(EducationLawError::ChildNotEnrolled { .. })
        ));
    }

    #[test]
    fn test_student_age_for_level() {
        // Valid age for primary
        assert!(validate_student_age_for_level(7, &EducationLevel::Primary { grades: 5 }).is_ok());

        // Too young for primary
        let result = validate_student_age_for_level(3, &EducationLevel::Primary { grades: 5 });
        assert!(result.is_err());
    }

    #[test]
    fn test_scholarship_eligibility() {
        let scholarship = Scholarship {
            scholarship_name: "Test Scholarship".to_string(),
            scholarship_name_lao: Some("ທຶນທົດສອບ".to_string()),
            provider: ScholarshipProvider::Government { ministry: None },
            eligibility_criteria: vec![
                EligibilityCriterion::MinimumGPA(3.0),
                EligibilityCriterion::EconomicNeed {
                    max_family_income: Some(5_000_000),
                },
            ],
            coverage: ScholarshipCoverage {
                tuition_percentage: 100.0,
                monthly_stipend_lak: Some(500_000),
                book_allowance_lak: Some(200_000),
                accommodation_provided: false,
                travel_allowance_lak: None,
                health_insurance_provided: false,
                other_benefits: vec![],
            },
            duration_years: 4,
            application_deadline: Some("2024-08-31".to_string()),
            num_awards: Some(50),
            field_restrictions: vec![],
            education_level: EducationLevel::HigherEducation {
                degree_type: DegreeType::BachelorDegree,
            },
            is_renewable: true,
            contact_info: None,
        };

        // Eligible applicant
        assert!(
            validate_scholarship_eligibility(&scholarship, 3.5, Some(3_000_000), None, None)
                .is_ok()
        );

        // GPA too low
        let result =
            validate_scholarship_eligibility(&scholarship, 2.5, Some(3_000_000), None, None);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(EducationLawError::GPABelowMinimum { .. })
        ));

        // Income too high
        let result =
            validate_scholarship_eligibility(&scholarship, 3.5, Some(10_000_000), None, None);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(EducationLawError::IncomeAboveMaximum { .. })
        ));
    }

    #[test]
    fn test_class_size_validation() {
        // Valid class size
        assert!(validate_class_size(30, &EducationLevel::Primary { grades: 5 }).is_ok());

        // Class too large
        let result = validate_class_size(60, &EducationLevel::Primary { grades: 5 });
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(EducationLawError::ClassSizeExceeded { .. })
        ));
    }

    #[test]
    fn test_teacher_student_ratio() {
        // Valid ratio
        assert!(
            validate_teacher_student_ratio(10, 200, &EducationLevel::Primary { grades: 5 }).is_ok()
        );

        // Ratio too high
        let result = validate_teacher_student_ratio(5, 300, &EducationLevel::Primary { grades: 5 });
        assert!(result.is_err());
    }

    #[test]
    fn test_comprehensive_validation() {
        let institution = create_valid_institution();
        let result = validate_institution_comprehensive(&institution, "2024-06-15");
        assert!(result.is_ok());
    }
}
