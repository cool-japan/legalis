//! Anti-Corruption Law Validators (ການກວດສອບກົດໝາຍຕ້ານການສໍ້ລາດບັງຫຼວງ)
//!
//! Validation functions for Lao anti-corruption law based on:
//! - **Anti-Corruption Law 2012** (Law No. 03/NA, amended 2019)
//!
//! # Validation Categories
//!
//! - **Corruption Offense Validators**: Validate corruption offenses
//! - **Asset Declaration Validators**: Validate asset declarations
//! - **Investigation Validators**: Validate investigation procedures
//! - **Penalty Validators**: Validate penalty determinations
//! - **Whistleblower Validators**: Validate whistleblower reports and protections
//! - **Prevention Validators**: Validate prevention measures
//! - **International Cooperation Validators**: Validate international cooperation requests

use crate::anti_corruption_law::error::{AntiCorruptionLawError, AntiCorruptionLawResult};
use crate::anti_corruption_law::types::*;

// ============================================================================
// Corruption Offense Validators - ການກວດສອບການກະທຳຜິດສໍ້ລາດບັງຫຼວງ
// ============================================================================

/// Validate a corruption offense
/// ກວດສອບການກະທຳຜິດສໍ້ລາດບັງຫຼວງ
///
/// # Arguments
/// * `offense` - The corruption offense to validate
///
/// # Returns
/// * `Ok(())` if the offense is valid
/// * `Err(AntiCorruptionLawError)` if validation fails
pub fn validate_corruption_offense(offense: &CorruptionOffense) -> AntiCorruptionLawResult<()> {
    // Check that description is not empty
    if offense.description_lao.is_empty() || offense.description_en.is_empty() {
        return Err(AntiCorruptionLawError::InvalidCorruptionOffense {
            article: 25,
            message_lao: "ຕ້ອງມີລາຍລະອຽດການກະທຳຜິດ".to_string(),
            message_en: "Offense description is required".to_string(),
        });
    }

    // Check that location is provided
    if offense.location_province.is_empty() {
        return Err(AntiCorruptionLawError::InvalidCorruptionOffense {
            article: 25,
            message_lao: "ຕ້ອງລະບຸສະຖານທີ່".to_string(),
            message_en: "Location must be specified".to_string(),
        });
    }

    // Validate specific offense types
    match &offense.offense_type {
        CorruptionOffenseType::Bribery { amount_lak, .. } => {
            if *amount_lak == 0 {
                return Err(AntiCorruptionLawError::BriberyOffense {
                    article: 25,
                    message_lao: "ຈຳນວນເງິນສິນບົນຕ້ອງລະບຸ".to_string(),
                    message_en: "Bribery amount must be specified".to_string(),
                });
            }
        }
        CorruptionOffenseType::Embezzlement { amount_lak, .. } => {
            if *amount_lak == 0 {
                return Err(AntiCorruptionLawError::EmbezzlementOffense {
                    article: 28,
                    message_lao: "ຈຳນວນເງິນສໍ້ໂກງຕ້ອງລະບຸ".to_string(),
                    message_en: "Embezzlement amount must be specified".to_string(),
                });
            }
        }
        CorruptionOffenseType::IllicitEnrichment {
            unexplained_assets_lak,
            declared_income_lak,
        } => {
            if *unexplained_assets_lak <= *declared_income_lak {
                return Err(AntiCorruptionLawError::IllicitEnrichment {
                    article: 42,
                    message_lao: "ຊັບສິນທີ່ບໍ່ສາມາດອະທິບາຍໄດ້ຕ້ອງເກີນລາຍຮັບທີ່ປະກາດ".to_string(),
                    message_en: "Unexplained assets must exceed declared income".to_string(),
                    unexplained_wealth_lak: *unexplained_assets_lak,
                });
            }
        }
        _ => {}
    }

    Ok(())
}

// ============================================================================
// Asset Declaration Validators - ການກວດສອບການປະກາດຊັບສິນ
// ============================================================================

/// Validate an asset declaration
/// ກວດສອບການປະກາດຊັບສິນ
///
/// # Arguments
/// * `declaration` - The asset declaration to validate
///
/// # Returns
/// * `Ok(())` if the declaration is valid
/// * `Err(AntiCorruptionLawError)` if validation fails
pub fn validate_asset_declaration(declaration: &AssetDeclaration) -> AntiCorruptionLawResult<()> {
    // Check required fields
    if declaration.official_id.is_empty() {
        return Err(AntiCorruptionLawError::InvalidAssetDeclaration {
            article: 50,
            message_lao: "ຕ້ອງມີລະຫັດພະນັກງານ".to_string(),
            message_en: "Official ID is required".to_string(),
        });
    }

    if declaration.official_name_lao.is_empty() || declaration.official_name_en.is_empty() {
        return Err(AntiCorruptionLawError::InvalidAssetDeclaration {
            article: 50,
            message_lao: "ຕ້ອງມີຊື່ພະນັກງານ".to_string(),
            message_en: "Official name is required".to_string(),
        });
    }

    if declaration.ministry.is_empty() {
        return Err(AntiCorruptionLawError::InvalidAssetDeclaration {
            article: 50,
            message_lao: "ຕ້ອງລະບຸກະຊວງ/ອົງການ".to_string(),
            message_en: "Ministry/Organization is required".to_string(),
        });
    }

    // Validate completeness
    validate_asset_declaration_completeness(declaration)?;

    Ok(())
}

/// Validate asset declaration completeness
/// ກວດສອບຄວາມຄົບຖ້ວນຂອງການປະກາດຊັບສິນ
pub fn validate_asset_declaration_completeness(
    declaration: &AssetDeclaration,
) -> AntiCorruptionLawResult<()> {
    let mut missing_fields = Vec::new();

    // Check for minimum required information
    if declaration.income_sources.is_empty() {
        missing_fields.push("income_sources".to_string());
    }

    if declaration.total_assets_lak == 0
        && declaration.real_estate.is_empty()
        && declaration.vehicles.is_empty()
        && declaration.bank_balance_lak == 0
    {
        missing_fields.push("assets".to_string());
    }

    if !missing_fields.is_empty() {
        return Err(AntiCorruptionLawError::IncompleteDeclaration {
            missing_fields,
            message_lao: "ການປະກາດຊັບສິນບໍ່ຄົບຖ້ວນ".to_string(),
            message_en: "Asset declaration is incomplete".to_string(),
        });
    }

    Ok(())
}

/// Validate declaration deadline compliance
/// ກວດສອບການປະຕິບັດຕາມກຳນົດເວລາປະກາດ
///
/// # Arguments
/// * `declaration_year` - Year of declaration
/// * `submission_date` - Date of submission (YYYY-MM-DD)
///
/// # Returns
/// * `Ok(())` if submitted on time
/// * `Err(AntiCorruptionLawError)` if late
pub fn validate_declaration_deadline(
    declaration_year: u16,
    submission_date: &str,
) -> AntiCorruptionLawResult<()> {
    // Parse submission date to check if it's within March of declaration year
    if submission_date.len() >= 7 {
        let parts: Vec<&str> = submission_date.split('-').collect();
        if parts.len() >= 2
            && let (Ok(year), Ok(month)) = (parts[0].parse::<u16>(), parts[1].parse::<u8>())
        {
            let deadline_year = declaration_year;
            let deadline_month = ANNUAL_DECLARATION_DEADLINE_MONTH;

            // Check if submission is late
            if year > deadline_year || (year == deadline_year && month > deadline_month) {
                let months_late = if year > deadline_year {
                    (year - deadline_year) as u32 * 12 + month as u32 - deadline_month as u32
                } else {
                    month as u32 - deadline_month as u32
                };
                let days_late = months_late * 30; // Approximate

                return Err(AntiCorruptionLawError::LateDeclaration {
                    days_late,
                    message_lao: format!(
                        "ການປະກາດຊັບສິນຊ້າ {} ວັນຫຼັງກຳນົດເດືອນ {}",
                        days_late, deadline_month
                    ),
                    message_en: format!(
                        "Declaration submitted {} days after deadline (March {})",
                        days_late, deadline_year
                    ),
                });
            }
        }
    }

    Ok(())
}

/// Check if an official is required to declare assets
/// ກວດວ່າພະນັກງານຕ້ອງປະກາດຊັບສິນຫຼືບໍ່
pub fn validate_declaration_required(grade: &PositionGrade) -> AntiCorruptionLawResult<bool> {
    Ok(grade.requires_asset_declaration())
}

// ============================================================================
// Investigation Validators - ການກວດສອບການສືບສວນ
// ============================================================================

/// Validate an investigation
/// ກວດສອບການສືບສວນ
pub fn validate_investigation(investigation: &Investigation) -> AntiCorruptionLawResult<()> {
    // Check required fields
    if investigation.investigation_id.is_empty() {
        return Err(AntiCorruptionLawError::InvalidInvestigation {
            article: 10,
            message_lao: "ຕ້ອງມີລະຫັດການສືບສວນ".to_string(),
            message_en: "Investigation ID is required".to_string(),
        });
    }

    if investigation.lead_investigator.is_empty() {
        return Err(AntiCorruptionLawError::InvalidInvestigation {
            article: 10,
            message_lao: "ຕ້ອງລະບຸຜູ້ສືບສວນຫຼັກ".to_string(),
            message_en: "Lead investigator must be specified".to_string(),
        });
    }

    if investigation.start_date.is_empty() {
        return Err(AntiCorruptionLawError::InvalidInvestigation {
            article: 12,
            message_lao: "ຕ້ອງລະບຸວັນທີເລີ່ມຕົ້ນ".to_string(),
            message_en: "Start date must be specified".to_string(),
        });
    }

    Ok(())
}

/// Validate investigation timeline
/// ກວດສອບກຳນົດເວລາການສືບສວນ
///
/// # Arguments
/// * `investigation_type` - Type of investigation
/// * `days_elapsed` - Number of days since start
///
/// # Returns
/// * `Ok(())` if within timeline
/// * `Err(AntiCorruptionLawError)` if timeline exceeded
pub fn validate_investigation_timeline(
    investigation_type: InvestigationType,
    days_elapsed: u32,
) -> AntiCorruptionLawResult<()> {
    let max_days = investigation_type.max_duration_days();

    if days_elapsed > max_days {
        return Err(AntiCorruptionLawError::InvestigationTimelineExceeded {
            days: days_elapsed,
            max_days,
            message_lao: format!(
                "ການສືບສວນ{} ເກີນກຳນົດເວລາ {} ວັນ",
                investigation_type.lao_name(),
                max_days
            ),
            message_en: format!(
                "{} exceeded {} day limit",
                if matches!(investigation_type, InvestigationType::Preliminary) {
                    "Preliminary investigation"
                } else {
                    "Full investigation"
                },
                max_days
            ),
        });
    }

    Ok(())
}

// ============================================================================
// SIA Validators - ການກວດສອບ ອກລ
// ============================================================================

/// Validate SIA jurisdiction
/// ກວດສອບຂອບເຂດອຳນາດ ອກລ
pub fn validate_sia_jurisdiction(
    office: &SIAOffice,
    offense_province: &str,
) -> AntiCorruptionLawResult<()> {
    match office.level {
        SIAOfficeLevel::Central => {
            // Central has jurisdiction everywhere
            Ok(())
        }
        SIAOfficeLevel::Provincial => {
            // Provincial only has jurisdiction in their province
            if let Some(province) = &office.province
                && province != offense_province
            {
                return Err(AntiCorruptionLawError::SiaJurisdictionError {
                    message_lao: format!(
                        "ຫ້ອງການ ອກລ ແຂວງ {} ບໍ່ມີອຳນາດໃນແຂວງ {}",
                        province, offense_province
                    ),
                    message_en: format!(
                        "Provincial SIA {} does not have jurisdiction in {}",
                        province, offense_province
                    ),
                });
            }
            Ok(())
        }
        SIAOfficeLevel::District => {
            // District has limited jurisdiction
            Err(AntiCorruptionLawError::SiaJurisdictionError {
                message_lao: "ຫ້ອງການ ອກລ ເມືອງ ມີອຳນາດຈຳກັດ".to_string(),
                message_en: "District SIA has limited jurisdiction".to_string(),
            })
        }
    }
}

/// Validate SIA powers for specific action
/// ກວດສອບອຳນາດ ອກລ ສຳລັບການດຳເນີນການສະເພາະ
pub fn validate_sia_powers(
    office_level: SIAOfficeLevel,
    power: SIAPower,
) -> AntiCorruptionLawResult<()> {
    // Central can exercise all powers
    if matches!(office_level, SIAOfficeLevel::Central) {
        return Ok(());
    }

    // Provincial can exercise most powers except arrest referral
    if matches!(office_level, SIAOfficeLevel::Provincial) {
        if matches!(power, SIAPower::ArrestReferral) {
            return Err(AntiCorruptionLawError::SiaPowerExceeded {
                power_type: power.lao_name().to_string(),
                message_lao: "ຫ້ອງການ ອກລ ແຂວງ ບໍ່ມີອຳນາດສົ່ງຈັບກຸມໂດຍກົງ".to_string(),
                message_en: "Provincial SIA cannot directly refer for arrest".to_string(),
            });
        }
        return Ok(());
    }

    // District has limited powers
    match power {
        SIAPower::Inspection | SIAPower::DocumentRequest => Ok(()),
        _ => Err(AntiCorruptionLawError::SiaPowerExceeded {
            power_type: power.lao_name().to_string(),
            message_lao: format!("ຫ້ອງການ ອກລ ເມືອງ ບໍ່ມີອຳນາດ{}", power.lao_name()),
            message_en: format!("District SIA does not have {} power", power.lao_name()),
        }),
    }
}

// ============================================================================
// Penalty Validators - ການກວດສອບການລົງໂທດ
// ============================================================================

/// Determine penalty range based on corruption offense
/// ກຳນົດຂອບເຂດການລົງໂທດຕາມການກະທຳຜິດສໍ້ລາດບັງຫຼວງ
pub fn determine_penalty_range(offense: &CorruptionOffense) -> PenaltyRange {
    let amount = offense.offense_type.amount_involved().unwrap_or(0);
    let severity = CorruptionSeverity::from_amount(amount);

    match severity {
        CorruptionSeverity::Minor => PenaltyRange::minor(),
        CorruptionSeverity::Medium => PenaltyRange::medium(),
        CorruptionSeverity::Serious => PenaltyRange::serious(),
        CorruptionSeverity::VerySerious => PenaltyRange::very_serious(),
    }
}

/// Validate a penalty against the offense
/// ກວດສອບການລົງໂທດຕໍ່ການກະທຳຜິດ
pub fn validate_penalty(
    penalty: &PenaltyType,
    offense: &CorruptionOffense,
) -> AntiCorruptionLawResult<()> {
    let range = determine_penalty_range(offense);

    match penalty {
        PenaltyType::Imprisonment {
            min_months,
            max_months,
        } => {
            if *min_months < range.min_imprisonment_months {
                return Err(AntiCorruptionLawError::PenaltyBelowMinimum {
                    months: *min_months,
                    min_months: range.min_imprisonment_months,
                    message_lao: "ໂທດຈຳຄຸກຕ່ຳກວ່າຂັ້ນຕ່ຳທີ່ກຳນົດ".to_string(),
                    message_en: "Imprisonment below minimum threshold".to_string(),
                });
            }

            if *max_months > range.max_imprisonment_months && !range.life_imprisonment_possible {
                return Err(AntiCorruptionLawError::PenaltyExceedsMaximum {
                    years: *max_months / 12,
                    max_years: range.max_imprisonment_months / 12,
                    message_lao: "ໂທດຈຳຄຸກເກີນສູງສຸດທີ່ກຳນົດ".to_string(),
                    message_en: "Imprisonment exceeds maximum threshold".to_string(),
                });
            }
        }
        PenaltyType::AssetForfeiture { .. } => {
            if !range.asset_forfeiture_applicable {
                return Err(AntiCorruptionLawError::DisproportionatePenalty {
                    message_lao: "ການຍຶດຊັບສິນບໍ່ນຳໃຊ້ກັບຄວາມຜິດຂະໜາດນ້ອຍ".to_string(),
                    message_en: "Asset forfeiture not applicable for minor offenses".to_string(),
                    severity: range.severity.lao_name().to_string(),
                    amount_lak: offense.offense_type.amount_involved().unwrap_or(0),
                });
            }
        }
        PenaltyType::Dismissal => {
            // Dismissal is always valid for corruption
        }
        _ => {}
    }

    Ok(())
}

// ============================================================================
// Whistleblower Validators - ການກວດສອບຜູ້ແຈ້ງຂ່າວ
// ============================================================================

/// Validate a whistleblower report
/// ກວດສອບການແຈ້ງຂ່າວ
pub fn validate_whistleblower_report(report: &WhistleblowerReport) -> AntiCorruptionLawResult<()> {
    // Check description is provided
    if report.description_lao.is_empty() || report.description_en.is_empty() {
        return Err(AntiCorruptionLawError::InsufficientReportDetails {
            missing_elements: vec!["description".to_string()],
            message_lao: "ຕ້ອງມີລາຍລະອຽດການແຈ້ງຂ່າວ".to_string(),
            message_en: "Report description is required".to_string(),
        });
    }

    // Check accused official is described
    if report.accused_official_description.is_empty() {
        return Err(AntiCorruptionLawError::InsufficientReportDetails {
            missing_elements: vec!["accused_official".to_string()],
            message_lao: "ຕ້ອງລະບຸຂໍ້ມູນພະນັກງານທີ່ຖືກກ່າວຫາ".to_string(),
            message_en: "Accused official information is required".to_string(),
        });
    }

    // Check submission date
    if report.submission_date.is_empty() {
        return Err(AntiCorruptionLawError::InvalidWhistleblowerReport {
            article: 85,
            message_lao: "ຕ້ອງລະບຸວັນທີຍື່ນ".to_string(),
            message_en: "Submission date is required".to_string(),
        });
    }

    // If not anonymous, check contact info
    if !report.anonymous && report.reporter_contact.is_none() {
        return Err(AntiCorruptionLawError::InsufficientReportDetails {
            missing_elements: vec!["reporter_contact".to_string()],
            message_lao: "ຕ້ອງມີຂໍ້ມູນຕິດຕໍ່ຜູ້ແຈ້ງຂ່າວ".to_string(),
            message_en: "Reporter contact information is required for non-anonymous reports"
                .to_string(),
        });
    }

    Ok(())
}

/// Validate whistleblower protection
/// ກວດສອບການປົກປ້ອງຜູ້ແຈ້ງຂ່າວ
pub fn validate_whistleblower_protection(
    protection: &WhistleblowerProtection,
) -> AntiCorruptionLawResult<()> {
    if protection.protection_types.is_empty() {
        return Err(AntiCorruptionLawError::InvalidWhistleblowerReport {
            article: 88,
            message_lao: "ຕ້ອງລະບຸປະເພດການປົກປ້ອງ".to_string(),
            message_en: "Protection type must be specified".to_string(),
        });
    }

    if protection.start_date.is_empty() {
        return Err(AntiCorruptionLawError::InvalidWhistleblowerReport {
            article: 88,
            message_lao: "ຕ້ອງລະບຸວັນທີເລີ່ມຕົ້ນການປົກປ້ອງ".to_string(),
            message_en: "Protection start date is required".to_string(),
        });
    }

    Ok(())
}

/// Validate whistleblower reward calculation
/// ກວດສອບການຄຳນວນລາງວັນຜູ້ແຈ້ງຂ່າວ
pub fn validate_whistleblower_reward(
    recovered_amount_lak: u64,
    reward_amount_lak: u64,
) -> AntiCorruptionLawResult<()> {
    if recovered_amount_lak == 0 {
        return Err(AntiCorruptionLawError::InvalidRewardCalculation {
            calculated_amount_lak: reward_amount_lak,
            message_lao: "ບໍ່ມີເງິນທີ່ຂໍຄືນໄດ້".to_string(),
            message_en: "No funds recovered".to_string(),
        });
    }

    let min_reward =
        (recovered_amount_lak as f64 * WHISTLEBLOWER_REWARD_MIN_PERCENT as f64 / 100.0) as u64;
    let max_reward =
        (recovered_amount_lak as f64 * WHISTLEBLOWER_REWARD_MAX_PERCENT as f64 / 100.0) as u64;

    if reward_amount_lak < min_reward {
        return Err(AntiCorruptionLawError::InvalidRewardCalculation {
            calculated_amount_lak: reward_amount_lak,
            message_lao: format!(
                "ລາງວັນ {} ກີບ ຕ່ຳກວ່າຂັ້ນຕ່ຳ {}% ({} ກີບ)",
                reward_amount_lak, WHISTLEBLOWER_REWARD_MIN_PERCENT, min_reward
            ),
            message_en: format!(
                "Reward {} LAK is below minimum {}% ({} LAK)",
                reward_amount_lak, WHISTLEBLOWER_REWARD_MIN_PERCENT, min_reward
            ),
        });
    }

    if reward_amount_lak > max_reward {
        return Err(AntiCorruptionLawError::InvalidRewardCalculation {
            calculated_amount_lak: reward_amount_lak,
            message_lao: format!(
                "ລາງວັນ {} ກີບ ເກີນສູງສຸດ {}% ({} ກີບ)",
                reward_amount_lak, WHISTLEBLOWER_REWARD_MAX_PERCENT, max_reward
            ),
            message_en: format!(
                "Reward {} LAK exceeds maximum {}% ({} LAK)",
                reward_amount_lak, WHISTLEBLOWER_REWARD_MAX_PERCENT, max_reward
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Prevention Measure Validators - ການກວດສອບມາດຕະການປ້ອງກັນ
// ============================================================================

/// Validate a prevention measure
/// ກວດສອບມາດຕະການປ້ອງກັນ
pub fn validate_prevention_measure(measure: &PreventionMeasure) -> AntiCorruptionLawResult<()> {
    if measure.description_lao.is_empty() || measure.description_en.is_empty() {
        return Err(AntiCorruptionLawError::ValidationError {
            message_lao: "ຕ້ອງມີລາຍລະອຽດມາດຕະການ".to_string(),
            message_en: "Measure description is required".to_string(),
        });
    }

    if measure.applicable_to.is_empty() {
        return Err(AntiCorruptionLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸກຸ່ມເປົ້າໝາຍ".to_string(),
            message_en: "Target group must be specified".to_string(),
        });
    }

    Ok(())
}

/// Validate gift against limits
/// ກວດສອບຂອງຂວັນຕໍ່ກຳນົດ
pub fn validate_gift(gift: &Gift) -> AntiCorruptionLawResult<()> {
    validate_gift_limit(gift.value_lak)
}

/// Validate gift limit
/// ກວດສອບກຳນົດຂອງຂວັນ
pub fn validate_gift_limit(value_lak: u64) -> AntiCorruptionLawResult<()> {
    if value_lak > GIFT_LIMIT_OFFICIAL_FUNCTION_LAK {
        return Err(AntiCorruptionLawError::GiftLimitExceeded {
            amount_lak: value_lak,
            limit_lak: GIFT_LIMIT_OFFICIAL_FUNCTION_LAK,
            message_lao: format!(
                "ມູນຄ່າຂອງຂວັນ {} ກີບ ເກີນກຳນົດ {} ກີບ",
                value_lak, GIFT_LIMIT_OFFICIAL_FUNCTION_LAK
            ),
            message_en: format!(
                "Gift value {} LAK exceeds limit of {} LAK",
                value_lak, GIFT_LIMIT_OFFICIAL_FUNCTION_LAK
            ),
        });
    }

    Ok(())
}

/// Validate cooling-off period
/// ກວດສອບໄລຍະຫ່າງ
pub fn validate_cooling_off_period(years_since_leaving: u32) -> AntiCorruptionLawResult<()> {
    if years_since_leaving < COOLING_OFF_PERIOD_YEARS as u32 {
        return Err(AntiCorruptionLawError::CoolingOffPeriodViolation {
            years_since_leaving,
            required_years: COOLING_OFF_PERIOD_YEARS as u32,
            message_lao: format!(
                "ໄລຍະຫ່າງ {} ປີ ບໍ່ພຽງພໍ, ຕ້ອງມີ {} ປີ",
                years_since_leaving, COOLING_OFF_PERIOD_YEARS
            ),
            message_en: format!(
                "Cooling-off period {} years is insufficient, {} years required",
                years_since_leaving, COOLING_OFF_PERIOD_YEARS
            ),
        });
    }

    Ok(())
}

/// Validate code of conduct compliance
/// ກວດສອບການປະຕິບັດຕາມລະບຽບພຶດຕິກຳ
pub fn validate_code_of_conduct_compliance(
    violation: &CodeOfConductViolation,
) -> AntiCorruptionLawResult<()> {
    if violation.description_lao.is_empty() || violation.description_en.is_empty() {
        return Err(AntiCorruptionLawError::CodeOfConductViolation {
            violation_type: violation.violation_type.lao_name().to_string(),
            message_lao: "ຕ້ອງມີລາຍລະອຽດການລະເມີດ".to_string(),
            message_en: "Violation description is required".to_string(),
        });
    }

    if violation.date_of_violation.is_empty() {
        return Err(AntiCorruptionLawError::CodeOfConductViolation {
            violation_type: violation.violation_type.lao_name().to_string(),
            message_lao: "ຕ້ອງລະບຸວັນທີລະເມີດ".to_string(),
            message_en: "Violation date is required".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// International Cooperation Validators - ການກວດສອບການຮ່ວມມືສາກົນ
// ============================================================================

/// Validate international cooperation request
/// ກວດສອບຄຳຮ້ອງການຮ່ວມມືສາກົນ
pub fn validate_international_cooperation(
    cooperation: &InternationalCooperation,
) -> AntiCorruptionLawResult<()> {
    if cooperation.foreign_country.is_empty() {
        return Err(AntiCorruptionLawError::InvalidInternationalCooperation {
            article: 120,
            message_lao: "ຕ້ອງລະບຸປະເທດ".to_string(),
            message_en: "Country must be specified".to_string(),
        });
    }

    if cooperation.description_lao.is_empty() || cooperation.description_en.is_empty() {
        return Err(AntiCorruptionLawError::InvalidInternationalCooperation {
            article: 120,
            message_lao: "ຕ້ອງມີລາຍລະອຽດຄຳຮ້ອງ".to_string(),
            message_en: "Request description is required".to_string(),
        });
    }

    if cooperation.request_date.is_empty() {
        return Err(AntiCorruptionLawError::InvalidInternationalCooperation {
            article: 120,
            message_lao: "ຕ້ອງລະບຸວັນທີຮ້ອງຂໍ".to_string(),
            message_en: "Request date is required".to_string(),
        });
    }

    // For asset recovery, amount must be specified
    if matches!(
        cooperation.cooperation_type,
        InternationalCooperationType::AssetRecovery
    ) && cooperation.amount_involved_lak.is_none()
    {
        return Err(AntiCorruptionLawError::AssetRecoveryError {
            foreign_jurisdiction: cooperation.foreign_country.clone(),
            amount_lak: 0,
            message_lao: "ຕ້ອງລະບຸຈຳນວນເງິນສຳລັບການຂໍຊັບຄືນ".to_string(),
            message_en: "Amount must be specified for asset recovery".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Official Category Validators - ການກວດສອບປະເພດພະນັກງານ
// ============================================================================

/// Validate official category is covered by the law
/// ກວດສອບວ່າປະເພດພະນັກງານຢູ່ໃນຂອບເຂດກົດໝາຍ
pub fn validate_official_category(category: &OfficialCategory) -> AntiCorruptionLawResult<()> {
    if category.is_covered() {
        Ok(())
    } else {
        Err(AntiCorruptionLawError::OfficialNotCovered {
            official_type: category.lao_name().to_string(),
            message_lao: "ປະເພດພະນັກງານນີ້ບໍ່ຢູ່ໃນຂອບເຂດກົດໝາຍຕ້ານການສໍ້ລາດບັງຫຼວງ".to_string(),
            message_en: "This official category is not covered by the Anti-Corruption Law"
                .to_string(),
        })
    }
}

// ============================================================================
// Tests - ການທົດສອບ
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_corruption_offense_valid() {
        let offense = CorruptionOffense {
            offense_type: CorruptionOffenseType::Bribery {
                direction: BriberyDirection::Receiving,
                amount_lak: 10_000_000,
            },
            perpetrator: OfficialType::GovernmentOfficial {
                position_grade: PositionGrade::Grade5,
                ministry: Some("Ministry of Finance".to_string()),
            },
            date_of_offense: "2025-06-15".to_string(),
            location_province: "Vientiane Capital".to_string(),
            description_lao: "ການຮັບສິນບົນໃນການອອກໃບອະນຸຍາດ".to_string(),
            description_en: "Accepting bribe for issuing permits".to_string(),
            evidence_collected: true,
            investigation_status: InvestigationStatus::UnderInvestigation,
        };

        assert!(validate_corruption_offense(&offense).is_ok());
    }

    #[test]
    fn test_validate_corruption_offense_missing_description() {
        let offense = CorruptionOffense {
            offense_type: CorruptionOffenseType::Bribery {
                direction: BriberyDirection::Receiving,
                amount_lak: 10_000_000,
            },
            perpetrator: OfficialType::GovernmentOfficial {
                position_grade: PositionGrade::Grade5,
                ministry: None,
            },
            date_of_offense: "2025-06-15".to_string(),
            location_province: "Vientiane Capital".to_string(),
            description_lao: "".to_string(),
            description_en: "".to_string(),
            evidence_collected: false,
            investigation_status: InvestigationStatus::Preliminary,
        };

        assert!(validate_corruption_offense(&offense).is_err());
    }

    #[test]
    fn test_validate_asset_declaration_valid() {
        let declaration = AssetDeclarationBuilder::new()
            .official_id("GOV-2025-001")
            .official_name_lao("ທ່ານ ສົມໃຈ")
            .official_name_en("Mr. Somjai")
            .position_grade(PositionGrade::Grade3)
            .ministry("Ministry of Justice")
            .declaration_year(2025)
            .bank_balance_lak(50_000_000)
            .add_income_source(IncomeSource {
                source_type: IncomeSourceType::Salary,
                description_lao: "ເງິນເດືອນລັດຖະກອນ".to_string(),
                description_en: "Government salary".to_string(),
                annual_amount_lak: 48_000_000,
            })
            .total_assets_lak(100_000_000)
            .submission_date("2025-03-15")
            .build();

        assert!(validate_asset_declaration(&declaration).is_ok());
    }

    #[test]
    fn test_validate_declaration_deadline_on_time() {
        assert!(validate_declaration_deadline(2025, "2025-03-15").is_ok());
    }

    #[test]
    fn test_validate_declaration_deadline_late() {
        assert!(validate_declaration_deadline(2025, "2025-05-15").is_err());
    }

    #[test]
    fn test_validate_gift_limit_within() {
        assert!(validate_gift_limit(400_000).is_ok());
    }

    #[test]
    fn test_validate_gift_limit_exceeded() {
        assert!(validate_gift_limit(600_000).is_err());
    }

    #[test]
    fn test_validate_cooling_off_period_sufficient() {
        assert!(validate_cooling_off_period(3).is_ok());
    }

    #[test]
    fn test_validate_cooling_off_period_insufficient() {
        assert!(validate_cooling_off_period(1).is_err());
    }

    #[test]
    fn test_determine_penalty_range_minor() {
        let offense = CorruptionOffense {
            offense_type: CorruptionOffenseType::Bribery {
                direction: BriberyDirection::Receiving,
                amount_lak: 3_000_000,
            },
            perpetrator: OfficialType::GovernmentOfficial {
                position_grade: PositionGrade::Grade6,
                ministry: None,
            },
            date_of_offense: "2025-06-15".to_string(),
            location_province: "Savannakhet".to_string(),
            description_lao: "ການຮັບສິນບົນຂະໜາດນ້ອຍ".to_string(),
            description_en: "Minor bribery".to_string(),
            evidence_collected: true,
            investigation_status: InvestigationStatus::Completed,
        };

        let range = determine_penalty_range(&offense);
        assert_eq!(range.severity, CorruptionSeverity::Minor);
        assert!(!range.dismissal_mandatory);
    }

    #[test]
    fn test_determine_penalty_range_very_serious() {
        let offense = CorruptionOffense {
            offense_type: CorruptionOffenseType::Embezzlement {
                amount_lak: 600_000_000,
                fund_source: FundSource::StateBudget,
            },
            perpetrator: OfficialType::GovernmentOfficial {
                position_grade: PositionGrade::Grade2,
                ministry: Some("Ministry of Finance".to_string()),
            },
            date_of_offense: "2025-06-15".to_string(),
            location_province: "Vientiane Capital".to_string(),
            description_lao: "ການສໍ້ໂກງງົບປະມານລັດຂະໜາດໃຫຍ່".to_string(),
            description_en: "Large-scale embezzlement of state budget".to_string(),
            evidence_collected: true,
            investigation_status: InvestigationStatus::Completed,
        };

        let range = determine_penalty_range(&offense);
        assert_eq!(range.severity, CorruptionSeverity::VerySerious);
        assert!(range.life_imprisonment_possible);
        assert!(range.dismissal_mandatory);
    }

    #[test]
    fn test_validate_whistleblower_report_valid() {
        let report = WhistleblowerReportBuilder::new()
            .anonymous(true)
            .allegation_type(CorruptionOffenseType::Bribery {
                direction: BriberyDirection::Receiving,
                amount_lak: 20_000_000,
            })
            .description_lao("ພະນັກງານຮັບສິນບົນໃນການອອກໃບອະນຸຍາດ".to_string())
            .description_en("Official accepting bribe for permit issuance".to_string())
            .accused_official_description("Director of Licensing Department".to_string())
            .submission_date("2025-07-01")
            .build();

        assert!(validate_whistleblower_report(&report).is_ok());
    }

    #[test]
    fn test_validate_whistleblower_reward_valid() {
        // 10% of 100 million = 10 million (within 5-15% range)
        assert!(validate_whistleblower_reward(100_000_000, 10_000_000).is_ok());
    }

    #[test]
    fn test_validate_whistleblower_reward_too_low() {
        // 3% of 100 million = 3 million (below 5% minimum)
        assert!(validate_whistleblower_reward(100_000_000, 3_000_000).is_err());
    }

    #[test]
    fn test_validate_whistleblower_reward_too_high() {
        // 20% of 100 million = 20 million (above 15% maximum)
        assert!(validate_whistleblower_reward(100_000_000, 20_000_000).is_err());
    }

    #[test]
    fn test_validate_investigation_timeline_within() {
        assert!(validate_investigation_timeline(InvestigationType::Preliminary, 60).is_ok());
    }

    #[test]
    fn test_validate_investigation_timeline_exceeded() {
        assert!(validate_investigation_timeline(InvestigationType::Preliminary, 100).is_err());
    }

    #[test]
    fn test_validate_sia_jurisdiction_central() {
        let office = SIAOffice {
            level: SIAOfficeLevel::Central,
            name_lao: "ຫ້ອງການສູນກາງ ອກລ".to_string(),
            name_en: "Central SIA Office".to_string(),
            province: None,
            district: None,
        };

        assert!(validate_sia_jurisdiction(&office, "Savannakhet").is_ok());
    }

    #[test]
    fn test_validate_sia_jurisdiction_provincial_mismatch() {
        let office = SIAOffice {
            level: SIAOfficeLevel::Provincial,
            name_lao: "ຫ້ອງການ ອກລ ແຂວງ ວຽງຈັນ".to_string(),
            name_en: "Vientiane Provincial SIA Office".to_string(),
            province: Some("Vientiane Capital".to_string()),
            district: None,
        };

        assert!(validate_sia_jurisdiction(&office, "Savannakhet").is_err());
    }

    #[test]
    fn test_validate_official_category() {
        assert!(validate_official_category(&OfficialCategory::Government).is_ok());
        assert!(validate_official_category(&OfficialCategory::Judge).is_ok());
        assert!(validate_official_category(&OfficialCategory::Prosecutor).is_ok());
    }

    #[test]
    fn test_position_grade_requires_declaration() {
        assert!(validate_declaration_required(&PositionGrade::Grade1).unwrap_or(false));
        assert!(validate_declaration_required(&PositionGrade::Grade5).unwrap_or(false));
        assert!(!validate_declaration_required(&PositionGrade::Grade6).unwrap_or(true));
    }
}
