//! Constitution of the Lao People's Democratic Republic
//! ລັດຖະທຳມະນູນ ແຫ່ງ ສາທາລະນະລັດ ປະຊາທິປະໄຕ ປະຊາຊົນລາວ
//!
//! This module implements the Constitution of Lao PDR (1991, amended 2003 and 2015),
//! providing type-safe representations of:
//! - Political regime and state structure
//! - Fundamental rights and duties of citizens
//! - State organs (National Assembly, President, Government)
//! - Judicial system (Courts and Prosecutors)
//! - Local administration
//! - Constitutional amendment procedures
//!
//! ## Historical Context
//!
//! The Constitution of the Lao People's Democratic Republic was first adopted on
//! August 14, 1991, following the establishment of the Lao PDR in 1975. It has been
//! amended twice:
//! - **2003 Amendment**: Strengthened private property rights and market economy provisions
//! - **2015 Amendment**: Enhanced human rights protections and modernized state structure
//!
//! The Constitution establishes Lao PDR as a people's democratic state under the leadership
//! of the Lao People's Revolutionary Party, with a socialist orientation combined with a
//! market economy.
//!
//! ## Structure
//!
//! The Constitution consists of 11 chapters and 108 articles:
//!
//! ### Chapter I: Political Regime (Articles 1-4)
//! Establishes Lao PDR as a people's democratic state with multi-ethnic sovereignty
//! and a socialist market economy.
//!
//! ### Chapter II: Socio-Economic System (Articles 5-33)
//! Defines the economic system, property rights, and development principles.
//!
//! ### Chapter III: Rights and Obligations of Citizens (Articles 34-51)
//! Guarantees fundamental rights including:
//! - Equality before the law (Article 34)
//! - Voting rights for citizens 18+ (Article 35)
//! - Freedom of expression, religion, privacy (Articles 36-40)
//! - Right to property, work, education, healthcare (Articles 41-45)
//! - Gender equality and family protection (Articles 46-48)
//! - Duties to defend nation, pay taxes, protect environment (Articles 49-51)
//!
//! ### Chapter IV: National Assembly (Articles 52-65)
//! The legislative body with powers to adopt/amend Constitution, enact laws,
//! elect President, approve Government, and supervise state administration.
//!
//! ### Chapter V: President of the State (Articles 66-70)
//! Head of state representing Lao PDR, commanding armed forces, and promulgating laws.
//!
//! ### Chapter VI: Government (Articles 71-80)
//! Executive body implementing laws and managing national development.
//!
//! ### Chapter VII: Local Administration (Articles 81-87)
//! Provincial, district, and village administration structure.
//!
//! ### Chapter VIII: People's Courts (Articles 88-95)
//! Independent judicial system with Supreme, Appeal, Provincial, and District courts.
//!
//! ### Chapter IX: People's Prosecutors (Articles 96-100)
//! Prosecutorial system supervising investigation and legal proceedings.
//!
//! ### Chapter X: State of War and Emergency (Articles 101-104)
//! Presidential powers during exceptional circumstances.
//!
//! ### Chapter XI: Amendment of the Constitution (Articles 105-108)
//! Procedures requiring 2/3 National Assembly majority.
//!
//! ## Examples
//!
//! ### Validating Voting Rights
//!
//! ```
//! use legalis_la::constitution::validator::validate_voting_rights;
//!
//! // Article 35: Citizens 18 years or older have the right to vote
//! assert!(validate_voting_rights(18).is_ok());
//! assert!(validate_voting_rights(25).is_ok());
//! assert!(validate_voting_rights(17).is_err());
//! ```
//!
//! ### Creating a National Assembly
//!
//! ```
//! use legalis_la::constitution::types::{
//!     ElectionMethod, NationalAssembly, NationalAssemblyPower,
//! };
//!
//! // 9th National Assembly (2021-2026): 164 members, 5-year term
//! let na = NationalAssembly::builder()
//!     .session(9)
//!     .members(164)
//!     .term_years(5)
//!     .election_method(ElectionMethod::default())
//!     .power(NationalAssemblyPower::EnactLaws)
//!     .power(NationalAssemblyPower::ApproveBudget)
//!     .build()
//!     .unwrap();
//!
//! use legalis_la::constitution::validator::validate_national_assembly;
//! assert!(validate_national_assembly(&na).is_ok());
//! ```
//!
//! ### Checking Fundamental Rights
//!
//! ```
//! use legalis_la::constitution::types::{
//!     FundamentalRight, LegitimateAim, RightsLimitation,
//! };
//! use legalis_la::constitution::validator::validate_rights_limitation;
//!
//! // Example: Limiting freedom of expression for public order
//! let limitation = RightsLimitation {
//!     right: FundamentalRight::FreedomOfExpression,
//!     legitimate_aim: LegitimateAim::PublicOrder,
//!     is_necessary: true,
//!     is_proportional: true,
//!     legal_basis: "Public Order Law 2020, Article 15".to_string(),
//! };
//!
//! // Limitation passes proportionality test
//! assert!(validate_rights_limitation(&limitation).is_ok());
//! ```
//!
//! ### Validating Court Independence
//!
//! ```
//! use legalis_la::constitution::types::{
//!     CourtLevel, CourtPower, Judge, PeoplesCourt,
//! };
//! use legalis_la::constitution::validator::validate_court_organization;
//!
//! // Article 88: Courts are independent
//! let supreme_court = PeoplesCourt {
//!     level: CourtLevel::Supreme,
//!     is_independent: true,
//!     judges: vec![
//!         Judge {
//!             name: "Chief Justice".to_string(),
//!             court_level: CourtLevel::Supreme,
//!             independent_judgment: true,
//!         }
//!     ],
//!     powers: vec![
//!         CourtPower::AdjudicateCriminal,
//!         CourtPower::AdjudicateCivil,
//!         CourtPower::AdjudicateAdministrative,
//!     ],
//! };
//!
//! assert!(validate_court_organization(&supreme_court).is_ok());
//! ```
//!
//! ### Constitutional Amendment Procedure
//!
//! ```
//! use legalis_la::constitution::types::{
//!     AmendmentProposer, ConstitutionalAmendment,
//! };
//! use legalis_la::constitution::validator::validate_constitutional_amendment;
//!
//! // Article 106: Amendment requires 2/3 majority of NA members
//! let total_members = 164u32;
//! let required_votes = (total_members * 2).div_ceil(3); // 110 votes
//!
//! let amendment = ConstitutionalAmendment {
//!     proposed_by: AmendmentProposer::President,
//!     proposed_changes_lao: "ແກ້ໄຂມາດຕາ 5: ເພີ່ມເຕີມກ່ຽວກັບສິດຊັບສິນ".to_string(),
//!     proposed_changes_english: "Amend Article 5: Enhance property rights".to_string(),
//!     required_votes,
//!     votes_received: Some(120),
//!     approved: true,
//!     amendment_date: None,
//! };
//!
//! assert!(validate_constitutional_amendment(&amendment, total_members).is_ok());
//! ```
//!
//! ## Legal References
//!
//! - Constitution of the Lao People's Democratic Republic (1991)
//! - Constitutional Amendment (2003)
//! - Constitutional Amendment (2015)
//! - National Assembly Law
//! - People's Court Law
//! - People's Prosecutor Law
//!
//! ## Bilingual Support
//!
//! All types and error messages support both Lao (ພາສາລາວ) and English languages,
//! reflecting the bilingual nature of legal practice in Lao PDR.

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use error::{ConstitutionalError, ConstitutionalResult, LimitationFailure};
pub use types::{
    AdministrativeAuthority, AdministrativeLevel, AmendmentProposer, ConstitutionalAmendment,
    CourtLevel, CourtPower, EconomicSystem, ElectedBy, ElectionMethod, FundamentalDuty,
    FundamentalRight, Government, GovernmentPower, Judge, LegitimateAim, LocalAdministration,
    LocalPower, Minister, NationalAssembly, NationalAssemblyPower, PeoplesCouncil, PeoplesCourt,
    PeoplesProsecutor, PoliticalRegime, President, PresidentialPower, ProsecutorLevel,
    ProsecutorPower, RightsLimitation, Sovereignty, StandingCommittee, StandingCommitteePower,
    StateForm, StateOrgan,
};
pub use validator::{
    validate_constitutional_amendment, validate_court_organization, validate_fundamental_right,
    validate_government, validate_local_administration, validate_na_candidacy,
    validate_national_assembly, validate_president, validate_prosecutor_organization,
    validate_rights_limitation, validate_state_structure, validate_voting_rights,
};
