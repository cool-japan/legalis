//! Administrative Law Types for Lao PDR (ປະເພດກົດໝາຍບໍລິຫານ)
//!
//! This module defines comprehensive types for administrative law in Lao PDR, including:
//! - Administrative decisions and acts
//! - Licensing and permit framework
//! - Administrative sanctions
//! - Administrative appeals and review
//! - State liability claims
//!
//! ## Legal Basis
//!
//! - **Administrative Procedure Law** (ກົດໝາຍວ່າດ້ວຍຂັ້ນຕອນການບໍລິຫານ)
//! - **State Liability Law** (ກົດໝາຍວ່າດ້ວຍຄວາມຮັບຜິດຊອບຂອງລັດ)
//! - **Law on People's Courts** (ກົດໝາຍວ່າດ້ວຍສານປະຊາຊົນ)

use serde::{Deserialize, Serialize};

// ============================================================================
// Constants (ຄ່າຄົງທີ່)
// ============================================================================

/// Administrative appeal deadline: 30 days from notification
/// ກຳນົດເວລາການອຸທອນບໍລິຫານ: 30 ວັນນັບແຕ່ວັນທີ່ໄດ້ຮັບແຈ້ງ
pub const ADMINISTRATIVE_APPEAL_DEADLINE_DAYS: u8 = 30;

/// Court appeal deadline: 60 days from administrative decision
/// ກຳນົດເວລາການຟ້ອງຕໍ່ສານ: 60 ວັນນັບແຕ່ວັນທີ່ມີການຕັດສິນໃຈບໍລິຫານ
pub const COURT_APPEAL_DEADLINE_DAYS: u8 = 60;

/// State liability claim deadline: 2 years from wrongful act
/// ກຳນົດເວລາການຮ້ອງຂໍຄ່າເສຍຫາຍຈາກລັດ: 2 ປີນັບແຕ່ວັນທີ່ມີການກະທຳຜິດ
pub const STATE_LIABILITY_CLAIM_DEADLINE_YEARS: u8 = 2;

/// Minimum fine amount in LAK
/// ຈຳນວນເງິນປັບຂັ້ນຕ່ຳເປັນກີບ
pub const MINIMUM_FINE_AMOUNT_LAK: u64 = 100_000;

/// Maximum suspension days for licenses
/// ຈຳນວນວັນສູງສຸດສຳລັບການລະງັບໃບອະນຸຍາດ
pub const MAXIMUM_SUSPENSION_DAYS: u32 = 365;

/// Maximum license revocation permanent ban years
/// ຈຳນວນປີສູງສຸດສຳລັບການຫ້າມຖາວອນ
pub const MAXIMUM_BAN_YEARS: u32 = 10;

/// Default notification period in days
/// ໄລຍະເວລາແຈ້ງເຕືອນເລີ່ມຕົ້ນເປັນວັນ
pub const DEFAULT_NOTIFICATION_PERIOD_DAYS: u8 = 15;

/// Village level jurisdiction limit in LAK
/// ຂອບເຂດອຳນາດລະດັບບ້ານເປັນກີບ
pub const VILLAGE_JURISDICTION_LIMIT_LAK: u64 = 5_000_000;

/// District level jurisdiction limit in LAK
/// ຂອບເຂດອຳນາດລະດັບເມືອງເປັນກີບ
pub const DISTRICT_JURISDICTION_LIMIT_LAK: u64 = 50_000_000;

/// Provincial level jurisdiction limit in LAK
/// ຂອບເຂດອຳນາດລະດັບແຂວງເປັນກີບ
pub const PROVINCIAL_JURISDICTION_LIMIT_LAK: u64 = 500_000_000;

// ============================================================================
// Administrative Authority Levels (ລະດັບອຳນາດບໍລິຫານ)
// ============================================================================

/// Administrative Authority Levels in Lao PDR
/// ລະດັບອຳນາດບໍລິຫານໃນ ສປປ ລາວ
///
/// Administrative authorities in Lao PDR are organized hierarchically:
/// - Central (ສູນກາງ): Ministries and central government agencies
/// - Provincial (ແຂວງ): Provincial government offices
/// - District (ເມືອງ): District government offices
/// - Village (ບ້ານ): Village administrative units
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdministrativeLevel {
    /// Central government level (ລະດັບສູນກາງ)
    /// Includes ministries and national agencies
    Central {
        /// Ministry name (ຊື່ກະຊວງ)
        ministry: String,
    },

    /// Provincial level (ລະດັບແຂວງ)
    /// Provincial government offices
    Provincial {
        /// Province name (ຊື່ແຂວງ)
        province: String,
    },

    /// District level (ລະດັບເມືອງ)
    /// District government offices
    District {
        /// District name (ຊື່ເມືອງ)
        district: String,
    },

    /// Village level (ລະດັບບ້ານ)
    /// Village administrative units
    Village {
        /// Village name (ຊື່ບ້ານ)
        village: String,
    },
}

impl AdministrativeLevel {
    /// Get the Lao name for this administrative level
    /// ໄດ້ຊື່ພາສາລາວຂອງລະດັບບໍລິຫານນີ້
    pub fn level_name_lao(&self) -> &'static str {
        match self {
            AdministrativeLevel::Central { .. } => "ສູນກາງ",
            AdministrativeLevel::Provincial { .. } => "ແຂວງ",
            AdministrativeLevel::District { .. } => "ເມືອງ",
            AdministrativeLevel::Village { .. } => "ບ້ານ",
        }
    }

    /// Get the English name for this administrative level
    /// ໄດ້ຊື່ພາສາອັງກິດຂອງລະດັບບໍລິຫານນີ້
    pub fn level_name_en(&self) -> &'static str {
        match self {
            AdministrativeLevel::Central { .. } => "Central",
            AdministrativeLevel::Provincial { .. } => "Provincial",
            AdministrativeLevel::District { .. } => "District",
            AdministrativeLevel::Village { .. } => "Village",
        }
    }

    /// Get the jurisdiction limit in LAK for this level
    /// ໄດ້ຂອບເຂດອຳນາດເປັນກີບຂອງລະດັບນີ້
    pub fn jurisdiction_limit_lak(&self) -> Option<u64> {
        match self {
            AdministrativeLevel::Central { .. } => None, // Unlimited
            AdministrativeLevel::Provincial { .. } => Some(PROVINCIAL_JURISDICTION_LIMIT_LAK),
            AdministrativeLevel::District { .. } => Some(DISTRICT_JURISDICTION_LIMIT_LAK),
            AdministrativeLevel::Village { .. } => Some(VILLAGE_JURISDICTION_LIMIT_LAK),
        }
    }

    /// Get the hierarchy level (0 = highest)
    /// ໄດ້ລຳດັບຊັ້ນ (0 = ສູງສຸດ)
    pub fn hierarchy_level(&self) -> u8 {
        match self {
            AdministrativeLevel::Central { .. } => 0,
            AdministrativeLevel::Provincial { .. } => 1,
            AdministrativeLevel::District { .. } => 2,
            AdministrativeLevel::Village { .. } => 3,
        }
    }

    /// Check if this level is superior to another
    /// ກວດສອບວ່າລະດັບນີ້ສູງກວ່າລະດັບອື່ນຫຼືບໍ່
    pub fn is_superior_to(&self, other: &AdministrativeLevel) -> bool {
        self.hierarchy_level() < other.hierarchy_level()
    }

    /// Get the entity name for this administrative level
    /// ໄດ້ຊື່ໜ່ວຍງານຂອງລະດັບບໍລິຫານນີ້
    pub fn entity_name(&self) -> &str {
        match self {
            AdministrativeLevel::Central { ministry } => ministry,
            AdministrativeLevel::Provincial { province } => province,
            AdministrativeLevel::District { district } => district,
            AdministrativeLevel::Village { village } => village,
        }
    }
}

// ============================================================================
// License Types (ປະເພດໃບອະນຸຍາດ)
// ============================================================================

/// License types issued by administrative authorities
/// ປະເພດໃບອະນຸຍາດທີ່ອອກໂດຍອົງການບໍລິຫານ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LicenseType {
    /// Business license (ໃບອະນຸຍາດປະກອບທຸລະກິດ)
    BusinessLicense,

    /// Import/Export license (ໃບອະນຸຍາດນຳເຂົ້າ/ສົ່ງອອກ)
    ImportExportLicense,

    /// Construction license (ໃບອະນຸຍາດກໍ່ສ້າງ)
    ConstructionLicense,

    /// Environmental license (ໃບອະນຸຍາດສິ່ງແວດລ້ອມ)
    EnvironmentalLicense,

    /// Mining license (ໃບອະນຸຍາດຂຸດຄົ້ນບໍ່ແຮ່)
    MiningLicense,

    /// Tourism license (ໃບອະນຸຍາດທ່ອງທ່ຽວ)
    TourismLicense,

    /// Transport license (ໃບອະນຸຍາດຂົນສົ່ງ)
    TransportLicense,

    /// Professional license with specific profession
    /// ໃບອະນຸຍາດປະກອບອາຊີບ
    ProfessionalLicense {
        /// Profession name (ຊື່ອາຊີບ)
        profession: String,
    },

    /// Health/Medical license (ໃບອະນຸຍາດສາທາລະນະສຸກ)
    HealthLicense,

    /// Education license (ໃບອະນຸຍາດການສຶກສາ)
    EducationLicense,

    /// Food service license (ໃບອະນຸຍາດບໍລິການອາຫານ)
    FoodServiceLicense,

    /// Financial services license (ໃບອະນຸຍາດບໍລິການການເງິນ)
    FinancialServicesLicense,

    /// Other license type with description
    /// ປະເພດໃບອະນຸຍາດອື່ນໆ
    Other {
        /// Description (ລາຍລະອຽດ)
        description: String,
    },
}

impl LicenseType {
    /// Get the Lao name for this license type
    /// ໄດ້ຊື່ພາສາລາວຂອງປະເພດໃບອະນຸຍາດນີ້
    pub fn name_lao(&self) -> String {
        match self {
            LicenseType::BusinessLicense => "ໃບອະນຸຍາດປະກອບທຸລະກິດ".to_string(),
            LicenseType::ImportExportLicense => "ໃບອະນຸຍາດນຳເຂົ້າ/ສົ່ງອອກ".to_string(),
            LicenseType::ConstructionLicense => "ໃບອະນຸຍາດກໍ່ສ້າງ".to_string(),
            LicenseType::EnvironmentalLicense => "ໃບອະນຸຍາດສິ່ງແວດລ້ອມ".to_string(),
            LicenseType::MiningLicense => "ໃບອະນຸຍາດຂຸດຄົ້ນບໍ່ແຮ່".to_string(),
            LicenseType::TourismLicense => "ໃບອະນຸຍາດທ່ອງທ່ຽວ".to_string(),
            LicenseType::TransportLicense => "ໃບອະນຸຍາດຂົນສົ່ງ".to_string(),
            LicenseType::ProfessionalLicense { profession } => {
                format!("ໃບອະນຸຍາດປະກອບອາຊີບ: {}", profession)
            }
            LicenseType::HealthLicense => "ໃບອະນຸຍາດສາທາລະນະສຸກ".to_string(),
            LicenseType::EducationLicense => "ໃບອະນຸຍາດການສຶກສາ".to_string(),
            LicenseType::FoodServiceLicense => "ໃບອະນຸຍາດບໍລິການອາຫານ".to_string(),
            LicenseType::FinancialServicesLicense => "ໃບອະນຸຍາດບໍລິການການເງິນ".to_string(),
            LicenseType::Other { description } => format!("ໃບອະນຸຍາດອື່ນໆ: {}", description),
        }
    }

    /// Get the English name for this license type
    /// ໄດ້ຊື່ພາສາອັງກິດຂອງປະເພດໃບອະນຸຍາດນີ້
    pub fn name_en(&self) -> String {
        match self {
            LicenseType::BusinessLicense => "Business License".to_string(),
            LicenseType::ImportExportLicense => "Import/Export License".to_string(),
            LicenseType::ConstructionLicense => "Construction License".to_string(),
            LicenseType::EnvironmentalLicense => "Environmental License".to_string(),
            LicenseType::MiningLicense => "Mining License".to_string(),
            LicenseType::TourismLicense => "Tourism License".to_string(),
            LicenseType::TransportLicense => "Transport License".to_string(),
            LicenseType::ProfessionalLicense { profession } => {
                format!("Professional License: {}", profession)
            }
            LicenseType::HealthLicense => "Health/Medical License".to_string(),
            LicenseType::EducationLicense => "Education License".to_string(),
            LicenseType::FoodServiceLicense => "Food Service License".to_string(),
            LicenseType::FinancialServicesLicense => "Financial Services License".to_string(),
            LicenseType::Other { description } => format!("Other License: {}", description),
        }
    }

    /// Get the minimum required administrative level for issuing this license
    /// ໄດ້ລະດັບບໍລິຫານຂັ້ນຕ່ຳທີ່ຕ້ອງການສຳລັບການອອກໃບອະນຸຍາດນີ້
    pub fn minimum_authority_level(&self) -> u8 {
        match self {
            LicenseType::MiningLicense
            | LicenseType::FinancialServicesLicense
            | LicenseType::ImportExportLicense => 0, // Central only
            LicenseType::EnvironmentalLicense
            | LicenseType::ConstructionLicense
            | LicenseType::TourismLicense => 1, // Provincial or higher
            LicenseType::BusinessLicense
            | LicenseType::TransportLicense
            | LicenseType::HealthLicense
            | LicenseType::EducationLicense
            | LicenseType::FoodServiceLicense => 2, // District or higher
            LicenseType::ProfessionalLicense { .. } | LicenseType::Other { .. } => 2,
        }
    }
}

// ============================================================================
// Permit Types (ປະເພດໃບຢັ້ງຢືນ)
// ============================================================================

/// Permit types issued by administrative authorities
/// ປະເພດໃບຢັ້ງຢືນທີ່ອອກໂດຍອົງການບໍລິຫານ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermitType {
    /// Work permit for foreign nationals (ໃບອະນຸຍາດເຮັດວຽກ)
    WorkPermit {
        /// Nationality of permit holder (ສັນຊາດ)
        nationality: String,
    },

    /// Building permit (ໃບອະນຸຍາດກໍ່ສ້າງ)
    BuildingPermit,

    /// Environmental permit (ໃບຢັ້ງຢືນສິ່ງແວດລ້ອມ)
    EnvironmentalPermit,

    /// Land use permit (ໃບອະນຸຍາດນຳໃຊ້ທີ່ດິນ)
    LandUsePermit,

    /// Event permit (ໃບອະນຸຍາດຈັດງານ)
    EventPermit,

    /// Residence permit for foreigners (ໃບຢູ່ອາໄສສຳລັບຄົນຕ່າງດ້າວ)
    ResidencePermit {
        /// Nationality (ສັນຊາດ)
        nationality: String,
    },

    /// Vehicle registration permit (ໃບທະບຽນລົດ)
    VehicleRegistrationPermit,

    /// Firearm permit (ໃບອະນຸຍາດຄອບຄອງອາວຸດ)
    FirearmPermit,

    /// Temporary activity permit (ໃບອະນຸຍາດກິດຈະກຳຊົ່ວຄາວ)
    TemporaryActivityPermit {
        /// Activity description (ລາຍລະອຽດກິດຈະກຳ)
        activity: String,
    },

    /// Other permit type
    /// ປະເພດໃບຢັ້ງຢືນອື່ນໆ
    Other {
        /// Description (ລາຍລະອຽດ)
        description: String,
    },
}

impl PermitType {
    /// Get the Lao name for this permit type
    pub fn name_lao(&self) -> String {
        match self {
            PermitType::WorkPermit { nationality } => {
                format!("ໃບອະນຸຍາດເຮັດວຽກ (ສັນຊາດ: {})", nationality)
            }
            PermitType::BuildingPermit => "ໃບອະນຸຍາດກໍ່ສ້າງ".to_string(),
            PermitType::EnvironmentalPermit => "ໃບຢັ້ງຢືນສິ່ງແວດລ້ອມ".to_string(),
            PermitType::LandUsePermit => "ໃບອະນຸຍາດນຳໃຊ້ທີ່ດິນ".to_string(),
            PermitType::EventPermit => "ໃບອະນຸຍາດຈັດງານ".to_string(),
            PermitType::ResidencePermit { nationality } => {
                format!("ໃບຢູ່ອາໄສ (ສັນຊາດ: {})", nationality)
            }
            PermitType::VehicleRegistrationPermit => "ໃບທະບຽນລົດ".to_string(),
            PermitType::FirearmPermit => "ໃບອະນຸຍາດຄອບຄອງອາວຸດ".to_string(),
            PermitType::TemporaryActivityPermit { activity } => {
                format!("ໃບອະນຸຍາດກິດຈະກຳຊົ່ວຄາວ: {}", activity)
            }
            PermitType::Other { description } => format!("ໃບຢັ້ງຢືນອື່ນໆ: {}", description),
        }
    }

    /// Get the English name for this permit type
    pub fn name_en(&self) -> String {
        match self {
            PermitType::WorkPermit { nationality } => {
                format!("Work Permit (Nationality: {})", nationality)
            }
            PermitType::BuildingPermit => "Building Permit".to_string(),
            PermitType::EnvironmentalPermit => "Environmental Permit".to_string(),
            PermitType::LandUsePermit => "Land Use Permit".to_string(),
            PermitType::EventPermit => "Event Permit".to_string(),
            PermitType::ResidencePermit { nationality } => {
                format!("Residence Permit (Nationality: {})", nationality)
            }
            PermitType::VehicleRegistrationPermit => "Vehicle Registration Permit".to_string(),
            PermitType::FirearmPermit => "Firearm Permit".to_string(),
            PermitType::TemporaryActivityPermit { activity } => {
                format!("Temporary Activity Permit: {}", activity)
            }
            PermitType::Other { description } => format!("Other Permit: {}", description),
        }
    }
}

// ============================================================================
// Order Types (ປະເພດຄຳສັ່ງ)
// ============================================================================

/// Order types issued by administrative authorities
/// ປະເພດຄຳສັ່ງທີ່ອອກໂດຍອົງການບໍລິຫານ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderType {
    /// Cease and desist order (ຄຳສັ່ງໃຫ້ຢຸດເຊົາ)
    CeaseAndDesist,

    /// Compliance order (ຄຳສັ່ງໃຫ້ປະຕິບັດຕາມ)
    Compliance,

    /// Remediation order (ຄຳສັ່ງໃຫ້ແກ້ໄຂ)
    Remediation,

    /// Demolition order (ຄຳສັ່ງໃຫ້ລື້ຖອນ)
    Demolition,

    /// Closure order (ຄຳສັ່ງໃຫ້ປິດ)
    Closure,

    /// Payment order (ຄຳສັ່ງໃຫ້ຈ່າຍເງິນ)
    Payment {
        /// Amount in LAK (ຈຳນວນເງິນເປັນກີບ)
        amount_lak: u64,
    },

    /// Eviction order (ຄຳສັ່ງໃຫ້ອອກ)
    Eviction,

    /// Seizure order (ຄຳສັ່ງຍຶດ)
    Seizure,

    /// Inspection order (ຄຳສັ່ງໃຫ້ກວດກາ)
    Inspection,

    /// Other order type
    /// ປະເພດຄຳສັ່ງອື່ນໆ
    Other {
        /// Description (ລາຍລະອຽດ)
        description: String,
    },
}

impl OrderType {
    /// Get the Lao name for this order type
    pub fn name_lao(&self) -> &'static str {
        match self {
            OrderType::CeaseAndDesist => "ຄຳສັ່ງໃຫ້ຢຸດເຊົາ",
            OrderType::Compliance => "ຄຳສັ່ງໃຫ້ປະຕິບັດຕາມ",
            OrderType::Remediation => "ຄຳສັ່ງໃຫ້ແກ້ໄຂ",
            OrderType::Demolition => "ຄຳສັ່ງໃຫ້ລື້ຖອນ",
            OrderType::Closure => "ຄຳສັ່ງໃຫ້ປິດ",
            OrderType::Payment { .. } => "ຄຳສັ່ງໃຫ້ຈ່າຍເງິນ",
            OrderType::Eviction => "ຄຳສັ່ງໃຫ້ອອກ",
            OrderType::Seizure => "ຄຳສັ່ງຍຶດ",
            OrderType::Inspection => "ຄຳສັ່ງໃຫ້ກວດກາ",
            OrderType::Other { .. } => "ຄຳສັ່ງອື່ນໆ",
        }
    }

    /// Get the English name for this order type
    pub fn name_en(&self) -> &'static str {
        match self {
            OrderType::CeaseAndDesist => "Cease and Desist Order",
            OrderType::Compliance => "Compliance Order",
            OrderType::Remediation => "Remediation Order",
            OrderType::Demolition => "Demolition Order",
            OrderType::Closure => "Closure Order",
            OrderType::Payment { .. } => "Payment Order",
            OrderType::Eviction => "Eviction Order",
            OrderType::Seizure => "Seizure Order",
            OrderType::Inspection => "Inspection Order",
            OrderType::Other { .. } => "Other Order",
        }
    }
}

// ============================================================================
// Decision Types (ປະເພດການຕັດສິນໃຈ)
// ============================================================================

/// Decision types for administrative decisions
/// ປະເພດການຕັດສິນໃຈບໍລິຫານ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecisionType {
    /// License issuance (ການອອກໃບອະນຸຍາດ)
    License {
        /// License type (ປະເພດໃບອະນຸຍາດ)
        license_type: LicenseType,
    },

    /// Permit issuance (ການອອກໃບຢັ້ງຢືນ)
    Permit {
        /// Permit type (ປະເພດໃບຢັ້ງຢືນ)
        permit_type: PermitType,
    },

    /// General approval (ການອະນຸມັດທົ່ວໄປ)
    Approval,

    /// Denial of application (ການປະຕິເສດຄຳຮ້ອງ)
    Denial,

    /// Revocation of existing authorization (ການຖອນຄືນການອະນຸຍາດ)
    Revocation,

    /// Suspension of authorization (ການລະງັບການອະນຸຍາດ)
    Suspension,

    /// Warning (ການເຕືອນ)
    Warning,

    /// Fine imposition (ການປັບໄໝ)
    Fine {
        /// Amount in LAK (ຈຳນວນເງິນເປັນກີບ)
        amount_lak: u64,
    },

    /// Administrative order (ຄຳສັ່ງບໍລິຫານ)
    Order {
        /// Order type (ປະເພດຄຳສັ່ງ)
        order_type: OrderType,
    },

    /// Registration decision (ການຕັດສິນໃຈການລົງທະບຽນ)
    Registration {
        /// Registration type (ປະເພດການລົງທະບຽນ)
        registration_type: String,
    },

    /// Certification decision (ການຢັ້ງຢືນ)
    Certification {
        /// Certification type (ປະເພດການຢັ້ງຢືນ)
        certification_type: String,
    },
}

impl DecisionType {
    /// Get the Lao name for this decision type
    pub fn name_lao(&self) -> String {
        match self {
            DecisionType::License { license_type } => {
                format!("ການອອກໃບອະນຸຍາດ: {}", license_type.name_lao())
            }
            DecisionType::Permit { permit_type } => {
                format!("ການອອກໃບຢັ້ງຢືນ: {}", permit_type.name_lao())
            }
            DecisionType::Approval => "ການອະນຸມັດ".to_string(),
            DecisionType::Denial => "ການປະຕິເສດ".to_string(),
            DecisionType::Revocation => "ການຖອນຄືນ".to_string(),
            DecisionType::Suspension => "ການລະງັບ".to_string(),
            DecisionType::Warning => "ການເຕືອນ".to_string(),
            DecisionType::Fine { amount_lak } => {
                format!("ການປັບໄໝ: {} ກີບ", amount_lak)
            }
            DecisionType::Order { order_type } => {
                format!("ຄຳສັ່ງບໍລິຫານ: {}", order_type.name_lao())
            }
            DecisionType::Registration { registration_type } => {
                format!("ການລົງທະບຽນ: {}", registration_type)
            }
            DecisionType::Certification { certification_type } => {
                format!("ການຢັ້ງຢືນ: {}", certification_type)
            }
        }
    }

    /// Get the English name for this decision type
    pub fn name_en(&self) -> String {
        match self {
            DecisionType::License { license_type } => {
                format!("License Issuance: {}", license_type.name_en())
            }
            DecisionType::Permit { permit_type } => {
                format!("Permit Issuance: {}", permit_type.name_en())
            }
            DecisionType::Approval => "Approval".to_string(),
            DecisionType::Denial => "Denial".to_string(),
            DecisionType::Revocation => "Revocation".to_string(),
            DecisionType::Suspension => "Suspension".to_string(),
            DecisionType::Warning => "Warning".to_string(),
            DecisionType::Fine { amount_lak } => {
                format!("Fine: {} LAK", amount_lak)
            }
            DecisionType::Order { order_type } => {
                format!("Administrative Order: {}", order_type.name_en())
            }
            DecisionType::Registration { registration_type } => {
                format!("Registration: {}", registration_type)
            }
            DecisionType::Certification { certification_type } => {
                format!("Certification: {}", certification_type)
            }
        }
    }
}

// ============================================================================
// Legal Basis (ພື້ນຖານທາງກົດໝາຍ)
// ============================================================================

/// Legal basis for administrative decisions
/// ພື້ນຖານທາງກົດໝາຍສຳລັບການຕັດສິນໃຈບໍລິຫານ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LegalBasis {
    /// Law name in Lao (ຊື່ກົດໝາຍເປັນພາສາລາວ)
    pub law_name_lao: String,

    /// Law name in English (ຊື່ກົດໝາຍເປັນພາສາອັງກິດ)
    pub law_name_en: String,

    /// Article number (ເລກມາດຕາ)
    pub article_number: u16,

    /// Paragraph number if applicable (ເລກວັກ)
    pub paragraph: Option<u8>,
}

impl LegalBasis {
    /// Create a new legal basis
    pub fn new(
        law_name_lao: impl Into<String>,
        law_name_en: impl Into<String>,
        article_number: u16,
        paragraph: Option<u8>,
    ) -> Self {
        Self {
            law_name_lao: law_name_lao.into(),
            law_name_en: law_name_en.into(),
            article_number,
            paragraph,
        }
    }

    /// Get formatted citation in Lao
    pub fn citation_lao(&self) -> String {
        match self.paragraph {
            Some(p) => format!(
                "{}, ມາດຕາ {}, ວັກ {}",
                self.law_name_lao, self.article_number, p
            ),
            None => format!("{}, ມາດຕາ {}", self.law_name_lao, self.article_number),
        }
    }

    /// Get formatted citation in English
    pub fn citation_en(&self) -> String {
        match self.paragraph {
            Some(p) => format!(
                "{}, Article {}, Paragraph {}",
                self.law_name_en, self.article_number, p
            ),
            None => format!("{}, Article {}", self.law_name_en, self.article_number),
        }
    }
}

// ============================================================================
// Affected Party (ຝ່າຍທີ່ໄດ້ຮັບຜົນກະທົບ)
// ============================================================================

/// Types of parties that can be affected by administrative decisions
/// ປະເພດຂອງຝ່າຍທີ່ອາດໄດ້ຮັບຜົນກະທົບຈາກການຕັດສິນໃຈບໍລິຫານ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartyType {
    /// Individual person (ບຸກຄົນ)
    Individual,

    /// Legal entity / company (ນິຕິບຸກຄົນ)
    LegalEntity,

    /// Government agency (ອົງການລັດຖະບານ)
    GovernmentAgency,

    /// Association / organization (ສະມາຄົມ/ອົງການຈັດຕັ້ງ)
    Association,

    /// Foreign national (ຄົນຕ່າງປະເທດ)
    ForeignNational {
        /// Nationality (ສັນຊາດ)
        nationality: String,
    },

    /// Foreign entity (ນິຕິບຸກຄົນຕ່າງປະເທດ)
    ForeignEntity {
        /// Country of registration (ປະເທດທີ່ຈົດທະບຽນ)
        country: String,
    },
}

impl PartyType {
    /// Get the Lao name for this party type
    pub fn name_lao(&self) -> String {
        match self {
            PartyType::Individual => "ບຸກຄົນ".to_string(),
            PartyType::LegalEntity => "ນິຕິບຸກຄົນ".to_string(),
            PartyType::GovernmentAgency => "ອົງການລັດຖະບານ".to_string(),
            PartyType::Association => "ສະມາຄົມ/ອົງການຈັດຕັ້ງ".to_string(),
            PartyType::ForeignNational { nationality } => {
                format!("ຄົນຕ່າງປະເທດ ({})", nationality)
            }
            PartyType::ForeignEntity { country } => {
                format!("ນິຕິບຸກຄົນຕ່າງປະເທດ ({})", country)
            }
        }
    }

    /// Get the English name for this party type
    pub fn name_en(&self) -> String {
        match self {
            PartyType::Individual => "Individual".to_string(),
            PartyType::LegalEntity => "Legal Entity".to_string(),
            PartyType::GovernmentAgency => "Government Agency".to_string(),
            PartyType::Association => "Association/Organization".to_string(),
            PartyType::ForeignNational { nationality } => {
                format!("Foreign National ({})", nationality)
            }
            PartyType::ForeignEntity { country } => {
                format!("Foreign Entity ({})", country)
            }
        }
    }
}

/// Affected party in an administrative decision
/// ຝ່າຍທີ່ໄດ້ຮັບຜົນກະທົບໃນການຕັດສິນໃຈບໍລິຫານ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AffectedParty {
    /// Party name (ຊື່ຝ່າຍ)
    pub party_name: String,

    /// Party type (ປະເພດຝ່າຍ)
    pub party_type: PartyType,

    /// Notification date if notified (ວັນທີແຈ້ງ)
    pub notification_date: Option<String>,

    /// Whether party has been notified (ໄດ້ແຈ້ງຫຼືບໍ່)
    pub is_notified: bool,
}

impl AffectedParty {
    /// Create a new affected party
    pub fn new(party_name: impl Into<String>, party_type: PartyType) -> Self {
        Self {
            party_name: party_name.into(),
            party_type,
            notification_date: None,
            is_notified: false,
        }
    }

    /// Set notification status
    pub fn with_notification(mut self, date: impl Into<String>) -> Self {
        self.notification_date = Some(date.into());
        self.is_notified = true;
        self
    }
}

// ============================================================================
// Administrative Decision (ການຕັດສິນໃຈບໍລິຫານ)
// ============================================================================

/// Administrative decision issued by a government authority
/// ການຕັດສິນໃຈບໍລິຫານທີ່ອອກໂດຍອົງການລັດຖະບານ
///
/// ## Legal Requirements
///
/// An administrative decision must contain:
/// 1. Decision number and issuing authority
/// 2. Date of issuance
/// 3. Legal basis (applicable laws and articles)
/// 4. Subject matter in both Lao and English
/// 5. Affected parties and notification status
/// 6. Appeal deadline information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdministrativeDecision {
    /// Decision number (ເລກທີການຕັດສິນໃຈ)
    pub decision_number: String,

    /// Issuing authority (ອົງການອອກ)
    pub issuing_authority: AdministrativeLevel,

    /// Decision date (ວັນທີຕັດສິນໃຈ)
    pub decision_date: String,

    /// Subject in Lao (ຫົວຂໍ້ເປັນພາສາລາວ)
    pub subject_lao: String,

    /// Subject in English (ຫົວຂໍ້ເປັນພາສາອັງກິດ)
    pub subject_en: String,

    /// Decision type (ປະເພດການຕັດສິນໃຈ)
    pub decision_type: DecisionType,

    /// Legal basis (ພື້ນຖານທາງກົດໝາຍ)
    pub legal_basis: Vec<LegalBasis>,

    /// Affected parties (ຝ່າຍທີ່ໄດ້ຮັບຜົນກະທົບ)
    pub affected_parties: Vec<AffectedParty>,

    /// Whether decision is final (ເປັນການຕັດສິນໃຈສຸດທ້າຍຫຼືບໍ່)
    pub is_final: bool,

    /// Appeal deadline in days (ກຳນົດເວລາອຸທອນເປັນວັນ)
    pub appeal_deadline_days: Option<u8>,

    /// Reasoning for the decision (ເຫດຜົນ)
    pub reasoning: Option<String>,

    /// Attachments (ເອກະສານແນບ)
    pub attachments: Vec<String>,
}

impl AdministrativeDecision {
    /// Create a new builder for AdministrativeDecision
    pub fn builder() -> AdministrativeDecisionBuilder {
        AdministrativeDecisionBuilder::default()
    }

    /// Check if all affected parties have been notified
    pub fn all_parties_notified(&self) -> bool {
        self.affected_parties.iter().all(|p| p.is_notified)
    }

    /// Get the number of days remaining until appeal deadline
    pub fn days_until_appeal_deadline(&self, _current_date: &str) -> Option<i32> {
        // This is a simplified implementation - in production would use proper date parsing
        self.appeal_deadline_days.map(|days| days as i32)
    }
}

/// Builder for AdministrativeDecision
/// ຕົວສ້າງສຳລັບ AdministrativeDecision
#[derive(Debug, Default)]
pub struct AdministrativeDecisionBuilder {
    decision_number: Option<String>,
    issuing_authority: Option<AdministrativeLevel>,
    decision_date: Option<String>,
    subject_lao: Option<String>,
    subject_en: Option<String>,
    decision_type: Option<DecisionType>,
    legal_basis: Vec<LegalBasis>,
    affected_parties: Vec<AffectedParty>,
    is_final: bool,
    appeal_deadline_days: Option<u8>,
    reasoning: Option<String>,
    attachments: Vec<String>,
}

impl AdministrativeDecisionBuilder {
    /// Set decision number
    pub fn decision_number(mut self, number: String) -> Self {
        self.decision_number = Some(number);
        self
    }

    /// Set issuing authority
    pub fn issuing_authority(mut self, authority: AdministrativeLevel) -> Self {
        self.issuing_authority = Some(authority);
        self
    }

    /// Set decision date
    pub fn decision_date(mut self, date: String) -> Self {
        self.decision_date = Some(date);
        self
    }

    /// Set subject in Lao
    pub fn subject_lao(mut self, subject: String) -> Self {
        self.subject_lao = Some(subject);
        self
    }

    /// Set subject in English
    pub fn subject_en(mut self, subject: String) -> Self {
        self.subject_en = Some(subject);
        self
    }

    /// Set decision type
    pub fn decision_type(mut self, dtype: DecisionType) -> Self {
        self.decision_type = Some(dtype);
        self
    }

    /// Add legal basis
    pub fn legal_basis(mut self, basis: LegalBasis) -> Self {
        self.legal_basis.push(basis);
        self
    }

    /// Add affected party
    pub fn affected_party(mut self, party: AffectedParty) -> Self {
        self.affected_parties.push(party);
        self
    }

    /// Set whether decision is final
    pub fn is_final(mut self, is_final: bool) -> Self {
        self.is_final = is_final;
        self
    }

    /// Set appeal deadline in days
    pub fn appeal_deadline_days(mut self, days: Option<u8>) -> Self {
        self.appeal_deadline_days = days;
        self
    }

    /// Set reasoning
    pub fn reasoning(mut self, reasoning: String) -> Self {
        self.reasoning = Some(reasoning);
        self
    }

    /// Add attachment
    pub fn attachment(mut self, attachment: String) -> Self {
        self.attachments.push(attachment);
        self
    }

    /// Build the AdministrativeDecision
    pub fn build(self) -> Result<AdministrativeDecision, String> {
        let decision_number = self.decision_number.ok_or("decision_number is required")?;
        let issuing_authority = self
            .issuing_authority
            .ok_or("issuing_authority is required")?;
        let decision_date = self.decision_date.ok_or("decision_date is required")?;
        let subject_lao = self.subject_lao.ok_or("subject_lao is required")?;
        let subject_en = self.subject_en.ok_or("subject_en is required")?;
        let decision_type = self.decision_type.ok_or("decision_type is required")?;

        if self.legal_basis.is_empty() {
            return Err("at least one legal_basis is required".to_string());
        }

        Ok(AdministrativeDecision {
            decision_number,
            issuing_authority,
            decision_date,
            subject_lao,
            subject_en,
            decision_type,
            legal_basis: self.legal_basis,
            affected_parties: self.affected_parties,
            is_final: self.is_final,
            appeal_deadline_days: self.appeal_deadline_days,
            reasoning: self.reasoning,
            attachments: self.attachments,
        })
    }
}

// ============================================================================
// Administrative Sanctions (ມາດຕະການລົງໂທດບໍລິຫານ)
// ============================================================================

/// Sanction types for administrative violations
/// ປະເພດການລົງໂທດສຳລັບການລະເມີດບໍລິຫານ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SanctionType {
    /// Warning - written or oral (ການເຕືອນ)
    Warning {
        /// Whether it is a written warning (ເປັນລາຍລັກອັກສອນຫຼືບໍ່)
        written: bool,
    },

    /// Fine with payment deadline (ການປັບໄໝ)
    Fine {
        /// Amount in LAK (ຈຳນວນເງິນເປັນກີບ)
        amount_lak: u64,
        /// Payment deadline date (ກຳນົດເວລາຈ່າຍ)
        payment_deadline: String,
    },

    /// License suspension (ການລະງັບໃບອະນຸຍາດ)
    LicenseSuspension {
        /// Duration in days (ໄລຍະເວລາເປັນວັນ)
        duration_days: u32,
    },

    /// License revocation (ການຖອນໃບອະນຸຍາດ)
    LicenseRevocation,

    /// Business closure (ການປິດກິດຈະການ)
    BusinessClosure {
        /// Whether temporary (ເປັນການປິດຊົ່ວຄາວຫຼືບໍ່)
        temporary: bool,
        /// Duration in days if temporary (ໄລຍະເວລາເປັນວັນຖ້າເປັນການປິດຊົ່ວຄາວ)
        duration_days: Option<u32>,
    },

    /// Activity prohibition (ການຫ້າມກິດຈະກຳ)
    ActivityProhibition {
        /// Prohibited activity (ກິດຈະກຳທີ່ຖືກຫ້າມ)
        activity: String,
    },

    /// Confiscation (ການຍຶດ)
    Confiscation {
        /// Description of confiscated items (ລາຍລະອຽດຂອງສິ່ງທີ່ຖືກຍຶດ)
        items: String,
    },

    /// Disqualification from profession (ການຫ້າມປະກອບອາຊີບ)
    Disqualification {
        /// Duration in months (ໄລຍະເວລາເປັນເດືອນ)
        duration_months: u32,
        /// Profession (ອາຊີບ)
        profession: String,
    },

    /// Combined sanctions (ການລົງໂທດລວມ)
    Combined {
        /// List of sanctions (ລາຍການລົງໂທດ)
        sanctions: Vec<Box<SanctionType>>,
    },
}

impl SanctionType {
    /// Get the Lao name for this sanction type
    pub fn name_lao(&self) -> String {
        match self {
            SanctionType::Warning { written } => {
                if *written {
                    "ການເຕືອນເປັນລາຍລັກອັກສອນ".to_string()
                } else {
                    "ການເຕືອນດ້ວຍວາຈາ".to_string()
                }
            }
            SanctionType::Fine { amount_lak, .. } => {
                format!("ການປັບໄໝ {} ກີບ", amount_lak)
            }
            SanctionType::LicenseSuspension { duration_days } => {
                format!("ການລະງັບໃບອະນຸຍາດ {} ວັນ", duration_days)
            }
            SanctionType::LicenseRevocation => "ການຖອນໃບອະນຸຍາດ".to_string(),
            SanctionType::BusinessClosure {
                temporary,
                duration_days,
            } => {
                if *temporary {
                    format!("ການປິດກິດຈະການຊົ່ວຄາວ {} ວັນ", duration_days.unwrap_or(0))
                } else {
                    "ການປິດກິດຈະການຖາວອນ".to_string()
                }
            }
            SanctionType::ActivityProhibition { activity } => {
                format!("ການຫ້າມກິດຈະກຳ: {}", activity)
            }
            SanctionType::Confiscation { items } => {
                format!("ການຍຶດ: {}", items)
            }
            SanctionType::Disqualification {
                duration_months,
                profession,
            } => {
                format!(
                    "ການຫ້າມປະກອບອາຊີບ {} ເປັນເວລາ {} ເດືອນ",
                    profession, duration_months
                )
            }
            SanctionType::Combined { sanctions } => {
                format!("ການລົງໂທດລວມ ({} ລາຍການ)", sanctions.len())
            }
        }
    }

    /// Get the English name for this sanction type
    pub fn name_en(&self) -> String {
        match self {
            SanctionType::Warning { written } => {
                if *written {
                    "Written Warning".to_string()
                } else {
                    "Oral Warning".to_string()
                }
            }
            SanctionType::Fine { amount_lak, .. } => {
                format!("Fine: {} LAK", amount_lak)
            }
            SanctionType::LicenseSuspension { duration_days } => {
                format!("License Suspension: {} days", duration_days)
            }
            SanctionType::LicenseRevocation => "License Revocation".to_string(),
            SanctionType::BusinessClosure {
                temporary,
                duration_days,
            } => {
                if *temporary {
                    format!(
                        "Temporary Business Closure: {} days",
                        duration_days.unwrap_or(0)
                    )
                } else {
                    "Permanent Business Closure".to_string()
                }
            }
            SanctionType::ActivityProhibition { activity } => {
                format!("Activity Prohibition: {}", activity)
            }
            SanctionType::Confiscation { items } => {
                format!("Confiscation: {}", items)
            }
            SanctionType::Disqualification {
                duration_months,
                profession,
            } => {
                format!(
                    "Professional Disqualification: {} for {} months",
                    profession, duration_months
                )
            }
            SanctionType::Combined { sanctions } => {
                format!("Combined Sanctions ({} items)", sanctions.len())
            }
        }
    }

    /// Get the severity level (1-5)
    pub fn severity_level(&self) -> u8 {
        match self {
            SanctionType::Warning { written: false } => 1,
            SanctionType::Warning { written: true } => 2,
            SanctionType::Fine { amount_lak, .. } => {
                if *amount_lak < 1_000_000 {
                    2
                } else if *amount_lak < 10_000_000 {
                    3
                } else {
                    4
                }
            }
            SanctionType::LicenseSuspension { duration_days } => {
                if *duration_days <= 30 {
                    3
                } else if *duration_days <= 90 {
                    4
                } else {
                    5
                }
            }
            SanctionType::LicenseRevocation => 5,
            SanctionType::BusinessClosure { temporary, .. } => {
                if *temporary {
                    4
                } else {
                    5
                }
            }
            SanctionType::ActivityProhibition { .. } => 3,
            SanctionType::Confiscation { .. } => 4,
            SanctionType::Disqualification { .. } => 4,
            SanctionType::Combined { sanctions } => sanctions
                .iter()
                .map(|s| s.severity_level())
                .max()
                .unwrap_or(1),
        }
    }
}

/// Administrative sanction
/// ມາດຕະການລົງໂທດບໍລິຫານ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdministrativeSanction {
    /// Sanction ID (ລະຫັດການລົງໂທດ)
    pub sanction_id: String,

    /// Sanction type (ປະເພດການລົງໂທດ)
    pub sanction_type: SanctionType,

    /// Issuing authority (ອົງການອອກ)
    pub issuing_authority: AdministrativeLevel,

    /// Legal basis (ພື້ນຖານທາງກົດໝາຍ)
    pub legal_basis: LegalBasis,

    /// Violation description in Lao (ລາຍລະອຽດການລະເມີດເປັນພາສາລາວ)
    pub violation_description_lao: String,

    /// Violation description in English (ລາຍລະອຽດການລະເມີດເປັນພາສາອັງກິດ)
    pub violation_description_en: String,

    /// Sanction date (ວັນທີລົງໂທດ)
    pub sanction_date: String,

    /// Whether appeal is available (ມີສິດອຸທອນຫຼືບໍ່)
    pub appeal_available: bool,

    /// Subject of sanction (ຜູ້ຖືກລົງໂທດ)
    pub subject: AffectedParty,

    /// Appeal deadline in days (ກຳນົດເວລາອຸທອນເປັນວັນ)
    pub appeal_deadline_days: u8,
}

impl AdministrativeSanction {
    /// Create a new builder for AdministrativeSanction
    pub fn builder() -> AdministrativeSanctionBuilder {
        AdministrativeSanctionBuilder::default()
    }
}

/// Builder for AdministrativeSanction
#[derive(Debug, Default)]
pub struct AdministrativeSanctionBuilder {
    sanction_id: Option<String>,
    sanction_type: Option<SanctionType>,
    issuing_authority: Option<AdministrativeLevel>,
    legal_basis: Option<LegalBasis>,
    violation_description_lao: Option<String>,
    violation_description_en: Option<String>,
    sanction_date: Option<String>,
    appeal_available: bool,
    subject: Option<AffectedParty>,
    appeal_deadline_days: u8,
}

impl AdministrativeSanctionBuilder {
    /// Set sanction ID
    pub fn sanction_id(mut self, id: String) -> Self {
        self.sanction_id = Some(id);
        self
    }

    /// Set sanction type
    pub fn sanction_type(mut self, stype: SanctionType) -> Self {
        self.sanction_type = Some(stype);
        self
    }

    /// Set issuing authority
    pub fn issuing_authority(mut self, authority: AdministrativeLevel) -> Self {
        self.issuing_authority = Some(authority);
        self
    }

    /// Set legal basis
    pub fn legal_basis(mut self, basis: LegalBasis) -> Self {
        self.legal_basis = Some(basis);
        self
    }

    /// Set violation description in Lao
    pub fn violation_description_lao(mut self, description: String) -> Self {
        self.violation_description_lao = Some(description);
        self
    }

    /// Set violation description in English
    pub fn violation_description_en(mut self, description: String) -> Self {
        self.violation_description_en = Some(description);
        self
    }

    /// Set sanction date
    pub fn sanction_date(mut self, date: String) -> Self {
        self.sanction_date = Some(date);
        self
    }

    /// Set whether appeal is available
    pub fn appeal_available(mut self, available: bool) -> Self {
        self.appeal_available = available;
        self
    }

    /// Set subject of sanction
    pub fn subject(mut self, subject: AffectedParty) -> Self {
        self.subject = Some(subject);
        self
    }

    /// Set appeal deadline in days
    pub fn appeal_deadline_days(mut self, days: u8) -> Self {
        self.appeal_deadline_days = days;
        self
    }

    /// Build the AdministrativeSanction
    pub fn build(self) -> Result<AdministrativeSanction, String> {
        Ok(AdministrativeSanction {
            sanction_id: self.sanction_id.ok_or("sanction_id is required")?,
            sanction_type: self.sanction_type.ok_or("sanction_type is required")?,
            issuing_authority: self
                .issuing_authority
                .ok_or("issuing_authority is required")?,
            legal_basis: self.legal_basis.ok_or("legal_basis is required")?,
            violation_description_lao: self
                .violation_description_lao
                .ok_or("violation_description_lao is required")?,
            violation_description_en: self
                .violation_description_en
                .ok_or("violation_description_en is required")?,
            sanction_date: self.sanction_date.ok_or("sanction_date is required")?,
            appeal_available: self.appeal_available,
            subject: self.subject.ok_or("subject is required")?,
            appeal_deadline_days: if self.appeal_deadline_days == 0 {
                ADMINISTRATIVE_APPEAL_DEADLINE_DAYS
            } else {
                self.appeal_deadline_days
            },
        })
    }
}

// ============================================================================
// Administrative Appeals (ການອຸທອນບໍລິຫານ)
// ============================================================================

/// Appeal grounds for administrative appeals
/// ເຫດຜົນສຳລັບການອຸທອນບໍລິຫານ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppealGround {
    /// Procedural error (ຄວາມຜິດພາດດ້ານຂັ້ນຕອນ)
    ProceduralError {
        /// Description of error (ລາຍລະອຽດຄວາມຜິດພາດ)
        description: String,
    },

    /// Factual error (ຄວາມຜິດພາດດ້ານຂໍ້ເທັດຈິງ)
    FactualError {
        /// Description of error (ລາຍລະອຽດຄວາມຜິດພາດ)
        description: String,
    },

    /// Legal error (ຄວາມຜິດພາດດ້ານກົດໝາຍ)
    LegalError {
        /// Description of error (ລາຍລະອຽດຄວາມຜິດພາດ)
        description: String,
    },

    /// Excess of authority (ການໃຊ້ອຳນາດເກີນຂອບເຂດ)
    ExcessOfAuthority,

    /// Violation of rights (ການລະເມີດສິດ)
    ViolationOfRights {
        /// The right that was violated (ສິດທີ່ຖືກລະເມີດ)
        right: String,
    },

    /// Disproportionate sanction (ການລົງໂທດບໍ່ສົມເຫດສົມຜົນ)
    DisproportionateSanction,

    /// New evidence (ພະຍານໃໝ່)
    NewEvidence {
        /// Description of new evidence (ລາຍລະອຽດພະຍານໃໝ່)
        description: String,
    },

    /// Lack of notification (ຂາດການແຈ້ງ)
    LackOfNotification,

    /// Bias or conflict of interest (ການລຳອຽງຫຼືຜົນປະໂຫຍດທັບຊ້ອນ)
    BiasOrConflict,
}

impl AppealGround {
    /// Get the Lao name for this appeal ground
    pub fn name_lao(&self) -> String {
        match self {
            AppealGround::ProceduralError { description } => {
                format!("ຄວາມຜິດພາດດ້ານຂັ້ນຕອນ: {}", description)
            }
            AppealGround::FactualError { description } => {
                format!("ຄວາມຜິດພາດດ້ານຂໍ້ເທັດຈິງ: {}", description)
            }
            AppealGround::LegalError { description } => {
                format!("ຄວາມຜິດພາດດ້ານກົດໝາຍ: {}", description)
            }
            AppealGround::ExcessOfAuthority => "ການໃຊ້ອຳນາດເກີນຂອບເຂດ".to_string(),
            AppealGround::ViolationOfRights { right } => {
                format!("ການລະເມີດສິດ: {}", right)
            }
            AppealGround::DisproportionateSanction => "ການລົງໂທດບໍ່ສົມເຫດສົມຜົນ".to_string(),
            AppealGround::NewEvidence { description } => {
                format!("ພະຍານໃໝ່: {}", description)
            }
            AppealGround::LackOfNotification => "ຂາດການແຈ້ງ".to_string(),
            AppealGround::BiasOrConflict => "ການລຳອຽງຫຼືຜົນປະໂຫຍດທັບຊ້ອນ".to_string(),
        }
    }

    /// Get the English name for this appeal ground
    pub fn name_en(&self) -> String {
        match self {
            AppealGround::ProceduralError { description } => {
                format!("Procedural Error: {}", description)
            }
            AppealGround::FactualError { description } => {
                format!("Factual Error: {}", description)
            }
            AppealGround::LegalError { description } => {
                format!("Legal Error: {}", description)
            }
            AppealGround::ExcessOfAuthority => "Excess of Authority".to_string(),
            AppealGround::ViolationOfRights { right } => {
                format!("Violation of Rights: {}", right)
            }
            AppealGround::DisproportionateSanction => "Disproportionate Sanction".to_string(),
            AppealGround::NewEvidence { description } => {
                format!("New Evidence: {}", description)
            }
            AppealGround::LackOfNotification => "Lack of Notification".to_string(),
            AppealGround::BiasOrConflict => "Bias or Conflict of Interest".to_string(),
        }
    }
}

/// Appeal level for administrative appeals
/// ລະດັບການອຸທອນ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppealLevel {
    /// Same authority reconsideration (ການພິຈາລະນາຄືນໂດຍອົງການເດີມ)
    SameAuthority,

    /// Superior authority appeal (ການອຸທອນຕໍ່ອົງການຂັ້ນເທິງ)
    SuperiorAuthority {
        /// Name of the superior authority (ຊື່ອົງການຂັ້ນເທິງ)
        authority: String,
    },

    /// Administrative court appeal (ການຟ້ອງຕໍ່ສານບໍລິຫານ)
    AdministrativeCourt,

    /// Supreme court appeal (ການຟ້ອງຕໍ່ສານສູງສຸດ)
    SupremeCourt,
}

impl AppealLevel {
    /// Get the deadline for this appeal level in days
    pub fn deadline_days(&self) -> u8 {
        match self {
            AppealLevel::SameAuthority => ADMINISTRATIVE_APPEAL_DEADLINE_DAYS,
            AppealLevel::SuperiorAuthority { .. } => ADMINISTRATIVE_APPEAL_DEADLINE_DAYS,
            AppealLevel::AdministrativeCourt => COURT_APPEAL_DEADLINE_DAYS,
            AppealLevel::SupremeCourt => COURT_APPEAL_DEADLINE_DAYS,
        }
    }

    /// Get the Lao name for this appeal level
    pub fn name_lao(&self) -> String {
        match self {
            AppealLevel::SameAuthority => "ການພິຈາລະນາຄືນໂດຍອົງການເດີມ".to_string(),
            AppealLevel::SuperiorAuthority { authority } => {
                format!("ການອຸທອນຕໍ່ອົງການຂັ້ນເທິງ: {}", authority)
            }
            AppealLevel::AdministrativeCourt => "ການຟ້ອງຕໍ່ສານບໍລິຫານ".to_string(),
            AppealLevel::SupremeCourt => "ການຟ້ອງຕໍ່ສານສູງສຸດ".to_string(),
        }
    }

    /// Get the English name for this appeal level
    pub fn name_en(&self) -> String {
        match self {
            AppealLevel::SameAuthority => "Reconsideration by Same Authority".to_string(),
            AppealLevel::SuperiorAuthority { authority } => {
                format!("Appeal to Superior Authority: {}", authority)
            }
            AppealLevel::AdministrativeCourt => "Administrative Court Appeal".to_string(),
            AppealLevel::SupremeCourt => "Supreme Court Appeal".to_string(),
        }
    }
}

/// Appeal status
/// ສະຖານະການອຸທອນ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppealStatus {
    /// Appeal filed (ໄດ້ຍື່ນອຸທອນແລ້ວ)
    Filed,

    /// Under review (ກຳລັງພິຈາລະນາ)
    UnderReview,

    /// Hearing scheduled (ກຳນົດມື້ພິຈາລະນາແລ້ວ)
    HearingScheduled {
        /// Hearing date (ວັນທີພິຈາລະນາ)
        date: String,
    },

    /// Appeal decided (ໄດ້ຕັດສິນແລ້ວ)
    Decided {
        /// Outcome of appeal (ຜົນການອຸທອນ)
        outcome: AppealOutcome,
    },

    /// Appeal withdrawn (ຖອນຄຳອຸທອນແລ້ວ)
    Withdrawn,

    /// Appeal dismissed (ຍົກຄຳອຸທອນ)
    Dismissed,
}

/// Appeal outcome
/// ຜົນການອຸທອນ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppealOutcome {
    /// Original decision upheld (ຮັບຮອງການຕັດສິນໃຈເດີມ)
    Upheld,

    /// Decision modified (ດັດແກ້ການຕັດສິນໃຈ)
    Modified {
        /// Description of new decision (ລາຍລະອຽດການຕັດສິນໃຈໃໝ່)
        new_decision: String,
    },

    /// Decision reversed (ຍົກເລີກການຕັດສິນໃຈເດີມ)
    Reversed,

    /// Case remanded (ສົ່ງຄືນເພື່ອພິຈາລະນາໃໝ່)
    Remanded {
        /// Reason for remand (ເຫດຜົນ)
        reason: String,
    },
}

impl AppealOutcome {
    /// Get the Lao name for this outcome
    pub fn name_lao(&self) -> String {
        match self {
            AppealOutcome::Upheld => "ຮັບຮອງການຕັດສິນໃຈເດີມ".to_string(),
            AppealOutcome::Modified { new_decision } => {
                format!("ດັດແກ້ການຕັດສິນໃຈ: {}", new_decision)
            }
            AppealOutcome::Reversed => "ຍົກເລີກການຕັດສິນໃຈເດີມ".to_string(),
            AppealOutcome::Remanded { reason } => {
                format!("ສົ່ງຄືນເພື່ອພິຈາລະນາໃໝ່: {}", reason)
            }
        }
    }

    /// Get the English name for this outcome
    pub fn name_en(&self) -> String {
        match self {
            AppealOutcome::Upheld => "Original Decision Upheld".to_string(),
            AppealOutcome::Modified { new_decision } => {
                format!("Decision Modified: {}", new_decision)
            }
            AppealOutcome::Reversed => "Decision Reversed".to_string(),
            AppealOutcome::Remanded { reason } => {
                format!("Remanded for Reconsideration: {}", reason)
            }
        }
    }
}

/// Administrative appeal
/// ການອຸທອນບໍລິຫານ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdministrativeAppeal {
    /// Appeal number (ເລກທີອຸທອນ)
    pub appeal_number: String,

    /// Original decision number being appealed (ເລກທີການຕັດສິນໃຈເດີມ)
    pub original_decision: String,

    /// Appellant (ຜູ້ອຸທອນ)
    pub appellant: AffectedParty,

    /// Appeal grounds (ເຫດຜົນການອຸທອນ)
    pub appeal_grounds: Vec<AppealGround>,

    /// Filing date (ວັນທີຍື່ນອຸທອນ)
    pub filing_date: String,

    /// Appeal level (ລະດັບການອຸທອນ)
    pub appeal_level: AppealLevel,

    /// Appeal status (ສະຖານະ)
    pub status: AppealStatus,

    /// Deadline date (ກຳນົດເວລາ)
    pub deadline_date: String,

    /// Supporting documents (ເອກະສານສະໜັບສະໜູນ)
    pub supporting_documents: Vec<String>,
}

impl AdministrativeAppeal {
    /// Create a new builder for AdministrativeAppeal
    pub fn builder() -> AdministrativeAppealBuilder {
        AdministrativeAppealBuilder::default()
    }
}

/// Builder for AdministrativeAppeal
#[derive(Debug, Default)]
pub struct AdministrativeAppealBuilder {
    appeal_number: Option<String>,
    original_decision: Option<String>,
    appellant: Option<AffectedParty>,
    appeal_grounds: Vec<AppealGround>,
    filing_date: Option<String>,
    appeal_level: Option<AppealLevel>,
    status: Option<AppealStatus>,
    deadline_date: Option<String>,
    supporting_documents: Vec<String>,
}

impl AdministrativeAppealBuilder {
    /// Set appeal number
    pub fn appeal_number(mut self, number: String) -> Self {
        self.appeal_number = Some(number);
        self
    }

    /// Set original decision number
    pub fn original_decision(mut self, decision: String) -> Self {
        self.original_decision = Some(decision);
        self
    }

    /// Set appellant
    pub fn appellant(mut self, appellant: AffectedParty) -> Self {
        self.appellant = Some(appellant);
        self
    }

    /// Add appeal ground
    pub fn appeal_ground(mut self, ground: AppealGround) -> Self {
        self.appeal_grounds.push(ground);
        self
    }

    /// Set filing date
    pub fn filing_date(mut self, date: String) -> Self {
        self.filing_date = Some(date);
        self
    }

    /// Set appeal level
    pub fn appeal_level(mut self, level: AppealLevel) -> Self {
        self.appeal_level = Some(level);
        self
    }

    /// Set status
    pub fn status(mut self, status: AppealStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Set deadline date
    pub fn deadline_date(mut self, date: String) -> Self {
        self.deadline_date = Some(date);
        self
    }

    /// Add supporting document
    pub fn supporting_document(mut self, doc: String) -> Self {
        self.supporting_documents.push(doc);
        self
    }

    /// Build the AdministrativeAppeal
    pub fn build(self) -> Result<AdministrativeAppeal, String> {
        if self.appeal_grounds.is_empty() {
            return Err("at least one appeal_ground is required".to_string());
        }

        Ok(AdministrativeAppeal {
            appeal_number: self.appeal_number.ok_or("appeal_number is required")?,
            original_decision: self
                .original_decision
                .ok_or("original_decision is required")?,
            appellant: self.appellant.ok_or("appellant is required")?,
            appeal_grounds: self.appeal_grounds,
            filing_date: self.filing_date.ok_or("filing_date is required")?,
            appeal_level: self.appeal_level.ok_or("appeal_level is required")?,
            status: self.status.unwrap_or(AppealStatus::Filed),
            deadline_date: self.deadline_date.ok_or("deadline_date is required")?,
            supporting_documents: self.supporting_documents,
        })
    }
}

// ============================================================================
// State Liability (ຄວາມຮັບຜິດຊອບຂອງລັດ)
// ============================================================================

/// Liability types for state liability claims
/// ປະເພດຄວາມຮັບຜິດຊອບສຳລັບການຮ້ອງຂໍຄ່າເສຍຫາຍຈາກລັດ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LiabilityType {
    /// Wrongful administrative decision (ການຕັດສິນໃຈບໍລິຫານທີ່ຜິດກົດໝາຍ)
    WrongfulDecision,

    /// Procedural violation (ການລະເມີດຂັ້ນຕອນ)
    ProceduralViolation,

    /// Negligence (ການລະເລີຍ)
    Negligence,

    /// Excess of authority (ການໃຊ້ອຳນາດເກີນຂອບເຂດ)
    ExcessOfAuthority,

    /// Delay in action (ການລ່າຊ້າ)
    DelayInAction,

    /// Wrongful arrest (ການຈັບກຸມທີ່ຜິດກົດໝາຍ)
    WrongfulArrest,

    /// Property damage (ຄວາມເສຍຫາຍຕໍ່ຊັບສິນ)
    PropertyDamage,

    /// Personal injury (ການບາດເຈັບສ່ວນບຸກຄົນ)
    PersonalInjury,

    /// Economic loss (ການສູນເສຍທາງເສດຖະກິດ)
    EconomicLoss,

    /// Wrongful detention (ການຄຸມຂັງທີ່ຜິດກົດໝາຍ)
    WrongfulDetention,
}

impl LiabilityType {
    /// Get the Lao name for this liability type
    pub fn name_lao(&self) -> &'static str {
        match self {
            LiabilityType::WrongfulDecision => "ການຕັດສິນໃຈບໍລິຫານທີ່ຜິດກົດໝາຍ",
            LiabilityType::ProceduralViolation => "ການລະເມີດຂັ້ນຕອນ",
            LiabilityType::Negligence => "ການລະເລີຍ",
            LiabilityType::ExcessOfAuthority => "ການໃຊ້ອຳນາດເກີນຂອບເຂດ",
            LiabilityType::DelayInAction => "ການລ່າຊ້າ",
            LiabilityType::WrongfulArrest => "ການຈັບກຸມທີ່ຜິດກົດໝາຍ",
            LiabilityType::PropertyDamage => "ຄວາມເສຍຫາຍຕໍ່ຊັບສິນ",
            LiabilityType::PersonalInjury => "ການບາດເຈັບສ່ວນບຸກຄົນ",
            LiabilityType::EconomicLoss => "ການສູນເສຍທາງເສດຖະກິດ",
            LiabilityType::WrongfulDetention => "ການຄຸມຂັງທີ່ຜິດກົດໝາຍ",
        }
    }

    /// Get the English name for this liability type
    pub fn name_en(&self) -> &'static str {
        match self {
            LiabilityType::WrongfulDecision => "Wrongful Administrative Decision",
            LiabilityType::ProceduralViolation => "Procedural Violation",
            LiabilityType::Negligence => "Negligence",
            LiabilityType::ExcessOfAuthority => "Excess of Authority",
            LiabilityType::DelayInAction => "Delay in Action",
            LiabilityType::WrongfulArrest => "Wrongful Arrest",
            LiabilityType::PropertyDamage => "Property Damage",
            LiabilityType::PersonalInjury => "Personal Injury",
            LiabilityType::EconomicLoss => "Economic Loss",
            LiabilityType::WrongfulDetention => "Wrongful Detention",
        }
    }
}

/// Claim status for state liability claims
/// ສະຖານະການຮ້ອງຂໍຄ່າເສຍຫາຍຈາກລັດ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClaimStatus {
    /// Claim filed (ໄດ້ຍື່ນຄຳຮ້ອງແລ້ວ)
    Filed,

    /// Under investigation (ກຳລັງສືບສວນ)
    UnderInvestigation,

    /// In negotiation (ກຳລັງເຈລະຈາ)
    Negotiation,

    /// Claim accepted (ຍອมຮັບຄຳຮ້ອງ)
    Accepted {
        /// Amount accepted in LAK (ຈຳນວນເງິນທີ່ຍອມຮັບເປັນກີບ)
        amount_lak: u64,
    },

    /// Claim rejected (ປະຕິເສດຄຳຮ້ອງ)
    Rejected {
        /// Reason for rejection (ເຫດຜົນ)
        reason: String,
    },

    /// Court proceeding (ການດຳເນີນຄະດີຕໍ່ສານ)
    CourtProceeding,

    /// Claim settled (ຕົກລົງແລ້ວ)
    Settled {
        /// Settlement amount in LAK (ຈຳນວນເງິນທີ່ຕົກລົງເປັນກີບ)
        amount_lak: u64,
    },
}

impl ClaimStatus {
    /// Get the Lao name for this status
    pub fn name_lao(&self) -> String {
        match self {
            ClaimStatus::Filed => "ໄດ້ຍື່ນຄຳຮ້ອງແລ້ວ".to_string(),
            ClaimStatus::UnderInvestigation => "ກຳລັງສືບສວນ".to_string(),
            ClaimStatus::Negotiation => "ກຳລັງເຈລະຈາ".to_string(),
            ClaimStatus::Accepted { amount_lak } => {
                format!("ຍອມຮັບຄຳຮ້ອງ: {} ກີບ", amount_lak)
            }
            ClaimStatus::Rejected { reason } => {
                format!("ປະຕິເສດຄຳຮ້ອງ: {}", reason)
            }
            ClaimStatus::CourtProceeding => "ການດຳເນີນຄະດີຕໍ່ສານ".to_string(),
            ClaimStatus::Settled { amount_lak } => {
                format!("ຕົກລົງແລ້ວ: {} ກີບ", amount_lak)
            }
        }
    }

    /// Get the English name for this status
    pub fn name_en(&self) -> String {
        match self {
            ClaimStatus::Filed => "Claim Filed".to_string(),
            ClaimStatus::UnderInvestigation => "Under Investigation".to_string(),
            ClaimStatus::Negotiation => "In Negotiation".to_string(),
            ClaimStatus::Accepted { amount_lak } => {
                format!("Claim Accepted: {} LAK", amount_lak)
            }
            ClaimStatus::Rejected { reason } => {
                format!("Claim Rejected: {}", reason)
            }
            ClaimStatus::CourtProceeding => "Court Proceeding".to_string(),
            ClaimStatus::Settled { amount_lak } => {
                format!("Settled: {} LAK", amount_lak)
            }
        }
    }
}

/// State liability claim
/// ການຮ້ອງຂໍຄ່າເສຍຫາຍຈາກລັດ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateLiability {
    /// Claim number (ເລກທີຄຳຮ້ອງ)
    pub claim_number: String,

    /// Claimant (ຜູ້ຮ້ອງຂໍ)
    pub claimant: AffectedParty,

    /// Responsible authority (ອົງການທີ່ຮັບຜິດຊອບ)
    pub responsible_authority: AdministrativeLevel,

    /// Liability type (ປະເພດຄວາມຮັບຜິດຊອບ)
    pub liability_type: LiabilityType,

    /// Damage description in Lao (ລາຍລະອຽດຄວາມເສຍຫາຍເປັນພາສາລາວ)
    pub damage_description_lao: String,

    /// Damage description in English (ລາຍລະອຽດຄວາມເສຍຫາຍເປັນພາສາອັງກິດ)
    pub damage_description_en: String,

    /// Claimed amount in LAK (ຈຳນວນເງິນທີ່ຮ້ອງຂໍເປັນກີບ)
    pub claimed_amount_lak: u64,

    /// Claim status (ສະຖານະ)
    pub claim_status: ClaimStatus,

    /// Date of wrongful act (ວັນທີເກີດການກະທຳຜິດ)
    pub wrongful_act_date: Option<String>,

    /// Filing date (ວັນທີຍື່ນຄຳຮ້ອງ)
    pub filing_date: Option<String>,

    /// Supporting evidence (ພະຍານສະໜັບສະໜູນ)
    pub supporting_evidence: Vec<String>,
}

impl StateLiability {
    /// Create a new state liability claim
    pub fn new(
        claim_number: impl Into<String>,
        claimant: AffectedParty,
        responsible_authority: AdministrativeLevel,
        liability_type: LiabilityType,
        damage_description_lao: impl Into<String>,
        damage_description_en: impl Into<String>,
        claimed_amount_lak: u64,
    ) -> Self {
        Self {
            claim_number: claim_number.into(),
            claimant,
            responsible_authority,
            liability_type,
            damage_description_lao: damage_description_lao.into(),
            damage_description_en: damage_description_en.into(),
            claimed_amount_lak,
            claim_status: ClaimStatus::Filed,
            wrongful_act_date: None,
            filing_date: None,
            supporting_evidence: Vec::new(),
        }
    }

    /// Set wrongful act date
    pub fn with_wrongful_act_date(mut self, date: impl Into<String>) -> Self {
        self.wrongful_act_date = Some(date.into());
        self
    }

    /// Set filing date
    pub fn with_filing_date(mut self, date: impl Into<String>) -> Self {
        self.filing_date = Some(date.into());
        self
    }

    /// Add supporting evidence
    pub fn with_evidence(mut self, evidence: impl Into<String>) -> Self {
        self.supporting_evidence.push(evidence.into());
        self
    }

    /// Update claim status
    pub fn with_status(mut self, status: ClaimStatus) -> Self {
        self.claim_status = status;
        self
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_administrative_level_hierarchy() {
        let central = AdministrativeLevel::Central {
            ministry: "Ministry of Justice".to_string(),
        };
        let provincial = AdministrativeLevel::Provincial {
            province: "Vientiane".to_string(),
        };
        let district = AdministrativeLevel::District {
            district: "Sisattanak".to_string(),
        };
        let village = AdministrativeLevel::Village {
            village: "Ban Nongbone".to_string(),
        };

        assert!(central.is_superior_to(&provincial));
        assert!(provincial.is_superior_to(&district));
        assert!(district.is_superior_to(&village));
        assert!(!village.is_superior_to(&central));
    }

    #[test]
    fn test_administrative_decision_builder() {
        let decision = AdministrativeDecision::builder()
            .decision_number("DEC-2024-001".to_string())
            .issuing_authority(AdministrativeLevel::Central {
                ministry: "Ministry of Industry and Commerce".to_string(),
            })
            .decision_date("2024-01-15".to_string())
            .subject_lao("ການອອກໃບອະນຸຍາດປະກອບທຸລະກິດ".to_string())
            .subject_en("Business License Issuance".to_string())
            .decision_type(DecisionType::License {
                license_type: LicenseType::BusinessLicense,
            })
            .legal_basis(LegalBasis::new(
                "ກົດໝາຍວ່າດ້ວຍວິສາຫະກິດ",
                "Enterprise Law",
                15,
                Some(1),
            ))
            .affected_party(AffectedParty::new(
                "ABC Company Ltd.",
                PartyType::LegalEntity,
            ))
            .is_final(false)
            .appeal_deadline_days(Some(30))
            .build();

        assert!(decision.is_ok());
        let decision = decision.expect("Failed to build decision");
        assert_eq!(decision.decision_number, "DEC-2024-001");
    }

    #[test]
    fn test_administrative_sanction_builder() {
        let sanction = AdministrativeSanction::builder()
            .sanction_id("SANC-2024-001".to_string())
            .sanction_type(SanctionType::Fine {
                amount_lak: 5_000_000,
                payment_deadline: "2024-02-15".to_string(),
            })
            .issuing_authority(AdministrativeLevel::Provincial {
                province: "Vientiane".to_string(),
            })
            .legal_basis(LegalBasis::new("ກົດໝາຍວ່າດ້ວຍພາສີ", "Tax Law", 50, None))
            .violation_description_lao("ການຍື່ນພາສີຊ້າ".to_string())
            .violation_description_en("Late tax filing".to_string())
            .sanction_date("2024-01-20".to_string())
            .appeal_available(true)
            .subject(AffectedParty::new("Company XYZ", PartyType::LegalEntity))
            .appeal_deadline_days(30)
            .build();

        assert!(sanction.is_ok());
    }

    #[test]
    fn test_administrative_appeal_builder() {
        let appeal = AdministrativeAppeal::builder()
            .appeal_number("APP-2024-001".to_string())
            .original_decision("DEC-2024-001".to_string())
            .appellant(AffectedParty::new("John Doe", PartyType::Individual))
            .appeal_ground(AppealGround::ProceduralError {
                description: "Not properly notified".to_string(),
            })
            .filing_date("2024-02-01".to_string())
            .appeal_level(AppealLevel::SuperiorAuthority {
                authority: "Ministry of Justice".to_string(),
            })
            .deadline_date("2024-02-15".to_string())
            .build();

        assert!(appeal.is_ok());
    }

    #[test]
    fn test_state_liability_creation() {
        let claim = StateLiability::new(
            "SLC-2024-001",
            AffectedParty::new("Jane Smith", PartyType::Individual),
            AdministrativeLevel::Provincial {
                province: "Savannakhet".to_string(),
            },
            LiabilityType::WrongfulDecision,
            "ຄວາມເສຍຫາຍຈາກການຕັດສິນໃຈທີ່ຜິດກົດໝາຍ",
            "Damage from wrongful administrative decision",
            50_000_000,
        )
        .with_wrongful_act_date("2023-11-01")
        .with_filing_date("2024-01-15")
        .with_evidence("Witness statement");

        assert_eq!(claim.claim_number, "SLC-2024-001");
        assert_eq!(claim.claimed_amount_lak, 50_000_000);
    }

    #[test]
    fn test_sanction_severity_levels() {
        let warning = SanctionType::Warning { written: false };
        let small_fine = SanctionType::Fine {
            amount_lak: 500_000,
            payment_deadline: "2024-02-15".to_string(),
        };
        let revocation = SanctionType::LicenseRevocation;

        assert_eq!(warning.severity_level(), 1);
        assert_eq!(small_fine.severity_level(), 2);
        assert_eq!(revocation.severity_level(), 5);
    }

    #[test]
    fn test_legal_basis_citations() {
        let basis = LegalBasis::new("ກົດໝາຍວ່າດ້ວຍວິສາຫະກິດ", "Enterprise Law", 15, Some(2));

        assert!(basis.citation_lao().contains("ມາດຕາ 15"));
        assert!(basis.citation_lao().contains("ວັກ 2"));
        assert!(basis.citation_en().contains("Article 15"));
        assert!(basis.citation_en().contains("Paragraph 2"));
    }

    #[test]
    fn test_appeal_level_deadlines() {
        assert_eq!(
            AppealLevel::SameAuthority.deadline_days(),
            ADMINISTRATIVE_APPEAL_DEADLINE_DAYS
        );
        assert_eq!(
            AppealLevel::AdministrativeCourt.deadline_days(),
            COURT_APPEAL_DEADLINE_DAYS
        );
    }

    #[test]
    fn test_license_type_authority_levels() {
        assert_eq!(LicenseType::MiningLicense.minimum_authority_level(), 0);
        assert_eq!(
            LicenseType::EnvironmentalLicense.minimum_authority_level(),
            1
        );
        assert_eq!(LicenseType::BusinessLicense.minimum_authority_level(), 2);
    }
}
