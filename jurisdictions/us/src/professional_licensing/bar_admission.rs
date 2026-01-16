//! Attorney Bar Admission and Licensing
//!
//! This module handles attorney licensing requirements across US jurisdictions,
//! focusing on the Uniform Bar Examination (UBE) system, interstate practice rules,
//! and state-specific admission pathways.
//!
//! # Overview
//!
//! Attorney licensing in the United States is highly decentralized, with each of
//! the 50 states, DC, and US territories maintaining independent licensing systems
//! through their respective state supreme courts and bar admission authorities.
//! This creates significant complexity for attorneys seeking to practice across
//! state lines.
//!
//! ## Historical Context
//!
//! Before the UBE, every jurisdiction had its own bar examination with unique
//! content, format, and scoring. This created barriers to interstate mobility,
//! requiring attorneys to take multiple state-specific bar exams to practice in
//! different jurisdictions. The only exceptions were:
//! - **Diploma Privilege**: Wisconsin (for UW-Madison JD graduates) and New Hampshire
//!   (formerly, ended 2024 for in-state law school graduates)
//! - **Motion Admission**: Limited reciprocity based on years of practice (5-10 years)
//!
//! # Uniform Bar Examination (UBE)
//!
//! The UBE is a standardized bar examination developed by the National Conference
//! of Bar Examiners (NCBE) and first offered in 2011. It represents the most
//! significant reform in bar admissions since the creation of the MBE in 1972.
//!
//! ## Structure and Components
//!
//! The UBE consists of three components administered over two days:
//!
//! **Day 1 (Tuesday):**
//! - **Multistate Essay Examination (MEE)**: 6 essays, 30% of total score
//!   - Covers 13 subjects: Agency, Business Associations, Civil Procedure, Conflict
//!     of Laws, Constitutional Law, Contracts, Criminal Law/Procedure, Evidence,
//!     Family Law, Real Property, Secured Transactions, Torts, Trusts/Estates
//!   - 30 minutes per essay
//!   - Scaled score: 0-200 points (30% weight = 0-120 in final score)
//!
//! - **Multistate Performance Test (MPT)**: 2 performance tasks, 20% of total score
//!   - Tests practical lawyering skills (drafting, analysis, client communication)
//!   - 90 minutes per task
//!   - Scaled score: 0-200 points (20% weight = 0-80 in final score)
//!
//! **Day 2 (Wednesday):**
//! - **Multistate Bar Examination (MBE)**: 200 multiple-choice questions, 50% of score
//!   - Covers 7 subjects: Civil Procedure, Constitutional Law, Contracts, Criminal
//!     Law/Procedure, Evidence, Real Property, Torts
//!   - Administered in two 100-question sessions (AM and PM)
//!   - Scaled score: 0-200 points (50% weight = 0-200 in final score)
//!
//! **Total UBE Score**: 0-400 points (sum of weighted components)
//!
//! ## Minimum Passing Scores
//!
//! UBE jurisdictions set their own minimum passing scores, typically ranging from
//! 260-280 (out of 400):
//! - **Highest**: Alaska (280)
//! - **Most Common**: 266-270 (33 jurisdictions)
//! - **Lowest**: Alabama, Minnesota, Missouri, New Mexico, North Dakota, South Dakota (260)
//!
//! ## UBE Adoption Timeline
//!
//! **First Wave (2011-2013)**: 9 jurisdictions
//! - 2011: Alabama, Missouri, Minnesota, North Dakota, New Hampshire (first 5 states)
//! - 2011: Colorado, Montana, Idaho, New Mexico (second half)
//! - 2012: Vermont
//! - 2013: Washington, Wyoming
//!
//! **Second Wave (2014-2017)**: 14 jurisdictions
//! - Major adopters: Connecticut (2014), Utah (2015), New York (2016), Illinois (2017)
//! - New York's 2016 adoption was particularly significant due to the state's large
//!   legal market and influential position
//!
//! **Third Wave (2018-2022)**: 14 jurisdictions
//! - Includes major markets: Texas (2021), Pennsylvania (2022), Oregon (2022)
//! - Southern states: Arkansas, West Virginia, Tennessee, South Carolina
//!
//! **Recent Adopters (2023-2024)**: 3 jurisdictions
//! - Ohio (2020), Maryland (2020), Virginia (2024)
//!
//! **Total**: 40 UBE jurisdictions as of 2024
//!
//! ## Notable Non-UBE States
//!
//! ### California
//! Largest legal market in the US, maintains its own California Bar Examination.
//! Known for:
//! - Being one of the most difficult bar exams (historically ~40-50% pass rate)
//! - Accepting graduates from California State Bar-accredited law schools (not just ABA)
//! - Extensive state-specific content (e.g., California Civil Procedure)
//!
//! ### Louisiana
//! Only state with a Civil Law system (inherited from French/Spanish colonial period).
//! The Louisiana Bar Examination tests both:
//! - Common law (for areas like constitutional law, contracts)
//! - Louisiana Civil Code (for areas like property, obligations, successions)
//!
//! ### Florida
//! Second-largest legal market without UBE. Known for:
//! - Public policy questions (unique essay component)
//! - High volume of bar applicants due to snowbird retirement and large law schools
//! - Strong resistance to UBE adoption despite neighboring states adopting
//!
//! ### Nevada and Delaware
//! Smaller states that maintain their own exams despite pressure to adopt UBE.
//!
//! # Score Portability and Transfer
//!
//! The primary benefit of the UBE is **score portability**: examinees can take the
//! UBE in any jurisdiction and later apply their score to other UBE jurisdictions.
//!
//! ## Transfer Rules
//! - **Time Limit**: Most jurisdictions require scores be transferred within 3-5 years
//! - **Minimum Score**: Must meet or exceed target jurisdiction's minimum passing score
//! - **Additional Requirements**: Some states require supplementary exams (e.g., NY)
//! - **No Retake Required**: Score earned in one UBE jurisdiction can be reused
//!
//! ## Strategic Considerations
//! Applicants often take the UBE in states with lower minimum scores (260-266) then
//! transfer to states requiring higher scores (270-280) if they score sufficiently high.
//! Example: Take exam in Minnesota (260 minimum), score 275, transfer to multiple states.

use crate::professional_licensing::types::ReciprocityType;
use crate::states::types::StateId;
use serde::{Deserialize, Serialize};

/// UBE adoption status for a jurisdiction
///
/// Indicates whether a jurisdiction has adopted the Uniform Bar Examination (UBE)
/// and, if so, provides details about adoption year, minimum passing score, and any
/// additional state-specific requirements beyond the standard UBE components.
///
/// # UBE vs. Non-UBE Jurisdictions
///
/// ## UBE Jurisdictions (40 as of 2024)
/// These jurisdictions administer the standard three-part UBE (MEE, MPT, MBE) and
/// accept score transfers from other UBE jurisdictions. Key characteristics:
/// - **Score Portability**: Scores can be transferred between UBE jurisdictions
/// - **Standardized Content**: All use the same exam on the same dates
/// - **Varying Standards**: Each sets its own minimum passing score (260-280)
/// - **Supplemental Requirements**: Some require additional state-specific components
///
/// **Example**: New York requires UBE score of 266 plus completion of the New York
/// Law Exam (NYLE) or NY Law Course to ensure knowledge of NY-specific law.
///
/// ## Non-UBE Jurisdictions (11 as of 2024)
/// These maintain their own state-specific bar examinations. Reasons for not adopting:
/// - **Unique Legal System**: Louisiana (Civil Law vs. Common Law)
/// - **Large Market**: California, Florida (sufficient local applicant pool)
/// - **State Autonomy**: Preference for state-specific content and control
/// - **Quality Concerns**: Perception that UBE may not adequately test local law
///
/// **Example**: California administers its own exam with extensive California-specific
/// content, including California Civil Procedure, California Evidence, and performance
/// tests based on California law.
///
/// # Trends
/// - **Momentum Slowing**: Only 3 adoptions in 2020-2024 vs. 14 in 2018-2022
/// - **Large Holdouts**: CA, FL, LA unlikely to adopt due to unique circumstances
/// - **Full Saturation**: ~78% of US jurisdictions (40/51) have adopted
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UBEStatus {
    /// State has adopted the UBE
    ///
    /// Jurisdiction administers the standard UBE and accepts score transfers from
    /// other UBE jurisdictions, subject to meeting minimum score requirements.
    Adopted {
        /// Year of adoption (2011-2024)
        ///
        /// First wave: 2011-2013 (9 states)
        /// Second wave: 2014-2017 (14 states)
        /// Third wave: 2018-2022 (14 states)
        /// Recent: 2023-2024 (3 states)
        adoption_year: u16,

        /// Minimum passing score (0-400 scale)
        ///
        /// Typical range: 260-280
        /// - Lowest: 260 (AL, MN, MO, NM, ND, SD)
        /// - Most common: 266-270 (33 jurisdictions)
        /// - Highest: 280 (AK)
        minimum_score: u16,

        /// Additional requirements beyond standard UBE
        ///
        /// Some jurisdictions require supplemental components to ensure knowledge
        /// of state-specific law:
        /// - **New York**: NY Law Exam (NYLE) or NY Law Course
        /// - **Future**: Other states may add similar requirements
        ///
        /// Empty vector indicates no additional requirements beyond standard UBE.
        additional_requirements: Vec<String>,
    },

    /// State has not adopted the UBE
    ///
    /// Jurisdiction maintains its own state-specific bar examination. Applicants
    /// cannot transfer UBE scores from other states.
    NotAdopted {
        /// Name of the state-specific bar exam
        ///
        /// Examples:
        /// - "California Bar Examination" (most difficult, ~40-50% pass rate)
        /// - "Louisiana Bar Examination (Civil Law)" (tests both Common and Civil Law)
        /// - "Florida Bar Examination" (includes unique public policy questions)
        exam_name: String,

        /// Whether partial UBE scores are accepted
        ///
        /// Some non-UBE states accept MBE scores from NCBE but not the full UBE.
        /// Currently, most non-UBE states do not accept partial scores.
        accepts_partial_ube: bool,
    },
}

/// Bar admission requirements for a jurisdiction
///
/// Complete set of requirements for admission to practice law in a jurisdiction.
/// This includes examination requirements (UBE or state-specific), character and
/// fitness review, law school credentials, and reciprocity provisions.
///
/// # Components
///
/// ## Examination Requirements
/// - **UBE States**: Must pass UBE with minimum score or transfer qualifying score
/// - **Non-UBE States**: Must pass state-specific bar examination
///
/// ## MPRE (Ethics Exam)
/// Most states require the Multistate Professional Responsibility Examination (MPRE),
/// a 60-question, 2-hour exam on legal ethics and professional responsibility:
/// - **Standard**: 85/120 (most states)
/// - **Higher**: 86/120 (California, Utah)
/// - **Lower**: 75/120 (Georgia, Arizona)
/// - **Exempt**: Wisconsin (diploma privilege), Puerto Rico, Palau
///
/// ## Character and Fitness
/// All jurisdictions conduct extensive background investigations covering:
/// - Criminal history, Academic misconduct, Financial responsibility (bankruptcy, debt)
/// - Mental health (substance abuse treatment), Employment history, Candor in application
///
/// ## Law School Requirements
/// Varies by jurisdiction. Most require ABA-accredited JD, but some accept:
/// - State-accredited schools (California)
/// - Foreign law degrees with LLM (many states)
/// - Correspondence/online programs (limited)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BarAdmissionRequirements {
    /// State identifier
    pub state_id: StateId,

    /// UBE status and requirements
    ///
    /// Indicates whether state uses UBE or state-specific exam
    pub ube_status: UBEStatus,

    /// MPRE minimum score (0-120 scale)
    ///
    /// None indicates MPRE not required (rare: WI, PR, Palau)
    /// Typical scores:
    /// - 85: Standard (42 states)
    /// - 86: California, Utah
    /// - 75: Georgia, Arizona
    pub mpre_minimum_score: Option<u16>,

    /// Character and fitness investigation required
    ///
    /// All US jurisdictions require this. Investigation typically takes 3-6 months
    /// and includes fingerprinting, references, and detailed personal history.
    pub character_and_fitness: bool,

    /// Law school requirements
    ///
    /// Educational credentials required for bar admission
    pub law_school_requirements: LawSchoolRequirements,

    /// Reciprocity with other states
    ///
    /// Rules for admission without examination based on prior admission elsewhere
    pub reciprocity: ReciprocityType,

    /// Pro hac vice admission available
    ///
    /// Whether temporary admission for specific cases is permitted
    pub pro_hac_vice_available: bool,
}

/// Law school requirements for bar admission
///
/// Educational credentials required to sit for the bar exam. The gold standard is
/// a Juris Doctor (JD) degree from an American Bar Association (ABA)-accredited
/// law school, but some jurisdictions accept alternatives.
///
/// # ABA Accreditation
/// The ABA Section of Legal Education and Admissions to the Bar accredits ~200 US
/// law schools. ABA accreditation ensures:
/// - Sufficient library resources and faculty credentials
/// - Rigorous curriculum (typically 3 years full-time or 4 years part-time)
/// - Clinical and experiential learning opportunities
///
/// # State-Accredited Schools
/// California uniquely accepts graduates from California State Bar-accredited schools,
/// which include:
/// - **Correspondence schools**: Self-paced programs
/// - **Distance learning**: Online law schools
/// - **Unaccredited fixed-facility schools**: Traditional but not ABA-accredited
///
/// California's approach reflects its "reading the law" tradition (apprenticeship),
/// formally abolished in most states by the early 20th century.
///
/// # Foreign Law Degrees
/// Most states allow foreign-educated lawyers to sit for the bar if they complete:
/// - LLM (Master of Laws) from ABA-accredited US law school
/// - Credential evaluation showing equivalence to US JD
///
/// Some states (NY, CA) have more generous rules for foreign lawyers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LawSchoolRequirements {
    /// Must be ABA-accredited
    ///
    /// If true, only ABA-accredited JD degrees qualify.
    /// If false, state may accept state-accredited or other alternatives.
    pub aba_accredited_required: bool,

    /// State-accredited schools accepted
    ///
    /// California accepts graduates from California State Bar-accredited schools
    /// (includes correspondence and distance learning programs).
    pub state_accredited_accepted: bool,

    /// Foreign law degrees accepted (with conditions)
    ///
    /// Most states accept foreign law degrees if holder completes US LLM program.
    /// Requirements vary: some mandate specific LLM content, others require
    /// credential evaluation.
    pub foreign_degrees_accepted: bool,
}

/// Pro hac vice admission rules
///
/// "Pro hac vice" (Latin: "for this occasion only") allows out-of-state attorneys
/// to appear in a single case without full bar admission. This promotes client
/// choice and access to specialized counsel.
///
/// # Purpose and Policy
/// Pro hac vice addresses the tension between:
/// - **Federalism**: Each state regulates its own bar
/// - **Client Choice**: Clients may want their existing attorney
/// - **Specialization**: Complex cases may require specialized out-of-state counsel
/// - **State Interest**: Protecting public from unqualified practitioners
///
/// # Typical Requirements
/// - **Local Counsel**: Out-of-state attorney must associate with local counsel
/// - **Application**: File motion with court, pay fee ($50-$500)
/// - **Good Standing**: Must be admitted in home jurisdiction
/// - **Reciprocity**: Some states require home state to grant similar privileges
/// - **Limitations**: Restricted number of pro hac vice admissions per year
///
/// # Federal Courts
/// Federal courts have their own pro hac vice rules (often more permissive than state).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProHacViceRules {
    /// State identifier
    pub state_id: StateId,

    /// Whether pro hac vice is available
    ///
    /// Nearly all jurisdictions allow pro hac vice in some form.
    pub available: bool,

    /// Maximum number of cases per year
    ///
    /// Some jurisdictions limit pro hac vice to prevent systematic practice:
    /// - Typical limit: 3-5 cases per year
    /// - None: No numerical limit (most common)
    pub max_cases_per_year: Option<u8>,

    /// Local counsel required
    ///
    /// Most jurisdictions require pro hac vice attorney to associate with local counsel
    /// who is fully admitted in the jurisdiction and actively participates in case.
    pub local_counsel_required: bool,

    /// Application fee (in cents for precision)
    ///
    /// Fee to file pro hac vice motion:
    /// - Range: $50-$500
    /// - None: No fee required (rare)
    pub application_fee: Option<u64>,

    /// Restrictions on pro hac vice practice
    ///
    /// Examples:
    /// - "May not handle domestic relations cases"
    /// - "Requires reciprocity from home state"
    /// - "Not available in criminal cases"
    /// - "Requires continuing legal education in state law"
    pub restrictions: Vec<String>,
}

/// Multijurisdictional practice (MJP) rules
///
/// Rules governing when attorneys can provide legal services across state lines
/// without being admitted in the jurisdiction where services are provided.
///
/// # Legal Framework
///
/// ## ABA Model Rule 5.5
/// The American Bar Association's Model Rule of Professional Conduct 5.5 authorizes
/// four categories of multijurisdictional practice:
///
/// 1. **Temporary Practice Related to Litigation**: Pro hac vice, arbitration,
///    mediation, and services reasonably related thereto
/// 2. **Temporary Practice Related to Transactional Matters**: Services arising out
///    of attorney's home-state practice, reasonably related to pending/potential litigation
/// 3. **Association with Local Counsel**: Temporary practice in association with local
///    attorney admitted in jurisdiction
/// 4. **Other Temporary Practice**: Services reasonably related to attorney's
///    home-state practice
///
/// ## Permanent Practice Prohibition
/// Attorneys generally cannot establish a systematic and continuous presence in a
/// jurisdiction without admission to its bar. Exceptions:
/// - **In-house counsel**: Many states allow admission for corporate counsel
/// - **Federal practice**: Federal admission doesn't require state admission
/// - **Special programs**: Some states have limited license programs
///
/// # Evolution
/// Pre-2002: Most states prohibited unauthorized practice strictly
/// Post-2002: ABA Model Rule 5.5 liberalized rules to reflect modern law practice
/// reality (interstate commerce, national firms, telecommuting)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultijurisdictionalPractice {
    /// Systematic and continuous presence allowed
    ///
    /// If true, attorney can establish ongoing practice without full admission
    /// (very rare; typically only for in-house counsel programs).
    ///
    /// If false, attorney must seek full bar admission for permanent practice.
    pub systematic_presence_allowed: bool,

    /// Temporary practice allowed
    ///
    /// If true, attorney can engage in limited temporary practice under ABA
    /// Model Rule 5.5 categories (litigation-related, transactional, etc.).
    ///
    /// Most states have adopted some version of this since 2002.
    pub temporary_practice_allowed: bool,

    /// Conditions for temporary practice
    ///
    /// Examples:
    /// - "Must be reasonably related to home-state practice"
    /// - "Limited to transactional work, not litigation"
    /// - "Requires notice to state bar"
    /// - "Must associate with local counsel"
    /// - "Limited to federal practice"
    pub temporary_conditions: Vec<String>,
}

/// Check if a UBE score can be transferred between jurisdictions
///
/// Determines whether a UBE score earned in one jurisdiction meets the minimum
/// passing score of another jurisdiction, enabling admission by score transfer
/// rather than retaking the examination.
///
/// # Score Transfer Mechanics
///
/// The UBE's portability is its primary benefit. When an examinee passes the UBE
/// in one jurisdiction, they can apply that score to any other UBE jurisdiction
/// within the score's validity period (typically 3-5 years). The examinee must:
///
/// 1. **Meet Minimum Score**: Score must equal or exceed target jurisdiction's minimum
/// 2. **Within Time Limit**: Most jurisdictions require transfer within 3-5 years
/// 3. **Complete C&F**: Pass character and fitness investigation in target jurisdiction
/// 4. **Meet MPRE**: Satisfy target jurisdiction's MPRE requirement (if any)
/// 5. **Additional Requirements**: Complete any jurisdiction-specific requirements
///    (e.g., New York Law Exam for NY)
///
/// # Strategic Considerations
///
/// ## Test in Lower-Minimum States
/// Examinees sometimes strategically take the UBE in states with lower minimum
/// passing scores (260-266) to increase chances of passing, then transfer to
/// higher-minimum states (270-280) if they score high enough.
///
/// **Example**: Take exam in Alabama (260 minimum). If score is 275, can transfer to:
/// - Colorado (276): No
/// - Alaska (280): No
/// - Oregon (270): Yes
/// - New York (266): Yes (but must also complete NYLE)
///
/// ## Multiple Transfers
/// A single UBE score can be transferred to multiple jurisdictions. This enables
/// "admission on motion" to several states without retaking the exam.
///
/// # Arguments
/// * `from_state` - Two-letter code of state where UBE was taken (e.g., "NY", "TX")
/// * `to_state` - Two-letter code of target state for admission (e.g., "CO", "IL")
/// * `score` - UBE score earned (0-400 scale)
///
/// # Returns
/// * `true` - Score meets target jurisdiction's minimum and both jurisdictions use UBE
/// * `false` - Score below target jurisdiction's minimum, or either jurisdiction doesn't use UBE
///
/// # Example
/// ```
/// use legalis_us::professional_licensing::bar_admission::can_transfer_ube_score;
///
/// // Transfer 280 score from NY to Colorado (CO requires 276)
/// assert!(can_transfer_ube_score("NY", "CO", 280));
///
/// // 270 is below Colorado's 276 minimum
/// assert!(!can_transfer_ube_score("NY", "CO", 270));
///
/// // Cannot transfer to California (doesn't use UBE)
/// assert!(!can_transfer_ube_score("NY", "CA", 300));
/// ```
///
/// # Limitations
///
/// This function only checks whether the score meets the minimum threshold. It does NOT:
/// - Check time limits (3-5 years)
/// - Verify character and fitness
/// - Check MPRE completion
/// - Verify additional requirements (e.g., NYLE for NY)
/// - Check reciprocity restrictions
pub fn can_transfer_ube_score(from_state: &str, to_state: &str, score: u16) -> bool {
    let from_ube = ube_status(from_state);
    let to_ube = ube_status(to_state);

    match (from_ube, to_ube) {
        (UBEStatus::Adopted { .. }, UBEStatus::Adopted { minimum_score, .. }) => {
            score >= minimum_score
        }
        _ => false,
    }
}

/// Get UBE status for a state
///
/// Returns the UBE adoption status and requirements for a given jurisdiction,
/// including adoption year, minimum passing score, and any additional requirements
/// beyond the standard three-part UBE.
///
/// # UBE vs. State-Specific Exams
///
/// This function distinguishes between:
/// - **UBE Jurisdictions (40)**: Return `UBEStatus::Adopted` with score requirements
/// - **Non-UBE Jurisdictions (11)**: Return `UBEStatus::NotAdopted` with exam name
///
/// # Notable Jurisdictions
///
/// ## New York
/// Returns `Adopted` with `additional_requirements` including the New York Law Exam
/// (NYLE) or NY Law Course. NY adopted UBE in 2016 but added this supplemental
/// requirement to ensure knowledge of NY-specific law.
///
/// ## Louisiana
/// Returns `NotAdopted` with exam name "Louisiana Bar Examination (Civil Law)".
/// Louisiana is the only US state with a Civil Law system, making UBE adoption
/// impractical since the UBE tests Common Law subjects.
///
/// ## California
/// Returns `NotAdopted` with exam name "California Bar Examination". Despite being
/// the largest legal market, California maintains its own exam with extensive
/// state-specific content.
///
/// # Arguments
/// * `state_code` - Two-letter state code (e.g., "NY", "CA", "TX")
///
/// # Returns
/// * `UBEStatus::Adopted` - If jurisdiction uses UBE, with adoption details
/// * `UBEStatus::NotAdopted` - If jurisdiction uses state-specific exam
///
/// # Example
/// ```
/// use legalis_us::professional_licensing::bar_admission::{ube_status, UBEStatus};
///
/// // Check New York (UBE state with additional requirements)
/// let ny = ube_status("NY");
/// match ny {
///     UBEStatus::Adopted { minimum_score, additional_requirements, .. } => {
///         assert_eq!(minimum_score, 266);
///         assert!(!additional_requirements.is_empty()); // Has NYLE requirement
///     }
///     _ => panic!("NY should be UBE"),
/// }
///
/// // Check California (non-UBE)
/// let ca = ube_status("CA");
/// assert!(matches!(ca, UBEStatus::NotAdopted { .. }));
/// ```
pub fn ube_status(state_code: &str) -> UBEStatus {
    match state_code {
        // UBE states with minimum scores (as of 2024)
        "AL" => UBEStatus::Adopted {
            adoption_year: 2011,
            minimum_score: 260,
            additional_requirements: vec![],
        },
        "AK" => UBEStatus::Adopted {
            adoption_year: 2017,
            minimum_score: 280,
            additional_requirements: vec![],
        },
        "AZ" => UBEStatus::Adopted {
            adoption_year: 2017,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "AR" => UBEStatus::Adopted {
            adoption_year: 2018,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "CO" => UBEStatus::Adopted {
            adoption_year: 2011,
            minimum_score: 276,
            additional_requirements: vec![],
        },
        "CT" => UBEStatus::Adopted {
            adoption_year: 2014,
            minimum_score: 266,
            additional_requirements: vec![],
        },
        "DC" => UBEStatus::Adopted {
            adoption_year: 2016,
            minimum_score: 266,
            additional_requirements: vec![],
        },
        "ID" => UBEStatus::Adopted {
            adoption_year: 2011,
            minimum_score: 272,
            additional_requirements: vec![],
        },
        "IL" => UBEStatus::Adopted {
            adoption_year: 2017,
            minimum_score: 266,
            additional_requirements: vec![],
        },
        "IA" => UBEStatus::Adopted {
            adoption_year: 2017,
            minimum_score: 266,
            additional_requirements: vec![],
        },
        "KS" => UBEStatus::Adopted {
            adoption_year: 2017,
            minimum_score: 266,
            additional_requirements: vec![],
        },
        "ME" => UBEStatus::Adopted {
            adoption_year: 2018,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "MD" => UBEStatus::Adopted {
            adoption_year: 2020,
            minimum_score: 266,
            additional_requirements: vec![],
        },
        "MA" => UBEStatus::Adopted {
            adoption_year: 2011,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "MN" => UBEStatus::Adopted {
            adoption_year: 2011,
            minimum_score: 260,
            additional_requirements: vec![],
        },
        "MO" => UBEStatus::Adopted {
            adoption_year: 2011,
            minimum_score: 260,
            additional_requirements: vec![],
        },
        "MT" => UBEStatus::Adopted {
            adoption_year: 2012,
            minimum_score: 266,
            additional_requirements: vec![],
        },
        "NE" => UBEStatus::Adopted {
            adoption_year: 2016,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "NH" => UBEStatus::Adopted {
            adoption_year: 2011,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "NJ" => UBEStatus::Adopted {
            adoption_year: 2019,
            minimum_score: 266,
            additional_requirements: vec![],
        },
        "NM" => UBEStatus::Adopted {
            adoption_year: 2011,
            minimum_score: 260,
            additional_requirements: vec![],
        },
        "NY" => UBEStatus::Adopted {
            adoption_year: 2016,
            minimum_score: 266,
            additional_requirements: vec!["New York Law Exam (NYLE) or NY Law Course".to_string()],
        },
        "NC" => UBEStatus::Adopted {
            adoption_year: 2019,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "ND" => UBEStatus::Adopted {
            adoption_year: 2011,
            minimum_score: 260,
            additional_requirements: vec![],
        },
        "OH" => UBEStatus::Adopted {
            adoption_year: 2020,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "OR" => UBEStatus::Adopted {
            adoption_year: 2022,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "PA" => UBEStatus::Adopted {
            adoption_year: 2022,
            minimum_score: 272,
            additional_requirements: vec![],
        },
        "RI" => UBEStatus::Adopted {
            adoption_year: 2018,
            minimum_score: 276,
            additional_requirements: vec![],
        },
        "SC" => UBEStatus::Adopted {
            adoption_year: 2018,
            minimum_score: 266,
            additional_requirements: vec![],
        },
        "SD" => UBEStatus::Adopted {
            adoption_year: 2017,
            minimum_score: 260,
            additional_requirements: vec![],
        },
        "TN" => UBEStatus::Adopted {
            adoption_year: 2019,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "TX" => UBEStatus::Adopted {
            adoption_year: 2021,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "UT" => UBEStatus::Adopted {
            adoption_year: 2015,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "VT" => UBEStatus::Adopted {
            adoption_year: 2012,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "VA" => UBEStatus::Adopted {
            adoption_year: 2024,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "WA" => UBEStatus::Adopted {
            adoption_year: 2013,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "WV" => UBEStatus::Adopted {
            adoption_year: 2018,
            minimum_score: 270,
            additional_requirements: vec![],
        },
        "WI" => UBEStatus::Adopted {
            adoption_year: 2020,
            minimum_score: 260,
            additional_requirements: vec![],
        },
        "WY" => UBEStatus::Adopted {
            adoption_year: 2013,
            minimum_score: 270,
            additional_requirements: vec![],
        },

        // Non-UBE states
        "CA" => UBEStatus::NotAdopted {
            exam_name: "California Bar Examination".to_string(),
            accepts_partial_ube: false,
        },
        "DE" => UBEStatus::NotAdopted {
            exam_name: "Delaware Bar Examination".to_string(),
            accepts_partial_ube: false,
        },
        "FL" => UBEStatus::NotAdopted {
            exam_name: "Florida Bar Examination".to_string(),
            accepts_partial_ube: false,
        },
        "GA" => UBEStatus::NotAdopted {
            exam_name: "Georgia Bar Examination".to_string(),
            accepts_partial_ube: false,
        },
        "HI" => UBEStatus::NotAdopted {
            exam_name: "Hawaii Bar Examination".to_string(),
            accepts_partial_ube: false,
        },
        "IN" => UBEStatus::NotAdopted {
            exam_name: "Indiana Bar Examination".to_string(),
            accepts_partial_ube: false,
        },
        "KY" => UBEStatus::NotAdopted {
            exam_name: "Kentucky Bar Examination".to_string(),
            accepts_partial_ube: false,
        },
        "LA" => UBEStatus::NotAdopted {
            exam_name: "Louisiana Bar Examination (Civil Law)".to_string(),
            accepts_partial_ube: false,
        },
        "MI" => UBEStatus::NotAdopted {
            exam_name: "Michigan Bar Examination".to_string(),
            accepts_partial_ube: false,
        },
        "MS" => UBEStatus::NotAdopted {
            exam_name: "Mississippi Bar Examination".to_string(),
            accepts_partial_ube: false,
        },
        "NV" => UBEStatus::NotAdopted {
            exam_name: "Nevada Bar Examination".to_string(),
            accepts_partial_ube: false,
        },
        "OK" => UBEStatus::NotAdopted {
            exam_name: "Oklahoma Bar Examination".to_string(),
            accepts_partial_ube: false,
        },

        // Default fallback
        _ => UBEStatus::NotAdopted {
            exam_name: "State Bar Examination".to_string(),
            accepts_partial_ube: false,
        },
    }
}

/// Get bar admission requirements for a state
///
/// Returns comprehensive bar admission requirements for a jurisdiction, including
/// examination type (UBE or state-specific), MPRE requirements, character and fitness
/// investigation, law school credentials, reciprocity rules, and pro hac vice availability.
///
/// # Returned Information
///
/// ## UBE Status
/// Whether jurisdiction uses UBE or state-specific exam (see `ube_status()`)
///
/// ## MPRE Requirements
/// Minimum score on Multistate Professional Responsibility Examination:
/// - **Standard (85)**: Most states (42 jurisdictions)
/// - **Higher (86)**: California, Utah (more stringent ethics requirements)
/// - **Lower (75)**: Georgia, Arizona (less stringent)
/// - **None**: Wisconsin (diploma privilege), Puerto Rico
///
/// ## Character and Fitness
/// All US jurisdictions require comprehensive background investigation covering:
/// - **Criminal History**: Felonies, misdemeanors, arrests, expungements
/// - **Academic Misconduct**: Law school discipline, plagiarism
/// - **Financial Responsibility**: Bankruptcy, unpaid student loans, judgments
/// - **Mental Health**: Substance abuse treatment (controversial, being reformed)
/// - **Employment**: Terminations, bar complaints at prior firms
/// - **Candor**: Truthfulness in application (most common denial reason)
///
/// Process typically takes 3-6 months and requires fingerprinting, references,
/// personal interviews, and submission of detailed questionnaires.
///
/// ## Law School Requirements
/// - **ABA Accreditation**: Most states require ABA-accredited JD
/// - **California Exception**: Accepts California State Bar-accredited schools
/// - **Foreign Degrees**: Most states allow foreign JD + US LLM pathway
///
/// ## Reciprocity
/// Rules for admission without examination:
/// - **Full Reciprocity**: Rare (Wisconsin diploma privilege)
/// - **Score Transfer**: UBE states accept transferred scores
/// - **Conditional**: Some states allow admission after 5-10 years practice
/// - **None**: Must take state's bar exam
///
/// ## Pro Hac Vice
/// Nearly all jurisdictions allow temporary admission for specific cases
///
/// # Arguments
/// * `state_code` - Two-letter state code (e.g., "NY", "CA", "TX")
///
/// # Returns
/// Complete `BarAdmissionRequirements` struct for the jurisdiction
///
/// # Example
/// ```
/// use legalis_us::professional_licensing::bar_admission::bar_requirements;
///
/// // Get New York requirements
/// let ny = bar_requirements("NY");
/// assert_eq!(ny.mpre_minimum_score, Some(85));
/// assert!(ny.character_and_fitness);
/// assert!(ny.pro_hac_vice_available);
///
/// // California has higher MPRE requirement and accepts state-accredited schools
/// let ca = bar_requirements("CA");
/// assert_eq!(ca.mpre_minimum_score, Some(86));
/// assert!(ca.law_school_requirements.state_accredited_accepted);
/// ```
pub fn bar_requirements(state_code: &str) -> BarAdmissionRequirements {
    let state_id = StateId::from_code(state_code);
    let ube = ube_status(state_code);

    // MPRE minimum scores (most states require 85, some require 86)
    let mpre_score = match state_code {
        "CA" | "UT" => Some(86),
        "GA" | "AZ" => Some(75),
        _ => Some(85),
    };

    BarAdmissionRequirements {
        state_id,
        ube_status: ube,
        mpre_minimum_score: mpre_score,
        character_and_fitness: true,
        law_school_requirements: LawSchoolRequirements {
            aba_accredited_required: state_code != "CA", // CA accepts state-accredited
            state_accredited_accepted: state_code == "CA",
            foreign_degrees_accepted: true, // Most states with LLM pathway
        },
        reciprocity: determine_reciprocity(state_code),
        pro_hac_vice_available: true, // Most states allow
    }
}

/// Determine reciprocity type for a state
fn determine_reciprocity(state_code: &str) -> ReciprocityType {
    match state_code {
        // Full reciprocity states (rare)
        "WI" => ReciprocityType::Full, // Diploma privilege for UW-Madison grads

        // UBE states have score transfer reciprocity
        "NY" | "CO" | "MA" | "IL" | "TX" | "WA" | "OR" | "PA" | "NJ" | "CT" | "MD" | "DC" => {
            ReciprocityType::ScoreTransfer {
                minimum_score: match ube_status(state_code) {
                    UBEStatus::Adopted { minimum_score, .. } => minimum_score,
                    _ => 270,
                },
                additional_requirements: if state_code == "NY" {
                    vec!["New York Law Exam (NYLE) or NY Law Course".to_string()]
                } else {
                    vec![]
                },
            }
        }

        // Limited reciprocity
        "CA" => ReciprocityType::Conditional {
            requirements: vec![
                "5 years practice in another jurisdiction".to_string(),
                "Pass California bar exam (no UBE)".to_string(),
            ],
        },

        // Default: no reciprocity
        _ => ReciprocityType::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ube_adoption_status() {
        // UBE states
        let ny_status = ube_status("NY");
        assert!(matches!(ny_status, UBEStatus::Adopted { .. }));
        if let UBEStatus::Adopted {
            minimum_score,
            additional_requirements,
            ..
        } = ny_status
        {
            assert_eq!(minimum_score, 266);
            assert!(!additional_requirements.is_empty()); // NY requires NYLE
        }

        let co_status = ube_status("CO");
        assert!(matches!(co_status, UBEStatus::Adopted { .. }));

        // Non-UBE states
        let ca_status = ube_status("CA");
        assert!(matches!(ca_status, UBEStatus::NotAdopted { .. }));

        let fl_status = ube_status("FL");
        assert!(matches!(fl_status, UBEStatus::NotAdopted { .. }));
    }

    #[test]
    fn test_ube_score_transfer() {
        // Valid transfer: 270 from NY to CO (CO requires 276)
        assert!(!can_transfer_ube_score("NY", "CO", 270)); // 270 < 276

        // Valid transfer: 280 from NY to CO
        assert!(can_transfer_ube_score("NY", "CO", 280));

        // Invalid: CA doesn't use UBE
        assert!(!can_transfer_ube_score("NY", "CA", 300));

        // Valid transfer: 266 from NY to NY (meets minimum)
        assert!(can_transfer_ube_score("NY", "NY", 266));
    }

    #[test]
    fn test_bar_requirements() {
        let ny_reqs = bar_requirements("NY");
        assert!(matches!(ny_reqs.ube_status, UBEStatus::Adopted { .. }));
        assert_eq!(ny_reqs.mpre_minimum_score, Some(85));
        assert!(ny_reqs.character_and_fitness);

        let ca_reqs = bar_requirements("CA");
        assert!(matches!(ca_reqs.ube_status, UBEStatus::NotAdopted { .. }));
        assert_eq!(ca_reqs.mpre_minimum_score, Some(86)); // CA requires 86
        assert!(ca_reqs.law_school_requirements.state_accredited_accepted);
    }

    #[test]
    fn test_reciprocity_determination() {
        let wi_reciprocity = determine_reciprocity("WI");
        assert_eq!(wi_reciprocity, ReciprocityType::Full);

        let ny_reciprocity = determine_reciprocity("NY");
        assert!(matches!(
            ny_reciprocity,
            ReciprocityType::ScoreTransfer { .. }
        ));

        let ca_reciprocity = determine_reciprocity("CA");
        assert!(matches!(
            ca_reciprocity,
            ReciprocityType::Conditional { .. }
        ));
    }

    #[test]
    fn test_minimum_scores_vary() {
        let al_status = ube_status("AL");
        if let UBEStatus::Adopted { minimum_score, .. } = al_status {
            assert_eq!(minimum_score, 260); // Alabama: 260
        }

        let ak_status = ube_status("AK");
        if let UBEStatus::Adopted { minimum_score, .. } = ak_status {
            assert_eq!(minimum_score, 280); // Alaska: 280 (highest)
        }

        let co_status = ube_status("CO");
        if let UBEStatus::Adopted { minimum_score, .. } = co_status {
            assert_eq!(minimum_score, 276); // Colorado: 276
        }
    }

    #[test]
    fn test_louisiana_civil_law() {
        let la_status = ube_status("LA");
        assert!(matches!(la_status, UBEStatus::NotAdopted { .. }));
        if let UBEStatus::NotAdopted { exam_name, .. } = la_status {
            assert!(exam_name.contains("Civil Law"));
        }
    }

    #[test]
    fn test_all_51_jurisdictions() {
        let states = vec![
            "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN",
            "IA", "KS", "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV",
            "NH", "NJ", "NM", "NY", "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN",
            "TX", "UT", "VT", "VA", "WA", "WV", "WI", "WY", "DC",
        ];

        for state in states {
            let status = ube_status(state);
            let reqs = bar_requirements(state);

            // All should have valid status
            match status {
                UBEStatus::Adopted { minimum_score, .. } => {
                    assert!((260..=280).contains(&minimum_score));
                }
                UBEStatus::NotAdopted { .. } => {
                    // OK
                }
            }

            // All should have MPRE requirement
            assert!(reqs.mpre_minimum_score.is_some());
        }
    }
}
