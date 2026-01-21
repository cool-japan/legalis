//! Indian Legal Citation System
//!
//! Implements citation formats for Indian laws, cases, and regulations.
//!
//! # Citation Formats
//!
//! ## Case Citations
//!
//! - **AIR (All India Reporter)**: `AIR 2024 SC 1234`
//! - **SCC (Supreme Court Cases)**: `(2024) 5 SCC 789`
//! - **SCR (Supreme Court Reports)**: `[2024] 3 SCR 456`
//! - **High Court**: `2024 Del HC 100`
//!
//! ## Statutory Citations
//!
//! - `The Companies Act, 2013, s. 173(1)`
//! - `Indian Contract Act, 1872, s. 10`
//! - `DPDP Act, 2023, s. 4(1)`

use serde::{Deserialize, Serialize};
use std::fmt;

/// Indian legal citation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Citation {
    /// Citation type
    pub citation_type: CitationType,
    /// Year
    pub year: u32,
    /// Source/Reporter
    pub source: String,
    /// Volume (if applicable)
    pub volume: Option<u32>,
    /// Page/Section number
    pub number: u32,
    /// Subsection (if applicable)
    pub subsection: Option<u32>,
}

/// Citation type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CitationType {
    /// Statutory citation
    Statute,
    /// AIR citation
    Air,
    /// SCC citation
    Scc,
    /// SCR citation
    Scr,
    /// High Court citation
    HighCourt(String),
}

impl Citation {
    /// Create statute citation
    pub fn statute(act_name: &str, section: u32, year: u32) -> Self {
        Self {
            citation_type: CitationType::Statute,
            year,
            source: act_name.to_string(),
            volume: None,
            number: section,
            subsection: None,
        }
    }

    /// Create statute citation with subsection
    pub fn statute_full(act_name: &str, section: u32, subsection: u32, year: u32) -> Self {
        Self {
            citation_type: CitationType::Statute,
            year,
            source: act_name.to_string(),
            volume: None,
            number: section,
            subsection: Some(subsection),
        }
    }

    /// Create AIR citation
    pub fn air(year: u32, court: &str, page: u32) -> Self {
        Self {
            citation_type: CitationType::Air,
            year,
            source: court.to_string(),
            volume: None,
            number: page,
            subsection: None,
        }
    }

    /// Create SCC citation
    pub fn scc(year: u32, volume: u32, page: u32) -> Self {
        Self {
            citation_type: CitationType::Scc,
            year,
            source: "SCC".to_string(),
            volume: Some(volume),
            number: page,
            subsection: None,
        }
    }

    /// Create SCR citation
    pub fn scr(year: u32, volume: u32, page: u32) -> Self {
        Self {
            citation_type: CitationType::Scr,
            year,
            source: "SCR".to_string(),
            volume: Some(volume),
            number: page,
            subsection: None,
        }
    }

    /// Create High Court citation
    pub fn high_court(year: u32, state_code: &str, case_number: u32) -> Self {
        Self {
            citation_type: CitationType::HighCourt(state_code.to_string()),
            year,
            source: format!("{} HC", state_code),
            volume: None,
            number: case_number,
            subsection: None,
        }
    }

    /// Format citation
    pub fn format(&self) -> String {
        match &self.citation_type {
            CitationType::Statute => {
                if let Some(sub) = self.subsection {
                    format!(
                        "{}, {}, s. {}({})",
                        self.source, self.year, self.number, sub
                    )
                } else {
                    format!("{}, {}, s. {}", self.source, self.year, self.number)
                }
            }
            CitationType::Air => {
                format!("AIR {} {} {}", self.year, self.source, self.number)
            }
            CitationType::Scc => {
                format!(
                    "({}) {} SCC {}",
                    self.year,
                    self.volume.unwrap_or(1),
                    self.number
                )
            }
            CitationType::Scr => {
                format!(
                    "[{}] {} SCR {}",
                    self.year,
                    self.volume.unwrap_or(1),
                    self.number
                )
            }
            CitationType::HighCourt(state) => {
                format!("{} {} HC {}", self.year, state, self.number)
            }
        }
    }

    /// Add subsection
    pub fn with_subsection(mut self, subsection: u32) -> Self {
        self.subsection = Some(subsection);
        self
    }
}

impl fmt::Display for Citation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

/// Major Indian Acts
pub mod acts {
    /// Digital Personal Data Protection Act, 2023
    pub const DPDPA: &str = "Digital Personal Data Protection Act";

    /// Companies Act, 2013
    pub const COMPANIES_ACT: &str = "Companies Act";

    /// Indian Contract Act, 1872
    pub const CONTRACT_ACT: &str = "Indian Contract Act";

    /// Information Technology Act, 2000
    pub const IT_ACT: &str = "Information Technology Act";

    /// Consumer Protection Act, 2019
    pub const CONSUMER_ACT: &str = "Consumer Protection Act";

    /// Labour Codes, 2020
    pub const LABOUR_CODE_WAGES: &str = "Code on Wages";
    pub const LABOUR_CODE_SS: &str = "Code on Social Security";
    pub const LABOUR_CODE_IR: &str = "Industrial Relations Code";
    pub const LABOUR_CODE_OSH: &str = "Occupational Safety, Health and Working Conditions Code";

    /// Indian Penal Code (now BNS)
    pub const BNS: &str = "Bharatiya Nyaya Sanhita";

    /// Transfer of Property Act, 1882
    pub const TRANSFER_OF_PROPERTY: &str = "Transfer of Property Act";

    /// Goods and Services Tax Act, 2017
    pub const GST_ACT: &str = "Central Goods and Services Tax Act";
}

/// Citation helpers
pub mod cite {
    use super::*;

    /// Create DPDPA citation
    pub fn dpdpa(section: u32) -> Citation {
        Citation::statute(acts::DPDPA, section, 2023)
    }

    /// Create Companies Act citation
    pub fn companies_act(section: u32) -> Citation {
        Citation::statute(acts::COMPANIES_ACT, section, 2013)
    }

    /// Create Indian Contract Act citation
    pub fn contract_act(section: u32) -> Citation {
        Citation::statute(acts::CONTRACT_ACT, section, 1872)
    }

    /// Create IT Act citation
    pub fn it_act(section: u32) -> Citation {
        Citation::statute(acts::IT_ACT, section, 2000)
    }

    /// Create Consumer Protection Act citation
    pub fn consumer_act(section: u32) -> Citation {
        Citation::statute(acts::CONSUMER_ACT, section, 2019)
    }

    /// Create GST Act citation
    pub fn gst_act(section: u32) -> Citation {
        Citation::statute(acts::GST_ACT, section, 2017)
    }

    /// Create CGST Act citation (alias for gst_act)
    pub fn cgst(section: u32) -> Citation {
        Citation::statute(acts::GST_ACT, section, 2017)
    }

    /// Create BNS citation
    pub fn bns(section: u32) -> Citation {
        Citation::statute(acts::BNS, section, 2023)
    }

    /// Create Code on Wages citation
    pub fn wages_code(section: u32) -> Citation {
        Citation::statute(acts::LABOUR_CODE_WAGES, section, 2019)
    }

    /// Create Code on Social Security citation
    pub fn social_security(section: u32) -> Citation {
        Citation::statute(acts::LABOUR_CODE_SS, section, 2020)
    }

    /// Create Industrial Relations Code citation
    pub fn ir_code(section: u32) -> Citation {
        Citation::statute(acts::LABOUR_CODE_IR, section, 2020)
    }

    /// Create OSH Code citation
    pub fn osh_code(section: u32) -> Citation {
        Citation::statute(acts::LABOUR_CODE_OSH, section, 2020)
    }

    /// Create Supreme Court AIR citation
    pub fn air_sc(year: u32, page: u32) -> Citation {
        Citation::air(year, "SC", page)
    }

    /// Create High Court AIR citation
    pub fn air_hc(year: u32, state: &str, page: u32) -> Citation {
        Citation::air(year, state, page)
    }
}

/// Court hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Court {
    /// Supreme Court of India
    SupremeCourt,
    /// High Court
    HighCourt,
    /// District Court
    DistrictCourt,
    /// Tribunal
    Tribunal,
    /// Consumer Forum
    ConsumerForum,
}

impl Court {
    /// Get court name
    pub fn name(&self) -> &str {
        match self {
            Self::SupremeCourt => "Supreme Court of India",
            Self::HighCourt => "High Court",
            Self::DistrictCourt => "District Court",
            Self::Tribunal => "Tribunal",
            Self::ConsumerForum => "Consumer Forum",
        }
    }

    /// Get court abbreviation
    pub fn abbreviation(&self) -> &str {
        match self {
            Self::SupremeCourt => "SC",
            Self::HighCourt => "HC",
            Self::DistrictCourt => "DC",
            Self::Tribunal => "Trib.",
            Self::ConsumerForum => "CF",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statute_citation() {
        let cite = Citation::statute("Companies Act", 173, 2013);
        assert_eq!(cite.format(), "Companies Act, 2013, s. 173");
    }

    #[test]
    fn test_statute_with_subsection() {
        let cite = Citation::statute_full("Companies Act", 173, 1, 2013);
        assert_eq!(cite.format(), "Companies Act, 2013, s. 173(1)");
    }

    #[test]
    fn test_air_citation() {
        let cite = Citation::air(2024, "SC", 1234);
        assert_eq!(cite.format(), "AIR 2024 SC 1234");
    }

    #[test]
    fn test_scc_citation() {
        let cite = Citation::scc(2024, 5, 789);
        assert_eq!(cite.format(), "(2024) 5 SCC 789");
    }

    #[test]
    fn test_scr_citation() {
        let cite = Citation::scr(2024, 3, 456);
        assert_eq!(cite.format(), "[2024] 3 SCR 456");
    }

    #[test]
    fn test_high_court_citation() {
        let cite = Citation::high_court(2024, "Del", 100);
        assert_eq!(cite.format(), "2024 Del HC 100");
    }

    #[test]
    fn test_cite_helpers() {
        let cite = cite::dpdpa(4);
        assert_eq!(
            cite.format(),
            "Digital Personal Data Protection Act, 2023, s. 4"
        );

        let cite = cite::companies_act(173);
        assert_eq!(cite.format(), "Companies Act, 2013, s. 173");
    }
}
