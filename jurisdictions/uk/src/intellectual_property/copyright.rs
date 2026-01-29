//! Copyright, Designs and Patents Act 1988 (CDPA) Implementation
//!
//! ## Copyright Protection (s.1)
//!
//! Copyright subsists in:
//! - **Literary works** (including computer programs)
//! - **Dramatic works**
//! - **Musical works**
//! - **Artistic works**
//! - **Films**
//! - **Sound recordings**
//! - **Broadcasts**
//! - **Typographical arrangements of published editions**
//!
//! ## Requirements
//!
//! 1. **Originality**: Author's own intellectual creation
//! 2. **Fixation**: Recorded in writing or otherwise
//! 3. **Qualifying person**: Author is UK national/resident or work first published in UK
//!
//! ## Duration (Chapter 6)
//!
//! - Literary/dramatic/musical/artistic: Life + 70 years
//! - Computer-generated: 50 years from creation
//! - Films: 70 years from death of last survivor (director, screenplay, dialogue, composer)
//! - Sound recordings: 50 years from creation (70 if published)
//! - Broadcasts: 50 years from broadcast
//!
//! ## Fair Dealing (ss.29-30)
//!
//! Permitted uses without infringement:
//! - Research and private study (s.29)
//! - Criticism and review (s.30)
//! - News reporting (s.30)

use super::error::{IpError, IpResult};
use super::types::IpOwner;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// Copyright work
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyrightWork {
    /// Title of work
    pub title: String,
    /// Type of work
    pub work_type: CopyrightWorkType,
    /// Author(s)
    pub authors: Vec<String>,
    /// Creation date
    pub creation_date: NaiveDate,
    /// Publication date (if published)
    pub publication_date: Option<NaiveDate>,
    /// Owner (may differ from author due to assignment)
    pub owner: IpOwner,
    /// Is it original?
    pub is_original: bool,
    /// Country of first publication
    pub country_of_publication: Option<String>,
}

/// Type of copyright work (CDPA s.1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CopyrightWorkType {
    /// Literary work (including computer programs, tables, compilations)
    Literary,
    /// Dramatic work
    Dramatic,
    /// Musical work
    Musical,
    /// Artistic work (graphic, photographic, sculpture, architecture, artistic craftsmanship)
    Artistic,
    /// Film
    Film,
    /// Sound recording
    SoundRecording,
    /// Broadcast
    Broadcast,
    /// Typographical arrangement of published edition
    TypographicalArrangement,
    /// Computer-generated work
    ComputerGenerated,
}

/// Copyright duration result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyrightDuration {
    /// Work type
    pub work_type: CopyrightWorkType,
    /// Start date of protection
    pub protection_start: NaiveDate,
    /// End date of protection
    pub protection_end: NaiveDate,
    /// Duration in years
    pub duration_years: u32,
    /// Is copyright still valid?
    pub still_valid: bool,
}

/// Fair dealing purpose (ss.29-30)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FairDealingPurpose {
    /// Research for non-commercial purpose (s.29)
    ResearchNonCommercial,
    /// Private study (s.29)
    PrivateStudy,
    /// Criticism or review (s.30)
    CriticismOrReview,
    /// Quotation (s.30(1ZA))
    Quotation,
    /// News reporting (s.30)
    NewsReporting,
}

/// Rights in performances (Chapter II)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceRight {
    /// Performer name
    pub performer: String,
    /// Performance date
    pub performance_date: NaiveDate,
    /// Recording date
    pub recording_date: Option<NaiveDate>,
    /// Duration (50 years from performance or release)
    pub protection_end: NaiveDate,
}

/// Validates if work qualifies for copyright protection
pub fn validate_copyright_work(work: &CopyrightWork) -> IpResult<()> {
    // Check originality
    if !work.is_original {
        return Err(IpError::LacksOriginality);
    }

    // Check if it's a recognized copyright work type
    if matches!(work.work_type, CopyrightWorkType::ComputerGenerated) && work.authors.is_empty() {
        // Computer-generated works don't need human author
    } else if work.authors.is_empty() {
        return Err(IpError::MissingInformation {
            field: "author".to_string(),
        });
    }

    // Check that work is fixed (evidenced by having a creation date)
    if work.creation_date.year() < 1900 {
        return Err(IpError::MissingInformation {
            field: "valid_creation_date".to_string(),
        });
    }

    Ok(())
}

/// Calculates copyright duration
pub fn calculate_copyright_duration(
    work: &CopyrightWork,
    author_death_date: Option<NaiveDate>,
) -> IpResult<CopyrightDuration> {
    let (protection_start, duration_years) = match work.work_type {
        CopyrightWorkType::Literary
        | CopyrightWorkType::Dramatic
        | CopyrightWorkType::Musical
        | CopyrightWorkType::Artistic => {
            // Life + 70 years
            if let Some(death_date) = author_death_date {
                (death_date, 70)
            } else {
                // Unknown death date - assume creation + 140 years (70 + average 70 lifespan)
                (work.creation_date, 140)
            }
        }
        CopyrightWorkType::ComputerGenerated => {
            // 50 years from creation
            (work.creation_date, 50)
        }
        CopyrightWorkType::Film => {
            // 70 years from death of last to die: director, author of screenplay,
            // author of dialogue, composer of music
            // Simplified: use 70 years from creation if no death date provided
            if let Some(death_date) = author_death_date {
                (death_date, 70)
            } else {
                (work.creation_date, 70)
            }
        }
        CopyrightWorkType::SoundRecording => {
            // 50 years from creation, or 70 years if published/communicated
            let duration = if work.publication_date.is_some() {
                70
            } else {
                50
            };
            (work.creation_date, duration)
        }
        CopyrightWorkType::Broadcast => {
            // 50 years from first broadcast
            (work.creation_date, 50)
        }
        CopyrightWorkType::TypographicalArrangement => {
            // 25 years from first publication
            if let Some(pub_date) = work.publication_date {
                (pub_date, 25)
            } else {
                return Err(IpError::MissingInformation {
                    field: "publication_date".to_string(),
                });
            }
        }
    };

    // Calculate end date (end of calendar year, not exact date)
    let end_year = protection_start.year() + duration_years as i32;
    let protection_end = NaiveDate::from_ymd_opt(end_year, 12, 31)
        .expect("valid date constant: December 31st is always valid");

    let today = chrono::Local::now().date_naive();
    let still_valid = today <= protection_end;

    Ok(CopyrightDuration {
        work_type: work.work_type,
        protection_start,
        protection_end,
        duration_years,
        still_valid,
    })
}

/// Checks if use qualifies as fair dealing
pub fn check_fair_dealing(
    purpose: FairDealingPurpose,
    use_description: &str,
    is_commercial: bool,
) -> IpResult<bool> {
    match purpose {
        FairDealingPurpose::ResearchNonCommercial => {
            if is_commercial {
                Ok(false) // Must be non-commercial
            } else {
                Ok(true)
            }
        }
        FairDealingPurpose::PrivateStudy => Ok(!is_commercial),
        FairDealingPurpose::CriticismOrReview => {
            // Must be accompanied by sufficient acknowledgment
            Ok(use_description.contains("acknowledgment") || use_description.contains("source"))
        }
        FairDealingPurpose::Quotation => {
            // Must be fair extent and accompanied by acknowledgment
            Ok(use_description.contains("short extract")
                && (use_description.contains("acknowledgment")
                    || use_description.contains("source")))
        }
        FairDealingPurpose::NewsReporting => {
            // Must be for news reporting and accompanied by acknowledgment
            Ok(use_description.contains("news") || use_description.contains("reporting"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_work() -> CopyrightWork {
        CopyrightWork {
            title: "Test Novel".to_string(),
            work_type: CopyrightWorkType::Literary,
            authors: vec!["Jane Author".to_string()],
            creation_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            publication_date: Some(NaiveDate::from_ymd_opt(2020, 6, 1).unwrap()),
            owner: IpOwner {
                name: "Jane Author".to_string(),
                owner_type: super::super::types::OwnerType::Individual,
                address: None,
                country: "GB".to_string(),
            },
            is_original: true,
            country_of_publication: Some("GB".to_string()),
        }
    }

    #[test]
    fn test_validate_copyright_work_valid() {
        let work = create_test_work();
        assert!(validate_copyright_work(&work).is_ok());
    }

    #[test]
    fn test_validate_copyright_work_lacks_originality() {
        let mut work = create_test_work();
        work.is_original = false;

        let result = validate_copyright_work(&work);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), IpError::LacksOriginality));
    }

    #[test]
    fn test_calculate_duration_literary_work() {
        let work = create_test_work();
        let author_death = NaiveDate::from_ymd_opt(2050, 12, 31).unwrap();

        let duration = calculate_copyright_duration(&work, Some(author_death)).unwrap();

        assert_eq!(duration.duration_years, 70);
        assert_eq!(duration.protection_end.year(), 2120); // 2050 + 70
        assert!(duration.still_valid); // Assuming test runs before 2120
    }

    #[test]
    fn test_calculate_duration_computer_generated() {
        let mut work = create_test_work();
        work.work_type = CopyrightWorkType::ComputerGenerated;

        let duration = calculate_copyright_duration(&work, None).unwrap();

        assert_eq!(duration.duration_years, 50);
        assert_eq!(duration.protection_end.year(), 2070); // 2020 + 50
    }

    #[test]
    fn test_calculate_duration_sound_recording_unpublished() {
        let mut work = create_test_work();
        work.work_type = CopyrightWorkType::SoundRecording;
        work.publication_date = None;

        let duration = calculate_copyright_duration(&work, None).unwrap();
        assert_eq!(duration.duration_years, 50);
    }

    #[test]
    fn test_calculate_duration_sound_recording_published() {
        let mut work = create_test_work();
        work.work_type = CopyrightWorkType::SoundRecording;
        work.publication_date = Some(NaiveDate::from_ymd_opt(2020, 6, 1).unwrap());

        let duration = calculate_copyright_duration(&work, None).unwrap();
        assert_eq!(duration.duration_years, 70);
    }

    #[test]
    fn test_fair_dealing_research_non_commercial() {
        let is_fair = check_fair_dealing(
            FairDealingPurpose::ResearchNonCommercial,
            "Academic research project",
            false,
        )
        .unwrap();
        assert!(is_fair);
    }

    #[test]
    fn test_fair_dealing_research_commercial() {
        let is_fair = check_fair_dealing(
            FairDealingPurpose::ResearchNonCommercial,
            "Commercial research",
            true,
        )
        .unwrap();
        assert!(!is_fair); // Must be non-commercial
    }

    #[test]
    fn test_fair_dealing_criticism_with_acknowledgment() {
        let is_fair = check_fair_dealing(
            FairDealingPurpose::CriticismOrReview,
            "Review of the book with acknowledgment of source",
            false,
        )
        .unwrap();
        assert!(is_fair);
    }

    #[test]
    fn test_fair_dealing_quotation() {
        let is_fair = check_fair_dealing(
            FairDealingPurpose::Quotation,
            "short extract quoted with source attribution",
            false,
        )
        .unwrap();
        assert!(is_fair);
    }
}
