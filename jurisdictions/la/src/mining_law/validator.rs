//! Mining Law Validators (ຕົວກວດສອບກົດໝາຍບໍ່ແຮ່)
//!
//! Validation functions for Lao mining law compliance based on:
//! - **Mining Law 2017** (Law No. 31/NA)
//!
//! # Legal References
//! - Articles 11-13: Mineral Classification
//! - Articles 18-21: Foreign Investment
//! - Articles 24-31: Licenses and Concessions
//! - Articles 37-40: Community Rights
//! - Articles 45-47: Royalties
//! - Articles 50-54: Environmental Requirements

use super::error::{MiningLawError, Result};
use super::types::*;

// ============================================================================
// License Validation (ການກວດສອບໃບອະນຸຍາດ)
// ============================================================================

/// Validate mining license (ກວດສອບໃບອະນຸຍາດບໍ່ແຮ່)
///
/// Article 24-28: License requirements and validity
///
/// # Arguments
/// * `license` - The mining license to validate
/// * `current_date` - Current date for expiry check (YYYY-MM-DD format)
///
/// # Returns
/// * `Ok(())` if license is valid
/// * `Err(MiningLawError)` if license is invalid or expired
pub fn validate_mining_license(license: &MiningLicense, current_date: &str) -> Result<()> {
    // Check required fields
    if license.license_number.trim().is_empty() {
        return Err(MiningLawError::MissingRequiredField {
            field_name: "license_number".to_string(),
        });
    }

    if license.holder_name.trim().is_empty() {
        return Err(MiningLawError::MissingRequiredField {
            field_name: "holder_name".to_string(),
        });
    }

    // Check status
    match license.status {
        LicenseStatus::Active | LicenseStatus::Renewed => {
            // Check expiry
            if current_date > license.expiry_date.as_str() {
                return Err(MiningLawError::LicenseExpired {
                    license_number: license.license_number.clone(),
                    expiry_date: license.expiry_date.clone(),
                });
            }
        }
        LicenseStatus::Expired => {
            return Err(MiningLawError::LicenseExpired {
                license_number: license.license_number.clone(),
                expiry_date: license.expiry_date.clone(),
            });
        }
        LicenseStatus::Suspended => {
            return Err(MiningLawError::LicenseSuspendedOrRevoked {
                license_number: license.license_number.clone(),
                status: "ໂຈະ (suspended)".to_string(),
            });
        }
        LicenseStatus::Revoked => {
            return Err(MiningLawError::LicenseSuspendedOrRevoked {
                license_number: license.license_number.clone(),
                status: "ຖືກຖອນ (revoked)".to_string(),
            });
        }
        LicenseStatus::Pending | LicenseStatus::UnderReview => {
            return Err(MiningLawError::ValidationError {
                message: "License is not yet approved".to_string(),
            });
        }
    }

    // Check renewal limits for exploration licenses
    if license.license_type == MiningLicenseType::Exploration
        && license.renewals > EXPLORATION_LICENSE_MAX_RENEWALS
    {
        return Err(MiningLawError::LicenseRenewalLimitExceeded {
            current_renewals: license.renewals,
            max_renewals: EXPLORATION_LICENSE_MAX_RENEWALS,
        });
    }

    // Check conditions compliance
    for condition in &license.conditions {
        if !condition.compliant {
            return Err(MiningLawError::NonCompliance {
                violation: condition.description.clone(),
                article: 28,
            });
        }
    }

    Ok(())
}

/// Validate license type for activity (ກວດສອບປະເພດໃບອະນຸຍາດສຳລັບກິດຈະກຳ)
///
/// Article 25-27: License types and permitted activities
pub fn validate_license_for_activity(
    license_type: MiningLicenseType,
    activity: &str,
) -> Result<()> {
    let valid = match license_type {
        MiningLicenseType::Exploration => {
            activity.contains("exploration")
                || activity.contains("survey")
                || activity.contains("ສຳຫຼວດ")
        }
        MiningLicenseType::Mining => {
            activity.contains("mining")
                || activity.contains("extraction")
                || activity.contains("ຂຸດຄົ້ນ")
        }
        MiningLicenseType::Processing => {
            activity.contains("processing")
                || activity.contains("refining")
                || activity.contains("ປຸງແຕ່ງ")
        }
        MiningLicenseType::SmallScale => {
            activity.contains("small-scale")
                || activity.contains("artisanal")
                || activity.contains("ຂະໜາດນ້ອຍ")
        }
    };

    if !valid {
        return Err(MiningLawError::InvalidLicenseType {
            license_type: license_type.lao_name().to_string(),
            activity: activity.to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Concession Validation (ການກວດສອບສຳປະທານ)
// ============================================================================

/// Validate mining concession (ກວດສອບສຳປະທານບໍ່ແຮ່)
///
/// Articles 30-35: Concession requirements
///
/// # Arguments
/// * `concession` - The mining concession to validate
///
/// # Returns
/// * `Ok(Vec<String>)` - List of warnings (non-critical issues)
/// * `Err(MiningLawError)` - Critical violation found
pub fn validate_mining_concession(concession: &MiningConcession) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Check required fields
    if concession.concession_id.trim().is_empty() {
        return Err(MiningLawError::MissingRequiredField {
            field_name: "concession_id".to_string(),
        });
    }

    if concession.holder_name.trim().is_empty() {
        return Err(MiningLawError::MissingRequiredField {
            field_name: "holder_name".to_string(),
        });
    }

    // Check area limits (Article 30)
    let classification = concession.primary_mineral.classification();
    let max_area = concession.concession_type.max_area_hectares(classification);

    if concession.area_hectares > max_area {
        return Err(MiningLawError::ConcessionAreaExceedsLimit {
            actual_hectares: concession.area_hectares,
            max_hectares: max_area,
            mineral_type: concession.primary_mineral.lao_name().to_string(),
        });
    }

    // Check duration limits (Article 31)
    let max_duration = concession
        .concession_type
        .max_duration_years(classification);

    if concession.duration_years > max_duration {
        return Err(MiningLawError::ConcessionDurationExceedsLimit {
            actual_years: concession.duration_years,
            max_years: max_duration,
        });
    }

    // Check status allows operations (Article 35)
    if !concession.status.allows_mining()
        && matches!(
            concession.concession_type,
            ConcessionType::Mining | ConcessionType::SmallScale
        )
    {
        return Err(MiningLawError::InvalidConcessionStatus {
            status: concession.status.lao_name().to_string(),
        });
    }

    // Warning if approaching area limit
    if concession.area_hectares > max_area * 0.9 {
        warnings.push(format!(
            "Concession area is {:.1}% of maximum allowed",
            (concession.area_hectares / max_area) * 100.0
        ));
    }

    Ok(warnings)
}

/// Validate concession area (ກວດສອບພື້ນທີ່ສຳປະທານ)
///
/// Article 30: Concession area limits
pub fn validate_concession_area(
    concession_type: ConcessionType,
    mineral_classification: MineralClassification,
    area_hectares: f64,
) -> Result<()> {
    if area_hectares < 0.0 {
        return Err(MiningLawError::ValidationError {
            message: "Concession area cannot be negative".to_string(),
        });
    }

    let max_area = concession_type.max_area_hectares(mineral_classification);

    if area_hectares > max_area {
        return Err(MiningLawError::ConcessionAreaExceedsLimit {
            actual_hectares: area_hectares,
            max_hectares: max_area,
            mineral_type: mineral_classification.lao_name().to_string(),
        });
    }

    Ok(())
}

/// Validate concession duration (ກວດສອບໄລຍະເວລາສຳປະທານ)
///
/// Article 31: Concession duration limits
pub fn validate_concession_duration(
    concession_type: ConcessionType,
    mineral_classification: MineralClassification,
    duration_years: u32,
) -> Result<()> {
    let max_duration = concession_type.max_duration_years(mineral_classification);

    if duration_years > max_duration {
        return Err(MiningLawError::ConcessionDurationExceedsLimit {
            actual_years: duration_years,
            max_years: max_duration,
        });
    }

    Ok(())
}

// ============================================================================
// Mineral Classification Validation (ການກວດສອບການຈັດປະເພດແຮ່)
// ============================================================================

/// Validate mineral classification requirements (ກວດສອບຂໍ້ກຳນົດການຈັດປະເພດແຮ່)
///
/// Articles 11-13: Mineral classification and special requirements
pub fn validate_mineral_classification(
    mineral_type: &MineralType,
    has_government_approval: bool,
) -> Result<()> {
    let classification = mineral_type.classification();

    // Check if government approval is required (Article 12)
    if classification.requires_government_approval() && !has_government_approval {
        return Err(MiningLawError::StrategicMineralRequiresApproval {
            mineral: mineral_type.lao_name().to_string(),
        });
    }

    // Special check for rare earth elements (Article 13)
    if matches!(classification, MineralClassification::RareEarth) {
        return Err(MiningLawError::RareEarthRestriction);
    }

    Ok(())
}

/// Validate mineral for export (ກວດສອບແຮ່ສຳລັບການສົ່ງອອກ)
///
/// Article 36: Raw ore export restrictions
pub fn validate_mineral_export(mineral_type: &MineralType, is_processed: bool) -> Result<()> {
    let classification = mineral_type.classification();

    // Strategic minerals require processing before export
    if classification == MineralClassification::Strategic && !is_processed {
        return Err(MiningLawError::RawOreExportRestricted {
            mineral: mineral_type.lao_name().to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Royalty Validation (ການກວດສອບຄ່າພາກຫຼວງ)
// ============================================================================

/// Validate royalty rate (ກວດສອບອັດຕາຄ່າພາກຫຼວງ)
///
/// Article 45: Royalty rate requirements
pub fn validate_royalty_rate(mineral_type: &MineralType, applied_rate: f64) -> Result<()> {
    let required_rate = mineral_type.royalty_rate();

    // Allow small tolerance for floating point comparison
    if (applied_rate - required_rate).abs() > 0.01 {
        return Err(MiningLawError::RoyaltyRateMismatch {
            mineral: mineral_type.lao_name().to_string(),
            actual_rate: applied_rate,
            required_rate,
        });
    }

    Ok(())
}

/// Validate royalty payment (ກວດສອບການຊຳລະຄ່າພາກຫຼວງ)
///
/// Article 47: Royalty payment requirements
pub fn validate_royalty_payment(
    payment: &RoyaltyPayment,
    current_date: &str,
) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Check production volume
    if payment.production_volume < 0.0 {
        return Err(MiningLawError::InvalidProductionVolume {
            volume: format!("{}", payment.production_volume),
        });
    }

    // Check for overdue payment
    if payment.status == PaymentStatus::Overdue
        || (payment.status == PaymentStatus::Pending && current_date > payment.due_date.as_str())
    {
        let days_overdue = estimate_days_between(&payment.due_date, current_date);
        return Err(MiningLawError::RoyaltyPaymentOverdue {
            days_overdue,
            amount_lak: payment.royalty_amount_lak,
        });
    }

    // Warning if payment is due soon
    if payment.status == PaymentStatus::Pending {
        let days_until_due = estimate_days_between(current_date, &payment.due_date);
        if days_until_due < 30 {
            warnings.push(format!(
                "Royalty payment of {} LAK due in {} days",
                payment.royalty_amount_lak, days_until_due
            ));
        }
    }

    Ok(warnings)
}

/// Calculate royalty amount (ຄິດໄລ່ຈຳນວນຄ່າພາກຫຼວງ)
///
/// Article 46: Royalty calculation
pub fn calculate_royalty_amount(mineral_type: &MineralType, market_value_lak: u64) -> u64 {
    let rate = mineral_type.royalty_rate() / 100.0;
    (market_value_lak as f64 * rate) as u64
}

// ============================================================================
// Environmental Validation (ການກວດສອບສິ່ງແວດລ້ອມ)
// ============================================================================

/// Validate mining environmental compliance (ກວດສອບການປະຕິບັດຕາມສິ່ງແວດລ້ອມບໍ່ແຮ່)
///
/// Articles 50-54: Environmental requirements
pub fn validate_environmental_compliance(
    compliance: &MiningEnvironmentalCompliance,
    concession_type: ConcessionType,
) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Check EIA requirement (Article 50)
    let requires_eia = matches!(
        concession_type,
        ConcessionType::Mining | ConcessionType::SmallScale
    );

    if requires_eia && !compliance.eia_approved {
        return Err(MiningLawError::MissingMiningEIA);
    }

    // Check rehabilitation bond (Article 52)
    if compliance.rehabilitation_bond_lak < compliance.required_rehabilitation_bond_lak {
        return Err(MiningLawError::InsufficientRehabilitationBond {
            actual_lak: compliance.rehabilitation_bond_lak,
            required_lak: compliance.required_rehabilitation_bond_lak,
        });
    }

    // Check closure plan (Article 53)
    if matches!(concession_type, ConcessionType::Mining) && !compliance.closure_plan_submitted {
        return Err(MiningLawError::MissingClosurePlan);
    }

    // Check for unresolved environmental violations
    let unresolved_violations: Vec<_> = compliance
        .environmental_violations
        .iter()
        .filter(|v| !v.corrective_action_completed)
        .collect();

    for violation in &unresolved_violations {
        if matches!(
            violation.severity,
            ViolationSeverity::Critical | ViolationSeverity::Major
        ) {
            return Err(MiningLawError::EnvironmentalViolation {
                description: violation.description.clone(),
            });
        }
    }

    // Warnings for minor issues
    if !unresolved_violations.is_empty() {
        warnings.push(format!(
            "{} unresolved environmental violations",
            unresolved_violations.len()
        ));
    }

    if compliance.monitoring_reports_submitted < 4 {
        warnings.push("Less than 4 environmental monitoring reports submitted".to_string());
    }

    Ok(warnings)
}

/// Validate distance from protected area (ກວດສອບໄລຍະຫ່າງຈາກເຂດປ່າປ້ອງກັນ)
///
/// Article 51: Protected area distance requirement
pub fn validate_protected_area_distance(distance_meters: u32) -> Result<()> {
    if distance_meters < MIN_DISTANCE_FROM_PROTECTED_AREA_METERS {
        return Err(MiningLawError::TooCloseToProtectedArea {
            distance_meters,
            required_meters: MIN_DISTANCE_FROM_PROTECTED_AREA_METERS,
        });
    }

    Ok(())
}

/// Validate rehabilitation bond amount (ກວດສອບຈຳນວນເງິນຄ້ຳປະກັນການຟື້ນຟູ)
///
/// Article 52: Rehabilitation bond requirement
pub fn validate_rehabilitation_bond(
    project_cost_lak: u64,
    rehabilitation_bond_lak: u64,
) -> Result<()> {
    let required_bond = (project_cost_lak as f64 * REHABILITATION_BOND_MIN_PERCENT / 100.0) as u64;

    if rehabilitation_bond_lak < required_bond {
        return Err(MiningLawError::InsufficientRehabilitationBond {
            actual_lak: rehabilitation_bond_lak,
            required_lak: required_bond,
        });
    }

    Ok(())
}

// ============================================================================
// Foreign Investment Validation (ການກວດສອບການລົງທຶນຕ່າງປະເທດ)
// ============================================================================

/// Validate foreign investment (ກວດສອບການລົງທຶນຕ່າງປະເທດ)
///
/// Articles 18-21: Foreign investment requirements
pub fn validate_foreign_investment(
    investment: &ForeignInvestment,
    mineral_classification: MineralClassification,
) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Check joint venture requirement (Article 18)
    if mineral_classification.requires_joint_venture() && investment.lao_partner_name.is_none() {
        return Err(MiningLawError::JointVentureRequired {
            mineral: mineral_classification.lao_name().to_string(),
        });
    }

    // Check foreign ownership limit (Article 19)
    let max_foreign = mineral_classification.max_foreign_ownership_percent();
    if investment.foreign_ownership_percent > max_foreign {
        return Err(MiningLawError::ForeignOwnershipExceedsLimit {
            actual_percent: investment.foreign_ownership_percent,
            max_percent: max_foreign,
            mineral_type: mineral_classification.lao_name().to_string(),
        });
    }

    // Check local content requirement (Article 20)
    if investment.local_content_percent_commitment < LOCAL_CONTENT_MIN_PERCENT {
        return Err(MiningLawError::LocalContentRequirementNotMet {
            actual_percent: investment.local_content_percent_commitment,
            required_percent: LOCAL_CONTENT_MIN_PERCENT,
        });
    }

    // Check technology transfer obligations (Article 21)
    let incomplete_transfers: Vec<_> = investment
        .technology_transfer_commitments
        .iter()
        .filter(|t| !t.completed)
        .collect();

    if !incomplete_transfers.is_empty() {
        warnings.push(format!(
            "{} technology transfer commitments pending",
            incomplete_transfers.len()
        ));
    }

    // Warning if approaching ownership limit
    if investment.foreign_ownership_percent > max_foreign * 0.9 {
        warnings.push(format!(
            "Foreign ownership at {:.1}% of {:.1}% limit",
            investment.foreign_ownership_percent, max_foreign
        ));
    }

    Ok(warnings)
}

/// Validate foreign ownership percentage (ກວດສອບເປີເຊັນການຖືຫຸ້ນຕ່າງປະເທດ)
///
/// Article 19: Foreign ownership limits
pub fn validate_foreign_ownership(
    mineral_classification: MineralClassification,
    foreign_ownership_percent: f64,
) -> Result<()> {
    let max_foreign = mineral_classification.max_foreign_ownership_percent();

    if foreign_ownership_percent > max_foreign {
        return Err(MiningLawError::ForeignOwnershipExceedsLimit {
            actual_percent: foreign_ownership_percent,
            max_percent: max_foreign,
            mineral_type: mineral_classification.lao_name().to_string(),
        });
    }

    Ok(())
}

/// Validate local content (ກວດສອບເນື້ອໃນທ້ອງຖິ່ນ)
///
/// Article 20: Local content requirements
pub fn validate_local_content(local_content_percent: f64) -> Result<()> {
    if local_content_percent < LOCAL_CONTENT_MIN_PERCENT {
        return Err(MiningLawError::LocalContentRequirementNotMet {
            actual_percent: local_content_percent,
            required_percent: LOCAL_CONTENT_MIN_PERCENT,
        });
    }

    Ok(())
}

// ============================================================================
// Community Rights Validation (ການກວດສອບສິດຊຸມຊົນ)
// ============================================================================

/// Validate prior consultation (ກວດສອບການປຶກສາຫາລືລ່ວງໜ້າ)
///
/// Article 37: Prior consultation requirements
pub fn validate_prior_consultation(consultations: &[CommunityConsultation]) -> Result<()> {
    if consultations.is_empty() {
        return Err(MiningLawError::MissingPriorConsultation);
    }

    // Check that all consultations have consent
    for consultation in consultations {
        if !consultation.consent_obtained {
            return Err(MiningLawError::MissingPriorConsultation);
        }
    }

    Ok(())
}

/// Validate community compensation (ກວດສອບການຊົດເຊີຍຊຸມຊົນ)
///
/// Article 38: Community compensation requirements
pub fn validate_community_compensation(
    actual_compensation_lak: u64,
    required_compensation_lak: u64,
) -> Result<()> {
    if actual_compensation_lak < required_compensation_lak {
        return Err(MiningLawError::InadequateCommunityCompensation {
            actual_lak: actual_compensation_lak,
            required_lak: required_compensation_lak,
        });
    }

    Ok(())
}

/// Validate local employment quota (ກວດສອບໂຄຕ້າການຈ້າງງານທ້ອງຖິ່ນ)
///
/// Article 39: Local employment requirements
pub fn validate_local_employment(employment: &LocalEmployment) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Check local employment quota
    if employment.local_percentage < LOCAL_EMPLOYMENT_MIN_PERCENT {
        return Err(MiningLawError::LocalEmploymentQuotaNotMet {
            actual_percent: employment.local_percentage,
            required_percent: LOCAL_EMPLOYMENT_MIN_PERCENT,
        });
    }

    // Warnings for approaching limits
    if employment.local_percentage < LOCAL_EMPLOYMENT_MIN_PERCENT + 5.0 {
        warnings.push(format!(
            "Local employment at {:.1}%, close to {:.1}% minimum",
            employment.local_percentage, LOCAL_EMPLOYMENT_MIN_PERCENT
        ));
    }

    Ok(warnings)
}

/// Validate revenue sharing (ກວດສອບການແບ່ງລາຍຮັບ)
///
/// Article 40: Revenue sharing requirements
pub fn validate_revenue_sharing(revenue_share_percent: f64) -> Result<()> {
    if revenue_share_percent < COMMUNITY_REVENUE_SHARE_MIN_PERCENT {
        return Err(MiningLawError::RevenueSharingNotCompliant {
            actual_percent: revenue_share_percent,
            required_percent: COMMUNITY_REVENUE_SHARE_MIN_PERCENT,
        });
    }

    Ok(())
}

// ============================================================================
// Small-Scale Mining Validation (ການກວດສອບບໍ່ແຮ່ຂະໜາດນ້ອຍ)
// ============================================================================

/// Validate small-scale mining (ກວດສອບບໍ່ແຮ່ຂະໜາດນ້ອຍ)
///
/// Articles 42-43: Small-scale and artisanal mining requirements
pub fn validate_small_scale_mining(
    area_hectares: f64,
    is_registered: bool,
    is_artisanal: bool,
) -> Result<()> {
    let max_area = if is_artisanal {
        ARTISANAL_MINING_MAX_HECTARES
    } else {
        SMALL_SCALE_MINING_MAX_HECTARES
    };

    if area_hectares > max_area {
        return Err(MiningLawError::SmallScaleMiningExceedsLimits {
            violation: format!(
                "Area {:.1} hectares exceeds limit of {:.1} hectares",
                area_hectares, max_area
            ),
        });
    }

    if is_artisanal && !is_registered {
        return Err(MiningLawError::ArtisanalMiningNotRegistered);
    }

    Ok(())
}

// ============================================================================
// Comprehensive Validation (ການກວດສອບແບບຄົບຖ້ວນ)
// ============================================================================

/// Perform comprehensive mining compliance validation
/// ກວດສອບການປະຕິບັດຕາມກົດໝາຍບໍ່ແຮ່ແບບຄົບຖ້ວນ
///
/// # Arguments
/// * `license` - Mining license
/// * `concession` - Mining concession
/// * `compliance` - Environmental compliance record
/// * `current_date` - Current date for expiry checks
///
/// # Returns
/// * `Ok(Vec<String>)` - List of warnings (non-critical issues)
/// * `Err(MiningLawError)` - Critical violation found
pub fn validate_mining_compliance(
    license: &MiningLicense,
    concession: &MiningConcession,
    compliance: &MiningEnvironmentalCompliance,
    current_date: &str,
) -> Result<Vec<String>> {
    let mut all_warnings = Vec::new();

    // Validate license
    validate_mining_license(license, current_date)?;

    // Validate concession
    let concession_warnings = validate_mining_concession(concession)?;
    all_warnings.extend(concession_warnings);

    // Validate environmental compliance
    let env_warnings = validate_environmental_compliance(compliance, concession.concession_type)?;
    all_warnings.extend(env_warnings);

    // Validate protected area distance if provided
    if let Some(distance) = concession.distance_from_protected_area_meters {
        validate_protected_area_distance(distance)?;
    }

    // Validate foreign ownership if applicable
    let classification = concession.primary_mineral.classification();
    validate_foreign_ownership(classification, concession.foreign_ownership_percent)?;

    // Validate local content
    validate_local_content(concession.local_content_percent)?;

    Ok(all_warnings)
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper function to estimate days between two dates
/// This is a simplified implementation - in production, use chrono
fn estimate_days_between(date1: &str, date2: &str) -> u32 {
    // Parse years
    let year1: i32 = date1.get(0..4).and_then(|s| s.parse().ok()).unwrap_or(0);
    let year2: i32 = date2.get(0..4).and_then(|s| s.parse().ok()).unwrap_or(0);

    // Parse months
    let month1: i32 = date1.get(5..7).and_then(|s| s.parse().ok()).unwrap_or(1);
    let month2: i32 = date2.get(5..7).and_then(|s| s.parse().ok()).unwrap_or(1);

    // Parse days
    let day1: i32 = date1.get(8..10).and_then(|s| s.parse().ok()).unwrap_or(1);
    let day2: i32 = date2.get(8..10).and_then(|s| s.parse().ok()).unwrap_or(1);

    // Rough estimate: 365 days per year, 30 days per month
    let total_days1 = year1 * 365 + month1 * 30 + day1;
    let total_days2 = year2 * 365 + month2 * 30 + day2;

    (total_days2 - total_days1).unsigned_abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_license_valid() {
        let license = MiningLicenseBuilder::new()
            .license_number("ML-2026-001")
            .license_type(MiningLicenseType::Mining)
            .holder_name("Mining Company Ltd")
            .issue_date("2024-01-01")
            .expiry_date("2029-01-01")
            .status(LicenseStatus::Active)
            .province("Savannakhet")
            .issuing_authority("MEM")
            .build();

        let result = validate_mining_license(&license, "2026-01-15");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_license_expired() {
        let license = MiningLicenseBuilder::new()
            .license_number("ML-2026-001")
            .license_type(MiningLicenseType::Mining)
            .holder_name("Mining Company Ltd")
            .issue_date("2020-01-01")
            .expiry_date("2025-01-01")
            .status(LicenseStatus::Active)
            .province("Savannakhet")
            .build();

        let result = validate_mining_license(&license, "2026-01-15");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MiningLawError::LicenseExpired { .. }
        ));
    }

    #[test]
    fn test_validate_concession_area() {
        // Valid area
        let result = validate_concession_area(
            ConcessionType::SmallScale,
            MineralClassification::Common,
            50.0,
        );
        assert!(result.is_ok());

        // Exceeds limit
        let result = validate_concession_area(
            ConcessionType::SmallScale,
            MineralClassification::Common,
            150.0,
        );
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MiningLawError::ConcessionAreaExceedsLimit { .. }
        ));
    }

    #[test]
    fn test_validate_royalty_rate() {
        // Gold - 5%
        let result = validate_royalty_rate(&MineralType::Gold, 5.0);
        assert!(result.is_ok());

        // Gold - wrong rate
        let result = validate_royalty_rate(&MineralType::Gold, 3.0);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MiningLawError::RoyaltyRateMismatch { .. }
        ));
    }

    #[test]
    fn test_calculate_royalty_amount() {
        // Gold at 5%
        let amount = calculate_royalty_amount(&MineralType::Gold, 1_000_000);
        assert_eq!(amount, 50_000);

        // Copper at 3%
        let amount = calculate_royalty_amount(&MineralType::Copper, 1_000_000);
        assert_eq!(amount, 30_000);
    }

    #[test]
    fn test_validate_protected_area_distance() {
        // Valid distance
        let result = validate_protected_area_distance(1500);
        assert!(result.is_ok());

        // Too close
        let result = validate_protected_area_distance(500);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MiningLawError::TooCloseToProtectedArea { .. }
        ));
    }

    #[test]
    fn test_validate_foreign_ownership() {
        // Valid for common minerals
        let result = validate_foreign_ownership(MineralClassification::Common, 100.0);
        assert!(result.is_ok());

        // Valid for strategic minerals
        let result = validate_foreign_ownership(MineralClassification::Strategic, 70.0);
        assert!(result.is_ok());

        // Exceeds limit for strategic minerals
        let result = validate_foreign_ownership(MineralClassification::Strategic, 80.0);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MiningLawError::ForeignOwnershipExceedsLimit { .. }
        ));
    }

    #[test]
    fn test_validate_local_content() {
        // Valid
        let result = validate_local_content(35.0);
        assert!(result.is_ok());

        // Below minimum
        let result = validate_local_content(25.0);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MiningLawError::LocalContentRequirementNotMet { .. }
        ));
    }

    #[test]
    fn test_validate_small_scale_mining() {
        // Valid small-scale
        let result = validate_small_scale_mining(50.0, true, false);
        assert!(result.is_ok());

        // Exceeds limit
        let result = validate_small_scale_mining(150.0, true, false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MiningLawError::SmallScaleMiningExceedsLimits { .. }
        ));

        // Artisanal not registered
        let result = validate_small_scale_mining(3.0, false, true);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MiningLawError::ArtisanalMiningNotRegistered
        ));
    }

    #[test]
    fn test_validate_revenue_sharing() {
        // Valid
        let result = validate_revenue_sharing(2.0);
        assert!(result.is_ok());

        // Below minimum
        let result = validate_revenue_sharing(0.5);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MiningLawError::RevenueSharingNotCompliant { .. }
        ));
    }

    #[test]
    fn test_validate_rehabilitation_bond() {
        // Valid bond (5% of 100,000,000 = 5,000,000)
        let result = validate_rehabilitation_bond(100_000_000, 5_000_000);
        assert!(result.is_ok());

        // Insufficient bond
        let result = validate_rehabilitation_bond(100_000_000, 3_000_000);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MiningLawError::InsufficientRehabilitationBond { .. }
        ));
    }

    #[test]
    fn test_validate_mineral_classification() {
        // Common mineral - no approval needed
        let result = validate_mineral_classification(&MineralType::Stone, false);
        assert!(result.is_ok());

        // Strategic mineral - needs approval
        let result = validate_mineral_classification(&MineralType::Gold, false);
        assert!(result.is_err());

        // Strategic mineral - has approval
        let result = validate_mineral_classification(&MineralType::Gold, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_mineral_export() {
        // Processed strategic mineral - ok
        let result = validate_mineral_export(&MineralType::Gold, true);
        assert!(result.is_ok());

        // Unprocessed strategic mineral - restricted
        let result = validate_mineral_export(&MineralType::Gold, false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MiningLawError::RawOreExportRestricted { .. }
        ));

        // Unprocessed common mineral - ok
        let result = validate_mineral_export(&MineralType::Stone, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_prior_consultation() {
        // No consultations
        let result = validate_prior_consultation(&[]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MiningLawError::MissingPriorConsultation
        ));

        // Valid consultation with consent
        let consultation = CommunityConsultation {
            consultation_id: "CC-001".to_string(),
            concession_id: "MC-001".to_string(),
            village_name: "Test Village".to_string(),
            village_name_lao: "ບ້ານທົດສອບ".to_string(),
            district: "Test District".to_string(),
            consultation_date: "2025-06-01".to_string(),
            participants_count: 50,
            households_represented: 30,
            issues_raised: vec![],
            consent_obtained: true,
            consent_conditions: vec![],
            documentation_available: true,
        };
        let result = validate_prior_consultation(&[consultation]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_license_for_activity() {
        // Valid combinations
        let result = validate_license_for_activity(MiningLicenseType::Exploration, "exploration");
        assert!(result.is_ok());

        let result = validate_license_for_activity(MiningLicenseType::Mining, "mining operation");
        assert!(result.is_ok());

        // Invalid combination
        let result = validate_license_for_activity(MiningLicenseType::Exploration, "mining");
        assert!(result.is_err());
    }
}
