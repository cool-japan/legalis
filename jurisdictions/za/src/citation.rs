//! South African Legal Citation System
//!
//! South Africa uses a citation format similar to UK/Commonwealth style.
//!
//! ## Citation Format
//!
//! - Acts: `[Short Title] [Number] of [Year], s. [section]([subsection])`
//! - Example: `Companies Act 71 of 2008, s. 22(1)`
//!
//! ## Court Citation
//!
//! - Constitutional Court: `[Year] ZACC [number]`
//! - Supreme Court of Appeal: `[Year] ZASCA [number]`
//! - High Courts: `[Year] ZAGP/ZAWC/ZAKZ/ZAEC [number]`

use serde::{Deserialize, Serialize};
use std::fmt;

/// Types of South African legal instruments
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalInstrumentType {
    /// Constitution of the Republic of South Africa
    Constitution,
    /// Act of Parliament
    Act,
    /// Amendment Act
    AmendmentAct,
    /// Regulation
    Regulation,
    /// Government Notice
    GovernmentNotice,
    /// Provincial Act
    ProvincialAct { province: Province },
}

/// South African Provinces
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Province {
    /// Eastern Cape (EC)
    EasternCape,
    /// Free State (FS)
    FreeState,
    /// Gauteng (GP)
    Gauteng,
    /// KwaZulu-Natal (KZN)
    KwaZuluNatal,
    /// Limpopo (LP)
    Limpopo,
    /// Mpumalanga (MP)
    Mpumalanga,
    /// Northern Cape (NC)
    NorthernCape,
    /// North West (NW)
    NorthWest,
    /// Western Cape (WC)
    WesternCape,
}

impl Province {
    /// Get province abbreviation
    pub fn abbreviation(&self) -> &'static str {
        match self {
            Self::EasternCape => "EC",
            Self::FreeState => "FS",
            Self::Gauteng => "GP",
            Self::KwaZuluNatal => "KZN",
            Self::Limpopo => "LP",
            Self::Mpumalanga => "MP",
            Self::NorthernCape => "NC",
            Self::NorthWest => "NW",
            Self::WesternCape => "WC",
        }
    }

    /// Get province name
    pub fn name(&self) -> &'static str {
        match self {
            Self::EasternCape => "Eastern Cape",
            Self::FreeState => "Free State",
            Self::Gauteng => "Gauteng",
            Self::KwaZuluNatal => "KwaZulu-Natal",
            Self::Limpopo => "Limpopo",
            Self::Mpumalanga => "Mpumalanga",
            Self::NorthernCape => "Northern Cape",
            Self::NorthWest => "North West",
            Self::WesternCape => "Western Cape",
        }
    }
}

impl LegalInstrumentType {
    /// Get the type description
    pub fn description(&self) -> &str {
        match self {
            Self::Constitution => "Constitution of the Republic of South Africa",
            Self::Act => "Act",
            Self::AmendmentAct => "Amendment Act",
            Self::Regulation => "Regulation",
            Self::GovernmentNotice => "Government Notice",
            Self::ProvincialAct { province } => match province {
                Province::Gauteng => "Gauteng Act",
                Province::WesternCape => "Western Cape Act",
                _ => "Provincial Act",
            },
        }
    }
}

/// South African Legal Citation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SouthAfricanCitation {
    /// Type of legal instrument
    pub instrument_type: LegalInstrumentType,
    /// Short title of the Act
    pub short_title: Option<String>,
    /// Act number
    pub number: Option<u32>,
    /// Year
    pub year: u32,
    /// Section number
    pub section: Option<String>,
    /// Subsection
    pub subsection: Option<String>,
    /// Schedule number (if applicable)
    pub schedule: Option<u32>,
}

impl SouthAfricanCitation {
    /// Create a new Act citation
    pub fn act(short_title: impl Into<String>, number: u32, year: u32) -> Self {
        Self {
            instrument_type: LegalInstrumentType::Act,
            short_title: Some(short_title.into()),
            number: Some(number),
            year,
            section: None,
            subsection: None,
            schedule: None,
        }
    }

    /// Create a Constitution citation
    pub fn constitution() -> Self {
        Self {
            instrument_type: LegalInstrumentType::Constitution,
            short_title: Some("Constitution of the Republic of South Africa".to_string()),
            number: None,
            year: 1996,
            section: None,
            subsection: None,
            schedule: None,
        }
    }

    /// Add section reference
    pub fn with_section(mut self, section: impl Into<String>) -> Self {
        self.section = Some(section.into());
        self
    }

    /// Add subsection reference
    pub fn with_subsection(mut self, subsection: impl Into<String>) -> Self {
        self.subsection = Some(subsection.into());
        self
    }

    /// Add schedule reference
    pub fn with_schedule(mut self, schedule: u32) -> Self {
        self.schedule = Some(schedule);
        self
    }

    /// Format section reference
    fn format_section(&self) -> String {
        let mut s = String::new();

        if let Some(ref section) = self.section {
            s.push_str(&format!(", s. {}", section));
            if let Some(ref subsection) = self.subsection {
                s.push_str(&format!("({})", subsection));
            }
        }

        if let Some(schedule) = self.schedule {
            s.push_str(&format!(", Schedule {}", schedule));
        }

        s
    }
}

impl fmt::Display for SouthAfricanCitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.instrument_type {
            LegalInstrumentType::Constitution => {
                write!(f, "Constitution of the Republic of South Africa, 1996")?;
            }
            _ => {
                if let Some(ref title) = self.short_title {
                    write!(f, "{}", title)?;
                }
                if let Some(number) = self.number {
                    write!(f, " {} of {}", number, self.year)?;
                } else {
                    write!(f, " of {}", self.year)?;
                }
            }
        }

        write!(f, "{}", self.format_section())
    }
}

/// Common South African law citations
pub mod common_citations {
    use super::*;

    /// Constitution of the Republic of South Africa, 1996
    pub fn constitution() -> SouthAfricanCitation {
        SouthAfricanCitation::constitution()
    }

    /// Companies Act 71 of 2008
    pub fn companies_act() -> SouthAfricanCitation {
        SouthAfricanCitation::act("Companies Act", 71, 2008)
    }

    /// Labour Relations Act 66 of 1995
    pub fn labour_relations_act() -> SouthAfricanCitation {
        SouthAfricanCitation::act("Labour Relations Act", 66, 1995)
    }

    /// Basic Conditions of Employment Act 75 of 1997
    pub fn bcea() -> SouthAfricanCitation {
        SouthAfricanCitation::act("Basic Conditions of Employment Act", 75, 1997)
    }

    /// Protection of Personal Information Act 4 of 2013 (POPIA)
    pub fn popia() -> SouthAfricanCitation {
        SouthAfricanCitation::act("Protection of Personal Information Act", 4, 2013)
    }

    /// Broad-Based Black Economic Empowerment Act 53 of 2003
    pub fn bbbee_act() -> SouthAfricanCitation {
        SouthAfricanCitation::act("Broad-Based Black Economic Empowerment Act", 53, 2003)
    }

    /// Employment Equity Act 55 of 1998
    pub fn employment_equity_act() -> SouthAfricanCitation {
        SouthAfricanCitation::act("Employment Equity Act", 55, 1998)
    }

    /// Consumer Protection Act 68 of 2008
    pub fn cpa() -> SouthAfricanCitation {
        SouthAfricanCitation::act("Consumer Protection Act", 68, 2008)
    }

    /// National Credit Act 34 of 2005
    pub fn nca() -> SouthAfricanCitation {
        SouthAfricanCitation::act("National Credit Act", 34, 2005)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_act_citation() {
        let citation = SouthAfricanCitation::act("Companies Act", 71, 2008)
            .with_section("22")
            .with_subsection("1");

        let formatted = citation.to_string();
        assert!(formatted.contains("Companies Act 71 of 2008"));
        assert!(formatted.contains("s. 22(1)"));
    }

    #[test]
    fn test_constitution_citation() {
        let citation = SouthAfricanCitation::constitution()
            .with_section("9")
            .with_subsection("3");

        let formatted = citation.to_string();
        assert!(formatted.contains("Constitution"));
        assert!(formatted.contains("1996"));
        assert!(formatted.contains("s. 9(3)"));
    }

    #[test]
    fn test_common_citations() {
        let companies = common_citations::companies_act();
        assert_eq!(companies.number, Some(71));
        assert_eq!(companies.year, 2008);

        let popia = common_citations::popia();
        assert_eq!(popia.number, Some(4));
        assert_eq!(popia.year, 2013);
    }

    #[test]
    fn test_provinces() {
        assert_eq!(Province::Gauteng.abbreviation(), "GP");
        assert_eq!(Province::WesternCape.name(), "Western Cape");
    }

    #[test]
    fn test_schedule_reference() {
        let citation = SouthAfricanCitation::act("Companies Act", 71, 2008).with_schedule(1);

        assert!(citation.to_string().contains("Schedule 1"));
    }
}
