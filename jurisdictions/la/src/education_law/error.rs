//! Education Law Error Types (ປະເພດຄວາມຜິດພາດກົດໝາຍການສຶກສາ)
//!
//! Comprehensive error types for Lao education law validation and compliance.
//! All errors include bilingual messages (Lao/English) where applicable.
//!
//! # Legal References
//! - Education Law 2015 (Law No. 62/NA) - ກົດໝາຍວ່າດ້ວຍການສຶກສາ ປີ 2015

use thiserror::Error;

/// Result type for education law operations
pub type Result<T> = std::result::Result<T, EducationLawError>;

/// Education law errors (ຄວາມຜິດພາດກົດໝາຍການສຶກສາ)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum EducationLawError {
    // ========================================================================
    // Institution License Errors (ຄວາມຜິດພາດໃບອະນຸຍາດສະຖາບັນ)
    // ========================================================================
    /// Institution is not licensed (Article 42)
    /// ສະຖາບັນບໍ່ມີໃບອະນຸຍາດ (ມາດຕາ 42)
    #[error(
        "Institution '{institution_name}' is not licensed to operate (Article 42)\nສະຖາບັນ '{institution_name}' ບໍ່ມີໃບອະນຸຍາດດຳເນີນການ (ມາດຕາ 42)"
    )]
    InstitutionUnlicensed { institution_name: String },

    /// Institution license has expired (Article 42)
    /// ໃບອະນຸຍາດສະຖາບັນໝົດອາຍຸ (ມາດຕາ 42)
    #[error(
        "Institution license expired on {expiry_date} (Article 42)\nໃບອະນຸຍາດສະຖາບັນໝົດອາຍຸໃນວັນທີ {expiry_date} (ມາດຕາ 42)"
    )]
    InstitutionLicenseExpired { expiry_date: String },

    /// Institution license is invalid (Article 42)
    /// ໃບອະນຸຍາດສະຖາບັນບໍ່ຖືກຕ້ອງ (ມາດຕາ 42)
    #[error(
        "Institution license '{license_number}' is invalid: {reason} (Article 42)\nໃບອະນຸຍາດສະຖາບັນ '{license_number}' ບໍ່ຖືກຕ້ອງ: {reason} (ມາດຕາ 42)"
    )]
    InstitutionLicenseInvalid {
        license_number: String,
        reason: String,
    },

    /// Student capacity exceeded (Article 47)
    /// ເກີນຄວາມຈຸນັກຮຽນ (ມາດຕາ 47)
    #[error(
        "Student enrollment {current} exceeds capacity of {capacity} (Article 47)\nຈຳນວນນັກຮຽນ {current} ເກີນຄວາມຈຸ {capacity} (ມາດຕາ 47)"
    )]
    CapacityExceeded { current: u32, capacity: u32 },

    /// Missing license number
    /// ຂາດເລກທີໃບອະນຸຍາດ
    #[error("Missing institution license number\nຂາດເລກທີໃບອະນຸຍາດສະຖາບັນ")]
    MissingLicenseNumber,

    /// License number format invalid
    /// ຮູບແບບເລກທີໃບອະນຸຍາດບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid license number format: {license_number}\nຮູບແບບເລກທີໃບອະນຸຍາດບໍ່ຖືກຕ້ອງ: {license_number}"
    )]
    InvalidLicenseFormat { license_number: String },

    // ========================================================================
    // Accreditation Errors (ຄວາມຜິດພາດການຮັບຮອງຄຸນນະພາບ)
    // ========================================================================
    /// Program is not accredited (Article 55)
    /// ຫຼັກສູດບໍ່ໄດ້ຮັບການຮັບຮອງ (ມາດຕາ 55)
    #[error(
        "Program '{program_name}' is not accredited (Article 55)\nຫຼັກສູດ '{program_name}' ບໍ່ໄດ້ຮັບການຮັບຮອງຄຸນນະພາບ (ມາດຕາ 55)"
    )]
    ProgramNotAccredited { program_name: String },

    /// Accreditation has expired (Article 55)
    /// ການຮັບຮອງຄຸນນະພາບໝົດອາຍຸ (ມາດຕາ 55)
    #[error(
        "Accreditation for '{program_name}' expired on {expiry_date} (Article 55)\nການຮັບຮອງຄຸນນະພາບຂອງ '{program_name}' ໝົດອາຍຸໃນວັນທີ {expiry_date} (ມາດຕາ 55)"
    )]
    AccreditationExpired {
        program_name: String,
        expiry_date: String,
    },

    /// Accreditation was revoked (Article 55)
    /// ການຮັບຮອງຄຸນນະພາບຖືກຖອນ (ມາດຕາ 55)
    #[error(
        "Accreditation for '{program_name}' was revoked: {reason} (Article 55)\nການຮັບຮອງຄຸນນະພາບຂອງ '{program_name}' ຖືກຖອນ: {reason} (ມາດຕາ 55)"
    )]
    AccreditationRevoked {
        program_name: String,
        reason: String,
    },

    /// Provisional accreditation conditions not met
    /// ເງື່ອນໄຂການຮັບຮອງຊົ່ວຄາວບໍ່ບັນລຸ
    #[error(
        "Provisional accreditation conditions not met: {conditions}\nເງື່ອນໄຂການຮັບຮອງຊົ່ວຄາວບໍ່ບັນລຸ: {conditions}"
    )]
    ProvisionalConditionsNotMet { conditions: String },

    /// Institution not accredited (Article 55)
    /// ສະຖາບັນບໍ່ໄດ້ຮັບການຮັບຮອງ (ມາດຕາ 55)
    #[error(
        "Institution '{institution_name}' is not accredited (Article 55)\nສະຖາບັນ '{institution_name}' ບໍ່ໄດ້ຮັບການຮັບຮອງຄຸນນະພາບ (ມາດຕາ 55)"
    )]
    InstitutionNotAccredited { institution_name: String },

    // ========================================================================
    // Teacher Qualification Errors (ຄວາມຜິດພາດຄຸນວຸດທິຄູ)
    // ========================================================================
    /// Teacher is unqualified for teaching level (Article 58)
    /// ຄູບໍ່ມີຄຸນວຸດທິສຳລັບລະດັບທີ່ສອນ (ມາດຕາ 58)
    #[error(
        "Teacher '{teacher_name}' does not meet qualification requirements for {level}: has {actual}, requires {required} (Article 58)\nຄູ '{teacher_name}' ບໍ່ມີຄຸນວຸດທິສຳລັບ {level}: ມີ {actual}, ຕ້ອງການ {required} (ມາດຕາ 58)"
    )]
    TeacherUnqualified {
        teacher_name: String,
        level: String,
        actual: String,
        required: String,
    },

    /// Teacher license has expired (Article 60)
    /// ໃບອະນຸຍາດຄູໝົດອາຍຸ (ມາດຕາ 60)
    #[error(
        "Teacher license for '{teacher_name}' expired on {expiry_date} (Article 60)\nໃບອະນຸຍາດຄູຂອງ '{teacher_name}' ໝົດອາຍຸໃນວັນທີ {expiry_date} (ມາດຕາ 60)"
    )]
    TeacherLicenseExpired {
        teacher_name: String,
        expiry_date: String,
    },

    /// Teacher license was revoked (Article 60)
    /// ໃບອະນຸຍາດຄູຖືກຖອນ (ມາດຕາ 60)
    #[error(
        "Teacher license for '{teacher_name}' was revoked: {reason} (Article 60)\nໃບອະນຸຍາດຄູຂອງ '{teacher_name}' ຖືກຖອນ: {reason} (ມາດຕາ 60)"
    )]
    TeacherLicenseRevoked {
        teacher_name: String,
        reason: String,
    },

    /// Teacher has no license (Article 60)
    /// ຄູບໍ່ມີໃບອະນຸຍາດ (ມາດຕາ 60)
    #[error(
        "Teacher '{teacher_name}' has no valid license (Article 60)\nຄູ '{teacher_name}' ບໍ່ມີໃບອະນຸຍາດທີ່ຖືກຕ້ອງ (ມາດຕາ 60)"
    )]
    TeacherNoLicense { teacher_name: String },

    /// Insufficient qualified teachers (Article 47)
    /// ຄູທີ່ມີຄຸນວຸດທິບໍ່ພຽງພໍ (ມາດຕາ 47)
    #[error(
        "Institution has insufficient qualified teachers: {qualified} out of {total} ({percentage:.1}%) (Article 47)\nສະຖາບັນມີຄູທີ່ມີຄຸນວຸດທິບໍ່ພຽງພໍ: {qualified} ຈາກ {total} ({percentage:.1}%) (ມາດຕາ 47)"
    )]
    InsufficientQualifiedTeachers {
        qualified: u32,
        total: u32,
        percentage: f64,
    },

    // ========================================================================
    // Compulsory Education Errors (ຄວາມຜິດພາດການສຶກສາບັງຄັບ)
    // ========================================================================
    /// Child not enrolled in compulsory education (Article 26)
    /// ເດັກບໍ່ໄດ້ລົງທະບຽນໃນການສຶກສາບັງຄັບ (ມາດຕາ 26)
    #[error(
        "Child '{child_name}' aged {age} is not enrolled in compulsory education (Article 26)\nເດັກ '{child_name}' ອາຍຸ {age} ປີ ບໍ່ໄດ້ລົງທະບຽນໃນການສຶກສາບັງຄັບ (ມາດຕາ 26)"
    )]
    ChildNotEnrolled { child_name: String, age: u8 },

    /// Child dropped out of school (Article 26)
    /// ເດັກອອກໂຮງຮຽນກາງຄັນ (ມາດຕາ 26)
    #[error(
        "Child '{child_name}' dropped out of compulsory education: {reason} (Article 26)\nເດັກ '{child_name}' ອອກການສຶກສາບັງຄັບກາງຄັນ: {reason} (ມາດຕາ 26)"
    )]
    ChildDropout { child_name: String, reason: String },

    /// Child is beyond compulsory age without completion
    /// ເດັກເກີນອາຍຸການສຶກສາບັງຄັບແຕ່ບໍ່ສຳເລັດ
    #[error(
        "Child '{child_name}' aged {age} has not completed compulsory education (completed grade {completed_grade}/9)\nເດັກ '{child_name}' ອາຍຸ {age} ປີ ບໍ່ໄດ້ສຳເລັດການສຶກສາບັງຄັບ (ຮຽນແລ້ວຊັ້ນ {completed_grade}/9)"
    )]
    CompulsoryEducationIncomplete {
        child_name: String,
        age: u8,
        completed_grade: u8,
    },

    /// Child suspended from school (Article 78)
    /// ເດັກຖືກພັກການຮຽນ (ມາດຕາ 78)
    #[error(
        "Child '{child_name}' suspended from school: {reason}\nເດັກ '{child_name}' ຖືກພັກການຮຽນ: {reason}"
    )]
    ChildSuspended { child_name: String, reason: String },

    /// Enrollment age not reached
    /// ອາຍຸເຂົ້າຮຽນບໍ່ເຖິງ
    #[error(
        "Child '{child_name}' has not reached enrollment age: current age {current_age}, required {required_age}\nເດັກ '{child_name}' ອາຍຸບໍ່ເຖິງການລົງທະບຽນ: ອາຍຸປັດຈຸບັນ {current_age}, ຕ້ອງການ {required_age}"
    )]
    EnrollmentAgeNotReached {
        child_name: String,
        current_age: u8,
        required_age: u8,
    },

    // ========================================================================
    // Student Rights Errors (ຄວາມຜິດພາດສິດນັກຮຽນ)
    // ========================================================================
    /// Discrimination violation (Article 72)
    /// ການລະເມີດການຈຳແນກ (ມາດຕາ 72)
    #[error(
        "Discrimination against student: {description} (Article 72)\nການຈຳແນກຕໍ່ນັກຮຽນ: {description} (ມາດຕາ 72)"
    )]
    Discrimination { description: String },

    /// Unsafe learning environment (Article 75)
    /// ສະພາບແວດລ້ອມການຮຽນບໍ່ປອດໄພ (ມາດຕາ 75)
    #[error(
        "Unsafe learning environment: {hazard} (Article 75)\nສະພາບແວດລ້ອມການຮຽນບໍ່ປອດໄພ: {hazard} (ມາດຕາ 75)"
    )]
    UnsafeEnvironment { hazard: String },

    /// Student rights violation (Article 72-79)
    /// ການລະເມີດສິດນັກຮຽນ (ມາດຕາ 72-79)
    #[error(
        "Student rights violation ({right_type}): {description} (Article {article})\nການລະເມີດສິດນັກຮຽນ ({right_type}): {description} (ມາດຕາ {article})"
    )]
    StudentRightsViolation {
        right_type: String,
        description: String,
        article: u16,
    },

    /// Privacy violation (Article 77)
    /// ການລະເມີດຄວາມເປັນສ່ວນຕົວ (ມາດຕາ 77)
    #[error(
        "Privacy violation: {description} (Article 77)\nການລະເມີດຄວາມເປັນສ່ວນຕົວ: {description} (ມາດຕາ 77)"
    )]
    PrivacyViolation { description: String },

    /// Denial of special needs accommodation (Article 22)
    /// ການປະຕິເສດການອຳນວຍຄວາມສະດວກສຳລັບຄວາມຕ້ອງການພິເສດ (ມາດຕາ 22)
    #[error(
        "Special needs accommodation denied: {description} (Article 22)\nການອຳນວຍຄວາມສະດວກສຳລັບຄວາມຕ້ອງການພິເສດຖືກປະຕິເສດ: {description} (ມາດຕາ 22)"
    )]
    SpecialNeedsAccommodationDenied { description: String },

    /// Mother tongue education denied (Article 73)
    /// ການສຶກສາພາສາແມ່ຖືກປະຕິເສດ (ມາດຕາ 73)
    #[error(
        "Mother tongue education denied for ethnic group '{ethnic_group}' (Article 73)\nການສຶກສາພາສາແມ່ຖືກປະຕິເສດສຳລັບຊົນເຜົ່າ '{ethnic_group}' (ມາດຕາ 73)"
    )]
    MotherTongueEducationDenied { ethnic_group: String },

    // ========================================================================
    // Age and Level Errors (ຄວາມຜິດພາດອາຍຸແລະລະດັບ)
    // ========================================================================
    /// Student age inappropriate for level
    /// ອາຍຸນັກຮຽນບໍ່ເໝາະສົມກັບລະດັບ
    #[error(
        "Student age {age} is inappropriate for {level}: expected age range {min_age}-{max_age}\nອາຍຸນັກຮຽນ {age} ບໍ່ເໝາະສົມກັບ {level}: ຊ່ວງອາຍຸທີ່ຄາດໝາຍ {min_age}-{max_age}"
    )]
    InappropriateAgeForLevel {
        age: u8,
        level: String,
        min_age: u8,
        max_age: u8,
    },

    /// Student below minimum age (Article 25-27)
    /// ນັກຮຽນຕ່ຳກວ່າອາຍຸຂັ້ນຕ່ຳ (ມາດຕາ 25-27)
    #[error(
        "Student age {age} is below minimum age {min_age} for {level}\nອາຍຸນັກຮຽນ {age} ຕ່ຳກວ່າອາຍຸຂັ້ນຕ່ຳ {min_age} ສຳລັບ {level}"
    )]
    BelowMinimumAge { age: u8, min_age: u8, level: String },

    /// Student above maximum age
    /// ນັກຮຽນເກີນອາຍຸສູງສຸດ
    #[error(
        "Student age {age} is above maximum age {max_age} for {level}\nອາຍຸນັກຮຽນ {age} ເກີນອາຍຸສູງສຸດ {max_age} ສຳລັບ {level}"
    )]
    AboveMaximumAge { age: u8, max_age: u8, level: String },

    // ========================================================================
    // Scholarship Errors (ຄວາມຜິດພາດທຶນການສຶກສາ)
    // ========================================================================
    /// Scholarship eligibility not met
    /// ບໍ່ມີສິດຮັບທຶນການສຶກສາ
    #[error(
        "Scholarship eligibility not met for '{scholarship_name}': {reason}\nບໍ່ມີສິດຮັບທຶນ '{scholarship_name}': {reason}"
    )]
    ScholarshipEligibilityNotMet {
        scholarship_name: String,
        reason: String,
    },

    /// GPA below minimum requirement
    /// ຄະແນນສະເລ່ຍຕ່ຳກວ່າຂັ້ນຕ່ຳ
    #[error(
        "GPA {actual} is below minimum requirement {required} for scholarship\nຄະແນນສະເລ່ຍ {actual} ຕ່ຳກວ່າຂັ້ນຕ່ຳ {required} ສຳລັບທຶນການສຶກສາ"
    )]
    GPABelowMinimum { actual: f64, required: f64 },

    /// Income above maximum for scholarship
    /// ລາຍໄດ້ເກີນເກນສູງສຸດສຳລັບທຶນການສຶກສາ
    #[error(
        "Family income {actual_lak} LAK exceeds maximum {max_lak} LAK for scholarship\nລາຍໄດ້ຄອບຄົວ {actual_lak} ກີບ ເກີນເກນສູງສຸດ {max_lak} ກີບ ສຳລັບທຶນການສຶກສາ"
    )]
    IncomeAboveMaximum { actual_lak: u64, max_lak: u64 },

    // ========================================================================
    // Curriculum Errors (ຄວາມຜິດພາດຫຼັກສູດ)
    // ========================================================================
    /// Curriculum not compliant with national standards (Article 16)
    /// ຫຼັກສູດບໍ່ສອດຄ່ອງກັບມາດຕະຖານແຫ່ງຊາດ (ມາດຕາ 16)
    #[error(
        "Curriculum does not comply with national standards: {violation} (Article 16)\nຫຼັກສູດບໍ່ສອດຄ່ອງກັບມາດຕະຖານແຫ່ງຊາດ: {violation} (ມາດຕາ 16)"
    )]
    CurriculumNonCompliant { violation: String },

    /// Missing required subject
    /// ຂາດວິຊາບັງຄັບ
    #[error("Missing required subject: {subject_name}\nຂາດວິຊາບັງຄັບ: {subject_name}")]
    MissingRequiredSubject { subject_name: String },

    /// Insufficient instructional hours
    /// ຊົ່ວໂມງສອນບໍ່ພຽງພໍ
    #[error(
        "Instructional hours {actual} are below required {required} for {subject}\nຊົ່ວໂມງສອນ {actual} ຕ່ຳກວ່າທີ່ຕ້ອງການ {required} ສຳລັບ {subject}"
    )]
    InsufficientInstructionalHours {
        actual: u32,
        required: u32,
        subject: String,
    },

    /// Program duration invalid
    /// ໄລຍະຫຼັກສູດບໍ່ຖືກຕ້ອງ
    #[error(
        "Program duration {actual_years} years is invalid for {level}: expected {expected_years} years\nໄລຍະຫຼັກສູດ {actual_years} ປີ ບໍ່ຖືກຕ້ອງສຳລັບ {level}: ຄາດໝາຍ {expected_years} ປີ"
    )]
    InvalidProgramDuration {
        actual_years: f32,
        expected_years: u8,
        level: String,
    },

    /// Insufficient credits
    /// ໜ່ວຍກິດບໍ່ພຽງພໍ
    #[error(
        "Credits {actual} are below minimum {required} for {degree_type}\nໜ່ວຍກິດ {actual} ຕ່ຳກວ່າຂັ້ນຕ່ຳ {required} ສຳລັບ {degree_type}"
    )]
    InsufficientCredits {
        actual: u16,
        required: u16,
        degree_type: String,
    },

    // ========================================================================
    // Program Errors (ຄວາມຜິດພາດຫຼັກສູດ)
    // ========================================================================
    /// Program not active
    /// ຫຼັກສູດບໍ່ເປີດສອນ
    #[error(
        "Program '{program_name}' is not currently active\nຫຼັກສູດ '{program_name}' ບໍ່ເປີດສອນໃນປັດຈຸບັນ"
    )]
    ProgramNotActive { program_name: String },

    /// Entry requirements not met
    /// ເງື່ອນໄຂເຂົ້າຮຽນບໍ່ບັນລຸ
    #[error(
        "Entry requirements not met for '{program_name}': {requirement}\nເງື່ອນໄຂເຂົ້າຮຽນບໍ່ບັນລຸສຳລັບ '{program_name}': {requirement}"
    )]
    EntryRequirementsNotMet {
        program_name: String,
        requirement: String,
    },

    /// Program enrollment full
    /// ຫຼັກສູດເຕັມແລ້ວ
    #[error(
        "Program '{program_name}' enrollment is full: {current}/{max}\nຫຼັກສູດ '{program_name}' ເຕັມແລ້ວ: {current}/{max}"
    )]
    ProgramEnrollmentFull {
        program_name: String,
        current: u32,
        max: u32,
    },

    // ========================================================================
    // General Errors (ຄວາມຜິດພາດທົ່ວໄປ)
    // ========================================================================
    /// Validation error
    /// ຄວາມຜິດພາດການກວດສອບ
    #[error("Validation error: {message}\nຄວາມຜິດພາດການກວດສອບ: {message}")]
    ValidationError { message: String },

    /// Missing required field
    /// ຂາດຊ່ອງຂໍ້ມູນທີ່ຈຳເປັນ
    #[error("Missing required field: {field_name}\nຂາດຊ່ອງຂໍ້ມູນທີ່ຈຳເປັນ: {field_name}")]
    MissingRequiredField { field_name: String },

    /// Invalid date format
    /// ຮູບແບບວັນທີບໍ່ຖືກຕ້ອງ
    #[error("Invalid date format: {date}\nຮູບແບບວັນທີບໍ່ຖືກຕ້ອງ: {date}")]
    InvalidDateFormat { date: String },

    /// General education law violation
    /// ການລະເມີດກົດໝາຍການສຶກສາ
    #[error(
        "Education law violation: {violation} (Article {article})\nການລະເມີດກົດໝາຍການສຶກສາ: {violation} (ມາດຕາ {article})"
    )]
    EducationLawViolation { violation: String, article: u16 },

    /// Institution type mismatch
    /// ປະເພດສະຖາບັນບໍ່ກົງກັນ
    #[error(
        "Institution type '{institution_type}' cannot offer '{program_level}' education\nປະເພດສະຖາບັນ '{institution_type}' ບໍ່ສາມາດສອນ '{program_level}'"
    )]
    InstitutionTypeMismatch {
        institution_type: String,
        program_level: String,
    },

    /// Class size exceeded
    /// ເກີນຈຳນວນນັກຮຽນຕໍ່ຫ້ອງ
    #[error(
        "Class size {actual} exceeds maximum of {max} students per class\nຈຳນວນນັກຮຽນຕໍ່ຫ້ອງ {actual} ເກີນສູງສຸດ {max} ຄົນ"
    )]
    ClassSizeExceeded { actual: u32, max: u32 },

    /// Teacher-student ratio exceeded
    /// ອັດຕາສ່ວນຄູຕໍ່ນັກຮຽນເກີນ
    #[error(
        "Teacher-student ratio {actual:.1}:1 exceeds maximum of {max:.1}:1\nອັດຕາສ່ວນຄູຕໍ່ນັກຮຽນ {actual:.1}:1 ເກີນສູງສຸດ {max:.1}:1"
    )]
    TeacherStudentRatioExceeded { actual: f64, max: f64 },
}

impl EducationLawError {
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
            EducationLawError::InstitutionUnlicensed { .. }
                | EducationLawError::ChildNotEnrolled { .. }
                | EducationLawError::ChildDropout { .. }
                | EducationLawError::Discrimination { .. }
                | EducationLawError::UnsafeEnvironment { .. }
                | EducationLawError::TeacherLicenseRevoked { .. }
                | EducationLawError::AccreditationRevoked { .. }
        )
    }

    /// Check if this error relates to student welfare
    /// ກວດສອບວ່າຄວາມຜິດພາດນີ້ກ່ຽວຂ້ອງກັບສະຫວັດດີການນັກຮຽນ
    pub fn is_student_welfare_related(&self) -> bool {
        matches!(
            self,
            EducationLawError::ChildNotEnrolled { .. }
                | EducationLawError::ChildDropout { .. }
                | EducationLawError::ChildSuspended { .. }
                | EducationLawError::Discrimination { .. }
                | EducationLawError::UnsafeEnvironment { .. }
                | EducationLawError::StudentRightsViolation { .. }
                | EducationLawError::PrivacyViolation { .. }
                | EducationLawError::SpecialNeedsAccommodationDenied { .. }
                | EducationLawError::MotherTongueEducationDenied { .. }
        )
    }

    /// Get the article number referenced in this error, if any
    /// ຮັບເລກມາດຕາທີ່ອ້າງອິງໃນຄວາມຜິດພາດນີ້
    pub fn article_number(&self) -> Option<u16> {
        match self {
            EducationLawError::InstitutionUnlicensed { .. } => Some(42),
            EducationLawError::InstitutionLicenseExpired { .. } => Some(42),
            EducationLawError::InstitutionLicenseInvalid { .. } => Some(42),
            EducationLawError::CapacityExceeded { .. } => Some(47),
            EducationLawError::ProgramNotAccredited { .. } => Some(55),
            EducationLawError::AccreditationExpired { .. } => Some(55),
            EducationLawError::AccreditationRevoked { .. } => Some(55),
            EducationLawError::InstitutionNotAccredited { .. } => Some(55),
            EducationLawError::TeacherUnqualified { .. } => Some(58),
            EducationLawError::TeacherLicenseExpired { .. } => Some(60),
            EducationLawError::TeacherLicenseRevoked { .. } => Some(60),
            EducationLawError::TeacherNoLicense { .. } => Some(60),
            EducationLawError::InsufficientQualifiedTeachers { .. } => Some(47),
            EducationLawError::ChildNotEnrolled { .. } => Some(26),
            EducationLawError::ChildDropout { .. } => Some(26),
            EducationLawError::ChildSuspended { .. } => Some(78),
            EducationLawError::Discrimination { .. } => Some(72),
            EducationLawError::UnsafeEnvironment { .. } => Some(75),
            EducationLawError::StudentRightsViolation { article, .. } => Some(*article),
            EducationLawError::PrivacyViolation { .. } => Some(77),
            EducationLawError::SpecialNeedsAccommodationDenied { .. } => Some(22),
            EducationLawError::MotherTongueEducationDenied { .. } => Some(73),
            EducationLawError::CurriculumNonCompliant { .. } => Some(16),
            EducationLawError::EducationLawViolation { article, .. } => Some(*article),
            _ => None,
        }
    }

    /// Get the error category
    /// ຮັບໝວດໝູ່ຄວາມຜິດພາດ
    pub fn category(&self) -> ErrorCategory {
        match self {
            EducationLawError::InstitutionUnlicensed { .. }
            | EducationLawError::InstitutionLicenseExpired { .. }
            | EducationLawError::InstitutionLicenseInvalid { .. }
            | EducationLawError::MissingLicenseNumber
            | EducationLawError::InvalidLicenseFormat { .. } => ErrorCategory::InstitutionLicense,

            EducationLawError::ProgramNotAccredited { .. }
            | EducationLawError::AccreditationExpired { .. }
            | EducationLawError::AccreditationRevoked { .. }
            | EducationLawError::ProvisionalConditionsNotMet { .. }
            | EducationLawError::InstitutionNotAccredited { .. } => ErrorCategory::Accreditation,

            EducationLawError::TeacherUnqualified { .. }
            | EducationLawError::TeacherLicenseExpired { .. }
            | EducationLawError::TeacherLicenseRevoked { .. }
            | EducationLawError::TeacherNoLicense { .. }
            | EducationLawError::InsufficientQualifiedTeachers { .. } => {
                ErrorCategory::TeacherQualification
            }

            EducationLawError::ChildNotEnrolled { .. }
            | EducationLawError::ChildDropout { .. }
            | EducationLawError::CompulsoryEducationIncomplete { .. }
            | EducationLawError::ChildSuspended { .. }
            | EducationLawError::EnrollmentAgeNotReached { .. } => {
                ErrorCategory::CompulsoryEducation
            }

            EducationLawError::Discrimination { .. }
            | EducationLawError::UnsafeEnvironment { .. }
            | EducationLawError::StudentRightsViolation { .. }
            | EducationLawError::PrivacyViolation { .. }
            | EducationLawError::SpecialNeedsAccommodationDenied { .. }
            | EducationLawError::MotherTongueEducationDenied { .. } => ErrorCategory::StudentRights,

            EducationLawError::ScholarshipEligibilityNotMet { .. }
            | EducationLawError::GPABelowMinimum { .. }
            | EducationLawError::IncomeAboveMaximum { .. } => ErrorCategory::Scholarship,

            EducationLawError::CurriculumNonCompliant { .. }
            | EducationLawError::MissingRequiredSubject { .. }
            | EducationLawError::InsufficientInstructionalHours { .. }
            | EducationLawError::InvalidProgramDuration { .. }
            | EducationLawError::InsufficientCredits { .. } => ErrorCategory::Curriculum,

            _ => ErrorCategory::General,
        }
    }
}

/// Error category (ໝວດໝູ່ຄວາມຜິດພາດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Institution license errors (ຄວາມຜິດພາດໃບອະນຸຍາດສະຖາບັນ)
    InstitutionLicense,
    /// Accreditation errors (ຄວາມຜິດພາດການຮັບຮອງ)
    Accreditation,
    /// Teacher qualification errors (ຄວາມຜິດພາດຄຸນວຸດທິຄູ)
    TeacherQualification,
    /// Compulsory education errors (ຄວາມຜິດພາດການສຶກສາບັງຄັບ)
    CompulsoryEducation,
    /// Student rights errors (ຄວາມຜິດພາດສິດນັກຮຽນ)
    StudentRights,
    /// Scholarship errors (ຄວາມຜິດພາດທຶນການສຶກສາ)
    Scholarship,
    /// Curriculum errors (ຄວາມຜິດພາດຫຼັກສູດ)
    Curriculum,
    /// General errors (ຄວາມຜິດພາດທົ່ວໄປ)
    General,
}

impl std::fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCategory::InstitutionLicense => write!(f, "Institution License"),
            ErrorCategory::Accreditation => write!(f, "Accreditation"),
            ErrorCategory::TeacherQualification => write!(f, "Teacher Qualification"),
            ErrorCategory::CompulsoryEducation => write!(f, "Compulsory Education"),
            ErrorCategory::StudentRights => write!(f, "Student Rights"),
            ErrorCategory::Scholarship => write!(f, "Scholarship"),
            ErrorCategory::Curriculum => write!(f, "Curriculum"),
            ErrorCategory::General => write!(f, "General"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilingual_error_messages() {
        let error = EducationLawError::ChildNotEnrolled {
            child_name: "ສົມຊາຍ".to_string(),
            age: 7,
        };

        let english = error.english_message();
        let lao = error.lao_message();

        assert!(english.contains("not enrolled"));
        assert!(lao.contains("ບໍ່ໄດ້ລົງທະບຽນ"));
    }

    #[test]
    fn test_critical_violations() {
        let child_not_enrolled = EducationLawError::ChildNotEnrolled {
            child_name: "Test".to_string(),
            age: 7,
        };
        assert!(child_not_enrolled.is_critical());

        let gpa_low = EducationLawError::GPABelowMinimum {
            actual: 2.5,
            required: 3.0,
        };
        assert!(!gpa_low.is_critical());
    }

    #[test]
    fn test_student_welfare_related() {
        let discrimination = EducationLawError::Discrimination {
            description: "Gender discrimination".to_string(),
        };
        assert!(discrimination.is_student_welfare_related());

        let license_expired = EducationLawError::InstitutionLicenseExpired {
            expiry_date: "2024-01-01".to_string(),
        };
        assert!(!license_expired.is_student_welfare_related());
    }

    #[test]
    fn test_article_numbers() {
        let error = EducationLawError::ChildNotEnrolled {
            child_name: "Test".to_string(),
            age: 7,
        };
        assert_eq!(error.article_number(), Some(26));

        let error = EducationLawError::InstitutionUnlicensed {
            institution_name: "Test School".to_string(),
        };
        assert_eq!(error.article_number(), Some(42));
    }

    #[test]
    fn test_error_category() {
        let error = EducationLawError::TeacherUnqualified {
            teacher_name: "Test".to_string(),
            level: "Secondary".to_string(),
            actual: "Diploma".to_string(),
            required: "Bachelor".to_string(),
        };
        assert_eq!(error.category(), ErrorCategory::TeacherQualification);

        let error = EducationLawError::Discrimination {
            description: "Test".to_string(),
        };
        assert_eq!(error.category(), ErrorCategory::StudentRights);
    }
}
