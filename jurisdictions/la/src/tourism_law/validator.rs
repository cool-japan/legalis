//! Tourism Law Validators (ຕົວກວດສອບກົດໝາຍທ່ອງທ່ຽວ)
//!
//! Validation functions for Lao tourism law compliance based on:
//! - **Tourism Law 2013** (Law No. 32/NA)
//! - **Tourism Development Regulations**
//! - **ASEAN Tourism Agreement**

use super::error::{Result, TourismLawError};
use super::types::*;
use chrono::Utc;

// ============================================================================
// Tourism Enterprise Validation (ການກວດສອບວິສາຫະກິດທ່ອງທ່ຽວ)
// ============================================================================

/// Validate tourism enterprise license (ກວດສອບໃບອະນຸຍາດວິສາຫະກິດທ່ອງທ່ຽວ)
///
/// Tourism Law 2013, Article 23-27: All tourism enterprises must be licensed.
///
/// # Arguments
/// * `enterprise` - Tourism enterprise to validate
///
/// # Returns
/// * `Ok(())` if enterprise license is valid
/// * `Err(TourismLawError)` if license is invalid or missing
pub fn validate_enterprise_license(enterprise: &TourismEnterprise) -> Result<()> {
    // Check license number is not empty
    if enterprise.tourism_license_number.trim().is_empty() {
        return Err(TourismLawError::EnterpriseUnlicensed {
            enterprise_name: enterprise.name_en.clone(),
        });
    }

    // Check license status
    match &enterprise.license_status {
        LicenseStatus::Active => {
            // Check if license is expired by date
            let now = Utc::now();
            if now >= enterprise.license_expiry_date {
                return Err(TourismLawError::EnterpriseLicenseExpired {
                    enterprise_name: enterprise.name_en.clone(),
                    expiry_date: enterprise
                        .license_expiry_date
                        .format("%Y-%m-%d")
                        .to_string(),
                });
            }
        }
        LicenseStatus::Expired { expired_on } => {
            return Err(TourismLawError::EnterpriseLicenseExpired {
                enterprise_name: enterprise.name_en.clone(),
                expiry_date: expired_on.format("%Y-%m-%d").to_string(),
            });
        }
        LicenseStatus::Suspended { reason, .. } => {
            return Err(TourismLawError::EnterpriseLicenseSuspended {
                enterprise_name: enterprise.name_en.clone(),
                reason: reason.clone(),
            });
        }
        LicenseStatus::Revoked { reason, .. } => {
            return Err(TourismLawError::EnterpriseLicenseRevoked {
                enterprise_name: enterprise.name_en.clone(),
                reason: reason.clone(),
            });
        }
        LicenseStatus::Pending { .. } => {
            return Err(TourismLawError::EnterpriseUnlicensed {
                enterprise_name: enterprise.name_en.clone(),
            });
        }
        LicenseStatus::PendingRenewal { .. } => {
            // Pending renewal is acceptable if the license hasn't expired yet
            let now = Utc::now();
            if now >= enterprise.license_expiry_date {
                return Err(TourismLawError::EnterpriseLicenseExpired {
                    enterprise_name: enterprise.name_en.clone(),
                    expiry_date: enterprise
                        .license_expiry_date
                        .format("%Y-%m-%d")
                        .to_string(),
                });
            }
        }
    }

    Ok(())
}

/// Validate foreign ownership limits (ກວດສອບຂີດຈຳກັດການເປັນເຈົ້າຂອງຕ່າງປະເທດ)
///
/// Tourism Law 2013, Article 25: Foreign ownership limits apply to tourism enterprises.
///
/// # Arguments
/// * `enterprise` - Tourism enterprise to validate
///
/// # Returns
/// * `Ok(())` if foreign ownership is within limits
/// * `Err(TourismLawError)` if foreign ownership exceeds limits
pub fn validate_foreign_ownership(enterprise: &TourismEnterprise) -> Result<()> {
    let max_percent = enterprise.category.max_foreign_ownership_percent();

    // Check if foreign investment is allowed
    if max_percent == 0.0 && enterprise.foreign_ownership_percent > 0.0 {
        return Err(TourismLawError::ForeignInvestmentNotPermitted {
            activity: enterprise.category.description_en().to_string(),
        });
    }

    // Check if foreign ownership exceeds limit
    if enterprise.foreign_ownership_percent > max_percent {
        return Err(TourismLawError::ForeignOwnershipLimitExceeded {
            actual_percent: enterprise.foreign_ownership_percent,
            max_percent,
            activity: enterprise.category.description_en().to_string(),
        });
    }

    Ok(())
}

/// Validate enterprise for activity (ກວດສອບວິສາຫະກິດສຳລັບກິດຈະກຳ)
///
/// # Arguments
/// * `enterprise` - Tourism enterprise
/// * `activity` - Activity being performed
///
/// # Returns
/// * `Ok(())` if enterprise is authorized for activity
/// * `Err(TourismLawError)` if enterprise type is invalid for activity
pub fn validate_enterprise_for_activity(
    enterprise: &TourismEnterprise,
    activity: &str,
) -> Result<()> {
    let activity_lower = activity.to_lowercase();

    // Check if tour operator is conducting correct type of tours
    match enterprise.category {
        TourismEnterpriseCategory::TourOperatorInbound => {
            if activity_lower.contains("outbound") {
                return Err(TourismLawError::TourOperatorScopeExceeded {
                    operator_name: enterprise.name_en.clone(),
                    license_type: "Inbound".to_string(),
                    activity: activity.to_string(),
                });
            }
        }
        TourismEnterpriseCategory::TourOperatorOutbound => {
            if activity_lower.contains("inbound") {
                return Err(TourismLawError::TourOperatorScopeExceeded {
                    operator_name: enterprise.name_en.clone(),
                    license_type: "Outbound".to_string(),
                    activity: activity.to_string(),
                });
            }
        }
        TourismEnterpriseCategory::TourOperatorDomestic => {
            if activity_lower.contains("international")
                || activity_lower.contains("outbound")
                || activity_lower.contains("cross-border")
            {
                return Err(TourismLawError::TourOperatorScopeExceeded {
                    operator_name: enterprise.name_en.clone(),
                    license_type: "Domestic".to_string(),
                    activity: activity.to_string(),
                });
            }
        }
        _ => {}
    }

    Ok(())
}

// ============================================================================
// Hotel Classification Validation (ການກວດສອບການຈັດລະດັບໂຮງແຮມ)
// ============================================================================

/// Validate hotel classification (ກວດສອບການຈັດລະດັບໂຮງແຮມ)
///
/// Tourism Law 2013, Article 30-33: Hotels must be classified according to standards.
///
/// # Arguments
/// * `accommodation` - Accommodation to validate
///
/// # Returns
/// * `Ok(())` if classification is valid
/// * `Err(TourismLawError)` if classification is invalid or missing
pub fn validate_hotel_classification(accommodation: &Accommodation) -> Result<()> {
    // Only validate star-rating applicable types
    if !accommodation.accommodation_type.star_rating_applicable() {
        return Ok(());
    }

    // Check if hotel has a star rating
    let star_rating =
        accommodation
            .star_rating
            .ok_or_else(|| TourismLawError::HotelNotClassified {
                hotel_name: accommodation.enterprise.name_en.clone(),
            })?;

    // Check classification status
    match &accommodation.classification_status {
        HotelClassificationStatus::Classified { expiry_date, .. } => {
            if Utc::now() >= *expiry_date {
                return Err(TourismLawError::StarRatingExpired {
                    hotel_name: accommodation.enterprise.name_en.clone(),
                    expiry_date: expiry_date.format("%Y-%m-%d").to_string(),
                });
            }
        }
        HotelClassificationStatus::Expired {
            previous_rating: _,
            expired_on,
        } => {
            return Err(TourismLawError::StarRatingExpired {
                hotel_name: accommodation.enterprise.name_en.clone(),
                expiry_date: expired_on.format("%Y-%m-%d").to_string(),
            });
        }
        HotelClassificationStatus::NotClassified | HotelClassificationStatus::Pending { .. } => {
            return Err(TourismLawError::HotelNotClassified {
                hotel_name: accommodation.enterprise.name_en.clone(),
            });
        }
    }

    // Check room count meets minimum for star rating
    let min_rooms = star_rating.minimum_rooms();
    if accommodation.room_count < min_rooms {
        return Err(TourismLawError::InsufficientRooms {
            star_rating: star_rating.as_u8(),
            actual: accommodation.room_count,
            required: min_rooms,
        });
    }

    Ok(())
}

/// Validate hotel facilities for star rating (ກວດສອບສິ່ງອຳນວຍຄວາມສະດວກສຳລັບລະດັບດາວ)
///
/// # Arguments
/// * `accommodation` - Accommodation to validate
///
/// # Returns
/// * `Ok(())` if facilities meet requirements
/// * `Err(TourismLawError)` if required facilities are missing
pub fn validate_hotel_facilities(accommodation: &Accommodation) -> Result<()> {
    if let Some(rating) = accommodation.star_rating {
        let required_facilities: Vec<HotelFacility> = match rating {
            StarRating::OneStar => vec![],
            StarRating::TwoStar => vec![HotelFacility::Reception24Hour],
            StarRating::ThreeStar => vec![
                HotelFacility::Reception24Hour,
                HotelFacility::Restaurant,
                HotelFacility::AirConditioning,
            ],
            StarRating::FourStar => vec![
                HotelFacility::Reception24Hour,
                HotelFacility::Restaurant,
                HotelFacility::AirConditioning,
                HotelFacility::RoomService,
                HotelFacility::WiFi,
            ],
            StarRating::FiveStar => vec![
                HotelFacility::Reception24Hour,
                HotelFacility::Restaurant,
                HotelFacility::AirConditioning,
                HotelFacility::RoomService,
                HotelFacility::WiFi,
                HotelFacility::SwimmingPool,
                HotelFacility::FitnessCenter,
                HotelFacility::Spa,
                HotelFacility::BusinessCenter,
            ],
        };

        for facility in required_facilities {
            if !accommodation.facilities.contains(&facility) {
                return Err(TourismLawError::MissingRequiredFacility {
                    facility: facility.description_en().to_string(),
                    star_rating: rating.as_u8(),
                });
            }
        }
    }

    Ok(())
}

// ============================================================================
// Tour Guide Validation (ການກວດສອບໄກດ໌ນຳທ່ຽວ)
// ============================================================================

/// Validate tour guide license (ກວດສອບໃບອະນຸຍາດໄກດ໌ນຳທ່ຽວ)
///
/// Tourism Law 2013, Article 35-40: Tour guides must be licensed.
///
/// # Arguments
/// * `guide` - Tour guide to validate
///
/// # Returns
/// * `Ok(())` if guide license is valid
/// * `Err(TourismLawError)` if license is invalid or missing
pub fn validate_guide_license(guide: &TourGuide) -> Result<()> {
    // Check license number is not empty
    if guide.license_number.trim().is_empty() {
        return Err(TourismLawError::GuideUnlicensed {
            guide_name: guide.name.clone(),
        });
    }

    // Check license status
    match &guide.license_status {
        LicenseStatus::Active => {
            // Check if license is expired by date
            let now = Utc::now();
            if now >= guide.license_expiry_date {
                return Err(TourismLawError::GuideLicenseExpired {
                    guide_name: guide.name.clone(),
                    expiry_date: guide.license_expiry_date.format("%Y-%m-%d").to_string(),
                });
            }
        }
        LicenseStatus::Expired { expired_on } => {
            return Err(TourismLawError::GuideLicenseExpired {
                guide_name: guide.name.clone(),
                expiry_date: expired_on.format("%Y-%m-%d").to_string(),
            });
        }
        LicenseStatus::Suspended { reason, .. } => {
            return Err(TourismLawError::GuideLicenseSuspended {
                guide_name: guide.name.clone(),
                reason: reason.clone(),
            });
        }
        LicenseStatus::Revoked { reason, .. } => {
            return Err(TourismLawError::GuideLicenseSuspended {
                guide_name: guide.name.clone(),
                reason: format!("License revoked: {}", reason),
            });
        }
        LicenseStatus::Pending { .. } | LicenseStatus::PendingRenewal { .. } => {
            let now = Utc::now();
            if now >= guide.license_expiry_date {
                return Err(TourismLawError::GuideLicenseExpired {
                    guide_name: guide.name.clone(),
                    expiry_date: guide.license_expiry_date.format("%Y-%m-%d").to_string(),
                });
            }
        }
    }

    Ok(())
}

/// Validate guide language requirements (ກວດສອບຂໍ້ກຳນົດພາສາຂອງໄກດ໌)
///
/// Tourism Law 2013, Article 36: Guides must be proficient in Lao and at least one foreign language.
///
/// # Arguments
/// * `guide` - Tour guide to validate
///
/// # Returns
/// * `Ok(())` if language requirements are met
/// * `Err(TourismLawError)` if language proficiency is insufficient
pub fn validate_guide_language(guide: &TourGuide) -> Result<()> {
    // Check Lao proficiency
    let has_lao = guide.language_skills.iter().any(|skill| {
        skill.language.to_lowercase() == "lao" && skill.proficiency >= LanguageProficiency::Fluent
    });

    if !has_lao {
        return Err(TourismLawError::InsufficientLanguageProficiency {
            guide_name: guide.name.clone(),
            language: "Lao".to_string(),
        });
    }

    // Check foreign language proficiency (at least intermediate in one foreign language)
    let has_foreign = guide.language_skills.iter().any(|skill| {
        skill.language.to_lowercase() != "lao"
            && skill.proficiency >= LanguageProficiency::Intermediate
    });

    if !has_foreign {
        return Err(TourismLawError::InsufficientLanguageProficiency {
            guide_name: guide.name.clone(),
            language: "Foreign Language".to_string(),
        });
    }

    Ok(())
}

/// Validate guide training requirements (ກວດສອບຂໍ້ກຳນົດການຝຶກອົບຮົມໄກດ໌)
///
/// Tourism Law 2013, Article 37: Guides must complete required training.
///
/// # Arguments
/// * `guide` - Tour guide to validate
///
/// # Returns
/// * `Ok(())` if training requirements are met
/// * `Err(TourismLawError)` if training is insufficient
pub fn validate_guide_training(guide: &TourGuide) -> Result<()> {
    if guide.training_hours < MIN_GUIDE_TRAINING_HOURS {
        return Err(TourismLawError::MissingTrainingCertification {
            guide_name: guide.name.clone(),
        });
    }

    if guide.training_certifications.is_empty() {
        return Err(TourismLawError::MissingTrainingCertification {
            guide_name: guide.name.clone(),
        });
    }

    Ok(())
}

/// Validate guide scope (ກວດສອບຂອບເຂດໄກດ໌)
///
/// Tourism Law 2013, Article 39: Guides must operate within their license scope.
///
/// # Arguments
/// * `guide` - Tour guide to validate
/// * `zone` - Tourism zone where guide is operating
///
/// # Returns
/// * `Ok(())` if guide is authorized for zone
/// * `Err(TourismLawError)` if guide scope is exceeded
pub fn validate_guide_scope(guide: &TourGuide, zone: &TourismZone) -> Result<()> {
    match &guide.license_category {
        GuideLicenseCategory::Provincial => {
            // Provincial guides can only operate in their province
            if let Some(guide_province) = &guide.province
                && guide_province != &zone.province
            {
                return Err(TourismLawError::GuideScopeExceeded {
                    guide_name: guide.name.clone(),
                    license_type: "Provincial".to_string(),
                    zone: zone.name_en.clone(),
                });
            }
        }
        GuideLicenseCategory::Community => {
            // Community guides can only operate in their community
            if let Some(guide_province) = &guide.province
                && guide_province != &zone.province
            {
                return Err(TourismLawError::GuideScopeExceeded {
                    guide_name: guide.name.clone(),
                    license_type: "Community".to_string(),
                    zone: zone.name_en.clone(),
                });
            }
        }
        GuideLicenseCategory::National | GuideLicenseCategory::Specialized { .. } => {
            // National and specialized guides can operate anywhere
        }
    }

    Ok(())
}

// ============================================================================
// Tourism Zone Validation (ການກວດສອບເຂດທ່ອງທ່ຽວ)
// ============================================================================

/// Validate tourism zone access (ກວດສອບການເຂົ້າເຖິງເຂດທ່ອງທ່ຽວ)
///
/// Tourism Law 2013, Article 44-46: Access to certain zones requires permits.
///
/// # Arguments
/// * `zone` - Tourism zone
/// * `has_permit` - Whether visitor has required permit
/// * `is_foreign_tourist` - Whether visitor is a foreign tourist
///
/// # Returns
/// * `Ok(())` if access is authorized
/// * `Err(TourismLawError)` if access is prohibited or permit is required
pub fn validate_zone_access(
    zone: &TourismZone,
    has_permit: bool,
    is_foreign_tourist: bool,
) -> Result<()> {
    // Check if zone is open to foreign tourists
    if is_foreign_tourist && !zone.zone_type.open_to_foreign_tourists() {
        return Err(TourismLawError::ProhibitedZone {
            zone_name: zone.name_en.clone(),
            reason: "Zone not open to foreign tourists".to_string(),
        });
    }

    // Check if special permit is required
    if zone.permit_required && !has_permit {
        return Err(TourismLawError::ZonePermitRequired {
            zone_name: zone.name_en.clone(),
        });
    }

    // Security zones are prohibited for tourists
    if matches!(zone.zone_type, TourismZoneType::SecurityZone) {
        return Err(TourismLawError::ProhibitedZone {
            zone_name: zone.name_en.clone(),
            reason: "Security zone - access prohibited".to_string(),
        });
    }

    Ok(())
}

/// Validate carrying capacity (ກວດສອບຄວາມສາມາດຮອງຮັບ)
///
/// Tourism Law 2013, Article 46: Tourism zones have carrying capacity limits.
///
/// # Arguments
/// * `zone` - Tourism zone
/// * `current_visitors` - Current number of visitors
///
/// # Returns
/// * `Ok(())` if within capacity
/// * `Err(TourismLawError)` if capacity is exceeded
pub fn validate_carrying_capacity(zone: &TourismZone, current_visitors: u32) -> Result<()> {
    if let Some(max_capacity) = zone.carrying_capacity
        && current_visitors > max_capacity
    {
        return Err(TourismLawError::CarryingCapacityExceeded {
            zone_name: zone.name_en.clone(),
            actual: current_visitors,
            max_capacity,
        });
    }

    Ok(())
}

// ============================================================================
// Tourist Rights Validation (ການກວດສອບສິດນັກທ່ອງທ່ຽວ)
// ============================================================================

/// Validate tourist complaint response (ກວດສອບການຕອບຄຳຮ້ອງທຸກນັກທ່ອງທ່ຽວ)
///
/// Tourism Law 2013, Article 50: Complaints must be addressed within 15 days.
///
/// # Arguments
/// * `complaint` - Tourist complaint
///
/// # Returns
/// * `Ok(())` if complaint is being addressed properly
/// * `Err(TourismLawError)` if complaint response is overdue
pub fn validate_complaint_response(complaint: &TouristComplaint) -> Result<()> {
    let now = Utc::now();
    let days_since_filing = (now - complaint.filing_date).num_days();

    match &complaint.status {
        ComplaintStatus::PendingResponse | ComplaintStatus::Filed { .. } => {
            if days_since_filing > COMPLAINT_RESPONSE_DEADLINE_DAYS as i64 {
                return Err(TourismLawError::ComplaintNotAddressed {
                    complaint_id: complaint.complaint_id.clone(),
                    days: days_since_filing as u32,
                });
            }
        }
        _ => {}
    }

    Ok(())
}

/// Validate travel insurance (ກວດສອບປະກັນການເດີນທາງ)
///
/// Tourism Law 2013, Article 51: Travel insurance may be required for certain tours.
///
/// # Arguments
/// * `insurance` - Travel insurance (if any)
/// * `tour_name` - Name of the tour
/// * `requires_insurance` - Whether insurance is required
///
/// # Returns
/// * `Ok(())` if insurance requirements are met
/// * `Err(TourismLawError)` if required insurance is missing
pub fn validate_travel_insurance(
    insurance: Option<&TravelInsurance>,
    tour_name: &str,
    requires_insurance: bool,
) -> Result<()> {
    if requires_insurance {
        match insurance {
            Some(ins) => {
                if !ins.is_valid() {
                    return Err(TourismLawError::MissingTravelInsurance {
                        tour_name: tour_name.to_string(),
                    });
                }
            }
            None => {
                return Err(TourismLawError::MissingTravelInsurance {
                    tour_name: tour_name.to_string(),
                });
            }
        }
    }

    Ok(())
}

// ============================================================================
// ASEAN Integration Validation (ການກວດສອບການເຊື່ອມໂຍງອາຊຽນ)
// ============================================================================

/// Validate ASEAN MRA certification (ກວດສອບໃບຢັ້ງຢືນ MRA ອາຊຽນ)
///
/// Tourism Law 2013, Article 65: ASEAN tourism professionals must comply with MRA.
///
/// # Arguments
/// * `certification` - ASEAN MRA certification
///
/// # Returns
/// * `Ok(())` if certification is valid
/// * `Err(TourismLawError)` if certification is invalid or expired
pub fn validate_asean_mra_certification(certification: &AseanMraCertification) -> Result<()> {
    if !certification.is_valid() {
        return Err(TourismLawError::AseanMraNonCompliance {
            professional_type: certification.professional_type.description_en().to_string(),
            issue: "Certification expired".to_string(),
        });
    }

    Ok(())
}

/// Validate visa for tourism (ກວດສອບວີຊາສຳລັບການທ່ອງທ່ຽວ)
///
/// Tourism Law 2013, Article 62: Foreign tourists must have valid visas.
///
/// # Arguments
/// * `visa_type` - Type of visa
/// * `tourist_name` - Name of tourist
/// * `nationality` - Tourist nationality
///
/// # Returns
/// * `Ok(())` if visa is valid for tourism
/// * `Err(TourismLawError)` if visa is invalid for tourism
pub fn validate_tourism_visa(
    visa_type: &TourismVisaType,
    tourist_name: &str,
    _nationality: &str,
) -> Result<()> {
    match visa_type {
        TourismVisaType::TransitVisa => {
            return Err(TourismLawError::InvalidVisaForTourism {
                tourist_name: tourist_name.to_string(),
                visa_type: "Transit Visa".to_string(),
            });
        }
        _ => {
            // All other visa types are valid for tourism
        }
    }

    Ok(())
}

// ============================================================================
// Sustainable Tourism Validation (ການກວດສອບການທ່ອງທ່ຽວແບບຍືນຍົງ)
// ============================================================================

/// Validate community-based tourism project (ກວດສອບໂຄງການທ່ອງທ່ຽວໂດຍຊຸມຊົນ)
///
/// Tourism Law 2013, Article 56: CBT projects must meet certain requirements.
///
/// # Arguments
/// * `cbt` - Community-based tourism project
///
/// # Returns
/// * `Ok(())` if project meets requirements
/// * `Err(TourismLawError)` if requirements are not met
pub fn validate_cbt_project(cbt: &CommunityBasedTourism) -> Result<()> {
    // Check minimum community revenue share (at least 60%)
    if cbt.community_revenue_share_percent < 60.0 {
        return Err(TourismLawError::CommunityBasedTourismViolation {
            violation: format!(
                "Insufficient community revenue share: {:.1}% (minimum 60%)",
                cbt.community_revenue_share_percent
            ),
            village_name: cbt.village_name.clone(),
        });
    }

    // Check environmental measures
    if cbt.environmental_measures.is_empty() {
        return Err(TourismLawError::CommunityBasedTourismViolation {
            violation: "No environmental protection measures defined".to_string(),
            village_name: cbt.village_name.clone(),
        });
    }

    Ok(())
}

// ============================================================================
// Comprehensive Validation (ການກວດສອບແບບຄົບຖ້ວນ)
// ============================================================================

/// Perform comprehensive validation of a tourism enterprise
///
/// # Arguments
/// * `enterprise` - Tourism enterprise to validate
///
/// # Returns
/// * `Ok(Vec<String>)` - List of warnings (non-critical issues)
/// * `Err(TourismLawError)` - Critical violation found
pub fn validate_enterprise_comprehensive(enterprise: &TourismEnterprise) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Critical validations
    validate_enterprise_license(enterprise)?;
    validate_foreign_ownership(enterprise)?;

    // Non-critical checks
    if enterprise.contact_phone.is_none() {
        warnings.push("Enterprise has no contact phone listed".to_string());
    }

    if enterprise.email.is_none() {
        warnings.push("Enterprise has no email listed".to_string());
    }

    // Check if license expires soon (within 90 days)
    if enterprise.needs_renewal_soon() {
        warnings.push(format!(
            "License expires in {} days - renewal recommended",
            enterprise.days_until_expiry()
        ));
    }

    Ok(warnings)
}

/// Perform comprehensive validation of an accommodation
///
/// # Arguments
/// * `accommodation` - Accommodation to validate
///
/// # Returns
/// * `Ok(Vec<String>)` - List of warnings (non-critical issues)
/// * `Err(TourismLawError)` - Critical violation found
pub fn validate_accommodation_comprehensive(accommodation: &Accommodation) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Critical validations
    validate_enterprise_license(&accommodation.enterprise)?;
    validate_foreign_ownership(&accommodation.enterprise)?;

    // Classification validation for applicable types
    if accommodation.accommodation_type.star_rating_applicable() {
        validate_hotel_classification(accommodation)?;
        validate_hotel_facilities(accommodation)?;
    }

    // Non-critical checks
    if accommodation.check_in_time.is_none() {
        warnings.push("Check-in time not specified".to_string());
    }

    if accommodation.check_out_time.is_none() {
        warnings.push("Check-out time not specified".to_string());
    }

    Ok(warnings)
}

/// Perform comprehensive validation of a tour guide
///
/// # Arguments
/// * `guide` - Tour guide to validate
///
/// # Returns
/// * `Ok(Vec<String>)` - List of warnings (non-critical issues)
/// * `Err(TourismLawError)` - Critical violation found
pub fn validate_guide_comprehensive(guide: &TourGuide) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Critical validations
    validate_guide_license(guide)?;
    validate_guide_language(guide)?;
    validate_guide_training(guide)?;

    // Non-critical checks
    if guide.affiliated_company.is_none() {
        warnings.push("Guide is not affiliated with any company".to_string());
    }

    // Check if license expires soon
    let days_until_expiry = (guide.license_expiry_date - Utc::now()).num_days();
    if days_until_expiry > 0 && days_until_expiry <= 60 {
        warnings.push(format!(
            "License expires in {} days - renewal recommended",
            days_until_expiry
        ));
    }

    Ok(warnings)
}

/// Validate tourism statistics report submission
///
/// Tourism Law 2013, Article 64: Enterprises must submit statistics reports.
///
/// # Arguments
/// * `report` - Statistics report
/// * `due_date` - Due date for submission
///
/// # Returns
/// * `Ok(())` if report is submitted on time
/// * `Err(TourismLawError)` if report is overdue
pub fn validate_statistics_submission(
    enterprise_name: &str,
    reporting_period: &str,
    submitted: bool,
) -> Result<()> {
    if !submitted {
        return Err(TourismLawError::StatisticsNotReported {
            enterprise_name: enterprise_name.to_string(),
            period: reporting_period.to_string(),
        });
    }

    Ok(())
}

/// Validate entrance fee compliance
///
/// Tourism Law 2013, Article 61: Entrance fees must not exceed set limits.
///
/// # Arguments
/// * `attraction_name` - Name of attraction
/// * `charged_fee` - Fee charged
/// * `max_fee` - Maximum allowed fee
///
/// # Returns
/// * `Ok(())` if fee is within limits
/// * `Err(TourismLawError)` if fee exceeds limit
pub fn validate_entrance_fee(attraction_name: &str, charged_fee: u64, max_fee: u64) -> Result<()> {
    if charged_fee > max_fee {
        return Err(TourismLawError::EntranceFeeOvercharge {
            attraction_name: attraction_name.to_string(),
            charged_lak: charged_fee,
            max_lak: max_fee,
        });
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn create_valid_enterprise() -> TourismEnterprise {
        TourismEnterprise {
            name_lao: "ວິສາຫະກິດທົດສອບ".to_string(),
            name_en: "Test Enterprise".to_string(),
            category: TourismEnterpriseCategory::TourOperatorInbound,
            registration_number: "REG-001".to_string(),
            tourism_license_number: "TL-001".to_string(),
            license_status: LicenseStatus::Active,
            license_issue_date: Utc::now(),
            license_expiry_date: Utc::now() + Duration::days(365),
            province: "Vientiane".to_string(),
            district: "Chanthabouly".to_string(),
            address: "123 Test Street".to_string(),
            contact_phone: Some("021-123456".to_string()),
            email: Some("test@example.com".to_string()),
            website: None,
            foreign_ownership_percent: 30.0,
            registered_capital_lak: 100_000_000,
            employee_count: 10,
            lao_employee_count: 10,
        }
    }

    fn create_valid_guide() -> TourGuide {
        TourGuide {
            name: "Test Guide".to_string(),
            name_lao: Some("ໄກດ໌ທົດສອບ".to_string()),
            license_number: "GL-001".to_string(),
            license_category: GuideLicenseCategory::National,
            license_status: LicenseStatus::Active,
            license_issue_date: Utc::now(),
            license_expiry_date: Utc::now() + Duration::days(365),
            language_skills: vec![
                LanguageSkill {
                    language: "Lao".to_string(),
                    proficiency: LanguageProficiency::Native,
                    certification: None,
                },
                LanguageSkill {
                    language: "English".to_string(),
                    proficiency: LanguageProficiency::Advanced,
                    certification: Some("TOEFL".to_string()),
                },
            ],
            training_certifications: vec!["Tourism Guide Certificate".to_string()],
            training_hours: 150,
            province: Some("Vientiane".to_string()),
            affiliated_company: Some("Test Tours".to_string()),
            years_of_experience: 5,
            nationality: "Lao".to_string(),
            date_of_birth: Utc::now() - Duration::days(365 * 30),
            id_card_number: "ID-123456".to_string(),
        }
    }

    #[test]
    fn test_valid_enterprise_license() {
        let enterprise = create_valid_enterprise();
        assert!(validate_enterprise_license(&enterprise).is_ok());
    }

    #[test]
    fn test_expired_enterprise_license() {
        let mut enterprise = create_valid_enterprise();
        enterprise.license_expiry_date = Utc::now() - Duration::days(1);

        let result = validate_enterprise_license(&enterprise);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TourismLawError::EnterpriseLicenseExpired { .. }
        ));
    }

    #[test]
    fn test_foreign_ownership_valid() {
        let enterprise = create_valid_enterprise();
        assert!(validate_foreign_ownership(&enterprise).is_ok());
    }

    #[test]
    fn test_foreign_ownership_exceeded() {
        let mut enterprise = create_valid_enterprise();
        enterprise.foreign_ownership_percent = 60.0; // Exceeds 49% limit

        let result = validate_foreign_ownership(&enterprise);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TourismLawError::ForeignOwnershipLimitExceeded { .. }
        ));
    }

    #[test]
    fn test_domestic_operator_foreign_ownership() {
        let mut enterprise = create_valid_enterprise();
        enterprise.category = TourismEnterpriseCategory::TourOperatorDomestic;
        enterprise.foreign_ownership_percent = 10.0; // Any foreign ownership not allowed

        let result = validate_foreign_ownership(&enterprise);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TourismLawError::ForeignInvestmentNotPermitted { .. }
        ));
    }

    #[test]
    fn test_valid_guide_license() {
        let guide = create_valid_guide();
        assert!(validate_guide_license(&guide).is_ok());
    }

    #[test]
    fn test_guide_language_valid() {
        let guide = create_valid_guide();
        assert!(validate_guide_language(&guide).is_ok());
    }

    #[test]
    fn test_guide_missing_lao() {
        let mut guide = create_valid_guide();
        guide.language_skills = vec![LanguageSkill {
            language: "English".to_string(),
            proficiency: LanguageProficiency::Advanced,
            certification: None,
        }];

        let result = validate_guide_language(&guide);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TourismLawError::InsufficientLanguageProficiency { .. }
        ));
    }

    #[test]
    fn test_guide_training_valid() {
        let guide = create_valid_guide();
        assert!(validate_guide_training(&guide).is_ok());
    }

    #[test]
    fn test_guide_training_insufficient() {
        let mut guide = create_valid_guide();
        guide.training_hours = 50; // Below minimum

        let result = validate_guide_training(&guide);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TourismLawError::MissingTrainingCertification { .. }
        ));
    }

    #[test]
    fn test_zone_access_security_zone() {
        let zone = TourismZone {
            name_lao: "ເຂດທົດສອບ".to_string(),
            name_en: "Test Zone".to_string(),
            zone_type: TourismZoneType::SecurityZone,
            province: "Vientiane".to_string(),
            district: "Test".to_string(),
            area_hectares: None,
            carrying_capacity: None,
            entrance_fee_lao_lak: None,
            entrance_fee_foreign_lak: None,
            permit_required: true,
            managing_authority: "Government".to_string(),
            unesco_world_heritage: false,
            description: None,
        };

        let result = validate_zone_access(&zone, true, false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TourismLawError::ProhibitedZone { .. }
        ));
    }

    #[test]
    fn test_carrying_capacity_exceeded() {
        let zone = TourismZone {
            name_lao: "ເຂດທົດສອບ".to_string(),
            name_en: "Test Zone".to_string(),
            zone_type: TourismZoneType::NationalTourismZone,
            province: "Vientiane".to_string(),
            district: "Test".to_string(),
            area_hectares: Some(100.0),
            carrying_capacity: Some(100),
            entrance_fee_lao_lak: None,
            entrance_fee_foreign_lak: None,
            permit_required: false,
            managing_authority: "MOICT".to_string(),
            unesco_world_heritage: false,
            description: None,
        };

        let result = validate_carrying_capacity(&zone, 150);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TourismLawError::CarryingCapacityExceeded { .. }
        ));
    }

    #[test]
    fn test_travel_insurance_required() {
        let result = validate_travel_insurance(None, "Adventure Tour", true);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TourismLawError::MissingTravelInsurance { .. }
        ));
    }

    #[test]
    fn test_valid_travel_insurance() {
        let insurance = TravelInsurance {
            policy_number: "POL-001".to_string(),
            insurance_company: "Test Insurance".to_string(),
            coverage_amount_usd: 50000,
            start_date: Utc::now() - Duration::days(10),
            end_date: Utc::now() + Duration::days(20),
            covers_medical: true,
            covers_evacuation: true,
            covers_cancellation: false,
        };

        assert!(validate_travel_insurance(Some(&insurance), "Tour", true).is_ok());
    }

    #[test]
    fn test_cbt_project_valid() {
        let cbt = CommunityBasedTourism {
            project_name: "Test CBT".to_string(),
            village_name: "Test Village".to_string(),
            province: "Vientiane".to_string(),
            district: "Test".to_string(),
            registration_number: Some("CBT-001".to_string()),
            participating_households: 20,
            activities: vec!["Trekking".to_string()],
            community_revenue_share_percent: 70.0,
            environmental_measures: vec!["Waste management".to_string()],
            cultural_preservation: vec!["Traditional crafts".to_string()],
            max_visitors_per_day: Some(50),
        };

        assert!(validate_cbt_project(&cbt).is_ok());
    }

    #[test]
    fn test_cbt_insufficient_revenue_share() {
        let cbt = CommunityBasedTourism {
            project_name: "Test CBT".to_string(),
            village_name: "Test Village".to_string(),
            province: "Vientiane".to_string(),
            district: "Test".to_string(),
            registration_number: None,
            participating_households: 20,
            activities: vec!["Trekking".to_string()],
            community_revenue_share_percent: 40.0, // Below 60%
            environmental_measures: vec!["Waste management".to_string()],
            cultural_preservation: vec![],
            max_visitors_per_day: None,
        };

        let result = validate_cbt_project(&cbt);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TourismLawError::CommunityBasedTourismViolation { .. }
        ));
    }

    #[test]
    fn test_entrance_fee_valid() {
        assert!(validate_entrance_fee("Test Attraction", 50000, 100000).is_ok());
    }

    #[test]
    fn test_entrance_fee_overcharge() {
        let result = validate_entrance_fee("Test Attraction", 150000, 100000);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TourismLawError::EntranceFeeOvercharge { .. }
        ));
    }

    #[test]
    fn test_enterprise_comprehensive_validation() {
        let enterprise = create_valid_enterprise();
        let result = validate_enterprise_comprehensive(&enterprise);
        assert!(result.is_ok());
    }

    #[test]
    fn test_guide_comprehensive_validation() {
        let guide = create_valid_guide();
        let result = validate_guide_comprehensive(&guide);
        assert!(result.is_ok());
    }
}
