//! Administrative Law Validation Functions (ຟັງຊັນກວດສອບກົດໝາຍບໍລິຫານ)
//!
//! This module provides comprehensive validation for administrative law in Lao PDR.
//!
//! ## Validation Categories
//!
//! - **Decision Validation**: Legal basis, authority, affected parties
//! - **License Validation**: Type, requirements, authority level
//! - **Permit Validation**: Type, conditions, documentation
//! - **Sanction Validation**: Proportionality, legal basis, authority
//! - **Appeal Validation**: Deadline, grounds, level
//! - **State Liability Validation**: Deadline, evidence, causation
//! - **Notification Validation**: Proper notification to affected parties
//! - **Authority Validation**: Jurisdiction, level appropriateness

use crate::administrative_law::error::{
    AdministrativeDecisionError, AdministrativeLawError, AdministrativeLawResult, AppealError,
    AuthorityError, LicenseError, NotificationError, PermitError, SanctionError,
    StateLiabilityError,
};
use crate::administrative_law::types::{
    ADMINISTRATIVE_APPEAL_DEADLINE_DAYS, AdministrativeAppeal, AdministrativeDecision,
    AdministrativeLevel, AdministrativeSanction, AffectedParty, AppealGround, AppealLevel,
    COURT_APPEAL_DEADLINE_DAYS, DISTRICT_JURISDICTION_LIMIT_LAK, DecisionType, LegalBasis,
    LicenseType, MAXIMUM_SUSPENSION_DAYS, MINIMUM_FINE_AMOUNT_LAK,
    PROVINCIAL_JURISDICTION_LIMIT_LAK, PermitType, STATE_LIABILITY_CLAIM_DEADLINE_YEARS,
    SanctionType, StateLiability, VILLAGE_JURISDICTION_LIMIT_LAK,
};

// ============================================================================
// Administrative Decision Validation
// ============================================================================

/// Validate an administrative decision
/// ກວດສອບການຕັດສິນໃຈບໍລິຫານ
///
/// ## Requirements
///
/// An administrative decision must have:
/// 1. Decision number
/// 2. Issuing authority with proper jurisdiction
/// 3. At least one legal basis
/// 4. Subject matter (bilingual)
/// 5. Proper decision type
///
/// ## Examples
///
/// ```
/// use legalis_la::administrative_law::{
///     AdministrativeDecision, AdministrativeLevel, DecisionType, LicenseType,
///     LegalBasis, AffectedParty, PartyType,
///     validate_administrative_decision,
/// };
///
/// let decision = AdministrativeDecision::builder()
///     .decision_number("DEC-2024-001".to_string())
///     .issuing_authority(AdministrativeLevel::Central {
///         ministry: "Ministry of Industry".to_string(),
///     })
///     .decision_date("2024-01-15".to_string())
///     .subject_lao("ການອອກໃບອະນຸຍາດ".to_string())
///     .subject_en("License Issuance".to_string())
///     .decision_type(DecisionType::License {
///         license_type: LicenseType::BusinessLicense,
///     })
///     .legal_basis(LegalBasis::new(
///         "ກົດໝາຍວ່າດ້ວຍວິສາຫະກິດ",
///         "Enterprise Law",
///         15,
///         None,
///     ))
///     .build();
///
/// if let Ok(decision) = decision {
///     let result = validate_administrative_decision(&decision);
///     assert!(result.is_ok());
/// }
/// ```
pub fn validate_administrative_decision(
    decision: &AdministrativeDecision,
) -> AdministrativeLawResult<()> {
    // Validate decision number
    if decision.decision_number.trim().is_empty() {
        return Err(AdministrativeDecisionError::MissingDecisionNumber.into());
    }

    // Validate decision date
    if decision.decision_date.trim().is_empty() {
        return Err(AdministrativeDecisionError::MissingDecisionDate.into());
    }

    // Validate subject (bilingual)
    if decision.subject_lao.trim().is_empty() || decision.subject_en.trim().is_empty() {
        return Err(AdministrativeDecisionError::MissingSubject.into());
    }

    // Validate legal basis
    validate_legal_basis(&decision.legal_basis)?;

    // Validate authority jurisdiction
    validate_authority_for_decision(&decision.issuing_authority, &decision.decision_type)?;

    // Validate affected parties notification (if any)
    for party in &decision.affected_parties {
        validate_affected_party(party)?;
    }

    Ok(())
}

/// Validate legal basis for a decision
/// ກວດສອບພື້ນຖານທາງກົດໝາຍສຳລັບການຕັດສິນໃຈ
///
/// ## Requirements
///
/// At least one legal basis must be provided with:
/// - Law name (Lao and English)
/// - Article number
///
/// ## Examples
///
/// ```
/// use legalis_la::administrative_law::{LegalBasis, validate_legal_basis};
///
/// let bases = vec![
///     LegalBasis::new("ກົດໝາຍວ່າດ້ວຍວິສາຫະກິດ", "Enterprise Law", 15, None),
/// ];
///
/// assert!(validate_legal_basis(&bases).is_ok());
///
/// // Empty legal basis should fail
/// let empty: Vec<LegalBasis> = vec![];
/// assert!(validate_legal_basis(&empty).is_err());
/// ```
pub fn validate_legal_basis(bases: &[LegalBasis]) -> AdministrativeLawResult<()> {
    if bases.is_empty() {
        return Err(AdministrativeDecisionError::MissingLegalBasis.into());
    }

    for basis in bases {
        if basis.law_name_lao.trim().is_empty() || basis.law_name_en.trim().is_empty() {
            return Err(AdministrativeDecisionError::InvalidLegalBasis {
                law_name: basis.law_name_en.clone(),
                article: basis.article_number,
                reason: "Law name cannot be empty".to_string(),
            }
            .into());
        }

        if basis.article_number == 0 {
            return Err(AdministrativeDecisionError::InvalidLegalBasis {
                law_name: basis.law_name_en.clone(),
                article: basis.article_number,
                reason: "Article number must be greater than 0".to_string(),
            }
            .into());
        }
    }

    Ok(())
}

/// Validate authority for a specific decision type
fn validate_authority_for_decision(
    authority: &AdministrativeLevel,
    decision_type: &DecisionType,
) -> AdministrativeLawResult<()> {
    let authority_level = authority.hierarchy_level();

    let required_level = match decision_type {
        DecisionType::License { license_type } => license_type.minimum_authority_level(),
        DecisionType::Fine { amount_lak } => {
            // Determine required level based on fine amount
            if *amount_lak > PROVINCIAL_JURISDICTION_LIMIT_LAK {
                0 // Central
            } else if *amount_lak > DISTRICT_JURISDICTION_LIMIT_LAK {
                1 // Provincial
            } else if *amount_lak > VILLAGE_JURISDICTION_LIMIT_LAK {
                2 // District
            } else {
                3 // Village
            }
        }
        DecisionType::Permit { .. } => 2, // District or higher
        DecisionType::Revocation | DecisionType::Suspension => 1, // Provincial or higher
        _ => 2,                           // Default: District or higher
    };

    if authority_level > required_level {
        return Err(AdministrativeDecisionError::InvalidAuthorityLevel {
            authority_level: authority.level_name_en().to_string(),
            decision_type: format!("{:?}", decision_type),
            required_level: match required_level {
                0 => "Central".to_string(),
                1 => "Provincial".to_string(),
                2 => "District".to_string(),
                _ => "Village".to_string(),
            },
        }
        .into());
    }

    Ok(())
}

// ============================================================================
// License Validation
// ============================================================================

/// Validate a license application
/// ກວດສອບຄຳຮ້ອງຂໍໃບອະນຸຍາດ
///
/// ## Requirements
///
/// A license application must have:
/// - Valid license type
/// - Proper issuing authority with jurisdiction
/// - Required documentation (implementation-specific)
///
/// ## Examples
///
/// ```
/// use legalis_la::administrative_law::{
///     AdministrativeLevel, LicenseType,
///     validate_license_application,
/// };
///
/// // Central authority can issue mining license
/// let result = validate_license_application(
///     &LicenseType::MiningLicense,
///     &AdministrativeLevel::Central { ministry: "Ministry of Energy".to_string() },
/// );
/// assert!(result.is_ok());
///
/// // Village authority cannot issue mining license
/// let result = validate_license_application(
///     &LicenseType::MiningLicense,
///     &AdministrativeLevel::Village { village: "Ban Test".to_string() },
/// );
/// assert!(result.is_err());
/// ```
pub fn validate_license_application(
    license_type: &LicenseType,
    issuing_authority: &AdministrativeLevel,
) -> AdministrativeLawResult<()> {
    let required_level = license_type.minimum_authority_level();
    let authority_level = issuing_authority.hierarchy_level();

    if authority_level > required_level {
        return Err(LicenseError::UnauthorizedAuthority {
            authority: issuing_authority.entity_name().to_string(),
            license_type: license_type.name_en(),
        }
        .into());
    }

    // Additional validation for specific license types
    match license_type {
        LicenseType::MiningLicense | LicenseType::FinancialServicesLicense => {
            // These require central authority only
            if !matches!(issuing_authority, AdministrativeLevel::Central { .. }) {
                return Err(LicenseError::UnauthorizedAuthority {
                    authority: issuing_authority.entity_name().to_string(),
                    license_type: license_type.name_en(),
                }
                .into());
            }
        }
        LicenseType::EnvironmentalLicense => {
            // Environmental licenses require provincial or higher
            if authority_level > 1 {
                return Err(LicenseError::UnauthorizedAuthority {
                    authority: issuing_authority.entity_name().to_string(),
                    license_type: license_type.name_en(),
                }
                .into());
            }
        }
        _ => {}
    }

    Ok(())
}

// ============================================================================
// Permit Validation
// ============================================================================

/// Validate a permit application
/// ກວດສອບຄຳຮ້ອງຂໍໃບຢັ້ງຢືນ
///
/// ## Requirements
///
/// A permit application must have:
/// - Valid permit type
/// - Required conditions met
/// - Proper issuing authority
///
/// ## Examples
///
/// ```
/// use legalis_la::administrative_law::{
///     AdministrativeLevel, PermitType,
///     validate_permit_application,
/// };
///
/// let result = validate_permit_application(
///     &PermitType::BuildingPermit,
///     &AdministrativeLevel::District { district: "Sisattanak".to_string() },
///     &["Approved building plan".to_string(), "Land ownership proof".to_string()],
/// );
/// assert!(result.is_ok());
/// ```
pub fn validate_permit_application(
    permit_type: &PermitType,
    issuing_authority: &AdministrativeLevel,
    conditions_met: &[String],
) -> AdministrativeLawResult<()> {
    let authority_level = issuing_authority.hierarchy_level();

    // Validate authority level for permit type
    let required_level = match permit_type {
        PermitType::WorkPermit { .. } | PermitType::ResidencePermit { .. } => 1, // Provincial
        PermitType::FirearmPermit => 0,                                          // Central only
        PermitType::LandUsePermit => 1,                                          // Provincial
        _ => 2, // District or higher
    };

    if authority_level > required_level {
        return Err(PermitError::UnauthorizedAuthority {
            authority: issuing_authority.entity_name().to_string(),
            permit_type: permit_type.name_en(),
        }
        .into());
    }

    // Check for required conditions based on permit type
    let required_conditions = get_required_permit_conditions(permit_type);
    let mut unmet: Vec<String> = Vec::new();

    for required in &required_conditions {
        if !conditions_met.iter().any(|c| c.contains(required)) {
            unmet.push(required.clone());
        }
    }

    if !unmet.is_empty() {
        return Err(PermitError::ConditionsNotMet {
            unmet_conditions: unmet.join(", "),
        }
        .into());
    }

    Ok(())
}

/// Get required conditions for a permit type
fn get_required_permit_conditions(permit_type: &PermitType) -> Vec<String> {
    match permit_type {
        PermitType::BuildingPermit => vec![
            "Approved building plan".to_string(),
            "Land ownership proof".to_string(),
        ],
        PermitType::WorkPermit { .. } => vec![
            "Employment contract".to_string(),
            "Valid passport".to_string(),
        ],
        PermitType::EnvironmentalPermit => vec!["Environmental impact assessment".to_string()],
        PermitType::LandUsePermit => {
            vec!["Land title".to_string(), "Proposed use plan".to_string()]
        }
        PermitType::EventPermit => vec!["Event plan".to_string(), "Security plan".to_string()],
        _ => vec![],
    }
}

// ============================================================================
// Sanction Validation
// ============================================================================

/// Validate an administrative sanction
/// ກວດສອບການລົງໂທດບໍລິຫານ
///
/// ## Requirements
///
/// A sanction must have:
/// - Valid sanction type
/// - Proportional to the violation
/// - Legal basis
/// - Proper issuing authority
///
/// ## Examples
///
/// ```
/// use legalis_la::administrative_law::{
///     AdministrativeSanction, AdministrativeLevel, SanctionType,
///     LegalBasis, AffectedParty, PartyType,
///     validate_sanction,
/// };
///
/// let sanction = AdministrativeSanction::builder()
///     .sanction_id("SANC-2024-001".to_string())
///     .sanction_type(SanctionType::Fine {
///         amount_lak: 1_000_000,
///         payment_deadline: "2024-02-15".to_string(),
///     })
///     .issuing_authority(AdministrativeLevel::District {
///         district: "Sisattanak".to_string(),
///     })
///     .legal_basis(LegalBasis::new("Tax Law", "Tax Law", 50, None))
///     .violation_description_lao("ການລະເມີດ".to_string())
///     .violation_description_en("Violation".to_string())
///     .sanction_date("2024-01-20".to_string())
///     .subject(AffectedParty::new("Company", PartyType::LegalEntity))
///     .build();
///
/// if let Ok(sanction) = sanction {
///     let result = validate_sanction(&sanction);
///     assert!(result.is_ok());
/// }
/// ```
pub fn validate_sanction(sanction: &AdministrativeSanction) -> AdministrativeLawResult<()> {
    // Validate legal basis
    if sanction.legal_basis.law_name_lao.trim().is_empty()
        || sanction.legal_basis.law_name_en.trim().is_empty()
    {
        return Err(SanctionError::MissingLegalBasis.into());
    }

    // Validate violation description
    if sanction.violation_description_lao.trim().is_empty()
        || sanction.violation_description_en.trim().is_empty()
    {
        return Err(SanctionError::MissingGrounds.into());
    }

    // Validate sanction type specific rules
    validate_sanction_type(&sanction.sanction_type, &sanction.issuing_authority)?;

    // Validate proportionality
    validate_proportionality(&sanction.sanction_type, &sanction.violation_description_en)?;

    Ok(())
}

/// Validate sanction type specific rules
fn validate_sanction_type(
    sanction_type: &SanctionType,
    authority: &AdministrativeLevel,
) -> AdministrativeLawResult<()> {
    match sanction_type {
        SanctionType::Fine { amount_lak, .. } => {
            // Check minimum amount
            if *amount_lak < MINIMUM_FINE_AMOUNT_LAK {
                return Err(SanctionError::FineBelowMinimum {
                    amount: *amount_lak,
                    minimum: MINIMUM_FINE_AMOUNT_LAK,
                }
                .into());
            }

            // Check jurisdiction limit
            if let Some(limit) = authority.jurisdiction_limit_lak()
                && *amount_lak > limit
            {
                return Err(SanctionError::FineExceedsLimit {
                    amount: *amount_lak,
                    limit,
                }
                .into());
            }
        }
        SanctionType::LicenseSuspension { duration_days } => {
            if *duration_days > MAXIMUM_SUSPENSION_DAYS {
                return Err(SanctionError::SuspensionExceedsMaximum {
                    days: *duration_days,
                    max_days: MAXIMUM_SUSPENSION_DAYS,
                }
                .into());
            }
        }
        SanctionType::LicenseRevocation
        | SanctionType::BusinessClosure {
            temporary: false, ..
        } => {
            // These severe sanctions require at least provincial authority
            if authority.hierarchy_level() > 1 {
                return Err(SanctionError::UnauthorizedAuthority {
                    authority: authority.entity_name().to_string(),
                }
                .into());
            }
        }
        _ => {}
    }

    Ok(())
}

/// Validate proportionality of sanction
/// ກວດສອບຄວາມສົມເຫດສົມຜົນຂອງການລົງໂທດ
///
/// ## Proportionality Principles
///
/// 1. Oral warning for first-time minor violations
/// 2. Written warning for repeated minor violations
/// 3. Fines proportional to severity and economic impact
/// 4. Suspension/revocation reserved for serious violations
/// 5. Business closure only for grave violations
pub fn validate_proportionality(
    sanction_type: &SanctionType,
    violation_description: &str,
) -> AdministrativeLawResult<()> {
    let severity = sanction_type.severity_level();
    let violation_lower = violation_description.to_lowercase();

    // Check for disproportionate sanctions
    let violation_severity = estimate_violation_severity(&violation_lower);

    // Severity difference of more than 2 levels is disproportionate
    if severity > violation_severity + 2 {
        return Err(SanctionError::Disproportionate {
            severity,
            violation: violation_description.to_string(),
        }
        .into());
    }

    // Specific checks
    match sanction_type {
        SanctionType::LicenseRevocation
        | SanctionType::BusinessClosure {
            temporary: false, ..
        } => {
            // These require serious violations
            if violation_severity < 4 {
                return Err(SanctionError::Disproportionate {
                    severity,
                    violation: violation_description.to_string(),
                }
                .into());
            }
        }
        _ => {}
    }

    Ok(())
}

/// Estimate violation severity from description (simplified heuristic)
fn estimate_violation_severity(description: &str) -> u8 {
    if description.contains("fraud")
        || description.contains("criminal")
        || description.contains("safety")
        || description.contains("death")
        || description.contains("serious injury")
    {
        5
    } else if description.contains("repeated")
        || description.contains("willful")
        || description.contains("significant")
    {
        4
    } else if description.contains("negligence")
        || description.contains("failure")
        || description.contains("violation")
    {
        3
    } else if description.contains("minor")
        || description.contains("first")
        || description.contains("technical")
    {
        2
    } else {
        3 // Default middle severity
    }
}

// ============================================================================
// Appeal Validation
// ============================================================================

/// Validate an administrative appeal
/// ກວດສອບການອຸທອນບໍລິຫານ
///
/// ## Requirements
///
/// An appeal must have:
/// - Filed within deadline
/// - At least one valid ground
/// - Proper appeal level
/// - Original decision reference
///
/// ## Examples
///
/// ```
/// use legalis_la::administrative_law::{
///     AdministrativeAppeal, AppealLevel, AppealGround,
///     AffectedParty, PartyType,
///     validate_administrative_appeal,
/// };
///
/// let appeal = AdministrativeAppeal::builder()
///     .appeal_number("APP-2024-001".to_string())
///     .original_decision("DEC-2024-001".to_string())
///     .appellant(AffectedParty::new("John Doe", PartyType::Individual))
///     .appeal_ground(AppealGround::ProceduralError {
///         description: "Not notified".to_string(),
///     })
///     .filing_date("2024-02-01".to_string())
///     .appeal_level(AppealLevel::SuperiorAuthority {
///         authority: "Ministry of Justice".to_string(),
///     })
///     .deadline_date("2024-02-15".to_string())
///     .build();
///
/// if let Ok(appeal) = appeal {
///     let result = validate_administrative_appeal(&appeal);
///     assert!(result.is_ok());
/// }
/// ```
pub fn validate_administrative_appeal(
    appeal: &AdministrativeAppeal,
) -> AdministrativeLawResult<()> {
    // Validate appeal number
    if appeal.appeal_number.trim().is_empty() {
        return Err(AdministrativeLawError::ValidationError(
            "Missing appeal number".to_string(),
        ));
    }

    // Validate original decision reference
    if appeal.original_decision.trim().is_empty() {
        return Err(AppealError::OriginalDecisionNotFound {
            decision_number: "empty".to_string(),
        }
        .into());
    }

    // Validate appeal grounds
    if appeal.appeal_grounds.is_empty() {
        return Err(AppealError::MissingGrounds.into());
    }

    // Validate each appeal ground
    for ground in &appeal.appeal_grounds {
        validate_appeal_ground(ground)?;
    }

    // Validate appeal level requirements
    validate_appeal_level(&appeal.appeal_level)?;

    Ok(())
}

/// Validate a single appeal ground
fn validate_appeal_ground(ground: &AppealGround) -> AdministrativeLawResult<()> {
    match ground {
        AppealGround::ProceduralError { description }
        | AppealGround::FactualError { description }
        | AppealGround::LegalError { description }
        | AppealGround::NewEvidence { description } => {
            if description.trim().is_empty() {
                return Err(AppealError::InsufficientGrounds.into());
            }
        }
        AppealGround::ViolationOfRights { right } => {
            if right.trim().is_empty() {
                return Err(AppealError::InsufficientGrounds.into());
            }
        }
        _ => {}
    }
    Ok(())
}

/// Validate appeal level requirements
fn validate_appeal_level(level: &AppealLevel) -> AdministrativeLawResult<()> {
    match level {
        AppealLevel::AdministrativeCourt | AppealLevel::SupremeCourt => {
            // Court appeals have longer deadlines
            // In real implementation, would check if administrative remedies exhausted
        }
        AppealLevel::SuperiorAuthority { authority } => {
            if authority.trim().is_empty() {
                return Err(AppealError::WrongLevel {
                    attempted_level: "SuperiorAuthority".to_string(),
                    correct_level: "Must specify superior authority".to_string(),
                }
                .into());
            }
        }
        _ => {}
    }
    Ok(())
}

/// Validate appeal deadline
/// ກວດສອບກຳນົດເວລາອຸທອນ
///
/// ## Deadlines
///
/// - Administrative appeal: 30 days from notification
/// - Court appeal: 60 days from administrative decision
///
/// ## Examples
///
/// ```
/// use legalis_la::administrative_law::validate_appeal_deadline;
///
/// // Within 30 days - valid
/// assert!(validate_appeal_deadline(25, 30).is_ok());
///
/// // Exactly 30 days - valid
/// assert!(validate_appeal_deadline(30, 30).is_ok());
///
/// // After 30 days - invalid
/// assert!(validate_appeal_deadline(35, 30).is_err());
/// ```
pub fn validate_appeal_deadline(
    days_since_notification: u8,
    deadline_days: u8,
) -> AdministrativeLawResult<()> {
    if days_since_notification > deadline_days {
        return Err(AppealError::DeadlineMissed {
            filed_date: format!("{} days after notification", days_since_notification),
            deadline_date: format!("{} days after notification", deadline_days),
            deadline_days,
        }
        .into());
    }
    Ok(())
}

/// Validate administrative appeal deadline (30 days)
pub fn validate_administrative_appeal_deadline(
    days_since_notification: u8,
) -> AdministrativeLawResult<()> {
    validate_appeal_deadline(days_since_notification, ADMINISTRATIVE_APPEAL_DEADLINE_DAYS)
}

/// Validate court appeal deadline (60 days)
pub fn validate_court_appeal_deadline(days_since_decision: u8) -> AdministrativeLawResult<()> {
    validate_appeal_deadline(days_since_decision, COURT_APPEAL_DEADLINE_DAYS)
}

// ============================================================================
// State Liability Validation
// ============================================================================

/// Validate a state liability claim
/// ກວດສອບການຮ້ອງຂໍຄ່າເສຍຫາຍຈາກລັດ
///
/// ## Requirements
///
/// A state liability claim must have:
/// - Filed within 2 years of wrongful act
/// - Sufficient evidence
/// - Established causation
/// - Description of damages
///
/// ## Examples
///
/// ```
/// use legalis_la::administrative_law::{
///     StateLiability, AdministrativeLevel, LiabilityType,
///     AffectedParty, PartyType,
///     validate_state_liability_claim,
/// };
///
/// let claim = StateLiability::new(
///     "SLC-2024-001",
///     AffectedParty::new("John Doe", PartyType::Individual),
///     AdministrativeLevel::Provincial { province: "Vientiane".to_string() },
///     LiabilityType::WrongfulDecision,
///     "ຄວາມເສຍຫາຍ",
///     "Damage description",
///     10_000_000,
/// )
/// .with_evidence("Witness statement");
///
/// let result = validate_state_liability_claim(&claim, 12);
/// assert!(result.is_ok());
/// ```
pub fn validate_state_liability_claim(
    claim: &StateLiability,
    months_since_wrongful_act: u16,
) -> AdministrativeLawResult<()> {
    // Validate claim deadline (2 years = 24 months)
    let deadline_months = STATE_LIABILITY_CLAIM_DEADLINE_YEARS as u16 * 12;
    if months_since_wrongful_act > deadline_months {
        let years_elapsed = (months_since_wrongful_act / 12) as u8;
        return Err(StateLiabilityError::ClaimDeadlineExceeded {
            years_elapsed,
            limit_years: STATE_LIABILITY_CLAIM_DEADLINE_YEARS,
        }
        .into());
    }

    // Validate damage descriptions
    if claim.damage_description_lao.trim().is_empty()
        || claim.damage_description_en.trim().is_empty()
    {
        return Err(StateLiabilityError::MissingDamageDescription.into());
    }

    // Validate claimed amount
    if claim.claimed_amount_lak == 0 {
        return Err(StateLiabilityError::AmountExceedsLimits { claimed: 0 }.into());
    }

    // Validate supporting evidence
    if claim.supporting_evidence.is_empty() {
        return Err(StateLiabilityError::InsufficientEvidence {
            details: "No supporting evidence provided".to_string(),
        }
        .into());
    }

    Ok(())
}

// ============================================================================
// Notification Validation
// ============================================================================

/// Validate notification to affected parties
/// ກວດສອບການແຈ້ງໃຫ້ຝ່າຍທີ່ໄດ້ຮັບຜົນກະທົບ
///
/// ## Requirements
///
/// Each affected party must be:
/// - Identified with name and type
/// - Notified before the decision takes effect
/// - Given notification date
///
/// ## Examples
///
/// ```
/// use legalis_la::administrative_law::{
///     AffectedParty, PartyType,
///     validate_notification,
/// };
///
/// let party = AffectedParty::new("Company ABC", PartyType::LegalEntity)
///     .with_notification("2024-01-15");
///
/// assert!(validate_notification(&party).is_ok());
/// ```
pub fn validate_notification(party: &AffectedParty) -> AdministrativeLawResult<()> {
    // Validate party name
    if party.party_name.trim().is_empty() {
        return Err(NotificationError::PartyNotNotified {
            party_name: "Unknown".to_string(),
        }
        .into());
    }

    // Validate notification status
    if !party.is_notified {
        return Err(NotificationError::PartyNotNotified {
            party_name: party.party_name.clone(),
        }
        .into());
    }

    // Validate notification date
    if party.notification_date.is_none() {
        return Err(NotificationError::MissingNotificationDate {
            party_name: party.party_name.clone(),
        }
        .into());
    }

    Ok(())
}

/// Validate all affected parties in a decision
fn validate_affected_party(party: &AffectedParty) -> AdministrativeLawResult<()> {
    if party.party_name.trim().is_empty() {
        return Err(AdministrativeDecisionError::MissingAffectedParties.into());
    }
    Ok(())
}

// ============================================================================
// Authority Jurisdiction Validation
// ============================================================================

/// Validate authority jurisdiction
/// ກວດສອບຂອບເຂດອຳນາດ
///
/// ## Jurisdiction Limits
///
/// - Central: Unlimited
/// - Provincial: 500,000,000 LAK
/// - District: 50,000,000 LAK
/// - Village: 5,000,000 LAK
///
/// ## Examples
///
/// ```
/// use legalis_la::administrative_law::{
///     AdministrativeLevel,
///     validate_authority_jurisdiction,
/// };
///
/// // District can handle 10M LAK fine
/// let result = validate_authority_jurisdiction(
///     &AdministrativeLevel::District { district: "Test".to_string() },
///     10_000_000,
/// );
/// assert!(result.is_ok());
///
/// // Village cannot handle 10M LAK fine
/// let result = validate_authority_jurisdiction(
///     &AdministrativeLevel::Village { village: "Test".to_string() },
///     10_000_000,
/// );
/// assert!(result.is_err());
/// ```
pub fn validate_authority_jurisdiction(
    authority: &AdministrativeLevel,
    amount_lak: u64,
) -> AdministrativeLawResult<()> {
    if let Some(limit) = authority.jurisdiction_limit_lak()
        && amount_lak > limit
    {
        return Err(AuthorityError::JurisdictionExceeded {
            authority: authority.entity_name().to_string(),
            reason: format!(
                "Amount {} LAK exceeds jurisdiction limit {} LAK",
                amount_lak, limit
            ),
        }
        .into());
    }
    Ok(())
}

/// Validate that superior authority exists for appeals
pub fn validate_superior_authority(
    _current: &AdministrativeLevel,
    appeal_level: &AppealLevel,
) -> AdministrativeLawResult<()> {
    match appeal_level {
        AppealLevel::SameAuthority => Ok(()),
        AppealLevel::SuperiorAuthority { authority } => {
            if authority.trim().is_empty() {
                return Err(AuthorityError::NotFound {
                    authority: "superior authority".to_string(),
                }
                .into());
            }
            Ok(())
        }
        AppealLevel::AdministrativeCourt | AppealLevel::SupremeCourt => {
            // Court appeals are always valid
            Ok(())
        }
    }
}

// ============================================================================
// Comprehensive Validation Functions
// ============================================================================

/// Validate a complete administrative decision with all requirements
pub fn validate_complete_decision(
    decision: &AdministrativeDecision,
    require_all_parties_notified: bool,
) -> AdministrativeLawResult<()> {
    // Basic validation
    validate_administrative_decision(decision)?;

    // Additional validation for complete decisions
    if require_all_parties_notified {
        for party in &decision.affected_parties {
            if !party.is_notified {
                return Err(NotificationError::PartyNotNotified {
                    party_name: party.party_name.clone(),
                }
                .into());
            }
        }
    }

    // Validate authority can issue this type of decision
    validate_authority_for_decision(&decision.issuing_authority, &decision.decision_type)?;

    Ok(())
}

/// Validate a complete sanction with all requirements
pub fn validate_complete_sanction(
    sanction: &AdministrativeSanction,
) -> AdministrativeLawResult<()> {
    validate_sanction(sanction)?;

    // Additional validation
    if sanction.subject.party_name.trim().is_empty() {
        return Err(SanctionError::SubjectNotIdentified.into());
    }

    Ok(())
}

/// Validate a complete appeal with all requirements
pub fn validate_complete_appeal(
    appeal: &AdministrativeAppeal,
    days_since_notification: u8,
) -> AdministrativeLawResult<()> {
    validate_administrative_appeal(appeal)?;

    // Validate deadline based on appeal level
    let deadline = appeal.appeal_level.deadline_days();
    validate_appeal_deadline(days_since_notification, deadline)?;

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::administrative_law::types::{LiabilityType, PartyType};

    #[test]
    fn test_validate_legal_basis() {
        let valid_basis = vec![LegalBasis::new(
            "ກົດໝາຍວ່າດ້ວຍວິສາຫະກິດ",
            "Enterprise Law",
            15,
            None,
        )];
        assert!(validate_legal_basis(&valid_basis).is_ok());

        let empty_basis: Vec<LegalBasis> = vec![];
        assert!(validate_legal_basis(&empty_basis).is_err());
    }

    #[test]
    fn test_validate_license_application() {
        // Central can issue mining license
        let result = validate_license_application(
            &LicenseType::MiningLicense,
            &AdministrativeLevel::Central {
                ministry: "Ministry of Energy".to_string(),
            },
        );
        assert!(result.is_ok());

        // Village cannot issue mining license
        let result = validate_license_application(
            &LicenseType::MiningLicense,
            &AdministrativeLevel::Village {
                village: "Ban Test".to_string(),
            },
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_appeal_deadline() {
        // Within deadline
        assert!(validate_appeal_deadline(25, 30).is_ok());
        assert!(validate_appeal_deadline(30, 30).is_ok());

        // After deadline
        assert!(validate_appeal_deadline(31, 30).is_err());
    }

    #[test]
    fn test_validate_state_liability_deadline() {
        // Within 2 years (24 months)
        assert!(
            validate_state_liability_claim(
                &StateLiability::new(
                    "SLC-001",
                    AffectedParty::new("John", PartyType::Individual),
                    AdministrativeLevel::Provincial {
                        province: "Vientiane".to_string()
                    },
                    LiabilityType::WrongfulDecision,
                    "ຄວາມເສຍຫາຍ",
                    "Damage",
                    1_000_000,
                )
                .with_evidence("Evidence"),
                12
            )
            .is_ok()
        );

        // After 2 years
        assert!(
            validate_state_liability_claim(
                &StateLiability::new(
                    "SLC-001",
                    AffectedParty::new("John", PartyType::Individual),
                    AdministrativeLevel::Provincial {
                        province: "Vientiane".to_string()
                    },
                    LiabilityType::WrongfulDecision,
                    "ຄວາມເສຍຫາຍ",
                    "Damage",
                    1_000_000,
                )
                .with_evidence("Evidence"),
                30
            )
            .is_err()
        );
    }

    #[test]
    fn test_validate_authority_jurisdiction() {
        // District can handle 10M LAK
        assert!(
            validate_authority_jurisdiction(
                &AdministrativeLevel::District {
                    district: "Test".to_string()
                },
                10_000_000
            )
            .is_ok()
        );

        // Village cannot handle 10M LAK
        assert!(
            validate_authority_jurisdiction(
                &AdministrativeLevel::Village {
                    village: "Test".to_string()
                },
                10_000_000
            )
            .is_err()
        );

        // Central has no limit
        assert!(
            validate_authority_jurisdiction(
                &AdministrativeLevel::Central {
                    ministry: "Test".to_string()
                },
                1_000_000_000
            )
            .is_ok()
        );
    }

    #[test]
    fn test_validate_notification() {
        let notified_party = AffectedParty::new("Company ABC", PartyType::LegalEntity)
            .with_notification("2024-01-15");
        assert!(validate_notification(&notified_party).is_ok());

        let unnotified_party = AffectedParty::new("Company XYZ", PartyType::LegalEntity);
        assert!(validate_notification(&unnotified_party).is_err());
    }

    #[test]
    fn test_validate_proportionality() {
        // Minor violation should not get license revocation
        let result = validate_proportionality(
            &SanctionType::LicenseRevocation,
            "minor technical violation",
        );
        assert!(result.is_err());

        // Serious violation can get license revocation
        let result = validate_proportionality(
            &SanctionType::LicenseRevocation,
            "fraud and criminal activity causing safety concerns",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_sanction_fine_limits() {
        // Fine below minimum
        let result = validate_sanction_type(
            &SanctionType::Fine {
                amount_lak: 50_000,
                payment_deadline: "2024-02-15".to_string(),
            },
            &AdministrativeLevel::District {
                district: "Test".to_string(),
            },
        );
        assert!(result.is_err());

        // Fine exceeds district limit
        let result = validate_sanction_type(
            &SanctionType::Fine {
                amount_lak: 100_000_000,
                payment_deadline: "2024-02-15".to_string(),
            },
            &AdministrativeLevel::District {
                district: "Test".to_string(),
            },
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_permit_conditions() {
        // Building permit with required conditions
        let result = validate_permit_application(
            &PermitType::BuildingPermit,
            &AdministrativeLevel::District {
                district: "Sisattanak".to_string(),
            },
            &[
                "Approved building plan".to_string(),
                "Land ownership proof".to_string(),
            ],
        );
        assert!(result.is_ok());

        // Building permit without required conditions
        let result = validate_permit_application(
            &PermitType::BuildingPermit,
            &AdministrativeLevel::District {
                district: "Sisattanak".to_string(),
            },
            &[],
        );
        assert!(result.is_err());
    }
}
