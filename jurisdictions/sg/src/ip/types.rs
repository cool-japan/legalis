//! Intellectual Property - Core Type Definitions
//!
//! This module provides type-safe representations of Singapore IP rights:
//! - Patents (Patents Act Cap. 221)
//! - Trademarks (Trade Marks Act Cap. 332)
//! - Copyright (Copyright Act 2021)
//! - Registered Designs (Registered Designs Act Cap. 266)

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

// ========== PATENTS ==========

/// Patent registration and validity information
///
/// ## Patents Act (Cap. 221)
/// - Term: 20 years from filing date (s. 36)
/// - Requirements: Novelty, inventive step, industrial application (s. 13-16)
/// - Grace period: 12 months before filing for disclosures (s. 14(4))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Patent {
    /// Patent application number (e.g., "SG 10201912345A")
    pub application_number: String,

    /// Title of the invention
    pub title: String,

    /// Inventor(s) name
    pub inventors: Vec<String>,

    /// Applicant/owner name
    pub applicant: String,

    /// Filing date (determines 20-year term)
    pub filing_date: NaiveDate,

    /// Grant date (if granted)
    pub grant_date: Option<NaiveDate>,

    /// Patent status
    pub status: PatentStatus,

    /// Technology field/IPC classification
    pub ipc_classification: Vec<String>,

    /// Abstract of the invention
    pub abstract_text: String,

    /// Claims defining the scope of protection
    pub claims: Vec<PatentClaim>,

    /// Prior art references
    pub prior_art: Vec<PriorArt>,
}

impl Patent {
    /// Creates a new patent application
    pub fn new(
        application_number: impl Into<String>,
        title: impl Into<String>,
        applicant: impl Into<String>,
        filing_date: NaiveDate,
    ) -> Self {
        Self {
            application_number: application_number.into(),
            title: title.into(),
            inventors: Vec::new(),
            applicant: applicant.into(),
            filing_date,
            grant_date: None,
            status: PatentStatus::Pending,
            ipc_classification: Vec::new(),
            abstract_text: String::new(),
            claims: Vec::new(),
            prior_art: Vec::new(),
        }
    }

    /// Returns the expiry date (20 years from filing)
    pub fn expiry_date(&self) -> NaiveDate {
        // Add 20 years (accounting for leap years properly)
        NaiveDate::from_ymd_opt(
            self.filing_date.year() + 20,
            self.filing_date.month(),
            self.filing_date.day(),
        )
        .unwrap_or(self.filing_date)
    }

    /// Returns whether the patent term has expired
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now().date_naive() > self.expiry_date()
    }

    /// Returns years since filing
    pub fn years_since_filing(&self) -> i32 {
        let today = chrono::Utc::now().date_naive();
        today.year() - self.filing_date.year()
    }
}

/// Patent status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PatentStatus {
    /// Application filed, awaiting examination
    Pending,
    /// Undergoing substantive examination
    UnderExamination,
    /// Granted and in force
    Granted,
    /// Lapsed (non-payment of renewal fees)
    Lapsed,
    /// Expired (20-year term ended)
    Expired,
    /// Revoked by court or IPOS
    Revoked,
    /// Withdrawn by applicant
    Withdrawn,
    /// Refused by IPOS
    Refused,
}

/// Individual patent claim
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatentClaim {
    /// Claim number (1, 2, 3...)
    pub number: u32,
    /// Claim type
    pub claim_type: ClaimType,
    /// Claim text
    pub text: String,
    /// Dependencies (e.g., claim 2 depends on claim 1)
    pub depends_on: Vec<u32>,
}

/// Type of patent claim
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClaimType {
    /// Independent claim (stands alone)
    Independent,
    /// Dependent claim (refers to other claims)
    Dependent,
}

/// Prior art reference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriorArt {
    /// Title/description of prior art
    pub description: String,
    /// Publication date
    pub date: Option<NaiveDate>,
    /// Document number (patent, publication, etc.)
    pub document_number: Option<String>,
    /// Relevance assessment
    pub relevance: PriorArtRelevance,
}

/// Prior art relevance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PriorArtRelevance {
    /// Highly relevant (X category - novelty destroyer)
    HighlyRelevant,
    /// Relevant (Y category - inventive step)
    Relevant,
    /// Background (A category - general background)
    Background,
}

// ========== TRADEMARKS ==========

/// Trademark registration information
///
/// ## Trade Marks Act (Cap. 332)
/// - Term: 10 years, renewable indefinitely (s. 18)
/// - Classification: Nice Classification (45 classes)
/// - Opposition period: 2 months after publication (s. 13)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trademark {
    /// Trademark registration/application number
    pub registration_number: String,

    /// The mark itself (word, logo description, etc.)
    pub mark: String,

    /// Owner/proprietor name
    pub proprietor: String,

    /// Nice Classification classes (1-45)
    pub classes: Vec<u8>,

    /// Goods/services description per class
    pub specifications: Vec<TrademarkSpecification>,

    /// Filing date
    pub filing_date: NaiveDate,

    /// Registration date (if registered)
    pub registration_date: Option<NaiveDate>,

    /// Trademark type
    pub mark_type: TrademarkType,

    /// Trademark status
    pub status: TrademarkStatus,

    /// Whether this is a well-known mark (s. 55)
    pub is_well_known: bool,

    /// Disclaimers (elements not exclusively claimed)
    pub disclaimers: Vec<String>,
}

impl Trademark {
    /// Creates a new trademark application
    pub fn new(
        registration_number: impl Into<String>,
        mark: impl Into<String>,
        proprietor: impl Into<String>,
        filing_date: NaiveDate,
    ) -> Self {
        Self {
            registration_number: registration_number.into(),
            mark: mark.into(),
            proprietor: proprietor.into(),
            classes: Vec::new(),
            specifications: Vec::new(),
            filing_date,
            registration_date: None,
            mark_type: TrademarkType::Word,
            status: TrademarkStatus::Pending,
            is_well_known: false,
            disclaimers: Vec::new(),
        }
    }

    /// Returns the renewal date (10 years from registration)
    pub fn renewal_date(&self) -> Option<NaiveDate> {
        self.registration_date.map(|date| {
            NaiveDate::from_ymd_opt(date.year() + 10, date.month(), date.day()).unwrap_or(date)
        })
    }

    /// Returns whether the trademark needs renewal
    pub fn needs_renewal(&self) -> bool {
        if let Some(renewal_date) = self.renewal_date() {
            chrono::Utc::now().date_naive() > renewal_date
        } else {
            false
        }
    }

    /// Returns years since registration
    pub fn years_since_registration(&self) -> Option<i32> {
        self.registration_date.map(|date| {
            let today = chrono::Utc::now().date_naive();
            today.year() - date.year()
        })
    }

    /// Calculate similarity score with another trademark (0-100%)
    pub fn similarity_score(&self, other: &Trademark) -> u8 {
        // Simple phonetic/visual similarity check
        let mark1 = self.mark.to_lowercase();
        let mark2 = other.mark.to_lowercase();

        // Exact match
        if mark1 == mark2 {
            return 100;
        }

        // Very similar (one letter difference)
        if levenshtein_distance(&mark1, &mark2) <= 1 {
            return 90;
        }

        // Phonetically similar (basic check)
        if mark1.starts_with(&mark2[..mark2.len().min(3)])
            || mark2.starts_with(&mark1[..mark1.len().min(3)])
        {
            return 70;
        }

        // Contains one another
        if mark1.contains(&mark2) || mark2.contains(&mark1) {
            return 60;
        }

        // Check for common overlap in classes
        let common_classes = self
            .classes
            .iter()
            .filter(|c| other.classes.contains(c))
            .count();

        if common_classes > 0 { 40 } else { 20 }
    }
}

/// Goods/services specification for a class
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrademarkSpecification {
    /// Nice Classification class (1-45)
    pub class: u8,
    /// Description of goods/services
    pub description: String,
}

/// Type of trademark
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrademarkType {
    /// Word mark (text only)
    Word,
    /// Device mark (logo/image)
    Device,
    /// Composite (word + device)
    Composite,
    /// Shape mark (3D)
    Shape,
    /// Sound mark
    Sound,
    /// Color mark
    Color,
    /// Series mark (variations)
    Series,
}

/// Trademark status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrademarkStatus {
    /// Application pending
    Pending,
    /// Published for opposition
    Published,
    /// Under opposition
    Opposed,
    /// Registered
    Registered,
    /// Expired (not renewed)
    Expired,
    /// Withdrawn
    Withdrawn,
    /// Refused
    Refused,
    /// Removed from register
    Removed,
}

// ========== COPYRIGHT ==========

/// Copyright-protected work
///
/// ## Copyright Act 2021
/// - No registration required (automatic protection)
/// - Term: Life + 70 years, or Publication + 70 years
/// - Categories: Literary, musical, artistic, dramatic, films, sound recordings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Copyright {
    /// Title of the work
    pub title: String,

    /// Author(s) name
    pub authors: Vec<Author>,

    /// Type of copyrighted work
    pub work_type: WorkType,

    /// Publication date (if published)
    pub publication_date: Option<NaiveDate>,

    /// Copyright owner (may differ from author)
    pub owner: String,

    /// Creation date
    pub creation_date: NaiveDate,

    /// Whether work is published
    pub is_published: bool,

    /// Country of first publication
    pub country_of_publication: Option<String>,
}

impl Copyright {
    /// Creates a new copyright work
    pub fn new(
        title: impl Into<String>,
        owner: impl Into<String>,
        work_type: WorkType,
        creation_date: NaiveDate,
    ) -> Self {
        Self {
            title: title.into(),
            authors: Vec::new(),
            work_type,
            publication_date: None,
            owner: owner.into(),
            creation_date,
            is_published: false,
            country_of_publication: None,
        }
    }

    /// Calculate copyright expiry date
    ///
    /// Rules:
    /// - For works with known author: Life + 70 years
    /// - For anonymous/corporate works: Publication + 70 years
    /// - For films: Publication + 70 years
    /// - For sound recordings: Publication + 70 years
    pub fn expiry_date(&self, author_death_date: Option<NaiveDate>) -> Option<NaiveDate> {
        match self.work_type {
            WorkType::Literary | WorkType::Musical | WorkType::Artistic | WorkType::Dramatic => {
                if let Some(death_date) = author_death_date {
                    // Life + 70 years
                    NaiveDate::from_ymd_opt(
                        death_date.year() + 70,
                        death_date.month(),
                        death_date.day(),
                    )
                } else if let Some(pub_date) = self.publication_date {
                    // Publication + 70 years (anonymous/corporate)
                    NaiveDate::from_ymd_opt(pub_date.year() + 70, pub_date.month(), pub_date.day())
                } else {
                    None
                }
            }
            WorkType::Film | WorkType::SoundRecording | WorkType::Broadcast => {
                // Publication + 70 years
                self.publication_date.and_then(|date| {
                    NaiveDate::from_ymd_opt(date.year() + 70, date.month(), date.day())
                })
            }
        }
    }

    /// Returns whether copyright has expired
    pub fn is_expired(&self, author_death_date: Option<NaiveDate>) -> bool {
        if let Some(expiry) = self.expiry_date(author_death_date) {
            chrono::Utc::now().date_naive() > expiry
        } else {
            false // Unknown expiry = assume not expired
        }
    }

    /// Returns years since author's death (if applicable)
    pub fn years_since_author_death(&self, death_date: NaiveDate) -> i32 {
        let today = chrono::Utc::now().date_naive();
        today.year() - death_date.year()
    }
}

/// Author information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Author {
    /// Author name
    pub name: String,
    /// Birth year (optional)
    pub birth_year: Option<i32>,
    /// Death year (if deceased)
    pub death_year: Option<i32>,
    /// Nationality
    pub nationality: Option<String>,
}

/// Type of copyrighted work
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkType {
    /// Literary works (books, articles, software code)
    Literary,
    /// Musical works (compositions, scores)
    Musical,
    /// Artistic works (paintings, sculptures, photographs)
    Artistic,
    /// Dramatic works (plays, scripts, choreography)
    Dramatic,
    /// Films (cinematographic works)
    Film,
    /// Sound recordings
    SoundRecording,
    /// Broadcasts
    Broadcast,
}

/// Fair dealing purpose (Copyright Act s. 35-42)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FairDealingPurpose {
    /// Research or study (s. 35)
    Research,
    /// Criticism or review (s. 36)
    Criticism,
    /// News reporting (s. 37)
    NewsReporting,
    /// Judicial proceedings (s. 38)
    JudicialProceedings,
    /// Professional advice (s. 39)
    ProfessionalAdvice,
}

// ========== REGISTERED DESIGNS ==========

/// Registered design information
///
/// ## Registered Designs Act (Cap. 266)
/// - Term: 5 years, renewable twice (max 15 years) (s. 21)
/// - Requirements: Novelty and individual character (s. 5)
/// - Must not be purely functional (s. 6)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisteredDesign {
    /// Design registration number
    pub registration_number: String,

    /// Title/description of the design
    pub title: String,

    /// Designer/author name
    pub designer: String,

    /// Proprietor/owner name
    pub proprietor: String,

    /// Filing date
    pub filing_date: NaiveDate,

    /// Registration date
    pub registration_date: Option<NaiveDate>,

    /// Products to which design applies
    pub products: Vec<String>,

    /// Locarno Classification (design classification)
    pub locarno_classes: Vec<String>,

    /// Design status
    pub status: DesignStatus,

    /// Representations (images/drawings)
    pub representations: Vec<String>,
}

impl RegisteredDesign {
    /// Creates a new design application
    pub fn new(
        registration_number: impl Into<String>,
        title: impl Into<String>,
        proprietor: impl Into<String>,
        filing_date: NaiveDate,
    ) -> Self {
        Self {
            registration_number: registration_number.into(),
            title: title.into(),
            designer: String::new(),
            proprietor: proprietor.into(),
            filing_date,
            registration_date: None,
            products: Vec::new(),
            locarno_classes: Vec::new(),
            status: DesignStatus::Pending,
            representations: Vec::new(),
        }
    }

    /// Returns the first renewal date (5 years from registration)
    pub fn first_renewal_date(&self) -> Option<NaiveDate> {
        self.registration_date.map(|date| {
            NaiveDate::from_ymd_opt(date.year() + 5, date.month(), date.day()).unwrap_or(date)
        })
    }

    /// Returns the maximum expiry date (15 years from registration)
    pub fn maximum_expiry_date(&self) -> Option<NaiveDate> {
        self.registration_date.map(|date| {
            NaiveDate::from_ymd_opt(date.year() + 15, date.month(), date.day()).unwrap_or(date)
        })
    }

    /// Returns whether the design has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.maximum_expiry_date() {
            chrono::Utc::now().date_naive() > expiry
        } else {
            false
        }
    }

    /// Returns years since registration
    pub fn years_since_registration(&self) -> Option<i32> {
        self.registration_date.map(|date| {
            let today = chrono::Utc::now().date_naive();
            today.year() - date.year()
        })
    }
}

/// Design status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DesignStatus {
    /// Application pending
    Pending,
    /// Registered
    Registered,
    /// Expired (not renewed)
    Expired,
    /// Cancelled
    Cancelled,
    /// Refused
    Refused,
}

// ========== HELPER FUNCTIONS ==========

/// Calculate Levenshtein distance between two strings
#[allow(clippy::needless_range_loop)]
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    #[allow(clippy::needless_range_loop)]
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for (j, cell) in matrix[0].iter_mut().enumerate().take(len2 + 1) {
        *cell = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patent_expiry() {
        let patent = Patent::new(
            "SG 10201912345A",
            "New Widget",
            "Acme Corp",
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        );

        let expiry = patent.expiry_date();
        assert_eq!(expiry.year(), 2040);
    }

    #[test]
    fn test_trademark_similarity() {
        let tm1 = Trademark {
            mark: "APPLE".to_string(),
            classes: vec![9],
            ..Trademark::new(
                "TM001",
                "APPLE",
                "Apple Inc",
                NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            )
        };

        let tm2 = Trademark {
            mark: "APPPLE".to_string(),
            classes: vec![9],
            ..Trademark::new(
                "TM002",
                "APPPLE",
                "Other Inc",
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            )
        };

        let similarity = tm1.similarity_score(&tm2);
        assert!(similarity >= 70); // Should be high due to one letter difference
    }

    #[test]
    fn test_copyright_expiry_life_plus_70() {
        let copyright = Copyright::new(
            "Great Novel",
            "Author Name",
            WorkType::Literary,
            NaiveDate::from_ymd_opt(1950, 1, 1).unwrap(),
        );

        let death_date = NaiveDate::from_ymd_opt(1980, 1, 1).unwrap();
        let expiry = copyright.expiry_date(Some(death_date));

        assert!(expiry.is_some());
        assert_eq!(expiry.unwrap().year(), 2050); // 1980 + 70
    }

    #[test]
    fn test_design_renewal() {
        let design = RegisteredDesign {
            registration_date: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            ..RegisteredDesign::new(
                "D001",
                "Chair Design",
                "Designer Inc",
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            )
        };

        let renewal = design.first_renewal_date();
        assert!(renewal.is_some());
        assert_eq!(renewal.unwrap().year(), 2025); // 2020 + 5
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("APPLE", "APPLE"), 0);
        assert_eq!(levenshtein_distance("APPLE", "APPPLE"), 1);
        assert_eq!(levenshtein_distance("APPLE", "ORANGE"), 5);
    }
}
