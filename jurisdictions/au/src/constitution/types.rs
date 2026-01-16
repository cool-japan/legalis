//! Australian Constitutional Types
//!
//! Core types for Australian constitutional law including
//! Commonwealth powers, separation of powers, and federalism.

use serde::{Deserialize, Serialize};

use crate::common::StateTerritory;

// ============================================================================
// Commonwealth Legislative Powers
// ============================================================================

/// Commonwealth legislative powers under the Constitution
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommonwealthPower {
    /// s.51(i) - Trade and commerce with other countries and among states
    TradeAndCommerce,
    /// s.51(ii) - Taxation (but not discriminate between states)
    Taxation,
    /// s.51(iii) - Bounties on production/export
    Bounties,
    /// s.51(iv) - Borrowing on public credit
    Borrowing,
    /// s.51(v) - Postal, telegraphic, telephonic services
    PostalServices,
    /// s.51(vi) - Naval and military defence
    Defence,
    /// s.51(ix) - Quarantine
    Quarantine,
    /// s.51(x) - Fisheries beyond territorial limits
    Fisheries,
    /// s.51(xi) - Census and statistics
    CensusStatistics,
    /// s.51(xii) - Currency, coinage, legal tender
    Currency,
    /// s.51(xiii) - Banking (except state banking)
    Banking,
    /// s.51(xiv) - Insurance (except state insurance)
    Insurance,
    /// s.51(xv) - Weights and measures
    WeightsMeasures,
    /// s.51(xvii) - Bankruptcy and insolvency
    BankruptcyInsolvency,
    /// s.51(xviii) - Copyrights, patents, trademarks
    IntellectualProperty,
    /// s.51(xix) - Naturalization and aliens
    Immigration,
    /// s.51(xx) - Foreign corporations, trading/financial corporations
    Corporations,
    /// s.51(xxi) - Marriage
    Marriage,
    /// s.51(xxii) - Divorce and matrimonial causes
    DivorceMatrimonial,
    /// s.51(xxiii) - Invalid and old-age pensions
    Pensions,
    /// s.51(xxiiiA) - Social services (1946 amendment)
    SocialServices,
    /// s.51(xxiv) - Service of civil process
    CivilProcess,
    /// s.51(xxv) - Seat of government
    SeatOfGovernment,
    /// s.51(xxvi) - People of any race (special laws)
    RacePower,
    /// s.51(xxix) - External affairs
    ExternalAffairs,
    /// s.51(xxx) - Relations with Pacific islands
    PacificIslands,
    /// s.51(xxxi) - Property acquisition on just terms
    PropertyAcquisition,
    /// s.51(xxxv) - Industrial disputes (interstate)
    IndustrialDisputes,
    /// s.51(xxxvii) - Referred powers from states
    ReferredPowers,
    /// s.51(xxxix) - Incidental power
    IncidentalPower,
    /// s.52 - Exclusive powers
    ExclusivePowers,
    /// s.122 - Territories power
    TerritoriesPower,
}

impl CommonwealthPower {
    /// Get the Constitution section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::TradeAndCommerce => "s.51(i)",
            Self::Taxation => "s.51(ii)",
            Self::Bounties => "s.51(iii)",
            Self::Borrowing => "s.51(iv)",
            Self::PostalServices => "s.51(v)",
            Self::Defence => "s.51(vi)",
            Self::Quarantine => "s.51(ix)",
            Self::Fisheries => "s.51(x)",
            Self::CensusStatistics => "s.51(xi)",
            Self::Currency => "s.51(xii)",
            Self::Banking => "s.51(xiii)",
            Self::Insurance => "s.51(xiv)",
            Self::WeightsMeasures => "s.51(xv)",
            Self::BankruptcyInsolvency => "s.51(xvii)",
            Self::IntellectualProperty => "s.51(xviii)",
            Self::Immigration => "s.51(xix)",
            Self::Corporations => "s.51(xx)",
            Self::Marriage => "s.51(xxi)",
            Self::DivorceMatrimonial => "s.51(xxii)",
            Self::Pensions => "s.51(xxiii)",
            Self::SocialServices => "s.51(xxiiiA)",
            Self::CivilProcess => "s.51(xxiv)",
            Self::SeatOfGovernment => "s.51(xxv)",
            Self::RacePower => "s.51(xxvi)",
            Self::ExternalAffairs => "s.51(xxix)",
            Self::PacificIslands => "s.51(xxx)",
            Self::PropertyAcquisition => "s.51(xxxi)",
            Self::IndustrialDisputes => "s.51(xxxv)",
            Self::ReferredPowers => "s.51(xxxvii)",
            Self::IncidentalPower => "s.51(xxxix)",
            Self::ExclusivePowers => "s.52",
            Self::TerritoriesPower => "s.122",
        }
    }

    /// Check if this is an exclusive Commonwealth power
    pub fn is_exclusive(&self) -> bool {
        matches!(
            self,
            Self::SeatOfGovernment | Self::ExclusivePowers | Self::TerritoriesPower
        )
    }

    /// Check if this is a concurrent power (shared with states)
    pub fn is_concurrent(&self) -> bool {
        !self.is_exclusive()
    }
}

// ============================================================================
// State Powers
// ============================================================================

/// Residual state powers (plenary power subject to Constitution)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatePower {
    /// Criminal law
    Criminal,
    /// Property and land
    Property,
    /// Education
    Education,
    /// Health services
    Health,
    /// Roads and transport
    Transport,
    /// Police
    Police,
    /// Courts and justice administration
    Courts,
    /// Local government
    LocalGovernment,
    /// Environment
    Environment,
    /// Planning and development
    Planning,
}

// ============================================================================
// Constitutional Provisions
// ============================================================================

/// Key constitutional provisions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstitutionalProvision {
    /// s.7 - Senate composition
    SenateComposition,
    /// s.24 - House of Representatives composition
    HouseComposition,
    /// s.51 - Legislative powers of Parliament
    LegislativePowers,
    /// s.52 - Exclusive powers
    ExclusivePowers,
    /// s.53 - Powers of Senate regarding money bills
    SenatePowers,
    /// s.57 - Double dissolution
    DoubleDissolution,
    /// s.61 - Executive power
    ExecutivePower,
    /// s.64 - Ministers must be parliamentarians
    MinisterialRequirement,
    /// s.71 - Judicial power
    JudicialPower,
    /// s.72 - Judges tenure
    JudicialTenure,
    /// s.75 - High Court original jurisdiction
    OriginalJurisdiction,
    /// s.76 - High Court appellate jurisdiction
    AppellateJurisdiction,
    /// s.90 - Customs and excise (exclusive)
    CustomsExcise,
    /// s.92 - Free trade among states
    FreeTrade,
    /// s.96 - Financial grants to states
    FinancialGrants,
    /// s.106 - State constitutions saved
    StateConstitutions,
    /// s.107 - State powers saved
    StatePowersSaved,
    /// s.109 - Inconsistency (Commonwealth prevails)
    Inconsistency,
    /// s.114 - States cannot raise forces
    NoStateForces,
    /// s.116 - Freedom of religion
    FreedomOfReligion,
    /// s.117 - State residents discrimination
    StateResidents,
    /// s.128 - Amendment procedure
    AmendmentProcedure,
}

impl ConstitutionalProvision {
    /// Get the section number
    pub fn section(&self) -> u32 {
        match self {
            Self::SenateComposition => 7,
            Self::HouseComposition => 24,
            Self::LegislativePowers => 51,
            Self::ExclusivePowers => 52,
            Self::SenatePowers => 53,
            Self::DoubleDissolution => 57,
            Self::ExecutivePower => 61,
            Self::MinisterialRequirement => 64,
            Self::JudicialPower => 71,
            Self::JudicialTenure => 72,
            Self::OriginalJurisdiction => 75,
            Self::AppellateJurisdiction => 76,
            Self::CustomsExcise => 90,
            Self::FreeTrade => 92,
            Self::FinancialGrants => 96,
            Self::StateConstitutions => 106,
            Self::StatePowersSaved => 107,
            Self::Inconsistency => 109,
            Self::NoStateForces => 114,
            Self::FreedomOfReligion => 116,
            Self::StateResidents => 117,
            Self::AmendmentProcedure => 128,
        }
    }
}

// ============================================================================
// Express Rights
// ============================================================================

/// Express constitutional rights (limited in Australian Constitution)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExpressRight {
    /// s.80 - Trial by jury for indictable Commonwealth offences
    TrialByJury,
    /// s.92 - Free trade (as protected right)
    FreeTrade,
    /// s.116 - Freedom of religion (Commonwealth laws only)
    FreedomOfReligion,
    /// s.117 - Protection against state discrimination
    StateResidentProtection,
    /// s.51(xxxi) - Just terms for property acquisition
    JustTerms,
}

impl ExpressRight {
    /// Get the section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::TrialByJury => "s.80",
            Self::FreeTrade => "s.92",
            Self::FreedomOfReligion => "s.116",
            Self::StateResidentProtection => "s.117",
            Self::JustTerms => "s.51(xxxi)",
        }
    }
}

// ============================================================================
// Implied Rights
// ============================================================================

/// Implied constitutional rights (developed through case law)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImpliedRight {
    /// Implied freedom of political communication (Lange v ABC)
    PoliticalCommunication,
    /// Implied right to vote (Roach v Electoral Commissioner)
    RightToVote,
    /// Implied protection of state courts from abolition
    StateCourtProtection,
    /// Implied requirement of fair trial (Dietrich)
    FairTrial,
}

impl ImpliedRight {
    /// Get the key case establishing this right
    pub fn leading_case(&self) -> &'static str {
        match self {
            Self::PoliticalCommunication => {
                "Lange v Australian Broadcasting Corporation (1997) 189 CLR 520"
            }
            Self::RightToVote => "Roach v Electoral Commissioner (2007) 233 CLR 162",
            Self::StateCourtProtection => {
                "Kable v Director of Public Prosecutions (NSW) (1996) 189 CLR 51"
            }
            Self::FairTrial => "Dietrich v The Queen (1992) 177 CLR 292",
        }
    }

    /// Get the constitutional basis
    pub fn constitutional_basis(&self) -> &'static str {
        match self {
            Self::PoliticalCommunication => {
                "Representative and responsible government (ss.7, 24, 64, 128)"
            }
            Self::RightToVote => "Directly chosen by the people (ss.7, 24)",
            Self::StateCourtProtection => "Chapter III and integrated court system",
            Self::FairTrial => "Chapter III - judicial power exercised by courts",
        }
    }
}

// ============================================================================
// Separation of Powers
// ============================================================================

/// Branch of government
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GovernmentBranch {
    /// Parliament (legislative)
    Legislative,
    /// Executive
    Executive,
    /// Judiciary
    Judicial,
}

/// Separation of powers doctrine
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SeparationDoctrine {
    /// Strict separation at Commonwealth level
    StrictCommonwealth,
    /// Kable doctrine - states cannot use courts for non-judicial functions
    KableDoctrine,
    /// Boilermakers - no mixing judicial and non-judicial functions
    BoilermakersDoctrine,
}

// ============================================================================
// Inconsistency (s.109)
// ============================================================================

/// Type of inconsistency under s.109
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InconsistencyType {
    /// Direct inconsistency - impossible to obey both
    Direct,
    /// Indirect/covering the field - Commonwealth law covers the field
    CoveringTheField,
}

/// Section 109 analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InconsistencyAnalysis {
    /// Commonwealth law identifier
    pub commonwealth_law: String,
    /// State law identifier
    pub state_law: String,
    /// Type of inconsistency found
    pub inconsistency_type: Option<InconsistencyType>,
    /// Whether state law is inoperative
    pub state_law_inoperative: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Characterization
// ============================================================================

/// Law characterization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterizationResult {
    /// Law being characterized
    pub law_id: String,
    /// Claimed constitutional head of power
    pub claimed_power: CommonwealthPower,
    /// Whether law is within power
    pub within_power: bool,
    /// Sufficient connection test result
    pub sufficient_connection: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Melbourne Corporation Doctrine
// ============================================================================

/// Melbourne Corporation doctrine (federal limits on states)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MelbourneCorporationAnalysis {
    /// Commonwealth law
    pub commonwealth_law: String,
    /// Affected state/territory
    pub affected_state: StateTerritory,
    /// Whether law discriminates against states
    pub discriminates: bool,
    /// Whether law impairs state capacity to function
    pub impairs_functioning: bool,
    /// Law is invalid under Melbourne Corporation
    pub invalid: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Key Cases
// ============================================================================

/// Constitutional law key case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalCase {
    /// Case name
    pub name: String,
    /// Year
    pub year: u16,
    /// Citation
    pub citation: String,
    /// Principle established
    pub principle: String,
    /// Relevant constitutional provision
    pub provision: Option<ConstitutionalProvision>,
}

impl ConstitutionalCase {
    /// Engineers Case - end of reserved state powers
    pub fn engineers() -> Self {
        Self {
            name: "Amalgamated Society of Engineers v Adelaide Steamship Co Ltd".to_string(),
            year: 1920,
            citation: "(1920) 28 CLR 129".to_string(),
            principle:
                "Constitution to be interpreted according to its terms; no reserved powers doctrine"
                    .to_string(),
            provision: Some(ConstitutionalProvision::LegislativePowers),
        }
    }

    /// Cole v Whitfield - s.92 reinterpretation
    pub fn cole_v_whitfield() -> Self {
        Self {
            name: "Cole v Whitfield".to_string(),
            year: 1988,
            citation: "(1988) 165 CLR 360".to_string(),
            principle: "s.92 prohibits only discriminatory/protectionist measures, not all trade regulation".to_string(),
            provision: Some(ConstitutionalProvision::FreeTrade),
        }
    }

    /// Victoria v Commonwealth (Industrial Relations Act Case)
    pub fn work_choices() -> Self {
        Self {
            name: "New South Wales v Commonwealth (Work Choices Case)".to_string(),
            year: 2006,
            citation: "(2006) 229 CLR 1".to_string(),
            principle: "Corporations power supports comprehensive workplace relations legislation"
                .to_string(),
            provision: Some(ConstitutionalProvision::LegislativePowers),
        }
    }

    /// Melbourne Corporation v Commonwealth
    pub fn melbourne_corporation() -> Self {
        Self {
            name: "Melbourne Corporation v Commonwealth".to_string(),
            year: 1947,
            citation: "(1947) 74 CLR 31".to_string(),
            principle: "Commonwealth cannot discriminate against states or impair their capacity to function".to_string(),
            provision: Some(ConstitutionalProvision::Inconsistency),
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
    fn test_commonwealth_power_sections() {
        assert_eq!(CommonwealthPower::TradeAndCommerce.section(), "s.51(i)");
        assert_eq!(CommonwealthPower::Corporations.section(), "s.51(xx)");
        assert_eq!(CommonwealthPower::ExternalAffairs.section(), "s.51(xxix)");
    }

    #[test]
    fn test_exclusive_powers() {
        assert!(CommonwealthPower::SeatOfGovernment.is_exclusive());
        assert!(!CommonwealthPower::Taxation.is_exclusive());
        assert!(CommonwealthPower::Taxation.is_concurrent());
    }

    #[test]
    fn test_express_rights() {
        assert_eq!(ExpressRight::TrialByJury.section(), "s.80");
        assert_eq!(ExpressRight::FreedomOfReligion.section(), "s.116");
    }

    #[test]
    fn test_implied_rights() {
        let pr = ImpliedRight::PoliticalCommunication;
        assert!(pr.leading_case().contains("Lange"));
        assert!(pr.constitutional_basis().contains("Representative"));
    }

    #[test]
    fn test_engineers_case() {
        let engineers = ConstitutionalCase::engineers();
        assert_eq!(engineers.year, 1920);
        assert!(engineers.principle.contains("reserved powers"));
    }
}
