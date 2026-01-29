//! Malaysian Legal Citation System
//!
//! Provides structures and formatters for Malaysian legal citations.
//!
//! # Citation Formats
//!
//! ## Statutes
//! - "Companies Act 2016, s. 241(1)"
//! - "Employment Act 1955, s. 60D(1)"
//! - "Federal Constitution, Art. 5(1)"
//!
//! ## Case Law
//! - "\[2024\] 1 MLJ 123" (Malayan Law Journal)
//! - "\[2023\] 5 CLJ 456" (Current Law Journal)
//! - "\[2024\] 1 FC 789" (Federal Court)
//! - "\[2024\] 1 CA 234" (Court of Appeal)
//! - "\[2024\] MLJU 567" (Malayan Law Journal Unreported)
//!
//! ## Syariah Citations
//! - "\[2024\] JH 1" (Jurnal Hukum - Islamic law journal)
//! - "\[2024\] ShLR 45" (Shariah Law Reports)

use serde::{Deserialize, Serialize};

/// Malaysian statute reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Statute {
    /// Statute name (e.g., "Companies Act").
    pub name: String,
    /// Year of enactment (e.g., 2016).
    pub year: u16,
    /// Optional Act number.
    pub act_number: Option<u16>,
}

impl Statute {
    /// Creates a new statute reference.
    #[must_use]
    pub fn new(name: impl Into<String>, year: u16) -> Self {
        Self {
            name: name.into(),
            year,
            act_number: None,
        }
    }

    /// Sets the Act number.
    #[must_use]
    pub fn with_act_number(mut self, act_number: u16) -> Self {
        self.act_number = Some(act_number);
        self
    }

    /// Formats the statute citation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_my::citation::Statute;
    ///
    /// let statute = Statute::new("Companies Act", 2016);
    /// assert_eq!(statute.format(), "Companies Act 2016");
    /// ```
    #[must_use]
    pub fn format(&self) -> String {
        if let Some(act_no) = self.act_number {
            format!("{} {} (Act {})", self.name, self.year, act_no)
        } else {
            format!("{} {}", self.name, self.year)
        }
    }
}

/// Section reference within a statute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatuteSection {
    /// The statute being referenced.
    pub statute: Statute,
    /// Section number (e.g., "241").
    pub section: String,
    /// Optional subsection (e.g., "1").
    pub subsection: Option<String>,
    /// Optional paragraph.
    pub paragraph: Option<String>,
}

impl StatuteSection {
    /// Creates a new statute section reference.
    #[must_use]
    pub fn new(statute: Statute, section: impl Into<String>) -> Self {
        Self {
            statute,
            section: section.into(),
            subsection: None,
            paragraph: None,
        }
    }

    /// Sets the subsection.
    #[must_use]
    pub fn with_subsection(mut self, subsection: impl Into<String>) -> Self {
        self.subsection = Some(subsection.into());
        self
    }

    /// Sets the paragraph.
    #[must_use]
    pub fn with_paragraph(mut self, paragraph: impl Into<String>) -> Self {
        self.paragraph = Some(paragraph.into());
        self
    }

    /// Formats the full citation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_my::citation::{Statute, StatuteSection};
    ///
    /// let statute = Statute::new("Companies Act", 2016);
    /// let section = StatuteSection::new(statute, "241")
    ///     .with_subsection("1");
    /// assert_eq!(section.format(), "Companies Act 2016, s. 241(1)");
    /// ```
    #[must_use]
    pub fn format(&self) -> String {
        let mut result = format!("{}, s. {}", self.statute.format(), self.section);

        if let Some(ref subsection) = self.subsection {
            result.push('(');
            result.push_str(subsection);
            result.push(')');
        }

        if let Some(ref paragraph) = self.paragraph {
            result.push('(');
            result.push_str(paragraph);
            result.push(')');
        }

        result
    }
}

/// Malaysian case citation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MalaysianCaseCitation {
    /// Year of the case.
    pub year: u16,
    /// Volume number.
    pub volume: u8,
    /// Law report series (MLJ, CLJ, FC, CA, etc.).
    pub series: LawReportSeries,
    /// Page number.
    pub page: u16,
    /// Optional case name.
    pub case_name: Option<String>,
}

/// Malaysian law report series.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LawReportSeries {
    /// Malayan Law Journal.
    MLJ,
    /// Current Law Journal.
    CLJ,
    /// Federal Court.
    FC,
    /// Court of Appeal.
    CA,
    /// Malayan Law Journal Unreported.
    MLJU,
    /// Jurnal Hukum (Islamic law).
    JH,
    /// Shariah Law Reports.
    ShLR,
}

impl LawReportSeries {
    /// Returns the abbreviation for the series.
    #[must_use]
    pub fn abbreviation(self) -> &'static str {
        match self {
            Self::MLJ => "MLJ",
            Self::CLJ => "CLJ",
            Self::FC => "FC",
            Self::CA => "CA",
            Self::MLJU => "MLJU",
            Self::JH => "JH",
            Self::ShLR => "ShLR",
        }
    }
}

impl MalaysianCaseCitation {
    /// Creates a new case citation.
    #[must_use]
    pub fn new(year: u16, volume: u8, series: LawReportSeries, page: u16) -> Self {
        Self {
            year,
            volume,
            series,
            page,
            case_name: None,
        }
    }

    /// Sets the case name.
    #[must_use]
    pub fn with_case_name(mut self, case_name: impl Into<String>) -> Self {
        self.case_name = Some(case_name.into());
        self
    }

    /// Formats the citation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_my::citation::{MalaysianCaseCitation, LawReportSeries};
    ///
    /// let citation = MalaysianCaseCitation::new(2024, 1, LawReportSeries::MLJ, 123)
    ///     .with_case_name("PP v. Ahmad");
    /// assert_eq!(citation.format(), "[2024] 1 MLJ 123 (PP v. Ahmad)");
    /// ```
    #[must_use]
    pub fn format(&self) -> String {
        let base = format!(
            "[{}] {} {} {}",
            self.year,
            self.volume,
            self.series.abbreviation(),
            self.page
        );

        if let Some(ref name) = self.case_name {
            format!("{} ({})", base, name)
        } else {
            base
        }
    }
}

/// Malaysian citation formatter.
#[derive(Debug, Clone)]
pub struct MalaysianCitation;

impl MalaysianCitation {
    /// Creates a statute reference.
    #[must_use]
    pub fn statute(name: impl Into<String>, year: u16) -> Statute {
        Statute::new(name, year)
    }

    /// Creates a section reference.
    #[must_use]
    pub fn section(statute: Statute, section: impl Into<String>) -> StatuteSection {
        StatuteSection::new(statute, section)
    }

    /// Creates a case citation.
    #[must_use]
    pub fn case(
        year: u16,
        volume: u8,
        series: LawReportSeries,
        page: u16,
    ) -> MalaysianCaseCitation {
        MalaysianCaseCitation::new(year, volume, series, page)
    }

    /// Formats a constitutional article reference.
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_my::citation::MalaysianCitation;
    ///
    /// let citation = MalaysianCitation::constitutional_article("5", Some("1"));
    /// assert_eq!(citation, "Federal Constitution, Art. 5(1)");
    /// ```
    #[must_use]
    pub fn constitutional_article(article: &str, subsection: Option<&str>) -> String {
        if let Some(sub) = subsection {
            format!("Federal Constitution, Art. {}({})", article, sub)
        } else {
            format!("Federal Constitution, Art. {}", article)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statute_format() {
        let statute = Statute::new("Companies Act", 2016);
        assert_eq!(statute.format(), "Companies Act 2016");

        let statute_with_act = Statute::new("Employment Act", 1955).with_act_number(265);
        assert_eq!(statute_with_act.format(), "Employment Act 1955 (Act 265)");
    }

    #[test]
    fn test_statute_section_format() {
        let statute = Statute::new("Companies Act", 2016);
        let section = StatuteSection::new(statute, "241").with_subsection("1");
        assert_eq!(section.format(), "Companies Act 2016, s. 241(1)");
    }

    #[test]
    fn test_case_citation_format() {
        let citation = MalaysianCaseCitation::new(2024, 1, LawReportSeries::MLJ, 123);
        assert_eq!(citation.format(), "[2024] 1 MLJ 123");

        let citation_with_name = citation.with_case_name("PP v. Ahmad");
        assert_eq!(
            citation_with_name.format(),
            "[2024] 1 MLJ 123 (PP v. Ahmad)"
        );
    }

    #[test]
    fn test_constitutional_article() {
        let citation = MalaysianCitation::constitutional_article("5", Some("1"));
        assert_eq!(citation, "Federal Constitution, Art. 5(1)");
    }
}
