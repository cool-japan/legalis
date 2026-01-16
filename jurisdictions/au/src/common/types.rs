//! Australian Common Types
//!
//! Core type definitions for Australian law, including states, territories,
//! and common enumerations.

use serde::{Deserialize, Serialize};

// ============================================================================
// States and Territories
// ============================================================================

/// Australian states and territories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateTerritory {
    /// New South Wales
    NewSouthWales,
    /// Victoria
    Victoria,
    /// Queensland
    Queensland,
    /// South Australia
    SouthAustralia,
    /// Western Australia
    WesternAustralia,
    /// Tasmania
    Tasmania,
    /// Northern Territory
    NorthernTerritory,
    /// Australian Capital Territory
    AustralianCapitalTerritory,
}

impl StateTerritory {
    /// Get the standard abbreviation
    pub fn abbreviation(&self) -> &'static str {
        match self {
            Self::NewSouthWales => "NSW",
            Self::Victoria => "VIC",
            Self::Queensland => "QLD",
            Self::SouthAustralia => "SA",
            Self::WesternAustralia => "WA",
            Self::Tasmania => "TAS",
            Self::NorthernTerritory => "NT",
            Self::AustralianCapitalTerritory => "ACT",
        }
    }

    /// Get the full name
    pub fn full_name(&self) -> &'static str {
        match self {
            Self::NewSouthWales => "New South Wales",
            Self::Victoria => "Victoria",
            Self::Queensland => "Queensland",
            Self::SouthAustralia => "South Australia",
            Self::WesternAustralia => "Western Australia",
            Self::Tasmania => "Tasmania",
            Self::NorthernTerritory => "Northern Territory",
            Self::AustralianCapitalTerritory => "Australian Capital Territory",
        }
    }

    /// Check if this is a territory (vs a state)
    pub fn is_territory(&self) -> bool {
        matches!(
            self,
            Self::NorthernTerritory | Self::AustralianCapitalTerritory
        )
    }

    /// Check if this is a state
    pub fn is_state(&self) -> bool {
        !self.is_territory()
    }

    /// Get all states
    pub fn states() -> &'static [StateTerritory] {
        &[
            StateTerritory::NewSouthWales,
            StateTerritory::Victoria,
            StateTerritory::Queensland,
            StateTerritory::SouthAustralia,
            StateTerritory::WesternAustralia,
            StateTerritory::Tasmania,
        ]
    }

    /// Get all territories
    pub fn territories() -> &'static [StateTerritory] {
        &[
            StateTerritory::NorthernTerritory,
            StateTerritory::AustralianCapitalTerritory,
        ]
    }

    /// Get all states and territories
    pub fn all() -> &'static [StateTerritory] {
        &[
            StateTerritory::NewSouthWales,
            StateTerritory::Victoria,
            StateTerritory::Queensland,
            StateTerritory::SouthAustralia,
            StateTerritory::WesternAustralia,
            StateTerritory::Tasmania,
            StateTerritory::NorthernTerritory,
            StateTerritory::AustralianCapitalTerritory,
        ]
    }

    /// Get the Civil Liability Act reference for this state/territory
    pub fn civil_liability_act(&self) -> &'static str {
        match self {
            Self::NewSouthWales => "Civil Liability Act 2002 (NSW)",
            Self::Victoria => "Wrongs Act 1958 (Vic)",
            Self::Queensland => "Civil Liability Act 2003 (Qld)",
            Self::SouthAustralia => "Civil Liability Act 1936 (SA)",
            Self::WesternAustralia => "Civil Liability Act 2002 (WA)",
            Self::Tasmania => "Civil Liability Act 2002 (Tas)",
            Self::NorthernTerritory => "Personal Injuries (Liabilities and Damages) Act 2003 (NT)",
            Self::AustralianCapitalTerritory => "Civil Law (Wrongs) Act 2002 (ACT)",
        }
    }

    /// Get timezone offset (hours from UTC, standard time)
    pub fn timezone_offset(&self) -> i32 {
        match self {
            Self::NewSouthWales
            | Self::Victoria
            | Self::Tasmania
            | Self::AustralianCapitalTerritory => 10, // AEST
            Self::Queensland => 10, // AEST (no DST)
            Self::SouthAustralia | Self::NorthernTerritory => 9, // ACST (9.5 rounded)
            Self::WesternAustralia => 8, // AWST
        }
    }
}

impl std::fmt::Display for StateTerritory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

// ============================================================================
// Jurisdiction Levels
// ============================================================================

/// Level of government in Australia
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JurisdictionLevel {
    /// Commonwealth (federal) government
    Commonwealth,
    /// State government
    State(StateTerritory),
    /// Territory government (more limited powers than states)
    Territory(StateTerritory),
    /// Local government
    Local,
}

impl JurisdictionLevel {
    /// Create a new state/territory jurisdiction
    pub fn from_state_territory(st: StateTerritory) -> Self {
        if st.is_territory() {
            Self::Territory(st)
        } else {
            Self::State(st)
        }
    }
}

// ============================================================================
// Court Hierarchy
// ============================================================================

/// Australian court hierarchy
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Court {
    /// High Court of Australia
    HighCourt,
    /// Federal Court of Australia
    FederalCourt,
    /// Federal Circuit Court
    FederalCircuitCourt,
    /// Family Court of Australia
    FamilyCourt,
    /// Administrative Appeals Tribunal
    AAT,
    /// Fair Work Commission
    FairWorkCommission,
    /// State Supreme Court
    StateSupremeCourt(StateTerritory),
    /// State District/County Court
    StateDistrictCourt(StateTerritory),
    /// Local/Magistrates Court
    LocalCourt(StateTerritory),
    /// State Industrial/Employment Tribunal
    StateIndustrialTribunal(StateTerritory),
    /// State Civil and Administrative Tribunal
    StateCivilTribunal(StateTerritory),
}

impl Court {
    /// Get the hierarchy level (higher = more authoritative)
    pub fn hierarchy_level(&self) -> u32 {
        match self {
            Self::HighCourt => 100,
            Self::FederalCourt | Self::StateSupremeCourt(_) => 80,
            Self::FederalCircuitCourt | Self::FamilyCourt | Self::StateDistrictCourt(_) => 60,
            Self::AAT
            | Self::FairWorkCommission
            | Self::StateIndustrialTribunal(_)
            | Self::StateCivilTribunal(_) => 40,
            Self::LocalCourt(_) => 20,
        }
    }

    /// Check if this court's decisions bind another court
    pub fn binds(&self, other: &Court) -> bool {
        self.hierarchy_level() > other.hierarchy_level()
    }
}

// ============================================================================
// Legal Areas
// ============================================================================

/// Areas of Australian law
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalArea {
    /// Constitutional law
    Constitutional,
    /// Contract law
    Contract,
    /// Tort law (negligence, nuisance, defamation)
    Tort,
    /// Property law (real property, Torrens)
    Property,
    /// Native title
    NativeTitle,
    /// Employment/workplace relations
    Employment,
    /// Criminal law
    Criminal,
    /// Family law
    Family,
    /// Corporate/commercial law
    Corporate,
    /// Competition/consumer law
    Competition,
    /// Administrative law
    Administrative,
    /// Environmental law
    Environmental,
    /// Intellectual property
    IntellectualProperty,
    /// Privacy law
    Privacy,
    /// Migration law
    Migration,
    /// Tax law
    Tax,
}

// ============================================================================
// Key Cases
// ============================================================================

/// Important Australian case reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AustralianCase {
    /// Case name
    pub name: String,
    /// Year
    pub year: u16,
    /// Citation
    pub citation: String,
    /// Court
    pub court: Court,
    /// Legal area
    pub area: LegalArea,
    /// Key principle established
    pub principle: String,
}

impl AustralianCase {
    /// Create a new case reference
    pub fn new(
        name: impl Into<String>,
        year: u16,
        citation: impl Into<String>,
        court: Court,
        area: LegalArea,
        principle: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            year,
            citation: citation.into(),
            court,
            area,
            principle: principle.into(),
        }
    }

    /// Mabo v Queensland (No 2) - Native title recognition
    pub fn mabo() -> Self {
        Self::new(
            "Mabo v Queensland (No 2)",
            1992,
            "(1992) 175 CLR 1",
            Court::HighCourt,
            LegalArea::NativeTitle,
            "Recognized native title as part of Australian common law, overturning terra nullius",
        )
    }

    /// Donoghue v Stevenson (adopted in Australia)
    pub fn donoghue_v_stevenson() -> Self {
        Self::new(
            "Donoghue v Stevenson",
            1932,
            "[1932] AC 562",
            Court::HighCourt, // Adopted
            LegalArea::Tort,
            "Neighbour principle - duty of care owed to those reasonably foreseeable as affected",
        )
    }

    /// Sullivan v Moody - Australian duty of care
    pub fn sullivan_v_moody() -> Self {
        Self::new(
            "Sullivan v Moody",
            2001,
            "(2001) 207 CLR 562",
            Court::HighCourt,
            LegalArea::Tort,
            "Novel duty of care requires coherence with existing legal principles",
        )
    }

    /// Lange v ABC - Implied freedom of political communication
    pub fn lange_v_abc() -> Self {
        Self::new(
            "Lange v Australian Broadcasting Corporation",
            1997,
            "(1997) 189 CLR 520",
            Court::HighCourt,
            LegalArea::Constitutional,
            "Implied freedom of political communication derived from constitutional structure",
        )
    }

    /// Wik Peoples v Queensland - Native title coexistence
    pub fn wik() -> Self {
        Self::new(
            "Wik Peoples v Queensland",
            1996,
            "(1996) 187 CLR 1",
            Court::HighCourt,
            LegalArea::NativeTitle,
            "Native title can coexist with pastoral leases",
        )
    }

    /// ACCC v Baxter Healthcare - Competition law
    pub fn accc_v_baxter() -> Self {
        Self::new(
            "ACCC v Baxter Healthcare",
            2007,
            "(2007) 232 CLR 1",
            Court::HighCourt,
            LegalArea::Competition,
            "Competition law applies to government entities when trading",
        )
    }

    /// Rogers v Whitaker - Medical negligence informed consent
    pub fn rogers_v_whitaker() -> Self {
        Self::new(
            "Rogers v Whitaker",
            1992,
            "(1992) 175 CLR 479",
            Court::HighCourt,
            LegalArea::Tort,
            "Duty to disclose material risks - rejected Bolam test for disclosure",
        )
    }

    /// Cole v Whitfield - s.92 free trade interpretation
    pub fn cole_v_whitfield() -> Self {
        Self::new(
            "Cole v Whitfield",
            1988,
            "(1988) 165 CLR 360",
            Court::HighCourt,
            LegalArea::Constitutional,
            "Section 92 only prohibits discriminatory/protectionist measures, not all trade restrictions",
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_abbreviations() {
        assert_eq!(StateTerritory::NewSouthWales.abbreviation(), "NSW");
        assert_eq!(StateTerritory::Victoria.abbreviation(), "VIC");
        assert_eq!(
            StateTerritory::AustralianCapitalTerritory.abbreviation(),
            "ACT"
        );
    }

    #[test]
    fn test_is_territory() {
        assert!(!StateTerritory::NewSouthWales.is_territory());
        assert!(!StateTerritory::Victoria.is_territory());
        assert!(StateTerritory::NorthernTerritory.is_territory());
        assert!(StateTerritory::AustralianCapitalTerritory.is_territory());
    }

    #[test]
    fn test_states_count() {
        assert_eq!(StateTerritory::states().len(), 6);
        assert_eq!(StateTerritory::territories().len(), 2);
        assert_eq!(StateTerritory::all().len(), 8);
    }

    #[test]
    fn test_court_hierarchy() {
        let high_court = Court::HighCourt;
        let federal = Court::FederalCourt;
        let nsw_supreme = Court::StateSupremeCourt(StateTerritory::NewSouthWales);
        let local = Court::LocalCourt(StateTerritory::Victoria);

        assert!(high_court.binds(&federal));
        assert!(high_court.binds(&nsw_supreme));
        assert!(federal.binds(&local));
        assert!(!local.binds(&federal));
    }

    #[test]
    fn test_mabo_case() {
        let mabo = AustralianCase::mabo();
        assert_eq!(mabo.year, 1992);
        assert_eq!(mabo.court, Court::HighCourt);
        assert!(mabo.principle.contains("native title"));
    }

    #[test]
    fn test_lange_case() {
        let lange = AustralianCase::lange_v_abc();
        assert_eq!(lange.year, 1997);
        assert!(lange.principle.contains("political communication"));
    }

    #[test]
    fn test_civil_liability_acts() {
        assert!(
            StateTerritory::NewSouthWales
                .civil_liability_act()
                .contains("2002")
        );
        assert!(
            StateTerritory::Victoria
                .civil_liability_act()
                .contains("Wrongs Act")
        );
    }
}
