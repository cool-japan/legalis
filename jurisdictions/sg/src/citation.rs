//! Singapore Legal Citation System
//!
//! This module provides types and utilities for representing Singapore legal citations.
//!
//! ## Citation Formats
//!
//! Singapore uses several citation formats depending on the type of legal source:
//!
//! ### Statutes (Acts of Parliament)
//!
//! **Older Acts** (pre-2000) use **Chapter numbers** (Cap.):
//! - Full: "Companies Act (Chapter 50), section 145(1)"
//! - Short: "CA s. 145(1)" or "Cap. 50, s. 145"
//!
//! **Modern Acts** (post-2000) have no chapter numbers:
//! - "Personal Data Protection Act 2012, section 13"
//! - "PDPA s. 13"
//!
//! ### Subsidiary Legislation
//!
//! - "Companies Regulations (Cap. 50, Rg 1), regulation 3"
//! - "Employment (Part-Time Employees) Regulations (Cap. 91, Rg 1)"
//!
//! ### Case Law
//!
//! Singapore courts use a neutral citation system:
//! - **Court of Appeal**: "[2024] SGCA 15" (year, court, number)
//! - **High Court**: "[2023] SGHC 150"
//! - **District Court**: "[2023] SGDC 42"
//! - **Magistrates' Court**: "[2023] SGMC 10"
//!
//! ## Examples
//!
//! ```
//! use legalis_sg::citation::*;
//!
//! // Companies Act citation
//! let companies_act = Statute::with_chapter(
//!     "Companies Act",
//!     50,
//!     Some(1967)
//! );
//! assert_eq!(companies_act.to_string(), "Companies Act (Cap. 50)");
//!
//! // PDPA citation (no chapter)
//! let pdpa = Statute::without_chapter(
//!     "Personal Data Protection Act",
//!     2012
//! );
//! assert_eq!(pdpa.to_string(), "Personal Data Protection Act 2012");
//!
//! // Specific section
//! let section = companies_act.section(145, Some(1));
//! assert_eq!(section.to_string(), "Companies Act (Cap. 50), s. 145(1)");
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a Singapore statute (Act of Parliament)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Statute {
    /// Full name of the statute (e.g., "Companies Act")
    pub name: String,

    /// Chapter number for older acts (e.g., Some(50) for Cap. 50)
    pub chapter: Option<u32>,

    /// Year of enactment or revision
    pub year: Option<u32>,

    /// Short name or abbreviation (e.g., "CA" for Companies Act)
    pub short_name: Option<String>,
}

impl Statute {
    /// Creates a statute with a chapter number (older acts)
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_sg::citation::Statute;
    ///
    /// let companies_act = Statute::with_chapter("Companies Act", 50, Some(1967));
    /// ```
    pub fn with_chapter(name: impl Into<String>, chapter: u32, year: Option<u32>) -> Self {
        Self {
            name: name.into(),
            chapter: Some(chapter),
            year,
            short_name: None,
        }
    }

    /// Creates a statute without a chapter number (modern acts)
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_sg::citation::Statute;
    ///
    /// let pdpa = Statute::without_chapter("Personal Data Protection Act", 2012);
    /// ```
    pub fn without_chapter(name: impl Into<String>, year: u32) -> Self {
        Self {
            name: name.into(),
            chapter: None,
            year: Some(year),
            short_name: None,
        }
    }

    /// Sets the short name/abbreviation
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_sg::citation::Statute;
    ///
    /// let ca = Statute::with_chapter("Companies Act", 50, Some(1967))
    ///     .with_short_name("CA");
    /// ```
    pub fn with_short_name(mut self, short_name: impl Into<String>) -> Self {
        self.short_name = Some(short_name.into());
        self
    }

    /// Creates a reference to a specific section
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_sg::citation::Statute;
    ///
    /// let ca = Statute::with_chapter("Companies Act", 50, Some(1967));
    /// let section = ca.section(145, Some(1)); // s. 145(1)
    /// ```
    pub fn section(&self, section: u32, subsection: Option<u32>) -> StatuteSection {
        StatuteSection {
            statute: self.clone(),
            section,
            subsection,
            paragraph: None,
        }
    }
}

impl fmt::Display for Statute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(chapter) = self.chapter {
            // Older act with chapter: "Companies Act (Cap. 50)"
            write!(f, "{} (Cap. {})", self.name, chapter)
        } else if let Some(year) = self.year {
            // Modern act without chapter: "Personal Data Protection Act 2012"
            write!(f, "{} {}", self.name, year)
        } else {
            // Just the name
            write!(f, "{}", self.name)
        }
    }
}

/// Represents a specific section within a statute
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatuteSection {
    /// The statute this section belongs to
    pub statute: Statute,

    /// Section number (e.g., 145)
    pub section: u32,

    /// Subsection number (e.g., 1 for subsection (1))
    pub subsection: Option<u32>,

    /// Paragraph within subsection (e.g., "a" for paragraph (a))
    pub paragraph: Option<String>,
}

impl StatuteSection {
    /// Creates a new statute section reference
    pub fn new(statute: Statute, section: u32) -> Self {
        Self {
            statute,
            section,
            subsection: None,
            paragraph: None,
        }
    }

    /// Sets the subsection
    pub fn with_subsection(mut self, subsection: u32) -> Self {
        self.subsection = Some(subsection);
        self
    }

    /// Sets the paragraph
    pub fn with_paragraph(mut self, paragraph: impl Into<String>) -> Self {
        self.paragraph = Some(paragraph.into());
        self
    }

    /// Returns the short citation format (e.g., "CA s. 145(1)")
    pub fn short_citation(&self) -> String {
        if let Some(short_name) = &self.statute.short_name {
            format!(
                "{} s. {}{}",
                short_name,
                self.section,
                self.subsection_str()
            )
        } else if let Some(chapter) = self.statute.chapter {
            format!(
                "Cap. {}, s. {}{}",
                chapter,
                self.section,
                self.subsection_str()
            )
        } else {
            format!("s. {}{}", self.section, self.subsection_str())
        }
    }

    fn subsection_str(&self) -> String {
        let mut result = String::new();
        if let Some(subsection) = self.subsection {
            result.push_str(&format!("({})", subsection));
        }
        if let Some(para) = &self.paragraph {
            result.push_str(&format!("({})", para));
        }
        result
    }
}

impl fmt::Display for StatuteSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, s. {}{}",
            self.statute,
            self.section,
            self.subsection_str()
        )
    }
}

/// Complete Singapore legal citation including case law
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SingaporeCitation {
    /// Statute citation (e.g., Companies Act Cap. 50 s. 145)
    Statute(StatuteSection),

    /// Case law citation (e.g., [2024] SGCA 15)
    Case {
        /// Year of decision
        year: u32,
        /// Court abbreviation (SGCA, SGHC, SGDC, SGMC)
        court: CourtLevel,
        /// Case number
        number: u32,
        /// Optional case name
        case_name: Option<String>,
    },

    /// Subsidiary legislation (regulations, rules, orders)
    SubsidiaryLegislation {
        /// Parent statute
        parent_statute: Statute,
        /// Type (e.g., "Regulations", "Rules", "Order")
        legislation_type: String,
        /// Regulation number (e.g., "Rg 1")
        regulation_number: String,
        /// Specific provision (optional)
        provision: Option<String>,
    },
}

/// Singapore court levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CourtLevel {
    /// Court of Appeal (highest court)
    CourtOfAppeal,
    /// High Court
    HighCourt,
    /// District Court
    DistrictCourt,
    /// Magistrates' Court
    MagistratesCourt,
    /// Family Court
    FamilyCourt,
    /// State Courts (general)
    StateCourts,
}

impl CourtLevel {
    /// Returns the court abbreviation (e.g., "SGCA" for Court of Appeal)
    pub fn abbreviation(&self) -> &'static str {
        match self {
            CourtLevel::CourtOfAppeal => "SGCA",
            CourtLevel::HighCourt => "SGHC",
            CourtLevel::DistrictCourt => "SGDC",
            CourtLevel::MagistratesCourt => "SGMC",
            CourtLevel::FamilyCourt => "SGFC",
            CourtLevel::StateCourts => "SGSC",
        }
    }
}

impl fmt::Display for CourtLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbreviation())
    }
}

impl fmt::Display for SingaporeCitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SingaporeCitation::Statute(section) => write!(f, "{}", section),
            SingaporeCitation::Case {
                year,
                court,
                number,
                case_name,
            } => {
                write!(f, "[{}] {} {}", year, court.abbreviation(), number)?;
                if let Some(name) = case_name {
                    write!(f, " ({})", name)?;
                }
                Ok(())
            }
            SingaporeCitation::SubsidiaryLegislation {
                parent_statute,
                legislation_type,
                regulation_number,
                provision,
            } => {
                write!(f, "{} {} (", legislation_type, parent_statute)?;
                if let Some(chapter) = parent_statute.chapter {
                    write!(f, "Cap. {}, ", chapter)?;
                }
                write!(f, "{})", regulation_number)?;
                if let Some(prov) = provision {
                    write!(f, ", {}", prov)?;
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statute_with_chapter() {
        let ca = Statute::with_chapter("Companies Act", 50, Some(1967));
        assert_eq!(ca.to_string(), "Companies Act (Cap. 50)");
        assert_eq!(ca.chapter, Some(50));
    }

    #[test]
    fn test_statute_without_chapter() {
        let pdpa = Statute::without_chapter("Personal Data Protection Act", 2012);
        assert_eq!(pdpa.to_string(), "Personal Data Protection Act 2012");
        assert_eq!(pdpa.chapter, None);
    }

    #[test]
    fn test_statute_section() {
        let ca = Statute::with_chapter("Companies Act", 50, Some(1967));
        let section = ca.section(145, Some(1));
        assert_eq!(section.to_string(), "Companies Act (Cap. 50), s. 145(1)");
    }

    #[test]
    fn test_short_citation() {
        let ca = Statute::with_chapter("Companies Act", 50, Some(1967)).with_short_name("CA");
        let section = ca.section(145, Some(1));
        assert_eq!(section.short_citation(), "CA s. 145(1)");
    }

    #[test]
    fn test_case_citation() {
        let citation = SingaporeCitation::Case {
            year: 2024,
            court: CourtLevel::CourtOfAppeal,
            number: 15,
            case_name: Some("Tan v Lee".to_string()),
        };
        assert_eq!(citation.to_string(), "[2024] SGCA 15 (Tan v Lee)");
    }

    #[test]
    fn test_subsidiary_legislation() {
        let parent = Statute::with_chapter("Companies Act", 50, Some(1967));
        let citation = SingaporeCitation::SubsidiaryLegislation {
            parent_statute: parent,
            legislation_type: "Companies Regulations".to_string(),
            regulation_number: "Rg 1".to_string(),
            provision: Some("reg. 3".to_string()),
        };
        assert_eq!(
            citation.to_string(),
            "Companies Regulations Companies Act (Cap. 50) (Cap. 50, Rg 1), reg. 3"
        );
    }

    #[test]
    fn test_court_abbreviations() {
        assert_eq!(CourtLevel::CourtOfAppeal.abbreviation(), "SGCA");
        assert_eq!(CourtLevel::HighCourt.abbreviation(), "SGHC");
        assert_eq!(CourtLevel::DistrictCourt.abbreviation(), "SGDC");
        assert_eq!(CourtLevel::MagistratesCourt.abbreviation(), "SGMC");
    }
}
