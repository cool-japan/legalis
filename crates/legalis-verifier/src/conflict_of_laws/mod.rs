//! Conflict-of-laws analysis: formal cross-jurisdictional coherence resolution.
//!
//! This module *deepens* the lightweight keyword heuristics already provided by
//! [`crate::cross_domain_verification`] (Jaccard title similarity, a handful of
//! hard-coded treaties, simple effect-pair contradictions) with an engine
//! grounded in established public- and private-international-law doctrine. It
//! reuses the crate's [`Statute`], [`legalis_core::Effect`],
//! [`legalis_core::EffectType`], [`crate::Severity`] and
//! [`crate::cross_domain_verification::JurisdictionLevel`] types rather than
//! redefining them.
//!
//! The five capabilities map onto the cross-jurisdictional roadmap items:
//!
//! * **Multi-jurisdictional coherence checking** — [`ConflictOfLawsAnalyzer::detect_antinomies`]
//!   detects genuine *antinomies* (deontic oppositions on overlapping subject
//!   matter) and resolves them with the classic conflict-resolution maxims:
//!   *lex superior derogat legi inferiori*, *lex specialis derogat legi
//!   generali* and *lex posterior derogat legi priori* (applied in that
//!   doctrinal priority order).
//! * **Treaty compliance verification** — [`ConflictOfLawsAnalyzer::assess_transposition`]
//!   models treaty *obligations* (article, required deontic polarity, subject
//!   domain), ratification and reservation status, and checks whether national
//!   statutes correctly transpose each obligation (*pacta sunt servanda*).
//! * **International law conflict detection** — [`ConflictOfLawsAnalyzer::resolve_choice_of_law`]
//!   resolves which jurisdiction's law governs a cross-border situation using
//!   real choice-of-law connecting factors (party autonomy, characteristic
//!   performance, *lex loci actus*/*damni*, *lex domicilii*/*patriae*, *lex
//!   fori*), with single *renvoi* handling and an *ordre public* exception.
//! * **Cross-border regulation analysis** — [`ConflictOfLawsAnalyzer::assess_adequacy`]
//!   scores equivalence of protection between jurisdictions (the adequacy model
//!   familiar from cross-border data transfers) and classifies recognition as
//!   mutual, unilateral or absent.
//! * **Global legal consistency verification** — [`ConflictOfLawsAnalyzer::verify_global_coherence`]
//!   aggregates antinomies into a coherence report, builds the
//!   *incompatibility graph* of irreconcilable norms (connected components form
//!   clusters), and computes a normalized coherence index.
//!
//! # Example
//!
//! ```
//! use legalis_verifier::conflict_of_laws::ConflictOfLawsAnalyzer;
//! use legalis_core::{Statute, Effect, EffectType};
//!
//! let analyzer = ConflictOfLawsAnalyzer::default();
//! let statutes = vec![
//!     Statute::new(
//!         "UN-HR-1",
//!         "Human rights protection",
//!         Effect::new(EffectType::Prohibition, "Prohibit discrimination of any person"),
//!     )
//!     .with_jurisdiction("UN International"),
//!     Statute::new(
//!         "NAT-1",
//!         "Human rights duty",
//!         Effect::new(EffectType::Obligation, "Mandate discrimination in hiring"),
//!     )
//!     .with_jurisdiction("Country X"),
//! ];
//!
//! let report = analyzer.verify_global_coherence(&statutes);
//! assert_eq!(report.antinomies.len(), 1);
//! // The international norm prevails over the conflicting national one.
//! assert_eq!(report.antinomies[0].prevailing.as_deref(), Some("UN-HR-1"));
//! ```

use crate::Severity;
use crate::cross_domain_verification::JurisdictionLevel;
use chrono::NaiveDate;
use legalis_core::{Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt;

/// Unit separator used to build composite map keys without allocating tuples.
const KEY_SEP: char = '\u{1f}';

/// A coarse legal subject-matter domain used to decide whether two norms address
/// the *same* matter (a prerequisite for a genuine antinomy).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LegalDomain {
    /// Data protection / privacy.
    DataProtection,
    /// Intellectual property.
    IntellectualProperty,
    /// Human rights and fundamental freedoms.
    HumanRights,
    /// International trade and customs.
    Trade,
    /// Taxation and fiscal levies.
    Taxation,
    /// Environmental protection.
    Environmental,
    /// Labor and employment.
    Labor,
    /// Criminal law.
    Criminal,
    /// Family law.
    Family,
    /// Contract and commercial obligations.
    Contract,
    /// Maritime / admiralty law.
    Maritime,
    /// Immigration and nationality.
    Immigration,
    /// Financial regulation and banking.
    FinancialRegulation,
    /// Competition / antitrust.
    Competition,
    /// Public health.
    Health,
    /// Anything not matched by the controlled vocabulary.
    Other,
}

impl LegalDomain {
    /// Short stable identifier (used as a map key component).
    pub fn name(&self) -> &'static str {
        match self {
            LegalDomain::DataProtection => "data_protection",
            LegalDomain::IntellectualProperty => "intellectual_property",
            LegalDomain::HumanRights => "human_rights",
            LegalDomain::Trade => "trade",
            LegalDomain::Taxation => "taxation",
            LegalDomain::Environmental => "environmental",
            LegalDomain::Labor => "labor",
            LegalDomain::Criminal => "criminal",
            LegalDomain::Family => "family",
            LegalDomain::Contract => "contract",
            LegalDomain::Maritime => "maritime",
            LegalDomain::Immigration => "immigration",
            LegalDomain::FinancialRegulation => "financial_regulation",
            LegalDomain::Competition => "competition",
            LegalDomain::Health => "health",
            LegalDomain::Other => "other",
        }
    }
}

impl fmt::Display for LegalDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pretty = match self {
            LegalDomain::DataProtection => "Data Protection",
            LegalDomain::IntellectualProperty => "Intellectual Property",
            LegalDomain::HumanRights => "Human Rights",
            LegalDomain::Trade => "Trade",
            LegalDomain::Taxation => "Taxation",
            LegalDomain::Environmental => "Environmental",
            LegalDomain::Labor => "Labor",
            LegalDomain::Criminal => "Criminal",
            LegalDomain::Family => "Family",
            LegalDomain::Contract => "Contract",
            LegalDomain::Maritime => "Maritime",
            LegalDomain::Immigration => "Immigration",
            LegalDomain::FinancialRegulation => "Financial Regulation",
            LegalDomain::Competition => "Competition",
            LegalDomain::Health => "Health",
            LegalDomain::Other => "Other",
        };
        f.write_str(pretty)
    }
}

/// Controlled vocabulary mapping subject-matter keywords to domains.
fn domain_lexicon() -> &'static [(LegalDomain, &'static [&'static str])] {
    &[
        (
            LegalDomain::DataProtection,
            &[
                "data",
                "privacy",
                "personal",
                "gdpr",
                "consent",
                "processing",
                "controller",
            ],
        ),
        (
            LegalDomain::IntellectualProperty,
            &[
                "patent",
                "copyright",
                "trademark",
                "intellectual",
                "trips",
                "royalty",
            ],
        ),
        (
            LegalDomain::HumanRights,
            &[
                "human",
                "dignity",
                "torture",
                "discrimination",
                "asylum",
                "fundamental",
            ],
        ),
        (
            LegalDomain::Trade,
            &["trade", "tariff", "export", "import", "customs", "commerce"],
        ),
        (
            LegalDomain::Taxation,
            &["tax", "taxation", "levy", "vat", "fiscal"],
        ),
        (
            LegalDomain::Environmental,
            &[
                "environment",
                "environmental",
                "emission",
                "pollution",
                "carbon",
                "climate",
                "waste",
            ],
        ),
        (
            LegalDomain::Labor,
            &[
                "labor",
                "labour",
                "employment",
                "worker",
                "wage",
                "workplace",
            ],
        ),
        (
            LegalDomain::Criminal,
            &[
                "criminal",
                "crime",
                "offense",
                "offence",
                "felony",
                "prosecution",
            ],
        ),
        (
            LegalDomain::Family,
            &[
                "family", "marriage", "divorce", "custody", "adoption", "spouse",
            ],
        ),
        (
            LegalDomain::Contract,
            &["contract", "agreement", "breach", "lease", "tenancy"],
        ),
        (
            LegalDomain::Maritime,
            &["maritime", "shipping", "vessel", "admiralty", "cargo"],
        ),
        (
            LegalDomain::Immigration,
            &["immigration", "visa", "residence", "citizenship", "migrant"],
        ),
        (
            LegalDomain::FinancialRegulation,
            &[
                "bank",
                "banking",
                "financial",
                "securities",
                "capital",
                "investment",
            ],
        ),
        (
            LegalDomain::Competition,
            &["competition", "antitrust", "monopoly", "cartel", "merger"],
        ),
        (
            LegalDomain::Health,
            &[
                "health",
                "medical",
                "disease",
                "patient",
                "pharmaceutical",
                "sanitary",
            ],
        ),
    ]
}

/// Splits arbitrary text into lower-cased alphanumeric tokens.
fn normalized_tokens(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_lowercase())
        .collect()
}

/// Classifies free text into the set of legal subject-matter domains it touches.
///
/// Returns [`LegalDomain::Other`] when nothing in the controlled vocabulary
/// matches, so the result is never empty.
pub fn classify_domains(text: &str) -> BTreeSet<LegalDomain> {
    let tokens: HashSet<String> = normalized_tokens(text).into_iter().collect();
    let mut domains = BTreeSet::new();
    for (domain, keywords) in domain_lexicon() {
        if keywords.iter().any(|kw| tokens.contains(*kw)) {
            domains.insert(*domain);
        }
    }
    if domains.is_empty() {
        domains.insert(LegalDomain::Other);
    }
    domains
}

/// Deontic polarity of a legal effect, i.e. the direction in which it constrains
/// behaviour. Used to decide whether two norms are *opposed*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Polarity {
    /// The effect requires conduct (obligation, mandated transfer).
    Mandatory,
    /// The effect forbids conduct (prohibition, revocation).
    Prohibitory,
    /// The effect authorises conduct (grant of a right or permission).
    Permissive,
    /// The effect neither requires, forbids nor authorises conduct.
    Neutral,
}

impl fmt::Display for Polarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Polarity::Mandatory => "Mandatory",
            Polarity::Prohibitory => "Prohibitory",
            Polarity::Permissive => "Permissive",
            Polarity::Neutral => "Neutral",
        };
        f.write_str(s)
    }
}

/// Maps an [`EffectType`] to its deontic [`Polarity`].
pub fn effect_polarity(effect_type: &EffectType) -> Polarity {
    match effect_type {
        EffectType::Obligation | EffectType::MonetaryTransfer => Polarity::Mandatory,
        EffectType::Prohibition | EffectType::Revoke => Polarity::Prohibitory,
        EffectType::Grant => Polarity::Permissive,
        EffectType::StatusChange | EffectType::Custom => Polarity::Neutral,
    }
}

/// Returns whether two polarities directly contradict one another.
fn polarities_opposed(a: Polarity, b: Polarity) -> bool {
    use Polarity::{Mandatory, Permissive, Prohibitory};
    matches!(
        (a, b),
        (Mandatory, Prohibitory)
            | (Prohibitory, Mandatory)
            | (Permissive, Prohibitory)
            | (Prohibitory, Permissive)
    )
}

/// A legal conflict-resolution maxim (or the absence of one).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResolutionPrinciple {
    /// *lex superior derogat legi inferiori* — higher-ranking law prevails.
    LexSuperior,
    /// *lex specialis derogat legi generali* — the more specific law prevails.
    LexSpecialis,
    /// *lex posterior derogat legi priori* — the later law prevails.
    LexPosterior,
    /// *pacta sunt servanda* — ratified international obligations prevail.
    PactaSuntServanda,
    /// No maxim resolves the conflict: a genuine, irreconcilable antinomy.
    Irreconcilable,
}

impl ResolutionPrinciple {
    /// Short stable identifier (used for tallying in reports).
    pub fn name(&self) -> &'static str {
        match self {
            ResolutionPrinciple::LexSuperior => "LexSuperior",
            ResolutionPrinciple::LexSpecialis => "LexSpecialis",
            ResolutionPrinciple::LexPosterior => "LexPosterior",
            ResolutionPrinciple::PactaSuntServanda => "PactaSuntServanda",
            ResolutionPrinciple::Irreconcilable => "Irreconcilable",
        }
    }

    /// The underlying Latin maxim (or a descriptive label).
    pub fn maxim(&self) -> &'static str {
        match self {
            ResolutionPrinciple::LexSuperior => "lex superior derogat legi inferiori",
            ResolutionPrinciple::LexSpecialis => "lex specialis derogat legi generali",
            ResolutionPrinciple::LexPosterior => "lex posterior derogat legi priori",
            ResolutionPrinciple::PactaSuntServanda => "pacta sunt servanda",
            ResolutionPrinciple::Irreconcilable => "(irreconcilable antinomy)",
        }
    }
}

impl fmt::Display for ResolutionPrinciple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.maxim())
    }
}

/// The structural kind of an antinomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AntinomyKind {
    /// A mandate clashes with a prohibition (or a permission with a prohibition).
    DeonticConflict,
    /// A grant of a right clashes with a revocation of the same right.
    GrantRevokeConflict,
    /// Two same-rank norms of different jurisdictions both purport to govern.
    ExtraterritorialClash,
    /// A lower-ranking norm contradicts a higher-ranking one.
    HierarchyInversion,
}

impl fmt::Display for AntinomyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AntinomyKind::DeonticConflict => "Deontic Conflict",
            AntinomyKind::GrantRevokeConflict => "Grant/Revoke Conflict",
            AntinomyKind::ExtraterritorialClash => "Extraterritorial Clash",
            AntinomyKind::HierarchyInversion => "Hierarchy Inversion",
        };
        f.write_str(s)
    }
}

/// A private-international-law choice-of-law connecting rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChoiceOfLawRule {
    /// Party autonomy / *lex voluntatis* (the law chosen by the parties).
    PartyAutonomy,
    /// Characteristic performance test (Rome I, art. 4).
    CharacteristicPerformance,
    /// *lex loci actus* — the law of the place where the act occurred.
    LexLociActus,
    /// *lex loci damni* — the law of the place where the damage occurred.
    LexLociDamni,
    /// *lex domicilii* — the law of the domicile.
    LexDomicilii,
    /// *lex patriae* — the law of nationality.
    LexPatriae,
    /// *lex fori* — the law of the forum (fallback / procedural).
    LexFori,
}

impl fmt::Display for ChoiceOfLawRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ChoiceOfLawRule::PartyAutonomy => "party autonomy (lex voluntatis)",
            ChoiceOfLawRule::CharacteristicPerformance => "characteristic performance",
            ChoiceOfLawRule::LexLociActus => "lex loci actus",
            ChoiceOfLawRule::LexLociDamni => "lex loci damni",
            ChoiceOfLawRule::LexDomicilii => "lex domicilii",
            ChoiceOfLawRule::LexPatriae => "lex patriae",
            ChoiceOfLawRule::LexFori => "lex fori",
        };
        f.write_str(s)
    }
}

/// Whether a national statute correctly transposes a treaty obligation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TranspositionStatus {
    /// An implementing statute of the correct polarity exists.
    Implemented,
    /// Statutes touch the matter but none enacts the required polarity.
    Partial,
    /// No statute addresses the obligation's subject domain.
    Missing,
    /// A statute enacts the opposite of what the obligation requires.
    Contradictory,
    /// The jurisdiction is not bound (not ratified, or article reserved).
    NotRequired,
}

impl fmt::Display for TranspositionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TranspositionStatus::Implemented => "Implemented",
            TranspositionStatus::Partial => "Partial",
            TranspositionStatus::Missing => "Missing",
            TranspositionStatus::Contradictory => "Contradictory",
            TranspositionStatus::NotRequired => "Not Required",
        };
        f.write_str(s)
    }
}

/// Cross-border recognition status between two jurisdictions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecognitionStatus {
    /// Each jurisdiction provides adequate protection relative to the other.
    MutualRecognition,
    /// Only the target provides adequate protection (one-way recognition).
    UnilateralRecognition,
    /// Neither side reaches the adequacy threshold.
    NoRecognition,
}

impl fmt::Display for RecognitionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RecognitionStatus::MutualRecognition => "Mutual Recognition",
            RecognitionStatus::UnilateralRecognition => "Unilateral Recognition",
            RecognitionStatus::NoRecognition => "No Recognition",
        };
        f.write_str(s)
    }
}

/// A normalised view of a [`Statute`] tailored for conflict analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalNorm {
    /// Originating statute id.
    pub statute_id: String,
    /// Jurisdiction string (verbatim from the statute).
    pub jurisdiction: String,
    /// Position in the normative hierarchy.
    pub level: JurisdictionLevel,
    /// The statute's effect type.
    pub effect_type: EffectType,
    /// Deontic polarity of the effect.
    pub polarity: Polarity,
    /// Subject-matter domains the norm touches.
    pub domains: BTreeSet<LegalDomain>,
    /// Specificity score (preconditions + exceptions); higher is more specific.
    pub specificity: u32,
    /// Date the norm took effect, if known.
    pub enacted_on: Option<NaiveDate>,
    /// Statute version.
    pub version: u32,
}

/// A detected and (where possible) resolved antinomy between two norms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormAntinomy {
    /// Opaque unique id.
    pub id: String,
    /// Structural kind of the conflict.
    pub kind: AntinomyKind,
    /// First statute id.
    pub norm_a: String,
    /// Second statute id.
    pub norm_b: String,
    /// Domains shared by both norms (the matter in conflict).
    pub shared_domains: Vec<LegalDomain>,
    /// The maxim that resolves (or fails to resolve) the conflict.
    pub resolution: ResolutionPrinciple,
    /// The prevailing statute id, or `None` when irreconcilable.
    pub prevailing: Option<String>,
    /// The displaced statute id, or `None` when irreconcilable.
    pub displaced: Option<String>,
    /// Severity of the conflict.
    pub severity: Severity,
    /// Human-readable rationale citing the applied maxim.
    pub rationale: String,
}

/// Connecting factors describing a cross-border legal situation, used as the
/// input to choice-of-law resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectingFactors {
    /// Subject-matter domain of the dispute.
    pub domain: LegalDomain,
    /// Law expressly chosen by the parties (*lex voluntatis*).
    pub chosen_law: Option<String>,
    /// Place where the relevant act was performed.
    pub place_of_act: Option<String>,
    /// Place where the damage / harm occurred.
    pub place_of_harm: Option<String>,
    /// Forum hearing the matter.
    pub forum: Option<String>,
    /// Domicile of the relevant party.
    pub domicile: Option<String>,
    /// Nationality of the relevant party.
    pub nationality: Option<String>,
    /// Seat of the party owing the characteristic performance.
    pub characteristic_performer_seat: Option<String>,
}

impl ConnectingFactors {
    /// Creates an otherwise-empty set of connecting factors for `domain`.
    pub fn new(domain: LegalDomain) -> Self {
        Self {
            domain,
            chosen_law: None,
            place_of_act: None,
            place_of_harm: None,
            forum: None,
            domicile: None,
            nationality: None,
            characteristic_performer_seat: None,
        }
    }

    /// Sets the chosen law (party autonomy).
    pub fn with_chosen_law(mut self, jurisdiction: impl Into<String>) -> Self {
        self.chosen_law = Some(jurisdiction.into());
        self
    }

    /// Sets the place of the act.
    pub fn with_place_of_act(mut self, jurisdiction: impl Into<String>) -> Self {
        self.place_of_act = Some(jurisdiction.into());
        self
    }

    /// Sets the place of the harm.
    pub fn with_place_of_harm(mut self, jurisdiction: impl Into<String>) -> Self {
        self.place_of_harm = Some(jurisdiction.into());
        self
    }

    /// Sets the forum.
    pub fn with_forum(mut self, jurisdiction: impl Into<String>) -> Self {
        self.forum = Some(jurisdiction.into());
        self
    }

    /// Sets the domicile.
    pub fn with_domicile(mut self, jurisdiction: impl Into<String>) -> Self {
        self.domicile = Some(jurisdiction.into());
        self
    }

    /// Sets the nationality.
    pub fn with_nationality(mut self, jurisdiction: impl Into<String>) -> Self {
        self.nationality = Some(jurisdiction.into());
        self
    }

    /// Sets the seat of the characteristic performer.
    pub fn with_characteristic_performer_seat(mut self, jurisdiction: impl Into<String>) -> Self {
        self.characteristic_performer_seat = Some(jurisdiction.into());
        self
    }
}

/// The outcome of choice-of-law resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceOfLawResolution {
    /// Domain the resolution was performed for.
    pub domain: LegalDomain,
    /// The jurisdiction whose law governs, if one could be determined.
    pub applicable_jurisdiction: Option<String>,
    /// The connecting rule that selected the initial governing law.
    pub rule: ChoiceOfLawRule,
    /// Whether a renvoi (remission/transmission) was followed.
    pub renvoi_applied: bool,
    /// Whether the forum's public policy displaced the foreign law.
    pub public_policy_override: bool,
    /// Step-by-step reasoning trace.
    pub reasoning: Vec<String>,
}

/// A structured treaty obligation against which national law is assessed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatyObligation {
    /// Treaty identifier (e.g. `"ICCPR"`).
    pub treaty_id: String,
    /// Article / provision identifier (e.g. `"Art. 7"`).
    pub article: String,
    /// The deontic polarity national law must enact to comply.
    pub required_polarity: Polarity,
    /// The subject-matter domain governed by the obligation.
    pub domain: LegalDomain,
    /// Short human-readable summary.
    pub summary: String,
}

impl TreatyObligation {
    /// Creates a new treaty obligation.
    pub fn new(
        treaty_id: impl Into<String>,
        article: impl Into<String>,
        required_polarity: Polarity,
        domain: LegalDomain,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            treaty_id: treaty_id.into(),
            article: article.into(),
            required_polarity,
            domain,
            summary: summary.into(),
        }
    }
}

/// The result of assessing transposition of one obligation into a jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspositionAssessment {
    /// Treaty identifier.
    pub treaty_id: String,
    /// Article identifier.
    pub article: String,
    /// Jurisdiction assessed.
    pub jurisdiction: String,
    /// Transposition status.
    pub status: TranspositionStatus,
    /// Statute ids relevant to the assessment.
    pub implementing_statutes: Vec<String>,
    /// Description of the gap (empty when fully implemented).
    pub gap_description: String,
    /// Severity of any shortfall.
    pub severity: Severity,
}

/// The result of an adequacy / equivalence assessment between two jurisdictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdequacyAssessment {
    /// Source jurisdiction (data exporter / requesting state).
    pub source: String,
    /// Target jurisdiction (data importer / requested state).
    pub target: String,
    /// Domain assessed.
    pub domain: LegalDomain,
    /// Aggregate protection score of the source.
    pub source_protection: u32,
    /// Aggregate protection score of the target.
    pub target_protection: u32,
    /// Whether the target reaches the adequacy threshold relative to the source.
    pub is_adequate: bool,
    /// Whether protection is adequate in both directions.
    pub is_reciprocal: bool,
    /// Recognition classification.
    pub recognition: RecognitionStatus,
    /// Narrative findings.
    pub findings: Vec<String>,
}

/// Aggregate report on the global coherence of a statute corpus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalCoherenceReport {
    /// Number of norms analysed.
    pub total_norms: usize,
    /// Number of distinct jurisdictions covered.
    pub jurisdictions: usize,
    /// All detected antinomies.
    pub antinomies: Vec<NormAntinomy>,
    /// Number of antinomies resolved by a maxim.
    pub resolved: usize,
    /// Number of irreconcilable antinomies.
    pub unresolved: usize,
    /// Normalised coherence index in `[0.0, 1.0]`.
    pub coherence_index: f64,
    /// Connected clusters of mutually-irreconcilable norms.
    pub incompatibility_clusters: Vec<Vec<String>>,
    /// Tally of resolutions grouped by maxim name.
    pub resolutions_by_principle: BTreeMap<String, usize>,
}

/// Configuration for the conflict-of-laws analyzer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictOfLawsConfig {
    /// Ratio of target/source protection required for adequacy (0.0-1.0).
    pub adequacy_ratio: f64,
    /// Whether to follow renvoi during choice-of-law resolution.
    pub enable_renvoi: bool,
    /// Whether to apply the public-policy (*ordre public*) exception.
    pub enable_public_policy: bool,
    /// Minimum specificity gap before *lex specialis* is invoked.
    pub min_specificity_gap: u32,
}

impl Default for ConflictOfLawsConfig {
    fn default() -> Self {
        Self {
            adequacy_ratio: 0.8,
            enable_renvoi: true,
            enable_public_policy: true,
            min_specificity_gap: 1,
        }
    }
}

/// Engine performing formal conflict-of-laws analysis over statutes.
#[derive(Debug, Clone)]
pub struct ConflictOfLawsAnalyzer {
    config: ConflictOfLawsConfig,
    level_overrides: HashMap<String, JurisdictionLevel>,
    treaty_obligations: Vec<TreatyObligation>,
    ratifications: HashMap<String, HashSet<String>>,
    reservations: HashMap<String, HashSet<String>>,
    renvoi_table: HashMap<String, String>,
    public_policy: HashMap<String, HashSet<LegalDomain>>,
}

impl Default for ConflictOfLawsAnalyzer {
    fn default() -> Self {
        Self::new(ConflictOfLawsConfig::default())
    }
}

impl ConflictOfLawsAnalyzer {
    /// Creates a new analyzer with the given configuration.
    pub fn new(config: ConflictOfLawsConfig) -> Self {
        Self {
            config,
            level_overrides: HashMap::new(),
            treaty_obligations: Vec::new(),
            ratifications: HashMap::new(),
            reservations: HashMap::new(),
            renvoi_table: HashMap::new(),
            public_policy: HashMap::new(),
        }
    }

    /// Returns the active configuration.
    pub fn config(&self) -> &ConflictOfLawsConfig {
        &self.config
    }

    /// Overrides the hierarchy level inferred for a jurisdiction string.
    pub fn set_jurisdiction_level(
        &mut self,
        jurisdiction: impl Into<String>,
        level: JurisdictionLevel,
    ) {
        self.level_overrides.insert(jurisdiction.into(), level);
    }

    /// Registers a treaty obligation for transposition analysis.
    pub fn register_treaty_obligation(&mut self, obligation: TreatyObligation) {
        self.treaty_obligations.push(obligation);
    }

    /// Records that `jurisdiction` has ratified `treaty_id`.
    pub fn ratify(&mut self, treaty_id: impl Into<String>, jurisdiction: impl Into<String>) {
        self.ratifications
            .entry(treaty_id.into())
            .or_default()
            .insert(jurisdiction.into());
    }

    /// Records a reservation excluding `article` of `treaty_id` for `jurisdiction`.
    pub fn add_reservation(
        &mut self,
        treaty_id: &str,
        jurisdiction: &str,
        article: impl Into<String>,
    ) {
        let key = format!("{treaty_id}{KEY_SEP}{jurisdiction}");
        self.reservations
            .entry(key)
            .or_default()
            .insert(article.into());
    }

    /// Sets the renvoi target: `from`'s conflict rule for `domain` refers to `to`.
    pub fn set_renvoi(&mut self, from: &str, domain: LegalDomain, to: impl Into<String>) {
        let key = format!("{from}{KEY_SEP}{}", domain.name());
        self.renvoi_table.insert(key, to.into());
    }

    /// Marks `domain` as blocked by `jurisdiction`'s public policy.
    pub fn block_for_public_policy(
        &mut self,
        jurisdiction: impl Into<String>,
        domain: LegalDomain,
    ) {
        self.public_policy
            .entry(jurisdiction.into())
            .or_default()
            .insert(domain);
    }

    fn has_reservation(&self, treaty_id: &str, jurisdiction: &str, article: &str) -> bool {
        let key = format!("{treaty_id}{KEY_SEP}{jurisdiction}");
        self.reservations
            .get(&key)
            .is_some_and(|set| set.contains(article))
    }

    /// Classifies a jurisdiction string into a hierarchy level, honouring any
    /// configured override before falling back to a heuristic.
    pub fn classify_jurisdiction_level(&self, jurisdiction: &str) -> JurisdictionLevel {
        if let Some(level) = self.level_overrides.get(jurisdiction) {
            return *level;
        }
        Self::heuristic_level(jurisdiction)
    }

    fn heuristic_level(jurisdiction: &str) -> JurisdictionLevel {
        let lower = jurisdiction.to_lowercase();
        let tokens: HashSet<String> = normalized_tokens(jurisdiction).into_iter().collect();
        let has = |w: &str| tokens.contains(w);

        if has("un")
            || has("uno")
            || has("international")
            || has("icj")
            || has("icc")
            || has("wto")
            || has("treaty")
            || has("convention")
            || has("global")
            || lower.contains("united nations")
        {
            return JurisdictionLevel::International;
        }
        if has("eu")
            || has("asean")
            || has("mercosur")
            || has("regional")
            || lower.contains("european union")
            || lower.contains("african union")
            || lower.contains("council of europe")
        {
            return JurisdictionLevel::Regional;
        }
        // National guard: prevents country names that happen to contain
        // sub-national tokens (e.g. "United States") from misclassifying.
        if has("federal") || has("national") || lower.contains("united states") {
            return JurisdictionLevel::National;
        }
        if has("city")
            || has("municipal")
            || has("municipality")
            || has("county")
            || has("local")
            || has("borough")
            || has("town")
        {
            return JurisdictionLevel::Local;
        }
        if has("state")
            || has("province")
            || has("provincial")
            || has("canton")
            || has("prefecture")
            || has("oblast")
        {
            return JurisdictionLevel::State;
        }
        JurisdictionLevel::National
    }

    /// Builds the analytic [`LegalNorm`] view of a statute.
    pub fn build_norm(&self, statute: &Statute) -> LegalNorm {
        let jurisdiction = statute
            .jurisdiction
            .clone()
            .unwrap_or_else(|| "Unspecified".to_string());
        let level = self.classify_jurisdiction_level(&jurisdiction);
        let enacted_on = statute.temporal_validity.effective_date.or_else(|| {
            statute
                .temporal_validity
                .enacted_at
                .map(|dt| dt.date_naive())
        });
        LegalNorm {
            statute_id: statute.id.clone(),
            jurisdiction,
            level,
            effect_type: statute.effect.effect_type.clone(),
            polarity: effect_polarity(&statute.effect.effect_type),
            domains: statute_domains(statute),
            specificity: (statute.preconditions.len() + statute.exceptions.len()) as u32,
            enacted_on,
            version: statute.version,
        }
    }

    /// Detects and resolves antinomies across a corpus of statutes.
    ///
    /// For every pair of statutes that address overlapping subject matter with
    /// directly opposed deontic effects, an antinomy is recorded and resolved
    /// using the conflict-resolution maxims in their doctrinal priority order
    /// (*lex superior* → *lex specialis* → *lex posterior*).
    pub fn detect_antinomies(&self, statutes: &[Statute]) -> Vec<NormAntinomy> {
        let norms: Vec<LegalNorm> = statutes.iter().map(|s| self.build_norm(s)).collect();
        let mut antinomies = Vec::new();
        for i in 0..norms.len() {
            for j in (i + 1)..norms.len() {
                if let Some(antinomy) = self.resolve_pair(&norms[i], &norms[j]) {
                    antinomies.push(antinomy);
                }
            }
        }
        antinomies
    }

    fn resolve_pair(&self, a: &LegalNorm, b: &LegalNorm) -> Option<NormAntinomy> {
        let shared: Vec<LegalDomain> = a
            .domains
            .iter()
            .filter(|d| b.domains.contains(*d) && **d != LegalDomain::Other)
            .copied()
            .collect();
        if shared.is_empty() {
            return None;
        }

        let grant_revoke = matches!(
            (&a.effect_type, &b.effect_type),
            (EffectType::Grant, EffectType::Revoke) | (EffectType::Revoke, EffectType::Grant)
        );
        let opposed = polarities_opposed(a.polarity, b.polarity);
        if !grant_revoke && !opposed {
            return None;
        }

        // Determine the structural kind of the conflict.
        let kind = if grant_revoke {
            AntinomyKind::GrantRevokeConflict
        } else if a.level != b.level {
            AntinomyKind::HierarchyInversion
        } else if a.jurisdiction != b.jurisdiction {
            AntinomyKind::ExtraterritorialClash
        } else {
            AntinomyKind::DeonticConflict
        };

        // Apply the conflict-resolution maxims in priority order.
        let (resolution, prevailing, displaced) = self.apply_maxims(a, b);

        let severity = match (&kind, &resolution) {
            (_, ResolutionPrinciple::Irreconcilable) => Severity::Error,
            (AntinomyKind::HierarchyInversion, _) | (_, ResolutionPrinciple::PactaSuntServanda) => {
                Severity::Critical
            }
            _ => Severity::Warning,
        };

        let domains_text = shared
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let rationale = match (&prevailing, &displaced) {
            (Some(p), Some(d)) => format!(
                "Conflict over [{}] resolved by {} ({}): {} prevails over {}.",
                domains_text,
                resolution.name(),
                resolution.maxim(),
                p,
                d
            ),
            _ => format!(
                "Conflict over [{}] is an irreconcilable antinomy: no maxim ranks {} above {}.",
                domains_text, a.statute_id, b.statute_id
            ),
        };

        Some(NormAntinomy {
            id: uuid::Uuid::new_v4().to_string(),
            kind,
            norm_a: a.statute_id.clone(),
            norm_b: b.statute_id.clone(),
            shared_domains: shared,
            resolution,
            prevailing,
            displaced,
            severity,
            rationale,
        })
    }

    /// Applies the resolution maxims, returning the principle plus the prevailing
    /// and displaced statute ids (both `None` when irreconcilable).
    fn apply_maxims(
        &self,
        a: &LegalNorm,
        b: &LegalNorm,
    ) -> (ResolutionPrinciple, Option<String>, Option<String>) {
        // lex superior: a smaller JurisdictionLevel ordinal is more authoritative
        // (International < Regional < National < State < Local).
        if a.level != b.level {
            let (high, low) = if a.level < b.level { (a, b) } else { (b, a) };
            let principle = if high.level == JurisdictionLevel::International {
                ResolutionPrinciple::PactaSuntServanda
            } else {
                ResolutionPrinciple::LexSuperior
            };
            return (
                principle,
                Some(high.statute_id.clone()),
                Some(low.statute_id.clone()),
            );
        }

        // lex specialis: the more specific (more preconditions/exceptions) wins.
        let gap = a.specificity.abs_diff(b.specificity);
        if gap >= self.config.min_specificity_gap && a.specificity != b.specificity {
            let (special, general) = if a.specificity > b.specificity {
                (a, b)
            } else {
                (b, a)
            };
            return (
                ResolutionPrinciple::LexSpecialis,
                Some(special.statute_id.clone()),
                Some(general.statute_id.clone()),
            );
        }

        // lex posterior: the later-enacted norm wins.
        if let (Some(da), Some(db)) = (a.enacted_on, b.enacted_on)
            && da != db
        {
            let (later, earlier) = if da > db { (a, b) } else { (b, a) };
            return (
                ResolutionPrinciple::LexPosterior,
                Some(later.statute_id.clone()),
                Some(earlier.statute_id.clone()),
            );
        }

        (ResolutionPrinciple::Irreconcilable, None, None)
    }

    /// Resolves which jurisdiction's law governs a cross-border situation.
    pub fn resolve_choice_of_law(&self, factors: &ConnectingFactors) -> ChoiceOfLawResolution {
        let mut reasoning = Vec::new();
        let (mut applicable, rule) = self.select_initial_law(factors, &mut reasoning);

        let mut renvoi_applied = false;
        if self.config.enable_renvoi
            && let (Some(jur), Some(forum)) = (applicable.clone(), factors.forum.clone())
            && jur != forum
        {
            let key = format!("{jur}{KEY_SEP}{}", factors.domain.name());
            if let Some(target) = self.renvoi_table.get(&key) {
                if *target == forum {
                    reasoning.push(format!(
                        "Renvoi: {jur}'s conflict rule for {} remits back to the forum; \
                         single renvoi accepted, applying lex fori ({forum}).",
                        factors.domain
                    ));
                    applicable = Some(forum);
                } else {
                    reasoning.push(format!(
                        "Renvoi: {jur}'s conflict rule for {} transmits to {target}.",
                        factors.domain
                    ));
                    applicable = Some(target.clone());
                }
                renvoi_applied = true;
            }
        }

        let mut public_policy_override = false;
        if self.config.enable_public_policy
            && let Some(jur) = applicable.clone()
        {
            let blocked = self
                .public_policy
                .get(&jur)
                .is_some_and(|set| set.contains(&factors.domain));
            if blocked && let Some(forum) = factors.forum.clone() {
                reasoning.push(format!(
                    "Public policy (ordre public): the forum refuses to apply {jur} law on \
                     {}; substituting lex fori ({forum}).",
                    factors.domain
                ));
                applicable = Some(forum);
                public_policy_override = true;
            }
        }

        ChoiceOfLawResolution {
            domain: factors.domain,
            applicable_jurisdiction: applicable,
            rule,
            renvoi_applied,
            public_policy_override,
            reasoning,
        }
    }

    fn select_initial_law(
        &self,
        factors: &ConnectingFactors,
        reasoning: &mut Vec<String>,
    ) -> (Option<String>, ChoiceOfLawRule) {
        use LegalDomain::{
            Competition, Contract, Criminal, DataProtection, Environmental, Family,
            FinancialRegulation, Health, HumanRights, Immigration, Trade,
        };

        if let Some(chosen) = &factors.chosen_law {
            reasoning.push(format!("Party autonomy: the parties selected {chosen}."));
            return (Some(chosen.clone()), ChoiceOfLawRule::PartyAutonomy);
        }

        match factors.domain {
            Contract | Trade | FinancialRegulation | Competition => {
                if let Some(seat) = &factors.characteristic_performer_seat {
                    reasoning.push(format!("Characteristic performance points to {seat}."));
                    return (
                        Some(seat.clone()),
                        ChoiceOfLawRule::CharacteristicPerformance,
                    );
                }
                if let Some(place) = &factors.place_of_act {
                    reasoning.push(format!("Lex loci actus points to {place}."));
                    return (Some(place.clone()), ChoiceOfLawRule::LexLociActus);
                }
            }
            Environmental | Health => {
                if let Some(harm) = &factors.place_of_harm {
                    reasoning.push(format!("Lex loci damni points to {harm}."));
                    return (Some(harm.clone()), ChoiceOfLawRule::LexLociDamni);
                }
                if let Some(place) = &factors.place_of_act {
                    reasoning.push(format!("Lex loci actus points to {place}."));
                    return (Some(place.clone()), ChoiceOfLawRule::LexLociActus);
                }
            }
            Family => {
                if let Some(dom) = &factors.domicile {
                    reasoning.push(format!("Lex domicilii points to {dom}."));
                    return (Some(dom.clone()), ChoiceOfLawRule::LexDomicilii);
                }
                if let Some(nat) = &factors.nationality {
                    reasoning.push(format!("Lex patriae points to {nat}."));
                    return (Some(nat.clone()), ChoiceOfLawRule::LexPatriae);
                }
            }
            DataProtection | HumanRights => {
                if let Some(dom) = &factors.domicile {
                    reasoning.push(format!(
                        "Protective connecting factor (lex domicilii) points to {dom}."
                    ));
                    return (Some(dom.clone()), ChoiceOfLawRule::LexDomicilii);
                }
                if let Some(harm) = &factors.place_of_harm {
                    reasoning.push(format!("Lex loci damni points to {harm}."));
                    return (Some(harm.clone()), ChoiceOfLawRule::LexLociDamni);
                }
            }
            Criminal | Immigration => {
                if let Some(place) = &factors.place_of_act {
                    reasoning.push(format!(
                        "Territoriality (lex loci actus) points to {place}."
                    ));
                    return (Some(place.clone()), ChoiceOfLawRule::LexLociActus);
                }
            }
            _ => {
                if let Some(place) = &factors.place_of_act {
                    reasoning.push(format!("Lex loci actus points to {place}."));
                    return (Some(place.clone()), ChoiceOfLawRule::LexLociActus);
                }
            }
        }

        if let Some(forum) = &factors.forum {
            reasoning.push(format!(
                "No specific connecting factor available; falling back to lex fori ({forum})."
            ));
            return (Some(forum.clone()), ChoiceOfLawRule::LexFori);
        }

        reasoning.push("No connecting factor and no forum supplied; law is indeterminate.".into());
        (None, ChoiceOfLawRule::LexFori)
    }

    /// Assesses how a jurisdiction transposes each registered treaty obligation.
    pub fn assess_transposition(
        &self,
        jurisdiction: &str,
        statutes: &[Statute],
    ) -> Vec<TranspositionAssessment> {
        let mut assessments = Vec::new();
        for obligation in &self.treaty_obligations {
            assessments.push(self.assess_one_obligation(jurisdiction, obligation, statutes));
        }
        assessments
    }

    fn assess_one_obligation(
        &self,
        jurisdiction: &str,
        obligation: &TreatyObligation,
        statutes: &[Statute],
    ) -> TranspositionAssessment {
        let ratified = self
            .ratifications
            .get(&obligation.treaty_id)
            .is_some_and(|set| set.contains(jurisdiction));
        let reserved =
            self.has_reservation(&obligation.treaty_id, jurisdiction, &obligation.article);

        if !ratified || reserved {
            let gap = if reserved {
                format!(
                    "{jurisdiction} entered a reservation excluding {} {}.",
                    obligation.treaty_id, obligation.article
                )
            } else {
                format!("{jurisdiction} has not ratified {}.", obligation.treaty_id)
            };
            return TranspositionAssessment {
                treaty_id: obligation.treaty_id.clone(),
                article: obligation.article.clone(),
                jurisdiction: jurisdiction.to_string(),
                status: TranspositionStatus::NotRequired,
                implementing_statutes: Vec::new(),
                gap_description: gap,
                severity: Severity::Info,
            };
        }

        let in_domain: Vec<&Statute> = statutes
            .iter()
            .filter(|s| s.jurisdiction.as_deref() == Some(jurisdiction))
            .filter(|s| statute_domains(s).contains(&obligation.domain))
            .collect();

        if in_domain.is_empty() {
            return TranspositionAssessment {
                treaty_id: obligation.treaty_id.clone(),
                article: obligation.article.clone(),
                jurisdiction: jurisdiction.to_string(),
                status: TranspositionStatus::Missing,
                implementing_statutes: Vec::new(),
                gap_description: format!(
                    "No {} statute implements {} {} ({}).",
                    obligation.domain, obligation.treaty_id, obligation.article, obligation.summary
                ),
                severity: Severity::Error,
            };
        }

        let matching: Vec<String> = in_domain
            .iter()
            .filter(|s| effect_polarity(&s.effect.effect_type) == obligation.required_polarity)
            .map(|s| s.id.clone())
            .collect();
        let contradicting: Vec<String> = in_domain
            .iter()
            .filter(|s| {
                polarities_opposed(
                    effect_polarity(&s.effect.effect_type),
                    obligation.required_polarity,
                )
            })
            .map(|s| s.id.clone())
            .collect();

        if !matching.is_empty() {
            TranspositionAssessment {
                treaty_id: obligation.treaty_id.clone(),
                article: obligation.article.clone(),
                jurisdiction: jurisdiction.to_string(),
                status: TranspositionStatus::Implemented,
                implementing_statutes: matching,
                gap_description: String::new(),
                severity: Severity::Info,
            }
        } else if !contradicting.is_empty() {
            TranspositionAssessment {
                treaty_id: obligation.treaty_id.clone(),
                article: obligation.article.clone(),
                jurisdiction: jurisdiction.to_string(),
                status: TranspositionStatus::Contradictory,
                implementing_statutes: contradicting,
                gap_description: format!(
                    "National law enacts the opposite of the required {} polarity for {} {}.",
                    obligation.required_polarity, obligation.treaty_id, obligation.article
                ),
                severity: Severity::Critical,
            }
        } else {
            TranspositionAssessment {
                treaty_id: obligation.treaty_id.clone(),
                article: obligation.article.clone(),
                jurisdiction: jurisdiction.to_string(),
                status: TranspositionStatus::Partial,
                implementing_statutes: in_domain.iter().map(|s| s.id.clone()).collect(),
                gap_description: format!(
                    "Statutes touch {} but none enacts the required {} polarity.",
                    obligation.domain, obligation.required_polarity
                ),
                severity: Severity::Warning,
            }
        }
    }

    /// Assesses equivalence of protection between two jurisdictions in a domain.
    pub fn assess_adequacy(
        &self,
        source: &str,
        target: &str,
        domain: LegalDomain,
        statutes: &[Statute],
    ) -> AdequacyAssessment {
        let source_protection = protection_score(source, domain, statutes);
        let target_protection = protection_score(target, domain, statutes);

        let threshold = (source_protection as f64 * self.config.adequacy_ratio).ceil() as u32;
        let is_adequate = target_protection >= threshold;
        let reverse_threshold =
            (target_protection as f64 * self.config.adequacy_ratio).ceil() as u32;
        let source_adequate = source_protection >= reverse_threshold;
        let is_reciprocal = is_adequate && source_adequate;

        let recognition = if is_reciprocal {
            RecognitionStatus::MutualRecognition
        } else if is_adequate {
            RecognitionStatus::UnilateralRecognition
        } else {
            RecognitionStatus::NoRecognition
        };

        let mut findings = Vec::new();
        findings.push(format!(
            "{source} protection score: {source_protection}; {target} protection score: \
             {target_protection} (adequacy threshold {threshold})."
        ));
        if is_adequate {
            findings.push(format!(
                "{target} offers protection adequate for transfers from {source} in {domain}."
            ));
        } else {
            findings.push(format!(
                "{target} falls short of adequate protection for {source} in {domain}; \
                 supplementary safeguards are required."
            ));
        }
        if is_reciprocal {
            findings.push("Protection is adequate in both directions: mutual recognition.".into());
        }

        AdequacyAssessment {
            source: source.to_string(),
            target: target.to_string(),
            domain,
            source_protection,
            target_protection,
            is_adequate,
            is_reciprocal,
            recognition,
            findings,
        }
    }

    /// Verifies the overall coherence of a statute corpus, aggregating antinomies
    /// into clusters and a normalised coherence index.
    pub fn verify_global_coherence(&self, statutes: &[Statute]) -> GlobalCoherenceReport {
        let antinomies = self.detect_antinomies(statutes);

        let jurisdictions: HashSet<&str> = statutes
            .iter()
            .filter_map(|s| s.jurisdiction.as_deref())
            .filter(|j| !j.is_empty())
            .collect();

        let resolved = antinomies.iter().filter(|a| a.prevailing.is_some()).count();
        let unresolved = antinomies.len() - resolved;

        let clusters = compute_incompatibility_clusters(&antinomies);
        let entangled: usize = clusters.iter().map(|c| c.len()).sum();

        let total_norms = statutes.len();
        let denom = total_norms.max(1) as f64;
        let structural = (denom - entangled as f64) / denom;
        let resolution = if antinomies.is_empty() {
            1.0
        } else {
            resolved as f64 / antinomies.len() as f64
        };
        let coherence_index = (0.6 * structural + 0.4 * resolution).clamp(0.0, 1.0);

        let mut resolutions_by_principle: BTreeMap<String, usize> = BTreeMap::new();
        for antinomy in &antinomies {
            *resolutions_by_principle
                .entry(antinomy.resolution.name().to_string())
                .or_insert(0) += 1;
        }

        GlobalCoherenceReport {
            total_norms,
            jurisdictions: jurisdictions.len(),
            antinomies,
            resolved,
            unresolved,
            coherence_index,
            incompatibility_clusters: clusters,
            resolutions_by_principle,
        }
    }

    /// Renders a global coherence report as Markdown.
    pub fn coherence_report_markdown(&self, report: &GlobalCoherenceReport) -> String {
        let mut out = String::new();
        out.push_str("# Global Legal Coherence Report\n\n");
        out.push_str("## Overview\n\n");
        out.push_str(&format!("- **Norms analysed**: {}\n", report.total_norms));
        out.push_str(&format!(
            "- **Jurisdictions covered**: {}\n",
            report.jurisdictions
        ));
        out.push_str(&format!(
            "- **Coherence index**: {:.3}\n",
            report.coherence_index
        ));
        out.push_str(&format!(
            "- **Antinomies**: {} ({} resolved, {} irreconcilable)\n\n",
            report.antinomies.len(),
            report.resolved,
            report.unresolved
        ));

        if !report.resolutions_by_principle.is_empty() {
            out.push_str("## Resolutions by Principle\n\n");
            for (principle, count) in &report.resolutions_by_principle {
                out.push_str(&format!("- {principle}: {count}\n"));
            }
            out.push('\n');
        }

        if !report.antinomies.is_empty() {
            out.push_str("## Antinomies\n\n");
            for antinomy in &report.antinomies {
                out.push_str(&format!(
                    "### {} vs {} — {} ({:?})\n",
                    antinomy.norm_a, antinomy.norm_b, antinomy.kind, antinomy.severity
                ));
                out.push_str(&format!("- **Resolution**: {}\n", antinomy.resolution));
                if let Some(prevailing) = &antinomy.prevailing {
                    out.push_str(&format!("- **Prevailing**: {prevailing}\n"));
                }
                out.push_str(&format!("- **Rationale**: {}\n\n", antinomy.rationale));
            }
        }

        if !report.incompatibility_clusters.is_empty() {
            out.push_str("## Incompatibility Clusters\n\n");
            for (idx, cluster) in report.incompatibility_clusters.iter().enumerate() {
                out.push_str(&format!("{}. {}\n", idx + 1, cluster.join(", ")));
            }
            out.push('\n');
        }

        out
    }
}

/// Returns the subject-matter domains touched by a statute (title, effect text
/// and precondition descriptions combined).
pub fn statute_domains(statute: &Statute) -> BTreeSet<LegalDomain> {
    let mut text = format!("{} {}", statute.title, statute.effect.description);
    for condition in &statute.preconditions {
        text.push(' ');
        text.push_str(&condition.to_string());
    }
    classify_domains(&text)
}

/// Weight of an effect as a measure of how strongly it protects subjects.
fn protection_weight(effect: &Effect) -> u32 {
    match effect.effect_type {
        EffectType::Obligation | EffectType::Prohibition => 3,
        EffectType::Grant => 2,
        EffectType::Revoke
        | EffectType::MonetaryTransfer
        | EffectType::StatusChange
        | EffectType::Custom => 1,
    }
}

/// Aggregate protection score of a jurisdiction within a domain.
fn protection_score(jurisdiction: &str, domain: LegalDomain, statutes: &[Statute]) -> u32 {
    statutes
        .iter()
        .filter(|s| s.jurisdiction.as_deref() == Some(jurisdiction))
        .filter(|s| statute_domains(s).contains(&domain))
        .map(|s| protection_weight(&s.effect))
        .sum()
}

/// Computes connected components of the irreconcilable-antinomy graph.
fn compute_incompatibility_clusters(antinomies: &[NormAntinomy]) -> Vec<Vec<String>> {
    let mut adjacency: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for antinomy in antinomies {
        if antinomy.prevailing.is_none() {
            adjacency
                .entry(antinomy.norm_a.clone())
                .or_default()
                .insert(antinomy.norm_b.clone());
            adjacency
                .entry(antinomy.norm_b.clone())
                .or_default()
                .insert(antinomy.norm_a.clone());
        }
    }

    let mut visited: BTreeSet<String> = BTreeSet::new();
    let mut clusters = Vec::new();
    let nodes: Vec<String> = adjacency.keys().cloned().collect();
    for node in nodes {
        if visited.contains(&node) {
            continue;
        }
        let mut component: BTreeSet<String> = BTreeSet::new();
        let mut stack = vec![node];
        while let Some(current) = stack.pop() {
            if !visited.insert(current.clone()) {
                continue;
            }
            component.insert(current.clone());
            if let Some(neighbours) = adjacency.get(&current) {
                for neighbour in neighbours {
                    if !visited.contains(neighbour) {
                        stack.push(neighbour.clone());
                    }
                }
            }
        }
        if component.len() >= 2 {
            clusters.push(component.into_iter().collect());
        }
    }
    clusters
}

#[cfg(test)]
mod tests;
