//! Core types for French Intellectual Property Law
//!
//! This module defines the fundamental types for patents, copyrights, trademarks,
//! and designs with comprehensive Builder pattern implementation.

use chrono::NaiveDate;

use super::error::{
    CopyrightErrorKind, DesignErrorKind, IPLawError, IPLawResult, PatentErrorKind,
    TrademarkErrorKind,
};

/// Patent representation (Brevet d'invention)
///
/// Governed by Code de la propriété intellectuelle (CPI), Book VI, Title I
/// Patents protect technical inventions for 20 years from filing date.
#[derive(Debug, Clone, PartialEq)]
pub struct Patent {
    /// Patent title
    pub title: String,
    /// Inventor name(s)
    pub inventor: String,
    /// Filing date (date de dépôt)
    pub filing_date: NaiveDate,
    /// Grant date (date de délivrance) if granted
    pub grant_date: Option<NaiveDate>,
    /// Novelty requirement satisfied (Article L611-10 §1)
    pub novelty: bool,
    /// Inventive step requirement satisfied (Article L611-10 §2)
    pub inventive_step: bool,
    /// Industrial applicability requirement satisfied (Article L611-10 §3)
    pub industrial_applicability: bool,
    /// Protection duration in years (20 from filing, Article L611-11)
    pub protection_years: u32,
}

/// Builder for Patent
#[derive(Debug, Default)]
pub struct PatentBuilder {
    title: Option<String>,
    inventor: Option<String>,
    filing_date: Option<NaiveDate>,
    grant_date: Option<NaiveDate>,
    novelty: bool,
    inventive_step: bool,
    industrial_applicability: bool,
    protection_years: u32,
}

impl Patent {
    /// Create a new Patent builder
    pub fn builder() -> PatentBuilder {
        PatentBuilder::default()
    }

    /// Check if patent is valid (all three requirements satisfied)
    pub fn is_valid(&self) -> bool {
        self.novelty && self.inventive_step && self.industrial_applicability
    }

    /// Check if patent has expired (20 years from filing, Article L611-11)
    pub fn is_expired(&self, current_date: NaiveDate) -> bool {
        let expiry_date = self
            .filing_date
            .checked_add_signed(chrono::Duration::days(365 * 20))
            .unwrap_or(self.filing_date);
        current_date > expiry_date
    }

    /// Get expiry date
    pub fn expiry_date(&self) -> NaiveDate {
        self.filing_date
            .checked_add_signed(chrono::Duration::days(365 * 20))
            .unwrap_or(self.filing_date)
    }
}

impl PatentBuilder {
    /// Set patent title
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Set inventor
    pub fn inventor(mut self, inventor: String) -> Self {
        self.inventor = Some(inventor);
        self
    }

    /// Set filing date
    pub fn filing_date(mut self, date: NaiveDate) -> Self {
        self.filing_date = Some(date);
        self
    }

    /// Set grant date
    pub fn grant_date(mut self, date: NaiveDate) -> Self {
        self.grant_date = Some(date);
        self
    }

    /// Set novelty requirement
    pub fn novelty(mut self, novelty: bool) -> Self {
        self.novelty = novelty;
        self
    }

    /// Set inventive step requirement
    pub fn inventive_step(mut self, inventive_step: bool) -> Self {
        self.inventive_step = inventive_step;
        self
    }

    /// Set industrial applicability requirement
    pub fn industrial_applicability(mut self, applicability: bool) -> Self {
        self.industrial_applicability = applicability;
        self
    }

    /// Set protection years (default 20)
    pub fn protection_years(mut self, years: u32) -> Self {
        self.protection_years = years;
        self
    }

    /// Build the Patent
    pub fn build(self) -> IPLawResult<Patent> {
        let title = self.title.ok_or_else(|| {
            IPLawError::PatentError(PatentErrorKind::MissingField("title".to_string()))
        })?;
        let inventor = self.inventor.ok_or_else(|| {
            IPLawError::PatentError(PatentErrorKind::MissingField("inventor".to_string()))
        })?;
        let filing_date = self.filing_date.ok_or_else(|| {
            IPLawError::PatentError(PatentErrorKind::MissingField("filing_date".to_string()))
        })?;

        if let Some(grant_date) = self.grant_date
            && grant_date < filing_date
        {
            return Err(IPLawError::PatentError(PatentErrorKind::InvalidGrantDate));
        }

        Ok(Patent {
            title,
            inventor,
            filing_date,
            grant_date: self.grant_date,
            novelty: self.novelty,
            inventive_step: self.inventive_step,
            industrial_applicability: self.industrial_applicability,
            protection_years: if self.protection_years == 0 {
                20
            } else {
                self.protection_years
            },
        })
    }
}

/// Type of copyrighted work
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkType {
    /// Literary works (books, articles, poems)
    Literary,
    /// Musical works and compositions
    Musical,
    /// Artistic works (paintings, sculptures, photographs)
    Artistic,
    /// Software and computer programs
    Software,
    /// Databases
    Database,
    /// Audiovisual works (films, videos)
    Audiovisual,
}

/// Copyright representation (Droit d'auteur)
///
/// Governed by CPI, Book I
/// Copyright protects original works for 70 years post-mortem.
#[derive(Debug, Clone, PartialEq)]
pub struct Copyright {
    /// Work title
    pub work_title: String,
    /// Author name
    pub author: String,
    /// Creation date
    pub creation_date: NaiveDate,
    /// Author's death date (for calculating expiry)
    pub author_death_date: Option<NaiveDate>,
    /// Type of work
    pub work_type: WorkType,
    /// Protection duration in years (70 post-mortem, Article L123-1)
    pub protection_years: u32,
}

/// Builder for Copyright
#[derive(Debug, Default)]
pub struct CopyrightBuilder {
    work_title: Option<String>,
    author: Option<String>,
    creation_date: Option<NaiveDate>,
    author_death_date: Option<NaiveDate>,
    work_type: Option<WorkType>,
    protection_years: u32,
}

impl Copyright {
    /// Create a new Copyright builder
    pub fn builder() -> CopyrightBuilder {
        CopyrightBuilder::default()
    }

    /// Check if copyright has expired (70 years post-mortem, Article L123-1)
    pub fn is_expired(&self, current_date: NaiveDate) -> bool {
        if let Some(death_date) = self.author_death_date {
            let expiry_date = death_date
                .checked_add_signed(chrono::Duration::days(365 * 70))
                .unwrap_or(death_date);
            current_date > expiry_date
        } else {
            false // Author still alive, copyright active
        }
    }

    /// Get expiry date (if author deceased)
    pub fn expiry_date(&self) -> Option<NaiveDate> {
        self.author_death_date.map(|death_date| {
            death_date
                .checked_add_signed(chrono::Duration::days(365 * 70))
                .unwrap_or(death_date)
        })
    }
}

impl CopyrightBuilder {
    /// Set work title
    pub fn work_title(mut self, title: String) -> Self {
        self.work_title = Some(title);
        self
    }

    /// Set author
    pub fn author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// Set creation date
    pub fn creation_date(mut self, date: NaiveDate) -> Self {
        self.creation_date = Some(date);
        self
    }

    /// Set author death date
    pub fn author_death_date(mut self, date: NaiveDate) -> Self {
        self.author_death_date = Some(date);
        self
    }

    /// Set work type
    pub fn work_type(mut self, work_type: WorkType) -> Self {
        self.work_type = Some(work_type);
        self
    }

    /// Set protection years (default 70)
    pub fn protection_years(mut self, years: u32) -> Self {
        self.protection_years = years;
        self
    }

    /// Build the Copyright
    pub fn build(self) -> IPLawResult<Copyright> {
        let work_title = self.work_title.ok_or_else(|| {
            IPLawError::CopyrightError(CopyrightErrorKind::MissingField("work_title".to_string()))
        })?;
        let author = self.author.ok_or_else(|| {
            IPLawError::CopyrightError(CopyrightErrorKind::MissingField("author".to_string()))
        })?;
        let creation_date = self.creation_date.ok_or_else(|| {
            IPLawError::CopyrightError(CopyrightErrorKind::MissingField(
                "creation_date".to_string(),
            ))
        })?;
        let work_type = self.work_type.ok_or_else(|| {
            IPLawError::CopyrightError(CopyrightErrorKind::MissingField("work_type".to_string()))
        })?;

        if let Some(death_date) = self.author_death_date
            && death_date < creation_date
        {
            return Err(IPLawError::CopyrightError(
                CopyrightErrorKind::InvalidDeathDate,
            ));
        }

        Ok(Copyright {
            work_title,
            author,
            creation_date,
            author_death_date: self.author_death_date,
            work_type,
            protection_years: if self.protection_years == 0 {
                70
            } else {
                self.protection_years
            },
        })
    }
}

/// Trademark representation (Marque)
///
/// Governed by CPI, Book VII
/// Trademarks protect distinctive signs for 10 years, renewable indefinitely.
#[derive(Debug, Clone, PartialEq)]
pub struct Trademark {
    /// Trademark mark/sign
    pub mark: String,
    /// Owner name
    pub owner: String,
    /// Registration date
    pub registration_date: NaiveDate,
    /// Nice Classification classes (1-45)
    pub classes: Vec<u32>,
    /// Distinctiveness requirement satisfied (Article L711-1)
    pub distinctiveness: bool,
    /// Protection duration in years (10 renewable, Article L712-1)
    pub protection_years: u32,
}

/// Builder for Trademark
#[derive(Debug, Default)]
pub struct TrademarkBuilder {
    mark: Option<String>,
    owner: Option<String>,
    registration_date: Option<NaiveDate>,
    classes: Vec<u32>,
    distinctiveness: bool,
    protection_years: u32,
}

impl Trademark {
    /// Create a new Trademark builder
    pub fn builder() -> TrademarkBuilder {
        TrademarkBuilder::default()
    }

    /// Check if trademark is valid (distinctiveness requirement)
    pub fn is_valid(&self) -> bool {
        self.distinctiveness
    }

    /// Check if trademark has expired (10 years, Article L712-1)
    pub fn is_expired(&self, current_date: NaiveDate) -> bool {
        let expiry_date = self
            .registration_date
            .checked_add_signed(chrono::Duration::days(365 * 10))
            .unwrap_or(self.registration_date);
        current_date > expiry_date
    }

    /// Get expiry date
    pub fn expiry_date(&self) -> NaiveDate {
        self.registration_date
            .checked_add_signed(chrono::Duration::days(365 * 10))
            .unwrap_or(self.registration_date)
    }

    /// Check if classes are valid (Nice Classification: 1-45)
    pub fn has_valid_classes(&self) -> bool {
        !self.classes.is_empty() && self.classes.iter().all(|&c| (1..=45).contains(&c))
    }
}

impl TrademarkBuilder {
    /// Set trademark mark
    pub fn mark(mut self, mark: String) -> Self {
        self.mark = Some(mark);
        self
    }

    /// Set owner
    pub fn owner(mut self, owner: String) -> Self {
        self.owner = Some(owner);
        self
    }

    /// Set registration date
    pub fn registration_date(mut self, date: NaiveDate) -> Self {
        self.registration_date = Some(date);
        self
    }

    /// Set Nice classes
    pub fn classes(mut self, classes: Vec<u32>) -> Self {
        self.classes = classes;
        self
    }

    /// Add a single class
    pub fn add_class(mut self, class: u32) -> Self {
        self.classes.push(class);
        self
    }

    /// Set distinctiveness requirement
    pub fn distinctiveness(mut self, distinctive: bool) -> Self {
        self.distinctiveness = distinctive;
        self
    }

    /// Set protection years (default 10)
    pub fn protection_years(mut self, years: u32) -> Self {
        self.protection_years = years;
        self
    }

    /// Build the Trademark
    pub fn build(self) -> IPLawResult<Trademark> {
        let mark = self.mark.ok_or_else(|| {
            IPLawError::TrademarkError(TrademarkErrorKind::MissingField("mark".to_string()))
        })?;
        let owner = self.owner.ok_or_else(|| {
            IPLawError::TrademarkError(TrademarkErrorKind::MissingField("owner".to_string()))
        })?;
        let registration_date = self.registration_date.ok_or_else(|| {
            IPLawError::TrademarkError(TrademarkErrorKind::MissingField(
                "registration_date".to_string(),
            ))
        })?;

        if self.classes.is_empty() {
            return Err(IPLawError::TrademarkError(
                TrademarkErrorKind::InvalidClasses,
            ));
        }

        if !self.classes.iter().all(|&c| (1..=45).contains(&c)) {
            return Err(IPLawError::TrademarkError(
                TrademarkErrorKind::InvalidClasses,
            ));
        }

        Ok(Trademark {
            mark,
            owner,
            registration_date,
            classes: self.classes,
            distinctiveness: self.distinctiveness,
            protection_years: if self.protection_years == 0 {
                10
            } else {
                self.protection_years
            },
        })
    }
}

/// Design representation (Dessin ou modèle)
///
/// Governed by CPI, Book V
/// Designs protect aesthetic appearance for up to 25 years.
#[derive(Debug, Clone, PartialEq)]
pub struct Design {
    /// Design title
    pub title: String,
    /// Creator name
    pub creator: String,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Novelty requirement satisfied (Article L511-1 §1)
    pub novelty: bool,
    /// Individual character requirement satisfied (Article L511-1 §2)
    pub individual_character: bool,
    /// Protection duration in years (max 25 in 5-year periods, Article L513-1)
    pub protection_years: u32,
}

/// Builder for Design
#[derive(Debug, Default)]
pub struct DesignBuilder {
    title: Option<String>,
    creator: Option<String>,
    filing_date: Option<NaiveDate>,
    novelty: bool,
    individual_character: bool,
    protection_years: u32,
}

impl Design {
    /// Create a new Design builder
    pub fn builder() -> DesignBuilder {
        DesignBuilder::default()
    }

    /// Check if design is valid (novelty and individual character)
    pub fn is_valid(&self) -> bool {
        self.novelty && self.individual_character
    }

    /// Check if design has expired (max 25 years, Article L513-1)
    pub fn is_expired(&self, current_date: NaiveDate) -> bool {
        let expiry_date = self
            .filing_date
            .checked_add_signed(chrono::Duration::days(365 * self.protection_years as i64))
            .unwrap_or(self.filing_date);
        current_date > expiry_date
    }

    /// Get expiry date
    pub fn expiry_date(&self) -> NaiveDate {
        self.filing_date
            .checked_add_signed(chrono::Duration::days(365 * self.protection_years as i64))
            .unwrap_or(self.filing_date)
    }
}

impl DesignBuilder {
    /// Set design title
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Set creator
    pub fn creator(mut self, creator: String) -> Self {
        self.creator = Some(creator);
        self
    }

    /// Set filing date
    pub fn filing_date(mut self, date: NaiveDate) -> Self {
        self.filing_date = Some(date);
        self
    }

    /// Set novelty requirement
    pub fn novelty(mut self, novelty: bool) -> Self {
        self.novelty = novelty;
        self
    }

    /// Set individual character requirement
    pub fn individual_character(mut self, character: bool) -> Self {
        self.individual_character = character;
        self
    }

    /// Set protection years (default 25, max 25)
    pub fn protection_years(mut self, years: u32) -> Self {
        self.protection_years = years.min(25);
        self
    }

    /// Build the Design
    pub fn build(self) -> IPLawResult<Design> {
        let title = self.title.ok_or_else(|| {
            IPLawError::DesignError(DesignErrorKind::MissingField("title".to_string()))
        })?;
        let creator = self.creator.ok_or_else(|| {
            IPLawError::DesignError(DesignErrorKind::MissingField("creator".to_string()))
        })?;
        let filing_date = self.filing_date.ok_or_else(|| {
            IPLawError::DesignError(DesignErrorKind::MissingField("filing_date".to_string()))
        })?;

        let protection_years = if self.protection_years == 0 {
            25
        } else {
            self.protection_years.min(25)
        };

        Ok(Design {
            title,
            creator,
            filing_date,
            novelty: self.novelty,
            individual_character: self.individual_character,
            protection_years,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patent_builder() {
        let patent = Patent::builder()
            .title("Novel Invention".to_string())
            .inventor("Jean Dupont".to_string())
            .filing_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .novelty(true)
            .inventive_step(true)
            .industrial_applicability(true)
            .build()
            .unwrap();

        assert_eq!(patent.title, "Novel Invention");
        assert_eq!(patent.protection_years, 20);
        assert!(patent.is_valid());
    }

    #[test]
    fn test_patent_expiry() {
        let patent = Patent::builder()
            .title("Test".to_string())
            .inventor("Test".to_string())
            .filing_date(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap())
            .build()
            .unwrap();

        let current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert!(patent.is_expired(current));
    }

    #[test]
    fn test_copyright_builder() {
        let copyright = Copyright::builder()
            .work_title("Great Novel".to_string())
            .author("Marie Curie".to_string())
            .creation_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .work_type(WorkType::Literary)
            .build()
            .unwrap();

        assert_eq!(copyright.work_title, "Great Novel");
        assert_eq!(copyright.protection_years, 70);
    }

    #[test]
    fn test_trademark_builder() {
        let trademark = Trademark::builder()
            .mark("ACME Corp".to_string())
            .owner("ACME Corporation".to_string())
            .registration_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .classes(vec![9, 35, 42])
            .distinctiveness(true)
            .build()
            .unwrap();

        assert_eq!(trademark.mark, "ACME Corp");
        assert_eq!(trademark.protection_years, 10);
        assert!(trademark.is_valid());
        assert!(trademark.has_valid_classes());
    }

    #[test]
    fn test_trademark_invalid_classes() {
        let result = Trademark::builder()
            .mark("Test".to_string())
            .owner("Test".to_string())
            .registration_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .classes(vec![0, 50])
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_design_builder() {
        let design = Design::builder()
            .title("Modern Chair".to_string())
            .creator("Philippe Starck".to_string())
            .filing_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .novelty(true)
            .individual_character(true)
            .build()
            .unwrap();

        assert_eq!(design.title, "Modern Chair");
        assert_eq!(design.protection_years, 25);
        assert!(design.is_valid());
    }

    #[test]
    fn test_missing_required_fields() {
        let result = Patent::builder().title("Test".to_string()).build();
        assert!(result.is_err());
    }
}
