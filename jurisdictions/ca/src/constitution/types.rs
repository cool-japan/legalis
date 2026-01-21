//! Canada Constitutional Law - Types
//!
//! Core types for Canadian constitutional law, including the Charter of Rights
//! and Freedoms and the division of powers under the Constitution Act, 1867.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::common::CaseCitation;

// ============================================================================
// Charter of Rights and Freedoms
// ============================================================================

/// Charter right or freedom
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharterRight {
    // Fundamental Freedoms (s.2)
    /// Freedom of conscience and religion (s.2(a))
    FreedomOfReligion,
    /// Freedom of thought, belief, opinion and expression (s.2(b))
    FreedomOfExpression,
    /// Freedom of peaceful assembly (s.2(c))
    FreedomOfAssembly,
    /// Freedom of association (s.2(d))
    FreedomOfAssociation,

    // Democratic Rights (ss.3-5)
    /// Right to vote (s.3)
    RightToVote,
    /// Maximum duration of legislative bodies (s.4)
    MaximumDuration,
    /// Annual sitting of legislative bodies (s.5)
    AnnualSitting,

    // Mobility Rights (s.6)
    /// Right to enter, remain in and leave Canada (s.6(1))
    MobilityOfCitizens,
    /// Right to move and gain livelihood (s.6(2))
    MobilityToWorkAndLive,

    // Legal Rights (ss.7-14)
    /// Life, liberty and security of person (s.7)
    LifeLibertySecurityOfPerson,
    /// Protection against unreasonable search and seizure (s.8)
    SearchAndSeizure,
    /// Protection against arbitrary detention (s.9)
    ArbitraryDetention,
    /// Rights on arrest or detention (s.10)
    RightsOnArrest,
    /// Proceedings in criminal and penal matters (s.11)
    CriminalProceedingsRights,
    /// Protection against cruel and unusual treatment (s.12)
    CruelAndUnusualTreatment,
    /// Right against self-incrimination (s.13)
    SelfIncrimination,
    /// Right to interpreter (s.14)
    Interpreter,

    // Equality Rights (s.15)
    /// Equality before and under law (s.15(1))
    EqualityRights,
    /// Affirmative action programs (s.15(2))
    AffirmativeAction,

    // Language Rights (ss.16-22)
    /// Official languages of Canada (s.16)
    OfficialLanguages,
    /// Right to use official languages in Parliament (s.17-18)
    LanguageInParliament,
    /// Right to communicate in official languages (s.20)
    LanguageOfService,
    /// Language of instruction (s.23)
    MinorityLanguageEducation,
}

impl CharterRight {
    /// Get the section number
    pub fn section(&self) -> &'static str {
        match self {
            Self::FreedomOfReligion => "2(a)",
            Self::FreedomOfExpression => "2(b)",
            Self::FreedomOfAssembly => "2(c)",
            Self::FreedomOfAssociation => "2(d)",
            Self::RightToVote => "3",
            Self::MaximumDuration => "4",
            Self::AnnualSitting => "5",
            Self::MobilityOfCitizens => "6(1)",
            Self::MobilityToWorkAndLive => "6(2)",
            Self::LifeLibertySecurityOfPerson => "7",
            Self::SearchAndSeizure => "8",
            Self::ArbitraryDetention => "9",
            Self::RightsOnArrest => "10",
            Self::CriminalProceedingsRights => "11",
            Self::CruelAndUnusualTreatment => "12",
            Self::SelfIncrimination => "13",
            Self::Interpreter => "14",
            Self::EqualityRights => "15(1)",
            Self::AffirmativeAction => "15(2)",
            Self::OfficialLanguages => "16",
            Self::LanguageInParliament => "17-18",
            Self::LanguageOfService => "20",
            Self::MinorityLanguageEducation => "23",
        }
    }

    /// Whether this right can be overridden by the notwithstanding clause (s.33)
    pub fn can_be_overridden(&self) -> bool {
        matches!(
            self,
            Self::FreedomOfReligion
                | Self::FreedomOfExpression
                | Self::FreedomOfAssembly
                | Self::FreedomOfAssociation
                | Self::LifeLibertySecurityOfPerson
                | Self::SearchAndSeizure
                | Self::ArbitraryDetention
                | Self::RightsOnArrest
                | Self::CriminalProceedingsRights
                | Self::CruelAndUnusualTreatment
                | Self::SelfIncrimination
                | Self::Interpreter
                | Self::EqualityRights
        )
    }

    /// Whether this is an absolute right (cannot be justified under s.1)
    pub fn is_absolute(&self) -> bool {
        // In practice, no Charter rights are truly absolute - all are subject to s.1
        // But some rights have been interpreted more strictly
        false
    }
}

// ============================================================================
// Section 1 - Oakes Test
// ============================================================================

/// The Oakes test for justifying Charter limitations (s.1)
/// From R v Oakes \[1986\] 1 SCR 103
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OakesTest {
    /// Pressing and substantial objective
    pub pressing_objective: PressAndSubstantial,
    /// Proportionality analysis
    pub proportionality: ProportionalityAnalysis,
}

/// Analysis of pressing and substantial objective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressAndSubstantial {
    /// The objective of the law
    pub objective: String,
    /// Whether objective relates to pressing and substantial concerns
    pub is_pressing: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Proportionality analysis (three-part test)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProportionalityAnalysis {
    /// Rational connection between means and objective
    pub rational_connection: RationalConnection,
    /// Minimal impairment of the right
    pub minimal_impairment: MinimalImpairment,
    /// Proportionality between effects and objective
    pub proportionality_stricto_sensu: ProportionalityStrictoSensu,
}

/// Rational connection analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RationalConnection {
    /// Whether there is a rational connection
    pub connected: bool,
    /// The means adopted
    pub means: String,
    /// Reasoning
    pub reasoning: String,
}

/// Minimal impairment analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimalImpairment {
    /// Whether the impairment is minimal
    pub is_minimal: bool,
    /// Alternatives considered
    pub alternatives: Vec<String>,
    /// Why alternatives are not as effective
    pub why_not_alternatives: String,
}

/// Proportionality stricto sensu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProportionalityStrictoSensu {
    /// Whether proportionate
    pub proportionate: bool,
    /// Benefits of the measure
    pub benefits: Vec<String>,
    /// Deleterious effects on the right
    pub deleterious_effects: Vec<String>,
    /// Overall balance
    pub balance: String,
}

// ============================================================================
// Division of Powers
// ============================================================================

/// Federal powers under s.91 of the Constitution Act, 1867
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FederalPower {
    /// Peace, order, and good government (POGG - residual power)
    Pogg,
    /// Public debt and property (s.91(1A))
    PublicDebt,
    /// Regulation of trade and commerce (s.91(2))
    TradeAndCommerce,
    /// Unemployment insurance (s.91(2A))
    UnemploymentInsurance,
    /// Direct and indirect taxation (s.91(3))
    Taxation,
    /// Borrowing on public credit (s.91(4))
    Borrowing,
    /// Postal service (s.91(5))
    PostalService,
    /// Census and statistics (s.91(6))
    CensusStatistics,
    /// Militia, military, naval service (s.91(7))
    DefenceForces,
    /// Salaries of civil servants (s.91(8))
    CivilServantSalaries,
    /// Beacons, buoys, lighthouses (s.91(9))
    NavigationAids,
    /// Navigation and shipping (s.91(10))
    NavigationShipping,
    /// Quarantine (s.91(11))
    Quarantine,
    /// Sea coast and inland fisheries (s.91(12))
    Fisheries,
    /// Ferries (s.91(13))
    Ferries,
    /// Currency and coinage (s.91(14))
    Currency,
    /// Banking (s.91(15))
    Banking,
    /// Savings banks (s.91(16))
    SavingsBanks,
    /// Weights and measures (s.91(17))
    WeightsMeasures,
    /// Bills of exchange and promissory notes (s.91(18))
    BillsOfExchange,
    /// Interest (s.91(19))
    Interest,
    /// Legal tender (s.91(20))
    LegalTender,
    /// Bankruptcy and insolvency (s.91(21))
    BankruptcyInsolvency,
    /// Patents of invention and discovery (s.91(22))
    Patents,
    /// Copyrights (s.91(23))
    Copyrights,
    /// Indians and lands reserved for Indians (s.91(24))
    IndiansAndLands,
    /// Naturalization and aliens (s.91(25))
    NaturalizationAliens,
    /// Marriage and divorce (s.91(26))
    MarriageDivorce,
    /// Criminal law (s.91(27))
    CriminalLaw,
    /// Penitentiaries (s.91(28))
    Penitentiaries,
    /// Works declared for general advantage (s.91(29))
    WorksGeneralAdvantage,
    /// Telecommunications (implied from POGG and trade)
    Telecommunications,
    /// Aeronautics (implied from POGG)
    Aeronautics,
    /// Nuclear energy (implied from POGG)
    NuclearEnergy,
}

impl FederalPower {
    /// Get the section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::Pogg => "91 (opening words)",
            Self::PublicDebt => "91(1A)",
            Self::TradeAndCommerce => "91(2)",
            Self::UnemploymentInsurance => "91(2A)",
            Self::Taxation => "91(3)",
            Self::Borrowing => "91(4)",
            Self::PostalService => "91(5)",
            Self::CensusStatistics => "91(6)",
            Self::DefenceForces => "91(7)",
            Self::CivilServantSalaries => "91(8)",
            Self::NavigationAids => "91(9)",
            Self::NavigationShipping => "91(10)",
            Self::Quarantine => "91(11)",
            Self::Fisheries => "91(12)",
            Self::Ferries => "91(13)",
            Self::Currency => "91(14)",
            Self::Banking => "91(15)",
            Self::SavingsBanks => "91(16)",
            Self::WeightsMeasures => "91(17)",
            Self::BillsOfExchange => "91(18)",
            Self::Interest => "91(19)",
            Self::LegalTender => "91(20)",
            Self::BankruptcyInsolvency => "91(21)",
            Self::Patents => "91(22)",
            Self::Copyrights => "91(23)",
            Self::IndiansAndLands => "91(24)",
            Self::NaturalizationAliens => "91(25)",
            Self::MarriageDivorce => "91(26)",
            Self::CriminalLaw => "91(27)",
            Self::Penitentiaries => "91(28)",
            Self::WorksGeneralAdvantage => "91(29)",
            Self::Telecommunications | Self::Aeronautics | Self::NuclearEnergy => "91 (implied)",
        }
    }
}

/// Provincial powers under s.92 of the Constitution Act, 1867
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvincialPower {
    /// Amendment of provincial constitution (s.92(1))
    ProvincialConstitution,
    /// Direct taxation within province (s.92(2))
    DirectTaxation,
    /// Borrowing on provincial credit (s.92(3))
    Borrowing,
    /// Provincial offices (s.92(4))
    ProvincialOffices,
    /// Public lands (s.92(5))
    PublicLands,
    /// Prisons (s.92(6))
    Prisons,
    /// Hospitals, asylums, charities (s.92(7))
    Hospitals,
    /// Municipal institutions (s.92(8))
    MunicipalInstitutions,
    /// Licences (s.92(9))
    Licences,
    /// Local works and undertakings (s.92(10))
    LocalWorks,
    /// Incorporation of provincial companies (s.92(11))
    ProvincialCompanies,
    /// Solemnization of marriage (s.92(12))
    SolemnizationOfMarriage,
    /// Property and civil rights (s.92(13))
    PropertyAndCivilRights,
    /// Administration of justice (s.92(14))
    AdministrationOfJustice,
    /// Penalties for provincial laws (s.92(15))
    Penalties,
    /// Matters of merely local or private nature (s.92(16))
    LocalMatters,
    /// Non-renewable natural resources (s.92A)
    NaturalResources,
    /// Education (s.93)
    Education,
    /// Health care (implied from hospitals + property/civil rights)
    HealthCare,
    /// Labour relations (from property and civil rights)
    LabourRelations,
    /// Securities regulation (from property and civil rights)
    SecuritiesRegulation,
}

impl ProvincialPower {
    /// Get the section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::ProvincialConstitution => "92(1)",
            Self::DirectTaxation => "92(2)",
            Self::Borrowing => "92(3)",
            Self::ProvincialOffices => "92(4)",
            Self::PublicLands => "92(5)",
            Self::Prisons => "92(6)",
            Self::Hospitals => "92(7)",
            Self::MunicipalInstitutions => "92(8)",
            Self::Licences => "92(9)",
            Self::LocalWorks => "92(10)",
            Self::ProvincialCompanies => "92(11)",
            Self::SolemnizationOfMarriage => "92(12)",
            Self::PropertyAndCivilRights => "92(13)",
            Self::AdministrationOfJustice => "92(14)",
            Self::Penalties => "92(15)",
            Self::LocalMatters => "92(16)",
            Self::NaturalResources => "92A",
            Self::Education => "93",
            Self::HealthCare | Self::LabourRelations | Self::SecuritiesRegulation => {
                "92(13) (implied)"
            }
        }
    }
}

// ============================================================================
// Pith and Substance Doctrine
// ============================================================================

/// Pith and substance analysis for determining constitutional validity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PithAndSubstance {
    /// The dominant characteristic of the law
    pub dominant_characteristic: String,
    /// The matter the law relates to
    pub matter: String,
    /// Federal or provincial head of power
    pub head_of_power: HeadOfPower,
    /// Whether valid under division of powers
    pub is_valid: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Head of power (federal or provincial)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeadOfPower {
    /// Federal power
    Federal(FederalPower),
    /// Provincial power
    Provincial(ProvincialPower),
    /// Both (double aspect doctrine)
    DoubleAspect {
        federal: FederalPower,
        provincial: ProvincialPower,
    },
}

// ============================================================================
// Doctrines
// ============================================================================

/// Constitutional doctrines
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstitutionalDoctrine {
    /// Pith and substance
    PithAndSubstance,
    /// Colourability (disguised attempt to legislate in other jurisdiction)
    Colourability,
    /// Double aspect (both federal and provincial can legislate)
    DoubleAspect,
    /// Paramountcy (federal law prevails in conflict)
    FederalParamountcy,
    /// Interjurisdictional immunity (core federal matters immune from provincial law)
    InterjurisdictionalImmunity,
    /// Ancillary powers (provincial can incidentally affect federal matters)
    AncillaryPowers,
    /// POGG emergency branch
    PoggEmergency,
    /// POGG national concern branch
    PoggNationalConcern,
    /// POGG residual gap branch
    PoggResidual,
    /// Living tree (progressive interpretation)
    LivingTree,
}

// ============================================================================
// Aboriginal Rights
// ============================================================================

/// Aboriginal and Treaty Rights (s.35 Constitution Act, 1982)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AboriginalRight {
    /// Existing aboriginal rights recognized and affirmed
    ExistingAboriginalRights,
    /// Existing treaty rights recognized and affirmed
    TreatyRights,
    /// Aboriginal title (proprietary interest in land)
    AboriginalTitle,
    /// Rights include rights under land claims agreements
    LandClaimsAgreements,
    /// Rights apply to Métis
    MetisRights,
    /// Duty to consult (derived from honour of the Crown)
    DutyToConsult,
}

impl AboriginalRight {
    /// Section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::ExistingAboriginalRights => "35(1)",
            Self::TreatyRights => "35(1)",
            Self::AboriginalTitle => "35(1) (Tsilhqot'in Nation)",
            Self::LandClaimsAgreements => "35(3)",
            Self::MetisRights => "35(2)",
            Self::DutyToConsult => "35(1) (Haida Nation)",
        }
    }
}

// ============================================================================
// Key Cases
// ============================================================================

/// Key constitutional case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalCase {
    /// Citation
    pub citation: CaseCitation,
    /// Charter right or constitutional issue
    pub issue: String,
    /// Key principle established
    pub principle: String,
    /// Whether still good law
    pub still_good_law: bool,
}

impl ConstitutionalCase {
    /// R v Oakes \[1986\] 1 SCR 103 (s.1 test)
    pub fn oakes() -> Self {
        Self {
            citation: CaseCitation::scc(
                "R v Oakes",
                1986,
                103,
                "Established the test for s.1 justification of Charter limitations",
            ),
            issue: "Section 1 - Reasonable limits".to_string(),
            principle: "Four-part test: pressing objective, rational connection, \
                minimal impairment, proportionality"
                .to_string(),
            still_good_law: true,
        }
    }

    /// Doré v Barreau du Québec 2012 SCC 12 (administrative Charter review)
    pub fn dore() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Doré v Barreau du Québec",
                2012,
                12,
                "Charter values apply to administrative decisions through reasonableness",
            ),
            issue: "Administrative law and Charter".to_string(),
            principle: "Administrative decision-makers must proportionately balance \
                Charter values"
                .to_string(),
            still_good_law: true,
        }
    }

    /// Haida Nation v BC \[2004\] 3 SCR 511 (duty to consult)
    pub fn haida_nation() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Haida Nation v British Columbia",
                2004,
                73,
                "Duty to consult arises when Crown has knowledge of potential Aboriginal \
                 right and contemplates conduct that might adversely affect it",
            ),
            issue: "Duty to consult".to_string(),
            principle: "Crown must consult and accommodate Aboriginal peoples when \
                contemplating action that may affect their rights"
                .to_string(),
            still_good_law: true,
        }
    }

    /// Tsilhqot'in Nation v BC \[2014\] 2 SCR 256 (Aboriginal title)
    pub fn tsilhqotin() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Tsilhqot'in Nation v British Columbia",
                2014,
                44,
                "First declaration of Aboriginal title; semi-nomadic use can establish title",
            ),
            issue: "Aboriginal title".to_string(),
            principle: "Aboriginal title proven by sufficient, continuous, and exclusive \
                occupation"
                .to_string(),
            still_good_law: true,
        }
    }

    /// Reference re Secession of Quebec \[1998\] 2 SCR 217
    pub fn secession_reference() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Reference re Secession of Quebec",
                1998,
                793,
                "Unilateral secession not permitted; negotiation required",
            ),
            issue: "Constitutional principles".to_string(),
            principle: "Four underlying principles: federalism, democracy, constitutionalism \
                and rule of law, protection of minorities"
                .to_string(),
            still_good_law: true,
        }
    }

    /// Carter v Canada \[2015\] 1 SCR 331 (medical assistance in dying)
    pub fn carter() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Carter v Canada (Attorney General)",
                2015,
                5,
                "Prohibition on physician-assisted death violates s.7",
            ),
            issue: "Section 7 - Life, liberty, security".to_string(),
            principle: "Competent adults have right to seek medical assistance in dying \
                in certain circumstances"
                .to_string(),
            still_good_law: true,
        }
    }

    /// Bedford v Canada \[2013\] 3 SCR 1101 (prostitution laws)
    pub fn bedford() -> Self {
        Self {
            citation: CaseCitation::scc(
                "Canada (Attorney General) v Bedford",
                2013,
                72,
                "Prostitution laws violated s.7; grossly disproportionate",
            ),
            issue: "Section 7 - Security of the person".to_string(),
            principle: "Laws that prevent sex workers from taking safety precautions \
                violate security of the person"
                .to_string(),
            still_good_law: true,
        }
    }

    /// R v Jordan \[2016\] 1 SCR 631 (trial delay)
    pub fn jordan() -> Self {
        Self {
            citation: CaseCitation::scc(
                "R v Jordan",
                2016,
                27,
                "New framework for s.11(b) delay; presumptive ceilings",
            ),
            issue: "Section 11(b) - Trial within reasonable time".to_string(),
            principle: "Presumptive ceiling of 18 months (provincial) or 30 months (superior)"
                .to_string(),
            still_good_law: true,
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
    fn test_charter_right_section() {
        assert_eq!(CharterRight::LifeLibertySecurityOfPerson.section(), "7");
        assert_eq!(CharterRight::EqualityRights.section(), "15(1)");
    }

    #[test]
    fn test_notwithstanding_clause() {
        assert!(CharterRight::FreedomOfExpression.can_be_overridden());
        assert!(!CharterRight::RightToVote.can_be_overridden());
    }

    #[test]
    fn test_federal_power_section() {
        assert_eq!(FederalPower::CriminalLaw.section(), "91(27)");
        assert_eq!(FederalPower::Banking.section(), "91(15)");
    }

    #[test]
    fn test_provincial_power_section() {
        assert_eq!(ProvincialPower::PropertyAndCivilRights.section(), "92(13)");
        assert_eq!(ProvincialPower::Education.section(), "93");
    }

    #[test]
    fn test_oakes_case() {
        let oakes = ConstitutionalCase::oakes();
        assert_eq!(oakes.citation.year, 1986);
        assert!(oakes.still_good_law);
    }

    #[test]
    fn test_aboriginal_right() {
        assert_eq!(
            AboriginalRight::DutyToConsult.section(),
            "35(1) (Haida Nation)"
        );
    }
}
