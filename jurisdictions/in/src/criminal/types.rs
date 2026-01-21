//! Bharatiya Nyaya Sanhita (BNS) 2023 Types
//!
//! Types for India's new criminal code that replaced the Indian Penal Code (IPC) 1860.
//! BNS came into effect on 1st July 2024.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Offence category under BNS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OffenceCategory {
    /// Offences against the State (Chapter VII)
    AgainstState,
    /// Offences relating to Army, Navy, Air Force (Chapter VIII)
    AgainstArmedForces,
    /// Offences relating to elections (Chapter IX)
    Elections,
    /// Offences relating to public tranquility (Chapter X)
    PublicTranquility,
    /// Offences by public servants (Chapter XI)
    PublicServants,
    /// Offences relating to contempt of lawful authority (Chapter XII)
    ContemptOfAuthority,
    /// Offences affecting public health, safety, convenience (Chapter XIV)
    PublicHealthSafety,
    /// Offences relating to religion (Chapter XV)
    Religion,
    /// Offences affecting human body (Chapter VI)
    HumanBody,
    /// Offences against property (Chapter XVII)
    Property,
    /// Offences relating to documents (Chapter XVIII)
    Documents,
    /// Offences relating to currency and stamps (Chapter XIX)
    CurrencyStamps,
    /// Sexual offences (Chapter V)
    Sexual,
    /// Offences relating to marriage (Chapter XX)
    Marriage,
    /// Defamation (Chapter XXI)
    Defamation,
    /// Criminal intimidation (Chapter XXII)
    Intimidation,
    /// Organized crime (Chapter IV)
    OrganizedCrime,
    /// Terrorism (Chapter IV)
    Terrorism,
}

impl OffenceCategory {
    /// Get BNS chapter reference
    pub fn chapter(&self) -> &'static str {
        match self {
            Self::AgainstState => "Chapter VII",
            Self::AgainstArmedForces => "Chapter VIII",
            Self::Elections => "Chapter IX",
            Self::PublicTranquility => "Chapter X",
            Self::PublicServants => "Chapter XI",
            Self::ContemptOfAuthority => "Chapter XII",
            Self::PublicHealthSafety => "Chapter XIV",
            Self::Religion => "Chapter XV",
            Self::HumanBody => "Chapter VI",
            Self::Property => "Chapter XVII",
            Self::Documents => "Chapter XVIII",
            Self::CurrencyStamps => "Chapter XIX",
            Self::Sexual => "Chapter V",
            Self::Marriage => "Chapter XX",
            Self::Defamation => "Chapter XXI",
            Self::Intimidation => "Chapter XXII",
            Self::OrganizedCrime => "Chapter IV",
            Self::Terrorism => "Chapter IV",
        }
    }
}

/// Specific offence type under BNS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Offence {
    // Chapter IV - Organized Crime
    /// Organized crime (Section 111)
    OrganizedCrime,
    /// Petty organized crime (Section 112)
    PettyOrganizedCrime,
    /// Terrorist act (Section 113)
    TerroristAct,

    // Chapter V - Sexual Offences
    /// Rape (Section 63)
    Rape,
    /// Gang rape (Section 70)
    GangRape,
    /// Sexual harassment (Section 75)
    SexualHarassment,
    /// Voyeurism (Section 77)
    Voyeurism,
    /// Stalking (Section 78)
    Stalking,

    // Chapter VI - Offences affecting human body
    /// Murder (Section 101)
    Murder,
    /// Culpable homicide (Section 100)
    CulpableHomicide,
    /// Causing death by negligence (Section 106)
    DeathByNegligence,
    /// Abetment of suicide (Section 107)
    AbetmentOfSuicide,
    /// Attempt to murder (Section 109)
    AttemptToMurder,
    /// Grievous hurt (Section 117)
    GrievousHurt,
    /// Voluntarily causing hurt (Section 115)
    VoluntaryHurt,
    /// Wrongful restraint (Section 126)
    WrongfulRestraint,
    /// Wrongful confinement (Section 127)
    WrongfulConfinement,
    /// Kidnapping (Section 137)
    Kidnapping,
    /// Abduction (Section 137)
    Abduction,
    /// Human trafficking (Section 143)
    HumanTrafficking,

    // Chapter VII - Offences against State
    /// Waging war against India (Section 147)
    WagingWar,
    /// Sedition (Section 150) - repealed/modified
    Sedition,
    /// Acts endangering sovereignty (Section 152)
    EndangeringSovereignty,

    // Chapter X - Public tranquility
    /// Rioting (Section 189)
    Rioting,
    /// Unlawful assembly (Section 187)
    UnlawfulAssembly,
    /// Affray (Section 195)
    Affray,

    // Chapter XVII - Property offences
    /// Theft (Section 303)
    Theft,
    /// Extortion (Section 308)
    Extortion,
    /// Robbery (Section 309)
    Robbery,
    /// Dacoity (Section 310)
    Dacoity,
    /// Criminal breach of trust (Section 316)
    CriminalBreachOfTrust,
    /// Cheating (Section 318)
    Cheating,
    /// Fraud (Section 318)
    Fraud,
    /// Mischief (Section 324)
    Mischief,
    /// Criminal trespass (Section 329)
    CriminalTrespass,

    // Chapter XVIII - Documents
    /// Forgery (Section 336)
    Forgery,
    /// Counterfeiting (Section 337)
    Counterfeiting,

    // Chapter XXI - Defamation
    /// Defamation (Section 356)
    Defamation,

    // Chapter XXII - Intimidation
    /// Criminal intimidation (Section 351)
    CriminalIntimidation,

    // New offences in BNS
    /// Hit and run (Section 106(2))
    HitAndRun,
    /// Mob lynching (Section 103(2))
    MobLynching,
    /// Snatching (Section 304)
    Snatching,
}

impl Offence {
    /// Get BNS section number
    pub fn section(&self) -> u32 {
        match self {
            Self::OrganizedCrime => 111,
            Self::PettyOrganizedCrime => 112,
            Self::TerroristAct => 113,
            Self::Rape => 63,
            Self::GangRape => 70,
            Self::SexualHarassment => 75,
            Self::Voyeurism => 77,
            Self::Stalking => 78,
            Self::Murder => 101,
            Self::CulpableHomicide => 100,
            Self::DeathByNegligence => 106,
            Self::AbetmentOfSuicide => 107,
            Self::AttemptToMurder => 109,
            Self::GrievousHurt => 117,
            Self::VoluntaryHurt => 115,
            Self::WrongfulRestraint => 126,
            Self::WrongfulConfinement => 127,
            Self::Kidnapping | Self::Abduction => 137,
            Self::HumanTrafficking => 143,
            Self::WagingWar => 147,
            Self::Sedition => 150,
            Self::EndangeringSovereignty => 152,
            Self::Rioting => 189,
            Self::UnlawfulAssembly => 187,
            Self::Affray => 195,
            Self::Theft => 303,
            Self::Extortion => 308,
            Self::Robbery => 309,
            Self::Dacoity => 310,
            Self::CriminalBreachOfTrust => 316,
            Self::Cheating | Self::Fraud => 318,
            Self::Mischief => 324,
            Self::CriminalTrespass => 329,
            Self::Forgery => 336,
            Self::Counterfeiting => 337,
            Self::Defamation => 356,
            Self::CriminalIntimidation => 351,
            Self::HitAndRun => 106,
            Self::MobLynching => 103,
            Self::Snatching => 304,
        }
    }

    /// Get corresponding IPC section (for reference)
    pub fn ipc_equivalent(&self) -> Option<u32> {
        match self {
            Self::Murder => Some(302),
            Self::CulpableHomicide => Some(299),
            Self::Rape => Some(376),
            Self::Theft => Some(378),
            Self::Robbery => Some(392),
            Self::Dacoity => Some(391),
            Self::Cheating => Some(420),
            Self::CriminalBreachOfTrust => Some(405),
            Self::Forgery => Some(463),
            Self::Defamation => Some(499),
            Self::Kidnapping => Some(359),
            Self::GrievousHurt => Some(320),
            Self::CriminalTrespass => Some(441),
            Self::Rioting => Some(146),
            _ => None, // New offences in BNS
        }
    }

    /// Check if offence is cognizable
    pub fn is_cognizable(&self) -> bool {
        matches!(
            self,
            Self::Murder
                | Self::Rape
                | Self::GangRape
                | Self::Robbery
                | Self::Dacoity
                | Self::Kidnapping
                | Self::HumanTrafficking
                | Self::TerroristAct
                | Self::OrganizedCrime
                | Self::MobLynching
                | Self::AttemptToMurder
                | Self::GrievousHurt
                | Self::WagingWar
        )
    }

    /// Check if offence is bailable
    pub fn is_bailable(&self) -> bool {
        !matches!(
            self,
            Self::Murder
                | Self::Rape
                | Self::GangRape
                | Self::Robbery
                | Self::Dacoity
                | Self::Kidnapping
                | Self::HumanTrafficking
                | Self::TerroristAct
                | Self::OrganizedCrime
                | Self::MobLynching
                | Self::AttemptToMurder
                | Self::WagingWar
        )
    }

    /// Check if offence is compoundable
    pub fn is_compoundable(&self) -> bool {
        matches!(
            self,
            Self::VoluntaryHurt
                | Self::Defamation
                | Self::CriminalTrespass
                | Self::Mischief
                | Self::SexualHarassment
        )
    }
}

/// Punishment type under BNS
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Punishment {
    /// Imprisonment type
    pub imprisonment: Option<ImprisonmentType>,
    /// Minimum imprisonment (years)
    pub min_years: Option<u32>,
    /// Maximum imprisonment (years)
    pub max_years: Option<u32>,
    /// Life imprisonment
    pub life_imprisonment: bool,
    /// Death penalty
    pub death_penalty: bool,
    /// Fine amount (if specified)
    pub fine: Option<f64>,
    /// Fine type
    pub fine_type: FineType,
    /// Community service (new in BNS)
    pub community_service: Option<u32>,
}

impl Punishment {
    /// Create punishment for murder (Section 101)
    pub fn for_murder() -> Self {
        Self {
            imprisonment: Some(ImprisonmentType::Rigorous),
            min_years: None,
            max_years: None,
            life_imprisonment: true,
            death_penalty: true,
            fine: None,
            fine_type: FineType::Discretionary,
            community_service: None,
        }
    }

    /// Create punishment for theft (Section 303)
    pub fn for_theft() -> Self {
        Self {
            imprisonment: Some(ImprisonmentType::Simple),
            min_years: None,
            max_years: Some(3),
            life_imprisonment: false,
            death_penalty: false,
            fine: None,
            fine_type: FineType::Discretionary,
            community_service: None,
        }
    }

    /// Create punishment for cheating (Section 318)
    pub fn for_cheating() -> Self {
        Self {
            imprisonment: Some(ImprisonmentType::Either),
            min_years: None,
            max_years: Some(7),
            life_imprisonment: false,
            death_penalty: false,
            fine: None,
            fine_type: FineType::Discretionary,
            community_service: None,
        }
    }

    /// Create punishment for organized crime (Section 111)
    pub fn for_organized_crime() -> Self {
        Self {
            imprisonment: Some(ImprisonmentType::Rigorous),
            min_years: Some(5),
            max_years: None,
            life_imprisonment: true,
            death_penalty: true,
            fine: Some(5_000_000.0),
            fine_type: FineType::Minimum,
            community_service: None,
        }
    }
}

/// Imprisonment type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImprisonmentType {
    /// Rigorous imprisonment
    Rigorous,
    /// Simple imprisonment
    Simple,
    /// Either rigorous or simple
    Either,
}

/// Fine type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FineType {
    /// Fixed fine amount
    Fixed,
    /// Minimum fine
    Minimum,
    /// Maximum fine
    Maximum,
    /// Court's discretion
    Discretionary,
    /// In lieu of imprisonment
    InLieuOfImprisonment,
    /// No fine
    None,
}

/// Criminal case under BNS
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CriminalCase {
    /// Case number
    pub case_number: String,
    /// Offence charged
    pub offence: Offence,
    /// Sections charged under
    pub sections: Vec<u32>,
    /// Date of offence
    pub offence_date: NaiveDate,
    /// FIR number
    pub fir_number: Option<String>,
    /// FIR date
    pub fir_date: Option<NaiveDate>,
    /// Investigation status
    pub investigation_status: InvestigationStatus,
    /// Accused persons
    pub accused: Vec<Accused>,
    /// Complainant
    pub complainant: Option<String>,
    /// Court
    pub court: Court,
    /// Case status
    pub status: CaseStatus,
}

/// Investigation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvestigationStatus {
    /// Investigation pending
    Pending,
    /// Investigation ongoing
    Ongoing,
    /// Chargesheet filed
    ChargesheetFiled,
    /// Closure report filed
    ClosureReport,
    /// Final report filed
    FinalReport,
}

/// Accused person details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Accused {
    /// Name
    pub name: String,
    /// Age
    pub age: Option<u32>,
    /// Address
    pub address: Option<String>,
    /// Arrested
    pub arrested: bool,
    /// Arrest date
    pub arrest_date: Option<NaiveDate>,
    /// Bail status
    pub bail_status: BailStatus,
    /// Previous convictions
    pub previous_convictions: u32,
}

/// Bail status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BailStatus {
    /// Not arrested
    NotArrested,
    /// In custody
    InCustody,
    /// On bail
    OnBail,
    /// Absconding
    Absconding,
    /// Surrendered
    Surrendered,
}

/// Court hierarchy for criminal matters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Court {
    /// Supreme Court of India
    SupremeCourt,
    /// High Court
    HighCourt,
    /// Sessions Court
    SessionsCourt,
    /// Chief Judicial Magistrate
    ChiefJudicialMagistrate,
    /// Judicial Magistrate First Class
    JudicialMagistrateFirstClass,
    /// Judicial Magistrate Second Class
    JudicialMagistrateSecondClass,
    /// Metropolitan Magistrate
    MetropolitanMagistrate,
    /// Special Court
    SpecialCourt,
    /// Fast Track Court
    FastTrackCourt,
}

impl Court {
    /// Get maximum imprisonment power
    pub fn max_imprisonment_years(&self) -> Option<u32> {
        match self {
            Self::JudicialMagistrateSecondClass => Some(1),
            Self::JudicialMagistrateFirstClass | Self::MetropolitanMagistrate => Some(3),
            Self::ChiefJudicialMagistrate => Some(7),
            Self::SessionsCourt
            | Self::HighCourt
            | Self::SupremeCourt
            | Self::SpecialCourt
            | Self::FastTrackCourt => None, // No upper limit
        }
    }

    /// Get maximum fine power (Rs.)
    pub fn max_fine(&self) -> Option<f64> {
        match self {
            Self::JudicialMagistrateSecondClass => Some(5_000.0),
            Self::JudicialMagistrateFirstClass | Self::MetropolitanMagistrate => Some(10_000.0),
            Self::ChiefJudicialMagistrate => None,
            Self::SessionsCourt
            | Self::HighCourt
            | Self::SupremeCourt
            | Self::SpecialCourt
            | Self::FastTrackCourt => None,
        }
    }
}

/// Case status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CaseStatus {
    /// Under investigation
    UnderInvestigation,
    /// Pending trial
    PendingTrial,
    /// Under trial
    UnderTrial,
    /// Reserved for judgment
    Reserved,
    /// Convicted
    Convicted,
    /// Acquitted
    Acquitted,
    /// Discharged
    Discharged,
    /// Compounded
    Compounded,
    /// Withdrawn
    Withdrawn,
    /// Abated
    Abated,
}

/// Limitation period for offences
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LimitationPeriod {
    /// No limitation (for offences punishable with death/life imprisonment)
    NoLimit,
    /// 3 years
    ThreeYears,
    /// 1 year
    OneYear,
    /// 6 months (for petty offences)
    SixMonths,
}

impl LimitationPeriod {
    /// Get period in months
    pub fn months(&self) -> Option<u32> {
        match self {
            Self::NoLimit => None,
            Self::ThreeYears => Some(36),
            Self::OneYear => Some(12),
            Self::SixMonths => Some(6),
        }
    }
}

/// Plea bargaining eligibility (Chapter XXIA, BNSS)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PleaBargaining {
    /// Offence
    pub offence: Offence,
    /// Eligible for plea bargaining
    pub eligible: bool,
    /// Maximum sentence reduction
    pub max_reduction: Option<f64>,
    /// Reason if not eligible
    pub ineligibility_reason: Option<String>,
}

impl PleaBargaining {
    /// Check if offence is eligible for plea bargaining
    pub fn check_eligibility(offence: &Offence) -> Self {
        // Plea bargaining not allowed for offences:
        // - Affecting socio-economic conditions
        // - Committed against women/children
        // - Punishable with death/life imprisonment
        // - Punishable with > 7 years imprisonment

        let (eligible, reason) = match offence {
            Offence::Murder
            | Offence::Rape
            | Offence::GangRape
            | Offence::TerroristAct
            | Offence::OrganizedCrime
            | Offence::WagingWar
            | Offence::HumanTrafficking
            | Offence::MobLynching => (
                false,
                Some("Punishable with death/life imprisonment".to_string()),
            ),

            Offence::SexualHarassment | Offence::Stalking | Offence::Voyeurism => {
                (false, Some("Offence against women".to_string()))
            }

            Offence::Kidnapping | Offence::Abduction => {
                (false, Some("May involve minors".to_string()))
            }

            _ => (true, None),
        };

        Self {
            offence: *offence,
            eligible,
            max_reduction: if eligible { Some(0.25) } else { None }, // Up to 25% reduction
            ineligibility_reason: reason,
        }
    }
}

/// Community service provisions (new in BNS)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunityService {
    /// Hours of service
    pub hours: u32,
    /// Nature of work
    pub work_type: CommunityWorkType,
    /// Completion deadline
    pub deadline: NaiveDate,
    /// Supervising authority
    pub supervisor: String,
}

/// Community service work type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommunityWorkType {
    /// Public cleanliness
    PublicCleanliness,
    /// Healthcare support
    Healthcare,
    /// Education support
    Education,
    /// Traffic management
    Traffic,
    /// Environmental protection
    Environment,
    /// Other public service
    Other,
}

/// Zero FIR concept (any police station can register FIR)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZeroFir {
    /// FIR number
    pub fir_number: String,
    /// Police station that registered
    pub registering_station: String,
    /// Actual jurisdiction station
    pub jurisdiction_station: String,
    /// Date of registration
    pub registration_date: NaiveDate,
    /// Transfer date to jurisdiction
    pub transfer_date: Option<NaiveDate>,
}

/// E-FIR (electronic FIR)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EFir {
    /// FIR number
    pub fir_number: String,
    /// Complainant details
    pub complainant: String,
    /// Offence reported
    pub offence: Vec<Offence>,
    /// Date of electronic filing
    pub filing_date: NaiveDate,
    /// Verification status
    pub verified: bool,
    /// Physical copy received
    pub physical_copy_received: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offence_sections() {
        assert_eq!(Offence::Murder.section(), 101);
        assert_eq!(Offence::Rape.section(), 63);
        assert_eq!(Offence::Theft.section(), 303);
    }

    #[test]
    fn test_ipc_equivalents() {
        assert_eq!(Offence::Murder.ipc_equivalent(), Some(302));
        assert_eq!(Offence::Rape.ipc_equivalent(), Some(376));
        assert_eq!(Offence::Cheating.ipc_equivalent(), Some(420));
    }

    #[test]
    fn test_cognizable_offences() {
        assert!(Offence::Murder.is_cognizable());
        assert!(Offence::Rape.is_cognizable());
        assert!(!Offence::Defamation.is_cognizable());
    }

    #[test]
    fn test_bailable_offences() {
        assert!(!Offence::Murder.is_bailable());
        assert!(!Offence::Rape.is_bailable());
        assert!(Offence::Theft.is_bailable());
        assert!(Offence::Defamation.is_bailable());
    }

    #[test]
    fn test_compoundable_offences() {
        assert!(Offence::Defamation.is_compoundable());
        assert!(Offence::CriminalTrespass.is_compoundable());
        assert!(!Offence::Murder.is_compoundable());
    }

    #[test]
    fn test_punishment_murder() {
        let punishment = Punishment::for_murder();
        assert!(punishment.death_penalty);
        assert!(punishment.life_imprisonment);
    }

    #[test]
    fn test_court_powers() {
        assert_eq!(
            Court::JudicialMagistrateSecondClass.max_imprisonment_years(),
            Some(1)
        );
        assert_eq!(
            Court::JudicialMagistrateFirstClass.max_imprisonment_years(),
            Some(3)
        );
        assert_eq!(Court::SessionsCourt.max_imprisonment_years(), None);
    }

    #[test]
    fn test_plea_bargaining_eligibility() {
        let murder = PleaBargaining::check_eligibility(&Offence::Murder);
        assert!(!murder.eligible);

        let theft = PleaBargaining::check_eligibility(&Offence::Theft);
        assert!(theft.eligible);
        assert_eq!(theft.max_reduction, Some(0.25));
    }

    #[test]
    fn test_limitation_periods() {
        assert_eq!(LimitationPeriod::ThreeYears.months(), Some(36));
        assert_eq!(LimitationPeriod::NoLimit.months(), None);
    }

    #[test]
    fn test_offence_category_chapters() {
        assert_eq!(OffenceCategory::Property.chapter(), "Chapter XVII");
        assert_eq!(OffenceCategory::Sexual.chapter(), "Chapter V");
    }
}
