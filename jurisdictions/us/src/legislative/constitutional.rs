//! State Constitutional Provisions Tracking
//!
//! This module tracks state constitutional provisions that provide protections
//! beyond the federal constitutional floor, demonstrating how state constitutions
//! often provide stronger protections than their federal counterpart.
//!
//! # Overview
//!
//! The United States operates under a system of **cooperative federalism** where
//! both federal and state governments exercise sovereign powers. While the U.S.
//! Constitution establishes minimum protections that apply nationwide (the "floor"),
//! state constitutions are free to provide greater protections (raising the "ceiling").
//!
//! ## The Floor vs. Ceiling Framework
//!
//! The Supremacy Clause (U.S. Const. Art. VI, cl. 2) establishes that federal law
//! is supreme when it conflicts with state law. However, this only prevents states
//! from providing **less** protection than the federal Constitution requires. States
//! remain free—and often choose—to provide **more** protection through:
//!
//! - **Explicit Constitutional Rights**: Rights enumerated in state constitutions
//!   but absent from the federal Constitution (e.g., privacy, environmental protection)
//! - **Broader Interpretations**: State courts interpreting parallel provisions more
//!   expansively than federal courts (e.g., free speech, search and seizure)
//! - **Structural Differences**: Direct democracy mechanisms not present federally
//!   (initiative and referendum)
//!
//! This principle, known as **"adequate and independent state grounds,"** allows
//! state supreme courts to interpret their own constitutions without federal judicial
//! review, as long as they meet or exceed federal protections.
//!
//! # Key Areas Tracked
//!
//! ## 1. Constitutional Privacy Rights
//!
//! Unlike the U.S. Constitution, which has no explicit privacy right (privacy is
//! inferred from the penumbras of other amendments per *Griswold v. Connecticut*),
//! **10 states** have enacted explicit constitutional privacy protections:
//!
//! - **California (1972)**: Made privacy an "inalienable right" via Prop 11
//! - **Florida (1980)**: Art. I, § 23 guarantees "right to be let alone"
//! - **Alaska (1972)**: Art. I, § 22 privacy protections
//! - **Plus 7 others**: Arizona, Hawaii, Illinois, Louisiana, Montana, South Carolina, Washington
//!
//! These provisions provide grounds for state courts to recognize privacy rights
//! in contexts where federal courts might not, including:
//! - Employee privacy in the workplace
//! - Consumer data privacy
//! - Reproductive rights (increasingly important post-*Dobbs*)
//! - Medical privacy
//!
//! Additionally, **5 states** (NY, MA, PA, NJ, MI) recognize implicit constitutional
//! privacy rights through state supreme court decisions.
//!
//! ## 2. Direct Democracy: Initiative and Referendum
//!
//! The U.S. Constitution provides no mechanism for direct citizen lawmaking—all
//! federal laws must pass through Congress. In contrast, **23 states** have adopted
//! some form of **citizen initiative**, allowing voters to directly enact laws or
//! constitutional amendments by gathering signatures and winning a majority vote.
//!
//! ### Historical Context: The Progressive Era
//!
//! Initiative and referendum emerged during the **Progressive Era (1890-1920)** as
//! a response to perceived legislative corruption and railroad monopoly influence:
//!
//! - **South Dakota (1898)**: First state to adopt initiative and referendum
//! - **Oregon (1902)**: Most influential early adopter, inspired national movement
//! - **1902-1918**: 19 states adopted direct democracy mechanisms
//! - **Post-1959**: Only 4 states have adopted (Alaska, Florida, Illinois, Wyoming)
//!
//! Direct democracy remains concentrated in western states, reflecting Progressive
//! Era populist movements. Southern and eastern states generally lack these powers.
//!
//! ### Types of Direct Democracy
//!
//! - **Statutory Initiative**: Citizens propose and enact statutes directly
//! - **Constitutional Initiative**: Citizens propose and adopt constitutional amendments
//! - **Popular Referendum**: Citizens veto recently enacted legislation
//! - **Legislative Referral**: Legislature refers measures to voters (all states)
//!
//! **Notable uses**:
//! - Cannabis legalization (CA Prop 64, CO Amendment 64)
//! - Tax limitations (CA Prop 13, CO TABOR)
//! - Social policy (marriage equality, affirmative action)
//! - Right to repair (MA automotive initiative)
//!
//! ## 3. State Constitutional Protections Beyond Federal Floor
//!
//! State constitutions often include provisions with no federal parallel:
//!
//! ### Environmental Rights
//! - **Montana**: "Right to a clean and healthful environment" (Art. II, § 3)
//! - **Pennsylvania**: "Right to clean air, pure water, and preservation of natural resources"
//! - **Hawaii**: Environmental rights and indigenous Hawaiian rights
//!
//! ### Education
//! - **Massachusetts**: Education as "duty of the legislature" (Art. V)
//! - Many states guarantee "adequate" or "thorough and efficient" education
//! - Basis for education funding lawsuits
//!
//! ### Government Transparency
//! - **Florida**: Strong public records and open meetings requirements (Art. I, § 24)
//! - "Sunshine laws" often have constitutional basis in state constitutions
//!
//! ### Economic and Social Rights
//! - Worker rights and union protections
//! - Prohibition on certain taxation (property tax limits)
//! - Victims' rights amendments
//! - English-only provisions
//!
//! # Legal Significance
//!
//! State constitutional provisions have profound legal implications:
//!
//! 1. **Independent Source of Rights**: State courts can rely exclusively on state
//!    constitutional grounds, insulating decisions from federal Supreme Court review
//! 2. **Post-*Dobbs* Protections**: After federal abortion rights were eliminated,
//!    many states turned to state constitutional privacy provisions
//! 3. **Data Privacy**: State constitutional privacy rights support consumer privacy
//!    legislation like CCPA/CPRA
//! 4. **Policy Experimentation**: Direct democracy enables rapid policy adoption
//!    bypassing legislative gridlock
//!
//! # Data Sources and Limitations
//!
//! This module tracks:
//! - Constitutional text and provision citations
//! - Year of adoption for privacy rights and direct democracy
//! - Notable ballot measures (non-exhaustive)
//! - Selected "beyond federal floor" provisions (non-exhaustive)
//!
//! Limitations:
//! - Does not track all state constitutional provisions (would require thousands of entries)
//! - Focus on provisions with significant interstate variation
//! - Case law references minimal (would require extensive legal research)
//! - "Beyond federal floor" examples illustrative, not comprehensive

use crate::states::types::StateId;
use serde::{Deserialize, Serialize};

/// Type of constitutional privacy right recognized in a state
///
/// This enum categorizes how (and whether) a state's constitution protects privacy.
/// The federal Constitution has no explicit privacy right—privacy protection is
/// inferred from the First, Third, Fourth, Fifth, and Ninth Amendments (the "penumbral
/// rights" theory from *Griswold v. Connecticut*, 381 U.S. 479 (1965)).
///
/// # Categories
///
/// ## Explicit Privacy Rights
///
/// Ten states have amended their constitutions to include explicit privacy provisions,
/// typically in their declarations of rights or bills of rights. These provisions
/// vary in scope and language:
///
/// - **California** (1972): "Privacy" listed among inalienable rights including life,
///   liberty, and pursuing happiness. Most influential state privacy provision.
/// - **Florida** (1980): "Right to be let alone" and freedom from governmental intrusion
///   into private life except as provided by law.
/// - **Alaska** (1972): Right to privacy explicitly protected, interpreted broadly by
///   Alaska Supreme Court (including marijuana possession in home).
///
/// These explicit provisions enable state courts to:
/// - Protect privacy in areas federal courts won't (e.g., private employer searches)
/// - Apply strict scrutiny to privacy infringements
/// - Provide baseline for statutory privacy protections (e.g., CCPA in California)
///
/// ## Implicit Privacy Rights
///
/// Five major states (NY, MA, PA, NJ, MI) recognize constitutional privacy rights
/// through judicial interpretation of other constitutional provisions, similar to
/// the federal approach but often more protective:
///
/// - May derive from state due process clauses
/// - May derive from state search and seizure provisions
/// - Generally narrower than explicit provisions
/// - More vulnerable to legislative override or judicial reconsideration
///
/// ## No Constitutional Privacy Right
///
/// Many states lack both explicit and judicially-recognized constitutional privacy
/// rights, though they may still have:
/// - Statutory privacy protections
/// - Common law privacy torts
/// - Constitutional protections against unreasonable searches
///
/// # Historical Context
///
/// The wave of state constitutional privacy amendments in 1972-1980 was influenced by:
/// - Warren & Brandeis's "The Right to Privacy" (1890 Harvard Law Review article)
/// - *Griswold v. Connecticut* (1965) establishing federal privacy right
/// - *Roe v. Wade* (1973) recognizing abortion as privacy right
/// - Rising concerns about computer databases and government surveillance
/// - Consumer privacy concerns in the information age
///
/// Post-*Dobbs* (2022), state constitutional privacy provisions have renewed importance
/// for protecting reproductive rights where the federal Constitution no longer does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstitutionalPrivacyRight {
    /// Explicit privacy right enumerated in state constitution
    ///
    /// The state constitution contains a specific provision protecting privacy,
    /// typically in the declaration or bill of rights.
    Explicit {
        /// Constitutional provision citation (e.g., "Art. I, § 1" for California)
        provision: String,

        /// Year the privacy provision was adopted or ratified
        ///
        /// Usually reflects constitutional amendment via initiative or legislative
        /// referral. For original constitutions (e.g., Washington 1889), indicates
        /// when privacy interpretation was adopted by courts.
        year_adopted: u16,
    },

    /// Implicit privacy right recognized through judicial interpretation
    ///
    /// State supreme court has recognized a constitutional privacy right derived
    /// from other provisions (due process, search and seizure, etc.) without an
    /// explicit constitutional text.
    Implicit {
        /// Key state supreme court case establishing the privacy right
        ///
        /// If known. Many states' privacy doctrines evolve through multiple cases
        /// without a single watershed decision.
        key_case: Option<String>,
    },

    /// No recognized constitutional privacy right
    ///
    /// The state constitution contains no explicit privacy provision, and state
    /// courts have not recognized an implicit constitutional privacy right. Does
    /// not mean the state has no privacy protections—statutory and common law
    /// privacy protections may still exist.
    None,
}

/// Initiative and referendum status for a state
///
/// This enum tracks direct democracy mechanisms that allow citizens to bypass
/// the legislature and directly enact laws or constitutional amendments. These
/// mechanisms emerged during the Progressive Era (1890-1920) as a response to
/// perceived legislative corruption and undue influence by railroads and monopolies.
///
/// # Terminology
///
/// - **Initiative**: Citizens propose new laws or amendments by gathering signatures,
///   then voters approve or reject at the ballot box
/// - **Referendum**: Citizens can force a public vote on recently enacted legislation
///   (popular referendum) or the legislature can refer measures to voters (legislative
///   referral)
/// - **Recall**: Some states also allow citizens to remove elected officials (not
///   tracked here)
///
/// # Types of Initiative
///
/// ## Constitutional Initiative
/// Citizens can propose and adopt constitutional amendments directly. This is the
/// most powerful form of direct democracy, as it allows citizens to make fundamental
/// changes to state governance that the legislature cannot easily reverse.
///
/// **Notable uses**:
/// - California Prop 13 (1978): Property tax limits
/// - California Prop 11 (1972): Privacy as inalienable right
/// - Colorado TABOR (1992): Taxpayer Bill of Rights
///
/// ## Statutory Initiative
/// Citizens can propose and enact ordinary statutes (laws). The legislature can
/// later amend or repeal these laws (unlike constitutional amendments).
///
/// **Notable uses**:
/// - California Prop 64 (2016): Recreational cannabis legalization
/// - Massachusetts Question 1 (2012): Automotive right to repair
/// - Colorado Amendment 64 (2012): Recreational cannabis legalization
///
/// # Distribution by Type
///
/// - **Both Initiative and Referendum**: 21 states (most common configuration)
/// - **Initiative Only**: 2 states (MS, though MS's was suspended in 2021)
/// - **Referendum Only** (Legislative Referral): 27 states + DC
/// - **None**: 0 states (all have at least legislative referral)
///
/// Note: "Referendum Only" means the legislature can refer measures to voters, but
/// citizens cannot initiate measures themselves. This is technically available in
/// all states and is not the defining characteristic.
///
/// # Geographic and Historical Patterns
///
/// **Western Concentration**: Initiative and referendum is concentrated in western
/// states, reflecting Progressive Era populist movements. Of the 23 states with
/// citizen initiative:
/// - **West**: 16 states (AK, AZ, AR, CA, CO, ID, MT, NE, NV, ND, OK, OR, SD, UT, WA, WY)
/// - **Midwest**: 5 states (IL, MI, MO, OH)
/// - **East**: 2 states (MA, ME - though ME is often grouped with New England)
/// - **South**: 2 states (AR, FL, MS, OK - if counting border states)
///
/// **Progressive Era Adoption (1898-1918)**:
/// - South Dakota (1898): First state to adopt
/// - Oregon (1902): Most influential early adopter, used extensively
/// - 1910-1912: Wave of adoptions (AR, CA, CO, AZ, ID, NE, NV, OH, WA)
///
/// **Post-1918 Adoptions** (only 4 states):
/// - Alaska (1959): At statehood
/// - Florida (1968): Constitutional revision
/// - Illinois (1970): Constitutional convention
/// - Wyoming (1968): Late western adopter
///
/// **Never Adopted**: Notably absent in Texas, New York, Pennsylvania, Virginia,
/// and most Southern states, reflecting different political cultures and traditions.
///
/// # Criticisms and Defenses
///
/// **Criticisms**:
/// - **Tyranny of the majority**: Can infringe on minority rights (e.g., Prop 8)
/// - **Complexity**: Voters may lack expertise to evaluate technical measures
/// - **Money influence**: Well-funded signature gathering and advertising campaigns
/// - **Unintended consequences**: Hastily drafted measures may have legal flaws
///
/// **Defenses**:
/// - **Legislative gridlock bypass**: Enables action when legislatures won't act
/// - **Popular sovereignty**: Ultimate expression of democratic self-governance
/// - **Policy innovation**: Allows experimentation (e.g., cannabis legalization)
/// - **Accountability**: Politicians fear override via initiative
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InitiativeReferendumStatus {
    /// State allows both citizen initiative and popular referendum
    ///
    /// This is the most common configuration in states with direct democracy
    /// (21 states). Citizens can both propose new measures (initiative) and
    /// force votes on recently enacted legislation (referendum).
    Both {
        /// Year when initiative/referendum powers were adopted
        ///
        /// For states that adopted at different times, reflects the earlier adoption.
        /// Most adoptions occurred 1898-1918 during the Progressive Era.
        year_adopted: u16,

        /// Whether citizens can propose constitutional amendments
        ///
        /// This is the most powerful form of direct democracy. When `true`, citizens
        /// can make fundamental changes to state governance that the legislature
        /// cannot easily reverse.
        constitutional_amendments: bool,

        /// Whether citizens can propose ordinary statutes (laws)
        ///
        /// When `true`, citizens can enact legislation directly, though the
        /// legislature can later amend or repeal these statutes (unlike amendments).
        statutes: bool,
    },

    /// State allows citizen initiative but not popular referendum
    ///
    /// Rare configuration (only Mississippi, and suspended in 2021). Citizens can
    /// propose measures but cannot force votes on enacted legislation.
    InitiativeOnly {
        /// Year initiative power was adopted
        year_adopted: u16,

        /// Whether constitutional amendments can be initiated
        constitutional_amendments: bool,

        /// Whether statutes can be initiated
        statutes: bool,
    },

    /// State allows legislative referral but not citizen initiative
    ///
    /// The legislature can refer measures to voters, but citizens cannot initiate
    /// measures themselves. This is the situation in 27 states + DC. All states
    /// have legislative referral (for constitutional amendments, at minimum), so
    /// this variant indicates the **absence** of citizen initiative power.
    ReferendumOnly {
        /// Year constitutional basis established (typically statehood)
        ///
        /// Since all states have had legislative referral since founding, this
        /// typically reflects statehood year or 1789 for original states.
        year_adopted: u16,
    },

    /// No initiative or referendum mechanisms
    ///
    /// This variant is technically never used, as all states have at least
    /// legislative referral. It exists for completeness and potential future use.
    None,
}

/// Direct democracy powers available in a state
///
/// Aggregates information about a state's direct democracy mechanisms, including
/// the type of initiative/referendum powers, signature requirements, and notable
/// historical uses.
///
/// # Signature Thresholds
///
/// Most states require a percentage of voters (or votes cast in the previous
/// gubernatorial/presidential election) to sign petitions to qualify an initiative
/// for the ballot. Typical thresholds:
///
/// - **Low**: 3-5% (e.g., Colorado 5%, North Dakota 4%)
/// - **Moderate**: 6-8% (e.g., California 8%, Oregon 8%)
/// - **High**: 10-15% (e.g., Arizona 10%, Wyoming 15%)
///
/// Higher thresholds make qualification more difficult, favoring well-funded
/// professional signature gathering operations. Lower thresholds enable grassroots
/// efforts but can lead to ballot overcrowding.
///
/// Constitutional amendments typically require higher signature thresholds than
/// statutory initiatives (e.g., California requires 8% for statutes, 8% for amendments,
/// but both use different base calculations).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectDemocracyPowers {
    /// Initiative and referendum status for this state
    ///
    /// Indicates whether the state allows citizen initiative, popular referendum,
    /// or only legislative referral.
    pub status: InitiativeReferendumStatus,

    /// Signature requirement as percentage of voters needed to qualify initiative
    ///
    /// Typically calculated as a percentage of votes cast in the most recent
    /// gubernatorial or presidential election. `None` indicates either the state
    /// has no initiative power, or the threshold varies by measure type.
    ///
    /// Examples:
    /// - California: 5% for statutes, 8% for constitutional amendments
    /// - Arizona: 10% (15% for constitutional amendments)
    /// - Colorado: 5% for both
    pub signature_threshold: Option<f64>,

    /// Notable ballot measures that shaped state policy or national discourse
    ///
    /// Non-exhaustive list of significant initiatives or referenda that:
    /// - Changed fundamental state governance (e.g., CA Prop 13)
    /// - Sparked national movements (e.g., cannabis legalization)
    /// - Tested constitutional boundaries (e.g., CA Prop 8, later struck down)
    /// - Demonstrated power of direct democracy
    pub notable_measures: Vec<String>,
}

/// Complete state constitutional provisions beyond federal floor
///
/// Aggregates all tracked state constitutional features that provide protections
/// or mechanisms beyond what the federal Constitution requires. This demonstrates
/// how states function as "laboratories of democracy" (Justice Brandeis), experimenting
/// with different governance structures and rights protections.
///
/// # Usage
///
/// Use this structure to:
/// - Compare constitutional frameworks across states
/// - Identify states with stronger protections in specific areas
/// - Understand state constitutional variation
/// - Analyze policy innovation mechanisms
///
/// # Example
///
/// ```rust
/// use legalis_us::legislative::constitutional::state_constitutional_provisions;
///
/// let ca = state_constitutional_provisions("CA");
/// // California has explicit privacy right, initiative/referendum, and notable provisions
/// assert!(!ca.beyond_federal_floor.is_empty());
/// assert!(!ca.direct_democracy.notable_measures.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateConstitutionalProvisions {
    /// State identifier
    pub state_id: StateId,

    /// Constitutional privacy right, if any
    ///
    /// Indicates whether the state has an explicit privacy provision, an implicit
    /// judicially-recognized right, or no constitutional privacy protection.
    pub privacy_right: ConstitutionalPrivacyRight,

    /// Direct democracy powers (initiative and referendum)
    ///
    /// Details about citizen initiative and referendum powers, including signature
    /// thresholds and notable ballot measures.
    pub direct_democracy: DirectDemocracyPowers,

    /// Notable state constitutional provisions that exceed federal protections
    ///
    /// Non-exhaustive list of provisions unique to this state or that provide
    /// protections beyond federal constitutional minimums. Examples include:
    /// - Environmental rights (Montana, Pennsylvania, Hawaii)
    /// - Education guarantees (Massachusetts, others)
    /// - Victims' rights (many states)
    /// - Government transparency (Florida sunshine laws)
    /// - Language requirements (English-only provisions)
    ///
    /// This list is illustrative rather than comprehensive. State constitutions
    /// contain hundreds of provisions; this tracks those most significant for
    /// legal and policy analysis.
    pub beyond_federal_floor: Vec<String>,
}

/// Get the constitutional privacy right status for a state
///
/// Returns whether the state has an explicit constitutional privacy provision,
/// an implicit judicially-recognized privacy right, or no constitutional privacy
/// protection.
///
/// # Arguments
///
/// * `state_code` - Two-letter state code (e.g., "CA", "NY", "TX")
///
/// # Returns
///
/// - `ConstitutionalPrivacyRight::Explicit` for 10 states with explicit provisions
///   (AK, AZ, CA, FL, HI, IL, LA, MT, SC, WA)
/// - `ConstitutionalPrivacyRight::Implicit` for 5 states with judicial recognition
///   (NY, MA, PA, NJ, MI)
/// - `ConstitutionalPrivacyRight::None` for all other states
///
/// # Example
///
/// ```
/// use legalis_us::legislative::constitutional::{constitutional_privacy_right, ConstitutionalPrivacyRight};
///
/// // California has explicit privacy right (added 1972)
/// let ca = constitutional_privacy_right("CA");
/// assert!(matches!(ca, ConstitutionalPrivacyRight::Explicit { .. }));
///
/// // New York has implicit privacy right
/// let ny = constitutional_privacy_right("NY");
/// assert!(matches!(ny, ConstitutionalPrivacyRight::Implicit { .. }));
///
/// // Texas has no constitutional privacy right
/// let tx = constitutional_privacy_right("TX");
/// assert_eq!(tx, ConstitutionalPrivacyRight::None);
/// ```
///
/// # Legal Context
///
/// State constitutional privacy rights can provide protection beyond what federal
/// courts recognize under the U.S. Constitution. This has become particularly
/// important post-*Dobbs v. Jackson Women's Health Organization* (2022), which
/// eliminated federal constitutional protection for abortion rights.
pub fn constitutional_privacy_right(state_code: &str) -> ConstitutionalPrivacyRight {
    match state_code {
        // States with explicit constitutional privacy rights (10 states)
        "CA" => ConstitutionalPrivacyRight::Explicit {
            provision: "Art. I, § 1".to_string(),
            year_adopted: 1972, // Prop 11 added privacy to "inalienable rights"
        },
        "FL" => ConstitutionalPrivacyRight::Explicit {
            provision: "Art. I, § 23".to_string(),
            year_adopted: 1980,
        },
        "AK" => ConstitutionalPrivacyRight::Explicit {
            provision: "Art. I, § 22".to_string(),
            year_adopted: 1972,
        },
        "AZ" => ConstitutionalPrivacyRight::Explicit {
            provision: "Art. II, § 8".to_string(),
            year_adopted: 1912,
        },
        "HI" => ConstitutionalPrivacyRight::Explicit {
            provision: "Art. I, § 6".to_string(),
            year_adopted: 1978,
        },
        "IL" => ConstitutionalPrivacyRight::Explicit {
            provision: "Art. I, § 6".to_string(),
            year_adopted: 1970,
        },
        "LA" => ConstitutionalPrivacyRight::Explicit {
            provision: "Art. I, § 5".to_string(),
            year_adopted: 1974,
        },
        "MT" => ConstitutionalPrivacyRight::Explicit {
            provision: "Art. II, § 10".to_string(),
            year_adopted: 1972,
        },
        "SC" => ConstitutionalPrivacyRight::Explicit {
            provision: "Art. I, § 10".to_string(),
            year_adopted: 1971,
        },
        "WA" => ConstitutionalPrivacyRight::Explicit {
            provision: "Art. I, § 7".to_string(),
            year_adopted: 1889, // Original constitution (privacy interpreted broadly)
        },

        // States with implicit privacy rights (recognized by courts)
        "NY" | "MA" | "PA" | "NJ" | "MI" => ConstitutionalPrivacyRight::Implicit {
            key_case: None, // Would require detailed case research per state
        },

        // No recognized constitutional privacy right
        _ => ConstitutionalPrivacyRight::None,
    }
}

/// Check if a state allows citizen initiative or referendum powers
///
/// This function checks specifically for **citizen-initiated** measures, not legislative
/// referral (which all states have). Returns `true` if citizens can propose laws or
/// constitutional amendments by gathering signatures, `false` if only the legislature
/// can refer measures to voters.
///
/// # Arguments
///
/// * `state_code` - Two-letter state code (e.g., "CA", "TX", "OR")
///
/// # Returns
///
/// - `true` for 23 states with citizen initiative power
/// - `false` for 27 states + DC with only legislative referral
///
/// # Example
///
/// ```
/// use legalis_us::legislative::constitutional::has_initiative_referendum;
///
/// // California has citizen initiative (since 1911)
/// assert!(has_initiative_referendum("CA"));
///
/// // Oregon has citizen initiative (since 1902, pioneer state)
/// assert!(has_initiative_referendum("OR"));
///
/// // Texas has only legislative referral (no citizen initiative)
/// assert!(!has_initiative_referendum("TX"));
///
/// // New York has only legislative referral
/// assert!(!has_initiative_referendum("NY"));
/// ```
///
/// # Use Cases
///
/// Use this function to:
/// - Determine if citizens can bypass the legislature to enact laws
/// - Identify states where policy change can occur via ballot measures
/// - Analyze direct democracy availability across states
/// - Assess feasibility of citizen-led policy reforms
///
/// # Historical Context
///
/// The 23 states with citizen initiative adopted these powers primarily during
/// the Progressive Era (1898-1918). They are concentrated in the West and largely
/// absent in the South and East Coast (except Massachusetts and Maine).
pub fn has_initiative_referendum(state_code: &str) -> bool {
    matches!(
        initiative_referendum_status(state_code),
        InitiativeReferendumStatus::Both { .. } | InitiativeReferendumStatus::InitiativeOnly { .. }
    )
}

/// Get detailed initiative and referendum status
fn initiative_referendum_status(state_code: &str) -> InitiativeReferendumStatus {
    match state_code {
        // States with both initiative and referendum (18 states)
        "AK" => InitiativeReferendumStatus::Both {
            year_adopted: 1959,
            constitutional_amendments: true,
            statutes: true,
        },
        "AZ" => InitiativeReferendumStatus::Both {
            year_adopted: 1912,
            constitutional_amendments: true,
            statutes: true,
        },
        "AR" => InitiativeReferendumStatus::Both {
            year_adopted: 1910,
            constitutional_amendments: true,
            statutes: true,
        },
        "CA" => InitiativeReferendumStatus::Both {
            year_adopted: 1911,
            constitutional_amendments: true,
            statutes: true,
        },
        "CO" => InitiativeReferendumStatus::Both {
            year_adopted: 1910,
            constitutional_amendments: true,
            statutes: true,
        },
        "FL" => InitiativeReferendumStatus::Both {
            year_adopted: 1968,
            constitutional_amendments: true,
            statutes: false, // FL has constitutional amendments only
        },
        "ID" => InitiativeReferendumStatus::Both {
            year_adopted: 1912,
            constitutional_amendments: false,
            statutes: true,
        },
        "IL" => InitiativeReferendumStatus::Both {
            year_adopted: 1970,
            constitutional_amendments: true,
            statutes: false, // IL limited to constitutional amendments
        },
        "MA" => InitiativeReferendumStatus::Both {
            year_adopted: 1918,
            constitutional_amendments: true,
            statutes: true,
        },
        "MI" => InitiativeReferendumStatus::Both {
            year_adopted: 1908,
            constitutional_amendments: true,
            statutes: true,
        },
        "MO" => InitiativeReferendumStatus::Both {
            year_adopted: 1908,
            constitutional_amendments: true,
            statutes: true,
        },
        "MT" => InitiativeReferendumStatus::Both {
            year_adopted: 1906,
            constitutional_amendments: true,
            statutes: true,
        },
        "NE" => InitiativeReferendumStatus::Both {
            year_adopted: 1912,
            constitutional_amendments: true,
            statutes: true,
        },
        "NV" => InitiativeReferendumStatus::Both {
            year_adopted: 1904,
            constitutional_amendments: true,
            statutes: true,
        },
        "ND" => InitiativeReferendumStatus::Both {
            year_adopted: 1914,
            constitutional_amendments: true,
            statutes: true,
        },
        "OH" => InitiativeReferendumStatus::Both {
            year_adopted: 1912,
            constitutional_amendments: true,
            statutes: true,
        },
        "OK" => InitiativeReferendumStatus::Both {
            year_adopted: 1907,
            constitutional_amendments: true,
            statutes: true,
        },
        "OR" => InitiativeReferendumStatus::Both {
            year_adopted: 1902,
            constitutional_amendments: true,
            statutes: true,
        },
        "SD" => InitiativeReferendumStatus::Both {
            year_adopted: 1898,
            constitutional_amendments: true,
            statutes: true,
        },
        "UT" => InitiativeReferendumStatus::Both {
            year_adopted: 1900,
            constitutional_amendments: false,
            statutes: true,
        },
        "WA" => InitiativeReferendumStatus::Both {
            year_adopted: 1912,
            constitutional_amendments: false,
            statutes: true,
        },
        "WY" => InitiativeReferendumStatus::Both {
            year_adopted: 1968,
            constitutional_amendments: false,
            statutes: true,
        },

        // Initiative only
        "MS" => InitiativeReferendumStatus::InitiativeOnly {
            year_adopted: 1992,
            constitutional_amendments: true,
            statutes: true,
        },

        // Referendum only (legislative referral - all states have this)
        // Listed here are states without citizen initiative
        "AL" | "CT" | "DE" | "GA" | "HI" | "IN" | "IA" | "KS" | "KY" | "LA" | "ME" | "MD"
        | "MN" | "NH" | "NJ" | "NM" | "NY" | "NC" | "PA" | "RI" | "SC" | "TN" | "TX" | "VT"
        | "VA" | "WV" | "WI" | "DC" => InitiativeReferendumStatus::ReferendumOnly {
            year_adopted: 1789, // All have legislative referral since founding
        },

        _ => InitiativeReferendumStatus::None,
    }
}

/// Get complete state constitutional provisions beyond the federal floor
///
/// Returns a comprehensive view of a state's constitutional features that provide
/// protections or mechanisms beyond what the U.S. Constitution requires. This includes
/// privacy rights, direct democracy powers, and other notable state-specific provisions.
///
/// # Arguments
///
/// * `state_code` - Two-letter state code (e.g., "CA", "MT", "FL")
///
/// # Returns
///
/// A `StateConstitutionalProvisions` structure containing:
/// - Constitutional privacy right status (explicit, implicit, or none)
/// - Direct democracy powers (initiative/referendum status, signature thresholds)
/// - Notable ballot measures that shaped policy
/// - Other provisions exceeding federal constitutional minimums
///
/// # Example
///
/// ```rust
/// use legalis_us::legislative::constitutional::state_constitutional_provisions;
///
/// // California: Extensive beyond-federal-floor provisions
/// let ca = state_constitutional_provisions("CA");
/// // Has explicit privacy right (Art. I, § 1, added 1972)
/// // Has initiative/referendum since 1911
/// // Notable measures include Prop 13, Prop 64, etc.
///
/// // Montana: Strong environmental protections
/// let mt = state_constitutional_provisions("MT");
/// // Has explicit privacy right (Art. II, § 10)
/// // Constitutional right to "clean and healthful environment"
///
/// // Texas: Minimal beyond-federal-floor provisions
/// let tx = state_constitutional_provisions("TX");
/// // No constitutional privacy right
/// // No citizen initiative power
/// ```
///
/// # Use Cases
///
/// This function is useful for:
/// - **Comparative constitutional law**: Compare frameworks across states
/// - **Rights analysis**: Identify states with stronger protections in specific areas
/// - **Policy advocacy**: Determine which states allow citizen-initiated reforms
/// - **Legal research**: Understand state constitutional variation and "adequate and
///   independent state grounds" doctrine
///
/// # Context: States as "Laboratories of Democracy"
///
/// Justice Brandeis famously described states as "laboratories of democracy" where
/// different governance structures and rights protections can be tested. This function
/// illustrates that diversity by showing how state constitutions often provide:
/// - Rights not present federally (privacy, environmental protection, education)
/// - Mechanisms not present federally (citizen initiative, recall)
/// - Broader interpretations of parallel provisions (free speech, search and seizure)
pub fn state_constitutional_provisions(state_code: &str) -> StateConstitutionalProvisions {
    let state_id = StateId::from_code(state_code);
    let privacy_right = constitutional_privacy_right(state_code);
    let status = initiative_referendum_status(state_code);

    let signature_threshold = match state_code {
        "CA" => Some(0.08), // 8% for statutes, 8% for amendments
        "AZ" => Some(0.10), // 10%
        "CO" => Some(0.05), // 5%
        "OR" => Some(0.08), // 8%
        _ => None,
    };

    let notable_measures = match state_code {
        "CA" => vec![
            "Prop 13 (1978) - Property tax limits".to_string(),
            "Prop 187 (1994) - Immigration (struck down)".to_string(),
            "Prop 209 (1996) - Ban on affirmative action".to_string(),
            "Prop 8 (2008) - Ban on same-sex marriage (struck down)".to_string(),
            "Prop 64 (2016) - Recreational cannabis".to_string(),
        ],
        "CO" => vec![
            "Amendment 64 (2012) - Recreational cannabis".to_string(),
            "TABOR (1992) - Taxpayer Bill of Rights".to_string(),
        ],
        "OR" => vec!["Measure 91 (2014) - Recreational cannabis".to_string()],
        _ => vec![],
    };

    let beyond_federal_floor = match state_code {
        "CA" => vec![
            "Privacy as inalienable right (Art. I, § 1)".to_string(),
            "Broader equal protection (Prop 209 notwithstanding)".to_string(),
            "Environmental protection (Art. XI)".to_string(),
        ],
        "FL" => vec![
            "Explicit privacy right (Art. I, § 23)".to_string(),
            "Public records and open meetings (Art. I, § 24)".to_string(),
        ],
        "MT" => vec![
            "Right to clean and healthful environment (Art. II, § 3)".to_string(),
            "Privacy in private affairs (Art. II, § 10)".to_string(),
        ],
        "MA" => vec![
            "Education as duty of commonwealth (Art. V)".to_string(),
            "Free speech protections exceed First Amendment".to_string(),
        ],
        _ => vec![],
    };

    StateConstitutionalProvisions {
        state_id,
        privacy_right,
        direct_democracy: DirectDemocracyPowers {
            status,
            signature_threshold,
            notable_measures,
        },
        beyond_federal_floor,
    }
}

/// Get list of states with explicit constitutional privacy rights
pub fn states_with_explicit_privacy_rights() -> Vec<&'static str> {
    vec!["AK", "AZ", "CA", "FL", "HI", "IL", "LA", "MT", "SC", "WA"]
}

/// Get list of states with initiative and referendum
pub fn states_with_initiative() -> Vec<&'static str> {
    vec![
        "AK", "AZ", "AR", "CA", "CO", "FL", "ID", "IL", "MA", "MI", "MS", "MO", "MT", "NE", "NV",
        "ND", "OH", "OK", "OR", "SD", "UT", "WA", "WY",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explicit_privacy_right_california() {
        let ca = constitutional_privacy_right("CA");
        assert!(matches!(
            ca,
            ConstitutionalPrivacyRight::Explicit {
                year_adopted: 1972,
                ..
            }
        ));
    }

    #[test]
    fn test_explicit_privacy_right_florida() {
        let fl = constitutional_privacy_right("FL");
        assert!(matches!(
            fl,
            ConstitutionalPrivacyRight::Explicit {
                year_adopted: 1980,
                ..
            }
        ));
    }

    #[test]
    fn test_explicit_privacy_right_count() {
        let privacy_states = states_with_explicit_privacy_rights();
        assert_eq!(privacy_states.len(), 10); // 10 states with explicit rights
    }

    #[test]
    fn test_implicit_privacy_right() {
        let ny = constitutional_privacy_right("NY");
        assert!(matches!(ny, ConstitutionalPrivacyRight::Implicit { .. }));
    }

    #[test]
    fn test_no_privacy_right() {
        let al = constitutional_privacy_right("AL");
        assert_eq!(al, ConstitutionalPrivacyRight::None);
    }

    #[test]
    fn test_initiative_referendum_california() {
        assert!(has_initiative_referendum("CA"));
        let status = initiative_referendum_status("CA");
        assert!(matches!(
            status,
            InitiativeReferendumStatus::Both {
                year_adopted: 1911,
                constitutional_amendments: true,
                statutes: true
            }
        ));
    }

    #[test]
    fn test_initiative_referendum_oregon() {
        assert!(has_initiative_referendum("OR"));
        let status = initiative_referendum_status("OR");
        assert!(matches!(
            status,
            InitiativeReferendumStatus::Both {
                year_adopted: 1902,
                ..
            }
        ));
    }

    #[test]
    fn test_no_initiative_texas() {
        assert!(!has_initiative_referendum("TX"));
        let status = initiative_referendum_status("TX");
        assert!(matches!(
            status,
            InitiativeReferendumStatus::ReferendumOnly { .. }
        ));
    }

    #[test]
    fn test_initiative_states_count() {
        let initiative_states = states_with_initiative();
        assert_eq!(initiative_states.len(), 23); // 23 states with initiative
    }

    #[test]
    fn test_state_provisions_california() {
        let ca = state_constitutional_provisions("CA");
        assert!(matches!(
            ca.privacy_right,
            ConstitutionalPrivacyRight::Explicit { .. }
        ));
        assert!(matches!(
            ca.direct_democracy.status,
            InitiativeReferendumStatus::Both { .. }
        ));
        assert!(ca.direct_democracy.signature_threshold.is_some());
        assert!(!ca.beyond_federal_floor.is_empty());
    }

    #[test]
    fn test_state_provisions_florida() {
        let fl = state_constitutional_provisions("FL");
        assert!(matches!(
            fl.privacy_right,
            ConstitutionalPrivacyRight::Explicit { .. }
        ));
        assert!(matches!(
            fl.direct_democracy.status,
            InitiativeReferendumStatus::Both { .. }
        ));
    }

    #[test]
    fn test_state_provisions_texas() {
        let tx = state_constitutional_provisions("TX");
        assert_eq!(tx.privacy_right, ConstitutionalPrivacyRight::None);
        assert!(matches!(
            tx.direct_democracy.status,
            InitiativeReferendumStatus::ReferendumOnly { .. }
        ));
    }

    #[test]
    fn test_california_notable_measures() {
        let ca = state_constitutional_provisions("CA");
        assert!(!ca.direct_democracy.notable_measures.is_empty());
        // Check for Prop 64 (cannabis)
        assert!(
            ca.direct_democracy
                .notable_measures
                .iter()
                .any(|m| m.contains("Prop 64"))
        );
    }
}
