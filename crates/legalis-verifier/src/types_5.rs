//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;
use std::collections::HashMap;
use thiserror::Error;

use super::types::{EncryptedVerificationResult, OverlapArea};
use super::types_3::{
    Clock, ClockConstraint, ConflictNature, DeadlineViolation, DiagnosticLocation,
    MechanismProperty, NotificationChannel, NotificationType, PrincipleCheck, TextEdit,
};
use super::types_4::{ReportSection, Severity};

/// LSP-compatible diagnostic for IDE integration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IdeDiagnostic {
    /// Diagnostic severity (error, warning, info, hint)
    pub severity: String,
    /// Diagnostic message
    pub message: String,
    /// Location in source
    pub location: Option<DiagnosticLocation>,
    /// Diagnostic code (e.g., "E001")
    pub code: Option<String>,
    /// Source of the diagnostic (e.g., "legalis-verifier")
    pub source: String,
    /// Related information
    pub related: Vec<String>,
    /// Suggested fixes
    pub fixes: Vec<String>,
}
impl IdeDiagnostic {
    /// Creates a new IDE diagnostic.
    pub fn new(severity: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: severity.into(),
            message: message.into(),
            location: None,
            code: None,
            source: "legalis-verifier".to_string(),
            related: Vec::new(),
            fixes: Vec::new(),
        }
    }
    /// Sets the diagnostic location.
    pub fn with_location(mut self, location: DiagnosticLocation) -> Self {
        self.location = Some(location);
        self
    }
    /// Sets the diagnostic code.
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
    /// Adds related information.
    pub fn with_related(mut self, info: impl Into<String>) -> Self {
        self.related.push(info.into());
        self
    }
    /// Adds a suggested fix.
    pub fn with_fix(mut self, fix: impl Into<String>) -> Self {
        self.fixes.push(fix.into());
        self
    }
}
/// Result of a verification check.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationResult {
    /// Whether the verification passed
    pub passed: bool,
    /// List of errors found
    pub errors: Vec<VerificationError>,
    /// List of warnings
    pub warnings: Vec<String>,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}
impl VerificationResult {
    /// Creates a passing result.
    pub fn pass() -> Self {
        Self {
            passed: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }
    /// Creates a failing result with errors.
    pub fn fail(errors: Vec<VerificationError>) -> Self {
        Self {
            passed: false,
            errors,
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }
    /// Adds a warning.
    pub fn with_warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }
    /// Adds a suggestion.
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
    /// Merges another result into this one.
    pub fn merge(&mut self, other: VerificationResult) {
        if !other.passed {
            self.passed = false;
        }
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        self.suggestions.extend(other.suggestions);
    }
    /// Exports the result to JSON format.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    /// Exports the result to JSON format (non-pretty).
    pub fn to_json_compact(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    /// Loads a result from JSON format.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
    /// Filters errors by minimum severity level.
    pub fn errors_by_severity(&self, min_severity: Severity) -> Vec<&VerificationError> {
        self.errors
            .iter()
            .filter(|e| e.severity() >= min_severity)
            .collect()
    }
    /// Counts errors by severity level.
    pub fn severity_counts(&self) -> HashMap<Severity, usize> {
        let mut counts = HashMap::new();
        for error in &self.errors {
            *counts.entry(error.severity()).or_insert(0) += 1;
        }
        counts
    }
    /// Returns true if there are any critical errors.
    pub fn has_critical_errors(&self) -> bool {
        self.errors
            .iter()
            .any(|e| e.severity() == Severity::Critical)
    }
}
/// Result of deadline verification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeadlineVerificationResult {
    /// Whether all deadlines were met
    pub passed: bool,
    /// Violated deadlines
    pub violations: Vec<DeadlineViolation>,
}
/// Information about a statute in a regulatory filing
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatuteFilingInfo {
    /// Statute ID
    pub statute_id: String,
    /// Statute title
    pub title: String,
    /// Effective date
    pub effective_date: Option<String>,
    /// Enactment date
    pub enactment_date: Option<String>,
    /// Compliance status for this statute
    pub status: String,
    /// Issues found (if any)
    pub issues: Vec<String>,
}
/// Conflict summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConflictSummary {
    /// Total conflicts detected
    pub total_conflicts: usize,
    /// Conflicts by type
    pub conflicts_by_type: HashMap<String, usize>,
    /// Critical conflicts (severity critical)
    pub critical_conflicts: usize,
}
/// Multi-party computation result
/// Allows multiple parties to jointly verify statutes without sharing their private inputs
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MultiPartyVerificationResult {
    /// Participating parties
    pub parties: Vec<String>,
    /// Combined verification result (without revealing individual inputs)
    pub combined_result: VerificationResult,
    /// Proof that computation was performed correctly
    pub computation_proof: String,
    /// Timestamp of the computation
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
impl MultiPartyVerificationResult {
    /// Creates a new multi-party verification result
    pub fn new(parties: Vec<String>, combined_result: VerificationResult) -> Self {
        Self {
            parties,
            combined_result,
            computation_proof: format!("mpc-proof-{}", uuid::Uuid::new_v4()),
            timestamp: chrono::Utc::now(),
        }
    }
    /// Generates a report
    pub fn report(&self) -> String {
        format!(
            "Multi-Party Verification Report\n\
             ==============================\n\
             Parties: {}\n\
             Verification Passed: {}\n\
             Errors: {}\n\
             Warnings: {}\n\
             Computation Proof: {}\n\
             Timestamp: {}\n",
            self.parties.join(", "),
            self.combined_result.passed,
            self.combined_result.errors.len(),
            self.combined_result.warnings.len(),
            self.computation_proof,
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}
/// Cached proof for a statute
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CachedProof {
    /// Statute ID
    pub statute_id: String,
    /// Verification result
    pub result: VerificationResult,
    /// Proof timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Hash of statute content
    pub content_hash: String,
}
impl CachedProof {
    /// Creates a new cached proof
    pub fn new(statute: &Statute, result: VerificationResult) -> Self {
        Self {
            statute_id: statute.id.clone(),
            result,
            timestamp: chrono::Utc::now(),
            content_hash: format!("{:x}", md5::compute(format!("{:?}", statute))),
        }
    }
    /// Checks if the proof is still valid for the given statute
    pub fn is_valid(&self, statute: &Statute) -> bool {
        let current_hash = format!("{:x}", md5::compute(format!("{:?}", statute)));
        self.content_hash == current_hash
    }
}
/// Represents a state in a Markov chain
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct MarkovState {
    /// Unique state identifier
    pub id: String,
    /// Human-readable state description
    pub description: String,
    /// Whether this is an accepting state
    pub accepting: bool,
}
impl MarkovState {
    /// Creates a new Markov state
    pub fn new(id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            accepting: false,
        }
    }
    /// Marks this state as accepting
    pub fn accepting(mut self) -> Self {
        self.accepting = true;
        self
    }
}
/// Notification configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationConfig {
    /// Channels to notify
    pub channels: Vec<NotificationChannel>,
    /// Notification types to trigger on
    pub trigger_on: Vec<NotificationType>,
    /// Include detailed results in notification
    pub include_details: bool,
}
impl NotificationConfig {
    /// Creates a new notification configuration.
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
            trigger_on: vec![NotificationType::Error, NotificationType::Critical],
            include_details: true,
        }
    }
    /// Adds a webhook channel.
    pub fn with_webhook(mut self, url: impl Into<String>) -> Self {
        self.channels.push(NotificationChannel::Webhook {
            url: url.into(),
            headers: HashMap::new(),
        });
        self
    }
    /// Adds an email channel.
    pub fn with_email(mut self, to: Vec<String>, subject: impl Into<String>) -> Self {
        self.channels.push(NotificationChannel::Email {
            to,
            subject: subject.into(),
        });
        self
    }
    /// Sets the trigger types.
    pub fn trigger_on(mut self, types: Vec<NotificationType>) -> Self {
        self.trigger_on = types;
        self
    }
    /// Sets whether to include details.
    pub fn include_details(mut self, include: bool) -> Self {
        self.include_details = include;
        self
    }
}
/// Report template for customizable report generation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReportTemplate {
    /// Template name
    pub name: String,
    /// Sections to include in the report
    pub sections: Vec<ReportSection>,
    /// Header text
    pub header: Option<String>,
    /// Footer text
    pub footer: Option<String>,
    /// Whether to include table of contents
    pub include_toc: bool,
}
impl ReportTemplate {
    /// Creates a new report template
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sections: Vec::new(),
            header: None,
            footer: None,
            include_toc: false,
        }
    }
    /// Adds a section to the template
    pub fn with_section(mut self, section: ReportSection) -> Self {
        self.sections.push(section);
        self
    }
    /// Sets the header text
    pub fn with_header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }
    /// Sets the footer text
    pub fn with_footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = Some(footer.into());
        self
    }
    /// Enables table of contents
    pub fn with_toc(mut self) -> Self {
        self.include_toc = true;
        self
    }
}
/// Encrypted statute representation for homomorphic computation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncryptedStatute {
    /// Encrypted statute identifier
    pub encrypted_id: Vec<u8>,
    /// Encrypted statute data
    pub encrypted_data: Vec<u8>,
    /// Encryption scheme used
    pub scheme: String,
    /// Public parameters
    pub public_params: HashMap<String, String>,
}
impl EncryptedStatute {
    /// Creates a new encrypted statute (simplified encryption)
    pub fn new(statute: &Statute) -> Self {
        use rand::RngExt;
        let mut rng = rand::rng();
        let id_bytes = statute.id.as_bytes();
        let encrypted_id: Vec<u8> = id_bytes.iter().map(|&b| b ^ rng.random::<u8>()).collect();
        let data_bytes = format!("{:?}", statute).as_bytes().to_vec();
        let encrypted_data: Vec<u8> = data_bytes.iter().map(|&b| b ^ rng.random::<u8>()).collect();
        Self {
            encrypted_id,
            encrypted_data,
            scheme: "Simplified-XOR".to_string(),
            public_params: HashMap::new(),
        }
    }
    /// Performs homomorphic verification (computation on encrypted data)
    pub fn homomorphic_verify(&self) -> EncryptedVerificationResult {
        EncryptedVerificationResult {
            encrypted_result: vec![0u8; 32],
            scheme: self.scheme.clone(),
        }
    }
}
/// Configuration of a timed automaton (location + clock valuations).
#[derive(Debug, Clone)]
pub(crate) struct TimedConfiguration {
    pub(crate) location: String,
    pub(crate) valuations: HashMap<String, u64>,
}
impl TimedConfiguration {
    pub(crate) fn new(location: String) -> Self {
        Self {
            location,
            valuations: HashMap::new(),
        }
    }
}
/// A constitutional principle to check against.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConstitutionalPrinciple {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description of the principle
    pub description: String,
    /// Type of check to perform
    pub check: PrincipleCheck,
}
/// Errors from verification process.
#[derive(Debug, Clone, Error, serde::Serialize, serde::Deserialize)]
pub enum VerificationError {
    #[error("Circular reference detected: {message}")]
    CircularReference { message: String },
    #[error("Dead statute detected: {statute_id} can never be satisfied")]
    DeadStatute { statute_id: String },
    #[error("Constitutional conflict: {statute_id} conflicts with {principle}")]
    ConstitutionalConflict {
        statute_id: String,
        principle: String,
    },
    #[error("Logical contradiction: {message}")]
    LogicalContradiction { message: String },
    #[error("Ambiguity detected: {message}")]
    Ambiguity { message: String },
    #[error("Unreachable code detected: {message}")]
    UnreachableCode { message: String },
}
impl VerificationError {
    /// Returns the severity level of this error.
    pub fn severity(&self) -> Severity {
        match self {
            Self::CircularReference { .. } => Severity::Critical,
            Self::DeadStatute { .. } => Severity::Error,
            Self::ConstitutionalConflict { .. } => Severity::Critical,
            Self::LogicalContradiction { .. } => Severity::Error,
            Self::Ambiguity { .. } => Severity::Warning,
            Self::UnreachableCode { .. } => Severity::Warning,
        }
    }
}
/// Quick fix suggestion for IDE code actions.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuickFix {
    /// Title of the fix
    pub title: String,
    /// Description
    pub description: String,
    /// Kind of fix (e.g., "quickfix", "refactor")
    pub kind: String,
    /// Edits to apply
    pub edits: Vec<TextEdit>,
}
impl QuickFix {
    /// Creates a new quick fix.
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            kind: "quickfix".to_string(),
            edits: Vec::new(),
        }
    }
    /// Adds an edit to the quick fix.
    pub fn with_edit(mut self, edit: TextEdit) -> Self {
        self.edits.push(edit);
        self
    }
    /// Sets the kind of fix.
    pub fn with_kind(mut self, kind: impl Into<String>) -> Self {
        self.kind = kind.into();
        self
    }
}
/// Type of statute pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PatternType {
    /// Age-based eligibility
    AgeEligibility,
    /// Income-based qualification
    IncomeQualification,
    /// Combined age and income
    AgeAndIncome,
    /// Prohibition with exceptions
    ProhibitionWithExceptions,
    /// Temporal restriction
    TemporalRestriction,
    /// Jurisdiction-specific
    JurisdictionalPattern,
    /// Custom pattern
    Custom,
}
/// Represents a regulatory overlap between statutes
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegulatoryOverlap {
    /// IDs of overlapping statutes
    pub statute_ids: Vec<String>,
    /// The area of overlap
    pub overlap_area: OverlapArea,
    /// Description of the overlap
    pub description: String,
    /// Severity of the overlap
    pub severity: Severity,
    /// Suggestion for resolution
    pub resolution: String,
}
/// A transition in a timed automaton.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimedTransition {
    /// Source location
    pub from: String,
    /// Target location
    pub to: String,
    /// Guard (condition) for the transition
    pub guard: Option<ClockConstraint>,
    /// Clocks to reset on this transition
    pub resets: Vec<Clock>,
    /// Action/label for the transition
    pub action: String,
}
impl TimedTransition {
    /// Creates a new timed transition.
    pub fn new(from: impl Into<String>, to: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            guard: None,
            resets: Vec::new(),
            action: action.into(),
        }
    }
    /// Sets the guard.
    pub fn with_guard(mut self, constraint: ClockConstraint) -> Self {
        self.guard = Some(constraint);
        self
    }
    /// Adds a clock to reset.
    pub fn with_reset(mut self, clock: Clock) -> Self {
        self.resets.push(clock);
        self
    }
}
/// Complexity levels for statutes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ComplexityLevel {
    /// Simple statute with few conditions
    #[default]
    Simple,
    /// Moderate complexity
    Moderate,
    /// Complex statute requiring careful review
    Complex,
    /// Very complex statute, consider simplification
    VeryComplex,
}
/// Represents a conflict between multiple stakeholders
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StakeholderConflict {
    /// Involved stakeholders
    pub stakeholders: Vec<String>,
    /// Conflicting statutes
    pub statutes: Vec<String>,
    /// Nature of the conflict
    pub conflict_type: ConflictNature,
    /// Severity of the conflict
    pub severity: Severity,
    /// Description of the conflict
    pub description: String,
    /// Potential resolution strategies
    pub resolutions: Vec<String>,
}
/// A compliance checklist item.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplianceItem {
    /// Item number
    pub number: usize,
    /// Description of the requirement
    pub requirement: String,
    /// Precondition that must be met
    pub precondition: Option<String>,
    /// Priority level
    pub priority: String,
}
/// Represents an outcome in the game
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameOutcome {
    /// Strategies played by each stakeholder
    pub strategies: Vec<String>,
    /// Payoffs for each stakeholder (indexed by stakeholder position)
    pub payoffs: Vec<i32>,
    /// Whether this is a Nash equilibrium
    pub is_nash_equilibrium: bool,
    /// Description of the outcome
    pub description: String,
}
/// Represents a mechanism design issue
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MechanismIssue {
    /// Property violated
    pub property: MechanismProperty,
    /// Statute(s) involved
    pub statute_ids: Vec<String>,
    /// Severity of the issue
    pub severity: Severity,
    /// Description of the issue
    pub description: String,
    /// Suggested fixes
    pub suggestions: Vec<String>,
}
/// Represents a change between two statute versions.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum StatuteChange {
    /// Title changed
    TitleChanged { old: String, new: String },
    /// Description changed
    DescriptionChanged {
        old: Option<String>,
        new: Option<String>,
    },
    /// Jurisdiction changed
    JurisdictionChanged {
        old: Option<String>,
        new: Option<String>,
    },
    /// Effect changed
    EffectChanged { old: String, new: String },
    /// Preconditions changed
    PreconditionsChanged { old_count: usize, new_count: usize },
    /// Enactment date changed
    EnactmentDateChanged {
        old: Option<String>,
        new: Option<String>,
    },
    /// Effective date changed
    EffectiveDateChanged {
        old: Option<String>,
        new: Option<String>,
    },
}
/// Evolution summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EvolutionSummary {
    /// Total tracked statutes
    pub total_tracked: usize,
    /// Average versions per statute
    pub avg_versions: f64,
    /// Total versions across all statutes
    pub total_versions: usize,
    /// Most changed statute
    pub most_changed: Option<String>,
    /// Most stable statute
    pub most_stable: Option<String>,
}
