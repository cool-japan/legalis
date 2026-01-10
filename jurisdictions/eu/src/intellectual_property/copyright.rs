//! EU Copyright Directives (InfoSoc 2001/29/EC, DSM 2019/790, etc.)
//!
//! Implements validation for copyright protection under EU law:
//! - InfoSoc Directive 2001/29/EC (Information Society Directive)
//! - DSM Directive (EU) 2019/790 (Digital Single Market)
//! - Software Directive 2009/24/EC
//! - Database Directive 96/9/EC

use super::error::IpError;
use super::types::{CopyrightException, WorkType};
use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Copyrighted work under EU law
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyrightWork {
    pub title: Option<String>,
    pub author: Option<String>,
    pub work_type: Option<WorkType>,
    pub creation_date: Option<DateTime<Utc>>,
    pub death_date_of_author: Option<DateTime<Utc>>,
    pub is_original: bool,
    pub is_fixated: bool,
    pub country_of_origin: Option<String>,
}

impl CopyrightWork {
    pub fn new() -> Self {
        Self {
            title: None,
            author: None,
            work_type: None,
            creation_date: None,
            death_date_of_author: None,
            is_original: false,
            is_fixated: false,
            country_of_origin: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn with_work_type(mut self, work_type: WorkType) -> Self {
        self.work_type = Some(work_type);
        self
    }

    pub fn with_creation_date(mut self, date: DateTime<Utc>) -> Self {
        self.creation_date = Some(date);
        self
    }

    pub fn with_death_date_of_author(mut self, date: DateTime<Utc>) -> Self {
        self.death_date_of_author = Some(date);
        self
    }

    pub fn with_originality(mut self, is_original: bool) -> Self {
        self.is_original = is_original;
        self
    }

    pub fn with_fixation(mut self, is_fixated: bool) -> Self {
        self.is_fixated = is_fixated;
        self
    }

    pub fn with_country_of_origin(mut self, country: impl Into<String>) -> Self {
        self.country_of_origin = Some(country.into());
        self
    }

    /// Validate copyright protection under EU law
    ///
    /// Checks:
    /// - InfoSoc Directive: Originality requirement (author's own intellectual creation)
    /// - Fixation requirement (for certain work types)
    /// - Protection duration calculation (life + 70 years)
    pub fn validate(&self) -> Result<CopyrightValidation, IpError> {
        // Check required fields
        if self.work_type.is_none() {
            return Err(IpError::missing_field("work_type"));
        }

        if self.author.is_none() {
            return Err(IpError::missing_field("author"));
        }

        // InfoSoc Directive: Originality requirement
        // Works must be "author's own intellectual creation"
        if !self.is_original {
            return Err(IpError::copyright_issue(
                "Work must be original (author's own intellectual creation) - InfoSoc Directive Art. 3",
            ));
        }

        // Fixation requirement (not required for all works under EU law, but commonly expected)
        // Note: EU law doesn't mandate fixation for all works, unlike US copyright law
        let fixation_required = matches!(
            self.work_type,
            Some(WorkType::Software) | Some(WorkType::Database) | Some(WorkType::Audiovisual)
        );

        if fixation_required && !self.is_fixated {
            return Err(IpError::copyright_issue(
                "Work of this type must be fixed in tangible medium",
            ));
        }

        // Calculate protection duration: Life + 70 years (Term Directive 2006/116/EC)
        let protection_expires = if let Some(death_date) = self.death_date_of_author {
            let expiry = death_date + chrono::Duration::days(70 * 365 + 17); // 70 years + leap days
            Some(expiry)
        } else {
            None // Cannot determine without death date
        };

        let is_protected = if let Some(expiry) = protection_expires {
            Utc::now() < expiry
        } else {
            // Assume protected if no death date provided and work is original
            true
        };

        Ok(CopyrightValidation {
            is_protectable: true,
            originality_established: self.is_original,
            fixation_requirement_met: !fixation_required || self.is_fixated,
            is_protected,
            protection_expires,
            applicable_exceptions: self.identify_applicable_exceptions(),
        })
    }

    /// Identify potentially applicable copyright exceptions
    fn identify_applicable_exceptions(&self) -> Vec<CopyrightException> {
        let mut exceptions = Vec::new();

        match self.work_type {
            Some(WorkType::Literary) | Some(WorkType::Software) => {
                // Text and data mining often applies to literary/software works
                exceptions.push(CopyrightException::TextDataMining);
                exceptions.push(CopyrightException::Quotation);
                exceptions.push(CopyrightException::EducationalUse);
            }
            Some(WorkType::Musical) | Some(WorkType::Audiovisual) => {
                exceptions.push(CopyrightException::Quotation);
                exceptions.push(CopyrightException::Parody);
                exceptions.push(CopyrightException::NewsReporting);
            }
            Some(WorkType::Photographic) | Some(WorkType::Artistic) => {
                exceptions.push(CopyrightException::Quotation);
                exceptions.push(CopyrightException::NewsReporting);
            }
            _ => {}
        }

        // All works can potentially have accessibility and private copying exceptions
        exceptions.push(CopyrightException::AccessibilityForDisabled);
        exceptions.push(CopyrightException::PrivateCopying);

        exceptions
    }
}

impl Default for CopyrightWork {
    fn default() -> Self {
        Self::new()
    }
}

/// Copyright validation result
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyrightValidation {
    /// Whether work is protectable by copyright
    pub is_protectable: bool,

    /// Whether originality is established
    pub originality_established: bool,

    /// Whether fixation requirement is met (if applicable)
    pub fixation_requirement_met: bool,

    /// Whether currently protected
    pub is_protected: bool,

    /// When protection expires (life + 70 years)
    pub protection_expires: Option<DateTime<Utc>>,

    /// Applicable copyright exceptions
    pub applicable_exceptions: Vec<CopyrightException>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_literary_work() {
        let work = CopyrightWork::new()
            .with_title("Digital Privacy Law")
            .with_author("Jane Author")
            .with_work_type(WorkType::Literary)
            .with_creation_date(Utc::now() - chrono::Duration::days(365))
            .with_originality(true)
            .with_fixation(true);

        let result = work.validate();
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(validation.is_protectable);
        assert!(validation.originality_established);
        assert!(validation.is_protected);
    }

    #[test]
    fn test_work_lacking_originality() {
        let work = CopyrightWork::new()
            .with_title("Phone Directory")
            .with_author("Company")
            .with_work_type(WorkType::Database)
            .with_originality(false); // Not author's own intellectual creation

        let result = work.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(IpError::CopyrightIssue { .. })));
    }

    #[test]
    fn test_software_requires_fixation() {
        let work = CopyrightWork::new()
            .with_title("MyApp")
            .with_author("Developer")
            .with_work_type(WorkType::Software)
            .with_originality(true)
            .with_fixation(false); // Not fixed in tangible form

        let result = work.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_protection_duration_calculation() {
        let death_date = Utc::now() - chrono::Duration::days(60 * 365); // Author died 60 years ago

        let work = CopyrightWork::new()
            .with_title("Classic Novel")
            .with_author("Historic Author")
            .with_work_type(WorkType::Literary)
            .with_creation_date(Utc::now() - chrono::Duration::days(80 * 365))
            .with_death_date_of_author(death_date)
            .with_originality(true);

        let result = work.validate();
        assert!(result.is_ok());
        let validation = result.unwrap();

        // Work should still be protected (life + 70 years, only 60 years passed)
        assert!(validation.is_protected);
        assert!(validation.protection_expires.is_some());
    }

    #[test]
    fn test_expired_copyright() {
        let death_date = Utc::now() - chrono::Duration::days(100 * 365); // Author died 100 years ago

        let work = CopyrightWork::new()
            .with_title("Ancient Work")
            .with_author("Ancient Author")
            .with_work_type(WorkType::Literary)
            .with_death_date_of_author(death_date)
            .with_originality(true);

        let result = work.validate();
        assert!(result.is_ok());
        let validation = result.unwrap();

        // Work should be in public domain (life + 70 years = 100 years passed)
        assert!(!validation.is_protected);
    }

    #[test]
    fn test_applicable_exceptions_literary_work() {
        let work = CopyrightWork::new()
            .with_title("Novel")
            .with_author("Author")
            .with_work_type(WorkType::Literary)
            .with_originality(true);

        let result = work.validate();
        assert!(result.is_ok());
        let validation = result.unwrap();

        // Literary works should have quotation, education, text mining exceptions
        assert!(
            validation
                .applicable_exceptions
                .contains(&CopyrightException::Quotation)
        );
        assert!(
            validation
                .applicable_exceptions
                .contains(&CopyrightException::TextDataMining)
        );
        assert!(
            validation
                .applicable_exceptions
                .contains(&CopyrightException::EducationalUse)
        );
    }

    #[test]
    fn test_audiovisual_parody_exception() {
        let work = CopyrightWork::new()
            .with_title("Film")
            .with_author("Director")
            .with_work_type(WorkType::Audiovisual)
            .with_originality(true)
            .with_fixation(true);

        let result = work.validate();
        assert!(result.is_ok());
        let validation = result.unwrap();

        // Audiovisual works should have parody exception
        assert!(
            validation
                .applicable_exceptions
                .contains(&CopyrightException::Parody)
        );
    }
}
