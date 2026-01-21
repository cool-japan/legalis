//! Validation functions for German commercial law partnerships (HGB)
//!
//! Multi-stage validation implementing HGB requirements for:
//! - General Partnership (OHG) - §105-160 HGB
//! - Limited Partnership (KG) - §161-177a HGB
//! - GmbH & Co. KG - Hybrid structure
//!
//! # Validation Strategy
//!
//! 1. **Field-level validation**: Individual field constraints
//! 2. **Relationship validation**: Cross-field dependencies
//! 3. **Type-specific validation**: Partnership-specific rules
//! 4. **Comprehensive error reporting**: Detailed context in errors

use crate::hgb::error::{HGBError, Result};
use crate::hgb::types::{FiscalYearEnd, GmbHCoKG, GmbHPartner, KG, LimitedPartner, OHG, Partner};

// =============================================================================
// Partnership Name Validation
// =============================================================================

/// Validate partnership name with required legal form suffix
///
/// # Legal Requirements
///
/// - **OHG**: Must include "OHG" or "offene Handelsgesellschaft" (§19 HGB)
/// - **KG**: Must include "KG" or "Kommanditgesellschaft" (§19 HGB)
/// - **GmbH & Co. KG**: Must include "GmbH & Co. KG" or similar variant
///
/// # Arguments
///
/// * `name` - Partnership name to validate
/// * `required_suffix` - Expected legal form suffix
///
/// # Examples
///
/// ```
/// use legalis_de::hgb::validator::validate_partnership_name;
///
/// // Valid OHG name
/// assert!(validate_partnership_name("Mustermann & Schmidt OHG", "OHG").is_ok());
///
/// // Invalid - missing suffix
/// assert!(validate_partnership_name("Mustermann & Schmidt", "OHG").is_err());
/// ```
pub fn validate_partnership_name(name: &str, required_suffix: &str) -> Result<()> {
    // Check for empty name
    if name.trim().is_empty() {
        return Err(HGBError::EmptyPartnershipName);
    }

    // Check minimum length (must be meaningful)
    if name.trim().len() < 3 {
        return Err(HGBError::PartnershipNameTooShort {
            name: name.to_string(),
        });
    }

    // Normalize for comparison (remove spaces, dots, case-insensitive)
    let normalized = name.replace([' ', '.', '&'], "").to_lowercase();

    // Check for required suffix
    let valid = match required_suffix {
        "OHG" => normalized.contains("ohg") || normalized.contains("offenehandelsgesellschaft"),
        "KG" => normalized.contains("kg") || normalized.contains("kommanditgesellschaft"),
        "GmbH & Co. KG" => {
            normalized.contains("gmbhcokg")
                || normalized.contains("gmbhundcokg")
                || (normalized.contains("gmbh") && normalized.contains("kg"))
        }
        _ => false,
    };

    if !valid {
        return Err(HGBError::MissingLegalFormSuffix {
            name: name.to_string(),
            required_suffix: required_suffix.to_string(),
        });
    }

    Ok(())
}

// =============================================================================
// Partner Validation
// =============================================================================

/// Validate individual partner
///
/// Checks that partner has:
/// - Non-empty name
/// - Non-empty address
/// - Valid contribution amounts (if specified)
pub fn validate_partner(partner: &Partner) -> Result<()> {
    // Validate name
    if partner.name.trim().is_empty() {
        return Err(HGBError::EmptyPartnerName);
    }

    // Validate address
    if partner.address.trim().is_empty() {
        return Err(HGBError::EmptyPartnerAddress);
    }

    // Validate contribution amounts if specified
    if let (Some(contribution), Some(paid)) = (&partner.contribution, &partner.contribution_paid)
        && paid.amount_cents > contribution.amount_cents
    {
        return Err(HGBError::ContributionPaidExceedsObligation {
            name: partner.name.clone(),
            paid: paid.to_euros(),
            contribution: contribution.to_euros(),
        });
    }

    Ok(())
}

/// Validate limited partner (Kommanditist)
///
/// # Legal Requirements (§171 HGB)
///
/// - Liability limit (Haftsumme) cannot be zero
/// - Liability limit must be at least €1
/// - Contribution paid cannot exceed liability limit
pub fn validate_limited_partner(partner: &LimitedPartner) -> Result<()> {
    // Validate name
    if partner.name.trim().is_empty() {
        return Err(HGBError::EmptyPartnerName);
    }

    // Validate address
    if partner.address.trim().is_empty() {
        return Err(HGBError::EmptyPartnerAddress);
    }

    // Validate liability limit (§171 HGB)
    if partner.liability_limit.amount_cents == 0 {
        return Err(HGBError::ZeroLiabilityLimit {
            name: partner.name.clone(),
        });
    }

    // Minimum liability limit of €1
    if partner.liability_limit.amount_cents < 100 {
        return Err(HGBError::LiabilityLimitTooLow {
            name: partner.name.clone(),
            liability: partner.liability_limit.to_euros(),
        });
    }

    // Contribution paid cannot exceed liability limit
    if partner.contribution_paid.amount_cents > partner.liability_limit.amount_cents {
        return Err(HGBError::ContributionExceedsLiabilityLimit {
            name: partner.name.clone(),
            paid: partner.contribution_paid.to_euros(),
            limit: partner.liability_limit.to_euros(),
        });
    }

    Ok(())
}

// =============================================================================
// Business Purpose Validation
// =============================================================================

/// Validate business purpose (Unternehmensgegenstand)
///
/// Must be:
/// - Non-empty
/// - At least 10 characters (must be meaningful)
pub fn validate_business_purpose(purpose: &str) -> Result<()> {
    if purpose.trim().is_empty() {
        return Err(HGBError::EmptyBusinessPurpose);
    }

    if purpose.trim().len() < 10 {
        return Err(HGBError::InvalidBusinessPurpose);
    }

    Ok(())
}

// =============================================================================
// Registered Office Validation
// =============================================================================

/// Validate registered office (Sitz)
///
/// Must be a German city (basic validation - checks non-empty)
///
/// # Note
///
/// This is a simplified validation. In production, you might want to
/// validate against a list of German cities.
pub fn validate_registered_office(city: &str) -> Result<()> {
    if city.trim().is_empty() {
        return Err(HGBError::EmptyRegisteredOffice);
    }

    // In production, validate against known German cities
    // For now, accept any non-empty string as valid

    Ok(())
}

// =============================================================================
// Fiscal Year End Validation
// =============================================================================

/// Validate fiscal year end date
///
/// Ensures month and day are valid calendar values.
pub fn validate_fiscal_year_end(fye: FiscalYearEnd) -> Result<()> {
    // Validate month (1-12)
    if !(1..=12).contains(&fye.month) {
        return Err(HGBError::InvalidFiscalYearEnd {
            month: fye.month,
            day: fye.day,
        });
    }

    // Validate day based on month
    let max_day = match fye.month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 29, // Accept Feb 29 (leap year)
        _ => {
            return Err(HGBError::InvalidFiscalYearEnd {
                month: fye.month,
                day: fye.day,
            });
        }
    };

    if fye.day < 1 || fye.day > max_day {
        return Err(HGBError::InvalidFiscalYearEnd {
            month: fye.month,
            day: fye.day,
        });
    }

    Ok(())
}

// =============================================================================
// OHG Validation
// =============================================================================

/// Validate OHG (General Partnership) per §105-160 HGB
///
/// # Legal Requirements
///
/// - **Minimum partners**: 2 (§105 Abs. 1 HGB)
/// - **Unlimited liability**: All partners must have unlimited liability (§128 HGB)
/// - **Partnership name**: Must include "OHG" suffix (§19 HGB)
/// - **Business purpose**: Required and meaningful
/// - **Registered office**: Valid German city
///
/// # Examples
///
/// ```
/// use legalis_de::hgb::{OHG, Partner, PartnerType, validate_ohg};
/// use legalis_de::gmbhg::Capital;
///
/// let ohg = OHG {
///     partnership_name: "Mustermann & Schmidt OHG".to_string(),
///     registered_office: "Berlin".to_string(),
///     business_purpose: "Softwareentwicklung und IT-Beratung".to_string(),
///     partners: vec![
///         Partner {
///             name: "Max Mustermann".to_string(),
///             address: "Berlin".to_string(),
///             contribution: Some(Capital::from_euros(10_000)),
///             contribution_paid: Some(Capital::from_euros(10_000)),
///             partner_type: PartnerType::NaturalPerson,
///             has_management_authority: true,
///             has_representation_authority: true,
///         },
///         Partner {
///             name: "Erika Schmidt".to_string(),
///             address: "Hamburg".to_string(),
///             contribution: Some(Capital::from_euros(10_000)),
///             contribution_paid: Some(Capital::from_euros(10_000)),
///             partner_type: PartnerType::NaturalPerson,
///             has_management_authority: true,
///             has_representation_authority: true,
///         },
///     ],
///     formation_date: None,
///     fiscal_year_end: None,
///     unlimited_liability: true,
/// };
///
/// assert!(validate_ohg(&ohg).is_ok());
/// ```
pub fn validate_ohg(ohg: &OHG) -> Result<()> {
    // Validate partnership name
    validate_partnership_name(&ohg.partnership_name, "OHG")?;

    // Validate registered office
    validate_registered_office(&ohg.registered_office)?;

    // Validate business purpose
    validate_business_purpose(&ohg.business_purpose)?;

    // Validate minimum 2 partners (§105 Abs. 1 HGB)
    if ohg.partners.len() < 2 {
        return Err(HGBError::OHGInsufficientPartners);
    }

    // Validate each partner
    for partner in &ohg.partners {
        validate_partner(partner)?;
    }

    // Validate unlimited liability requirement (§128 HGB)
    if !ohg.unlimited_liability {
        return Err(HGBError::OHGLimitedLiabilityNotAllowed);
    }

    // Validate fiscal year end if specified
    if let Some(fye) = ohg.fiscal_year_end {
        validate_fiscal_year_end(fye)?;
    }

    Ok(())
}

// =============================================================================
// KG Validation
// =============================================================================

/// Validate KG (Limited Partnership) per §161-177a HGB
///
/// # Legal Requirements
///
/// - **General partners**: At least 1 (Komplementär with unlimited liability)
/// - **Limited partners**: At least 1 (Kommanditist with limited liability)
/// - **Partnership name**: Must include "KG" suffix (§19 HGB)
/// - **Liability limits**: Limited partners must have valid liability limits (§171 HGB)
/// - **Business purpose**: Required and meaningful
/// - **Registered office**: Valid German city
///
/// # Examples
///
/// ```
/// use legalis_de::hgb::{KG, Partner, LimitedPartner, PartnerType, validate_kg};
/// use legalis_de::gmbhg::Capital;
///
/// let kg = KG {
///     partnership_name: "Tech Ventures KG".to_string(),
///     registered_office: "München".to_string(),
///     business_purpose: "IT-Beratung und Softwareentwicklung".to_string(),
///     general_partners: vec![
///         Partner {
///             name: "Max Mustermann".to_string(),
///             address: "München".to_string(),
///             contribution: Some(Capital::from_euros(20_000)),
///             contribution_paid: Some(Capital::from_euros(20_000)),
///             partner_type: PartnerType::NaturalPerson,
///             has_management_authority: true,
///             has_representation_authority: true,
///         },
///     ],
///     limited_partners: vec![
///         LimitedPartner {
///             name: "Anna Schmidt".to_string(),
///             address: "Hamburg".to_string(),
///             liability_limit: Capital::from_euros(50_000),
///             contribution_paid: Capital::from_euros(50_000),
///             partner_type: PartnerType::NaturalPerson,
///             has_special_representation: false,
///         },
///     ],
///     formation_date: None,
///     fiscal_year_end: None,
/// };
///
/// assert!(validate_kg(&kg).is_ok());
/// ```
pub fn validate_kg(kg: &KG) -> Result<()> {
    // Validate partnership name
    validate_partnership_name(&kg.partnership_name, "KG")?;

    // Validate registered office
    validate_registered_office(&kg.registered_office)?;

    // Validate business purpose
    validate_business_purpose(&kg.business_purpose)?;

    // Validate minimum 1 general partner (§161 Abs. 1 HGB)
    if kg.general_partners.is_empty() {
        return Err(HGBError::KGNoGeneralPartner);
    }

    // Validate minimum 1 limited partner (§161 Abs. 1 HGB)
    if kg.limited_partners.is_empty() {
        return Err(HGBError::KGNoLimitedPartner);
    }

    // Validate each general partner
    for partner in &kg.general_partners {
        validate_partner(partner)?;
    }

    // Validate each limited partner
    for partner in &kg.limited_partners {
        validate_limited_partner(partner)?;
    }

    // Validate fiscal year end if specified
    if let Some(fye) = kg.fiscal_year_end {
        validate_fiscal_year_end(fye)?;
    }

    Ok(())
}

// =============================================================================
// GmbH & Co. KG Validation
// =============================================================================

/// Validate GmbH partner for GmbH & Co. KG
///
/// # Legal Requirements
///
/// - GmbH must have valid company name (must include "GmbH")
/// - Share capital must be at least €25,000 (GmbHG §5)
/// - Must have at least one managing director
/// - Registered office must be valid
fn validate_gmbh_partner(gmbh: &GmbHPartner) -> Result<()> {
    // Validate company name includes "GmbH"
    let normalized = gmbh.company_name.replace([' ', '.'], "").to_lowercase();

    if !normalized.contains("gmbh") && !normalized.contains("gesellschaftmitbeschränkterhaftung") {
        return Err(HGBError::InvalidGmbHPartner {
            name: gmbh.company_name.clone(),
            reason: "Company name must include 'GmbH'".to_string(),
        });
    }

    // Validate registered office
    if gmbh.registered_office.trim().is_empty() {
        return Err(HGBError::InvalidGmbHPartner {
            name: gmbh.company_name.clone(),
            reason: "Registered office cannot be empty".to_string(),
        });
    }

    // Validate share capital (minimum €25,000 for GmbH)
    const GMBH_MINIMUM_CENTS: u64 = 2_500_000; // €25,000
    if gmbh.share_capital.amount_cents < GMBH_MINIMUM_CENTS {
        return Err(HGBError::GmbHPartnerInsufficientCapital {
            name: gmbh.company_name.clone(),
            actual: gmbh.share_capital.to_euros(),
        });
    }

    // Validate at least one managing director
    if gmbh.managing_directors.is_empty() {
        return Err(HGBError::GmbHPartnerNoDirectors {
            name: gmbh.company_name.clone(),
        });
    }

    // Validate managing director names are non-empty
    for director in &gmbh.managing_directors {
        if director.trim().is_empty() {
            return Err(HGBError::InvalidGmbHPartner {
                name: gmbh.company_name.clone(),
                reason: "Managing director name cannot be empty".to_string(),
            });
        }
    }

    Ok(())
}

/// Validate GmbH & Co. KG (Hybrid Partnership)
///
/// # Legal Requirements
///
/// - **General partner**: Must be a valid GmbH
/// - **GmbH capital**: Minimum €25,000 (GmbHG §5)
/// - **Limited partners**: At least 1 required
/// - **Partnership name**: Must include "GmbH & Co. KG" or similar
/// - **Business purpose**: Required and meaningful
/// - **Registered office**: Valid German city
///
/// # Examples
///
/// ```
/// use legalis_de::hgb::{GmbHCoKG, GmbHPartner, LimitedPartner, PartnerType, validate_gmbh_co_kg};
/// use legalis_de::gmbhg::Capital;
///
/// let gmbh_co_kg = GmbHCoKG {
///     partnership_name: "Verwaltungs GmbH & Co. KG".to_string(),
///     registered_office: "Berlin".to_string(),
///     business_purpose: "Vermögensverwaltung und Beteiligungen".to_string(),
///     gmbh_general_partner: GmbHPartner {
///         company_name: "Verwaltungs GmbH".to_string(),
///         registered_office: "Berlin".to_string(),
///         managing_directors: vec!["Max Mustermann".to_string()],
///         share_capital: Capital::from_euros(25_000),
///     },
///     limited_partners: vec![
///         LimitedPartner {
///             name: "Anna Schmidt".to_string(),
///             address: "Hamburg".to_string(),
///             liability_limit: Capital::from_euros(100_000),
///             contribution_paid: Capital::from_euros(100_000),
///             partner_type: PartnerType::NaturalPerson,
///             has_special_representation: false,
///         },
///     ],
///     formation_date: None,
///     fiscal_year_end: None,
/// };
///
/// assert!(validate_gmbh_co_kg(&gmbh_co_kg).is_ok());
/// ```
pub fn validate_gmbh_co_kg(gmbh_co_kg: &GmbHCoKG) -> Result<()> {
    // Validate partnership name
    validate_partnership_name(&gmbh_co_kg.partnership_name, "GmbH & Co. KG")?;

    // Validate registered office
    validate_registered_office(&gmbh_co_kg.registered_office)?;

    // Validate business purpose
    validate_business_purpose(&gmbh_co_kg.business_purpose)?;

    // Validate GmbH general partner
    validate_gmbh_partner(&gmbh_co_kg.gmbh_general_partner)?;

    // Validate at least 1 limited partner
    if gmbh_co_kg.limited_partners.is_empty() {
        return Err(HGBError::KGNoLimitedPartner);
    }

    // Validate each limited partner
    for partner in &gmbh_co_kg.limited_partners {
        validate_limited_partner(partner)?;
    }

    // Validate fiscal year end if specified
    if let Some(fye) = gmbh_co_kg.fiscal_year_end {
        validate_fiscal_year_end(fye)?;
    }

    Ok(())
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gmbhg::Capital;
    use crate::hgb::types::PartnerType;

    // =========================================================================
    // Partnership Name Validation Tests
    // =========================================================================

    #[test]
    fn test_validate_partnership_name_ohg_valid() {
        assert!(validate_partnership_name("Mustermann & Schmidt OHG", "OHG").is_ok());
        assert!(validate_partnership_name("Offene Handelsgesellschaft Müller", "OHG").is_ok());
    }

    #[test]
    fn test_validate_partnership_name_ohg_invalid() {
        let result = validate_partnership_name("Mustermann & Schmidt", "OHG");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::MissingLegalFormSuffix { .. }
        ));
    }

    #[test]
    fn test_validate_partnership_name_kg_valid() {
        assert!(validate_partnership_name("Tech Ventures KG", "KG").is_ok());
        assert!(validate_partnership_name("Kommanditgesellschaft Schmidt", "KG").is_ok());
    }

    #[test]
    fn test_validate_partnership_name_gmbh_co_kg_valid() {
        assert!(validate_partnership_name("Verwaltungs GmbH & Co. KG", "GmbH & Co. KG").is_ok());
        assert!(validate_partnership_name("Holdings GmbH und Co. KG", "GmbH & Co. KG").is_ok());
    }

    #[test]
    fn test_validate_partnership_name_empty() {
        let result = validate_partnership_name("", "OHG");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::EmptyPartnershipName
        ));
    }

    #[test]
    fn test_validate_partnership_name_too_short() {
        let result = validate_partnership_name("AB", "OHG");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::PartnershipNameTooShort { .. }
        ));
    }

    // =========================================================================
    // Partner Validation Tests
    // =========================================================================

    #[test]
    fn test_validate_partner_valid() {
        let partner = Partner {
            name: "Max Mustermann".to_string(),
            address: "Berlin".to_string(),
            contribution: Some(Capital::from_euros(10_000)),
            contribution_paid: Some(Capital::from_euros(10_000)),
            partner_type: PartnerType::NaturalPerson,
            has_management_authority: true,
            has_representation_authority: true,
        };
        assert!(validate_partner(&partner).is_ok());
    }

    #[test]
    fn test_validate_partner_empty_name() {
        let partner = Partner {
            name: "".to_string(),
            address: "Berlin".to_string(),
            contribution: None,
            contribution_paid: None,
            partner_type: PartnerType::NaturalPerson,
            has_management_authority: true,
            has_representation_authority: true,
        };
        let result = validate_partner(&partner);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HGBError::EmptyPartnerName));
    }

    #[test]
    fn test_validate_partner_empty_address() {
        let partner = Partner {
            name: "Max Mustermann".to_string(),
            address: "".to_string(),
            contribution: None,
            contribution_paid: None,
            partner_type: PartnerType::NaturalPerson,
            has_management_authority: true,
            has_representation_authority: true,
        };
        let result = validate_partner(&partner);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HGBError::EmptyPartnerAddress));
    }

    #[test]
    fn test_validate_partner_contribution_paid_exceeds() {
        let partner = Partner {
            name: "Max Mustermann".to_string(),
            address: "Berlin".to_string(),
            contribution: Some(Capital::from_euros(10_000)),
            contribution_paid: Some(Capital::from_euros(15_000)), // Exceeds!
            partner_type: PartnerType::NaturalPerson,
            has_management_authority: true,
            has_representation_authority: true,
        };
        let result = validate_partner(&partner);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::ContributionPaidExceedsObligation { .. }
        ));
    }

    // =========================================================================
    // Limited Partner Validation Tests
    // =========================================================================

    #[test]
    fn test_validate_limited_partner_valid() {
        let partner = LimitedPartner {
            name: "Anna Schmidt".to_string(),
            address: "Hamburg".to_string(),
            liability_limit: Capital::from_euros(50_000),
            contribution_paid: Capital::from_euros(50_000),
            partner_type: PartnerType::NaturalPerson,
            has_special_representation: false,
        };
        assert!(validate_limited_partner(&partner).is_ok());
    }

    #[test]
    fn test_validate_limited_partner_zero_liability() {
        let partner = LimitedPartner {
            name: "Anna Schmidt".to_string(),
            address: "Hamburg".to_string(),
            liability_limit: Capital::from_euros(0),
            contribution_paid: Capital::from_euros(0),
            partner_type: PartnerType::NaturalPerson,
            has_special_representation: false,
        };
        let result = validate_limited_partner(&partner);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::ZeroLiabilityLimit { .. }
        ));
    }

    #[test]
    fn test_validate_limited_partner_liability_too_low() {
        let partner = LimitedPartner {
            name: "Anna Schmidt".to_string(),
            address: "Hamburg".to_string(),
            liability_limit: Capital {
                amount_cents: 50, // €0.50 - too low!
            },
            contribution_paid: Capital { amount_cents: 50 },
            partner_type: PartnerType::NaturalPerson,
            has_special_representation: false,
        };
        let result = validate_limited_partner(&partner);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::LiabilityLimitTooLow { .. }
        ));
    }

    #[test]
    fn test_validate_limited_partner_contribution_exceeds_limit() {
        let partner = LimitedPartner {
            name: "Anna Schmidt".to_string(),
            address: "Hamburg".to_string(),
            liability_limit: Capital::from_euros(50_000),
            contribution_paid: Capital::from_euros(60_000), // Exceeds limit!
            partner_type: PartnerType::NaturalPerson,
            has_special_representation: false,
        };
        let result = validate_limited_partner(&partner);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::ContributionExceedsLiabilityLimit { .. }
        ));
    }

    // =========================================================================
    // Business Purpose Validation Tests
    // =========================================================================

    #[test]
    fn test_validate_business_purpose_valid() {
        assert!(validate_business_purpose("Softwareentwicklung und IT-Beratung").is_ok());
    }

    #[test]
    fn test_validate_business_purpose_empty() {
        let result = validate_business_purpose("");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::EmptyBusinessPurpose
        ));
    }

    #[test]
    fn test_validate_business_purpose_too_short() {
        let result = validate_business_purpose("Software");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::InvalidBusinessPurpose
        ));
    }

    // =========================================================================
    // Registered Office Validation Tests
    // =========================================================================

    #[test]
    fn test_validate_registered_office_valid() {
        assert!(validate_registered_office("Berlin").is_ok());
        assert!(validate_registered_office("München").is_ok());
    }

    #[test]
    fn test_validate_registered_office_empty() {
        let result = validate_registered_office("");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::EmptyRegisteredOffice
        ));
    }

    // =========================================================================
    // Fiscal Year End Validation Tests
    // =========================================================================

    #[test]
    fn test_validate_fiscal_year_end_valid() {
        assert!(validate_fiscal_year_end(FiscalYearEnd { month: 12, day: 31 }).is_ok());
        assert!(validate_fiscal_year_end(FiscalYearEnd { month: 6, day: 30 }).is_ok());
        assert!(validate_fiscal_year_end(FiscalYearEnd { month: 2, day: 29 }).is_ok()); // Leap year
    }

    #[test]
    fn test_validate_fiscal_year_end_invalid_month() {
        let result = validate_fiscal_year_end(FiscalYearEnd { month: 13, day: 31 });
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::InvalidFiscalYearEnd { .. }
        ));
    }

    #[test]
    fn test_validate_fiscal_year_end_invalid_day() {
        let result = validate_fiscal_year_end(FiscalYearEnd { month: 2, day: 30 });
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::InvalidFiscalYearEnd { .. }
        ));
    }

    // =========================================================================
    // OHG Validation Tests
    // =========================================================================

    fn create_test_ohg() -> OHG {
        OHG {
            partnership_name: "Mustermann & Schmidt OHG".to_string(),
            registered_office: "Berlin".to_string(),
            business_purpose: "Softwareentwicklung und IT-Beratung".to_string(),
            partners: vec![
                Partner {
                    name: "Max Mustermann".to_string(),
                    address: "Berlin".to_string(),
                    contribution: Some(Capital::from_euros(10_000)),
                    contribution_paid: Some(Capital::from_euros(10_000)),
                    partner_type: PartnerType::NaturalPerson,
                    has_management_authority: true,
                    has_representation_authority: true,
                },
                Partner {
                    name: "Erika Schmidt".to_string(),
                    address: "Hamburg".to_string(),
                    contribution: Some(Capital::from_euros(10_000)),
                    contribution_paid: Some(Capital::from_euros(10_000)),
                    partner_type: PartnerType::NaturalPerson,
                    has_management_authority: true,
                    has_representation_authority: true,
                },
            ],
            formation_date: None,
            fiscal_year_end: None,
            unlimited_liability: true,
        }
    }

    #[test]
    fn test_validate_ohg_valid() {
        let ohg = create_test_ohg();
        assert!(validate_ohg(&ohg).is_ok());
    }

    #[test]
    fn test_validate_ohg_insufficient_partners() {
        let mut ohg = create_test_ohg();
        ohg.partners.truncate(1); // Only 1 partner

        let result = validate_ohg(&ohg);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::OHGInsufficientPartners
        ));
    }

    #[test]
    fn test_validate_ohg_limited_liability_not_allowed() {
        let mut ohg = create_test_ohg();
        ohg.unlimited_liability = false;

        let result = validate_ohg(&ohg);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::OHGLimitedLiabilityNotAllowed
        ));
    }

    // =========================================================================
    // KG Validation Tests
    // =========================================================================

    fn create_test_kg() -> KG {
        KG {
            partnership_name: "Tech Ventures KG".to_string(),
            registered_office: "München".to_string(),
            business_purpose: "IT-Beratung und Softwareentwicklung".to_string(),
            general_partners: vec![Partner {
                name: "Max Mustermann".to_string(),
                address: "München".to_string(),
                contribution: Some(Capital::from_euros(20_000)),
                contribution_paid: Some(Capital::from_euros(20_000)),
                partner_type: PartnerType::NaturalPerson,
                has_management_authority: true,
                has_representation_authority: true,
            }],
            limited_partners: vec![LimitedPartner {
                name: "Anna Schmidt".to_string(),
                address: "Hamburg".to_string(),
                liability_limit: Capital::from_euros(50_000),
                contribution_paid: Capital::from_euros(50_000),
                partner_type: PartnerType::NaturalPerson,
                has_special_representation: false,
            }],
            formation_date: None,
            fiscal_year_end: None,
        }
    }

    #[test]
    fn test_validate_kg_valid() {
        let kg = create_test_kg();
        assert!(validate_kg(&kg).is_ok());
    }

    #[test]
    fn test_validate_kg_no_general_partner() {
        let mut kg = create_test_kg();
        kg.general_partners.clear();

        let result = validate_kg(&kg);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HGBError::KGNoGeneralPartner));
    }

    #[test]
    fn test_validate_kg_no_limited_partner() {
        let mut kg = create_test_kg();
        kg.limited_partners.clear();

        let result = validate_kg(&kg);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HGBError::KGNoLimitedPartner));
    }

    // =========================================================================
    // GmbH & Co. KG Validation Tests
    // =========================================================================

    fn create_test_gmbh_co_kg() -> GmbHCoKG {
        GmbHCoKG {
            partnership_name: "Verwaltungs GmbH & Co. KG".to_string(),
            registered_office: "Berlin".to_string(),
            business_purpose: "Vermögensverwaltung und Beteiligungen".to_string(),
            gmbh_general_partner: GmbHPartner {
                company_name: "Verwaltungs GmbH".to_string(),
                registered_office: "Berlin".to_string(),
                managing_directors: vec!["Max Mustermann".to_string()],
                share_capital: Capital::from_euros(25_000),
            },
            limited_partners: vec![LimitedPartner {
                name: "Anna Schmidt".to_string(),
                address: "Hamburg".to_string(),
                liability_limit: Capital::from_euros(100_000),
                contribution_paid: Capital::from_euros(100_000),
                partner_type: PartnerType::NaturalPerson,
                has_special_representation: false,
            }],
            formation_date: None,
            fiscal_year_end: None,
        }
    }

    #[test]
    fn test_validate_gmbh_co_kg_valid() {
        let gmbh_co_kg = create_test_gmbh_co_kg();
        assert!(validate_gmbh_co_kg(&gmbh_co_kg).is_ok());
    }

    #[test]
    fn test_validate_gmbh_partner_insufficient_capital() {
        let mut gmbh_co_kg = create_test_gmbh_co_kg();
        gmbh_co_kg.gmbh_general_partner.share_capital = Capital::from_euros(20_000);

        let result = validate_gmbh_co_kg(&gmbh_co_kg);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::GmbHPartnerInsufficientCapital { .. }
        ));
    }

    #[test]
    fn test_validate_gmbh_partner_no_directors() {
        let mut gmbh_co_kg = create_test_gmbh_co_kg();
        gmbh_co_kg.gmbh_general_partner.managing_directors.clear();

        let result = validate_gmbh_co_kg(&gmbh_co_kg);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HGBError::GmbHPartnerNoDirectors { .. }
        ));
    }

    #[test]
    fn test_validate_gmbh_co_kg_no_limited_partners() {
        let mut gmbh_co_kg = create_test_gmbh_co_kg();
        gmbh_co_kg.limited_partners.clear();

        let result = validate_gmbh_co_kg(&gmbh_co_kg);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HGBError::KGNoLimitedPartner));
    }
}
