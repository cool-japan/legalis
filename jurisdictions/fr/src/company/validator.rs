//! Company validation logic (Logique de validation des sociétés)
//!
//! Validation functions for French company law under the Code de commerce.
//!
//! # Purpose of Validation
//!
//! French company law imposes numerous mandatory requirements on company formation and governance.
//! This module provides programmatic validation of these requirements, enabling:
//!
//! 1. **Pre-incorporation checks**: Verify articles of incorporation (statuts) comply with legal minimums
//!    before filing with Registre du Commerce et des Sociétés (RCS).
//! 2. **Governance compliance**: Ensure board composition, meeting quorums, and voting thresholds
//!    satisfy Code de commerce mandates.
//! 3. **Error prevention**: Catch violations early, avoiding costly re-filings or invalid corporate acts.
//!
//! ## Legal Consequences of Non-Compliance
//!
//! ### Formation Defects (Nullité de la société)
//!
//! Articles L235-1 and L235-2 specify limited grounds for nullity (nullité):
//!
//! **Absolute nullity** (can be raised by anyone, anytime):
//! - Unlawful purpose (objet illicite) - e.g., company formed to commit crimes
//! - Violation of ordre public (public policy) - e.g., company formed to evade tax laws
//!
//! **Relative nullity** (only shareholders can raise, within 3 years):
//! - Insufficient capital (capital insuffisant) below statutory minimum
//! - Missing business purpose (objet social absent)
//! - Defective statuts (statuts non conformes)
//! - Invalid shareholder consent (vice de consentement)
//!
//! However, Article L235-3 allows **curing defects** before court judgment. Most formation errors
//! are fixed via corrective filings rather than voiding the company entirely. Courts favor
//! preservation (maintien de la société) over nullity.
//!
//! ### Governance Defects (Nullité des délibérations)
//!
//! Article L225-121 governs nullity of shareholder meeting resolutions:
//!
//! **Grounds for nullity**:
//! - Quorum not met (quorum non atteint) - AGO/AGE failed to reach required percentage
//! - Voting threshold not reached (majorité non atteinte) - resolution got <50% or <66.67%
//! - Procedural violations (vices de forme) - improper notice, agenda defects
//! - Violation of law or statuts (violation de la loi ou des statuts)
//!
//! **Time limits**:
//! - 3 years for shareholders to sue for nullity
//! - 3 months for procedural violations (shorter to encourage swift action)
//!
//! **Limitation**: Courts rarely void resolutions if defect is cured (e.g., second meeting with
//! proper quorum). Practical effect: many defects go unchallenged or are settled.
//!
//! ## Validation Philosophy: Strict vs. Flexible
//!
//! This module adopts **strict validation** approach:
//! - All mandatory Code requirements enforced
//! - No "substantial compliance" exceptions
//! - Rationale: Better to catch errors programmatically than risk court nullity
//!
//! However, real-world practice is more flexible:
//! - Greffe (registry) clerks sometimes overlook minor defects
//! - Courts interpret requirements pragmatically (protecting third parties' reliance)
//! - Many violations never reach litigation
//!
//! **Design choice**: This library validates strictly. Users can choose to ignore warnings
//! if they accept legal risk, but library won't silently permit violations.

use super::error::{CompanyLawError, ValidationResult};
use super::types::{
    ArticlesOfIncorporation, BoardOfDirectors, CompanyType, MeetingType, ResolutionType,
    ShareholdersMeeting,
};

/// Validate articles of incorporation (Validation des statuts)
///
/// Validates the articles of incorporation according to French company law.
///
/// # Validations
///
/// - Company name includes type suffix (SA, SARL, or SAS)
/// - Capital meets minimum requirements
/// - Business purpose is specified
/// - Head office is specified
/// - At least one shareholder
/// - For SARL: Maximum 100 partners (Article L223-3)
/// - Fiscal year end is valid (1-12)
/// - Duration does not exceed 99 years
///
/// # Arguments
///
/// * `articles` - The articles of incorporation to validate
///
/// # Returns
///
/// * `Ok(())` if valid
/// * `Err(CompanyLawError)` if validation fails
pub fn validate_articles_of_incorporation(
    articles: &ArticlesOfIncorporation,
) -> ValidationResult<()> {
    let mut errors = Vec::new();

    // Company name must include type suffix
    if !articles.has_valid_name_suffix() {
        errors.push(CompanyLawError::InvalidCompanyName {
            company_type: articles.company_type,
            suffix: articles.company_type.abbreviation().to_string(),
        });
    }

    // Capital requirements
    if !articles.capital.is_valid_for(articles.company_type) {
        errors.push(CompanyLawError::InsufficientCapital {
            company_type: articles.company_type,
            required: articles.company_type.minimum_capital(),
            actual: articles.capital.amount_eur,
            article: match articles.company_type {
                CompanyType::SA => "225-1".to_string(),
                CompanyType::SARL => "223-1".to_string(),
                CompanyType::SAS => "227-1".to_string(),
            },
        });
    }

    // Business purpose must be specified
    if articles.business_purpose.is_empty() {
        errors.push(CompanyLawError::MissingBusinessPurpose);
    }

    // Head office must be specified
    if articles.head_office.is_empty() {
        errors.push(CompanyLawError::MissingHeadOffice);
    }

    // Must have at least one shareholder
    if articles.shareholders.is_empty() {
        errors.push(CompanyLawError::NoShareholders);
    }

    // SARL: Maximum 100 partners (Article L223-3)
    if articles.company_type == CompanyType::SARL && articles.shareholders.len() > 100 {
        errors.push(CompanyLawError::TooManyPartners {
            count: articles.shareholders.len(),
        });
    }

    // Fiscal year end must be 1-12
    if !(1..=12).contains(&articles.fiscal_year_end) {
        errors.push(CompanyLawError::InvalidFiscalYearEnd {
            month: articles.fiscal_year_end,
        });
    }

    // Duration cannot exceed 99 years
    if articles.duration_years > 99 {
        errors.push(CompanyLawError::DurationTooLong {
            years: articles.duration_years,
        });
    }

    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().unwrap())
    } else {
        Err(CompanyLawError::MultipleErrors(errors))
    }
}

/// Validate SA board of directors (Validation du conseil d'administration)
///
/// Validates that a board of directors meets SA requirements.
///
/// # Requirements (Article L225-17, L225-18)
///
/// - 3-18 directors
/// - Director terms max 6 years
///
/// # Arguments
///
/// * `board` - The board to validate
///
/// # Returns
///
/// * `Ok(())` if valid
/// * `Err(CompanyLawError)` if validation fails
pub fn validate_sa_board(board: &BoardOfDirectors) -> ValidationResult<()> {
    let mut errors = Vec::new();

    // Article L225-17: Board must have 3-18 members
    if !board.is_valid_size_for_sa() {
        errors.push(CompanyLawError::InvalidBoardSize { size: board.size() });
    }

    // Article L225-18: Director terms max 6 years
    for director in &board.members {
        if director.term_years > 6 {
            errors.push(CompanyLawError::DirectorTermTooLong {
                years: director.term_years,
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().unwrap())
    } else {
        Err(CompanyLawError::MultipleErrors(errors))
    }
}

/// Validate shareholders meeting (Validation de l'assemblée générale)
///
/// Validates that a shareholders meeting meets requirements.
///
/// # Requirements
///
/// ## Quorum (Article L225-98+)
/// - Ordinary meeting (AGO): 20% on first call, no minimum on second call
/// - Extraordinary meeting (AGE): 25% on first call, 20% on second call
///
/// ## Voting
/// - Ordinary resolution: Simple majority (> 50%)
/// - Special resolution: 2/3 majority (> 66.67%)
///
/// # Arguments
///
/// * `meeting` - The meeting to validate
/// * `resolution_type` - Type of resolution being voted on
/// * `is_second_call` - Whether this is a second convocation
///
/// # Returns
///
/// * `Ok(())` if meeting is valid and resolution passes
/// * `Err(CompanyLawError)` if validation fails
pub fn validate_shareholders_meeting(
    meeting: &ShareholdersMeeting,
    resolution_type: ResolutionType,
    is_second_call: bool,
) -> ValidationResult<()> {
    // Determine quorum requirement
    let quorum_required = match (meeting.meeting_type, is_second_call) {
        (MeetingType::OrdinaryGeneralMeeting, false) => 20.0,
        (MeetingType::OrdinaryGeneralMeeting, true) => 0.0, // No minimum on second call
        (MeetingType::ExtraordinaryGeneralMeeting, false) => 25.0,
        (MeetingType::ExtraordinaryGeneralMeeting, true) => 20.0,
    };

    // Check quorum
    if !meeting.has_quorum(quorum_required) {
        return Err(CompanyLawError::QuorumNotMet {
            required: quorum_required,
            actual: meeting.quorum_percentage(),
        });
    }

    // Check if resolution passed
    if !meeting.is_approved(resolution_type) {
        let required = match resolution_type {
            ResolutionType::Ordinary => 50.0,
            ResolutionType::Special => 66.67,
        };
        return Err(CompanyLawError::ResolutionNotApproved {
            required,
            actual: meeting.approval_percentage(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::company::types::{Capital, Director, Shareholder};
    use chrono::Utc;

    #[test]
    fn test_valid_sa_articles() {
        let articles = ArticlesOfIncorporation::new(
            "TechCorp SA".to_string(),
            CompanyType::SA,
            Capital::new(100_000),
        )
        .with_business_purpose("Software development".to_string())
        .with_head_office("Paris, France".to_string())
        .with_shareholder(Shareholder::new("Founder".to_string(), 1000, 100_000));

        assert!(validate_articles_of_incorporation(&articles).is_ok());
    }

    #[test]
    fn test_insufficient_capital_sa() {
        let articles = ArticlesOfIncorporation::new(
            "Company SA".to_string(),
            CompanyType::SA,
            Capital::new(30_000), // Below €37,000 minimum
        )
        .with_business_purpose("Business".to_string())
        .with_head_office("Paris".to_string())
        .with_shareholder(Shareholder::new("Owner".to_string(), 100, 30_000));

        let result = validate_articles_of_incorporation(&articles);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CompanyLawError::InsufficientCapital { .. }
        ));
    }

    #[test]
    fn test_invalid_company_name() {
        let articles = ArticlesOfIncorporation::new(
            "MyCompany".to_string(), // Missing SA suffix
            CompanyType::SA,
            Capital::new(100_000),
        )
        .with_business_purpose("Business".to_string())
        .with_head_office("Paris".to_string())
        .with_shareholder(Shareholder::new("Owner".to_string(), 100, 100_000));

        let result = validate_articles_of_incorporation(&articles);
        assert!(result.is_err());
    }

    #[test]
    fn test_too_many_sarl_partners() {
        let mut articles = ArticlesOfIncorporation::new(
            "Company SARL".to_string(),
            CompanyType::SARL,
            Capital::new(10_000),
        )
        .with_business_purpose("Business".to_string())
        .with_head_office("Paris".to_string());

        // Add 101 partners
        for i in 1..=101 {
            articles =
                articles.with_shareholder(Shareholder::new(format!("Partner {}", i), 10, 100));
        }

        let result = validate_articles_of_incorporation(&articles);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CompanyLawError::TooManyPartners { .. }
        ));
    }

    #[test]
    fn test_missing_business_purpose() {
        let articles = ArticlesOfIncorporation::new(
            "Company SA".to_string(),
            CompanyType::SA,
            Capital::new(100_000),
        )
        .with_head_office("Paris".to_string())
        .with_shareholder(Shareholder::new("Owner".to_string(), 100, 100_000));
        // Business purpose not added

        let result = validate_articles_of_incorporation(&articles);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_sa_board() {
        let board = BoardOfDirectors::new()
            .with_director(Director::new(
                "Director 1".to_string(),
                Utc::now().naive_utc().date(),
                6,
            ))
            .with_director(Director::new(
                "Director 2".to_string(),
                Utc::now().naive_utc().date(),
                6,
            ))
            .with_director(Director::new(
                "Director 3".to_string(),
                Utc::now().naive_utc().date(),
                6,
            ))
            .with_chairman("Director 1".to_string());

        assert!(validate_sa_board(&board).is_ok());
    }

    #[test]
    fn test_invalid_board_size() {
        let board = BoardOfDirectors::new()
            .with_director(Director::new(
                "Director 1".to_string(),
                Utc::now().naive_utc().date(),
                6,
            ))
            .with_director(Director::new(
                "Director 2".to_string(),
                Utc::now().naive_utc().date(),
                6,
            ));
        // Only 2 directors - need at least 3

        let result = validate_sa_board(&board);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CompanyLawError::InvalidBoardSize { .. }
        ));
    }

    #[test]
    fn test_director_term_too_long() {
        let board = BoardOfDirectors::new()
            .with_director(Director::new(
                "Director 1".to_string(),
                Utc::now().naive_utc().date(),
                10, // Exceeds 6 year maximum
            ))
            .with_director(Director::new(
                "Director 2".to_string(),
                Utc::now().naive_utc().date(),
                6,
            ))
            .with_director(Director::new(
                "Director 3".to_string(),
                Utc::now().naive_utc().date(),
                6,
            ));

        let result = validate_sa_board(&board);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_shareholders_meeting_ordinary() {
        let meeting = ShareholdersMeeting::new(
            MeetingType::OrdinaryGeneralMeeting,
            Utc::now().naive_utc().date(),
            10_000,
        )
        .with_votes(3_000, 2_000, 500, 500); // 30% quorum, 80% approval

        let result = validate_shareholders_meeting(&meeting, ResolutionType::Ordinary, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_shareholders_meeting_quorum_not_met() {
        let meeting = ShareholdersMeeting::new(
            MeetingType::OrdinaryGeneralMeeting,
            Utc::now().naive_utc().date(),
            10_000,
        )
        .with_votes(1_500, 1_000, 300, 200); // Only 15% quorum (need 20%)

        let result = validate_shareholders_meeting(&meeting, ResolutionType::Ordinary, false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CompanyLawError::QuorumNotMet { .. }
        ));
    }

    #[test]
    fn test_shareholders_meeting_resolution_not_approved() {
        let meeting = ShareholdersMeeting::new(
            MeetingType::ExtraordinaryGeneralMeeting,
            Utc::now().naive_utc().date(),
            10_000,
        )
        .with_votes(3_000, 1_500, 1_200, 300); // 30% quorum, but only 55.6% approval (need > 66.67%)

        let result = validate_shareholders_meeting(&meeting, ResolutionType::Special, false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CompanyLawError::ResolutionNotApproved { .. }
        ));
    }

    #[test]
    fn test_shareholders_meeting_second_call_no_quorum() {
        let meeting = ShareholdersMeeting::new(
            MeetingType::OrdinaryGeneralMeeting,
            Utc::now().naive_utc().date(),
            10_000,
        )
        .with_votes(500, 400, 50, 50); // Only 5% quorum, but second call has no minimum

        let result = validate_shareholders_meeting(&meeting, ResolutionType::Ordinary, true);
        assert!(result.is_ok()); // Should pass on second call
    }
}
