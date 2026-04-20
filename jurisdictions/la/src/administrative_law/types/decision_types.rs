//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use serde::{Deserialize, Serialize};

use super::functions::{ADMINISTRATIVE_APPEAL_DEADLINE_DAYS, COURT_APPEAL_DEADLINE_DAYS};
use super::types_2::{
    AdministrativeAppeal, AdministrativeDecisionBuilder, AdministrativeLevel, AppealStatus,
    ClaimStatus, LiabilityType, PartyType, PermitType, SanctionType,
};

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
            LicenseType::Other { description } => {
                format!("ໃບອະນຸຍາດອື່ນໆ: {}", description)
            }
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
            LicenseType::Other { description } => {
                format!("Other License: {}", description)
            }
        }
    }
    /// Get the minimum required administrative level for issuing this license
    /// ໄດ້ລະດັບບໍລິຫານຂັ້ນຕ່ຳທີ່ຕ້ອງການສຳລັບການອອກໃບອະນຸຍາດນີ້
    pub fn minimum_authority_level(&self) -> u8 {
        match self {
            LicenseType::MiningLicense
            | LicenseType::FinancialServicesLicense
            | LicenseType::ImportExportLicense => 0,
            LicenseType::EnvironmentalLicense
            | LicenseType::ConstructionLicense
            | LicenseType::TourismLicense => 1,
            LicenseType::BusinessLicense
            | LicenseType::TransportLicense
            | LicenseType::HealthLicense
            | LicenseType::EducationLicense
            | LicenseType::FoodServiceLicense => 2,
            LicenseType::ProfessionalLicense { .. } | LicenseType::Other { .. } => 2,
        }
    }
}
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
        self.appeal_deadline_days.map(|days| days as i32)
    }
}
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
            DecisionType::Fine { amount_lak } => format!("Fine: {} LAK", amount_lak),
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
/// Builder for AdministrativeSanction
#[derive(Debug, Default)]
pub struct AdministrativeSanctionBuilder {
    pub(super) sanction_id: Option<String>,
    pub(super) sanction_type: Option<SanctionType>,
    pub(super) issuing_authority: Option<AdministrativeLevel>,
    pub(super) legal_basis: Option<LegalBasis>,
    pub(super) violation_description_lao: Option<String>,
    pub(super) violation_description_en: Option<String>,
    pub(super) sanction_date: Option<String>,
    pub(super) appeal_available: bool,
    pub(super) subject: Option<AffectedParty>,
    pub(super) appeal_deadline_days: u8,
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
/// Builder for AdministrativeAppeal
#[derive(Debug, Default)]
pub struct AdministrativeAppealBuilder {
    pub(super) appeal_number: Option<String>,
    pub(super) original_decision: Option<String>,
    pub(super) appellant: Option<AffectedParty>,
    pub(super) appeal_grounds: Vec<AppealGround>,
    pub(super) filing_date: Option<String>,
    pub(super) appeal_level: Option<AppealLevel>,
    pub(super) status: Option<AppealStatus>,
    pub(super) deadline_date: Option<String>,
    pub(super) supporting_documents: Vec<String>,
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
            Some(p) => {
                format!(
                    "{}, ມາດຕາ {}, ວັກ {}",
                    self.law_name_lao, self.article_number, p
                )
            }
            None => {
                format!("{}, ມາດຕາ {}", self.law_name_lao, self.article_number)
            }
        }
    }
    /// Get formatted citation in English
    pub fn citation_en(&self) -> String {
        match self.paragraph {
            Some(p) => {
                format!(
                    "{}, Article {}, Paragraph {}",
                    self.law_name_en, self.article_number, p
                )
            }
            None => format!("{}, Article {}", self.law_name_en, self.article_number),
        }
    }
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
