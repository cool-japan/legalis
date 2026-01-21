//! Canada Common Types
//!
//! Core types shared across all Canadian law modules.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

// ============================================================================
// Province and Territory
// ============================================================================

/// Canadian provinces and territories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Province {
    // Provinces
    /// Alberta
    Alberta,
    /// British Columbia
    BritishColumbia,
    /// Manitoba
    Manitoba,
    /// New Brunswick
    NewBrunswick,
    /// Newfoundland and Labrador
    NewfoundlandLabrador,
    /// Nova Scotia
    NovaScotia,
    /// Ontario
    Ontario,
    /// Prince Edward Island
    PrinceEdwardIsland,
    /// Quebec (civil law jurisdiction)
    Quebec,
    /// Saskatchewan
    Saskatchewan,
    // Territories
    /// Northwest Territories
    NorthwestTerritories,
    /// Nunavut
    Nunavut,
    /// Yukon
    Yukon,
}

impl Province {
    /// Returns the abbreviation for the province
    pub fn abbreviation(&self) -> &'static str {
        match self {
            Self::Alberta => "AB",
            Self::BritishColumbia => "BC",
            Self::Manitoba => "MB",
            Self::NewBrunswick => "NB",
            Self::NewfoundlandLabrador => "NL",
            Self::NovaScotia => "NS",
            Self::Ontario => "ON",
            Self::PrinceEdwardIsland => "PE",
            Self::Quebec => "QC",
            Self::Saskatchewan => "SK",
            Self::NorthwestTerritories => "NT",
            Self::Nunavut => "NU",
            Self::Yukon => "YT",
        }
    }

    /// Returns the full name of the province
    pub fn full_name(&self) -> &'static str {
        match self {
            Self::Alberta => "Alberta",
            Self::BritishColumbia => "British Columbia",
            Self::Manitoba => "Manitoba",
            Self::NewBrunswick => "New Brunswick",
            Self::NewfoundlandLabrador => "Newfoundland and Labrador",
            Self::NovaScotia => "Nova Scotia",
            Self::Ontario => "Ontario",
            Self::PrinceEdwardIsland => "Prince Edward Island",
            Self::Quebec => "Québec",
            Self::Saskatchewan => "Saskatchewan",
            Self::NorthwestTerritories => "Northwest Territories",
            Self::Nunavut => "Nunavut",
            Self::Yukon => "Yukon",
        }
    }

    /// Whether this is a territory (vs province)
    pub fn is_territory(&self) -> bool {
        matches!(
            self,
            Self::NorthwestTerritories | Self::Nunavut | Self::Yukon
        )
    }

    /// Whether this jurisdiction uses civil law (Quebec)
    pub fn is_civil_law(&self) -> bool {
        matches!(self, Self::Quebec)
    }

    /// Whether this jurisdiction uses common law
    pub fn is_common_law(&self) -> bool {
        !self.is_civil_law()
    }

    /// All provinces (not territories)
    pub fn provinces() -> &'static [Province] {
        &[
            Self::Alberta,
            Self::BritishColumbia,
            Self::Manitoba,
            Self::NewBrunswick,
            Self::NewfoundlandLabrador,
            Self::NovaScotia,
            Self::Ontario,
            Self::PrinceEdwardIsland,
            Self::Quebec,
            Self::Saskatchewan,
        ]
    }

    /// All territories
    pub fn territories() -> &'static [Province] {
        &[Self::NorthwestTerritories, Self::Nunavut, Self::Yukon]
    }

    /// All provinces and territories
    pub fn all() -> &'static [Province] {
        &[
            Self::Alberta,
            Self::BritishColumbia,
            Self::Manitoba,
            Self::NewBrunswick,
            Self::NewfoundlandLabrador,
            Self::NovaScotia,
            Self::Ontario,
            Self::PrinceEdwardIsland,
            Self::Quebec,
            Self::Saskatchewan,
            Self::NorthwestTerritories,
            Self::Nunavut,
            Self::Yukon,
        ]
    }
}

impl std::fmt::Display for Province {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

// ============================================================================
// Jurisdictional Level
// ============================================================================

/// Level of jurisdiction (federal vs provincial/territorial)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JurisdictionalLevel {
    /// Federal (Parliament of Canada)
    Federal,
    /// Provincial/Territorial (provincial legislature)
    Provincial(Province),
    /// Municipal (delegated from province)
    Municipal { province: Province },
}

impl JurisdictionalLevel {
    /// Whether this is federal jurisdiction
    pub fn is_federal(&self) -> bool {
        matches!(self, Self::Federal)
    }

    /// Get the province if applicable
    pub fn province(&self) -> Option<Province> {
        match self {
            Self::Federal => None,
            Self::Provincial(p) | Self::Municipal { province: p } => Some(*p),
        }
    }
}

// ============================================================================
// Court Hierarchy
// ============================================================================

/// Canadian court hierarchy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Court {
    /// Supreme Court of Canada (highest court)
    SupremeCourt,
    /// Federal Court of Appeal
    FederalCourtOfAppeal,
    /// Federal Court
    FederalCourt,
    /// Tax Court of Canada
    TaxCourt,
    /// Provincial/Territorial Court of Appeal
    ProvincialCourtOfAppeal { province: Province },
    /// Provincial/Territorial Superior Court
    SuperiorCourt { province: Province, name: String },
    /// Provincial Court
    ProvincialCourt { province: Province },
    /// Administrative Tribunal
    Tribunal { name: String },
}

impl Court {
    /// Supreme Court of Canada
    pub fn scc() -> Self {
        Self::SupremeCourt
    }

    /// Ontario Court of Appeal
    pub fn onca() -> Self {
        Self::ProvincialCourtOfAppeal {
            province: Province::Ontario,
        }
    }

    /// British Columbia Court of Appeal
    pub fn bcca() -> Self {
        Self::ProvincialCourtOfAppeal {
            province: Province::BritishColumbia,
        }
    }

    /// Quebec Court of Appeal
    pub fn qcca() -> Self {
        Self::ProvincialCourtOfAppeal {
            province: Province::Quebec,
        }
    }

    /// Whether this court's decisions bind all lower courts
    pub fn is_apex_court(&self) -> bool {
        matches!(self, Self::SupremeCourt)
    }

    /// Precedent weight (higher = more authoritative)
    pub fn precedent_weight(&self) -> u32 {
        match self {
            Self::SupremeCourt => 100,
            Self::FederalCourtOfAppeal | Self::ProvincialCourtOfAppeal { .. } => 80,
            Self::FederalCourt | Self::SuperiorCourt { .. } => 60,
            Self::TaxCourt => 50,
            Self::ProvincialCourt { .. } => 40,
            Self::Tribunal { .. } => 20,
        }
    }
}

// ============================================================================
// Case Citation
// ============================================================================

/// Canadian case citation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseCitation {
    /// Case name (style of cause)
    pub name: String,
    /// Year of decision
    pub year: u32,
    /// Neutral citation (e.g., "2001 SCC 79")
    pub neutral_citation: Option<String>,
    /// Report citation (e.g., `[2001] 3 SCR 537`)
    pub report_citation: Option<String>,
    /// Court that decided the case
    pub court: Court,
    /// Key legal principle established
    pub principle: String,
}

impl CaseCitation {
    /// Create a new SCC case citation
    pub fn scc(name: &str, year: u32, scc_number: u32, principle: &str) -> Self {
        Self {
            name: name.to_string(),
            year,
            neutral_citation: Some(format!("{year} SCC {scc_number}")),
            report_citation: None,
            court: Court::SupremeCourt,
            principle: principle.to_string(),
        }
    }
}

// ============================================================================
// Statute Reference
// ============================================================================

/// Reference to a Canadian statute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteReference {
    /// Short title of the statute
    pub title: String,
    /// Full citation (e.g., "RSC 1985, c C-46")
    pub citation: String,
    /// Jurisdictional level
    pub level: JurisdictionalLevel,
    /// Section reference (if any)
    pub section: Option<String>,
}

impl StatuteReference {
    /// Criminal Code of Canada
    pub fn criminal_code() -> Self {
        Self {
            title: "Criminal Code".to_string(),
            citation: "RSC 1985, c C-46".to_string(),
            level: JurisdictionalLevel::Federal,
            section: None,
        }
    }

    /// Canada Business Corporations Act
    pub fn cbca() -> Self {
        Self {
            title: "Canada Business Corporations Act".to_string(),
            citation: "RSC 1985, c C-44".to_string(),
            level: JurisdictionalLevel::Federal,
            section: None,
        }
    }

    /// Income Tax Act
    pub fn income_tax_act() -> Self {
        Self {
            title: "Income Tax Act".to_string(),
            citation: "RSC 1985, c 1 (5th Supp)".to_string(),
            level: JurisdictionalLevel::Federal,
            section: None,
        }
    }

    /// Personal Information Protection and Electronic Documents Act
    pub fn pipeda() -> Self {
        Self {
            title: "Personal Information Protection and Electronic Documents Act".to_string(),
            citation: "SC 2000, c 5".to_string(),
            level: JurisdictionalLevel::Federal,
            section: None,
        }
    }

    /// Divorce Act
    pub fn divorce_act() -> Self {
        Self {
            title: "Divorce Act".to_string(),
            citation: "RSC 1985, c 3 (2nd Supp)".to_string(),
            level: JurisdictionalLevel::Federal,
            section: None,
        }
    }

    /// Canadian Charter of Rights and Freedoms
    pub fn charter() -> Self {
        Self {
            title: "Canadian Charter of Rights and Freedoms".to_string(),
            citation: "Part I of the Constitution Act, 1982".to_string(),
            level: JurisdictionalLevel::Federal,
            section: None,
        }
    }

    /// Constitution Act, 1867
    pub fn constitution_1867() -> Self {
        Self {
            title: "Constitution Act, 1867".to_string(),
            citation: "30 & 31 Vict, c 3".to_string(),
            level: JurisdictionalLevel::Federal,
            section: None,
        }
    }

    /// Civil Code of Quebec
    pub fn ccq() -> Self {
        Self {
            title: "Civil Code of Québec".to_string(),
            citation: "CQLR c CCQ-1991".to_string(),
            level: JurisdictionalLevel::Provincial(Province::Quebec),
            section: None,
        }
    }

    /// With a specific section
    pub fn with_section(mut self, section: &str) -> Self {
        self.section = Some(section.to_string());
        self
    }
}

// ============================================================================
// Legal System
// ============================================================================

/// Legal system type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalSystem {
    /// Common law (English tradition)
    CommonLaw,
    /// Civil law (French tradition - Quebec)
    CivilLaw,
    /// Bijural (both common and civil law apply)
    Bijural,
}

impl LegalSystem {
    /// Get the legal system for a province
    pub fn for_province(province: Province) -> Self {
        if province.is_civil_law() {
            Self::CivilLaw
        } else {
            Self::CommonLaw
        }
    }

    /// Federal level is bijural
    pub fn federal() -> Self {
        Self::Bijural
    }
}

// ============================================================================
// Language
// ============================================================================

/// Official languages of Canada
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfficialLanguage {
    /// English
    English,
    /// French
    French,
}

/// Bilingual requirement status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BilingualRequirement {
    /// Whether bilingual service is required
    pub required: bool,
    /// Jurisdiction level
    pub level: JurisdictionalLevel,
    /// Applicable statute
    pub statute: Option<String>,
}

impl BilingualRequirement {
    /// Federal institutions (Official Languages Act)
    pub fn federal() -> Self {
        Self {
            required: true,
            level: JurisdictionalLevel::Federal,
            statute: Some("Official Languages Act, RSC 1985, c 31 (4th Supp)".to_string()),
        }
    }

    /// New Brunswick (only officially bilingual province)
    pub fn new_brunswick() -> Self {
        Self {
            required: true,
            level: JurisdictionalLevel::Provincial(Province::NewBrunswick),
            statute: Some("Official Languages Act of New Brunswick".to_string()),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_province_abbreviations() {
        assert_eq!(Province::Ontario.abbreviation(), "ON");
        assert_eq!(Province::Quebec.abbreviation(), "QC");
        assert_eq!(Province::BritishColumbia.abbreviation(), "BC");
    }

    #[test]
    fn test_province_legal_system() {
        assert!(Province::Quebec.is_civil_law());
        assert!(Province::Ontario.is_common_law());
        assert!(!Province::Quebec.is_common_law());
    }

    #[test]
    fn test_province_territory() {
        assert!(Province::Yukon.is_territory());
        assert!(!Province::Ontario.is_territory());
    }

    #[test]
    fn test_court_precedent() {
        let scc = Court::SupremeCourt;
        let onca = Court::onca();
        assert!(scc.precedent_weight() > onca.precedent_weight());
    }

    #[test]
    fn test_statute_reference() {
        let charter = StatuteReference::charter();
        assert!(charter.title.contains("Charter"));

        let cc = StatuteReference::criminal_code();
        assert!(cc.citation.contains("C-46"));
    }

    #[test]
    fn test_legal_system() {
        assert_eq!(
            LegalSystem::for_province(Province::Quebec),
            LegalSystem::CivilLaw
        );
        assert_eq!(
            LegalSystem::for_province(Province::Ontario),
            LegalSystem::CommonLaw
        );
    }
}
