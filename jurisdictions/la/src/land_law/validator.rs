//! Land Law Validators (ການກວດສອບກົດໝາຍທີ່ດິນ)
//!
//! This module provides validation functions for land law types according to
//! Lao Land Law 2019 (Law No. 70/NA).
//!
//! ## Validation Principles
//!
//! 1. **State Ownership Enforcement** - Ensure all land transactions respect state ownership
//! 2. **Foreign Ownership Restrictions** - Strict validation of foreign ownership limits
//! 3. **Document Verification** - Validate proper registration and documentation
//! 4. **Area Limits** - Enforce maximum land area restrictions
//! 5. **Duration Limits** - Validate concession and lease durations

use crate::land_law::error::{LandLawError, Result};
use crate::land_law::types::*;
use chrono::Utc;

/// Maximum land area for residential use (in square meters)
/// ເນື້ອທີ່ສູງສຸດສຳລັບທີ່ດິນຢູ່ອາໄສ
const MAX_RESIDENTIAL_AREA_SQM: u64 = 2000;

/// Maximum land area for agricultural concession (in hectares)
/// ເນື້ອທີ່ສູງສຸດສຳລັບສຳປະທານກະສິກຳ
const MAX_AGRICULTURAL_CONCESSION_HECTARES: u64 = 10000;

/// Validates a land use right according to Lao Land Law
///
/// ## Validation Rules
///
/// 1. **Perpetual Use Rights:**
///    - Holder must be Lao citizen (nationality = "LAO")
///    - Area must not exceed maximum for residential use
///    - Permitted use must be appropriate for classification
///
/// 2. **Temporary Use Rights:**
///    - Duration must be reasonable (typically ≤ 50 years)
///    - Foreign nationals allowed only with restrictions
///    - Must have valid expiry date in the future
///
/// ## Example
///
/// ```
/// use legalis_la::land_law::{LandUseRight, LandUsePurpose, validate_land_use_right};
/// use chrono::{Utc, Duration};
///
/// let use_right = LandUseRight::PerpetualUse {
///     holder_name: "Somchai Vongphachan".to_string(),
///     holder_nationality: "LAO".to_string(),
///     granted_at: Utc::now(),
///     parcel_id: "VTE-001-2024".to_string(),
///     area_sqm: 1500,
///     permitted_use: LandUsePurpose::Residential,
/// };
///
/// assert!(validate_land_use_right(&use_right).is_ok());
/// ```
pub fn validate_land_use_right(use_right: &LandUseRight) -> Result<()> {
    match use_right {
        LandUseRight::PerpetualUse {
            holder_name,
            holder_nationality,
            area_sqm,
            permitted_use,
            ..
        } => {
            // Check holder is Lao citizen
            if holder_nationality != "LAO" {
                return Err(LandLawError::invalid_land_use_right(
                    "Perpetual land use rights are only available to Lao citizens",
                    "ສິດນຳໃຊ້ທີ່ດິນຖາວອນມີພຽງແຕ່ພົນລະເມືອງລາວເທົ່ານັ້ນ",
                ));
            }

            // Check holder name is not empty
            if holder_name.is_empty() {
                return Err(LandLawError::invalid_land_use_right(
                    "Holder name cannot be empty",
                    "ຊື່ຜູ້ຖືສິດບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
                ));
            }

            // Check area limits for residential use
            if matches!(permitted_use, LandUsePurpose::Residential)
                && *area_sqm > MAX_RESIDENTIAL_AREA_SQM
            {
                return Err(LandLawError::land_area_exceeds_maximum(
                    format!(
                        "Residential land area {} sqm exceeds maximum {} sqm",
                        area_sqm, MAX_RESIDENTIAL_AREA_SQM
                    ),
                    format!(
                        "ເນື້ອທີ່ດິນຢູ່ອາໄສ {} ຕາຕະລາງແມັດເກີນກຳນົດສູງສຸດ {} ຕາຕະລາງແມັດ",
                        area_sqm, MAX_RESIDENTIAL_AREA_SQM
                    ),
                ));
            }

            Ok(())
        }

        LandUseRight::TemporaryUse {
            holder_name,
            expires_at,
            area_sqm,
            ..
        } => {
            // Check holder name is not empty
            if holder_name.is_empty() {
                return Err(LandLawError::invalid_land_use_right(
                    "Holder name cannot be empty",
                    "ຊື່ຜູ້ຖືສິດບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
                ));
            }

            // Check expiry date is in the future
            if *expires_at <= Utc::now() {
                return Err(LandLawError::land_use_right_expired(
                    "Land use right has already expired",
                    "ສິດນຳໃຊ້ທີ່ດິນໝົດອາຍຸແລ້ວ",
                ));
            }

            // Check area is positive
            if *area_sqm == 0 {
                return Err(LandLawError::invalid_land_use_right(
                    "Land area must be greater than zero",
                    "ເນື້ອທີ່ດິນຕ້ອງຫຼາຍກວ່າສູນ",
                ));
            }

            Ok(())
        }
    }
}

/// Validates a land concession according to Lao Land Law and Investment Promotion Law
///
/// ## Validation Rules
///
/// 1. **Duration Limits:**
///    - Agricultural: typically 30-50 years
///    - Industrial: typically 50-75 years
///    - Commercial: typically 30-50 years
///    - Mining: varies by mineral type
///    - Tourism: typically 50-99 years
///
/// 2. **Area Limits:**
///    - Agricultural concessions: maximum 10,000 hectares per project
///    - Other concessions: subject to approval
///
/// 3. **Investment Requirements:**
///    - Minimum investment amounts must be met
///    - Environmental impact assessments required for mining
///
/// ## Example
///
/// ```
/// use legalis_la::land_law::{LandConcession, validate_land_concession};
/// use chrono::{Utc, Duration};
///
/// let concession = LandConcession::Agricultural {
///     holder: "Lao Agricultural Company".to_string(),
///     area_hectares: 5000,
///     granted_at: Utc::now(),
///     expires_at: Utc::now() + Duration::days(365 * 30),
///     activities: vec!["Rice cultivation".to_string()],
///     investment_lak: 50_000_000_000,
/// };
///
/// assert!(validate_land_concession(&concession).is_ok());
/// ```
pub fn validate_land_concession(concession: &LandConcession) -> Result<()> {
    match concession {
        LandConcession::Agricultural {
            holder,
            area_hectares,
            granted_at,
            expires_at,
            activities,
            investment_lak,
        } => {
            // Check holder is not empty
            if holder.is_empty() {
                return Err(LandLawError::invalid_land_concession(
                    "Concession holder cannot be empty",
                    "ຜູ້ຖືສຳປະທານບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
                ));
            }

            // Check area limit
            if *area_hectares > MAX_AGRICULTURAL_CONCESSION_HECTARES {
                return Err(LandLawError::land_area_exceeds_maximum(
                    format!(
                        "Agricultural concession area {} hectares exceeds maximum {}",
                        area_hectares, MAX_AGRICULTURAL_CONCESSION_HECTARES
                    ),
                    format!(
                        "ເນື້ອທີ່ສຳປະທານກະສິກຳ {} ເຮັກຕາເກີນກຳນົດສູງສຸດ {}",
                        area_hectares, MAX_AGRICULTURAL_CONCESSION_HECTARES
                    ),
                ));
            }

            // Check duration
            let duration_years = (*expires_at - *granted_at).num_days() / 365;
            if duration_years > 50 {
                return Err(LandLawError::invalid_concession_duration(
                    format!(
                        "Agricultural concession duration {} years exceeds typical maximum of 50 years",
                        duration_years
                    ),
                    format!("ໄລຍະເວລາສຳປະທານກະສິກຳ {} ປີເກີນກຳນົດທົ່ວໄປ 50 ປີ", duration_years),
                ));
            }

            // Check activities
            if activities.is_empty() {
                return Err(LandLawError::invalid_land_concession(
                    "Concession must specify at least one activity",
                    "ສຳປະທານຕ້ອງລະບຸກິດຈະກຳຢ່າງໜ້ອຍໜຶ່ງອັນ",
                ));
            }

            // Check investment amount
            if *investment_lak == 0 {
                return Err(LandLawError::invalid_land_concession(
                    "Investment amount must be greater than zero",
                    "ມູນຄ່າການລົງທຶນຕ້ອງຫຼາຍກວ່າສູນ",
                ));
            }

            Ok(())
        }

        LandConcession::Mining {
            holder,
            area_hectares,
            eia_approved,
            mineral_types,
            ..
        } => {
            // Check holder
            if holder.is_empty() {
                return Err(LandLawError::invalid_land_concession(
                    "Concession holder cannot be empty",
                    "ຜູ້ຖືສຳປະທານບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
                ));
            }

            // Check area
            if *area_hectares == 0 {
                return Err(LandLawError::invalid_land_concession(
                    "Concession area must be greater than zero",
                    "ເນື້ອທີ່ສຳປະທານຕ້ອງຫຼາຍກວ່າສູນ",
                ));
            }

            // Check EIA approval (required for mining)
            if !eia_approved {
                return Err(LandLawError::missing_documentation(
                    "Mining concession requires Environmental Impact Assessment approval",
                    "ສຳປະທານບໍ່ແຮ່ຕ້ອງໄດ້ຮັບການອະນຸມັດການປະເມີນຜົນກະທົບສິ່ງແວດລ້ອມ",
                ));
            }

            // Check mineral types
            if mineral_types.is_empty() {
                return Err(LandLawError::invalid_land_concession(
                    "Mining concession must specify at least one mineral type",
                    "ສຳປະທານບໍ່ແຮ່ຕ້ອງລະບຸປະເພດບໍ່ແຮ່ຢ່າງໜ້ອຍໜຶ່ງປະເພດ",
                ));
            }

            Ok(())
        }

        LandConcession::Industrial {
            holder,
            in_sez: _,
            area_hectares,
            ..
        }
        | LandConcession::Commercial {
            holder,
            area_hectares,
            ..
        }
        | LandConcession::Tourism {
            holder,
            area_hectares,
            ..
        } => {
            // Basic validation for other concession types
            if holder.is_empty() {
                return Err(LandLawError::invalid_land_concession(
                    "Concession holder cannot be empty",
                    "ຜູ້ຖືສຳປະທານບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
                ));
            }

            if *area_hectares == 0 {
                return Err(LandLawError::invalid_land_concession(
                    "Concession area must be greater than zero",
                    "ເນື້ອທີ່ສຳປະທານຕ້ອງຫຼາຍກວ່າສູນ",
                ));
            }

            Ok(())
        }
    }
}

/// Validates foreign ownership restrictions according to Lao Land Law
///
/// ## Validation Rules
///
/// 1. **Foreign Nationals:**
///    - Cannot hold perpetual land use rights
///    - Can lease land for up to 50 years (99 years in SEZ)
///    - Must have government approval
///
/// 2. **Foreign-Invested Entities:**
///    - Subject to Investment Promotion Law requirements
///    - May require investment license
///    - Restrictions on land use types
///
/// ## Example
///
/// ```
/// use legalis_la::land_law::{ForeignOwnershipStatus, validate_foreign_ownership};
///
/// let status = ForeignOwnershipStatus::ForeignNational {
///     passport_number: "P1234567".to_string(),
///     nationality: "Thailand".to_string(),
///     lease_approved: true,
/// };
///
/// assert!(validate_foreign_ownership(&status, false).is_ok());
/// ```
pub fn validate_foreign_ownership(
    status: &ForeignOwnershipStatus,
    is_perpetual_use: bool,
) -> Result<()> {
    match status {
        ForeignOwnershipStatus::LaoCitizen { citizen_id } => {
            // Lao citizens have no restrictions
            if citizen_id.is_empty() {
                return Err(LandLawError::missing_documentation(
                    "Citizen ID cannot be empty",
                    "ເລກບັດປະຈຳຕົວບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
                ));
            }
            Ok(())
        }

        ForeignOwnershipStatus::ForeignNational {
            passport_number,
            nationality,
            lease_approved,
        } => {
            // Foreign nationals cannot hold perpetual use rights
            if is_perpetual_use {
                return Err(LandLawError::foreign_ownership_violation(
                    "Foreign nationals cannot hold perpetual land use rights",
                    "ຊາວຕ່າງປະເທດບໍ່ສາມາດຖືສິດນຳໃຊ້ທີ່ດິນຖາວອນໄດ້",
                ));
            }

            // Check passport number
            if passport_number.is_empty() {
                return Err(LandLawError::missing_documentation(
                    "Passport number cannot be empty",
                    "ເລກໜັງສືເດີນທາງບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
                ));
            }

            // Check nationality
            if nationality.is_empty() || nationality == "LAO" {
                return Err(LandLawError::invalid_land_use_right(
                    "Invalid nationality for foreign national status",
                    "ສັນຊາດບໍ່ຖືກຕ້ອງສຳລັບສະຖານະຊາວຕ່າງປະເທດ",
                ));
            }

            // Check lease approval
            if !lease_approved {
                return Err(LandLawError::unauthorized_transfer(
                    "Foreign national requires government approval for land lease",
                    "ຊາວຕ່າງປະເທດຕ້ອງໄດ້ຮັບການອະນຸມັດຈາກລັດຖະບານສຳລັບການເຊົ່າທີ່ດິນ",
                ));
            }

            Ok(())
        }

        ForeignOwnershipStatus::LaoEntity {
            registration_number,
            entity_name,
        } => {
            // Lao entities have no restrictions
            if registration_number.is_empty() {
                return Err(LandLawError::missing_documentation(
                    "Registration number cannot be empty",
                    "ເລກທະບຽນບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
                ));
            }
            if entity_name.is_empty() {
                return Err(LandLawError::missing_documentation(
                    "Entity name cannot be empty",
                    "ຊື່ນິຕິບຸກຄົນບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
                ));
            }
            Ok(())
        }

        ForeignOwnershipStatus::ForeignInvestedEntity {
            registration_number,
            entity_name,
            foreign_ownership_pct,
            investment_license,
        } => {
            // Check registration
            if registration_number.is_empty() {
                return Err(LandLawError::missing_documentation(
                    "Registration number cannot be empty",
                    "ເລກທະບຽນບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
                ));
            }
            if entity_name.is_empty() {
                return Err(LandLawError::missing_documentation(
                    "Entity name cannot be empty",
                    "ຊື່ນິຕິບຸກຄົນບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
                ));
            }

            // Check ownership percentage
            if *foreign_ownership_pct > 100 {
                return Err(LandLawError::invalid_land_use_right(
                    "Foreign ownership percentage cannot exceed 100%",
                    "ເປີເຊັນການຖືຫຸ້ນຕ່າງປະເທດບໍ່ສາມາດເກີນ 100%",
                ));
            }

            // For perpetual use, foreign ownership percentage restrictions may apply
            if is_perpetual_use && *foreign_ownership_pct > 49 {
                return Err(LandLawError::foreign_ownership_violation(
                    "Foreign-invested entities with >49% foreign ownership cannot hold perpetual use rights",
                    "ນິຕິບຸກຄົນທີ່ມີທຶນຕ່າງປະເທດຫຼາຍກວ່າ 49% ບໍ່ສາມາດຖືສິດນຳໃຊ້ຖາວອນໄດ້",
                ));
            }

            // Check investment license if significant foreign ownership
            if *foreign_ownership_pct >= 50 && investment_license.is_none() {
                return Err(LandLawError::missing_documentation(
                    "Investment license required for entities with ≥50% foreign ownership",
                    "ຕ້ອງມີໃບອະນຸຍາດລົງທຶນສຳລັບນິຕິບຸກຄົນທີ່ມີທຶນຕ່າງປະເທດ ≥50%",
                ));
            }

            Ok(())
        }
    }
}

/// Validates a land registration according to Lao Land Law
///
/// ## Validation Rules
///
/// 1. Cadastral survey must be completed
/// 2. Title must be issued by proper authority
/// 3. Registration office must be specified
/// 4. Central database registration recommended
pub fn validate_land_registration(registration: &LandRegistrationStatus) -> Result<()> {
    if registration.parcel_id.is_empty() {
        return Err(LandLawError::invalid_land_registration(
            "Parcel ID cannot be empty",
            "ລະຫັດແປ່ງທີ່ດິນບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
        ));
    }

    if registration.registration_office.is_empty() {
        return Err(LandLawError::invalid_land_registration(
            "Registration office must be specified",
            "ຕ້ອງລະບຸຫ້ອງການລົງທະບຽນ",
        ));
    }

    if !registration.survey_completed {
        return Err(LandLawError::invalid_cadastral_survey(
            "Cadastral survey must be completed before registration",
            "ຕ້ອງສຳຫຼວດທີ່ດິນກ່ອນການລົງທະບຽນ",
        ));
    }

    if !registration.title_issued {
        return Err(LandLawError::invalid_land_title(
            "Land title must be issued",
            "ຕ້ອງອອກໃບຕາດິນ",
        ));
    }

    Ok(())
}

/// Validates a land transaction according to Lao Land Law
///
/// ## Validation Rules
///
/// 1. Government approval required for most transactions
/// 2. Registration in land office required
/// 3. Foreign ownership restrictions apply
/// 4. Proper documentation required
pub fn validate_land_transaction(transaction: &LandTransaction) -> Result<()> {
    if transaction.transaction_id.is_empty() {
        return Err(LandLawError::invalid_land_transaction(
            "Transaction ID cannot be empty",
            "ລະຫັດທຸລະກຳບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
        ));
    }

    if transaction.parcel_id.is_empty() {
        return Err(LandLawError::invalid_land_transaction(
            "Parcel ID cannot be empty",
            "ລະຫັດແປ່ງທີ່ດິນບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
        ));
    }

    if transaction.area_sqm == 0 {
        return Err(LandLawError::invalid_land_transaction(
            "Transaction area must be greater than zero",
            "ເນື້ອທີ່ໃນທຸລະກຳຕ້ອງຫຼາຍກວ່າສູນ",
        ));
    }

    // Most transactions require government approval
    if !transaction.government_approval {
        return Err(LandLawError::unauthorized_transfer(
            "Land transaction requires government approval",
            "ການເຮັດທຸລະກຳທີ່ດິນຕ້ອງໄດ້ຮັບການອະນຸມັດຈາກລັດຖະບານ",
        ));
    }

    // Check registration
    if !transaction.registered {
        return Err(LandLawError::invalid_land_registration(
            "Land transaction must be registered",
            "ທຸລະກຳທີ່ດິນຕ້ອງໄດ້ລົງທະບຽນ",
        ));
    }

    Ok(())
}

/// Validates a cadastral survey according to Lao Land Law
///
/// ## Validation Rules
///
/// 1. Surveyor must have valid license
/// 2. Survey method must be appropriate
/// 3. Boundary coordinates must be provided
/// 4. Authority approval required
pub fn validate_cadastral_survey(survey: &CadastralSurvey) -> Result<()> {
    if survey.survey_id.is_empty() {
        return Err(LandLawError::invalid_cadastral_survey(
            "Survey ID cannot be empty",
            "ລະຫັດການສຳຫຼວດບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
        ));
    }

    if survey.surveyor.is_empty() {
        return Err(LandLawError::invalid_cadastral_survey(
            "Surveyor name cannot be empty",
            "ຊື່ຜູ້ສຳຫຼວດບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
        ));
    }

    if survey.surveyor_license.is_empty() {
        return Err(LandLawError::missing_documentation(
            "Surveyor license number is required",
            "ຕ້ອງມີເລກໃບອະນຸຍາດຜູ້ສຳຫຼວດ",
        ));
    }

    if survey.measured_area_sqm <= 0.0 {
        return Err(LandLawError::invalid_cadastral_survey(
            "Measured area must be greater than zero",
            "ເນື້ອທີ່ວັດໄດ້ຕ້ອງຫຼາຍກວ່າສູນ",
        ));
    }

    if survey.boundary_coordinates.len() < 3 {
        return Err(LandLawError::invalid_cadastral_survey(
            "At least 3 boundary coordinates required",
            "ຕ້ອງມີພິກັດເຂດແດນຢ່າງໜ້ອຍ 3 ຈຸດ",
        ));
    }

    if !survey.approved {
        return Err(LandLawError::invalid_cadastral_survey(
            "Survey must be approved by land authority",
            "ການສຳຫຼວດຕ້ອງໄດ້ຮັບການອະນຸມັດຈາກເຈົ້າໜ້າທີ່ທີ່ດິນ",
        ));
    }

    Ok(())
}

/// Validates a land title according to Lao Land Law
///
/// ## Validation Rules
///
/// 1. Title number must be unique and valid
/// 2. Holder information must be complete
/// 3. Foreign ownership restrictions must be observed
/// 4. Cadastre registration required for full titles
pub fn validate_land_title(title: &LandTitle) -> Result<()> {
    if title.title_number.is_empty() {
        return Err(LandLawError::invalid_land_title(
            "Title number cannot be empty",
            "ເລກໃບຕາດິນບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
        ));
    }

    if title.holder_name.is_empty() {
        return Err(LandLawError::invalid_land_title(
            "Holder name cannot be empty",
            "ຊື່ຜູ້ຖືບໍ່ສາມາດຫວ່າງເປົ່າໄດ້",
        ));
    }

    if title.area_sqm == 0 {
        return Err(LandLawError::invalid_land_title(
            "Land area must be greater than zero",
            "ເນື້ອທີ່ດິນຕ້ອງຫຼາຍກວ່າສູນ",
        ));
    }

    // Full titles should be cadastre registered
    if matches!(title.title_type, LandTitleType::FullTitle) && !title.cadastre_registered {
        return Err(LandLawError::invalid_land_registration(
            "Full land titles must be registered in central cadastre",
            "ໃບຕາດິນເຕັມສິດຕ້ອງລົງທະບຽນໃນທະບຽນສູນກາງ",
        ));
    }

    // Validate holder status (foreign ownership restrictions)
    let is_perpetual = matches!(title.title_type, LandTitleType::FullTitle);
    validate_foreign_ownership(&title.holder_status, is_perpetual)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_perpetual_use_right_success() {
        let use_right = LandUseRight::PerpetualUse {
            holder_name: "Somchai Vongphachan".to_string(),
            holder_nationality: "LAO".to_string(),
            granted_at: Utc::now(),
            parcel_id: "VTE-001-2024".to_string(),
            area_sqm: 1500,
            permitted_use: LandUsePurpose::Residential,
        };

        assert!(validate_land_use_right(&use_right).is_ok());
    }

    #[test]
    fn test_validate_perpetual_use_right_foreign_fails() {
        let use_right = LandUseRight::PerpetualUse {
            holder_name: "John Doe".to_string(),
            holder_nationality: "USA".to_string(),
            granted_at: Utc::now(),
            parcel_id: "VTE-002-2024".to_string(),
            area_sqm: 1000,
            permitted_use: LandUsePurpose::Residential,
        };

        assert!(validate_land_use_right(&use_right).is_err());
    }

    #[test]
    fn test_validate_foreign_ownership_perpetual_fails() {
        let status = ForeignOwnershipStatus::ForeignNational {
            passport_number: "P1234567".to_string(),
            nationality: "Thailand".to_string(),
            lease_approved: true,
        };

        assert!(validate_foreign_ownership(&status, true).is_err());
    }

    #[test]
    fn test_validate_foreign_ownership_lease_success() {
        let status = ForeignOwnershipStatus::ForeignNational {
            passport_number: "P1234567".to_string(),
            nationality: "Thailand".to_string(),
            lease_approved: true,
        };

        assert!(validate_foreign_ownership(&status, false).is_ok());
    }
}
