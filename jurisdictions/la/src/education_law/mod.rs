//! Education Law Module (ກົດໝາຍການສຶກສາ)
//!
//! This module provides comprehensive support for Lao education law based on
//! **Education Law 2015** (Law No. 62/NA, dated July 16, 2015).
//!
//! # Legal Framework
//!
//! The Education Law 2015 is the primary legislation governing the education system
//! in the Lao People's Democratic Republic. It establishes the framework for:
//!
//! ## Key Provisions
//!
//! ### Education Levels (ລະດັບການສຶກສາ)
//! - **Article 25**: Pre-primary education (3-5 years) - ການສຶກສາກ່ອນປະຖົມ
//! - **Article 26**: Compulsory education (ages 6-14, 9 years) - ການສຶກສາບັງຄັບ
//! - **Article 27**: Primary education (Grades 1-5) - ການສຶກສາປະຖົມ
//! - **Article 28**: Lower secondary (Grades 6-9) - ການສຶກສາມັດທະຍົມຕົ້ນ
//! - **Article 29**: Upper secondary (Grades 10-12) - ການສຶກສາມັດທະຍົມປາຍ
//! - **Article 30**: Technical and vocational education - ການສຶກສາວິຊາຊີບ
//! - **Article 31-35**: Higher education - ການສຶກສາຊັ້ນສູງ
//! - **Article 36**: Non-formal education - ການສຶກສານອກລະບົບ
//!
//! ### Educational Institutions (ສະຖາບັນການສຶກສາ)
//! - **Article 42-47**: Establishment and operation requirements
//! - **Article 48-54**: Types of educational institutions
//! - **Article 55**: Accreditation and quality assurance
//!
//! ### Teachers and Staff (ຄູແລະພະນັກງານ)
//! - **Article 56-57**: Rights and duties of teachers
//! - **Article 58**: Teacher qualification requirements
//! - **Article 59**: Teacher training and development
//! - **Article 60**: Teacher licensing
//!
//! ### Student Rights (ສິດຂອງນັກຮຽນ)
//! - **Article 72**: Non-discrimination
//! - **Article 73**: Mother tongue education for ethnic minorities
//! - **Article 74**: Quality education access
//! - **Article 75**: Safe learning environment
//! - **Article 76**: Participation in school activities
//! - **Article 77**: Privacy of student records
//! - **Article 78**: Appeal rights
//! - **Article 79**: Access to educational information
//!
//! ### Special Education (ການສຶກສາພິເສດ)
//! - **Article 22**: Education for students with special needs
//! - Accommodations and inclusive education provisions
//!
//! # Features
//!
//! - **Bilingual Support**: All types and errors support both Lao (ລາວ) and English
//! - **Type-safe Validation**: Compile-time guarantees for education law compliance
//! - **Comprehensive Coverage**: All major aspects of Education Law 2015
//! - **Builder Patterns**: Ergonomic construction of complex types
//!
//! # Examples
//!
//! ## Validating an Educational Institution
//!
//! ```rust
//! use legalis_la::education_law::*;
//!
//! let institution = EducationalInstitutionBuilder::new()
//!     .name_lao("ໂຮງຮຽນປະຖົມວຽງຈັນ")
//!     .name_en("Vientiane Primary School")
//!     .institution_type(InstitutionType::PrimarySchool)
//!     .ownership_type(OwnershipType::Public {
//!         ministry: "Ministry of Education and Sports".to_string(),
//!     })
//!     .license_number("LIC-2024-001")
//!     .license_dates("2024-01-01", "2029-01-01")
//!     .location("Vientiane", "Chanthabuly", "123 Education Road")
//!     .accreditation_status(AccreditationStatus::FullyAccredited {
//!         accreditation_date: "2024-01-01".to_string(),
//!         expiry_date: "2029-01-01".to_string(),
//!         accrediting_body: "MOES Quality Assurance".to_string(),
//!     })
//!     .capacity(500, 350)
//!     .teachers(20, 18)
//!     .established_year(2000)
//!     .build()
//!     .expect("Should build successfully");
//!
//! // Validate the institution
//! match validate_institution_license(&institution, "2024-06-15") {
//!     Ok(()) => println!("Institution license is valid! / ໃບອະນຸຍາດສະຖາບັນຖືກຕ້ອງ!"),
//!     Err(e) => {
//!         println!("English: {}", e.english_message());
//!         println!("Lao: {}", e.lao_message());
//!     }
//! }
//! ```
//!
//! ## Validating Compulsory Education Enrollment
//!
//! ```rust
//! use legalis_la::education_law::*;
//!
//! let child = CompulsoryEducation {
//!     child_name: "ສົມຊາຍ ວົງສະຫວັນ".to_string(),
//!     child_name_lao: Some("ສົມຊາຍ ວົງສະຫວັນ".to_string()),
//!     date_of_birth: "2017-01-15".to_string(),
//!     current_age: 7,
//!     enrollment_status: EnrollmentStatus::Enrolled {
//!         enrollment_date: "2023-09-01".to_string(),
//!         student_id: Some("STU-2023-001".to_string()),
//!     },
//!     current_grade: 2,
//!     school_name: "Vientiane Primary School".to_string(),
//!     school_id: Some("SCH-001".to_string()),
//!     guardian_name: "ພໍ່ແມ່ ວົງສະຫວັນ".to_string(),
//!     guardian_relationship: "Parent".to_string(),
//!     guardian_contact: Some("020-12345678".to_string()),
//!     province: "Vientiane".to_string(),
//!     district: "Chanthabuly".to_string(),
//!     village: "Ban Phonsinuan".to_string(),
//!     is_ethnic_minority: false,
//!     ethnic_group: None,
//!     has_special_needs: false,
//!     special_needs_type: None,
//! };
//!
//! // Validate compulsory education enrollment (Article 26)
//! match validate_compulsory_enrollment(&child) {
//!     Ok(()) => println!("Child is properly enrolled in compulsory education"),
//!     Err(e) => println!("Enrollment issue: {}", e),
//! }
//! ```
//!
//! ## Validating Teacher Qualifications
//!
//! ```rust
//! use legalis_la::education_law::*;
//!
//! let teacher = Teacher {
//!     name: "ຄູສົມຍິງ ພິມມະລາດ".to_string(),
//!     name_lao: Some("ຄູສົມຍິງ ພິມມະລາດ".to_string()),
//!     teacher_id: "TCH-2020-001".to_string(),
//!     qualification: TeacherQualification::BachelorInEducation,
//!     teaching_level: EducationLevel::Primary { grades: 5 },
//!     subject_specialization: Some("Mathematics".to_string()),
//!     license_status: TeacherLicenseStatus::Licensed {
//!         expiry_date: "2028-12-31".to_string(),
//!         license_number: "LIC-TCH-2020-001".to_string(),
//!     },
//!     years_of_experience: 8,
//!     employment_status: TeacherEmploymentStatus::FullTime,
//!     institution_id: Some("SCH-001".to_string()),
//!     professional_development_hours: 120,
//!     teaching_subjects: vec!["Mathematics".to_string(), "Science".to_string()],
//! };
//!
//! // Validate teacher qualification (Article 58)
//! match validate_teacher_qualification(&teacher) {
//!     Ok(()) => println!("Teacher is qualified / ຄູມີຄຸນວຸດທິ"),
//!     Err(e) => println!("Qualification issue: {}", e),
//! }
//! ```
//!
//! ## Checking Scholarship Eligibility
//!
//! ```rust
//! use legalis_la::education_law::*;
//!
//! let scholarship = Scholarship {
//!     scholarship_name: "Government Merit Scholarship".to_string(),
//!     scholarship_name_lao: Some("ທຶນລັດຖະບານສຳລັບນັກຮຽນເກັ່ງ".to_string()),
//!     provider: ScholarshipProvider::Government {
//!         ministry: Some("Ministry of Education and Sports".to_string()),
//!     },
//!     eligibility_criteria: vec![
//!         EligibilityCriterion::MinimumGPA(3.5),
//!         EligibilityCriterion::EconomicNeed {
//!             max_family_income: Some(5_000_000),
//!         },
//!     ],
//!     coverage: ScholarshipCoverage {
//!         tuition_percentage: 100.0,
//!         monthly_stipend_lak: Some(800_000),
//!         book_allowance_lak: Some(300_000),
//!         accommodation_provided: true,
//!         travel_allowance_lak: Some(500_000),
//!         health_insurance_provided: true,
//!         other_benefits: vec!["Laptop".to_string()],
//!     },
//!     duration_years: 4,
//!     application_deadline: Some("2024-08-31".to_string()),
//!     num_awards: Some(100),
//!     field_restrictions: vec![],
//!     education_level: EducationLevel::HigherEducation {
//!         degree_type: DegreeType::BachelorDegree,
//!     },
//!     is_renewable: true,
//!     contact_info: Some("scholarship@moes.gov.la".to_string()),
//! };
//!
//! // Check eligibility
//! match validate_scholarship_eligibility(&scholarship, 3.8, Some(3_000_000), Some(18), None) {
//!     Ok(()) => println!("Applicant is eligible for scholarship"),
//!     Err(e) => println!("Not eligible: {}", e),
//! }
//! ```
//!
//! # Bilingual Error Messages
//!
//! All errors include both English and Lao messages:
//!
//! ```rust
//! use legalis_la::education_law::*;
//!
//! let error = EducationLawError::ChildNotEnrolled {
//!     child_name: "ສົມຊາຍ".to_string(),
//!     age: 7,
//! };
//!
//! println!("English: {}", error.english_message());
//! // "Child 'ສົມຊາຍ' aged 7 is not enrolled in compulsory education (Article 26)"
//!
//! println!("Lao: {}", error.lao_message());
//! // "ເດັກ 'ສົມຊາຍ' ອາຍຸ 7 ປີ ບໍ່ໄດ້ລົງທະບຽນໃນການສຶກສາບັງຄັບ (ມາດຕາ 26)"
//! ```
//!
//! # Legal Context
//!
//! The Education Law 2015 was enacted to modernize Lao education regulations and
//! ensure quality education for all citizens. Key principles include:
//!
//! - Universal access to quality education
//! - Free and compulsory primary and lower secondary education
//! - Protection of student rights and welfare
//! - Professional standards for teachers
//! - Quality assurance through accreditation
//! - Inclusive education for students with special needs
//! - Mother tongue education for ethnic minorities
//!
//! # Compliance Notes
//!
//! When implementing education law compliance in Laos:
//!
//! 1. **Compulsory Education**: All children aged 6-14 must complete 9 years of education
//! 2. **Teacher Qualification**: Teachers must meet minimum qualification for their level
//! 3. **Institution Licensing**: All educational institutions must be licensed
//! 4. **Accreditation**: Programs should be accredited by the Ministry of Education
//! 5. **Student Rights**: Non-discrimination and safe environment are mandatory
//! 6. **Special Needs**: Accommodations must be provided for students with disabilities
//! 7. **Quality Standards**: National curriculum standards must be followed

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types and functions
pub use error::{EducationLawError, ErrorCategory, Result};

// Re-export types for convenience
pub use types::{
    // Constants
    ACADEMIC_YEAR_END_MONTH,
    ACADEMIC_YEAR_START_MONTH,
    ACCREDITATION_VALIDITY_YEARS,
    // Institutions
    AccreditationStatus,
    COMPULSORY_EDUCATION_END_AGE,
    COMPULSORY_EDUCATION_START_AGE,
    COMPULSORY_EDUCATION_YEARS,
    // Students
    CompulsoryEducation,
    // Education Levels
    DegreeType,
    EducationLevel,
    // Programs
    EducationProgram,
    EducationalInstitution,
    EducationalInstitutionBuilder,
    // Scholarships
    EligibilityCriterion,
    EnrollmentStatus,
    EntryRequirement,
    InstitutionType,
    InstructionLanguage,
    LICENSE_VALIDITY_YEARS,
    LOWER_SECONDARY_YEARS,
    MAX_CLASS_SIZE_PRIMARY,
    MAX_CLASS_SIZE_SECONDARY,
    MIN_CREDITS_BACHELOR,
    MIN_CREDITS_MASTER,
    MIN_TEACHER_QUALIFICATION_PRIMARY,
    MIN_TEACHER_QUALIFICATION_SECONDARY,
    // Curriculum
    NationalCurriculum,
    NonEnrollmentReason,
    OwnershipType,
    PRE_PRIMARY_MAX_AGE,
    PRE_PRIMARY_MIN_AGE,
    PRIMARY_EDUCATION_YEARS,
    RightScope,
    Scholarship,
    ScholarshipCoverage,
    ScholarshipProvider,
    SpecialEducationType,
    StudentRight,
    StudentRightType,
    Subject,
    // Teachers
    Teacher,
    TeacherEmploymentStatus,
    TeacherLicenseStatus,
    TeacherQualification,
    UPPER_SECONDARY_YEARS,
};

pub use validator::{
    // Compulsory education validators
    check_enrollment_requirement,
    // Institution validators
    validate_all_programs,
    validate_class_size,
    validate_compulsory_enrollment,
    // Curriculum validators
    validate_curriculum_compliance,
    validate_institution_accreditation,
    validate_institution_comprehensive,
    validate_institution_license,
    validate_institution_programs,
    // Student rights validators
    validate_non_discrimination,
    // Age validators
    validate_pre_primary_age,
    validate_primary_entry_age,
    // Program validators
    validate_program_accreditation,
    validate_program_credits,
    validate_program_duration,
    validate_safe_environment,
    // Scholarship validators
    validate_scholarship_eligibility,
    validate_student_age_for_level,
    validate_student_rights,
    validate_subject_hours,
    // Teacher validators
    validate_teacher_license,
    validate_teacher_qualification,
    validate_teacher_student_ratio,
};
