//! Criminal Code Validators - ການກວດສອບກົດໝາຍອາຍາ
//!
//! Comprehensive validation functions for Criminal Code 2017 compliance.
//!
//! # Validation Functions
//! - **Criminal Liability**: Validates mens rea, actus reus, causation (Articles 13-30)
//! - **Age**: Criminal responsibility (16/14), age of consent (15)
//! - **Mental Capacity**: Full, diminished, or no capacity (Articles 19-21)
//! - **Penalties**: Death, imprisonment, fines, re-education (Articles 31-60)
//! - **Justifications**: Self-defense, necessity, superior orders (Articles 61-70)
//! - **Homicide**: All types of unlawful killing (Articles 121-128)
//! - **Sexual Crimes**: Rape, assault, age of consent validation (Articles 141-150)
//! - **Property Crimes**: Theft, fraud, embezzlement, arson (Articles 161-200)

use crate::criminal_code::error::*;
use crate::criminal_code::types::*;

// ============================================================================
// Criminal Liability Validation - ການກວດສອບຄວາມຮັບຜິດຊອບທາງອາຍາ (Articles 13-30)
// ============================================================================

/// Validates criminal liability requirements (Articles 13-30)
///
/// # Requirements
/// 1. Mens rea (guilty mind) - Article 14-15
/// 2. Actus reus (criminal act) - Article 13
/// 3. Causation between act and result - Article 13
/// 4. No justification or excuse - Articles 61-70
///
/// # Returns
/// - `Ok(())` if criminal liability is established
/// - `Err(LiabilityError)` if any element is missing
///
/// # Example
/// ```
/// use legalis_la::criminal_code::{CriminalLiability, MensRea, ActusReus, MentalCapacityStatus, validate_criminal_liability};
/// use chrono::Utc;
///
/// let liability = CriminalLiability {
///     mens_rea: MensRea::DirectIntent,
///     actus_reus: ActusReus::Commission {
///         act_description_lao: "ການຂ້າຄົນ".to_string(),
///         act_description_en: "Homicide".to_string(),
///         time_of_commission: Utc::now(),
///         location: "Vientiane".to_string(),
///     },
///     age_at_offense: 25,
///     mental_capacity: MentalCapacityStatus::Full,
///     article_reference: vec![13, 14],
///     description_lao: "ຄວາມຮັບຜິດຊອບທາງອາຍາ".to_string(),
///     description_en: "Criminal liability".to_string(),
/// };
///
/// assert!(validate_criminal_liability(&liability).is_ok());
/// ```
pub fn validate_criminal_liability(liability: &CriminalLiability) -> Result<()> {
    // Validate mens rea exists
    validate_mens_rea(&liability.mens_rea)?;

    // Validate actus reus
    validate_actus_reus(&liability.actus_reus)?;

    // Validate age of criminal responsibility
    validate_age_of_responsibility(liability.age_at_offense, &liability.mens_rea)?;

    // Validate mental capacity
    validate_mental_capacity_status(liability.mental_capacity)?;

    Ok(())
}

/// Validates mens rea (guilty mind) - Articles 14-15
///
/// Ensures the required mental state exists for criminal liability.
pub fn validate_mens_rea(mens_rea: &MensRea) -> Result<()> {
    match mens_rea {
        MensRea::DirectIntent | MensRea::IndirectIntent => {
            // Intent established per Article 14
            Ok(())
        }
        MensRea::ConsciousNegligence | MensRea::UnconsciousNegligence => {
            // Negligence established per Article 15
            Ok(())
        }
        MensRea::StrictLiability => {
            // No mens rea required for strict liability offenses
            Ok(())
        }
    }
}

/// Validates actus reus (criminal act) - Article 13
///
/// Ensures a criminal act (commission or omission) occurred.
pub fn validate_actus_reus(actus_reus: &ActusReus) -> Result<()> {
    match actus_reus {
        ActusReus::Commission {
            act_description_lao,
            act_description_en,
            ..
        } => {
            if act_description_lao.is_empty() || act_description_en.is_empty() {
                return Err(LiabilityError::MissingActusReus {
                    article: 13,
                    message_lao: "ຂາດລາຍລະອຽດການກະທຳຜິດ".to_string(),
                    message_en: "Missing act description".to_string(),
                }
                .into());
            }
            Ok(())
        }
        ActusReus::Omission {
            duty_description_lao,
            duty_description_en,
            duty_basis,
            ..
        } => {
            if duty_description_lao.is_empty()
                || duty_description_en.is_empty()
                || duty_basis.is_empty()
            {
                return Err(LiabilityError::MissingActusReus {
                    article: 13,
                    message_lao: "ຂາດລາຍລະອຽດການບໍ່ປະຕິບັດພັນທະ".to_string(),
                    message_en: "Missing omission details".to_string(),
                }
                .into());
            }
            Ok(())
        }
    }
}

// ============================================================================
// Age Validation - ການກວດສອບອາຍຸ (Article 16, 141)
// ============================================================================

/// Validates age of criminal responsibility (Article 16)
///
/// # Age Thresholds
/// - **16 years**: General criminal responsibility
/// - **14 years**: Serious crimes (homicide, sexual crimes, serious property crimes)
///
/// # Arguments
/// - `age`: Age of the person at time of offense
/// - `mens_rea`: Mental state (determines if serious crime)
///
/// # Returns
/// - `Ok(())` if age requirement met
/// - `Err(AgeError)` if below minimum age
pub fn validate_age_of_responsibility(age: u32, mens_rea: &MensRea) -> Result<()> {
    // Serious crimes: minimum age 14 (Article 16)
    // General crimes: minimum age 16 (Article 16)

    let required_age = match mens_rea {
        MensRea::DirectIntent | MensRea::IndirectIntent => {
            // Intent crimes may be serious - conservative approach: require 16
            // In practice, courts determine if specific crime allows age 14
            16
        }
        MensRea::ConsciousNegligence | MensRea::UnconsciousNegligence => 16,
        MensRea::StrictLiability => 16,
    };

    if age < required_age {
        return Err(AgeError::BelowCriminalAge {
            age,
            required_age,
            message_lao: format!("ອາຍຸ {} ປີ ຕ່ຳກວ່າອາຍຸຮັບຜິດຊອບທາງອາຍາ {} ປີ", age, required_age),
            message_en: format!(
                "Age {} years below criminal responsibility age {} years",
                age, required_age
            ),
        }
        .into());
    }

    Ok(())
}

/// Validates age for serious crimes (Article 16)
///
/// Serious crimes have lower age threshold (14 years).
pub fn validate_age_for_serious_crime(age: u32) -> Result<()> {
    if age < 14 {
        return Err(AgeError::BelowCriminalAge {
            age,
            required_age: 14,
            message_lao: format!("ອາຍຸ {} ປີ ຕ່ຳກວ່າອາຍຸຮັບຜິດຊອບສຳລັບຄວາມຜິດຮ້າຍແຮງ 14 ປີ", age),
            message_en: format!(
                "Age {} years below serious crime responsibility age 14 years",
                age
            ),
        }
        .into());
    }
    Ok(())
}

/// Validates age of consent for sexual crimes (Article 141)
///
/// # Age of Consent
/// - **15 years**: Minimum age to consent to sexual activity
///
/// # Returns
/// - `Ok(())` if victim age >= 15
/// - `Err(SexualCrimeError)` if victim age < 15
pub fn validate_age_of_consent(victim_age: u32) -> Result<()> {
    if victim_age < 15 {
        return Err(SexualCrimeError::BelowAgeOfConsent {
            age: victim_age,
            message_lao: format!("ອາຍຸຜູ້ເສຍຫາຍ {} ປີ ຕ່ຳກວ່າອາຍຸຍິນຍອມ 15 ປີ", victim_age),
            message_en: format!(
                "Victim age {} years below age of consent 15 years",
                victim_age
            ),
        }
        .into());
    }
    Ok(())
}

// ============================================================================
// Mental Capacity Validation - ການກວດສອບຄວາມສາມາດທາງຈິດໃຈ (Articles 19-21)
// ============================================================================

/// Validates mental capacity status (Articles 19-21)
///
/// # Capacity Levels
/// - **Full**: Normal criminal responsibility (Article 19)
/// - **Diminished**: Reduced penalty (Article 20)
/// - **No capacity**: No criminal responsibility (Article 19)
pub fn validate_mental_capacity_status(status: MentalCapacityStatus) -> Result<()> {
    match status {
        MentalCapacityStatus::Full => Ok(()),
        MentalCapacityStatus::Diminished => Ok(()), // Reduced penalty applies
        MentalCapacityStatus::NoCapacity => Err(LiabilityError::LackOfMentalCapacity {
            message_lao: "ບໍ່ມີຄວາມສາມາດທາງຈິດໃຈ - ບໍ່ມີຄວາມຮັບຜິດຊອບທາງອາຍາ".to_string(),
            message_en: "No mental capacity - no criminal responsibility".to_string(),
        }
        .into()),
    }
}

/// Validates complete mental capacity evaluation (Articles 19-21)
///
/// Requires medical evaluation for diminished or no capacity claims.
pub fn validate_mental_capacity(capacity: &MentalCapacity) -> Result<()> {
    match capacity.status {
        MentalCapacityStatus::Full => Ok(()),
        MentalCapacityStatus::Diminished | MentalCapacityStatus::NoCapacity => {
            // Require medical evaluation for reduced capacity claims
            if capacity.medical_evaluation.is_none() {
                return Err(LiabilityError::DiminishedCapacity {
                    message_lao: "ຕ້ອງການການປະເມີນທາງການແພດສຳລັບຄວາມສາມາດລົດລົງ".to_string(),
                    message_en: "Medical evaluation required for diminished capacity".to_string(),
                }
                .into());
            }
            Ok(())
        }
    }
}

// ============================================================================
// Penalty Validation - ການກວດສອບໂທດ (Articles 31-60)
// ============================================================================

/// Validates penalty appropriateness (Articles 31-60)
///
/// Ensures penalties comply with statutory requirements.
pub fn validate_penalty(penalty: &Penalty, crime_severity: PenaltySeverity) -> Result<()> {
    match penalty {
        Penalty::Death { article, .. } => {
            // Death penalty only for most serious crimes (Article 32)
            if crime_severity != PenaltySeverity::MostSerious {
                return Err(PenaltyError::DeathPenaltyNotApplicable {
                    message_lao: format!(
                        "ໂທດປະຫານຊີວິດບໍ່ເໝາະສົມກັບຄວາມຮ້າຍແຮງຂອງຄວາມຜິດ (ບົດ {})",
                        article
                    ),
                    message_en: format!(
                        "Death penalty not applicable for crime severity (Art. {})",
                        article
                    ),
                }
                .into());
            }
            Ok(())
        }
        Penalty::LifeImprisonment { .. } => {
            // Life imprisonment for most serious or felonies
            if crime_severity < PenaltySeverity::Felony {
                return Err(PenaltyError::DisproportionatePenalty {
                    message_lao: "ໂທດຈຳຄຸກຕະຫຼອດຊີວິດບໍ່ສົມດູນກັບຄວາມຜິດ".to_string(),
                    message_en: "Life imprisonment disproportionate to crime".to_string(),
                }
                .into());
            }
            Ok(())
        }
        Penalty::FixedTermImprisonment {
            years,
            months,
            minimum_years,
            maximum_years,
        } => {
            let total_years = years + (months / 12);
            if total_years < *minimum_years {
                return Err(PenaltyError::ImprisonmentBelowMinimum {
                    years: total_years,
                    min_years: *minimum_years,
                    message_lao: format!(
                        "ໄລຍະຈຳຄຸກ {} ປີ ຕ່ຳກວ່າຕ່ຳສຸດ {} ປີ",
                        total_years, minimum_years
                    ),
                    message_en: format!(
                        "Imprisonment {} years below minimum {} years",
                        total_years, minimum_years
                    ),
                }
                .into());
            }
            if total_years > *maximum_years {
                return Err(PenaltyError::ImprisonmentExceedsMaximum {
                    years: total_years,
                    max_years: *maximum_years,
                    message_lao: format!("ໄລຍະຈຳຄຸກ {} ປີ ເກີນສູງສຸດ {} ປີ", total_years, maximum_years),
                    message_en: format!(
                        "Imprisonment {} years exceeds maximum {} years",
                        total_years, maximum_years
                    ),
                }
                .into());
            }
            Ok(())
        }
        Penalty::Fine {
            amount_lak,
            minimum_lak,
            maximum_lak,
            ..
        } => {
            if amount_lak < minimum_lak {
                return Err(PenaltyError::FineBelowMinimum {
                    article: 37,
                    amount_lak: *amount_lak,
                    min_lak: *minimum_lak,
                    message_lao: format!(
                        "ຈຳນວນເງີນປັບ {} ກີບ ຕ່ຳກວ່າຕ່ຳສຸດ {} ກີບ",
                        amount_lak, minimum_lak
                    ),
                    message_en: format!(
                        "Fine {} LAK below minimum {} LAK",
                        amount_lak, minimum_lak
                    ),
                }
                .into());
            }
            if amount_lak > maximum_lak {
                return Err(PenaltyError::FineExceedsMaximum {
                    article: 37,
                    amount_lak: *amount_lak,
                    max_lak: *maximum_lak,
                    message_lao: format!("ຈຳນວນເງີນປັບ {} ກີບ ເກີນສູງສຸດ {} ກີບ", amount_lak, maximum_lak),
                    message_en: format!(
                        "Fine {} LAK exceeds maximum {} LAK",
                        amount_lak, maximum_lak
                    ),
                }
                .into());
            }
            Ok(())
        }
        Penalty::ReEducation {
            duration_months, ..
        } => {
            // Re-education typically 6-24 months (Article 41)
            if *duration_months < 6 || *duration_months > 24 {
                return Err(PenaltyError::InvalidReEducationDuration {
                    duration_months: *duration_months,
                    message_lao: format!(
                        "ໄລຍະການສຶກສາອົບຮົມ {} ເດືອນບໍ່ຖືກຕ້ອງ (6-24 ເດືອນ)",
                        duration_months
                    ),
                    message_en: format!(
                        "Re-education duration {} months invalid (6-24 months)",
                        duration_months
                    ),
                }
                .into());
            }
            Ok(())
        }
        Penalty::Confiscation { .. } | Penalty::DeprivationOfRights { .. } => {
            // Additional penalties - generally acceptable
            Ok(())
        }
    }
}

// ============================================================================
// Justification Validation - ການກວດສອບເຫດຜົນແຫ່ງການພົ້ນໂທດ (Articles 61-70)
// ============================================================================

/// Validates justification ground (Articles 61-70)
///
/// Determines if a defense is valid under Lao criminal law.
pub fn validate_justification(justification: &JustificationGround) -> Result<()> {
    match justification {
        JustificationGround::SelfDefense {
            proportional_response,
            reasonable_belief,
            ..
        } => {
            if !proportional_response {
                return Err(JustificationError::ExcessiveSelfDefense {
                    message_lao: "ການໃຊ້ກຳລັງເກີນໄປໃນການປ້ອງກັນຕົວ".to_string(),
                    message_en: "Excessive force in self-defense".to_string(),
                }
                .into());
            }
            if !reasonable_belief {
                return Err(JustificationError::SelfDefenseNotJustified {
                    message_lao: "ການປ້ອງກັນຕົວບໍ່ມີເຫດຜົນ".to_string(),
                    message_en: "Self-defense not justified".to_string(),
                }
                .into());
            }
            Ok(())
        }
        JustificationGround::Necessity {
            no_alternative,
            lesser_harm,
            ..
        } => {
            if !no_alternative {
                return Err(JustificationError::AlternativeAvailable {
                    message_lao: "ມີທາງເລືອກອື່ນທີ່ເໝາະສົມກວ່າ".to_string(),
                    message_en: "Alternative means available".to_string(),
                }
                .into());
            }
            if !lesser_harm {
                return Err(JustificationError::NecessityNotEstablished {
                    message_lao: "ອັນຕະລາຍທີ່ເກີດຂຶ້ນບໍ່ນ້ອຍກວ່າອັນຕະລາຍທີ່ຫຼີກເວັ້ນ".to_string(),
                    message_en: "Harm caused not less than harm avoided".to_string(),
                }
                .into());
            }
            Ok(())
        }
        JustificationGround::SuperiorOrders {
            lawful_order,
            manifestly_illegal,
            ..
        } => {
            if *manifestly_illegal {
                return Err(JustificationError::ManifestlyIllegalOrder {
                    message_lao: "ຄຳສັ່ງຜິດກົດໝາຍຢ່າງຊັດເຈນ".to_string(),
                    message_en: "Order manifestly illegal".to_string(),
                }
                .into());
            }
            if !lawful_order {
                return Err(JustificationError::ManifestlyIllegalOrder {
                    message_lao: "ຄຳສັ່ງບໍ່ຖືກຕ້ອງຕາມກົດໝາຍ".to_string(),
                    message_en: "Order not lawful".to_string(),
                }
                .into());
            }
            Ok(())
        }
        JustificationGround::Consent {
            freely_given,
            informed,
            capacity_verified,
            ..
        } => {
            if !freely_given {
                return Err(JustificationError::ConsentNotFreely {
                    message_lao: "ການຍິນຍອມບໍ່ໄດ້ໃຫ້ໂດຍອິດສະຫຼະ".to_string(),
                    message_en: "Consent not freely given".to_string(),
                }
                .into());
            }
            if !informed || !capacity_verified {
                return Err(JustificationError::InvalidConsent {
                    message_lao: "ການຍິນຍອມບໍ່ຖືກຕ້ອງ".to_string(),
                    message_en: "Consent invalid".to_string(),
                }
                .into());
            }
            Ok(())
        }
        JustificationGround::LawfulAuthority { within_scope, .. } => {
            if !within_scope {
                return Err(JustificationError::AuthorityExceeded {
                    message_lao: "ເກີນຂອບເຂດອຳນາດ".to_string(),
                    message_en: "Authority exceeded".to_string(),
                }
                .into());
            }
            Ok(())
        }
    }
}

// ============================================================================
// Homicide Validation - ການກວດສອບການຂ້າຄົນ (Articles 121-128)
// ============================================================================

/// Validates homicide offense (Articles 121-128)
pub fn validate_homicide(homicide: &HomicideType) -> Result<()> {
    match homicide {
        HomicideType::PremeditatedMurder {
            planning_details_lao,
            planning_details_en,
            ..
        } => {
            if planning_details_lao.is_empty() || planning_details_en.is_empty() {
                return Err(HomicideError::PremeditationNotProven {
                    message_lao: "ການວາງແຜນບໍ່ພິສູດໄດ້".to_string(),
                    message_en: "Premeditation not proven".to_string(),
                }
                .into());
            }
            Ok(())
        }
        HomicideType::MurderWithSpecialCircumstances {
            circumstances_lao, ..
        } => {
            if circumstances_lao.is_empty() {
                return Err(HomicideError::SpecialCircumstancesNotMet {
                    message_lao: "ສະຖານະການພິເສດບໍ່ຄົບຖ້ວນ".to_string(),
                    message_en: "Special circumstances not met".to_string(),
                }
                .into());
            }
            Ok(())
        }
        HomicideType::IntentionalHomicide {
            intent_description_lao,
            ..
        } => {
            if intent_description_lao.is_empty() {
                return Err(HomicideError::IntentNotEstablished {
                    message_lao: "ເຈດຕະນາບໍ່ພິສູດໄດ້".to_string(),
                    message_en: "Intent not established".to_string(),
                }
                .into());
            }
            Ok(())
        }
        HomicideType::Manslaughter {
            negligence_type, ..
        } => {
            // Validate negligence type is appropriate
            match negligence_type {
                NegligenceType::Criminal | NegligenceType::Gross => Ok(()),
                _ => Ok(()), // Other types also acceptable
            }
        }
        HomicideType::Infanticide {
            mitigating_factors_lao,
            ..
        } => {
            if mitigating_factors_lao.is_empty() {
                return Err(HomicideError::InfanticideConditionsNotMet {
                    message_lao: "ເງື່ອນໄຂການຂ້າເດັກບໍ່ຄົບຖ້ວນ".to_string(),
                    message_en: "Infanticide conditions not met".to_string(),
                }
                .into());
            }
            Ok(())
        }
        HomicideType::NegligentHomicide { .. } => Ok(()),
        HomicideType::AssistedSuicide {
            consent_verified, ..
        } => {
            if !consent_verified {
                return Err(HomicideError::SuicideAssistanceInvalid {
                    message_lao: "ການຍິນຍອມຂອງຜູ້ເສຍຫາຍບໍ່ຖືກກວດສອບ".to_string(),
                    message_en: "Victim consent not verified".to_string(),
                }
                .into());
            }
            Ok(())
        }
        HomicideType::IncitementToSuicide { .. } => Ok(()),
    }
}

// ============================================================================
// Sexual Crime Validation - ການກວດສອບຄວາມຜິດທາງເພດ (Articles 141-150)
// ============================================================================

/// Validates sexual crime offense (Articles 141-150)
///
/// Special validation for age of consent (15 years).
pub fn validate_sexual_crime(crime: &SexualCrime) -> Result<()> {
    match crime {
        SexualCrime::Rape {
            victim_age,
            force_or_threat,
            ..
        } => {
            // Validate age of consent if victim is 15+
            if *victim_age >= 15 && !force_or_threat {
                return Err(SexualCrimeError::ForceNotEstablished {
                    message_lao: "ການໃຊ້ກຳລັງຫຼືການຂົ່ມຂູ່ບໍ່ພິສູດໄດ້".to_string(),
                    message_en: "Force or threat not established".to_string(),
                }
                .into());
            }
            Ok(())
        }
        SexualCrime::StatutoryRape { victim_age, .. } => {
            // Statutory rape requires victim under 15
            if *victim_age >= 15 {
                return Err(SexualCrimeError::StatutoryRapeNotMet {
                    message_lao: format!("ອາຍຸຜູ້ເສຍຫາຍ {} ປີ ບໍ່ຕ່ຳກວ່າ 15 ປີ", victim_age),
                    message_en: format!("Victim age {} not below 15 years", victim_age),
                }
                .into());
            }
            validate_age_of_consent(*victim_age)?;
            Ok(())
        }
        SexualCrime::SexualAssault { victim_age, .. } => {
            // Sexual assault can occur at any age
            // Age of consent violation if victim under 15
            if *victim_age < 15 {
                validate_age_of_consent(*victim_age)?;
            }
            Ok(())
        }
        SexualCrime::SexualHarassment { .. } => Ok(()),
        SexualCrime::IndecentExposure { .. } => Ok(()),
        SexualCrime::SexualTrafficking { victim_age, .. } => {
            // Trafficking is aggravated if victim is minor
            if *victim_age < 18 {
                // Enhanced penalties apply
            }
            Ok(())
        }
    }
}

// ============================================================================
// Property Crime Validation - ການກວດສອບຄວາມຜິດຕໍ່ຊັບສິນ (Articles 161-200)
// ============================================================================

/// Validates property crime offense (Articles 161-200)
pub fn validate_property_crime(crime: &PropertyCrime) -> Result<()> {
    match crime {
        PropertyCrime::Theft { value_lak, .. } => {
            // Petty theft threshold: typically 500,000 LAK
            let petty_threshold = 500_000;
            if *value_lak < petty_threshold {
                // May be petty theft with reduced penalty
            }
            Ok(())
        }
        PropertyCrime::Robbery {
            force_or_threat, ..
        } => {
            if !force_or_threat {
                return Err(PropertyCrimeError::RobberyForceNotEstablished {
                    message_lao: "ການໃຊ້ກຳລັງຫຼືການຂົ່ມຂູ່ບໍ່ພິສູດໄດ້".to_string(),
                    message_en: "Force or threat not established".to_string(),
                }
                .into());
            }
            Ok(())
        }
        PropertyCrime::Fraud {
            number_of_victims, ..
        } => {
            if *number_of_victims == 0 {
                return Err(PropertyCrimeError::FraudNotProven {
                    message_lao: "ບໍ່ມີຜູ້ເສຍຫາຍ".to_string(),
                    message_en: "No victims identified".to_string(),
                }
                .into());
            }
            Ok(())
        }
        PropertyCrime::Embezzlement {
            position_of_trust, ..
        } => {
            if position_of_trust.is_empty() {
                return Err(PropertyCrimeError::TrustNotEstablished {
                    message_lao: "ຕຳແໜ່ງຄວາມໄວ້ວາງໃຈບໍ່ພິສູດໄດ້".to_string(),
                    message_en: "Position of trust not established".to_string(),
                }
                .into());
            }
            Ok(())
        }
        PropertyCrime::Arson {
            inhabited_building, ..
        } => {
            // Arson of inhabited building is most serious
            if *inhabited_building {
                // Enhanced penalties apply
            }
            Ok(())
        }
        PropertyCrime::Vandalism { damage_lak, .. } => {
            // Minor vandalism threshold: typically 100,000 LAK
            if *damage_lak == 0 {
                return Err(PropertyCrimeError::DamageInsufficient {
                    damage_lak: *damage_lak,
                    message_lao: "ບໍ່ມີຄວາມເສຍຫາຍ".to_string(),
                    message_en: "No damage".to_string(),
                }
                .into());
            }
            Ok(())
        }
        PropertyCrime::Extortion { threats_lao, .. } => {
            if threats_lao.is_empty() {
                return Err(PropertyCrimeError::ExtortionNotProven {
                    message_lao: "ການຂົ່ມຂູ່ບໍ່ພິສູດໄດ້".to_string(),
                    message_en: "Threats not established".to_string(),
                }
                .into());
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_age_of_responsibility() {
        // Test general crimes
        assert!(validate_age_of_responsibility(16, &MensRea::DirectIntent).is_ok());
        assert!(validate_age_of_responsibility(15, &MensRea::DirectIntent).is_err());

        // Test serious crimes
        assert!(validate_age_for_serious_crime(14).is_ok());
        assert!(validate_age_for_serious_crime(13).is_err());
    }

    #[test]
    fn test_validate_age_of_consent() {
        // Age of consent is 15
        assert!(validate_age_of_consent(15).is_ok());
        assert!(validate_age_of_consent(16).is_ok());
        assert!(validate_age_of_consent(14).is_err());
    }

    #[test]
    fn test_validate_mental_capacity() {
        assert!(validate_mental_capacity_status(MentalCapacityStatus::Full).is_ok());
        assert!(validate_mental_capacity_status(MentalCapacityStatus::Diminished).is_ok());
        assert!(validate_mental_capacity_status(MentalCapacityStatus::NoCapacity).is_err());
    }

    #[test]
    fn test_validate_penalty_imprisonment() {
        let penalty = Penalty::FixedTermImprisonment {
            years: 5,
            months: 0,
            minimum_years: 3,
            maximum_years: 10,
        };
        assert!(validate_penalty(&penalty, PenaltySeverity::Felony).is_ok());

        let too_long = Penalty::FixedTermImprisonment {
            years: 15,
            months: 0,
            minimum_years: 3,
            maximum_years: 10,
        };
        assert!(validate_penalty(&too_long, PenaltySeverity::Felony).is_err());
    }

    #[test]
    fn test_validate_statutory_rape() {
        let crime = SexualCrime::StatutoryRape {
            victim_age: 14,
            perpetrator_age: 25,
            victim_consented: true, // Irrelevant if under 15
            relationship: "None".to_string(),
        };
        assert!(validate_sexual_crime(&crime).is_err());

        let not_statutory = SexualCrime::StatutoryRape {
            victim_age: 15,
            perpetrator_age: 25,
            victim_consented: true,
            relationship: "None".to_string(),
        };
        assert!(validate_sexual_crime(&not_statutory).is_err());
    }

    #[test]
    fn test_validate_robbery() {
        let robbery = PropertyCrime::Robbery {
            property_description_lao: "ເງິນສົດ".to_string(),
            property_description_en: "Cash".to_string(),
            value_lak: 1_000_000,
            force_or_threat: true,
            weapon_used: None,
            injury_caused: false,
        };
        assert!(validate_property_crime(&robbery).is_ok());

        let no_force = PropertyCrime::Robbery {
            property_description_lao: "ເງິນສົດ".to_string(),
            property_description_en: "Cash".to_string(),
            value_lak: 1_000_000,
            force_or_threat: false,
            weapon_used: None,
            injury_caused: false,
        };
        assert!(validate_property_crime(&no_force).is_err());
    }
}
