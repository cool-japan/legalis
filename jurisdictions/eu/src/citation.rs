//! EUR-Lex citation system and CELEX identifiers
//!
//! This module provides structures for working with EU legal citations in the
//! standardized EUR-Lex/CELEX format.
//!
//! ## CELEX Number Format
//!
//! CELEX (Communitatis Europeae LEX) numbers uniquely identify EU legal documents:
//! - `32016R0679` - Regulation 2016/679 (GDPR)
//! - `32011L0083` - Directive 2011/83/EU (Consumer Rights)
//! - `12012E/TXT` - Treaty (TFEU consolidated version)
//!
//! Format: `[sector][year][document type][sequential number]`
//! - Sector: 3 = Secondary legislation
//! - Year: 4 digits
//! - Type: R=Regulation, L=Directive (Lex), D=Decision
//! - Number: 4 digits (zero-padded)

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// EU legal citation with CELEX identifier and EUR-Lex URL
///
/// ## Example
///
/// ```rust
/// use legalis_eu::citation::EuCitation;
///
/// let gdpr = EuCitation::regulation(2016, 679);
/// assert_eq!(gdpr.celex, "32016R0679");
/// assert_eq!(gdpr.display, "Regulation (EU) 2016/679");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EuCitation {
    /// CELEX number (e.g., "32016R0679" for GDPR)
    pub celex: String,

    /// Human-readable citation
    pub display: String,

    /// EUR-Lex URL for official text
    pub eur_lex_url: String,

    /// Article number (if applicable)
    pub article: Option<u32>,

    /// Paragraph/section (if applicable)
    pub paragraph: Option<u32>,
}

impl EuCitation {
    /// Create citation for an EU Regulation
    ///
    /// ## Example
    ///
    /// ```rust
    /// use legalis_eu::citation::EuCitation;
    ///
    /// let gdpr = EuCitation::regulation(2016, 679);
    /// assert_eq!(gdpr.celex, "32016R0679");
    /// ```
    pub fn regulation(year: u16, number: u32) -> Self {
        let celex = format!("3{}R{:04}", year, number);
        Self {
            celex: celex.clone(),
            display: format!("Regulation (EU) {}/{}", year, number),
            eur_lex_url: format!(
                "https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:{}",
                celex
            ),
            article: None,
            paragraph: None,
        }
    }

    /// Create citation for an EU Directive
    ///
    /// ## Example
    ///
    /// ```rust
    /// use legalis_eu::citation::EuCitation;
    ///
    /// let crd = EuCitation::directive(2011, 83);
    /// assert_eq!(crd.celex, "32011L0083");
    /// assert_eq!(crd.display, "Directive 2011/83/EU");
    /// ```
    pub fn directive(year: u16, number: u32) -> Self {
        let celex = format!("3{}L{:04}", year, number);
        Self {
            celex: celex.clone(),
            display: format!("Directive {}/{}/EU", year, number),
            eur_lex_url: format!(
                "https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:{}",
                celex
            ),
            article: None,
            paragraph: None,
        }
    }

    /// Create citation for a Treaty article
    ///
    /// ## Example
    ///
    /// ```rust
    /// use legalis_eu::citation::{EuCitation, TreatyType};
    ///
    /// let art101 = EuCitation::treaty_article(TreatyType::TFEU, 101);
    /// assert_eq!(art101.display, "Article 101 TFEU");
    /// ```
    pub fn treaty_article(treaty: TreatyType, article: u32) -> Self {
        let treaty_str = match treaty {
            TreatyType::TFEU => "TFEU",
            TreatyType::TEU => "TEU",
            TreatyType::Charter => "Charter",
        };

        Self {
            celex: "12012E/TXT".to_string(), // Consolidated TFEU
            display: format!("Article {} {}", article, treaty_str),
            eur_lex_url: "https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:12012E/TXT"
                .to_string(),
            article: Some(article),
            paragraph: None,
        }
    }

    /// Add article number to citation
    pub fn with_article(mut self, article: u32) -> Self {
        self.article = Some(article);
        self.display = format!("{}, Art. {}", self.display, article);
        self
    }

    /// Add paragraph number to citation
    pub fn with_paragraph(mut self, paragraph: u32) -> Self {
        self.paragraph = Some(paragraph);
        if let Some(art) = self.article {
            self.display = format!("Art. {}({})", art, paragraph);
        }
        self
    }

    /// Format citation for specific language
    ///
    /// ## Example
    ///
    /// ```rust
    /// use legalis_eu::citation::EuCitation;
    ///
    /// let gdpr = EuCitation::regulation(2016, 679).with_article(6);
    /// assert_eq!(gdpr.format_for_language("en"), "Art. 6 GDPR");
    /// assert_eq!(gdpr.format_for_language("de"), "Art. 6 DSGVO");
    /// ```
    pub fn format_for_language(&self, lang: &str) -> String {
        if self.celex == "32016R0679" {
            // GDPR special case
            let abbrev = match lang {
                "de" | "DE" => "DSGVO",
                "fr" | "FR" => "RGPD",
                "es" | "ES" => "RGPD",
                _ => "GDPR",
            };

            if let Some(art) = self.article {
                if let Some(para) = self.paragraph {
                    return format!("Art. {}({}) {}", art, para, abbrev);
                } else {
                    return format!("Art. {} {}", art, abbrev);
                }
            }
            abbrev.to_string()
        } else {
            // Generic format
            self.display.clone()
        }
    }
}

impl fmt::Display for EuCitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

/// Treaty types in the EU legal system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TreatyType {
    /// Treaty on European Union
    TEU,

    /// Treaty on the Functioning of the European Union
    TFEU,

    /// Charter of Fundamental Rights
    Charter,
}

/// EU legal instrument types
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EuLegalInstrument {
    /// Regulation - Directly applicable, binding in its entirety
    Regulation {
        year: u16,
        number: u32,
        celex: String,
    },

    /// Directive - Binding as to result, member states choose implementation
    Directive {
        year: u16,
        number: u32,
        celex: String,
    },

    /// Treaty Article - Primary law (highest authority)
    TreatyArticle {
        treaty: TreatyType,
        article: u32,
        paragraph: Option<u32>,
    },

    /// Decision - Binding on specific addressees
    Decision {
        year: u16,
        number: u32,
        celex: String,
    },
}

impl EuLegalInstrument {
    /// Create a Regulation instrument
    pub fn regulation(year: u16, number: u32) -> Self {
        Self::Regulation {
            year,
            number,
            celex: format!("3{}R{:04}", year, number),
        }
    }

    /// Create a Directive instrument
    pub fn directive(year: u16, number: u32) -> Self {
        Self::Directive {
            year,
            number,
            celex: format!("3{}L{:04}", year, number),
        }
    }

    /// Create a Treaty Article instrument
    pub fn treaty_article(treaty: TreatyType, article: u32) -> Self {
        Self::TreatyArticle {
            treaty,
            article,
            paragraph: None,
        }
    }

    /// Get CELEX identifier for this instrument
    pub fn celex(&self) -> String {
        match self {
            Self::Regulation { celex, .. } => celex.clone(),
            Self::Directive { celex, .. } => celex.clone(),
            Self::TreatyArticle { .. } => "12012E/TXT".to_string(),
            Self::Decision { celex, .. } => celex.clone(),
        }
    }

    /// Check if this instrument is directly applicable
    pub fn is_directly_applicable(&self) -> bool {
        matches!(self, Self::Regulation { .. } | Self::TreatyArticle { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regulation_citation() {
        let gdpr = EuCitation::regulation(2016, 679);
        assert_eq!(gdpr.celex, "32016R0679");
        assert_eq!(gdpr.display, "Regulation (EU) 2016/679");
        assert!(gdpr.eur_lex_url.contains("32016R0679"));
    }

    #[test]
    fn test_directive_citation() {
        let crd = EuCitation::directive(2011, 83);
        assert_eq!(crd.celex, "32011L0083");
        assert_eq!(crd.display, "Directive 2011/83/EU");
    }

    #[test]
    fn test_treaty_citation() {
        let art101 = EuCitation::treaty_article(TreatyType::TFEU, 101);
        assert_eq!(art101.display, "Article 101 TFEU");
        assert_eq!(art101.article, Some(101));
    }

    #[test]
    fn test_citation_with_article() {
        let citation = EuCitation::regulation(2016, 679).with_article(6);
        assert_eq!(citation.article, Some(6));
        assert!(citation.display.contains("Art. 6"));
    }

    #[test]
    fn test_language_specific_formatting() {
        let gdpr = EuCitation::regulation(2016, 679).with_article(6);
        assert_eq!(gdpr.format_for_language("en"), "Art. 6 GDPR");
        assert_eq!(gdpr.format_for_language("de"), "Art. 6 DSGVO");
        assert_eq!(gdpr.format_for_language("fr"), "Art. 6 RGPD");
    }

    #[test]
    fn test_legal_instrument_celex() {
        let gdpr = EuLegalInstrument::regulation(2016, 679);
        assert_eq!(gdpr.celex(), "32016R0679");
    }

    #[test]
    fn test_direct_applicability() {
        let regulation = EuLegalInstrument::regulation(2016, 679);
        assert!(regulation.is_directly_applicable());

        let directive = EuLegalInstrument::directive(2011, 83);
        assert!(!directive.is_directly_applicable());

        let treaty = EuLegalInstrument::treaty_article(TreatyType::TFEU, 101);
        assert!(treaty.is_directly_applicable());
    }
}
