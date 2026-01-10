//! Types for German Constitutional Law (Grundgesetz - GG)
//!
//! This module provides type-safe representations of German constitutional law concepts
//! including basic rights (Grundrechte), federal structure, and constitutional review.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Basic Right (Grundrecht) from Articles 1-19 GG
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicRight {
    pub article: BasicRightArticle,
    pub holder: RightHolder,
    pub content: String,
    pub restrictions: Vec<RightsRestriction>,
}

/// Basic Rights Articles (Grundrechte) - Articles 1-19 GG
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BasicRightArticle {
    /// Art. 1 - Human dignity (Menschenwürde)
    HumanDignity,
    /// Art. 2 Para. 1 - General freedom of action (Allgemeine Handlungsfreiheit)
    GeneralFreedomOfAction,
    /// Art. 2 Para. 2 - Right to life and physical integrity (Recht auf Leben)
    RightToLife,
    /// Art. 3 - Equality before the law (Gleichheitssatz)
    Equality,
    /// Art. 4 Para. 1 - Freedom of faith and conscience (Glaubensfreiheit)
    FreedomOfFaith,
    /// Art. 4 Para. 2 - Freedom to profess religious views (Bekenntnisfreiheit)
    FreedomOfReligiousProfession,
    /// Art. 4 Para. 3 - Freedom of conscience (Gewissensfreiheit)
    FreedomOfConscience,
    /// Art. 5 Para. 1 - Freedom of expression (Meinungsfreiheit)
    FreedomOfExpression,
    /// Art. 5 Para. 1 - Freedom of press (Pressefreiheit)
    FreedomOfPress,
    /// Art. 5 Para. 3 - Freedom of art and science (Kunst- und Wissenschaftsfreiheit)
    FreedomOfArtAndScience,
    /// Art. 6 - Marriage and family (Ehe und Familie)
    MarriageAndFamily,
    /// Art. 7 - Education system (Schulwesen)
    EducationSystem,
    /// Art. 8 - Freedom of assembly (Versammlungsfreiheit)
    FreedomOfAssembly,
    /// Art. 9 - Freedom of association (Vereinigungsfreiheit)
    FreedomOfAssociation,
    /// Art. 10 - Secrecy of correspondence (Brief-, Post- und Fernmeldegeheimnis)
    SecrecyOfCorrespondence,
    /// Art. 11 - Freedom of movement (Freizügigkeit)
    FreedomOfMovement,
    /// Art. 12 - Occupational freedom (Berufsfreiheit)
    OccupationalFreedom,
    /// Art. 13 - Inviolability of home (Unverletzlichkeit der Wohnung)
    InviolabilityOfHome,
    /// Art. 14 - Property rights (Eigentumsgarantie)
    PropertyRights,
    /// Art. 16 - Citizenship and extradition (Staatsangehörigkeit, Auslieferung)
    CitizenshipAndExtradition,
    /// Art. 16a - Asylum (Asylrecht)
    Asylum,
    /// Art. 17 - Right to petition (Petitionsrecht)
    RightToPetition,
    /// Art. 19 Para. 4 - Legal recourse (Rechtsweg)
    LegalRecourse,
}

/// Holder of a basic right
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RightHolder {
    pub name: String,
    pub holder_type: RightHolderType,
    pub german_citizen: bool, // Some rights limited to German citizens
}

/// Type of right holder
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RightHolderType {
    /// Natural person (Natürliche Person)
    NaturalPerson,
    /// Legal entity (Juristische Person)
    LegalEntity,
    /// Public authority (Öffentliche Gewalt) - NOT a right holder
    PublicAuthority,
}

/// Rights restriction (Grundrechtseingriff)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RightsRestriction {
    pub restricting_authority: PublicAuthority,
    pub legal_basis: String, // Law authorizing restriction
    pub restriction_type: RestrictionType,
    pub date_of_restriction: NaiveDate,
    pub justification: String,
}

/// Type of restriction on basic rights
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestrictionType {
    /// Complete prohibition (Verbot)
    Prohibition,
    /// Permit requirement (Erlaubnispflicht)
    PermitRequirement,
    /// Content regulation (Inhaltsregelung)
    ContentRegulation,
    /// Procedure regulation (Verfahrensregelung)
    ProcedureRegulation,
}

/// Public authority (Öffentliche Gewalt)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicAuthority {
    pub name: String,
    pub authority_type: AuthorityType,
    pub level: FederalLevel,
}

/// Type of public authority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorityType {
    /// Legislative authority (Gesetzgebung)
    Legislative,
    /// Executive authority (Vollziehende Gewalt)
    Executive,
    /// Judicial authority (Rechtsprechung)
    Judicial,
}

/// Federal level (Staatsebene)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FederalLevel {
    /// Federal (Bund)
    Federal,
    /// State (Land)
    State,
    /// Municipal (Kommune)
    Municipal,
}

/// Proportionality test (Verhältnismäßigkeitsprüfung)
///
/// Three-step test for justifying rights restrictions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProportionalityTest {
    pub restriction: RightsRestriction,
    pub legitimate_purpose: String,
    pub suitable: SuitabilityAssessment,
    pub necessary: NecessityAssessment,
    pub proportionate_stricto_sensu: ProportionalityStrictoSensu,
}

impl ProportionalityTest {
    /// Check if restriction passes all three prongs of proportionality test
    pub fn passes_test(&self) -> bool {
        self.suitable.is_suitable
            && self.necessary.is_necessary
            && self.proportionate_stricto_sensu.is_proportionate
    }
}

/// Suitability (Geeignetheit) - First prong
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuitabilityAssessment {
    pub is_suitable: bool,
    pub reasoning: String,
}

/// Necessity (Erforderlichkeit) - Second prong
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NecessityAssessment {
    pub is_necessary: bool,
    pub alternative_measures: Vec<String>, // Less restrictive alternatives
    pub reasoning: String,
}

/// Proportionality stricto sensu (Angemessenheit) - Third prong
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProportionalityStrictoSensu {
    pub is_proportionate: bool,
    pub public_interest: String,
    pub private_interest: String,
    pub balancing: String, // Balancing of interests (Abwägung)
}

/// Constitutional complaint (Verfassungsbeschwerde) - Art. 93 Para. 1 No. 4a GG
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstitutionalComplaint {
    pub complainant: RightHolder,
    pub violated_right: BasicRightArticle,
    pub infringing_act: InfringingAct,
    pub subsidiarity_met: bool,      // Exhausted other legal remedies
    pub directly_affected: bool,     // Self, current, immediate
    pub filed_within_deadline: bool, // One year or one month
    pub complaint_date: NaiveDate,
}

/// Act allegedly infringing on basic rights
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfringingAct {
    /// Statute (Gesetz)
    Statute { name: String, date: NaiveDate },
    /// Administrative act (Verwaltungsakt)
    AdministrativeAct {
        description: String,
        date: NaiveDate,
    },
    /// Court decision (Gerichtsentscheidung)
    CourtDecision { court: String, date: NaiveDate },
}

/// Federal Constitutional Court decision (BVerfG-Entscheidung)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalCourtDecision {
    pub case_number: String,
    pub decision_date: NaiveDate,
    pub decision_type: DecisionType,
    pub violated_rights: Vec<BasicRightArticle>,
    pub outcome: DecisionOutcome,
}

/// Type of constitutional court decision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionType {
    /// Abstract norm control (Abstrakte Normenkontrolle)
    AbstractNormControl,
    /// Concrete norm control (Konkrete Normenkontrolle)
    ConcreteNormControl,
    /// Constitutional complaint (Verfassungsbeschwerde)
    ConstitutionalComplaint,
    /// Federal dispute (Bund-Länder-Streit)
    FederalDispute,
}

/// Outcome of constitutional court decision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionOutcome {
    /// Law declared unconstitutional and void (Nichtig)
    Unconstitutional,
    /// Law declared incompatible but not void (Unvereinbar)
    Incompatible,
    /// Complaint successful
    Successful,
    /// Complaint unsuccessful
    Unsuccessful,
    /// Inadmissible (Unzulässig)
    Inadmissible,
}

/// Federal structure entities
/// Bundestag (Federal Parliament) - Art. 38-49 GG
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bundestag {
    pub legislative_period: u32, // Wahlperiode (4 years)
    pub members: Vec<BundestagMember>,
    pub president: String, // Bundestagspräsident
}

/// Member of Bundestag (Abgeordneter)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundestagMember {
    pub name: String,
    pub party: Option<String>,
    pub constituency: Option<String>, // Wahlkreis
    pub free_mandate: bool,           // Art. 38 Para. 1 - Representatives of whole people
}

/// Bundesrat (Federal Council) - Art. 50-53 GG
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bundesrat {
    pub state_delegations: Vec<StateDelegation>,
}

/// State delegation to Bundesrat
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateDelegation {
    pub state_name: String, // Land name
    pub votes: u8,          // 3-6 votes depending on population
    pub delegates: Vec<String>,
}

impl StateDelegation {
    /// Determine number of Bundesrat votes based on population (Art. 51 Para. 2 GG)
    pub fn votes_for_population(population: u64) -> u8 {
        if population < 2_000_000 {
            3
        } else if population < 6_000_000 {
            4
        } else if population < 7_000_000 {
            5
        } else {
            6
        }
    }
}

/// Federal President (Bundespräsident) - Art. 54-61 GG
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederalPresident {
    pub name: String,
    pub term_start: NaiveDate,
    pub term_number: u8, // Max 2 consecutive terms (5 years each)
}

/// Federal Government (Bundesregierung) - Art. 62-69 GG
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederalGovernment {
    pub chancellor: FederalChancellor,
    pub ministers: Vec<FederalMinister>,
}

/// Federal Chancellor (Bundeskanzler) - Art. 63-67 GG
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederalChancellor {
    pub name: String,
    pub elected_date: NaiveDate,
    pub policy_guidelines: bool, // Art. 65 - Richtlinienkompetenz
}

/// Federal Minister (Bundesminister)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederalMinister {
    pub name: String,
    pub ministry: String,
    pub departmental_autonomy: bool, // Art. 65 - Ressortprinzip
}

/// Legislative competence (Gesetzgebungskompetenz)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegislativeCompetence {
    /// Exclusive federal competence (Ausschließliche Gesetzgebung) - Art. 71-73 GG
    ExclusiveFederal,
    /// Concurrent competence (Konkurrierende Gesetzgebung) - Art. 72, 74 GG
    Concurrent,
    /// Framework competence (Rahmengesetzgebung) - Art. 75 GG (repealed 2006)
    Framework,
    /// State competence (Ländergesetzgebung) - Art. 70 GG
    State,
}

/// Subject matter for legislative competence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubjectMatter {
    pub description: String,
    pub competence_type: LegislativeCompetence,
    pub article_reference: String,
}
