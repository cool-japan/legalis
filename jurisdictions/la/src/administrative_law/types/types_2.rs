//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use serde::{Deserialize, Serialize};

use super::decision_types::{
    AdministrativeAppealBuilder, AdministrativeDecision, AffectedParty, AppealGround, AppealLevel,
    AppealOutcome, DecisionType, LegalBasis,
};
use super::functions::{
    DISTRICT_JURISDICTION_LIMIT_LAK, PROVINCIAL_JURISDICTION_LIMIT_LAK,
    VILLAGE_JURISDICTION_LIMIT_LAK,
};

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
            PermitType::Other { description } => {
                format!("ໃບຢັ້ງຢືນອື່ນໆ: {}", description)
            }
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
            SanctionType::Fine { amount_lak, .. } => format!("Fine: {} LAK", amount_lak),
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
            SanctionType::Confiscation { items } => format!("Confiscation: {}", items),
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
            ClaimStatus::Rejected { reason } => format!("Claim Rejected: {}", reason),
            ClaimStatus::CourtProceeding => "Court Proceeding".to_string(),
            ClaimStatus::Settled { amount_lak } => format!("Settled: {} LAK", amount_lak),
        }
    }
}
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
            AdministrativeLevel::Central { .. } => None,
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
