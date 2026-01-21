//! Education Law Types (ກົດໝາຍການສຶກສາ)
//!
//! Type definitions for Lao education law based on:
//! - **Education Law 2015** (Law No. 62/NA, dated July 16, 2015)
//! - National Assembly of the Lao PDR
//!
//! # Legal References
//! - Education Law 2015 (Law No. 62/NA) - ກົດໝາຍວ່າດ້ວຍການສຶກສາ ປີ 2015
//! - National Education System Decree - ດຳລັດວ່າດ້ວຍລະບົບການສຶກສາແຫ່ງຊາດ
//! - Teacher Professional Standards - ມາດຕະຖານວິຊາຊີບຄູ
//!
//! # Bilingual Support
//! All types include both Lao (ລາວ) and English field names where applicable.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Education Law 2015 (ກົດໝາຍການສຶກສາ ປີ 2015)
// ============================================================================

/// Compulsory education starting age (Article 26)
/// ອາຍຸເລີ່ມການສຶກສາບັງຄັບ - 6 ປີ
pub const COMPULSORY_EDUCATION_START_AGE: u8 = 6;

/// Compulsory education ending age (Article 26)
/// ອາຍຸສິ້ນສຸດການສຶກສາບັງຄັບ - 14 ປີ
pub const COMPULSORY_EDUCATION_END_AGE: u8 = 14;

/// Compulsory education duration in years (Article 26)
/// ໄລຍະການສຶກສາບັງຄັບ - 9 ປີ (ປະຖົມ 5 ປີ + ມັດທະຍົມຕົ້ນ 4 ປີ)
pub const COMPULSORY_EDUCATION_YEARS: u8 = 9;

/// Primary education duration (Article 27)
/// ໄລຍະການສຶກສາປະຖົມ - 5 ປີ
pub const PRIMARY_EDUCATION_YEARS: u8 = 5;

/// Lower secondary education duration (Article 28)
/// ໄລຍະການສຶກສາມັດທະຍົມຕົ້ນ - 4 ປີ
pub const LOWER_SECONDARY_YEARS: u8 = 4;

/// Upper secondary education duration (Article 29)
/// ໄລຍະການສຶກສາມັດທະຍົມປາຍ - 3 ປີ
pub const UPPER_SECONDARY_YEARS: u8 = 3;

/// Pre-primary education minimum age (Article 25)
/// ອາຍຸຂັ້ນຕ່ຳການສຶກສາກ່ອນປະຖົມ - 3 ປີ
pub const PRE_PRIMARY_MIN_AGE: u8 = 3;

/// Pre-primary education maximum age (Article 25)
/// ອາຍຸສູງສຸດການສຶກສາກ່ອນປະຖົມ - 5 ປີ
pub const PRE_PRIMARY_MAX_AGE: u8 = 5;

/// Academic year start month (September)
/// ເດືອນເລີ່ມປີການສຶກສາ - ກັນຍາ
pub const ACADEMIC_YEAR_START_MONTH: u8 = 9;

/// Academic year end month (June)
/// ເດືອນສິ້ນສຸດປີການສຶກສາ - ມິຖຸນາ
pub const ACADEMIC_YEAR_END_MONTH: u8 = 6;

/// Minimum teacher qualification for primary level
/// ຄຸນວຸດທິຂັ້ນຕ່ຳຄູປະຖົມ - ອະນຸປະລິນຍາ
pub const MIN_TEACHER_QUALIFICATION_PRIMARY: &str = "Diploma";

/// Minimum teacher qualification for secondary level
/// ຄຸນວຸດທິຂັ້ນຕ່ຳຄູມັດທະຍົມ - ປະລິນຍາຕີ
pub const MIN_TEACHER_QUALIFICATION_SECONDARY: &str = "Bachelor";

/// Minimum credits for bachelor degree
/// ໜ່ວຍກິດຂັ້ນຕ່ຳປະລິນຍາຕີ - 120 ໜ່ວຍກິດ
pub const MIN_CREDITS_BACHELOR: u16 = 120;

/// Minimum credits for master degree
/// ໜ່ວຍກິດຂັ້ນຕ່ຳປະລິນຍາໂທ - 36 ໜ່ວຍກິດ
pub const MIN_CREDITS_MASTER: u16 = 36;

/// Maximum class size for primary education
/// ຈຳນວນນັກຮຽນສູງສຸດຕໍ່ຫ້ອງປະຖົມ - 45 ຄົນ
pub const MAX_CLASS_SIZE_PRIMARY: u32 = 45;

/// Maximum class size for secondary education
/// ຈຳນວນນັກຮຽນສູງສຸດຕໍ່ຫ້ອງມັດທະຍົມ - 45 ຄົນ
pub const MAX_CLASS_SIZE_SECONDARY: u32 = 45;

/// License validity period in years
/// ໄລຍະໃບອະນຸຍາດ - 5 ປີ
pub const LICENSE_VALIDITY_YEARS: u8 = 5;

/// Accreditation validity period in years
/// ໄລຍະການຮັບຮອງຄຸນນະພາບ - 5 ປີ
pub const ACCREDITATION_VALIDITY_YEARS: u8 = 5;

// ============================================================================
// Education Levels (ລະດັບການສຶກສາ)
// ============================================================================

/// Education level types (ປະເພດລະດັບການສຶກສາ)
///
/// Article 23-35: Classification of education levels in Lao PDR
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EducationLevel {
    /// Pre-primary education (ການສຶກສາກ່ອນປະຖົມ)
    /// Article 25: For children aged 3-5 years
    PrePrimary {
        /// Age range for pre-primary (start, end)
        age_range: (u8, u8),
    },

    /// Primary education (ການສຶກສາປະຖົມ)
    /// Article 27: Grades 1-5, ages 6-10 years
    Primary {
        /// Number of grades (typically 5)
        grades: u8,
    },

    /// Lower secondary education (ການສຶກສາມັດທະຍົມຕົ້ນ)
    /// Article 28: Grades 6-9, ages 11-14 years
    LowerSecondary {
        /// Number of grades (typically 4)
        grades: u8,
    },

    /// Upper secondary education (ການສຶກສາມັດທະຍົມປາຍ)
    /// Article 29: Grades 10-12, ages 15-17 years
    UpperSecondary {
        /// Number of grades (typically 3)
        grades: u8,
    },

    /// Technical and vocational education (ການສຶກສາວິຊາຊີບ)
    /// Article 30: Technical and vocational training
    TechnicalVocational {
        /// Program duration in years
        years: u8,
    },

    /// Higher education (ການສຶກສາຊັ້ນສູງ)
    /// Article 31-35: University and college education
    HigherEducation {
        /// Type of degree program
        degree_type: DegreeType,
    },

    /// Non-formal education (ການສຶກສານອກລະບົບ)
    /// Article 36: Education outside the formal system
    NonFormal,

    /// Special education (ການສຶກສາພິເສດ)
    /// Article 22: Education for students with special needs
    SpecialEducation {
        /// Type of special education
        special_type: SpecialEducationType,
    },
}

impl EducationLevel {
    /// Get the standard duration for this education level
    /// ຮັບໄລຍະມາດຕະຖານຂອງລະດັບການສຶກສານີ້
    pub fn standard_duration_years(&self) -> Option<u8> {
        match self {
            EducationLevel::PrePrimary { .. } => Some(3),
            EducationLevel::Primary { grades } => Some(*grades),
            EducationLevel::LowerSecondary { grades } => Some(*grades),
            EducationLevel::UpperSecondary { grades } => Some(*grades),
            EducationLevel::TechnicalVocational { years } => Some(*years),
            EducationLevel::HigherEducation { degree_type } => {
                degree_type.standard_duration_years()
            }
            EducationLevel::NonFormal => None,
            EducationLevel::SpecialEducation { .. } => None,
        }
    }

    /// Check if this level is part of compulsory education
    /// ກວດສອບວ່າລະດັບນີ້ເປັນສ່ວນໜຶ່ງຂອງການສຶກສາບັງຄັບ
    pub fn is_compulsory(&self) -> bool {
        matches!(
            self,
            EducationLevel::Primary { .. } | EducationLevel::LowerSecondary { .. }
        )
    }

    /// Get the typical age range for this education level
    /// ຮັບຊ່ວງອາຍຸປົກກະຕິສຳລັບລະດັບການສຶກສານີ້
    pub fn typical_age_range(&self) -> Option<(u8, u8)> {
        match self {
            EducationLevel::PrePrimary { age_range } => Some(*age_range),
            EducationLevel::Primary { .. } => Some((6, 10)),
            EducationLevel::LowerSecondary { .. } => Some((11, 14)),
            EducationLevel::UpperSecondary { .. } => Some((15, 17)),
            EducationLevel::TechnicalVocational { .. } => Some((15, 22)),
            EducationLevel::HigherEducation { .. } => Some((18, 30)),
            _ => None,
        }
    }
}

/// Special education types (ປະເພດການສຶກສາພິເສດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SpecialEducationType {
    /// Visual impairment (ຄວາມບົກຜ່ອງທາງການເບິ່ງ)
    VisualImpairment,
    /// Hearing impairment (ຄວາມບົກຜ່ອງທາງການໄດ້ຍິນ)
    HearingImpairment,
    /// Physical disability (ຄວາມພິການທາງຮ່າງກາຍ)
    PhysicalDisability,
    /// Intellectual disability (ຄວາມພິການທາງສະຕິປັນຍາ)
    IntellectualDisability,
    /// Learning disability (ຄວາມບົກຜ່ອງໃນການຮຽນຮູ້)
    LearningDisability,
    /// Multiple disabilities (ຄວາມພິການຫຼາຍປະເພດ)
    MultipleDisabilities,
    /// Gifted education (ການສຶກສາສຳລັບເດັກມີພອນສະຫວັນ)
    Gifted,
}

// ============================================================================
// Degree Types (ປະເພດລະດັບປະລິນຍາ)
// ============================================================================

/// Degree types in higher education (ປະເພດລະດັບປະລິນຍາ)
///
/// Article 31-35: Higher education qualifications
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DegreeType {
    /// Associate degree (ອະນຸປະລິນຍາ)
    /// 2-year program, typically 60 credits
    AssociateDegree,

    /// Bachelor's degree (ປະລິນຍາຕີ)
    /// 4-year program, minimum 120 credits
    BachelorDegree,

    /// Master's degree (ປະລິນຍາໂທ)
    /// 2-year program, minimum 36 credits
    MasterDegree,

    /// Doctorate degree (ປະລິນຍາເອກ)
    /// 3+ year program with dissertation
    Doctorate,

    /// Diploma (ອະນຸປະລິນຍາ/ໃບປະກາດ)
    /// Short-term professional qualification
    Diploma,

    /// Certificate (ໃບຢັ້ງຢືນ)
    /// Short-term training certification
    Certificate,

    /// Professional degree (ປະລິນຍາວິຊາຊີບ)
    /// Professional qualifications (medicine, law, etc.)
    ProfessionalDegree {
        /// Field of professional study
        field: String,
    },
}

impl DegreeType {
    /// Get the standard duration for this degree type
    /// ຮັບໄລຍະມາດຕະຖານຂອງປະເພດປະລິນຍານີ້
    pub fn standard_duration_years(&self) -> Option<u8> {
        match self {
            DegreeType::AssociateDegree => Some(2),
            DegreeType::BachelorDegree => Some(4),
            DegreeType::MasterDegree => Some(2),
            DegreeType::Doctorate => Some(3),
            DegreeType::Diploma => Some(2),
            DegreeType::Certificate => Some(1),
            DegreeType::ProfessionalDegree { .. } => Some(5),
        }
    }

    /// Get the minimum credits required
    /// ຮັບໜ່ວຍກິດຂັ້ນຕ່ຳທີ່ຕ້ອງການ
    pub fn minimum_credits(&self) -> Option<u16> {
        match self {
            DegreeType::AssociateDegree => Some(60),
            DegreeType::BachelorDegree => Some(120),
            DegreeType::MasterDegree => Some(36),
            DegreeType::Doctorate => Some(48),
            DegreeType::Diploma => Some(30),
            DegreeType::Certificate => Some(15),
            DegreeType::ProfessionalDegree { .. } => Some(150),
        }
    }
}

// ============================================================================
// Educational Institutions (ສະຖາບັນການສຶກສາ)
// ============================================================================

/// Educational institution (ສະຖາບັນການສຶກສາ)
///
/// Article 42-54: Types and requirements for educational institutions
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EducationalInstitution {
    /// Institution name in Lao (ຊື່ສະຖາບັນເປັນພາສາລາວ)
    pub name_lao: String,

    /// Institution name in English (ຊື່ສະຖາບັນເປັນພາສາອັງກິດ)
    pub name_en: String,

    /// Type of institution (ປະເພດສະຖາບັນ)
    pub institution_type: InstitutionType,

    /// Ownership type (ປະເພດການເປັນເຈົ້າຂອງ)
    pub ownership_type: OwnershipType,

    /// License number (ເລກທະບຽນໃບອະນຸຍາດ)
    pub license_number: String,

    /// License issue date (ວັນທີອອກໃບອະນຸຍາດ)
    pub license_issue_date: String,

    /// License expiry date (ວັນທີໝົດອາຍຸໃບອະນຸຍາດ)
    pub license_expiry_date: String,

    /// Province location (ແຂວງທີ່ຕັ້ງ)
    pub province: String,

    /// District location (ເມືອງທີ່ຕັ້ງ)
    pub district: String,

    /// Full address (ທີ່ຢູ່ເຕັມ)
    pub address: String,

    /// Accreditation status (ສະຖານະການຮັບຮອງຄຸນນະພາບ)
    pub accreditation_status: AccreditationStatus,

    /// Student capacity (ຄວາມຈຸນັກຮຽນ)
    pub student_capacity: u32,

    /// Current enrollment (ຈຳນວນນັກຮຽນປັດຈຸບັນ)
    pub current_enrollment: u32,

    /// Programs offered (ຫຼັກສູດທີ່ສອນ)
    pub programs_offered: Vec<EducationProgram>,

    /// Number of teachers (ຈຳນວນຄູ)
    pub teacher_count: u32,

    /// Number of qualified teachers (ຈຳນວນຄູທີ່ມີຄຸນວຸດທິ)
    pub qualified_teacher_count: u32,

    /// Contact information (ຂໍ້ມູນຕິດຕໍ່)
    pub contact_phone: Option<String>,

    /// Email address (ອີເມວ)
    pub contact_email: Option<String>,

    /// Website (ເວັບໄຊ)
    pub website: Option<String>,

    /// Year established (ປີສ້າງຕັ້ງ)
    pub established_year: u16,
}

impl EducationalInstitution {
    /// Check if the institution has valid license
    /// ກວດສອບວ່າສະຖາບັນມີໃບອະນຸຍາດທີ່ຖືກຕ້ອງ
    pub fn has_valid_license(&self, current_date: &str) -> bool {
        self.license_expiry_date.as_str() >= current_date
    }

    /// Check if the institution is at capacity
    /// ກວດສອບວ່າສະຖາບັນເຕັມຄວາມຈຸ
    pub fn is_at_capacity(&self) -> bool {
        self.current_enrollment >= self.student_capacity
    }

    /// Calculate teacher-student ratio
    /// ຄຳນວນອັດຕາສ່ວນຄູຕໍ່ນັກຮຽນ
    pub fn teacher_student_ratio(&self) -> Option<f64> {
        if self.teacher_count == 0 {
            return None;
        }
        Some(self.current_enrollment as f64 / self.teacher_count as f64)
    }

    /// Calculate qualified teacher percentage
    /// ຄຳນວນເປີເຊັນຄູທີ່ມີຄຸນວຸດທິ
    pub fn qualified_teacher_percentage(&self) -> f64 {
        if self.teacher_count == 0 {
            return 0.0;
        }
        (self.qualified_teacher_count as f64 / self.teacher_count as f64) * 100.0
    }
}

/// Institution type (ປະເພດສະຖາບັນ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InstitutionType {
    /// Kindergarten (ໂຮງຮຽນອະນຸບານ)
    Kindergarten,

    /// Primary school (ໂຮງຮຽນປະຖົມ)
    PrimarySchool,

    /// Secondary school (ໂຮງຮຽນມັດທະຍົມ)
    SecondarySchool,

    /// Complete secondary school (ໂຮງຮຽນມັດທະຍົມສົມບູນ)
    CompleteSecondarySchool,

    /// Vocational school (ໂຮງຮຽນວິຊາຊີບ)
    VocationalSchool,

    /// Technical college (ວິທະຍາໄລເຕັກນິກ)
    TechnicalCollege,

    /// Teacher training college (ວິທະຍາໄລຄູ)
    TeacherTrainingCollege,

    /// University (ມະຫາວິທະຍາໄລ)
    University,

    /// Research institute (ສະຖາບັນຄົ້ນຄວ້າ)
    ResearchInstitute,

    /// Language center (ສູນພາສາ)
    LanguageCenter,

    /// Special needs school (ໂຮງຮຽນການສຶກສາພິເສດ)
    SpecialNeedsSchool,

    /// Non-formal education center (ສູນການສຶກສານອກລະບົບ)
    NonFormalEducationCenter,

    /// Private tutoring center (ສູນສອນພິເສດ)
    PrivateTutoringCenter,
}

impl InstitutionType {
    /// Get the education levels this institution type provides
    /// ຮັບລະດັບການສຶກສາທີ່ປະເພດສະຖາບັນນີ້ໃຫ້ບໍລິການ
    pub fn education_levels(&self) -> Vec<&'static str> {
        match self {
            InstitutionType::Kindergarten => vec!["Pre-Primary"],
            InstitutionType::PrimarySchool => vec!["Primary"],
            InstitutionType::SecondarySchool => vec!["Lower Secondary", "Upper Secondary"],
            InstitutionType::CompleteSecondarySchool => {
                vec!["Primary", "Lower Secondary", "Upper Secondary"]
            }
            InstitutionType::VocationalSchool => vec!["Technical Vocational"],
            InstitutionType::TechnicalCollege => vec!["Technical Vocational", "Higher Education"],
            InstitutionType::TeacherTrainingCollege => vec!["Higher Education"],
            InstitutionType::University => vec!["Higher Education"],
            InstitutionType::ResearchInstitute => vec!["Higher Education", "Research"],
            InstitutionType::LanguageCenter => vec!["Non-Formal"],
            InstitutionType::SpecialNeedsSchool => vec!["Special Education"],
            InstitutionType::NonFormalEducationCenter => vec!["Non-Formal"],
            InstitutionType::PrivateTutoringCenter => vec!["Non-Formal"],
        }
    }
}

/// Ownership type (ປະເພດການເປັນເຈົ້າຂອງ)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OwnershipType {
    /// Public institution (ສະຖາບັນຂອງລັດ)
    Public {
        /// Ministry responsible (ກະຊວງທີ່ຮັບຜິດຊອບ)
        ministry: String,
    },

    /// Private institution (ສະຖາບັນເອກະຊົນ)
    Private {
        /// Owner name (ຊື່ເຈົ້າຂອງ)
        owner: String,
    },

    /// Community-based institution (ສະຖາບັນຂອງຊຸມຊົນ)
    CommunityBased,

    /// Religious institution (ສະຖາບັນສາສະໜາ)
    Religious {
        /// Religious organization (ອົງການສາສະໜາ)
        organization: String,
    },

    /// International institution (ສະຖາບັນສາກົນ)
    International {
        /// Country of origin (ປະເທດຕົ້ນກຳເນີດ)
        country: String,
    },

    /// Public-private partnership (ຮ່ວມມືລັດ-ເອກະຊົນ)
    PublicPrivatePartnership {
        /// Government partner (ຄູ່ຮ່ວມລັດ)
        government_partner: String,
        /// Private partner (ຄູ່ຮ່ວມເອກະຊົນ)
        private_partner: String,
    },
}

/// Accreditation status (ສະຖານະການຮັບຮອງຄຸນນະພາບ)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AccreditationStatus {
    /// Fully accredited (ຮັບຮອງເຕັມຮູບແບບ)
    FullyAccredited {
        /// Accreditation date (ວັນທີຮັບຮອງ)
        accreditation_date: String,
        /// Expiry date (ວັນທີໝົດອາຍຸ)
        expiry_date: String,
        /// Accrediting body (ອົງການຮັບຮອງ)
        accrediting_body: String,
    },

    /// Provisionally accredited (ຮັບຮອງຊົ່ວຄາວ)
    ProvisionallyAccredited {
        /// Provisional period end date (ວັນທີສິ້ນສຸດໄລຍະຊົ່ວຄາວ)
        end_date: String,
        /// Conditions to meet (ເງື່ອນໄຂທີ່ຕ້ອງປະຕິບັດ)
        conditions: Vec<String>,
    },

    /// Pending accreditation (ລໍຖ້າການຮັບຮອງ)
    Pending {
        /// Application date (ວັນທີຍື່ນຄຳຮ້ອງ)
        application_date: String,
    },

    /// Not accredited (ບໍ່ໄດ້ຮັບຮອງ)
    NotAccredited,

    /// Accreditation expired (ການຮັບຮອງໝົດອາຍຸ)
    Expired {
        /// Last valid date (ວັນທີຖືກຕ້ອງຄັ້ງສຸດທ້າຍ)
        last_valid_date: String,
    },

    /// Accreditation revoked (ຖືກຖອນການຮັບຮອງ)
    Revoked {
        /// Revocation date (ວັນທີຖືກຖອນ)
        revocation_date: String,
        /// Reason for revocation (ເຫດຜົນການຖືກຖອນ)
        reason: String,
    },
}

// ============================================================================
// Education Programs (ຫຼັກສູດການສຶກສາ)
// ============================================================================

/// Education program (ຫຼັກສູດການສຶກສາ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EducationProgram {
    /// Program name in Lao (ຊື່ຫຼັກສູດເປັນພາສາລາວ)
    pub program_name_lao: String,

    /// Program name in English (ຊື່ຫຼັກສູດເປັນພາສາອັງກິດ)
    pub program_name_en: String,

    /// Education level (ລະດັບການສຶກສາ)
    pub level: EducationLevel,

    /// Program duration in years (ໄລຍະຫຼັກສູດເປັນປີ)
    pub duration_years: f32,

    /// Total credits required (ໜ່ວຍກິດທັງໝົດ)
    pub credits: Option<u16>,

    /// Accreditation status (ສະຖານະການຮັບຮອງ)
    pub accreditation_status: AccreditationStatus,

    /// Language of instruction (ພາສາທີ່ໃຊ້ສອນ)
    pub language_of_instruction: InstructionLanguage,

    /// Program code (ລະຫັດຫຼັກສູດ)
    pub program_code: Option<String>,

    /// Field of study (ສາຂາວິຊາ)
    pub field_of_study: String,

    /// Maximum enrollment (ຈຳນວນຮັບສູງສຸດ)
    pub max_enrollment: Option<u32>,

    /// Entry requirements (ເງື່ອນໄຂເຂົ້າຮຽນ)
    pub entry_requirements: Vec<EntryRequirement>,

    /// Is active (ເປີດສອນຢູ່)
    pub is_active: bool,
}

impl EducationProgram {
    /// Check if the program is accredited
    /// ກວດສອບວ່າຫຼັກສູດໄດ້ຮັບການຮັບຮອງ
    pub fn is_accredited(&self) -> bool {
        matches!(
            self.accreditation_status,
            AccreditationStatus::FullyAccredited { .. }
                | AccreditationStatus::ProvisionallyAccredited { .. }
        )
    }
}

/// Instruction language (ພາສາທີ່ໃຊ້ສອນ)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InstructionLanguage {
    /// Lao language (ພາສາລາວ)
    Lao,

    /// English language (ພາສາອັງກິດ)
    English,

    /// Bilingual Lao-English (ສອງພາສາ ລາວ-ອັງກິດ)
    Bilingual,

    /// Trilingual with ethnic language (ສາມພາສາ)
    Trilingual {
        /// Ethnic language used (ພາສາຊົນເຜົ່າ)
        ethnic_language: String,
    },

    /// Other language (ພາສາອື່ນ)
    Other {
        /// Language name (ຊື່ພາສາ)
        language: String,
    },
}

/// Entry requirement (ເງື່ອນໄຂເຂົ້າຮຽນ)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EntryRequirement {
    /// Minimum age requirement (ອາຍຸຂັ້ນຕ່ຳ)
    MinimumAge(u8),

    /// Previous education level (ລະດັບການສຶກສາກ່ອນໜ້າ)
    PreviousEducation(String),

    /// Entrance examination (ສອບເສັງເຂົ້າ)
    EntranceExam {
        /// Exam name (ຊື່ການສອບເສັງ)
        exam_name: String,
        /// Minimum score (ຄະແນນຂັ້ນຕ່ຳ)
        min_score: Option<f64>,
    },

    /// Language proficiency (ຄວາມສາມາດທາງພາສາ)
    LanguageProficiency {
        /// Language (ພາສາ)
        language: String,
        /// Required level (ລະດັບທີ່ຕ້ອງການ)
        level: String,
    },

    /// Other requirement (ເງື່ອນໄຂອື່ນ)
    Other(String),
}

// ============================================================================
// Teachers (ຄູ)
// ============================================================================

/// Teacher (ຄູ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Teacher {
    /// Teacher name (ຊື່ຄູ)
    pub name: String,

    /// Teacher name in Lao (ຊື່ຄູເປັນພາສາລາວ)
    pub name_lao: Option<String>,

    /// Teacher ID (ລະຫັດຄູ)
    pub teacher_id: String,

    /// Qualification (ຄຸນວຸດທິ)
    pub qualification: TeacherQualification,

    /// Teaching level (ລະດັບທີ່ສອນ)
    pub teaching_level: EducationLevel,

    /// Subject specialization (ວິຊາສະເພາະ)
    pub subject_specialization: Option<String>,

    /// License status (ສະຖານະໃບອະນຸຍາດ)
    pub license_status: TeacherLicenseStatus,

    /// Years of experience (ປີປະສົບການ)
    pub years_of_experience: u8,

    /// Employment status (ສະຖານະການຈ້າງງານ)
    pub employment_status: TeacherEmploymentStatus,

    /// Institution ID (ລະຫັດສະຖາບັນ)
    pub institution_id: Option<String>,

    /// Professional development hours (ຊົ່ວໂມງພັດທະນາວິຊາຊີບ)
    pub professional_development_hours: u32,

    /// Teaching subjects (ວິຊາທີ່ສອນ)
    pub teaching_subjects: Vec<String>,
}

impl Teacher {
    /// Check if teacher has valid license
    /// ກວດສອບວ່າຄູມີໃບອະນຸຍາດທີ່ຖືກຕ້ອງ
    pub fn has_valid_license(&self) -> bool {
        matches!(self.license_status, TeacherLicenseStatus::Licensed { .. })
    }

    /// Check if teacher meets minimum qualification for their level
    /// ກວດສອບວ່າຄູມີຄຸນວຸດທິຂັ້ນຕ່ຳສຳລັບລະດັບທີ່ສອນ
    pub fn meets_minimum_qualification(&self) -> bool {
        let min_level = match &self.teaching_level {
            EducationLevel::PrePrimary { .. } => TeacherQualification::TeacherCertificate,
            EducationLevel::Primary { .. } => TeacherQualification::Diploma,
            EducationLevel::LowerSecondary { .. } => TeacherQualification::BachelorInEducation,
            EducationLevel::UpperSecondary { .. } => TeacherQualification::BachelorInEducation,
            EducationLevel::TechnicalVocational { .. } => TeacherQualification::BachelorInEducation,
            EducationLevel::HigherEducation { .. } => TeacherQualification::MasterInEducation,
            _ => TeacherQualification::TeacherCertificate,
        };

        self.qualification.level() >= min_level.level()
    }
}

/// Teacher qualification (ຄຸນວຸດທິຄູ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TeacherQualification {
    /// Teacher certificate (ໃບຢັ້ງຢືນຄູ)
    TeacherCertificate,

    /// Diploma in education (ອະນຸປະລິນຍາການສຶກສາ)
    Diploma,

    /// Bachelor in education (ປະລິນຍາຕີການສຶກສາ)
    BachelorInEducation,

    /// Master in education (ປະລິນຍາໂທການສຶກສາ)
    MasterInEducation,

    /// Doctorate in education (ປະລິນຍາເອກການສຶກສາ)
    Doctorate,

    /// Bachelor in subject field (ປະລິນຍາຕີສາຂາວິຊາ)
    BachelorInField,

    /// Master in subject field (ປະລິນຍາໂທສາຂາວິຊາ)
    MasterInField,
}

impl TeacherQualification {
    /// Get the level of this qualification (for comparison)
    /// ຮັບລະດັບຂອງຄຸນວຸດທິນີ້ (ສຳລັບການປຽບທຽບ)
    pub fn level(&self) -> u8 {
        match self {
            TeacherQualification::TeacherCertificate => 1,
            TeacherQualification::Diploma => 2,
            TeacherQualification::BachelorInEducation | TeacherQualification::BachelorInField => 3,
            TeacherQualification::MasterInEducation | TeacherQualification::MasterInField => 4,
            TeacherQualification::Doctorate => 5,
        }
    }
}

/// Teacher license status (ສະຖານະໃບອະນຸຍາດຄູ)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TeacherLicenseStatus {
    /// Licensed (ມີໃບອະນຸຍາດ)
    Licensed {
        /// License expiry date (ວັນທີໝົດອາຍຸ)
        expiry_date: String,
        /// License number (ເລກທີໃບອະນຸຍາດ)
        license_number: String,
    },

    /// Provisional license (ໃບອະນຸຍາດຊົ່ວຄາວ)
    Provisional {
        /// Reason for provisional status (ເຫດຜົນ)
        reason: String,
        /// End date of provisional period (ວັນທີສິ້ນສຸດ)
        end_date: String,
    },

    /// Expired license (ໃບອະນຸຍາດໝົດອາຍຸ)
    Expired {
        /// Last valid date (ວັນທີຖືກຕ້ອງຄັ້ງສຸດທ້າຍ)
        last_valid: String,
    },

    /// Revoked license (ຖືກຖອນໃບອະນຸຍາດ)
    Revoked {
        /// Reason for revocation (ເຫດຜົນການຖືກຖອນ)
        reason: String,
        /// Revocation date (ວັນທີຖືກຖອນ)
        revocation_date: String,
    },

    /// No license (ບໍ່ມີໃບອະນຸຍາດ)
    None,
}

/// Teacher employment status (ສະຖານະການຈ້າງງານຄູ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TeacherEmploymentStatus {
    /// Full-time (ເຕັມເວລາ)
    FullTime,
    /// Part-time (ບາງເວລາ)
    PartTime,
    /// Contract (ສັນຍາ)
    Contract,
    /// Volunteer (ອາສາສະໝັກ)
    Volunteer,
    /// Retired (ບຳນານ)
    Retired,
    /// On leave (ລາພັກ)
    OnLeave,
}

// ============================================================================
// Student Rights (ສິດຂອງນັກຮຽນ)
// ============================================================================

/// Student right (ສິດຂອງນັກຮຽນ)
///
/// Article 72-79: Rights of students in Lao education system
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StudentRight {
    /// Right type (ປະເພດສິດ)
    pub right_type: StudentRightType,

    /// Article reference in Education Law (ອ້າງອີງມາດຕາ)
    pub article_reference: u16,

    /// Description in Lao (ລາຍລະອຽດເປັນພາສາລາວ)
    pub description_lao: String,

    /// Description in English (ລາຍລະອຽດເປັນພາສາອັງກິດ)
    pub description_en: String,

    /// Scope of right (ຂອບເຂດສິດ)
    pub scope: RightScope,

    /// Is constitutional right (ເປັນສິດຕາມລັດຖະທຳມະນູນ)
    pub is_constitutional: bool,
}

/// Student right type (ປະເພດສິດນັກຮຽນ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StudentRightType {
    /// Free compulsory education (ການສຶກສາບັງຄັບໂດຍບໍ່ເສຍຄ່າ)
    /// Article 26: Primary education is free for all
    FreeCompulsoryEducation,

    /// Non-discrimination (ບໍ່ຈຳແນກ)
    /// Article 72: No discrimination based on gender, ethnicity, religion
    NonDiscrimination,

    /// Safe learning environment (ສະພາບແວດລ້ອມການຮຽນທີ່ປອດໄພ)
    /// Article 75: Safe and healthy learning environment
    SafeLearningEnvironment,

    /// Quality education (ການສຶກສາທີ່ມີຄຸນນະພາບ)
    /// Article 74: Access to quality education
    QualityEducation,

    /// Mother tongue education (ການສຶກສາພາສາແມ່)
    /// Article 73: Ethnic minorities can learn in their language
    MotherTongueEducation,

    /// Special needs accommodation (ການອຳນວຍຄວາມສະດວກສຳລັບຄວາມຕ້ອງການພິເສດ)
    /// Article 22: Education for students with disabilities
    SpecialNeedsAccommodation,

    /// Privacy of records (ຄວາມເປັນສ່ວນຕົວຂອງບັນທຶກ)
    /// Article 77: Protection of student records
    PrivacyOfRecords,

    /// Participation in activities (ການມີສ່ວນຮ່ວມໃນກິດຈະກຳ)
    /// Article 76: Right to participate in school activities
    ParticipationInActivities,

    /// Appeal and grievance (ການອຸທອນແລະຮ້ອງຟ້ອງ)
    /// Article 78: Right to appeal decisions
    AppealAndGrievance,

    /// Access to information (ການເຂົ້າເຖິງຂໍ້ມູນ)
    /// Article 79: Access to educational information
    AccessToInformation,
}

/// Right scope (ຂອບເຂດສິດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RightScope {
    /// Universal - applies to all students (ທົ່ວໄປ)
    Universal,
    /// Specific level only (ສະເພາະລະດັບ)
    LevelSpecific,
    /// Special category students (ນັກຮຽນປະເພດພິເສດ)
    SpecialCategory,
    /// Geographic specific (ສະເພາະພາກພື້ນ)
    GeographicSpecific,
}

// ============================================================================
// Compulsory Education (ການສຶກສາບັງຄັບ)
// ============================================================================

/// Compulsory education record (ບັນທຶກການສຶກສາບັງຄັບ)
///
/// Article 26: All children aged 6-14 must complete primary and lower secondary education
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CompulsoryEducation {
    /// Child name (ຊື່ເດັກ)
    pub child_name: String,

    /// Child name in Lao (ຊື່ເດັກເປັນພາສາລາວ)
    pub child_name_lao: Option<String>,

    /// Date of birth (ວັນເດືອນປີເກີດ)
    pub date_of_birth: String,

    /// Current age (ອາຍຸປັດຈຸບັນ)
    pub current_age: u8,

    /// Enrollment status (ສະຖານະການລົງທະບຽນ)
    pub enrollment_status: EnrollmentStatus,

    /// Current grade (ຊັ້ນປັດຈຸບັນ)
    pub current_grade: u8,

    /// School name (ຊື່ໂຮງຮຽນ)
    pub school_name: String,

    /// School ID (ລະຫັດໂຮງຮຽນ)
    pub school_id: Option<String>,

    /// Guardian name (ຊື່ຜູ້ປົກຄອງ)
    pub guardian_name: String,

    /// Guardian relationship (ຄວາມສຳພັນກັບຜູ້ປົກຄອງ)
    pub guardian_relationship: String,

    /// Guardian contact (ຂໍ້ມູນຕິດຕໍ່ຜູ້ປົກຄອງ)
    pub guardian_contact: Option<String>,

    /// Province (ແຂວງ)
    pub province: String,

    /// District (ເມືອງ)
    pub district: String,

    /// Village (ບ້ານ)
    pub village: String,

    /// Is ethnic minority (ເປັນຊົນເຜົ່າສ່ວນນ້ອຍ)
    pub is_ethnic_minority: bool,

    /// Ethnic group (ກຸ່ມຊົນເຜົ່າ)
    pub ethnic_group: Option<String>,

    /// Has special needs (ມີຄວາມຕ້ອງການພິເສດ)
    pub has_special_needs: bool,

    /// Special needs type (ປະເພດຄວາມຕ້ອງການພິເສດ)
    pub special_needs_type: Option<SpecialEducationType>,
}

impl CompulsoryEducation {
    /// Check if child is within compulsory education age
    /// ກວດສອບວ່າເດັກຢູ່ໃນອາຍຸການສຶກສາບັງຄັບ
    pub fn is_compulsory_age(&self) -> bool {
        self.current_age >= COMPULSORY_EDUCATION_START_AGE
            && self.current_age <= COMPULSORY_EDUCATION_END_AGE
    }

    /// Check if child should be enrolled
    /// ກວດສອບວ່າເດັກຄວນຈະລົງທະບຽນ
    pub fn should_be_enrolled(&self) -> bool {
        self.is_compulsory_age()
            && !matches!(
                self.enrollment_status,
                EnrollmentStatus::Enrolled { .. } | EnrollmentStatus::Graduated { .. }
            )
    }

    /// Calculate expected grade based on age
    /// ຄຳນວນຊັ້ນທີ່ຄາດໝາຍຕາມອາຍຸ
    pub fn expected_grade(&self) -> u8 {
        if self.current_age < COMPULSORY_EDUCATION_START_AGE {
            0
        } else {
            (self.current_age - COMPULSORY_EDUCATION_START_AGE + 1).min(9)
        }
    }
}

/// Enrollment status (ສະຖານະການລົງທະບຽນ)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EnrollmentStatus {
    /// Enrolled (ລົງທະບຽນແລ້ວ)
    Enrolled {
        /// Enrollment date (ວັນທີລົງທະບຽນ)
        enrollment_date: String,
        /// Student ID (ລະຫັດນັກຮຽນ)
        student_id: Option<String>,
    },

    /// Not enrolled (ບໍ່ໄດ້ລົງທະບຽນ)
    NotEnrolled {
        /// Reason for non-enrollment (ເຫດຜົນ)
        reason: NonEnrollmentReason,
    },

    /// Dropped out (ອອກກາງຄັນ)
    Dropped {
        /// Reason for dropping out (ເຫດຜົນ)
        reason: String,
        /// Date of dropping out (ວັນທີອອກ)
        date: String,
        /// Last grade completed (ຊັ້ນສຸດທ້າຍທີ່ຮຽນ)
        last_grade: u8,
    },

    /// Graduated (ສຳເລັດການສຶກສາ)
    Graduated {
        /// Graduation date (ວັນທີສຳເລັດການສຶກສາ)
        date: String,
        /// Certificate number (ເລກທີໃບຢັ້ງຢືນ)
        certificate_number: Option<String>,
    },

    /// Transferred (ຍ້າຍໂຮງຮຽນ)
    Transferred {
        /// Destination school (ໂຮງຮຽນປາຍທາງ)
        to_school: String,
        /// Transfer date (ວັນທີຍ້າຍ)
        date: String,
    },

    /// Suspended (ຖືກພັກການຮຽນ)
    Suspended {
        /// Reason for suspension (ເຫດຜົນ)
        reason: String,
        /// Start date (ວັນທີເລີ່ມ)
        start_date: String,
        /// Expected end date (ວັນທີສິ້ນສຸດທີ່ຄາດໝາຍ)
        expected_end_date: String,
    },
}

/// Non-enrollment reason (ເຫດຜົນບໍ່ລົງທະບຽນ)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NonEnrollmentReason {
    /// Age not reached (ອາຍຸບໍ່ເຖິງ)
    AgeNotReached,

    /// No school access (ບໍ່ສາມາດເຂົ້າເຖິງໂຮງຮຽນ)
    NoSchoolAccess {
        /// Distance to nearest school in km (ໄລຍະຫ່າງຈາກໂຮງຮຽນໃກ້ທີ່ສຸດ)
        distance_km: Option<f64>,
    },

    /// Economic hardship (ຄວາມຫຍຸ້ງຍາກທາງເສດຖະກິດ)
    EconomicHardship,

    /// Disability (ຄວາມພິການ)
    Disability {
        /// Type of disability (ປະເພດຄວາມພິການ)
        disability_type: String,
    },

    /// Parental refusal (ພໍ່ແມ່ປະຕິເສດ)
    ParentalRefusal,

    /// Child labor (ແຮງງານເດັກ)
    ChildLabor,

    /// Health issues (ບັນຫາສຸຂະພາບ)
    HealthIssues {
        /// Description (ລາຍລະອຽດ)
        description: String,
    },

    /// Migration (ການເຄື່ອນຍ້າຍ)
    Migration,

    /// Other reason (ເຫດຜົນອື່ນ)
    Other {
        /// Reason description (ລາຍລະອຽດເຫດຜົນ)
        reason: String,
    },
}

// ============================================================================
// Scholarship and Financial Aid (ທຶນການສຶກສາ)
// ============================================================================

/// Scholarship (ທຶນການສຶກສາ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Scholarship {
    /// Scholarship name (ຊື່ທຶນການສຶກສາ)
    pub scholarship_name: String,

    /// Scholarship name in Lao (ຊື່ທຶນການສຶກສາເປັນພາສາລາວ)
    pub scholarship_name_lao: Option<String>,

    /// Provider (ຜູ້ໃຫ້ທຶນ)
    pub provider: ScholarshipProvider,

    /// Eligibility criteria (ເກນການຄັດເລືອກ)
    pub eligibility_criteria: Vec<EligibilityCriterion>,

    /// Coverage type (ປະເພດການຄຸ້ມຄອງ)
    pub coverage: ScholarshipCoverage,

    /// Duration in years (ໄລຍະເວລາເປັນປີ)
    pub duration_years: u8,

    /// Application deadline (ກຳນົດຍື່ນໃບສະໝັກ)
    pub application_deadline: Option<String>,

    /// Number of awards (ຈຳນວນທຶນ)
    pub num_awards: Option<u32>,

    /// Field of study restrictions (ສາຂາວິຊາທີ່ຈຳກັດ)
    pub field_restrictions: Vec<String>,

    /// Education level (ລະດັບການສຶກສາ)
    pub education_level: EducationLevel,

    /// Is renewable (ສາມາດຕໍ່ໄດ້)
    pub is_renewable: bool,

    /// Contact information (ຂໍ້ມູນຕິດຕໍ່)
    pub contact_info: Option<String>,
}

/// Scholarship provider (ຜູ້ໃຫ້ທຶນການສຶກສາ)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScholarshipProvider {
    /// Government (ລັດຖະບານ)
    Government {
        /// Ministry providing the scholarship (ກະຊວງ)
        ministry: Option<String>,
    },

    /// Foreign government (ລັດຖະບານຕ່າງປະເທດ)
    ForeignGovernment {
        /// Country name (ຊື່ປະເທດ)
        country: String,
    },

    /// International organization (ອົງການສາກົນ)
    InternationalOrganization {
        /// Organization name (ຊື່ອົງການ)
        name: String,
    },

    /// Private sector (ພາກເອກະຊົນ)
    PrivateSector {
        /// Company name (ຊື່ບໍລິສັດ)
        company: String,
    },

    /// NGO (ອົງການບໍ່ຫວັງຜົນກຳໄລ)
    NGO {
        /// NGO name (ຊື່ອົງການ)
        name: String,
    },

    /// Educational institution (ສະຖາບັນການສຶກສາ)
    EducationalInstitution {
        /// Institution name (ຊື່ສະຖາບັນ)
        institution: String,
    },

    /// Community (ຊຸມຊົນ)
    Community,

    /// Religious organization (ອົງການສາສະໜາ)
    Religious {
        /// Organization name (ຊື່ອົງການ)
        organization: String,
    },
}

/// Eligibility criterion (ເກນການຄັດເລືອກ)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EligibilityCriterion {
    /// Minimum GPA (ຄະແນນສະເລ່ຍຂັ້ນຕ່ຳ)
    MinimumGPA(f64),

    /// Economic need (ຄວາມຕ້ອງການທາງເສດຖະກິດ)
    EconomicNeed {
        /// Maximum family income (ລາຍໄດ້ຄອບຄົວສູງສຸດ)
        max_family_income: Option<u64>,
    },

    /// Geographic origin (ພາກພື້ນຕົ້ນກຳເນີດ)
    GeographicOrigin {
        /// Province (ແຂວງ)
        province: Option<String>,
        /// Is rural required (ຕ້ອງເປັນຊົນນະບົດ)
        rural_required: bool,
    },

    /// Ethnic minority (ຊົນເຜົ່າສ່ວນນ້ອຍ)
    EthnicMinority {
        /// Specific ethnic group (ກຸ່ມຊົນເຜົ່າສະເພາະ)
        ethnic_group: Option<String>,
    },

    /// Gender requirement (ເພດ)
    Gender {
        /// Required gender (ເພດທີ່ຕ້ອງການ)
        gender: String,
    },

    /// Age limit (ຂີດຈຳກັດອາຍຸ)
    AgeLimit {
        /// Minimum age (ອາຍຸຂັ້ນຕ່ຳ)
        min: Option<u8>,
        /// Maximum age (ອາຍຸສູງສຸດ)
        max: Option<u8>,
    },

    /// Previous education (ການສຶກສາກ່ອນໜ້າ)
    PreviousEducation {
        /// Required level (ລະດັບທີ່ຕ້ອງການ)
        level: String,
    },

    /// Field of study (ສາຂາວິຊາ)
    FieldOfStudy {
        /// Allowed fields (ສາຂາທີ່ອະນຸຍາດ)
        fields: Vec<String>,
    },

    /// Disability (ຄວາມພິການ)
    Disability,

    /// Other criterion (ເກນອື່ນ)
    Other(String),
}

/// Scholarship coverage (ການຄຸ້ມຄອງທຶນການສຶກສາ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScholarshipCoverage {
    /// Tuition coverage percentage (ເປີເຊັນຄຸ້ມຄອງຄ່າຮຽນ)
    pub tuition_percentage: f64,

    /// Monthly stipend in LAK (ເງິນອຸດໜູນລາຍເດືອນ)
    pub monthly_stipend_lak: Option<u64>,

    /// Book allowance in LAK (ເງິນຊື້ປຶ້ມ)
    pub book_allowance_lak: Option<u64>,

    /// Accommodation provided (ໃຫ້ທີ່ພັກ)
    pub accommodation_provided: bool,

    /// Travel allowance in LAK (ເງິນຄ່າເດີນທາງ)
    pub travel_allowance_lak: Option<u64>,

    /// Health insurance provided (ໃຫ້ປະກັນສຸຂະພາບ)
    pub health_insurance_provided: bool,

    /// Other benefits (ຜົນປະໂຫຍດອື່ນ)
    pub other_benefits: Vec<String>,
}

// ============================================================================
// Curriculum (ຫຼັກສູດ)
// ============================================================================

/// National curriculum (ຫຼັກສູດແຫ່ງຊາດ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NationalCurriculum {
    /// Curriculum name (ຊື່ຫຼັກສູດ)
    pub name: String,

    /// Education level (ລະດັບການສຶກສາ)
    pub level: EducationLevel,

    /// Version/Year (ສະບັບ/ປີ)
    pub version: String,

    /// Approval date (ວັນທີອະນຸມັດ)
    pub approval_date: String,

    /// Core subjects (ວິຊາຫຼັກ)
    pub core_subjects: Vec<Subject>,

    /// Elective subjects (ວິຊາເລືອກ)
    pub elective_subjects: Vec<Subject>,

    /// Total hours per year (ຊົ່ວໂມງທັງໝົດຕໍ່ປີ)
    pub total_hours_per_year: u32,

    /// Assessment methods (ວິທີການປະເມີນ)
    pub assessment_methods: Vec<String>,
}

/// Subject (ວິຊາ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Subject {
    /// Subject name in Lao (ຊື່ວິຊາເປັນພາສາລາວ)
    pub name_lao: String,

    /// Subject name in English (ຊື່ວິຊາເປັນພາສາອັງກິດ)
    pub name_en: String,

    /// Hours per week (ຊົ່ວໂມງຕໍ່ອາທິດ)
    pub hours_per_week: u8,

    /// Is mandatory (ບັງຄັບ)
    pub is_mandatory: bool,

    /// Grade levels (ລະດັບຊັ້ນ)
    pub grade_levels: Vec<u8>,
}

// ============================================================================
// Builder Patterns
// ============================================================================

/// Builder for EducationalInstitution
#[derive(Debug, Default)]
pub struct EducationalInstitutionBuilder {
    name_lao: Option<String>,
    name_en: Option<String>,
    institution_type: Option<InstitutionType>,
    ownership_type: Option<OwnershipType>,
    license_number: Option<String>,
    license_issue_date: Option<String>,
    license_expiry_date: Option<String>,
    province: Option<String>,
    district: Option<String>,
    address: Option<String>,
    accreditation_status: Option<AccreditationStatus>,
    student_capacity: Option<u32>,
    current_enrollment: Option<u32>,
    programs_offered: Vec<EducationProgram>,
    teacher_count: Option<u32>,
    qualified_teacher_count: Option<u32>,
    contact_phone: Option<String>,
    contact_email: Option<String>,
    website: Option<String>,
    established_year: Option<u16>,
}

impl EducationalInstitutionBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the institution name in Lao
    pub fn name_lao(mut self, name: impl Into<String>) -> Self {
        self.name_lao = Some(name.into());
        self
    }

    /// Set the institution name in English
    pub fn name_en(mut self, name: impl Into<String>) -> Self {
        self.name_en = Some(name.into());
        self
    }

    /// Set the institution type
    pub fn institution_type(mut self, inst_type: InstitutionType) -> Self {
        self.institution_type = Some(inst_type);
        self
    }

    /// Set the ownership type
    pub fn ownership_type(mut self, ownership: OwnershipType) -> Self {
        self.ownership_type = Some(ownership);
        self
    }

    /// Set the license number
    pub fn license_number(mut self, number: impl Into<String>) -> Self {
        self.license_number = Some(number.into());
        self
    }

    /// Set license dates
    pub fn license_dates(
        mut self,
        issue_date: impl Into<String>,
        expiry_date: impl Into<String>,
    ) -> Self {
        self.license_issue_date = Some(issue_date.into());
        self.license_expiry_date = Some(expiry_date.into());
        self
    }

    /// Set the location
    pub fn location(
        mut self,
        province: impl Into<String>,
        district: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        self.province = Some(province.into());
        self.district = Some(district.into());
        self.address = Some(address.into());
        self
    }

    /// Set the accreditation status
    pub fn accreditation_status(mut self, status: AccreditationStatus) -> Self {
        self.accreditation_status = Some(status);
        self
    }

    /// Set student capacity and enrollment
    pub fn capacity(mut self, capacity: u32, current: u32) -> Self {
        self.student_capacity = Some(capacity);
        self.current_enrollment = Some(current);
        self
    }

    /// Set teacher counts
    pub fn teachers(mut self, total: u32, qualified: u32) -> Self {
        self.teacher_count = Some(total);
        self.qualified_teacher_count = Some(qualified);
        self
    }

    /// Add a program
    pub fn add_program(mut self, program: EducationProgram) -> Self {
        self.programs_offered.push(program);
        self
    }

    /// Set contact information
    pub fn contact(
        mut self,
        phone: Option<String>,
        email: Option<String>,
        website: Option<String>,
    ) -> Self {
        self.contact_phone = phone;
        self.contact_email = email;
        self.website = website;
        self
    }

    /// Set the established year
    pub fn established_year(mut self, year: u16) -> Self {
        self.established_year = Some(year);
        self
    }

    /// Build the EducationalInstitution
    pub fn build(self) -> Result<EducationalInstitution, String> {
        Ok(EducationalInstitution {
            name_lao: self.name_lao.ok_or("name_lao is required")?,
            name_en: self.name_en.ok_or("name_en is required")?,
            institution_type: self
                .institution_type
                .ok_or("institution_type is required")?,
            ownership_type: self.ownership_type.ok_or("ownership_type is required")?,
            license_number: self.license_number.ok_or("license_number is required")?,
            license_issue_date: self
                .license_issue_date
                .ok_or("license_issue_date is required")?,
            license_expiry_date: self
                .license_expiry_date
                .ok_or("license_expiry_date is required")?,
            province: self.province.ok_or("province is required")?,
            district: self.district.ok_or("district is required")?,
            address: self.address.ok_or("address is required")?,
            accreditation_status: self
                .accreditation_status
                .unwrap_or(AccreditationStatus::NotAccredited),
            student_capacity: self.student_capacity.unwrap_or(0),
            current_enrollment: self.current_enrollment.unwrap_or(0),
            programs_offered: self.programs_offered,
            teacher_count: self.teacher_count.unwrap_or(0),
            qualified_teacher_count: self.qualified_teacher_count.unwrap_or(0),
            contact_phone: self.contact_phone,
            contact_email: self.contact_email,
            website: self.website,
            established_year: self.established_year.unwrap_or(2000),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_education_level_compulsory() {
        let primary = EducationLevel::Primary { grades: 5 };
        assert!(primary.is_compulsory());

        let upper_secondary = EducationLevel::UpperSecondary { grades: 3 };
        assert!(!upper_secondary.is_compulsory());
    }

    #[test]
    fn test_degree_type_duration() {
        assert_eq!(
            DegreeType::BachelorDegree.standard_duration_years(),
            Some(4)
        );
        assert_eq!(DegreeType::MasterDegree.standard_duration_years(), Some(2));
    }

    #[test]
    fn test_teacher_qualification_level() {
        assert!(
            TeacherQualification::Doctorate.level()
                > TeacherQualification::BachelorInEducation.level()
        );
    }

    #[test]
    fn test_compulsory_education_age() {
        let child = CompulsoryEducation {
            child_name: "Test Child".to_string(),
            child_name_lao: None,
            date_of_birth: "2018-01-01".to_string(),
            current_age: 7,
            enrollment_status: EnrollmentStatus::Enrolled {
                enrollment_date: "2024-09-01".to_string(),
                student_id: Some("STU001".to_string()),
            },
            current_grade: 2,
            school_name: "Test School".to_string(),
            school_id: None,
            guardian_name: "Parent".to_string(),
            guardian_relationship: "Mother".to_string(),
            guardian_contact: None,
            province: "Vientiane".to_string(),
            district: "Chanthabuly".to_string(),
            village: "Test Village".to_string(),
            is_ethnic_minority: false,
            ethnic_group: None,
            has_special_needs: false,
            special_needs_type: None,
        };

        assert!(child.is_compulsory_age());
        assert_eq!(child.expected_grade(), 2);
    }

    #[test]
    fn test_institution_builder() {
        let result = EducationalInstitutionBuilder::new()
            .name_lao("ໂຮງຮຽນທົດສອບ")
            .name_en("Test School")
            .institution_type(InstitutionType::PrimarySchool)
            .ownership_type(OwnershipType::Public {
                ministry: "Ministry of Education".to_string(),
            })
            .license_number("LIC-2024-001")
            .license_dates("2024-01-01", "2029-01-01")
            .location("Vientiane", "Chanthabuly", "123 Test Road")
            .capacity(500, 350)
            .teachers(20, 18)
            .established_year(2000)
            .build();

        assert!(result.is_ok());
        let institution = result.expect("Should build successfully");
        assert_eq!(institution.name_en, "Test School");
        assert_eq!(institution.student_capacity, 500);
    }
}
