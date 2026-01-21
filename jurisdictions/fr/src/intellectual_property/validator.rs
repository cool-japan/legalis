//! Validation functions for French Intellectual Property Law
//!
//! This module provides comprehensive validation functions for patents,
//! copyrights, trademarks, and designs under French IP law.

use chrono::NaiveDate;

use super::error::{
    CopyrightErrorKind, DesignErrorKind, IPLawError, IPLawResult, PatentErrorKind,
    TrademarkErrorKind,
};
use super::types::{Copyright, Design, Patent, Trademark};

/// Validate a patent under Articles L611-10 and L611-11
///
/// Checks:
/// - Novelty requirement (Article L611-10 §1)
/// - Inventive step requirement (Article L611-10 §2)
/// - Industrial applicability requirement (Article L611-10 §3)
/// - Duration (20 years from filing, Article L611-11)
pub fn validate_patent(patent: &Patent, current_date: NaiveDate) -> IPLawResult<()> {
    // Article L611-10 §1: Novelty requirement
    if !patent.novelty {
        return Err(IPLawError::PatentError(PatentErrorKind::LackOfNovelty));
    }

    // Article L611-10 §2: Inventive step requirement
    if !patent.inventive_step {
        return Err(IPLawError::PatentError(
            PatentErrorKind::LackOfInventiveStep,
        ));
    }

    // Article L611-10 §3: Industrial applicability requirement
    if !patent.industrial_applicability {
        return Err(IPLawError::PatentError(
            PatentErrorKind::LackOfIndustrialApplicability,
        ));
    }

    // Article L611-11: 20-year duration check
    if patent.is_expired(current_date) {
        return Err(IPLawError::PatentError(PatentErrorKind::PatentExpired));
    }

    // Validate grant date is after filing date
    if let Some(grant_date) = patent.grant_date
        && grant_date < patent.filing_date
    {
        return Err(IPLawError::PatentError(PatentErrorKind::InvalidGrantDate));
    }

    Ok(())
}

/// Validate patent novelty (Article L611-10 §1)
///
/// A patent is novel if it is not part of the state of the art.
/// State of the art comprises everything made available to the public
/// before the filing date.
pub fn validate_patent_novelty(patent: &Patent) -> IPLawResult<()> {
    if !patent.novelty {
        Err(IPLawError::PatentError(PatentErrorKind::LackOfNovelty))
    } else {
        Ok(())
    }
}

/// Validate patent inventive step (Article L611-10 §2)
///
/// A patent involves an inventive step if it is not obvious to a person
/// skilled in the art, having regard to the state of the art.
pub fn validate_patent_inventive_step(patent: &Patent) -> IPLawResult<()> {
    if !patent.inventive_step {
        Err(IPLawError::PatentError(
            PatentErrorKind::LackOfInventiveStep,
        ))
    } else {
        Ok(())
    }
}

/// Validate patent industrial applicability (Article L611-10 §3)
///
/// A patent is industrially applicable if it can be made or used in any
/// kind of industry, including agriculture.
pub fn validate_patent_industrial_applicability(patent: &Patent) -> IPLawResult<()> {
    if !patent.industrial_applicability {
        Err(IPLawError::PatentError(
            PatentErrorKind::LackOfIndustrialApplicability,
        ))
    } else {
        Ok(())
    }
}

/// Validate patent duration (Article L611-11)
///
/// Patents are granted for 20 years from the filing date.
pub fn validate_patent_duration(patent: &Patent, current_date: NaiveDate) -> IPLawResult<()> {
    if patent.is_expired(current_date) {
        Err(IPLawError::PatentError(PatentErrorKind::PatentExpired))
    } else {
        Ok(())
    }
}

/// Validate copyright under Articles L122-1 and L123-1
///
/// Checks:
/// - Originality (implicit in Article L111-1)
/// - Duration (70 years post-mortem, Article L123-1)
pub fn validate_copyright(copyright: &Copyright, current_date: NaiveDate) -> IPLawResult<()> {
    // Article L123-1: 70-year post-mortem duration check
    if copyright.is_expired(current_date) {
        return Err(IPLawError::CopyrightError(
            CopyrightErrorKind::CopyrightExpired,
        ));
    }

    // Validate death date is after creation date
    if let Some(death_date) = copyright.author_death_date
        && death_date < copyright.creation_date
    {
        return Err(IPLawError::CopyrightError(
            CopyrightErrorKind::InvalidDeathDate,
        ));
    }

    Ok(())
}

/// Validate copyright duration (Article L123-1)
///
/// Copyright protection lasts for 70 years after the author's death.
/// For collaborative works, it's 70 years after the last surviving author's death.
pub fn validate_copyright_duration(
    copyright: &Copyright,
    current_date: NaiveDate,
) -> IPLawResult<()> {
    if copyright.is_expired(current_date) {
        Err(IPLawError::CopyrightError(
            CopyrightErrorKind::CopyrightExpired,
        ))
    } else {
        Ok(())
    }
}

/// Validate copyright originality
///
/// Copyright protects only original works (Article L111-1).
/// Originality means the work bears the imprint of the author's personality.
pub fn validate_copyright_originality(copyright: &Copyright) -> IPLawResult<()> {
    // In practice, originality is presumed unless challenged
    // This is a simplified validation
    if copyright.work_title.is_empty() || copyright.author.is_empty() {
        Err(IPLawError::CopyrightError(
            CopyrightErrorKind::LackOfOriginality,
        ))
    } else {
        Ok(())
    }
}

/// Validate trademark under Articles L711-1 and L712-1
///
/// Checks:
/// - Distinctiveness requirement (Article L711-1)
/// - Duration (10 years renewable, Article L712-1)
/// - Valid Nice classes (1-45)
pub fn validate_trademark(trademark: &Trademark, current_date: NaiveDate) -> IPLawResult<()> {
    // Article L711-1: Distinctiveness requirement
    if !trademark.distinctiveness {
        return Err(IPLawError::TrademarkError(
            TrademarkErrorKind::LackOfDistinctiveness,
        ));
    }

    // Article L712-1: 10-year duration check
    if trademark.is_expired(current_date) {
        return Err(IPLawError::TrademarkError(
            TrademarkErrorKind::TrademarkExpired,
        ));
    }

    // Validate Nice classes
    if !trademark.has_valid_classes() {
        return Err(IPLawError::TrademarkError(
            TrademarkErrorKind::InvalidClasses,
        ));
    }

    Ok(())
}

/// Validate trademark distinctiveness (Article L711-1)
///
/// A trademark must be distinctive to identify goods or services.
/// Generic or descriptive terms cannot be registered.
pub fn validate_trademark_distinctiveness(trademark: &Trademark) -> IPLawResult<()> {
    if !trademark.distinctiveness {
        Err(IPLawError::TrademarkError(
            TrademarkErrorKind::LackOfDistinctiveness,
        ))
    } else {
        Ok(())
    }
}

/// Validate trademark duration (Article L712-1)
///
/// Trademark protection lasts for 10 years from registration,
/// renewable indefinitely.
pub fn validate_trademark_duration(
    trademark: &Trademark,
    current_date: NaiveDate,
) -> IPLawResult<()> {
    if trademark.is_expired(current_date) {
        Err(IPLawError::TrademarkError(
            TrademarkErrorKind::TrademarkExpired,
        ))
    } else {
        Ok(())
    }
}

/// Validate trademark classes (Nice Classification)
///
/// Nice Classification has 45 classes (1-34 for goods, 35-45 for services).
pub fn validate_trademark_classes(trademark: &Trademark) -> IPLawResult<()> {
    if !trademark.has_valid_classes() {
        Err(IPLawError::TrademarkError(
            TrademarkErrorKind::InvalidClasses,
        ))
    } else {
        Ok(())
    }
}

/// Validate design under Articles L511-1 and L513-1
///
/// Checks:
/// - Novelty requirement (Article L511-1 §1)
/// - Individual character requirement (Article L511-1 §2)
/// - Duration (max 25 years, Article L513-1)
pub fn validate_design(design: &Design, current_date: NaiveDate) -> IPLawResult<()> {
    // Article L511-1 §1: Novelty requirement
    if !design.novelty {
        return Err(IPLawError::DesignError(DesignErrorKind::LackOfNovelty));
    }

    // Article L511-1 §2: Individual character requirement
    if !design.individual_character {
        return Err(IPLawError::DesignError(
            DesignErrorKind::LackOfIndividualCharacter,
        ));
    }

    // Article L513-1: 25-year max duration check
    if design.is_expired(current_date) {
        return Err(IPLawError::DesignError(DesignErrorKind::DesignExpired));
    }

    Ok(())
}

/// Validate design novelty (Article L511-1 §1)
///
/// A design is novel if no identical design has been made available
/// to the public before the filing date.
pub fn validate_design_novelty(design: &Design) -> IPLawResult<()> {
    if !design.novelty {
        Err(IPLawError::DesignError(DesignErrorKind::LackOfNovelty))
    } else {
        Ok(())
    }
}

/// Validate design individual character (Article L511-1 §2)
///
/// A design has individual character if the overall impression it produces
/// on an informed user differs from the overall impression produced by any
/// design made available to the public.
pub fn validate_design_individual_character(design: &Design) -> IPLawResult<()> {
    if !design.individual_character {
        Err(IPLawError::DesignError(
            DesignErrorKind::LackOfIndividualCharacter,
        ))
    } else {
        Ok(())
    }
}

/// Validate design duration (Article L513-1)
///
/// Design protection lasts for one or more periods of 5 years,
/// up to a maximum of 25 years from filing.
pub fn validate_design_duration(design: &Design, current_date: NaiveDate) -> IPLawResult<()> {
    if design.is_expired(current_date) {
        Err(IPLawError::DesignError(DesignErrorKind::DesignExpired))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_patent_success() {
        let patent = Patent::builder()
            .title("Test".to_string())
            .inventor("Test".to_string())
            .filing_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .novelty(true)
            .inventive_step(true)
            .industrial_applicability(true)
            .build()
            .unwrap();

        let current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert!(validate_patent(&patent, current).is_ok());
    }

    #[test]
    fn test_validate_patent_lack_of_novelty() {
        let patent = Patent::builder()
            .title("Test".to_string())
            .inventor("Test".to_string())
            .filing_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .novelty(false)
            .build()
            .unwrap();

        let current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let result = validate_patent(&patent, current);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            IPLawError::PatentError(PatentErrorKind::LackOfNovelty)
        ));
    }

    #[test]
    fn test_validate_patent_expired() {
        let patent = Patent::builder()
            .title("Test".to_string())
            .inventor("Test".to_string())
            .filing_date(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap())
            .novelty(true)
            .inventive_step(true)
            .industrial_applicability(true)
            .build()
            .unwrap();

        let current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let result = validate_patent(&patent, current);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            IPLawError::PatentError(PatentErrorKind::PatentExpired)
        ));
    }

    #[test]
    fn test_validate_copyright_success() {
        let copyright = Copyright::builder()
            .work_title("Test".to_string())
            .author("Test".to_string())
            .creation_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .work_type(super::super::types::WorkType::Literary)
            .build()
            .unwrap();

        let current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert!(validate_copyright(&copyright, current).is_ok());
    }

    #[test]
    fn test_validate_copyright_expired() {
        let copyright = Copyright::builder()
            .work_title("Test".to_string())
            .author("Test".to_string())
            .creation_date(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap())
            .author_death_date(NaiveDate::from_ymd_opt(1950, 1, 1).unwrap())
            .work_type(super::super::types::WorkType::Literary)
            .build()
            .unwrap();

        let current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let result = validate_copyright(&copyright, current);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            IPLawError::CopyrightError(CopyrightErrorKind::CopyrightExpired)
        ));
    }

    #[test]
    fn test_validate_trademark_success() {
        let trademark = Trademark::builder()
            .mark("Test".to_string())
            .owner("Test".to_string())
            .registration_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .classes(vec![9, 35])
            .distinctiveness(true)
            .build()
            .unwrap();

        let current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert!(validate_trademark(&trademark, current).is_ok());
    }

    #[test]
    fn test_validate_trademark_lack_distinctiveness() {
        let trademark = Trademark::builder()
            .mark("Test".to_string())
            .owner("Test".to_string())
            .registration_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .classes(vec![9])
            .distinctiveness(false)
            .build()
            .unwrap();

        let current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let result = validate_trademark(&trademark, current);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            IPLawError::TrademarkError(TrademarkErrorKind::LackOfDistinctiveness)
        ));
    }

    #[test]
    fn test_validate_design_success() {
        let design = Design::builder()
            .title("Test".to_string())
            .creator("Test".to_string())
            .filing_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .novelty(true)
            .individual_character(true)
            .build()
            .unwrap();

        let current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert!(validate_design(&design, current).is_ok());
    }

    #[test]
    fn test_validate_design_lack_individual_character() {
        let design = Design::builder()
            .title("Test".to_string())
            .creator("Test".to_string())
            .filing_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .novelty(true)
            .individual_character(false)
            .build()
            .unwrap();

        let current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let result = validate_design(&design, current);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            IPLawError::DesignError(DesignErrorKind::LackOfIndividualCharacter)
        ));
    }
}
