//! Constitutional types for the Lao People's Democratic Republic.
//!
//! This module implements types representing the Constitution of Lao PDR
//! (ລັດຖະທຳມະນູນ ແຫ່ງ ສາທາລະນະລັດ ປະຊາທິປະໄຕ ປະຊາຊົນລາວ)
//!
//! **Legal Basis:** Constitution of 1991, amended 2003 and 2015
//!
//! ## Structure
//! - Political Regime (ລະບອບການເມືອງ) - Articles 1-4
//! - Rights and Obligations of Citizens (ສິດ ແລະ ໜ້າທີ່ ຂອງພົນລະເມືອງ) - Articles 34-51
//! - National Assembly (ສະພາແຫ່ງຊາດ) - Articles 52-65
//! - President of the State (ປະທານປະເທດ) - Articles 66-70
//! - Government (ລັດຖະບານ) - Articles 71-80
//! - Local Administration (ການປົກຄອງທ້ອງຖິ່ນ) - Articles 81-87
//! - People's Courts (ສານປະຊາຊົນ) - Articles 88-95
//! - People's Prosecutors (ອົງການໄອຍະການປະຊາຊົນ) - Articles 96-100
//! - Constitutional Amendment (ການແກ້ໄຂເພີ່ມເຕີມລັດຖະທຳມະນູນ) - Articles 105-108

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// Political Regime (ລະບອບການເມືອງ) - Articles 1-4
// ============================================================================

/// Political regime of Lao PDR (Article 1)
/// ລະບອບການເມືອງ ຂອງ ສປປ ລາວ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PoliticalRegime {
    /// State form: People's Democratic State
    /// ປະເພດລັດ: ລັດປະຊາທິປະໄຕປະຊາຊົນ
    pub state_form: StateForm,

    /// Socialist orientation with market economy (Article 4)
    /// ທິດທາງສັງຄົມນິຍົມ ພ້ອມດ້ວຍເສດຖະກິດຕະຫຼາດ
    pub economic_system: EconomicSystem,

    /// Multi-ethnic people as master (Article 2)
    /// ປະຊາຊົນບັນດາເຜົ່າເປັນເຈົ້າ
    pub sovereignty: Sovereignty,
}

/// State form of Lao PDR
/// ປະເພດລັດ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateForm {
    /// People's Democratic State (Article 1)
    /// ລັດປະຊາທິປະໄຕປະຊາຊົນ
    PeoplesDemocraticState,
}

/// Economic system (Article 4)
/// ລະບົບເສດຖະກິດ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EconomicSystem {
    /// Socialist orientation with market economy
    /// ທິດທາງສັງຄົມນິຍົມ ພ້ອມດ້ວຍເສດຖະກິດຕະຫຼາດ
    SocialistMarketEconomy,
}

/// Sovereignty of the people (Article 2)
/// ອຳນາດອະທິປະໄຕຂອງປະຊາຊົນ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Sovereignty {
    /// The multi-ethnic people are the masters of the country
    /// ປະຊາຊົນບັນດາເຜົ່າເປັນເຈົ້າຂອງປະເທດ
    pub lao_text: String,

    /// English translation
    pub english_text: String,

    /// Exercise through National Assembly and other state organs
    /// ການໃຊ້ສິດຜ່ານສະພາແຫ່ງຊາດ ແລະ ອົງການລັດອື່ນໆ
    pub exercised_through: Vec<StateOrgan>,
}

/// State organs through which sovereignty is exercised
/// ອົງການລັດ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateOrgan {
    /// National Assembly (Articles 52-65)
    /// ສະພາແຫ່ງຊາດ
    NationalAssembly,

    /// President of the State (Articles 66-70)
    /// ປະທານປະເທດ
    President,

    /// Government (Articles 71-80)
    /// ລັດຖະບານ
    Government,

    /// People's Courts (Articles 88-95)
    /// ສານປະຊາຊົນ
    PeoplesCourts,

    /// People's Prosecutors (Articles 96-100)
    /// ອົງການໄອຍະການປະຊາຊົນ
    PeoplesProsecutors,

    /// Local Administration (Articles 81-87)
    /// ການປົກຄອງທ້ອງຖິ່ນ
    LocalAdministration,
}

// ============================================================================
// Fundamental Rights and Duties (ສິດ ແລະ ໜ້າທີ່) - Articles 34-51
// ============================================================================

/// Fundamental rights guaranteed by the Constitution (Chapter VI)
/// ສິດພື້ນຖານທີ່ລັດຖະທຳມະນູນຮັບປະກັນ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FundamentalRight {
    /// Equality before the law (Article 34)
    /// ຄວາມສະເໝີພາບຕໍ່ໜ້າກົດໝາຍ
    Equality,

    /// Right to vote (Article 35) - 18 years and above
    /// ສິດເລືອກຕັ້ງ - ອາຍຸ 18 ປີຂຶ້ນໄປ
    VotingRight { age_requirement: u8 },

    /// Freedom of speech, press, and assembly (Article 36)
    /// ເສລີພາບໃນການປາກເວົ້າ, ການພິມ, ການຊຸມນຸມ
    FreedomOfExpression,

    /// Freedom of religion and belief (Article 37)
    /// ເສລີພາບໃນການນັບຖືສາສະໜາ ແລະ ຄວາມເຊື່ອ
    FreedomOfReligion,

    /// Right to petition (Article 38)
    /// ສິດຍື່ນຄຳຮ້ອງທຸກ
    RightToPetition,

    /// Inviolability of person and home (Article 39)
    /// ການບໍ່ລະເມີດຕໍ່ບຸກຄົນ ແລະ ທີ່ຢູ່ອາໄສ
    InviolabilityOfPerson,

    /// Right to privacy and correspondence (Article 40)
    /// ສິດຄວາມເປັນສ່ວນຕົວ ແລະ ຈົດໝາຍ
    RightToPrivacy,

    /// Right to property (Article 41)
    /// ສິດມີຊັບສິນ
    RightToProperty,

    /// Right to work and rest (Article 42)
    /// ສິດໃນການເຮັດວຽກ ແລະ ພັກຜ່ອນ
    RightToWork,

    /// Right to education (Article 43)
    /// ສິດໃນການສຶກສາ
    RightToEducation,

    /// Right to healthcare (Article 44)
    /// ສິດໃນການດູແລສຸຂະພາບ
    RightToHealthcare,

    /// Right to welfare and social security (Article 45)
    /// ສິດໃນສະຫວັດດີການ ແລະ ປະກັນສັງຄົມ
    RightToWelfare,

    /// Gender equality (Article 46)
    /// ຄວາມສະເໝີພາບລະຫວ່າງຍິງ-ຊາຍ
    GenderEquality,

    /// Protection of families and children (Article 47)
    /// ການປົກປ້ອງຄອບຄົວ ແລະ ເດັກນ້ອຍ
    ProtectionOfFamilies,

    /// Protection of elderly (Article 48)
    /// ການປົກປ້ອງຜູ້ສູງອາຍຸ
    ProtectionOfElderly,
}

/// Fundamental duties of citizens (Articles 49-51)
/// ໜ້າທີ່ພື້ນຖານຂອງພົນລະເມືອງ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FundamentalDuty {
    /// Duty to defend the nation (Article 49)
    /// ໜ້າທີ່ປົກປ້ອງຊາດ
    DefendNation,

    /// Duty to pay taxes (Article 50)
    /// ໜ້າທີ່ເສຍພາສີ
    PayTaxes,

    /// Duty to protect the environment (Article 51)
    /// ໜ້າທີ່ປົກປ້ອງສິ່ງແວດລ້ອມ
    ProtectEnvironment,
}

/// Rights limitation framework (proportionality test)
/// ກອບການຈຳກັດສິດ (ການທົດສອບຄວາມສົມເຫດສົມຜົນ)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RightsLimitation {
    /// The right being limited
    /// ສິດທີ່ຖືກຈຳກັດ
    pub right: FundamentalRight,

    /// Legitimate aim (public order, national security, public health, morals)
    /// ຈຸດປະສົງທີ່ຊອບທຳ
    pub legitimate_aim: LegitimateAim,

    /// Necessity test: Is the limitation necessary?
    /// ການທົດສອບຄວາມຈຳເປັນ
    pub is_necessary: bool,

    /// Proportionality test: Is the limitation proportional to the aim?
    /// ການທົດສອບຄວາມສົມສ່ວນ
    pub is_proportional: bool,

    /// Legal basis for limitation
    /// ພື້ນຖານທາງກົດໝາຍ
    pub legal_basis: String,
}

/// Legitimate aims for limiting fundamental rights
/// ຈຸດປະສົງທີ່ຊອບທຳ ສຳລັບການຈຳກັດສິດພື້ນຖານ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegitimateAim {
    /// Public order and safety
    /// ຄວາມສະຫງົບເລຽບຮ້ອຍ ແລະ ຄວາມປອດໄພສາທາລະນະ
    PublicOrder,

    /// National security
    /// ຄວາມໝັ້ນຄົງແຫ່ງຊາດ
    NationalSecurity,

    /// Public health
    /// ສຸຂະພາບອະນາໄມສາທາລະນະ
    PublicHealth,

    /// Public morals
    /// ສິນທຳສາທາລະນະ
    PublicMorals,

    /// Rights and freedoms of others
    /// ສິດ ແລະ ເສລີພາບຂອງຜູ້ອື່ນ
    RightsOfOthers,
}

// ============================================================================
// National Assembly (ສະພາແຫ່ງຊາດ) - Articles 52-65
// ============================================================================

/// National Assembly - Legislative body of Lao PDR (Article 52)
/// ສະພາແຫ່ງຊາດ - ອົງການນິຕິບັນຍັດຂອງ ສປປ ລາວ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NationalAssembly {
    /// Session number (legislature)
    /// ສະໄໝ
    pub session: u32,

    /// Number of members
    /// ຈຳນວນສະມາຊິກ
    pub members: u32,

    /// Term: 5 years (Article 53)
    /// ວາລະ: 5 ປີ
    pub term_years: u8,

    /// Elected by universal, direct, and secret ballot
    /// ເລືອກຕັ້ງໂດຍການລົງຄະແນນສຽງທົ່ວໄປ, ໂດຍກົງ ແລະ ລັບ
    pub election_method: ElectionMethod,

    /// Powers and functions (Articles 54-56)
    /// ອຳນາດ ແລະ ໜ້າທີ່
    pub powers: Vec<NationalAssemblyPower>,

    /// Standing Committee (Articles 63-65)
    /// ຄະນະປະຈຳສະພາແຫ່ງຊາດ
    pub standing_committee: Option<StandingCommittee>,
}

/// Election method (Article 53)
/// ວິທີການເລືອກຕັ້ງ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ElectionMethod {
    /// Universal suffrage
    /// ການເລືອກຕັ້ງທົ່ວໄປ
    pub universal: bool,

    /// Direct election
    /// ການເລືອກຕັ້ງໂດຍກົງ
    pub direct: bool,

    /// Secret ballot
    /// ການລົງຄະແນນລັບ
    pub secret: bool,
}

/// Powers of the National Assembly (Articles 54-56)
/// ອຳນາດຂອງສະພາແຫ່ງຊາດ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NationalAssemblyPower {
    /// Adopt and amend the Constitution (Article 54.1)
    /// ຮັບຮອງ ແລະ ແກ້ໄຂລັດຖະທຳມະນູນ
    AdoptAmendConstitution,

    /// Enact and amend laws (Article 54.2)
    /// ຕັ້ງ ແລະ ແກ້ໄຂກົດໝາຍ
    EnactLaws,

    /// Elect and remove President (Article 54.3)
    /// ເລືອກຕັ້ງ ແລະ ຖອດຖອນປະທານປະເທດ
    ElectRemovePresident,

    /// Approve Prime Minister and Government (Article 54.4)
    /// ຮັບຮອງນາຍົກລັດຖະມົນຕີ ແລະ ລັດຖະບານ
    ApproveGovernment,

    /// Approve national socio-economic development plan (Article 54.5)
    /// ຮັບຮອງແຜນພັດທະນາເສດຖະກິດ-ສັງຄົມແຫ່ງຊາດ
    ApproveDevPlan,

    /// Approve state budget (Article 54.6)
    /// ຮັບຮອງງົບປະມານແຫ່ງລັດ
    ApproveBudget,

    /// Decide on war and peace (Article 54.7)
    /// ຕັດສິນກ່ຽວກັບສົງຄາມ ແລະ ສັນຕິພາບ
    DecideWarPeace,

    /// Approve international treaties (Article 54.8)
    /// ຮັບຮອງສົນທິສັນຍາສາກົນ
    ApproveTreaties,

    /// Grant amnesty (Article 54.9)
    /// ປະກາດນິໄລໂທດ
    GrantAmnesty,

    /// Supervise state administration (Article 55)
    /// ຄວບຄຸມການບໍລິຫານລັດ
    SuperviseAdministration,
}

/// Standing Committee of the National Assembly (Articles 63-65)
/// ຄະນະປະຈຳສະພາແຫ່ງຊາດ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandingCommittee {
    /// Chairman
    /// ປະທານ
    pub chairman: String,

    /// Vice-chairmen
    /// ຮອງປະທານ
    pub vice_chairmen: Vec<String>,

    /// Members
    /// ສະມາຊິກ
    pub members: Vec<String>,

    /// Powers during NA recess (Article 64)
    /// ອຳນາດລະຫວ່າງສະໄໝປະຊຸມ
    pub interim_powers: Vec<StandingCommitteePower>,
}

/// Powers of the Standing Committee (Article 64)
/// ອຳນາດຂອງຄະນະປະຈຳ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StandingCommitteePower {
    /// Interpret Constitution and laws
    /// ຕີຄວາມລັດຖະທຳມະນູນ ແລະ ກົດໝາຍ
    InterpretLaws,

    /// Issue decrees
    /// ອອກຄຳສັ່ງ
    IssueDecrees,

    /// Supervise implementation of Constitution and laws
    /// ຄວບຄຸມການປະຕິບັດລັດຖະທຳມະນູນ ແລະ ກົດໝາຍ
    SuperviseImplementation,

    /// Prepare NA sessions
    /// ກະກຽມການປະຊຸມສະພາແຫ່ງຊາດ
    PrepareSessions,
}

// ============================================================================
// President of the State (ປະທານປະເທດ) - Articles 66-70
// ============================================================================

/// President of the State - Head of State (Article 66)
/// ປະທານປະເທດ - ປະມຸກລັດ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct President {
    /// Name of the President
    /// ຊື່ປະທານປະເທດ
    pub name: String,

    /// Elected by National Assembly (Article 67)
    /// ເລືອກຕັ້ງໂດຍສະພາແຫ່ງຊາດ
    pub elected_by: ElectedBy,

    /// Term: 5 years (Article 67)
    /// ວາລະ: 5 ປີ
    pub term_years: u8,

    /// Powers and duties (Articles 68-70)
    /// ອຳນາດ ແລະ ໜ້າທີ່
    pub powers: Vec<PresidentialPower>,
}

/// How the President is elected (Article 67)
/// ວິທີການເລືອກຕັ້ງປະທານປະເທດ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectedBy {
    /// Elected by National Assembly
    /// ເລືອກຕັ້ງໂດຍສະພາແຫ່ງຊາດ
    NationalAssembly,
}

/// Powers of the President (Articles 68-70)
/// ອຳນາດຂອງປະທານປະເທດ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PresidentialPower {
    /// Represent Lao PDR internally and externally (Article 68.1)
    /// ເປັນຕົວແທນຂອງ ສປປ ລາວ ໃນ ແລະ ຕ່າງປະເທດ
    RepresentState,

    /// Command armed forces (Article 68.2)
    /// ບັນຊາກຳລັງປະກອບອາວຸດ
    CommandArmedForces,

    /// Chair National Defense and Security Council (Article 68.3)
    /// ເປັນປະທານສະພາປ້ອງກັນ ແລະ ປອດໄພແຫ່ງຊາດ
    ChairDefenseCouncil,

    /// Promulgate Constitution and laws (Article 69.1)
    /// ປະກາດໃຊ້ລັດຖະທຳມະນູນ ແລະ ກົດໝາຍ
    PromulgateLaws,

    /// Issue presidential decrees (Article 69.2)
    /// ອອກລັດຖະດຳລັດປະທານປະເທດ
    IssueDecrees,

    /// Appoint and remove government officials (Article 69.3)
    /// ແຕ່ງຕັ້ງ ແລະ ຖອດຖອນເຈົ້າໜ້າທີ່ລັດຖະບານ
    AppointOfficials,

    /// Declare state of war or emergency (Article 70)
    /// ປະກາດສະຖານະການສົງຄາມ ຫຼື ສຸກເສີນ
    DeclareEmergency,

    /// Grant pardons (Article 69.4)
    /// ປະກາດອະໄພໂທດ
    GrantPardons,

    /// Ratify and abrogate international treaties (Article 69.5)
    /// ໃຫ້ສັດຕະຍາບັນ ແລະ ຍົກເລີກສົນທິສັນຍາສາກົນ
    RatifyTreaties,
}

// ============================================================================
// Government (ລັດຖະບານ) - Articles 71-80
// ============================================================================

/// Government - Executive body (Article 71)
/// ລັດຖະບານ - ອົງການບໍລິຫານ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Government {
    /// Prime Minister
    /// ນາຍົກລັດຖະມົນຕີ
    pub prime_minister: String,

    /// Deputy Prime Ministers
    /// ຮອງນາຍົກລັດຖະມົນຕີ
    pub deputy_prime_ministers: Vec<String>,

    /// Ministers
    /// ລັດຖະມົນຕີ
    pub ministers: Vec<Minister>,

    /// Powers and duties (Articles 73-76)
    /// ອຳນາດ ແລະ ໜ້າທີ່
    pub powers: Vec<GovernmentPower>,
}

/// Minister
/// ລັດຖະມົນຕີ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Minister {
    /// Name
    /// ຊື່
    pub name: String,

    /// Ministry/portfolio
    /// ກະຊວງ
    pub ministry: String,
}

/// Powers of the Government (Articles 73-76)
/// ອຳນາດຂອງລັດຖະບານ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GovernmentPower {
    /// Implement Constitution, laws, and NA resolutions (Article 73.1)
    /// ປະຕິບັດລັດຖະທຳມະນູນ, ກົດໝາຍ ແລະ ມະຕິສະພາແຫ່ງຊາດ
    ImplementLaws,

    /// Manage national socio-economic development (Article 73.2)
    /// ຄຸ້ມຄອງການພັດທະນາເສດຖະກິດ-ສັງຄົມແຫ່ງຊາດ
    ManageDevelopment,

    /// Manage state budget (Article 73.3)
    /// ຄຸ້ມຄອງງົບປະມານແຫ່ງລັດ
    ManageBudget,

    /// Manage national defense and security (Article 73.4)
    /// ຄຸ້ມຄອງການປ້ອງກັນຊາດ ແລະ ຮັກສາຄວາມສະຫງົບ
    ManageDefense,

    /// Manage foreign affairs (Article 73.5)
    /// ຄຸ້ມຄອງການຕ່າງປະເທດ
    ManageForeignAffairs,

    /// Issue government decrees and decisions (Article 74)
    /// ອອກລັດຖະດຳລັດຖະບານ ແລະ ມະຕິ
    IssueDecrees,

    /// Supervise ministries and local administration (Article 75)
    /// ຄວບຄຸມກະຊວງ ແລະ ການປົກຄອງທ້ອງຖິ່ນ
    SuperviseMinistries,
}

// ============================================================================
// Local Administration (ການປົກຄອງທ້ອງຖິ່ນ) - Articles 81-87
// ============================================================================

/// Local administration levels (Articles 81-87)
/// ລະດັບການປົກຄອງທ້ອງຖິ່ນ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalAdministration {
    /// Administrative level
    /// ລະດັບການປົກຄອງ
    pub level: AdministrativeLevel,

    /// Name of the administrative unit
    /// ຊື່ຫົວໜ່ວຍການປົກຄອງ
    pub name: String,

    /// People's Council (optional)
    /// ສະພາປະຊາຊົນ
    pub peoples_council: Option<PeoplesCouncil>,

    /// Administrative authority
    /// ອຳນາດການປົກຄອງ
    pub administrative_authority: AdministrativeAuthority,
}

/// Administrative levels in Lao PDR (Article 81)
/// ລະດັບການປົກຄອງໃນ ສປປ ລາວ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdministrativeLevel {
    /// Province (ແຂວງ)
    Province,

    /// District (ເມືອງ)
    District,

    /// Village (ບ້ານ)
    Village,
}

/// People's Council at local level (Article 82)
/// ສະພາປະຊາຊົນຂັ້ນທ້ອງຖິ່ນ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeoplesCouncil {
    /// Members elected by local people
    /// ສະມາຊິກເລືອກຕັ້ງໂດຍປະຊາຊົນທ້ອງຖິ່ນ
    pub members: Vec<String>,

    /// Term: 5 years
    /// ວາລະ: 5 ປີ
    pub term_years: u8,
}

/// Administrative authority at local level (Article 83)
/// ອຳນາດການປົກຄອງຂັ້ນທ້ອງຖິ່ນ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdministrativeAuthority {
    /// Governor (for province), Mayor (for district), Village Chief (for village)
    /// ແຂວງ: ເຈົ້າແຂວງ, ເມືອງ: ເຈົ້າເມືອງ, ບ້ານ: ນາຍບ້ານ
    pub chief: String,

    /// Powers and responsibilities
    /// ອຳນາດ ແລະ ຄວາມຮັບຜິດຊອບ
    pub powers: Vec<LocalPower>,
}

/// Powers of local administration (Articles 84-87)
/// ອຳນາດຂອງການປົກຄອງທ້ອງຖິ່ນ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocalPower {
    /// Implement Constitution, laws, and higher-level decisions
    /// ປະຕິບັດລັດຖະທຳມະນູນ, ກົດໝາຍ ແລະ ມະຕິຂັ້ນເທິງ
    ImplementLaws,

    /// Manage local socio-economic development
    /// ຄຸ້ມຄອງການພັດທະນາເສດຖະກິດ-ສັງຄົມທ້ອງຖິ່ນ
    ManageLocalDevelopment,

    /// Manage local budget
    /// ຄຸ້ມຄອງງົບປະມານທ້ອງຖິ່ນ
    ManageLocalBudget,

    /// Maintain public order and security
    /// ຮັກສາຄວາມສະຫງົບເລຽບຮ້ອຍ
    MaintainOrder,
}

// ============================================================================
// Judicial System (ລະບົບຍຸຕິທຳ) - Articles 88-100
// ============================================================================

/// People's Courts - Judicial body (Articles 88-95)
/// ສານປະຊາຊົນ - ອົງການຍຸຕິທຳ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeoplesCourt {
    /// Court level
    /// ລະດັບສານ
    pub level: CourtLevel,

    /// Independence of courts (Article 88)
    /// ຄວາມເປັນເອກະລາດຂອງສານ
    pub is_independent: bool,

    /// Judges
    /// ຜູ້ພິພາກສາ
    pub judges: Vec<Judge>,

    /// Powers (Articles 89-91)
    /// ອຳນາດ
    pub powers: Vec<CourtPower>,
}

/// Court levels (Article 89)
/// ລະດັບສານ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CourtLevel {
    /// Supreme People's Court (ສານປະຊາຊົນສູງສຸດ)
    Supreme,

    /// People's Court of Appeal (ສານປະຊາຊົນອຸທອນ)
    Appeal,

    /// Provincial People's Court (ສານປະຊາຊົນແຂວງ)
    Provincial,

    /// District People's Court (ສານປະຊາຊົນເມືອງ)
    District,
}

/// Judge information (Article 92)
/// ຂໍ້ມູນຜູ້ພິພາກສາ
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Judge {
    /// Name
    /// ຊື່
    pub name: String,

    /// Court level
    /// ລະດັບສານ
    pub court_level: CourtLevel,

    /// Independence guaranteed (Article 92)
    /// ຮັບປະກັນຄວາມເປັນເອກະລາດ
    pub independent_judgment: bool,
}

/// Powers of the Courts (Articles 89-91)
/// ອຳນາດຂອງສານ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CourtPower {
    /// Adjudicate criminal cases
    /// ພິຈາລະນາຄະດີອາຍາ
    AdjudicateCriminal,

    /// Adjudicate civil cases
    /// ພິຈາລະນາຄະດີແພ່ງ
    AdjudicateCivil,

    /// Adjudicate administrative cases
    /// ພິຈາລະນາຄະດີບໍລິຫານ
    AdjudicateAdministrative,

    /// Adjudicate economic cases
    /// ພິຈາລະນາຄະດີເສດຖະກິດ
    AdjudicateEconomic,
}

/// People's Prosecutors - Prosecutorial body (Articles 96-100)
/// ອົງການໄອຍະການປະຊາຊົນ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeoplesProsecutor {
    /// Prosecutorial level
    /// ລະດັບອົງການໄອຍະການ
    pub level: ProsecutorLevel,

    /// Powers (Articles 97-99)
    /// ອຳນາດ
    pub powers: Vec<ProsecutorPower>,
}

/// Prosecutorial levels (Article 96)
/// ລະດັບອົງການໄອຍະການ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProsecutorLevel {
    /// Supreme People's Prosecutor (ອົງການໄອຍະການປະຊາຊົນສູງສຸດ)
    Supreme,

    /// Provincial People's Prosecutor (ອົງການໄອຍະການປະຊາຊົນແຂວງ)
    Provincial,

    /// District People's Prosecutor (ອົງການໄອຍະການປະຊາຊົນເມືອງ)
    District,
}

/// Powers of the Prosecutors (Articles 97-99)
/// ອຳນາດຂອງອົງການໄອຍະການ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProsecutorPower {
    /// Prosecute criminal offenses
    /// ດຳເນີນຄະດີອາຍາ
    ProsecuteCriminal,

    /// Supervise investigation
    /// ຄວບຄຸມການສືບສວນ
    SuperviseInvestigation,

    /// Supervise detention and imprisonment
    /// ຄວບຄຸມການຄຸມຂັງ ແລະ ຈຳຄຸກ
    SuperviseDetention,

    /// Supervise legality of court proceedings
    /// ຄວບຄຸມຄວາມຖືກຕ້ອງຕາມກົດໝາຍຂອງການພິຈາລະນາຄະດີ
    SuperviseLegality,
}

// ============================================================================
// Constitutional Amendment (ການແກ້ໄຂເພີ່ມເຕີມລັດຖະທຳມະນູນ) - Articles 105-108
// ============================================================================

/// Constitutional amendment framework (Articles 105-108)
/// ກອບການແກ້ໄຂເພີ່ມເຕີມລັດຖະທຳມະນູນ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstitutionalAmendment {
    /// Proposal (Article 105): By President, NA Standing Committee, or Prime Minister
    /// ການສະເໜີ: ປະທານປະເທດ, ຄະນະປະຈຳສະພາແຫ່ງຊາດ, ຫຼື ນາຍົກລັດຖະມົນຕີ
    pub proposed_by: AmendmentProposer,

    /// Proposed amendments (bilingual)
    /// ການແກ້ໄຂທີ່ສະເໜີ
    pub proposed_changes_lao: String,
    pub proposed_changes_english: String,

    /// Required approval: 2/3 majority of NA members (Article 106)
    /// ການຮັບຮອງທີ່ຕ້ອງການ: 2/3 ຂອງສະມາຊິກສະພາແຫ່ງຊາດ
    pub required_votes: u32,

    /// Actual votes received
    /// ຄະແນນສຽງທີ່ໄດ້ຮັບ
    pub votes_received: Option<u32>,

    /// Approval status
    /// ສະຖານະການຮັບຮອງ
    pub approved: bool,

    /// Date of amendment
    /// ວັນທີແກ້ໄຂ
    pub amendment_date: Option<DateTime<Utc>>,
}

/// Who can propose constitutional amendments (Article 105)
/// ຜູ້ທີ່ສາມາດສະເໜີການແກ້ໄຂລັດຖະທຳມະນູນ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AmendmentProposer {
    /// President of the State
    /// ປະທານປະເທດ
    President,

    /// Standing Committee of the National Assembly
    /// ຄະນະປະຈຳສະພາແຫ່ງຊາດ
    StandingCommittee,

    /// Prime Minister
    /// ນາຍົກລັດຖະມົນຕີ
    PrimeMinister,
}

// ============================================================================
// Builder Patterns
// ============================================================================

impl NationalAssembly {
    /// Builder for National Assembly
    pub fn builder() -> NationalAssemblyBuilder {
        NationalAssemblyBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct NationalAssemblyBuilder {
    session: Option<u32>,
    members: Option<u32>,
    term_years: Option<u8>,
    election_method: Option<ElectionMethod>,
    powers: Vec<NationalAssemblyPower>,
    standing_committee: Option<StandingCommittee>,
}

impl NationalAssemblyBuilder {
    pub fn session(mut self, session: u32) -> Self {
        self.session = Some(session);
        self
    }

    pub fn members(mut self, members: u32) -> Self {
        self.members = Some(members);
        self
    }

    pub fn term_years(mut self, term_years: u8) -> Self {
        self.term_years = Some(term_years);
        self
    }

    pub fn election_method(mut self, method: ElectionMethod) -> Self {
        self.election_method = Some(method);
        self
    }

    pub fn power(mut self, power: NationalAssemblyPower) -> Self {
        self.powers.push(power);
        self
    }

    pub fn standing_committee(mut self, committee: StandingCommittee) -> Self {
        self.standing_committee = Some(committee);
        self
    }

    pub fn build(self) -> Result<NationalAssembly, String> {
        Ok(NationalAssembly {
            session: self.session.ok_or("session is required")?,
            members: self.members.ok_or("members is required")?,
            term_years: self.term_years.unwrap_or(5),
            election_method: self.election_method.ok_or("election_method is required")?,
            powers: self.powers,
            standing_committee: self.standing_committee,
        })
    }
}

impl Default for ElectionMethod {
    fn default() -> Self {
        ElectionMethod {
            universal: true,
            direct: true,
            secret: true,
        }
    }
}
