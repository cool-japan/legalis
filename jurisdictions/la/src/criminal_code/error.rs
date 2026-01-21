//! Criminal Code Error Types - ປະເພດຄວາມຜິດພາດກົດໝາຍອາຍາ
//!
//! Comprehensive error types for Criminal Code validation and processing.
//!
//! All errors include:
//! - Bilingual error messages (Lao/English)
//! - Article references from Criminal Code 2017
//! - Structured error categories
//!
//! # Error Categories
//! - **LiabilityError**: Criminal liability determination errors
//! - **PenaltyError**: Penalty application errors
//! - **JustificationError**: Defense/justification errors
//! - **HomicideError**: Homicide-specific errors
//! - **BodilyHarmError**: Bodily harm crime errors
//! - **SexualCrimeError**: Sexual offense errors
//! - **PropertyCrimeError**: Property crime errors
//! - **AgeError**: Age-related requirement errors

use thiserror::Error;

/// Result type for criminal code operations
pub type Result<T> = std::result::Result<T, CriminalCodeError>;

/// Main criminal code error type - ປະເພດຄວາມຜິດພາດກົດໝາຍອາຍາ
///
/// Comprehensive error type covering all aspects of criminal code validation.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CriminalCodeError {
    /// Criminal liability errors - ຄວາມຜິດພາດຄວາມຮັບຜິດຊອບທາງອາຍາ
    #[error("Criminal liability error: {0}")]
    Liability(#[from] LiabilityError),

    /// Penalty errors - ຄວາມຜິດພາດໂທດ
    #[error("Penalty error: {0}")]
    Penalty(#[from] PenaltyError),

    /// Justification errors - ຄວາມຜິດພາດເຫດຜົນແຫ່ງການພົ້ນໂທດ
    #[error("Justification error: {0}")]
    Justification(#[from] JustificationError),

    /// Homicide errors - ຄວາມຜິດພາດການຂ້າຄົນ
    #[error("Homicide error: {0}")]
    Homicide(#[from] HomicideError),

    /// Bodily harm errors - ຄວາມຜິດພາດການທຳຮ້າຍຮ່າງກາຍ
    #[error("Bodily harm error: {0}")]
    BodilyHarm(#[from] BodilyHarmError),

    /// Sexual crime errors - ຄວາມຜິດພາດຄວາມຜິດທາງເພດ
    #[error("Sexual crime error: {0}")]
    SexualCrime(#[from] SexualCrimeError),

    /// Property crime errors - ຄວາມຜິດພາດຄວາມຜິດຕໍ່ຊັບສິນ
    #[error("Property crime error: {0}")]
    PropertyCrime(#[from] PropertyCrimeError),

    /// Age-related errors - ຄວາມຜິດພາດທີ່ກ່ຽວກັບອາຍຸ
    #[error("Age requirement error: {0}")]
    Age(#[from] AgeError),

    /// Validation errors - ຄວາມຜິດພາດການກວດສອບ
    #[error("Validation error: {0}")]
    Validation(String),
}

// ============================================================================
// Criminal Liability Errors - ຄວາມຜິດພາດຄວາມຮັບຜິດຊອບທາງອາຍາ
// ============================================================================

/// Criminal liability error - ຄວາມຜິດພາດຄວາມຮັບຜິດຊອບທາງອາຍາ (Articles 13-30)
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum LiabilityError {
    /// Insufficient mens rea - ສະພາບຈິດໃຈບໍ່ພຽງພໍ (Article 13-15)
    #[error("Insufficient mens rea (Art. {article}): {message_en} / {message_lao}")]
    InsufficientMensRea {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// Missing actus reus - ຂາດການກະທຳຜິດ (Article 13)
    #[error("Missing actus reus (Art. {article}): {message_en} / {message_lao}")]
    MissingActusReus {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// No causation - ບໍ່ມີຄວາມສຳພັນເຫດແລະຜົນ (Article 13)
    #[error("Causation not established (Art. {article}): {message_en} / {message_lao}")]
    NoCausation {
        article: u32,
        message_lao: String,
        message_en: String,
    },

    /// Below age of criminal responsibility - ອາຍຸຕ່ຳກວ່າການຮັບຜິດຊອບທາງອາຍາ (Article 16)
    #[error("Below age of criminal responsibility (Art. 16): {message_en} / {message_lao}")]
    BelowCriminalAge {
        age: u32,
        required_age: u32,
        message_lao: String,
        message_en: String,
    },

    /// Lack of mental capacity - ຂາດຄວາມສາມາດທາງຈິດໃຈ (Article 19)
    #[error("Lack of mental capacity (Art. 19): {message_en} / {message_lao}")]
    LackOfMentalCapacity {
        message_lao: String,
        message_en: String,
    },

    /// Diminished capacity - ຄວາມສາມາດລົດລົງ (Article 20)
    #[error("Diminished capacity (Art. 20): {message_en} / {message_lao}")]
    DiminishedCapacity {
        message_lao: String,
        message_en: String,
    },

    /// Involuntary conduct - ການກະທຳທີ່ບໍ່ໄດ້ຕັ້ງໃຈ (Article 21)
    #[error("Involuntary conduct (Art. 21): {message_en} / {message_lao}")]
    InvoluntaryConduct {
        message_lao: String,
        message_en: String,
    },
}

// ============================================================================
// Penalty Errors - ຄວາມຜິດພາດໂທດ
// ============================================================================

/// Penalty error - ຄວາມຜິດພາດໂທດ (Articles 31-60)
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PenaltyError {
    /// Death penalty inappropriately applied - ໂທດປະຫານຊີວິດບໍ່ເໝາະສົມ (Article 32)
    #[error("Death penalty not applicable (Art. 32): {message_en} / {message_lao}")]
    DeathPenaltyNotApplicable {
        message_lao: String,
        message_en: String,
    },

    /// Imprisonment term exceeds maximum - ໄລຍະຈຳຄຸກເກີນສູງສຸດ (Article 34)
    #[error(
        "Imprisonment exceeds maximum (Art. 34): {years} years exceeds {max_years} years / {message_lao}"
    )]
    ImprisonmentExceedsMaximum {
        years: u32,
        max_years: u32,
        message_lao: String,
        message_en: String,
    },

    /// Imprisonment term below minimum - ໄລຍະຈຳຄຸກຕ່ຳກວ່າຕ່ຳສຸດ (Article 34)
    #[error(
        "Imprisonment below minimum (Art. 34): {years} years below {min_years} years / {message_lao}"
    )]
    ImprisonmentBelowMinimum {
        years: u32,
        min_years: u32,
        message_lao: String,
        message_en: String,
    },

    /// Fine exceeds maximum - ຈຳນວນເງີນປັບເກີນສູງສຸດ (Articles 37-40)
    #[error(
        "Fine exceeds maximum (Art. {article}): {amount_lak} LAK exceeds {max_lak} LAK / {message_lao}"
    )]
    FineExceedsMaximum {
        article: u32,
        amount_lak: u64,
        max_lak: u64,
        message_lao: String,
        message_en: String,
    },

    /// Fine below minimum - ຈຳນວນເງີນປັບຕ່ຳກວ່າຕ່ຳສຸດ (Articles 37-40)
    #[error(
        "Fine below minimum (Art. {article}): {amount_lak} LAK below {min_lak} LAK / {message_lao}"
    )]
    FineBelowMinimum {
        article: u32,
        amount_lak: u64,
        min_lak: u64,
        message_lao: String,
        message_en: String,
    },

    /// Invalid penalty combination - ການລວມໂທດບໍ່ຖືກຕ້ອງ (Article 31)
    #[error("Invalid penalty combination (Art. 31): {message_en} / {message_lao}")]
    InvalidCombination {
        message_lao: String,
        message_en: String,
    },

    /// Re-education duration invalid - ໄລຍະເວລາການສຶກສາອົບຮົມບໍ່ຖືກຕ້ອງ (Article 41)
    #[error("Invalid re-education duration (Art. 41): {duration_months} months / {message_lao}")]
    InvalidReEducationDuration {
        duration_months: u32,
        message_lao: String,
        message_en: String,
    },

    /// Penalty disproportionate to crime - ໂທດບໍ່ສົມດູນກັບຄວາມຜິດ (Article 31)
    #[error("Penalty disproportionate to crime (Art. 31): {message_en} / {message_lao}")]
    DisproportionatePenalty {
        message_lao: String,
        message_en: String,
    },
}

// ============================================================================
// Justification Errors - ຄວາມຜິດພາດເຫດຜົນແຫ່ງການພົ້ນໂທດ
// ============================================================================

/// Justification error - ຄວາມຜິດພາດເຫດຜົນແຫ່ງການພົ້ນໂທດ (Articles 61-70)
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum JustificationError {
    /// Self-defense not justified - ການປ້ອງກັນຕົວບໍ່ຊອບທຳ (Article 61)
    #[error("Self-defense not justified (Art. 61): {message_en} / {message_lao}")]
    SelfDefenseNotJustified {
        message_lao: String,
        message_en: String,
    },

    /// Excessive force in self-defense - ການໃຊ້ກຳລັງເກີນໄປ (Article 61)
    #[error("Excessive force in self-defense (Art. 61): {message_en} / {message_lao}")]
    ExcessiveSelfDefense {
        message_lao: String,
        message_en: String,
    },

    /// Necessity not established - ຄວາມຈຳເປັນບໍ່ຖືກຕ້ອງ (Article 62)
    #[error("Necessity not established (Art. 62): {message_en} / {message_lao}")]
    NecessityNotEstablished {
        message_lao: String,
        message_en: String,
    },

    /// Alternative means available - ມີທາງເລືອກອື່ນ (Article 62)
    #[error("Alternative means available (Art. 62): {message_en} / {message_lao}")]
    AlternativeAvailable {
        message_lao: String,
        message_en: String,
    },

    /// Superior order manifestly illegal - ຄຳສັ່ງຜິດກົດໝາຍຢ່າງຊັດເຈນ (Article 63)
    #[error("Superior order manifestly illegal (Art. 63): {message_en} / {message_lao}")]
    ManifestlyIllegalOrder {
        message_lao: String,
        message_en: String,
    },

    /// Consent invalid - ການຍິນຍອມບໍ່ຖືກຕ້ອງ (Article 64)
    #[error("Consent invalid (Art. 64): {message_en} / {message_lao}")]
    InvalidConsent {
        message_lao: String,
        message_en: String,
    },

    /// Consent not freely given - ການຍິນຍອມບໍ່ໄດ້ໃຫ້ໂດຍອິດສະຫຼະ (Article 64)
    #[error("Consent not freely given (Art. 64): {message_en} / {message_lao}")]
    ConsentNotFreely {
        message_lao: String,
        message_en: String,
    },

    /// Authority exceeded - ເກີນອຳນາດ (Article 65)
    #[error("Authority exceeded (Art. 65): {message_en} / {message_lao}")]
    AuthorityExceeded {
        message_lao: String,
        message_en: String,
    },
}

// ============================================================================
// Homicide Errors - ຄວາມຜິດພາດການຂ້າຄົນ
// ============================================================================

/// Homicide error - ຄວາມຜິດພາດການຂ້າຄົນ (Articles 121-128)
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum HomicideError {
    /// Premeditation not proven - ການວາງແຜນບໍ່ພິສູດໄດ້ (Article 121)
    #[error("Premeditation not proven (Art. 121): {message_en} / {message_lao}")]
    PremeditationNotProven {
        message_lao: String,
        message_en: String,
    },

    /// Special circumstances not met - ສະຖານະການພິເສດບໍ່ຄົບຖ້ວນ (Article 122)
    #[error("Special circumstances not met (Art. 122): {message_en} / {message_lao}")]
    SpecialCircumstancesNotMet {
        message_lao: String,
        message_en: String,
    },

    /// Intent not established - ເຈດຕະນາບໍ່ພິສູດໄດ້ (Article 123)
    #[error("Intent not established (Art. 123): {message_en} / {message_lao}")]
    IntentNotEstablished {
        message_lao: String,
        message_en: String,
    },

    /// Negligence standard not met - ມາດຕະຖານຄວາມປະມາດບໍ່ຄົບຖ້ວນ (Article 126)
    #[error("Negligence standard not met (Art. 126): {message_en} / {message_lao}")]
    NegligenceNotEstablished {
        message_lao: String,
        message_en: String,
    },

    /// Infanticide conditions not met - ເງື່ອນໄຂການຂ້າເດັກບໍ່ຄົບຖ້ວນ (Article 125)
    #[error("Infanticide conditions not met (Art. 125): {message_en} / {message_lao}")]
    InfanticideConditionsNotMet {
        message_lao: String,
        message_en: String,
    },

    /// Victim not deceased - ຜູ້ເສຍຫາຍບໍ່ເສຍຊີວິດ
    #[error("Victim not deceased: {message_en} / {message_lao}")]
    VictimNotDeceased {
        message_lao: String,
        message_en: String,
    },

    /// Suicide assistance conditions not met - ເງື່ອນໄຂການຊ່ວຍເຫຼືອການຂ້າຕົວຕາຍບໍ່ຄົບຖ້ວນ (Article 127)
    #[error("Suicide assistance conditions not met (Art. 127): {message_en} / {message_lao}")]
    SuicideAssistanceInvalid {
        message_lao: String,
        message_en: String,
    },
}

// ============================================================================
// Bodily Harm Errors - ຄວາມຜິດພາດການທຳຮ້າຍຮ່າງກາຍ
// ============================================================================

/// Bodily harm error - ຄວາມຜິດພາດການທຳຮ້າຍຮ່າງກາຍ (Articles 129-140)
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BodilyHarmError {
    /// Grievous harm standard not met - ມາດຕະຖານການທຳຮ້າຍຮ້າຍແຮງບໍ່ຄົບຖ້ວນ (Article 129)
    #[error("Grievous harm standard not met (Art. 129): {message_en} / {message_lao}")]
    GrievousHarmNotMet {
        message_lao: String,
        message_en: String,
    },

    /// Injury severity insufficient - ຄວາມຮ້າຍແຮງບໍ່ພຽງພໍ (Article 130)
    #[error("Injury severity insufficient (Art. 130): {message_en} / {message_lao}")]
    InsufficientSeverity {
        message_lao: String,
        message_en: String,
    },

    /// Medical treatment requirement not met - ຄວາມຕ້ອງການປິ່ນປົວບໍ່ຄົບຖ້ວນ
    #[error(
        "Medical treatment requirement not met: {days} days below {required_days} days / {message_lao}"
    )]
    InsufficientMedicalTreatment {
        days: u32,
        required_days: u32,
        message_lao: String,
        message_en: String,
    },

    /// Torture elements not proven - ອົງປະກອບການທໍລະມານບໍ່ພິສູດໄດ້ (Article 133)
    #[error("Torture elements not proven (Art. 133): {message_en} / {message_lao}")]
    TortureNotProven {
        message_lao: String,
        message_en: String,
    },

    /// No significant injury - ບໍ່ມີການບາດເຈັບທີ່ສຳຄັນ
    #[error("No significant injury: {message_en} / {message_lao}")]
    NoSignificantInjury {
        message_lao: String,
        message_en: String,
    },
}

// ============================================================================
// Sexual Crime Errors - ຄວາມຜິດພາດຄວາມຜິດທາງເພດ
// ============================================================================

/// Sexual crime error - ຄວາມຜິດພາດຄວາມຜິດທາງເພດ (Articles 141-150)
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SexualCrimeError {
    /// Victim below age of consent - ຜູ້ເສຍຫາຍອາຍຸຕ່ຳກວ່າອາຍຸຍິນຍອມ (Article 141)
    /// Age of consent in Laos: 15 years
    #[error("Victim below age of consent (Art. 141): {age} years below 15 years / {message_lao}")]
    BelowAgeOfConsent {
        age: u32,
        message_lao: String,
        message_en: String,
    },

    /// Force or threat not established - ການໃຊ້ກຳລັງຫຼືການຂົ່ມຂູ່ບໍ່ພິສູດໄດ້ (Article 141)
    #[error("Force or threat not established (Art. 141): {message_en} / {message_lao}")]
    ForceNotEstablished {
        message_lao: String,
        message_en: String,
    },

    /// Statutory rape elements not met - ອົງປະກອບການຂົ່ມຂືນເດັກບໍ່ຄົບຖ້ວນ (Article 141)
    #[error("Statutory rape elements not met (Art. 141): {message_en} / {message_lao}")]
    StatutoryRapeNotMet {
        message_lao: String,
        message_en: String,
    },

    /// Victim age verification failed - ການກວດສອບອາຍຸບໍ່ສຳເລັດ
    #[error("Victim age verification failed: {message_en} / {message_lao}")]
    AgeVerificationFailed {
        message_lao: String,
        message_en: String,
    },

    /// Sexual assault elements not proven - ອົງປະກອບການລ່ວງລະເມີດທາງເພດບໍ່ພິສູດໄດ້ (Article 142)
    #[error("Sexual assault elements not proven (Art. 142): {message_en} / {message_lao}")]
    SexualAssaultNotProven {
        message_lao: String,
        message_en: String,
    },

    /// Harassment standard not met - ມາດຕະຖານການລົບກວນບໍ່ຄົບຖ້ວນ (Article 143)
    #[error("Harassment standard not met (Art. 143): {message_en} / {message_lao}")]
    HarassmentNotMet {
        message_lao: String,
        message_en: String,
    },

    /// Trafficking elements not proven - ອົງປະກອບການຄ້າມະນຸດບໍ່ພິສູດໄດ້ (Article 145)
    #[error("Trafficking elements not proven (Art. 145): {message_en} / {message_lao}")]
    TraffickingNotProven {
        message_lao: String,
        message_en: String,
    },
}

// ============================================================================
// Property Crime Errors - ຄວາມຜິດພາດຄວາມຜິດຕໍ່ຊັບສິນ
// ============================================================================

/// Property crime error - ຄວາມຜິດພາດຄວາມຜິດຕໍ່ຊັບສິນ (Articles 161-200)
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PropertyCrimeError {
    /// Theft value below threshold - ມູນຄ່າການລັກຕ່ຳກວ່າເກນ (Article 161)
    #[error(
        "Theft value below threshold (Art. 161): {value_lak} LAK below {threshold_lak} LAK / {message_lao}"
    )]
    TheftValueBelowThreshold {
        value_lak: u64,
        threshold_lak: u64,
        message_lao: String,
        message_en: String,
    },

    /// Force or threat not established - ການໃຊ້ກຳລັງຫຼືການຂົ່ມຂູ່ບໍ່ພິສູດໄດ້ (Article 162)
    #[error("Force or threat not established for robbery (Art. 162): {message_en} / {message_lao}")]
    RobberyForceNotEstablished {
        message_lao: String,
        message_en: String,
    },

    /// Fraud elements not proven - ອົງປະກອບການສໍ້ໂກງບໍ່ພິສູດໄດ້ (Article 165)
    #[error("Fraud elements not proven (Art. 165): {message_en} / {message_lao}")]
    FraudNotProven {
        message_lao: String,
        message_en: String,
    },

    /// Position of trust not established - ຕຳແໜ່ງຄວາມໄວ້ວາງໃຈບໍ່ພິສູດໄດ້ (Article 170)
    #[error(
        "Position of trust not established for embezzlement (Art. 170): {message_en} / {message_lao}"
    )]
    TrustNotEstablished {
        message_lao: String,
        message_en: String,
    },

    /// Arson elements not proven - ອົງປະກອບການວາງເພີງບໍ່ພິສູດໄດ້ (Article 180)
    #[error("Arson elements not proven (Art. 180): {message_en} / {message_lao}")]
    ArsonNotProven {
        message_lao: String,
        message_en: String,
    },

    /// Damage insufficient for vandalism - ຄວາມເສຍຫາຍບໍ່ພຽງພໍ (Article 185)
    #[error("Damage insufficient for vandalism (Art. 185): {damage_lak} LAK / {message_lao}")]
    DamageInsufficient {
        damage_lak: u64,
        message_lao: String,
        message_en: String,
    },

    /// Extortion elements not proven - ອົງປະກອບການຂູດຮີດບໍ່ພິສູດໄດ້ (Article 190)
    #[error("Extortion elements not proven (Art. 190): {message_en} / {message_lao}")]
    ExtortionNotProven {
        message_lao: String,
        message_en: String,
    },
}

// ============================================================================
// Age Errors - ຄວາມຜິດພາດທີ່ກ່ຽວກັບອາຍຸ
// ============================================================================

/// Age-related error - ຄວາມຜິດພາດທີ່ກ່ຽວກັບອາຍຸ
///
/// Handles age-related requirements under the Criminal Code.
///
/// # Key Age Thresholds
/// - **Criminal responsibility**: 16 years general, 14 years for serious crimes (Article 16)
/// - **Age of consent**: 15 years for sexual crimes (Article 141)
/// - **Full capacity**: 18 years (aligned with Civil Code)
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AgeError {
    /// Below minimum age for criminal responsibility - ຕ່ຳກວ່າອາຍຸຮັບຜິດຊອບທາງອາຍາ (Article 16)
    #[error(
        "Below minimum age for criminal responsibility (Art. 16): {age} years below {required_age} years / {message_lao}"
    )]
    BelowCriminalAge {
        age: u32,
        required_age: u32,
        message_lao: String,
        message_en: String,
    },

    /// Below age of consent for sexual crimes - ຕ່ຳກວ່າອາຍຸຍິນຍອມ (Article 141)
    #[error("Below age of consent (Art. 141): {age} years below 15 years / {message_lao}")]
    BelowAgeOfConsent {
        age: u32,
        message_lao: String,
        message_en: String,
    },

    /// Age verification required - ຕ້ອງການກວດສອບອາຍຸ
    #[error("Age verification required: {message_en} / {message_lao}")]
    AgeVerificationRequired {
        message_lao: String,
        message_en: String,
    },

    /// Invalid age value - ຄ່າອາຍຸບໍ່ຖືກຕ້ອງ
    #[error("Invalid age value: {age} / {message_lao}")]
    InvalidAge {
        age: u32,
        message_lao: String,
        message_en: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criminal_code_error_display() {
        let error = CriminalCodeError::Age(AgeError::BelowCriminalAge {
            age: 15,
            required_age: 16,
            message_lao: "ອາຍຸຕ່ຳກວ່າທີ່ກຳນົດ".to_string(),
            message_en: "Age below required threshold".to_string(),
        });
        let display = format!("{}", error);
        assert!(display.contains("15"));
        assert!(display.contains("16"));
    }

    #[test]
    fn test_liability_error() {
        let error = LiabilityError::InsufficientMensRea {
            article: 14,
            message_lao: "ສະພາບຈິດໃຈບໍ່ພຽງພໍ".to_string(),
            message_en: "Insufficient guilty mind".to_string(),
        };
        assert!(error.to_string().contains("Art. 14"));
    }

    #[test]
    fn test_sexual_crime_age_of_consent() {
        let error = SexualCrimeError::BelowAgeOfConsent {
            age: 14,
            message_lao: "ຕ່ຳກວ່າອາຍຸຍິນຍອມ 15 ປີ".to_string(),
            message_en: "Below age of consent (15 years)".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("14"));
        assert!(display.contains("15"));
    }

    #[test]
    fn test_penalty_error_range() {
        let error = PenaltyError::ImprisonmentExceedsMaximum {
            years: 25,
            max_years: 20,
            message_lao: "ໄລຍະຈຳຄຸກເກີນສູງສຸດ".to_string(),
            message_en: "Imprisonment exceeds maximum".to_string(),
        };
        assert!(error.to_string().contains("25"));
        assert!(error.to_string().contains("20"));
    }
}
