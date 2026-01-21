//! Copyright Act 1968 Implementation
//!
//! Australian copyright law protecting original works and other subject matter.
//!
//! ## Protected Works (Part III)
//!
//! - Literary works (s.10)
//! - Dramatic works (s.10)
//! - Musical works (s.10)
//! - Artistic works (s.10)
//!
//! ## Other Subject Matter (Part IV)
//!
//! - Sound recordings (s.89)
//! - Cinematograph films (s.90)
//! - Television and sound broadcasts (s.91)
//! - Published editions (s.92)
//!
//! ## Key Cases
//!
//! - **IceTV v Nine Network (2009)**: Originality requires independent intellectual effort
//! - **Telstra v Phone Directories (2010)**: Databases and originality
//! - **Roadshow v iiNet (2012)**: Authorization of infringement

use super::error::{IpError, Result};
use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Copyright work under Australian law
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyrightWork {
    /// Type of work
    pub work_type: WorkType,
    /// Title
    pub title: String,
    /// Author(s)
    pub authors: Vec<Author>,
    /// Date of creation
    pub creation_date: NaiveDate,
    /// Date of first publication (if published)
    pub publication_date: Option<NaiveDate>,
    /// Country of first publication
    pub publication_country: Option<String>,
    /// Whether work is original
    pub is_original: bool,
    /// Description of originality
    pub originality_description: String,
}

/// Type of copyright work
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkType {
    /// Literary work (includes computer programs)
    LiteraryWork,
    /// Dramatic work
    DramaticWork,
    /// Musical work
    MusicalWork,
    /// Artistic work
    ArtisticWork,
    /// Computer program (literary work subtype)
    ComputerProgram,
}

/// Part IV subject matter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartIVSubject {
    /// Sound recording (s.89)
    SoundRecording,
    /// Cinematograph film (s.90)
    Film,
    /// Television broadcast (s.91)
    TvBroadcast,
    /// Sound broadcast (s.91)
    SoundBroadcast,
    /// Published edition (s.92)
    PublishedEdition,
}

/// Author of a work
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Author {
    /// Author name
    pub name: String,
    /// Author type
    pub author_type: AuthorType,
    /// Nationality/residence
    pub nationality: String,
    /// Date of death (if applicable)
    pub death_date: Option<NaiveDate>,
}

/// Type of author
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthorType {
    /// Individual author
    Individual,
    /// Joint authors
    JointAuthor,
    /// Employee (work for hire)
    Employee,
    /// Company (deemed author for certain works)
    Company,
    /// Crown (government)
    Crown,
    /// Unknown author
    Unknown,
}

/// Duration of copyright
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyrightDuration {
    /// Start date of protection
    pub start_date: NaiveDate,
    /// End date of protection
    pub end_date: NaiveDate,
    /// Basis for calculation
    pub basis: DurationBasis,
    /// Is copyright currently subsisting?
    pub is_subsisting: bool,
}

/// Basis for copyright duration calculation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DurationBasis {
    /// Life of author + 70 years (s.33)
    LifePlus70,
    /// 70 years from first publication (unpublished works)
    SeventyFromPublication,
    /// 70 years from making (Part IV subject matter)
    SeventyFromMaking,
    /// Crown copyright (50 years from creation)
    CrownCopyright,
}

/// Fair dealing purpose (ss.40-43)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FairDealingPurpose {
    /// Research or study (s.40)
    ResearchOrStudy,
    /// Criticism or review (s.41)
    CriticismOrReview,
    /// Parody or satire (s.41A)
    ParodyOrSatire,
    /// Reporting news (s.42)
    ReportingNews,
    /// Judicial proceedings (s.43)
    JudicialProceedings,
    /// Professional advice (s.43)
    ProfessionalAdvice,
}

impl FairDealingPurpose {
    /// Get the relevant section
    pub fn section(&self) -> &'static str {
        match self {
            FairDealingPurpose::ResearchOrStudy => "s.40",
            FairDealingPurpose::CriticismOrReview => "s.41",
            FairDealingPurpose::ParodyOrSatire => "s.41A",
            FairDealingPurpose::ReportingNews => "s.42",
            FairDealingPurpose::JudicialProceedings => "s.43",
            FairDealingPurpose::ProfessionalAdvice => "s.43",
        }
    }

    /// Whether sufficient acknowledgment is required
    pub fn requires_acknowledgment(&self) -> bool {
        matches!(
            self,
            FairDealingPurpose::CriticismOrReview | FairDealingPurpose::ReportingNews
        )
    }
}

/// Moral rights (Part IX)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoralRights {
    /// Right of attribution (s.193)
    pub attribution: bool,
    /// Right against false attribution (s.195AC)
    pub against_false_attribution: bool,
    /// Right of integrity (s.195AI)
    pub integrity: bool,
    /// Whether moral rights have been consented to
    pub consent_given: bool,
}

/// Fair dealing assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FairDealingAssessment {
    /// Purpose of use
    pub purpose: FairDealingPurpose,
    /// Is fair dealing?
    pub is_fair_dealing: bool,
    /// Factors considered
    pub factors: FairDealingFactors,
    /// Analysis
    pub analysis: String,
}

/// Factors for fair dealing (s.40(2))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FairDealingFactors {
    /// Purpose and character of dealing
    pub purpose_character: String,
    /// Nature of work
    pub nature_of_work: String,
    /// Amount and substantiality taken
    pub amount_taken: String,
    /// Effect on potential market
    pub market_effect: String,
    /// Whether it was possible to obtain at ordinary price
    pub could_obtain_normally: bool,
}

/// Check if copyright subsists in a work
pub fn check_copyright_subsistence(work: &CopyrightWork) -> Result<bool> {
    // Check originality (IceTV test)
    if !work.is_original {
        return Err(IpError::LacksOriginality);
    }

    // Check work type is protected
    let valid_work = matches!(
        work.work_type,
        WorkType::LiteraryWork
            | WorkType::DramaticWork
            | WorkType::MusicalWork
            | WorkType::ArtisticWork
            | WorkType::ComputerProgram
    );

    if !valid_work {
        return Err(IpError::NotCopyrightWork {
            work_type: format!("{:?}", work.work_type),
        });
    }

    // Check authorship qualification
    let has_qualified_author = work.authors.iter().any(|a| {
        a.nationality == "AU"
            || a.nationality == "Australia"
            || is_qualifying_country(&a.nationality)
    });

    if !has_qualified_author && work.publication_country.as_deref() != Some("AU") {
        return Err(IpError::AuthorshipNotQualified);
    }

    Ok(true)
}

/// Check if country is a qualifying country for copyright
fn is_qualifying_country(country: &str) -> bool {
    // Berne Convention and TRIPS members
    let qualifying = [
        "AU", "US", "UK", "GB", "NZ", "CA", "FR", "DE", "JP", "SG", "IN", "CN",
    ];
    qualifying.contains(&country)
}

/// Calculate copyright duration (s.33)
pub fn calculate_copyright_duration(work: &CopyrightWork) -> CopyrightDuration {
    let today = Utc::now().date_naive();

    // Find last surviving author's death date
    let last_death = work.authors.iter().filter_map(|a| a.death_date).max();

    let (end_date, basis) = if work
        .authors
        .iter()
        .any(|a| a.author_type == AuthorType::Crown)
    {
        // Crown copyright: 50 years from creation
        let end = work
            .creation_date
            .with_year(work.creation_date.year() + 50)
            .unwrap_or(work.creation_date);
        (end, DurationBasis::CrownCopyright)
    } else if let Some(death) = last_death {
        // Life + 70 years
        let end = death.with_year(death.year() + 70).unwrap_or(death);
        (end, DurationBasis::LifePlus70)
    } else if let Some(pub_date) = work.publication_date {
        // 70 years from first publication
        let end = pub_date.with_year(pub_date.year() + 70).unwrap_or(pub_date);
        (end, DurationBasis::SeventyFromPublication)
    } else {
        // Unpublished: 70 years from creation (simplified)
        let end = work
            .creation_date
            .with_year(work.creation_date.year() + 70)
            .unwrap_or(work.creation_date);
        (end, DurationBasis::SeventyFromMaking)
    };

    CopyrightDuration {
        start_date: work.creation_date,
        end_date,
        basis,
        is_subsisting: end_date > today,
    }
}

/// Check fair dealing exception
pub fn check_fair_dealing(
    purpose: FairDealingPurpose,
    factors: &FairDealingFactors,
) -> FairDealingAssessment {
    // Assess fairness based on factors (simplified)
    let is_fair = match purpose {
        FairDealingPurpose::ResearchOrStudy => {
            // 10% or one chapter rule for research/study
            factors.amount_taken.contains("10%")
                || factors.amount_taken.contains("one chapter")
                || factors.amount_taken.contains("reasonable portion")
        }
        FairDealingPurpose::CriticismOrReview | FairDealingPurpose::ReportingNews => {
            // Must be fair dealing + sufficient acknowledgment
            !factors.market_effect.contains("significant harm")
        }
        FairDealingPurpose::ParodyOrSatire => {
            // Parody/satire must be genuine
            factors.purpose_character.contains("parody")
                || factors.purpose_character.contains("satire")
        }
        FairDealingPurpose::JudicialProceedings | FairDealingPurpose::ProfessionalAdvice => {
            // Generally allowed for legal purposes
            true
        }
    };

    let analysis = format!(
        "Fair dealing assessment for {} under Copyright Act 1968 {}. \
         {}",
        format!("{:?}", purpose).to_lowercase().replace('_', " "),
        purpose.section(),
        if is_fair {
            "Use qualifies as fair dealing."
        } else {
            "Use does not qualify as fair dealing."
        }
    );

    FairDealingAssessment {
        purpose,
        is_fair_dealing: is_fair,
        factors: factors.clone(),
        analysis,
    }
}

/// Check for copyright infringement (s.36)
pub fn check_infringement(
    work: &CopyrightWork,
    alleged_copy: &str,
    purpose: Option<FairDealingPurpose>,
) -> Result<bool> {
    // Check if copyright still subsists
    let duration = calculate_copyright_duration(work);
    if !duration.is_subsisting {
        return Err(IpError::CopyrightExpired {
            expiry_date: duration.end_date.to_string(),
        });
    }

    // Check fair dealing defense
    if let Some(purpose) = purpose {
        let factors = FairDealingFactors {
            purpose_character: format!("{:?}", purpose),
            nature_of_work: format!("{:?}", work.work_type),
            amount_taken: "unknown".to_string(),
            market_effect: "unknown".to_string(),
            could_obtain_normally: true,
        };

        let assessment = check_fair_dealing(purpose, &factors);
        if assessment.is_fair_dealing {
            return Err(IpError::FairDealing {
                purpose: format!("{:?}", purpose),
            });
        }
    }

    // If no defense applies, infringement may have occurred
    // In reality, this requires detailed comparison
    Ok(!alleged_copy.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_work() -> CopyrightWork {
        CopyrightWork {
            work_type: WorkType::LiteraryWork,
            title: "Test Book".to_string(),
            authors: vec![Author {
                name: "John Smith".to_string(),
                author_type: AuthorType::Individual,
                nationality: "AU".to_string(),
                death_date: None,
            }],
            creation_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            publication_date: Some(NaiveDate::from_ymd_opt(2020, 6, 1).unwrap()),
            publication_country: Some("AU".to_string()),
            is_original: true,
            originality_description: "Original literary work with independent intellectual effort"
                .to_string(),
        }
    }

    #[test]
    fn test_copyright_subsistence() {
        let work = create_test_work();
        assert!(check_copyright_subsistence(&work).is_ok());
    }

    #[test]
    fn test_copyright_not_original() {
        let mut work = create_test_work();
        work.is_original = false;

        let result = check_copyright_subsistence(&work);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), IpError::LacksOriginality));
    }

    #[test]
    fn test_copyright_duration() {
        let work = create_test_work();
        let duration = calculate_copyright_duration(&work);

        assert!(duration.is_subsisting);
        assert_eq!(duration.basis, DurationBasis::SeventyFromPublication);
    }

    #[test]
    fn test_copyright_duration_deceased_author() {
        let mut work = create_test_work();
        work.authors[0].death_date = Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());

        let duration = calculate_copyright_duration(&work);
        assert_eq!(duration.basis, DurationBasis::LifePlus70);
    }

    #[test]
    fn test_fair_dealing_research() {
        let factors = FairDealingFactors {
            purpose_character: "Academic research".to_string(),
            nature_of_work: "Literary work".to_string(),
            amount_taken: "10% of the work".to_string(),
            market_effect: "No significant harm".to_string(),
            could_obtain_normally: true,
        };

        let assessment = check_fair_dealing(FairDealingPurpose::ResearchOrStudy, &factors);
        assert!(assessment.is_fair_dealing);
    }

    #[test]
    fn test_fair_dealing_purpose_sections() {
        assert_eq!(FairDealingPurpose::ResearchOrStudy.section(), "s.40");
        assert_eq!(FairDealingPurpose::CriticismOrReview.section(), "s.41");
        assert_eq!(FairDealingPurpose::ParodyOrSatire.section(), "s.41A");
    }
}
