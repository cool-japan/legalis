//! Consumer law validation functions
//!
//! Validators for ACL compliance, product safety, and ACCC enforcement.

use chrono::{Datelike, NaiveDate, Weekday};

use super::error::{ConsumerLawError, Result};
use super::types::*;

// =============================================================================
// Product Safety Validation
// =============================================================================

/// Product safety assessment result
#[derive(Debug, Clone)]
pub struct ProductSafetyAssessment {
    /// Product name
    pub product_name: String,
    /// Safety status
    pub status: ProductSafetyStatus,
    /// Compliant with safety standards
    pub standards_compliant: bool,
    /// Issues identified
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Assess product safety compliance
pub fn assess_product_safety(
    product_name: &str,
    product_category: ProductCategory,
    has_safety_standard: bool,
    meets_safety_standard: bool,
    has_known_hazard: bool,
    hazard_description: Option<&str>,
    injury_reports: u32,
) -> ProductSafetyAssessment {
    let mut issues = Vec::new();
    let mut recommendations = Vec::new();
    let mut legal_references = vec![
        "ACL Part 3-3 (Product Safety)".to_string(),
        "Competition and Consumer Act 2010 s.132A-132D".to_string(),
    ];

    let mut status = ProductSafetyStatus::Safe;

    // Check safety standard compliance
    if has_safety_standard && !meets_safety_standard {
        issues.push(format!(
            "Product does not comply with mandatory safety standard for {:?}",
            product_category
        ));
        legal_references.push("ACL s.106 - Compliance with safety standards".to_string());
        recommendations.push("Modify product to comply with safety standard".to_string());
        status = ProductSafetyStatus::UnderInvestigation;
    }

    // Check known hazards
    if has_known_hazard {
        if let Some(hazard) = hazard_description {
            issues.push(format!("Known hazard identified: {}", hazard));
        }
        recommendations.push("Consider voluntary recall".to_string());
        status = ProductSafetyStatus::UnderInvestigation;
    }

    // Check injury reports
    if injury_reports > 0 {
        issues.push(format!(
            "{} injury report(s) received - mandatory reporting may apply",
            injury_reports
        ));
        legal_references.push("ACL s.131 - Mandatory reporting of serious injuries".to_string());

        if injury_reports >= 3 {
            recommendations.push("Immediate recall assessment required".to_string());
            status = ProductSafetyStatus::VoluntaryRecall;
        }
    }

    // High-risk categories
    if matches!(
        product_category,
        ProductCategory::ChildrensToys
            | ProductCategory::BabyProducts
            | ProductCategory::SwimmingAids
    ) && has_known_hazard
    {
        recommendations
            .push("High-risk product category - priority safety assessment required".to_string());
    }

    ProductSafetyAssessment {
        product_name: product_name.to_string(),
        status,
        standards_compliant: !has_safety_standard || meets_safety_standard,
        issues,
        recommendations,
        legal_references,
    }
}

/// Validate mandatory injury report timing
pub fn validate_injury_report_timing(
    incident_date: NaiveDate,
    report_date: NaiveDate,
) -> Result<()> {
    // Must report within 2 days of becoming aware
    let days_elapsed = (report_date - incident_date).num_days();

    if days_elapsed > 2 {
        return Err(ConsumerLawError::InjuryReportNotMade {
            product_name: "Unknown".to_string(),
            injury_type: "Unknown".to_string(),
        });
    }

    Ok(())
}

// =============================================================================
// ACCC Enforcement
// =============================================================================

/// Calculate penalty amount
pub fn calculate_penalty(
    contravention: AclContravention,
    recipient_type: RecipientType,
    aggravating_factors: u32,
) -> f64 {
    let base_units = contravention.base_penalty_units();
    let multiplier = recipient_type.penalty_multiplier();
    let unit_value = AclContravention::penalty_unit_value();

    // Base penalty
    let mut penalty = base_units as f64 * multiplier as f64 * unit_value;

    // Aggravating factors can increase penalty
    if aggravating_factors > 0 {
        let aggravation_multiplier = 1.0 + (aggravating_factors as f64 * 0.1);
        penalty *= aggravation_multiplier;
    }

    penalty
}

/// Infringement notice assessment
#[derive(Debug, Clone)]
pub struct InfringementNoticeAssessment {
    /// Contravention type
    pub contravention: AclContravention,
    /// Infringement notice appropriate
    pub infringement_notice_appropriate: bool,
    /// Suggested penalty
    pub suggested_penalty: f64,
    /// Reasoning
    pub reasoning: String,
}

/// Assess whether infringement notice is appropriate
pub fn assess_infringement_notice(
    contravention: AclContravention,
    recipient_type: RecipientType,
    first_offence: bool,
    minor_contravention: bool,
    cooperation: bool,
) -> InfringementNoticeAssessment {
    let mut appropriate = true;
    let mut reasons = Vec::new();

    // Infringement notices suitable for:
    // - First offences
    // - Minor contraventions
    // - When party shows cooperation

    if !first_offence && !minor_contravention {
        appropriate = false;
        reasons.push("Repeat offence of significant nature - court action preferred");
    }

    // Certain contraventions may not be suitable for infringement notices
    if matches!(
        contravention,
        AclContravention::UnconscionableGeneral | AclContravention::UnconscionableTradeCommerce
    ) {
        appropriate = false;
        reasons.push("Unconscionable conduct - requires judicial assessment");
    }

    // Calculate infringement notice penalty (typically 1/5 of max court penalty)
    let max_penalty = calculate_penalty(contravention, recipient_type, 0);
    let suggested_penalty = max_penalty / 5.0;

    if appropriate {
        reasons.push("Infringement notice appropriate for this contravention");
        if cooperation {
            reasons.push("Cooperation may support reduced penalty");
        }
    }

    InfringementNoticeAssessment {
        contravention,
        infringement_notice_appropriate: appropriate,
        suggested_penalty,
        reasoning: reasons.join(". "),
    }
}

// =============================================================================
// Unsolicited Consumer Agreement Validation
// =============================================================================

/// Unsolicited agreement compliance result
#[derive(Debug, Clone)]
pub struct UnsolicitedAgreementComplianceResult {
    /// Compliant
    pub compliant: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Consumer can terminate
    pub consumer_can_terminate: bool,
    /// Agreement void
    pub agreement_void: bool,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Validate unsolicited consumer agreement compliance
#[allow(clippy::too_many_arguments)] // ACL compliance check requires all these factors
pub fn validate_unsolicited_agreement(
    agreement_type: UnsolicitedAgreementType,
    agreement_value: f64,
    dealer_identified: bool,
    supplier_identified: bool,
    purpose_disclosed: bool,
    written_agreement_provided: bool,
    cooling_off_notice_provided: bool,
    contact_time: Option<(u32, Weekday)>, // (hour, day of week)
) -> UnsolicitedAgreementComplianceResult {
    let mut issues = Vec::new();
    let mut legal_references = vec![
        "ACL Part 3-2 Division 2".to_string(),
        format!("Agreement type: {:?}", agreement_type),
    ];
    let mut agreement_void = false;

    // Check value threshold - ACL only applies to agreements > $100
    if agreement_value <= 100.0 {
        return UnsolicitedAgreementComplianceResult {
            compliant: true,
            issues: vec![
                "Agreement value <= $100 - ACL unsolicited agreement provisions do not apply"
                    .to_string(),
            ],
            consumer_can_terminate: false,
            agreement_void: false,
            legal_references: vec!["ACL s.69(1)(b) - threshold not met".to_string()],
        };
    }

    // Check identification requirements (s.74)
    if !dealer_identified {
        issues.push("Dealer did not clearly identify themselves".to_string());
        legal_references.push("ACL s.74 - Identification requirements".to_string());
    }

    if !supplier_identified {
        issues.push("Supplier not identified".to_string());
        legal_references.push("ACL s.74 - Supplier identification".to_string());
    }

    // Check purpose disclosure (s.74)
    if !purpose_disclosed {
        issues.push("Purpose of contact not disclosed at start".to_string());
        legal_references.push("ACL s.74 - Purpose disclosure".to_string());
    }

    // Check written agreement (s.79)
    if !written_agreement_provided {
        issues.push("Written agreement not provided".to_string());
        agreement_void = true;
        legal_references.push("ACL s.79 - Agreement must be in writing".to_string());
    }

    // Check cooling-off notice (s.76)
    if !cooling_off_notice_provided {
        issues.push("Cooling-off rights notice not provided".to_string());
        agreement_void = true;
        legal_references.push("ACL s.76 - Cooling-off notice required".to_string());
    }

    // Check permitted contact times (s.73)
    if let Some((hour, day)) = contact_time {
        let permitted = match day {
            Weekday::Sun => false,
            Weekday::Sat => (9..17).contains(&hour),
            _ => (9..18).contains(&hour),
        };

        if !permitted {
            issues.push("Contact made outside permitted hours".to_string());
            legal_references
                .push("ACL s.73 - Permitted hours: 9am-6pm Mon-Fri, 9am-5pm Sat".to_string());
        }
    }

    UnsolicitedAgreementComplianceResult {
        compliant: issues.is_empty(),
        issues,
        consumer_can_terminate: true, // Always within cooling-off period
        agreement_void,
        legal_references,
    }
}

/// Calculate cooling-off end date (10 business days)
pub fn calculate_cooling_off_end_date(agreement_date: NaiveDate) -> NaiveDate {
    let mut business_days = 0;
    let mut current = agreement_date;

    while business_days < 10 {
        current = current.succ_opt().unwrap_or(current);
        let weekday = current.weekday();
        if weekday != Weekday::Sat && weekday != Weekday::Sun {
            business_days += 1;
        }
    }

    current
}

// =============================================================================
// Country of Origin Validation
// =============================================================================

/// Country of origin claim assessment
#[derive(Debug, Clone)]
pub struct CountryOfOriginAssessment {
    /// Claim valid
    pub valid: bool,
    /// Safe harbour available
    pub safe_harbour_available: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Requirements for safe harbour
    pub safe_harbour_requirements: Vec<String>,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Validate country of origin claim
pub fn validate_country_of_origin_claim(
    claim: &CountryOfOriginClaimDetails,
) -> CountryOfOriginAssessment {
    let mut issues = Vec::new();
    let mut safe_harbour_requirements = Vec::new();
    let legal_references = vec![
        "ACL Part 5-3 (Country of Origin)".to_string(),
        "Competition and Consumer (Country of Origin) Information Standard 2016".to_string(),
    ];

    let safe_harbour_available = match claim.claim_type {
        CountryOfOriginClaim::MadeIn => {
            safe_harbour_requirements.push("Substantially transformed in Australia".to_string());
            safe_harbour_requirements.push("50% or more production costs in Australia".to_string());

            if claim.country == "Australia" {
                claim.made_in_australia_safe_harbour()
            } else {
                // For other countries, need substantial transformation
                claim.substantially_transformed
            }
        }
        CountryOfOriginClaim::ProductOf => {
            safe_harbour_requirements.push("100% Australian ingredients/components".to_string());
            safe_harbour_requirements.push("Substantially transformed in Australia".to_string());

            if claim.country == "Australia" {
                claim.product_of_australia_safe_harbour()
            } else {
                false
            }
        }
        CountryOfOriginClaim::GrownIn => {
            safe_harbour_requirements.push("100% grown in specified country".to_string());

            if claim.country == "Australia" {
                claim.grown_in_australia_safe_harbour()
            } else {
                claim.australian_ingredient_percentage == Some(100.0)
            }
        }
        CountryOfOriginClaim::PackedIn => {
            // Packed in has lower threshold
            true
        }
        CountryOfOriginClaim::AustralianMadeLogo => {
            safe_harbour_requirements.push("Licensed by Australian Made Campaign".to_string());
            safe_harbour_requirements.push("Meet Made in Australia requirements".to_string());
            claim.made_in_australia_safe_harbour()
        }
        CountryOfOriginClaim::BarChart => {
            // Bar chart claims for food must show percentage
            if claim.is_food && claim.australian_ingredient_percentage.is_none() {
                issues.push(
                    "Bar chart for food must show percentage of Australian ingredients".to_string(),
                );
                false
            } else {
                true
            }
        }
    };

    // Check if claim is valid
    if !safe_harbour_available {
        match claim.claim_type {
            CountryOfOriginClaim::MadeIn => {
                if !claim.substantially_transformed {
                    issues.push("Product not substantially transformed in Australia".to_string());
                }
                if !claim.fifty_percent_production_costs {
                    issues.push(
                        "Less than 50% of production costs incurred in Australia".to_string(),
                    );
                }
            }
            CountryOfOriginClaim::ProductOf => {
                if claim.australian_ingredient_percentage != Some(100.0) {
                    issues.push("Product of claim requires 100% Australian origin".to_string());
                }
            }
            _ => {}
        }
    }

    CountryOfOriginAssessment {
        valid: issues.is_empty(),
        safe_harbour_available,
        issues,
        safe_harbour_requirements,
        legal_references,
    }
}

// =============================================================================
// Lay-by Agreement Validation
// =============================================================================

/// Lay-by agreement compliance result
#[derive(Debug, Clone)]
pub struct LayByComplianceResult {
    /// Compliant
    pub compliant: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Termination fee valid
    pub termination_fee_valid: bool,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Validate lay-by agreement
pub fn validate_layby_agreement(
    total_price: f64,
    termination_fee: Option<f64>,
    terms_clearly_stated: bool,
    completion_date_stated: bool,
) -> LayByComplianceResult {
    let mut issues = Vec::new();
    let legal_references = vec![
        "ACL s.96 - Requirements for lay-by agreements".to_string(),
        "ACL s.97-99 - Termination of lay-by agreements".to_string(),
    ];

    // Check terms clearly stated
    if !terms_clearly_stated {
        issues.push("Lay-by terms not clearly stated in writing".to_string());
    }

    // Check completion date
    if !completion_date_stated {
        issues.push("Expected completion date not stated".to_string());
    }

    // Check termination fee reasonableness
    let termination_fee_valid = if let Some(fee) = termination_fee {
        // Fee should be reasonable - generally 10-20% of price is considered reasonable
        let max_reasonable_fee = total_price * 0.20;
        if fee > max_reasonable_fee {
            issues.push(format!(
                "Termination fee ${:.2} may exceed reasonable amount (20% = ${:.2})",
                fee, max_reasonable_fee
            ));
            false
        } else {
            true
        }
    } else {
        true
    };

    LayByComplianceResult {
        compliant: issues.is_empty(),
        issues,
        termination_fee_valid,
        legal_references,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_safety_compliant() {
        let result =
            assess_product_safety("Widget", ProductCategory::Other, true, true, false, None, 0);
        assert_eq!(result.status, ProductSafetyStatus::Safe);
        assert!(result.issues.is_empty());
    }

    #[test]
    fn test_product_safety_non_compliant() {
        let result = assess_product_safety(
            "Dangerous Toy",
            ProductCategory::ChildrensToys,
            true,
            false,
            true,
            Some("Small parts hazard"),
            2,
        );
        assert_ne!(result.status, ProductSafetyStatus::Safe);
        assert!(!result.issues.is_empty());
    }

    #[test]
    fn test_calculate_penalty_individual() {
        let penalty = calculate_penalty(
            AclContravention::FalseRepresentations,
            RecipientType::Individual,
            0,
        );
        // 2500 units * 1 * $313 = $782,500
        assert_eq!(penalty, 782500.0);
    }

    #[test]
    fn test_calculate_penalty_body_corporate() {
        let penalty = calculate_penalty(
            AclContravention::FalseRepresentations,
            RecipientType::BodyCorporate,
            0,
        );
        // 2500 units * 5 * $313 = $3,912,500
        assert_eq!(penalty, 3912500.0);
    }

    #[test]
    fn test_infringement_notice_appropriate() {
        let result = assess_infringement_notice(
            AclContravention::FalseRepresentations,
            RecipientType::Individual,
            true,
            true,
            true,
        );
        assert!(result.infringement_notice_appropriate);
    }

    #[test]
    fn test_infringement_notice_not_appropriate_unconscionable() {
        let result = assess_infringement_notice(
            AclContravention::UnconscionableTradeCommerce,
            RecipientType::BodyCorporate,
            true,
            true,
            true,
        );
        assert!(!result.infringement_notice_appropriate);
    }

    #[test]
    fn test_unsolicited_agreement_compliant() {
        let result = validate_unsolicited_agreement(
            UnsolicitedAgreementType::DoorToDoor,
            500.0,
            true,
            true,
            true,
            true,
            true,
            Some((10, Weekday::Mon)),
        );
        assert!(result.compliant);
    }

    #[test]
    fn test_unsolicited_agreement_invalid_time() {
        let result = validate_unsolicited_agreement(
            UnsolicitedAgreementType::Telemarketing,
            500.0,
            true,
            true,
            true,
            true,
            true,
            Some((20, Weekday::Mon)), // 8pm - outside permitted hours
        );
        assert!(!result.compliant);
    }

    #[test]
    fn test_cooling_off_calculation() {
        let agreement_date = NaiveDate::from_ymd_opt(2026, 1, 5).expect("valid date"); // Monday
        let end_date = calculate_cooling_off_end_date(agreement_date);

        // 10 business days from Monday Jan 5
        // Week 1: Tue-Fri (4 days) = 4
        // Week 2: Mon-Fri (5 days) = 9
        // Week 3: Mon (1 day) = 10
        // End date should be Mon Jan 19
        assert_eq!(
            end_date,
            NaiveDate::from_ymd_opt(2026, 1, 19).expect("valid date")
        );
    }

    #[test]
    fn test_country_of_origin_made_in_australia() {
        let claim = CountryOfOriginClaimDetails {
            claim_type: CountryOfOriginClaim::MadeIn,
            country: "Australia".to_string(),
            product: "Widget".to_string(),
            is_food: false,
            australian_ingredient_percentage: None,
            substantially_transformed: true,
            fifty_percent_production_costs: true,
            last_transformation_in_australia: true,
        };
        let result = validate_country_of_origin_claim(&claim);
        assert!(result.valid);
        assert!(result.safe_harbour_available);
    }

    #[test]
    fn test_country_of_origin_invalid_claim() {
        let claim = CountryOfOriginClaimDetails {
            claim_type: CountryOfOriginClaim::MadeIn,
            country: "Australia".to_string(),
            product: "Imported Widget".to_string(),
            is_food: false,
            australian_ingredient_percentage: None,
            substantially_transformed: false,
            fifty_percent_production_costs: false,
            last_transformation_in_australia: false,
        };
        let result = validate_country_of_origin_claim(&claim);
        assert!(!result.valid);
        assert!(!result.safe_harbour_available);
    }

    #[test]
    fn test_layby_valid() {
        let result = validate_layby_agreement(1000.0, Some(100.0), true, true);
        assert!(result.compliant);
        assert!(result.termination_fee_valid);
    }

    #[test]
    fn test_layby_excessive_fee() {
        let result = validate_layby_agreement(1000.0, Some(500.0), true, true);
        assert!(!result.compliant);
        assert!(!result.termination_fee_valid);
    }
}
