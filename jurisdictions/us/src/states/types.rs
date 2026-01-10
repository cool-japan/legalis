//! Core data types for US state-specific legal variations.
//!
//! This module defines the fundamental types used throughout the US jurisdiction module
//! to represent state-level legal variations, comparisons, and metadata. These types
//! enable systematic comparison of how different states handle the same legal issues,
//! supporting choice-of-law analysis, conflicts resolution, and multi-state practice.
//!
//! # Overview
//!
//! The United States operates under a federal system where each of the 50 states (plus
//! DC and territories) maintains substantial sovereignty over matters not delegated to
//! the federal government. This creates significant **interstate variation** in legal
//! rules, even within a single country sharing a common Constitution.
//!
//! ## Common Law vs. Civil Law
//!
//! **49 States + DC**: Common Law tradition inherited from English law
//! - Precedent-based system (stare decisis)
//! - Judge-made law supplemented by statutes
//! - Case reporters and Restatements as authoritative sources
//!
//! **Louisiana**: Civil Law tradition inherited from French/Spanish colonial period
//! - Code-based system (Louisiana Civil Code, enacted 1808)
//! - Statutes as primary source, cases as interpretive guides
//! - Different terminology (e.g., "obligations" not "contracts", "successions" not "estates")
//!
//! ## Why States Differ
//!
//! ### Historical Reasons
//! - **Regional Variation**: Western states adopted more progressive laws (e.g., no-fault divorce)
//! - **Settlement Patterns**: Different European settlers brought different legal traditions
//! - **Economic Needs**: Agricultural vs. industrial vs. commercial states developed different rules
//!
//! ### Policy Reasons
//! - **Laboratory of Democracy**: States experiment with different legal approaches
//! - **Local Preferences**: State populations have different values and priorities
//! - **Interest Groups**: Varying influence of plaintiff/defense bars, industries, labor
//!
//! ### Legal Doctrine
//! - **Federalism**: Tenth Amendment reserves powers to states
//! - **Police Powers**: States regulate health, safety, morals, general welfare
//! - **State Constitutions**: Can provide greater protections than federal floor
//!
//! # Key Types
//!
//! ## [`StateId`]
//! Identifies a US state with its two-letter code, full name, and legal tradition.
//! Provides conversion to jurisdiction strings compatible with legalis-core.
//!
//! ## [`LegalTradition`]
//! Distinguishes Common Law (49 states + DC) from Civil Law (Louisiana) and rare
//! Mixed traditions. Critical for understanding how legal reasoning differs.
//!
//! ## [`StateLawVariation`]
//! Represents how a specific state handles a particular legal topic, including:
//! - The rule applied (e.g., pure comparative negligence)
//! - Statutory codification (if any)
//! - Case law establishing the rule
//! - Date of adoption
//!
//! ## [`LegalTopic`]
//! Comprehensive enumeration of areas where states differ, covering:
//! - **Tort Law**: 7 topics (comparative negligence, products liability, etc.)
//! - **Contract Law**: 6 topics (statute of frauds, non-compete enforceability, etc.)
//! - **Property Law**: 5 topics (tenancy by entirety, community property, etc.)
//! - **Criminal Law**: 4 topics (death penalty, castle doctrine, marijuana legalization, etc.)
//! - **Procedure**: 5 topics (statute of limitations, discovery rules, etc.)
//!
//! ## [`StateRule`]
//! Specific rule adopted by a state for a given topic. For example, for comparative
//! negligence topic, rules include:
//! - `PureComparativeNegligence`: Plaintiff recovers reduced damages even if 99% at fault
//! - `ModifiedComparativeNegligence50`: Plaintiff barred if 50%+ at fault
//! - `ModifiedComparativeNegligence51`: Plaintiff barred if 51%+ at fault
//! - `ContributoryNegligence`: Plaintiff barred if any fault (minority rule, 4 states)
//!
//! ## Rule Classifications (Majority vs. Minority)
//!
//! Rules are classified as **majority** (followed by most states) or **minority**
//! (followed by few states). This classification helps predict how courts in
//! jurisdictions without clear precedent might rule. The [`states::comparator`](crate::states::comparator)
//! module provides tools to automatically identify majority and minority rules.
//!
//! ## [`CaseReference`] and [`StatuteReference`]
//! Citation metadata for legal sources supporting a state's adoption of a particular rule.
//!
//! # Usage Patterns
//!
//! ## Creating State Identifiers
//! ```rust
//! use legalis_us::states::types::StateId;
//!
//! // Use convenience constructors
//! let ca = StateId::california();
//! let ny = StateId::new_york();
//!
//! // Or from state code
//! let tx = StateId::from_code("TX");
//! assert_eq!(tx.name, "Texas");
//! ```
//!
//! ## Representing State Rules
//! ```rust
//! use legalis_us::states::types::{StateId, StateLawVariation, LegalTopic, StateRule};
//!
//! let ny_negligence = StateLawVariation::new(
//!     StateId::new_york(),
//!     LegalTopic::ComparativeNegligence,
//!     StateRule::PureComparativeNegligence,
//! )
//! .with_notes("Adopted in 1975, replacing contributory negligence");
//! ```
//!
//! ## Comparing Across States
//! Use [`states::comparator`](crate::states::comparator) module to identify majority/minority
//! rules and measure similarity between states' legal regimes.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// State identifier with metadata.
///
/// Represents a US state with its legal system classification and basic information.
/// This type serves as the primary identifier for jurisdictions throughout the
/// legalis-us library and provides interoperability with legalis-core through
/// jurisdiction strings.
///
/// # State Codes
///
/// Uses standard two-letter postal abbreviations (USPS codes) as defined by
/// ISO 3166-2:US. These are the same codes used for:
/// - Postal addresses (e.g., "CA" for California mailing addresses)
/// - Vehicle registration plates
/// - Federal government databases (Census Bureau, etc.)
///
/// ## Special Codes
/// - **DC**: District of Columbia (federal district, not a state)
/// - **US**: United States federal jurisdiction (rare, for federal-level rules)
///
/// # Legal Tradition
///
/// Every state is classified by its legal tradition:
/// - **Common Law (50 jurisdictions)**: All states except Louisiana, plus DC
/// - **Civil Law (1 jurisdiction)**: Louisiana only
/// - **Mixed**: Unused in current implementation but reserved for territories
///
/// Louisiana's Civil Law heritage from French/Spanish colonization (1699-1803)
/// makes it unique. The Louisiana Purchase (1803) transferred sovereignty to
/// the US, but Louisiana retained its Civil Law system, codified in 1808.
///
/// # Usage
///
/// ```rust
/// use legalis_us::states::types::StateId;
///
/// // Convenience constructors for major states
/// let ca = StateId::california();
/// let ny = StateId::new_york();
/// let la = StateId::louisiana(); // Only Civil Law state
///
/// // From any state code
/// let tx = StateId::from_code("TX");
/// assert_eq!(tx.code, "TX");
/// assert_eq!(tx.name, "Texas");
///
/// // Convert to jurisdiction string for legalis-core
/// let jurisdiction = ca.jurisdiction_string();
/// assert_eq!(jurisdiction, "US-CA");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StateId {
    /// Two-letter state code (e.g., "CA", "NY", "TX")
    ///
    /// Uses ISO 3166-2:US / USPS postal abbreviations.
    /// Always uppercase in canonical form.
    pub code: String,

    /// Full state name (e.g., "California", "New York")
    ///
    /// Official state name as recognized by the US Census Bureau.
    /// For DC, "District of Columbia".
    pub name: String,

    /// Legal tradition classification
    ///
    /// Distinguishes Common Law (49 states + DC) from Civil Law (Louisiana).
    /// Critical for understanding how legal reasoning and sources differ.
    pub legal_tradition: LegalTradition,
}

impl StateId {
    /// Create a new StateId.
    #[must_use]
    pub fn new(
        code: impl Into<String>,
        name: impl Into<String>,
        tradition: LegalTradition,
    ) -> Self {
        Self {
            code: code.into(),
            name: name.into(),
            legal_tradition: tradition,
        }
    }

    /// Create California state ID.
    #[must_use]
    pub fn california() -> Self {
        Self::new("CA", "California", LegalTradition::CommonLaw)
    }

    /// Create New York state ID.
    #[must_use]
    pub fn new_york() -> Self {
        Self::new("NY", "New York", LegalTradition::CommonLaw)
    }

    /// Create Texas state ID.
    #[must_use]
    pub fn texas() -> Self {
        Self::new("TX", "Texas", LegalTradition::CommonLaw)
    }

    /// Create Louisiana state ID (only Civil Law state).
    #[must_use]
    pub fn louisiana() -> Self {
        Self::new("LA", "Louisiana", LegalTradition::CivilLaw)
    }

    /// Create Florida state ID.
    #[must_use]
    pub fn florida() -> Self {
        Self::new("FL", "Florida", LegalTradition::CommonLaw)
    }

    /// Get jurisdiction string for use with legalis-core types.
    ///
    /// Returns format "US-{CODE}" (e.g., "US-CA", "US-NY").
    #[must_use]
    pub fn jurisdiction_string(&self) -> String {
        format!("US-{}", self.code)
    }

    /// Create a StateId from a state code string.
    ///
    /// This is a convenience function that looks up the state name and legal tradition
    /// based on the two-letter state code.
    ///
    /// # Arguments
    /// * `code` - Two-letter state code (case insensitive)
    ///
    /// # Returns
    /// StateId with proper name and legal tradition, or a generic StateId if code is unknown
    ///
    /// # Example
    /// ```
    /// use legalis_us::states::types::StateId;
    /// let tx = StateId::from_code("TX");
    /// assert_eq!(tx.code, "TX");
    /// assert_eq!(tx.name, "Texas");
    /// ```
    #[must_use]
    pub fn from_code(code: impl AsRef<str>) -> Self {
        let code_upper = code.as_ref().to_uppercase();
        match code_upper.as_str() {
            "AL" => Self::new("AL", "Alabama", LegalTradition::CommonLaw),
            "AK" => Self::new("AK", "Alaska", LegalTradition::CommonLaw),
            "AZ" => Self::new("AZ", "Arizona", LegalTradition::CommonLaw),
            "AR" => Self::new("AR", "Arkansas", LegalTradition::CommonLaw),
            "CA" => Self::california(),
            "CO" => Self::new("CO", "Colorado", LegalTradition::CommonLaw),
            "CT" => Self::new("CT", "Connecticut", LegalTradition::CommonLaw),
            "DE" => Self::new("DE", "Delaware", LegalTradition::CommonLaw),
            "FL" => Self::florida(),
            "GA" => Self::new("GA", "Georgia", LegalTradition::CommonLaw),
            "HI" => Self::new("HI", "Hawaii", LegalTradition::CommonLaw),
            "ID" => Self::new("ID", "Idaho", LegalTradition::CommonLaw),
            "IL" => Self::new("IL", "Illinois", LegalTradition::CommonLaw),
            "IN" => Self::new("IN", "Indiana", LegalTradition::CommonLaw),
            "IA" => Self::new("IA", "Iowa", LegalTradition::CommonLaw),
            "KS" => Self::new("KS", "Kansas", LegalTradition::CommonLaw),
            "KY" => Self::new("KY", "Kentucky", LegalTradition::CommonLaw),
            "LA" => Self::louisiana(),
            "ME" => Self::new("ME", "Maine", LegalTradition::CommonLaw),
            "MD" => Self::new("MD", "Maryland", LegalTradition::CommonLaw),
            "MA" => Self::new("MA", "Massachusetts", LegalTradition::CommonLaw),
            "MI" => Self::new("MI", "Michigan", LegalTradition::CommonLaw),
            "MN" => Self::new("MN", "Minnesota", LegalTradition::CommonLaw),
            "MS" => Self::new("MS", "Mississippi", LegalTradition::CommonLaw),
            "MO" => Self::new("MO", "Missouri", LegalTradition::CommonLaw),
            "MT" => Self::new("MT", "Montana", LegalTradition::CommonLaw),
            "NE" => Self::new("NE", "Nebraska", LegalTradition::CommonLaw),
            "NV" => Self::new("NV", "Nevada", LegalTradition::CommonLaw),
            "NH" => Self::new("NH", "New Hampshire", LegalTradition::CommonLaw),
            "NJ" => Self::new("NJ", "New Jersey", LegalTradition::CommonLaw),
            "NM" => Self::new("NM", "New Mexico", LegalTradition::CommonLaw),
            "NY" => Self::new_york(),
            "NC" => Self::new("NC", "North Carolina", LegalTradition::CommonLaw),
            "ND" => Self::new("ND", "North Dakota", LegalTradition::CommonLaw),
            "OH" => Self::new("OH", "Ohio", LegalTradition::CommonLaw),
            "OK" => Self::new("OK", "Oklahoma", LegalTradition::CommonLaw),
            "OR" => Self::new("OR", "Oregon", LegalTradition::CommonLaw),
            "PA" => Self::new("PA", "Pennsylvania", LegalTradition::CommonLaw),
            "RI" => Self::new("RI", "Rhode Island", LegalTradition::CommonLaw),
            "SC" => Self::new("SC", "South Carolina", LegalTradition::CommonLaw),
            "SD" => Self::new("SD", "South Dakota", LegalTradition::CommonLaw),
            "TN" => Self::new("TN", "Tennessee", LegalTradition::CommonLaw),
            "TX" => Self::texas(),
            "UT" => Self::new("UT", "Utah", LegalTradition::CommonLaw),
            "VT" => Self::new("VT", "Vermont", LegalTradition::CommonLaw),
            "VA" => Self::new("VA", "Virginia", LegalTradition::CommonLaw),
            "WA" => Self::new("WA", "Washington", LegalTradition::CommonLaw),
            "WV" => Self::new("WV", "West Virginia", LegalTradition::CommonLaw),
            "WI" => Self::new("WI", "Wisconsin", LegalTradition::CommonLaw),
            "WY" => Self::new("WY", "Wyoming", LegalTradition::CommonLaw),
            "DC" => Self::new("DC", "District of Columbia", LegalTradition::CommonLaw),
            "US" => Self::new("US", "United States", LegalTradition::CommonLaw),
            _ => Self::new(
                code_upper.clone(),
                code_upper.clone(),
                LegalTradition::CommonLaw,
            ),
        }
    }
}

impl fmt::Display for StateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}

/// Legal tradition classification for US jurisdictions.
///
/// While most US states follow Common Law inherited from English law,
/// Louisiana uniquely follows Civil Law tradition derived from French and Spanish law.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalTradition {
    /// Common Law tradition (49 states)
    ///
    /// Based on English Common Law with judge-made precedents (stare decisis).
    /// Source of law: Cases and statutes.
    CommonLaw,

    /// Civil Law tradition (Louisiana only)
    ///
    /// Based on French Civil Code and Spanish law.
    /// Source of law: Codes and statutes, with less emphasis on precedent.
    CivilLaw,

    /// Mixed legal tradition
    ///
    /// Historical jurisdictions with elements of both traditions.
    /// Rarely used in modern US state classification.
    Mixed,
}

impl fmt::Display for LegalTradition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommonLaw => write!(f, "Common Law"),
            Self::CivilLaw => write!(f, "Civil Law"),
            Self::Mixed => write!(f, "Mixed Legal Tradition"),
        }
    }
}

/// State-specific variation on a legal topic.
///
/// Represents how a particular state handles a specific area of law,
/// including the rule applied, statutory/case basis, and adoption timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateLawVariation {
    /// State applying this variation
    pub state: StateId,

    /// Legal topic this variation addresses
    pub topic: LegalTopic,

    /// Specific rule applied by this state
    pub rule: StateRule,

    /// Statutory basis (if codified)
    pub statutory_basis: Option<StatuteReference>,

    /// Case law basis (landmark cases establishing this rule)
    pub case_basis: Vec<CaseReference>,

    /// Date this rule was adopted (if known)
    pub adoption_date: Option<NaiveDate>,

    /// Additional notes or context
    pub notes: String,
}

impl StateLawVariation {
    /// Create a new state law variation.
    #[must_use]
    pub fn new(state: StateId, topic: LegalTopic, rule: StateRule) -> Self {
        Self {
            state,
            topic,
            rule,
            statutory_basis: None,
            case_basis: Vec::new(),
            adoption_date: None,
            notes: String::new(),
        }
    }

    /// Add statutory basis.
    #[must_use]
    pub fn with_statute(mut self, statute: StatuteReference) -> Self {
        self.statutory_basis = Some(statute);
        self
    }

    /// Add case law basis.
    #[must_use]
    pub fn with_case(mut self, case: CaseReference) -> Self {
        self.case_basis.push(case);
        self
    }

    /// Add adoption date.
    #[must_use]
    pub fn with_adoption_date(mut self, date: NaiveDate) -> Self {
        self.adoption_date = Some(date);
        self
    }

    /// Add notes.
    #[must_use]
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = notes.into();
        self
    }
}

/// Legal topics for cross-state comparison.
///
/// Represents specific areas of law where states may have different approaches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalTopic {
    // ===== Tort Law =====
    /// Comparative vs contributory negligence rules
    ComparativeNegligence,

    /// Dram shop liability (alcohol vendor liability)
    DramShopLiability,

    /// Good Samaritan protection (emergency medical assistance)
    GoodSamaritanProtection,

    /// Products liability standards
    ProductsLiability,

    /// Emotional distress claims
    EmotionalDistress,

    /// Punitive damages availability and caps
    PunitiveDamages,

    /// Medical malpractice rules
    MedicalMalpractice,

    // ===== Contract Law =====
    /// Statute of Frauds requirements
    StatuteOfFrauds,

    /// Parol evidence rule
    ParolEvidenceRule,

    /// Non-compete agreement enforceability
    NonCompeteAgreements,

    // ===== Property Law =====
    /// Adverse possession requirements
    AdversePossession,

    /// Community property vs separate property
    MaritalProperty,

    // ===== Procedure =====
    /// Statute of limitations (with cause of action parameter)
    StatuteOfLimitations(CauseOfAction),

    /// Damage caps by claim type
    DamagesCaps,

    /// Joint and several liability
    JointAndSeveralLiability,

    // ===== Criminal Law =====
    /// Self-defense standards (e.g., Stand Your Ground)
    SelfDefense,

    /// Castle Doctrine
    CastleDoctrine,

    // ===== Other =====
    /// Choice of law methodology
    ChoiceOfLawApproach,
}

impl fmt::Display for LegalTopic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComparativeNegligence => write!(f, "Comparative/Contributory Negligence"),
            Self::DramShopLiability => write!(f, "Dram Shop Liability"),
            Self::GoodSamaritanProtection => write!(f, "Good Samaritan Protection"),
            Self::ProductsLiability => write!(f, "Products Liability"),
            Self::EmotionalDistress => write!(f, "Emotional Distress"),
            Self::PunitiveDamages => write!(f, "Punitive Damages"),
            Self::MedicalMalpractice => write!(f, "Medical Malpractice"),
            Self::StatuteOfFrauds => write!(f, "Statute of Frauds"),
            Self::ParolEvidenceRule => write!(f, "Parol Evidence Rule"),
            Self::NonCompeteAgreements => write!(f, "Non-Compete Agreements"),
            Self::AdversePossession => write!(f, "Adverse Possession"),
            Self::MaritalProperty => write!(f, "Marital Property"),
            Self::StatuteOfLimitations(cause) => write!(f, "Statute of Limitations ({})", cause),
            Self::DamagesCaps => write!(f, "Damages Caps"),
            Self::JointAndSeveralLiability => write!(f, "Joint and Several Liability"),
            Self::SelfDefense => write!(f, "Self-Defense"),
            Self::CastleDoctrine => write!(f, "Castle Doctrine"),
            Self::ChoiceOfLawApproach => write!(f, "Choice of Law Approach"),
        }
    }
}

/// Cause of action for statute of limitations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CauseOfAction {
    /// Personal injury torts
    PersonalInjury,

    /// Property damage
    PropertyDamage,

    /// Breach of contract (written)
    ContractWritten,

    /// Breach of contract (oral)
    ContractOral,

    /// Fraud
    Fraud,

    /// Medical malpractice
    MedicalMalpractice,

    /// Products liability
    ProductsLiability,
}

impl fmt::Display for CauseOfAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PersonalInjury => write!(f, "Personal Injury"),
            Self::PropertyDamage => write!(f, "Property Damage"),
            Self::ContractWritten => write!(f, "Written Contract"),
            Self::ContractOral => write!(f, "Oral Contract"),
            Self::Fraud => write!(f, "Fraud"),
            Self::MedicalMalpractice => write!(f, "Medical Malpractice"),
            Self::ProductsLiability => write!(f, "Products Liability"),
        }
    }
}

/// State-specific legal rule variation.
///
/// Represents different approaches states take on specific legal issues.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StateRule {
    // ===== Negligence Rules =====
    /// Pure comparative negligence (CA, NY, FL)
    ///
    /// Damages reduced by plaintiff's percentage of fault, no bar.
    /// Plaintiff 90% at fault can still recover 10%.
    PureComparativeNegligence,

    /// Modified comparative negligence - 50% bar (12 states)
    ///
    /// Plaintiff can recover if ≤50% at fault.
    ModifiedComparative50,

    /// Modified comparative negligence - 51% bar (TX, 21 states)
    ///
    /// Plaintiff can recover if <51% at fault.
    ModifiedComparative51,

    /// Contributory negligence (NC, VA, MD, DC, AL)
    ///
    /// Complete bar to recovery if plaintiff has any fault.
    ContributoryNegligence,

    // ===== Damages Rules =====
    /// Damages cap with specific amount
    DamagesCap {
        /// Type of damages capped
        damage_type: DamagesType,

        /// Cap amount in USD
        cap_amount: u64,

        /// Additional conditions or exceptions
        conditions: Vec<String>,
    },

    /// No damages cap
    NoDamagesCap,

    // ===== Joint Liability =====
    /// Pure joint and several liability
    JointAndSeveralLiability,

    /// Several liability only (proportionate share)
    SeveralLiabilityOnly,

    /// Modified joint and several (hybrid)
    ModifiedJointAndSeveral {
        /// Threshold percentage for joint liability
        threshold_percent: u8,
    },

    // ===== Custom Rules =====
    /// Custom state-specific rule not fitting standard categories
    Custom {
        /// Rule name
        name: String,

        /// Rule description
        description: String,

        /// Parameters specific to this rule
        parameters: HashMap<String, String>,
    },
}

impl fmt::Display for StateRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PureComparativeNegligence => write!(f, "Pure Comparative Negligence"),
            Self::ModifiedComparative50 => write!(f, "Modified Comparative (50% bar)"),
            Self::ModifiedComparative51 => write!(f, "Modified Comparative (51% bar)"),
            Self::ContributoryNegligence => write!(f, "Contributory Negligence"),
            Self::DamagesCap {
                damage_type,
                cap_amount,
                ..
            } => {
                write!(f, "{} Cap: ${}", damage_type, cap_amount)
            }
            Self::NoDamagesCap => write!(f, "No Damages Cap"),
            Self::JointAndSeveralLiability => write!(f, "Joint and Several Liability"),
            Self::SeveralLiabilityOnly => write!(f, "Several Liability Only"),
            Self::ModifiedJointAndSeveral { threshold_percent } => {
                write!(
                    f,
                    "Modified Joint and Several ({}% threshold)",
                    threshold_percent
                )
            }
            Self::Custom { name, .. } => write!(f, "{}", name),
        }
    }
}

/// Type of damages for cap purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamagesType {
    /// Economic damages (medical bills, lost wages)
    Economic,

    /// Non-economic damages (pain and suffering)
    NonEconomic,

    /// Punitive damages
    Punitive,

    /// Total damages (all types combined)
    Total,
}

impl fmt::Display for DamagesType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Economic => write!(f, "Economic Damages"),
            Self::NonEconomic => write!(f, "Non-Economic Damages"),
            Self::Punitive => write!(f, "Punitive Damages"),
            Self::Total => write!(f, "Total Damages"),
        }
    }
}

/// Reference to a statute or code section.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatuteReference {
    /// Citation (e.g., "Cal. Civ. Code § 1714", "Tex. Civ. Prac. & Rem. Code § 33.001")
    pub citation: String,

    /// Short title or description
    pub title: Option<String>,

    /// Year enacted or last amended
    pub year: Option<u32>,
}

impl StatuteReference {
    /// Create a new statute reference.
    #[must_use]
    pub fn new(citation: impl Into<String>) -> Self {
        Self {
            citation: citation.into(),
            title: None,
            year: None,
        }
    }

    /// Add title.
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add year.
    #[must_use]
    pub fn with_year(mut self, year: u32) -> Self {
        self.year = Some(year);
        self
    }
}

impl fmt::Display for StatuteReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.citation)?;
        if let Some(ref title) = self.title {
            write!(f, " ({})", title)?;
        }
        Ok(())
    }
}

/// Reference to a case.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CaseReference {
    /// Case citation (e.g., "Li v. Yellow Cab Co., 13 Cal.3d 804 (1975)")
    pub citation: String,

    /// Short name (e.g., "Li v. Yellow Cab")
    pub short_name: String,

    /// Year decided
    pub year: u32,

    /// Brief holding or significance
    pub significance: Option<String>,
}

impl CaseReference {
    /// Create a new case reference.
    #[must_use]
    pub fn new(citation: impl Into<String>, short_name: impl Into<String>, year: u32) -> Self {
        Self {
            citation: citation.into(),
            short_name: short_name.into(),
            year,
            significance: None,
        }
    }

    /// Add significance note.
    #[must_use]
    pub fn with_significance(mut self, significance: impl Into<String>) -> Self {
        self.significance = Some(significance.into());
        self
    }
}

impl fmt::Display for CaseReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.short_name, self.year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_id_creation() {
        let ca = StateId::california();
        assert_eq!(ca.code, "CA");
        assert_eq!(ca.name, "California");
        assert_eq!(ca.legal_tradition, LegalTradition::CommonLaw);
        assert_eq!(ca.jurisdiction_string(), "US-CA");
    }

    #[test]
    fn test_louisiana_civil_law() {
        let la = StateId::louisiana();
        assert_eq!(la.legal_tradition, LegalTradition::CivilLaw);
        assert_eq!(la.jurisdiction_string(), "US-LA");
    }

    #[test]
    fn test_state_law_variation_builder() {
        let variation = StateLawVariation::new(
            StateId::california(),
            LegalTopic::ComparativeNegligence,
            StateRule::PureComparativeNegligence,
        )
        .with_statute(StatuteReference::new("Cal. Civ. Code § 1714"))
        .with_case(CaseReference::new(
            "Li v. Yellow Cab Co., 13 Cal.3d 804",
            "Li v. Yellow Cab",
            1975,
        ))
        .with_notes("Adopted in 1975, replacing contributory negligence");

        assert_eq!(variation.state.code, "CA");
        assert_eq!(variation.case_basis.len(), 1);
        assert!(variation.statutory_basis.is_some());
    }

    #[test]
    fn test_legal_topic_display() {
        let topic = LegalTopic::ComparativeNegligence;
        assert!(topic.to_string().contains("Negligence"));

        let topic_with_param = LegalTopic::StatuteOfLimitations(CauseOfAction::PersonalInjury);
        assert!(topic_with_param.to_string().contains("Personal Injury"));
    }

    #[test]
    fn test_state_rule_variants() {
        let pure = StateRule::PureComparativeNegligence;
        assert_eq!(pure.to_string(), "Pure Comparative Negligence");

        let modified = StateRule::ModifiedComparative51;
        assert!(modified.to_string().contains("51%"));

        let cap = StateRule::DamagesCap {
            damage_type: DamagesType::NonEconomic,
            cap_amount: 250_000,
            conditions: vec!["Medical malpractice only".to_string()],
        };
        assert!(cap.to_string().contains("250000"));
    }

    #[test]
    fn test_statute_reference_builder() {
        let statute = StatuteReference::new("Cal. Civ. Code § 1714")
            .with_title("Duty of Care")
            .with_year(1872);

        assert_eq!(statute.citation, "Cal. Civ. Code § 1714");
        assert_eq!(statute.year, Some(1872));
    }

    #[test]
    fn test_case_reference() {
        let case = CaseReference::new(
            "Li v. Yellow Cab Co., 13 Cal.3d 804 (1975)",
            "Li v. Yellow Cab",
            1975,
        )
        .with_significance("Established pure comparative negligence in California");

        assert_eq!(case.year, 1975);
        assert!(case.significance.is_some());
    }
}
