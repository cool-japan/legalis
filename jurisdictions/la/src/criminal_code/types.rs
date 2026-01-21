//! Criminal Code Types - ປະເພດກົດໝາຍອາຍາ
//!
//! This module implements the core types for the Criminal Code of Lao PDR (2017).
//!
//! Legal Basis: Law No. 26/NA (Criminal Code 2017), effective May 27, 2018
//!
//! ## Structure
//! - **General Provisions** (Articles 1-70): Criminal liability, intent, negligence, justifications
//! - **Penalties** (Articles 31-60): Death, imprisonment, fines, re-education
//! - **Crimes Against Persons** (Articles 121-160): Homicide, bodily harm, sexual crimes
//! - **Crimes Against Property** (Articles 161-200): Theft, fraud, embezzlement, arson
//!
//! ## Key Provisions
//! - **Age of criminal responsibility**: 16 years general, 14 years for serious crimes (Article 16)
//! - **Age of consent**: 15 years for sexual crimes (Article 141)
//! - **Mental capacity**: Evaluated per Articles 19-21
//! - **Death penalty**: Retained for most serious crimes (Article 32)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// General Provisions - ບົດບັນຍັດທົ່ວໄປ (Articles 1-70)
// ============================================================================

/// Criminal liability basis - ພື້ນຖານຄວາມຮັບຜິດຊອບທາງອາຍາ (Articles 13-30)
///
/// Establishes the foundation for criminal responsibility under Lao law.
///
/// # Bilingual
/// - Lao: ຄວາມຮັບຜິດຊອບທາງອາຍາ
/// - English: Criminal Liability
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriminalLiability {
    /// Type of mens rea - ສະພາບຈິດໃຈ
    pub mens_rea: MensRea,
    /// Actus reus performed - ການກະທຳຜິດ
    pub actus_reus: ActusReus,
    /// Age at time of offense - ອາຍຸໃນເວລາກະທຳຜິດ
    pub age_at_offense: u32,
    /// Mental capacity status - ສະພາບຈິດໃຈ
    pub mental_capacity: MentalCapacityStatus,
    /// Article reference - ບົດມາດຕາອ້າງອີງ
    pub article_reference: Vec<u32>,
    /// Bilingual description - ຄຳອະທິບາຍສອງພາສາ
    pub description_lao: String,
    pub description_en: String,
}

/// Mens rea (guilty mind) - ສະພາບຈິດໃຈ
///
/// The mental state required for criminal liability.
///
/// # Articles
/// - Article 13: General principles of criminal liability
/// - Article 14: Intent (ຈົງໃຈ)
/// - Article 15: Negligence (ປະມາດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MensRea {
    /// Direct intent - ຈົງໃຈໂດຍກົງ (Article 14(1))
    /// Knowing and desiring the criminal result
    DirectIntent,
    /// Indirect intent - ຈົງໃຈໂດຍທາງອ້ອມ (Article 14(2))
    /// Knowing the result is likely but not desiring it
    IndirectIntent,
    /// Conscious negligence - ປະມາດໂດຍມີສະຕິ (Article 15(1))
    /// Foreseeing but hoping to avoid the result
    ConsciousNegligence,
    /// Unconscious negligence - ປະມາດໂດຍບໍ່ມີສະຕິ (Article 15(2))
    /// Should have foreseen but did not
    UnconsciousNegligence,
    /// Strict liability - ຄວາມຮັບຜິດຊອບໂດຍກົງ
    /// No mens rea required for certain offenses
    StrictLiability,
}

/// Actus reus (criminal act) - ການກະທຳຜິດ
///
/// The physical element of a crime.
///
/// # Types
/// - Commission: Positive act (ການກະທຳ)
/// - Omission: Failure to act when duty exists (ການບໍ່ກະທຳ)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActusReus {
    /// Commission - การกระทำผิด
    Commission {
        /// Description of the act - ລາຍລະອຽດການກະທຳ
        act_description_lao: String,
        act_description_en: String,
        /// Time of commission - ເວລາກະທຳຜິດ
        time_of_commission: DateTime<Utc>,
        /// Location - ສະຖານທີ່
        location: String,
    },
    /// Omission - การละเว้น
    Omission {
        /// Duty that was breached - ພັນທະທີ່ຖືກລະເມີດ
        duty_description_lao: String,
        duty_description_en: String,
        /// Time of omission - ເວລາບໍ່ປະຕິບັດພັນທະ
        time_of_omission: DateTime<Utc>,
        /// Legal basis of duty - ພື້ນຖານກົດໝາຍຂອງພັນທະ
        duty_basis: String,
    },
}

/// Mental capacity status - ສະຖານະຄວາມສາມາດທາງຈິດໃຈ (Articles 19-21)
///
/// Determines whether a person can be held criminally responsible.
///
/// # Bilingual
/// - Lao: ຄວາມສາມາດທາງຈິດໃຈ
/// - English: Mental Capacity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MentalCapacityStatus {
    /// Full capacity - ຄວາມສາມາດເຕັມທີ່
    Full,
    /// Diminished capacity - ຄວາມສາມາດລົດລົງ (Article 20)
    /// Penalty may be reduced
    Diminished,
    /// No capacity - ບໍ່ມີຄວາມສາມາດ (Article 19)
    /// No criminal responsibility
    NoCapacity,
}

/// Mental capacity evaluation - ການປະເມີນຄວາມສາມາດທາງຈິດໃຈ
///
/// Comprehensive assessment of mental capacity for criminal responsibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalCapacity {
    /// Status - ສະຖານະ
    pub status: MentalCapacityStatus,
    /// Medical evaluation - ການປະເມີນທາງການແພດ
    pub medical_evaluation: Option<String>,
    /// Expert opinion - ຄວາມເຫັນຂອງຜູ້ຊ່ຽວຊານ
    pub expert_opinion: Option<String>,
    /// Evaluation date - ວັນທີປະເມີນ
    pub evaluation_date: DateTime<Utc>,
    /// Article 19-21 application - ການນຳໃຊ້ບົດມາດຕາ 19-21
    pub article_basis: Vec<u32>,
}

// ============================================================================
// Penalties - ໂທດ (Articles 31-60)
// ============================================================================

/// Penalty type - ປະເພດໂທດ
///
/// Types of penalties under the Criminal Code 2017.
///
/// # Articles
/// - Article 31: Types of penalties
/// - Article 32: Death penalty - ໂທດປະຫານຊີວິດ
/// - Article 33-36: Imprisonment - ໂທດຈຳຄຸກ
/// - Article 37-40: Fines - ໂທດປັບ
/// - Article 41-44: Re-education - ການສຶກສາອົບຮົມ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Penalty {
    /// Death penalty - ໂທດປະຫານຊີວິດ (Article 32)
    /// Only for most serious crimes
    Death {
        /// Bilingual justification
        justification_lao: String,
        justification_en: String,
        /// Article reference
        article: u32,
        /// May be commuted to life imprisonment
        commutable: bool,
    },
    /// Life imprisonment - ໂທດຈຳຄຸກຕະຫຼອດຊີວິດ (Article 33)
    LifeImprisonment {
        /// Bilingual description
        description_lao: String,
        description_en: String,
        /// Eligible for parole after minimum years
        parole_eligible_after_years: Option<u32>,
    },
    /// Fixed-term imprisonment - ໂທດຈຳຄຸກມີກຳນົດເວລາ (Article 34)
    FixedTermImprisonment {
        /// Years of imprisonment - ຈຳນວນປີ
        years: u32,
        /// Months (in addition to years) - ຈຳນວນເດືອນ
        months: u32,
        /// Minimum term - ໄລຍະຕ່ຳສຸດ
        minimum_years: u32,
        /// Maximum term - ໄລຍະສູງສຸດ
        maximum_years: u32,
    },
    /// Fine - ໂທດປັບ (Articles 37-40)
    Fine {
        /// Amount in LAK - ຈຳນວນເງີນ (ກີບ)
        amount_lak: u64,
        /// Minimum fine - ຈຳນວນຕ່ຳສຸດ
        minimum_lak: u64,
        /// Maximum fine - ຈຳນວນສູງສຸດ
        maximum_lak: u64,
        /// Convertible to imprisonment if unpaid - ສາມາດປ່ຽນເປັນຈຳຄຸກ
        convertible_to_imprisonment: bool,
    },
    /// Re-education without detention - ການສຶກສາອົບຮົມໂດຍບໍ່ຄຸມຂັງ (Article 41)
    ReEducation {
        /// Duration in months - ໄລຍະເວລາ (ເດືອນ)
        duration_months: u32,
        /// Conditions - ເງື່ອນໄຂ
        conditions_lao: Vec<String>,
        conditions_en: Vec<String>,
        /// Community service hours - ຊົ່ວໂມງບໍລິການຊຸມຊົນ
        community_service_hours: Option<u32>,
    },
    /// Confiscation of property - ການຍຶດຊັບສິນ (Article 45)
    Confiscation {
        /// Property description - ລາຍລະອຽດຊັບສິນ
        property_description_lao: String,
        property_description_en: String,
        /// Estimated value in LAK - ມູນຄ່າປະມານ
        estimated_value_lak: Option<u64>,
    },
    /// Deprivation of rights - ການຍົກເລີກສິດ (Article 46)
    DeprivationOfRights {
        /// Rights to be deprived - ສິດທີ່ຖືກຍົກເລີກ
        rights_lao: Vec<String>,
        rights_en: Vec<String>,
        /// Duration in years - ໄລຍະເວລາ (ປີ)
        duration_years: u32,
    },
}

/// Penalty severity - ລະດັບຄວາມໜັກ
///
/// Classification of crime severity for sentencing purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PenaltySeverity {
    /// Petty offense - ຄວາມຜິດເລັກນ້ອຍ
    Petty,
    /// Misdemeanor - ຄວາມຜິດປານກາງ
    Misdemeanor,
    /// Felony - ຄວາມຜິດໜັກ
    Felony,
    /// Most serious crime - ຄວາມຜິດຮ້າຍແຮງທີ່ສຸດ
    MostSerious,
}

// ============================================================================
// Justification and Excuse - ເຫດຜົນແຫ່ງການພົ້ນໂທດ (Articles 61-70)
// ============================================================================

/// Justification grounds - ເຫດຜົນແຫ່ງການພົ້ນໂທດ
///
/// Legal defenses that justify or excuse criminal conduct.
///
/// # Articles
/// - Article 61: Self-defense - ການປ້ອງກັນຕົວ
/// - Article 62: Necessity - ຄວາມຈຳເປັນ
/// - Article 63: Superior orders - ຄຳສັ່ງຂອງຜູ້ບັງຄັບບັນຊາ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JustificationGround {
    /// Self-defense - ການປ້ອງກັນຕົວ (Article 61)
    SelfDefense {
        /// Imminent threat description - ໄພຂົ່ມຂູ່ທີ່ໃກ້ຈະເກີດຂຶ້ນ
        threat_description_lao: String,
        threat_description_en: String,
        /// Proportionality of response - ຄວາມເໝາະສົມຂອງການຕອບໂຕ້
        proportional_response: bool,
        /// Reasonable belief in necessity - ຄວາມເຊື່ອທີ່ມີເຫດຜົນ
        reasonable_belief: bool,
    },
    /// Necessity - ຄວາມຈຳເປັນ (Article 62)
    Necessity {
        /// Imminent danger description - ອັນຕະລາຍທີ່ໃກ້ຈະເກີດ
        danger_description_lao: String,
        danger_description_en: String,
        /// No reasonable alternative - ບໍ່ມີທາງເລືອກອື່ນ
        no_alternative: bool,
        /// Lesser harm principle - ຫຼັກການອັນຕະລາຍທີ່ນ້ອຍກວ່າ
        lesser_harm: bool,
    },
    /// Superior orders - ຄຳສັ່ງຂອງຜູ້ບັງຄັບບັນຊາ (Article 63)
    SuperiorOrders {
        /// Order description - ລາຍລະອຽດຄຳສັ່ງ
        order_description_lao: String,
        order_description_en: String,
        /// Lawfulness of order - ຄວາມຖືກຕ້ອງຕາມກົດໝາຍ
        lawful_order: bool,
        /// Manifestly illegal - ຜິດກົດໝາຍຢ່າງຊັດເຈນ
        manifestly_illegal: bool,
        /// Superior's identity - ຕົວຕົນຂອງຜູ້ບັງຄັບບັນຊາ
        superior_identity: String,
    },
    /// Consent of victim - ການຍິນຍອມຂອງຜູ້ເສຍຫາຍ (Article 64)
    Consent {
        /// Consent details - ລາຍລະອຽດການຍິນຍອມ
        consent_description_lao: String,
        consent_description_en: String,
        /// Freely given - ໃຫ້ໂດຍອິດສະຫຼະ
        freely_given: bool,
        /// Informed consent - ຍິນຍອມໂດຍຮູ້ຂໍ້ມູນ
        informed: bool,
        /// Capacity to consent - ຄວາມສາມາດໃນການຍິນຍອມ
        capacity_verified: bool,
    },
    /// Lawful authority - ອຳນາດທາງກົດໝາຍ (Article 65)
    LawfulAuthority {
        /// Authority basis - ພື້ນຖານອຳນາດ
        authority_basis_lao: String,
        authority_basis_en: String,
        /// Statutory reference - ອ້າງອີງກົດໝາຍ
        statutory_reference: String,
        /// Within scope of authority - ຢູ່ໃນຂອບເຂດອຳນາດ
        within_scope: bool,
    },
}

// ============================================================================
// Crimes Against Persons - ຄວາມຜິດຕໍ່ບຸກຄົນ (Articles 121-160)
// ============================================================================

/// Homicide types - ປະເພດການຂ້າຄົນ (Articles 121-128)
///
/// Different categories of unlawful killings under Lao criminal law.
///
/// # Bilingual
/// - Lao: ຄວາມຜິດຕໍ່ຊີວິດ
/// - English: Crimes Against Life
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HomicideType {
    /// Premeditated murder - ການຂ້າຄົນໂດຍມີການວາງແຜນ (Article 121)
    /// Death penalty or life imprisonment
    PremeditatedMurder {
        /// Planning details - ລາຍລະອຽດການວາງແຜນ
        planning_details_lao: String,
        planning_details_en: String,
        /// Time between planning and act - ເວລາລະຫວ່າງວາງແຜນແລະກະທຳ
        planning_duration: String,
        /// Article 121 elements
        aggravating_factors_lao: Vec<String>,
        aggravating_factors_en: Vec<String>,
    },
    /// Murder with special circumstances - ການຂ້າຄົນທີ່ມີສະຖານະການພິເສດ (Article 122)
    /// Death penalty
    MurderWithSpecialCircumstances {
        /// Special circumstances - ສະຖານະການພິເສດ
        circumstances_lao: Vec<String>,
        circumstances_en: Vec<String>,
        /// Examples: murder of official, multiple victims, torture
        victim_category: VictimCategory,
    },
    /// Intentional homicide - ການຂ້າຄົນໂດຍເຈດຕະນາ (Article 123)
    /// 10-20 years imprisonment
    IntentionalHomicide {
        /// Intent description - ລາຍລະອຽດເຈດຕະນາ
        intent_description_lao: String,
        intent_description_en: String,
        /// No premeditation
        heat_of_passion: bool,
    },
    /// Manslaughter - ການຂ້າຄົນໂດຍບໍ່ໄດ້ຕັ້ງໃຈ (Article 124)
    /// 5-10 years imprisonment
    Manslaughter {
        /// Circumstances - ສະຖານະການ
        circumstances_lao: String,
        circumstances_en: String,
        /// Negligence type
        negligence_type: NegligenceType,
    },
    /// Infanticide - ການຂ້າເດັກທີ່ເກີດໃໝ່ (Article 125)
    /// 3-7 years imprisonment
    Infanticide {
        /// Mother's mental state - ສະພາບຈິດໃຈຂອງແມ່
        mental_state_lao: String,
        mental_state_en: String,
        /// Time after birth - ເວລາຫຼັງເກີດ
        time_after_birth: String,
        /// Mitigating factors
        mitigating_factors_lao: Vec<String>,
        mitigating_factors_en: Vec<String>,
    },
    /// Causing death through negligence - ການເຮັດໃຫ້ເກີດຄວາມຕາຍໂດຍປະມາດ (Article 126)
    /// 1-5 years imprisonment
    NegligentHomicide {
        /// Negligence description - ລາຍລະອຽດຄວາມປະມາດ
        negligence_description_lao: String,
        negligence_description_en: String,
        /// Professional negligence - ຄວາມປະມາດທາງວິຊາຊີບ
        professional_negligence: bool,
    },
    /// Assisted suicide - ການຊ່ວຍເຫຼືອການຂ້າຕົວຕາຍ (Article 127)
    /// 1-3 years imprisonment
    AssistedSuicide {
        /// Assistance description - ລາຍລະອຽດການຊ່ວຍເຫຼືອ
        assistance_description_lao: String,
        assistance_description_en: String,
        /// Victim's consent verified
        consent_verified: bool,
    },
    /// Incitement to suicide - ການຍຸຍົງໃຫ້ຂ້າຕົວຕາຍ (Article 128)
    /// 3-7 years imprisonment
    IncitementToSuicide {
        /// Incitement details - ລາຍລະອຽດການຍຸຍົງ
        incitement_details_lao: String,
        incitement_details_en: String,
        /// Relationship to victim
        relationship_to_victim: String,
    },
}

/// Victim category - ປະເພດຜູ້ເສຍຫາຍ
///
/// Special categories of victims that may affect sentencing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VictimCategory {
    /// Government official - ເຈົ້າໜ້າທີ່ລັດ
    GovernmentOfficial,
    /// Child - ເດັກນ້ອຍ
    Child,
    /// Pregnant woman - ແມ່ຍິງຖືພາ
    PregnantWoman,
    /// Elderly person - ຜູ້ສູງອາຍຸ
    ElderlyPerson,
    /// Disabled person - ຄົນພິການ
    DisabledPerson,
    /// Multiple victims - ຫຼາຍຄົນເສຍຫາຍ
    MultipleVictims,
    /// General public - ປະຊາຊົນທົ່ວໄປ
    GeneralPublic,
}

/// Negligence type - ປະເພດຄວາມປະມາດ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NegligenceType {
    /// Criminal negligence - ຄວາມປະມາດທາງອາຍາ
    Criminal,
    /// Gross negligence - ຄວາມປະມາດຢ່າງຮ້າຍແຮງ
    Gross,
    /// Professional negligence - ຄວາມປະມາດທາງວິຊາຊີບ
    Professional,
    /// Traffic-related negligence - ຄວາມປະມາດທາງຈະລາຈອນ
    TrafficRelated,
}

/// Bodily harm types - ປະເພດການເຮັດຮ້າຍຮ່າງກາຍ (Articles 129-140)
///
/// Different degrees of bodily harm under Lao criminal law.
///
/// # Bilingual
/// - Lao: ຄວາມຜິດຕໍ່ຮ່າງກາຍ
/// - English: Crimes Against Physical Integrity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BodilyHarmType {
    /// Grievous bodily harm - ການທຳຮ້າຍຮ່າງກາຍຮ້າຍແຮງ (Article 129)
    /// 5-15 years imprisonment
    GrievousBodilyHarm {
        /// Injury description - ລາຍລະອຽດການບາດເຈັບ
        injury_description_lao: String,
        injury_description_en: String,
        /// Permanent disability - ຄວາມພິການຖາວອນ
        permanent_disability: bool,
        /// Loss of organ/limb - ການສູນເສຍອະໄວຍະວະ
        organ_loss: bool,
        /// Disfigurement - ການທຳລາຍໜ້າຕາ
        disfigurement: bool,
    },
    /// Simple bodily harm - ການທຳຮ້າຍຮ່າງກາຍ (Article 130)
    /// 1-5 years imprisonment
    SimpleBodilyHarm {
        /// Injury description - ລາຍລະອຽດການບາດເຈັບ
        injury_description_lao: String,
        injury_description_en: String,
        /// Medical treatment required - ຕ້ອງການການປິ່ນປົວ
        medical_treatment_days: u32,
    },
    /// Minor bodily harm - ການທຳຮ້າຍຮ່າງກາຍເລັກນ້ອຍ (Article 131)
    /// Fine or up to 1 year imprisonment
    MinorBodilyHarm {
        /// Injury description - ລາຍລະອຽດການບາດເຈັບ
        injury_description_lao: String,
        injury_description_en: String,
    },
    /// Battery - ການຕີຮົບ (Article 132)
    /// Fine
    Battery {
        /// Circumstances - ສະຖານະການ
        circumstances_lao: String,
        circumstances_en: String,
        /// No significant injury
        no_significant_injury: bool,
    },
    /// Torture - ການທໍລະມານ (Article 133)
    /// 10-20 years or life imprisonment
    Torture {
        /// Torture methods - ວິທີການທໍລະມານ
        methods_lao: Vec<String>,
        methods_en: Vec<String>,
        /// Purpose - ຈຸດປະສົງ
        purpose_lao: String,
        purpose_en: String,
        /// Official capacity - ໃນຖານະເຈົ້າໜ້າທີ່
        official_capacity: bool,
    },
}

/// Sexual crimes - ຄວາມຜິດທາງເພດ (Articles 141-150)
///
/// Sexual offenses under Lao criminal law.
///
/// # Key Provisions
/// - **Age of consent**: 15 years (Article 141)
/// - Statutory rape: Sexual intercourse with person under 15
///
/// # Bilingual
/// - Lao: ຄວາມຜິດທາງເພດ
/// - English: Sexual Crimes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SexualCrime {
    /// Rape - ການຂົ່ມຂືນ (Article 141)
    /// 7-15 years imprisonment
    Rape {
        /// Victim age - ອາຍຸຜູ້ເສຍຫາຍ
        victim_age: u32,
        /// Force or threat used - ການໃຊ້ກຳລັງຫຼືການຂົ່ມຂູ່
        force_or_threat: bool,
        /// Victim incapacitated - ຜູ້ເສຍຫາຍບໍ່ສາມາດຕ້ານທານ
        victim_incapacitated: bool,
        /// Aggravating factors
        aggravating_factors_lao: Vec<String>,
        aggravating_factors_en: Vec<String>,
    },
    /// Statutory rape - ການມີເພດສຳພັນກັບຜູ້ທີ່ອາຍຸຕ່ຳກວ່າ 15 ປີ (Article 141)
    /// 5-15 years imprisonment (age of consent: 15)
    StatutoryRape {
        /// Victim age (must be under 15) - ອາຍຸຜູ້ເສຍຫາຍ
        victim_age: u32,
        /// Perpetrator age - ອາຍຸຜູ້ກະທຳຜິດ
        perpetrator_age: u32,
        /// Consent irrelevant if victim under 15
        victim_consented: bool,
        /// Relationship to victim
        relationship: String,
    },
    /// Sexual assault - ການລ່ວງລະເມີດທາງເພດ (Article 142)
    /// 3-10 years imprisonment
    SexualAssault {
        /// Victim age - ອາຍຸຜູ້ເສຍຫາຍ
        victim_age: u32,
        /// Assault description - ລາຍລະອຽດການລ່ວງລະເມີດ
        assault_description_lao: String,
        assault_description_en: String,
        /// Force or threat - ການໃຊ້ກຳລັງຫຼືການຂົ່ມຂູ່
        force_or_threat: bool,
    },
    /// Sexual harassment - ການລົບກວນທາງເພດ (Article 143)
    /// 6 months - 3 years imprisonment
    SexualHarassment {
        /// Harassment description - ລາຍລະອຽດການລົບກວນ
        harassment_description_lao: String,
        harassment_description_en: String,
        /// Pattern of behavior - ຮູບແບບພຶດຕິກຳ
        pattern_of_behavior: bool,
        /// Workplace harassment - ການລົບກວນໃນບ່ອນເຮັດວຽກ
        workplace: bool,
    },
    /// Indecent exposure - ການເປີດເຜີຍອະໄວຍະວະເພດ (Article 144)
    /// Fine or up to 1 year imprisonment
    IndecentExposure {
        /// Location - ສະຖານທີ່
        location: String,
        /// Public place - ສະຖານທີ່ສາທາລະນະ
        public_place: bool,
        /// Witnesses - ພະຍານ
        witnesses: Vec<String>,
    },
    /// Trafficking for sexual exploitation - ການຄ້າມະນຸດເພື່ອຂູດຮີດທາງເພດ (Article 145)
    /// 15-20 years or life imprisonment
    SexualTrafficking {
        /// Trafficking details - ລາຍລະອຽດການຄ້າມະນຸດ
        trafficking_details_lao: String,
        trafficking_details_en: String,
        /// Victim age - ອາຍຸຜູ້ເສຍຫາຍ
        victim_age: u32,
        /// Cross-border trafficking - ການຄ້າມະນຸດຂ້າມຊາຍແດນ
        cross_border: bool,
        /// Organized crime involvement
        organized_crime: bool,
    },
}

// ============================================================================
// Property Crimes - ຄວາມຜິດຕໍ່ຊັບສິນ (Articles 161-200)
// ============================================================================

/// Property crime types - ປະເພດຄວາມຜິດຕໍ່ຊັບສິນ
///
/// Crimes against property under Lao criminal law.
///
/// # Bilingual
/// - Lao: ຄວາມຜິດຕໍ່ຊັບສິນ
/// - English: Crimes Against Property
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyCrime {
    /// Theft - ການລັກຊັບ (Article 161)
    /// 1-10 years imprisonment depending on value
    Theft {
        /// Property description - ລາຍລະອຽດຊັບສິນ
        property_description_lao: String,
        property_description_en: String,
        /// Value in LAK - ມູນຄ່າ (ກີບ)
        value_lak: u64,
        /// Burglary - ການລັກໂດຍທະລຸເຂົ້າ
        burglary: bool,
        /// Night-time - ເວລາກາງຄືນ
        night_time: bool,
        /// Multiple perpetrators - ຫຼາຍຄົນກະທຳ
        multiple_perpetrators: bool,
    },
    /// Robbery - ການກະທຳຊິງ (Article 162)
    /// 5-15 years imprisonment
    Robbery {
        /// Property description - ລາຍລະອຽດຊັບສິນ
        property_description_lao: String,
        property_description_en: String,
        /// Value in LAK - ມູນຄ່າ (ກີບ)
        value_lak: u64,
        /// Force or threat used - ການໃຊ້ກຳລັງຫຼືການຂົ່ມຂູ່
        force_or_threat: bool,
        /// Weapon used - ການໃຊ້ອາວຸດ
        weapon_used: Option<String>,
        /// Injury caused - ເຮັດໃຫ້ບາດເຈັບ
        injury_caused: bool,
    },
    /// Fraud - ການສໍ້ໂກງ (Article 165)
    /// 2-10 years imprisonment
    Fraud {
        /// Fraud scheme description - ລາຍລະອຽດການສໍ້ໂກງ
        scheme_description_lao: String,
        scheme_description_en: String,
        /// Amount defrauded in LAK - ຈຳນວນເງີນ (ກີບ)
        amount_lak: u64,
        /// Number of victims - ຈຳນວນຜູ້ເສຍຫາຍ
        number_of_victims: u32,
        /// Sophisticated scheme - ການສໍ້ໂກງທີ່ຊັບຊ້ອນ
        sophisticated: bool,
    },
    /// Embezzlement - ການຍັກຍ້າຍ (Article 170)
    /// 3-15 years imprisonment
    Embezzlement {
        /// Property description - ລາຍລະອຽດຊັບສິນ
        property_description_lao: String,
        property_description_en: String,
        /// Amount embezzled in LAK - ຈຳນວນເງີນ (ກີບ)
        amount_lak: u64,
        /// Position of trust - ຕຳແໜ່ງຄວາມໄວ້ວາງໃຈ
        position_of_trust: String,
        /// Public official - ເຈົ້າໜ້າທີ່ລັດ
        public_official: bool,
        /// Duration of embezzlement - ໄລຍະເວລາການຍັກຍ້າຍ
        duration: String,
    },
    /// Arson - ການວາງເພີງ (Article 180)
    /// 5-20 years or life imprisonment
    Arson {
        /// Property description - ລາຍລະອຽດຊັບສິນ
        property_description_lao: String,
        property_description_en: String,
        /// Estimated damage in LAK - ຄວາມເສຍຫາຍປະມານ (ກີບ)
        damage_lak: u64,
        /// Inhabited building - ອາຄານທີ່ມີຄົນຢູ່ອາໄສ
        inhabited_building: bool,
        /// Danger to life - ອັນຕະລາຍຕໍ່ຊີວິດ
        danger_to_life: bool,
        /// Deaths resulted - ມີຄົນເສຍຊີວິດ
        deaths_resulted: u32,
    },
    /// Vandalism - ການທຳລາຍຊັບສິນ (Article 185)
    /// Fine or 1-5 years imprisonment
    Vandalism {
        /// Property description - ລາຍລະອຽດຊັບສິນ
        property_description_lao: String,
        property_description_en: String,
        /// Damage description - ລາຍລະອຽດຄວາມເສຍຫາຍ
        damage_description_lao: String,
        damage_description_en: String,
        /// Estimated damage in LAK - ຄວາມເສຍຫາຍປະມານ (ກີບ)
        damage_lak: u64,
    },
    /// Extortion - ການຂູດຮີດ (Article 190)
    /// 3-10 years imprisonment
    Extortion {
        /// Extortion description - ລາຍລະອຽດການຂູດຮີດ
        extortion_description_lao: String,
        extortion_description_en: String,
        /// Amount extorted in LAK - ຈຳນວນເງີນ (ກີບ)
        amount_lak: u64,
        /// Threats made - ການຂົ່ມຂູ່
        threats_lao: Vec<String>,
        threats_en: Vec<String>,
        /// Violence used - ການໃຊ້ຄວາມຮຸນແຮງ
        violence_used: bool,
    },
}

// ============================================================================
// Crime Details - ລາຍລະອຽດຄວາມຜິດ
// ============================================================================

/// Crime classification - ການຈັດປະເພດຄວາມຜິດ
///
/// Comprehensive classification of criminal offenses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CrimeType {
    /// Homicide - ການຂ້າຄົນ
    Homicide(HomicideType),
    /// Bodily harm - ການທຳຮ້າຍຮ່າງກາຍ
    BodilyHarm(BodilyHarmType),
    /// Sexual crime - ຄວາມຜິດທາງເພດ
    SexualCrime(SexualCrime),
    /// Property crime - ຄວາມຜິດຕໍ່ຊັບສິນ
    PropertyCrime(PropertyCrime),
}

/// Complete crime record - ບັນທຶກຄວາມຜິດ
///
/// Comprehensive record of a criminal offense.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crime {
    /// Crime type - ປະເພດຄວາມຜິດ
    pub crime_type: CrimeType,
    /// Criminal liability - ຄວາມຮັບຜິດຊອບທາງອາຍາ
    pub liability: CriminalLiability,
    /// Applicable penalties - ໂທດທີ່ນຳໃຊ້
    pub penalties: Vec<Penalty>,
    /// Justification grounds (if any) - ເຫດຜົນແຫ່ງການພົ້ນໂທດ
    pub justification_grounds: Vec<JustificationGround>,
    /// Severity - ລະດັບຄວາມໜັກ
    pub severity: PenaltySeverity,
    /// Article references - ບົດມາດຕາອ້າງອີງ
    pub article_references: Vec<u32>,
    /// Date of offense - ວັນທີກະທຳຜິດ
    pub offense_date: DateTime<Utc>,
    /// Location - ສະຖານທີ່
    pub location: String,
    /// Victims - ຜູ້ເສຍຫາຍ
    pub victims: Vec<Victim>,
    /// Perpetrator - ຜູ້ກະທຳຜິດ
    pub perpetrator: Perpetrator,
}

/// Victim information - ຂໍ້ມູນຜູ້ເສຍຫາຍ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Victim {
    /// Name - ຊື່
    pub name: String,
    /// Age - ອາຍຸ
    pub age: u32,
    /// Category - ປະເພດ
    pub category: VictimCategory,
    /// Relationship to perpetrator - ຄວາມສຳພັນກັບຜູ້ກະທຳຜິດ
    pub relationship_to_perpetrator: Option<String>,
    /// Injury description - ລາຍລະອຽດການບາດເຈັບ
    pub injury_description_lao: Option<String>,
    pub injury_description_en: Option<String>,
}

/// Perpetrator information - ຂໍ້ມູນຜູ້ກະທຳຜິດ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perpetrator {
    /// Name - ຊື່
    pub name: String,
    /// Age - ອາຍຸ
    pub age: u32,
    /// Mental capacity - ຄວາມສາມາດທາງຈິດໃຈ
    pub mental_capacity: MentalCapacity,
    /// Prior convictions - ຄວາມຜິດທີ່ຜ່ານມາ
    pub prior_convictions: Vec<String>,
    /// Employment status - ສະຖານະການເຮັດວຽກ
    pub employment_status: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mens_rea_types() {
        let direct = MensRea::DirectIntent;
        let indirect = MensRea::IndirectIntent;
        assert_ne!(direct, indirect);
    }

    #[test]
    fn test_penalty_severity_ordering() {
        assert!(PenaltySeverity::Petty < PenaltySeverity::Misdemeanor);
        assert!(PenaltySeverity::Misdemeanor < PenaltySeverity::Felony);
        assert!(PenaltySeverity::Felony < PenaltySeverity::MostSerious);
    }

    #[test]
    fn test_mental_capacity_status() {
        let full = MentalCapacityStatus::Full;
        let diminished = MentalCapacityStatus::Diminished;
        let no_capacity = MentalCapacityStatus::NoCapacity;

        assert_ne!(full, diminished);
        assert_ne!(full, no_capacity);
        assert_ne!(diminished, no_capacity);
    }

    #[test]
    fn test_victim_category() {
        let child = VictimCategory::Child;
        let official = VictimCategory::GovernmentOfficial;
        assert_ne!(child, official);
    }
}
